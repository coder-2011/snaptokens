#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;

#[derive(Arbitrary, Debug)]
struct TokenizerConfig {
    vocab_size: u16,
    num_merges: u8,
    byte_fallback: bool,
    model_type: ModelType,
    pre_tokenizer: Option<PreTokenizerType>,
    decoder_type: Option<DecoderType>,
    added_tokens: Vec<AddedToken>,
}

#[derive(Arbitrary, Debug)]
enum ModelType {
    BPE,
    Unigram,
    WordPiece,
}

#[derive(Arbitrary, Debug)]
enum PreTokenizerType {
    ByteLevel { add_prefix_space: bool },
    Split,
    Whitespace,
}

#[derive(Arbitrary, Debug)]
enum DecoderType {
    ByteLevel,
    ByteFallback,
    Sequence,
}

#[derive(Arbitrary, Debug)]
struct AddedToken {
    id: u16,
    content: String,
    special: bool,
}

fn build_json(config: &TokenizerConfig) -> serde_json::Value {
    let vocab_size = (config.vocab_size as usize).min(1024).max(1);
    let mut vocab = serde_json::Map::new();

    for i in 0..vocab_size {
        let token = if i < 256 {
            format!("<0x{:02X}>", i)
        } else {
            format!("tok_{}", i)
        };
        vocab.insert(token, (i as u32).into());
    }

    let num_merges = (config.num_merges as usize).min(vocab_size.saturating_sub(1));
    let mut merges = Vec::new();
    for i in 0..num_merges {
        let a = format!("<0x{:02X}>", i % 256);
        let b = format!("<0x{:02X}>", (i + 1) % 256);
        merges.push(format!("{} {}", a, b));
    }

    let model_type = match config.model_type {
        ModelType::BPE => "BPE",
        ModelType::Unigram => "Unigram",
        ModelType::WordPiece => "WordPiece",
    };

    let mut json = serde_json::json!({
        "model": {
            "type": model_type,
            "vocab": serde_json::Value::Object(vocab),
            "merges": merges,
            "byte_fallback": config.byte_fallback
        }
    });

    if let Some(ref pt) = config.pre_tokenizer {
        json["pre_tokenizer"] = match pt {
            PreTokenizerType::ByteLevel { add_prefix_space } => serde_json::json!({
                "type": "ByteLevel",
                "add_prefix_space": add_prefix_space
            }),
            PreTokenizerType::Split => serde_json::json!({
                "type": "Split",
                "pattern": { "Regex": r"\s+" },
                "behavior": "Isolated",
                "invert": false
            }),
            PreTokenizerType::Whitespace => serde_json::json!({
                "type": "Whitespace"
            }),
        };
    }

    if let Some(ref dec) = config.decoder_type {
        json["decoder"] = match dec {
            DecoderType::ByteLevel => serde_json::json!({ "type": "ByteLevel" }),
            DecoderType::ByteFallback => serde_json::json!({
                "type": "Sequence",
                "decoders": [{"type": "ByteFallback"}, {"type": "Fuse"}]
            }),
            DecoderType::Sequence => serde_json::json!({
                "type": "Sequence",
                "decoders": [{"type": "Strip", "content": " ", "start": 1, "stop": 0}]
            }),
        };
    }

    if !config.added_tokens.is_empty() {
        let tokens: Vec<_> = config
            .added_tokens
            .iter()
            .take(10)
            .map(|t| {
                serde_json::json!({
                    "id": t.id as u32,
                    "content": &t.content,
                    "special": t.special
                })
            })
            .collect();
        json["added_tokens"] = serde_json::Value::Array(tokens);
    }

    json
}

fuzz_target!(|config: TokenizerConfig| {
    let json = build_json(&config);
    let _ = Tokenizer::from_json(json);
});
