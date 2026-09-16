use std::{
    collections::HashSet,
    env,
    ffi::{CStr, CString, c_char, c_int, c_void},
    fs,
    hint::black_box,
    marker::PhantomData,
    path::{Path, PathBuf},
    ptr::NonNull,
    slice,
    time::Instant,
};

use anyhow::{Context, Result, anyhow, ensure};
use libloading::Library;
use rayon::prelude::*;
use serde_json::{Value, json};
use splintr::Tokenize;
use tokenizers::EncodeInput;

#[path = "../../shared.rs"]
mod shared;

use shared::balanced_orders;

const MIN_SAMPLE_BYTES: usize = 4 * 1024 * 1024;
const PARITY_CHUNK_INPUTS: usize = 1024;
const BUILD_SOURCE_COMMIT: &str = env!("SNAPTOKENS_BUILD_SOURCE_COMMIT");
const BUILD_IREE_SOURCE_DIR: Option<&str> = option_env!("IREE_SOURCE_DIR");
const BUILD_CMAKE_TOOLCHAIN_FILE: Option<&str> = option_env!("CMAKE_TOOLCHAIN_FILE");

/// Identifies a tokenizer implementation without loading it or warming its caches.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenizerBackend {
    Snaptokens,
    Fastokens,
    HuggingFace,
    Gigatoken,
    Iree,
    QuickTok,
    Kitoken,
    Tokie,
    Splintr,
}

impl TokenizerBackend {
    // Order is part of the benchmark schedule and serialized result contract.
    const ALL: &'static [Self] = &[
        Self::Snaptokens,
        Self::Fastokens,
        Self::HuggingFace,
        Self::Gigatoken,
        Self::Iree,
        Self::QuickTok,
        Self::Kitoken,
        Self::Tokie,
        Self::Splintr,
    ];

    /// Returns the stable implementation label used by result readers.
    const fn label(self) -> &'static str {
        match self {
            Self::Snaptokens => "snaptokens",
            Self::Fastokens => "fastokens",
            Self::HuggingFace => "huggingface",
            Self::Gigatoken => "gigatoken",
            Self::Iree => "iree",
            Self::QuickTok => "quicktok-qwen3-c-abi",
            Self::Kitoken => "kitoken",
            Self::Tokie => "tokie",
            Self::Splintr => "splintr",
        }
    }
}

struct Gigatoken {
    tokenizer: gigatoken_rs::Tokenizer,
    workers: gigatoken_rs::WorkerPool,
}

impl Gigatoken {
    fn load(path: &Path) -> Result<Self> {
        let tokenizer = gigatoken_rs::load_tokenizer::hf::load_hf_bpe(path)
            .map_err(|error| anyhow!(error.to_string()))?;
        Ok(Self {
            tokenizer,
            workers: gigatoken_rs::WorkerPool::new(),
        })
    }

    fn ids(&self, inputs: &[String], add_special_tokens: bool) -> Result<Vec<Vec<u32>>> {
        ensure!(
            !add_special_tokens,
            "Gigatoken does not execute arbitrary Hugging Face post-processors"
        );
        let docs: Vec<_> = inputs.iter().map(|input| input.as_bytes()).collect();
        let (ids, lengths) =
            gigatoken_rs::encode_docs_ragged(&self.workers, &self.tokenizer, &docs);
        let lengths = lengths
            .into_iter()
            .map(|length| usize::try_from(length).context("negative Gigatoken row length"))
            .collect::<Result<Vec<_>>>()?;
        rows_from_ragged(ids, lengths, "Gigatoken")
    }

    fn run_once(&self, inputs: &[String], add_special_tokens: bool) -> Result<()> {
        ensure!(
            !add_special_tokens,
            "Gigatoken does not execute arbitrary Hugging Face post-processors"
        );
        let docs: Vec<_> = inputs.iter().map(|input| input.as_bytes()).collect();
        black_box(gigatoken_rs::encode_docs_ragged(
            &self.workers,
            &self.tokenizer,
            &docs,
        ));
        Ok(())
    }
}

fn rows_from_ragged(
    ids: Vec<u32>,
    lengths: Vec<usize>,
    implementation: &str,
) -> Result<Vec<Vec<u32>>> {
    let mut offset = 0usize;
    let mut rows = Vec::with_capacity(lengths.len());
    for length in lengths {
        let end = offset
            .checked_add(length)
            .with_context(|| format!("{implementation} row length overflow"))?;
        ensure!(
            end <= ids.len(),
            "{implementation} row lengths exceed its ID buffer"
        );
        rows.push(ids[offset..end].to_vec());
        offset = end;
    }
    ensure!(
        offset == ids.len(),
        "{implementation} returned trailing unowned IDs"
    );
    Ok(rows)
}

type QuickTokLoad =
    unsafe extern "C" fn(*const c_char, *const c_char, *mut c_char, usize) -> *mut c_void;
type QuickTokFree = unsafe extern "C" fn(*mut c_void);
type QuickTokEncode =
    unsafe extern "C" fn(*const c_void, *const c_char, usize, *mut *mut u32, *mut usize) -> c_int;
type QuickTokIdsFree = unsafe extern "C" fn(*mut u32);

struct QuickTok {
    _library: Library,
    handle: NonNull<c_void>,
    tokenizer_free: QuickTokFree,
    encode_with_special: QuickTokEncode,
    ids_free: QuickTokIdsFree,
}

// SAFETY: QuickTok documents that a loaded handle is immutable and safe for
// concurrent encode calls; this wrapper exposes no mutation or handle aliasing.
unsafe impl Send for QuickTok {}
// SAFETY: The same upstream thread-safety contract permits shared encode calls,
// and the handle is freed only after the owning engine is no longer borrowed.
unsafe impl Sync for QuickTok {}

