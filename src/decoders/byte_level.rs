use crate::pre_tokenizers::byte_level::BYTE_TO_CHAR;

const NON_ASCII_TO_BYTE: [u16; 256] = build_non_ascii_to_byte();

const fn build_non_ascii_to_byte() -> [u16; 256] {
    // A wider entry keeps byte zero distinct from unmapped codepoints.
    let mut table = [u16::MAX; 256];
    let mut i = 0u16;
    while i < 256 {
        let cp = BYTE_TO_CHAR[i as usize] as usize;
        if cp >= 128 {
            table[cp - 128] = i;
        }
        i += 1;
    }
    table
}

/// Decoder for the GPT-2 byte-level Unicode representation.
#[derive(Debug)]
pub struct ByteLevelDecoder;

impl ByteLevelDecoder {
    /// Apply byte-level decoding to a list of token strings.
    ///
    /// Maps every token character back to its byte value using the reverse
    /// GPT-2 table, preserving the entire token if any character is unmapped,
    /// then interprets the combined bytes as UTF-8.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        let capacity = tokens.iter().map(String::len).sum();
        vec![self.decode_tokens(tokens, capacity)]
    }

    pub(crate) fn decode_tokens<I>(&self, tokens: I, capacity: usize) -> String
    where
        I: IntoIterator,
        I::Item: AsRef<[u8]>,
    {
        let mut tokens = tokens.into_iter();
        let Some(first) = tokens.next() else {
            return String::new();
        };
        let mut bytes: Vec<u8> = Vec::with_capacity(capacity);
        for token in std::iter::once(first).chain(tokens) {
            let token = token.as_ref();
            let token_start = bytes.len();
            let mut offset = 0;
            while offset < token.len() {
                let lead = token[offset];
                if (b'!'..=b'~').contains(&lead) {
                    bytes.push(lead);
                    offset += 1;
                    continue;
                }
                if (0xc2..=0xc5).contains(&lead)
                    && let Some(&next) = token.get(offset + 1)
                    && (0x80..=0xbf).contains(&next)
                {
                    let index = usize::from(lead - 0xc2) * 64 + usize::from(next - 0x80);
                    let byte = NON_ASCII_TO_BYTE[index];
                    if byte != u16::MAX {
                        bytes.push(byte as u8);
                        offset += 2;
                        continue;
                    }
                }

                bytes.truncate(token_start);
                bytes.extend_from_slice(token);
                break;
            }
        }
        if bytes.is_empty() {
            return String::new();
        }
        String::from_utf8(bytes)
            .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn literal_scalars_and_mixed_tokens_match_hugging_face() {
        let reference = tokenizers::pre_tokenizers::byte_level::ByteLevel::default();
        for scalar in 0..=0x200 {
            let character = char::from_u32(scalar).unwrap();
            for token in [
                character.to_string(),
                format!("Ġ{character}"),
                format!("{character}Ġ"),
                format!("Ġ{character}Ġ"),
            ] {
                let tokens = vec![token];
                let expected = reference.decode_chain(tokens.clone()).unwrap();
                assert_eq!(
                    ByteLevelDecoder.decode_chain(tokens.clone()),
                    expected,
                    "{tokens:?}"
                );
                assert_eq!(
                    ByteLevelDecoder.decode_tokens(tokens.iter().map(String::as_str), 0),
                    expected[0],
                    "{tokens:?}"
                );
            }
        }
    }

    #[test]
    fn mixed_mapped_unmapped_tokens_match_hugging_face() {
        let reference = tokenizers::pre_tokenizers::byte_level::ByteLevel::default();
        for tokens in [
            vec![],
            vec!["", ""],
            vec!["Ġ", "叫"],
            vec!["Ġ叫"],
            vec!["叫Ġa"],
            vec!["Ġa\u{00ad}"],
            vec!["Hello", "Ġ叫", "Ġworld"],
        ] {
            let tokens: Vec<String> = tokens.into_iter().map(str::to_owned).collect();
            let expected = reference.decode_chain(tokens.clone()).unwrap();
            assert_eq!(
                ByteLevelDecoder.decode_chain(tokens.clone()),
                expected,
                "{tokens:?}"
            );
            for capacity in [0, 1, tokens.iter().map(String::len).sum()] {
                assert_eq!(
                    ByteLevelDecoder.decode_tokens(tokens.iter().map(String::as_str), capacity),
                    expected[0],
                    "{tokens:?}"
                );
            }
        }
    }

    #[test]
    fn borrowed_byte_sequences_match_hugging_face() {
        let reference = tokenizers::pre_tokenizers::byte_level::ByteLevel::default();
        for bytes in [
            (0..=255).collect::<Vec<u8>>(),
            b"\xe2\x82\xac".to_vec(),
            b"\xf0\x90\x80x".to_vec(),
            b"\xff\xfe".to_vec(),
            b"a\xc3".to_vec(),
            b"\xed\xa0\x80".to_vec(),
            b"\xc0\xaf".to_vec(),
            vec![b'a'; 16_384],
        ] {
            for split in [false, true] {
                let tokens: Vec<String> = if split {
                    bytes
                        .iter()
                        .map(|&byte| BYTE_TO_CHAR[byte as usize].to_string())
                        .collect()
                } else {
                    vec![
                        bytes
                            .iter()
                            .map(|&byte| BYTE_TO_CHAR[byte as usize])
                            .collect(),
                    ]
                };
                let expected = reference.decode_chain(tokens.clone()).unwrap();
                assert_eq!(ByteLevelDecoder.decode_chain(tokens.clone()), expected);
                for capacity in [0, 1, tokens.iter().map(String::len).sum()] {
                    assert_eq!(
                        ByteLevelDecoder.decode_tokens(tokens.iter().map(String::as_str), capacity),
                        expected[0]
                    );
                }
            }
        }
    }

    #[test]
    fn borrowed_id_decode_preserves_filtering_and_added_tokens() {
        let config = json!({
            "version": "1.0",
            "added_tokens": [
                {"id": 4, "content": "<special>", "special": true, "single_word": false,
                    "lstrip": false, "rstrip": false, "normalized": false},
                {"id": 5, "content": "Ġadded", "special": false, "single_word": false,
                    "lstrip": false, "rstrip": false, "normalized": false}
            ],
            "decoder": {"type": "ByteLevel", "add_prefix_space": false,
                "trim_offsets": false, "use_regex": false}
        });
        for model in [
            json!({"type": "BPE", "vocab": {"a": 0, "Ġ": 1, "": 2, "Ġ叫": 3}, "merges": []}),
            json!({"type": "Unigram", "vocab": [["a", 0.0], ["Ġ", -1.0], ["", -2.0],
                ["Ġ叫", -3.0]], "unk_id": 0, "byte_fallback": false}),
        ] {
            let mut config = config.clone();
            config["model"] = model;
            let tokenizer = crate::Tokenizer::from_json(config.clone()).unwrap();
            let reference =
                tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config).unwrap()).unwrap();
            for ids in [
                vec![],
                vec![u32::MAX],
                vec![2, 2],
                vec![4, 4],
                vec![0, u32::MAX, 1, 2, 3, 4, 5, 6],
            ] {
                for skip in [false, true] {
                    assert_eq!(
                        tokenizer.decode(&ids, skip).unwrap(),
                        reference.decode(&ids, skip).unwrap()
                    );
                }
            }

            let mut empty_added = config.clone();
            empty_added["added_tokens"][1]["content"] = json!("");
            assert!(crate::Tokenizer::from_json(empty_added).is_err());

            // HF reassigns these configured IDs, so preserve the local text-lookup contract.
            config["added_tokens"][1]["id"] = json!(0);
            let mut sparse = config["added_tokens"][1].clone();
            sparse["id"] = json!(65_535);
            sparse["content"] = json!("Ġsparse");
            config["added_tokens"].as_array_mut().unwrap().push(sparse);
            let overridden = crate::Tokenizer::from_json(config.clone()).unwrap();
            config["decoder"] = json!({"type": "Sequence", "decoders": [{"type": "ByteLevel"}]});
            let owned = crate::Tokenizer::from_json(config).unwrap();
            for skip in [false, true] {
                assert_eq!(overridden.id_to_token(0), Some("Ġadded"));
                assert_eq!(overridden.id_to_token(65_535), Some("Ġsparse"));
                assert_eq!(overridden.decode(&[0], skip).unwrap(), " added");
                assert_eq!(
                    overridden
                        .decode(&[65_534, 65_535, 65_536, u32::MAX], skip)
                        .unwrap(),
                    " sparse"
                );
                let ids = [0, 4, 2, 65_534, 65_535, 65_536, u32::MAX];
                assert_eq!(
                    overridden.decode(&ids, skip).unwrap(),
                    owned.decode(&ids, skip).unwrap()
                );
            }
        }
    }

    #[test]
    fn raw_token_bytes_preserve_whole_token_fallback() {
        for token in [
            b"\xc4".as_slice(),
            b"\xc4A",
            b"\xc4\xa0\xc4",
            b"\xc4\xa0\xc4A",
            b"\xc4\xa0\xed\xa0\x80",
        ] {
            let expected = [b"before".as_slice(), token, b" after"].concat();
            assert_eq!(
                ByteLevelDecoder
                    .decode_tokens([b"before".as_slice(), token, "Ġafter".as_bytes()], 0),
                String::from_utf8_lossy(&expected)
            );
        }
    }

    #[test]
    fn roundtrip_ascii() {
        let dec = ByteLevelDecoder;
        let result = dec.decode_chain(vec!["Hello".to_string()]);
        assert_eq!(result, vec!["Hello"]);
    }

    #[test]
    fn roundtrip_space() {
        let dec = ByteLevelDecoder;
        let result = dec.decode_chain(vec!["\u{120}Hello".to_string()]);
        assert_eq!(result, vec![" Hello"]);
    }

    #[test]
    fn roundtrip_multibyte() {
        let dec = ByteLevelDecoder;
        let encoded: String = [0xE2u8, 0x82, 0xAC]
            .iter()
            .map(|&b| BYTE_TO_CHAR[b as usize])
            .collect();
        let result = dec.decode_chain(vec![encoded]);
        assert_eq!(result, vec!["€"]);
    }

    #[test]
    fn non_gpt2_chars_preserved() {
        let dec = ByteLevelDecoder;
        let result = dec.decode_chain(vec![
            "\u{00AD}".to_string(),
            "<\u{FF5C}begin\u{2581}of\u{2581}sentence\u{FF5C}>".to_string(),
        ]);
        assert_eq!(result, vec!["\u{00AD}<｜begin▁of▁sentence｜>"]);
    }

    #[test]
    fn malformed_bytes_keep_lossy_utf8_replacement_boundaries() {
        for bytes in [
            b"\xf0\x90\x80x".as_slice(),
            b"\xff\xfe",
            b"a\xc3",
            b"\xed\xa0\x80",
        ] {
            let tokens = bytes
                .iter()
                .map(|&byte| BYTE_TO_CHAR[byte as usize].to_string())
                .collect();
            assert_eq!(
                ByteLevelDecoder.decode_chain(tokens),
                [String::from_utf8_lossy(bytes)]
            );
        }
    }
}
