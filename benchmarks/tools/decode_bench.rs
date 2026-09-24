//! Decode-throughput evaluator (decode-eval-v1).
//!
//! Builds deterministic ID streams from a corpus, verifies exact Hugging Face
//! decode parity around every timed pool, and times steady-state `decode` /
//! `decode_batch` calls including output allocation and destruction.
//!
//! Modes:
//! - `--write-manifest`: encode rows, optionally cross-check IDs and record
//!   reference decode output hashes against the Hugging Face `tokenizers`
//!   crate (`--hf-verify`), then write the frozen cell manifest.
//! - `--time`: load the manifest, require identical IDs, verify decode parity
//!   before and after the timed rounds, and print per-round JSON lines.

use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;
use std::io::Write as _;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use clap::Parser;
use sha2::{Digest, Sha256};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the Hugging Face tokenizer.json fixture.
    #[arg(long)]
    tokenizer: PathBuf,

    /// Corpus file: `.txt` (split by --row-bytes) or `.jsonl` with {"text": ...} rows.
    #[arg(long)]
    corpus: PathBuf,

    /// Row size in bytes for `.txt` corpora (rows end on char boundaries).
    #[arg(long, default_value_t = 4096)]
    row_bytes: usize,

    /// Maximum number of rows taken from the corpus.
    #[arg(long)]
    max_rows: Option<usize>,

    /// Maximum total corpus bytes taken before splitting.
    #[arg(long)]
    max_bytes: Option<usize>,

    /// Encode rows with the tokenizer's special-token post-processing.
    #[arg(long, default_value_t = false)]
    insert_specials: bool,

    /// skip_special_tokens flag used for the timed decode calls.
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    skip_special: bool,

    /// Rows per decode_batch call in the timed loop (1 uses decode()).
    #[arg(long, default_value_t = 1)]
    batch: usize,

    /// Full passes over the row set per timed round.
    #[arg(long, default_value_t = 1)]
    repeat: usize,

    /// Timed rounds.
    #[arg(long, default_value_t = 5)]
    rounds: usize,

    /// Untimed warmup passes.
    #[arg(long, default_value_t = 1)]
    warmup: usize,

    /// Manifest path for this cell.
    #[arg(long)]
    manifest: PathBuf,

    /// Write the manifest instead of timing.
    #[arg(long, default_value_t = false)]
    write_manifest: bool,

    /// During --write-manifest, cross-check IDs and decode references with the
    /// Hugging Face tokenizers crate.
    #[arg(long, default_value_t = false)]
    hf_verify: bool,

    /// Run timed rounds.
    #[arg(long, default_value_t = false)]
    time: bool,
}

fn sha_hex(hasher: Sha256) -> String {
    let digest = hasher.finalize();
    let mut out = String::with_capacity(64);
    for byte in digest {
        write!(out, "{byte:02x}").unwrap();
    }
    out
}

fn file_sha256(path: &PathBuf) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(sha_hex(hasher))
}

/// Splits corpus text into rows, honoring char boundaries for `.txt` input.
fn build_rows(args: &Args) -> Result<Vec<String>> {
    let is_jsonl = args
        .corpus
        .extension()
        .is_some_and(|extension| extension == "jsonl");
    let mut rows = Vec::new();
    let raw = fs::read_to_string(&args.corpus)
        .with_context(|| format!("reading {}", args.corpus.display()))?;

    if is_jsonl {
        for line in raw.lines() {
            if line.is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_str(line)?;
            let text = value
                .get("text")
                .and_then(|t| t.as_str())
                .context("jsonl row missing text")?;
            rows.push(text.to_string());
            if args.max_rows.is_some_and(|m| rows.len() >= m) {
                break;
            }
        }
    } else {
        let mut text = raw.as_str();
        if let Some(max) = args.max_bytes
            && text.len() > max
        {
            let mut end = max;
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            text = &text[..end];
        }
        while !text.is_empty() {
            let mut end = args.row_bytes.min(text.len());
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            if end == 0 {
                break;
            }
            rows.push(text[..end].to_string());
            text = &text[end..];
            if args.max_rows.is_some_and(|m| rows.len() >= m) {
                break;
            }
        }
    }
    if rows.is_empty() {
        bail!("no rows produced from {}", args.corpus.display());
    }
    Ok(rows)
}

fn ids_sha256(id_rows: &[Vec<u32>]) -> String {
    let mut hasher = Sha256::new();
    for row in id_rows {
        hasher.update((row.len() as u64).to_le_bytes());
        for &id in row {
            hasher.update(id.to_le_bytes());
        }
    }
    sha_hex(hasher)
}

fn outputs_sha256(outputs: impl Iterator<Item = String>) -> String {
    let mut hasher = Sha256::new();
    for output in outputs {
        hasher.update((output.len() as u64).to_le_bytes());
        hasher.update(output.as_bytes());
    }
    sha_hex(hasher)
}

