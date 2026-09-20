use anyhow::{Context, Result, anyhow, bail, ensure};
use serde_json::{Value, json};
use std::env;
use std::fs;
use std::hint::black_box;
use std::io::{self, BufWriter, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[path = "../../shared.rs"]
mod shared;

use shared::balanced_orders;

const RUN_VERSION: u32 = 1;
const REQUESTED_ROUNDS: usize = 4;
const CANDIDATE_COUNT: usize = 4;
const BUILD_SOURCE_COMMIT: &str = env!("SNAPTOKENS_BUILD_SOURCE_COMMIT");
const HF_REVISION: &str = "b62132e4e0ec7518caba201408a680819dfdcd22";
const CXUU_REVISION: &str = "1c8b302cfbde3b5f4b78476b0dec7bf37d04cbb6";
const FASTOKENS_REVISION: &str = "326cb5afc5a033d2f7885832d12fd43b9ea50cdd";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TokenizerBackend {
    Snaptokens,
    Fastokens,
    HuggingFace,
    CxuuParallel,
}

impl TokenizerBackend {
    const fn label(self) -> &'static str {
        match self {
            Self::Snaptokens => "snaptokens",
            Self::Fastokens => "fastokens",
            Self::HuggingFace => "huggingface-serial",
            Self::CxuuParallel => "cxuu-parallel",
        }
    }
}

const BACKENDS: [TokenizerBackend; CANDIDATE_COUNT] = [
    TokenizerBackend::Snaptokens,
    TokenizerBackend::Fastokens,
    TokenizerBackend::HuggingFace,
    TokenizerBackend::CxuuParallel,
];

enum Candidate {
    Snaptokens(snaptokens::Tokenizer),
    Fastokens(fastokens::Tokenizer),
    HuggingFace(hf_tokenizers::Tokenizer),
    CxuuParallel(cxuu_tokenizers::Tokenizer),
}

impl Candidate {
    fn load(backend: TokenizerBackend, path: &Path) -> Result<Self> {
        match backend {
            TokenizerBackend::Snaptokens => {
                Ok(Self::Snaptokens(snaptokens::Tokenizer::load_file(path)?))
            }
            TokenizerBackend::Fastokens => {
                Ok(Self::Fastokens(fastokens::Tokenizer::from_file(path)?))
            }
            TokenizerBackend::HuggingFace => hf_tokenizers::Tokenizer::from_file(path)
                .map(Self::HuggingFace)
                .map_err(|error| anyhow!(error)),
            TokenizerBackend::CxuuParallel => cxuu_tokenizers::Tokenizer::from_file(path)
                .map(Self::CxuuParallel)
                .map_err(|error| anyhow!(error)),
        }
    }

    fn encode(&self, input: &str) -> Result<EncodeOutput> {
        match self {
            Self::Snaptokens(tokenizer) => Ok(EncodeOutput::Ids(tokenizer.encode(input, false)?)),
            Self::Fastokens(tokenizer) => Ok(EncodeOutput::Ids(tokenizer.encode(input)?)),
            Self::HuggingFace(tokenizer) => tokenizer
                .encode_fast(input, false)
                .map(EncodeOutput::HuggingFace)
                .map_err(|error| anyhow!(error)),
            Self::CxuuParallel(tokenizer) => tokenizer
                .encode_parallel_single(input, false)
                .map(EncodeOutput::CxuuParallel)
                .map_err(|error| anyhow!(error)),
        }
    }
}

// Retains each backend's native output so its destruction stays inside the timer.
enum EncodeOutput {
    Ids(Vec<u32>),
    HuggingFace(hf_tokenizers::Encoding),
    CxuuParallel(cxuu_tokenizers::Encoding),
}

impl EncodeOutput {
    fn ids(&self) -> &[u32] {
        match self {
            Self::Ids(ids) => ids,
            Self::HuggingFace(encoding) => encoding.get_ids(),
            Self::CxuuParallel(encoding) => encoding.get_ids(),
        }
    }
}

struct Args {
    model: String,
    tokenizer_path: PathBuf,
    corpus_path: PathBuf,
    input_bytes: usize,
}

