use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::json_structs::PostProcessorConfig;

/// An error while constructing a post-processor.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A template or special-token definition was invalid.
    #[error("invalid post-processor config: {0}")]
    InvalidConfig(String),
}

/// A sequence referenced by a template post-processor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SequenceId {
    /// The primary encoded sequence.
    A,
    /// The optional pair sequence.
    B,
}

/// One operation in a template post-processor.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum TemplatePiece {
    /// Inserts one encoded sequence.
    Sequence {
        /// The template sequence to insert.
        id: SequenceId,
        /// Type ID stamped on every inserted position.
        type_id: u32,
    },
    /// Inserts IDs for a named configured special token.
    SpecialToken {
        /// Name of the special-token definition to insert.
        id: String,
        /// Type ID stamped on every inserted position.
        type_id: u32,
    },
}

/// Token IDs with the aligned per-token metadata produced by post-processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostProcessed {
    /// Final token IDs.
    pub ids: Vec<u32>,
    /// Per-token type IDs from template pieces, or `1` on a plainly
    /// concatenated second sequence.
    pub type_ids: Vec<u32>,
    /// Ones exactly at positions inserted from template special tokens.
    pub special_tokens_mask: Vec<u32>,
}

impl PostProcessed {
    pub(crate) fn untyped(ids: Vec<u32>) -> Self {
        let len = ids.len();
        Self {
            ids,
            type_ids: vec![0; len],
            special_tokens_mask: vec![0; len],
        }
    }
}

// Hugging Face's default when no template merges a pair: concatenate with type 1.
pub(crate) fn concat_pair(first: Vec<u32>, second: &[u32]) -> PostProcessed {
    let mut type_ids = vec![0; first.len()];
    type_ids.resize(first.len() + second.len(), 1);
    let mut ids = first;
    ids.extend_from_slice(second);
    let len = ids.len();
    PostProcessed {
        ids,
        type_ids,
        special_tokens_mask: vec![0; len],
    }
}

#[derive(Debug, Deserialize)]
struct SpecialTokenDef {
    id: String,
    ids: Vec<u32>,
}

/// A single- and pair-sequence template post-processor.
#[derive(Debug)]
pub struct TemplateProcessing {
    single: Vec<TemplatePiece>,
    pair: Vec<TemplatePiece>,
    special_tokens: HashMap<String, Vec<u32>>,
}

impl TemplateProcessing {
    /// Builds a template post-processor from raw Hugging Face JSON values.
    pub fn from_config(
        single: Value,
        pair: Value,
        special_tokens_val: Value,
    ) -> Result<Self, Error> {
        let single: Vec<TemplatePiece> = match single {
            Value::Null => vec![],
            v => serde_json::from_value(v)
                .map_err(|e| Error::InvalidConfig(format!("single template: {e}")))?,
        };
        let pair: Vec<TemplatePiece> = match pair {
            Value::Null => vec![],
            v => serde_json::from_value(v)
                .map_err(|e| Error::InvalidConfig(format!("pair template: {e}")))?,
        };
        let special_tokens_raw: HashMap<String, SpecialTokenDef> = match special_tokens_val {
            Value::Null => HashMap::new(),
            v => serde_json::from_value(v)
                .map_err(|e| Error::InvalidConfig(format!("special_tokens: {e}")))?,
        };

        let special_tokens: HashMap<String, Vec<u32>> = special_tokens_raw
            .into_values()
            .map(|v| (v.id, v.ids))
            .collect();

        Ok(Self {
            single,
            pair,
            special_tokens,
        })
    }

