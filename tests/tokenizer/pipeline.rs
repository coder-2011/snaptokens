use crate::common::{Comparison, load_reference_tokenizer, load_tokenizer};
use snaptokens::{DecodeStream, TruncationDirection, decode_stream_step};

#[test]
fn encode_batch_matches_sequential() {
    let model = "MiniMaxAI/MiniMax-M2.1";
    let ours = load_tokenizer(model).unwrap();

    let inputs = &["Hello, world!", "The quick brown fox", "Test", ""];
    let batch_results = ours.encode_batch(inputs, false).unwrap();

    for (input, batch_result) in inputs.iter().zip(&batch_results) {
        let sequential_result = ours.encode(input, false).unwrap();
        assert_eq!(
            batch_result, &sequential_result,
            "batch mismatch for {input:?}"
        );
    }
}

#[test]
fn encode_batch_ragged_matches_nested() {
    let ours = load_tokenizer("openai-community/gpt2").unwrap();
    let mut inputs = vec!["Hello, world! ".repeat(32); 64];
    inputs[1] = "<|endoftext|>".into();
    inputs[2] = String::new();
    let expected = ours.encode_batch(&inputs, false).unwrap();
    let (actual_ids, actual_lengths) = ours.encode_batch_ragged(&inputs, false).unwrap();

    let expected_lengths = expected.iter().map(Vec::len).collect::<Vec<_>>();
    let expected_ids = expected.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(actual_lengths, expected_lengths);
    assert_eq!(actual_ids, expected_ids);
}

#[test]
fn newline_partitioned_ragged_matches_sequential() {
    let ours = load_tokenizer("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").unwrap();
    for input in [
        "Text and numbers 123. Unicode: café, 你好. Punctuation?!\r\n\n".repeat(1_200),
        "one long line with no partition boundary ".repeat(2_000),
    ] {
        let expected = ours.encode(&input, false).unwrap();
        let (actual, lengths) = ours.encode_batch_ragged(&[input.as_str()], false).unwrap();
        assert_eq!(lengths, [expected.len()]);
        assert_eq!(actual, expected);
    }
}

#[test]
fn cache_consistency() {
    let model = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
    let ours = load_tokenizer(model).unwrap();

    let inputs = &[
        "Hello, world!",
        "The quick brown fox jumps over the lazy dog.",
        "caf\u{00e9} r\u{00e9}sum\u{00e9}",
        "\u{4f60}\u{597d}\u{4e16}\u{754c}",
        "fn main() { println!(\"hello\"); }",
        "a b c d e f g h i j k l m n o p",
        "aaaaaaaaaa bbbbbbbbbb cccccccccc",
    ];

    for &input in inputs {
        let first = ours.encode(input, false).unwrap();
        let second = ours.encode(input, false).unwrap();
        assert_eq!(first, second, "cache inconsistency for {input:?}");
        let third = ours.encode(input, false).unwrap();
        assert_eq!(first, third, "cache inconsistency (3rd call) for {input:?}");
    }
}

#[test]
fn cache_consistency_byte_level() {
    let model = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
    let ours = load_tokenizer(model).unwrap();

    let input = "The year 2024 was notable for advances in AI. Models like \
                          GPT-4 and Claude demonstrated remarkable capabilities.";
    let baseline = ours.encode(input, false).unwrap();
    for i in 0..20 {
        let result = ours.encode(input, false).unwrap();
        assert_eq!(result, baseline, "byte-level cache drift on iteration {i}");
    }
}

#[test]
fn decode_skip_special_tokens() {
    let model = "mistralai/Mistral-Nemo-Instruct-2407";
    let tok = load_tokenizer(model).unwrap();
    let text = "hello world";
    let ids_with = tok.encode(text, true).unwrap();
    let ids_without = tok.encode(text, false).unwrap();
    assert!(
        ids_with.len() > ids_without.len(),
        "expected BOS/EOS from {model}"
    );

    let skipped = tok.decode(&ids_with, true).unwrap();
    assert_eq!(skipped, text);

    let full = tok.decode(&ids_with, false).unwrap();
    assert_ne!(full, text);
    assert!(full.contains(text));
}

#[test]
fn decode_tokens_matches_decode_by_id() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    for text in &["Hello, world!", "The quick brown fox", "🌍 emoji"] {
        let ids = tok.encode(text, false).unwrap();
        let token_strings: Vec<String> = ids
            .iter()
            .map(|&id| tok.id_to_token(id).unwrap().to_string())
            .collect();
        let via_ids = tok.decode(&ids, false).unwrap();
        let via_tokens = tok.decode_tokens(token_strings).unwrap();
        assert_eq!(via_ids, via_tokens, "mismatch for {text:?}");
    }
}

#[test]
fn encode_is_stable_after_decode() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    for text in &["hello world", "日本語テスト", "fn foo() {}"] {
        let ids1 = tok.encode(text, false).unwrap();
        let decoded = tok.decode(&ids1, false).unwrap();
        let ids2 = tok.encode(&decoded, false).unwrap();
        assert_eq!(ids1, ids2, "encode not stable after decode for {text:?}");
    }
}

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

#[test]
fn add_bos_token() {
    let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
    let bos_id = tok.token_to_id("<s>").expect("<s> not in vocabulary");

    let with_bos = tok.encode("hello world", true).unwrap();
    let without_bos = tok.encode("hello world", false).unwrap();

    assert_eq!(
        with_bos.first().copied(),
        Some(bos_id),
        "first token should be BOS when add_special_tokens=true"
    );
    assert_ne!(
        without_bos.first().copied(),
        Some(bos_id),
        "BOS should be absent when add_special_tokens=false"
    );
    assert_eq!(&with_bos[1..], without_bos.as_slice());

    let tok_q = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    let with_flag = tok_q.encode("hello world", true).unwrap();
    let without_flag = tok_q.encode("hello world", false).unwrap();
    assert_eq!(
        with_flag, without_flag,
        "Qwen3 has no BOS post-processor — add_special_tokens should have no effect"
    );
}