impl QuickTok {
    fn load() -> Result<Self> {
        let library_path = env::var("QUICKTOK_LIBRARY").context("missing QUICKTOK_LIBRARY")?;
        let data_dir = env::var("QUICKTOK_DATA_DIR").context("missing QUICKTOK_DATA_DIR")?;
        let data_dir = CString::new(data_dir).context("QuickTok data path contains NUL")?;
        let encoding = CString::new("qwen3").expect("static encoding has no NUL");

        // SAFETY: The path is supplied by the pinned benchmark driver, and the
        // loaded library remains owned by this wrapper until after handle free.
        let library = unsafe { Library::new(&library_path) }
            .with_context(|| format!("failed to load QuickTok library {library_path}"))?;
        // SAFETY: The symbol names and signatures are fixed by QuickTok's public
        // quicktok.h at the pinned revision recorded in benchmark metadata.
        let load = unsafe { *library.get::<QuickTokLoad>(b"qt_load_dir\0")? };
        // SAFETY: Same pinned C ABI contract as `load` above.
        let tokenizer_free = unsafe { *library.get::<QuickTokFree>(b"qt_tokenizer_free\0")? };
        // SAFETY: Same pinned C ABI contract as `load` above.
        let encode_with_special =
            unsafe { *library.get::<QuickTokEncode>(b"qt_encode_with_special\0")? };
        // SAFETY: Same pinned C ABI contract as `load` above.
        let ids_free = unsafe { *library.get::<QuickTokIdsFree>(b"qt_ids_free\0")? };

        let mut error = [0 as c_char; 512];
        // SAFETY: Both C strings and the writable error buffer remain valid for
        // the complete call, and `load` returns an owned opaque handle.
        let handle = unsafe {
            load(
                data_dir.as_ptr(),
                encoding.as_ptr(),
                error.as_mut_ptr(),
                error.len(),
            )
        };
        let handle = NonNull::new(handle).ok_or_else(|| {
            // SAFETY: QuickTok guarantees a NUL-terminated message in this
            // fixed buffer whenever construction returns a null handle.
            let message = unsafe { CStr::from_ptr(error.as_ptr()) };
            anyhow!("QuickTok load failed: {}", message.to_string_lossy())
        })?;
        Ok(Self {
            _library: library,
            handle,
            tokenizer_free,
            encode_with_special,
            ids_free,
        })
    }

    fn encode(&self, input: &str) -> Result<QuickTokIds<'_>> {
        let mut ids = std::ptr::null_mut();
        let mut len = 0;
        // SAFETY: The handle and input bytes remain valid for the call, output
        // pointers are writable, and the returned allocation uses `ids_free`.
        let status = unsafe {
            (self.encode_with_special)(
                self.handle.as_ptr(),
                input.as_ptr().cast(),
                input.len(),
                &mut ids,
                &mut len,
            )
        };
        ensure!(status == 0, "QuickTok encode failed with status {status}");
        let ids = NonNull::new(ids).context("QuickTok returned a null ID allocation")?;
        Ok(QuickTokIds {
            ids,
            len,
            free: self.ids_free,
            _tokenizer: PhantomData,
        })
    }

    fn ids(&self, inputs: &[String], add_special_tokens: bool) -> Result<Vec<Vec<u32>>> {
        ensure!(
            !add_special_tokens,
            "QuickTok does not execute arbitrary Hugging Face post-processors"
        );
        if let [input] = inputs {
            return Ok(vec![self.encode(input)?.as_slice().to_vec()]);
        }
        inputs
            .par_iter()
            .map(|input| Ok(self.encode(input)?.as_slice().to_vec()))
            .collect()
    }

    fn run_once(&self, inputs: &[String], add_special_tokens: bool) -> Result<()> {
        ensure!(
            !add_special_tokens,
            "QuickTok does not execute arbitrary Hugging Face post-processors"
        );
        if let [input] = inputs {
            black_box(self.encode(input)?);
            return Ok(());
        }
        let encoded: Result<Vec<_>> = inputs.par_iter().map(|input| self.encode(input)).collect();
        black_box(encoded?);
        Ok(())
    }
}

impl Drop for QuickTok {
    fn drop(&mut self) {
        // SAFETY: This wrapper exclusively owns the live handle, and the C ABI
        // free function belongs to the still-loaded `_library` field.
        unsafe { (self.tokenizer_free)(self.handle.as_ptr()) };
    }
}

struct QuickTokIds<'tokenizer> {
    ids: NonNull<u32>,
    len: usize,
    free: QuickTokIdsFree,
    // Keeps the dynamic library borrowed until the paired C-ABI free completes.
    _tokenizer: PhantomData<&'tokenizer QuickTok>,
}

// SAFETY: Each owner has exclusive access to one C allocation, and QuickTok
// documents both encoding and the paired free function as thread-safe.
unsafe impl Send for QuickTokIds<'_> {}

impl QuickTokIds<'_> {
    fn as_slice(&self) -> &[u32] {
        // SAFETY: QuickTok returns `len` initialized u32 values and preserves
        // them until this owner calls the paired C ABI free function.
        unsafe { slice::from_raw_parts(self.ids.as_ptr(), self.len) }
    }
}

impl Drop for QuickTokIds<'_> {
    fn drop(&mut self) {
        // SAFETY: This owner frees the allocation exactly once with the paired
        // function exported by the same still-loaded QuickTok library.
        unsafe { (self.free)(self.ids.as_ptr()) };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Track {
    Novel,
    WarmRepeated,
}

impl Track {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "novel" => Ok(Self::Novel),
            "warm-repeated" => Ok(Self::WarmRepeated),
            _ => Err(anyhow!("unknown track: {value}")),
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Novel => "novel_inputs_persistent_instance",
            Self::WarmRepeated => "warm_repeated",
        }
    }
}

enum Engine {
    Snaptokens(snaptokens::Tokenizer),
    Fastokens(fastokens::Tokenizer),
    HuggingFace(tokenizers::Tokenizer),
    Gigatoken(Gigatoken),
    Iree(iree_tokenizer::Tokenizer),
    QuickTok(QuickTok),
    Kitoken(kitoken::Kitoken),
    Tokie(tokie::Tokenizer),
    Splintr(splintr::AnyTokenizer),
}

