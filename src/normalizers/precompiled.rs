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
    // ASCII whitespace bytes whose normalized output still ends in whitespace,
    // making the following byte a safe parallel-partition anchor.
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

    /// Returns the per-byte parallel-partition anchor table when printable
    /// ASCII is identity; `None` marks this charsmap partition-unsafe.
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

    /// Copies known-identity ASCII runs and delegates only uncertain grapheme spans.
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

            // A preceding ASCII byte can be absorbed by a non-ASCII grapheme,
            // so include it and let the reference implementation decide.
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

/// Finds the next control, DEL, or non-ASCII byte without decoding printable runs.
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

#[cfg(test)]
mod tests {
    use super::Precompiled;
    use crate::json_structs::{NormalizerConfig, TokenizerJson};
    use crate::test_support::tokenizer_json_path;
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use std::fs;

    fn precompiled_charsmap(config: &NormalizerConfig) -> Option<&str> {
        match config {
            NormalizerConfig::Precompiled {
                precompiled_charsmap,
            } => Some(precompiled_charsmap),
            NormalizerConfig::Sequence { normalizers } => {
                normalizers.iter().find_map(precompiled_charsmap)
            }
            NormalizerConfig::Nfc | NormalizerConfig::Replace { .. } => None,
        }
    }

    #[test]
    fn t5_precompiled_normalizer_matches_its_reference_charsmap() {
        let path = tokenizer_json_path("google-t5/t5-small").unwrap();
        let tokenizer_json: TokenizerJson =
            serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        let charsmap = precompiled_charsmap(tokenizer_json.normalizer.as_ref().unwrap())
            .unwrap()
            .to_owned();
        let reference_bytes = STANDARD.decode(&charsmap).unwrap();
        let reference = spm_precompiled::Precompiled::from(&reference_bytes).unwrap();
        let optimized = Precompiled::from_config(charsmap).unwrap();
        let printable_ascii = (0x20u8..0x7f).map(char::from).collect::<String>();
        let all_ascii = (0..0x80u8).map(char::from).collect::<String>();
        let inputs = [
            String::new(),
            "plain ASCII text with punctuation?! 12345 ".repeat(64),
            printable_ascii,
            all_ascii,
            "line one\r\nline two\tthree\u{7f}four".to_owned(),
            "cafe\u{301} ﬁ ① ＡＢＣ".to_owned(),
            "ASCII-before-combining-a\u{301} and 東京 😀 after".to_owned(),
        ];

        for input in inputs {
            assert_eq!(
                optimized.normalize(&input),
                reference.normalize_string(&input),
                "precompiled normalizer mismatch for {input:?}"
            );
        }
    }
}
