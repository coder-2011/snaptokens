use serde_json::{Value, json};
use snaptokens::Tokenizer;

fn config(fused: bool, normalized_token: bool, normalizer: Value) -> Value {
    let mut alphabet: Vec<_> = tokenizers::pre_tokenizers::byte_level::ByteLevel::alphabet()
        .into_iter()
        .collect();
    alphabet.sort_unstable();
    let vocab: serde_json::Map<_, _> = alphabet
        .into_iter()
        .enumerate()
        .map(|(id, character)| (character.to_string(), json!(id)))
        .collect();
    let mut added_tokens = vec![json!({
        "id": 256, "content": "[RAW]", "normalized": false,
        "single_word": false, "lstrip": false, "rstrip": false, "special": false
    })];
    if normalized_token {
        added_tokens.push(json!({
            "id": 257, "content": "e\u{301}!", "normalized": true,
            "single_word": false, "lstrip": false, "rstrip": false, "special": false
        }));
    }
    let split = json!({
        "type": "Split", "pattern": { "Regex": "\\s+|\\S+" },
        "behavior": "Isolated", "invert": false
    });
    let first = if fused {
        split
    } else {
        json!({ "type": "Sequence", "pretokenizers": [split] })
    };
    json!({
        "added_tokens": added_tokens,
        "normalizer": normalizer,
        "pre_tokenizer": {
            "type": "Sequence",
            "pretokenizers": [first, {
                "type": "ByteLevel", "add_prefix_space": false,
                "trim_offsets": false, "use_regex": false
            }]
        },
        "model": { "type": "BPE", "vocab": vocab, "merges": [] }
    })
}

#[test]
fn generic_and_fused_preparation_match_huggingface() {
    let long = format!("[RAW]{}[RAW]", "e\u{301} 中 🦀 ".repeat(300));
    let inputs = [
        "",
        "plain text",
        "[RAW]",
        "[RAW][RAW]",
        "e\u{301}![RAW]é!",
        "e\u{301} 中[RAW]🦀 e\u{301}",
        "erase[RAW]erase",
        long.as_str(),
    ];
    for normalizer in [
        Value::Null,
        json!({ "type": "NFC" }),
        json!({ "type": "Replace", "pattern": { "String": "erase" }, "content": "" }),
    ] {
        for normalized_token in [false, true] {
            for fused in [false, true] {
                let value = config(fused, normalized_token, normalizer.clone());
                let ours = Tokenizer::from_json(value.clone()).unwrap();
                let reference =
                    tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&value).unwrap()).unwrap();
                let expected: Vec<Vec<u32>> = inputs
                    .iter()
                    .map(|input| reference.encode(*input, false).unwrap().get_ids().to_vec())
                    .collect();
                for (input, ids) in inputs.iter().zip(&expected) {
                    assert_eq!(
                        &ours.encode(input).unwrap(),
                        ids,
                        "fused={fused}, normalized_token={normalized_token}, input={input:?}"
                    );
                }
                assert_eq!(ours.encode_batch(&inputs, false).unwrap(), expected);
                let (ids, lengths) = ours.encode_batch_ragged(&inputs, false).unwrap();
                assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
                assert_eq!(ids, expected.into_iter().flatten().collect::<Vec<_>>());
            }
        }
    }
}
