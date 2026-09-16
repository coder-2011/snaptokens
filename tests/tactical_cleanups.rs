use serde_json::json;
use snaptokens::Tokenizer;

#[test]
fn decoding_preserves_added_token_precedence_and_unknown_id_filtering() {
    for decoder in [json!(null), json!({"type": "Fuse"})] {
        let tokenizer = Tokenizer::from_json(json!({
            "model": {"type": "BPE", "vocab": {"a": 0, "b": 1}, "merges": []},
            "added_tokens": [
                {"id": 0, "content": "override", "special": true},
                {"id": 2, "content": "_"}
            ],
            "decoder": decoder
        }))
        .unwrap();
        assert_eq!(
            tokenizer.decode(&[0, 1, u32::MAX, 2], false).unwrap(),
            "overrideb_"
        );
        assert_eq!(tokenizer.decode(&[0, 1, u32::MAX, 2], true).unwrap(), "b_");
        assert_eq!(tokenizer.decode(&[u32::MAX], false).unwrap(), "");
        for tokens in [vec![], vec!["é".into()], vec!["a".into(), "中".into()]] {
            let expected = tokens.concat();
            assert_eq!(tokenizer.decode_tokens(tokens).unwrap(), expected);
        }
    }
}
