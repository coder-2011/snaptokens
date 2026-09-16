#![recursion_limit = "256"]

mod native;

use std::{
    collections::HashSet,
    env, fs,
    hint::black_box,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use anyhow::{Context, Result, anyhow, ensure};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use native::{NativeTokenizer, tokendagger_runtime};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokenizers::EncodeInput;
use wordchipper::{
    SpecialFilter, TokenEncoder as _, TokenizerOptions, pretrained::openai::OATokenizer,
};

#[path = "../../shared.rs"]
mod shared;

use shared::{balanced_orders, williams_cycle};

const MIN_SAMPLE_BYTES: usize = 4 * 1024 * 1024;
const BUILD_SOURCE_COMMIT: &str = env!("SNAPTOKENS_BUILD_SOURCE_COMMIT");
const GPT2_PATTERN: &str =
    r"'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s";
const O200K_PATTERN: &str = concat!(
    r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    r"|\p{N}{1,3}",
    r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
    r"|\s*[\r\n]+",
    r"|\s+(?!\S)|\s+",
);

#[derive(Clone, Copy)]
enum Model {
    Gpt2,
    GptOss,
}

struct ModelSpec {
    label: &'static str,
    tiktoken_encoding: &'static str,
    tokenizer_sha256: &'static str,
    rank_sha256: &'static str,
    rank_url: &'static str,
    riptoken_pattern: &'static str,
    wordchipper_encoding: OATokenizer,
}

impl Model {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "gpt-2" => Ok(Self::Gpt2),
            "gpt-oss" => Ok(Self::GptOss),
            _ => Err(anyhow!("unsupported specialist model: {value}")),
        }
    }

    const fn spec(self) -> ModelSpec {
        match self {
            Self::Gpt2 => ModelSpec {
                label: "gpt-2",
                tiktoken_encoding: "gpt2",
                tokenizer_sha256: "8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6",
                rank_sha256: "306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930",
                rank_url: "https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken",
                riptoken_pattern: GPT2_PATTERN,
                wordchipper_encoding: OATokenizer::R50kBase,
            },
            Self::GptOss => ModelSpec {
                label: "gpt-oss",
                tiktoken_encoding: "o200k_harmony",
                tokenizer_sha256: "0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3",
                rank_sha256: "446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d",
                rank_url: "https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken",
                riptoken_pattern: O200K_PATTERN,
                wordchipper_encoding: OATokenizer::O200kHarmony,
            },
        }
    }

    fn candidate_exclusive_probes(self) -> &'static [&'static str] {
        match self {
            Self::Gpt2 => &[],
            Self::GptOss => &["<|reserved_200018|>", "<|reserved_201087|>"],
        }
    }
}

#[derive(Deserialize)]
struct AddedToken {
    id: u32,
    content: String,
    #[serde(default)]
    single_word: bool,
    #[serde(default)]
    lstrip: bool,
    #[serde(default)]
    rstrip: bool,
    #[serde(default)]
    normalized: bool,
    #[serde(default)]
    special: bool,
}

#[derive(Deserialize)]
struct TokenizerDocument {
    #[serde(default)]
    added_tokens: Vec<AddedToken>,
    normalizer: Option<Value>,
}

#[derive(Clone, Copy)]
enum Kind {
    Snaptokens,
    OpenAiTiktoken,
    RustTiktoken,
    Riptoken,
    Wordchipper,
    BlazeBpe,
    TokenDagger,
}

impl Kind {
    const fn label(self) -> &'static str {
        match self {
            Self::Snaptokens => "snaptokens",
            Self::OpenAiTiktoken => "openai-tiktoken",
            Self::RustTiktoken => "rust-tiktoken",
            Self::Riptoken => "riptoken",
            Self::Wordchipper => "wordchipper",
            Self::BlazeBpe => "blaze-bpe",
            Self::TokenDagger => "tokendagger",
        }
    }
}

