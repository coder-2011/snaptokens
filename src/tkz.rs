use std::path::Path;

use bincode::{Decode, Encode};

use crate::{
    Error, ModelConfig, TokenizerJson,
    cache::{Format, TokenizerParts, bincode_config},
    json_structs::LoadError,
    models::bpe::{Bpe, ExactTokenTrie, ResolvedBpe},
};

const VERSION: u32 = 5;
const LEGACY_VERSION: u32 = 4;

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
    construct: impl FnMut(TokenizerJson) -> Result<T, E>,
) -> Result<T, LoadError<E>> {
    Format::Tkz.load_or_create(path, construct, load_tkz, from_json_bytes, encode_file)
}

fn from_json_bytes(source: &[u8]) -> Result<(TokenizerJson, PayloadV5), Error> {
    let (config, pipeline_json) = Format::Tkz.parse_json(source)?;
    let ModelConfig::Bpe(bpe) = &config.model else {
        return Err(Error::Tkz(
            "Unigram tokenizers cannot use .tkz caching yet; use Tokenizer::load_file".into(),
        ));
    };
    let resolved = bpe.resolved_config();
    let exact_token_trie = resolved.exact_token_trie().map_err(Error::Model)?;

    let payload = PayloadV5 {
        pipeline_json,
        bpe: resolved,
        exact_token_trie,
    };
    Ok((config, payload))
}

fn load_tkz(path: &Path, source_hash: Option<[u8; 32]>) -> Result<TokenizerJson, Error> {
    let file = Format::Tkz.read(path)?;
    let payload = decode_file(&file, source_hash)?;
    if source_hash.is_some() && !payload.is_current() {
        return Err(Error::Tkz("sidecar format needs regeneration".into()));
    }
    payload.into_config()
}

fn encode_file(payload: &PayloadV5, source_hash: [u8; 32]) -> Result<Vec<u8>, Error> {
    Format::Tkz.encode(payload, VERSION, source_hash)
}

fn decode_file(file: &[u8], source_hash: Option<[u8; 32]>) -> Result<DecodedPayload, Error> {
    let (version, payload) = Format::Tkz.payload(file, source_hash, &[VERSION, LEGACY_VERSION])?;
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);
    const MAGIC: &[u8; 8] = b"SNAPTKZ\0";
    use crate::Tokenizer;
    use serde_json::{Value, json};

    use crate::LoadMode;

    use super::*;
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
    fn duplicate_json_merges_round_trip() {
        let directory = test_directory("tkz-duplicate-merges");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
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
        let cached = Tokenizer::load_file(&json_path, LoadMode::TkzCache).unwrap();
        fs::remove_file(&json_path).unwrap();
        let direct = Tokenizer::load_file(&tkz_path, LoadMode::TkzCache).unwrap();

        for input in ["", "ab", "bc", "abc", "abcabc"] {
            let expected = reference.encode(input, false).unwrap();
            for tokenizer in [&json, &cached, &direct] {
                assert_eq!(tokenizer.encode(input, false).unwrap(), expected.get_ids());
            }
        }
        let expected_model = cached.model().bpe().expect("tkz round-trip is BPE-only");
        let restored_model = direct.model().bpe().expect("tkz round-trip is BPE-only");
        assert_eq!(
            bincode::encode_to_vec(restored_model.resolved_config(), bincode_config()).unwrap(),
            bincode::encode_to_vec(expected_model.resolved_config(), bincode_config()).unwrap()
        );
        assert_eq!(direct.encode("abc", false).unwrap(), [0, 4]);
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

    #[test]
    fn unigram_json_refuses_tkz_without_creating_a_sidecar() {
        let directory = test_directory("tkz-unigram");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        fs::write(
            &json_path,
            serde_json::to_vec(&json!({
                "normalizer": null,
                "pre_tokenizer": null,
                "model": {
                    "type": "Unigram",
                    "unk_id": 0,
                    "vocab": [["<unk>", 0.0], ["a", 1.0]]
                },
                "post_processor": null,
                "decoder": null
            }))
            .unwrap(),
        )
        .unwrap();

        let error = match Tokenizer::load_file(&json_path, LoadMode::TkzCache) {
            Ok(_) => panic!("Unigram JSON must not create a BPE .tkz sidecar"),
            Err(error) => error,
        };
        assert!(
            error
                .to_string()
                .contains("Unigram tokenizers cannot use .tkz caching yet")
        );
        assert!(!tkz_path.exists());
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
    #[test]
    fn snapshot_wire_bytes_are_stable() {
        let models = [
            serde_json::json!({"type": "BPE", "vocab": {"a": 0, "b": 1, "ab": 2}, "merges": [["a", "b"]]}),
        ];
        for (index, model) in models.into_iter().enumerate() {
            let source =
                serde_json::to_vec(&serde_json::json!({"model": model, "metadata": "preserved"}))
                    .unwrap();
            let (_, payload) = from_json_bytes(&source).unwrap();
            let file = encode_file(&payload, *blake3::hash(&source).as_bytes()).unwrap();
            let expected = ["f809f83ecbe3193d748776d045c23c913992dac34266c1c6dea5dfc070566e45"];
            assert_eq!(blake3::hash(&file).to_hex().as_str(), expected[index]);
            let config = decode_file(&file, None).unwrap().into_config().unwrap();
            let tokenizer = Tokenizer::from_config(config).unwrap();
            assert_eq!(tokenizer.encode("a", false).unwrap().len(), 1);
        }
    }
    #[test]
    fn legacy_snapshot_loads_directly_and_upgrades_only_with_source() {
        let directory = test_directory("tkz-legacy");
        let source_path = directory.join("tokenizer.json");
        let sidecar = directory.join("tokenizer.tkz");
        let source = serde_json::to_vec(&fixture(true)).unwrap();
        let (_, current) = from_json_bytes(&source).unwrap();
        let legacy = PayloadV4 {
            pipeline_json: current.pipeline_json,
            bpe: current.bpe,
        };
        let bytes = Format::Tkz
            .encode(&legacy, LEGACY_VERSION, *blake3::hash(&source).as_bytes())
            .unwrap();
        fs::write(&sidecar, &bytes).unwrap();
        for path in [&sidecar, &source_path] {
            let tokenizer = Tokenizer::load_file(path, LoadMode::TkzCache).unwrap();
            assert_eq!(tokenizer.encode("ab", false).unwrap(), [2]);
            assert_eq!(fs::read(&sidecar).unwrap(), bytes);
        }
        fs::write(&source_path, source).unwrap();
        let tokenizer = Tokenizer::load_file(&source_path, LoadMode::TkzCache).unwrap();
        assert_eq!(tokenizer.encode("ab", false).unwrap(), [2]);
        let upgraded = fs::read(&sidecar).unwrap();
        assert_eq!(&upgraded[8..12], &VERSION.to_le_bytes());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn malformed_model_precedes_pipeline_validation() {
        let source = serde_json::to_vec(&fixture(true)).unwrap();
        let (_, mut payload) = from_json_bytes(&source).unwrap();
        // Two empty vectors decode but do not describe a valid trie root.
        payload.exact_token_trie = bincode::decode_from_slice(&[0; 16], bincode_config())
            .unwrap()
            .0;
        payload.pipeline_json = b"[".to_vec();
        assert!(matches!(
            DecodedPayload::V5(payload).into_config(),
            Err(Error::Model(_))
        ));
    }
}
