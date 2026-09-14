use serde_json::{Value, json};

use super::*;

fn fixture(merged: bool) -> Value {
    let merges = if merged {
        json!([["a", "b"]])
    } else {
        json!([])
    };
    json!({
        "added_tokens": [
            {
                "id": 5,
                "content": "<s>",
                "single_word": true,
                "lstrip": true,
                "rstrip": true,
                "normalized": false,
                "special": true
            },
            {
                "id": 6,
                "content": "e\u{301}!",
                "single_word": false,
                "lstrip": false,
                "rstrip": false,
                "normalized": true,
                "special": false
            }
        ],
        "normalizer": {"type": "NFC"},
        "pre_tokenizer": null,
        "model": {
            "type": "BPE",
            "vocab": {
                "a": 0,
                "b": 1,
                "ab": 2,
                "<0xC3>": 3,
                "<0xA9>": 4
            },
            "merges": merges,
            "byte_fallback": true,
            "ignore_merges": merged
        },
        "post_processor": {
            "type": "TemplateProcessing",
            "single": [
                {"SpecialToken": {"id": "<s>", "type_id": 0}},
                {"Sequence": {"id": "A", "type_id": 0}}
            ],
            "pair": [],
            "special_tokens": {
                "<s>": {"id": "<s>", "ids": [5], "tokens": ["<s>"]}
            }
        },
        "decoder": {
            "type": "Sequence",
            "decoders": [{"type": "ByteFallback"}, {"type": "Fuse"}]
        }
    })
}

fn test_directory(name: &str) -> PathBuf {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "snaptokens-{name}-{}-{counter}",
        std::process::id()
    ));
    fs::create_dir(&path).unwrap();
    path
}

fn write_fixture(path: &Path, merged: bool) {
    fs::write(path, serde_json::to_vec(&fixture(merged)).unwrap()).unwrap();
}

#[test]
fn round_trip_preserves_pipeline_and_direct_loads() {
    let directory = test_directory("tkz-round-trip");
    let json_path = directory.join("tokenizer.json");
    let tkz_path = directory.join("tokenizer.tkz");
    write_fixture(&json_path, true);

    let json = Tokenizer::load_file(&json_path).unwrap();
    let disabled = Tokenizer::load_file(&json_path).unwrap();
    assert!(!tkz_path.exists());
    assert_eq!(json.encode("ab").unwrap(), disabled.encode("ab").unwrap());

    let cached = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
    let direct = Tokenizer::load_file_with_tkz_cache(&tkz_path).unwrap();
    assert!(tkz_path.is_file());

    for input in ["ab", "é", "e\u{301}!", "  <s>  "] {
        let expected = json.encode(input).unwrap();
        assert_eq!(cached.encode(input).unwrap(), expected);
        assert_eq!(direct.encode(input).unwrap(), expected);
    }
    let expected = json.encode_with_special_tokens("ab", true).unwrap();
    assert_eq!(
        cached.encode_with_special_tokens("ab", true).unwrap(),
        expected
    );
    assert_eq!(direct.decode(&[3, 4], false).unwrap(), "é");
    assert!(direct.is_special_token(5));

    fs::remove_file(&json_path).unwrap();
    let sidecar_only = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
    assert_eq!(sidecar_only.encode("é").unwrap(), vec![3, 4]);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn rebuilds_invalid_sidecars_and_handles_concurrent_creation() {
    let directory = test_directory("tkz-recovery");
    let json_path = directory.join("tokenizer.json");
    let tkz_path = directory.join("tokenizer.tkz");
    write_fixture(&json_path, true);
    Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();

    write_fixture(&json_path, false);
    let refreshed = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
    assert_eq!(refreshed.encode("ab").unwrap(), vec![0, 1]);

    fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
    let recovered = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
    assert_eq!(recovered.encode("ab").unwrap(), vec![0, 1]);

    fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
    fs::remove_file(&json_path).unwrap();
    assert!(Tokenizer::load_file_with_tkz_cache(&tkz_path).is_err());
    fs::remove_dir_all(&directory).unwrap();

    let directory = test_directory("tkz-concurrent");
    let json_path = directory.join("tokenizer.json");
    let tkz_path = directory.join("tokenizer.tkz");
    write_fixture(&json_path, true);
    let threads = (0..4)
        .map(|_| {
            let json_path = json_path.clone();
            std::thread::spawn(move || {
                Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
            })
        })
        .collect::<Vec<_>>();
    for thread in threads {
        thread.join().unwrap();
    }
    let final_tokenizer = Tokenizer::load_file_with_tkz_cache(&tkz_path).unwrap();
    assert_eq!(final_tokenizer.encode("ab").unwrap(), vec![2]);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn unigram_json_refuses_tkz_without_creating_a_sidecar() {
    let directory = test_directory("tkz-unigram");
    let json_path = directory.join("tokenizer.json");
    let tkz_path = directory.join("tokenizer.tkz");
    fs::write(
        &json_path,
        serde_json::to_vec(&json!({
            "normalizer": null,
            "pre_tokenizer": null,
            "model": {
                "type": "Unigram",
                "unk_id": 0,
                "vocab": [["<unk>", 0.0], ["a", 1.0]]
            },
            "post_processor": null,
            "decoder": null
        }))
        .unwrap(),
    )
    .unwrap();

    let error = match Tokenizer::load_file_with_tkz_cache(&json_path) {
        Ok(_) => panic!("Unigram JSON must not create a BPE .tkz sidecar"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("Unigram tokenizers cannot use .tkz caching yet")
    );
    assert!(!tkz_path.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn native_sentencepiece_paths_fail_explicitly() {
    let error = match Tokenizer::load_file(Path::new("fixture.model")) {
        Ok(_) => panic!("native SentencePiece paths must be rejected before reading"),
        Err(error) => error,
    };
    assert_eq!(
        error.to_string(),
        "unsupported tokenizer format: native SentencePiece .model files are not supported; export a compatible tokenizer.json"
    );
}
