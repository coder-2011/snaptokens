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

// Per-ASCII-char result of the mapped-or-passthrough rule: mapped chars
// ('!'..='~') decode to themselves, everything else passes through.
const ASCII_TO_BYTE: [u8; 128] = build_ascii_to_byte();

const fn build_ascii_to_byte() -> [u8; 128] {
    let mut table = [0u8; 128];
    let mut c = 0usize;
    while c < 128 {
        let mapped = CHAR_TO_BYTE[c];
        table[c] = if BYTE_TO_CHAR[mapped as usize] as usize == c {
            mapped
        } else {
            c as u8
        };
        c += 1;
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
        vec![self.decode_tokens_fused(tokens.iter().map(String::as_str), capacity)]
    }

    /// Decodes borrowed token strings straight into one output string,
    /// emitting exactly the bytes `decode_chain` concatenates before its
    /// single UTF-8 assembly.
    pub fn decode_tokens_fused<'a>(
        &self,
        tokens: impl Iterator<Item = &'a str>,
        capacity_hint: usize,
    ) -> String {
        let mut bytes: Vec<u8> = Vec::with_capacity(capacity_hint);
        for token in tokens {
            let token_bytes = token.as_bytes();
            let mut i = 0;
            // Byte-driven walk over the alphabet's UTF-8 shape: ASCII chars
            // are one byte, mapped non-ASCII chars have C2–C5 leads, and
            // every other char passes through whole. Valid UTF-8 guarantees
            // the continuation byte behind a C2–C5 lead.
            while i < token_bytes.len() {
                let lead = token_bytes[i];
                if lead < 0x80 {
                    bytes.push(ASCII_TO_BYTE[lead as usize]);
                    i += 1;
                } else if (0xC2..=0xC5).contains(&lead) {
                    let cp = ((lead as usize & 0x1F) << 6) | (token_bytes[i + 1] as usize & 0x3F);
                    if cp < CHAR_TO_BYTE.len()
                        && BYTE_TO_CHAR[CHAR_TO_BYTE[cp] as usize] as usize == cp
                    {
                        bytes.push(CHAR_TO_BYTE[cp]);
                    } else {
                        bytes.push(lead);
                        bytes.push(token_bytes[i + 1]);
                    }
                    i += 2;
                } else {
                    let len = match lead {
                        0xF0.. => 4,
                        0xE0.. => 3,
                        _ => 2,
                    };
                    bytes.extend_from_slice(&token_bytes[i..i + len]);
                    i += len;
                }
            }
        }
        String::from_utf8(bytes)
            .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_walk_matches_per_char_mapping() {
        fn reference(tokens: &[&str]) -> Vec<u8> {
            let mut bytes = Vec::new();
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
                    let mut buf = [0u8; 4];
                    bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                }
            }
            bytes
        }

        let alphabet: String = (0u16..256).map(|b| BYTE_TO_CHAR[b as usize]).collect();
        let low_plane: String = (0u32..0x200).filter_map(char::from_u32).collect();
        let passthrough = "叫🦀\u{FFFD}▁ \u{7F}\u{A0}\u{143}\u{144}\u{17F}\u{180}";
        for tokens in [
            vec![alphabet.as_str()],
            vec![low_plane.as_str()],
            vec![passthrough],
            vec![alphabet.as_str(), passthrough, low_plane.as_str()],
        ] {
            let expected = reference(&tokens);
            let fused = ByteLevelDecoder.decode_tokens_fused(tokens.iter().copied(), 0);
            let via_lossy = String::from_utf8(expected.clone())
                .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());
            assert_eq!(fused, via_lossy, "tokens {tokens:?}");
            assert!(expected == fused.as_bytes() || via_lossy.as_bytes() == fused.as_bytes());
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
