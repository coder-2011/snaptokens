use serde_json::json;

use super::*;

#[test]
fn rejects_unimplemented_pipeline_types() {
    assert!(serde_json::from_value::<NormalizerConfig>(json!({"type": "Lowercase"})).is_err());
    assert!(serde_json::from_value::<PreTokenizerConfig>(json!({"type": "Whitespace"})).is_err());
    assert!(serde_json::from_value::<ModelConfig>(json!({"type": "WordPiece"})).is_err());
    assert!(
        serde_json::from_value::<PostProcessorConfig>(json!({"type": "BertProcessing"})).is_err()
    );
    assert!(serde_json::from_value::<DecoderConfig>(json!({"type": "WordPiece"})).is_err());
}

#[test]
fn accepts_legacy_untagged_bpe_model() {
    let model = json!({"vocab": {"a": 0}, "merges": []});
    assert!(serde_json::from_value::<ModelConfig>(model).is_ok());
}
