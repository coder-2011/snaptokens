use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

const UNKNOWN_PENALTY: f64 = 10.0;

/// A scored Unigram vocabulary with SentencePiece-compatible Viterbi inference.
#[derive(Clone, Debug)]
pub struct Unigram {
    id_to_token: Vec<String>,
    scores: Vec<f64>,
    token_to_id: HashMap<String, u32>,
    matcher: PrefixTrie,
    unk_id: Option<u32>,
    min_score: f64,
    byte_fallback: bool,
}

#[derive(Deserialize)]
struct RawUnigram {
    #[serde(rename = "type")]
    model_type: Option<String>,
    vocab: Vec<(String, f64)>,
    #[serde(default)]
    unk_id: Option<usize>,
    #[serde(default)]
    byte_fallback: bool,
}

impl<'de> Deserialize<'de> for Unigram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawUnigram::deserialize(deserializer)?;
        if let Some(model_type) = raw.model_type
            && model_type != "Unigram"
        {
            return Err(serde::de::Error::custom(format!(
                "unsupported model type: {model_type}"
            )));
        }
        Self::from_parts(raw.vocab, raw.unk_id, raw.byte_fallback).map_err(serde::de::Error::custom)
    }
}

impl Unigram {
    /// Builds an immutable scored vocabulary and its all-prefix matcher.
    pub(crate) fn from_parts(
        vocab: Vec<(String, f64)>,
        unk_id: Option<usize>,
        byte_fallback: bool,
    ) -> Result<Self, String> {
        if let Some(unk_id) = unk_id
            && unk_id >= vocab.len()
        {
            return Err("Unigram unk_id is outside the vocabulary".into());
        }
        if vocab.len() > u32::MAX as usize {
            return Err("Unigram vocabulary exceeds u32 token IDs".into());
        }

        let mut id_to_token = Vec::with_capacity(vocab.len());
        let mut scores = Vec::with_capacity(vocab.len());
        let mut token_to_id = HashMap::with_capacity(vocab.len());
        let mut min_score = f64::INFINITY;
        for (id, (token, score)) in vocab.into_iter().enumerate() {
            if !score.is_finite() {
                return Err("Unigram piece score must be finite".into());
            }
            min_score = min_score.min(score);
            token_to_id.insert(token.clone(), id as u32);
            id_to_token.push(token);
            scores.push(score);
        }

        Ok(Self {
            matcher: PrefixTrie::from_tokens(&id_to_token)?,
            id_to_token,
            scores,
            token_to_id,
            unk_id: unk_id.map(|id| id as u32),
            min_score,
            byte_fallback,
        })
    }

    /// Appends the best-scoring exact SentencePiece segmentation for one split.
    pub fn tokenize_into(&self, input: &str, out: &mut Vec<u32>) -> Result<(), String> {
        if input.is_empty() {
            return Ok(());
        }

        let bytes = input.as_bytes();
        let mut best = vec![None; input.len() + 1];
        best[0] = Some(BestPathNode {
            score: 0.0,
            starts_at: 0,
            id: 0,
        });

        for (starts_at, character) in input.char_indices() {
            let current = best[starts_at].ok_or_else(|| {
                "Unigram Viterbi path ended before a character boundary".to_string()
            })?;
            let character_end = starts_at + character.len_utf8();
            let mut has_single_character_piece = false;
            self.matcher
                .for_each_prefix(bytes, starts_at, |ends_at, id| {
                    let score = current.score + self.scores[id as usize];
                    let target = &mut best[ends_at];
                    // Strictly greater preserves Hugging Face's first-prefix tie break.
                    if target.is_none_or(|node: BestPathNode| score > node.score) {
                        *target = Some(BestPathNode {
                            score,
                            starts_at,
                            id,
                        });
                    }
                    has_single_character_piece |= ends_at == character_end;
                });

            if !has_single_character_piece {
                let unk_id = self
                    .unk_id
                    .ok_or_else(|| "Unigram encountered text but has no unk_id".to_string())?;
                let score = current.score + self.min_score - UNKNOWN_PENALTY;
                let target = &mut best[character_end];
                if target.is_none_or(|node: BestPathNode| score > node.score) {
                    *target = Some(BestPathNode {
                        score,
                        starts_at,
                        id: unk_id,
                    });
                }
            }
        }

        let pieces = Self::backtrack(&best, input.len())?;
        self.append_ids_for_pieces(input, &pieces, out)
    }

