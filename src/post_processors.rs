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
mod tests;