enum Candidate {
    Snaptokens(snaptokens::Tokenizer),
    OpenAiTiktoken(openai_tiktoken::CoreBPE),
    RustTiktoken(&'static rust_tiktoken::CoreBpe),
    Riptoken(riptoken::CoreBPE),
    Wordchipper {
        tokenizer: Arc<wordchipper::Tokenizer<u32>>,
        special_filter: SpecialFilter,
    },
    BlazeBpe(blaze_bpe::Tokenizer),
    TokenDagger(NativeTokenizer),
}

impl Candidate {
    fn ids(&self, inputs: &[String], allowed_special: &HashSet<&str>) -> Result<Vec<Vec<u32>>> {
        match self {
            Self::Snaptokens(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode(&inputs[0], false)?])
            }
            Self::Snaptokens(tokenizer) => Ok(tokenizer.encode_batch(inputs, false)?),
            Self::OpenAiTiktoken(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode(&inputs[0], allowed_special)?.0])
            }
            Self::OpenAiTiktoken(tokenizer) => Ok(inputs
                .par_iter()
                .map(|input| {
                    tokenizer
                        .encode(input, allowed_special)
                        .map(|(ids, _)| ids)
                        .map_err(anyhow::Error::from)
                })
                .collect::<Result<_>>()?),
            Self::RustTiktoken(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode_with_special_tokens(&inputs[0])])
            }
            Self::RustTiktoken(tokenizer) => Ok(inputs
                .par_iter()
                .map(|input| tokenizer.encode_with_special_tokens(input))
                .collect()),
            Self::Riptoken(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode(&inputs[0], allowed_special)])
            }
            Self::Riptoken(tokenizer) => {
                // Keep Riptoken's borrowed-input setup inside timing.
                let refs: Vec<_> = inputs.iter().map(String::as_str).collect();
                Ok(tokenizer.encode_batch(&refs, allowed_special))
            }
            Self::Wordchipper {
                tokenizer,
                special_filter,
            } if inputs.len() == 1 => Ok(vec![
                tokenizer.try_encode(&inputs[0], Some(special_filter))?,
            ]),
            Self::Wordchipper {
                tokenizer,
                special_filter,
            } => {
                // Keep Wordchipper's borrowed-input setup inside timing.
                let refs: Vec<_> = inputs.iter().map(String::as_str).collect();
                Ok(tokenizer.try_encode_batch(&refs, Some(special_filter))?)
            }
            Self::BlazeBpe(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode_with_specials(&inputs[0])])
            }
            Self::BlazeBpe(tokenizer) => Ok(inputs
                .par_iter()
                .map(|input| tokenizer.encode_with_specials(input))
                .collect()),
            Self::TokenDagger(tokenizer) if inputs.len() == 1 => {
                Ok(vec![tokenizer.encode(&inputs[0])?])
            }
            Self::TokenDagger(tokenizer) => {
                // TokenDagger has no batch API; Rayon preserves input order.
                Ok(inputs
                    .par_iter()
                    .map(|input| tokenizer.encode(input))
                    .collect::<Result<_>>()?)
            }
        }
    }

    fn run_once(&self, inputs: &[String], allowed_special: &HashSet<&str>) -> Result<()> {
        black_box(self.ids(inputs, allowed_special)?);
        Ok(())
    }
}

struct Args {
    model: Model,
    tokenizer_path: PathBuf,
    rank_path: PathBuf,
    corpus_label: String,
    corpus_path: PathBuf,
    input_bytes: usize,
    batch_size: usize,
    requested_rounds: usize,
    sample_bytes: usize,
}

struct RunMeta {
    run_id: String,
    host: String,
    host_fingerprint: String,
    cpu_set: String,
    rayon_threads: usize,
    available_parallelism: usize,
    source_commit: String,
    binary_sha256: String,
    build_command: String,
    rustc_version: String,
    build_rustflags: String,
    cargo_lock_sha256: String,
    pcre2_library_path: String,
    pcre2_library_sha256: String,
}

struct Measurement {
    candidate_index: usize,
    kind: Kind,
    round: usize,
    position: usize,
    elapsed_ns: u128,
    total_bytes: usize,
}

struct Failure {
    status: &'static str,
    detail: String,
}

