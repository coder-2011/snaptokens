mod byte_fallback;
mod byte_level;
mod replace;

use crate::json_structs::DecoderConfig;

pub use self::byte_fallback::ByteFallbackDecoder;
pub use self::byte_level::ByteLevelDecoder;
pub use self::replace::ReplaceDecoder;

/// An error while constructing or applying a decoder.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A decoder configuration could not be deserialized.
    #[error("invalid config value: {0}")]
    Json(#[from] serde_json::Error),

    /// A regular-expression decoder pattern was invalid.
    #[error("regex error: {0}")]
    Regex(#[from] fancy_regex::Error),
}

impl From<crate::normalizers::Error> for Error {
    fn from(e: crate::normalizers::Error) -> Self {
        match e {
            crate::normalizers::Error::Json(j) => Self::Json(j),
            crate::normalizers::Error::Regex(r) => Self::Regex(r),
        }
    }
}

/// A supported Hugging Face decoder pipeline step.
#[derive(Debug)]
pub enum Decoder {
    /// Decodes `<0xNN>` byte fallback tokens.
    ByteFallback(ByteFallbackDecoder),
    /// Reverses the GPT-2 byte-to-Unicode mapping.
    ByteLevel(ByteLevelDecoder),
    /// Replaces literal or regular-expression matches.
    Replace(ReplaceDecoder),
    /// Applies decoder steps from left to right.
    Sequence(Vec<Decoder>),
}

impl Decoder {
    /// Builds a decoder from a parsed tokenizer configuration.
    pub fn from_config(config: DecoderConfig) -> Result<Self, Error> {
        match config {
            DecoderConfig::ByteFallback => Ok(Self::ByteFallback(ByteFallbackDecoder)),
            DecoderConfig::ByteLevel(_) => Ok(Self::ByteLevel(ByteLevelDecoder)),
            DecoderConfig::Replace { pattern, content } => Ok(Self::Replace(
                ReplaceDecoder::from_config(pattern, content)?,
            )),
            DecoderConfig::Fuse => Ok(Self::Sequence(vec![])), // identity/no-op
            DecoderConfig::Sequence { decoders } => {
                let steps = decoders
                    .into_iter()
                    .map(Self::from_config)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Sequence(steps))
            }
        }
    }

    /// Apply this decoder step to a list of token strings, returning the
    /// transformed list.
    ///
    /// Follows HuggingFace's `decode_chain` semantics: each decoder step
    /// transforms the token list, and the final result is joined.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Result<Vec<String>, Error> {
        match self {
            Self::ByteFallback(bf) => Ok(bf.decode_chain(tokens)),
            Self::ByteLevel(bl) => Ok(bl.decode_chain(tokens)),
            Self::Replace(repl) => Ok(repl.decode_chain(tokens)),
            Self::Sequence(steps) => {
                let mut current = tokens;
                for step in steps {
                    current = step.decode_chain(current)?;
                }
                Ok(current)
            }
        }
    }

    /// Decodes token strings and joins the resulting pieces.
    pub fn decode(&self, tokens: Vec<String>) -> Result<String, Error> {
        let result = self.decode_chain(tokens)?;
        Ok(join_tokens(result))
    }
}

/// Reuse a single owned piece, retaining exact preallocation for multiple pieces.
pub(crate) fn join_tokens(mut tokens: Vec<String>) -> String {
    if tokens.len() == 1 {
        tokens.pop().unwrap()
    } else {
        tokens.concat()
    }
}
