use crate::common::{Comparison, tokenizer_json_path};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use snaptokens::Tokenizer;
use snaptokens::json_structs::{NormalizerConfig, TokenizerJson};
use snaptokens::normalizers::Precompiled;
use std::fs;

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
        // A combining mark can join the preceding space's grapheme; partition cuts must preserve it.
        "x \u{301}accent e\u{301}tude words here pad pad pad ".repeat(800),
    ];
    for input in &inputs {
        assert!(input.len() > 16 * 1024);
    }
    Comparison::new(model).assert_parity(&inputs, false);
}

#[test]
fn t5_unigram_normalized_partitions_match_hugging_face() {
    // Wrapping Precompiled in Sequence disables raw partitioning and exercises normalized partitioning.
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
fn t5_precompiled_normalizer_matches_its_reference_charsmap() {
    let path = tokenizer_json_path("google-t5/t5-small").unwrap();
    let tokenizer_json: TokenizerJson = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let charsmap = precompiled_charsmap(tokenizer_json.normalizer.as_ref().unwrap())
        .unwrap()
        .to_owned();
    let reference_bytes = STANDARD.decode(&charsmap).unwrap();
    let reference = spm_precompiled::Precompiled::from(&reference_bytes).unwrap();
    let optimized = Precompiled::from_config(charsmap).unwrap();
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
