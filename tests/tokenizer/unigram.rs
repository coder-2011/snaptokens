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
fn t5_unigram_st_preserves_precompiled_pipeline_and_rows() {
    let source = tokenizer_json_path("google-t5/t5-small").unwrap();
    let directory = std::env::temp_dir().join(format!("snaptokens-t5-st-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let json_path = directory.join("tokenizer.json");
    fs::copy(&source, &json_path).unwrap();
    let cached = Tokenizer::load_file_with_st_cache(&json_path).unwrap();
    let direct = Tokenizer::load_file_with_st_cache(json_path.with_extension("st")).unwrap();
    let reference = tokenizers::Tokenizer::from_file(&source).unwrap();
    let inputs = [
        String::new(),
        "① ﬁ Å ＡＢＣ\u{00a0}x café 東京 😀 <extra_id_0>".to_owned(),
        "x \u{301}accent e\u{301}tude words\r\nwide\u{3000}space ".repeat(900),
        "prefix <extra_id_1> answer <extra_id_2> suffix ".repeat(600),
    ];
    fs::remove_file(&json_path).unwrap();
    let sidecar_only = Tokenizer::load_file_with_st_cache(&json_path).unwrap();
    for tokenizer in [cached, direct, sidecar_only] {
        for special in [false, true] {
            let expected: Vec<_> = inputs
                .iter()
                .map(|input| {
                    reference
                        .encode(input.as_str(), special)
                        .unwrap()
                        .get_ids()
                        .to_vec()
                })
                .collect();
            for (input, ids) in inputs.iter().zip(&expected) {
                assert_eq!(tokenizer.encode(input, special).unwrap(), *ids);
                for skip in [false, true] {
                    assert_eq!(
                        tokenizer.decode(ids, skip).unwrap(),
                        reference.decode(ids, skip).unwrap()
                    );
                }
            }
            assert_eq!(tokenizer.encode_batch(&inputs, special).unwrap(), expected);
            let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, special).unwrap();
            assert_eq!(ids, expected.concat());
            assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
        }
        for (text, id) in reference.get_vocab(false) {
            assert_eq!(tokenizer.token_to_id(&text), Some(id));
            assert_eq!(tokenizer.id_to_token(id), Some(text.as_str()));
        }
    }
    fs::remove_dir_all(directory).unwrap();
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