struct RunMeta {
    host: String,
    host_fingerprint: String,
    source_commit: String,
    binary_sha256: String,
    cargo_lock_sha256: String,
    rustc_version: String,
    build_command: String,
    build_rustflags: String,
    tokenizer_sha256: String,
    corpus_sha256: String,
    cpu_set: String,
    threads: usize,
    tokenizers_parallelism: String,
}

impl RunMeta {
    fn load() -> Result<Self> {
        let threads = required_env("THREADS")?
            .parse()
            .context("THREADS is not a positive integer")?;
        ensure!(threads > 0, "THREADS must be positive");

        let rayon_threads: usize = required_env("RAYON_NUM_THREADS")?
            .parse()
            .context("RAYON_NUM_THREADS is not a positive integer")?;
        ensure!(
            rayon_threads == threads,
            "RAYON_NUM_THREADS={rayon_threads} differs from THREADS={threads}"
        );

        let tokenizers_parallelism = required_env("TOKENIZERS_PARALLELISM")?;
        ensure!(
            tokenizers_parallelism == "true",
            "TOKENIZERS_PARALLELISM must be exactly true"
        );

        Ok(Self {
            host: required_env("HOST_LABEL")?,
            host_fingerprint: required_env("HOST_FINGERPRINT")?,
            source_commit: required_env("SOURCE_COMMIT")?,
            binary_sha256: required_env("BINARY_SHA256")?,
            cargo_lock_sha256: required_env("CARGO_LOCK_SHA256")?,
            rustc_version: required_env("RUSTC_VERSION")?,
            build_command: required_env("BUILD_COMMAND")?,
            build_rustflags: required_env("BUILD_RUSTFLAGS")?,
            tokenizer_sha256: required_env("SNAPTOKENS_TOKENIZER_SHA256")?,
            corpus_sha256: required_env("SNAPTOKENS_CORPUS_SHA256")?,
            cpu_set: required_env("CPU_SET")?,
            threads,
            tokenizers_parallelism,
        })
    }
}

struct Measurement {
    round: usize,
    position: usize,
    backend: TokenizerBackend,
    corpus_range: Range<usize>,
    elapsed_ns: u64,
    id_count: usize,
    first_id: Option<u32>,
    last_id: Option<u32>,
}

fn required_env(name: &str) -> Result<String> {
    let value = env::var(name).with_context(|| format!("missing {name}"))?;
    ensure!(!value.is_empty(), "{name} must not be empty");
    Ok(value)
}

fn parse_args() -> Result<Args> {
    let mut args = env::args().skip(1);
    let model = args.next().context("missing model label")?;
    ensure!(
        model == "gpt-2" || model == "gpt-oss",
        "unsupported model {model}"
    );
    let tokenizer_path = PathBuf::from(args.next().context("missing tokenizer.json path")?);
    let corpus_path = PathBuf::from(args.next().context("missing enwik8 path")?);
    let input_bytes = args
        .next()
        .context("missing input byte size")?
        .parse()
        .context("input byte size is not an integer")?;
    ensure!(
        matches!(input_bytes, 262_144 | 1_048_576 | 4_194_304),
        "input size must be 256 KiB, 1 MiB, or 4 MiB"
    );
    ensure!(args.next().is_none(), "unexpected extra argument");
    Ok(Args {
        model,
        tokenizer_path,
        corpus_path,
        input_bytes,
    })
}

fn load_candidates(
    path: &Path,
    model: &str,
) -> Result<(Vec<(TokenizerBackend, Candidate)>, Option<String>)> {
    let mut candidates = Vec::with_capacity(CANDIDATE_COUNT);
    let mut fastokens_error = None;
    for backend in BACKENDS {
        match Candidate::load(backend, path) {
            Ok(candidate) => candidates.push((backend, candidate)),
            Err(error)
                if backend == TokenizerBackend::Fastokens
                    && model == "gpt-2"
                    && error.to_string().contains("unsupported model type") =>
            {
                fastokens_error = Some(error.to_string());
            }
            Err(error) => return Err(error.context(format!("failed to load {}", backend.label()))),
        }
    }
    Ok((candidates, fastokens_error))
}