    // Hugging Face applies templates even without special tokens requested,
    // dropping only the SpecialToken insertions; sequence pieces still stamp types.
    fn apply_template(
        &self,
        template: &[TemplatePiece],
        first: &[u32],
        second: Option<&[u32]>,
        add_special_tokens: bool,
    ) -> PostProcessed {
        let mut out = PostProcessed {
            ids: Vec::with_capacity(first.len() + second.map_or(0, <[u32]>::len) + 4),
            type_ids: Vec::new(),
            special_tokens_mask: Vec::new(),
        };
        for piece in template {
            match piece {
                TemplatePiece::Sequence { id, type_id } => {
                    let sequence = match id {
                        SequenceId::A => first,
                        SequenceId::B => second.unwrap_or(&[]),
                    };
                    out.ids.extend_from_slice(sequence);
                    out.type_ids.resize(out.ids.len(), *type_id);
                    out.special_tokens_mask.resize(out.ids.len(), 0);
                }
                TemplatePiece::SpecialToken { id, type_id } => {
                    if add_special_tokens && let Some(ids) = self.special_tokens.get(id) {
                        out.ids.extend_from_slice(ids);
                        out.type_ids.resize(out.ids.len(), *type_id);
                        out.special_tokens_mask.resize(out.ids.len(), 1);
                    }
                }
            }
        }
        out
    }

    /// Applies the pair template to two encoded sequences.
    pub fn apply_pair(
        &self,
        first: &[u32],
        second: &[u32],
        add_special_tokens: bool,
    ) -> PostProcessed {
        self.apply_template(&self.pair, first, Some(second), add_special_tokens)
    }

    /// Applies the single-sequence template to encoded IDs.
    pub fn apply_single(&self, encoded: Vec<u32>) -> Vec<u32> {
        // Metadata-free hot-path twin of apply_template; keep piece semantics in sync.
        // Only this exact template is identity; A may otherwise repeat or be absent.
        if matches!(
            self.single.as_slice(),
            [TemplatePiece::Sequence {
                id: SequenceId::A,
                ..
            }]
        ) {
            return encoded;
        }
        let mut result = Vec::with_capacity(encoded.len() + 4);
        for piece in &self.single {
            match piece {
                TemplatePiece::Sequence {
                    id: SequenceId::A, ..
                } => {
                    result.extend_from_slice(&encoded);
                }
                TemplatePiece::SpecialToken { id, .. } => {
                    if let Some(ids) = self.special_tokens.get(id) {
                        result.extend_from_slice(ids);
                    }
                }
                _ => {}
            }
        }
        result
    }
}

/// A supported token-ID post-processing step.
#[derive(Debug)]
pub enum PostProcessor {
    /// Identity byte-level post-processor.
    ByteLevel,
    /// A template that inserts special token IDs.
    TemplateProcessing(TemplateProcessing),
    /// Post-processors applied from left to right.
    Sequence(Vec<PostProcessor>),
}

