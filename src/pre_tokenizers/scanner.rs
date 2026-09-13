use super::FusedPieceSink;

mod mask_scanner;

/// Table mapping each recognized pattern to its regex source string.
/// Used by `PatternId::from_source` to identify known tokenizer patterns.
const PATTERN_TABLE: &[(PatternId, &str)] = &[
    // Llama family (Upper+Lower word style)
    (
        PatternId::Llama,
        concat!(
            r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]+",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]*",
            r"|\p{N}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    (
        PatternId::LlamaContraction,
        concat!(
            r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|\p{N}{1,3}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    (
        PatternId::Kimi,
        concat!(
            r"[\p{Han}]+",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}&&[^\p{Han}]]*",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}&&[^\p{Han}]]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}&&[^\p{Han}]]+",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}&&[^\p{Han}]]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|\p{N}{1,3}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    // Qwen family (letter-run word style)
    (
        PatternId::Qwen,
        concat!(
            r"(?i:'s|'t|'re|'ve|'m|'ll|'d)",
            r"|[^\r\n\p{L}\p{N}]?\p{L}+",
            r"|\p{N}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    (
        PatternId::QwenMark,
        concat!(
            r"(?i:'s|'t|'re|'ve|'m|'ll|'d)",
            r"|[^\r\n\p{L}\p{N}]?[\p{L}\p{M}]+",
            r"|\p{N}",
            r"| ?[^\s\p{L}\p{M}\p{N}]+[\r\n]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    (
        PatternId::PhiGlm,
        concat!(
            r"(?i:'s|'t|'re|'ve|'m|'ll|'d)",
            r"|[^\r\n\p{L}\p{N}]?\p{L}+",
            r"|\p{N}{1,3}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+",
        ),
    ),
    // DeepSeek (3-stage pipeline)
    (PatternId::DeepSeekNumber, r"\p{N}{1,3}"),
    (PatternId::DeepSeekCjk, r"[一-龥぀-ゟ゠-ヿ]+"),
    (
        PatternId::DeepSeekMain,
        r##"[!"#$%&'()*+,\-./:;<=>?@\[\\\]^_`{|}~][A-Za-z]+|[^\r\n\p{L}\p{P}\p{S}]?[\p{L}\p{M}]+| ?[\p{P}\p{S}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+"##,
    ),
    // GPT-2 family
    (
        PatternId::Gpt2,
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+",
    ),
    (
        PatternId::Gpt2CaseInsensitive,
        r"'(?i:[sdmt])|'(?i:ll|ve|re)| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+",
    ),
];

const LETTER: u8 = 1 << 0;
const UPPER_WORD: u8 = 1 << 1;
const LOWER_WORD: u8 = 1 << 2;
const MARK: u8 = 1 << 3;
const NUMBER: u8 = 1 << 4;
const PUNCT_OR_SYMBOL: u8 = 1 << 5;
const WHITESPACE: u8 = 1 << 6;
const HAN: u8 = 1 << 7;

const ASCII_CLASSES: [u8; 128] = build_ascii_classes();
const BYTE_HIGH_BITS: u64 = 0x8080_8080_8080_8080;

const fn build_ascii_classes() -> [u8; 128] {
    let mut classes = [0; 128];
    let mut value = 0usize;
    while value < classes.len() {
        let byte = value as u8;
        classes[value] = if byte >= b'A' && byte <= b'Z' {
            LETTER | UPPER_WORD
        } else if byte >= b'a' && byte <= b'z' {
            LETTER | LOWER_WORD
        } else if byte >= b'0' && byte <= b'9' {
            NUMBER
        } else if byte == b' ' || (byte >= b'\t' && byte <= b'\r') {
            WHITESPACE
        } else if (byte >= b'!' && byte <= b'/')
            || (byte >= b':' && byte <= b'@')
            || (byte >= b'[' && byte <= b'`')
            || (byte >= b'{' && byte <= b'~')
        {
            PUNCT_OR_SYMBOL
        } else {
            0
        };
        value += 1;
    }
    classes
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PatternId {
    Llama,
    LlamaContraction,
    Kimi,
    Qwen,
    QwenMark,
    PhiGlm,
    DeepSeekNumber,
    DeepSeekCjk,
    DeepSeekMain,
    Gpt2,
    Gpt2CaseInsensitive,
}

impl PatternId {
    #[cfg(test)]
    pub(crate) fn pattern(self) -> &'static str {
        PATTERN_TABLE
            .iter()
            .find(|(id, _)| *id == self)
            .map(|(_, pat)| *pat)
            .expect("all PatternId variants are in PATTERN_TABLE")
    }

    pub(crate) fn from_source(source: &str) -> Option<Self> {
        // Look up in the pattern table
        if let Some(&(id, _)) = PATTERN_TABLE.iter().find(|(_, pat)| *pat == source) {
            return Some(id);
        }

        // DeepSeek stores CR/LF as literal JSON control characters instead of
        // backslash escapes; normalize only that construction-time spelling.
        if source.contains(['\r', '\n']) {
            let normalized = source.replace('\r', r"\r").replace('\n', r"\n");
            return Self::from_source(&normalized);
        }
        None
    }

    pub(crate) fn for_each_match(self, input: &str, emit: impl FnMut(usize, usize)) {
        let mut sink = RangeSink(emit);
        if stream_mask_matches(self, input, &mut sink) {
            return;
        }

        let RangeSink(emit) = sink;
        match self {
            Self::DeepSeekNumber => scan_number_matches::<3>(input, emit),
            Self::DeepSeekCjk => scan_deepseek_cjk_matches(input, emit),
            Self::DeepSeekMain => scan_matches(input, scan_deepseek_main, emit),
            Self::Gpt2CaseInsensitive => scan_gpt2_case_insensitive_matches(input, emit),
            _ => unreachable!("mask grammars returned above"),
        }
    }

    pub(crate) fn for_each_ascii_match(self, input: &str, emit: impl FnMut(usize, usize)) -> bool {
        match self {
            Self::Qwen | Self::QwenMark => scan_ascii_matches(input, scan_qwen_ascii::<1>, emit),
            _ => false,
        }
    }
}

struct RangeSink<F>(F);

impl<F: FnMut(usize, usize)> FusedPieceSink for RangeSink<F> {
    #[inline(always)]
    unsafe fn push_piece(&mut self, _input: &str, start: usize, end: usize) {
        (self.0)(start, end);
    }
}

#[inline(always)]
pub(super) fn stream_mask_matches(
    pattern: PatternId,
    input: &str,
    sink: &mut impl FusedPieceSink,
) -> bool {
    match pattern {
        PatternId::Llama => mask_scanner::for_each_match_into::<false, 1, _>(input, sink),
        PatternId::LlamaContraction => mask_scanner::for_each_match_into::<true, 3, _>(input, sink),
        PatternId::Kimi => mask_scanner::for_each_kimi_match_into(input, sink),
        PatternId::Qwen => mask_scanner::for_each_qwen_match_into::<false, 1, _>(input, sink),
        PatternId::QwenMark => mask_scanner::for_each_qwen_match_into::<true, 1, _>(input, sink),
        PatternId::PhiGlm => mask_scanner::for_each_qwen_match_into::<false, 3, _>(input, sink),
        PatternId::Gpt2 => mask_scanner::for_each_r50k_match_into(input, sink),
        _ => return false,
    }
    true
}

fn scan_number_matches<const LIMIT: usize>(input: &str, mut emit: impl FnMut(usize, usize)) {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos].is_ascii() {
            match scan_ascii_number::<LIMIT>(bytes, pos) {
                AsciiMatch::Match(end) => {
                    emit(pos, end);
                    pos = end;
                }
                AsciiMatch::NoMatch => pos += 1,
                AsciiMatch::NeedsUnicode => {
                    let end = scan_number::<LIMIT>(input, pos).expect("ASCII digit run");
                    emit(pos, end);
                    pos = end;
                }
            }
            continue;
        }

        let unit = unit_at_non_ascii(input, pos);
        if unit.is_number() {
            let end = scan_number::<LIMIT>(input, pos).expect("Unicode number run");
            emit(pos, end);
            pos = end;
        } else {
            pos = unit.end;
        }
    }
}

