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