impl Engine {
    /// Derives identity from the loaded variant without touching its tokenizer.
    fn backend(&self) -> TokenizerBackend {
        match self {
            Self::Snaptokens(_) => TokenizerBackend::Snaptokens,
            Self::Fastokens(_) => TokenizerBackend::Fastokens,
            Self::HuggingFace(_) => TokenizerBackend::HuggingFace,
            Self::Gigatoken(_) => TokenizerBackend::Gigatoken,
            Self::Iree(_) => TokenizerBackend::Iree,
            Self::QuickTok(_) => TokenizerBackend::QuickTok,
            Self::Kitoken(_) => TokenizerBackend::Kitoken,
            Self::Tokie(_) => TokenizerBackend::Tokie,
            Self::Splintr(_) => TokenizerBackend::Splintr,
        }
    }

    /// Loads the selected backend; fixed-model backends use their own artifact.
    fn load(backend: TokenizerBackend, path: &Path) -> Result<Self> {
        match backend {
            TokenizerBackend::Snaptokens => Ok(Self::Snaptokens(snaptokens::Tokenizer::load_file(
                path,
                snaptokens::LoadMode::JsonOnly,
            )?)),
            TokenizerBackend::Fastokens => {
                Ok(Self::Fastokens(fastokens::Tokenizer::from_file(path)?))
            }
            TokenizerBackend::HuggingFace => tokenizers::Tokenizer::from_file(path)
                .map(Self::HuggingFace)
                .map_err(|error| anyhow!(error)),
            TokenizerBackend::Gigatoken => Gigatoken::load(path).map(Self::Gigatoken),
            TokenizerBackend::Iree => iree_tokenizer::Tokenizer::from_file(path)
                .map(Self::Iree)
                .map_err(|error| anyhow!(error)),
            TokenizerBackend::QuickTok => QuickTok::load().map(Self::QuickTok),
            TokenizerBackend::Kitoken => kitoken::Kitoken::from_file(path)
                .map(Self::Kitoken)
                .map_err(|error| anyhow!(error)),
            TokenizerBackend::Tokie => tokie::Tokenizer::from_json(path)
                .map(Self::Tokie)
                .map_err(|error| anyhow!(error)),
            TokenizerBackend::Splintr => splintr::from_json_path(path)
                .map(Self::Splintr)
                .map_err(|error| anyhow!(error)),
        }
    }

    fn ids(&self, inputs: &[String], add_special_tokens: bool) -> Result<Vec<Vec<u32>>> {
        match self {
            Self::Snaptokens(tokenizer) => {
                let (ids, lengths) = tokenizer.encode_batch_ragged(inputs, add_special_tokens)?;
                rows_from_ragged(ids, lengths, "Snaptokens")
            }
            Self::Fastokens(tokenizer) => Ok(tokenizer.encode_batch(inputs, add_special_tokens)?),
            Self::HuggingFace(tokenizer) => {
                let encoded = tokenizer
                    .encode_batch_fast(hf_inputs(inputs), add_special_tokens)
                    .map_err(|error| anyhow!(error))?;
                Ok(encoded
                    .iter()
                    .map(|encoding| encoding.get_ids().to_vec())
                    .collect())
            }
            Self::Gigatoken(tokenizer) => tokenizer.ids(inputs, add_special_tokens),
            Self::Iree(tokenizer) => {
                let input_refs = input_refs(inputs);
                tokenizer
                    .encode_batch(&input_refs, add_special_tokens)?
                    .into_iter()
                    .map(iree_ids)
                    .collect()
            }
            Self::QuickTok(tokenizer) => tokenizer.ids(inputs, add_special_tokens),
            Self::Kitoken(tokenizer) => inputs
                .par_iter()
                .map(|input| {
                    // Kitoken's boolean recognizes special-token text; it is not
                    // equivalent to Hugging Face post-processor insertion.
                    tokenizer
                        .encode(input, true)
                        .map_err(|error| anyhow!(error))
                })
                .collect(),
            Self::Tokie(tokenizer) => {
                let refs = input_refs(inputs);
                let encoded = tokenizer.encode_batch(&refs, add_special_tokens);
                Ok(encoded.into_iter().map(|encoding| encoding.ids).collect())
            }
            Self::Splintr(tokenizer) => Ok(inputs
                .par_iter()
                .map(|input| {
                    if add_special_tokens {
                        tokenizer.encode_with_special_tokens(input)
                    } else {
                        tokenizer.encode(input)
                    }
                })
                .collect()),
        }
    }

    fn run_once(&self, inputs: &[String], add_special_tokens: bool) -> Result<()> {
        match self {
            Self::Snaptokens(tokenizer) => {
                black_box(tokenizer.encode_batch_ragged(inputs, add_special_tokens)?);
            }
            Self::Fastokens(tokenizer) => {
                black_box(tokenizer.encode_batch(inputs, add_special_tokens)?);
            }
            Self::HuggingFace(tokenizer) => {
                let encoded = tokenizer
                    .encode_batch_fast(hf_inputs(inputs), add_special_tokens)
                    .map_err(|error| anyhow!(error))?;
                black_box(encoded);
            }
            Self::Gigatoken(tokenizer) => tokenizer.run_once(inputs, add_special_tokens)?,
            Self::Iree(tokenizer) => {
                let input_refs = input_refs(inputs);
                let encoded = tokenizer.encode_batch(&input_refs, add_special_tokens)?;
                black_box(encoded);
            }
            Self::QuickTok(tokenizer) => tokenizer.run_once(inputs, add_special_tokens)?,
            Self::Kitoken(tokenizer) => {
                let encoded: Result<Vec<_>> = inputs
                    .par_iter()
                    .map(|input| {
                        tokenizer
                            .encode(input, true)
                            .map_err(|error| anyhow!(error))
                    })
                    .collect();
                black_box(encoded?);
            }
            Self::Tokie(tokenizer) => {
                let refs = input_refs(inputs);
                black_box(tokenizer.encode_batch(&refs, add_special_tokens));
            }
            Self::Splintr(tokenizer) => {
                let encoded: Vec<_> = inputs
                    .par_iter()
                    .map(|input| {
                        if add_special_tokens {
                            tokenizer.encode_with_special_tokens(input)
                        } else {
                            tokenizer.encode(input)
                        }
                    })
                    .collect();
                black_box(encoded);
            }
        }
        Ok(())
    }
}