impl PostProcessor {
    /// Builds a post-processor from parsed tokenizer JSON configuration.
    pub fn from_config(config: PostProcessorConfig) -> Result<Self, Error> {
        match config {
            PostProcessorConfig::ByteLevel { .. } => Ok(Self::ByteLevel),
            PostProcessorConfig::TemplateProcessing {
                single,
                pair,
                special_tokens,
            } => Ok(Self::TemplateProcessing(TemplateProcessing::from_config(
                single,
                pair,
                special_tokens,
            )?)),
            PostProcessorConfig::Sequence { processors } => {
                let steps = processors
                    .into_iter()
                    .map(Self::from_config)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Sequence(steps))
            }
        }
    }

    /// Applies this step to one ID sequence when special tokens are requested.
    pub fn post_process_single(&self, encoded: Vec<u32>, add_special_tokens: bool) -> Vec<u32> {
        if !add_special_tokens {
            return encoded;
        }
        match self {
            Self::ByteLevel => encoded,
            Self::TemplateProcessing(tp) => tp.apply_single(encoded),
            Self::Sequence(steps) => steps.iter().fold(encoded, |acc, step| {
                step.post_process_single(acc, add_special_tokens)
            }),
        }
    }

    /// Applies this step chain to a pair of encoded sequences.
    ///
    /// Hugging Face keeps the sequences separate until a template merges them,
    /// so a chain with no template falls back to plain concatenation with the
    /// second sequence typed `1`.
    pub fn post_process_pair(
        &self,
        first: Vec<u32>,
        second: Vec<u32>,
        add_special_tokens: bool,
    ) -> PostProcessed {
        match self.fold_pair(PairState::Split(first, second), add_special_tokens) {
            PairState::Split(first, second) => concat_pair(first, &second),
            PairState::Merged(processed) => processed,
        }
    }

    fn fold_pair(&self, state: PairState, add_special_tokens: bool) -> PairState {
        match self {
            Self::ByteLevel => state,
            Self::TemplateProcessing(tp) => PairState::Merged(match state {
                PairState::Split(first, second) => {
                    tp.apply_pair(&first, &second, add_special_tokens)
                }
                PairState::Merged(processed) => {
                    tp.apply_template(&tp.single, &processed.ids, None, add_special_tokens)
                }
            }),
            Self::Sequence(steps) => steps.iter().fold(state, |state, step| {
                step.fold_pair(state, add_special_tokens)
            }),
        }
    }

    /// Applies this step to one sequence, also computing aligned metadata.
    ///
    /// IDs always equal [`Self::post_process_single`]; when a degenerate
    /// template makes the metadata replay diverge from that ID-only path, the
    /// metadata falls back to zeros.
    pub fn post_process_single_meta(
        &self,
        encoded: Vec<u32>,
        add_special_tokens: bool,
    ) -> PostProcessed {
        // Only templates produce nonzero metadata, so other chains skip the replay.
        if !self.contains_template() {
            return PostProcessed::untyped(self.post_process_single(encoded, add_special_tokens));
        }
        let replay = self.fold_single(PostProcessed::untyped(encoded.clone()), add_special_tokens);
        let ids = self.post_process_single(encoded, add_special_tokens);
        if replay.ids == ids {
            replay
        } else {
            PostProcessed::untyped(ids)
        }
    }

    pub(crate) fn contains_template(&self) -> bool {
        match self {
            Self::ByteLevel => false,
            Self::TemplateProcessing(_) => true,
            Self::Sequence(steps) => steps.iter().any(Self::contains_template),
        }
    }

    fn fold_single(&self, state: PostProcessed, add_special_tokens: bool) -> PostProcessed {
        match self {
            Self::ByteLevel => state,
            Self::TemplateProcessing(tp) => {
                tp.apply_template(&tp.single, &state.ids, None, add_special_tokens)
            }
            Self::Sequence(steps) => steps.iter().fold(state, |state, step| {
                step.fold_single(state, add_special_tokens)
            }),
        }
    }
}

enum PairState {
    Split(Vec<u32>, Vec<u32>),
    Merged(PostProcessed),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tokenizer;
    use serde_json::{Value, json};