fn scan_matches(
    input: &str,
    mut scan_at: impl FnMut(&str, usize) -> Option<usize>,
    mut emit: impl FnMut(usize, usize),
) {
    let mut pos = 0;
    while pos < input.len() {
        if let Some(end) = scan_at(input, pos) {
            debug_assert!(end > pos);
            emit(pos, end);
            pos = end;
        } else {
            pos = unit_at(input, pos).end;
        }
    }
}

fn scan_ascii_matches(
    input: &str,
    mut scan_at: impl FnMut(&str, usize) -> AsciiMatch,
    mut emit: impl FnMut(usize, usize),
) -> bool {
    let mut pos = 0;
    while pos < input.len() {
        if !input.as_bytes()[pos].is_ascii() {
            return false;
        }
        match scan_at(input, pos) {
            AsciiMatch::Match(end) => {
                emit(pos, end);
                pos = end;
            }
            AsciiMatch::NoMatch => pos += 1,
            AsciiMatch::NeedsUnicode => return false,
        }
    }
    true
}

#[derive(Clone, Copy)]
struct Unit {
    ch: char,
    end: usize,
    class: u8,
}

enum AsciiMatch {
    Match(usize),
    NoMatch,
    NeedsUnicode,
}

impl Unit {
    fn is_letter(self) -> bool {
        self.class & LETTER != 0
    }

    fn is_upper_word(self) -> bool {
        self.class & UPPER_WORD != 0
    }

    fn is_lower_word(self) -> bool {
        self.class & LOWER_WORD != 0
    }

    fn is_mark(self) -> bool {
        self.class & MARK != 0
    }

    fn is_number(self) -> bool {
        self.class & NUMBER != 0
    }

    fn is_punctuation_or_symbol(self) -> bool {
        self.class & PUNCT_OR_SYMBOL != 0
    }

    fn is_whitespace(self) -> bool {
        self.class & WHITESPACE != 0
    }

    fn is_newline(self) -> bool {
        self.ch == '\r' || self.ch == '\n'
    }
}

#[inline(always)]
fn unit_at(input: &str, pos: usize) -> Unit {
    let byte = input.as_bytes()[pos];
    if byte < 128 {
        return Unit {
            ch: byte as char,
            end: pos + 1,
            class: ASCII_CLASSES[byte as usize],
        };
    }

    unit_at_non_ascii(input, pos)
}

#[cold]
#[inline(never)]
fn unit_at_non_ascii(input: &str, pos: usize) -> Unit {
    let ch = input[pos..].chars().next().expect("valid UTF-8 boundary");
    Unit {
        ch,
        end: pos + ch.len_utf8(),
        class: unicode_class(ch),
    }
}

