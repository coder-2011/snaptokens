mod nfc;
mod precompiled;
mod replace;

use std::borrow::Cow;

pub use self::nfc::Nfc;
pub use self::precompiled::Precompiled;
pub use self::replace::Replace;
use crate::json_structs::NormalizerConfig;

/// An error while constructing or applying a normalizer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A normalizer configuration could not be deserialized.
    #[error("invalid config value: {0}")]
    Json(#[from] serde_json::Error),

    /// A regular-expression normalizer pattern was invalid.
    #[error("regex error: {0}")]
    Regex(#[from] fancy_regex::Error),

    /// A SentencePiece precompiled charsmap was malformed.
    #[error("invalid SentencePiece precompiled charsmap: {0}")]
    Precompiled(String),
}

/// A supported text-normalization step.
#[derive(Debug)]
pub enum Normalizer {
    /// Unicode NFC normalization.
    Nfc(Nfc),
    /// A literal or regular-expression replacement.
    Replace(Replace),
    /// SentencePiece's serialized character-rewrite normalizer.
    Precompiled(Precompiled),
    /// Normalization steps applied from left to right.
    Sequence(Vec<Normalizer>),
}

impl Normalizer {
    /// Builds a normalizer from parsed tokenizer JSON configuration.
    pub fn from_config(config: NormalizerConfig) -> Result<Self, Error> {
        match config {
            NormalizerConfig::Nfc => Ok(Self::Nfc(Nfc)),
            NormalizerConfig::Replace { pattern, content } => {
                Ok(Self::Replace(Replace::from_config(pattern, content)?))
            }
            NormalizerConfig::Precompiled {
                precompiled_charsmap,
            } => Ok(Self::Precompiled(Precompiled::from_config(
                precompiled_charsmap,
            )?)),
            NormalizerConfig::Sequence { normalizers } => {
                let steps = normalizers
                    .into_iter()
                    .map(Self::from_config)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Sequence(steps))
            }
        }
    }

    /// Returns the whitespace-anchor table proving raw parallel partitioning
    /// exact for this normalizer, or `None` when partitioning is unsafe.
    pub(crate) fn partition_anchor_table(&self) -> Option<&[bool; 128]> {
        match self {
            Self::Precompiled(precompiled) => precompiled.partition_anchor_table(),
            Self::Nfc(_) | Self::Replace(_) | Self::Sequence(_) => None,
        }
    }

    /// Normalizes text, borrowing the input when no change is needed.
    pub fn normalize<'a>(&self, input: &'a str) -> Cow<'a, str> {
        match self {
            Self::Nfc(nfc) => nfc.normalize(input),
            Self::Replace(replace) => replace.normalize(input),
            Self::Precompiled(precompiled) => precompiled.normalize(input),
            Self::Sequence(steps) => {
                let mut current = Cow::Borrowed(input);
                for step in steps {
                    // A borrowed result from this closed enum means unchanged,
                    // so keep the current owner instead of cloning it.
                    if let Cow::Owned(normalized) = step.normalize(current.as_ref()) {
                        current = Cow::Owned(normalized);
                    }
                }
                current
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_structs::NormalizerConfig;
    use crate::pre_tokenizers::{assert_encodings_match, tokenizer_config};
    use crate::{Tokenizer, TruncationDirection};
    use serde_json::{Value, json};

    #[test]
    fn normalizer_replacement_matches_fresh_construction() {
        let inputs = ["e\u{301}!", "é!", "[e\u{301}?]", "[é?]", "x", ""];
        for fused in [false, true] {
            let mut ours =
                Tokenizer::from_json(tokenizer_config(fused, true, Value::Null)).unwrap();
            for next in [
                json!({ "type": "NFC" }),
                json!({ "type": "Replace", "pattern": { "String": "!" }, "content": "?" }),
                Value::Null,
            ] {
                let normalizer: Option<NormalizerConfig> =
                    serde_json::from_value(next.clone()).unwrap();
                ours.set_normalizer(normalizer.map(Normalizer::from_config).transpose().unwrap())
                    .unwrap();
                let value = tokenizer_config(fused, true, next);
                let fresh = Tokenizer::from_json(value.clone()).unwrap();
                let reference =
                    tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&value).unwrap()).unwrap();
                assert_eq!(
                    ours.encode_batch(&inputs, false).unwrap(),
                    fresh.encode_batch(&inputs, false).unwrap()
                );
                assert_encodings_match(&ours, &reference, &inputs);
                assert_eq!(ours.encode("[e\u{301}?]", false).unwrap(), vec![256]);
            }
        }
    }

    #[test]
    fn failed_normalizer_replacement_preserves_state() {
        for fused in [false, true] {
            let mut ours =
                Tokenizer::from_json(tokenizer_config(fused, true, json!({ "type": "NFC" })))
                    .unwrap();
            let invalid = Normalizer::Replace(
                Replace::from_config(json!({ "String": "e\u{301}!" }), String::new()).unwrap(),
            );
            assert!(ours.set_normalizer(Some(invalid)).is_err());
            for input in ["é!", "e\u{301}!"] {
                assert_eq!(ours.encode(input, false).unwrap(), vec![257]);
            }
        }
    }

    #[test]
    fn limited_empty_input_bypasses_normalization() {
        let ours = Tokenizer::from_json(json!({
            "model": {"type": "BPE", "vocab": {"a": 0}, "merges": []},
            "normalizer": {"type": "Replace", "pattern": {"String": ""}, "content": "a"}
        }))
        .unwrap();
        assert_eq!(ours.encode("", false).unwrap(), Vec::<u32>::new());
        for direction in [TruncationDirection::Left, TruncationDirection::Right] {
            for limit in [0, 1] {
                assert_eq!(
                    ours.encode_with_limit("", limit, direction).unwrap(),
                    (Vec::new(), false)
                );
            }
        }
    }
}