fn ensure_inventory(
    stage: &str,
    expected: &[TokenizerBackend],
    actual: &[(TokenizerBackend, Candidate)],
) -> Result<()> {
    let same = expected.len() == actual.len()
        && expected
            .iter()
            .zip(actual)
            .all(|(expected_kind, (actual_kind, _))| expected_kind == actual_kind);
    ensure!(same, "{stage} changed the runnable candidate inventory");
    Ok(())
}

fn make_windows(corpus: &str, width: usize, count: usize) -> Result<Vec<Range<usize>>> {
    let mut windows = Vec::with_capacity(count);
    let mut cursor = 0usize;
    while windows.len() < count {
        ensure!(
            cursor + width <= corpus.len(),
            "enwik8 cannot supply {count} non-wrapping {width}-byte windows"
        );
        if corpus.is_char_boundary(cursor) && corpus.is_char_boundary(cursor + width) {
            windows.push(cursor..cursor + width);
            cursor += width;
        } else {
            cursor += 1;
        }
    }
    Ok(windows)
}

fn make_probes(oracle: &hf_tokenizers::Tokenizer, corpus: &str) -> Result<Vec<(String, String)>> {
    let mut probes = vec![
        ("empty".into(), String::new()),
        ("ascii".into(), "Hello, tokenizer!  123\n".into()),
        (
            "whitespace-unicode-long".into(),
            "  café e\u{301} 中文 العربية 👩🏽‍💻\t\n\r\n\u{00a0}word  ".repeat(4096),
        ),
    ];

    let seam_window = make_windows(corpus, 262_144, 1)?
        .pop()
        .context("missing seam probe window")?;
    let seam_input = &corpus[seam_window];
    let mut seam = seam_input.len() / 2;
    while !seam_input.is_char_boundary(seam) {
        seam += 1;
    }

    let mut added_tokens: Vec<_> = oracle.get_added_tokens_decoder().into_iter().collect();
    added_tokens.sort_by_key(|(id, _)| *id);
    for (id, token) in added_tokens {
        probes.push((
            format!("added-{id}-simple"),
            format!("left  {}  right", token.content),
        ));
        probes.push((
            format!("added-{id}-seam"),
            format!(
                "{} left  {}  right {}",
                &seam_input[..seam],
                token.content,
                &seam_input[seam..]
            ),
        ));
    }
    Ok(probes)
}

fn ensure_ids(
    stage: &str,
    label: &str,
    backend: TokenizerBackend,
    expected: &[u32],
    actual: &[u32],
) -> Result<()> {
    if expected == actual {
        return Ok(());
    }
    let first_difference = expected
        .iter()
        .zip(actual)
        .position(|(left, right)| left != right)
        .unwrap_or(expected.len().min(actual.len()));
    bail!(
        "{stage} mismatch for {label} in {} at token {first_difference}: expected {} IDs, got {}",
        backend.label(),
        expected.len(),
        actual.len()
    )
}

fn check_probes(
    candidates: &[(TokenizerBackend, Candidate)],
    oracle: &hf_tokenizers::Tokenizer,
    probes: &[(String, String)],
) -> Result<()> {
    for (label, input) in probes {
        let expected = oracle
            .encode_fast(input.as_str(), false)
            .map_err(|error| anyhow!(error))?;
        for (backend, candidate) in candidates {
            let actual = candidate.encode(input)?;
            ensure_ids(
                "pre-timing probe",
                label,
                *backend,
                expected.get_ids(),
                actual.ids(),
            )?;
        }
    }
    Ok(())
}

fn warm_candidates(candidates: &[(TokenizerBackend, Candidate)], input: &str) -> Result<()> {
    for (_, candidate) in candidates {
        let output = candidate.encode(black_box(input))?;
        let ids = output.ids();
        black_box((ids.len(), ids.first().copied(), ids.last().copied()));
        drop(output);
    }
    Ok(())
}

