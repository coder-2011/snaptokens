#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;

#[derive(Arbitrary, Debug)]
struct UnigramInput {
    pieces: Vec<String>,
    input: String,
    byte_fallback: bool,
}

fn bounded_text(text: &str, max_characters: usize) -> String {
    text.chars().take(max_characters).collect()
}

fuzz_target!(|config: UnigramInput| {
    let mut vocab = vec![serde_json::json!(["<unk>", 0.0])];
    for (index, raw_piece) in config.pieces.iter().take(64).enumerate() {
        let piece = bounded_text(raw_piece, 16);
        if !piece.is_empty() {
            vocab.push(serde_json::json!([piece, -(index as f64 + 1.0)]));
        }
    }
    let json = serde_json::json!({
        "model": {
            "type": "Unigram",
            "unk_id": 0,
            "vocab": vocab,
            "byte_fallback": config.byte_fallback
        }
    });
    let input = bounded_text(&config.input, 512);
    if let Ok(tokenizer) = Tokenizer::from_json(json) {
        let _ = tokenizer.encode(&input);
    }
});
