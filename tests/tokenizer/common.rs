//! Shared Hugging Face fixtures and comparison helpers for integration tests.

use snaptokens::{LoadMode, Tokenizer};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

struct HfFixture {
    model: &'static str,
    revision: &'static str,
    blake3: &'static str,
}

const HF_FIXTURES: &[HfFixture] = &[
    HfFixture {
        model: "Qwen/Qwen3-0.6B",
        revision: "c1899de289a04d12100db370d81485cdf75e47ca",
        blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
    },
    HfFixture {
        model: "Qwen/Qwen3.5-27B",
        revision: "fc05daec18b0a78c049392ed2e771dde82bdf654",
        blake3: "3fd498e7fff0650ca4cf62dccd4e0844ed5ad2f1abcee9adfda013fef06c4220",
    },
    HfFixture {
        model: "zai-org/GLM-4.7",
        revision: "602d01efcdd332c5238ca4bcede555defbe83eb7",
        blake3: "ce097ff5c5a465244c42eab34b0c09e298c17bebaab2fd6bef354d5bd501a683",
    },
    HfFixture {
        model: "deepseek-ai/DeepSeek-V3.2",
        revision: "a7e62ac04ecb2c0a54d736dc46601c5606cf10a6",
        blake3: "43340ce86c8386401d11d3a3fb5a1798ffa696befffc0592a160ea493df6f3ac",
    },
    HfFixture {
        model: "MiniMaxAI/MiniMax-M2.1",
        revision: "cd97f59135f37b2a6bf09356e485d5e4aeb7dc9c",
        blake3: "b5a4e31f8028abc9bf05a2b4049d26ef2b02c132414221f07985fe85abc4a95a",
    },
    HfFixture {
        model: "openai/gpt-oss-120b",
        revision: "b5c939de8f754692c1647ca79fbf85e8c1e70f8a",
        blake3: "0596814de1ca4de2980968802fdb4340e84e39f8ffa67467648e8e606680acb1",
    },
    HfFixture {
        model: "mistralai/Mistral-Nemo-Instruct-2407",
        revision: "04d8a90549d23fc6bd7f642064003592df51e9b3",
        blake3: "5309310c331b5fca7b15536ff8fb20fa1602493f58cfb9aba4d3508d97af2f98",
    },
    HfFixture {
        model: "mistralai/Mistral-Large-3-675B-Instruct-2512",
        revision: "383ffea2c7d60dfd44ca960e8e691709d4fdb9cd",
        blake3: "3d69466e68ef8a541dba19ef2761a1c2e9eb504fa9517607d48984daedbe5887",
    },
    HfFixture {
        model: "Qwen/Qwen3-235B-A22B-Instruct-2507",
        revision: "ac9c66cc9b46af7306746a9250f23d47083d689e",
        blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
    },
    HfFixture {
        model: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
        revision: "9d90cf8fca1bf7b7acca42d3fc9ae694a2194069",
        blake3: "e4bf149b34e0372424d0cd7afba11bbda682d8a3bb29b95a02a7c97e89dc63a2",
    },
    HfFixture {
        model: "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
        revision: "bf77c3174f68ad409e1c2aa60daeb46e32d1c606",
        blake3: "90962b12be7538b0a19316f293747faab874ec037a9d632a5eb3c72456581dca",
    },
    HfFixture {
        model: "nvidia/Qwen3-Nemotron-235B-A22B-GenRM",
        revision: "b127d6abced0dd19b8d1f9678a881bfd30380d4b",
        blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
    },
    HfFixture {
        model: "hoangquan456/Kimi-K2.5",
        revision: "cb77384a429dcec7a14359e25ccdb1b7eb61a6b1",
        blake3: "5ebdd385ae20df5aeb5d6b49ab5e3add2093bc17073a636e9b6c125319d6ee7c",
    },
    HfFixture {
        model: "openai-community/gpt2",
        revision: "607a30d783dfa663caf39e06633721c8d4cfcd7e",
        blake3: "1f9b61de3382db2e111c702730ef4ad5b12788d3c040db87936da6c7f988f861",
    },
    HfFixture {
        model: "google-t5/t5-small",
        revision: "df1b051c49625cf57a3d0d8d3863ed4d13564fe4",
        blake3: "9ce04442387ff177fd52c14e1bd1901ba563d71e956ca2cffd39eeb3ef4bb820",
    },
    HfFixture {
        model: "google/gemma-3-1b-it",
        revision: "dcc83ea841ab6100d6b47a070329e1ba4cf78752",
        blake3: "38e6c65074653102e6e238195e25938a4cb1ea2df4c7c01283d45de480696a11",
    },
];