fn measure_candidate(
    candidate: &Candidate,
    input: &str,
) -> Result<(u64, usize, Option<u32>, Option<u32>)> {
    let start = Instant::now();
    let output = candidate.encode(black_box(input))?;
    let ids = output.ids();
    let observation = (ids.len(), ids.first().copied(), ids.last().copied());
    black_box(observation);
    drop(output);
    let elapsed_ns = u64::try_from(start.elapsed().as_nanos())
        .context("one encode duration exceeds u64 nanoseconds")?;
    Ok((elapsed_ns, observation.0, observation.1, observation.2))
}

fn run_schedule(
    candidates: &[(TokenizerBackend, Candidate)],
    orders: &[Vec<usize>],
    corpus: &str,
    windows: &[Range<usize>],
) -> Result<Vec<Measurement>> {
    ensure!(
        windows.len() == orders.len(),
        "Williams cycle needs one window per actual round"
    );
    let mut measurements = Vec::with_capacity(orders.len() * candidates.len());
    for (round, order) in orders.iter().enumerate() {
        let corpus_range = windows[round].clone();
        let input = &corpus[corpus_range.clone()];
        for (position, candidate_index) in order.iter().copied().enumerate() {
            let (backend, candidate) = &candidates[candidate_index];
            let (elapsed_ns, id_count, first_id, last_id) = measure_candidate(candidate, input)?;
            measurements.push(Measurement {
                round,
                position,
                backend: *backend,
                corpus_range: corpus_range.clone(),
                elapsed_ns,
                id_count,
                first_id,
                last_id,
            });
        }
    }
    Ok(measurements)
}

fn check_timed_windows(
    candidates: &[(TokenizerBackend, Candidate)],
    oracle: &hf_tokenizers::Tokenizer,
    corpus: &str,
    windows: &[Range<usize>],
) -> Result<()> {
    for (round, range) in windows.iter().enumerate() {
        let input = &corpus[range.clone()];
        let expected = oracle
            .encode_fast(input, false)
            .map_err(|error| anyhow!(error))?;
        for (backend, candidate) in candidates {
            let actual = candidate.encode(input)?;
            ensure_ids(
                "post-timing fresh-instance parity",
                &format!("round-{round}"),
                *backend,
                expected.get_ids(),
                actual.ids(),
            )?;
        }
    }
    Ok(())
}

