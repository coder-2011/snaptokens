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
            DecoderConfig::Fuse => Ok(Self::Sequence(vec![])), // identity/no-op
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

/// Reuse a single owned piece, retaining exact preallocation for multiple pieces.
pub(crate) fn join_tokens(mut tokens: Vec<String>) -> String {
    if tokens.len() == 1 {
        tokens.pop().unwrap()
    } else {
        tokens.concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_support::load_tokenizer;

    #[test]
    fn decode_skip_special_tokens() {
        let model = "mistralai/Mistral-Nemo-Instruct-2407";
        let tok = load_tokenizer(model).unwrap();
        let text = "hello world";
        let ids_with = tok.encode(text, true).unwrap();
        let ids_without = tok.encode(text, false).unwrap();
        assert!(
            ids_with.len() > ids_without.len(),
            "expected BOS/EOS from {model}"
        );

        let skipped = tok.decode(&ids_with, true).unwrap();
        assert_eq!(skipped, text);

        let full = tok.decode(&ids_with, false).unwrap();
        assert_ne!(full, text);
        assert!(full.contains(text));
    }

    #[test]
    fn decode_tokens_matches_decode_by_id() {
        let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in &["Hello, world!", "The quick brown fox", "🌍 emoji"] {
            let ids = tok.encode(text, false).unwrap();
            let token_strings: Vec<String> = ids
                .iter()
                .map(|&id| tok.id_to_token(id).unwrap().to_string())
                .collect();
            let via_ids = tok.decode(&ids, false).unwrap();
            let via_tokens = tok.decode_tokens(token_strings).unwrap();
            assert_eq!(via_ids, via_tokens, "mismatch for {text:?}");
        }
    }

    #[test]
    fn encode_is_stable_after_decode() {
        let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in &["hello world", "日本語テスト", "fn foo() {}"] {
            let ids1 = tok.encode(text, false).unwrap();
            let decoded = tok.decode(&ids1, false).unwrap();
            let ids2 = tok.encode(&decoded, false).unwrap();
            assert_eq!(ids1, ids2, "encode not stable after decode for {text:?}");
        }
    }
}
