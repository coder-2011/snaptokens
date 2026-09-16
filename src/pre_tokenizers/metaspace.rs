use crate::{
    json_structs::{MetaspaceConfig, MetaspacePrependScheme},
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
};

/// Rewrites SentencePiece spaces with one marker and optionally splits on it.
#[derive(Clone, Copy, Debug)]
pub struct Metaspace {
    replacement: char,
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
        Ok(Self {
            replacement,
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
                Self::push_preserved_split(text, split.token_id, &mut buffer, &mut splits);
                continue;
            }

            let mut rewritten = text.replace(' ', &self.replacement.to_string());
            if self.prepend_scheme == MetaspacePrependScheme::Always
                && !rewritten.starts_with(self.replacement)
            {
                rewritten.insert(0, self.replacement);
            }
            if !rewritten.is_empty() {
                let base = buffer.len();
                buffer.push_str(&rewritten);
                self.append_split_ranges(&rewritten, base, &mut splits);
            }
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
                Self::push_preserved_split(text, split.token_id, &mut buffer, &mut splits);
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

    /// Copies an added-token placeholder without rewriting its text.
    fn push_preserved_split(
        text: &str,
        token_id: Option<u32>,
        buffer: &mut String,
        splits: &mut Vec<PtSplit>,
    ) {
        let start = buffer.len();
        buffer.push_str(text);
        splits.push(PtSplit {
            range: start..buffer.len(),
            token_id,
        });
    }

    /// Adds split ranges for transformed text that has already been appended.
    fn append_split_ranges(&self, text: &str, base: usize, splits: &mut Vec<PtSplit>) {
        let _ = self.for_each_marker_range::<()>(text, |start, end| {
            splits.push(PtSplit {
                range: base + start..base + end,
                token_id: None,
            });
            Ok(())
        });
    }

    /// Emits each Metaspace piece range of already-rewritten text.
    fn for_each_marker_range<E>(
        &self,
        text: &str,
        mut emit: impl FnMut(usize, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        if text.is_empty() {
            return Ok(());
        }
        if !self.split {
            return emit(0, text.len());
        }
        let mut start = 0;
        for (offset, character) in text.char_indices() {
            if character == self.replacement && offset > start {
                emit(start, offset)?;
                start = offset;
            }
        }
        if start < text.len() {
            emit(start, text.len())?;
        }
        Ok(())
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
        let mut word_start = 0;
        for (offset, character) in text.char_indices() {
            if character.is_whitespace() {
                self.emit_word_pieces(&text[word_start..offset], scratch, &mut emit)?;
                word_start = offset + character.len_utf8();
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
        self.for_each_marker_range(piece, |start, end| emit(&piece[start..end]))
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
        // Unicode whitespace runs, marker-bearing words, a leading marker,
        // and boundary whitespace all reduce to the same piece sequence.
        let text = "  hello\tworld ▁already a▁b▁ café\u{3000}x\n\nend ";
        for config in [
            json!({"replacement": "▁", "add_prefix_space": true, "split": true}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": true}),
            json!({"replacement": "▁", "add_prefix_space": true, "split": false}),
            json!({"replacement": "▁", "prepend_scheme": "never", "split": false}),
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
