use serde::Deserialize;

use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};

use super::{
    Error, FusedPieceSink,
    scanner::{self, PatternId},
};

pub(crate) const BYTE_TO_CHAR: [char; 256] = build_byte_to_char();

const fn build_byte_to_char() -> [char; 256] {
    let mut table = ['\0'; 256];
    let mut next: u32 = 256;
    let mut i: u16 = 0;
    while i < 256 {
        let b = i as u8;
        let nice = (b >= b'!' && b <= b'~') || (b >= 0xA1 && b <= 0xAC) || b >= 0xAE;
        let cp = if nice {
            i as u32
        } else {
            let cp = next;
            next += 1;
            cp
        };
        table[i as usize] = match char::from_u32(cp) {
            Some(c) => c,
            None => panic!("invalid codepoint"),
        };
        i += 1;
    }
    table
}

const BYTE_TO_UTF8: [[u8; 2]; 256] = build_byte_to_utf8();

const BYTE_TO_UTF8_LEN: [u8; 256] = build_byte_to_utf8_len();

const fn build_byte_to_utf8() -> [[u8; 2]; 256] {
    let mut table = [[0u8; 2]; 256];
    let mut i: u16 = 0;
    while i < 256 {
        let cp = BYTE_TO_CHAR[i as usize] as u32;
        if cp < 0x80 {
            table[i as usize] = [cp as u8, 0];
        } else {
            table[i as usize] = [(0xC0 | (cp >> 6)) as u8, (0x80 | (cp & 0x3F)) as u8];
        }
        i += 1;
    }
    table
}

const fn build_byte_to_utf8_len() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i: u16 = 0;
    while i < 256 {
        let cp = BYTE_TO_CHAR[i as usize] as u32;
        table[i as usize] = if cp < 0x80 { 1 } else { 2 };
        i += 1;
    }
    table
}

// SAFETY: `out` must have at least `src.len() * 2` bytes of spare capacity.
unsafe fn encode_bytes_bulk(src: &[u8], out: &mut Vec<u8>) {
    let mut pos = out.len();
    let base = out.as_mut_ptr();
    for &b in src {
        let utf8 = BYTE_TO_UTF8[b as usize];
        let len = BYTE_TO_UTF8_LEN[b as usize] as usize;
        unsafe {
            std::ptr::copy_nonoverlapping(utf8.as_ptr(), base.add(pos), 2);
        }
        pos += len;
    }
    unsafe { out.set_len(pos) };
}

pub(crate) fn encode_bytes_into(s: &str, out: &mut String) {
    unsafe {
        let v = out.as_mut_vec();
        v.reserve(s.len() * 2);
        encode_bytes_bulk(s.as_bytes(), v);
    }
}

#[cfg(test)]
fn encode_bytes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    encode_bytes_into(s, &mut out);
    out
}

fn default_true() -> bool {
    true
}

/// GPT-2-compatible byte-level pre-tokenizer configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct ByteLevel {
    #[serde(default = "default_true")]
    use_regex: bool,
    #[serde(default = "default_true")]
    add_prefix_space: bool,
}

impl ByteLevel {
    /// Builds a byte-level pre-tokenizer from Hugging Face configuration flags.
    pub fn from_config(
        add_prefix_space: bool,
        _trim_offsets: bool,
        use_regex: bool,
    ) -> Result<Self, Error> {
        Ok(Self {
            use_regex,
            add_prefix_space,
        })
    }

    /// Returns whether this configuration transforms bytes without regex splitting.
    pub const fn is_bulk_only(&self) -> bool {
        !self.use_regex && !self.add_prefix_space
    }

    /// Returns whether byte-level work can be fused with model tokenization.
    pub const fn can_fuse(&self) -> bool {
        !self.add_prefix_space
    }

    pub(crate) fn pre_tokenize_fused(&self, pts: &mut PreTokenizedString) {
        if !self.use_regex {
            return;
        }

        let mut new_splits = Vec::with_capacity(pts.splits().len() * 4);
        for split in pts.splits() {
            if split.token_id.is_some() {
                new_splits.push(split.clone());
                continue;
            }

            let text = pts.split_text(split);
            let offset = split.range.start;
            PatternId::Gpt2.for_each_match(text, |start, end| {
                new_splits.push(PtSplit {
                    range: offset + start..offset + end,
                    token_id: None,
                });
            });
        }
        pts.refine_splits(new_splits);
    }

    pub(crate) fn stream_fused(&self, text: &str, sink: &mut impl FusedPieceSink) {
        if text.is_empty() {
            return;
        }
        if !self.use_regex {
            unsafe { sink.push_piece(text, 0, text.len()) };
            return;
        }
        scanner::stream_gpt2_matches(text, sink);
    }

    /// Applies byte remapping and optional GPT-2 pattern splitting.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
        if self.is_bulk_only() {
            Self::pre_tokenize_bulk(pts);
            return Ok(());
        }

        let old_buf = pts.buffer();
        let mut new_buf = String::with_capacity(old_buf.len().saturating_mul(2));
        let mut new_splits = Vec::with_capacity(pts.splits().len() * 4);