fn build_generic_unicode_class_table() -> Box<[u8]> {
    use icu_properties::{
        CodePointMapData, CodePointSetData,
        props::{GeneralCategory, Script, WhiteSpace},
    };

    let mut classes = vec![0; 0x110000];
    for range in CodePointMapData::<GeneralCategory>::new().iter_ranges() {
        let class = match range.value {
            GeneralCategory::UppercaseLetter | GeneralCategory::TitlecaseLetter => {
                LETTER | UPPER_WORD
            }
            GeneralCategory::LowercaseLetter => LETTER | LOWER_WORD,
            GeneralCategory::ModifierLetter | GeneralCategory::OtherLetter => {
                LETTER | UPPER_WORD | LOWER_WORD
            }
            GeneralCategory::NonspacingMark
            | GeneralCategory::SpacingMark
            | GeneralCategory::EnclosingMark => MARK | UPPER_WORD | LOWER_WORD,
            GeneralCategory::DecimalNumber
            | GeneralCategory::LetterNumber
            | GeneralCategory::OtherNumber => NUMBER,
            GeneralCategory::DashPunctuation
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::ConnectorPunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::OtherPunctuation
            | GeneralCategory::MathSymbol
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::ModifierSymbol
            | GeneralCategory::OtherSymbol => PUNCT_OR_SYMBOL,
            _ => 0,
        };
        classes[*range.range.start() as usize..=*range.range.end() as usize].fill(class);
    }
    for range in CodePointSetData::new::<WhiteSpace>().iter_ranges() {
        classes[*range.start() as usize..=*range.end() as usize].fill(WHITESPACE);
    }
    for range in CodePointMapData::<Script>::new().iter_ranges_for_value(Script::Han) {
        // Preserve the general category while caching Kimi's orthogonal script test.
        for class in &mut classes[*range.start() as usize..=*range.end() as usize] {
            *class |= HAN;
        }
    }
    classes.into_boxed_slice()
}

static GENERIC_UNICODE_CLASSES: std::sync::LazyLock<Box<[u8]>> =
    std::sync::LazyLock::new(build_generic_unicode_class_table);

fn unicode_class(ch: char) -> u8 {
    GENERIC_UNICODE_CLASSES[ch as usize]
}

#[inline(always)]
fn scan_llama<const CONTRACTIONS: bool, const DIGITS: usize>(
    input: &str,
    pos: usize,
) -> Option<usize> {
    if input.as_bytes()[pos].is_ascii() {
        match scan_llama_ascii::<CONTRACTIONS, DIGITS>(input, pos) {
            AsciiMatch::Match(end) => return Some(end),
            AsciiMatch::NoMatch | AsciiMatch::NeedsUnicode => {}
        }
    }

    let first = unit_at(input, pos);
    let has_prefix = !first.is_newline() && !first.is_letter() && !first.is_number();
    let word_start = if has_prefix { first.end } else { pos };

    let mut word_end = scan_llama_word::<false>(input, word_start);
    if word_end.is_none() && has_prefix {
        // The optional prefix is greedy, so retry without it when the word body fails.
        word_end = scan_llama_word::<false>(input, pos);
    }
    if let Some(end) = word_end {
        if CONTRACTIONS {
            return Some(scan_contraction(input, end, true).unwrap_or(end));
        }
        return Some(end);
    }

    scan_number::<DIGITS>(input, pos)
        .or_else(|| scan_punctuation(input, pos, false, true, true))
        .or_else(|| scan_whitespace(input, pos, true))
}

fn scan_llama_ascii<const CONTRACTIONS: bool, const DIGITS: usize>(
    input: &str,
    pos: usize,
) -> AsciiMatch {
    let bytes = input.as_bytes();
    let first = bytes[pos];
    if first.is_ascii_alphabetic() {
        return match scan_llama_ascii_word(bytes, pos) {
            AsciiMatch::Match(end) if CONTRACTIONS => {
                AsciiMatch::Match(scan_contraction(input, end, true).unwrap_or(end))
            }
            result => result,
        };
    }

    if first.is_ascii_digit() {
        return scan_ascii_number::<DIGITS>(bytes, pos);
    }

    if first == b'\r' || first == b'\n' {
        return scan_ascii_whitespace(bytes, pos, true);
    }

    // Every remaining byte may prefix one case-sensitive word run.
    if pos + 1 < bytes.len() {
        if bytes[pos + 1] >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        match scan_llama_ascii_word(bytes, pos + 1) {
            AsciiMatch::Match(end) => {
                if CONTRACTIONS {
                    return AsciiMatch::Match(scan_contraction(input, end, true).unwrap_or(end));
                }
                return AsciiMatch::Match(end);
            }
            AsciiMatch::NeedsUnicode => return AsciiMatch::NeedsUnicode,
            AsciiMatch::NoMatch => {}
        }
    }

    match scan_ascii_punctuation(bytes, pos, true, true) {
        AsciiMatch::NoMatch => scan_ascii_whitespace(bytes, pos, true),
        result => result,
    }
}

#[inline(always)]
fn scan_ascii_number<const LIMIT: usize>(bytes: &[u8], pos: usize) -> AsciiMatch {
    if !bytes[pos].is_ascii_digit() {
        return AsciiMatch::NoMatch;
    }

    let mut cursor = pos + 1;
    let mut count = 1usize;
    while cursor < bytes.len() && count < LIMIT {
        let byte = bytes[cursor];
        if byte >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        if !byte.is_ascii_digit() {
            break;
        }
        cursor += 1;
        count += 1;
    }
    AsciiMatch::Match(cursor)
}

#[inline(always)]
fn is_tokenizer_whitespace(byte: u8) -> bool {
    byte == b' ' || matches!(byte, b'\t'..=b'\r')
}

fn scan_llama_ascii_word(bytes: &[u8], pos: usize) -> AsciiMatch {
    if pos >= bytes.len() {
        return AsciiMatch::NoMatch;
    }
    if bytes[pos] >= 128 {
        return AsciiMatch::NeedsUnicode;
    }

    let mut cursor = pos;
    while cursor < bytes.len() && bytes[cursor].is_ascii_uppercase() {
        cursor += 1;
    }
    if cursor < bytes.len() && bytes[cursor] >= 128 {
        return AsciiMatch::NeedsUnicode;
    }

    let lower_start = cursor;
    while cursor < bytes.len() && bytes[cursor].is_ascii_lowercase() {
        cursor += 1;
    }
    if cursor < bytes.len() && bytes[cursor] >= 128 {
        return AsciiMatch::NeedsUnicode;
    }
    if cursor > lower_start || lower_start > pos {
        return AsciiMatch::Match(cursor);
    }
    AsciiMatch::NoMatch
}

