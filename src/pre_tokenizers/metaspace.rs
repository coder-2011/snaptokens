use crate::{
    json_structs::{MetaspaceConfig, MetaspacePrependScheme},
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
};

// Byte classes for the word walk: PLAIN never begins whitespace, ASCII_WS is a
// one-byte whitespace character, WS_LEAD may begin a multi-byte whitespace
// character. The complete White_Space repertoire outside ASCII is U+0085,
// U+00A0, U+1680, U+2000-200A, U+2028, U+2029, U+202F, U+205F, and U+3000,
// whose UTF-8 lead bytes are exactly C2, E1, E2, and E3.
const WS_PLAIN: u8 = 0;
const WS_ASCII: u8 = 1;
const WS_LEAD: u8 = 2;

const WS_CLASS: [u8; 256] = {
    let mut table = [WS_PLAIN; 256];
    table[0x09] = WS_ASCII;
    table[0x0A] = WS_ASCII;
    table[0x0B] = WS_ASCII;
    table[0x0C] = WS_ASCII;
    table[0x0D] = WS_ASCII;
    table[0x20] = WS_ASCII;
    table[0xC2] = WS_LEAD;
    table[0xE1] = WS_LEAD;
    table[0xE2] = WS_LEAD;
    table[0xE3] = WS_LEAD;
    table
};

/// Rewrites SentencePiece spaces with one marker and optionally splits on it.
#[derive(Clone, Copy, Debug)]
pub struct Metaspace {
    replacement: char,
    // The marker's UTF-8 bytes, precomputed for byte-comparison scans.
    replacement_utf8: [u8; 4],
    replacement_len: u8,
    prepend_scheme: MetaspacePrependScheme,
    split: bool,
}

impl Metaspace {
    /// Validates the JSON forms needed by SentencePiece-style tokenizer files.
    pub fn from_config(config: MetaspaceConfig) -> Result<Self, String> {
        let mut replacement = config.replacement.chars();
        let replacement = replacement
            .next()
            .filter(|_| replacement.next().is_none())
            .ok_or_else(|| "Metaspace replacement must be exactly one character".to_string())?;
        let prepend_scheme = config
            .prepend_scheme
            .unwrap_or(MetaspacePrependScheme::Always);
        if config.add_prefix_space == Some(false) && prepend_scheme != MetaspacePrependScheme::Never
        {
            return Err("Metaspace add_prefix_space conflicts with prepend_scheme".into());
        }
        if prepend_scheme == MetaspacePrependScheme::First {
            return Err("Metaspace prepend_scheme=first is not supported without offsets".into());
        }
        let mut replacement_utf8 = [0u8; 4];
        let replacement_len = replacement.encode_utf8(&mut replacement_utf8).len() as u8;
        Ok(Self {
            replacement,
            replacement_utf8,
            replacement_len,
            prepend_scheme,
            split: config.split.unwrap_or(true),
        })
    }

