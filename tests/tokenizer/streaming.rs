use crate::common::load_tokenizer;
use snaptokens::{DecodeStream, decode_stream_step};

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