        for split in pts.splits() {
            let text = pts.split_text(split);

            if split.token_id.is_some() {
                let start = new_buf.len();
                encode_bytes_into(text, &mut new_buf);
                let end = new_buf.len();
                new_splits.push(PtSplit {
                    range: start..end,
                    token_id: split.token_id,
                });
                continue;
            }

            if text.is_empty() {
                continue;
            }

            let prefixed;
            let text = if self.add_prefix_space && !text.starts_with(' ') {
                prefixed = format!(" {text}");
                prefixed.as_str()
            } else {
                text
            };

            if self.use_regex {
                PatternId::Gpt2.for_each_match(text, |match_start, match_end| {
                    let start = new_buf.len();
                    encode_bytes_into(&text[match_start..match_end], &mut new_buf);
                    let end = new_buf.len();
                    new_splits.push(PtSplit {
                        range: start..end,
                        token_id: None,
                    });
                });
            } else {
                let start = new_buf.len();
                encode_bytes_into(text, &mut new_buf);
                let end = new_buf.len();
                if start < end {
                    new_splits.push(PtSplit {
                        range: start..end,
                        token_id: None,
                    });
                }
            }
        }

        pts.set_buffer(new_buf, new_splits);
        Ok(())
    }

    fn pre_tokenize_bulk(pts: &mut PreTokenizedString) {
        let old_buf = pts.buffer();
        let mut new_buf = String::with_capacity(old_buf.len() * 2);
        let mut new_splits = Vec::with_capacity(pts.splits().len());

        for split in pts.splits() {
            let text = pts.split_text(split);
            if text.is_empty() && split.token_id.is_none() {
                continue;
            }
            let start = new_buf.len();
            encode_bytes_into(text, &mut new_buf);
            let end = new_buf.len();
            new_splits.push(PtSplit {
                range: start..end,
                token_id: split.token_id,
            });
        }

        pts.set_buffer(new_buf, new_splits);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_alphabet_and_encoding_match_gpt2() {
        let alphabet: std::collections::HashSet<_> = BYTE_TO_CHAR.into_iter().collect();
        assert_eq!(alphabet.len(), 256);
        for byte in [b'!', b'A', b'z', b'~', 0xA1, 0xAC, 0xAE, 0xFF] {
            assert_eq!(BYTE_TO_CHAR[byte as usize], char::from(byte));
        }
        for (input, expected) in [("Hello", "Hello"), (" ", "Ġ"), ("\0\n", "ĀĊ"), ("€", "âĤ¬")]
        {
            assert_eq!(encode_bytes(input), expected, "{input:?}");
        }
    }

    #[test]
    fn prefix_and_regex_options_preserve_pieces() {
        for (prefix, regex, input, expected) in [
            (false, true, "Hello world", vec!["Hello", "Ġworld"]),
            (false, true, "I'm", vec!["I", "'m"]),
            (false, true, "'The", vec!["'", "The"]),
            (false, true, "price: $100", vec!["price", ":", "Ġ$", "100"]),
            (true, true, "Hello", vec!["ĠHello"]),
            (true, true, " Hello", vec!["ĠHello"]),
            (false, false, "Hello world", vec!["HelloĠworld"]),
            (false, true, "", vec![]),
            (true, true, "", vec![]),
            (false, true, "   ", vec!["ĠĠĠ"]),
            (false, true, "猫", vec!["çĮ«"]),
        ] {
            let byte_level = ByteLevel::from_config(prefix, true, regex).unwrap();
            let mut text = PreTokenizedString::from_text(input);
            byte_level.pre_tokenize(&mut text).unwrap();
            let actual: Vec<_> = text
                .splits()
                .iter()
                .map(|split| text.split_text(split))
                .collect();
            assert_eq!(
                actual, expected,
                "prefix={prefix} regex={regex} input={input:?}"
            );
        }
    }

    #[test]
    fn bulk_encoding_supports_overlapping_splits() {
        let bl = ByteLevel::from_config(false, true, false).unwrap();
        let split = PtSplit {
            range: 0..3,
            token_id: None,
        };
        let mut pts = PreTokenizedString::new("abc".into(), vec![split; 4]);

        bl.pre_tokenize(&mut pts).unwrap();

        assert_eq!(pts.buffer(), "abcabcabcabc");
        assert_eq!(
            pts.splits()
                .iter()
                .map(|split| split.range.clone())
                .collect::<Vec<_>>(),
            vec![0..3, 3..6, 6..9, 9..12]
        );
    }

    #[test]
    fn added_token_splits_preserved() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let buffer = "hello<sep>world".to_string();
        let splits = vec![
            PtSplit {
                range: 0..5,
                token_id: None,
            },
            PtSplit {
                range: 5..10,
                token_id: Some(42),
            },
            PtSplit {
                range: 10..15,
                token_id: None,
            },
        ];
        let mut pts = PreTokenizedString::new(buffer, splits);
        bl.pre_tokenize(&mut pts).unwrap();

        let added = pts
            .splits()
            .iter()
            .find(|s| s.token_id == Some(42))
            .expect("added token split missing");
        assert_eq!(pts.split_text(added), encode_bytes("<sep>"));
    }

    #[test]
    fn deserialization_preserves_defaults_and_overrides() {
        for (json, regex) in [("{}", true), (r#"{"use_regex":false}"#, false)] {
            let byte_level: ByteLevel = serde_json::from_str(json).unwrap();
            assert_eq!(byte_level.use_regex, regex);
            assert!(byte_level.add_prefix_space);
        }
    }
}
