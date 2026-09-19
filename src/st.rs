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
    models::{bpe::NativeBpeTables, unigram::UnigramSnapshot},
};

const MAGIC: &[u8; 8] = b"SNAPST\0\0";
const HEADER_LEN: usize = 52;
const MAX_FILE_BYTES: usize = 512 * 1024 * 1024;
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

const BPE_VERSION: u32 = 3;
const UNIGRAM_VERSION: u32 = 4;

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
}

impl TokenizerParts {
    fn with_model(self, model: ModelConfig) -> TokenizerJson {
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

#[derive(Encode, Decode)]
struct PayloadV3 {
    pipeline_json: Vec<u8>,
    bpe: NativeBpeTables,
}

#[derive(Encode, Decode)]
struct PayloadV4 {
    pipeline_json: Vec<u8>,
    unigram: UnigramSnapshot,
}

enum Payload {
    Bpe(Box<PayloadV3>),
    Unigram(PayloadV4),
}

pub(crate) fn load_or_create<T, E>(
    path: &Path,
    mut construct: impl FnMut(TokenizerJson) -> Result<T, E>,
) -> Result<T, LoadError<E>> {
    if path.extension() == Some(OsStr::new("st")) {
        return construct(load_st(path)?).map_err(LoadError::Construct);
    }

    let sidecar = path.with_extension("st");
    let cache_error = match load_st(&sidecar)
        .map_err(LoadError::Load)
        .and_then(|config| construct(config).map_err(LoadError::Construct))
    {
        Ok(tokenizer) => return Ok(tokenizer),
        Err(error) => error,
    };
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && sidecar.is_file() => {
            return Err(cache_error);
        }
        Err(error) => return Err(Error::from(error).into()),
    };

    let (config, payload) = from_json_bytes(&source)?;
    // Publish only after the consumer accepts the complete configuration.
    let tokenizer = construct(config).map_err(LoadError::Construct)?;
    let encoded = encode_file(&payload)?;
    write_atomic(&sidecar, &encoded)?;
    Ok(tokenizer)
}

fn from_json_bytes(source: &[u8]) -> Result<(TokenizerJson, Payload), Error> {
    let mut json: Value = serde_json::from_slice(source)?;
    let object = json
        .as_object_mut()
        .ok_or_else(|| Error::St("tokenizer JSON root must be an object".into()))?;
    let model_json = object
        .remove("model")
        .ok_or_else(|| Error::St("tokenizer JSON is missing its model".into()))?;

    let pipeline_json = serde_json::to_vec(&json)?;
    let parts: TokenizerParts = serde_json::from_value(json)?;
    let model: ModelConfig = serde_json::from_value(model_json)?;
    let config = parts.with_model(model);
    let payload = match &config.model {
        ModelConfig::Bpe(bpe) => Payload::Bpe(Box::new(PayloadV3 {
            pipeline_json,
            bpe: NativeBpeTables::from_model(bpe).map_err(Error::Model)?,
        })),
        ModelConfig::Unigram(unigram) => Payload::Unigram(PayloadV4 {
            pipeline_json,
            unigram: UnigramSnapshot::from_model(unigram),
        }),
    };
    Ok((config, payload))
}

fn load_st(path: &Path) -> Result<TokenizerJson, Error> {
    if fs::metadata(path)?.len() > MAX_FILE_BYTES as u64 {
        return Err(Error::St("file exceeds the 512 MiB limit".into()));
    }
    let file = fs::read(path)?;
    decode_file(&file)?.into_config()
}

impl Payload {
    fn into_config(self) -> Result<TokenizerJson, Error> {
        match self {
            Self::Bpe(payload) => {
                let parts: TokenizerParts = serde_json::from_slice(&payload.pipeline_json)?;
                let bpe = payload.bpe.into_model().map_err(Error::Model)?;
                Ok(parts.with_model(ModelConfig::Bpe(Box::new(bpe))))
            }
            Self::Unigram(payload) => {
                let parts: TokenizerParts = serde_json::from_slice(&payload.pipeline_json)?;
                let unigram = payload.unigram.into_model().map_err(Error::Model)?;
                Ok(parts.with_model(ModelConfig::Unigram(Box::new(unigram))))
            }
        }
    }
}

fn encode_file(payload: &Payload) -> Result<Vec<u8>, Error> {
    match payload {
        Payload::Bpe(payload) => encode_payload(payload, BPE_VERSION),
        Payload::Unigram(payload) => encode_payload(payload, UNIGRAM_VERSION),
    }
}

fn decode_file(file: &[u8]) -> Result<Payload, Error> {
    let (version, payload) = decode_header(file)?;
    let decoded = match version {
        BPE_VERSION => bincode::decode_from_slice(payload, bincode_config())
            .map(|(decoded, consumed)| (Payload::Bpe(decoded), consumed)),
        _ => bincode::decode_from_slice(payload, bincode_config())
            .map(|(decoded, consumed)| (Payload::Unigram(decoded), consumed)),
    };
    let (decoded, consumed) =
        decoded.map_err(|error| Error::St(format!("failed to decode payload: {error}")))?;
    if consumed != payload.len() {
        return Err(Error::St("trailing payload bytes".into()));
    }
    Ok(decoded)
}

fn encode_payload<P: bincode::Encode>(payload: &P, version: u32) -> Result<Vec<u8>, Error> {
    let mut file = vec![0; HEADER_LEN];
    bincode::encode_into_std_write(payload, &mut file, bincode_config())
        .map_err(|error| Error::St(format!("failed to encode payload: {error}")))?;
    let payload_len = file.len() - HEADER_LEN;
    let payload_hash = blake3::hash(&file[HEADER_LEN..]);
    file[0..8].copy_from_slice(MAGIC);
    file[8..12].copy_from_slice(&version.to_le_bytes());
    file[12..20].copy_from_slice(&(payload_len as u64).to_le_bytes());
    file[20..52].copy_from_slice(payload_hash.as_bytes());
    Ok(file)
}

fn decode_header(file: &[u8]) -> Result<(u32, &[u8]), Error> {
    if file.len() < HEADER_LEN {
        return Err(Error::St("file is shorter than its header".into()));
    }
    if &file[0..8] != MAGIC {
        return Err(Error::St("unrecognized file magic".into()));
    }

    let mut version = [0; 4];
    version.copy_from_slice(&file[8..12]);
    let version = u32::from_le_bytes(version);
    if version != BPE_VERSION && version != UNIGRAM_VERSION {
        return Err(Error::St("unsupported format version".into()));
    }

    let mut encoded_len = [0; 8];
    encoded_len.copy_from_slice(&file[12..20]);
    let encoded_len = u64::from_le_bytes(encoded_len);
    if encoded_len != (file.len() - HEADER_LEN) as u64 {
        return Err(Error::St("payload length mismatch".into()));
    }
    let payload = &file[HEADER_LEN..];
    if blake3::hash(payload).as_bytes() != &file[20..52] {
        return Err(Error::St("payload checksum mismatch".into()));
    }

    Ok((version, payload))
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
        .ok_or_else(|| Error::St("sidecar path has no file name".into()))?;
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".{}.{}.tmp", std::process::id(), counter));
    Ok(path.with_file_name(temporary))
}

