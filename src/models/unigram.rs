use std::{collections::HashMap, fmt};

use daachorse::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder, Match};
use serde::{Deserialize, Deserializer};

const UNKNOWN_PENALTY: f64 = 10.0;
const UNREACHED_START: usize = usize::MAX;

/// A scored Unigram vocabulary with SentencePiece-compatible Viterbi inference.
#[derive(Clone)]
pub struct Unigram {
    id_to_token: Vec<String>,
    scores: Vec<f64>,
    token_to_id: HashMap<String, u32>,
    automaton: Option<DoubleArrayAhoCorasick<u32>>,
    unk_id: Option<u32>,
    min_score: f64,
    /// Exact `<0xNN>` IDs when byte fallback is enabled.
    byte_fallback_ids: Option<[Option<u32>; 256]>,
}

impl fmt::Debug for Unigram {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Unigram")
            .field("vocab_size", &self.id_to_token.len())
            .field("unk_id", &self.unk_id)
            .field("byte_fallback", &self.byte_fallback_ids.is_some())
            .finish_non_exhaustive()
    }
}

/// `tokenizer.json` Unigram object before the scored tables are built.
///
/// `ModelConfig` owns the `type` tag dispatch, so only Unigram payloads reach this.
#[derive(Deserialize)]
struct UnigramConfig {
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
        let config = UnigramConfig::deserialize(deserializer)?;
        Self::from_parts(config.vocab, config.unk_id, config.byte_fallback)
            .map_err(serde::de::Error::custom)
    }
}

impl Unigram {
    /// Builds an immutable scored vocabulary and its all-prefix automaton.
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

        let byte_fallback_ids = byte_fallback.then(|| {
            let mut ids = [None; 256];
            for byte in 0..=255u8 {
                ids[byte as usize] = token_to_id.get(&format!("<0x{byte:02X}>")).copied();
            }
            ids
        });