fn parse_args() -> Result<Args> {
    let mut args = env::args().skip(1);
    let model = Model::parse(&args.next().context("missing model label")?)?;
    let tokenizer_path = PathBuf::from(args.next().context("missing tokenizer.json path")?);
    let rank_path = PathBuf::from(args.next().context("missing .tiktoken path")?);
    let corpus_label = args.next().context("missing corpus label")?;
    let corpus_path = PathBuf::from(args.next().context("missing corpus path")?);
    let input_bytes = args
        .next()
        .context("missing input byte size")?
        .parse()
        .context("invalid input byte size")?;
    let batch_size = args
        .next()
        .context("missing batch size")?
        .parse()
        .context("invalid batch size")?;
    let requested_rounds = args
        .next()
        .unwrap_or_else(|| "18".into())
        .parse()
        .context("invalid round count")?;
    let sample_mib: usize = args
        .next()
        .unwrap_or_else(|| "4".into())
        .parse()
        .context("invalid sample MiB")?;
    ensure!(input_bytes > 0, "input byte size must be positive");
    ensure!(batch_size > 0, "batch size must be positive");
    ensure!(requested_rounds > 0, "round count must be positive");
    ensure!(sample_mib > 0, "sample MiB must be positive");
    Ok(Args {
        model,
        tokenizer_path,
        rank_path,
        corpus_label,
        corpus_path,
        input_bytes,
        batch_size,
        requested_rounds,
        sample_bytes: sample_mib * 1024 * 1024,
    })
}

fn run_meta() -> Result<RunMeta> {
    Ok(RunMeta {
        run_id: env::var("SNAPTOKENS_BENCH_RUN_ID").context("missing benchmark run ID")?,
        host: env::var("SNAPTOKENS_BENCH_HOST").context("missing benchmark host")?,
        host_fingerprint: env::var("SNAPTOKENS_HOST_FINGERPRINT")
            .context("missing host fingerprint")?,
        cpu_set: env::var("SNAPTOKENS_CPU_SET").context("missing CPU affinity set")?,
        rayon_threads: rayon::current_num_threads(),
        available_parallelism: std::thread::available_parallelism()?.get(),
        source_commit: env::var("SNAPTOKENS_SOURCE_COMMIT").context("missing source commit")?,
        binary_sha256: env::var("SNAPTOKENS_BINARY_SHA256")
            .context("missing benchmark binary SHA-256")?,
        build_command: env::var("SNAPTOKENS_BUILD_COMMAND").context("missing build command")?,
        rustc_version: env::var("SNAPTOKENS_RUSTC_VERSION").context("missing rustc version")?,
        build_rustflags: env::var("SNAPTOKENS_BUILD_RUSTFLAGS")
            .context("missing build RUSTFLAGS")?,
        cargo_lock_sha256: env::var("SNAPTOKENS_CARGO_LOCK_SHA256")
            .context("missing Cargo.lock SHA-256")?,
        pcre2_library_path: env::var("SNAPTOKENS_PCRE2_LIBRARY_PATH")
            .context("missing PCRE2 library path")?,
        pcre2_library_sha256: env::var("SNAPTOKENS_PCRE2_LIBRARY_SHA256")
            .context("missing PCRE2 library SHA-256")?,
    })
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn load_document(path: &Path, expected_sha256: &str) -> Result<TokenizerDocument> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let actual = sha256(&bytes);
    ensure!(
        actual == expected_sha256,
        "tokenizer SHA-256 mismatch: expected {expected_sha256}, got {actual}"
    );
    serde_json::from_slice(&bytes).context("failed to parse tokenizer.json")
}

fn validate_added_tokens(document: &TokenizerDocument) -> Result<()> {
    ensure!(
        document.normalizer.is_none(),
        "specialist candidates cannot reproduce tokenizer normalization"
    );
    let mut contents = HashSet::with_capacity(document.added_tokens.len());
    let mut ids = HashSet::with_capacity(document.added_tokens.len());
    for token in &document.added_tokens {
        ensure!(!token.content.is_empty(), "empty AddedToken is unsupported");
        ensure!(
            !token.single_word && !token.lstrip && !token.rstrip,
            "AddedToken {:?} needs unsupported boundary or whitespace behavior",
            token.content
        );
        ensure!(
            !token.normalized || document.normalizer.is_none(),
            "AddedToken {:?} needs normalized matching",
            token.content
        );
        ensure!(token.special, "non-special AddedToken is unsupported");
        ensure!(
            contents.insert(token.content.as_str()),
            "duplicate AddedToken content {:?}",
            token.content
        );
        ensure!(ids.insert(token.id), "duplicate AddedToken ID {}", token.id);
    }
    Ok(())
}