enum CorpusSource {
    Synthetic(&'static str),
    File(Vec<String>),
}

struct Corpus {
    label: String,
    input_bytes: usize,
    source: CorpusSource,
}

impl Corpus {
    fn parse(spec: &str) -> Result<Self> {
        if let Some(rest) = spec.strip_prefix("file:") {
            let mut parts = rest.splitn(3, ':');
            let label = parts.next().context("missing file corpus label")?;
            let input_bytes = parts
                .next()
                .context("missing file corpus input size")?
                .parse()
                .context("invalid file corpus input size")?;
            let path = parts.next().context("missing file corpus path")?;
            ensure!(input_bytes > 0, "file corpus input size must be positive");
            let text = fs::read_to_string(path)
                .with_context(|| format!("failed to read corpus {path}"))?;
            let chunks = utf8_chunks(&text, input_bytes);
            ensure!(
                chunks.len() >= 2,
                "file corpus must contain at least two rows"
            );
            return Ok(Self {
                label: label.into(),
                input_bytes,
                source: CorpusSource::File(chunks),
            });
        }

        let (seed, input_bytes) = match spec {
            "repeated-chat64" => ("Summarize café 東京 in Rust. ", 64),
            "repeated-chat140" => (
                "Explain the tradeoff, preserve café and 東京, then return concise Rust code. ",
                140,
            ),
            "repeated-chat512" => (
                "Compare two systems carefully; include evidence, a caveat, emoji 🧪, and one code example. ",
                512,
            ),
            "repeated-code4k" => (
                "fn scan(bytes: &[u8]) -> usize { bytes.iter().filter(|&&b| b == b' ').count() } // exact UTF-8\n",
                4 * 1024,
            ),
            "repeated-multilingual4k" => (
                "English café; 日本語 東京; العربية لغة; हिन्दी भाषा; ไทย ภาษา; emoji 👩🏽‍💻; e\u{301} vs é. ",
                4 * 1024,
            ),
            "repeated-document4k" => (
                "Tokenization performance depends on scanning, normalization, allocation, cache locality, and merge scheduling. ",
                4 * 1024,
            ),
            "repeated-long64k" => (
                "Long-context inference should preserve exact identifiers, punctuation, whitespace, and Unicode while sustaining throughput. ",
                64 * 1024,
            ),
            "repeated-byte64k" => ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa ", 64 * 1024),
            _ => return Err(anyhow!("unknown corpus: {spec}")),
        };
        Ok(Self {
            label: spec.into(),
            input_bytes,
            source: CorpusSource::Synthetic(seed),
        })
    }

    fn make_inputs(&self, batch_size: usize, first_index: usize) -> Vec<String> {
        match &self.source {
            CorpusSource::Synthetic(seed) => (first_index..first_index + batch_size)
                .map(|index| sized_input(seed, index, self.input_bytes))
                .collect(),
            CorpusSource::File(chunks) => (first_index..first_index + batch_size)
                .map(|index| chunks[index % chunks.len()].clone())
                .collect(),
        }
    }
}

struct Args {
    path: PathBuf,
    model: String,
    corpus: Corpus,
    batch_size: usize,
    requested_rounds: usize,
    sample_bytes: usize,
    add_special_tokens: bool,
    track: Track,
}

struct RunMeta {
    run_id: String,
    host: String,
    host_fingerprint: String,
    tokenizer_sha256: String,
    cpu_set: String,
    rayon_threads: usize,
    available_parallelism: usize,
    source_commit: String,
    binary_sha256: String,
    build_command: String,
    rustc_version: String,
    build_rustflags: String,
    cargo_lock_sha256: String,
}

struct Difference {
    input_index: usize,
    token_index: usize,
    expected_id: Option<u32>,
    actual_id: Option<u32>,
    expected_inputs: usize,
    actual_inputs: usize,
}

struct Candidate {
    engine: Engine,
    failure: Option<String>,
}

fn hf_inputs(inputs: &[String]) -> Vec<EncodeInput<'_>> {
    inputs
        .iter()
        .map(|input| EncodeInput::Single(input.as_str().into()))
        .collect()
}

fn input_refs(inputs: &[String]) -> Vec<&str> {
    inputs.iter().map(String::as_str).collect()
}

fn iree_ids(ids: Vec<i32>) -> Result<Vec<u32>> {
    ids.into_iter()
        .map(|id| u32::try_from(id).map_err(|_| anyhow!("IREE returned negative token ID {id}")))
        .collect()
}

fn utf8_chunks(text: &str, target_bytes: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start + target_bytes <= text.len() {
        let mut end = start + target_bytes;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            break;
        }
        chunks.push(text[start..end].to_owned());
        start = end;
    }
    chunks
}

fn sized_input(seed: &str, index: usize, target_bytes: usize) -> String {
    let mut input = String::with_capacity(target_bytes + seed.len());
    let mut part = 0u64;
    while input.len() < target_bytes {
        // The marker prevents whole-input cache hits; repeated phrases remain
        // intentional in this separately labeled synthetic cache stress case.
        let marker = (index as u64)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(part as u32)
            ^ part;
        input.push_str(seed);
        input.push_str(&format!("{marker:016x} "));
        part += 1;
    }
    let mut end = target_bytes.min(input.len());
    while !input.is_char_boundary(end) {
        end -= 1;
    }
    input.truncate(end);
    input
}

fn make_pool(
    corpus: &Corpus,
    batch_size: usize,
    first_index: usize,
    batches: usize,
) -> Vec<Vec<String>> {
    (0..batches)
        .map(|batch| corpus.make_inputs(batch_size, first_index + batch * batch_size))
        .collect()
}

