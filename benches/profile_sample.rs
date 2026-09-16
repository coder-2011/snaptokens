use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use hf_hub::api::sync::Api;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    // Load dataset
    let api = Api::new()?;
    let repo = api.dataset("zai-org/LongBench-v2".into());
    let json_path = repo.get("data.json")?;
    let data: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(json_path)?)?;

    // If --sizes flag, just print sample sizes
    if args.get(1).is_some_and(|arg| arg == "--sizes") {
        let start: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let end: usize = args
            .get(3)
            .and_then(|s| s.parse().ok())
            .unwrap_or(data.len());
        for (i, item) in data
            .iter()
            .enumerate()
            .take(end.min(data.len()))
            .skip(start)
        {
            if let Some(ctx) = item.get("context").and_then(|v| v.as_str()) {
                println!("[{:>3}] {} chars", i, ctx.len());
            }
        }
        return Ok(());
    }

    let tokenizer_json = args
        .get(1)
        .context("usage: profile_sample <tokenizer.json> [sample index]")?;
    let sample_idx: usize = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("10")
        .parse()
        .context("invalid sample index")?;

    let input = data[sample_idx]
        .get("context")
        .and_then(|v| v.as_str())
        .context("no context field")?;
    println!("Sample {sample_idx}: {} chars", input.len());

    // Load tokenizer
    let tokenizer = snaptokens::Tokenizer::load_file(Path::new(tokenizer_json))?;

    // Step 1: Pre-tokenize only
    let t0 = Instant::now();
    let mut pts = tokenizer.build_pre_tokenized(input);
    if let Some(pt) = tokenizer.pre_tokenizer() {
        pt.pre_tokenize(&mut pts)?;
    }
    let pre_tok_time = t0.elapsed();

    let splits = pts.splits();
    let text_splits: Vec<&str> = splits
        .iter()
        .filter(|s| s.token_id.is_none() && !s.range.is_empty())
        .map(|s| &pts.buffer()[s.range.clone()])
        .collect();

    let unique: HashSet<&str> = text_splits.iter().copied().collect();

    let mut sizes = text_splits
        .iter()
        .map(|split| split.len())
        .collect::<Vec<_>>();
    sizes.sort_unstable();

    let total_bytes: usize = sizes.iter().sum();
    let big_splits: Vec<usize> = sizes.iter().copied().filter(|&s| s > 50).collect();
    let big_total: usize = big_splits.iter().sum();

    println!(
        "Pre-tokenize: {:.2} ms",
        pre_tok_time.as_secs_f64() * 1000.0
    );
    println!("Total splits: {}", text_splits.len());
    println!("Unique splits: {}", unique.len());
    println!("Total split bytes: {total_bytes}");
    println!(
        "Splits > 50 bytes: {} (total {} bytes)",
        big_splits.len(),
        big_total
    );
    if let Some(&max) = sizes.last() {
        println!("Max split size: {max} bytes");
    }
    if !sizes.is_empty() {
        println!("Median split size: {} bytes", sizes[sizes.len() / 2]);
        println!(
            "p95 split size: {} bytes",
            sizes[(sizes.len() as f64 * 0.95) as usize]
        );
        println!(
            "p99 split size: {} bytes",
            sizes[(sizes.len() as f64 * 0.99) as usize]
        );
    }

    // Step 2: Full encode (cold)
    let t0 = Instant::now();
    let ids = tokenizer.encode(input, false)?;
    let cold_time = t0.elapsed();
    println!(
        "\nCold encode: {:.2} ms ({} tokens)",
        cold_time.as_secs_f64() * 1000.0,
        ids.len()
    );

    // Step 3: Full encode (warm)
    let t0 = Instant::now();
    let _ = tokenizer.encode(input, false)?;
    let warm_time = t0.elapsed();
    println!("Warm encode: {:.2} ms", warm_time.as_secs_f64() * 1000.0);

    // Step 4: HF comparison
    let hf =
        tokenizers::Tokenizer::from_file(tokenizer_json).map_err(|error| anyhow::anyhow!(error))?;
    let t0 = Instant::now();
    let _ = hf
        .encode_fast(input, true)
        .map_err(|e| anyhow::anyhow!(e))?;
    let hf_time = t0.elapsed();
    println!("HF encode: {:.2} ms", hf_time.as_secs_f64() * 1000.0);

    println!(
        "\nCold speedup vs HF: {:.1}x",
        hf_time.as_secs_f64() / cold_time.as_secs_f64()
    );
    println!(
        "Warm speedup vs HF: {:.1}x",
        hf_time.as_secs_f64() / warm_time.as_secs_f64()
    );

    Ok(())
}