fn load_ranks(path: &Path, expected_sha256: &str) -> Result<FxHashMap<Vec<u8>, u32>> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let actual = sha256(&bytes);
    ensure!(
        actual == expected_sha256,
        "rank artifact SHA-256 mismatch: expected {expected_sha256}, got {actual}"
    );
    let text = std::str::from_utf8(&bytes).context("rank artifact is not UTF-8")?;
    let mut ranks = FxHashMap::default();
    ranks.reserve(text.lines().count());
    for (line_index, line) in text.lines().enumerate() {
        let (encoded, rank) = line
            .split_once(' ')
            .with_context(|| format!("invalid rank line {}", line_index + 1))?;
        let token = BASE64
            .decode(encoded)
            .with_context(|| format!("invalid base64 on rank line {}", line_index + 1))?;
        let rank = rank
            .parse()
            .with_context(|| format!("invalid rank on line {}", line_index + 1))?;
        ensure!(
            ranks.insert(token, rank).is_none(),
            "duplicate token on rank line {}",
            line_index + 1
        );
    }
    ensure!(!ranks.is_empty(), "rank artifact is empty");
    Ok(ranks)
}

fn load_candidates(
    args: &Args,
    spec: &ModelSpec,
    ranks: FxHashMap<Vec<u8>, u32>,
    added_tokens: &[AddedToken],
) -> Result<Vec<(Kind, Candidate)>> {
    let snaptokens = snaptokens::Tokenizer::load_file(&args.tokenizer_path)?;
    let rust_tiktoken = rust_tiktoken::get_encoding(spec.tiktoken_encoding)
        .with_context(|| format!("missing tiktoken encoding {}", spec.tiktoken_encoding))?;
    let specials: FxHashMap<_, _> = added_tokens
        .iter()
        .map(|token| (token.content.clone(), token.id))
        .collect();
    let tokendagger = matches!(args.model, Model::Gpt2)
        .then(|| NativeTokenizer::tokendagger(&ranks, &specials, spec.riptoken_pattern))
        .transpose()
        .context("failed to construct TokenDagger")?;
    let openai_tiktoken = openai_tiktoken::CoreBPE::new::<_, _, Vec<(String, (u32, u32))>>(
        ranks.clone(),
        specials.clone(),
        spec.riptoken_pattern,
    )
    .map_err(|error| anyhow!(error.to_string()))?;
    let riptoken = riptoken::CoreBPE::new(ranks, specials, spec.riptoken_pattern)?;
    let wordchipper_vocab = spec
        .wordchipper_encoding
        .load_path::<u32>(&args.rank_path)?;
    let wordchipper = TokenizerOptions::default()
        .with_parallel(true)
        .build(Arc::new(wordchipper_vocab));
    let wordchipper_special_filter = SpecialFilter::Include(
        added_tokens
            .iter()
            .map(|token| token.content.clone())
            .collect(),
    );
    let mut candidates = vec![
        (Kind::Snaptokens, Candidate::Snaptokens(snaptokens)),
        (
            Kind::OpenAiTiktoken,
            Candidate::OpenAiTiktoken(openai_tiktoken),
        ),
        (Kind::RustTiktoken, Candidate::RustTiktoken(rust_tiktoken)),
        (Kind::Riptoken, Candidate::Riptoken(riptoken)),
        (
            Kind::Wordchipper,
            Candidate::Wordchipper {
                tokenizer: wordchipper,
                special_filter: wordchipper_special_filter,
            },
        ),
    ];
    if let Some(tokenizer) = tokendagger {
        candidates.push((Kind::TokenDagger, Candidate::TokenDagger(tokenizer)));
    }
    if matches!(args.model, Model::GptOss) {
        // Blaze-BPE is admitted only for GPT-OSS/o200k.
        let data = fs::read_to_string(&args.rank_path)
            .with_context(|| format!("failed to read {}", args.rank_path.display()))?;
        let specials: Vec<_> = added_tokens
            .iter()
            .map(|token| (token.content.as_str(), token.id))
            .collect();
        let tokenizer =
            blaze_bpe::Tokenizer::from_tiktoken_data(&data, blaze_bpe::Pattern::O200k, &specials)?;
        candidates.push((Kind::BlazeBpe, Candidate::BlazeBpe(tokenizer)));
    }
    Ok(candidates)
}

