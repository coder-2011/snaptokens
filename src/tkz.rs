use std::{
    ffi::{OsStr, OsString},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use bincode::{Decode, Encode};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    AddedTokenConfig, DecoderConfig, Error, ModelConfig, NormalizerConfig, PostProcessorConfig,
    PreTokenizerConfig, TokenizerJson,
    json_structs::{LoadError, PaddingParams, TruncationParams},
    models::bpe::{Bpe, ExactTokenTrie, ResolvedBpe},
};

const MAGIC: &[u8; 8] = b"SNAPTKZ\0";
const VERSION: u32 = 5;
const LEGACY_VERSION: u32 = 4;
const HEADER_LEN: usize = 84;
const MAX_TKZ_BYTES: usize = 512 * 1024 * 1024;

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize)]
struct TokenizerParts {
    truncation: Option<TruncationParams>,
    padding: Option<PaddingParams>,
    #[serde(default)]
    added_tokens: Vec<AddedTokenConfig>,
    normalizer: Option<NormalizerConfig>,
    pre_tokenizer: Option<PreTokenizerConfig>,
    post_processor: Option<PostProcessorConfig>,
    decoder: Option<DecoderConfig>,
    #[serde(flatten)]
    extra: serde_json::Map<String, Value>,
    #[serde(default, rename = "model")]
    model_metadata: serde_json::Map<String, Value>,
}

impl TokenizerParts {
    fn with_model(self, mut model: ModelConfig) -> TokenizerJson {
        let ModelConfig::Bpe(bpe) = &mut model;
        bpe.json_metadata = self.model_metadata;
        TokenizerJson {
            truncation: self.truncation,
            padding: self.padding,
            added_tokens: self.added_tokens,
            normalizer: self.normalizer,
            pre_tokenizer: self.pre_tokenizer,
            model,
            post_processor: self.post_processor,
            decoder: self.decoder,
            extra: self.extra,
        }
    }
}

#[derive(Encode, Decode)]
struct PayloadV4 {
    pipeline_json: Vec<u8>,
    bpe: ResolvedBpe,
}

#[derive(Encode, Decode)]
struct PayloadV5 {
    pipeline_json: Vec<u8>,
    bpe: ResolvedBpe,
    exact_token_trie: ExactTokenTrie,
}

enum DecodedPayload {
    V4(PayloadV4),
    V5(PayloadV5),
}

impl DecodedPayload {
    /// V4 remains readable directly, but JSON-adjacent sidecars are renewed as V5.
    fn is_current(&self) -> bool {
        matches!(self, Self::V5(_))
    }

    fn into_config(self) -> Result<TokenizerJson, Error> {
        let (pipeline_json, bpe) = match self {
            Self::V4(PayloadV4 { pipeline_json, bpe }) => (
                pipeline_json,
                Bpe::from_resolved(bpe).map_err(Error::Model)?,
            ),
            Self::V5(PayloadV5 {
                pipeline_json,
                bpe,
                exact_token_trie,
            }) => (
                pipeline_json,
                Bpe::from_resolved_with_exact_token_trie(bpe, Some(exact_token_trie))
                    .map_err(Error::Model)?,
            ),
        };
        let parts: TokenizerParts = serde_json::from_slice(&pipeline_json)?;
        let json = parts.with_model(ModelConfig::Bpe(Box::new(bpe)));
        Ok(json)
    }
}

pub(crate) fn load_or_create<T, E>(
    path: &Path,
    mut construct: impl FnMut(TokenizerJson) -> Result<T, E>,
) -> Result<T, LoadError<E>> {
    if path.extension() == Some(OsStr::new("tkz")) {
        return construct(load_tkz(path, None, false)?).map_err(LoadError::Construct);
    }

    let sidecar = path.with_extension("tkz");
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && sidecar.is_file() => {
            return construct(load_tkz(&sidecar, None, false)?).map_err(LoadError::Construct);
        }
        Err(error) => return Err(Error::from(error).into()),
    };
    let source_hash = *blake3::hash(&source).as_bytes();

    if sidecar.is_file()
        && let Ok(config) = load_tkz(&sidecar, Some(source_hash), true)
        && let Ok(tokenizer) = construct(config)
    {
        return Ok(tokenizer);
    }

    let (config, payload) = from_json_bytes(&source)?;
    // A failed construction must leave any existing sidecar intact.
    let tokenizer = construct(config).map_err(LoadError::Construct)?;
    let encoded = encode_file(&payload, source_hash)?;
    write_atomic(&sidecar, &encoded)?;
    Ok(tokenizer)
}

