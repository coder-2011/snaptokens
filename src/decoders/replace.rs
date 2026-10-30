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
            .map(|token| match self.decode_token(&token) {
                Cow::Borrowed(_) => token,
                Cow::Owned(replaced) => replaced,
            })
            .collect()
    }

    pub(crate) fn decode_token<'a>(&self, token: &'a str) -> Cow<'a, str> {
        if token.is_empty() {
            Cow::Borrowed(token)
        } else {
            self.inner.normalize(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn first_replace_preserves_pipeline_boundaries() {
        for (pattern, content) in [
            (json!({"String": "|"}), ""),
            (json!({"String": "X"}), "<0x+a>"),
            (json!({"String": ""}), "-"),
            (json!({"Regex": "^"}), "$1"),
        ] {
            let replace = json!({"type": "Replace", "pattern": pattern, "content": content});
            let fallback = json!({"type": "ByteFallback"});
            let fuse = json!({"type": "Fuse"});
            for decoder in [
                replace.clone(),
                json!({"type": "Sequence", "decoders": []}),
                json!({"type": "Sequence", "decoders": [replace]}),
                json!({"type": "Sequence", "decoders": [replace, fallback, fuse]}),
                json!({"type": "Sequence", "decoders": [replace, fuse,
                    {"type": "Replace", "pattern": {"String": "ab"}, "content": "x"}]}),
                json!({"type": "Sequence", "decoders": [replace,
                    {"type": "Sequence", "decoders": [fallback, fuse]}]}),
                json!({"type": "Sequence", "decoders": [replace,
                    {"type": "ByteLevel", "add_prefix_space": false,
                        "trim_offsets": false, "use_regex": false}]}),
                json!({"type": "Sequence", "decoders": [replace,
                    {"type": "Metaspace", "replacement": "▁",
                        "prepend_scheme": "always", "split": true}]}),
                json!({"type": "Sequence", "decoders": [fuse, replace]}),
                json!({"type": "Sequence", "decoders": [
                    {"type": "Sequence", "decoders": [replace]}, fallback, fuse]}),
                json!({"type": "Sequence", "decoders": [fallback, replace, fuse]}),
            ] {
                let config = json!({
                    "version": "1.0",
                    "model": {"type": "BPE", "merges": [], "vocab": {
                        "": 0, "a": 1, "b": 2, "|": 3, "X": 4, "<0x61>": 5,
                        "<0xff>": 6, "<0xe2>": 7, "<0x82>": 8, "<0xac>": 9,
                        "Ġ": 10, "叫": 11, "▁a": 12
                    }},
                    "added_tokens": [
                        {"id": 13, "content": "<special>", "special": true,
                            "single_word": false, "lstrip": false, "rstrip": false,
                            "normalized": false},
                        {"id": 14, "content": "|added", "special": false,
                            "single_word": false, "lstrip": false, "rstrip": false,
                            "normalized": false}
                    ],
                    "decoder": decoder
                });
                let tokenizer = crate::Tokenizer::from_json(config.clone()).unwrap();
                let reference =
                    tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config).unwrap())
                        .unwrap();
                let reference_decoder: tokenizers::decoders::DecoderWrapper =
                    serde_json::from_value(decoder.clone()).unwrap();
                for ids in [
                    vec![],
                    vec![0],
                    vec![0, 12],
                    vec![1, 2],
                    vec![5, 6],
                    vec![5, 0, 6],
                    vec![7, 8, 9],
                    vec![7, 13, 8, 9],
                    vec![4, 0, 4],
                    vec![1, 3, 4, 14, u32::MAX, 12, 10, 11],
                    vec![u32::MAX, 13],
                ] {
                    for skip in [false, true] {
                        let expected = reference.decode(&ids, skip).unwrap();
                        assert_eq!(tokenizer.decode(&ids, skip).unwrap(), expected);
                        let tokens: Vec<String> = ids
                            .iter()
                            .copied()
                            .filter(|&id| !skip || !tokenizer.is_special_token(id))
                            .filter_map(|id| tokenizer.id_to_token(id).map(str::to_owned))
                            .collect();
                        assert_eq!(tokenizer.decode_tokens(tokens.clone()).unwrap(), expected);
                        assert_eq!(
                            tokenizer
                                .decoder()
                                .unwrap()
                                .decode_chain(tokens.clone())
                                .unwrap(),
                            reference_decoder.decode_chain(tokens).unwrap()
                        );
                    }
                }
            }
        }
    }

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
