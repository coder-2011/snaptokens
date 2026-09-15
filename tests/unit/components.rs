mod json_structs {
    use serde_json::json;

    use crate::json_structs::*;

    #[test]
    fn rejects_unimplemented_pipeline_types() {
        assert!(serde_json::from_value::<NormalizerConfig>(json!({"type": "Lowercase"})).is_err());
        assert!(
            serde_json::from_value::<PreTokenizerConfig>(json!({"type": "Whitespace"})).is_err()
        );
        assert!(serde_json::from_value::<ModelConfig>(json!({"type": "WordPiece"})).is_err());
        assert!(
            serde_json::from_value::<PostProcessorConfig>(json!({"type": "BertProcessing"}))
                .is_err()
        );
        assert!(serde_json::from_value::<DecoderConfig>(json!({"type": "WordPiece"})).is_err());
    }

    #[test]
    fn accepts_legacy_untagged_bpe_model() {
        let model = json!({"vocab": {"a": 0}, "merges": []});
        assert!(serde_json::from_value::<ModelConfig>(model).is_ok());
    }

    #[test]
    fn accepts_legacy_untagged_unigram_model() {
        let model = json!({"unk_id": 0, "vocab": [["<unk>", 0.0], ["a", 1.0]]});
        assert!(matches!(
            serde_json::from_value::<ModelConfig>(model),
            Ok(ModelConfig::Unigram(_))
        ));
    }
}

mod post_processors {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::json_structs::PostProcessorConfig;
    use crate::post_processors::*;

    #[test]
    fn template_processing_bos_only() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        };
        assert_eq!(tp.apply_single(vec![100, 200, 300]), vec![1, 100, 200, 300]);
    }

    #[test]
    fn template_processing_cls_sep() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "[CLS]".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "[SEP]".into() },
            ],
            special_tokens: HashMap::from([
                ("[CLS]".to_string(), vec![101]),
                ("[SEP]".to_string(), vec![102]),
            ]),
        };
        assert_eq!(tp.apply_single(vec![50, 60]), vec![101, 50, 60, 102]);
    }

    #[test]
    fn template_processing_empty_input() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        };
        assert_eq!(tp.apply_single(vec![]), vec![1]);
    }

    #[test]
    fn parse_from_json() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
        ]);
        let pair = serde_json::json!([]);
        let special_tokens = serde_json::json!({
            "<s>": {"id": "<s>", "ids": [1], "tokens": ["<s>"]}
        });
        let tp = TemplateProcessing::from_config(single, pair, special_tokens).unwrap();
        assert_eq!(tp.apply_single(vec![10, 20]), vec![1, 10, 20]);
    }

    #[test]
    fn post_process_single_respects_flag() {
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<s>".to_string(), vec![1])]),
        });

        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![1, 10, 20]);
        assert_eq!(pp.post_process_single(vec![10, 20], false), vec![10, 20]);
    }

    #[test]
    fn template_processing_null_special_tokens_loads_ok() {
        let single = serde_json::json!([
            {"Sequence": {"id": "A", "type_id": 0}}
        ]);
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: Value::Null,
            special_tokens: Value::Null,
        })
        .unwrap();
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![10, 20]);
    }

    #[test]
    fn template_processing_special_token_def_without_tokens_field() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<bos>", "type_id": 0}},
            {"Sequence":     {"id": "A",    "type_id": 0}},
        ]);
        let special_tokens = serde_json::json!({
            "<bos>": {"id": "<bos>", "ids": [1]}
        });
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: Value::Null,
            special_tokens,
        })
        .unwrap();
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![1, 10, 20]);
    }

    #[test]
    fn template_processing_special_token_keyed_by_inner_id() {
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence":     {"id": "A",   "type_id": 0}},
        ]);
        let special_tokens = serde_json::json!({
            "bos_alias": {"id": "<s>", "ids": [1], "tokens": ["<s>"]}
        });
        let pp = PostProcessor::from_config(PostProcessorConfig::TemplateProcessing {
            single,
            pair: serde_json::json!([]),
            special_tokens,
        })
        .unwrap();

        assert_eq!(
            pp.post_process_single(vec![10, 20], true),
            vec![1, 10, 20],
            "BOS must be added even when outer JSON key differs from SpecialToken.id"
        );
    }

    #[test]
    fn template_processing_suffix_only() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
            special_tokens: HashMap::from([("</s>".to_string(), vec![2])]),
        };
        assert_eq!(tp.apply_single(vec![10, 20]), vec![10, 20, 2]);
    }

    #[test]
    fn template_processing_bos_and_eos() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
            special_tokens: HashMap::from([
                ("<s>".to_string(), vec![1]),
                ("</s>".to_string(), vec![2]),
            ]),
        };
        assert_eq!(tp.apply_single(vec![10, 20, 30]), vec![1, 10, 20, 30, 2]);
        assert_eq!(tp.apply_single(vec![]), vec![1, 2]);
    }

    #[test]
    fn template_processing_multi_id_special_token() {
        let tp = TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken {
                    id: "<prefix>".into(),
                },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<prefix>".to_string(), vec![100, 101])]),
        };
        assert_eq!(tp.apply_single(vec![10, 20]), vec![100, 101, 10, 20]);
    }

    #[test]
    fn byte_level_post_processor_is_identity() {
        let pp = PostProcessor::ByteLevel;
        assert_eq!(pp.post_process_single(vec![1, 2, 3], true), vec![1, 2, 3]);
        assert_eq!(pp.post_process_single(vec![1, 2, 3], false), vec![1, 2, 3]);
        assert_eq!(
            pp.post_process_single(Vec::<u32>::new(), true),
            Vec::<u32>::new()
        );
    }

    #[test]
    fn num_special_tokens_matches_post_process_delta() {
        let pp = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<s>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
                TemplatePiece::SpecialToken { id: "</s>".into() },
            ],
            special_tokens: HashMap::from([
                ("<s>".to_string(), vec![1]),
                ("</s>".to_string(), vec![2]),
            ]),
        });
        let payload = vec![10u32, 20, 30];
        let with_special = pp.post_process_single(payload.clone(), true);
        let without_special = pp.post_process_single(payload.clone(), false);
        assert_eq!(with_special.len() - without_special.len(), 2);
    }

    #[test]
    fn sequence_post_processor_applies_all() {
        let pp_inner_a = PostProcessor::TemplateProcessing(TemplateProcessing {
            single: vec![
                TemplatePiece::SpecialToken { id: "<a>".into() },
                TemplatePiece::Sequence { id: SequenceId::A },
            ],
            special_tokens: HashMap::from([("<a>".to_string(), vec![99])]),
        });
        let pp_inner_b = PostProcessor::ByteLevel;
        let pp = PostProcessor::Sequence(vec![pp_inner_a, pp_inner_b]);
        assert_eq!(pp.post_process_single(vec![10, 20], true), vec![99, 10, 20]);
        assert_eq!(pp.post_process_single(vec![10, 20], false), vec![10, 20]);
    }
}