    /// Rewrites each ordinary split and preserves added-token placeholders verbatim.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) {
        let mut buffer = String::with_capacity(pts.buffer().len());
        let mut splits = Vec::with_capacity(pts.splits().len() * 2);
        for split in pts.splits() {
            let text = pts.split_text(split);
            if split.token_id.is_some() {
                let start = buffer.len();
                buffer.push_str(text);
                splits.push(PtSplit {
                    range: start..buffer.len(),
                    token_id: split.token_id,
                });
                continue;
            }

            let mut rewritten = text.replace(' ', &self.replacement.to_string());
            if self.prepend_scheme == MetaspacePrependScheme::Always
                && !rewritten.starts_with(self.replacement)
            {
                rewritten.insert(0, self.replacement);
            }
            self.append_splits(&rewritten, &mut buffer, &mut splits);
        }
        pts.set_buffer(buffer, splits);
    }

    /// Fuses a preceding WhitespaceSplit into this Metaspace transformation.
    pub(crate) fn pre_tokenize_after_whitespace(&self, pts: &mut PreTokenizedString) {
        let mut buffer = String::with_capacity(pts.buffer().len());
        let mut splits = Vec::with_capacity(pts.splits().len() * 2);
        for split in pts.splits() {
            let text = pts.split_text(split);
            if split.token_id.is_some() {
                let start = buffer.len();
                buffer.push_str(text);
                splits.push(PtSplit {
                    range: start..buffer.len(),
                    token_id: split.token_id,
                });
                continue;
            }

            let mut word_start = 0;
            for (offset, character) in text.char_indices() {
                if character.is_whitespace() {
                    self.append_whitespace_free(
                        &text[word_start..offset],
                        &mut buffer,
                        &mut splits,
                    );
                    word_start = offset + character.len_utf8();
                }
            }
            self.append_whitespace_free(&text[word_start..], &mut buffer, &mut splits);
        }
        pts.set_buffer(buffer, splits);
    }

    /// Emits the marker with the following text, matching Hugging Face Metaspace splitting.
    fn append_splits(&self, text: &str, buffer: &mut String, splits: &mut Vec<PtSplit>) {
        if text.is_empty() {
            return;
        }
        let base = buffer.len();
        buffer.push_str(text);
        self.append_split_ranges(text, base, splits);
    }

    /// Adds split ranges for transformed text that has already been appended.
    fn append_split_ranges(&self, text: &str, base: usize, splits: &mut Vec<PtSplit>) {
        if !self.split {
            splits.push(PtSplit {
                range: base..base + text.len(),
                token_id: None,
            });
            return;
        }

        let mut start = 0;
        for (offset, character) in text.char_indices() {
            if character == self.replacement && offset > start {
                splits.push(PtSplit {
                    range: base + start..base + offset,
                    token_id: None,
                });
                start = offset;
            }
        }
        if start < text.len() {
            splits.push(PtSplit {
                range: base + start..base + text.len(),
                token_id: None,
            });
        }
    }

    /// Emits every word piece of one text range exactly as the fused walker
    /// would split it, without materializing a rewritten buffer. Each
    /// whitespace-delimited word is marker-treated and piece-split using only
    /// the word itself, so any caller-chosen range that never divides a word
    /// reproduces the serial piece sequence.
    pub(crate) fn for_each_word_piece<E>(
        &self,
        text: &str,
        scratch: &mut String,
        mut emit: impl FnMut(&str) -> Result<(), E>,
    ) -> Result<(), E> {
        // Continuation bytes are 0x80-0xBF and never match an ASCII or lead
        // byte, so stepping bytewise through PLAIN bytes cannot misalign.
        let bytes = text.as_bytes();
        let mut word_start = 0;
        let mut index = 0;
        while index < bytes.len() {
            match WS_CLASS[bytes[index] as usize] {
                WS_ASCII => {
                    self.emit_word_pieces(&text[word_start..index], scratch, &mut emit)?;
                    index += 1;
                    word_start = index;
                }
                WS_LEAD => {
                    // Only a decoded character proves multi-byte whitespace.
                    let character = text[index..].chars().next().unwrap_or('\0');
                    let width = character.len_utf8();
                    if character.is_whitespace() {
                        self.emit_word_pieces(&text[word_start..index], scratch, &mut emit)?;
                        word_start = index + width;
                    }
                    index += width;
                }
                _ => index += 1,
            }
        }
        self.emit_word_pieces(&text[word_start..], scratch, &mut emit)
    }

    /// Applies the marker and interior-marker splitting to one word.
    fn emit_word_pieces<E>(
        &self,
        word: &str,
        scratch: &mut String,
        emit: &mut impl FnMut(&str) -> Result<(), E>,
    ) -> Result<(), E> {
        if word.is_empty() {
            return Ok(());
        }
        let piece: &str = if self.prepend_scheme == MetaspacePrependScheme::Always
            && !word.starts_with(self.replacement)
        {
            scratch.clear();
            scratch.push(self.replacement);
            scratch.push_str(word);
            scratch
        } else {
            word
        };
        if !self.split {
            return emit(piece);
        }
        // Every occurrence of the marker's UTF-8 bytes starts a character:
        // its first byte is an ASCII or lead byte, never a continuation.
        let marker = &self.replacement_utf8[..self.replacement_len as usize];
        let bytes = piece.as_bytes();
        let mut start = 0;
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == marker[0] && bytes[index..].starts_with(marker) {
                // A marker at the running start extends the current piece.
                if index > start {
                    emit(&piece[start..index])?;
                    start = index;
                }
                index += marker.len();
            } else {
                index += 1;
            }
        }
        if start < piece.len() {
            emit(&piece[start..])?;
        }
        Ok(())
    }

    /// Appends one whitespace-free word with the same marker treatment as Metaspace.
    fn append_whitespace_free(&self, text: &str, buffer: &mut String, splits: &mut Vec<PtSplit>) {
        if text.is_empty() {
            return;
        }
        let base = buffer.len();
        if self.prepend_scheme == MetaspacePrependScheme::Always
            && !text.starts_with(self.replacement)
        {
            buffer.push(self.replacement);
        }
        buffer.push_str(text);
        self.append_split_ranges(&buffer[base..], base, splits);
    }

    /// Decodes markers after model token strings have been assembled.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            .enumerate()
            .map(|(index, token)| {
                token
                    .chars()
                    .filter_map(|character| {
                        if character != self.replacement {
                            Some(character)
                        } else if index == 0 && self.prepend_scheme != MetaspacePrependScheme::Never
                        {
                            None
                        } else {
                            Some(' ')
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn word_piece_walker_matches_fused_split_pieces() {
        // Every multi-byte White_Space character class (C2/E1/E2/E3 leads),
        // non-whitespace characters sharing those leads, marker-bearing
        // words, consecutive markers, and boundary whitespace all reduce to
        // the same piece sequence as the fused walker.
        let text = "  hello\tworld ▁already a▁b▁ ▁▁x café\u{3000}x\n\nzero\u{200b}width \
                    nel\u{85}nbsp\u{a0}ogham\u{1680}fig\u{2007}sep\u{2028}nnbsp\u{202f}\
                    mmsp\u{205f}ell\u{2113}kana\u{3041}end ";
        for config in [
            json!({"replacement": "▁", "add_prefix_space": true, "split": true}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": true}),
            json!({"replacement": "▁", "add_prefix_space": true, "split": false}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": false}),
            json!({"replacement": "_", "add_prefix_space": true, "split": true}),
        ] {
            let metaspace =
                Metaspace::from_config(serde_json::from_value(config).unwrap()).unwrap();

            let mut fused = PreTokenizedString::new(
                text.to_owned(),
                vec![PtSplit {
                    range: 0..text.len(),
                    token_id: None,
                }],
            );
            metaspace.pre_tokenize_after_whitespace(&mut fused);
            let expected: Vec<String> = fused
                .splits()
                .iter()
                .map(|split| fused.split_text(split).to_owned())
                .collect();

            let mut walked = Vec::new();
            let mut scratch = String::new();
            metaspace
                .for_each_word_piece::<()>(text, &mut scratch, |piece| {
                    walked.push(piece.to_owned());
                    Ok(())
                })
                .unwrap();
            assert_eq!(walked, expected);
        }
    }

    #[test]
    fn whitespace_fusion_preserves_serial_buffer_ranges_and_added_ids() {
        let original = PreTokenizedString::new(
            "  hello\tworld<added>▁already  café\n".to_owned(),
            vec![
                PtSplit {
                    range: 0..13,
                    token_id: None,
                },
                PtSplit {
                    range: 13..20,
                    token_id: Some(99),
                },
                PtSplit {
                    range: 20..37,
                    token_id: None,
                },
            ],
        );
        for config in [
            json!({"replacement": "▁", "add_prefix_space": true, "split": true}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": true}),
            json!({"replacement": "▁", "add_prefix_space": true, "split": false}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": false}),
        ] {
            let metaspace =
                Metaspace::from_config(serde_json::from_value(config).unwrap()).unwrap();
            let mut serial = original.clone();
            crate::pre_tokenizers::WhitespaceSplit.pre_tokenize(&mut serial);
            metaspace.pre_tokenize(&mut serial);

            let mut fused = original.clone();
            metaspace.pre_tokenize_after_whitespace(&mut fused);
            assert_eq!(fused.buffer(), serial.buffer());
            assert_eq!(fused.splits(), serial.splits());
        }
    }
}
