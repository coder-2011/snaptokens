#![no_main]

use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;
use std::sync::LazyLock;

static TOKENIZER: LazyLock<Tokenizer> = LazyLock::new(|| {
    let mut vocab = serde_json::Map::new();
    let base_tokens = [
        "a", "b", "c", "d", "ab", "cd", "abcd", "Ġ", "Ġa", "<s>", "</s>",
    ];
    for (i, t) in base_tokens.iter().enumerate() {
        vocab.insert(t.to_string(), (i as u32).into());
    }
    let base = base_tokens.len() as u32;
    for byte in 0u8..=255 {
        vocab.insert(format!("<0x{:02X}>", byte), (base + byte as u32).into());
    }

    Tokenizer::from_json(serde_json::json!({
        "model": {
            "type": "BPE",
            "vocab": serde_json::Value::Object(vocab),
            "merges": ["a b", "c d", "ab cd"],
            "byte_fallback": true
        },
        "added_tokens": [
            {"id": 9, "content": "<s>", "special": true},
            {"id": 10, "content": "</s>", "special": true}
        ],
        "decoder": {
            "type": "Sequence",
            "decoders": [{"type": "ByteFallback"}, {"type": "Fuse"}]
        }
    }))
    .unwrap()
});

fuzz_target!(|data: &[u8]| {
    if data.len() % 4 != 0 || data.is_empty() {
        return;
    }
    let ids: Vec<u32> = data
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    let _ = TOKENIZER.decode(&ids, false);
    let _ = TOKENIZER.decode(&ids, true);
});