fn bincode_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_limit::<MAX_FILE_BYTES>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LoadMode, Tokenizer};
    use serde_json::{Value, json};
    use std::cell::Cell;

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

    #[test]
    fn round_trip_preserves_pipeline_and_direct_loads() {
        let directory = test_directory("st-round-trip");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        write_fixture(&json_path, true);

        let json = Tokenizer::load_file(&json_path, LoadMode::JsonOnly).unwrap();
        let cached = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        let direct = Tokenizer::load_file(&st_path, LoadMode::StCache).unwrap();
        assert!(st_path.is_file());

        for input in ["ab", "é", "e\u{301}!", "  <s>  "] {
            let expected = json.encode(input, false).unwrap();
            assert_eq!(cached.encode(input, false).unwrap(), expected);
            assert_eq!(direct.encode(input, false).unwrap(), expected);
        }
        assert_eq!(
            json.encode("ab", true).unwrap(),
            direct.encode("ab", true).unwrap()
        );
        assert_eq!(direct.decode(&[3, 4], false).unwrap(), "é");
        assert!(direct.is_special_token(5));

        fs::remove_file(&json_path).unwrap();
        let sidecar_only = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(sidecar_only.encode("é", false).unwrap(), vec![3, 4]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn duplicate_json_merges_round_trip() {
        let directory = test_directory("st-duplicate-merges");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        let source = serde_json::to_vec(&json!({
            "model": {
                "type": "BPE",
                "vocab": {"a": 0, "b": 1, "c": 2, "ab": 3, "bc": 4},
                "merges": [["a", "b"], ["b", "c"], ["a", "b"]]
            }
        }))
        .unwrap();
        fs::write(&json_path, &source).unwrap();
        let reference = tokenizers::Tokenizer::from_bytes(&source).unwrap();
        let json = Tokenizer::load_file(&json_path, LoadMode::JsonOnly).unwrap();
        let cached = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        fs::remove_file(&json_path).unwrap();
        let direct = Tokenizer::load_file(&st_path, LoadMode::StCache).unwrap();

        for input in ["", "ab", "bc", "abc", "abcabc"] {
            let expected = reference.encode(input, false).unwrap();
            for tokenizer in [&json, &cached, &direct] {
                assert_eq!(tokenizer.encode(input, false).unwrap(), expected.get_ids());
            }
        }
        assert_eq!(direct.encode("abc", false).unwrap(), [0, 4]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rebuilds_invalid_sidecars_and_handles_concurrent_creation() {
        let directory = test_directory("st-recovery");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        write_fixture(&json_path, true);
        Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();

        write_fixture(&json_path, false);
        let reused = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(reused.encode("ab", false).unwrap(), vec![2]);

        fs::remove_file(&json_path).unwrap();
        fs::create_dir(&json_path).unwrap();
        let unread_source = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(unread_source.encode("ab", false).unwrap(), vec![2]);
        fs::remove_dir(&json_path).unwrap();
        write_fixture(&json_path, false);

        let mut legacy = fs::read(&st_path).unwrap();
        legacy[8..12].copy_from_slice(&1u32.to_le_bytes());
        legacy.splice(20..20, [0; 32]);
        fs::write(&st_path, legacy).unwrap();
        assert!(Tokenizer::load_file(&st_path, LoadMode::StCache).is_err());
        let upgraded = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(upgraded.encode("ab", false).unwrap(), vec![0, 1]);
        assert_eq!(
            &fs::read(&st_path).unwrap()[8..12],
            &BPE_VERSION.to_le_bytes()
        );

        fs::write(&st_path, b"SNAPST\0\0").unwrap();
        let recovered = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(recovered.encode("ab", false).unwrap(), vec![0, 1]);

        fs::write(&st_path, b"SNAPST\0\0").unwrap();
        fs::remove_file(&json_path).unwrap();
        assert!(Tokenizer::load_file(&st_path, LoadMode::StCache).is_err());
        fs::remove_dir_all(&directory).unwrap();

        let directory = test_directory("st-concurrent");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        write_fixture(&json_path, true);
        let threads = (0..4)
            .map(|_| {
                let json_path = json_path.clone();
                std::thread::spawn(move || {
                    Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        let final_tokenizer = Tokenizer::load_file(&st_path, LoadMode::StCache).unwrap();
        assert_eq!(final_tokenizer.encode("ab", false).unwrap(), vec![2]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn unigram_snapshot_preserves_pipeline_scores_and_fallback() {
        let directory = test_directory("st-unigram");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        let source = serde_json::to_vec(&json!({
            "normalizer": {"type": "NFC"},
            "pre_tokenizer": {"type": "WhitespaceSplit"},
            "added_tokens": [{"id": 11, "content": "<s>", "single_word": false,
                "lstrip": true, "rstrip": true, "normalized": false, "special": true}],
            "post_processor": {"type": "TemplateProcessing",
                "single": [{"Sequence": {"id": "A", "type_id": 0}},
                    {"SpecialToken": {"id": "<s>", "type_id": 0}}],
                "pair": [], "special_tokens": {
                    "<s>": {"id": "<s>", "ids": [11], "tokens": ["<s>"]}}},
            "decoder": {"type": "Sequence", "decoders": [
                {"type": "ByteFallback"}, {"type": "Fuse"}]},
            "model": {"type": "Unigram", "unk_id": 0, "byte_fallback": true,
                "vocab": [["<unk>", 0.0], ["", -1.0], ["a", -2.0], ["b", -2.0],
                    ["ab", -1.0], ["ab", -0.5], ["é", -2.0], ["<0xF0>", -3.0],
                    ["<0x9F>", -3.0], ["<0x98>", -3.0], ["<0x80>", -3.0]]}
        }))
        .unwrap();
        fs::write(&json_path, &source).unwrap();
        let reference = tokenizers::Tokenizer::from_bytes(&source).unwrap();
        let json = Tokenizer::load_file(&json_path, LoadMode::JsonOnly).unwrap();
        let cached = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        let file = fs::read(&st_path).unwrap();
        assert_eq!(&file[8..12], &UNIGRAM_VERSION.to_le_bytes());
        let direct = Tokenizer::load_file(&st_path, LoadMode::StCache).unwrap();
        let reused = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(fs::read(&st_path).unwrap(), file);
        fs::remove_file(&json_path).unwrap();
        let sidecar_only = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        let inputs = ["", "ab a b", "e\u{301}", "😀", "unknown", "a <s> ab"];
        for special in [false, true] {
            let expected: Vec<Vec<u32>> = inputs
                .iter()
                .map(|text| reference.encode(*text, special).unwrap().get_ids().to_vec())
                .collect();
            for tokenizer in [&json, &cached, &direct, &reused, &sidecar_only] {
                for (input, ids) in inputs.iter().zip(&expected) {
                    assert_eq!(tokenizer.encode(input, special).unwrap(), *ids);
                    assert_eq!(
                        tokenizer.decode(ids, false).unwrap(),
                        reference.decode(ids, false).unwrap()
                    );
                }
                assert_eq!(tokenizer.encode_batch(&inputs, special).unwrap(), expected);
                let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, special).unwrap();
                assert_eq!(ids, expected.concat());
                assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
                for (text, id) in reference.get_vocab(false) {
                    assert_eq!(tokenizer.token_to_id(&text), Some(id));
                    assert_eq!(tokenizer.id_to_token(id), Some(text.as_str()));
                }
            }
        }
        let mut corrupt = file;
        corrupt[HEADER_LEN] ^= 1;
        fs::write(&st_path, corrupt).unwrap();
        assert!(Tokenizer::load_file(&st_path, LoadMode::StCache).is_err());
        fs::write(&json_path, source).unwrap();
        let recovered = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(recovered.encode("ab", false).unwrap(), [5]);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn snapshot_wire_bytes_are_stable() {
        let models = [
            serde_json::json!({"type": "BPE", "vocab": {"a": 0, "b": 1, "ab": 2}, "merges": [["a", "b"]]}),
            serde_json::json!({"type": "Unigram", "vocab": [["<unk>", 0.0], ["a", -1.0]], "unk_id": 0}),
        ];
        for (index, model) in models.into_iter().enumerate() {
            let source =
                serde_json::to_vec(&serde_json::json!({"model": model, "metadata": "preserved"}))
                    .unwrap();
            let (_, payload) = from_json_bytes(&source).unwrap();
            let file = encode_file(&payload).unwrap();
            let expected = [
                "a5647eb0a48296294c246a9247b1533acb5b68c972f5b9076be95fd36e0bd183",
                "4b8acd793c3fc0f2753c80bf23131dc6eb6184a149cac2ac46d23735f6c6fd61",
            ];
            assert_eq!(blake3::hash(&file).to_hex().as_str(), expected[index]);
            let config = decode_file(&file).unwrap().into_config().unwrap();
            let tokenizer = Tokenizer::from_config(config).unwrap();
            assert_eq!(tokenizer.encode("a", false).unwrap().len(), 1);
        }
    }
    #[test]
    fn malformed_pipeline_precedes_native_table_validation() {
        let source = serde_json::to_vec(&fixture(true)).unwrap();
        let (_, Payload::Bpe(mut payload)) = from_json_bytes(&source).unwrap() else {
            unreachable!()
        };
        let mut tables = bincode::encode_to_vec(&payload.bpe, bincode_config()).unwrap();
        // The arena begins after its fixed-width vector length.
        tables[8] = 0xff;
        payload.bpe = bincode::decode_from_slice(&tables, bincode_config())
            .unwrap()
            .0;
        payload.pipeline_json = b"[".to_vec();
        assert!(matches!(
            Payload::Bpe(payload).into_config(),
            Err(Error::Json(_))
        ));
    }
    #[test]
    fn header_errors_preserve_validation_order() {
        let original = encode_payload(&vec![1u8, 2, 3], BPE_VERSION).unwrap();
        let mut file = original.clone();
        file[0] ^= 1;
        file[8..12].copy_from_slice(&9u32.to_le_bytes());
        file[12] ^= 1;
        file[HEADER_LEN] ^= 1;
        for (range, expected) in [
            (0..8, "unrecognized file magic"),
            (8..12, "unsupported format version"),
            (12..20, "payload length mismatch"),
            (HEADER_LEN..original.len(), "payload checksum mismatch"),
        ] {
            let error = decode_header(&file).unwrap_err();
            assert_eq!(error.to_string(), Error::St(expected.into()).to_string());
            file[range.clone()].copy_from_slice(&original[range]);
        }
        assert_eq!(decode_header(&file).unwrap().0, BPE_VERSION);
        for version in [1u32, 2] {
            file[8..12].copy_from_slice(&version.to_le_bytes());
            assert!(matches!(decode_header(&file), Err(Error::St(message))
                if message == "unsupported format version"));
        }
    }

    #[test]
    fn consumer_rejection_preserves_cache_and_retries_only_with_source() {
        let mode = LoadMode::StCache;
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
        let sidecar = source.with_extension("st");
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

    #[test]
    fn validates_pipeline_before_publishing_or_accepting_cache() {
        let directory = test_directory("st-pipeline-validation");
        let json_path = directory.join("tokenizer.json");
        let st_path = directory.join("tokenizer.st");
        let source = serde_json::to_vec(&fixture(true)).unwrap();
        let mut invalid = fixture(true);
        invalid["normalizer"] = json!({
            "type": "Replace", "pattern": { "Regex": "[" }, "content": ""
        });
        let invalid = serde_json::to_vec(&invalid).unwrap();
        fs::write(&json_path, &invalid).unwrap();
        assert!(Tokenizer::load_file(&json_path, LoadMode::StCache).is_err());
        assert!(!st_path.exists());

        fs::write(&json_path, &source).unwrap();
        Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        let original = fs::read(&st_path).unwrap();
        let (_, payload) = from_json_bytes(&invalid).unwrap();
        let corrupted = encode_file(&payload).unwrap();
        fs::write(&st_path, corrupted).unwrap();
        let recovered = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(recovered.encode("ab", false).unwrap(), vec![2]);
        assert_eq!(fs::read(&st_path).unwrap(), original);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn native_sentencepiece_paths_fail_explicitly() {
        let error = match Tokenizer::load_file(Path::new("fixture.model"), LoadMode::JsonOnly) {
            Ok(_) => panic!("native SentencePiece paths must be rejected before reading"),
            Err(error) => error,
        };
        assert_eq!(
            error.to_string(),
            "unsupported tokenizer format: native SentencePiece .model files are not supported; export a compatible tokenizer.json"
        );
    }
}