fn load_corpus(path: &Path, target_bytes: usize) -> Result<Vec<String>> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read corpus {}", path.display()))?;
    let mut chunks = Vec::new();
    let mut start = 0;
    while start + target_bytes <= text.len() {
        let mut end = start + target_bytes;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        ensure!(end > start, "input byte size cannot hold one UTF-8 scalar");
        chunks.push(text[start..end].to_owned());
        start = end;
    }
    ensure!(!chunks.is_empty(), "corpus is smaller than one input");
    Ok(chunks)
}

fn round_pool(
    corpus: &[String],
    first_row: usize,
    batch_size: usize,
    loops: usize,
) -> Result<Vec<&[String]>> {
    let rows = batch_size
        .checked_mul(loops)
        .context("row count overflow")?;
    let end = first_row.checked_add(rows).context("row index overflow")?;
    ensure!(end <= corpus.len(), "novel-input corpus would wrap");
    Ok(corpus[first_row..end].chunks_exact(batch_size).collect())
}

fn measure(
    candidate: &Candidate,
    pool: &[&[String]],
    allowed_special: &HashSet<&str>,
) -> Result<(u128, usize)> {
    let total_bytes = pool
        .iter()
        .flat_map(|inputs| inputs.iter())
        .map(String::len)
        .sum();
    let started = Instant::now();
    for inputs in pool {
        candidate.run_once(inputs, allowed_special)?;
    }
    Ok((started.elapsed().as_nanos(), total_bytes))
}

fn oracle_ids(oracle: &tokenizers::Tokenizer, inputs: &[String]) -> Result<Vec<Vec<u32>>> {
    let inputs: Vec<_> = inputs
        .iter()
        .map(|input| EncodeInput::Single(input.as_str().into()))
        .collect();
    let encodings = oracle
        .encode_batch_fast(inputs, false)
        .map_err(|error| anyhow!(error))?;
    Ok(encodings
        .iter()
        .map(|encoding| encoding.get_ids().to_vec())
        .collect())
}

fn first_mismatch(expected: &[Vec<u32>], actual: &[Vec<u32>]) -> Option<String> {
    if expected.len() != actual.len() {
        return Some(format!(
            "expected_inputs={} actual_inputs={}",
            expected.len(),
            actual.len()
        ));
    }
    for (input_index, (expected_ids, actual_ids)) in expected.iter().zip(actual).enumerate() {
        let token_count = expected_ids.len().max(actual_ids.len());
        for token_index in 0..token_count {
            let expected_id = expected_ids.get(token_index).copied();
            let actual_id = actual_ids.get(token_index).copied();
            if expected_id != actual_id {
                return Some(format!(
                    "input={input_index} token={token_index} expected={expected_id:?} actual={actual_id:?}"
                ));
            }
        }
    }
    None
}

fn parity_probes(model: Model, added_tokens: &[AddedToken]) -> Vec<String> {
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
    for token in added_tokens {
        let content = &token.content;
        probes.extend([
            content.clone(),
            format!("left{content}right"),
            format!("left {content} right"),
            format!("{content}{content}"),
            format!("\t{content}\r\n"),
            format!("é{content}e\u{301}"),
            content.to_lowercase(),
            content.to_uppercase(),
        ]);
    }
    probes.extend(
        model
            .candidate_exclusive_probes()
            .iter()
            .map(|probe| (*probe).to_owned()),
    );
    let mut seen = HashSet::with_capacity(probes.len());
    probes.retain(|probe| seen.insert(probe.clone()));
    probes
}

