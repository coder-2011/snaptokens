pub(crate) mod byte_level;
mod metaspace;
mod scanner;
mod split;

use crate::{
    json_structs::PreTokenizerConfig,
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
};

pub use self::{
    byte_level::ByteLevel,
    metaspace::Metaspace,
    split::{Split, SplitBehavior},
};

pub(crate) use self::byte_level::BYTE_TO_CHAR;

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
            let _ = Self::for_each_word::<()>(text, |start, end| {
                splits.push(PtSplit {
                    range: split.range.start + start..split.range.start + end,
                    token_id: None,
                });
                Ok(())
            });
        }
        pts.refine_splits(splits);
    }

    /// Emits each non-empty Unicode-whitespace-delimited word, dropping the whitespace.
    #[inline(always)]
    pub(crate) fn for_each_word<E>(
        text: &str,
        mut emit: impl FnMut(usize, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut start = 0;
        for (offset, character) in text.char_indices() {
            if character.is_whitespace() {
                if start < offset {
                    emit(start, offset)?;
                }
                start = offset + character.len_utf8();
            }
        }
        if start < text.len() {
            emit(start, text.len())?;
        }
        Ok(())
    }
}

pub(crate) trait FusedPieceSink {
    /// Queue one non-empty range from `input` without revalidating it.
    ///
    /// # Safety
    ///
    /// `start..end` must be an in-bounds UTF-8 range of `input`.
    unsafe fn push_piece(&mut self, input: &str, start: usize, end: usize);

    #[inline(always)]
    unsafe fn push_mask(
        &mut self,
        input: &str,
        mask_base: usize,
        start: &mut usize,
        mut mask: u64,
    ) {
        while mask != 0 {
            let end = mask_base + mask.trailing_zeros() as usize;
            mask &= mask - 1;
            unsafe { self.push_piece(input, *start, end) };
            *start = end;
        }
    }
}

/// An error while constructing or applying a pre-tokenizer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON value could not be deserialized into the expected type (e.g. an
    /// unrecognized pattern format or behavior string).
    #[error("invalid config value: {0}")]
    Json(#[from] serde_json::Error),

    /// A configuration uses a pre-tokenizer form that Snaptokens does not support.
    #[error("unsupported pre-tokenizer configuration: {0}")]
    Unsupported(String),
}

/// A supported text pre-tokenization step.
#[derive(Clone, Debug)]
pub enum PreTokenizer {
    /// GPT-2-style byte-level encoding and splitting.
    ByteLevel(ByteLevel),
    /// Pattern-based text splitting.
    Split(Split),
    /// Splits ordinary text on Unicode whitespace and drops the delimiters.
    WhitespaceSplit(WhitespaceSplit),
    /// Rewrites SentencePiece spaces and splits on the replacement marker.
    Metaspace(Metaspace),
    /// Pre-tokenizer steps applied from left to right.
    Sequence(Vec<PreTokenizer>),
}

#[derive(Clone, Copy)]
pub(crate) struct FusedSplits<'a> {
    steps: &'a [PreTokenizer],
}

