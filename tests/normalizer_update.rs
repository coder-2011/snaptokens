use serde_json::{Value, json};
use snaptokens::{Normalizer, NormalizerConfig, Tokenizer};

fn config(normalizer: Value) -> Value {
    json!({
        "added_tokens": [
            { "id": 6, "content": "e\u{301}!", "normalized": true,
              "single_word": false, "lstrip": false, "rstrip": false, "special": false },
            { "id": 7, "content": "e\u{301}?", "normalized": false,
              "single_word": false, "lstrip": false, "rstrip": false, "special": false }
        ],
        "normalizer": normalizer,
        "model": {
            "type": "BPE",
            "vocab": { "e": 0, "é": 1, "\u{301}": 2, "!": 3, "?": 4, "x": 5 },
            "merges": []
        }
    })
}

fn normalizer(config: Value) -> Option<Normalizer> {
    if config.is_null() {
        None
    } else {
        let config: NormalizerConfig = serde_json::from_value(config).unwrap();
        Some(Normalizer::from_config(config).unwrap())
    }
}

#[test]
fn replacement_matches_fresh_construction() {
    let mut tokenizer = Tokenizer::from_json(config(Value::Null)).unwrap();
    let inputs = ["e\u{301}!", "é!", "e\u{301}?", "é?", "x", ""];
    for next in [
        json!({ "type": "NFC" }),
        json!({ "type": "Replace", "pattern": { "String": "!" }, "content": "?" }),
        Value::Null,
    ] {
        tokenizer.set_normalizer(normalizer(next.clone())).unwrap();
        let fresh = Tokenizer::from_json(config(next.clone())).unwrap();
        let reference =
            tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config(next)).unwrap()).unwrap();
        for input in inputs {
            let ids = tokenizer.encode(input, false).unwrap();
            assert_eq!(ids, fresh.encode(input, false).unwrap(), "{input:?}");
            assert_eq!(
                ids,
                reference.encode(input, false).unwrap().get_ids(),
                "{input:?}"
            );
        }
        assert_eq!(tokenizer.encode("e\u{301}?", false).unwrap(), vec![7]);
    }
}

#[test]
fn failed_replacement_preserves_consistent_state() {
    let mut tokenizer = Tokenizer::from_json(config(json!({ "type": "NFC" }))).unwrap();
    let before = tokenizer.encode("é!", false).unwrap();
    let invalid = json!({
        "type": "Replace", "pattern": { "String": "e\u{301}!" }, "content": ""
    });
    assert!(tokenizer.set_normalizer(normalizer(invalid)).is_err());
    assert_eq!(tokenizer.encode("é!", false).unwrap(), before);
    assert_eq!(tokenizer.encode("e\u{301}!", false).unwrap(), before);
}
