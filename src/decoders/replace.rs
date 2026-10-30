use std::borrow::Cow;

use serde_json::Value;

use super::Error;
use crate::normalizers::Replace;

/// Decoder that applies a configured literal or regular-expression replacement.
#[derive(Clone, Debug)]
pub struct ReplaceDecoder {
    inner: Replace,
}

impl ReplaceDecoder {
    /// Builds a replacement decoder from a JSON pattern and replacement text.
    pub fn from_config(pattern: Value, content: String) -> Result<Self, Error> {
        Ok(Self {
            inner: Replace::from_config(pattern, content)?,
        })
    }

    /// Applies the replacement independently to every token string.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            .map(|token| {
                if token.is_empty() {
                    return token;
                }
                match self.inner.normalize(&token) {
                    Cow::Borrowed(_) => token,
                    Cow::Owned(replaced) => replaced,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn empty_tokens_match_hugging_face() {
        for pattern in [
            json!({"String": ""}),
            json!({"Regex": ""}),
            json!({"Regex": "^"}),
            json!({"Regex": "$"}),
            json!({"Regex": "a*"}),
        ] {
            let replace = json!({"type": "Replace", "pattern": pattern, "content": "-"});
            for fuse in [false, true] {
                let config = if fuse {
                    json!({"type": "Sequence", "decoders": [{"type": "Fuse"}, replace]})
                } else {
                    replace.clone()
                };
                let reference: tokenizers::decoders::DecoderWrapper =
                    serde_json::from_value(config.clone()).unwrap();
                let decoder =
                    crate::decoders::Decoder::from_config(serde_json::from_value(config).unwrap())
                        .unwrap();
                for tokens in [
                    vec![],
                    vec![""],
                    vec!["", ""],
                    vec!["ab"],
                    vec!["", "ab", ""],
                    vec!["é", "", "a"],
                ] {
                    let tokens: Vec<String> = tokens.into_iter().map(str::to_owned).collect();
                    assert_eq!(
                        decoder.decode_chain(tokens.clone()).unwrap(),
                        reference.decode_chain(tokens.clone()).unwrap(),
                        "pattern={pattern}, fuse={fuse}, tokens={tokens:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn literal_replace_decoder() {
        let dec = ReplaceDecoder::from_config(json!("▁"), " ".to_string()).unwrap();
        let out = dec.decode_chain(vec!["▁Hello".to_string(), "▁world".to_string()]);
        assert_eq!(out, vec![" Hello", " world"]);
    }

    #[test]
    fn regex_replace_decoder() {
        let dec = ReplaceDecoder::from_config(json!({"Regex": "[0-9]+"}), "#".to_string()).unwrap();
        let out = dec.decode_chain(vec!["a12b".to_string(), "34".to_string()]);
        assert_eq!(out, vec!["a#b", "#"]);
    }

    #[test]
    fn literal_empty_pattern_is_not_noop() {
        let dec = ReplaceDecoder::from_config(json!(""), "-".to_string()).unwrap();
        let out = dec.decode_chain(vec!["ab".to_string()]);
        assert_eq!(out, vec!["-a-b-"]);
    }

    #[test]
    fn regex_replacement_content_is_literal() {
        let dec =
            ReplaceDecoder::from_config(json!({"Regex": "([a-z]+)"}), "$1".to_string()).unwrap();
        let out = dec.decode_chain(vec!["abc".to_string()]);
        assert_eq!(out, vec!["$1"]);
    }
}
