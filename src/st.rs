use std::path::Path;

use bincode::{Decode, Encode};

use crate::{
    Error, ModelConfig, TokenizerJson,
    cache::{Format, TokenizerParts, bincode_config},
    json_structs::LoadError,
    models::{bpe::NativeBpeTables, unigram::UnigramSnapshot},
};

const BPE_VERSION: u32 = 1;
const UNIGRAM_VERSION: u32 = 2;

#[derive(Encode, Decode)]
struct PayloadV1 {
    pipeline_json: Vec<u8>,
    bpe: NativeBpeTables,
}

#[derive(Encode, Decode)]
struct PayloadV2 {
    pipeline_json: Vec<u8>,
    unigram: UnigramSnapshot,
}

enum Payload {
    Bpe(Box<PayloadV1>),
    Unigram(PayloadV2),
}

pub(crate) fn load_or_create<T, E>(
    path: &Path,
    construct: impl FnMut(TokenizerJson) -> Result<T, E>,
) -> Result<T, LoadError<E>> {
    Format::St.load_or_create(path, construct, load_st, from_json_bytes, encode_file)
}

fn from_json_bytes(source: &[u8]) -> Result<(TokenizerJson, Payload), Error> {
    let (config, pipeline_json) = Format::St.parse_json(source)?;
    let payload = match &config.model {
        ModelConfig::Bpe(bpe) => Payload::Bpe(Box::new(PayloadV1 {
            pipeline_json,
            bpe: NativeBpeTables::from_model(bpe).map_err(Error::Model)?,
        })),
        ModelConfig::Unigram(unigram) => Payload::Unigram(PayloadV2 {
            pipeline_json,
            unigram: UnigramSnapshot::from_model(unigram),
        }),
    };
    Ok((config, payload))
}

fn load_st(path: &Path, source_hash: Option<[u8; 32]>) -> Result<TokenizerJson, Error> {
    let file = Format::St.read(path)?;
    decode_file(&file, source_hash)?.into_config()
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

fn encode_file(payload: &Payload, source_hash: [u8; 32]) -> Result<Vec<u8>, Error> {
    match payload {
        Payload::Bpe(payload) => Format::St.encode(payload, BPE_VERSION, source_hash),
        Payload::Unigram(payload) => Format::St.encode(payload, UNIGRAM_VERSION, source_hash),
    }
}

fn decode_file(file: &[u8], source_hash: Option<[u8; 32]>) -> Result<Payload, Error> {
    let (version, payload) =
        Format::St.payload(file, source_hash, &[BPE_VERSION, UNIGRAM_VERSION])?;
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);
    use super::*;
    use crate::cache::HEADER_LEN;
    use crate::{LoadMode, Tokenizer};
    use serde_json::{Value, json};
    use std::path::Path;

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
        let refreshed = Tokenizer::load_file(&json_path, LoadMode::StCache).unwrap();
        assert_eq!(refreshed.encode("ab", false).unwrap(), vec![0, 1]);

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
            let file = encode_file(&payload, *blake3::hash(&source).as_bytes()).unwrap();
            let expected = [
                "0fc4ffa7f90919c2f060b223fa9b9e67ee4e1d5c3e4bd19fa65c221c3fe59e8c",
                "f27d5d275b5249092ff5640a6e5141cf08e8f571be817c01452e6903be8663ab",
            ];
            assert_eq!(blake3::hash(&file).to_hex().as_str(), expected[index]);
            let config = decode_file(&file, None).unwrap().into_config().unwrap();
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
}
