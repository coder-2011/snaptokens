use memchr::{memchr_iter, memmem};
use serde::Deserialize;
use serde_json::Value;

use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};

use super::{Error, scanner::PatternId};

#[derive(Clone, Debug, Deserialize)]
pub enum Pattern {
    String(std::string::String),
    Regex(std::string::String),
}

#[derive(Clone, Debug)]
enum Matcher {
    Literal(Box<str>),
    Fixed(PatternId),
    Regex(fancy_regex::Regex),
}

impl Matcher {
    fn from_pattern(pattern: Pattern) -> Result<Self, Error> {
        match pattern {
            Pattern::String(literal) => Ok(Self::Literal(literal.into_boxed_str())),
            Pattern::Regex(source) => {
                if let Some(pattern_id) = PatternId::from_source(&source) {
                    return Ok(Self::Fixed(pattern_id));
                }
                // Fall back to regex engine for unrecognized patterns.
                let regex = fancy_regex::Regex::new(&source)
                    .map_err(|e| Error::Unsupported(format!("Invalid regex `{source}`: {e}")))?;
                Ok(Self::Regex(regex))
            }
        }
    }

    fn for_each_match(&self, input: &str, mut emit: impl FnMut(usize, usize)) {
        match self {
            Self::Fixed(pattern) => pattern.for_each_match(input, emit),
            Self::Literal(literal) => {
                let needle = literal.as_bytes();
                if needle.is_empty() {
                    return;
                }
                if needle.len() == 1 {
                    for start in memchr_iter(needle[0], input.as_bytes()) {
                        emit(start, start + 1);
                    }
                    return;
                }
                for start in memmem::find_iter(input.as_bytes(), needle) {
                    emit(start, start + needle.len());
                }
            }
            Self::Regex(regex) => {
                for m in regex.find_iter(input).flatten() {
                    emit(m.start(), m.end());
                }
            }
        }
    }
}

/// How a pattern match participates in the resulting split sequence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum SplitBehavior {
    /// Drops matching text.
    Removed,
    /// Keeps each match as its own split.
    #[default]
    Isolated,
    /// Attaches each match to the preceding split.
    MergedWithPrevious,
    /// Attaches each match to the following split.
    MergedWithNext,
    /// Keeps matching and unmatched text in contiguous spans.
    Contiguous,
}

#[derive(Deserialize)]
struct SplitRaw {
    pattern: Pattern,
    #[serde(default)]
    behavior: SplitBehavior,
    #[serde(default)]
    invert: bool,
}

/// A compiled literal, fixed, or regular-expression pre-tokenizer splitter.
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "SplitRaw")]
pub struct Split {
    matcher: Matcher,
    behavior: SplitBehavior,
    invert: bool,
}

impl TryFrom<SplitRaw> for Split {
    type Error = Error;

    fn try_from(raw: SplitRaw) -> Result<Self, Error> {
        Self::from_parts(raw.pattern, raw.behavior, raw.invert)
    }
}

impl Split {
    fn from_parts(pattern: Pattern, behavior: SplitBehavior, invert: bool) -> Result<Self, Error> {
        Ok(Self {
            matcher: Matcher::from_pattern(pattern)?,
            behavior,
            invert,
        })
    }

    /// Builds a splitter from Hugging Face JSON pattern and behavior values.
    pub fn from_config(pattern: &Value, behavior: &str, invert: bool) -> Result<Self, Error> {
        let pattern = serde_json::from_value(pattern.clone())?;
        let behavior = serde_json::from_value(Value::String(behavior.to_string()))?;
        Self::from_parts(pattern, behavior, invert)
    }

    pub(crate) fn supports_ascii_nfc_fusion(&self) -> bool {
        self.behavior == SplitBehavior::Isolated
            && !self.invert
            && matches!(
                self.matcher,
                Matcher::Fixed(PatternId::Qwen | PatternId::QwenMark)
            )
    }