fn scan_ascii_punctuation(
    bytes: &[u8],
    pos: usize,
    suffix_newlines: bool,
    suffix_slash: bool,
) -> AsciiMatch {
    let mut cursor = if bytes[pos] == b' ' { pos + 1 } else { pos };
    let body_start = cursor;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        if byte >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        if is_tokenizer_whitespace(byte) || byte.is_ascii_alphabetic() || byte.is_ascii_digit() {
            break;
        }
        cursor += 1;
    }
    if cursor == body_start {
        return AsciiMatch::NoMatch;
    }

    while cursor < bytes.len() {
        let byte = bytes[cursor];
        let newline = byte == b'\r' || byte == b'\n';
        if !(suffix_newlines && newline) && !(suffix_slash && byte == b'/') {
            break;
        }
        cursor += 1;
    }
    AsciiMatch::Match(cursor)
}

fn scan_ascii_whitespace(bytes: &[u8], pos: usize, newline_alternative: bool) -> AsciiMatch {
    if !is_tokenizer_whitespace(bytes[pos]) {
        return AsciiMatch::NoMatch;
    }

    let mut cursor = pos;
    let mut last_start = pos;
    let mut last_newline_end = None;
    let mut count = 0usize;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        if byte >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        if !is_tokenizer_whitespace(byte) {
            break;
        }
        last_start = cursor;
        if byte == b'\r' || byte == b'\n' {
            last_newline_end = Some(cursor + 1);
        }
        cursor += 1;
        count += 1;
    }

    if newline_alternative && let Some(end) = last_newline_end {
        return AsciiMatch::Match(end);
    }
    if cursor == bytes.len() {
        return AsciiMatch::Match(cursor);
    }
    if count >= 2 {
        return AsciiMatch::Match(last_start);
    }
    AsciiMatch::Match(cursor)
}

fn scan_kimi(input: &str, pos: usize) -> Option<usize> {
    if let Some(end) = scan_run(input, pos, is_han) {
        return Some(end);
    }

    let first = unit_at(input, pos);
    let has_prefix = !first.is_newline() && !first.is_letter() && !first.is_number();
    let word_start = if has_prefix { first.end } else { pos };
    let mut word_end = scan_llama_word::<true>(input, word_start);
    if word_end.is_none() && has_prefix {
        word_end = scan_llama_word::<true>(input, pos);
    }
    if let Some(end) = word_end {
        return Some(scan_contraction(input, end, true).unwrap_or(end));
    }

    scan_number::<3>(input, pos)
        .or_else(|| scan_punctuation(input, pos, false, true, false))
        .or_else(|| scan_whitespace(input, pos, true))
}

fn scan_llama_word<const EXCLUDE_HAN: bool>(input: &str, pos: usize) -> Option<usize> {
    if pos >= input.len() {
        return None;
    }
    scan_llama_lower_branch::<EXCLUDE_HAN>(input, pos)
        .or_else(|| scan_llama_upper_branch::<EXCLUDE_HAN>(input, pos))
}

fn scan_llama_lower_branch<const EXCLUDE_HAN: bool>(input: &str, pos: usize) -> Option<usize> {
    let mut cursor = pos;
    let mut last_lower_start = None;

    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !unit.is_upper_word() || (EXCLUDE_HAN && is_han(unit)) {
            break;
        }
        if unit.is_lower_word() {
            last_lower_start = Some(cursor);
        }
        cursor = unit.end;
    }

    if cursor < input.len() {
        let unit = unit_at(input, cursor);
        if unit.is_lower_word() && (!EXCLUDE_HAN || !is_han(unit)) {
            return scan_run(input, cursor, |unit| {
                unit.is_lower_word() && (!EXCLUDE_HAN || !is_han(unit))
            });
        }
    }
    last_lower_start.and_then(|start| {
        scan_run(input, start, |unit| {
            unit.is_lower_word() && (!EXCLUDE_HAN || !is_han(unit))
        })
    })
}

fn scan_llama_upper_branch<const EXCLUDE_HAN: bool>(input: &str, pos: usize) -> Option<usize> {
    let mut cursor = scan_run(input, pos, |unit| {
        unit.is_upper_word() && (!EXCLUDE_HAN || !is_han(unit))
    })?;
    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !unit.is_lower_word() || (EXCLUDE_HAN && is_han(unit)) {
            break;
        }
        cursor = unit.end;
    }
    Some(cursor)
}

fn is_han(unit: Unit) -> bool {
    unit.class & HAN != 0
}

#[inline(always)]
fn scan_qwen<const WORD_MARKS: bool, const DIGITS: usize>(
    input: &str,
    pos: usize,
) -> Option<usize> {
    if input.as_bytes()[pos].is_ascii() {
        match scan_qwen_ascii::<DIGITS>(input, pos) {
            AsciiMatch::Match(end) => return Some(end),
            AsciiMatch::NoMatch | AsciiMatch::NeedsUnicode => {}
        }
    }

    if let Some(end) = scan_contraction(input, pos, true) {
        return Some(end);
    }

    let first = unit_at(input, pos);
    let has_prefix = !first.is_newline() && !first.is_letter() && !first.is_number();
    let word_start = if has_prefix { first.end } else { pos };
    let word_predicate = |unit: Unit| unit.is_letter() || (WORD_MARKS && unit.is_mark());

    let mut word_end = scan_run(input, word_start, word_predicate);
    if word_end.is_none() && has_prefix {
        // A mark can satisfy both the greedy prefix and the Qwen 3.5 word body.
        word_end = scan_run(input, pos, word_predicate);
    }
    if word_end.is_some() {
        return word_end;
    }

    scan_number::<DIGITS>(input, pos)
        .or_else(|| scan_punctuation(input, pos, WORD_MARKS, true, false))
        .or_else(|| scan_whitespace(input, pos, true))
}

