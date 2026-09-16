use std::collections::HashMap;
use std::fmt;

use daachorse::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder};
use icu_properties::{
    CodePointMapData, CodePointSetData,
    props::{Alphabetic, GeneralCategory, JoinControl},
};

use crate::{json_structs::AddedTokenConfig, normalizers::Normalizer};

struct AddedTokenMatcher {
    daac: DoubleArrayAhoCorasick<u32>,
    start_bytes: Vec<u8>,
    shared_prefix: Option<[u8; 2]>,
    max_token_len: usize,
}

impl AddedTokenMatcher {
    fn new(patterns: Vec<(String, u32)>) -> Result<Option<Self>, String> {
        if patterns.is_empty() {
            return Ok(None);
        }

        let mut start_set = [false; 256];
        let mut shared_prefix = None;
        let mut max_token_len = 0;
        for (index, (content, _)) in patterns.iter().enumerate() {
            let bytes = content.as_bytes();
            if let Some(&byte) = bytes.first() {
                start_set[byte as usize] = true;
            }
            let prefix = bytes.get(..2).map(|bytes| [bytes[0], bytes[1]]);
            if index == 0 {
                shared_prefix = prefix;
            } else if shared_prefix != prefix {
                shared_prefix = None;
            }
            max_token_len = max_token_len.max(content.len());
        }
        let start_bytes = start_set
            .iter()
            .enumerate()
            .filter(|&(_, present)| *present)
            .map(|(byte, _)| byte as u8)
            .collect();
        let daac = DoubleArrayAhoCorasickBuilder::new()
            .match_kind(daachorse::MatchKind::LeftmostLongest)
            .build_with_values(patterns.iter().map(|(content, id)| (content.as_str(), *id)))
            .map_err(|error| format!("error building added-tokens DAAC: {error}"))?;
        Ok(Some(Self {
            daac,
            start_bytes,
            shared_prefix,
            max_token_len,
        }))
    }

    #[inline]
    fn has_candidate(&self, input: &str) -> bool {
        let bytes = input.as_bytes();
        if let Some(prefix) = self.shared_prefix {
            return memchr::memchr_iter(prefix[0], bytes)
                .any(|index| bytes.get(index + 1) == Some(&prefix[1]));
        }
        match self.start_bytes.as_slice() {
            [first] => memchr::memchr(*first, bytes).is_some(),
            [first, second] => memchr::memchr2(*first, *second, bytes).is_some(),
            [first, second, third] => memchr::memchr3(*first, *second, *third, bytes).is_some(),
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct AddedTokenFlags(u8);

impl AddedTokenFlags {
    const SINGLE_WORD: u8 = 1;
    const LSTRIP: u8 = 2;
    const RSTRIP: u8 = 4;
    const SPECIAL: u8 = 8;

    fn from_config(config: &AddedTokenConfig) -> Self {
        Self(
            u8::from(config.single_word)
                | (u8::from(config.lstrip) << 1)
                | (u8::from(config.rstrip) << 2)
                | (u8::from(config.special) << 3),
        )
    }

    fn contains(self, flag: u8) -> bool {
        self.0 & flag != 0
    }
}

/// Added tokens declared outside the model vocabulary.
pub struct AddedTokens {
    non_normalized: Option<AddedTokenMatcher>,
    normalized: Option<AddedTokenMatcher>,
    normalized_patterns: Vec<(String, u32)>,
    flags: Vec<AddedTokenFlags>,
    id_to_content: HashMap<u32, String>,
    content_to_id: HashMap<String, u32>,
}

/// One item emitted while splitting text around added tokens.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    /// An added token ID that bypasses model encoding.
    Token(u32),
    /// Text that remains available for ordinary tokenization.
    Text(&'a str),
}

/// Metadata for one added token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddedTokenInfo<'a> {
    /// The token ID.
    pub id: u32,
    /// The configured token text.
    pub content: &'a str,
    /// Whether the token is marked special in the tokenizer configuration.
    pub special: bool,
}

impl AddedTokens {
    /// Builds an added-token table from tokenizer JSON configuration.
    pub fn from_configs(configs: &[AddedTokenConfig]) -> Result<Option<Self>, String> {
        Self::from_configs_with_normalizer(configs, None)
    }

    pub(crate) fn from_configs_with_normalizer(
        configs: &[AddedTokenConfig],
        normalizer: Option<&Normalizer>,
    ) -> Result<Option<Self>, String> {
        if configs.is_empty() {
            return Ok(None);
        }

        let max_id = configs.iter().map(|c| c.id).max().unwrap_or(0);
        const MAX_REASONABLE_ID: u32 = 16 * 1024 * 1024;
        if max_id > MAX_REASONABLE_ID {
            return Err(format!(
                "added token ID {max_id} exceeds maximum supported ID {MAX_REASONABLE_ID}"
            ));
        }
        let mut flags = vec![AddedTokenFlags::default(); (max_id + 1) as usize];
        let mut id_to_content = HashMap::with_capacity(configs.len());
        let mut content_to_id = HashMap::with_capacity(configs.len());
        let mut non_normalized_patterns = Vec::new();
        let mut normalized_patterns = Vec::new();

        for config in configs {
            flags[config.id as usize] = AddedTokenFlags::from_config(config);
            id_to_content.insert(config.id, config.content.clone());
            content_to_id.insert(config.content.clone(), config.id);

            if config.normalized {
                normalized_patterns.push((config.content.clone(), config.id));
            } else {
                non_normalized_patterns.push((config.content.clone(), config.id));
            }
        }

        let mut added_tokens = Self {
            non_normalized: AddedTokenMatcher::new(non_normalized_patterns)?,
            normalized: None,
            normalized_patterns,
            flags,
            id_to_content,
            content_to_id,
        };
        added_tokens.set_normalizer(normalizer)?;
        Ok(Some(added_tokens))
    }

    pub(crate) fn set_normalizer(&mut self, normalizer: Option<&Normalizer>) -> Result<(), String> {
        let patterns = self
            .normalized_patterns
            .iter()
            .map(|(content, id)| {
                let normalized = normalizer.map_or_else(
                    || content.clone(),
                    |normalizer| normalizer.normalize(content).into_owned(),
                );
                (normalized, *id)
            })
            .collect();
        self.normalized = AddedTokenMatcher::new(patterns)?;
        Ok(())
    }

    /// Returns the configured text for an added-token ID.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.id_to_content.get(&id).map(String::as_str)
    }

    /// Returns the ID of an added token with exactly this text.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.content_to_id.get(token).copied()
    }