pub const HF_MODELS: &[&str] = &[
    "Qwen/Qwen3-0.6B",
    "zai-org/GLM-4.7",
    "deepseek-ai/DeepSeek-V3.2",
    "MiniMaxAI/MiniMax-M2.1",
    "openai/gpt-oss-120b",
    "mistralai/Mistral-Nemo-Instruct-2407",
    "Qwen/Qwen3-235B-A22B-Instruct-2507",
    "Qwen/Qwen3-Coder-480B-A35B-Instruct",
    "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
    "nvidia/Qwen3-Nemotron-235B-A22B-GenRM",
    "hoangquan456/Kimi-K2.5",
];

pub fn tokenizer_json_path(model: &str) -> anyhow::Result<PathBuf> {
    static VERIFIED: OnceLock<Mutex<HashMap<&'static str, PathBuf>>> = OnceLock::new();
    let fixture = HF_FIXTURES
        .iter()
        .find(|fixture| fixture.model == model)
        .ok_or_else(|| anyhow::anyhow!("missing pinned fixture for {model}"))?;
    let verified = VERIFIED.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(path) = verified.lock().unwrap().get(model).cloned() {
        return Ok(path);
    }
    let api = hf_hub::api::sync::Api::new()?;
    let repo = hf_hub::Repo::with_revision(
        model.to_string(),
        hf_hub::RepoType::Model,
        fixture.revision.to_string(),
    );
    let path = api.repo(repo).get("tokenizer.json")?;
    let actual = blake3::hash(&fs::read(&path)?).to_hex().to_string();
    anyhow::ensure!(
        actual == fixture.blake3,
        "{model}@{} tokenizer.json hash mismatch: expected {}, got {actual}",
        fixture.revision,
        fixture.blake3,
    );
    verified.lock().unwrap().insert(fixture.model, path.clone());
    Ok(path)
}

pub fn load_tokenizer(model: &str) -> anyhow::Result<Tokenizer> {
    let path = tokenizer_json_path(model)?;
    Ok(Tokenizer::load_file(&path, LoadMode::JsonOnly)?)
}

pub fn load_reference_tokenizer(model: &str) -> anyhow::Result<tokenizers::Tokenizer> {
    let path = tokenizer_json_path(model)?;
    tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow::anyhow!(error))
}

/// One oracle for scalar, nested, and ragged APIs, including warm cache calls.
pub struct Comparison {
    model: String,
    ours: Tokenizer,
    reference: tokenizers::Tokenizer,
}

impl Comparison {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.into(),
            ours: load_tokenizer(model).unwrap_or_else(|error| panic!("{model}: {error}")),
            reference: load_reference_tokenizer(model)
                .unwrap_or_else(|error| panic!("{model}: {error}")),
        }
    }

    pub fn assert_parity(&self, inputs: &[impl AsRef<str> + Sync], special: bool) {
        let expected: Vec<_> = inputs
            .iter()
            .map(|input| {
                self.reference
                    .encode(input.as_ref(), special)
                    .unwrap()
                    .get_ids()
                    .to_vec()
            })
            .collect();
        for (input, ids) in inputs.iter().zip(&expected) {
            let input = input.as_ref();
            for _ in 0..2 {
                assert_eq!(
                    self.ours.encode(input, special).unwrap(),
                    *ids,
                    "{} scalar special={special} input={input:?}",
                    self.model
                );
            }
            for skip in [false, true] {
                assert_eq!(
                    self.ours.decode(ids, skip).unwrap(),
                    self.reference.decode(ids, skip).unwrap(),
                    "{} decode skip={skip} input={input:?}",
                    self.model
                );
            }
        }
        assert_eq!(
            self.ours.encode_batch(inputs, special).unwrap(),
            expected,
            "{} batch",
            self.model
        );
        let (ids, lengths) = self.ours.encode_batch_ragged(inputs, special).unwrap();
        assert_eq!(
            lengths,
            expected.iter().map(Vec::len).collect::<Vec<_>>(),
            "{} row lengths",
            self.model
        );
        assert_eq!(
            ids,
            expected.into_iter().flatten().collect::<Vec<_>>(),
            "{} ragged IDs",
            self.model
        );
    }
}

