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