    #[test]
    fn template_processing_bos_only() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        };
        assert_eq!(tp.apply_single(vec![100, 200, 300]), vec![1, 100, 200, 300]);
    }

    #[test]
    fn template_processing_cls_sep() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "[CLS]".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "[SEP]".into(),
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([
                ("[CLS]".to_string(), vec![101]),
                ("[SEP]".to_string(), vec![102]),
            ]),
        };
        assert_eq!(tp.apply_single(vec![50, 60]), vec![101, 50, 60, 102]);
    }

    #[test]
    fn template_processing_empty_input() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        };
        assert_eq!(tp.apply_single(vec![]), vec![1]);
    }

    #[test]
    fn parse_from_json() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
        ]);
        let pair = serde_json::json!([]);
        let special_tokens = serde_json::json!({
            "<s>": {"id": "<s>", "ids": [1], "tokens": ["<s>"]}
        });
        let tp = TemplateProcessing::from_config(single, pair, special_tokens).unwrap();
        assert_eq!(tp.apply_single(vec![10, 20]), vec![1, 10, 20]);
    }

    #[test]
    fn post_process_single_respects_flag() {
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        });

        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![1, 10, 20]);
        assert_eq!(pp.post_process_single(vec![10, 20], false), vec![10, 20]);
    }

    #[test]
    fn template_processing_null_special_tokens_loads_ok() {
        let single = serde_json::json!([
            {"Sequence": {"id": "A", "type_id": 0}}
        ]);
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: Value::Null,
            special_tokens: Value::Null,
        })
        .unwrap();
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![10, 20]);
    }

    #[test]
    fn template_processing_special_token_def_without_tokens_field() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<bos>", "type_id": 0}},
            {"Sequence":     {"id": "A",    "type_id": 0}},
        ]);
        let special_tokens = serde_json::json!({
            "<bos>": {"id": "<bos>", "ids": [1]}
        });
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: Value::Null,
            special_tokens,
        })
        .unwrap();
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![1, 10, 20]);
    }

    #[test]
    fn template_processing_special_token_keyed_by_inner_id() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence":     {"id": "A",   "type_id": 0}},
        ]);
        let special_tokens = serde_json::json!({
            "bos_alias": {"id": "<s>", "ids": [1], "tokens": ["<s>"]}
        });
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: serde_json::json!([]),
            special_tokens,
        })
        .unwrap();

        assert_eq!(
            pp.post_process_single(vec![10, 20], true),
            vec![1, 10, 20],
            "BOS must be added even when outer JSON key differs from SpecialToken.id"
        );
    }

    #[test]
    fn template_processing_suffix_only() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("</s>".to_string(), vec![2])]),
        };
        assert_eq!(tp.apply_single(vec![10, 20]), vec![10, 20, 2]);
    }

    #[test]
    fn template_processing_bos_and_eos() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([
                ("<s>".to_string(), vec![1]),
                ("</s>".to_string(), vec![2]),
            ]),
        };
        assert_eq!(tp.apply_single(vec![10, 20, 30]), vec![1, 10, 20, 30, 2]);
        assert_eq!(tp.apply_single(vec![]), vec![1, 2]);
    }

    #[test]
    fn template_processing_multi_id_special_token() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<prefix>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("<prefix>".to_string(), vec![100, 101])]),
        };
        assert_eq!(tp.apply_single(vec![10, 20]), vec![100, 101, 10, 20]);
    }

    #[test]
    fn byte_level_post_processor_is_identity() {
        let pp = PostProcessor::ByteLevel;
        assert_eq!(pp.post_process_single(vec![1, 2, 3], true), vec![1, 2, 3]);
        assert_eq!(pp.post_process_single(vec![1, 2, 3], false), vec![1, 2, 3]);
        assert_eq!(
            pp.post_process_single(Vec::<u32>::new(), true),
            Vec::<u32>::new()
        );
    }

    #[test]
    fn num_special_tokens_matches_post_process_delta() {
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([
                ("<s>".to_string(), vec![1]),
                ("</s>".to_string(), vec![2]),
            ]),
        });
        let payload = vec![10u32, 20, 30];
        let with_special = pp.post_process_single(payload.clone(), true);
        let without_special = pp.post_process_single(payload.clone(), false);
        assert_eq!(with_special.len() - without_special.len(), 2);
    }

    #[test]
    fn sequence_post_processor_applies_all() {
        let pp_inner_a = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<a>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("<a>".to_string(), vec![99])]),
        });
        let pp_inner_b = PostProcessor::ByteLevel;
        let pp = PostProcessor::Sequence(vec![pp_inner_a, pp_inner_b]);
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![99, 10, 20]);
        assert_eq!(pp.post_process_single(vec![10, 20], false), vec![10, 20]);
    }

    #[test]
    fn identity_template_does_not_capture_repeated_or_absent_sequences() {
        for (single, expected) in [
            (
                vec![TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                }],
                vec![10, 20],
            ),
            (
                vec![
                    TemplatePiece::Sequence {
                        id: SequenceId::A,
                        type_id: 0
                    };
                    2
                ],
                vec![10, 20, 10, 20],
            ),
            (
                vec![TemplatePiece::Sequence {
                    id: SequenceId::B,
                    type_id: 0,
                }],
                vec![],
            ),
            (vec![], vec![]),
        ] {
            let template = TemplateProcessing {
                single,
                pair: vec![],
                special_tokens: HashMap::new(),
            };
            assert_eq!(template.apply_single(vec![10, 20]), expected);
        }
    }

    #[test]
    fn post_processing_applies_only_when_requested() {
        for processor in [
            Value::Null,
            json!({
                "type":"TemplateProcessing", "single":[
                    {"SpecialToken":{"id":"<s>","type_id":0}},
                    {"Sequence":{"id":"A","type_id":0}}
                ], "special_tokens":{"<s>":{"id":"<s>","ids":[1],"tokens":["<s>"]}}
            }),
        ] {
            let has_processor = !processor.is_null();
            let tokenizer = Tokenizer::from_json(json!({
                "model":{"type":"BPE","vocab":{"a":0,"<s>":1},"merges":[]},
                "post_processor":processor
            }))
            .unwrap();
            for input in ["", "a"] {
                let raw = tokenizer.encode(input, false).unwrap();
                assert_eq!(tokenizer.post_process(raw.clone(), false), raw);
                let mut expected = if has_processor { vec![1] } else { vec![] };
                expected.extend(&raw);
                assert_eq!(tokenizer.post_process(raw, true), expected);
                assert_eq!(tokenizer.encode(input, true).unwrap(), expected);
            }
        }
    }

    // Mirrors the Hugging Face reference template: <s>:0 $A:0 </s>:1 $B:1 </s>:1.
    fn pair_template() -> TemplateProcessing {
        TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
            ],
            pair: vec![
                TemplatePiece::SpecialToken {
                    id: "<s>".into(),
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 1,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::B,
                    type_id: 1,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 1,
                },
            ],
            special_tokens: HashMap::from([
                ("<s>".to_string(), vec![2]),
                ("</s>".to_string(), vec![3]),
            ]),
        }
    }

    #[test]
    fn pair_template_matches_hugging_face() {
        let pp = PostProcessor::TemplateProcessing(pair_template());
        let out = pp.post_process_pair(vec![0, 1], vec![1, 0], true);
        assert_eq!(out.ids, vec![2, 0, 1, 3, 1, 0, 3]);
        assert_eq!(out.type_ids, vec![0, 0, 0, 1, 1, 1, 1]);
        assert_eq!(out.special_tokens_mask, vec![1, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    fn pair_template_without_special_tokens_still_stamps_types() {
        let pp = PostProcessor::TemplateProcessing(pair_template());
        let out = pp.post_process_pair(vec![0, 1], vec![1, 0], false);
        assert_eq!(out.ids, vec![0, 1, 1, 0]);
        assert_eq!(out.type_ids, vec![0, 0, 1, 1]);
        assert_eq!(out.special_tokens_mask, vec![0, 0, 0, 0]);
    }

    #[test]
    fn pair_template_type_ids_override_default_pair_types() {
        // A pair template typing B as 0 must not fall back to the default 1.
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![TemplatePiece::Sequence {
                id: SequenceId::A,
                type_id: 0,
            }],
            pair: vec![
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::Sequence {
                    id: SequenceId::B,
                    type_id: 0,
                },
            ],
            special_tokens: HashMap::new(),
        });
        let out = pp.post_process_pair(vec![0, 1], vec![1, 0], true);
        assert_eq!(out.ids, vec![0, 1, 1, 0]);
        assert_eq!(out.type_ids, vec![0, 0, 0, 0]);
    }

    #[test]
    fn byte_level_pair_concatenates_with_default_types() {
        let out = PostProcessor::ByteLevel.post_process_pair(vec![0, 1], vec![1, 0], true);
        assert_eq!(out.ids, vec![0, 1, 1, 0]);
        assert_eq!(out.type_ids, vec![0, 0, 1, 1]);
        assert_eq!(out.special_tokens_mask, vec![0, 0, 0, 0]);
    }

    #[test]
    fn empty_pair_template_produces_empty_output() {
        // Hugging Face loads an empty pair template and encodes pairs to [].
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![TemplatePiece::Sequence {
                id: SequenceId::A,
                type_id: 0,
            }],
            pair: vec![],
            special_tokens: HashMap::new(),
        });
        let out = pp.post_process_pair(vec![0], vec![1], true);
        assert_eq!(out.ids, Vec::<u32>::new());
    }

    #[test]
    fn sequence_chain_lets_template_merge_pair() {
        let pp = PostProcessor::Sequence(vec![
            PostProcessor::ByteLevel,
            PostProcessor::TemplateProcessing(pair_template()),
        ]);
        let out = pp.post_process_pair(vec![0, 1], vec![1, 0], true);
        assert_eq!(out.ids, vec![2, 0, 1, 3, 1, 0, 3]);
        assert_eq!(out.type_ids, vec![0, 0, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn single_meta_stamps_template_type_ids_and_mask() {
        // Hugging Face stamps template type IDs on singles too: $A:0 </s>:1.
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 0,
                },
                TemplatePiece::SpecialToken {
                    id: "</s>".into(),
                    type_id: 1,
                },
            ],
            pair: vec![],
            special_tokens: HashMap::from([("</s>".to_string(), vec![3])]),
        });
        let with = pp.post_process_single_meta(vec![0, 1], true);
        assert_eq!(with.ids, vec![0, 1, 3]);
        assert_eq!(with.type_ids, vec![0, 0, 1]);
        assert_eq!(with.special_tokens_mask, vec![0, 0, 1]);
        let without = pp.post_process_single_meta(vec![0, 1], false);
        assert_eq!(without.ids, vec![0, 1]);
        assert_eq!(without.type_ids, vec![0, 0]);
        assert_eq!(without.special_tokens_mask, vec![0, 0]);
    }

    #[test]
    fn single_meta_type_stamp_applies_without_special_tokens() {
        // Hugging Face applies the template minus specials, so $A:1 still types 1.
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![TemplatePiece::Sequence {
                id: SequenceId::A,
                type_id: 1,
            }],
            pair: vec![],
            special_tokens: HashMap::new(),
        });
        let out = pp.post_process_single_meta(vec![0, 1], false);
        assert_eq!(out.ids, vec![0, 1]);
        assert_eq!(out.type_ids, vec![1, 1]);
    }

    #[test]
    fn single_meta_falls_back_when_replay_diverges() {
        // A repeated-A template duplicates IDs only in the metadata replay when
        // specials are off; IDs must follow the ID-only path with zero metadata.
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::Sequence {
                    id: SequenceId::A,
                    type_id: 1,
                };
                2
            ],
            pair: vec![],
            special_tokens: HashMap::new(),
        });
        let out = pp.post_process_single_meta(vec![7, 8], false);
        assert_eq!(out.ids, vec![7, 8]);
        assert_eq!(out.type_ids, vec![0, 0]);
    }

    #[test]
    fn tokenizer_encode_pair_matches_hugging_face_probe() {
        let tokenizer = Tokenizer::from_json(json!({
            "pre_tokenizer": {"type": "WhitespaceSplit"},
            "model": {"type":"BPE","vocab":{"a":0,"b":1,"<s>":2,"</s>":3},"merges":[]},
            "post_processor": {"type":"TemplateProcessing",
                "single":[{"SpecialToken":{"id":"<s>","type_id":0}},{"Sequence":{"id":"A","type_id":0}}],
                "pair":[{"SpecialToken":{"id":"<s>","type_id":0}},{"Sequence":{"id":"A","type_id":0}},
                        {"SpecialToken":{"id":"</s>","type_id":1}},{"Sequence":{"id":"B","type_id":1}},
                        {"SpecialToken":{"id":"</s>","type_id":1}}],
                "special_tokens":{"<s>":{"id":"<s>","ids":[2],"tokens":["<s>"]},
                                   "</s>":{"id":"</s>","ids":[3],"tokens":["</s>"]}}}
        }))
        .unwrap();
        let out = tokenizer.encode_pair("a b", "b a", true).unwrap();
        assert_eq!(out.ids, vec![2, 0, 1, 3, 1, 0, 3]);
        assert_eq!(out.type_ids, vec![0, 0, 0, 1, 1, 1, 1]);
        assert_eq!(out.special_tokens_mask, vec![1, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    fn tokenizer_without_post_processor_concatenates_pair() {
        let tokenizer = Tokenizer::from_json(json!({
            "pre_tokenizer": {"type": "WhitespaceSplit"},
            "model": {"type":"BPE","vocab":{"a":0,"b":1},"merges":[]}
        }))
        .unwrap();
        let out = tokenizer.encode_pair("a b", "b a", true).unwrap();
        assert_eq!(out.ids, vec![0, 1, 1, 0]);
        assert_eq!(out.type_ids, vec![0, 0, 1, 1]);
        assert_eq!(out.special_tokens_mask, vec![0, 0, 0, 0]);
    }
}
