use super::{corpus::CORPUS, support::*};

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
fn ignore_merges_glm47() {
    let model = "zai-org/GLM-4.7";
    let hf = load_reference_tokenizer(model).unwrap();
    let ours = load_tokenizer(model).unwrap();

    let text = " имущества";
    let hf_ids = hf.encode(text, false).unwrap().get_ids().to_vec();
    let our_ids = ours.encode(text, false).unwrap();
    assert_eq!(
        our_ids, hf_ids,
        "ignore_merges mismatch on {text:?}: ours={our_ids:?} hf={hf_ids:?}"
    );

    let vocab_size = hf.get_vocab_size(false) as u64;
    let random_ids: Vec<u32> = (0..5000)
        .map(|i| ((i as u64).wrapping_mul(6364136223846793005).wrapping_add(1) % vocab_size) as u32)
        .collect();
    let text = hf.decode(&random_ids, true).unwrap();
    let hf_enc = hf.encode(text.as_str(), false).unwrap().get_ids().to_vec();
    let our_enc = ours.encode(&text, false).unwrap();
    assert_eq!(
        our_enc,
        hf_enc,
        "ignore_merges random-decode mismatch: {} vs {} tokens",
        our_enc.len(),
        hf_enc.len()
    );
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
fn correctness_gpt2() {
    Comparison::new("openai-community/gpt2").assert_parity(CORPUS, false);
}

#[test]
fn correctness_minimax_m2_1() {
    Comparison::new("MiniMaxAI/MiniMax-M2.1").assert_parity(CORPUS, false);
}

#[test]
fn correctness_nemotron() {
    Comparison::new("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").assert_parity(CORPUS, false);
}

#[test]
fn correctness_deepseek_v3_2() {
    Comparison::new("deepseek-ai/DeepSeek-V3.2").assert_parity(CORPUS, false);
}

#[test]
fn correctness_gpt_oss() {
    Comparison::new("openai/gpt-oss-120b").assert_parity(CORPUS, false);
}

#[test]
fn correctness_qwen3() {
    Comparison::new("Qwen/Qwen3-0.6B").assert_parity(CORPUS, false);
}

#[test]
fn correctness_mistral_nemo() {
    Comparison::new("mistralai/Mistral-Nemo-Instruct-2407").assert_parity(CORPUS, false);
}

#[test]
fn correctness_qwen3_nemotron() {
    Comparison::new("nvidia/Qwen3-Nemotron-235B-A22B-GenRM").assert_parity(CORPUS, false);
}

#[test]
fn correctness_kimi_k2_5() {
    Comparison::new("hoangquan456/Kimi-K2.5").assert_parity(CORPUS, false);
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
fn model_vocabulary_and_roundtrips() {
    let texts = [
        "Hello, world!",
        "日本語テスト",
        "The quick brown fox jumps over the lazy dog.",
        "fn main() { println!(\"hello\"); }",
        "   leading and trailing spaces   ",
        "line1\nline2\ttabbed",
        "0123456789",
        "🌍🎉✨",
    ];
    for model in HF_MODELS {
        let tokenizer = load_tokenizer(model).unwrap();
        assert!(tokenizer.vocab_size() > 0, "{model}");
        for text in texts {
            let ids = tokenizer.encode(text, false).unwrap();
            assert_eq!(tokenizer.decode(&ids, false).unwrap(), text, "{model}");
        }
        for id in [0, 1, 2, 100, 1000, 10_000] {
            let token = tokenizer.id_to_token(id).expect(model);
            assert_eq!(tokenizer.token_to_id(token), Some(id), "{model} id={id}");
        }
        let payload = vec![100, 200, 300];
        assert_eq!(
            tokenizer.post_process(payload.clone(), false),
            payload,
            "{model}"
        );
    }
}

#[test]
fn long_inputs_match_hugging_face() {
    let input = "The quick brown fox jumps over the lazy dog. Numbers: 42, 3.14, 1000. Code: fn main() {} Unicode: café, 你好. Special: @#$%^&*(). ".repeat(100);
    assert!(input.len() > 8000);
    for model in [
        "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
        "MiniMaxAI/MiniMax-M2.1",
    ] {
        Comparison::new(model).assert_parity(&[&input], false);
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
fn limited_encoding_matches_full_prefixes_and_suffixes() {
    use snaptokens::TruncationDirection::{Left, Right};
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