impl<'a> FusedSplits<'a> {
    pub(crate) fn single(self) -> Option<&'a Split> {
        match self.steps {
            [PreTokenizer::Split(split)] => Some(split),
            _ => None,
        }
    }

    pub(crate) fn newline_partition_ranges(
        self,
        input: &str,
        parts: usize,
    ) -> Option<Vec<std::ops::Range<usize>>> {
        let supported = self.is_deepseek()
            || matches!(
                self.single().and_then(Split::pattern_id),
                Some(
                    scanner::PatternId::Llama
                        | scanner::PatternId::LlamaContraction
                        | scanner::PatternId::Kimi
                        | scanner::PatternId::Qwen
                        | scanner::PatternId::QwenMark
                        | scanner::PatternId::PhiGlm
                )
            );
        if !supported {
            return None;
        }

        let parts = parts.max(1);
        let mut ranges = Vec::with_capacity(parts);
        let mut start = 0;
        let mut next_part = 1;
        // A raw newline can sit inside `\s*[\r\n]+`, so cutting after its run can
        // split one model input. Whole-input fused-piece ends preserve the serial inputs.
        self.for_each_piece(input, |_, _, end| {
            let mut crossed_target = false;
            while next_part < parts && next_part * input.len() / parts <= end {
                crossed_target = true;
                next_part += 1;
            }
            if crossed_target && end > start && end < input.len() {
                ranges.push(start..end);
                start = end;
            }
        });
        ranges.push(start..input.len());
        Some(ranges)
    }

    pub(crate) fn for_each_piece(self, input: &str, mut emit: impl FnMut(&str, usize, usize)) {
        if let Some(split) = self.single() {
            split.for_each_fused_piece(input, emit);
            return;
        }
        if self.is_deepseek() {
            scanner::for_each_deepseek_piece(input, |start, end| emit(input, start, end));
            return;
        }
        visit_fused_splits(self.steps, input, 0, input.len(), &mut emit);
    }

    #[inline(always)]
    pub(crate) fn stream_into(self, input: &str, sink: &mut impl FusedPieceSink) {
        if let Some(split) = self.single()
            && let Some(pattern) = split.pattern_id()
            && scanner::stream_mask_matches(pattern, input, sink)
        {
            return;
        }
        self.for_each_piece(input, |input, start, end| {
            // SAFETY: every Split emitter returns exact ranges of `input`.
            unsafe { sink.push_piece(input, start, end) }
        });
    }

    /// Applies each fused Split step to an already-built pre-tokenized buffer.
    pub(crate) fn pre_tokenize(self, pts: &mut PreTokenizedString) -> Result<(), Error> {
        for step in self.steps {
            step.pre_tokenize(pts)?;
        }
        Ok(())
    }

    fn is_deepseek(self) -> bool {
        match self.steps {
            [
                PreTokenizer::Split(numbers),
                PreTokenizer::Split(cjk),
                PreTokenizer::Split(main),
            ] => {
                numbers.pattern_id() == Some(scanner::PatternId::DeepSeekNumber)
                    && cjk.pattern_id() == Some(scanner::PatternId::DeepSeekCjk)
                    && main.pattern_id() == Some(scanner::PatternId::DeepSeekMain)
            }
            _ => false,
        }
    }
}

fn visit_fused_splits(
    steps: &[PreTokenizer],
    input: &str,
    start: usize,
    end: usize,
    emit: &mut dyn FnMut(&str, usize, usize),
) {
    let Some((step, remaining)) = steps.split_first() else {
        emit(input, start, end);
        return;
    };
    let PreTokenizer::Split(split) = step else {
        unreachable!("fused Split chain was validated at construction");
    };
    split.for_each_fused_piece(&input[start..end], |_, piece_start, piece_end| {
        visit_fused_splits(
            remaining,
            input,
            start + piece_start,
            start + piece_end,
            emit,
        );
    });
}

