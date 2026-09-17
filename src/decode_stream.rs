use crate::Tokenizer;

/// Stateful incremental decoder that waits for valid UTF-8 before yielding text.
pub struct DecodeStream {
    skip_special_tokens: bool,
    ids: Vec<u32>,
    prefix: String,
    prefix_index: usize,
}

impl DecodeStream {
    /// Starts a stream, optionally seeded with already-decoded prompt IDs.
    pub fn new(ids: Vec<u32>, skip_special_tokens: bool) -> Self {
        Self {
            skip_special_tokens,
            ids,
            prefix: String::new(),
            prefix_index: 0,
        }
    }

    /// Appends IDs and returns newly completed text, if any.
    pub fn step(
        &mut self,
        tokenizer: &Tokenizer,
        token_ids: Vec<u32>,
    ) -> Result<Option<String>, String> {
        decode_stream_step(
            tokenizer,
            token_ids,
            self.skip_special_tokens,
            &mut self.ids,
            &mut self.prefix,
            &mut self.prefix_index,
        )
    }
}

/// Advances explicit incremental-decoder state, retaining incomplete UTF-8 until it resolves.
pub fn decode_stream_step(
    tokenizer: &Tokenizer,
    token_ids: Vec<u32>,
    skip_special_tokens: bool,
    ids: &mut Vec<u32>,
    prefix: &mut String,
    prefix_index: &mut usize,
) -> Result<Option<String>, String> {
    const REPLACEMENT: char = '\u{FFFD}';

    // If the prefix is empty but we already have buffered IDs (e.g. seeded
    // with prompt tokens), prime the prefix before adding the new token.
    if prefix.is_empty() && !ids.is_empty() {
        let s = tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| e.to_string())?;
        if !s.ends_with(REPLACEMENT) {
            *prefix = s;
            *prefix_index = ids.len();
        }
    }

    ids.extend(token_ids);

    let string = tokenizer
        .decode(ids, skip_special_tokens)
        .map_err(|e| e.to_string())?;

    if string.len() > prefix.len() && !string.ends_with(REPLACEMENT) {
        if !string.starts_with(prefix.as_str()) {
            return Err(format!(
                "Invalid prefix encountered while decoding stream. \
                 Expected prefix: '{}', Actual string: '{}'",
                prefix, string,
            ));
        }
        let new_text = string[prefix.len()..].to_string();
        drop(ids.drain(..*prefix_index));
        *prefix = tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| e.to_string())?;
        *prefix_index = ids.len();
        Ok(Some(new_text))
    } else {
        Ok(None)
    }
}
