use memchr::{memchr_iter, memmem};
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use serde_json::Value;

use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};

use super::{Error, scanner::PatternId};

/// A literal or regular-expression splitting pattern.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Pattern {
    /// Match literal text.
    String(std::string::String),
    /// Match a regular expression.
    Regex(std::string::String),
}

#[derive(Clone, Debug)]
enum Matcher {
    Literal(Box<str>),
    Fixed(PatternId, Box<str>),
    Regex(fancy_regex::Regex),
}

impl Serialize for Matcher {
    /// Preserve the original pattern even when execution uses a fixed scanner.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Literal(text) => {
                serializer.serialize_newtype_variant("Pattern", 0, "String", text)
            }
            Self::Fixed(_, source) => {
                serializer.serialize_newtype_variant("Pattern", 1, "Regex", source)
            }
            Self::Regex(regex) => {
                serializer.serialize_newtype_variant("Pattern", 1, "Regex", regex.as_str())
            }
        }
    }
}

impl Matcher {
    /// Compile a pattern while retaining its spelling for serialization.
    fn from_pattern(pattern: Pattern) -> Result<Self, Error> {
        match pattern {
            Pattern::String(literal) => Ok(Self::Literal(literal.into_boxed_str())),
            Pattern::Regex(source) => {
                if let Some(pattern_id) = PatternId::from_source(&source) {
                    return Ok(Self::Fixed(pattern_id, source.into_boxed_str()));
                }
                // Fall back to regex engine for unrecognized patterns.
                let regex = fancy_regex::Regex::new(&source)
                    .map_err(|e| Error::Unsupported(format!("Invalid regex `{source}`: {e}")))?;
                Ok(Self::Regex(regex))
            }
        }
    }

    /// Emits matches while preserving regex execution failures.
    fn for_each_match(&self, input: &str, mut emit: impl FnMut(usize, usize)) -> Result<(), Error> {
        match self {
            Self::Fixed(pattern, _) => pattern.for_each_match(input, emit),
            Self::Literal(literal) => {
                let needle = literal.as_bytes();
                if needle.is_empty() {
                    return Ok(());
                }
                if needle.len() == 1 {
                    for start in memchr_iter(needle[0], input.as_bytes()) {
                        emit(start, start + 1);
                    }
                    return Ok(());
                }
                for start in memmem::find_iter(input.as_bytes(), needle) {
                    emit(start, start + needle.len());
                }
            }
            Self::Regex(regex) => {
                for result in regex.find_iter(input) {
                    let matched = result?;
                    emit(matched.start(), matched.end());
                }
            }
        }
        Ok(())
    }
}

/// How a pattern match participates in the resulting split sequence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
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
    #[serde(flatten)]
    extra: serde_json::Map<String, Value>,
}

/// A compiled literal, fixed, or regular-expression pre-tokenizer splitter.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "SplitRaw")]
pub struct Split {
    #[serde(rename = "pattern")]
    matcher: Matcher,
    behavior: SplitBehavior,
    invert: bool,
    #[serde(flatten)]
    extra: serde_json::Map<String, Value>,
}

impl TryFrom<SplitRaw> for Split {
    type Error = Error;

    /// Compile the matcher and retain extension fields for later saves.
    fn try_from(raw: SplitRaw) -> Result<Self, Error> {
        let mut split = Self::from_parts(raw.pattern, raw.behavior, raw.invert)?;
        split.extra = raw.extra;
        Ok(split)
    }
}

impl Split {
    fn from_parts(pattern: Pattern, behavior: SplitBehavior, invert: bool) -> Result<Self, Error> {
        Ok(Self {
            matcher: Matcher::from_pattern(pattern)?,
            behavior,
            invert,
            extra: Default::default(),
        })
    }

    /// Builds a splitter from Hugging Face JSON pattern and behavior values.
    pub fn from_config(pattern: &Value, behavior: &str, invert: bool) -> Result<Self, Error> {
        let pattern = Pattern::deserialize(pattern)?;
        let behavior =
            SplitBehavior::deserialize(behavior.into_deserializer()).map_err(Error::Json)?;
        Self::from_parts(pattern, behavior, invert)
    }