#[test]
fn batch_special_tokens_match_scalar_and_ragged() {
    let tokenizer = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
    let inputs = ["hello world", "", "second input"];
    let expected = inputs
        .iter()
        .map(|input| tokenizer.encode(input, true).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(tokenizer.encode_batch(&inputs, true).unwrap(), expected);

    let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, true).unwrap();
    assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
    assert_eq!(ids, expected.into_iter().flatten().collect::<Vec<_>>());
}

#[test]
fn token_to_id_searches_added_tokens() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    for token in &[
        "<|image_pad|>",
        "<|vision_start|>",
        "<|vision_end|>",
        "<|im_start|>",
    ] {
        let id = tok.token_to_id(token);
        assert!(id.is_some(), "token_to_id({token:?}) returned None");
        assert_eq!(tok.id_to_token(id.unwrap()), Some(*token));
    }
}

#[test]
fn public_added_token_accessors_expose_added_vocab() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    let added_tokens = tok.added_tokens().expect("expected added tokens");

    let think_id = tok.token_to_id("<think>").expect("<think> should exist");
    assert_eq!(added_tokens.token_to_id("<think>"), Some(think_id));
    assert_eq!(added_tokens.id_to_token(think_id), Some("<think>"));

    let mut entries: Vec<_> = added_tokens.iter().collect();
    entries.sort_by_key(|entry| entry.id);
    let special_entry = entries
        .iter()
        .find(|entry| entry.special)
        .expect("expected at least one special added token");
    assert!(tok.is_special_token(special_entry.id));
    assert!(
        entries
            .iter()
            .any(|entry| entry.id == think_id && entry.content == "<think>"),
        "added-token iterator should expose <think>"
    );
}

#[test]
fn added_tokens_minimax() {
    let corpus = &[
        "<filename>",
        "open <filename> for reading",
        "<filename><reponame>",
        "printf(\"%s <filename>\\n\")",
        "<think>Let me reason about this.</think>",
        "<think>load <filename> from <reponame></think>",
        "<file> is not <filename>",
        "<fim_prefix>code here<fim_suffix>more code<fim_middle>",
    ];
    Comparison::new("MiniMaxAI/MiniMax-M2.1").assert_parity(corpus, false);
}

#[test]
fn added_tokens_deepseek() {
    let corpus = &[
        "<|begin▁of▁sentence|>Hello",
        "Hello<|end▁of▁sentence|>",
        "<|User|>What is 2+2?<|Assistant|>4<|end▁of▁sentence|>",
        "Normal text without special tokens",
        "<|tool▁calls▁begin|>call<|tool▁calls▁end|>",
    ];
    Comparison::new("deepseek-ai/DeepSeek-V3.2").assert_parity(corpus, false);
}

#[test]
fn added_tokens_qwen3() {
    let corpus = &[
        "<|im_start|>system\nYou are a helpful assistant.<|im_end|>",
        "<|im_start|>user\nHello!<|im_end|>",
        "<|endoftext|>",
        "Plain text with no special tokens at all.",
    ];
    Comparison::new("Qwen/Qwen3-0.6B").assert_parity(corpus, false);
}

#[test]
fn added_tokens_qwen3vl_vision_sequence() {
    let corpus = &[
        "<|vision_start|><|image_pad|><|vision_end|>",
        "<|image_pad|>",
        "<|vision_start|><|image_pad|><|image_pad|><|image_pad|><|image_pad|><|vision_end|>",
        "<|vision_start|><|image_pad|><|vision_end|>\nDescribe this image.",
    ];
    Comparison::new("Qwen/Qwen3.5-27B").assert_parity(corpus, false);
}

#[test]
fn added_tokens_nemotron() {
    let corpus = &[
        "<|begin_of_text|>Hello world",
        "Hello<|end_of_text|>",
        "<|start_header_id|>system<|end_header_id|>\n\nYou are helpful.<|eot_id|>",
        "<|start_header_id|>user<|end_header_id|>\n\nHi!<|eot_id|>",
        "No special tokens here.",
    ];
    Comparison::new("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").assert_parity(corpus, false);
}

#[test]
fn limited_encoding_matches_full_prefixes_and_suffixes() {
    use TruncationDirection::{Left, Right};
    let text = "Hello 世界! café 12345\n<|endoftext|> repeated words ".repeat(64);
    for model in [
        "openai-community/gpt2",
        "Qwen/Qwen3-0.6B",
        "zai-org/GLM-4.7",
        "mistralai/Mistral-Nemo-Instruct-2407",
    ] {
        let ours = load_tokenizer(model).unwrap();
        let reference = load_reference_tokenizer(model).unwrap();
        let full = reference
            .encode(text.as_str(), false)
            .unwrap()
            .get_ids()
            .to_vec();
        assert_eq!(ours.encode(&text, false).unwrap(), full);
        for direction in [Left, Right] {
            for limit in [0, 1, 7, full.len(), full.len() + 1] {
                let (ids, truncated) = ours.encode_with_limit(&text, limit, direction).unwrap();
                let expected = match direction {
                    Left => &full[full.len().saturating_sub(limit)..],
                    Right => &full[..limit.min(full.len())],
                };
                assert_eq!(ids, expected, "{model}: {direction:?} {limit}");
                assert_eq!(truncated, limit < full.len());
            }
        }
    }
}