fn parity_probes(path: &Path) -> Result<(Vec<String>, usize)> {
    let document: Value = serde_json::from_slice(&fs::read(path)?)?;
    let added_tokens = document
        .get("added_tokens")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
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
        let Some(content) = token.get("content").and_then(Value::as_str) else {
            continue;
        };
        probes.extend([
            content.to_owned(),
            format!("left{content}right"),
            format!("left {content} right"),
            format!("left  {content}  right"),
            format!("\t{content}\r\n"),
            format!("é{content}e\u{301}"),
        ]);
        let lowercase = content.to_lowercase();
        let uppercase = content.to_uppercase();
        if lowercase != content {
            probes.push(lowercase);
        }
        if uppercase != content {
            probes.push(uppercase);
        }
        if content.contains('|') {
            probes.push(content.replace('|', "｜"));
        }
        if content.contains('｜') {
            probes.push(content.replace('｜', "|"));
        }
    }
    let mut seen = HashSet::with_capacity(probes.len());
    probes.retain(|probe| seen.insert(probe.clone()));
    Ok((probes, added_tokens.len()))
}

fn checksum(batch: &[Vec<u32>]) -> u64 {
    batch.iter().fold(batch.len() as u64, |hash, ids| {
        ids.iter().fold(
            hash.wrapping_mul(1_099_511_628_211)
                .wrapping_add(ids.len() as u64),
            |inner, &id| {
                inner
                    .wrapping_mul(1_099_511_628_211)
                    .wrapping_add(id as u64 + 1)
            },
        )
    })
}

fn chunked_ids(
    engine: &Engine,
    inputs: &[String],
    add_special_tokens: bool,
) -> Result<Vec<Vec<u32>>> {
    let mut ids = Vec::with_capacity(inputs.len());
    for chunk in inputs.chunks(PARITY_CHUNK_INPUTS) {
        ids.extend(engine.ids(chunk, add_special_tokens)?);
    }
    Ok(ids)
}

fn first_mismatch(expected: &[Vec<u32>], actual: &[Vec<u32>]) -> Option<Difference> {
    if expected.len() != actual.len() {
        let input_index = expected.len().min(actual.len());
        return Some(Difference {
            input_index,
            token_index: 0,
            expected_id: expected
                .get(input_index)
                .and_then(|ids| ids.first())
                .copied(),
            actual_id: actual.get(input_index).and_then(|ids| ids.first()).copied(),
            expected_inputs: expected.len(),
            actual_inputs: actual.len(),
        });
    }
    for (input_index, (expected_ids, actual_ids)) in expected.iter().zip(actual).enumerate() {
        let token_count = expected_ids.len().max(actual_ids.len());
        for token_index in 0..token_count {
            let expected_id = expected_ids.get(token_index).copied();
            let actual_id = actual_ids.get(token_index).copied();
            if expected_id != actual_id {
                return Some(Difference {
                    input_index,
                    token_index,
                    expected_id,
                    actual_id,
                    expected_inputs: expected.len(),
                    actual_inputs: actual.len(),
                });
            }
        }
    }
    None
}

fn measure(
    engine: &Engine,
    pool: &[Vec<String>],
    add_special_tokens: bool,
    loops: usize,
) -> Result<(u128, usize)> {
    let total_bytes = (0..loops)
        .map(|loop_index| {
            pool[loop_index % pool.len()]
                .iter()
                .map(String::len)
                .sum::<usize>()
        })
        .sum();
    let started = Instant::now();
    for loop_index in 0..loops {
        let inputs = &pool[loop_index % pool.len()];
        engine.run_once(inputs, add_special_tokens)?;
    }
    Ok((started.elapsed().as_nanos(), total_bytes))
}

fn parse_args() -> Result<Args> {
    let mut args = env::args().skip(1);
    let path = PathBuf::from(args.next().context("missing tokenizer path")?);
    let model = args.next().context("missing model label")?;
    let corpus = Corpus::parse(&args.next().context("missing corpus specification")?)?;
    let batch_size = args
        .next()
        .context("missing batch size")?
        .parse()
        .context("invalid batch size")?;
    let requested_rounds = args
        .next()
        .unwrap_or_else(|| "20".into())
        .parse()
        .context("invalid round count")?;
    let sample_mib: usize = args
        .next()
        .unwrap_or_else(|| "4".into())
        .parse()
        .context("invalid sample MiB")?;
    let add_special_tokens = args
        .next()
        .unwrap_or_else(|| "false".into())
        .parse()
        .context("invalid add-special-tokens flag")?;
    let track = Track::parse(&args.next().unwrap_or_else(|| "novel".into()))?;
    ensure!(batch_size > 0, "batch size must be positive");
    ensure!(requested_rounds > 0, "round count must be positive");
    ensure!(sample_mib > 0, "sample MiB must be positive");
    Ok(Args {
        path,
        model,
        corpus,
        batch_size,
        requested_rounds,
        sample_bytes: sample_mib * 1024 * 1024,
        add_special_tokens,
        track,
    })
}

fn run_meta() -> Result<RunMeta> {
    Ok(RunMeta {
        run_id: env::var("SNAPTOKENS_BENCH_RUN_ID").context("missing benchmark run ID")?,
        host: env::var("SNAPTOKENS_BENCH_HOST").context("missing benchmark host")?,
        host_fingerprint: env::var("SNAPTOKENS_HOST_FINGERPRINT")
            .context("missing host fingerprint")?,
        tokenizer_sha256: env::var("SNAPTOKENS_TOKENIZER_SHA256")
            .context("missing tokenizer SHA-256")?,
        cpu_set: env::var("SNAPTOKENS_CPU_SET").context("missing CPU affinity set")?,
        rayon_threads: rayon::current_num_threads(),
        available_parallelism: std::thread::available_parallelism()?.get(),
        source_commit: env::var("SNAPTOKENS_SOURCE_COMMIT").context("missing source commit")?,
        binary_sha256: env::var("SNAPTOKENS_BINARY_SHA256")
            .context("missing benchmark binary SHA-256")?,
        build_command: env::var("SNAPTOKENS_BUILD_COMMAND")
            .context("missing benchmark build command")?,
        rustc_version: env::var("SNAPTOKENS_RUSTC_VERSION").context("missing rustc version")?,
        build_rustflags: env::var("SNAPTOKENS_BUILD_RUSTFLAGS")
            .context("missing build RUSTFLAGS")?,
        cargo_lock_sha256: env::var("SNAPTOKENS_CARGO_LOCK_SHA256")
            .context("missing Cargo.lock SHA-256")?,
    })
}

