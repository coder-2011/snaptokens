use std::borrow::Cow;

use serde_json::json;

use super::*;

#[test]
fn literal_replace() {
    let repl = Replace::from_config(json!({"String": " "}), "▁".to_string()).unwrap();
    assert_eq!(repl.normalize("a b c"), "a▁b▁c");
}

#[test]
fn literal_no_change_borrowed() {
    let repl = Replace::from_config(json!({"String": "x"}), "y".to_string()).unwrap();
    let out = repl.normalize("abc");
    assert_eq!(out, "abc");
    assert!(matches!(out, Cow::Borrowed(_)));
}

#[test]
fn regex_replace() {
    let repl = Replace::from_config(json!({"Regex": "\\s+"}), " ".to_string()).unwrap();
    assert_eq!(repl.normalize("hello   world"), "hello world");
}

#[test]
fn accepts_plain_string_pattern() {
    let repl = Replace::from_config(json!("."), " ".to_string()).unwrap();
    assert_eq!(repl.normalize("hello.world"), "hello world");
}

#[test]
fn literal_empty_pattern_is_not_noop() {
    let repl = Replace::from_config(json!(""), "-".to_string()).unwrap();
    assert_eq!(repl.normalize("ab"), "-a-b-");
}

#[test]
fn regex_replacement_content_is_literal() {
    let repl = Replace::from_config(json!({"Regex": "([a-z]+)"}), "$1".to_string()).unwrap();
    assert_eq!(repl.normalize("abc"), "$1");
}