        Ok(Self {
            automaton: build_automaton(&id_to_token, &token_to_id)?,
            id_to_token,
            scores,
            token_to_id,
            unk_id: unk_id.map(|id| id as u32),
            min_score,
            byte_fallback_ids,
        })
    }

    /// Appends Unigram Viterbi IDs for one pre-tokenized slice.
    pub(crate) fn append_viterbi_ids(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        scratch: &mut ViterbiScratch,
    ) -> Result<(), String> {
        if input.is_empty() {
            return Ok(());
        }

        let Some(automaton) = &self.automaton else {
            return self.tokenize_without_matches(input, out);
        };
        self.tokenize_matches_into(
            input,
            out,
            scratch,
            automaton.find_overlapping_iter(input.as_bytes()),
        )
    }

    /// Runs one Viterbi pass; an uncovered character takes the unknown fallback
    /// or fails when the model has no `unk_id`.
    ///
    /// An unreached boundary reads as score `0.0`, exactly like Hugging Face's
    /// default best-path nodes, so no reached-boundary read needs a check.
    fn tokenize_matches_into<I>(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        scratch: &mut ViterbiScratch,
        mut matches: I,
    ) -> Result<(), String>
    where
        I: Iterator<Item = Match<u32>>,
    {
        let best = &mut scratch.best;
        best.truncate(input.len() + 1);
        best.resize(input.len() + 1, BestPathNode::unreached());
        best[0] = BestPathNode {
            score: 0.0,
            starts_at: 0,
            id: 0,
        };

        let mut next_match = matches.next();
        for (starts_at, character) in input.char_indices() {
            let current = best[starts_at];
            let character_end = starts_at + character.len_utf8();
            // No prior match can end here because the automaton is end ordered.
            best[character_end].starts_at = UNREACHED_START;

            let mut has_single_character_piece = false;
            while let Some(matched) = next_match {
                if matched.end() != character_end {
                    break;
                }
                let match_start = matched.start();
                has_single_character_piece |= match_start == starts_at;
                let source = best[match_start];
                let id = matched.value();
                let score = source.score + self.scores[id as usize];
                let target = &mut best[character_end];
                // A smaller source offset is the old left-to-right first tie winner.
                if target.starts_at == UNREACHED_START
                    || score > target.score
                    || (score == target.score && match_start < target.starts_at)
                {
                    *target = BestPathNode {
                        score,
                        starts_at: match_start,
                        id,
                    };
                }
                next_match = matches.next();
            }

            if !has_single_character_piece {
                let score = current.score + self.min_score - UNKNOWN_PENALTY;
                let target = &mut best[character_end];
                if target.starts_at == UNREACHED_START || score > target.score {
                    // Hugging Face requires `unk_id` only when the unknown
                    // fallback would win this boundary; a longer piece that
                    // already covers it better keeps encoding without one.
                    let Some(unk_id) = self.unk_id else {
                        return Err("Unigram encountered text but has no unk_id".to_string());
                    };
                    *target = BestPathNode {
                        score,
                        starts_at,
                        id: unk_id,
                    };
                }
            }
        }
        if next_match.is_some() {
            return Err("Unigram matcher reported a non-character boundary".to_string());
        }

        Self::backtrack_into(best, input.len(), &mut scratch.pieces)?;
        self.emit_path_ids(input, &scratch.pieces, out);
        Ok(())
    }

    /// Appends Viterbi IDs for pre-tokenized splits, holding one scratch
    /// workspace per chunk rather than per word.
    pub(crate) fn append_split_viterbi_ids(
        &self,
        buffer: &str,
        splits: &[crate::pre_tokenized::Split],
        out: &mut Vec<u32>,
    ) -> Result<(), String> {
        let mut scratch = ViterbiScratch::default();
        for split in splits {
            if let Some(id) = split.token_id {
                out.push(id);
            } else if !split.range.is_empty() {
                self.append_viterbi_ids(&buffer[split.range.clone()], out, &mut scratch)?;
            }
        }
        Ok(())
    }

    /// Emits the exact fused-unknown result when no nonempty vocabulary piece exists.
    fn tokenize_without_matches(&self, input: &str, out: &mut Vec<u32>) -> Result<(), String> {
        let unk_id = self
            .unk_id
            .ok_or_else(|| "Unigram encountered text but has no unk_id".to_string())?;
        self.emit_path_ids(
            input,
            &[PathPiece {
                id: unk_id,
                starts_at: 0,
                ends_at: input.len(),
            }],
            out,
        );
        Ok(())
    }

    /// Returns the vocabulary text for an ID.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.id_to_token.get(id as usize).map(String::as_str)
    }

    /// Returns the vocabulary ID for exact token text.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Returns the number of entries in the model vocabulary.
    pub fn vocab_size(&self) -> usize {
        self.id_to_token.len()
    }

    /// Reconstructs the highest-scoring path from the final byte boundary.
    fn backtrack_into(
        best: &[BestPathNode],
        mut ends_at: usize,
        reverse: &mut Vec<PathPiece>,
    ) -> Result<(), String> {
        reverse.clear();
        while ends_at != 0 {
            let node = best[ends_at];
            if node.starts_at == UNREACHED_START {
                return Err("Unigram Viterbi path did not reach the final boundary".to_string());
            }
            reverse.push(PathPiece {
                id: node.id,
                starts_at: node.starts_at,
                ends_at,
            });
            ends_at = node.starts_at;
        }
        reverse.reverse();
        Ok(())
    }

    /// Emits regular pieces directly and applies Hugging Face's fused-unknown fallback.
    /// Writes IDs for a Viterbi path, fusing adjacent unknown pieces first.
    fn emit_path_ids(&self, input: &str, pieces: &[PathPiece], out: &mut Vec<u32>) {
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
            // Hugging Face resolves the fused unknown spelling in the
            // vocabulary before `<0xNN>` pieces, so a Viterbi unk run whose
            // text is itself a token (commonly `"<unk>"`) keeps that ID.
            if let Some(&id) = self.token_to_id.get(unknown) {
                out.push(id);
            } else if !self.emit_byte_fallback_ids(unknown, out) {
                out.push(piece.id);
            }
        }
    }

    /// Emits `<0xNN>` pieces only when every byte has an exact vocabulary entry.
    fn emit_byte_fallback_ids(&self, unknown: &str, out: &mut Vec<u32>) -> bool {
        let Some(byte_fallback_ids) = &self.byte_fallback_ids else {
            return false;
        };
        // Two passes keep the all-or-nothing contract without a temporary buffer.
        if !unknown
            .bytes()
            .all(|byte| byte_fallback_ids[byte as usize].is_some())
        {
            return false;
        }
        out.extend(
            unknown
                .bytes()
                .filter_map(|byte| byte_fallback_ids[byte as usize]),
        );
        true
    }
}

#[derive(Clone, Copy)]
struct BestPathNode {
    score: f64,
    starts_at: usize,
    id: u32,
}