mod pre_tokenized {
    use crate::pre_tokenized::*;
    use std::cell::Cell;

    #[test]
    fn inner_parallelism_scope_restores_state() {
        let enabled = || INNER_PARALLELISM.with(Cell::get);
        assert!(enabled());

        without_inner_parallelism(|| {
            assert!(!enabled());
            without_inner_parallelism(|| assert!(!enabled()));
            assert!(!enabled());
        });
        assert!(enabled());

        let panic = std::panic::catch_unwind(|| {
            without_inner_parallelism(|| panic!("test panic"));
        });
        assert!(panic.is_err());
        assert!(enabled());
    }

    #[test]
    fn from_text_empty() {
        let pts = PreTokenizedString::from_text("");
        assert!(pts.splits().is_empty());
        assert!(pts.buffer().is_empty());
    }

    #[test]
    fn from_text_single_span() {
        let pts = PreTokenizedString::from_text("hello world");
        assert_eq!(pts.splits().len(), 1);
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello world");
        assert_eq!(pts.splits()[0].token_id, None);
    }

    #[test]
    fn new_with_mixed_splits() {
        let buffer = "hello<sep>world".to_string();
        let splits = vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..10,
                token_id: Some(42),
            },
            Split {
                range: 10..15,
                token_id: None,
            },
        ];
        let pts = PreTokenizedString::new(buffer, splits);
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
        assert_eq!(pts.split_text(&pts.splits()[1]), "<sep>");
        assert_eq!(pts.splits()[1].token_id, Some(42));
        assert_eq!(pts.split_text(&pts.splits()[2]), "world");
    }

    #[test]
    fn set_buffer_replaces() {
        let mut pts = PreTokenizedString::from_text("old");
        pts.set_buffer(
            "new text".to_string(),
            vec![Split {
                range: 0..3,
                token_id: None,
            }],
        );
        assert_eq!(pts.buffer(), "new text");
        assert_eq!(pts.split_text(&pts.splits()[0]), "new");
    }

    #[test]
    fn refine_splits_keeps_buffer() {
        let mut pts = PreTokenizedString::from_text("hello world");
        pts.refine_splits(vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..11,
                token_id: None,
            },
        ]);
        assert_eq!(pts.buffer(), "hello world");
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
        assert_eq!(pts.split_text(&pts.splits()[1]), " world");
    }

    #[test]
    fn tokenize_text_splits() {
        let pts = PreTokenizedString::from_text("ab");
        let ids = pts
            .tokenize(|text, out| {
                out.extend(text.bytes().map(u32::from));
                Ok(())
            })
            .unwrap();
        assert_eq!(ids, vec![97, 98]);
    }

    #[test]
    fn tokenize_mixed_splits() {
        let buffer = "helloXworld".to_string();
        let splits = vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..6,
                token_id: Some(99),
            },
            Split {
                range: 6..11,
                token_id: None,
            },
        ];
        let pts = PreTokenizedString::new(buffer, splits);
        let ids = pts
            .tokenize(|text, out| {
                out.push(text.len() as u32);
                Ok(())
            })
            .unwrap();
        assert_eq!(ids, vec![5, 99, 5]);
    }

    #[test]
    fn tokenize_empty() {
        let pts = PreTokenizedString::from_text("");
        let ids = pts
            .tokenize(|_, out| {
                out.push(1);
                Ok(())
            })
            .unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn tokenize_propagates_error() {
        let pts = PreTokenizedString::from_text("x");
        let err = pts.tokenize(|_, _out| Err("boom".to_string())).unwrap_err();
        assert_eq!(err, "boom");
    }
}

