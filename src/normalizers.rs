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
