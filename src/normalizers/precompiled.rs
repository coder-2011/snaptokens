use std::borrow::Cow;

use base64::{Engine as _, engine::general_purpose::STANDARD};

use super::Error;

/// Applies the SentencePiece charsmap embedded in Hugging Face tokenizer JSON.
#[derive(Debug)]
pub struct Precompiled {
    charsmap: spm_precompiled::Precompiled,
    ascii_map: [Option<Box<str>>; 128],
    crlf: Option<Box<str>>,
    printable_ascii_identity: bool,
    ws_partition_anchor: [bool; 128],
}

impl Precompiled {
    /// Decodes and validates one base64-encoded SentencePiece charsmap.
    pub fn from_config(precompiled_charsmap: String) -> Result<Self, Error> {
        let bytes = STANDARD
            .decode(precompiled_charsmap)
            .map_err(|error| Error::Precompiled(error.to_string()))?;
        let charsmap = spm_precompiled::Precompiled::from(&bytes)
            .map_err(|error| Error::Precompiled(error.to_string()))?;
        let mut ascii_map: [Option<Box<str>>; 128] = std::array::from_fn(|_| None);
        let mut printable_ascii_identity = true;
        for byte in 0..128u8 {
            let mut buffer = [0; 4];
            let input = (byte as char).encode_utf8(&mut buffer);
            if let Some(normalized) = charsmap.transform(input)
                && normalized != input
            {
                if (0x20..0x7f).contains(&byte) {
                    printable_ascii_identity = false;
                }
                ascii_map[byte as usize] = Some(normalized.into());
            }
        }
        let crlf = charsmap.transform("\r\n").map(Into::into);
        let mut ws_partition_anchor = [false; 128];
        for byte in 0..128u8 {
            if !byte.is_ascii_whitespace() {
                continue;
            }
            ws_partition_anchor[byte as usize] = match &ascii_map[byte as usize] {
                None => true,
                Some(mapped) => mapped.chars().next_back().is_some_and(char::is_whitespace),
            };
        }
        Ok(Self {
            charsmap,
            ascii_map,
            crlf,
            printable_ascii_identity,
            ws_partition_anchor,
        })
    }

    pub(crate) fn partition_anchor_table(&self) -> Option<&[bool; 128]> {
        self.printable_ascii_identity
            .then_some(&self.ws_partition_anchor)
    }

    /// Returns a borrowed input when the charsmap makes no textual change.
    pub fn normalize<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if !self.printable_ascii_identity {
            let normalized = self.charsmap.normalize_string(input);
            return if normalized == input {
                Cow::Borrowed(input)
            } else {
                Cow::Owned(normalized)
            };
        }
        if input
            .as_bytes()
            .iter()
            .all(|byte| (0x20..0x7f).contains(byte))
        {
            return Cow::Borrowed(input);
        }

        let mut normalized = String::with_capacity(input.len());
        self.normalize_ascii_runs_into(input, &mut normalized);
        if normalized == input {
            Cow::Borrowed(input)
        } else {
            Cow::Owned(normalized)
        }
    }

    fn normalize_ascii_runs_into(&self, input: &str, out: &mut String) {
        let bytes = input.as_bytes();
        let mut start = 0;
        while start < bytes.len() {
            let attention = first_non_printable_ascii(bytes, start);
            if attention == bytes.len() {
                out.push_str(&input[start..]);
                return;
            }

            let byte = bytes[attention];
            if byte < 0x80 {
                out.push_str(&input[start..attention]);
                if byte == b'\r' && bytes.get(attention + 1) == Some(&b'\n') {
                    match &self.crlf {
                        Some(normalized) => out.push_str(normalized),
                        None => out.push_str("\r\n"),
                    }
                    start = attention + 2;
                } else {
                    match &self.ascii_map[byte as usize] {
                        Some(normalized) => out.push_str(normalized),
                        None => out.push(byte as char),
                    }
                    start = attention + 1;
                }
                continue;
            }

            // Include the preceding ASCII byte because the next Unicode grapheme may absorb it.
            let span_start = if attention > start {
                attention - 1
            } else {
                attention
            };
            out.push_str(&input[start..span_start]);
            let mut end = attention;
            while end < bytes.len() {
                let current = bytes[end];
                if current < 0x20 || current == 0x7f {
                    break;
                }
                if current < 0x80 && bytes.get(end + 1).is_none_or(|next| *next < 0x80) {
                    end += 1;
                    break;
                }
                end += 1;
            }
            out.push_str(&self.charsmap.normalize_string(&input[span_start..end]));
            start = end;
        }
    }
}

fn first_non_printable_ascii(bytes: &[u8], mut index: usize) -> usize {
    while index + 8 <= bytes.len() {
        let word = u64::from_ne_bytes(bytes[index..index + 8].try_into().unwrap());
        if word & 0x8080_8080_8080_8080 != 0 {
            break;
        }
        if word
            .to_ne_bytes()
            .iter()
            .any(|byte| *byte < 0x20 || *byte == 0x7f)
        {
            break;
        }
        index += 8;
    }
    while index < bytes.len() && (0x20..0x7f).contains(&bytes[index]) {
        index += 1;
    }
    index
}
