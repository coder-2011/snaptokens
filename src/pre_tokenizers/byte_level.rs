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

/// # Safety
/// `out` must have at least `src.len() * 2` bytes of remaining capacity.
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
    // SAFETY: BYTE_TO_UTF8 entries produce valid UTF-8.
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
            // SAFETY: the non-empty full string is one exact UTF-8 range.
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
mod tests;
