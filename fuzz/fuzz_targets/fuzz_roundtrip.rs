#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;
use std::sync::LazyLock;

#[derive(Arbitrary, Debug)]
struct RoundtripInput<'a> {
    text: &'a str,
    tokenizer_idx: u8,
    skip_special: bool,
}

static TOKENIZERS: LazyLock<Vec<Tokenizer>> = LazyLock::new(|| {
    vec![
        make_gpt2_style(),
        make_llama_style(),
        make_qwen_style(),
        make_bytelevel(),
    ]
});

fn make_gpt2_style() -> Tokenizer {
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": {
            "type": "Split",
            "pattern": { "Regex": r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+" },
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

fn make_llama_style() -> Tokenizer {
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": {
            "type": "Split",
            "pattern": { "Regex": concat!(
                r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*",
                r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]+",
                r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+",
                r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]*",
                r"|\p{N}{1,3}",
                r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
                r"|\s*[\r\n]+|\s+(?!\S)|\s+",
            ) },
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
    }))
    .unwrap()
}

fn make_qwen_style() -> Tokenizer {
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": {
            "type": "Split",
            "pattern": { "Regex": concat!(
                r"(?i:'s|'t|'re|'ve|'m|'ll|'d)|",
                r"[^\r\n\p{L}\p{N}]?\p{L}+|",
                r"\p{N}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+",
            ) },
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
    }))
    .unwrap()
}

fn make_bytelevel() -> Tokenizer {
    // ByteLevel needs every byte's mapped character to preserve arbitrary UTF-8.
    let mut vocab = serde_json::Map::new();
    let mut next = 256;
    for byte in 0u8..=255 {
        let codepoint = if matches!(byte, b'!'..=b'~' | 0xA1..=0xAC | 0xAE..=0xFF) {
            u32::from(byte)
        } else {
            let codepoint = next;
            next += 1;
            codepoint
        };
        vocab.insert(char::from_u32(codepoint).unwrap().to_string(), byte.into());
    }
    for token in ["ab", "cd", "abcd", "ef", "Ġa", "Ġab"] {
        let id = vocab.len() as u32;
        vocab.insert(token.to_owned(), id.into());
    }
    Tokenizer::from_json(serde_json::json!({
        "pre_tokenizer": { "type": "ByteLevel", "add_prefix_space": false },
        "model": {
            "type": "BPE",
            "vocab": vocab,
            "merges": ["a b", "c d", "ab cd", "e f", "Ġ a", "Ġa b"]
        },
        "decoder": { "type": "ByteLevel" }
    }))
    .unwrap()
}

fn build_vocab() -> serde_json::Value {
    let mut vocab = serde_json::Map::new();
    let base_tokens = [
        "a", "b", "c", "d", "e", "f", "ab", "cd", "abcd", "ef", "Ġ", "Ġa", "Ġab",
    ];
    for (i, t) in base_tokens.iter().enumerate() {
        vocab.insert(t.to_string(), (i as u32).into());
    }
    let base = base_tokens.len() as u32;
    for byte in 0u8..=255 {
        vocab.insert(format!("<0x{:02X}>", byte), (base + byte as u32).into());
    }
    serde_json::Value::Object(vocab)
}

fuzz_target!(|input: RoundtripInput| {
    let idx = (input.tokenizer_idx as usize) % TOKENIZERS.len();
    let tokenizer = &TOKENIZERS[idx];

    let ids = tokenizer
        .encode(input.text, !input.skip_special)
        .expect("lossless fixture must encode valid UTF-8");
    let decoded = tokenizer
        .decode(&ids, input.skip_special)
        .expect("lossless fixture must decode its own token IDs");
    assert_eq!(decoded, input.text, "tokenizer index {idx}");
});