#[inline(always)]
fn advance_qwen(input: &str, start: usize) -> usize {
    let bytes = input.as_bytes();
    let first = bytes[start];
    if first.is_ascii_alphabetic() {
        return scan_gpt2_letters(input, start + 1);
    }

    if first == b' ' {
        let Some(&second) = bytes.get(start + 1) else {
            return start + 1;
        };
        if second.is_ascii_alphabetic() {
            return scan_gpt2_letters(input, start + 2);
        }
        if second.is_ascii_digit() {
            return start + 1;
        }
        if second >= 128 {
            let unit = unit_at_non_ascii(input, start + 1);
            if unit.is_letter() {
                return scan_gpt2_letters(input, unit.end);
            }
            if unit.is_number() {
                return start + 1;
            }
            if unit.is_whitespace() {
                return scan_whitespace(input, start, true).expect("leading space is whitespace");
            }
            return scan_qwen_other(input, unit.end);
        }
        if is_tokenizer_whitespace(second) {
            return scan_whitespace(input, start, true).expect("leading space is whitespace");
        }
        return scan_qwen_other(input, start + 2);
    }

    if first >= 128 {
        let unit = unit_at_non_ascii(input, start);
        if unit.is_letter() {
            return scan_gpt2_letters(input, unit.end);
        }
        if unit.is_number() {
            return unit.end;
        }
        if unit.is_newline() {
            return scan_whitespace(input, start, true).expect("newline is whitespace");
        }
        if unit.end < bytes.len() {
            let next = unit_at(input, unit.end);
            if next.is_letter() {
                return scan_gpt2_letters(input, next.end);
            }
        }
        if unit.is_whitespace() {
            return scan_whitespace(input, start, true).expect("classified whitespace");
        }
        return scan_qwen_other(input, unit.end);
    }

    if first.is_ascii_digit() {
        return start + 1;
    }
    if first == b'\''
        && let Some(end) = scan_contraction(input, start, true)
    {
        return end;
    }
    if first == b'\r' || first == b'\n' {
        return scan_whitespace(input, start, true).expect("newline is whitespace");
    }

    // Non-newline whitespace and punctuation may prefix one letter run.
    if start + 1 < bytes.len() {
        let next = unit_at(input, start + 1);
        if next.is_letter() {
            return scan_gpt2_letters(input, next.end);
        }
    }
    if is_tokenizer_whitespace(first) {
        return scan_whitespace(input, start, true).expect("classified whitespace");
    }
    scan_qwen_other(input, start + 1)
}

#[inline(always)]
fn scan_qwen_other(input: &str, mut pos: usize) -> usize {
    pos = scan_gpt2_other(input, pos);
    while pos < input.len() && matches!(input.as_bytes()[pos], b'\r' | b'\n') {
        pos += 1;
    }
    pos
}

fn scan_qwen_ascii<const DIGITS: usize>(input: &str, pos: usize) -> AsciiMatch {
    let bytes = input.as_bytes();
    let first = bytes[pos];

    if first.is_ascii_alphabetic() {
        let end = scan_ascii_letters(bytes, pos + 1);
        if end < bytes.len() && bytes[end] >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        return AsciiMatch::Match(end);
    }

    if first.is_ascii_digit() {
        return scan_ascii_number::<DIGITS>(bytes, pos);
    }

    if first == b'\''
        && let Some(end) = scan_contraction(input, pos, true)
    {
        return AsciiMatch::Match(end);
    }

    if first == b'\r' || first == b'\n' {
        return scan_ascii_whitespace(bytes, pos, true);
    }

    // Every remaining byte can prefix a word, except a space before a number.
    if let Some(&next) = bytes.get(pos + 1) {
        if next >= 128 {
            return AsciiMatch::NeedsUnicode;
        }
        if next.is_ascii_alphabetic() {
            let end = scan_ascii_letters(bytes, pos + 2);
            if end < bytes.len() && bytes[end] >= 128 {
                return AsciiMatch::NeedsUnicode;
            }
            return AsciiMatch::Match(end);
        }
    }

    // A literal space joins the following punctuation run.
    if first == b' '
        && bytes
            .get(pos + 1)
            .is_some_and(|next| !next.is_ascii_digit() && !is_tokenizer_whitespace(*next))
    {
        return scan_ascii_punctuation(bytes, pos, true, false);
    }

    if is_tokenizer_whitespace(first) {
        return scan_ascii_whitespace(bytes, pos, true);
    }

    scan_ascii_punctuation(bytes, pos, true, false)
}

fn scan_gpt2_case_insensitive_matches(input: &str, mut emit: impl FnMut(usize, usize)) {
    let mut start = 0;
    while start < input.len() {
        let end = advance_gpt2(input, start, true);
        emit(start, end);
        start = end;
    }
}

#[inline(always)]
pub(super) fn stream_gpt2_matches(input: &str, sink: &mut impl FusedPieceSink) {
    mask_scanner::for_each_r50k_match_into(input, sink);
}

