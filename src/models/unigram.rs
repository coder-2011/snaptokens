use std::{collections::HashMap, fmt};

use daachorse::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder};
use serde::{Deserialize, Deserializer};

const UNKNOWN_PENALTY: f64 = 10.0;
const UNREACHED_START: usize = usize::MAX;

/// A scored Unigram vocabulary with SentencePiece-compatible Viterbi inference.
#[derive(Clone, Debug)]
pub struct Unigram {
    id_to_token: Vec<String>,
    scores: Vec<f64>,
    token_to_id: HashMap<String, u32>,
    matcher: PrefixMatcher,
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
            matcher: PrefixMatcher::from_tokens(&id_to_token, &token_to_id)?,
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
        let mut scratch = ViterbiScratch::default();
        self.tokenize_into_with_scratch(input, out, &mut scratch)
    }

    /// Tokenizes independent pre-tokenized splits while reusing one Viterbi workspace.
    pub(crate) fn tokenize_splits_into(
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
                self.tokenize_into_with_scratch(&buffer[split.range.clone()], out, &mut scratch)?;
            }
        }
        Ok(())
    }

    /// Runs Viterbi using cleared, chunk-local workspace from a preceding split.
    fn tokenize_into_with_scratch(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        scratch: &mut ViterbiScratch,
    ) -> Result<(), String> {
        if input.is_empty() {
            return Ok(());
        }

        let Some(automaton) = &self.matcher.automaton else {
            return self.tokenize_without_matches(input, out);
        };
        if let Some(unk_id) = self.unk_id {
            self.tokenize_reachable_matches_into(
                input,
                out,
                scratch,
                automaton.find_overlapping_iter(input.as_bytes()),
                unk_id,
            )
        } else {
            self.tokenize_checked_matches_into(
                input,
                out,
                scratch,
                automaton.find_overlapping_iter(input.as_bytes()),
            )
        }
    }

    /// Retains checked Viterbi reachability for Unigram models without an unknown ID.
    fn tokenize_checked_matches_into<I>(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        scratch: &mut ViterbiScratch,
        mut matches: I,
    ) -> Result<(), String>
    where
        I: Iterator<Item = UnigramMatch>,
    {
        let best = &mut scratch.best;
        best.clear();
        best.resize(input.len() + 1, None);
        best[0] = Some(BestPathNode {
            score: 0.0,
            starts_at: 0,
            id: 0,
        });

        let mut next_match = matches.next();
        for (starts_at, character) in input.char_indices() {
            let current = best[starts_at].ok_or_else(|| {
                "Unigram Viterbi path ended before a character boundary".to_string()
            })?;
            let character_end = starts_at + character.len_utf8();

            let mut has_single_character_piece = false;
            while let Some(matched) = next_match {
                if matched.end != character_end {
                    break;
                }
                let match_start = matched.start();
                has_single_character_piece |= match_start == starts_at;
                let source = best[match_start].ok_or_else(|| {
                    "Unigram Viterbi path ended before a match boundary".to_string()
                })?;
                let id = matched.id;
                let score = source.score + self.scores[id as usize];
                let target = &mut best[character_end];
                // A smaller source offset is the old left-to-right first tie winner.
                if target.is_none_or(|node: BestPathNode| {
                    score > node.score || (score == node.score && match_start < node.starts_at)
                }) {
                    *target = Some(BestPathNode {
                        score,
                        starts_at: match_start,
                        id,
                    });
                }
                next_match = matches.next();
            }

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
        if next_match.is_some() {
            return Err("Unigram matcher reported a non-character boundary".to_string());
        }

        Self::backtrack_into(best, input.len(), &mut scratch.pieces)?;
        self.append_ids_for_pieces(input, &scratch.pieces, out)
    }

    /// Uses the guaranteed unknown fallback to reset only the next character boundary.
    fn tokenize_reachable_matches_into<I>(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        scratch: &mut ViterbiScratch,
        mut matches: I,
        unk_id: u32,
    ) -> Result<(), String>
    where
        I: Iterator<Item = UnigramMatch>,
    {
        let best = &mut scratch.reachable_best;
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
                if matched.end != character_end {
                    break;
                }
                let match_start = matched.start();
                has_single_character_piece |= match_start == starts_at;
                let source = best[match_start];
                let id = matched.id;
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

        Self::backtrack_reachable_into(best, input.len(), &mut scratch.pieces)?;
        self.append_ids_for_pieces(input, &scratch.pieces, out)
    }

    /// Emits the exact fused-unknown result when no nonempty vocabulary piece exists.
    fn tokenize_without_matches(&self, input: &str, out: &mut Vec<u32>) -> Result<(), String> {
        let unk_id = self
            .unk_id
            .ok_or_else(|| "Unigram encountered text but has no unk_id".to_string())?;
        self.append_ids_for_pieces(
            input,
            &[PathPiece {
                id: unk_id,
                starts_at: 0,
                ends_at: input.len(),
            }],
            out,
        )
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
    fn backtrack_into(
        best: &[Option<BestPathNode>],
        mut ends_at: usize,
        reverse: &mut Vec<PathPiece>,
    ) -> Result<(), String> {
        reverse.clear();
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
        Ok(())
    }

    /// Reconstructs a path whose character-boundary reachability is guaranteed by `unk_id`.
    fn backtrack_reachable_into(
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

/// Holds one end-ordered vocabulary match for the shared Viterbi recurrence.
#[derive(Clone, Copy)]
struct UnigramMatch {
    length: u32,
    end: usize,
    id: u32,
}

impl UnigramMatch {
    /// Recovers the start offset from the validated nonempty output length.
    #[inline(always)]
    fn start(self) -> usize {
        self.end - self.length as usize
    }
}

/// Per-chunk Viterbi buffers, reused only after each independent split finishes.
#[derive(Default)]
struct ViterbiScratch {
    best: Vec<Option<BestPathNode>>,
    reachable_best: Vec<BestPathNode>,
    pieces: Vec<PathPiece>,
}

/// A bytewise all-match automaton for Viterbi's scored vocabulary pieces.
#[derive(Clone)]
struct PrefixMatcher {
    automaton: Option<FastDoubleArray>,
}

impl PrefixMatcher {
    /// Builds an all-match automaton with the same last-duplicate vocabulary IDs as Hugging Face.
    fn from_tokens(tokens: &[String], token_to_id: &HashMap<String, u32>) -> Result<Self, String> {
        let patterns = tokens
            .iter()
            .enumerate()
            .filter_map(|(id, token)| {
                (token_to_id.get(token) == Some(&(id as u32)) && !token.is_empty())
                    .then_some((token.as_str(), id as u32))
            })
            .collect::<Vec<_>>();
        let automaton = (!patterns.is_empty())
            .then(|| {
                DoubleArrayAhoCorasickBuilder::new()
                    .build_with_values(patterns)
                    .map_err(|error| format!("error building Unigram prefix automaton: {error}"))
            })
            .transpose()?
            .map(FastDoubleArray::from_daachorse)
            .transpose()?;
        Ok(Self { automaton })
    }

    /// Appends every overlapping vocabulary match in nondecreasing end-offset order for tests.
    #[cfg(test)]
    fn find_matches(&self, input: &[u8], out: &mut Vec<(usize, usize, u32)>) {
        if let Some(automaton) = &self.automaton {
            out.extend(
                automaton
                    .find_overlapping_iter(input)
                    .map(|matched| (matched.start(), matched.end, matched.id)),
            );
        }
    }
}

#[derive(Clone, Copy)]
struct DoubleArrayState {
    base: u32,
    fail: u32,
    output_and_check: u32,
}

#[derive(Clone, Copy)]
struct DoubleArrayOutput {
    id: u32,
    length: u32,
    parent: u32,
}

/// Stores validated Daachorse tables while preserving its overlapping iterator schedule.
#[derive(Clone)]
struct FastDoubleArray {
    states: Vec<DoubleArrayState>,
    outputs: Vec<DoubleArrayOutput>,
}

impl FastDoubleArray {
    /// Decodes the pinned Daachorse 1.0.0 serialization after validating every iterator reference.
    fn from_daachorse(automaton: DoubleArrayAhoCorasick<u32>) -> Result<Self, String> {
        let serialized = automaton.serialize();
        let mut cursor = 0;
        let states_len = Self::read_u32(&serialized, &mut cursor)? as usize;
        if states_len < 2 {
            return Err("Unigram double-array serialization has no root and dead states".into());
        }

        let mut states = Vec::with_capacity(states_len);
        for _ in 0..states_len {
            states.push(DoubleArrayState {
                base: Self::read_u32(&serialized, &mut cursor)?,
                fail: Self::read_u32(&serialized, &mut cursor)?,
                output_and_check: Self::read_u32(&serialized, &mut cursor)?,
            });
        }

        let outputs_len = Self::read_u32(&serialized, &mut cursor)? as usize;
        let mut outputs = Vec::with_capacity(outputs_len);
        for _ in 0..outputs_len {
            outputs.push(DoubleArrayOutput {
                id: Self::read_u32(&serialized, &mut cursor)?,
                length: Self::read_u32(&serialized, &mut cursor)?,
                parent: Self::read_u32(&serialized, &mut cursor)?,
            });
        }

        let match_kind = *serialized.get(cursor).ok_or_else(|| {
            "Unigram double-array serialization is missing match kind".to_string()
        })?;
        cursor += 1;
        let num_states = Self::read_u32(&serialized, &mut cursor)? as usize;
        if cursor != serialized.len() || match_kind != 0 || num_states > states.len() {
            return Err("Unigram double-array serialization has an unsupported layout".into());
        }
        for state in &states {
            if state.fail as usize >= states.len()
                || (state.output_and_check >> 8) as usize > outputs.len()
            {
                return Err(
                    "Unigram double-array serialization has an invalid state reference".into(),
                );
            }
        }
        for output in &outputs {
            if output.length == 0 || output.parent as usize > outputs.len() {
                return Err(
                    "Unigram double-array serialization has an invalid output reference".into(),
                );
            }
        }
        Ok(Self { states, outputs })
    }

    /// Reads one little-endian field from Daachorse's fixed-width 1.0.0 serialization.
    fn read_u32(serialized: &[u8], cursor: &mut usize) -> Result<u32, String> {
        let end = cursor
            .checked_add(4)
            .ok_or_else(|| "Unigram double-array serialization length overflowed".to_string())?;
        let field: [u8; 4] = serialized
            .get(*cursor..end)
            .ok_or_else(|| "Unigram double-array serialization ended unexpectedly".to_string())?
            .try_into()
            .map_err(|_| "Unigram double-array serialization has an invalid field".to_string())?;
        *cursor = end;
        Ok(u32::from_le_bytes(field))
    }

    /// Starts an iterator whose output order matches Daachorse's overlapping iterator exactly.
    #[inline(always)]
    fn find_overlapping_iter<'a>(&'a self, input: &'a [u8]) -> FastOverlappingIterator<'a> {
        FastOverlappingIterator {
            automaton: self,
            input,
            state: 0,
            scanned: 0,
            end: 0,
            output: 0,
        }
    }

    /// Follows the same byte transition and failure links as Daachorse's overlapping iterator.
    #[inline(always)]
    fn advance(&self, mut state: usize, byte: u8) -> usize {
        loop {
            let current = self.states[state];
            if current.base != 0 {
                let child = (current.base ^ u32::from(byte)) as usize;
                if child < self.states.len() && (self.states[child].output_and_check as u8) == byte
                {
                    return child;
                }
            }
            if state == 0 {
                return 0;
            }
            state = current.fail as usize;
        }
    }

    /// Returns the first one-based output-chain index for a reached automaton state.
    #[inline(always)]
    fn output_position(&self, state: usize) -> u32 {
        self.states[state].output_and_check >> 8
    }

    /// Returns the already-validated one-based output entry used by the current end group.
    #[inline(always)]
    fn output(&self, position: u32) -> DoubleArrayOutput {
        self.outputs[(position - 1) as usize]
    }
}

/// Iterates a validated double array without coupling its byte cursor to Viterbi's character loop.
struct FastOverlappingIterator<'a> {
    automaton: &'a FastDoubleArray,
    input: &'a [u8],
    state: usize,
    scanned: usize,
    end: usize,
    output: u32,
}

impl Iterator for FastOverlappingIterator<'_> {
    type Item = UnigramMatch;

    /// Finishes one output chain before scanning to the next output-bearing byte state.
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.output != 0 {
                let output = self.automaton.output(self.output);
                self.output = output.parent;
                return Some(UnigramMatch {
                    length: output.length,
                    end: self.end,
                    id: output.id,
                });
            }

            let byte = *self.input.get(self.scanned)?;
            self.state = self.automaton.advance(self.state, byte);
            self.scanned += 1;
            self.output = self.automaton.output_position(self.state);
            self.end = self.scanned;
        }
    }
}