/// Publish an explicit snapshot and refresh an existing JSON sidecar.
pub(crate) fn save(
    path: &Path,
    config: &TokenizerJson<&crate::Model>,
    pretty: bool,
) -> Result<(), Error> {
    let mut document = serde_json::to_value(config)?;
    let source = if pretty {
        serde_json::to_vec_pretty(&document)?
    } else {
        serde_json::to_vec(&document)?
    };
    let binary = path.extension() == Some(OsStr::new("tkz"));
    let sidecar = path.with_extension("tkz");
    let encoded = if binary || sidecar.is_file() {
        let object = document
            .as_object_mut()
            .expect("tokenizer configuration is an object");
        let metadata = model_metadata(&object["model"]);
        object.insert("model".into(), Value::Object(metadata));
        let crate::Model::Bpe(model) = config.model;
        let bpe = model.resolved_config();
        let exact_token_trie = bpe.exact_token_trie().map_err(Error::Model)?;
        let payload = PayloadV5 {
            pipeline_json: serde_json::to_vec(&document)?,
            bpe,
            exact_token_trie,
        };
        Some(encode_file(&payload, *blake3::hash(&source).as_bytes())?)
    } else {
        None
    };
    if binary {
        return write_atomic(path, encoded.as_ref().expect("binary snapshot was encoded"));
    }
    // JSON is authoritative. If cache publication fails or another writer races,
    // the source hash prevents a subsequent cached JSON load from using stale data.
    write_atomic(path, &source)?;
    if let Some(encoded) = encoded {
        write_atomic(&sidecar, &encoded)?;
    }
    Ok(())
}