fn emit(value: Value) {
    println!("{value}");
}

/// Adds run identity to a result record without changing its serialized field names.
fn keyed_row(
    record_type: &str,
    args: &Args,
    meta: &RunMeta,
    implementation: &str,
) -> serde_json::Map<String, Value> {
    let mut row = serde_json::Map::new();
    row.insert("kind".into(), record_type.into());
    row.insert("run_id".into(), meta.run_id.clone().into());
    row.insert("host".into(), meta.host.clone().into());
    row.insert(
        "host_fingerprint".into(),
        meta.host_fingerprint.clone().into(),
    );
    row.insert("cpu_set".into(), meta.cpu_set.clone().into());
    row.insert("rayon_threads".into(), meta.rayon_threads.into());
    row.insert(
        "available_parallelism".into(),
        meta.available_parallelism.into(),
    );
    row.insert("model".into(), args.model.clone().into());
    row.insert(
        "tokenizer_sha256".into(),
        meta.tokenizer_sha256.clone().into(),
    );
    row.insert("corpus".into(), args.corpus.label.clone().into());
    row.insert("batch".into(), args.batch_size.into());
    row.insert("add_special_tokens".into(), args.add_special_tokens.into());
    row.insert("track".into(), args.track.label().into());
    row.insert("implementation".into(), implementation.into());
    row
}

fn emit_failure(
    args: &Args,
    meta: &RunMeta,
    backend: TokenizerBackend,
    status: &str,
    error: String,
) {
    let mut row = keyed_row("coverage", args, meta, backend.label());
    row.insert("status".into(), status.into());
    row.insert("error".into(), error.into());
    emit(Value::Object(row));
}

fn loadable_candidates(args: &Args, meta: &RunMeta) -> Vec<TokenizerBackend> {
    let mut candidates = Vec::new();
    for &backend in TokenizerBackend::ALL {
        if backend == TokenizerBackend::QuickTok && args.model != "qwen-3" {
            emit_failure(
                args,
                meta,
                backend,
                "unsupported",
                "QuickTok is a fixed Qwen3 artifact, not a generic tokenizer.json engine".into(),
            );
            continue;
        }
        if matches!(
            backend,
            TokenizerBackend::QuickTok | TokenizerBackend::Gigatoken
        ) && args.add_special_tokens
        {
            emit_failure(
                args,
                meta,
                backend,
                "unsupported",
                format!(
                    "{} does not execute arbitrary Hugging Face post-processors",
                    backend.label()
                ),
            );
            continue;
        }
        match Engine::load(backend, &args.path) {
            Ok(_) => candidates.push(backend),
            Err(error) => emit_failure(args, meta, backend, "unsupported", error.to_string()),
        }
    }
    candidates
}

fn semantic_candidates(
    args: &Args,
    meta: &RunMeta,
    probes: &[String],
    candidates: &[TokenizerBackend],
) -> Result<Vec<TokenizerBackend>> {
    let oracle = Engine::load(TokenizerBackend::HuggingFace, &args.path)?;
    let expected = chunked_ids(&oracle, probes, args.add_special_tokens)?;
    let mut exact = Vec::new();
    for &backend in candidates {
        let engine = match Engine::load(backend, &args.path) {
            Ok(engine) => engine,
            Err(error) => {
                emit_failure(args, meta, backend, "reload_error", error.to_string());
                continue;
            }
        };
        let actual = match chunked_ids(&engine, probes, args.add_special_tokens) {
            Ok(ids) => ids,
            Err(error) => {
                emit_failure(args, meta, backend, "probe_encode_error", error.to_string());
                continue;
            }
        };
        if let Some(difference) = first_mismatch(&expected, &actual) {
            emit_failure(
                args,
                meta,
                backend,
                "probe_mismatch",
                format!(
                    "flag={} input={} token={} expected={:?} actual={:?} expected_inputs={} actual_inputs={}",
                    args.add_special_tokens,
                    difference.input_index,
                    difference.token_index,
                    difference.expected_id,
                    difference.actual_id,
                    difference.expected_inputs,
                    difference.actual_inputs,
                ),
            );
            continue;
        }
        exact.push(backend);
    }
    Ok(exact)
}

