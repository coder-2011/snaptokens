use std::{
    ffi::{OsStr, OsString},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::Deserialize;
use serde_json::Value;

use crate::{
    AddedTokenConfig, DecoderConfig, Error, ModelConfig, NormalizerConfig, PostProcessorConfig,
    PreTokenizerConfig, TokenizerJson,
    json_structs::{LoadError, PaddingParams, TruncationParams},
};

pub(crate) const HEADER_LEN: usize = 84;
const MAX_FILE_BYTES: usize = 512 * 1024 * 1024;
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
pub(crate) enum Format {
    St,
    Tkz,
}

#[derive(Deserialize)]
pub(crate) struct TokenizerParts {
    truncation: Option<TruncationParams>,
    padding: Option<PaddingParams>,
    #[serde(default)]
    added_tokens: Vec<AddedTokenConfig>,
    normalizer: Option<NormalizerConfig>,
    pre_tokenizer: Option<PreTokenizerConfig>,
    post_processor: Option<PostProcessorConfig>,
    decoder: Option<DecoderConfig>,
}

impl TokenizerParts {
    pub(crate) fn with_model(self, model: ModelConfig) -> TokenizerJson {
        TokenizerJson {
            truncation: self.truncation,
            padding: self.padding,
            added_tokens: self.added_tokens,
            normalizer: self.normalizer,
            pre_tokenizer: self.pre_tokenizer,
            model,
            post_processor: self.post_processor,
            decoder: self.decoder,
        }
    }
}

impl Format {
    fn extension(self) -> &'static str {
        match self {
            Self::St => "st",
            Self::Tkz => "tkz",
        }
    }

    fn magic(self) -> &'static [u8; 8] {
        match self {
            Self::St => b"SNAPST\0\0",
            Self::Tkz => b"SNAPTKZ\0",
        }
    }

    fn error(self, message: String) -> Error {
        match self {
            Self::St => Error::St(message),
            Self::Tkz => Error::Tkz(message),
        }
    }

    pub(crate) fn load_or_create<T, E, P>(
        self,
        path: &Path,
        mut construct: impl FnMut(TokenizerJson) -> Result<T, E>,
        load: impl Fn(&Path, Option<[u8; 32]>) -> Result<TokenizerJson, Error>,
        prepare: impl Fn(&[u8]) -> Result<(TokenizerJson, P), Error>,
        encode: impl Fn(&P, [u8; 32]) -> Result<Vec<u8>, Error>,
    ) -> Result<T, LoadError<E>> {
        if path.extension() == Some(OsStr::new(self.extension())) {
            return construct(load(path, None)?).map_err(LoadError::Construct);
        }

        let sidecar = path.with_extension(self.extension());
        let source = match fs::read(path) {
            Ok(source) => source,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && sidecar.is_file() => {
                return construct(load(&sidecar, None)?).map_err(LoadError::Construct);
            }
            Err(error) => return Err(Error::from(error).into()),
        };
        let source_hash = *blake3::hash(&source).as_bytes();

        if sidecar.is_file()
            && let Ok(config) = load(&sidecar, Some(source_hash))
            && let Ok(tokenizer) = construct(config)
        {
            return Ok(tokenizer);
        }

        let (config, payload) = prepare(&source)?;
        // Publish only after the consumer accepts the complete configuration.
        let tokenizer = construct(config).map_err(LoadError::Construct)?;
        let encoded = encode(&payload, source_hash)?;
        self.write_atomic(&sidecar, &encoded)?;
        Ok(tokenizer)
    }

    pub(crate) fn parse_json(self, source: &[u8]) -> Result<(TokenizerJson, Vec<u8>), Error> {
        let mut json: Value = serde_json::from_slice(source)?;
        let object = json
            .as_object_mut()
            .ok_or_else(|| self.error("tokenizer JSON root must be an object".into()))?;
        let model_json = object
            .remove("model")
            .ok_or_else(|| self.error("tokenizer JSON is missing its model".into()))?;

        let pipeline_json = serde_json::to_vec(&json)?;
        let parts: TokenizerParts = serde_json::from_value(json)?;
        let model: ModelConfig = serde_json::from_value(model_json)?;
        Ok((parts.with_model(model), pipeline_json))
    }

    pub(crate) fn read(self, path: &Path) -> Result<Vec<u8>, Error> {
        if fs::metadata(path)?.len() > MAX_FILE_BYTES as u64 {
            return Err(self.error("file exceeds the 512 MiB limit".into()));
        }
        Ok(fs::read(path)?)
    }

    pub(crate) fn encode<P: bincode::Encode>(
        self,
        payload: &P,
        version: u32,
        source_hash: [u8; 32],
    ) -> Result<Vec<u8>, Error> {
        let mut file = vec![0; HEADER_LEN];
        bincode::encode_into_std_write(payload, &mut file, bincode_config())
            .map_err(|error| self.error(format!("failed to encode payload: {error}")))?;
        let payload_len = file.len() - HEADER_LEN;
        let payload_hash = blake3::hash(&file[HEADER_LEN..]);
        file[0..8].copy_from_slice(self.magic());
        file[8..12].copy_from_slice(&version.to_le_bytes());
        file[12..20].copy_from_slice(&(payload_len as u64).to_le_bytes());
        file[20..52].copy_from_slice(&source_hash);
        file[52..84].copy_from_slice(payload_hash.as_bytes());
        Ok(file)
    }

    pub(crate) fn payload<'a>(
        self,
        file: &'a [u8],
        source_hash: Option<[u8; 32]>,
        versions: &[u32],
    ) -> Result<(u32, &'a [u8]), Error> {
        if file.len() < HEADER_LEN {
            return Err(self.error("file is shorter than its header".into()));
        }
        if &file[0..8] != self.magic() {
            return Err(self.error("unrecognized file magic".into()));
        }

        let mut version = [0; 4];
        version.copy_from_slice(&file[8..12]);
        let version = u32::from_le_bytes(version);
        if !versions.contains(&version) {
            return Err(self.error("unsupported format version".into()));
        }

        let mut encoded_len = [0; 8];
        encoded_len.copy_from_slice(&file[12..20]);
        let encoded_len = u64::from_le_bytes(encoded_len);
        if encoded_len != (file.len() - HEADER_LEN) as u64 {
            return Err(self.error("payload length mismatch".into()));
        }
        if source_hash.is_some_and(|hash| file[20..52] != hash) {
            return Err(self.error("sidecar does not match its source JSON".into()));
        }

        let payload = &file[HEADER_LEN..];
        if blake3::hash(payload).as_bytes() != &file[52..84] {
            return Err(self.error("payload checksum mismatch".into()));
        }

        Ok((version, payload))
    }

    fn write_atomic(self, path: &Path, bytes: &[u8]) -> Result<(), Error> {
        let temporary = self.temporary_path(path)?;
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

    fn temporary_path(self, path: &Path) -> Result<PathBuf, Error> {
        let name = path
            .file_name()
            .ok_or_else(|| self.error("sidecar path has no file name".into()))?;
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut temporary = OsString::from(".");
        temporary.push(name);
        temporary.push(format!(".{}.{}.tmp", std::process::id(), counter));
        Ok(path.with_file_name(temporary))
    }
}