    /// Returns token IDs in a standalone allocation for callers using the older API.
    pub fn tokenize(&self, input: &str) -> Result<Vec<u32>, String> {
        let mut ids = Vec::new();
        self.tokenize_into(input, &mut ids)?;
        Ok(ids)
    }

    /// Returns the exact vocabulary spelling assigned to one model ID.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.id_to_token.get(id as usize).map(String::as_str)
    }

    /// Returns the final vocabulary ID for one token spelling, matching Hugging Face.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Returns the number of scored vocabulary entries, including duplicate spellings.
    pub fn vocab_size(&self) -> usize {
        self.id_to_token.len()
    }

    /// Reconstructs the highest-scoring path from the final byte boundary.
    fn backtrack(
        best: &[Option<BestPathNode>],
        mut ends_at: usize,
    ) -> Result<Vec<PathPiece>, String> {
        let mut reverse = Vec::new();
        while ends_at != 0 {
            let node = best[ends_at].ok_or_else(|| {
                "Unigram Viterbi path did not reach the final boundary".to_string()
            })?;
            reverse.push(PathPiece {
                id: node.id,
                starts_at: node.starts_at,
                ends_at,
            });
            ends_at = node.starts_at;
        }
        reverse.reverse();
        Ok(reverse)
    }

    /// Emits regular pieces directly and applies Hugging Face's fused-unknown fallback.
    fn append_ids_for_pieces(
        &self,
        input: &str,
        pieces: &[PathPiece],
        out: &mut Vec<u32>,
    ) -> Result<(), String> {
        let mut index = 0;
        while index < pieces.len() {
            let piece = pieces[index];
            if Some(piece.id) != self.unk_id {
                out.push(piece.id);
                index += 1;
                continue;
            }

            let start = piece.starts_at;
            let mut end = piece.ends_at;
            index += 1;
            while index < pieces.len() && Some(pieces[index].id) == self.unk_id {
                end = pieces[index].ends_at;
                index += 1;
            }
            let unknown = &input[start..end];
            if !self.append_byte_fallback(unknown, out) {
                out.push(piece.id);
            }
        }
        Ok(())
    }

    /// Emits `<0xNN>` pieces only when every byte has an exact vocabulary entry.
    fn append_byte_fallback(&self, unknown: &str, out: &mut Vec<u32>) -> bool {
        if !self.byte_fallback {
            return false;
        }
        let mut byte_ids = Vec::with_capacity(unknown.len());
        for byte in unknown.bytes() {
            let spelling = format!("<0x{byte:02X}>");
            let Some(id) = self.token_to_id(&spelling) else {
                return false;
            };
            byte_ids.push(id);
        }
        out.extend(byte_ids);
        true
    }
}

#[derive(Clone, Copy)]
struct BestPathNode {
    score: f64,
    starts_at: usize,
    id: u32,
}

#[derive(Clone, Copy)]
struct PathPiece {
    id: u32,
    starts_at: usize,
    ends_at: usize,
}

/// A compact byte trie that visits every vocabulary prefix of an input suffix.
#[derive(Clone, Debug)]
struct PrefixTrie {
    nodes: Vec<TrieNode>,
    edges: Vec<TrieEdge>,
}

