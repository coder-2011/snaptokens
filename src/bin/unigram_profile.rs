//! Disposable steady-state attribution driver for a pinned Unigram workload.

use std::{hint::black_box, io, path::Path};

/// Loads one tokenizer once, then repeatedly materializes public encode outputs for profiling.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let usage = "usage: unigram_profile <tokenizer.json> <LongBench data.json> [repetitions]";
    let tokenizer_path = arguments
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, usage))?;
    let dataset_path = arguments
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, usage))?;
    let repetitions = arguments
        .next()
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(100);

    let tokenizer = snaptokens::Tokenizer::load_file(Path::new(&tokenizer_path))?;
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(dataset_path)?)?;
    let input = rows
        .get(10)
        .and_then(|row| row.get("context"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "LongBench sample 10 has no context",
            )
        })?;

    for _ in 0..4 {
        black_box(tokenizer.encode_with_special_tokens(input, true)?);
    }
    for _ in 0..repetitions {
        black_box(tokenizer.encode_with_special_tokens(input, true)?);
    }
    Ok(())
}
