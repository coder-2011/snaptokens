use crate::pre_tokenizers::byte_level::BYTE_TO_CHAR;

/// Reverse mapping: Unicode char → original byte value.
///
/// All chars in `BYTE_TO_CHAR` are in the range U+0000..U+0143 (max codepoint
/// 323), so a flat 324-element array gives O(1) lookup.
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
    /// GPT-2 table, then interprets the combined bytes as UTF-8.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        let capacity = tokens.iter().map(String::len).sum();
        let mut bytes: Vec<u8> = Vec::with_capacity(capacity);
        for token in tokens {
            for c in token.chars() {
                let cp = c as usize;
                if cp < CHAR_TO_BYTE.len() {
                    let byte = CHAR_TO_BYTE[cp];
                    if BYTE_TO_CHAR[byte as usize] == c {
                        bytes.push(byte);
                        continue;
                    }
                }

                // Characters absent from the GPT-2 table must retain their
                // original UTF-8, even when their codepoint is below U+0144.
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(s.as_bytes());
            }
        }
        vec![String::from_utf8_lossy(&bytes).into_owned()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