fn timed_candidates(
    args: &Args,
    meta: &RunMeta,
    candidates: Vec<TokenizerBackend>,
    loops: usize,
    coverage_rounds: usize,
) -> Result<Vec<TokenizerBackend>> {
    let oracle = Engine::load(TokenizerBackend::HuggingFace, &args.path)?;
    let mut states = Vec::with_capacity(candidates.len());
    for backend in candidates {
        match Engine::load(backend, &args.path) {
            Ok(engine) => states.push(Candidate {
                engine,
                failure: None,
            }),
            Err(error) => emit_failure(args, meta, backend, "reload_error", error.to_string()),
        }
    }

    for round in 0..coverage_rounds {
        let first_index = if args.track == Track::Novel {
            round * loops * args.batch_size
        } else {
            0
        };
        let pool_batches = if args.track == Track::Novel { loops } else { 1 };
        let pool = make_pool(&args.corpus, args.batch_size, first_index, pool_batches);
        for (batch_index, inputs) in pool.iter().enumerate() {
            let expected = oracle.ids(inputs, args.add_special_tokens)?;
            for state in &mut states {
                if state.failure.is_some() {
                    continue;
                }
                let actual = match state.engine.ids(inputs, args.add_special_tokens) {
                    Ok(ids) => ids,
                    Err(error) => {
                        state.failure = Some(format!(
                            "round={round} batch={batch_index} encode_error={error}"
                        ));
                        continue;
                    }
                };
                if let Some(difference) = first_mismatch(&expected, &actual) {
                    state.failure = Some(format!(
                        "round={round} batch={batch_index} input={} token={} expected={:?} actual={:?} expected_inputs={} actual_inputs={}",
                        difference.input_index,
                        difference.token_index,
                        difference.expected_id,
                        difference.actual_id,
                        difference.expected_inputs,
                        difference.actual_inputs,
                    ));
                }
            }
        }
    }

    let mut exact = Vec::new();
    for state in states {
        if let Some(error) = state.failure {
            emit_failure(
                args,
                meta,
                state.engine.backend(),
                "timed_input_mismatch",
                error,
            );
        } else {
            exact.push(state.engine.backend());
        }
    }
    Ok(exact)
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let meta = run_meta()?;
    // Reject binaries whose compile-time native-source provenance does not
    // match the clean checkout declared by the benchmark driver.
    ensure!(
        BUILD_SOURCE_COMMIT == meta.source_commit,
        "binary was built from {BUILD_SOURCE_COMMIT}, not {}",
        meta.source_commit
    );
    ensure!(
        BUILD_IREE_SOURCE_DIR.is_none(),
        "IREE_SOURCE_DIR overrides the pinned dependency source"
    );
    ensure!(
        BUILD_CMAKE_TOOLCHAIN_FILE.is_some(),
        "binary was built without the pinned IREE CMake compatibility file"
    );
    ensure!(
        args.sample_bytes >= MIN_SAMPLE_BYTES,
        "samples smaller than 4 MiB are not permitted"
    );
    let (probes, added_tokens) = parity_probes(&args.path)?;
    let first = args.corpus.make_inputs(args.batch_size, 0);
    let batch_bytes = first.iter().map(String::len).sum::<usize>();
    let loops = args.sample_bytes.div_ceil(batch_bytes).max(1);
    emit(json!({
        "kind": "metadata",
        "run_id": meta.run_id,
        "host": meta.host,
        "host_fingerprint": meta.host_fingerprint,
        "cpu_set": meta.cpu_set,
        "rayon_threads": meta.rayon_threads,
        "available_parallelism": meta.available_parallelism,
        "tokenizer_sha256": meta.tokenizer_sha256,
        "source_commit": meta.source_commit,
        "binary_sha256": meta.binary_sha256,
        "build_command": meta.build_command,
        "rustc_version": meta.rustc_version,
        "build_rustflags": meta.build_rustflags,
        "build_cc": option_env!("CC"),
        "build_cxx": option_env!("CXX"),
        "build_cflags": option_env!("CFLAGS"),
        "build_cxxflags": option_env!("CXXFLAGS"),
        "cargo_lock_sha256": meta.cargo_lock_sha256,
        "tokenizers_parallelism": env::var("TOKENIZERS_PARALLELISM").unwrap_or_default(),
        "public_api_contract": "text-to-IDs through public APIs; Snaptokens and Gigatoken timers use flat ragged output; Hugging Face uses encode_batch_fast; QuickTok uses its stable C ABI; Hugging Face and Tokie richer Encoding allocations remain in scope",
        "batch_contract": "native batch: snaptokens, fastokens, Hugging Face, Gigatoken, IREE, Tokie; direct scalar C ABI at batch 1 and order-preserving harness Rayon above batch 1: QuickTok; order-preserving harness Rayon adapter: Kitoken, Splintr; caller-side borrowed-input conversion remains in scope",
        "snaptokens_output_contract": "timing retains the native flat Vec<u32> plus row lengths, including allocation; untimed parity reconstructs rows from the same flat-ragged API",
        "gigatoken_revision": "e6564f3ad3364e58ff02b2b251a96b518b6eaaa2",
        "gigatoken_scope": "supported ByteLevel tokenizer.json subset; exact cells only; no arbitrary post-processor semantics",
        "gigatoken_output_contract": "native flat Vec<u32> plus row lengths, including gather and allocation; nested row conversion occurs only during untimed parity checks",
        "iree_output_contract": "timing retains the public Vec<i32> IDs; untimed eligibility checks convert them to the harness Vec<u32> contract and reject negative IDs",
        "iree_threading_contract": "IREE's public encode_batch is measured with the pinned native library's internal threading disabled",
        "iree_source_override": BUILD_IREE_SOURCE_DIR,
        "iree_cmake_toolchain_file": BUILD_CMAKE_TOOLCHAIN_FILE,
        "quicktok_revision": "c69b29886ac508841c698487799d1904d0f488e8",
        "quicktok_scope": "fixed Qwen3 specialist; excluded from generic tokenizer.json coverage and aggregates",
        "quicktok_library_sha256": env::var("QUICKTOK_LIBRARY_SHA256").unwrap_or_default(),
        "quicktok_build_command": env::var("QUICKTOK_BUILD_COMMAND").unwrap_or_default(),
        "quicktok_cxx_version": env::var("QUICKTOK_CXX_VERSION").unwrap_or_default(),
        "quicktok_data_sha256": {
            "qwen3.vocab": "6114087889f61af31f72abe80529bbd09826df4d396753a281a14bcc21abc2fa",
            "qwen3.special": "d3fccd273ee759d3d91c2a73458db4f5738d56e104f8cccfd21cb36f401b020a",
            "uniclass.bin": "6039f6f7bd056c0d211d02a907a8ed8f3317bb719fc495b9093f9cccca948809",
            "nfc.bin": "c5b8a5beee77e7dcea1a85aee9980d863a961d3ebd8aab0cca49fe4644eb4b72"
        },
        "quicktok_output_contract": "the public C ABI copies its native vector into an ABI-owned ID array; that allocation, copy, and paired free remain in scope",
        "initialization_warmup": "unique whole inputs on a warmed persistent instance: one untimed disjoint batch precedes the first novel timer; repeated-input cells additionally warm immediately before every sample",
        "eligibility_contract": "fresh-instance exact flat IDs and row lengths on model-derived probes and every timed input before and after timing",
        "sample_bytes_minimum": args.sample_bytes,
    }));

    let loadable = loadable_candidates(&args, &meta);
    ensure!(
        !loadable.is_empty(),
        "no implementation loaded the tokenizer"
    );
    let rows_per_round = loops
        .checked_mul(args.batch_size)
        .context("rows per round overflow")?;
    let max_coverage_rounds = (1..=loadable.len())
        .map(|count| balanced_orders(count, args.requested_rounds).len())
        .max()
        .context("no candidate schedules")?;
    if let CorpusSource::File(chunks) = &args.corpus.source {
        // Reserve for every smaller schedule because odd Williams designs can
        // require more rows after an inexact competitor is removed.
        let required_rows = rows_per_round
            .checked_mul(max_coverage_rounds)
            .context("maximum rows per cell overflow")?
            .checked_add(args.batch_size)
            .context("warmup row count overflow")?;
        ensure!(
            chunks.len() >= required_rows,
            "file corpus repeats inside one nominally novel cell"
        );
    }

    let mut candidates = semantic_candidates(&args, &meta, &probes, &loadable)?;
    loop {
        ensure!(
            !candidates.is_empty(),
            "no implementation passed preflight parity"
        );
        let coverage_rounds = balanced_orders(candidates.len(), args.requested_rounds).len();
        let exact = timed_candidates(&args, &meta, candidates.clone(), loops, coverage_rounds)?;
        if exact.len() == candidates.len() {
            candidates = exact;
            break;
        }
        // Removing one engine changes the balanced schedule, so verify the
        // surviving engines against the newly planned timed inputs.
        candidates = exact;
    }
    ensure!(
        candidates.contains(&TokenizerBackend::Snaptokens),
        "Snaptokens failed pre-timing parity"
    );
    let orders = balanced_orders(candidates.len(), args.requested_rounds);
    let coverage_rounds = orders.len();
    let rows_per_cell = rows_per_round
        .checked_mul(coverage_rounds)
        .context("rows per cell overflow")?;

    let engines = candidates
        .iter()
        .map(|&backend| Engine::load(backend, &args.path))
        .collect::<Result<Vec<_>>>()?;
    let warm_first_index = if args.track == Track::Novel {
        rows_per_cell
    } else {
        0
    };
    let initialization_pool = make_pool(&args.corpus, args.batch_size, warm_first_index, 1);
    for engine in &engines {
        // A disjoint whole input pays lazy scanner/runtime setup; shared token
        // pieces and allocator state may remain warm by design.
        black_box(measure(
            engine,
            &initialization_pool,
            args.add_special_tokens,
            1,
        )?);
    }
    let warm_loops = MIN_SAMPLE_BYTES.div_ceil(batch_bytes).max(1);
    let mut measurements = Vec::with_capacity(orders.len() * candidates.len());

    for (round, order) in orders.iter().enumerate() {
        let first_index = if args.track == Track::Novel {
            round * loops * args.batch_size
        } else {
            0
        };
        let pool_batches = if args.track == Track::Novel { loops } else { 1 };
        let pool = make_pool(&args.corpus, args.batch_size, first_index, pool_batches);
        for (position, &engine_index) in order.iter().enumerate() {
            let backend = candidates[engine_index];
            let engine = &engines[engine_index];
            if args.track == Track::WarmRepeated {
                // Immediate equal-byte warmup makes the recorded sample a
                // declared steady-state cache measurement.
                black_box(measure(engine, &pool, args.add_special_tokens, warm_loops)?);
            }
            let (elapsed_ns, total_bytes) = measure(engine, &pool, args.add_special_tokens, loops)?;
            let mib_per_s =
                total_bytes as f64 * 1_000_000_000.0 / elapsed_ns as f64 / (1024.0 * 1024.0);
            let mut row = keyed_row("measurement", &args, &meta, backend.label());
            row.insert("round".into(), round.into());
            row.insert("position".into(), position.into());
            row.insert("actual_rounds".into(), orders.len().into());
            row.insert("loops".into(), loops.into());
            row.insert("total_bytes".into(), total_bytes.into());
            row.insert("elapsed_ns".into(), elapsed_ns.to_string().into());
            row.insert("mib_per_s".into(), mib_per_s.into());
            measurements.push(Value::Object(row));
        }
    }

    // Fresh instances repeat the complete gate after every timer without
    // changing the state of any measured engine.
    let semantic_exact = semantic_candidates(&args, &meta, &probes, &candidates)?;
    let exact = timed_candidates(&args, &meta, semantic_exact, loops, coverage_rounds)?;
    ensure!(
        exact.contains(&TokenizerBackend::Snaptokens),
        "Snaptokens failed post-timing parity"
    );
    let probe_oracle = Engine::load(TokenizerBackend::HuggingFace, &args.path)?;
    let probe_checksum_false = checksum(&chunked_ids(&probe_oracle, &probes, false)?);
    let unique_batches_per_round = if args.track == Track::Novel { loops } else { 1 };
    for backend in &exact {
        let mut row = keyed_row("coverage", &args, &meta, backend.label());
        row.insert(
            "status".into(),
            "exact_on_probes_and_all_timed_inputs".into(),
        );
        row.insert("probe_inputs".into(), probes.len().into());
        row.insert("added_tokens_probed".into(), added_tokens.into());
        row.insert("coverage_rounds".into(), coverage_rounds.into());
        row.insert("timed_batches_per_round".into(), loops.into());
        row.insert(
            "unique_timed_batches_checked_per_round".into(),
            unique_batches_per_round.into(),
        );
        row.insert("pre_timing_fresh_instances".into(), true.into());
        row.insert("post_timing_fresh_instances".into(), true.into());
        row.insert("probe_checksum_false".into(), probe_checksum_false.into());
        emit(Value::Object(row));
    }
    let exact_labels: HashSet<_> = exact.iter().map(|backend| backend.label()).collect();
    for measurement in measurements {
        let implementation = measurement
            .get("implementation")
            .and_then(Value::as_str)
            .context("measurement missing implementation")?;
        if exact_labels.contains(implementation) {
            emit(measurement);
        }
    }
    Ok(())
}
