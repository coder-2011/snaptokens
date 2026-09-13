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
mod tests;
