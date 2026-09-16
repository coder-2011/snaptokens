use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use snaptokens::{NormalizerConfig, Tokenizer, decode_stream_step, json_structs::TokenizerJson};

struct HfFixture {
    model: &'static str,
    revision: &'static str,
    blake3: &'static str,
}

const HF_FIXTURES: &[HfFixture] = &[
    HfFixture {
        model: "google-t5/t5-small",
        revision: "df1b051c49625cf57a3d0d8d3863ed4d13564fe4",
        blake3: "9ce04442387ff177fd52c14e1bd1901ba563d71e956ca2cffd39eeb3ef4bb820",
    },
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

#[test]
fn parity_models_have_immutable_fixtures() {
    for model in HF_MODELS {
        assert!(HF_FIXTURES.iter().any(|fixture| fixture.model == *model));
    }
}

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
    Ok(Tokenizer::load_file(&path)?)
}

fn load_reference_tokenizer(model: &str) -> anyhow::Result<tokenizers::Tokenizer> {
    let path = tokenizer_json_path(model)?;
    tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow::anyhow!(error))
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
fn load_hf_json() {
    for model in HF_MODELS {
        let json_path = tokenizer_json_path(model).unwrap_or_else(|e| panic!("{model}: {e}"));
        let tokenizer = Tokenizer::load_file(&json_path).unwrap_or_else(|e| panic!("{model}: {e}"));
        assert!(
            tokenizer.vocab_size() > 0,
            "{model}: loaded empty vocabulary"
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
        let sequential_result = ours.encode(input).unwrap();
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
fn byte_fallback_merge_crosses_unicode_boundary() {
    let tokenizer = Tokenizer::from_json(serde_json::json!({
        "normalizer": null,
        "pre_tokenizer": null,
        "model": {
            "type": "BPE",
            "vocab": {
                "<unk>": 0,
                "<0xC3>": 1,
                "<0xA9>": 2,
                "<0xAA>": 3,
                "<0xA9><0xC3>": 4
            },
            "merges": [["<0xA9>", "<0xC3>"]],
            "unk_token": "<unk>",
            "byte_fallback": true
        },
        "post_processor": null,
        "decoder": null
    }))
    .unwrap();

    assert_eq!(tokenizer.encode("éê").unwrap(), vec![1, 4, 3]);
}

#[test]
fn normalized_added_tokens_follow_the_two_phase_pipeline() {
    let tokenizer = Tokenizer::from_json(serde_json::json!({
        "added_tokens": [
            {
                "id": 10,
                "content": "e\u{301}!",
                "single_word": false,
                "lstrip": false,
                "rstrip": false,
                "normalized": true,
                "special": false
            },
            {
                "id": 11,
                "content": "e\u{301}?",
                "single_word": false,
                "lstrip": false,
                "rstrip": false,
                "normalized": false,
                "special": false
            }
        ],
        "normalizer": { "type": "NFC" },
        "pre_tokenizer": null,
        "model": {
            "type": "BPE",
            "vocab": { "<unk>": 0 },
            "merges": [],
            "unk_token": "<unk>",
            "byte_fallback": false
        },
        "post_processor": null,
        "decoder": null
    }))
    .unwrap();

    assert_eq!(tokenizer.encode("é!").unwrap(), vec![10]);
    assert_eq!(tokenizer.encode("e\u{301}!").unwrap(), vec![10]);
    assert_eq!(tokenizer.encode("e\u{301}?").unwrap(), vec![11]);
}

#[test]
fn vocab_access() {
    let model = "MiniMaxAI/MiniMax-M2.1";
    let ours = load_tokenizer(model).unwrap();

    assert!(ours.vocab_size() > 0);

    let token_str = ours.id_to_token(0).expect("token 0 should exist");
    let id = ours
        .token_to_id(token_str)
        .expect("reverse lookup should work");
    assert_eq!(id, 0);
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

fn compare_encode_decode(model_name: &str, corpus: &[&str]) -> Vec<String> {
    let hf = load_reference_tokenizer(model_name)
        .unwrap_or_else(|e| panic!("{model_name}: HF load failed: {e}"));
    let ours = load_tokenizer(model_name)
        .unwrap_or_else(|e| panic!("{model_name}: fastokens load failed: {e}"));

    let mut failures = Vec::new();
    for &input in corpus {
        let hf_enc = hf
            .encode(input, false)
            .unwrap_or_else(|e| panic!("{model_name}: HF encode({input:?}): {e}"));
        let hf_ids = hf_enc.get_ids().to_vec();
        let our_ids = match ours.encode(input) {
            Ok(ids) => ids,
            Err(e) => {
                failures.push(format!("  encode error on {input:?}: {e}"));
                continue;
            }
        };
        if our_ids != hf_ids {
            failures.push(format!(
                "  encode mismatch on {input:?}: got {} tokens, expected {}\n\
                     \x20   ours: {:?}\n\
                     \x20   hf:   {:?}",
                our_ids.len(),
                hf_ids.len(),
                &our_ids[..our_ids.len().min(20)],
                &hf_ids[..hf_ids.len().min(20)],
            ));
        }

        if input.is_empty() || hf_ids.is_empty() {
            continue;
        }
        let hf_decoded = match hf.decode(&hf_ids, false) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let our_decoded = match ours.decode(&hf_ids, false) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("  decode error on {input:?}: {e}"));
                continue;
            }
        };
        if our_decoded != hf_decoded {
            failures.push(format!(
                "  decode mismatch on {input:?}:\n\
                     \x20   ours: {:?}\n\
                     \x20   hf:   {:?}",
                &our_decoded[..our_decoded.len().min(100)],
                &hf_decoded[..hf_decoded.len().min(100)],
            ));
        }
    }
    failures
}

/// Nested batch rows and the flat-ragged reconstruction must match `expected`.
fn assert_batch_and_ragged_eq<S: AsRef<str> + Sync>(
    ours: &Tokenizer,
    inputs: &[S],
    add_special_tokens: bool,
    expected: &[Vec<u32>],
) {
    assert_eq!(
        ours.encode_batch(inputs, add_special_tokens).unwrap(),
        expected
    );
    let (ids, lengths) = ours
        .encode_batch_ragged(inputs, add_special_tokens)
        .unwrap();
    let mut offset = 0;
    for (expected, length) in expected.iter().zip(lengths) {
        assert_eq!(&ids[offset..offset + length], expected.as_slice());
        offset += length;
    }
    assert_eq!(offset, ids.len());
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
    let failures = compare_encode_decode(model, &corpus);
    assert!(
        failures.is_empty(),
        "T5 Unigram parity failures:\n{}",
        failures.join("\n")
    );

    let ours = load_tokenizer(model).unwrap();
    let hf = load_reference_tokenizer(model).unwrap();
    let expected: Vec<Vec<u32>> = corpus
        .iter()
        .map(|input| hf.encode(*input, true).unwrap().get_ids().to_vec())
        .collect();
    assert_batch_and_ragged_eq(&ours, &corpus, true, &expected);
}

#[test]
fn t5_unigram_repeated_prefixes_match_hugging_face() {
    let model = "google-t5/t5-small";
    let inputs = [
        "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda ".repeat(128),
        "The quick brown fox jumps over the lazy dog. ".repeat(192),
        "café 東京 😀 punctuation?! numbers 12345 ".repeat(96),
    ];
    let ours = load_tokenizer(model).unwrap();
    let hf = load_reference_tokenizer(model).unwrap();
    let expected: Vec<Vec<u32>> = inputs
        .iter()
        .map(|input| hf.encode(input.as_str(), false).unwrap().get_ids().to_vec())
        .collect();

    assert_batch_and_ragged_eq(&ours, &inputs, false, &expected);
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
    let ours = load_tokenizer(model).unwrap();
    let hf = load_reference_tokenizer(model).unwrap();
    let expected: Vec<Vec<u32>> = inputs
        .iter()
        .map(|input| {
            assert!(input.len() > 16 * 1024);
            let expected = hf.encode(input.as_str(), false).unwrap().get_ids().to_vec();
            assert_eq!(ours.encode(input).unwrap(), expected);
            expected
        })
        .collect();
    // Four large rows take the wide-batch Unigram path, which must keep the
    // partitioned walker and still match sequential Hugging Face IDs.
    assert_eq!(ours.encode_batch(&inputs, false).unwrap(), expected);
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
            assert_eq!(ours.encode(input).unwrap(), expected);
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

#[test]
fn correctness_gpt2() {
    let f = compare_encode_decode("openai-community/gpt2", CORPUS);
    assert!(f.is_empty(), "openai-community/gpt2:\n{}", f.join("\n"));
}

#[test]
fn correctness_minimax_m2_1() {
    let f = compare_encode_decode("MiniMaxAI/MiniMax-M2.1", CORPUS);
    assert!(f.is_empty(), "MiniMaxAI/MiniMax-M2.1:\n{}", f.join("\n"));
}

#[test]
fn correctness_nemotron() {
    let f = compare_encode_decode("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16", CORPUS);
    assert!(
        f.is_empty(),
        "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16:\n{}",
        f.join("\n")
    );
}

#[test]
fn correctness_deepseek_v3_2() {
    let f = compare_encode_decode("deepseek-ai/DeepSeek-V3.2", CORPUS);
    assert!(f.is_empty(), "deepseek-ai/DeepSeek-V3.2:\n{}", f.join("\n"));
}

#[test]
fn correctness_gpt_oss() {
    let f = compare_encode_decode("openai/gpt-oss-120b", CORPUS);
    assert!(f.is_empty(), "openai/gpt-oss-120b:\n{}", f.join("\n"));
}

#[test]
fn ignore_merges_preserves_piece_semantics() {
    let tokenizer = |ignore_merges| {
        Tokenizer::from_json(serde_json::json!({
            "normalizer": null,
            "pre_tokenizer": null,
            "model": {
                "type": "BPE",
                "vocab": {"a": 0, "b": 1, "c": 2, "d": 3, "ab": 4, "abc": 5},
                "merges": [],
                "ignore_merges": ignore_merges
            },
            "post_processor": null,
            "decoder": null
        }))
        .unwrap()
    };

    let merged = tokenizer(false);
    assert_eq!(merged.encode("ab").unwrap(), vec![0, 1]);

    let ignored = tokenizer(true);
    assert_eq!(ignored.encode("abc").unwrap(), vec![5]);
    assert_eq!(ignored.encode("abd").unwrap(), vec![0, 1, 3]);
}

#[test]
fn ignore_merges_glm47() {
    let model = "zai-org/GLM-4.7";
    let hf = load_reference_tokenizer(model).unwrap();
    let ours = load_tokenizer(model).unwrap();

    let text = " имущества";
    let hf_ids = hf.encode(text, false).unwrap().get_ids().to_vec();
    let our_ids = ours.encode(text).unwrap();
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
    let our_enc = ours.encode(&text).unwrap();
    assert_eq!(
        our_enc,
        hf_enc,
        "ignore_merges random-decode mismatch: {} vs {} tokens",
        our_enc.len(),
        hf_enc.len()
    );
}

#[test]
fn correctness_qwen3() {
    let f = compare_encode_decode("Qwen/Qwen3-0.6B", CORPUS);
    assert!(f.is_empty(), "Qwen/Qwen3-0.6B:\n{}", f.join("\n"));
}

#[test]
fn correctness_mistral_nemo() {
    let f = compare_encode_decode("mistralai/Mistral-Nemo-Instruct-2407", CORPUS);
    assert!(
        f.is_empty(),
        "mistralai/Mistral-Nemo-Instruct-2407:\n{}",
        f.join("\n")
    );
}

#[test]
fn correctness_qwen3_nemotron() {
    let f = compare_encode_decode("nvidia/Qwen3-Nemotron-235B-A22B-GenRM", CORPUS);
    assert!(
        f.is_empty(),
        "nvidia/Qwen3-Nemotron-235B-A22B-GenRM:\n{}",
        f.join("\n")
    );
}

#[test]
fn correctness_kimi_k2_5() {
    let f = compare_encode_decode("hoangquan456/Kimi-K2.5", CORPUS);
    assert!(f.is_empty(), "hoangquan456/Kimi-K2.5:\n{}", f.join("\n"));
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
        let first = ours.encode(input).unwrap();
        let second = ours.encode(input).unwrap();
        assert_eq!(first, second, "cache inconsistency for {input:?}");
        let third = ours.encode(input).unwrap();
        assert_eq!(first, third, "cache inconsistency (3rd call) for {input:?}");
    }
}

#[test]
fn cache_consistency_byte_level() {
    let model = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
    let ours = load_tokenizer(model).unwrap();

    let input = "The year 2024 was notable for advances in AI. Models like \
                      GPT-4 and Claude demonstrated remarkable capabilities.";
    let baseline = ours.encode(input).unwrap();
    for i in 0..20 {
        let result = ours.encode(input).unwrap();
        assert_eq!(result, baseline, "byte-level cache drift on iteration {i}");
    }
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
    let f = compare_encode_decode("MiniMaxAI/MiniMax-M2.1", corpus);
    assert!(
        f.is_empty(),
        "MiniMaxAI/MiniMax-M2.1 added tokens:\n{}",
        f.join("\n")
    );
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
    let f = compare_encode_decode("deepseek-ai/DeepSeek-V3.2", corpus);
    assert!(
        f.is_empty(),
        "deepseek-ai/DeepSeek-V3.2 added tokens:\n{}",
        f.join("\n")
    );
}

#[test]
fn added_tokens_qwen3() {
    let corpus = &[
        "<|im_start|>system\nYou are a helpful assistant.<|im_end|>",
        "<|im_start|>user\nHello!<|im_end|>",
        "<|endoftext|>",
        "Plain text with no special tokens at all.",
    ];
    let f = compare_encode_decode("Qwen/Qwen3-0.6B", corpus);
    assert!(
        f.is_empty(),
        "Qwen/Qwen3-0.6B added tokens:\n{}",
        f.join("\n")
    );
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
fn added_tokens_qwen3vl_vision_sequence() {
    let corpus = &[
        "<|vision_start|><|image_pad|><|vision_end|>",
        "<|image_pad|>",
        "<|vision_start|><|image_pad|><|image_pad|><|image_pad|><|image_pad|><|vision_end|>",
        "<|vision_start|><|image_pad|><|vision_end|>\nDescribe this image.",
    ];
    let f = compare_encode_decode("Qwen/Qwen3.5-27B", corpus);
    assert!(
        f.is_empty(),
        "Qwen/Qwen3.5-27B VL vision sequence:\n{}",
        f.join("\n")
    );
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
    let f = compare_encode_decode("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16", corpus);
    assert!(
        f.is_empty(),
        "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16 added tokens:\n{}",
        f.join("\n")
    );
}

#[test]
fn long_input_correctness() {
    let model_name = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
    let hf = load_reference_tokenizer(model_name).unwrap();
    let ours = load_tokenizer(model_name).unwrap();

    let block = "The quick brown fox jumps over the lazy dog. \
                      Numbers: 42, 3.14, 1000. Code: fn main() {} \
                      Unicode: caf\u{00e9}, \u{4f60}\u{597d}. \
                      Special: @#$%^&*(). ";
    let input: String = block.repeat(100);
    assert!(input.len() > 8000);

    let hf_ids = hf.encode(input.as_str(), false).unwrap().get_ids().to_vec();
    let our_ids = ours.encode(&input).unwrap();
    assert_eq!(
        our_ids,
        hf_ids,
        "long input mismatch: {} vs {} tokens",
        our_ids.len(),
        hf_ids.len(),
    );
}

#[test]
fn newline_partitioned_ragged_matches_sequential() {
    let ours = load_tokenizer("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").unwrap();
    for input in [
        "Text and numbers 123. Unicode: café, 你好. Punctuation?!\r\n\n".repeat(1_200),
        "one long line with no partition boundary ".repeat(2_000),
    ] {
        let expected = ours.encode(&input).unwrap();
        let (actual, lengths) = ours.encode_batch_ragged(&[input.as_str()], false).unwrap();
        assert_eq!(lengths, [expected.len()]);
        assert_eq!(actual, expected);
    }
}

#[test]
fn long_input_correctness_minimax() {
    let model_name = "MiniMaxAI/MiniMax-M2.1";
    let hf = load_reference_tokenizer(model_name).unwrap();
    let ours = load_tokenizer(model_name).unwrap();

    let block = "The quick brown fox jumps over the lazy dog. \
                      Numbers: 42, 3.14, 1000. Code: fn main() {} \
                      Unicode: caf\u{00e9}, \u{4f60}\u{597d}. \
                      Special: @#$%^&*(). ";
    let input: String = block.repeat(100);

    let hf_ids = hf.encode(input.as_str(), false).unwrap().get_ids().to_vec();
    let our_ids = ours.encode(&input).unwrap();
    assert_eq!(
        our_ids,
        hf_ids,
        "long input mismatch: {} vs {} tokens",
        our_ids.len(),
        hf_ids.len(),
    );
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

fn compare_encode_decode_batched(
    model_name: &str,
    corpus: &[String],
    batch_size: usize,
    progress: bool,
) -> Vec<String> {
    let hf = load_reference_tokenizer(model_name)
        .unwrap_or_else(|e| panic!("{model_name}: HF load failed: {e}"));
    let ours = load_tokenizer(model_name)
        .unwrap_or_else(|e| panic!("{model_name}: fastokens load failed: {e}"));

    let total = corpus.len();
    let mut processed = 0usize;
    let mut failures = Vec::new();
    for chunk in corpus.chunks(batch_size) {
        let hf_results: Vec<Vec<u32>> = chunk
            .iter()
            .map(|input| {
                hf.encode(input.as_str(), false)
                    .unwrap_or_else(|e| panic!("{model_name}: HF encode: {e}"))
                    .get_ids()
                    .to_vec()
            })
            .collect();

        let our_results = match ours.encode_batch(chunk, false) {
            Ok(r) => r,
            Err(e) => {
                failures.push(format!("  encode_batch error: {e}"));
                continue;
            }
        };

        for (i, (hf_ids, our_ids)) in hf_results.iter().zip(our_results.iter()).enumerate() {
            let input = &chunk[i];
            let input_preview = {
                let mut end = input.len().min(80);
                while end < input.len() && !input.is_char_boundary(end) {
                    end += 1;
                }
                &input[..end]
            };

            if our_ids != hf_ids {
                failures.push(format!(
                    "  encode mismatch on {:?}: got {} tokens, expected {}\n\
                         \x20   ours: {:?}\n\
                         \x20   hf:   {:?}",
                    input_preview,
                    our_ids.len(),
                    hf_ids.len(),
                    &our_ids[..our_ids.len().min(20)],
                    &hf_ids[..hf_ids.len().min(20)],
                ));
            }

            if hf_ids.is_empty() || input.is_empty() {
                continue;
            }
            let hf_decoded = match hf.decode(hf_ids, false) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let our_decoded = match ours.decode(hf_ids, false) {
                Ok(d) => d,
                Err(e) => {
                    failures.push(format!("  decode error on {input_preview:?}: {e}"));
                    continue;
                }
            };
            if our_decoded != hf_decoded {
                failures.push(format!(
                    "  decode mismatch on {input_preview:?}:\n\
                         \x20   ours: {:?}\n\
                         \x20   hf:   {:?}",
                    &our_decoded[..our_decoded.len().min(100)],
                    &hf_decoded[..hf_decoded.len().min(100)],
                ));
            }
        }
        processed += chunk.len();
        if progress {
            eprint!(
                "\r  {model_name}: {processed}/{total} ({:.0}%)",
                processed as f64 / total as f64 * 100.0,
            );
        }
    }
    if progress {
        eprintln!();
    }
    failures
}

fn run_extended(model_name: &str) {
    let progress = std::env::var("EXTENDED_PROGRESS").is_ok();
    let corpus = extended_corpus();
    if progress {
        eprintln!(
            "  {model_name}: longbench ({} samples)",
            corpus.longbench.len()
        );
    }
    let mut failures = compare_encode_decode_batched(model_name, &corpus.longbench, 10, progress);
    if progress {
        eprintln!(
            "  {model_name}: sharegpt ({} samples)",
            corpus.sharegpt.len()
        );
    }
    failures.extend(compare_encode_decode_batched(
        model_name,
        &corpus.sharegpt,
        10,
        progress,
    ));
    assert!(
        failures.is_empty(),
        "{model_name} extended ({} failures):\n{}",
        failures.len(),
        failures.join("\n"),
    );
}

#[test]
#[ignore]
fn extended_minimax_m2_1() {
    run_extended("MiniMaxAI/MiniMax-M2.1");
}

#[test]
#[ignore]
fn extended_nemotron() {
    run_extended("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16");
}

#[test]
#[ignore]
fn extended_deepseek_v3_2() {
    run_extended("deepseek-ai/DeepSeek-V3.2");
}

#[test]
#[ignore]
fn extended_gpt_oss() {
    run_extended("openai/gpt-oss-120b");
}

#[test]
#[ignore]
fn extended_qwen3() {
    run_extended("Qwen/Qwen3-0.6B");
}

#[test]
#[ignore]
fn extended_mistral_nemo() {
    run_extended("mistralai/Mistral-Nemo-Instruct-2407");
}

#[test]
#[ignore]
fn extended_qwen3_nemotron() {
    run_extended("nvidia/Qwen3-Nemotron-235B-A22B-GenRM");
}

#[test]
#[ignore]
fn extended_mistral_large() {
    run_extended("mistralai/Mistral-Large-3-675B-Instruct-2512");
}

#[test]
#[ignore]
fn extended_qwen_small() {
    run_extended("Qwen/Qwen3-0.6B");
}

#[test]
fn encode_decode_roundtrip_all_models() {
    let texts = &[
        "Hello, world!",
        "日本語テスト",
        "The quick brown fox jumps over the lazy dog.",
        "fn main() { println!(\"hello\"); }",
        "   leading and trailing spaces   ",
        "line1\nline2\ttabbed",
        "0123456789",
        "🌍🎉✨",
    ];
    let failures: Vec<String> = HF_MODELS
        .iter()
        .flat_map(|model| {
            let tok = match load_tokenizer(model) {
                Ok(t) => t,
                Err(e) => return vec![format!("{model}: load error: {e}")],
            };
            texts
                .iter()
                .filter_map(|text| {
                    let ids = tok.encode_with_special_tokens(text, false).ok()?;
                    let decoded = tok.decode(&ids, false).ok()?;
                    if decoded != *text {
                        Some(format!("{model}: {text:?} → {decoded:?}"))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    assert!(
        failures.is_empty(),
        "encode→decode roundtrip failures:\n{}",
        failures.join("\n")
    );
}

#[test]
fn add_bos_token() {
    let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
    let bos_id = tok.token_to_id("<s>").expect("<s> not in vocabulary");

    let with_bos = tok.encode_with_special_tokens("hello world", true).unwrap();
    let without_bos = tok
        .encode_with_special_tokens("hello world", false)
        .unwrap();

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
    let with_flag = tok_q
        .encode_with_special_tokens("hello world", true)
        .unwrap();
    let without_flag = tok_q
        .encode_with_special_tokens("hello world", false)
        .unwrap();
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
    let ids_with = tok.encode_with_special_tokens(text, true).unwrap();
    let ids_without = tok.encode_with_special_tokens(text, false).unwrap();
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
fn decode_batch_matches_sequential() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    let sentences = &["first sentence", "second sentence", "日本語テスト", ""];
    let id_batches: Vec<Vec<u32>> = sentences
        .iter()
        .map(|s| tok.encode_with_special_tokens(s, false).unwrap())
        .collect();
    let refs: Vec<&[u32]> = id_batches.iter().map(Vec::as_slice).collect();
    let batch_out = tok.decode_batch(&refs, false).unwrap();
    for (out, expected) in batch_out.iter().zip(sentences.iter()) {
        assert_eq!(out, expected);
    }
}

#[test]
fn decode_tokens_matches_decode_by_id() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    for text in &["Hello, world!", "The quick brown fox", "🌍 emoji"] {
        let ids = tok.encode_with_special_tokens(text, false).unwrap();
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
fn empty_string_encode_decode() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    let ids = tok.encode_with_special_tokens("", false).unwrap();
    assert!(ids.is_empty(), "expected no tokens for empty string");
    assert_eq!(tok.decode(&[], false).unwrap(), "");
}

#[test]
fn encode_is_stable_after_decode() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    for text in &["hello world", "日本語テスト", "fn foo() {}"] {
        let ids1 = tok.encode_with_special_tokens(text, false).unwrap();
        let decoded = tok.decode(&ids1, false).unwrap();
        let ids2 = tok.encode_with_special_tokens(&decoded, false).unwrap();
        assert_eq!(ids1, ids2, "encode not stable after decode for {text:?}");
    }
}

#[test]
fn post_process_false_is_identity_all_models() {
    for model in HF_MODELS {
        let tok = load_tokenizer(model).unwrap();
        let payload = vec![100u32, 200, 300];
        let out = tok.post_process(payload.clone(), false);
        assert_eq!(
            out, payload,
            "{model}: post_process(false) should be identity"
        );
    }
}

#[test]
fn post_process_true_adds_special_tokens() {
    let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
    let payload = vec![10u32, 20, 30];
    let without = tok.post_process(payload.clone(), false);
    let with_sp = tok.post_process(payload.clone(), true);
    assert_eq!(without, payload);
    assert!(
        with_sp.len() > without.len(),
        "expected special tokens to be added"
    );
    assert!(
        with_sp
            .windows(payload.len())
            .any(|w| w == payload.as_slice()),
        "payload should appear contiguously in post-processed output"
    );
}

#[test]
fn decode_unknown_id_is_skipped() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    assert_eq!(tok.decode(&[u32::MAX], false).unwrap(), "");
}

#[test]
fn decode_mixed_valid_and_unknown_ids() {
    let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
    let valid = tok.encode_with_special_tokens("hello", false).unwrap();
    let mut mixed = valid.clone();
    mixed.push(u32::MAX);
    mixed.extend(tok.encode_with_special_tokens(" world", false).unwrap());
    let expected = tok.decode(&valid, false).unwrap()
        + &tok
            .decode(
                &tok.encode_with_special_tokens(" world", false).unwrap(),
                false,
            )
            .unwrap();
    assert_eq!(tok.decode(&mixed, false).unwrap(), expected);
}

#[test]
fn token_id_roundtrip_all_models() {
    let probe_ids = [0u32, 1, 2, 100, 1000, 10_000];
    let failures: Vec<String> = HF_MODELS
        .iter()
        .flat_map(|model| {
            let tok = match load_tokenizer(model) {
                Ok(t) => t,
                Err(e) => return vec![format!("{model}: load error: {e}")],
            };
            probe_ids
                .iter()
                .filter_map(|&id| {
                    let token = tok.id_to_token(id)?;
                    let back = tok.token_to_id(token)?;
                    if back != id {
                        Some(format!("{model}: id {id} → {token:?} → {back}"))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    assert!(
        failures.is_empty(),
        "id↔token roundtrip failures:\n{}",
        failures.join("\n")
    );
}

const STREAM_MODEL: &str = "Qwen/Qwen3-0.6B";

fn stream_tok() -> Tokenizer {
    load_tokenizer(STREAM_MODEL).expect("failed to load tokenizer")
}

fn stream_collect(tok: &Tokenizer, ids: &[u32], skip: bool) -> (String, usize) {
    let mut buf = Vec::new();
    let mut prefix = String::new();
    let mut prefix_index = 0usize;
    let mut out = String::new();
    for &id in ids {
        let chunk: Option<String> = decode_stream_step(
            tok,
            vec![id],
            skip,
            &mut buf,
            &mut prefix,
            &mut prefix_index,
        )
        .unwrap();
        if let Some(c) = chunk {
            out.push_str(&c);
        }
    }
    (out, buf.len())
}

#[test]
fn decode_stream_reconstructs_ascii() {
    let tok = stream_tok();
    let text = "Hello, world! This is a streaming decode test.";
    let ids = tok.encode_with_special_tokens(text, false).unwrap();
    let (decoded, _) = stream_collect(&tok, &ids, false);
    assert_eq!(decoded, text);
}

#[test]
fn decode_stream_reconstructs_unicode() {
    let tok = stream_tok();
    let text = "日本語テスト: こんにちは 🌍 — привет мир";
    let ids = tok.encode_with_special_tokens(text, false).unwrap();
    let (decoded, _) = stream_collect(&tok, &ids, false);
    assert_eq!(decoded, text);
}

#[test]
fn decode_stream_reconstructs_code() {
    let tok = stream_tok();
    let text = r#"fn main() { println!("hello"); }"#;
    let ids = tok.encode_with_special_tokens(text, false).unwrap();
    let (decoded, _) = stream_collect(&tok, &ids, false);
    assert_eq!(decoded, text);
}

#[test]
fn decode_stream_empty_ids_no_output() {
    let tok = stream_tok();
    let (decoded, buf_len) = stream_collect(&tok, &[], false);
    assert!(decoded.is_empty());
    assert_eq!(buf_len, 0);
}

#[test]
fn decode_stream_single_token() {
    let tok = stream_tok();
    let ids = tok.encode_with_special_tokens("hello", false).unwrap();
    assert!(!ids.is_empty());
    let (decoded, _) = stream_collect(&tok, &ids[..1], false);
    assert!(!decoded.is_empty());
}

#[test]
fn decode_stream_batch_step_matches_sequential() {
    let tok = stream_tok();
    let text = "The quick brown fox jumps over the lazy dog.";
    let ids = tok.encode_with_special_tokens(text, false).unwrap();
    let (sequential, _) = stream_collect(&tok, &ids, false);
    let mut buf = Vec::new();
    let mut prefix = String::new();
    let mut prefix_index = 0usize;
    let batch: String = decode_stream_step(
        &tok,
        ids.clone(),
        false,
        &mut buf,
        &mut prefix,
        &mut prefix_index,
    )
    .unwrap()
    .unwrap_or_default();
    assert_eq!(sequential, batch);
}

#[test]
fn decode_stream_pre_seeded_only_returns_new_tokens() {
    let tok = stream_tok();
    let prompt = "The capital of France is";
    let cont = " Paris.";
    let prompt_ids = tok.encode_with_special_tokens(prompt, false).unwrap();
    let cont_ids = tok.encode_with_special_tokens(cont, false).unwrap();
    let mut buf = prompt_ids.clone();
    let mut prefix = String::new();
    let mut prefix_index = 0usize;
    let mut out = String::new();
    for &id in &cont_ids {
        let chunk: Option<String> = decode_stream_step(
            &tok,
            vec![id],
            false,
            &mut buf,
            &mut prefix,
            &mut prefix_index,
        )
        .unwrap();
        if let Some(c) = chunk {
            out.push_str(&c);
        }
    }
    assert_eq!(out, cont);
}

#[test]
fn decode_stream_skip_special_tokens() {
    let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
    let text = "hello";
    let ids_with = tok.encode_with_special_tokens(text, true).unwrap();
    let ids_without = tok.encode_with_special_tokens(text, false).unwrap();
    assert!(
        ids_with.len() > ids_without.len(),
        "expected BOS/EOS tokens"
    );
    let (with_sp, _) = stream_collect(&tok, &ids_with, false);
    let (no_sp, _) = stream_collect(&tok, &ids_with, true);
    assert_eq!(no_sp, text);
    assert!(with_sp.contains(&no_sp));
}

#[test]
fn decode_stream_buffer_does_not_grow_unboundedly() {
    let tok = stream_tok();
    let text = "word ".repeat(80);
    let ids = tok.encode_with_special_tokens(text.trim(), false).unwrap();
    let (_, final_buf_len) = stream_collect(&tok, &ids, false);
    assert!(
        final_buf_len < 10,
        "buffer grew to {final_buf_len} entries after {} tokens",
        ids.len()
    );
}

#[test]
fn decode_stream_chunks_are_non_empty_and_concatenate() {
    let tok = stream_tok();
    let text = "one two three four five six seven eight nine ten";
    let ids = tok.encode_with_special_tokens(text, false).unwrap();
    let mut buf = Vec::new();
    let mut prefix = String::new();
    let mut prefix_index = 0usize;
    let mut chunks: Vec<String> = Vec::new();
    for &id in &ids {
        let chunk: Option<String> = decode_stream_step(
            &tok,
            vec![id],
            false,
            &mut buf,
            &mut prefix,
            &mut prefix_index,
        )
        .unwrap();
        if let Some(c) = chunk {
            assert!(!c.is_empty(), "stream emitted an empty chunk");
            chunks.push(c);
        }
    }
    assert_eq!(chunks.concat(), text);
}

#[test]
fn decode_stream_unknown_id_does_not_error() {
    let tok = stream_tok();
    let mut buf = Vec::new();
    let mut prefix = String::new();
    let mut prefix_index = 0usize;
    let result = decode_stream_step(
        &tok,
        vec![u32::MAX],
        false,
        &mut buf,
        &mut prefix,
        &mut prefix_index,
    );
    assert!(result.is_ok(), "expected Ok, got {result:?}");
}

#[test]
fn decode_stream_invalid_prefix_error_message() {
    let tok = stream_tok();
    let ids = tok.encode_with_special_tokens("hello", false).unwrap();
    let mut buf = ids.clone();
    let mut prefix = "ZZZZZZZ".to_string();
    let mut prefix_index = 0usize;
    let result: Result<Option<String>, String> = decode_stream_step(
        &tok,
        vec![*ids.last().unwrap()],
        false,
        &mut buf,
        &mut prefix,
        &mut prefix_index,
    );
    if let Err(msg) = result {
        assert!(
            msg.starts_with("Invalid prefix encountered"),
            "unexpected error: {msg:?}"
        );
    }
}