fn parity_failures(
    args: &Args,
    candidates: &[(Kind, Candidate)],
    oracle: &tokenizers::Tokenizer,
    allowed_special: &HashSet<&str>,
    probes: &[String],
    corpus: &[String],
    loops: usize,
    rounds: usize,
) -> Result<Vec<Option<Failure>>> {
    let expected_probes = oracle_ids(oracle, probes)?;
    let mut failures: Vec<Option<Failure>> = (0..candidates.len()).map(|_| None).collect();
    for (index, (_, candidate)) in candidates.iter().enumerate() {
        match candidate.ids(probes, allowed_special) {
            Ok(actual) => {
                if let Some(detail) = first_mismatch(&expected_probes, &actual) {
                    failures[index] = Some(Failure {
                        status: "probe_mismatch_timings_discarded",
                        detail,
                    });
                }
            }
            Err(error) => {
                failures[index] = Some(Failure {
                    status: "probe_encode_error_timings_discarded",
                    detail: error.to_string(),
                });
            }
        }
    }

    for round in 0..rounds {
        let first_row = round * loops * args.batch_size;
        let pool = round_pool(corpus, first_row, args.batch_size, loops)?;
        for (batch_index, inputs) in pool.iter().enumerate() {
            let expected = oracle_ids(oracle, inputs)?;
            for (index, (_, candidate)) in candidates.iter().enumerate() {
                if failures[index].is_some() {
                    continue;
                }
                match candidate.ids(inputs, allowed_special) {
                    Ok(actual) => {
                        if let Some(detail) = first_mismatch(&expected, &actual) {
                            failures[index] = Some(Failure {
                                status: "timed_input_mismatch_timings_discarded",
                                detail: format!("round={round} batch={batch_index} {detail}"),
                            });
                        }
                    }
                    Err(error) => {
                        failures[index] = Some(Failure {
                            status: "timed_input_encode_error_timings_discarded",
                            detail: format!("round={round} batch={batch_index} error={error}"),
                        });
                    }
                }
            }
        }
    }
    Ok(failures)
}

fn emit(value: Value) {
    println!("{value}");
}