impl fmt::Debug for PrefixMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrefixMatcher")
            .field("has_automaton", &self.automaton.is_some())
            .finish()
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
    fn all_empty_vocabulary_uses_one_fused_unknown_piece() {
        let unigram = model(&[("", 0.0)], false);
        assert_eq!(unigram.tokenize("abc").unwrap(), vec![0]);
    }

    #[test]
    fn no_unknown_id_keeps_checked_character_coverage() {
        let unigram = Unigram::from_parts(vec![("a".to_string(), 0.0)], None, false).unwrap();
        assert_eq!(unigram.tokenize("a").unwrap(), vec![0]);
        assert_eq!(
            unigram.tokenize("b").unwrap_err(),
            "Unigram encountered text but has no unk_id"
        );
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

    #[test]
    fn matcher_reports_overlapping_last_duplicate_pieces() {
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
        let mut matches = Vec::new();
        unigram.matcher.find_matches(b"ab", &mut matches);
        assert_eq!(matches, vec![(0, 1, 3), (0, 2, 2), (1, 2, 4)]);
    }

    #[test]
    fn matcher_preserves_multibyte_piece_boundaries() {
        let unigram = model(
            &[("<unk>", 0.0), ("▁", 0.0), ("▁a", 0.0), ("a", 0.0)],
            false,
        );
        let mut matches = Vec::new();
        unigram.matcher.find_matches("▁a".as_bytes(), &mut matches);
        assert_eq!(matches, vec![(0, 3, 1), (0, 4, 2), (3, 4, 3)]);
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
            .tokenize_splits_into("ab!zc", &splits, &mut ids)
            .unwrap();
        assert_eq!(ids, vec![3, 99, 0, 4]);
    }
}
