use super::*;

#[test]
fn identity_template_does_not_capture_repeated_or_absent_sequences() {
    for (single, expected) in [
        (
            vec![TemplatePiece::Sequence { id: SequenceId::A }],
            vec![10, 20],
        ),
        (
            vec![TemplatePiece::Sequence { id: SequenceId::A }; 2],
            vec![10, 20, 10, 20],
        ),
        (vec![TemplatePiece::Sequence { id: SequenceId::B }], vec![]),
        (vec![], vec![]),
    ] {
        let template = TemplateProcessing {
            single,
            special_tokens: HashMap::new(),
        };
        assert_eq!(template.apply_single(vec![10, 20]), expected);
    }
}

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