    pub(crate) fn supports_fused_stream(&self) -> bool {
        if !self.invert {
            // Kimi's exhaustive regex has no gaps for MergedWithPrevious to absorb.
            return self.behavior == SplitBehavior::Isolated
                || (self.behavior == SplitBehavior::MergedWithPrevious
                    && matches!(self.matcher, Matcher::Fixed(PatternId::Kimi)));
        }
        self.behavior == SplitBehavior::Removed
            && matches!(
                self.matcher,
                Matcher::Fixed(PatternId::Llama | PatternId::LlamaContraction)
            )
    }

    pub(crate) fn pattern_id(&self) -> Option<PatternId> {
        match &self.matcher {
            Matcher::Fixed(pattern) => Some(*pattern),
            Matcher::Literal(_) | Matcher::Regex(_) => None,
        }
    }

    pub(crate) fn for_each_fused_piece(
        &self,
        input: &str,
        mut emit: impl FnMut(&str, usize, usize),
    ) {
        debug_assert!(self.supports_fused_stream());
        let must_cover_input = self.behavior != SplitBehavior::Isolated || self.invert;
        let mut previous = 0;
        self.matcher.for_each_match(input, |start, end| {
            debug_assert!(!must_cover_input || start == previous);
            if start > previous {
                emit(input, previous, start);
            }
            emit(input, start, end);
            previous = end;
        });
        // Fused non-isolated behaviors are sound only for exhaustive fixed grammars.
        debug_assert!(!must_cover_input || previous == input.len());
        if previous < input.len() {
            emit(input, previous, input.len());
        }
    }

    /// Applies this splitter to every ordinary text split.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
        if self.behavior == SplitBehavior::Isolated && !self.invert {
            self.pre_tokenize_isolated(pts);
            return Ok(());
        }
        if self.behavior == SplitBehavior::Removed && self.invert {
            self.pre_tokenize_removed_inverted(pts);
            return Ok(());
        }

        let mut new_splits = Vec::with_capacity(pts.splits().len().saturating_mul(2));
        for split in pts.splits() {
            if split.token_id.is_some() {
                new_splits.push(split.clone());
                continue;
            }

            let text = pts.split_text(split);
            if text.is_empty() {
                continue;
            }

            let base = split.range.start;
            let Some(segments) = self.find_segments(text) else {
                // Removed+invert was handled above; all remaining behaviors keep
                // a nonempty unmatched span intact, without either intermediate Vec.
                new_splits.push(split.clone());
                continue;
            };
            let ranges = self.apply_behavior(&segments);
            for (start, end) in ranges {
                if start < end {
                    new_splits.push(PtSplit {
                        range: (base + start)..(base + end),
                        token_id: None,
                    });
                }
            }
        }

