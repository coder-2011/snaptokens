use std::{env, fs, hint::black_box, path::Path, time::Instant};

use anyhow::{Context, Result, anyhow, ensure};
use serde_json::json;
use splintr::Tokenize;

const BUILD_SOURCE_COMMIT: &str = env!("SNAPTOKENS_BUILD_SOURCE_COMMIT");

fn checksum(ids: &[u32]) -> u64 {
    ids.iter().fold(ids.len() as u64, |hash, &id| {
        hash.wrapping_mul(1_099_511_628_211)
            .wrapping_add(id as u64 + 1)
    })
}

fn required_env(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("missing {name}"))
}

fn required_usize_env(name: &str) -> Result<usize> {
    required_env(name)?
        .parse()
        .with_context(|| format!("invalid {name}"))
}

fn parity_ids(implementation: &str, path: &Path, inputs: &[String]) -> Result<Vec<Vec<u32>>> {
    match implementation {
        "snaptokens-json" => Ok(snaptokens::Tokenizer::load_file(
            path,
            snaptokens::LoadMode::JsonOnly,
        )?
        .encode_batch(inputs, false)?),
        "snaptokens-json-sidecar" | "snaptokens-json-create" => Ok(
            snaptokens::Tokenizer::load_file(path, snaptokens::LoadMode::TkzCache)?
                .encode_batch(inputs, false)?,
        ),
        "snaptokens-tkz-direct" => {
            ensure!(
                path.extension().and_then(|part| part.to_str()) == Some("tkz"),
                "direct TKZ mode requires a .tkz path"
            );
            Ok(
                snaptokens::Tokenizer::load_file(path, snaptokens::LoadMode::TkzCache)?
                    .encode_batch(inputs, false)?,
            )
        }
        "fastokens-json" => Ok(fastokens::Tokenizer::from_file(path)?.encode_batch(inputs, false)?),
        "huggingface-json" => {
            let tokenizer =
                tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow!(error))?;
            let encodings = tokenizer
                .encode_batch_fast(inputs.iter().map(String::as_str).collect(), false)
                .map_err(|error| anyhow!(error))?;
            Ok(encodings
                .iter()
                .map(|encoding| encoding.get_ids().to_vec())
                .collect())
        }
        "kitoken-json" | "kitoken-kit" => {
            let tokenizer = kitoken::Kitoken::from_file(path).map_err(|error| anyhow!(error))?;
            inputs
                .iter()
                .map(|input| {
                    tokenizer
                        .encode(input, true)
                        .map_err(|error| anyhow!(error))
                })
                .collect()
        }
        "tokie-json" => {
            let tokenizer = tokie::Tokenizer::from_json(path).map_err(|error| anyhow!(error))?;
            let refs: Vec<_> = inputs.iter().map(String::as_str).collect();
            Ok(tokenizer
                .encode_batch(&refs, false)
                .into_iter()
                .map(|encoding| encoding.ids)
                .collect())
        }
        "tokie-tkz" => {
            let tokenizer = tokie::Tokenizer::from_file(path).map_err(|error| anyhow!(error))?;
            let refs: Vec<_> = inputs.iter().map(String::as_str).collect();
            Ok(tokenizer
                .encode_batch(&refs, false)
                .into_iter()
                .map(|encoding| encoding.ids)
                .collect())
        }
        "splintr-json" => {
            let tokenizer = splintr::from_json_path(path).map_err(|error| anyhow!(error))?;
            Ok(inputs.iter().map(|input| tokenizer.encode(input)).collect())
        }
        _ => Err(anyhow!("unknown implementation: {implementation}")),
    }
}