pub(crate) fn bincode_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_limit::<MAX_FILE_BYTES>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LoadMode, Tokenizer};
    use std::cell::Cell;

    #[test]
    fn header_errors_preserve_validation_order() {
        for format in [Format::St, Format::Tkz] {
            let original = format.encode(&vec![1u8, 2, 3], 1, [7; 32]).unwrap();
            let mut file = original.clone();
            file[0] ^= 1;
            file[8..12].copy_from_slice(&9u32.to_le_bytes());
            file[12] ^= 1;
            file[20] ^= 1;
            file[HEADER_LEN] ^= 1;
            for (range, expected) in [
                (0..8, "unrecognized file magic"),
                (8..12, "unsupported format version"),
                (12..20, "payload length mismatch"),
                (20..52, "sidecar does not match its source JSON"),
                (HEADER_LEN..original.len(), "payload checksum mismatch"),
            ] {
                let error = format.payload(&file, Some([7; 32]), &[1]).unwrap_err();
                assert_eq!(error.to_string(), format.error(expected.into()).to_string());
                file[range.clone()].copy_from_slice(&original[range]);
            }
            assert_eq!(format.payload(&file, Some([7; 32]), &[1]).unwrap().0, 1);
        }
    }

    #[test]
    fn consumer_rejection_preserves_cache_and_retries_only_with_source() {
        for (format, mode) in [
            (Format::St, LoadMode::StCache),
            (Format::Tkz, LoadMode::TkzCache),
        ] {
            let directory = std::env::temp_dir().join(format!(
                "snaptokens-consumer-{}-{}",
                std::process::id(),
                TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&directory).unwrap();
            let source = directory.join("tokenizer.json");
            fs::write(
                &source,
                br#"{"model":{"type":"BPE","vocab":{"a":0},"merges":[]}}"#,
            )
            .unwrap();
            Tokenizer::load_file(&source, mode).unwrap();
            let sidecar = source.with_extension(format.extension());
            let original = fs::read(&sidecar).unwrap();
            let calls = Cell::new(0);
            let result = TokenizerJson::load_file_with(&source, mode, |_| {
                calls.set(calls.get() + 1);
                if calls.get() == 1 {
                    Err("cached consumer rejected")
                } else {
                    Ok(())
                }
            });
            assert!(result.is_ok());
            assert_eq!(calls.get(), 2);
            assert_eq!(fs::read(&sidecar).unwrap(), original);

            for missing_source in [false, true] {
                if missing_source {
                    fs::remove_file(&source).unwrap();
                }
                calls.set(0);
                let result: Result<(), _> = TokenizerJson::load_file_with(&source, mode, |_| {
                    calls.set(calls.get() + 1);
                    Err("consumer rejected")
                });
                assert!(matches!(
                    result,
                    Err(LoadError::Construct("consumer rejected"))
                ));
                assert_eq!(calls.get(), if missing_source { 1 } else { 2 });
                assert_eq!(fs::read(&sidecar).unwrap(), original);
            }
            assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
            fs::remove_dir_all(directory).unwrap();
        }
    }
}
