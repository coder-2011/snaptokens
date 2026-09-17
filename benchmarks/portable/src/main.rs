use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    hint::black_box,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow, ensure};
use rayon::ThreadPoolBuilder;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokenizers::EncodeInput;

#[path = "../../shared.rs"]
mod shared;

use shared::balanced_orders;

const BUILD_SOURCE_COMMIT: &str = env!("SNAPTOKENS_BUILD_SOURCE_COMMIT");
const BUILD_RUSTC_VERSION: &str = env!("SNAPTOKENS_RUSTC_VERSION");
const BUILD_RUSTFLAGS: &str = env!("SNAPTOKENS_BUILD_RUSTFLAGS");
const BUILD_EVALUATOR_SHA256: &str = env!("SNAPTOKENS_BUILD_EVALUATOR_SHA256");
const BUILD_CARGO_LOCK_SHA256: &str = env!("SNAPTOKENS_BUILD_CARGO_LOCK_SHA256");
const BUILD_COMMAND: &str = env!("SNAPTOKENS_BUILD_COMMAND");
const BUILD_ID: &str = env!("SNAPTOKENS_BUILD_ID");
const PROMPT: &str = "Tokenization should not be the bottleneck. café 東京 👩🏽‍💻";
const DEFAULT_LOAD_ROUNDS: usize = 6;
const DEFAULT_ENCODE_ROUNDS: usize = 6;
const DEFAULT_SAMPLE_MIB: usize = 2;
const DEFAULT_THREADS: usize = 4;
const PARITY_CHUNK_INPUTS: usize = 1024;

#[derive(Clone, Copy)]
struct Shape {
    label: &'static str,
    input_bytes: usize,
    batch: usize,
}

const SHAPES: [Shape; 5] = [
    Shape {
        label: "140-byte batch 1",
        input_bytes: 140,
        batch: 1,
    },
    Shape {
        label: "140-byte batch 32",
        input_bytes: 140,
        batch: 32,
    },
    Shape {
        label: "140-byte batch 512",
        input_bytes: 140,
        batch: 512,
    },
    Shape {
        label: "4 KiB single",
        input_bytes: 4 * 1024,
        batch: 1,
    },
    Shape {
        label: "64 KiB single",
        input_bytes: 64 * 1024,
        batch: 1,
    },
];

struct Model {
    label: String,
    json: PathBuf,
    tkz: PathBuf,
    tokenizer_sha256: String,
}

struct Gigatoken {
    tokenizer: gigatoken_rs::load_tokenizer::hf::HfTokenizer,
    workers: gigatoken_rs::WorkerPool,
}

impl Gigatoken {
    fn load(path: &Path) -> Result<Self> {
        let bytes = fs::read(path)?;
        let tokenizer = gigatoken_rs::load_tokenizer::hf::load_hf_slice(&bytes)
            .map_err(|error| anyhow!(error.to_string()))?;
        Ok(Self {
            tokenizer,
            workers: gigatoken_rs::WorkerPool::new(),
        })
    }

    fn ragged(&self, inputs: &[String]) -> Result<(Vec<u32>, Vec<usize>)> {
        use gigatoken_rs::load_tokenizer::hf::HfTokenizer;

        let (ids, lengths) = match &self.tokenizer {
            HfTokenizer::Bpe(tokenizer) => {
                let docs: Vec<_> = inputs.iter().map(|input| input.as_bytes()).collect();
                gigatoken_rs::encode_docs_ragged(&self.workers, tokenizer, &docs)
            }
            HfTokenizer::SentencePiece(tokenizer) => {
                let docs: Vec<_> = inputs.iter().map(String::as_str).collect();
                gigatoken_rs::sp_encode_docs_ragged(tokenizer, &docs)
            }
        };
        let lengths = lengths
            .into_iter()
            .map(|length| usize::try_from(length).context("negative Gigatoken row length"))
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            lengths.iter().sum::<usize>() == ids.len(),
            "Gigatoken row lengths do not own every ID"
        );
        Ok((ids, lengths))
    }
}

