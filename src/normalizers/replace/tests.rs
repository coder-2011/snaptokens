use std::borrow::Cow;

use serde_json::json;

use super::*;

#[test]
fn literal_matches_preserve_unicode_empty_and_overlap_semantics() {
    for input in ["", "ababa", "aaaa", "aé中é", "🚀🚀", "\0a\0"] {
        for needle in ["", "a", "aba", "aa", "é", "🚀", "\0", "missing"] {
            for replacement in ["", "$1", "中中"] {
                let result = replace_literal(input, needle, replacement);
                assert_eq!(result, input.replace(needle, replacement));
                assert_eq!(matches!(result, Cow::Borrowed(_)), !input.contains(needle));
            }
        }
    }
}

#[test]
fn pattern_object_preserves_literal_precedence_and_regex_fallback() {
    let literal = Replace::from_config(json!({"String": "a", "Regex": "["}), "b".into()).unwrap();
    assert_eq!(literal.normalize("a"), "b");
    let regex = Replace::from_config(json!({"String": null, "Regex": "a+"}), "b".into()).unwrap();
    assert_eq!(regex.normalize("aaa"), "b");
    for pattern in [
        json!(null),
        json!([]),
        json!({"String": 3}),
        json!({"Regex": false}),
    ] {
        assert!(Replace::from_config(pattern, "b".into()).is_err());
    }
}

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
