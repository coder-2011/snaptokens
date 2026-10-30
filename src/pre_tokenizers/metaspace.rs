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

            let base = buffer.len();
            if self.prepend_scheme == MetaspacePrependScheme::Always
                && !text.starts_with([' ', self.replacement])
            {
                buffer.push(self.replacement);
            }
            for character in text.chars() {
                buffer.push(if character == ' ' {
                    self.replacement
                } else {
                    character
                });
            }
            if buffer.len() > base {
                self.append_split_ranges(&buffer[base..], base, &mut splits);
            }
        }
        pts.set_buffer(buffer, splits);
    }

    pub(crate) fn pre_tokenize_after_whitespace(&self, pts: &mut PreTokenizedString) {
        let mut buffer = String::with_capacity(pts.buffer().len());
        let mut splits = Vec::with_capacity(pts.splits().len() * 2);
        for split in pts.splits() {
            let text = pts.split_text(split);
            if split.token_id.is_some() {
                Self::push_preserved_split(text, split.token_id, &mut buffer, &mut splits);
                continue;
            }

            let _ = super::WhitespaceSplit::for_each_word::<()>(text, |start, end| {
                self.append_whitespace_free(&text[start..end], &mut buffer, &mut splits);
                Ok(())
            });
        }
        pts.set_buffer(buffer, splits);
    }

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

    fn append_split_ranges(&self, text: &str, base: usize, splits: &mut Vec<PtSplit>) {
        let _ = self.for_each_marker_range::<()>(text, |start, end| {
            splits.push(PtSplit {
                range: base + start..base + end,
                token_id: None,
            });
            Ok(())
        });
    }

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

    pub(crate) fn for_each_word_piece<E>(
        &self,
        text: &str,
        scratch: &mut String,
        mut emit: impl FnMut(&str) -> Result<(), E>,
    ) -> Result<(), E> {
        super::WhitespaceSplit::for_each_word(text, |start, end| {
            self.emit_word_pieces(&text[start..end], scratch, &mut emit)
        })
    }

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

    pub(crate) fn decode_tokens(&self, tokens: &[&str]) -> String {
        let width = self.replacement.len_utf8();
        let remove_first = self.prepend_scheme != MetaspacePrependScheme::Never;
        let capacity = tokens
            .iter()
            .enumerate()
            .map(|(index, token)| {
                let removed = if index == 0 && remove_first {
                    width
                } else {
                    width - 1
                };
                if removed == 0 {
                    token.len()
                } else {
                    let markers = token.chars().filter(|&c| c == self.replacement).count();
                    token.len() - markers * removed
                }
            })
            .sum();
        let mut output = String::with_capacity(capacity);
        for (index, token) in tokens.iter().enumerate() {
            for character in token.chars() {
                if character != self.replacement {
                    output.push(character);
                } else if index != 0 || !remove_first {
                    output.push(' ');
                }
            }
        }
        output
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
    use tokenizers::Decoder as _;

    use super::*;

    #[test]
    fn borrowed_decode_matches_hugging_face_marker_boundaries() {
        for marker in [' ', '~', 'é', '▁', '🦀'] {
            for scheme in ["always", "never"] {
                let config = json!({"type": "Metaspace", "replacement": marker.to_string(),
                    "prepend_scheme": scheme, "split": true});
                let metaspace =
                    Metaspace::from_config(serde_json::from_value(config.clone()).unwrap())
                        .unwrap();
                let reference: tokenizers::pre_tokenizers::metaspace::Metaspace =
                    serde_json::from_value(config).unwrap();
                for tokens in [
                    vec![],
                    vec![String::new()],
                    vec![String::new(), String::new()],
                    vec![format!("{marker}{marker}{marker}")],
                    vec![String::new(), format!("{marker}a{marker}")],
                    vec![
                        format!("x{marker}y{marker}"),
                        format!("{marker}z"),
                        String::new(),
                    ],
                    vec![
                        "plain café 東京".into(),
                        format!("{marker}🦊{marker}é"),
                        marker.to_string(),
                    ],
                    vec![marker.to_string(), String::new(), marker.to_string()],
                ] {
                    let expected = reference.decode_chain(tokens.clone()).unwrap();
                    assert_eq!(metaspace.decode_chain(tokens.clone()), expected);
                    let borrowed: Vec<&str> = tokens.iter().map(String::as_str).collect();
                    assert_eq!(
                        metaspace.decode_tokens(&borrowed),
                        expected.concat(),
                        "marker={marker:?}, scheme={scheme}, tokens={tokens:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn borrowed_id_decode_preserves_filtering_and_added_overrides() {
        for marker in [' ', '~', 'é', '▁', '🦀'] {
            let vocabulary = [
                String::new(),
                "a".into(),
                marker.to_string(),
                format!("x{marker}y"),
            ];
            let bpe_vocab: serde_json::Map<String, serde_json::Value> = vocabulary
                .iter()
                .enumerate()
                .map(|(id, token)| (token.clone(), json!(id)))
                .collect();
            let unigram_vocab: Vec<_> = vocabulary
                .iter()
                .enumerate()
                .map(|(id, token)| (token, -(id as f64)))
                .collect();
            for scheme in ["always", "never"] {
                for model in [
                    json!({"type": "BPE", "vocab": bpe_vocab, "merges": []}),
                    json!({"type": "Unigram", "vocab": unigram_vocab,
                        "unk_id": 1, "byte_fallback": false}),
                ] {
                    let mut config = json!({
                        "version": "1.0", "model": model,
                        "added_tokens": [
                            {"id": 4, "content": format!("{marker}special"), "special": true,
                                "single_word": false, "lstrip": false, "rstrip": false, "normalized": false},
                            {"id": 5, "content": format!("{marker}added"), "special": false,
                                "single_word": false, "lstrip": false, "rstrip": false, "normalized": false}
                        ],
                        "decoder": {"type": "Metaspace", "replacement": marker.to_string(),
                            "prepend_scheme": scheme, "split": true}
                    });
                    let tokenizer = crate::Tokenizer::from_json(config.clone()).unwrap();
                    let reference =
                        tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config).unwrap())
                            .unwrap();
                    for ids in [
                        vec![],
                        vec![u32::MAX],
                        vec![0, 0],
                        vec![4, 4],
                        vec![u32::MAX, 2],
                        vec![0, 2],
                        vec![4, 2],
                        vec![2, 0, 2],
                        vec![u32::MAX, 4, 0, 3, 5, 6],
                    ] {
                        for skip in [false, true] {
                            assert_eq!(
                                tokenizer.decode(&ids, skip).unwrap(),
                                reference.decode(&ids, skip).unwrap(),
                                "marker={marker:?}, scheme={scheme}, ids={ids:?}, skip={skip}"
                            );
                        }
                    }

                    // HF reassigns configured overlaps; compare the preserved local owned route.
                    config["added_tokens"][1]["id"] = json!(1);
                    let overridden = crate::Tokenizer::from_json(config.clone()).unwrap();
                    config["decoder"] =
                        json!({"type": "Sequence", "decoders": [config["decoder"].clone()]});
                    let owned = crate::Tokenizer::from_json(config).unwrap();
                    for skip in [false, true] {
                        let expected = if scheme == "always" {
                            "added"
                        } else {
                            " added"
                        };
                        assert_eq!(overridden.decode(&[1], skip).unwrap(), expected);
                        for ids in [vec![1, 4, 2], vec![0, 1, 4, 2, u32::MAX]] {
                            assert_eq!(
                                overridden.decode(&ids, skip).unwrap(),
                                owned.decode(&ids, skip).unwrap()
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn word_piece_walker_matches_fused_split_pieces() {
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
