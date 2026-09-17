use base64::{Engine as _, engine::general_purpose::STANDARD};
use snaptokens::{
    DecodeStream, LoadMode, NormalizerConfig, Tokenizer, decode_stream_step,
    json_structs::TokenizerJson,
};
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

const HF_MODELS: &[&str] = &[
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

fn tokenizer_json_path(model: &str) -> anyhow::Result<PathBuf> {
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

fn load_tokenizer(model: &str) -> anyhow::Result<Tokenizer> {
    let path = tokenizer_json_path(model)?;
    Ok(Tokenizer::load_file(&path, LoadMode::JsonOnly)?)
}

fn load_reference_tokenizer(model: &str) -> anyhow::Result<tokenizers::Tokenizer> {
    let path = tokenizer_json_path(model)?;
    tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow::anyhow!(error))
}

/// One oracle for scalar, nested, and ragged APIs, including warm cache calls.
struct Comparison {
    model: String,
    ours: Tokenizer,
    reference: tokenizers::Tokenizer,
}

impl Comparison {
    fn new(model: &str) -> Self {
        Self {
            model: model.into(),
            ours: load_tokenizer(model).unwrap_or_else(|error| panic!("{model}: {error}")),
            reference: load_reference_tokenizer(model)
                .unwrap_or_else(|error| panic!("{model}: {error}")),
        }
    }

    fn assert_parity(&self, inputs: &[impl AsRef<str> + Sync], special: bool) {
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

const CORPUS: &[&str] = &[
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

/// Cover ordinary, fused, and nested normalized/added-token error paths.
#[test]
fn regex_matching_errors_reach_encode_callers() {
    let split = serde_json::json!({
        "type":"Split", "pattern":{"Regex":r"z|(?i)(a|b|ab)*(?>c)|a"},
        "behavior":"Isolated", "invert":false
    });
    let byte_level = serde_json::json!({
        "type":"ByteLevel", "add_prefix_space":false, "use_regex":false
    });
    let first_split = serde_json::json!({
        "type":"Split", "pattern":{"String":"|"}, "behavior":"Isolated", "invert":false
    });
    for (pre_tokenizer, prefix) in [
        (split.clone(), ""),
        (
            serde_json::json!({"type":"Sequence", "pretokenizers":[split, byte_level]}),
            "",
        ),
        (
            serde_json::json!({"type":"Sequence", "pretokenizers":[first_split, split, byte_level]}),
            "<s>",
        ),
    ] {
        let tokenizer = Tokenizer::from_json(serde_json::json!({
            "model":{"type":"BPE", "vocab":{"a":0,"b":1,"ab":2,"z":3}, "merges":[["a","b"]]},
            "pre_tokenizer":pre_tokenizer,
            "normalizer":{"type":"Replace", "pattern":{"String":"A"}, "content":"a"},
            "added_tokens":[{"id":4,"content":"<s>","special":true,"normalized":false}]
        }))
        .unwrap();
        let input = format!("{prefix}z{}", "Ab".repeat(20));
        for error in [
            tokenizer.encode(&input, false).unwrap_err(),
            tokenizer
                .encode_batch_ragged(&["zab", &input], false)
                .unwrap_err(),
        ] {
            assert!(
                matches!(
                    error,
                    snaptokens::Error::PreTokenizer(snaptokens::pre_tokenizers::Error::Regex(_))
                ),
                "{error}"
            );
        }
        assert_eq!(tokenizer.encode("zab", false).unwrap(), [3, 0, 1]);
    }
}

/// A scan failure must surface even when draining pending BPE work also
/// fails, matching the error the unfused pipeline reports.
#[test]
fn fused_regex_error_precedes_model_error() {
    let tokenizer = Tokenizer::from_json(serde_json::json!({
        "model":{"type":"BPE", "vocab":{"a":0,"b":1,"ab":2}, "merges":[["a","b"]]},
        "pre_tokenizer":{"type":"Sequence", "pretokenizers":[
            {"type":"Split", "pattern":{"Regex":r"z|(?i)(a|b|ab)*(?>c)|a"},
             "behavior":"Isolated", "invert":false},
            {"type":"ByteLevel", "add_prefix_space":false, "use_regex":false}
        ]}
    }))
    .unwrap();
    // The matched z is queued before the remaining input exceeds the regex limit.
    let input = format!("z{}", "ab".repeat(20));
    for error in [
        tokenizer.encode(&input, false).unwrap_err(),
        tokenizer.encode_batch_ragged(&[&input], false).unwrap_err(),
    ] {
        assert!(
            matches!(
                error,
                snaptokens::Error::PreTokenizer(snaptokens::pre_tokenizers::Error::Regex(_))
            ),
            "{error}"
        );
    }
}

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

fn precompiled_charsmap(config: &NormalizerConfig) -> Option<&str> {
    match config {
        NormalizerConfig::Precompiled {
            precompiled_charsmap,
        } => Some(precompiled_charsmap),
        NormalizerConfig::Sequence { normalizers } => {
            normalizers.iter().find_map(precompiled_charsmap)
        }
        NormalizerConfig::Nfc | NormalizerConfig::Replace { .. } => None,
    }
}

#[test]
fn t5_unigram_matches_hugging_face_pipeline() {
    let model = "google-t5/t5-small";
    let corpus = [
        "",
        "hello world",
        " hello  world ",
        "café déjà vu",
        "① ﬁ Å ＡＢＣ\u{00a0}x",
        "こんにちは、世界！",
        "emoji: 😀",
        "line one\nline two\tthree",
        "<extra_id_0> answer <extra_id_1>",
    ];
    let comparison = Comparison::new(model);
    comparison.assert_parity(&corpus, false);
    comparison.assert_parity(&corpus, true);
}

#[test]
fn t5_unigram_repeated_prefixes_match_hugging_face() {
    let model = "google-t5/t5-small";
    let inputs = [
        "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda ".repeat(128),
        "The quick brown fox jumps over the lazy dog. ".repeat(192),
        "café 東京 😀 punctuation?! numbers 12345 ".repeat(96),
    ];
    Comparison::new(model).assert_parity(&inputs, false);
}

#[test]
fn t5_unigram_partitioned_documents_match_hugging_face() {
    // Large single documents dispatch to the parallel partitioned fused
    // path; mixed whitespace, markers, CJK, and added tokens must reproduce
    // Hugging Face exactly across partition cuts.
    let model = "google-t5/t5-small";
    let paragraph = "The archive spans genres; nested clauses, ▁markers, \
        tabs\tand CRLF\r\nlines, café naïve déjà, 東京タワー statistics 12345, \
        emoji 😀🚀, wide\u{3000}space and thin\u{2009}space. ";
    let inputs = [
        paragraph.repeat(400),
        format!(
            "{}<extra_id_0>{}<extra_id_1> tail",
            paragraph.repeat(220),
            paragraph.repeat(220)
        ),
        "solitary-run-without-any-whitespace-".repeat(2000),
        // A combining mark directly after a space joins that space's grapheme
        // cluster, and decomposed accents span ASCII-adjacent boundaries; no
        // partition cut may separate either.
        "x \u{301}accent e\u{301}tude words here pad pad pad ".repeat(800),
    ];
    for input in &inputs {
        assert!(input.len() > 16 * 1024);
    }
    // Four large rows take the wide-batch Unigram path, which must keep the
    // partitioned walker and still match sequential Hugging Face IDs.
    Comparison::new(model).assert_parity(&inputs, false);
}

#[test]
fn t5_unigram_normalized_partitions_match_hugging_face() {
    // A Sequence-wrapped Precompiled is not partition-safe, so large
    // documents take the post-normalization parallel walker.
    let path = tokenizer_json_path("google-t5/t5-small").unwrap();
    let mut json: serde_json::Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let normalizer = json["normalizer"].take();
    json["normalizer"] = serde_json::json!({
        "type": "Sequence",
        "normalizers": [normalizer]
    });
    let encoded = json.to_string();
    let ours = Tokenizer::from_json(json).unwrap();
    let hf = tokenizers::Tokenizer::from_bytes(encoded.as_bytes()).unwrap();
    let paragraph = "The archive spans genres; nested clauses, ▁markers, \
        tabs\tand CRLF\r\nlines, café naïve déjà, 東京タワー statistics 12345, \
        emoji 😀🚀, wide\u{3000}space and thin\u{2009}space. ";
    let inputs = [
        paragraph.repeat(400),
        format!(
            "{}<extra_id_0>{}<extra_id_1> tail",
            paragraph.repeat(220),
            paragraph.repeat(220)
        ),
    ];
    let expected: Vec<Vec<u32>> = inputs
        .iter()
        .map(|input| {
            assert!(input.len() > 16 * 1024);
            let expected = hf.encode(input.as_str(), false).unwrap().get_ids().to_vec();
            assert_eq!(ours.encode(input, false).unwrap(), expected);
            expected
        })
        .collect();
    assert_eq!(ours.encode_batch(&inputs, false).unwrap(), expected);
}

#[test]
fn t5_precompiled_normalizer_matches_its_reference_charsmap() {
    let path = tokenizer_json_path("google-t5/t5-small").unwrap();
    let tokenizer_json: TokenizerJson = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let charsmap = precompiled_charsmap(tokenizer_json.normalizer.as_ref().unwrap())
        .unwrap()
        .to_owned();
    let reference_bytes = STANDARD.decode(&charsmap).unwrap();
    let reference = spm_precompiled::Precompiled::from(&reference_bytes).unwrap();
    let optimized = snaptokens::normalizers::Precompiled::from_config(charsmap).unwrap();
    let printable_ascii = (0x20u8..0x7f).map(char::from).collect::<String>();
    let all_ascii = (0..0x80u8).map(char::from).collect::<String>();
    let inputs = [
        String::new(),
        "plain ASCII text with punctuation?! 12345 ".repeat(64),
        printable_ascii,
        all_ascii,
        "line one\r\nline two\tthree\u{7f}four".to_owned(),
        "cafe\u{301} ﬁ ① ＡＢＣ".to_owned(),
        "ASCII-before-combining-a\u{301} and 東京 😀 after".to_owned(),
    ];

    for input in inputs {
        assert_eq!(
            optimized.normalize(&input),
            reference.normalize_string(&input),
            "precompiled normalizer mismatch for {input:?}"
        );
    }
}