impl BestPathNode {
    /// Marks the one endpoint being reconsidered before its end-ordered match group arrives.
    const fn unreached() -> Self {
        Self {
            score: 0.0,
            starts_at: UNREACHED_START,
            id: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct PathPiece {
    id: u32,
    starts_at: usize,
    ends_at: usize,
}

/// Per-chunk Viterbi buffers, reused only after each independent split finishes.
#[derive(Default)]
pub(crate) struct ViterbiScratch {
    best: Vec<BestPathNode>,
    pieces: Vec<PathPiece>,
}

/// Builds an all-match automaton with the same last-duplicate vocabulary IDs as Hugging Face.
fn build_automaton(
    tokens: &[String],
    token_to_id: &HashMap<String, u32>,
) -> Result<Option<DoubleArrayAhoCorasick<u32>>, String> {
    let patterns = tokens
        .iter()
        .enumerate()
        .filter_map(|(id, token)| {
            (token_to_id.get(token) == Some(&(id as u32)) && !token.is_empty())
                .then_some((token.as_str(), id as u32))
        })
        .collect::<Vec<_>>();
    (!patterns.is_empty())
        .then(|| {
            DoubleArrayAhoCorasickBuilder::new()
                .build_with_values(patterns)
                .map_err(|error| format!("error building Unigram prefix automaton: {error}"))
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::{Unigram, ViterbiScratch};

    fn ids(unigram: &Unigram, input: &str) -> Result<Vec<u32>, String> {
        let mut out = Vec::new();
        let mut scratch = ViterbiScratch::default();
        unigram.append_viterbi_ids(input, &mut out, &mut scratch)?;
        Ok(out)
    }

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
        assert_eq!(ids(&unigram, "abc").unwrap(), vec![1, 3]);
    }

    #[test]
    fn preserves_the_first_prefix_on_equal_scores() {
        let unigram = model(
            &[("<unk>", 0.0), ("a", 0.0), ("b", 1.0), ("ab", 1.0)],
            false,
        );
        assert_eq!(ids(&unigram, "ab").unwrap(), vec![3]);
    }

    #[test]
    fn fuses_adjacent_unknown_characters() {
        let unigram = model(&[("<unk>", 0.0), ("a", 0.0)], false);
        assert_eq!(ids(&unigram, "a☃b").unwrap(), vec![1, 0]);
    }

    #[test]
    fn all_empty_vocabulary_uses_one_fused_unknown_piece() {
        let unigram = model(&[("", 0.0)], false);
        assert_eq!(ids(&unigram, "abc").unwrap(), vec![0]);
    }

    #[test]
    fn no_unknown_id_keeps_checked_character_coverage() {
        let unigram = Unigram::from_parts(vec![("a".to_string(), 0.0)], None, false).unwrap();
        assert_eq!(ids(&unigram, "a").unwrap(), vec![0]);
        assert_eq!(
            ids(&unigram, "b").unwrap_err(),
            "Unigram encountered text but has no unk_id"
        );
    }

    #[test]
    fn no_unknown_id_allows_longer_pieces_to_cover_characters() {
        // Matches Hugging Face: the unknown fallback is required only where it
        // would win a boundary, so a longer covering piece keeps encoding.
        let unigram = Unigram::from_parts(
            vec![("ab".to_string(), 0.0), ("a".to_string(), -1.0)],
            None,
            false,
        )
        .unwrap();
        assert_eq!(ids(&unigram, "ab").unwrap(), vec![0]);

        let unigram = Unigram::from_parts(vec![("ab".to_string(), 0.0)], None, false).unwrap();
        assert_eq!(
            ids(&unigram, "ab").unwrap_err(),
            "Unigram encountered text but has no unk_id"
        );
    }

    #[test]
    fn uses_byte_fallback_only_when_every_byte_piece_exists() {
        let unigram = model(&[("<unk>", 0.0), ("<0xC3>", 0.0), ("<0xA9>", 0.0)], true);
        assert_eq!(ids(&unigram, "é").unwrap(), vec![1, 2]);
    }

    #[test]
    fn fused_unknown_vocab_spelling_wins_over_byte_fallback() {
        let mut vocab = vec![("<unk>".to_string(), 0.0), ("a".to_string(), -1.0)];
        for byte in 0..=255u8 {
            vocab.push((format!("<0x{byte:02X}>"), -5.0));
        }
        let unigram = Unigram::from_parts(vocab, Some(0), true).unwrap();
        assert_eq!(ids(&unigram, "<unk>").unwrap(), vec![0]);
        assert_eq!(ids(&unigram, "a<unk>a").unwrap(), vec![1, 0, 1]);
    }

    #[test]
    fn accepts_legacy_untagged_hugging_face_json() {
        let unigram: Unigram =
            serde_json::from_str(r#"{"unk_id":0,"vocab":[["<unk>",0.0],["a",1.0]]}"#).unwrap();
        assert_eq!(ids(&unigram, "a").unwrap(), vec![1]);
    }

    #[test]
    fn last_duplicate_piece_id_wins_on_equal_length_matches() {
        let unigram = model(
            &[
                ("<unk>", 0.0),
                ("a", 0.0),
                ("ab", 0.0),
                ("a", 1.0),
                ("b", 0.0),
            ],
            false,
        );
        // The later "a" spelling keeps ID 3; its score beats the "ab" piece.
        assert_eq!(ids(&unigram, "ab").unwrap(), vec![3, 4]);
    }

    #[test]
    fn scratch_batch_preserves_split_boundaries_and_added_ids() {
        let unigram = model(
            &[
                ("<unk>", 0.0),
                ("a", 0.0),
                ("b", 0.0),
                ("ab", 2.0),
                ("c", 0.0),
            ],
            false,
        );
        let splits = vec![
            crate::pre_tokenized::Split {
                range: 0..2,
                token_id: None,
            },
            crate::pre_tokenized::Split {
                range: 2..3,
                token_id: Some(99),
            },
            crate::pre_tokenized::Split {
                range: 3..5,
                token_id: None,
            },
        ];

        let mut ids = Vec::new();
        unigram
            .append_split_viterbi_ids("ab!zc", &splits, &mut ids)
            .unwrap();
        assert_eq!(ids, vec![3, 99, 0, 4]);
    }
}