#[inline(always)]
fn advance_gpt2(input: &str, start: usize, case_insensitive: bool) -> usize {
    let bytes = input.as_bytes();
    let first = bytes[start];
    if first.is_ascii_alphabetic() {
        return scan_gpt2_letters(input, start + 1);
    }

    if first == b' ' {
        if start + 1 == bytes.len() {
            return start + 1;
        }
        let second = bytes[start + 1];
        if second.is_ascii_alphabetic() {
            return scan_gpt2_letters(input, start + 2);
        }
        if second.is_ascii_digit() {
            return scan_gpt2_numbers(input, start + 2);
        }
        if second >= 128 {
            let unit = unit_at_non_ascii(input, start + 1);
            if unit.is_letter() {
                return scan_gpt2_letters(input, unit.end);
            }
            if unit.is_number() {
                return scan_gpt2_numbers(input, unit.end);
            }
            if unit.is_whitespace() {
                return scan_whitespace(input, start, false).unwrap();
            }
            return scan_gpt2_other(input, unit.end);
        }
        if is_tokenizer_whitespace(second) {
            return scan_whitespace(input, start, false).unwrap();
        }
        return scan_gpt2_other(input, start + 2);
    }

    if first >= 128 {
        let unit = unit_at_non_ascii(input, start);
        if unit.is_letter() {
            return scan_gpt2_letters(input, unit.end);
        }
        if unit.is_number() {
            return scan_gpt2_numbers(input, unit.end);
        }
        if unit.is_whitespace() {
            return scan_whitespace(input, start, false).unwrap();
        }
        return scan_gpt2_other(input, unit.end);
    }
    if first.is_ascii_digit() {
        return scan_gpt2_numbers(input, start + 1);
    }
    if first == b'\''
        && let Some(end) = scan_contraction(input, start, case_insensitive)
    {
        return end;
    }
    if is_tokenizer_whitespace(first) {
        return scan_whitespace(input, start, false).unwrap();
    }
    scan_gpt2_other(input, start + 1)
}

#[inline(always)]
fn scan_gpt2_letters(input: &str, mut pos: usize) -> usize {
    let bytes = input.as_bytes();
    loop {
        pos = scan_ascii_letters(bytes, pos);
        if pos < bytes.len() && bytes[pos] >= 128 {
            let unit = unit_at_non_ascii(input, pos);
            if unit.is_letter() {
                pos = unit.end;
                continue;
            }
        }
        return pos;
    }
}

#[inline(always)]
fn scan_gpt2_numbers(input: &str, mut pos: usize) -> usize {
    let bytes = input.as_bytes();
    loop {
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos < bytes.len() && bytes[pos] >= 128 {
            let unit = unit_at_non_ascii(input, pos);
            if unit.is_number() {
                pos = unit.end;
                continue;
            }
        }
        return pos;
    }
}

#[inline(always)]
fn scan_gpt2_other(input: &str, mut pos: usize) -> usize {
    let bytes = input.as_bytes();
    loop {
        while pos < bytes.len() {
            let byte = bytes[pos];
            if byte >= 128 {
                break;
            }
            if byte.is_ascii_alphanumeric() || is_tokenizer_whitespace(byte) {
                return pos;
            }
            pos += 1;
        }
        if pos < bytes.len() {
            let unit = unit_at_non_ascii(input, pos);
            if !unit.is_letter() && !unit.is_number() && !unit.is_whitespace() {
                pos = unit.end;
                continue;
            }
        }
        return pos;
    }
}

#[inline(always)]
fn scan_ascii_letters(bytes: &[u8], mut pos: usize) -> usize {
    while pos + 8 <= bytes.len() {
        let word = unsafe { (bytes.as_ptr().add(pos) as *const u64).read_unaligned() };
        if word & BYTE_HIGH_BITS != 0 {
            break;
        }

        let lowered = word | 0x2020_2020_2020_2020;
        let above_a = (lowered | BYTE_HIGH_BITS).wrapping_sub(0x6161_6161_6161_6161);
        let below_z = 0xFAFA_FAFA_FAFA_FAFA_u64.wrapping_sub(lowered);
        let non_letter = !(above_a & below_z) & BYTE_HIGH_BITS;
        if non_letter != 0 {
            return pos + non_letter.to_le().trailing_zeros() as usize / 8;
        }
        pos += 8;
    }

    while pos < bytes.len() && bytes[pos].is_ascii_alphabetic() {
        pos += 1;
    }
    pos
}

pub(crate) fn for_each_deepseek_piece(input: &str, mut emit: impl FnMut(usize, usize)) {
    let mut start = 0;
    while start < input.len() {
        let end = advance_deepseek_piece(input, start);
        emit(start, end);
        start = end;
    }
}

#[inline(always)]
fn is_deepseek_cjk(unit: Unit) -> bool {
    matches!(unit.ch as u32, 0x4e00..=0x9fa5 | 0x3040..=0x30ff)
}

#[inline(always)]
fn deepseek_lm_end_at(input: &str, pos: usize, cjk_region: bool) -> Option<usize> {
    if pos >= input.len() {
        return None;
    }
    let unit = unit_at(input, pos);
    (is_deepseek_cjk(unit) == cjk_region && (unit.is_letter() || unit.is_mark()))
        .then_some(unit.end)
}

#[inline(always)]
fn scan_deepseek_lm(input: &str, mut pos: usize, cjk_region: bool) -> usize {
    while pos < input.len() {
        if !cjk_region {
            pos = scan_ascii_letters(input.as_bytes(), pos);
            if pos == input.len() {
                return pos;
            }
        }
        let unit = unit_at(input, pos);
        if is_deepseek_cjk(unit) != cjk_region || !(unit.is_letter() || unit.is_mark()) {
            return pos;
        }
        pos = unit.end;
    }
    pos
}

#[inline(always)]
fn scan_deepseek_punctuation_region(input: &str, mut pos: usize, cjk_region: bool) -> usize {
    while pos < input.len() {
        let byte = input.as_bytes()[pos];
        if !cjk_region && byte.is_ascii() {
            if !byte.is_ascii_punctuation() {
                return pos;
            }
            pos += 1;
            continue;
        }
        let unit = unit_at(input, pos);
        if is_deepseek_cjk(unit) != cjk_region || !unit.is_punctuation_or_symbol() {
            return pos;
        }
        pos = unit.end;
    }
    pos
}