/// Keep non-runtime model fields in the pipeline section of binary snapshots.
fn model_metadata(model: &Value) -> serde_json::Map<String, Value> {
    model
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(key, _)| {
            !matches!(
                key.as_str(),
                "type" | "vocab" | "merges" | "byte_fallback" | "ignore_merges"
            )
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn from_json_bytes(source: &[u8]) -> Result<(TokenizerJson, PayloadV5), Error> {
    let mut json: Value = serde_json::from_slice(source)?;
    let object = json
        .as_object_mut()
        .ok_or_else(|| Error::Tkz("tokenizer JSON root must be an object".into()))?;
    let model_json = object
        .remove("model")
        .ok_or_else(|| Error::Tkz("tokenizer JSON is missing its model".into()))?;

    object.insert("model".into(), Value::Object(model_metadata(&model_json)));
    let pipeline_json = serde_json::to_vec(&json)?;
    let parts: TokenizerParts = serde_json::from_value(json)?;
    let model: ModelConfig = serde_json::from_value(model_json)?;
    let ModelConfig::Bpe(bpe) = model;
    let resolved = bpe.resolved_config();
    let exact_token_trie = resolved.exact_token_trie().map_err(Error::Model)?;

    let config = parts.with_model(ModelConfig::Bpe(bpe));
    let payload = PayloadV5 {
        pipeline_json,
        bpe: resolved,
        exact_token_trie,
    };
    Ok((config, payload))
}

fn load_tkz(
    path: &Path,
    source_hash: Option<[u8; 32]>,
    require_current: bool,
) -> Result<TokenizerJson, Error> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > MAX_TKZ_BYTES as u64 {
        return Err(Error::Tkz("file exceeds the 512 MiB limit".into()));
    }
    let file = fs::read(path)?;
    let payload = decode_file(&file, source_hash)?;
    if require_current && !payload.is_current() {
        return Err(Error::Tkz("sidecar format needs regeneration".into()));
    }
    payload.into_config()
}

fn encode_file(payload: &PayloadV5, source_hash: [u8; 32]) -> Result<Vec<u8>, Error> {
    let mut file = vec![0; HEADER_LEN];
    bincode::encode_into_std_write(payload, &mut file, bincode_config())
        .map_err(|error| Error::Tkz(format!("failed to encode payload: {error}")))?;

    let payload_len = file.len() - HEADER_LEN;
    let payload_hash = blake3::hash(&file[HEADER_LEN..]);
    file[0..8].copy_from_slice(MAGIC);
    file[8..12].copy_from_slice(&VERSION.to_le_bytes());
    file[12..20].copy_from_slice(&(payload_len as u64).to_le_bytes());
    file[20..52].copy_from_slice(&source_hash);
    file[52..84].copy_from_slice(payload_hash.as_bytes());
    Ok(file)
}

fn decode_file(file: &[u8], source_hash: Option<[u8; 32]>) -> Result<DecodedPayload, Error> {
    if file.len() < HEADER_LEN {
        return Err(Error::Tkz("file is shorter than its header".into()));
    }
    if &file[0..8] != MAGIC {
        return Err(Error::Tkz("unrecognized file magic".into()));
    }

    let mut version = [0; 4];
    version.copy_from_slice(&file[8..12]);
    let version = u32::from_le_bytes(version);
    if version != VERSION && version != LEGACY_VERSION {
        return Err(Error::Tkz("unsupported format version".into()));
    }

    let mut encoded_len = [0; 8];
    encoded_len.copy_from_slice(&file[12..20]);
    let encoded_len = u64::from_le_bytes(encoded_len);
    if encoded_len != (file.len() - HEADER_LEN) as u64 {
        return Err(Error::Tkz("payload length mismatch".into()));
    }
    if source_hash.is_some_and(|hash| file[20..52] != hash) {
        return Err(Error::Tkz("sidecar does not match its source JSON".into()));
    }

    let payload = &file[HEADER_LEN..];
    if blake3::hash(payload).as_bytes() != &file[52..84] {
        return Err(Error::Tkz("payload checksum mismatch".into()));
    }

    let (decoded, consumed) = match version {
        LEGACY_VERSION => {
            let (decoded, consumed): (PayloadV4, usize) =
                bincode::decode_from_slice(payload, bincode_config()).map_err(|error| {
                    Error::Tkz(format!("failed to decode version-4 payload: {error}"))
                })?;
            (DecodedPayload::V4(decoded), consumed)
        }
        VERSION => {
            let (decoded, consumed): (PayloadV5, usize) =
                bincode::decode_from_slice(payload, bincode_config()).map_err(|error| {
                    Error::Tkz(format!("failed to decode version-5 payload: {error}"))
                })?;
            (DecodedPayload::V5(decoded), consumed)
        }
        _ => unreachable!("version was checked above"),
    };
    if consumed != payload.len() {
        return Err(Error::Tkz("trailing payload bytes".into()));
    }
    Ok(decoded)
}

fn bincode_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_limit::<MAX_TKZ_BYTES>()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let temporary = temporary_path(path)?;
    let result = (|| -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(Error::Io)
}

fn temporary_path(path: &Path) -> Result<PathBuf, Error> {
    let name = path
        .file_name()
        .ok_or_else(|| Error::Tkz("sidecar path has no file name".into()))?;
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".{}.{}.tmp", std::process::id(), counter));
    Ok(path.with_file_name(temporary))
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;
    use serde_json::{Value, json};

    use crate::LoadMode;

    use super::*;

    fn fixture(merged: bool) -> Value {
        let merges = if merged {
            json!([["a", "b"]])
        } else {
            json!([])
        };
        json!({
            "added_tokens": [
                {
                    "id": 5,
                    "content": "<s>",
                    "single_word": true,
                    "lstrip": true,
                    "rstrip": true,
                    "normalized": false,
                    "special": true
                },
                {
                    "id": 6,
                    "content": "e\u{301}!",
                    "single_word": false,
                    "lstrip": false,
                    "rstrip": false,
                    "normalized": true,
                    "special": false
                }
            ],
            "normalizer": {"type": "NFC"},
            "pre_tokenizer": null,
            "model": {
                "type": "BPE",
                "vocab": {
                    "a": 0,
                    "b": 1,
                    "ab": 2,
                    "<0xC3>": 3,
                    "<0xA9>": 4
                },
                "merges": merges,
                "byte_fallback": true,
                "ignore_merges": merged
            },
            "post_processor": {
                "type": "TemplateProcessing",
                "single": [
                    {"SpecialToken": {"id": "<s>", "type_id": 0}},
                    {"Sequence": {"id": "A", "type_id": 0}}
                ],
                "pair": [],
                "special_tokens": {
                    "<s>": {"id": "<s>", "ids": [5], "tokens": ["<s>"]}
                }
            },
            "decoder": {
                "type": "Sequence",
                "decoders": [{"type": "ByteFallback"}, {"type": "Fuse"}]
            }
        })
    }

    fn test_directory(name: &str) -> PathBuf {
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "snaptokens-{name}-{}-{counter}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    fn write_fixture(path: &Path, merged: bool) {
        fs::write(path, serde_json::to_vec(&fixture(merged)).unwrap()).unwrap();
    }

    // Recreates the valid envelope the fuzz target adds around corpus payload bytes.
    fn fuzz_tkz_file(payload: &[u8]) -> Vec<u8> {
        let mut file = Vec::with_capacity(84 + payload.len());
        file.extend_from_slice(MAGIC);
        file.extend_from_slice(&VERSION.to_le_bytes());
        file.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        file.extend_from_slice(&[0; 32]);
        file.extend_from_slice(blake3::hash(payload).as_bytes());
        file.extend_from_slice(payload);
        file
    }

    #[test]
    fn fuzz_tkz_seed_reconstructs_a_tokenizer() {
        let payload = include_bytes!("../fuzz/corpus/fuzz_tkz/v5-minimal-payload");
        let tokenizer = decode_file(&fuzz_tkz_file(payload), None)
            .unwrap()
            .into_config()
            .and_then(Tokenizer::from_config)
            .unwrap();
        assert_eq!(tokenizer.encode("a", false).unwrap(), vec![0]);
    }

    #[test]
    fn round_trip_preserves_pipeline_and_direct_loads() {
        let directory = test_directory("tkz-round-trip");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);

        let json = Tokenizer::load_file(&json_path, LoadMode::JsonOnly).unwrap();
        let disabled = Tokenizer::load_file(&json_path, LoadMode::JsonOnly).unwrap();
        assert!(!tkz_path.exists());
        assert_eq!(
            json.encode("ab", false).unwrap(),
            disabled.encode("ab", false).unwrap()
        );

        let cached = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        let direct = Tokenizer::load_file(&tkz_path, LoadMode::TkzCache).unwrap();
        assert!(tkz_path.is_file());

        for input in ["ab", "é", "e\u{301}!", "  <s>  "] {
            let expected = json.encode(input, false).unwrap();
            assert_eq!(cached.encode(input, false).unwrap(), expected);
            assert_eq!(direct.encode(input, false).unwrap(), expected);
        }
        let expected = json.encode("ab", true).unwrap();
        assert_eq!(cached.encode("ab", true).unwrap(), expected);
        assert_eq!(direct.decode(&[3, 4], false).unwrap(), "é");
        assert!(direct.is_special_token(5));

        fs::remove_file(&json_path).unwrap();
        let sidecar_only = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        assert_eq!(sidecar_only.encode("é", false).unwrap(), vec![3, 4]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn validates_pipeline_before_publishing_or_accepting_cache() {
        let directory = test_directory("tkz-pipeline-validation");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        let source = serde_json::to_vec(&fixture(true)).unwrap();
        fs::write(&json_path, &source).unwrap();
        Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        let original = fs::read(&tkz_path).unwrap();

        let mut invalid = fixture(true);
        invalid["normalizer"] = json!({
            "type": "Replace", "pattern": { "Regex": "[" }, "content": ""
        });
        let invalid = serde_json::to_vec(&invalid).unwrap();
        fs::write(&json_path, &invalid).unwrap();
        assert!(Tokenizer::load_file(&json_path, LoadMode::TkzCache).is_err());
        assert_eq!(fs::read(&tkz_path).unwrap(), original);

        // A checksummed cache may still contain a pipeline that cannot be compiled.
        fs::write(&json_path, &source).unwrap();
        let (_, payload) = from_json_bytes(&invalid).unwrap();
        let corrupted = encode_file(&payload, *blake3::hash(&source).as_bytes()).unwrap();
        fs::write(&tkz_path, corrupted).unwrap();
        let recovered = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        assert_eq!(recovered.encode("ab", false).unwrap(), vec![2]);
        assert_eq!(fs::read(&tkz_path).unwrap(), original);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rebuilds_invalid_sidecars_and_handles_concurrent_creation() {
        let directory = test_directory("tkz-recovery");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);
        Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();

        write_fixture(&json_path, false);
        let refreshed = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        assert_eq!(refreshed.encode("ab", false).unwrap(), vec![0, 1]);

        fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
        let recovered = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        assert_eq!(recovered.encode("ab", false).unwrap(), vec![0, 1]);

        fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
        fs::remove_file(&json_path).unwrap();
        assert!(Tokenizer::load_file(&tkz_path, LoadMode::TkzCache).is_err());
        fs::remove_dir_all(&directory).unwrap();

        let directory = test_directory("tkz-concurrent");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);
        let threads = (0..4)
            .map(|_| {
                let json_path = json_path.clone();
                std::thread::spawn(move || {
                    Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        let final_tokenizer = Tokenizer::load_file(&tkz_path, LoadMode::TkzCache).unwrap();
        assert_eq!(final_tokenizer.encode("ab", false).unwrap(), vec![2]);
        fs::remove_dir_all(directory).unwrap();
    }
}