pub const CORPUS: &[&str] = &[
    "",
    " ",
    "  ",
    "\n",
    "\t",
    "\r\n",
    "a",
    "Z",
    "0",
    "!",
    "\u{00e9}",
    "\u{4e2d}",
    "Hello, world!",
    "The quick brown fox jumps over the lazy dog.",
    "A short sentence.",
    "  leading spaces",
    "trailing spaces  ",
    "  both  sides  ",
    "multiple    internal    spaces",
    "tabs\there\tand\tthere",
    "line\none\nline\ntwo",
    "windows\r\nline\r\nendings",
    "mixed\n\ttabs and\r\nnewlines  with  spaces",
    "42",
    "3.14159",
    "1,000,000",
    "0xFF",
    "1e-10",
    "Numbers 1234567890 and mixed ABC123def",
    "Hello!!! How are you???",
    "@user #hashtag $100 %50 ^caret &amp *star",
    "a-b_c.d,e;f:g",
    "(parentheses) [brackets] {braces}",
    "\"double quotes\" 'single quotes' `backticks`",
    "path/to/file.txt",
    "https://example.com/path?q=test&lang=en#section",
    "Special chars: @#$%^&*()_+-=[]{}|;':\",./<>?",
    "caf\u{00e9} r\u{00e9}sum\u{00e9} na\u{00ef}ve",
    "\u{00fc}ber stra\u{00df}e gr\u{00f6}\u{00df}e",
    "se\u{00f1}or ni\u{00f1}o a\u{00f1}o",
    "\u{4f60}\u{597d}\u{4e16}\u{754c}",
    "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}",
    "\u{c548}\u{b155}\u{d558}\u{c138}\u{c694}",
    "\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442} \u{043c}\u{0438}\u{0440}",
    "\u{0645}\u{0631}\u{062d}\u{0628}\u{0627}",
    "\u{0928}\u{092e}\u{0938}\u{094d}\u{0924}\u{0947}",
    "\u{1f600}\u{1f680}\u{2764}\u{fe0f}",
    "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}\u{200d}\u{1f466}",
    "\u{1f1fa}\u{1f1f8}",
    "e\u{0301}",
    "n\u{0303}",
    "a\u{0308}",
    "Hello \u{4e16}\u{754c} \u{041c}\u{0438}\u{0440}!",
    "User123 wrote: \u{4f60}\u{597d}!",
    "fn main() { println!(\"hello\"); }",
    "def foo(x: int) -> str:\n    return str(x)",
    "SELECT * FROM users WHERE id = 1;",
    "if (x > 0 && y < 10) { z = x + y; }",
    "<html><body><p>Hello</p></body></html>",
    "#include <stdio.h>\nint main() { return 0; }",
    "import numpy as np\nx = np.array([1, 2, 3])",
    "{\"key\": \"value\", \"number\": 42, \"array\": [1, 2, 3]}",
    "[{\"id\": 1}, {\"id\": 2}]",
    "aaaaaaaaaa",
    "abababababababab",
    "the the the the the the the the",
    "....",
    "----",
    "    ",
    "\n\n\n\n",
    "This is a longer sentence with various elements: numbers (42, 3.14), \
         symbols (@#$), Unicode (caf\u{00e9}, \u{4f60}\u{597d}), and more.",
    "The year 2024 was notable for advances in AI. Models like GPT-4 and \
         Claude demonstrated remarkable capabilities in reasoning, coding, and \
         multilingual understanding.",
    "a b c d e f g h i j k l m n o p q r s t u v w x y z",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "0123456789",
    "a\nb\nc\n",
    "# Heading\n\n- item 1\n- item 2\n\n```code```",
    "\u{ffff}",
    "\u{0080}",
    "\u{07ff}",
    "\u{0800}",
    "\u{10000}",
    "\u{fffd}",
    "\u{feff}Hello",
    "\u{0000}",
    "abc\u{0000}def",
    "\u{fffe}",
    "\u{fdd0}",
    "\u{200b}\u{200c}\u{200d}",
    "\u{202e}Hello\u{202c}",
    "\u{0001}\u{0002}\u{001f}\u{007f}",
    "\u{0300}",
    "a\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}",
    "\u{e000}\u{f8ff}",
    "\u{01c5}\u{01c8}\u{01cb}",
    "\u{2028}\u{2029}",
    "\u{fff9}\u{fffa}\u{fffb}",
    "\u{d7ff}\u{10ffff}",
    "ab",
    "abc",
    "abcd",
    "aaa",
    "aaaa",
    "aaaaa",
    "**bold** *italic* ~~strikethrough~~ __underline__",
    "```rust\nfn main() {}\n```",
    "> blockquote\n>> nested",
    "| col1 | col2 |\n|------|------|\n| a    | b    |",
];

const LONG_BENCH_V2_REVISION: &str = "2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9";

const GEMMA_LONG_BENCH_INPUT_BLAKE3: &str =
    "23cf94a05e536b67d180de21be65ee2e9753dcc999cabf48da7e434177828379";

/// Loads the exact LongBench context that exposed the Gemma pipeline mismatch.
pub fn gemma_longbench_input() -> anyhow::Result<String> {
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

/// Compares a model against Hugging Face on the LongBench and ShareGPT samples.
pub fn run_extended(model: &str) {
    let comparison = Comparison::new(model);
    let corpus = extended_corpus();
    for texts in [&corpus.longbench, &corpus.sharegpt] {
        assert!(!texts.is_empty(), "extended corpus is empty");
        for chunk in texts.chunks(10) {
            comparison.assert_parity(chunk, false);
        }
    }
}