impl PreTokenizer {
    /// Builds a pre-tokenizer from parsed tokenizer JSON configuration.
    pub fn from_config(config: PreTokenizerConfig) -> Result<Self, Error> {
        match config {
            PreTokenizerConfig::ByteLevel(bl) => Ok(Self::ByteLevel(bl)),
            PreTokenizerConfig::Split(s) => Ok(Self::Split(s)),
            PreTokenizerConfig::WhitespaceSplit => Ok(Self::WhitespaceSplit(WhitespaceSplit)),
            PreTokenizerConfig::Metaspace(config) => Ok(Self::Metaspace(
                Metaspace::from_config(config).map_err(Error::Unsupported)?,
            )),
            PreTokenizerConfig::Sequence { pretokenizers } => {
                let steps = pretokenizers
                    .into_iter()
                    .map(Self::from_config)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Sequence(steps))
            }
        }
    }

    /// Refines a normalized intermediate string into tokenization splits.
    pub fn pre_tokenize(&self, pts: &mut PreTokenizedString) -> Result<(), Error> {
        match self {
            Self::ByteLevel(bl) => bl.pre_tokenize(pts),
            Self::Split(s) => s.pre_tokenize(pts),
            Self::WhitespaceSplit(whitespace) => {
                whitespace.pre_tokenize(pts);
                Ok(())
            }
            Self::Metaspace(metaspace) => {
                metaspace.pre_tokenize(pts);
                Ok(())
            }
            Self::Sequence(steps) => {
                for step in steps {
                    step.pre_tokenize(pts)?;
                }
                Ok(())
            }
        }
    }

    pub(crate) fn fused_byte_level(&self) -> Option<(Option<FusedSplits<'_>>, &ByteLevel)> {
        match self {
            Self::ByteLevel(byte_level) if byte_level.can_fuse() => Some((None, byte_level)),
            Self::Sequence(steps) => {
                let (last, split_steps) = steps.split_last()?;
                let Self::ByteLevel(byte_level) = last else {
                    return None;
                };
                if split_steps.is_empty()
                    || !byte_level.is_bulk_only()
                    || !split_steps.iter().all(
                        |step| matches!(step, Self::Split(split) if split.supports_fused_stream()),
                    )
                {
                    return None;
                }
                Some((Some(FusedSplits { steps: split_steps }), byte_level))
            }
            _ => None,
        }
    }

    /// Recognizes the SentencePiece word pipeline used by supported Unigram JSON.
    pub(crate) fn fused_whitespace_metaspace(&self) -> Option<&Metaspace> {
        match self {
            Self::Sequence(steps) => match steps.as_slice() {
                [Self::WhitespaceSplit(_), Self::Metaspace(metaspace)] => Some(metaspace),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn contains_byte_level(&self) -> bool {
        match self {
            Self::ByteLevel(_) => true,
            Self::Split(_) | Self::WhitespaceSplit(_) | Self::Metaspace(_) => false,
            Self::Sequence(steps) => steps.iter().any(Self::contains_byte_level),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deepseek_fused_scanner_matches_split_chain() {
        let patterns = [
            scanner::PatternId::DeepSeekNumber.pattern(),
            scanner::PatternId::DeepSeekCjk.pattern(),
            scanner::PatternId::DeepSeekMain.pattern(),
        ];
        let steps: Vec<_> = patterns
            .into_iter()
            .map(|pattern| {
                PreTokenizer::Split(
                    Split::from_config(&json!({"Regex": pattern}), "Isolated", false).unwrap(),
                )
            })
            .collect();
        let fused = FusedSplits { steps: &steps };

        let compare = |input: &str| {
            let mut expected = Vec::new();
            visit_fused_splits(&steps, input, 0, input.len(), &mut |_, start, end| {
                expected.push((start, end));
            });
            let mut actual = Vec::new();
            fused.for_each_piece(input, |_, start, end| actual.push((start, end)));
            assert_eq!(actual, expected, "DeepSeek pieces diverged on {input:?}");
        };

        for input in [
            "a  1 日本語・テスト\n",
            "cafe\u{301} \u{301}leading ١٢٣٤",
            "x\0\0b \u{200b}word 3.14159",
            "\u{3040}あ ゛カナ 一二三１２３",
            "@user #tag <|endoftext|>",
        ] {
            compare(input);
        }

        let atoms = [
            "a", "Z", "é", "١", " ", "\t", "\n", "\u{301}", ".", "☃", "\0", "\u{200b}", "一", "あ",
            "・",
        ];
        let mut state = 0xDEE9_5EEC_u64;
        for _ in 0..1_000 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let length = 1 + (state as usize % 39);
            let mut input = String::new();
            for _ in 0..length {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                input.push_str(atoms[state as usize % atoms.len()]);
            }
            compare(&input);
        }
    }
}
