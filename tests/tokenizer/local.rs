use snaptokens::Tokenizer;

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

    assert_eq!(tokenizer.encode("éê", false).unwrap(), vec![1, 4, 3]);
    for limit in 0..=4 {
        use snaptokens::TruncationDirection::{Left, Right};
        let full = [1, 4, 3];
        assert_eq!(
            tokenizer.encode_with_limit("éê", limit, Right).unwrap(),
            (full[..limit.min(3)].to_vec(), limit < 3)
        );
        assert_eq!(
            tokenizer.encode_with_limit("éê", limit, Left).unwrap(),
            (full[3usize.saturating_sub(limit)..].to_vec(), limit < 3)
        );
    }
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
    assert_eq!(merged.encode("ab", false).unwrap(), vec![0, 1]);

    let ignored = tokenizer(true);
    assert_eq!(ignored.encode("abc", false).unwrap(), vec![5]);
    assert_eq!(ignored.encode("abd", false).unwrap(), vec![0, 1, 3]);
}

#[test]
fn vocabulary_and_decode_accessors_preserve_added_tokens() {
    let tokenizer = Tokenizer::from_json(serde_json::json!({
        "model": {"type": "BPE", "vocab": {"a":0, "b":1}, "merges":[]},
        "added_tokens": [
            {"id":2, "content":"<special>", "special":true},
            {"id":3, "content":"<plain>", "special":false}
        ],
        "decoder": {"type":"Fuse"}
    }))
    .unwrap();
    assert_eq!(tokenizer.vocab_size(), 4);
    for (id, text) in [(0, "a"), (1, "b"), (2, "<special>"), (3, "<plain>")] {
        assert_eq!(tokenizer.id_to_token(id), Some(text));
        assert_eq!(tokenizer.token_to_id(text), Some(id));
    }
    let added = tokenizer.added_tokens().unwrap();
    assert_eq!(added.token_to_id("<plain>"), Some(3));
    assert_eq!(added.id_to_token(3), Some("<plain>"));
    assert!(tokenizer.is_special_token(2));
    assert!(!tokenizer.is_special_token(3));
    assert_eq!(tokenizer.decode(&[u32::MAX], false).unwrap(), "");
    assert_eq!(
        tokenizer.decode(&[0, u32::MAX, 2, 1], false).unwrap(),
        "a<special>b"
    );
    assert_eq!(tokenizer.decode(&[0, u32::MAX, 2, 1], true).unwrap(), "ab");
    assert_eq!(
        tokenizer.decode_batch(&[&[], &[0, 1], &[2]], true).unwrap(),
        ["", "ab", ""]
    );
    assert_eq!(
        tokenizer
            .decode_tokens(vec!["a".into(), "b".into()])
            .unwrap(),
        "ab"
    );
}

#[test]
fn post_processing_applies_only_when_requested() {
    for processor in [
        serde_json::Value::Null,
        serde_json::json!({
            "type":"TemplateProcessing", "single":[
                {"SpecialToken":{"id":"<s>","type_id":0}},
                {"Sequence":{"id":"A","type_id":0}}
            ], "special_tokens":{"<s>":{"id":"<s>","ids":[1],"tokens":["<s>"]}}
        }),
    ] {
        let has_processor = !processor.is_null();
        let tokenizer = Tokenizer::from_json(serde_json::json!({
            "model":{"type":"BPE","vocab":{"a":0,"<s>":1},"merges":[]},
            "post_processor":processor
        }))
        .unwrap();
        for input in ["", "a"] {
            let raw = tokenizer.encode(input, false).unwrap();
            assert_eq!(tokenizer.post_process(raw.clone(), false), raw);
            let mut expected = if has_processor { vec![1] } else { vec![] };
            expected.extend(&raw);
            assert_eq!(tokenizer.post_process(raw, true), expected);
            assert_eq!(tokenizer.encode(input, true).unwrap(), expected);
        }
    }
}