        pts.refine_splits(new_splits);
        Ok(())
    }

    pub(crate) fn append_ascii_splits(
        &self,
        input: &str,
        base: usize,
        output: &mut Vec<PtSplit>,
    ) -> bool {
        debug_assert!(self.supports_ascii_nfc_fusion());
        if input.is_empty() {
            return true;
        }

        let Matcher::Fixed(pattern) = &self.matcher else {
            unreachable!("eligible ASCII fusion requires a fixed scanner");
        };
        let mut previous = 0;
        let complete = pattern.for_each_ascii_match(input, |start, end| {
            if start > previous {
                output.push(PtSplit {
                    range: (base + previous)..(base + start),
                    token_id: None,
                });
            }
            output.push(PtSplit {
                range: (base + start)..(base + end),
                token_id: None,
            });
            previous = end;
        });
        if !complete {
            return false;
        }
        if previous < input.len() {
            output.push(PtSplit {
                range: (base + previous)..(base + input.len()),
                token_id: None,
            });
        }
        true
    }

    fn pre_tokenize_isolated(&self, pts: &mut PreTokenizedString) {
        // Real tokenizer pieces average four to five source bytes, so this
        // avoids repeated growth without reserving for the one-byte worst case.
        let capacity = pts.buffer().len().div_ceil(4).max(pts.splits().len());
        let mut new_splits = Vec::with_capacity(capacity);
        for split in pts.splits() {
            if split.token_id.is_some() {
                new_splits.push(split.clone());
                continue;
            }

            let text = pts.split_text(split);
            if text.is_empty() {
                continue;
            }
            let base = split.range.start;
            self.for_each_fused_piece(text, |_, start, end| {
                // Regex lookarounds can match an empty span; HF emits no empty piece.
                if start == end {
                    return;
                }
                new_splits.push(PtSplit {
                    range: (base + start)..(base + end),
                    token_id: None,
                });
            });
        }

        pts.refine_splits(new_splits);
    }

    fn pre_tokenize_removed_inverted(&self, pts: &mut PreTokenizedString) {
        // Inversion marks original matches as gaps, which Removed retains.
        let capacity = pts.buffer().len().div_ceil(4).max(pts.splits().len());
        let mut new_splits = Vec::with_capacity(capacity);
        for split in pts.splits() {
            if split.token_id.is_some() {
                new_splits.push(split.clone());
                continue;
            }

            let text = pts.split_text(split);
            let base = split.range.start;
            self.matcher.for_each_match(text, |start, end| {
                // Keep the boundary semantics without materializing an empty token.
                if start == end {
                    return;
                }
                new_splits.push(PtSplit {
                    range: (base + start)..(base + end),
                    token_id: None,
                });
            });
        }

        pts.refine_splits(new_splits);
    }

    fn find_segments(&self, input: &str) -> Option<Vec<(usize, usize, bool)>> {
        let mut segments = Vec::new();
        let mut previous = 0;
        self.matcher.for_each_match(input, |start, end| {
            if start > previous {
                segments.push((previous, start, false));
            }
            segments.push((start, end, true));
            previous = end;
        });
        // Test emitted segments rather than `previous`: even a zero-width match
        // must retain the ordinary delimiter behavior instead of taking this path.
        if segments.is_empty() {
            return None;
        }
        if previous < input.len() {
            segments.push((previous, input.len(), false));
        }

        if self.invert {
            for segment in &mut segments {
                segment.2 = !segment.2;
            }
        }
        Some(segments)
    }

    fn apply_behavior(&self, segments: &[(usize, usize, bool)]) -> Vec<(usize, usize)> {
        match self.behavior {
            SplitBehavior::Removed => segments
                .iter()
                .filter(|&&(_, _, is_match)| !is_match)
                .map(|&(start, end, _)| (start, end))
                .collect(),
            SplitBehavior::Isolated => segments
                .iter()
                .map(|&(start, end, _)| (start, end))
                .collect(),
            SplitBehavior::Contiguous => {
                let mut result: Vec<(usize, usize)> = Vec::new();
                let mut previous_match = None;
                for &(start, end, is_match) in segments {
                    if previous_match == Some(is_match) {
                        if let Some(last) = result.last_mut() {
                            last.1 = end;
                        }
                    } else {
                        result.push((start, end));
                    }
                    previous_match = Some(is_match);
                }
                result
            }
            SplitBehavior::MergedWithPrevious => {
                let mut result: Vec<(usize, usize)> = Vec::new();
                let mut previous_was_match = false;
                for &(start, end, is_match) in segments {
                    if is_match && !previous_was_match {
                        if let Some(last) = result.last_mut() {
                            last.1 = end;
                        } else {
                            result.push((start, end));
                        }
                    } else {
                        result.push((start, end));
                    }
                    previous_was_match = is_match;
                }
                result
            }
            SplitBehavior::MergedWithNext => {
                let mut result: Vec<(usize, usize)> = Vec::new();
                let mut previous_was_match = false;
                for &(start, end, is_match) in segments.iter().rev() {
                    if is_match && !previous_was_match {
                        if let Some(last) = result.last_mut() {
                            last.0 = start;
                        } else {
                            result.push((start, end));
                        }
                    } else {
                        result.push((start, end));
                    }
                    previous_was_match = is_match;
                }
                result.reverse();
                result
            }
        }
    }
}

#[cfg(test)]
mod tests;