enum NestedEngine<'a> {
    SnaptokensJson(&'a snaptokens::Tokenizer),
    SnaptokensTkz(&'a snaptokens::Tokenizer),
    Fastokens(&'a fastokens::Tokenizer),
    HuggingFace(&'a tokenizers::Tokenizer),
}

impl NestedEngine<'_> {
    fn label(&self) -> &'static str {
        match self {
            Self::SnaptokensJson(_) => "snaptokens-json",
            Self::SnaptokensTkz(_) => "snaptokens-tkz",
            Self::Fastokens(_) => "fastokens-json",
            Self::HuggingFace(_) => "huggingface-json",
        }
    }

    fn ids(&self, inputs: &[String]) -> Result<Vec<Vec<u32>>> {
        match self {
            Self::SnaptokensJson(tokenizer) | Self::SnaptokensTkz(tokenizer) => {
                Ok(tokenizer.encode_batch(inputs, false)?)
            }
            Self::Fastokens(tokenizer) => Ok(tokenizer.encode_batch(inputs, false)?),
            Self::HuggingFace(tokenizer) => hf_ids(tokenizer, inputs),
        }
    }

    fn run_once(&self, inputs: &[String]) -> Result<()> {
        match self {
            Self::SnaptokensJson(tokenizer) | Self::SnaptokensTkz(tokenizer) => {
                black_box(tokenizer.encode_batch(inputs, false)?);
            }
            Self::Fastokens(tokenizer) => {
                black_box(tokenizer.encode_batch(inputs, false)?);
            }
            Self::HuggingFace(tokenizer) => {
                let inputs = hf_inputs(inputs);
                let encoded = tokenizer
                    .encode_batch_fast(inputs, false)
                    .map_err(|error| anyhow!(error))?;
                black_box(encoded);
            }
        }
        Ok(())
    }
}

enum RaggedEngine<'a> {
    SnaptokensJson(&'a snaptokens::Tokenizer),
    Gigatoken(&'a Gigatoken),
}

impl RaggedEngine<'_> {
    fn label(&self) -> &'static str {
        match self {
            Self::SnaptokensJson(_) => "snaptokens-json",
            Self::Gigatoken(_) => "gigatoken-json",
        }
    }

    fn ids(&self, inputs: &[String]) -> Result<(Vec<u32>, Vec<usize>)> {
        match self {
            Self::SnaptokensJson(tokenizer) => Ok(tokenizer.encode_batch_ragged(inputs, false)?),
            Self::Gigatoken(tokenizer) => tokenizer.ragged(inputs),
        }
    }

    fn run_once(&self, inputs: &[String]) -> Result<()> {
        match self {
            Self::SnaptokensJson(tokenizer) => {
                black_box(tokenizer.encode_batch_ragged(inputs, false)?);
            }
            Self::Gigatoken(tokenizer) => {
                black_box(tokenizer.ragged(inputs)?);
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("run") => {
            let tokenizer_dir = args.next().context("missing tokenizer directory")?;
            let corpus = args.next().context("missing corpus path")?;
            let output = args.next().context("missing output path")?;
            run_matrix(
                Path::new(&tokenizer_dir),
                Path::new(&corpus),
                Path::new(&output),
            )
        }
        Some("convert-one") => {
            let path = args.next().context("missing JSON path")?;
            let model = args.next().context("missing model label")?;
            let round = parse_usize(args.next(), "conversion round")?;
            convert_one(Path::new(&path), &model, round)
        }
        Some("load-one") => {
            let implementation = args.next().context("missing implementation")?;
            let path = args.next().context("missing tokenizer path")?;
            let model = args.next().context("missing model label")?;
            let round = parse_usize(args.next(), "load round")?;
            load_one(&implementation, Path::new(&path), &model, round)
        }
        _ => Err(anyhow!(
            "usage: snaptokens-portable-benchmark run <tokenizers> <corpus> <output>"
        )),
    }
}

fn parse_usize(value: Option<String>, label: &str) -> Result<usize> {
    value
        .with_context(|| format!("missing {label}"))?
        .parse()
        .with_context(|| format!("invalid {label}"))
}

fn run_matrix(tokenizer_dir: &Path, corpus_path: &Path, output_path: &Path) -> Result<()> {
    let threads = env_usize("SNAP_THREADS", DEFAULT_THREADS)?;
    let load_rounds = env_usize("SNAP_LOAD_ROUNDS", DEFAULT_LOAD_ROUNDS)?;
    let encode_rounds = env_usize("SNAP_ENCODE_ROUNDS", DEFAULT_ENCODE_ROUNDS)?;
    let sample_mib = env_usize("SNAP_SAMPLE_MIB", DEFAULT_SAMPLE_MIB)?;
    ensure!(
        threads > 0 && load_rounds > 0 && encode_rounds > 0 && sample_mib > 0,
        "benchmark controls must be positive"
    );
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .context("failed to configure the benchmark Rayon pool")?;

    let corpus = fs::read_to_string(corpus_path)
        .with_context(|| format!("failed to read corpus {}", corpus_path.display()))?;
    ensure!(
        corpus.len() >= 64 * 1024,
        "corpus must contain at least 64 KiB"
    );
    let mut models = discover_models(tokenizer_dir)?;
    ensure!(!models.is_empty(), "no tokenizer JSON files found");

    let temporary_output = output_path.with_extension("jsonl.tmp");
    if let Some(parent) = temporary_output.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut writer = BufWriter::new(File::create(&temporary_output)?);
    emit(
        &mut writer,
        json!({
            "kind": "host",
            "run_id": env::var("SNAP_RUN_ID").unwrap_or_else(|_| unix_seconds().to_string()),
            "host": host_label(),
            "cpu": cpu_identity(),
            "os": env::consts::OS,
            "arch": env::consts::ARCH,
            "logical_cpus": std::thread::available_parallelism()?.get(),
            "rayon_threads": threads,
            "source_commit": BUILD_SOURCE_COMMIT,
            "rustc_version": BUILD_RUSTC_VERSION,
            "rustflags": BUILD_RUSTFLAGS,
            "evaluator_sha256": BUILD_EVALUATOR_SHA256,
            "cargo_lock_sha256": BUILD_CARGO_LOCK_SHA256,
            "build_command": BUILD_COMMAND,
            "build_id": BUILD_ID,
            "binary_sha256": hash_file(&env::current_exe()?)?,
            "corpus_sha256": hash_file(corpus_path)?,
            "tokenizers": models.len(),
            "load_rounds_requested": load_rounds,
            "encode_rounds_requested": encode_rounds,
            "sample_mib_per_engine_round": sample_mib,
            "filesystem_cache": "fresh process with warm filesystem pages",
            "process_startup_in_load_timer": false,
        }),
    )?;

    let work_root = tokenizer_dir.join(format!(".portable-work-{}", std::process::id()));
    fs::create_dir(&work_root)?;
    for model in &mut models {
        prepare_tkz(model, &work_root, load_rounds, &mut writer)?;
        run_load_matrix(model, load_rounds, &mut writer)?;
        run_encode_matrix(model, &corpus, encode_rounds, sample_mib, &mut writer)?;
    }
    fs::remove_dir(&work_root)?;
    writer.flush()?;
    drop(writer);
    fs::rename(temporary_output, output_path)?;
    Ok(())
}

fn env_usize(name: &str, default: usize) -> Result<usize> {
    env::var(name)
        .ok()
        .map(|value| value.parse().with_context(|| format!("invalid {name}")))
        .transpose()
        .map(|value| value.unwrap_or(default))
}

fn discover_models(tokenizer_dir: &Path) -> Result<Vec<Model>> {
    let mut json_paths = fs::read_dir(tokenizer_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|part| part.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    json_paths.sort();
    json_paths
        .into_iter()
        .map(|json| {
            let label = json
                .file_stem()
                .and_then(|part| part.to_str())
                .context("non-UTF-8 tokenizer filename")?
                .to_owned();
            let tkz = json.with_extension("tkz");
            let tokenizer_sha256 = hash_file(&json)?;
            Ok(Model {
                label,
                json,
                tkz,
                tokenizer_sha256,
            })
        })
        .collect()
}

fn prepare_tkz(
    model: &Model,
    work_root: &Path,
    rounds: usize,
    writer: &mut impl Write,
) -> Result<()> {
    if model.tkz.exists() {
        fs::remove_file(&model.tkz)?;
    }
    for round in 0..rounds {
        let round_dir = work_root.join(format!("{}-{round}", model.label));
        fs::create_dir(&round_dir)?;
        let round_json = round_dir.join(format!("{}.json", model.label));
        let round_tkz = round_json.with_extension("tkz");
        fs::copy(&model.json, &round_json)?;
        let row = child_json(&[
            "convert-one".into(),
            path_string(&round_json)?,
            model.label.clone(),
            round.to_string(),
        ])?;
        ensure!(
            row.get("tokenizer_sha256").and_then(Value::as_str)
                == Some(model.tokenizer_sha256.as_str()),
            "conversion child saw a different tokenizer"
        );
        if round == 0 {
            fs::copy(&round_tkz, &model.tkz)?;
        }
        emit(writer, row)?;
        fs::remove_file(round_tkz)?;
        fs::remove_file(round_json)?;
        fs::remove_dir(round_dir)?;
    }
    emit(
        writer,
        json!({
            "kind": "artifact",
            "host": host_label(),
            "model": model.label,
            "tokenizer_sha256": model.tokenizer_sha256,
            "json_bytes": fs::metadata(&model.json)?.len(),
            "tkz_bytes": fs::metadata(&model.tkz)?.len(),
            "tkz_sha256": hash_file(&model.tkz)?,
        }),
    )
}

fn run_load_matrix(model: &Model, rounds: usize, writer: &mut impl Write) -> Result<()> {
    let hf = tokenizers::Tokenizer::from_file(&model.json).map_err(|error| anyhow!(error))?;
    let expected = hf
        .encode(PROMPT, false)
        .map_err(|error| anyhow!(error))?
        .get_ids()
        .to_vec();
    let mut implementations = vec![
        ("snaptokens-json", &model.json),
        ("snaptokens-tkz", &model.tkz),
    ];
    let fast_is_exact = fastokens::Tokenizer::from_file(&model.json)
        .and_then(|tokenizer| tokenizer.encode(PROMPT))
        .is_ok_and(|ids| ids == expected);
    if fast_is_exact {
        implementations.push(("fastokens-json", &model.json));
    }
    implementations.push(("huggingface-json", &model.json));
    let orders = balanced_orders(implementations.len(), rounds);
    for (round, order) in orders.iter().enumerate() {
        for (position, &index) in order.iter().enumerate() {
            let (implementation, path) = implementations[index];
            let mut row = child_json(&[
                "load-one".into(),
                implementation.to_owned(),
                path_string(path)?,
                model.label.clone(),
                round.to_string(),
            ])?;
            let ids = row
                .get("ids")
                .and_then(Value::as_array)
                .context("load child omitted IDs")?
                .iter()
                .map(|id| {
                    id.as_u64()
                        .and_then(|id| u32::try_from(id).ok())
                        .context("load child returned an invalid ID")
                })
                .collect::<Result<Vec<_>>>()?;
            ensure!(
                ids == expected,
                "{implementation} first encode differs for {}",
                model.label
            );
            row["position"] = position.into();
            row["actual_rounds"] = orders.len().into();
            row["tokenizer_sha256"] = model.tokenizer_sha256.clone().into();
            emit(writer, row)?;
        }
    }
    Ok(())
}

fn run_encode_matrix(
    model: &Model,
    corpus: &str,
    requested_rounds: usize,
    sample_mib: usize,
    writer: &mut impl Write,
) -> Result<()> {
    let probes = parity_probes(&model.json)?;
    let hf = tokenizers::Tokenizer::from_file(&model.json).map_err(|error| anyhow!(error))?;
    let snap_json = snaptokens::Tokenizer::load_file(&model.json, snaptokens::LoadMode::JsonOnly)?;
    let snap_tkz = snaptokens::Tokenizer::load_file(&model.tkz, snaptokens::LoadMode::TkzCache)?;
    let snap_engines = [
        NestedEngine::SnaptokensJson(&snap_json),
        NestedEngine::SnaptokensTkz(&snap_tkz),
    ];
    verify_nested(&snap_engines, &hf, std::slice::from_ref(&probes))
        .with_context(|| format!("Snaptokens probe mismatch for {}", model.label))?;

    let fast = fastokens::Tokenizer::from_file(&model.json)
        .ok()
        .filter(|tokenizer| {
            let engine = [NestedEngine::Fastokens(tokenizer)];
            verify_nested(&engine, &hf, std::slice::from_ref(&probes)).is_ok()
        });
    emit(
        writer,
        coverage_row(
            model,
            "fastokens-json",
            if fast.is_some() {
                "exact"
            } else {
                "unsupported_or_probe_mismatch"
            },
            probes.len(),
        ),
    )?;

    let giga = Gigatoken::load(&model.json).ok().filter(|tokenizer| {
        let engine = [RaggedEngine::Gigatoken(tokenizer)];
        verify_ragged(&engine, &hf, std::slice::from_ref(&probes)).is_ok()
    });
    emit(
        writer,
        coverage_row(
            model,
            "gigatoken-json",
            if giga.is_some() {
                "exact"
            } else {
                "unsupported_or_probe_mismatch"
            },
            probes.len(),
        ),
    )?;
    emit(
        writer,
        coverage_row(model, "snaptokens-json", "exact", probes.len()),
    )?;
    emit(
        writer,
        coverage_row(model, "snaptokens-tkz", "exact", probes.len()),
    )?;
    emit(
        writer,
        coverage_row(model, "huggingface-json", "oracle", probes.len()),
    )?;

    for shape in SHAPES {
        run_nested_shape(
            model,
            corpus,
            shape,
            requested_rounds,
            sample_mib,
            &snap_json,
            fast.as_ref(),
            &hf,
            writer,
        )?;

        let ragged_json =
            snaptokens::Tokenizer::load_file(&model.json, snaptokens::LoadMode::JsonOnly)?;
        run_ragged_shape(
            model,
            corpus,
            shape,
            requested_rounds,
            sample_mib,
            &ragged_json,
            giga.as_ref(),
            &hf,
            writer,
        )?;
    }
    Ok(())
}

fn coverage_row(model: &Model, implementation: &str, status: &str, probes: usize) -> Value {
    json!({
        "kind": "coverage",
        "host": host_label(),
        "model": model.label,
        "tokenizer_sha256": model.tokenizer_sha256,
        "implementation": implementation,
        "status": status,
        "probe_count": probes,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_nested_shape(
    model: &Model,
    corpus: &str,
    shape: Shape,
    requested_rounds: usize,
    sample_mib: usize,
    snap_json: &snaptokens::Tokenizer,
    fast: Option<&fastokens::Tokenizer>,
    hf: &tokenizers::Tokenizer,
    writer: &mut impl Write,
) -> Result<()> {
    let mut engines = vec![NestedEngine::SnaptokensJson(snap_json)];
    if let Some(fast) = fast {
        engines.push(NestedEngine::Fastokens(fast));
    }
    engines.push(NestedEngine::HuggingFace(hf));
    let orders = balanced_orders(engines.len(), requested_rounds);
    let sample_bytes = sample_mib * 1024 * 1024;
    let warm = make_pool(corpus, shape, 8_000_000, sample_bytes);
    verify_nested(&engines, hf, &warm)?;
    for engine in &engines {
        for batch in &warm {
            engine.run_once(batch)?;
        }
    }

    for (round, order) in orders.iter().enumerate() {
        let first_index = round * sample_bytes.div_ceil(shape.input_bytes) + 10_000_000;
        let pool = make_pool(corpus, shape, first_index, sample_bytes);
        let mut rows = Vec::with_capacity(engines.len());
        for (position, &index) in order.iter().enumerate() {
            let engine = &engines[index];
            let (elapsed_ns, total_bytes) = measure_nested(engine, &pool)?;
            rows.push(measurement_row(
                model,
                "nested",
                engine.label(),
                shape,
                round,
                position,
                orders.len(),
                elapsed_ns,
                total_bytes,
            ));
        }
        verify_nested(&engines, hf, &pool)?;
        for row in rows {
            emit(writer, row)?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_ragged_shape(
    model: &Model,
    corpus: &str,
    shape: Shape,
    requested_rounds: usize,
    sample_mib: usize,
    snap_json: &snaptokens::Tokenizer,
    giga: Option<&Gigatoken>,
    hf: &tokenizers::Tokenizer,
    writer: &mut impl Write,
) -> Result<()> {
    let mut engines = vec![RaggedEngine::SnaptokensJson(snap_json)];
    if let Some(giga) = giga {
        engines.push(RaggedEngine::Gigatoken(giga));
    }
    let orders = balanced_orders(engines.len(), requested_rounds);
    let sample_bytes = sample_mib * 1024 * 1024;
    let warm = make_pool(corpus, shape, 18_000_000, sample_bytes);
    verify_ragged(&engines, hf, &warm)?;
    for engine in &engines {
        for batch in &warm {
            engine.run_once(batch)?;
        }
    }

    for (round, order) in orders.iter().enumerate() {
        let first_index = round * sample_bytes.div_ceil(shape.input_bytes) + 20_000_000;
        let pool = make_pool(corpus, shape, first_index, sample_bytes);
        let mut rows = Vec::with_capacity(engines.len());
        for (position, &index) in order.iter().enumerate() {
            let engine = &engines[index];
            let (elapsed_ns, total_bytes) = measure_ragged(engine, &pool)?;
            rows.push(measurement_row(
                model,
                "flat-ragged",
                engine.label(),
                shape,
                round,
                position,
                orders.len(),
                elapsed_ns,
                total_bytes,
            ));
        }
        verify_ragged(&engines, hf, &pool)?;
        for row in rows {
            emit(writer, row)?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn measurement_row(
    model: &Model,
    contract: &str,
    implementation: &str,
    shape: Shape,
    round: usize,
    position: usize,
    actual_rounds: usize,
    elapsed_ns: u128,
    total_bytes: usize,
) -> Value {
    let mib_per_s = total_bytes as f64 * 1_000_000_000.0 / elapsed_ns as f64 / (1024.0 * 1024.0);
    json!({
        "kind": "encode_measurement",
        "host": host_label(),
        "model": model.label,
        "tokenizer_sha256": model.tokenizer_sha256,
        "contract": contract,
        "implementation": implementation,
        "shape": shape.label,
        "input_bytes": shape.input_bytes,
        "batch": shape.batch,
        "round": round,
        "position": position,
        "actual_rounds": actual_rounds,
        "elapsed_ns": elapsed_ns.to_string(),
        "total_bytes": total_bytes,
        "mib_per_s": mib_per_s,
        "cache_policy": "persistent instance, disjoint-index warmup, rotating corpus windows",
        "output_allocation_and_drop_timed": true,
        "timed_inputs_exact": true,
    })
}

fn measure_nested(engine: &NestedEngine<'_>, pool: &[Vec<String>]) -> Result<(u128, usize)> {
    let total_bytes = pool.iter().flatten().map(String::len).sum();
    let started = Instant::now();
    for inputs in pool {
        engine.run_once(inputs)?;
    }
    Ok((started.elapsed().as_nanos(), total_bytes))
}

fn measure_ragged(engine: &RaggedEngine<'_>, pool: &[Vec<String>]) -> Result<(u128, usize)> {
    let total_bytes = pool.iter().flatten().map(String::len).sum();
    let started = Instant::now();
    for inputs in pool {
        engine.run_once(inputs)?;
    }
    Ok((started.elapsed().as_nanos(), total_bytes))
}

fn verify_nested(
    engines: &[NestedEngine<'_>],
    hf: &tokenizers::Tokenizer,
    pool: &[Vec<String>],
) -> Result<()> {
    let inputs = flatten_pool(pool);
    for inputs in inputs.chunks(PARITY_CHUNK_INPUTS) {
        let expected = hf_ids(hf, inputs)?;
        for engine in engines {
            ensure!(
                engine.ids(inputs)? == expected,
                "{} returned different IDs",
                engine.label()
            );
        }
    }
    Ok(())
}

fn verify_ragged(
    engines: &[RaggedEngine<'_>],
    hf: &tokenizers::Tokenizer,
    pool: &[Vec<String>],
) -> Result<()> {
    let inputs = flatten_pool(pool);
    for inputs in inputs.chunks(PARITY_CHUNK_INPUTS) {
        let expected = hf_ids(hf, inputs)?;
        for engine in engines {
            ensure!(
                rows_from_ragged(engine.ids(inputs)?)? == expected,
                "{} returned different IDs",
                engine.label()
            );
        }
    }
    Ok(())
}

fn flatten_pool(pool: &[Vec<String>]) -> Vec<String> {
    pool.iter().flatten().cloned().collect()
}

fn rows_from_ragged((ids, lengths): (Vec<u32>, Vec<usize>)) -> Result<Vec<Vec<u32>>> {
    let mut offset = 0usize;
    let mut rows = Vec::with_capacity(lengths.len());
    for length in lengths {
        let end = offset
            .checked_add(length)
            .context("ragged row length overflow")?;
        ensure!(end <= ids.len(), "ragged row length exceeds ID buffer");
        rows.push(ids[offset..end].to_vec());
        offset = end;
    }
    ensure!(offset == ids.len(), "ragged output has trailing IDs");
    Ok(rows)
}

fn make_pool(
    corpus: &str,
    shape: Shape,
    first_index: usize,
    sample_bytes: usize,
) -> Vec<Vec<String>> {
    let batch_bytes = shape.input_bytes * shape.batch;
    let batches = sample_bytes.div_ceil(batch_bytes).max(1);
    (0..batches)
        .map(|batch| {
            (0..shape.batch)
                .map(|row| {
                    let index = first_index + batch * shape.batch + row;
                    corpus_input(corpus, shape.input_bytes, index)
                })
                .collect()
        })
        .collect()
}

fn corpus_input(corpus: &str, target_bytes: usize, index: usize) -> String {
    let mut input = String::with_capacity(target_bytes);
    let mut at = index.wrapping_mul(104_729) % corpus.len();
    while !corpus.is_char_boundary(at) {
        at = (at + 1) % corpus.len();
    }
    while input.len() < target_bytes {
        let remaining = target_bytes - input.len();
        let mut end = (at + remaining).min(corpus.len());
        while end > at && !corpus.is_char_boundary(end) {
            end -= 1;
        }
        if end == at {
            input.push(' ');
        } else {
            input.push_str(&corpus[at..end]);
            at = if end == corpus.len() { 0 } else { end };
        }
    }
    input
}

fn hf_inputs(inputs: &[String]) -> Vec<EncodeInput<'_>> {
    inputs
        .iter()
        .map(|input| EncodeInput::Single(input.as_str().into()))
        .collect()
}

fn hf_ids(tokenizer: &tokenizers::Tokenizer, inputs: &[String]) -> Result<Vec<Vec<u32>>> {
    tokenizer
        .encode_batch_fast(hf_inputs(inputs), false)
        .map_err(|error| anyhow!(error))
        .map(|encodings| {
            encodings
                .iter()
                .map(|encoding| encoding.get_ids().to_vec())
                .collect()
        })
}

fn parity_probes(path: &Path) -> Result<Vec<String>> {
    let document: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mut probes = vec![
        String::new(),
        "hello world".into(),
        " café e\u{301} CAFÉ ".into(),
        "東京 日本語 中文 한글".into(),
        "العربية हिन्दी ไทย".into(),
        "👩🏽‍💻🏳️‍🌈🙂".into(),
        "\0\u{1}\u{7f}\u{80}\u{7ff}\u{800}\u{10000}\u{10ffff}".into(),
        "\t  spaces\r\nnewlines   ".into(),
        "!?!?!...---___'''\"\"".into(),
        " can't I'M we'll 123 1234 foo_bar\n".into(),
        "fn main() { println!(\"hello\"); }".into(),
        "a".repeat(1024),
    ];
    if let Some(tokens) = document.get("added_tokens").and_then(Value::as_array) {
        for token in tokens {
            let Some(content) = token.get("content").and_then(Value::as_str) else {
                continue;
            };
            probes.extend([
                content.to_owned(),
                format!("left{content}right"),
                format!("left  {content}  right"),
            ]);
        }
    }
    let mut seen = HashSet::with_capacity(probes.len());
    probes.retain(|probe| seen.insert(probe.clone()));
    Ok(probes)
}

fn convert_one(path: &Path, model: &str, round: usize) -> Result<()> {
    let tokenizer_sha256 = hash_file(path)?;
    let rss_before = peak_rss_bytes()?;
    let started = Instant::now();
    let tokenizer = snaptokens::Tokenizer::load_file(path, snaptokens::LoadMode::TkzCache)?;
    let conversion_ns = started.elapsed().as_nanos();
    black_box(tokenizer);
    let rss_after = peak_rss_bytes()?;
    let tkz = path.with_extension("tkz");
    println!(
        "{}",
        json!({
            "kind": "conversion_measurement",
            "host": host_label(),
            "model": model,
            "round": round,
            "tokenizer_sha256": tokenizer_sha256,
            "conversion_ns": conversion_ns.to_string(),
            "json_bytes": fs::metadata(path)?.len(),
            "tkz_bytes": fs::metadata(&tkz)?.len(),
            "peak_rss_before_bytes": rss_before,
            "peak_rss_after_bytes": rss_after,
            "peak_rss_delta_bytes": rss_after.saturating_sub(rss_before),
            "filesystem_cache": "fresh process with warm filesystem pages",
        })
    );
    Ok(())
}

fn load_one(implementation: &str, path: &Path, model: &str, round: usize) -> Result<()> {
    let rss_before = peak_rss_bytes()?;
    let started = Instant::now();
    let (load_ns, first_encode_ns, ids) = match implementation {
        "snaptokens-json" | "snaptokens-tkz" => {
            let mode = if implementation == "snaptokens-tkz" {
                snaptokens::LoadMode::TkzCache
            } else {
                snaptokens::LoadMode::JsonOnly
            };
            let tokenizer = snaptokens::Tokenizer::load_file(path, mode)?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(PROMPT, false)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "fastokens-json" => {
            let tokenizer = fastokens::Tokenizer::from_file(path)?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer.encode(PROMPT)?;
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        "huggingface-json" => {
            let tokenizer =
                tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow!(error))?;
            let load_ns = started.elapsed().as_nanos();
            let encode_started = Instant::now();
            let ids = tokenizer
                .encode(PROMPT, false)
                .map_err(|error| anyhow!(error))?
                .get_ids()
                .to_vec();
            (load_ns, encode_started.elapsed().as_nanos(), ids)
        }
        _ => return Err(anyhow!("unknown load implementation {implementation}")),
    };
    let rss_after = peak_rss_bytes()?;
    black_box(&ids);
    println!(
        "{}",
        json!({
            "kind": "load_measurement",
            "host": host_label(),
            "model": model,
            "implementation": implementation,
            "round": round,
            "artifact_sha256": hash_file(path)?,
            "artifact_bytes": fs::metadata(path)?.len(),
            "load_ns": load_ns.to_string(),
            "first_encode_ns": first_encode_ns.to_string(),
            "load_plus_first_encode_ns": (load_ns + first_encode_ns).to_string(),
            "peak_rss_before_bytes": rss_before,
            "peak_rss_after_bytes": rss_after,
            "peak_rss_delta_bytes": rss_after.saturating_sub(rss_before),
            "ids": ids,
            "process_startup_in_load_timer": false,
            "filesystem_cache": "fresh process with warm filesystem pages",
        })
    );
    Ok(())
}

fn child_json(arguments: &[String]) -> Result<Value> {
    let output = Command::new(env::current_exe()?)
        .args(arguments)
        .output()
        .context("failed to launch benchmark child")?;
    parse_child_output(output)
}

fn parse_child_output(output: Output) -> Result<Value> {
    ensure!(
        output.status.success(),
        "benchmark child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).context("benchmark child returned invalid JSON")
}

fn emit(writer: &mut impl Write, value: Value) -> Result<()> {
    serde_json::to_writer(&mut *writer, &value)?;
    writer.write_all(b"\n")?;
    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    std::io::copy(&mut file, &mut digest)?;
    Ok(format!("{:x}", digest.finalize()))
}

fn path_string(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .context("benchmark path is not UTF-8")
}

fn host_label() -> String {
    env::var("SNAP_HOST").unwrap_or_else(|_| "local".into())
}

fn cpu_identity() -> String {
    if let Ok(value) = env::var("SNAP_CPU") {
        return value;
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo")
            && let Some(value) = cpuinfo.lines().find_map(|line| {
                line.strip_prefix("model name")
                    .and_then(|line| line.split_once(':'))
                    .map(|(_, value)| value.trim().to_owned())
            })
        {
            return value;
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
            && output.status.success()
        {
            return String::from_utf8_lossy(&output.stdout).trim().to_owned();
        }
    }
    "unknown".into()
}

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn peak_rss_bytes() -> Result<u64> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    let status = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
    ensure!(status == 0, "getrusage failed");
    let usage = unsafe { usage.assume_init() };
    #[cfg(target_os = "macos")]
    {
        Ok(usage.ru_maxrss as u64)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(usage.ru_maxrss as u64 * 1024)
    }
}
