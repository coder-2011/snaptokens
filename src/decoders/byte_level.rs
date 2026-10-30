use crate::pre_tokenizers::byte_level::BYTE_TO_CHAR;

const CHAR_TO_BYTE: [u8; 324] = build_char_to_byte();

const fn build_char_to_byte() -> [u8; 324] {
    let mut table = [0u8; 324];
    let mut i = 0u16;
    while i < 256 {
        let ch = BYTE_TO_CHAR[i as usize];
        table[ch as usize] = i as u8;
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
        let mut bytes: Vec<u8> = Vec::with_capacity(capacity);
        for token in tokens {
            let token_start = bytes.len();
            for c in token.chars() {
                let cp = c as usize;
                if cp < CHAR_TO_BYTE.len() {
                    let byte = CHAR_TO_BYTE[cp];
                    if BYTE_TO_CHAR[byte as usize] == c {
                        bytes.push(byte);
                        continue;
                    }
                }

                bytes.truncate(token_start);
                bytes.extend_from_slice(token.as_bytes());
                break;
            }
        }
        vec![
            String::from_utf8(bytes)
                .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn mixed_mapped_unmapped_tokens_match_hugging_face() {
        let reference = tokenizers::pre_tokenizers::byte_level::ByteLevel::default();
        for tokens in [
            vec!["Ġ", "叫"],
            vec!["Ġ叫"],
            vec!["叫Ġa"],
            vec!["Ġa\u{00ad}"],
            vec!["Hello", "Ġ叫", "Ġworld"],
        ] {
            let tokens: Vec<String> = tokens.into_iter().map(str::to_owned).collect();
            assert_eq!(
                ByteLevelDecoder.decode_chain(tokens.clone()),
                reference.decode_chain(tokens.clone()).unwrap(),
                "{tokens:?}"
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
