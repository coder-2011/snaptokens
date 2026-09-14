use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};

/// Splits ordinary text on Unicode whitespace and removes the whitespace itself.
#[derive(Clone, Copy, Debug)]
pub struct WhitespaceSplit;

impl WhitespaceSplit {
    /// Refines each ordinary split while leaving added-token placeholders intact.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) {
        let mut splits = Vec::with_capacity(pts.splits().len());
        for split in pts.splits() {
            if split.token_id.is_some() {
                splits.push(split.clone());
                continue;
            }

            let text = pts.split_text(split);
            let mut start = 0;
            for (offset, character) in text.char_indices() {
                if character.is_whitespace() {
                    if start < offset {
                        splits.push(PtSplit {
                            range: split.range.start + start..split.range.start + offset,
                            token_id: None,
                        });
                    }
                    start = offset + character.len_utf8();
                }
            }
            if start < text.len() {
                splits.push(PtSplit {
                    range: split.range.start + start..split.range.end,
                    token_id: None,
                });
            }
        }
        pts.refine_splits(splits);
    }
}
