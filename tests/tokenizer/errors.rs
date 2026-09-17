use serde_json::json;
use snaptokens::Tokenizer;

#[test]
fn public_pre_tokenizer_regex_error_is_exported() {
    let tokenizer = Tokenizer::from_json(json!({
        "model":{"type":"BPE", "vocab":{"a":0,"b":1,"ab":2,"z":3}, "merges":[["a","b"]]},
        "pre_tokenizer":{"type":"Sequence", "pretokenizers":[
            {"type":"Split", "pattern":{"Regex":r"z|(?i)(a|b|ab)*(?>c)|a"},
             "behavior":"Isolated", "invert":false},
            {"type":"ByteLevel", "add_prefix_space":false, "use_regex":false}
        ]}
    }))
    .unwrap();
    let input = format!("z{}", "Ab".repeat(20));
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
}
