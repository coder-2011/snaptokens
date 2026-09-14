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
    PreTokenizerConfig, Tokenizer, TokenizerJson,
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
    #[serde(default)]
    added_tokens: Vec<AddedTokenConfig>,
    normalizer: Option<NormalizerConfig>,
    pre_tokenizer: Option<PreTokenizerConfig>,
    post_processor: Option<PostProcessorConfig>,
    decoder: Option<DecoderConfig>,
}

impl TokenizerParts {
    fn with_model(self, model: ModelConfig) -> TokenizerJson {
        TokenizerJson {
            added_tokens: self.added_tokens,
            normalizer: self.normalizer,
            pre_tokenizer: self.pre_tokenizer,
            model,
            post_processor: self.post_processor,
            decoder: self.decoder,
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

    fn into_tokenizer(self) -> Result<Tokenizer, Error> {
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
        Tokenizer::build(json)
    }
}

pub(crate) fn load_or_create(path: &Path) -> Result<Tokenizer, Error> {
    if path.extension() == Some(OsStr::new("tkz")) {
        return load_tkz(path, None, false);
    }

    let sidecar = path.with_extension("tkz");
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && sidecar.is_file() => {
            return load_tkz(&sidecar, None, false);
        }
        Err(error) => return Err(error.into()),
    };
    let source_hash = *blake3::hash(&source).as_bytes();

    if sidecar.is_file()
        && let Ok(tokenizer) = load_tkz(&sidecar, Some(source_hash), true)
    {
        return Ok(tokenizer);
    }

    let (tokenizer, payload) = from_json_bytes(&source)?;
    let encoded = encode_file(&payload, source_hash)?;
    write_atomic(&sidecar, &encoded)?;
    Ok(tokenizer)
}

fn from_json_bytes(source: &[u8]) -> Result<(Tokenizer, PayloadV5), Error> {
    let mut json: Value = serde_json::from_slice(source)?;
    let object = json
        .as_object_mut()
        .ok_or_else(|| Error::Tkz("tokenizer JSON root must be an object".into()))?;
    let model_json = object
        .remove("model")
        .ok_or_else(|| Error::Tkz("tokenizer JSON is missing its model".into()))?;

    let pipeline_json = serde_json::to_vec(&json)?;
    let parts: TokenizerParts = serde_json::from_value(json)?;
    let model: ModelConfig = serde_json::from_value(model_json)?;
    let ModelConfig::Bpe(bpe) = model else {
        return Err(Error::Tkz(
            "Unigram tokenizers cannot use .tkz caching yet; use Tokenizer::load_file".into(),
        ));
    };
    let resolved = bpe.resolved_config();
    let exact_token_trie = resolved.exact_token_trie().map_err(Error::Model)?;

    let tokenizer = Tokenizer::build(parts.with_model(ModelConfig::Bpe(bpe)))?;
    let payload = PayloadV5 {
        pipeline_json,
        bpe: resolved,
        exact_token_trie,
    };
    Ok((tokenizer, payload))
}

fn load_tkz(
    path: &Path,
    source_hash: Option<[u8; 32]>,
    require_current: bool,
) -> Result<Tokenizer, Error> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > MAX_TKZ_BYTES as u64 {
        return Err(Error::Tkz("file exceeds the 512 MiB limit".into()));
    }
    let file = fs::read(path)?;
    let payload = decode_file(&file, source_hash)?;
    if require_current && !payload.is_current() {
        return Err(Error::Tkz("sidecar format needs regeneration".into()));
    }
    payload.into_tokenizer()
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
mod tests;
