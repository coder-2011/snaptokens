use super::support::*;
use std::{fs, sync::OnceLock};

const LONG_BENCH_V2_REVISION: &str = "2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9";
const GEMMA_LONG_BENCH_INPUT_BLAKE3: &str =
    "23cf94a05e536b67d180de21be65ee2e9753dcc999cabf48da7e434177828379";

/// Loads the exact LongBench context that exposed the Gemma pipeline mismatch.
fn gemma_longbench_input() -> anyhow::Result<String> {
    let api = hf_hub::api::sync::Api::new()?;
    let repo = hf_hub::Repo::with_revision(
        "zai-org/LongBench-v2".to_string(),
        hf_hub::RepoType::Dataset,
        LONG_BENCH_V2_REVISION.to_string(),
    );
    let path = api.repo(repo).get("data.json")?;
    let data: Vec<serde_json::Value> = serde_json::from_slice(&fs::read(path)?)?;
    let input = data
        .get(10)
        .and_then(|item| item.get("context"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("LongBench-v2 input 10 has no context"))?
        .to_owned();
    let digest = blake3::hash(input.as_bytes()).to_hex().to_string();
    anyhow::ensure!(
        digest == GEMMA_LONG_BENCH_INPUT_BLAKE3,
        "LongBench-v2 input 10 BLAKE3 mismatch: expected {GEMMA_LONG_BENCH_INPUT_BLAKE3}, got {digest}"
    );
    Ok(input)
}

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

struct ExtendedCorpus {
    longbench: Vec<String>,
    sharegpt: Vec<String>,
}

fn extended_corpus() -> &'static ExtendedCorpus {
    static CORPUS: OnceLock<ExtendedCorpus> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let api = hf_hub::api::sync::Api::new().unwrap();

        let lb_repo = api.dataset("zai-org/LongBench-v2".to_string());
        let lb_path = lb_repo.get("data.json").unwrap();
        let lb_data: Vec<serde_json::Value> =
            serde_json::from_str(&fs::read_to_string(lb_path).unwrap()).unwrap();
        let longbench: Vec<String> = lb_data
            .iter()
            .filter_map(|item| {
                let ctx = item.get("context")?.as_str()?;
                if ctx.is_empty() {
                    None
                } else {
                    Some(ctx.to_string())
                }
            })
            .take(100)
            .collect();

        let sg_repo = api.dataset("RyokoAI/ShareGPT52K".to_string());
        let sg_path = sg_repo.get("sg_90k_part1.json").unwrap();
        let sg_data: Vec<serde_json::Value> =
            serde_json::from_str(&fs::read_to_string(sg_path).unwrap()).unwrap();
        let sharegpt: Vec<String> = sg_data
            .iter()
            .filter_map(|item| {
                let messages = item.get("conversations")?.as_array()?;
                let parts: Vec<String> = messages
                    .iter()
                    .filter_map(|msg| {
                        let role = msg
                            .get("from")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let value = msg.get("value").and_then(|v| v.as_str())?;
                        if value.is_empty() {
                            return None;
                        }
                        Some(format!("[{role}]: {value}"))
                    })
                    .collect();
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join("\n\n"))
                }
            })
            .take(1_000)
            .collect();

        ExtendedCorpus {
            longbench,
            sharegpt,
        }
    })
}

fn run_extended(model: &str) {
    let comparison = Comparison::new(model);
    let corpus = extended_corpus();
    for texts in [&corpus.longbench, &corpus.sharegpt] {
        assert!(!texts.is_empty(), "extended corpus is empty");
        for chunk in texts.chunks(10) {
            comparison.assert_parity(chunk, false);
        }
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