fn write_rows(rows: &[Value]) -> Result<()> {
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    for row in rows {
        serde_json::to_writer(&mut output, row)?;
        output.write_all(b"\n")?;
    }
    output.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let meta = RunMeta::load()?;
    ensure!(
        BUILD_SOURCE_COMMIT == meta.source_commit,
        "binary was built from {BUILD_SOURCE_COMMIT}, not {}",
        meta.source_commit
    );
    ensure!(
        env::var_os("DEBUG_PARALLEL").is_none(),
        "DEBUG_PARALLEL must be unset because cxuu prints from the timed path"
    );

    rayon::ThreadPoolBuilder::new()
        .num_threads(meta.threads)
        .build_global()
        .context("Rayon global pool was initialized before the benchmark")?;
    ensure!(
        rayon::current_num_threads() == meta.threads,
        "Rayon worker count differs from THREADS"
    );

    let corpus = fs::read_to_string(&args.corpus_path)
        .with_context(|| format!("failed to read UTF-8 corpus {}", args.corpus_path.display()))?;

    let probe_oracle = hf_tokenizers::Tokenizer::from_file(&args.tokenizer_path)
        .map_err(|error| anyhow!(error))?;
    let probes = make_probes(&probe_oracle, &corpus)?;
    let (probe_candidates, fastokens_error) = load_candidates(&args.tokenizer_path, &args.model)?;
    check_probes(&probe_candidates, &probe_oracle, &probes)?;
    let runnable_backends: Vec<_> = probe_candidates
        .iter()
        .map(|(backend, _)| *backend)
        .collect();
    let orders = balanced_orders(runnable_backends.len(), REQUESTED_ROUNDS);
    let windows = make_windows(&corpus, args.input_bytes, orders.len() + 1)?;
    let warmup_range = windows[0].clone();
    let timed_windows = &windows[1..];
    drop(probe_candidates);
    drop(probe_oracle);

    let (timed_candidates, _) = load_candidates(&args.tokenizer_path, &args.model)?;
    ensure_inventory("pre-timing reload", &runnable_backends, &timed_candidates)?;
    warm_candidates(&timed_candidates, &corpus[warmup_range.clone()])?;
    let measurements = run_schedule(&timed_candidates, &orders, &corpus, timed_windows)?;
    drop(timed_candidates);

    let post_oracle = hf_tokenizers::Tokenizer::from_file(&args.tokenizer_path)
        .map_err(|error| anyhow!(error))?;
    let (post_candidates, _) = load_candidates(&args.tokenizer_path, &args.model)?;
    ensure_inventory("post-timing reload", &runnable_backends, &post_candidates)?;
    check_timed_windows(&post_candidates, &post_oracle, &corpus, timed_windows)?;

    let schedule: Vec<Vec<&str>> = orders
        .iter()
        .map(|order| {
            order
                .iter()
                .map(|index| runnable_backends[*index].label())
                .collect()
        })
        .collect();
    let runnable_labels: Vec<_> = runnable_backends
        .iter()
        .map(|backend| backend.label())
        .collect();
    let actual_rounds = orders.len();
    let runnable_candidate_count = runnable_backends.len();
    let unsupported_candidate_count = usize::from(fastokens_error.is_some());
    let timed_ranges: Vec<[usize; 2]> = timed_windows
        .iter()
        .map(|range| [range.start, range.end])
        .collect();
    let mut rows = Vec::with_capacity(measurements.len() + BACKENDS.len() + 2);
    rows.push(json!({
        "kind": "long_single_meta",
        "run_version": RUN_VERSION,
        "host": meta.host,
        "host_fingerprint": meta.host_fingerprint,
        "source_commit": meta.source_commit,
        "binary_sha256": meta.binary_sha256,
        "cargo_lock_sha256": meta.cargo_lock_sha256,
        "rustc_version": meta.rustc_version,
        "build_command": meta.build_command,
        "build_rustflags": meta.build_rustflags,
        "model": args.model,
        "tokenizer_sha256": meta.tokenizer_sha256,
        "corpus": "enwik8",
        "corpus_sha256": meta.corpus_sha256,
        "input_bytes": args.input_bytes,
        "cpu_set": meta.cpu_set,
        "threads": meta.threads,
        "rayon_threads": rayon::current_num_threads(),
        "rayon_num_threads_env": meta.threads,
        "tokenizers_parallelism": meta.tokenizers_parallelism,
        "requested_rounds": REQUESTED_ROUNDS,
        "actual_rounds": actual_rounds,
        "declared_candidate_count": BACKENDS.len(),
        "runnable_candidate_count": runnable_candidate_count,
        "unsupported_candidate_count": unsupported_candidate_count,
        "runnable_candidates": runnable_labels,
        "schedule": schedule,
        "schedule_scope": "one complete Williams cycle; position and directed carryover balanced within actual rounds; requested_rounds is a minimum",
        "warmups_per_candidate": 1,
        "warmup_range": [warmup_range.start, warmup_range.end],
        "timed_ranges": timed_ranges,
        "add_special_tokens": false,
        "output_cost_scope": "natural public output allocation, observation, and destruction are timed",
        "candidate_apis": {
            "snaptokens": "Tokenizer::encode; Vec token IDs",
            "fastokens": "Tokenizer::encode; Vec token IDs",
            "huggingface-serial": "Tokenizer::encode_fast; offset computation skipped; Encoding retained",
            "cxuu-parallel": "Tokenizer::encode_parallel_single; richer Encoding retained",
        },
        "semantic_scope": "complete token IDs only",
        "cxuu_debug_parallel_env": false,
        "word_ids_comparable": false,
        "word_ids_note": "cxuu parallel does not preserve upstream serial word IDs; this lane neither compares nor claims word-ID equivalence",
        "hf_revision": HF_REVISION,
        "cxuu_revision": CXUU_REVISION,
        "fastokens_revision": FASTOKENS_REVISION,
    }));

    for measurement in measurements {
        rows.push(json!({
            "kind": "long_single_measurement",
            "run_version": RUN_VERSION,
            "host": meta.host,
            "host_fingerprint": meta.host_fingerprint,
            "source_commit": meta.source_commit,
            "binary_sha256": meta.binary_sha256,
            "cargo_lock_sha256": meta.cargo_lock_sha256,
            "model": args.model,
            "tokenizer_sha256": meta.tokenizer_sha256,
            "corpus": "enwik8",
            "corpus_sha256": meta.corpus_sha256,
            "input_bytes": args.input_bytes,
            "corpus_start": measurement.corpus_range.start,
            "corpus_end": measurement.corpus_range.end,
            "round": measurement.round,
            "position": measurement.position,
            "implementation": measurement.backend.label(),
            "requested_rounds": REQUESTED_ROUNDS,
            "actual_rounds": actual_rounds,
            "runnable_candidate_count": runnable_candidate_count,
            "elapsed_ns": measurement.elapsed_ns,
            "id_count": measurement.id_count,
            "first_id": measurement.first_id,
            "last_id": measurement.last_id,
            "threads": meta.threads,
            "cpu_set": meta.cpu_set,
            "add_special_tokens": false,
            "semantic_scope": "complete token IDs only",
        }));
    }

    for backend in BACKENDS {
        let unsupported_error = if backend == TokenizerBackend::Fastokens {
            fastokens_error.as_deref()
        } else {
            None
        };
        let supported = unsupported_error.is_none();
        let checked_probe_count = if supported { probes.len() } else { 0 };
        let checked_timed_input_count = if supported { timed_windows.len() } else { 0 };
        let status = if supported {
            "exact_on_probes_and_all_timed_inputs"
        } else {
            "unsupported_model_type"
        };
        let candidate_measurements = if supported { actual_rounds } else { 0 };
        rows.push(json!({
            "kind": "long_single_candidate_coverage",
            "run_version": RUN_VERSION,
            "host": meta.host,
            "host_fingerprint": meta.host_fingerprint,
            "source_commit": meta.source_commit,
            "binary_sha256": meta.binary_sha256,
            "cargo_lock_sha256": meta.cargo_lock_sha256,
            "model": args.model,
            "tokenizer_sha256": meta.tokenizer_sha256,
            "corpus": "enwik8",
            "corpus_sha256": meta.corpus_sha256,
            "input_bytes": args.input_bytes,
            "implementation": backend.label(),
            "requested_rounds": REQUESTED_ROUNDS,
            "actual_rounds": actual_rounds,
            "checked_probe_count": checked_probe_count,
            "checked_timed_input_count": checked_timed_input_count,
            "measurement_count": candidate_measurements,
            "complete_id_parity": supported.then_some(true),
            "post_timing_fresh_instances": supported,
            "status": status,
            "error": unsupported_error,
        }));
    }

    rows.push(json!({
        "kind": "long_single_coverage",
        "run_version": RUN_VERSION,
        "host": meta.host,
        "host_fingerprint": meta.host_fingerprint,
        "source_commit": meta.source_commit,
        "binary_sha256": meta.binary_sha256,
        "cargo_lock_sha256": meta.cargo_lock_sha256,
        "model": args.model,
        "tokenizer_sha256": meta.tokenizer_sha256,
        "corpus": "enwik8",
        "corpus_sha256": meta.corpus_sha256,
        "input_bytes": args.input_bytes,
        "requested_rounds": REQUESTED_ROUNDS,
        "actual_rounds": actual_rounds,
        "probe_count": probes.len(),
        "timed_input_count": timed_windows.len(),
        "declared_candidate_count": BACKENDS.len(),
        "runnable_candidate_count": runnable_candidate_count,
        "unsupported_candidate_count": unsupported_candidate_count,
        "measurement_count": actual_rounds * runnable_candidate_count,
        "complete_id_parity_for_all_runnable_candidates": true,
        "post_timing_fresh_instances": true,
        "status": "exact_on_all_runnable_candidates",
    }));
    write_rows(&rows)
}