fn cell_row(
    kind: &str,
    implementation: &str,
    args: &Args,
    spec: &ModelSpec,
    meta: &RunMeta,
) -> serde_json::Map<String, Value> {
    let mut row = serde_json::Map::new();
    row.insert("kind".into(), kind.into());
    row.insert("run_id".into(), meta.run_id.clone().into());
    row.insert("host".into(), meta.host.clone().into());
    row.insert("cpu_set".into(), meta.cpu_set.clone().into());
    row.insert("rayon_threads".into(), meta.rayon_threads.into());
    row.insert("model".into(), spec.label.into());
    row.insert("corpus".into(), args.corpus_label.clone().into());
    row.insert("input_bytes".into(), args.input_bytes.into());
    row.insert("batch".into(), args.batch_size.into());
    row.insert("track".into(), "novel_inputs_persistent_instance".into());
    row.insert("implementation".into(), implementation.into());
    row
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let spec = args.model.spec();
    let meta = run_meta()?;
    ensure!(
        BUILD_SOURCE_COMMIT == meta.source_commit,
        "binary was built from {BUILD_SOURCE_COMMIT}, not {}",
        meta.source_commit
    );
    let native_runtime = tokendagger_runtime()?;
    ensure!(
        args.sample_bytes >= MIN_SAMPLE_BYTES,
        "samples smaller than 4 MiB are not permitted"
    );
    let document = load_document(&args.tokenizer_path, spec.tokenizer_sha256)?;
    validate_added_tokens(&document)?;
    let ranks = load_ranks(&args.rank_path, spec.rank_sha256)?;
    let corpus = load_corpus(&args.corpus_path, args.input_bytes)?;
    let first_batch_bytes = corpus
        .iter()
        .take(args.batch_size)
        .map(String::len)
        .sum::<usize>();
    ensure!(
        first_batch_bytes > 0,
        "corpus has fewer rows than one batch"
    );
    let loops = args.sample_bytes.div_ceil(first_batch_bytes).max(1);
    let candidates = load_candidates(&args, &spec, ranks, &document.added_tokens)?;
    let williams_cycle_orders = williams_cycle(candidates.len()).len();
    let orders = balanced_orders(candidates.len(), args.requested_rounds);
    let rows_per_cell = orders
        .len()
        .checked_mul(loops)
        .and_then(|rows| rows.checked_mul(args.batch_size))
        .context("cell row count overflow")?;
    let required_rows = rows_per_cell
        .checked_add(args.batch_size)
        .context("warmup row count overflow")?;
    ensure!(
        corpus.len() >= required_rows,
        "corpus has {} rows but this novel-input cell needs {required_rows}",
        corpus.len()
    );

    let allowed_special: HashSet<_> = document
        .added_tokens
        .iter()
        .map(|token| token.content.as_str())
        .collect();
    let oracle =
        tokenizers::Tokenizer::from_file(&args.tokenizer_path).map_err(|error| anyhow!(error))?;

    // Pay lazy scanner, allocator, and per-thread cache setup on one whole
    // batch that cannot reappear in any timed round.
    let initialization_pool = round_pool(&corpus, rows_per_cell, args.batch_size, 1)?;
    for (kind, candidate) in &candidates {
        candidate
            .run_once(initialization_pool[0], &allowed_special)
            .with_context(|| format!("{} initialization failed", kind.label()))?;
    }

    emit(json!({
        "kind": "metadata",
        "run_id": meta.run_id,
        "host": meta.host,
        "host_fingerprint": meta.host_fingerprint,
        "cpu_set": meta.cpu_set,
        "rayon_threads": meta.rayon_threads,
        "available_parallelism": meta.available_parallelism,
        "source_commit": meta.source_commit,
        "binary_sha256": meta.binary_sha256,
        "build_command": meta.build_command,
        "rustc_version": meta.rustc_version,
        "build_rustflags": meta.build_rustflags,
        "cargo_lock_sha256": meta.cargo_lock_sha256,
        "model": spec.label,
        "tokenizer_sha256": spec.tokenizer_sha256,
        "rank_sha256": spec.rank_sha256,
        "rank_url": spec.rank_url,
        "openai_tiktoken_version": "0.13.0",
        "openai_tiktoken_revision": "08a5f3b2c987ada4fc5aa1f16c643c203fa8acaa",
        "goliajp_rust_tiktoken_version": "3.5.1",
        "goliajp_rust_tiktoken_revision": "1e3efa1c665b646dfe46cc667b4d46dddf56cf71",
        "riptoken_version": "0.3.0",
        "riptoken_revision": "1ac8add56e02c3d711b13f1059f6b522ee7b2010",
        "wordchipper_version": "0.9.2",
        "wordchipper_revision": "e82a4bdb656afe6d212f4ffbaa4f14e7d7071fca",
        "blaze_bpe_version": "0.1.0",
        "blaze_bpe_revision": "b753de2a2688c4acf09e64605585ea8e514dddd4",
        "blaze_bpe_scope": "fixed GPT-OSS/o200k specialist; GPT-2 is unsupported",
        "tokendagger_revision": env!("TOKENDAGGER_REVISION"),
        "tokendagger_scope": "fixed GPT-2/r50k public scalar API; GPT-OSS excluded for source-level iterator invalidation",
        "tokendagger_build_policy": "unmodified vendored C++17 source compiled at -O3 with the target-architecture equivalent of host-native tuning",
        "tokendagger_cxx_path": env!("TOKENDAGGER_CXX_PATH"),
        "tokendagger_cxx_version": env!("TOKENDAGGER_CXX_VERSION"),
        "tokendagger_cxx_arguments": env!("TOKENDAGGER_CXX_ARGS"),
        "tokendagger_pcre2_header_version": native_runtime.pcre2_header_version,
        "tokendagger_pcre2_runtime_version": native_runtime.pcre2_version,
        "tokendagger_pcre2_jit_available": native_runtime.pcre2_jit_available,
        "tokendagger_pcre2_library_path": meta.pcre2_library_path,
        "tokendagger_pcre2_library_sha256": meta.pcre2_library_sha256,
        "oracle_revision": "b62132e4e0ec7518caba201408a680819dfdcd22",
        "public_api_contract": "text-to-token-ID vectors; no count-only or private core API",
        "batch_contract": "batch 1 uses direct encode for all; wider batches use native Snaptokens/Riptoken/Wordchipper batch and an order-preserving outer Rayon adapter for OpenAI tiktoken, rust-tiktoken, Blaze-BPE, and TokenDagger",
        "native_bridge_contract": "public C++ scalar encode returning std::vector; timed C ABI path copies each Rust input into std::string and copies output IDs once into the harness-owned Rust result",
        "eligibility_contract": "exact full IDs against Hugging Face add_special_tokens=false on every AddedToken-derived probe, candidate-only special-token probes, and every timed input",
        "timing_contract": "persistent instances; novel non-wrapping inputs; output allocation and destruction in scope; parity after timers; inexact timing rows discarded",
        "initialization_warmup": "one untimed disjoint whole batch precedes the first timer for every candidate",
        "gpu_used": false,
        "requested_rounds": args.requested_rounds,
        "actual_rounds": orders.len(),
        "williams_cycle_orders": williams_cycle_orders,
        "complete_williams_cycles": orders.len() / williams_cycle_orders,
        "sample_bytes_minimum": args.sample_bytes,
        "loops": loops,
    }));

    let mut measurements = Vec::with_capacity(orders.len() * candidates.len());
    for (round, order) in orders.iter().enumerate() {
        let first_row = round * loops * args.batch_size;
        let pool = round_pool(&corpus, first_row, args.batch_size, loops)?;
        for (position, &candidate_index) in order.iter().enumerate() {
            let (kind, candidate) = &candidates[candidate_index];
            let (elapsed_ns, total_bytes) = measure(candidate, &pool, &allowed_special)?;
            measurements.push(Measurement {
                candidate_index,
                kind: *kind,
                round,
                position,
                elapsed_ns,
                total_bytes,
            });
        }
    }

    let probes = parity_probes(args.model, &document.added_tokens);
    let failures = parity_failures(
        &args,
        &candidates,
        &oracle,
        &allowed_special,
        &probes,
        &corpus,
        loops,
        orders.len(),
    )?;
    for ((kind, _), failure) in candidates.iter().zip(&failures) {
        let mut row = cell_row("coverage", kind.label(), &args, &spec, &meta);
        match failure {
            Some(failure) => {
                row.insert("status".into(), failure.status.into());
                row.insert("error".into(), failure.detail.clone().into());
            }
            None => {
                row.insert(
                    "status".into(),
                    "exact_on_model_probes_and_all_timed_inputs".into(),
                );
                row.insert("probe_inputs".into(), probes.len().into());
                row.insert(
                    "added_tokens_probed".into(),
                    document.added_tokens.len().into(),
                );
                row.insert("coverage_rounds".into(), orders.len().into());
                row.insert("timed_batches_per_round".into(), loops.into());
            }
        }
        emit(Value::Object(row));
    }
    if matches!(args.model, Model::Gpt2) {
        let mut row = cell_row("coverage", Kind::BlazeBpe.label(), &args, &spec, &meta);
        row.insert("status".into(), "unsupported_model_pattern".into());
        row.insert(
            "error".into(),
            "Blaze-BPE 0.1.0 supports cl100k and o200k, not GPT-2/r50k".into(),
        );
        emit(Value::Object(row));
    }
    if matches!(args.model, Model::GptOss) {
        let mut row = cell_row("coverage", Kind::TokenDagger.label(), &args, &spec, &meta);
        row.insert("status".into(), "excluded_source_undefined_behavior".into());
        row.insert(
            "error".into(),
            "TokenDagger 1eed815 erases the current special-token map key during range iteration; GPT-OSS was not executed"
                .into(),
        );
        row.insert(
            "source_evidence".into(),
            "benchmarks/specialists/audits/tokendagger.md".into(),
        );
        emit(Value::Object(row));
    }
    for measurement in measurements {
        if failures[measurement.candidate_index].is_some() {
            continue;
        }
        let mib_per_s = measurement.total_bytes as f64 * 1_000_000_000.0
            / measurement.elapsed_ns as f64
            / (1024.0 * 1024.0);
        let mut row = cell_row("measurement", measurement.kind.label(), &args, &spec, &meta);
        row.insert("round".into(), measurement.round.into());
        row.insert("position".into(), measurement.position.into());
        row.insert("actual_rounds".into(), orders.len().into());
        row.insert("loops".into(), loops.into());
        row.insert("total_bytes".into(), measurement.total_bytes.into());
        row.insert(
            "elapsed_ns".into(),
            measurement.elapsed_ns.to_string().into(),
        );
        row.insert("mib_per_s".into(), mib_per_s.into());
        emit(Value::Object(row));
    }

    ensure!(failures[0].is_none(), "Snaptokens failed the parity gate");
    ensure!(
        failures[1..].iter().any(Option::is_none),
        "no specialist competitor passed the parity gate"
    );
    Ok(())
}