    /// Returns whether an added-token ID is marked special.
    pub fn is_special(&self, id: u32) -> bool {
        self.flags
            .get(id as usize)
            .is_some_and(|flags| flags.contains(AddedTokenFlags::SPECIAL))
    }

    /// Returns the number of configured added tokens.
    pub fn len(&self) -> usize {
        self.id_to_content.len()
    }

    /// Returns whether no added tokens are configured.
    pub fn is_empty(&self) -> bool {
        self.id_to_content.is_empty()
    }

    /// The iteration order is unspecified. Callers that need a stable order
    /// should sort by `id` themselves.
    pub fn iter(&self) -> impl Iterator<Item = AddedTokenInfo<'_>> {
        self.id_to_content
            .iter()
            .map(|(&id, content)| AddedTokenInfo {
                id,
                content: content.as_str(),
                special: self.flags[id as usize].contains(AddedTokenFlags::SPECIAL),
            })
    }

    /// Splits text into added-token IDs and ordinary text spans.
    pub fn split<'a>(&self, input: &'a str) -> Vec<Segment<'a>> {
        let raw_segments = self.split_non_normalized(input);
        if self.normalized.is_none() {
            return raw_segments;
        }

        let mut segments = Vec::with_capacity(raw_segments.len());
        for segment in raw_segments {
            match segment {
                Segment::Token(id) => segments.push(Segment::Token(id)),
                Segment::Text(text) => segments.extend(self.split_normalized(text)),
            }
        }
        segments
    }

    pub(crate) fn split_non_normalized<'a>(&self, input: &'a str) -> Vec<Segment<'a>> {
        self.split_with(input, self.non_normalized.as_ref())
    }

    pub(crate) fn split_normalized<'a>(&self, input: &'a str) -> Vec<Segment<'a>> {
        self.split_with(input, self.normalized.as_ref())
    }

    pub(crate) fn has_normalized(&self) -> bool {
        self.normalized.is_some()
    }

    pub(crate) fn has_candidate(&self, input: &str) -> bool {
        self.non_normalized
            .as_ref()
            .is_some_and(|matcher| matcher.has_candidate(input))
            || self
                .normalized
                .as_ref()
                .is_some_and(|matcher| matcher.has_candidate(input))
    }

    fn split_with<'a>(
        &self,
        input: &'a str,
        matcher: Option<&AddedTokenMatcher>,
    ) -> Vec<Segment<'a>> {
        let Some(matcher) = matcher else {
            return if input.is_empty() {
                Vec::new()
            } else {
                vec![Segment::Text(input)]
            };
        };

        match matcher.start_bytes.len() {
            1 => self.split_prefilter(
                input,
                matcher,
                memchr::memchr_iter(matcher.start_bytes[0], input.as_bytes()),
            ),
            2 => self.split_prefilter(
                input,
                matcher,
                memchr::memchr2_iter(
                    matcher.start_bytes[0],
                    matcher.start_bytes[1],
                    input.as_bytes(),
                ),
            ),
            3 => self.split_prefilter(
                input,
                matcher,
                memchr::memchr3_iter(
                    matcher.start_bytes[0],
                    matcher.start_bytes[1],
                    matcher.start_bytes[2],
                    input.as_bytes(),
                ),
            ),
            _ => self.split_full_scan(input, matcher),
        }
    }

    /// Probe candidate starts without truncating a UTF-8 character at the window end.
    fn split_prefilter<'a>(
        &self,
        input: &'a str,
        matcher: &AddedTokenMatcher,
        candidates: impl Iterator<Item = usize>,
    ) -> Vec<Segment<'a>> {
        let mut segments = Vec::new();
        let mut prev_match_end = 0;
        let mut prev_end = 0;

        for pos in candidates {
            if pos < prev_match_end {
                continue;
            }
            let window_end = input.ceil_char_boundary(pos + matcher.max_token_len);
            let window = &input[pos..window_end];
            if let Some(m) = matcher.daac.leftmost_find_iter(window).next()
                && m.start() == 0
            {
                let match_end = pos + m.end();
                prev_match_end = match_end;
                if !self.is_single_word_match(input, m.value(), pos, match_end) {
                    continue;
                }
                let (start, end) = self.strip_bounds(input, m.value(), pos, match_end, prev_end);
                if start > prev_end {
                    segments.push(Segment::Text(&input[prev_end..start]));
                }
                segments.push(Segment::Token(m.value()));
                prev_end = end;
            }
        }

        if prev_end < input.len() {
            segments.push(Segment::Text(&input[prev_end..]));
        }

        segments
    }

    fn is_single_word_match(&self, input: &str, id: u32, start: usize, end: usize) -> bool {
        if !self.flags[id as usize].contains(AddedTokenFlags::SINGLE_WORD) {
            return true;
        }

        let starts_at_boundary = input[..start]
            .chars()
            .next_back()
            .is_none_or(|ch| !is_word_character(ch));
        let ends_at_boundary = input[end..]
            .chars()
            .next()
            .is_none_or(|ch| !is_word_character(ch));
        starts_at_boundary && ends_at_boundary
    }

    /// Absorb Unicode whitespace without crossing a previously emitted token.
    fn strip_bounds(
        &self,
        input: &str,
        id: u32,
        mut start: usize,
        mut end: usize,
        floor: usize,
    ) -> (usize, usize) {
        let flags = self.flags[id as usize];
        if flags.contains(AddedTokenFlags::LSTRIP) {
            start = start.max(floor);
            start = floor + input[floor..start].trim_end().len();
        }
        if flags.contains(AddedTokenFlags::RSTRIP) {
            end = input.len() - input[end..].trim_start().len();
        }
        (start, end)
    }

    fn split_full_scan<'a>(&self, input: &'a str, matcher: &AddedTokenMatcher) -> Vec<Segment<'a>> {
        let mut segments = Vec::new();
        let mut prev_end = 0;

        for m in matcher.daac.leftmost_find_iter(input) {
            if !self.is_single_word_match(input, m.value(), m.start(), m.end()) {
                continue;
            }
            let (start, end) = self.strip_bounds(input, m.value(), m.start(), m.end(), prev_end);
            if start > prev_end {
                segments.push(Segment::Text(&input[prev_end..start]));
            }
            segments.push(Segment::Token(m.value()));
            prev_end = end;
        }

        if prev_end < input.len() {
            segments.push(Segment::Text(&input[prev_end..]));
        }

        segments
    }
}

