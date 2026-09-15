#![no_main]

use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;
use std::sync::LazyLock;

static TOKENIZERS: LazyLock<Vec<Tokenizer>> = LazyLock::new(|| {
    vec![
        // GPT-2 style
        make_tokenizer(r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+"),
        // Llama style
        make_tokenizer(concat!(
            r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]+",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+",
            r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]*",
            r"|\p{N}{1,3}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
            r"|\s*[\r\n]+|\s+(?!\S)|\s+",
        )),
        // Qwen style
        make_tokenizer(concat!(
            r"(?i:'s|'t|'re|'ve|'m|'ll|'d)|",
            r"[^\r\n\p{L}\p{N}]?\p{L}+|",
            r"\p{N}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+",
        )),
        // ByteLevel (no pre-tokenizer pattern)
        make_tokenizer_bytelevel(),
    ]
});

fn make_tokenizer(pattern: &str) -> Tokenizer {
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": {
            "type": "Split",
            "pattern": { "Regex": pattern },
            "behavior": "Isolated",
            "invert": false
        },
        "model": {
            "type": "BPE",
            "vocab": build_vocab(),
            "merges": ["a b", "c d", "ab cd", "e f", "Ġ a", "Ġa b"],
            "byte_fallback": true
        },
        "decoder": {
            "type": "Sequence",
            "decoders": [{"type": "ByteFallback"}, {"type": "Fuse"}]
        }
    })).unwrap()
}

fn make_tokenizer_bytelevel() -> Tokenizer {
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": { "type": "ByteLevel", "add_prefix_space": true },
        "model": {
            "type": "BPE",
            "vocab": build_vocab(),
            "merges": ["a b", "c d", "ab cd", "e f", "Ġ a", "Ġa b"]
        },
        "decoder": { "type": "ByteLevel" }
    })).unwrap()
}

fn build_vocab() -> serde_json::Value {
    let mut vocab = serde_json::Map::new();
    let base_tokens = ["a", "b", "c", "d", "e", "f", "ab", "cd", "abcd", "ef", "Ġ", "Ġa", "Ġab"];
    for (i, t) in base_tokens.iter().enumerate() {
        vocab.insert(t.to_string(), (i as u32).into());
    }
    let base = base_tokens.len() as u32;
    for byte in 0u8..=255 {
        vocab.insert(format!("<0x{:02X}>", byte), (base + byte as u32).into());
    }
    serde_json::Value::Object(vocab)
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        for tokenizer in TOKENIZERS.iter() {
            let _ = tokenizer.encode(s, false);
            let _ = tokenizer.encode(s, true);
        }
    }
});