mod tkz {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::Ordering,
    };

    use crate::Tokenizer;
    use serde_json::{Value, json};

    use crate::tkz::*;

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
        let directory = test_directory("tkz-round-trip");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);

        let json = Tokenizer::load_file(&json_path).unwrap();
        let disabled = Tokenizer::load_file(&json_path).unwrap();
        assert!(!tkz_path.exists());
        assert_eq!(json.encode("ab").unwrap(), disabled.encode("ab").unwrap());

        let cached = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
        let direct = Tokenizer::load_file_with_tkz_cache(&tkz_path).unwrap();
        assert!(tkz_path.is_file());

        for input in ["ab", "é", "e\u{301}!", "  <s>  "] {
            let expected = json.encode(input).unwrap();
            assert_eq!(cached.encode(input).unwrap(), expected);
            assert_eq!(direct.encode(input).unwrap(), expected);
        }
        let expected = json.encode_with_special_tokens("ab", true).unwrap();
        assert_eq!(
            cached.encode_with_special_tokens("ab", true).unwrap(),
            expected
        );
        assert_eq!(direct.decode(&[3, 4], false).unwrap(), "é");
        assert!(direct.is_special_token(5));

        fs::remove_file(&json_path).unwrap();
        let sidecar_only = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
        assert_eq!(sidecar_only.encode("é").unwrap(), vec![3, 4]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rebuilds_invalid_sidecars_and_handles_concurrent_creation() {
        let directory = test_directory("tkz-recovery");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);
        Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();

        write_fixture(&json_path, false);
        let refreshed = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
        assert_eq!(refreshed.encode("ab").unwrap(), vec![0, 1]);

        fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
        let recovered = Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
        assert_eq!(recovered.encode("ab").unwrap(), vec![0, 1]);

        fs::write(&tkz_path, b"SNAPTKZ\0").unwrap();
        fs::remove_file(&json_path).unwrap();
        assert!(Tokenizer::load_file_with_tkz_cache(&tkz_path).is_err());
        fs::remove_dir_all(&directory).unwrap();

        let directory = test_directory("tkz-concurrent");
        let json_path = directory.join("tokenizer.json");
        let tkz_path = directory.join("tokenizer.tkz");
        write_fixture(&json_path, true);
        let threads = (0..4)
            .map(|_| {
                let json_path = json_path.clone();
                std::thread::spawn(move || {
                    Tokenizer::load_file_with_tkz_cache(&json_path).unwrap();
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        let final_tokenizer = Tokenizer::load_file_with_tkz_cache(&tkz_path).unwrap();
        assert_eq!(final_tokenizer.encode("ab").unwrap(), vec![2]);
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

        let error = match Tokenizer::load_file_with_tkz_cache(&json_path) {
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
        let error = match Tokenizer::load_file(Path::new("fixture.model")) {
            Ok(_) => panic!("native SentencePiece paths must be rejected before reading"),
            Err(error) => error,
        };
        assert_eq!(
            error.to_string(),
            "unsupported tokenizer format: native SentencePiece .model files are not supported; export a compatible tokenizer.json"
        );
    }
}