#[derive(Clone, Copy, Debug)]
struct TrieNode {
    first_edge: usize,
    edge_count: usize,
    token_id: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
struct TrieEdge {
    byte: u8,
    node: usize,
}

impl PrefixTrie {
    /// Builds a deterministic prefix matcher, retaining Hugging Face's last duplicate ID.
    fn from_tokens(tokens: &[String]) -> Result<Self, String> {
        #[derive(Default)]
        struct BuildNode {
            token_id: Option<u32>,
            children: Vec<(u8, usize)>,
        }

        let mut build_nodes = vec![BuildNode::default()];
        for (id, token) in tokens.iter().enumerate() {
            let id = u32::try_from(id).map_err(|_| "Unigram vocabulary exceeds u32 token IDs")?;
            if token.is_empty() {
                continue;
            }
            let mut node = 0;
            for &byte in token.as_bytes() {
                let next = match build_nodes[node]
                    .children
                    .iter()
                    .find(|(existing, _)| *existing == byte)
                {
                    Some((_, child)) => *child,
                    None => {
                        let child = build_nodes.len();
                        build_nodes.push(BuildNode::default());
                        build_nodes[node].children.push((byte, child));
                        child
                    }
                };
                node = next;
            }
            build_nodes[node].token_id = Some(id);
        }

        let mut nodes = Vec::with_capacity(build_nodes.len());
        let mut edges = Vec::new();
        for mut node in build_nodes {
            node.children.sort_unstable_by_key(|(byte, _)| *byte);
            let first_edge = edges.len();
            edges.extend(
                node.children
                    .into_iter()
                    .map(|(byte, node)| TrieEdge { byte, node }),
            );
            nodes.push(TrieNode {
                first_edge,
                edge_count: edges.len() - first_edge,
                token_id: node.token_id,
            });
        }
        Ok(Self { nodes, edges })
    }

    /// Calls `visit` in increasing-prefix-length order for one input boundary.
    fn for_each_prefix(&self, input: &[u8], starts_at: usize, mut visit: impl FnMut(usize, u32)) {
        let mut node = 0;
        for (offset, &byte) in input[starts_at..].iter().enumerate() {
            let current = self.nodes[node];
            let edges = &self.edges[current.first_edge..current.first_edge + current.edge_count];
            let Ok(edge) = edges.binary_search_by_key(&byte, |edge| edge.byte) else {
                break;
            };
            node = edges[edge].node;
            if let Some(id) = self.nodes[node].token_id {
                visit(starts_at + offset + 1, id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Unigram;

    fn model(vocab: &[(&str, f64)], byte_fallback: bool) -> Unigram {
        Unigram::from_parts(
            vocab
                .iter()
                .map(|(token, score)| ((*token).to_string(), *score))
                .collect(),
            Some(0),
            byte_fallback,
        )
        .unwrap()
    }

    #[test]
    fn chooses_the_global_viterbi_path() {
        let unigram = model(
            &[
                ("<unk>", 0.0),
                ("a", 0.0),
                ("ab", 2.0),
                ("bc", 5.0),
                ("c", 0.0),
            ],
            false,
        );
        assert_eq!(unigram.tokenize("abc").unwrap(), vec![1, 3]);
    }

    #[test]
    fn preserves_the_first_prefix_on_equal_scores() {
        let unigram = model(
            &[("<unk>", 0.0), ("a", 0.0), ("b", 1.0), ("ab", 1.0)],
            false,
        );
        assert_eq!(unigram.tokenize("ab").unwrap(), vec![3]);
    }

    #[test]
    fn fuses_adjacent_unknown_characters() {
        let unigram = model(&[("<unk>", 0.0), ("a", 0.0)], false);
        assert_eq!(unigram.tokenize("a☃b").unwrap(), vec![1, 0]);
    }

    #[test]
    fn uses_byte_fallback_only_when_every_byte_piece_exists() {
        let unigram = model(&[("<unk>", 0.0), ("<0xC3>", 0.0), ("<0xA9>", 0.0)], true);
        assert_eq!(unigram.tokenize("é").unwrap(), vec![1, 2]);
    }

    #[test]
    fn accepts_legacy_untagged_hugging_face_json() {
        let unigram: Unigram =
            serde_json::from_str(r#"{"unk_id":0,"vocab":[["<unk>",0.0],["a",1.0]]}"#).unwrap();
        assert_eq!(unigram.tokenize("a").unwrap(), vec![1]);
    }
}
