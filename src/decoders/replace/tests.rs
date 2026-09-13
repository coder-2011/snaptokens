use serde_json::json;

use super::*;

#[test]
fn literal_replace_decoder() {
    let dec = ReplaceDecoder::from_config(json!("▁"), " ".to_string()).unwrap();
    let out = dec.decode_chain(vec!["▁Hello".to_string(), "▁world".to_string()]);
    assert_eq!(out, vec![" Hello", " world"]);
}

#[test]
fn regex_replace_decoder() {
    let dec = ReplaceDecoder::from_config(json!({"Regex": "[0-9]+"}), "#".to_string()).unwrap();
    let out = dec.decode_chain(vec!["a12b".to_string(), "34".to_string()]);
    assert_eq!(out, vec!["a#b", "#"]);
}

#[test]
fn literal_empty_pattern_is_not_noop() {
    let dec = ReplaceDecoder::from_config(json!(""), "-".to_string()).unwrap();
    let out = dec.decode_chain(vec!["ab".to_string()]);
    assert_eq!(out, vec!["-a-b-"]);
}

#[test]
fn regex_replacement_content_is_literal() {
    let dec = ReplaceDecoder::from_config(json!({"Regex": "([a-z]+)"}), "$1".to_string()).unwrap();
    let out = dec.decode_chain(vec!["abc".to_string()]);
    assert_eq!(out, vec!["$1"]);
}
