use crate::common::{
    gemma_longbench_input, load_reference_tokenizer, load_tokenizer, run_extended,
};

// Runs only with authorized Gemma access because it downloads the pinned 465 MB LongBench fixture.
#[test]
#[ignore = "requires authorized Gemma access and downloads the pinned LongBench-v2 fixture"]
fn gemma_longbench_input_matches_hugging_face() {
    let model = "google/gemma-3-1b-it";
    let hf = load_reference_tokenizer(model).unwrap();
    let ours = load_tokenizer(model).unwrap();
    let input = gemma_longbench_input().unwrap();

    for add_special_tokens in [false, true] {
        let expected = hf
            .encode(input.as_str(), add_special_tokens)
            .unwrap()
            .get_ids()
            .to_vec();
        let actual = ours.encode(&input, add_special_tokens).unwrap();
        let first_difference =
            std::iter::zip(&expected, &actual).position(|(left, right)| left != right);
        assert!(
            actual == expected,
            "{model} LongBench input 10 add_special_tokens={add_special_tokens}: \
                 expected {} IDs, got {}; first differing position: {first_difference:?}",
            expected.len(),
            actual.len(),
        );
    }
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_minimax_m2_1() {
    run_extended("MiniMaxAI/MiniMax-M2.1");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_nemotron() {
    run_extended("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_deepseek_v3_2() {
    run_extended("deepseek-ai/DeepSeek-V3.2");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_gpt_oss() {
    run_extended("openai/gpt-oss-120b");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_qwen3() {
    run_extended("Qwen/Qwen3-0.6B");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_mistral_nemo() {
    run_extended("mistralai/Mistral-Nemo-Instruct-2407");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_qwen3_nemotron() {
    run_extended("nvidia/Qwen3-Nemotron-235B-A22B-GenRM");
}

#[test]
#[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
fn extended_mistral_large() {
    run_extended("mistralai/Mistral-Large-3-675B-Instruct-2512");
}