fn snap_decode_sha(
    tokenizer: &snaptokens::Tokenizer,
    id_rows: &[Vec<u32>],
    skip_special: bool,
) -> Result<(String, u64)> {
    let mut hasher = Sha256::new();
    let mut decoded_bytes = 0u64;
    for row in id_rows {
        let output = tokenizer.decode(row, skip_special)?;
        decoded_bytes += output.len() as u64;
        hasher.update((output.len() as u64).to_le_bytes());
        hasher.update(output.as_bytes());
    }
    Ok((sha_hex(hasher), decoded_bytes))
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.write_manifest == args.time {
        bail!("pass exactly one of --write-manifest or --time");
    }

    let rows = build_rows(&args)?;
    let tokenizer = snaptokens::Tokenizer::load_file(&args.tokenizer)
        .map_err(|e| anyhow::anyhow!("loading snaptokens tokenizer: {e}"))?;

    let mut id_rows: Vec<Vec<u32>> = Vec::with_capacity(rows.len());
    for row in &rows {
        id_rows.push(tokenizer.encode(row, args.insert_specials)?);
    }
    let ids_sha = ids_sha256(&id_rows);
    let total_tokens: u64 = id_rows.iter().map(|r| r.len() as u64).sum();
    let total_text_bytes: u64 = rows.iter().map(|r| r.len() as u64).sum();

    if args.write_manifest {
        let mut hf_output_sha_skip = None;
        let mut hf_output_sha_noskip = None;
        if args.hf_verify {
            let hf = tokenizers::Tokenizer::from_file(&args.tokenizer)
                .map_err(|e| anyhow::anyhow!("loading hf tokenizer: {e}"))?;
            for (index, (row, ids)) in rows.iter().zip(&id_rows).enumerate() {
                let encoding = hf
                    .encode(row.as_str(), args.insert_specials)
                    .map_err(|e| anyhow::anyhow!("hf encode: {e}"))?;
                if encoding.get_ids() != ids.as_slice() {
                    bail!("HF ID mismatch on row {index}");
                }
            }
            for (skip, slot) in [
                (true, &mut hf_output_sha_skip),
                (false, &mut hf_output_sha_noskip),
            ] {
                let mut outputs = Vec::with_capacity(id_rows.len());
                for row in &id_rows {
                    outputs.push(
                        hf.decode(row, skip)
                            .map_err(|e| anyhow::anyhow!("hf decode: {e}"))?,
                    );
                }
                *slot = Some(outputs_sha256(outputs.into_iter()));
            }
        }
        let manifest = serde_json::json!({
            "tool": "decode-eval-v1",
            "tokenizer_sha256": file_sha256(&args.tokenizer)?,
            "corpus_sha256": file_sha256(&args.corpus)?,
            "row_bytes": args.row_bytes,
            "max_rows": args.max_rows,
            "max_bytes": args.max_bytes,
            "insert_specials": args.insert_specials,
            "rows": rows.len(),
            "total_tokens": total_tokens,
            "total_text_bytes": total_text_bytes,
            "ids_sha256": ids_sha,
            "hf_verified": args.hf_verify,
            "hf_output_sha256_skip": hf_output_sha_skip,
            "hf_output_sha256_noskip": hf_output_sha_noskip,
        });
        fs::write(&args.manifest, serde_json::to_string_pretty(&manifest)?)?;
        println!("{manifest}");
        return Ok(());
    }

    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&args.manifest)
            .with_context(|| format!("reading {}", args.manifest.display()))?,
    )?;
    if manifest["ids_sha256"].as_str() != Some(ids_sha.as_str()) {
        bail!("ID stream does not match manifest; refusing to time");
    }
    let expected_key = if args.skip_special {
        "hf_output_sha256_skip"
    } else {
        "hf_output_sha256_noskip"
    };
    let expected_output_sha = manifest[expected_key]
        .as_str()
        .context("manifest missing HF output hash; regenerate with --hf-verify")?
        .to_string();

    // Pre-timing exactness gate over the complete timed input.
    let (pre_sha, decoded_bytes) = snap_decode_sha(&tokenizer, &id_rows, args.skip_special)?;
    if pre_sha != expected_output_sha {
        bail!("pre-timing decode parity failure");
    }

    let row_refs: Vec<&[u32]> = id_rows.iter().map(Vec::as_slice).collect();
    let mut round_seconds = Vec::with_capacity(args.rounds);
    for round in 0..args.warmup + args.rounds {
        let start = Instant::now();
        let mut consumed = 0usize;
        for _ in 0..args.repeat {
            if args.batch <= 1 {
                for row in &row_refs {
                    let output = tokenizer.decode(row, args.skip_special)?;
                    consumed += black_box(&output).len();
                }
            } else {
                for chunk in row_refs.chunks(args.batch) {
                    let outputs = tokenizer.decode_batch(chunk, args.skip_special)?;
                    consumed += black_box(&outputs).iter().map(String::len).sum::<usize>();
                }
            }
        }
        let elapsed = start.elapsed().as_secs_f64();
        black_box(consumed);
        if round >= args.warmup {
            round_seconds.push(elapsed);
            println!(
                "{}",
                serde_json::json!({"round": round - args.warmup, "seconds": elapsed})
            );
            std::io::stdout().flush().ok();
        }
    }

    // Post-timing exactness gate.
    let (post_sha, _) = snap_decode_sha(&tokenizer, &id_rows, args.skip_special)?;
    if post_sha != expected_output_sha {
        bail!("post-timing decode parity failure");
    }

    let mut sorted = round_seconds.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let median = sorted[sorted.len() / 2];
    let pass_tokens = total_tokens * args.repeat as u64;
    let pass_bytes = decoded_bytes * args.repeat as u64;
    let summary = serde_json::json!({
        "summary": true,
        "tokenizer": args.tokenizer,
        "corpus": args.corpus,
        "row_bytes": args.row_bytes,
        "rows": rows.len(),
        "batch": args.batch,
        "repeat": args.repeat,
        "skip_special": args.skip_special,
        "insert_specials": args.insert_specials,
        "ids_sha256": ids_sha,
        "parity": "pass",
        "round_seconds": round_seconds,
        "median_seconds": median,
        "tokens_per_second": pass_tokens as f64 / median,
        "decoded_bytes_per_second": pass_bytes as f64 / median,
        "pass_tokens": pass_tokens,
        "pass_decoded_bytes": pass_bytes,
    });
    println!("{summary}");
    Ok(())
}