    pub(crate) fn supports_fused_stream(&self) -> bool {
        if !self.invert {
            // Kimi's exhaustive regex has no gaps for MergedWithPrevious to absorb.
            return self.behavior == SplitBehavior::Isolated
                || (self.behavior == SplitBehavior::MergedWithPrevious
                    && matches!(self.matcher, Matcher::Fixed(PatternId::Kimi, _)));
        }
        self.behavior == SplitBehavior::Removed
            && matches!(
                self.matcher,
                Matcher::Fixed(PatternId::Llama | PatternId::LlamaContraction, _)
            )
    }

    pub(crate) fn pattern_id(&self) -> Option<PatternId> {
        match &self.matcher {
            Matcher::Fixed(pattern, _) => Some(*pattern),
            Matcher::Literal(_) | Matcher::Regex(_) => None,
        }
    }

    /// Emits fused ranges without turning a failed regex scan into unmatched text.
    pub(crate) fn for_each_fused_piece(
        &self,
        input: &str,
        mut emit: impl FnMut(&str, usize, usize),
    ) -> Result<(), Error> {
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
        })?;
        // Fused non-isolated behaviors are sound only for exhaustive fixed grammars.
        debug_assert!(!must_cover_input || previous == input.len());
        if previous < input.len() {
            emit(input, previous, input.len());
        }
        Ok(())
    }

    /// Applies this splitter to every ordinary text split.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
        if self.behavior == SplitBehavior::Isolated && !self.invert {
            return self.pre_tokenize_isolated(pts);
        }
        if self.behavior == SplitBehavior::Removed && self.invert {
            return self.pre_tokenize_removed_inverted(pts);
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
            let segments = self.find_segments(text)?;
            if segments.is_empty() {
                // Removed+invert was handled above; all remaining behaviors keep
                // a nonempty unmatched span intact, without either intermediate Vec.
                new_splits.push(split.clone());
                continue;
            }
            self.apply_behavior(&segments, |start, end| {
                if start < end {
                    new_splits.push(PtSplit {
                        range: (base + start)..(base + end),
                        token_id: None,
                    });
                }
            });
        }

        pts.refine_splits(new_splits);
        Ok(())
    }

    /// Refines isolated ranges only after every regex scan succeeds.
    fn pre_tokenize_isolated(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
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
            })?;
        }

        pts.refine_splits(new_splits);
        Ok(())
    }

    /// Retains matched ranges without committing partial results on regex failure.
    fn pre_tokenize_removed_inverted(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
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
            })?;
        }

        pts.refine_splits(new_splits);
        Ok(())
    }

    /// Collects match and gap ranges, distinguishing no matches from scan failure.
    fn find_segments(&self, input: &str) -> Result<Vec<(usize, usize, bool)>, Error> {
        let mut segments = Vec::new();
        let mut previous = 0;
        self.matcher.for_each_match(input, |start, end| {
            if start > previous {
                segments.push((previous, start, false));
            }
            segments.push((start, end, true));
            previous = end;
        })?;
        // Test emitted segments rather than `previous`: even a zero-width match
        // must retain the ordinary delimiter behavior instead of taking this path.
        if segments.is_empty() {
            return Ok(segments);
        }
        if previous < input.len() {
            segments.push((previous, input.len(), false));
        }

        if self.invert {
            for segment in &mut segments {
                segment.2 = !segment.2;
            }
        }
        Ok(segments)
    }

    /// Emit simple ranges directly; merged behaviors retain their stateful range buffer.
    fn apply_behavior(
        &self,
        segments: &[(usize, usize, bool)],
        mut emit: impl FnMut(usize, usize),
    ) {
        let ranges = match self.behavior {
            SplitBehavior::Removed | SplitBehavior::Isolated => {
                for &(start, end, is_match) in segments {
                    if self.behavior == SplitBehavior::Isolated || !is_match {
                        emit(start, end);
                    }
                }
                return;
            }
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
        };
        for (start, end) in ranges {
            emit(start, end);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pre_tokenizers::scanner::PatternId;
    use serde_json::json;
    use tokenizers::{
        OffsetReferential, OffsetType, PreTokenizedString as HfPreTokenizedString, PreTokenizer,
        pre_tokenizers::split::Split as HfSplit,
    };

    const BEHAVIORS: &[&str] = &[
        "Removed",
        "Isolated",
        "MergedWithPrevious",
        "MergedWithNext",
        "Contiguous",
    ];

    /// A failure after an emitted match must leave the caller's splits intact.
    #[test]
    fn regex_failure_preserves_original_splits() {
        let regex = fancy_regex::RegexBuilder::new("a|(x+x+)+(?>y)")
            .backtrack_limit(1)
            .build()
            .unwrap();
        let input = "axxxxxxxxxxy";
        // These enter the two specialized paths and the shared segment path.
        for (behavior, invert) in [
            (SplitBehavior::Isolated, false),
            (SplitBehavior::Removed, true),
            (SplitBehavior::MergedWithNext, false),
        ] {
            let split = Split {
                matcher: Matcher::Regex(regex.clone()),
                behavior,
                invert,
                extra: Default::default(),
            };
            let mut pts = PreTokenizedString::from_text(input);
            let original = pts.splits().to_vec();
            assert!(matches!(
                split.pre_tokenize(&mut pts),
                Err(Error::Regex(fancy_regex::Error::RuntimeError(
                    fancy_regex::RuntimeError::BacktrackLimitExceeded
                )))
            ));
            assert_eq!(pts.splits(), original);
        }
    }

    #[test]
    fn unmatched_input_needs_no_segments() {
        // No matches need neither a segment allocation nor a behavior buffer.
        for literal in ["-", ""] {
            let split =
                Split::from_config(&json!({"String": literal}), "MergedWithPrevious", false)
                    .unwrap();
            for input in ["", "a", "café中文"] {
                assert!(split.find_segments(input).unwrap().is_empty());
            }
        }
        let split =
            Split::from_config(&json!({"String": "-"}), "MergedWithPrevious", false).unwrap();
        assert!(!split.find_segments("-a").unwrap().is_empty());
    }

    fn assert_matches_hf(pattern: &Value, behavior: &str, invert: bool, input: &str) {
        let split = Split::from_config(pattern, behavior, invert).unwrap();
        let reference: HfSplit = serde_json::from_value(json!({
            "type": "Split", "pattern": pattern, "behavior": behavior, "invert": invert,
        }))
        .unwrap();
        let mut actual = PreTokenizedString::from_text(input);
        split.pre_tokenize(&mut actual).unwrap();
        let actual: Vec<_> = actual
            .splits()
            .iter()
            .map(|piece| {
                (
                    actual.split_text(piece),
                    (piece.range.start, piece.range.end),
                )
            })
            .collect();
        let mut expected = HfPreTokenizedString::from(input);
        reference.pre_tokenize(&mut expected).unwrap();
        let expected: Vec<_> = expected
            .get_splits(OffsetReferential::Original, OffsetType::Byte)
            .into_iter()
            .map(|(text, range, _)| (text, range))
            .collect();
        assert_eq!(
            actual, expected,
            "{pattern} {behavior} invert={invert} input={input:?}"
        );
    }

    #[test]
    fn literal_and_regex_ranges_match_hf() {
        for pattern in [
            json!({"String":"-"}),
            json!({"String":"▁"}),
            json!({"String":"::"}),
            json!({"String":"[a]"}),
            json!({"Regex":r"\d+"}),
            json!({"Regex":r"(?=a)"}),
            json!({"Regex":r"(?<=a)"}),
            json!({"Regex":r"z+"}),
        ] {
            for behavior in BEHAVIORS {
                for invert in [false, true] {
                    for input in [
                        "",
                        "x",
                        "café中文",
                        "-leading",
                        "trailing-",
                        "a--b",
                        "é▁中▁",
                        "::a::::b::",
                        "a[a]b",
                        "abc123def456",
                        "a",
                        "baa",
                        "the-final--countdown",
                    ] {
                        assert_matches_hf(&pattern, behavior, invert, input);
                    }
                }
            }
        }
    }

    #[test]
    fn fixed_scanners_match_hugging_face_on_semantic_edges() {
        let patterns = [
            (PatternId::Llama.pattern(), "Isolated", false),
            (PatternId::LlamaContraction.pattern(), "Isolated", false),
            (PatternId::LlamaContraction.pattern(), "Removed", true),
            (PatternId::Kimi.pattern(), "Isolated", false),
            (PatternId::Kimi.pattern(), "MergedWithPrevious", false),
            (PatternId::Qwen.pattern(), "Isolated", false),
            (PatternId::QwenMark.pattern(), "Isolated", false),
            (PatternId::PhiGlm.pattern(), "Isolated", false),
            (PatternId::PhiGlm.pattern(), "Removed", true),
            (PatternId::DeepSeekNumber.pattern(), "Isolated", false),
            (PatternId::DeepSeekCjk.pattern(), "Isolated", false),
            (PatternId::DeepSeekMain.pattern(), "Isolated", false),
            (PatternId::Gpt2.pattern(), "Isolated", false),
            (PatternId::Gpt2CaseInsensitive.pattern(), "Isolated", false),
        ];
        let inputs = [
            "I'm DON'T we're HTTPServer camelCase 12345",
            " café\u{301} \u{01c5}uro 你好世界! \t\n  trailing  ",
            "a.\n\nx",
            "$True foo\tbar\r\n/baz 🚀",
            "\u{216b}\u{b2} numbers\u{85}\u{a0}space\u{2028}line",
            "\u{300}\u{301}\u{302}",
            "汉字𠀀かなカナ mixedHan字word",
            "\u{3}\u{b}BASKETBALL",
        ];

        for (source, behavior, invert) in patterns {
            for input in inputs {
                assert_matches_hf(&json!({"Regex": source}), behavior, invert, input);
            }
        }
    }

    #[test]
    fn invalid_regex_is_rejected() {
        let error =
            Split::from_config(&json!({"Regex": "(unclosed"}), "Isolated", false).unwrap_err();
        assert!(matches!(error, Error::Unsupported(_)));
    }

    #[test]
    fn no_match_preserves_protected_ranges_for_every_behavior() {
        let original = vec![
            PtSplit {
                range: 0..2,
                token_id: None,
            },
            PtSplit {
                range: 2..3,
                token_id: Some(u32::MAX),
            },
            PtSplit {
                range: 3..6,
                token_id: None,
            },
            PtSplit {
                range: 6..6,
                token_id: Some(0),
            },
        ];
        for &behavior in BEHAVIORS {
            for invert in [false, true] {
                let split = Split::from_config(&json!({"String": "-"}), behavior, invert).unwrap();
                let mut ranges = original.clone();
                ranges.push(PtSplit {
                    range: 6..6,
                    token_id: None,
                });
                let mut pts = PreTokenizedString::new("é|中".to_string(), ranges);
                split.pre_tokenize(&mut pts).unwrap();
                let expected: Vec<_> = original
                    .iter()
                    .filter(|piece| behavior != "Removed" || !invert || piece.token_id.is_some())
                    .cloned()
                    .collect();
                assert_eq!(pts.splits(), expected, "{behavior} invert={invert}");
                assert_eq!(pts.buffer(), "é|中");
            }
        }
    }

    #[test]
    fn pre_tokenize_preserves_added_tokens() {
        let split = Split::from_config(&json!({"String": " "}), "Removed", false).unwrap();
        let mut pts = PreTokenizedString::new(
            "hello world".to_string(),
            vec![
                PtSplit {
                    range: 0..5,
                    token_id: None,
                },
                PtSplit {
                    range: 5..5,
                    token_id: Some(42),
                },
                PtSplit {
                    range: 5..11,
                    token_id: None,
                },
            ],
        );
        split.pre_tokenize(&mut pts).unwrap();
        assert!(pts.splits().iter().any(|piece| piece.token_id == Some(42)));
    }
}