fn is_word_character(ch: char) -> bool {
    if ch.is_ascii() {
        return ch.is_ascii_alphanumeric() || ch == '_';
    }
    if CodePointSetData::new::<Alphabetic>().contains(ch)
        || CodePointSetData::new::<JoinControl>().contains(ch)
    {
        return true;
    }

    matches!(
        CodePointMapData::<GeneralCategory>::new().get(ch),
        GeneralCategory::NonspacingMark
            | GeneralCategory::SpacingMark
            | GeneralCategory::EnclosingMark
            | GeneralCategory::DecimalNumber
            | GeneralCategory::ConnectorPunctuation
    )
}

impl fmt::Debug for AddedTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddedTokens")
            .field("count", &self.id_to_content.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Segment::{Text, Token};

    fn make_config(id: u32, content: &str) -> AddedTokenConfig {
        AddedTokenConfig {
            id,
            content: content.to_string(),
            single_word: false,
            lstrip: false,
            rstrip: false,
            normalized: false,
            special: false,
        }
    }

    #[test]
    fn splits_preserve_text_and_added_token_boundaries() {
        let configs = [make_config(1, "<a>"), make_config(2, "<b>")];
        let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
        for (input, expected) in [
            ("", vec![]),
            ("hello world", vec![Text("hello world")]),
            ("<a>", vec![Token(1)]),
            ("<a>hello", vec![Token(1), Text("hello")]),
            ("hello<a>", vec![Text("hello"), Token(1)]),
            (
                "hello<a>world",
                vec![Text("hello"), Token(1), Text("world")],
            ),
            (
                "prefix <a> suffix",
                vec![Text("prefix "), Token(1), Text(" suffix")],
            ),
            (
                "x<a>y<b>z",
                vec![Text("x"), Token(1), Text("y"), Token(2), Text("z")],
            ),
            ("<a><b>", vec![Token(1), Token(2)]),
            ("<a><a><a>", vec![Token(1), Token(1), Token(1)]),
        ] {
            assert_eq!(added.split(input), expected, "{input:?}");
        }
        assert!(AddedTokens::from_configs(&[]).unwrap().is_none());
    }

    #[test]
    fn shared_prefixes_choose_the_longest_token() {
        let configs = [
            make_config(1, "<"),
            make_config(2, "<file>"),
            make_config(3, "<filename>"),
        ];
        let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            added.split("x<filename>y<file>z<"),
            [
                Text("x"),
                Token(3),
                Text("y"),
                Token(2),
                Text("z"),
                Token(1)
            ]
        );
    }

    #[test]
    fn unicode_and_distinct_start_bytes_match() {
        for spellings in [
            ["▁", "Ġ", "日本語", "🌍"],
            ["<bos>", "[SEP]", "{pad}", "|mask|"],
        ] {
            let configs: Vec<_> = spellings
                .iter()
                .enumerate()
                .map(|(id, text)| make_config(id as u32, text))
                .collect();
            for count in 1..=configs.len() {
                let added = AddedTokens::from_configs(&configs[..count])
                    .unwrap()
                    .unwrap();
                for (id, text) in spellings[..count].iter().enumerate() {
                    let input = format!("hello {text} world");
                    assert_eq!(
                        added.split(&input),
                        [Text("hello "), Token(id as u32), Text(" world")]
                    );
                    assert_eq!(added.token_to_id(text), Some(id as u32));
                }
                let expected: Vec<_> = (0..count).map(|id| Token(id as u32)).collect();
                assert_eq!(added.split(&spellings[..count].concat()), expected);
            }
        }
    }

    #[test]
    fn vocabulary_access_preserves_ids_and_special_flags() {
        let mut special = make_config(10, "<bos>");
        special.special = true;
        let configs = [special, make_config(11, "<eos>"), make_config(12, "<pad>")];
        let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(added.len(), configs.len());
        assert!(!added.is_empty());
        for config in &configs {
            assert_eq!(added.token_to_id(&config.content), Some(config.id));
            assert_eq!(added.id_to_token(config.id), Some(config.content.as_str()));
            assert_eq!(added.is_special(config.id), config.special);
        }
        assert_eq!(added.token_to_id("<unknown>"), None);
        assert_eq!(added.id_to_token(99), None);
        assert!(!added.is_special(99));
        let mut entries: Vec<_> = added.iter().collect();
        entries.sort_by_key(|entry| entry.id);
        let expected: Vec<_> = configs
            .iter()
            .map(|config| AddedTokenInfo {
                id: config.id,
                content: &config.content,
                special: config.special,
            })
            .collect();
        assert_eq!(entries, expected);
    }

    #[test]
    fn single_word_respects_ascii_and_unicode_boundaries() {
        let mut mask = make_config(1, "<mask>");
        mask.single_word = true;
        let input = "<mask>, <mask>- ◌̰<mask> A<mask> <mask>";
        let expected = vec![
            Segment::Token(1),
            Segment::Text(", "),
            Segment::Token(1),
            Segment::Text("- ◌̰<mask> A<mask> "),
            Segment::Token(1),
        ];

        let prefiltered = AddedTokens::from_configs(&[mask.clone()]).unwrap().unwrap();
        assert_eq!(prefiltered.split(input), expected);

        let full_scan = AddedTokens::from_configs(&[
            mask,
            make_config(2, "[unused]"),
            make_config(3, "{unused}"),
            make_config(4, "|unused|"),
        ])
        .unwrap()
        .unwrap();
        assert_eq!(full_scan.split(input), expected);
    }

    fn make_strip_config(id: u32, content: &str, lstrip: bool, rstrip: bool) -> AddedTokenConfig {
        AddedTokenConfig {
            id,
            content: content.to_string(),
            single_word: false,
            lstrip,
            rstrip,
            normalized: false,
            special: true,
        }
    }

    #[test]
    fn stripping_absorbs_only_the_requested_whitespace() {
        for (left, right, input, expected) in [
            (
                true,
                false,
                "ab   <s>cd",
                vec![Text("ab"), Token(1), Text("cd")],
            ),
            (
                false,
                true,
                "ab<s>   cd",
                vec![Text("ab"), Token(1), Text("cd")],
            ),
            (
                true,
                true,
                "ab \t <s> \n cd",
                vec![Text("ab"), Token(1), Text("cd")],
            ),
            (
                false,
                false,
                "ab <s> cd",
                vec![Text("ab "), Token(1), Text(" cd")],
            ),
        ] {
            let added = AddedTokens::from_configs(&[make_strip_config(1, "<s>", left, right)])
                .unwrap()
                .unwrap();
            assert_eq!(added.split(input), expected, "left={left}, right={right}");
        }
    }

    #[test]
    fn adjacent_strip_tokens_share_whitespace() {
        let configs = vec![
            make_strip_config(1, "<|im_end|>", true, true),
            make_strip_config(2, "<|im_start|>", true, true),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("hi<|im_end|>\n<|im_start|>user"),
            vec![
                Segment::Text("hi"),
                Segment::Token(1),
                Segment::Token(2),
                Segment::Text("user"),
            ]
        );
    }

    #[test]
    fn lstrip_bounded_by_previous_token() {
        let configs = vec![
            make_strip_config(1, "<a>", false, false),
            make_strip_config(2, "<b>", true, false),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("<a><b>"),
            vec![Segment::Token(1), Segment::Token(2)]
        );
    }

    #[test]
    fn strip_via_full_scan_path() {
        let configs = vec![
            make_strip_config(1, "<bos>", false, true),
            make_config(2, "[SEP]"),
            make_config(3, "{pad}"),
            make_config(4, "|mask|"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("<bos>   x"),
            vec![Segment::Token(1), Segment::Text("x")]
        );
    }

    #[test]
    fn strip_overlap_matches_huggingface_in_both_paths() {
        let prefilter_configs = vec![
            make_strip_config(1, "<a>", false, true),
            make_strip_config(2, " X", true, false),
            make_config(3, "[d]"),
        ];
        let prefiltered = AddedTokens::from_configs(&prefilter_configs)
            .unwrap()
            .unwrap();

        let mut full_scan_configs = prefilter_configs;
        full_scan_configs.push(make_config(4, "{e}"));
        let full_scan = AddedTokens::from_configs(&full_scan_configs)
            .unwrap()
            .unwrap();

        let expected = vec![Segment::Token(1), Segment::Token(2)];
        assert_eq!(prefiltered.split("<a> X"), expected);
        assert_eq!(full_scan.split("<a> X"), expected);
    }
}
