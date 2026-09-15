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

/// Added tokens declared outside the BPE vocabulary.
pub struct AddedTokens {
    non_normalized: Option<AddedTokenMatcher>,
    normalized: Option<AddedTokenMatcher>,
    flags: Vec<AddedTokenFlags>,
    id_to_content: HashMap<u32, String>,
    content_to_id: HashMap<String, u32>,
}

/// One item emitted while splitting text around added tokens.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    /// An added token ID that bypasses BPE encoding.
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
                let content = normalizer.map_or_else(
                    || config.content.clone(),
                    |normalizer| normalizer.normalize(&config.content).into_owned(),
                );
                normalized_patterns.push((content, config.id));
            } else {
                non_normalized_patterns.push((config.content.clone(), config.id));
            }
        }

        Ok(Some(Self {
            non_normalized: AddedTokenMatcher::new(non_normalized_patterns)?,
            normalized: AddedTokenMatcher::new(normalized_patterns)?,
            flags,
            id_to_content,
            content_to_id,
        }))
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
            let mut window_end = (pos + matcher.max_token_len).min(input.len());
            while window_end < input.len() && !input.is_char_boundary(window_end) {
                window_end += 1;
            }
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
            for (rel_i, c) in input[floor..start].char_indices().rev() {
                if c.is_whitespace() {
                    start = floor + rel_i;
                } else {
                    break;
                }
            }
        }
        if flags.contains(AddedTokenFlags::RSTRIP) {
            for c in input[end..].chars() {
                if c.is_whitespace() {
                    end += c.len_utf8();
                } else {
                    break;
                }
            }
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

    #[test]
    fn packed_flags_preserve_all_combinations() {
        assert_eq!(std::mem::size_of::<AddedTokenFlags>(), 1);
        for bits in 0u8..16 {
            let mut config = make_config(0, "x");
            config.single_word = bits & 1 != 0;
            config.lstrip = bits & 2 != 0;
            config.rstrip = bits & 4 != 0;
            config.special = bits & 8 != 0;
            let flags = AddedTokenFlags::from_config(&config);
            assert_eq!(
                flags.contains(AddedTokenFlags::SINGLE_WORD),
                config.single_word
            );
            assert_eq!(flags.contains(AddedTokenFlags::LSTRIP), config.lstrip);
            assert_eq!(flags.contains(AddedTokenFlags::RSTRIP), config.rstrip);
            assert_eq!(flags.contains(AddedTokenFlags::SPECIAL), config.special);
        }
    }

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
    fn empty_configs() {
        let result = AddedTokens::from_configs(&[]).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn no_match() {
        let configs = vec![make_config(100, "<special>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("hello world");
        assert_eq!(segs, vec![Segment::Text("hello world")]);
    }

    #[test]
    fn single_match_at_start() {
        let configs = vec![make_config(100, "<s>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("<s>hello");
        assert_eq!(segs, vec![Segment::Token(100), Segment::Text("hello")]);
    }

    #[test]
    fn single_match_at_end() {
        let configs = vec![make_config(100, "</s>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("hello</s>");
        assert_eq!(segs, vec![Segment::Text("hello"), Segment::Token(100)]);
    }

    #[test]
    fn match_in_middle() {
        let configs = vec![make_config(42, "<sep>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("hello<sep>world");
        assert_eq!(
            segs,
            vec![
                Segment::Text("hello"),
                Segment::Token(42),
                Segment::Text("world"),
            ]
        );
    }

    #[test]
    fn multiple_matches() {
        let configs = vec![make_config(1, "<a>"), make_config(2, "<b>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("x<a>y<b>z");
        assert_eq!(
            segs,
            vec![
                Segment::Text("x"),
                Segment::Token(1),
                Segment::Text("y"),
                Segment::Token(2),
                Segment::Text("z"),
            ]
        );
    }

    #[test]
    fn adjacent_matches() {
        let configs = vec![make_config(1, "<a>"), make_config(2, "<b>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("<a><b>");
        assert_eq!(segs, vec![Segment::Token(1), Segment::Token(2)]);
    }

    #[test]
    fn longest_match_wins() {
        let configs = vec![make_config(1, "<file>"), make_config(2, "<filename>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("a<filename>b");
        assert_eq!(
            segs,
            vec![Segment::Text("a"), Segment::Token(2), Segment::Text("b"),]
        );
    }

    #[test]
    fn entire_input_is_added_token() {
        let configs = vec![make_config(99, "hello")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("hello");
        assert_eq!(segs, vec![Segment::Token(99)]);
    }

    #[test]
    fn empty_input() {
        let configs = vec![make_config(1, "<s>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("");
        assert!(segs.is_empty());
    }

    #[test]
    fn token_to_id_finds_added_token() {
        let configs = vec![make_config(42, "<special>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(at.token_to_id("<special>"), Some(42));
    }

    #[test]
    fn token_to_id_returns_none_for_unknown() {
        let configs = vec![make_config(1, "<known>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(at.token_to_id("<unknown>"), None);
    }

    #[test]
    fn token_to_id_and_id_to_token_are_inverses() {
        let configs = vec![
            make_config(10, "<bos>"),
            make_config(11, "<eos>"),
            make_config(12, "<pad>"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        for cfg in &configs {
            let id = at.token_to_id(&cfg.content).unwrap();
            assert_eq!(id, cfg.id);
            assert_eq!(at.id_to_token(id), Some(cfg.content.as_str()));
        }
    }

    #[test]
    fn unicode_token_content() {
        let configs = vec![
            make_config(1, "▁"),
            make_config(2, "Ġ"),
            make_config(3, "日本語"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("▁hello"),
            vec![Segment::Token(1), Segment::Text("hello")]
        );
        assert_eq!(
            at.split("Ġworld"),
            vec![Segment::Token(2), Segment::Text("world")]
        );
        assert_eq!(
            at.split("日本語text"),
            vec![Segment::Token(3), Segment::Text("text")]
        );
        assert_eq!(at.token_to_id("▁"), Some(1));
        assert_eq!(at.token_to_id("Ġ"), Some(2));
        assert_eq!(at.token_to_id("日本語"), Some(3));
    }

    #[test]
    fn emoji_token_content() {
        let configs = vec![make_config(7, "🌍")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("hello 🌍 world"),
            vec![
                Segment::Text("hello "),
                Segment::Token(7),
                Segment::Text(" world"),
            ]
        );
    }

    #[test]
    fn is_special_only_for_marked_tokens() {
        let mut special = make_config(1, "<bos>");
        special.special = true;
        let non_special = make_config(2, "<extra>");
        let at = AddedTokens::from_configs(&[special, non_special])
            .unwrap()
            .unwrap();
        assert!(at.is_special(1));
        assert!(!at.is_special(2));
        assert!(!at.is_special(99));
    }

    #[test]
    fn iter_exposes_id_content_and_special_flag() {
        let mut special = make_config(1, "<bos>");
        special.special = true;
        let plain = make_config(2, "<extra>");
        let at = AddedTokens::from_configs(&[special, plain])
            .unwrap()
            .unwrap();

        let mut entries: Vec<_> = at.iter().collect();
        entries.sort_by_key(|entry| entry.id);

        assert_eq!(
            entries,
            vec![
                AddedTokenInfo {
                    id: 1,
                    content: "<bos>",
                    special: true,
                },
                AddedTokenInfo {
                    id: 2,
                    content: "<extra>",
                    special: false,
                },
            ]
        );
    }

    #[test]
    fn len_returns_token_count() {
        let configs = vec![
            make_config(1, "<a>"),
            make_config(2, "<b>"),
            make_config(3, "<c>"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(at.len(), 3);
        assert!(!at.is_empty());
    }

    #[test]
    fn three_tokens_with_shared_start_byte() {
        let configs = vec![
            make_config(1, "<"),
            make_config(2, "<s>"),
            make_config(3, "<sep>"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("x<sep>y<s>z<");
        assert_eq!(
            segs,
            vec![
                Segment::Text("x"),
                Segment::Token(3),
                Segment::Text("y"),
                Segment::Token(2),
                Segment::Text("z"),
                Segment::Token(1),
            ]
        );
    }

    #[test]
    fn four_distinct_start_bytes_uses_full_scan() {
        let configs = vec![
            make_config(1, "<bos>"),
            make_config(2, "[SEP]"),
            make_config(3, "{pad}"),
            make_config(4, "|mask|"),
        ];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("<bos>[SEP]{pad}|mask|");
        assert_eq!(
            segs,
            vec![
                Segment::Token(1),
                Segment::Token(2),
                Segment::Token(3),
                Segment::Token(4),
            ]
        );
    }

    #[test]
    fn token_surrounded_by_text() {
        let configs = vec![make_config(5, "<mid>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("prefix <mid> suffix");
        assert_eq!(
            segs,
            vec![
                Segment::Text("prefix "),
                Segment::Token(5),
                Segment::Text(" suffix"),
            ]
        );
    }

    #[test]
    fn repeated_same_token() {
        let configs = vec![make_config(9, "<r>")];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        let segs = at.split("<r><r><r>");
        assert_eq!(
            segs,
            vec![Segment::Token(9), Segment::Token(9), Segment::Token(9)]
        );
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
    fn lstrip_absorbs_leading_whitespace() {
        let configs = vec![make_strip_config(1, "<s>", true, false)];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("ab   <s>cd"),
            vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
        );
    }

    #[test]
    fn rstrip_absorbs_trailing_whitespace() {
        let configs = vec![make_strip_config(1, "<s>", false, true)];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("ab<s>   cd"),
            vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
        );
    }

    #[test]
    fn strip_both_sides() {
        let configs = vec![make_strip_config(1, "<s>", true, true)];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("ab \t <s> \n cd"),
            vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
        );
    }

    #[test]
    fn no_strip_keeps_whitespace() {
        let configs = vec![make_strip_config(1, "<s>", false, false)];
        let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
        assert_eq!(
            at.split("ab <s> cd"),
            vec![
                Segment::Text("ab "),
                Segment::Token(1),
                Segment::Text(" cd"),
            ]
        );
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
