use crate::common::{CORPUS, Comparison, HF_MODELS, load_reference_tokenizer, load_tokenizer};

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

macro_rules! correctness_models {
    ($($name:ident => $model:expr),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                Comparison::new($model).assert_parity(CORPUS, false);
            }
        )+
    };
}

correctness_models! {
    correctness_gpt2 => "openai-community/gpt2",
    correctness_minimax_m2_1 => "MiniMaxAI/MiniMax-M2.1",
    correctness_nemotron => "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
    correctness_deepseek_v3_2 => "deepseek-ai/DeepSeek-V3.2",
    correctness_gpt_oss => "openai/gpt-oss-120b",
    correctness_qwen3 => "Qwen/Qwen3-0.6B",
    correctness_mistral_nemo => "mistralai/Mistral-Nemo-Instruct-2407",
    correctness_qwen3_nemotron => "nvidia/Qwen3-Nemotron-235B-A22B-GenRM",
    correctness_kimi_k2_5 => "hoangquan456/Kimi-K2.5",
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
