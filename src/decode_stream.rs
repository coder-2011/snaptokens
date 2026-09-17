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

#[cfg(test)]
mod tests {
    use super::{DecodeStream, decode_stream_step};
    use crate::test_support::load_tokenizer;

    #[test]
    fn streaming_reconstructs_text_for_single_and_batched_steps() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in [
            "",
            "hello",
            "Hello, world! This is a streaming decode test.",
            "日本語テスト: こんにちは 🌍 — привет мир",
            r#"fn main() { println!("hello"); }"#,
            "The quick brown fox jumps over the lazy dog.",
            "one two three four five six seven eight nine ten",
        ] {
            let ids = tokenizer.encode(text, false).unwrap();
            for chunk_size in [1, 3, ids.len().max(1)] {
                let mut stream = DecodeStream::new(vec![], false);
                let mut output = String::new();
                for chunk in ids.chunks(chunk_size) {
                    if let Some(piece) = stream.step(&tokenizer, chunk.to_vec()).unwrap() {
                        assert!(!piece.is_empty());
                        output.push_str(&piece);
                    }
                }
                assert_eq!(output, text, "chunk_size={chunk_size}");
            }
        }
    }

    #[test]
    fn seeded_stream_emits_only_the_continuation() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let prompt = tokenizer.encode("The capital of France is", false).unwrap();
        let continuation = tokenizer.encode(" Paris.", false).unwrap();
        let mut stream = DecodeStream::new(prompt, false);
        let output: String = continuation
            .into_iter()
            .filter_map(|id| stream.step(&tokenizer, vec![id]).unwrap())
            .collect();
        assert_eq!(output, " Paris.");
    }

    #[test]
    fn streaming_skips_special_and_unknown_ids() {
        let tokenizer = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let mut ids = tokenizer.encode("hello", true).unwrap();
        ids.insert(1, u32::MAX);
        for skip in [false, true] {
            let mut stream = DecodeStream::new(vec![], skip);
            assert_eq!(stream.step(&tokenizer, vec![u32::MAX]).unwrap(), None);
            assert_eq!(
                stream.step(&tokenizer, ids.clone()).unwrap(),
                Some(tokenizer.decode(&ids, skip).unwrap())
            );
        }
    }

    #[test]
    fn stream_state_remains_bounded_and_rejects_invalid_prefix() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let ids = tokenizer.encode("word ".repeat(80).trim(), false).unwrap();
        let (mut buffer, mut prefix, mut index) = (vec![], String::new(), 0);
        for id in ids {
            decode_stream_step(
                &tokenizer,
                vec![id],
                false,
                &mut buffer,
                &mut prefix,
                &mut index,
            )
            .unwrap();
        }
        assert!(buffer.len() < 10, "buffer grew to {} entries", buffer.len());
        // The mismatch check requires decoded text longer than the supplied prefix.
        let mut prefix = "Z".to_owned();
        let error = decode_stream_step(
            &tokenizer,
            tokenizer.encode("hello", false).unwrap(),
            false,
            &mut vec![],
            &mut prefix,
            &mut 0,
        )
        .unwrap_err();
        assert!(error.starts_with("Invalid prefix encountered"), "{error}");
    }
}