#[inline(always)]
fn scan_crlf(input: &str, mut pos: usize) -> usize {
    while pos < input.len() && matches!(input.as_bytes()[pos], b'\r' | b'\n') {
        pos += 1;
    }
    pos
}

fn scan_deepseek_whitespace(input: &str, start: usize) -> usize {
    let mut pos = start;
    let mut last_start = start;
    let mut last_newline_end = None;
    let mut at_boundary = false;
    while pos < input.len() {
        let unit = unit_at(input, pos);
        if !unit.is_whitespace() {
            at_boundary = unit.is_number() || is_deepseek_cjk(unit);
            break;
        }
        last_start = pos;
        if unit.is_newline() {
            last_newline_end = Some(unit.end);
        }
        pos = unit.end;
    }

    if let Some(end) = last_newline_end {
        return end;
    }
    if pos == input.len() || at_boundary {
        return pos;
    }
    if last_start > start {
        return last_start;
    }
    pos
}

fn scan_deepseek_gap(input: &str, mut pos: usize, cjk_region: bool) -> usize {
    while pos < input.len() {
        let unit = unit_at(input, pos);
        let is_other = !(unit.is_letter()
            || unit.is_mark()
            || unit.is_number()
            || unit.is_punctuation_or_symbol()
            || unit.is_whitespace());
        if is_deepseek_cjk(unit) != cjk_region || !is_other {
            return pos;
        }
        // An otherwise unmatched scalar prefixes the following letter-or-mark run.
        if deepseek_lm_end_at(input, unit.end, cjk_region).is_some() {
            return pos;
        }
        pos = unit.end;
    }
    pos
}

#[inline(always)]
fn advance_deepseek_main(input: &str, start: usize) -> usize {
    let bytes = input.as_bytes();
    let first = bytes[start];
    if first.is_ascii_alphabetic() {
        return scan_deepseek_lm(input, start + 1, false);
    }

    if first == b' ' {
        let Some(&second) = bytes.get(start + 1) else {
            return start + 1;
        };
        if second.is_ascii_alphabetic() {
            return scan_deepseek_lm(input, start + 2, false);
        }
        if second.is_ascii_digit() {
            return start + 1;
        }
        if second.is_ascii() {
            if second.is_ascii_punctuation() {
                let end = scan_deepseek_punctuation_region(input, start + 2, false);
                return scan_crlf(input, end);
            }
            return if is_tokenizer_whitespace(second) {
                scan_deepseek_whitespace(input, start)
            } else {
                start + 1
            };
        }

        let unit = unit_at_non_ascii(input, start + 1);
        if is_deepseek_cjk(unit) {
            return start + 1;
        }
        if unit.is_letter() || unit.is_mark() {
            return scan_deepseek_lm(input, unit.end, false);
        }
        if unit.is_number() {
            return start + 1;
        }
        if unit.is_whitespace() {
            return scan_deepseek_whitespace(input, start);
        }
        if unit.is_punctuation_or_symbol() {
            let end = scan_deepseek_punctuation_region(input, unit.end, false);
            return scan_crlf(input, end);
        }
        return start + 1;
    }

    if first.is_ascii() {
        if first == b'\r' || first == b'\n' {
            return scan_deepseek_whitespace(input, start);
        }
        if is_tokenizer_whitespace(first) {
            if let Some(end) = deepseek_lm_end_at(input, start + 1, false) {
                return scan_deepseek_lm(input, end, false);
            }
            return scan_deepseek_whitespace(input, start);
        }
        if first.is_ascii_punctuation() {
            if bytes.get(start + 1).is_some_and(u8::is_ascii_alphabetic) {
                return scan_ascii_letters(bytes, start + 1);
            }
            let end = scan_deepseek_punctuation_region(input, start + 1, false);
            return scan_crlf(input, end);
        }
        if let Some(end) = deepseek_lm_end_at(input, start + 1, false) {
            return scan_deepseek_lm(input, end, false);
        }
        return scan_deepseek_gap(input, start + 1, false);
    }

    let unit = unit_at_non_ascii(input, start);
    if unit.is_letter() || unit.is_mark() {
        return scan_deepseek_lm(input, unit.end, false);
    }
    if unit.is_whitespace() {
        if let Some(end) = deepseek_lm_end_at(input, unit.end, false) {
            return scan_deepseek_lm(input, end, false);
        }
        return scan_deepseek_whitespace(input, start);
    }
    if unit.is_punctuation_or_symbol() {
        let end = scan_deepseek_punctuation_region(input, unit.end, false);
        return scan_crlf(input, end);
    }
    if let Some(end) = deepseek_lm_end_at(input, unit.end, false) {
        return scan_deepseek_lm(input, end, false);
    }
    scan_deepseek_gap(input, unit.end, false)
}

fn advance_deepseek_cjk(input: &str, start: usize) -> usize {
    let unit = unit_at_non_ascii(input, start);
    if unit.is_letter() || unit.is_mark() {
        return scan_deepseek_lm(input, unit.end, true);
    }
    if unit.is_punctuation_or_symbol() {
        return scan_deepseek_punctuation_region(input, unit.end, true);
    }
    if let Some(end) = deepseek_lm_end_at(input, unit.end, true) {
        return scan_deepseek_lm(input, end, true);
    }
    scan_deepseek_gap(input, unit.end, true)
}

#[inline(always)]
fn advance_deepseek_piece(input: &str, start: usize) -> usize {
    let first = input.as_bytes()[start];
    if first.is_ascii() {
        // Only ASCII digits can match the first number-isolation stage.
        if first.is_ascii_digit() {
            return scan_number::<3>(input, start).expect("number start was classified");
        }
        return advance_deepseek_main(input, start);
    }

    let unit = unit_at_non_ascii(input, start);
    if unit.is_number() {
        return scan_number::<3>(input, start).expect("number start was classified");
    }
    if is_deepseek_cjk(unit) {
        return advance_deepseek_cjk(input, start);
    }
    advance_deepseek_main(input, start)
}

