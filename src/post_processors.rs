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
    },
    /// Inserts IDs for a named configured special token.
    SpecialToken {
        /// Name of the special-token definition to insert.
        id: String,
    },
}

#[derive(Debug, Deserialize)]
struct SpecialTokenDef {
    id: String,
    ids: Vec<u32>,
}

/// A single-sequence template post-processor.
#[derive(Debug)]
pub struct TemplateProcessing {
    single: Vec<TemplatePiece>,
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
        if !pair.is_null() {
            serde_json::from_value::<Vec<TemplatePiece>>(pair)
                .map_err(|e| Error::InvalidConfig(format!("pair template: {e}")))?;
        }
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
            special_tokens,
        })
    }

    /// Applies the single-sequence template to encoded IDs.
    pub fn apply_single(&self, encoded: Vec<u32>) -> Vec<u32> {
        // Only this exact template is identity; A may otherwise repeat or be absent.
        if matches!(
            self.single.as_slice(),
            [TemplatePiece::Sequence { id: SequenceId::A }]
        ) {
            return encoded;
        }
        let mut result = Vec::with_capacity(encoded.len() + 4);
        for piece in &self.single {
            match piece {
                TemplatePiece::Sequence { id: SequenceId::A } => {
                    result.extend_from_slice(&encoded);
                }
                TemplatePiece::SpecialToken { id } => {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tokenizer;
    use crate::test_support::load_tokenizer;
    use serde_json::{Value, json};

    #[test]
    fn template_processing_bos_only() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        };
        assert_eq!(tp.apply_single(vec![100, 200, 300]), vec![1, 100, 200, 300]);
    }

    #[test]
    fn template_processing_cls_sep() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "[CLS]".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "[SEP]".into() },
            ],
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
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
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
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
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
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
            special_tokens: HashMap::from([("</s>".to_string(), vec![2])]),
        };
        assert_eq!(tp.apply_single(vec![10, 20]), vec![10, 20, 2]);
    }

    #[test]
    fn template_processing_bos_and_eos() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
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
                },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
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
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
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
                TemplatePiece::SpecialToken { id: "<a>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
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
                vec![TemplatePiece::Sequence { id: SequenceId::A }],
                vec![10, 20],
            ),
            (
                vec![TemplatePiece::Sequence { id: SequenceId::A }; 2],
                vec![10, 20, 10, 20],
            ),
            (vec![TemplatePiece::Sequence { id: SequenceId::B }], vec![]),
            (vec![], vec![]),
        ] {
            let template = TemplateProcessing {
                single,
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

    #[test]
    fn add_bos_token() {
        let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let bos_id = tok.token_to_id("<s>").expect("<s> not in vocabulary");

        let with_bos = tok.encode("hello world", true).unwrap();
        let without_bos = tok.encode("hello world", false).unwrap();

        assert_eq!(
            with_bos.first().copied(),
            Some(bos_id),
            "first token should be BOS when add_special_tokens=true"
        );
        assert_ne!(
            without_bos.first().copied(),
            Some(bos_id),
            "BOS should be absent when add_special_tokens=false"
        );
        assert_eq!(&with_bos[1..], without_bos.as_slice());

        let tok_q = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let with_flag = tok_q.encode("hello world", true).unwrap();
        let without_flag = tok_q.encode("hello world", false).unwrap();
        assert_eq!(
            with_flag, without_flag,
            "Qwen3 has no BOS post-processor — add_special_tokens should have no effect"
        );
    }

    #[test]
    fn batch_special_tokens_match_scalar_and_ragged() {
        let tokenizer = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let inputs = ["hello world", "", "second input"];
        let expected = inputs
            .iter()
            .map(|input| tokenizer.encode(input, true).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(tokenizer.encode_batch(&inputs, true).unwrap(), expected);

        let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, true).unwrap();
        assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
        assert_eq!(ids, expected.into_iter().flatten().collect::<Vec<_>>());
    }
}
