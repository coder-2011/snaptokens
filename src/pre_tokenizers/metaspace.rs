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

    /// Appends transformed bytes once, then emits marker-delimited ranges over them.
    fn append_splits(&self, text: &str, buffer: &mut String, splits: &mut Vec<PtSplit>) {
        if text.is_empty() {
            return;
        }
        let base = buffer.len();
        buffer.push_str(text);
        let end = buffer.len();
        if !self.split {
            splits.push(PtSplit {
                range: base..end,
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
        splits.push(PtSplit {
            range: base + start..end,
            token_id: None,
        });
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