fn run_parity(implementation: &str, path: &Path, manifest: &Path) -> Result<()> {
    let inputs: Vec<String> = serde_json::from_slice(&fs::read(manifest)?)?;
    ensure!(!inputs.is_empty(), "parity manifest is empty");
    let ids = parity_ids(implementation, path, &inputs)?;
    println!(
        "{}",
        json!({
            "kind": "load_parity",
            "implementation": implementation,
            "probe_count": inputs.len(),
            "ids": ids,
        })
    );
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let implementation = args.next().context("missing implementation")?;
    if implementation == "parity" {
        let implementation = args.next().context("missing parity implementation")?;
        let path = args.next().context("missing tokenizer path")?;
        let manifest = args.next().context("missing parity manifest")?;
        return run_parity(&implementation, Path::new(&path), Path::new(&manifest));
    }
    let path = args.next().context("missing tokenizer path")?;
    let value = args
        .next()
        .unwrap_or_else(|| "Tokenization should not be the bottleneck.".into());

    if implementation == "tokie-create" {
        let tokenizer = tokie::Tokenizer::from_json(&path).map_err(|error| anyhow!(error))?;
        tokenizer.to_file(&value).map_err(|error| anyhow!(error))?;
        return Ok(());
    }
    if implementation == "kitoken-create" {
        let tokenizer = kitoken::Kitoken::from_file(&path).map_err(|error| anyhow!(error))?;
        tokenizer.to_file(&value)?;
        return Ok(());
    }
    if implementation == "snaptokens-create" {
        snaptokens::Tokenizer::load_file(Path::new(&path), snaptokens::LoadMode::TkzCache)?;
        return Ok(());
    }

    let run_id = required_env("SNAPTOKENS_BENCH_RUN_ID")?;
    let host = required_env("SNAPTOKENS_BENCH_HOST")?;
    let model = required_env("SNAPTOKENS_MODEL")?;
    let tokenizer_sha256 = required_env("SNAPTOKENS_TOKENIZER_SHA256")?;
    let cache_state = required_env("SNAPTOKENS_CACHE_STATE")?;
    let repetition = required_env("SNAPTOKENS_REPETITION")?;
    let order = required_env("SNAPTOKENS_ORDER")?;
    let cpu_set = required_env("SNAPTOKENS_CPU_SET")?;
    let source_commit = required_env("SNAPTOKENS_SOURCE_COMMIT")?;
    ensure!(
        BUILD_SOURCE_COMMIT == source_commit,
        "binary was built from {BUILD_SOURCE_COMMIT}, not {source_commit}"
    );
    let binary_sha256 = required_env("SNAPTOKENS_BINARY_SHA256")?;
    let build_command = required_env("SNAPTOKENS_BUILD_COMMAND")?;
    let rustc_version = required_env("SNAPTOKENS_RUSTC_VERSION")?;
    let build_rustflags = required_env("SNAPTOKENS_BUILD_RUSTFLAGS")?;
    let cargo_lock_sha256 = required_env("SNAPTOKENS_CARGO_LOCK_SHA256")?;
    let artifact_sha256 = required_env("SNAPTOKENS_ARTIFACT_SHA256")?;
    let artifact_policy = required_env("SNAPTOKENS_ARTIFACT_POLICY")?;
    let sidecar_sha256 = env::var("SNAPTOKENS_SIDECAR_SHA256")
        .ok()
        .filter(|value| !value.is_empty());
    let prompt_sha256 = required_env("SNAPTOKENS_PROMPT_SHA256")?;
    let prompt_bytes = required_usize_env("SNAPTOKENS_PROMPT_BYTES")?;
    let probe_manifest_sha256 = required_env("SNAPTOKENS_PROBE_MANIFEST_SHA256")?;
    let probe_count = required_usize_env("SNAPTOKENS_PROBE_COUNT")?;
    let requested_rounds = required_usize_env("SNAPTOKENS_REQUESTED_ROUNDS")?;
    let actual_rounds = required_usize_env("SNAPTOKENS_ACTUAL_ROUNDS")?;
    let cycle_length = required_usize_env("SNAPTOKENS_CYCLE_LENGTH")?;
    let schedule_version = required_env("SNAPTOKENS_SCHEDULE_VERSION")?;

    let started = Instant::now();
    let (load_ns, first_encode_ns, ids) = match implementation.as_str() {
        "snaptokens-json" => {
            let tokenizer =
                snaptokens::Tokenizer::load_file(Path::new(&path), snaptokens::LoadMode::JsonOnly)?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(&value, false)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "snaptokens-json-sidecar" | "snaptokens-json-create" => {
            let tokenizer =
                snaptokens::Tokenizer::load_file(Path::new(&path), snaptokens::LoadMode::TkzCache)?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(&value, false)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "snaptokens-tkz-direct" => {
            ensure!(
                Path::new(&path).extension().and_then(|part| part.to_str()) == Some("tkz"),
                "direct TKZ mode requires a .tkz path"
            );
            let tokenizer =
                snaptokens::Tokenizer::load_file(Path::new(&path), snaptokens::LoadMode::TkzCache)?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(&value, false)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "fastokens-json" => {
            let tokenizer = fastokens::Tokenizer::from_file(Path::new(&path))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(&value)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "huggingface-json" => {
            let tokenizer =
                tokenizers::Tokenizer::from_file(&path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let encoding = tokenizer
                .encode(value, false)
                .map_err(|error| anyhow!(error))?;
            let ids = encoding.get_ids().to_vec();
            let first_encode_ns = encode_started.elapsed().as_nanos();
            (load_ns, first_encode_ns, ids)
        }
        "kitoken-json" | "kitoken-kit" => {
            let tokenizer = kitoken::Kitoken::from_file(&path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer
                .encode(&value, true)
                .map_err(|error| anyhow!(error))?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "tokie-json" => {
            let tokenizer = tokie::Tokenizer::from_json(&path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let encoding = tokenizer.encode(&value, false);
            let first_encode_ns = encode_started.elapsed().as_nanos();
            (load_ns, first_encode_ns, encoding.ids)
        }
        "tokie-tkz" => {
            let tokenizer = tokie::Tokenizer::from_file(&path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let encoding = tokenizer.encode(&value, false);
            let first_encode_ns = encode_started.elapsed().as_nanos();
            (load_ns, first_encode_ns, encoding.ids)
        }
        "splintr-json" => {
            let tokenizer = splintr::from_json_path(&path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(&value);
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        _ => return Err(anyhow!("unknown implementation: {implementation}")),
    };
    // File-size inspection stays outside both timed regions so it cannot warm cold metadata.
    let artifact_bytes = fs::metadata(&path)?.len();
    let sidecar_path = match implementation.as_str() {
        "snaptokens-json-sidecar" | "snaptokens-json-create" => {
            Some(Path::new(&path).with_extension("tkz"))
        }
        "snaptokens-tkz-direct" | "tokie-tkz" => Some(Path::new(&path).to_path_buf()),
        _ => None,
    };
    let sidecar_bytes = sidecar_path
        .as_deref()
        .and_then(|sidecar| fs::metadata(sidecar).ok())
        .map(|metadata| metadata.len());
    black_box(&ids);
    let exact_checksum = checksum(&ids);
    println!(
        "{}",
        json!({
            "kind": "load_measurement",
            "run_id": run_id,
            "host": host,
            "model": model,
            "tokenizer_sha256": tokenizer_sha256,
            "implementation": implementation,
            "cache_state": cache_state,
            "repetition": repetition,
            "order": order,
            "cpu_set": cpu_set,
            "source_commit": source_commit,
            "binary_sha256": binary_sha256,
            "build_command": build_command,
            "rustc_version": rustc_version,
            "build_rustflags": build_rustflags,
            "cargo_lock_sha256": cargo_lock_sha256,
            "artifact_bytes": artifact_bytes,
            "artifact_sha256": artifact_sha256,
            "artifact_policy": artifact_policy,
            "sidecar_bytes": sidecar_bytes,
            "sidecar_sha256": sidecar_sha256,
            "prompt_sha256": prompt_sha256,
            "prompt_bytes": prompt_bytes,
            "probe_manifest_sha256": probe_manifest_sha256,
            "probe_count": probe_count,
            "requested_rounds": requested_rounds,
            "actual_rounds": actual_rounds,
            "cycle_length": cycle_length,
            "schedule_version": schedule_version,
            "load_ns": load_ns,
            "first_encode_ns": first_encode_ns,
            "token_count": ids.len(),
            "exact_checksum": exact_checksum,
            "ids": ids,
            "process_startup_excluded": true,
        })
    );
    Ok(())
}
