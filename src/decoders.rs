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

    /// A decoder configuration uses a form Snaptokens does not support.
    #[error("unsupported decoder configuration: {0}")]
    Unsupported(String),
}

impl From<crate::normalizers::Error> for Error {
    fn from(e: crate::normalizers::Error) -> Self {
        match e {
            crate::normalizers::Error::Json(j) => Self::Json(j),
            crate::normalizers::Error::Regex(r) => Self::Regex(r),
            crate::normalizers::Error::Precompiled(error) => Self::Unsupported(error),
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
    /// Concatenates all token strings into one piece.
    Fuse,
    /// Replaces literal or regular-expression matches.
    Replace(ReplaceDecoder),
    /// Reverses SentencePiece's Metaspace marker.
    Metaspace(crate::pre_tokenizers::Metaspace),
    /// Applies decoder steps from left to right.
    Sequence(Vec<Decoder>),
}

impl Decoder {
    /// Builds a decoder from a parsed tokenizer configuration.
    pub fn from_config(config: DecoderConfig) -> Result<Self, Error> {
        match config {
            DecoderConfig::ByteFallback => Ok(Self::ByteFallback(ByteFallbackDecoder)),
            DecoderConfig::ByteLevel => Ok(Self::ByteLevel(ByteLevelDecoder)),
            DecoderConfig::Replace { pattern, content } => Ok(Self::Replace(
                ReplaceDecoder::from_config(pattern, content)?,
            )),
            DecoderConfig::Fuse => Ok(Self::Fuse),
            DecoderConfig::Metaspace(config) => Ok(Self::Metaspace(
                crate::pre_tokenizers::Metaspace::from_config(config)
                    .map_err(Error::Unsupported)?,
            )),
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
            Self::Fuse => Ok(vec![join_tokens(tokens)]),
            Self::Replace(repl) => Ok(repl.decode_chain(tokens)),
            Self::Metaspace(metaspace) => Ok(metaspace.decode_chain(tokens)),
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

pub(crate) fn join_tokens(mut tokens: Vec<String>) -> String {
    if tokens.len() == 1 {
        tokens.pop().unwrap()
    } else {
        tokens.concat()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn fuse_preserves_hugging_face_stage_boundaries() {
        for tokens in [vec!["a", "b"], vec!["", "ab"]] {
            let config = json!({
                "type": "Sequence",
                "decoders": [
                    {"type": "Fuse"},
                    {"type": "Replace", "pattern": {"String": "ab"}, "content": "x"}
                ]
            });
            let reference: tokenizers::decoders::DecoderWrapper =
                serde_json::from_value(config.clone()).unwrap();
            let decoder = Decoder::from_config(serde_json::from_value(config).unwrap()).unwrap();
            let tokens: Vec<String> = tokens.into_iter().map(str::to_owned).collect();
            assert_eq!(
                decoder.decode_chain(tokens.clone()).unwrap(),
                reference.decode_chain(tokens).unwrap()
            );
        }
    }

    #[test]
    fn fuse_and_empty_sequence_preserve_hugging_face_piece_boundaries() {
        for config in [
            json!({"type": "Fuse"}),
            json!({"type": "Sequence", "decoders": []}),
        ] {
            let reference: tokenizers::decoders::DecoderWrapper =
                serde_json::from_value(config.clone()).unwrap();
            let decoder = Decoder::from_config(serde_json::from_value(config).unwrap()).unwrap();
            for tokens in [vec!["a", "b"], vec![], vec![""]] {
                let tokens: Vec<String> = tokens.into_iter().map(str::to_owned).collect();
                assert_eq!(
                    decoder.decode_chain(tokens.clone()).unwrap(),
                    reference.decode_chain(tokens).unwrap()
                );
            }
        }
    }
}