fn scan_deepseek_main(input: &str, pos: usize) -> Option<usize> {
    if let Some(end) = scan_ascii_punctuation_word(input, pos) {
        return Some(end);
    }

    let first = unit_at(input, pos);
    let has_prefix = !first.is_newline() && !first.is_letter() && !first.is_punctuation_or_symbol();
    let word_start = if has_prefix { first.end } else { pos };
    let word_predicate = |unit: Unit| unit.is_letter() || unit.is_mark();

    let mut word_end = scan_run(input, word_start, word_predicate);
    if word_end.is_none() && has_prefix {
        word_end = scan_run(input, pos, word_predicate);
    }
    if word_end.is_some() {
        return word_end;
    }

    scan_deepseek_punctuation(input, pos).or_else(|| scan_whitespace(input, pos, true))
}

fn scan_deepseek_cjk_matches(input: &str, mut emit: impl FnMut(usize, usize)) {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos].is_ascii() {
            pos += 1;
            continue;
        }

        let unit = unit_at_non_ascii(input, pos);
        let is_cjk = matches!(
            unit.ch,
            '\u{4e00}'..='\u{9fa5}' | '\u{3040}'..='\u{309f}' | '\u{30a0}'..='\u{30ff}'
        );
        if !is_cjk {
            pos = unit.end;
            continue;
        }

        let start = pos;
        pos = unit.end;
        while pos < bytes.len() && !bytes[pos].is_ascii() {
            let unit = unit_at_non_ascii(input, pos);
            let is_cjk = matches!(
                unit.ch,
                '\u{4e00}'..='\u{9fa5}' | '\u{3040}'..='\u{309f}' | '\u{30a0}'..='\u{30ff}'
            );
            if !is_cjk {
                break;
            }
            pos = unit.end;
        }
        emit(start, pos);
    }
}

fn scan_ascii_punctuation_word(input: &str, pos: usize) -> Option<usize> {
    let punctuation = unit_at(input, pos);
    if !punctuation.ch.is_ascii_punctuation() {
        return None;
    }
    scan_run(input, punctuation.end, |unit| unit.ch.is_ascii_alphabetic())
}

fn scan_contraction(input: &str, pos: usize, case_insensitive: bool) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.get(pos) != Some(&b'\'') {
        return None;
    }

    let first = *bytes.get(pos + 1)?;
    let folded_first = if case_insensitive {
        first.to_ascii_lowercase()
    } else {
        first
    };
    if matches!(folded_first, b's' | b't' | b'm' | b'd') {
        return Some(pos + 2);
    }

    let second = *bytes.get(pos + 2)?;
    let folded_second = if case_insensitive {
        second.to_ascii_lowercase()
    } else {
        second
    };
    let is_long = matches!(
        (folded_first, folded_second),
        (b'r', b'e') | (b'v', b'e') | (b'l', b'l')
    );
    is_long.then_some(pos + 3)
}

fn scan_number<const LIMIT: usize>(input: &str, pos: usize) -> Option<usize> {
    let mut cursor = pos;
    let mut count = 0usize;
    while cursor < input.len() && count < LIMIT {
        let unit = unit_at(input, cursor);
        if !unit.is_number() {
            break;
        }
        cursor = unit.end;
        count += 1;
    }
    (cursor > pos).then_some(cursor)
}

fn scan_punctuation(
    input: &str,
    pos: usize,
    exclude_marks: bool,
    suffix_newlines: bool,
    suffix_slash: bool,
) -> Option<usize> {
    let mut cursor = if input.as_bytes()[pos] == b' ' {
        pos + 1
    } else {
        pos
    };
    let body_start = cursor;

    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        let excluded = unit.is_whitespace()
            || unit.is_letter()
            || unit.is_number()
            || (exclude_marks && unit.is_mark());
        if excluded {
            break;
        }
        cursor = unit.end;
    }
    if cursor == body_start {
        return None;
    }

    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !(suffix_newlines && unit.is_newline()) && !(suffix_slash && unit.ch == '/') {
            break;
        }
        cursor = unit.end;
    }
    Some(cursor)
}

fn scan_deepseek_punctuation(input: &str, pos: usize) -> Option<usize> {
    let mut cursor = if input.as_bytes()[pos] == b' ' {
        pos + 1
    } else {
        pos
    };
    let body_start = cursor;

    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !unit.is_punctuation_or_symbol() {
            break;
        }
        cursor = unit.end;
    }
    if cursor == body_start {
        return None;
    }

    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !unit.is_newline() {
            break;
        }
        cursor = unit.end;
    }
    Some(cursor)
}

fn scan_whitespace(input: &str, pos: usize, newline_alternative: bool) -> Option<usize> {
    if !unit_at(input, pos).is_whitespace() {
        return None;
    }

    let mut cursor = pos;
    let mut last_start = pos;
    let mut last_newline_end = None;
    let mut count = 0usize;
    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !unit.is_whitespace() {
            break;
        }
        last_start = cursor;
        if unit.is_newline() {
            last_newline_end = Some(unit.end);
        }
        cursor = unit.end;
        count += 1;
    }

    if newline_alternative && let Some(end) = last_newline_end {
        return Some(end);
    }
    if cursor == input.len() {
        return Some(cursor);
    }
    if count >= 2 {
        return Some(last_start);
    }
    Some(cursor)
}

fn scan_run(input: &str, pos: usize, predicate: impl Fn(Unit) -> bool) -> Option<usize> {
    let mut cursor = pos;
    while cursor < input.len() {
        let unit = unit_at(input, cursor);
        if !predicate(unit) {
            break;
        }
        cursor = unit.end;
    }
    (cursor > pos).then_some(cursor)
}

#[cfg(test)]
mod tests;
