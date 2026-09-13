use std::borrow::Cow;

use super::*;

#[test]
fn ascii_unchanged() {
    let out = Nfc.normalize("hello world");
    assert_eq!(out, "hello world");
    assert!(matches!(out, Cow::Borrowed(_)));
}

#[test]
fn already_composed() {
    let out = Nfc.normalize("\u{e9}");
    assert_eq!(out, "\u{e9}");
    assert!(matches!(out, Cow::Borrowed(_)));
}

#[test]
fn decomposes_then_composes() {
    let out = Nfc.normalize("e\u{0301}");
    assert_eq!(out, "\u{e9}");
    assert!(matches!(out, Cow::Owned(_)));
}

#[test]
fn empty_string() {
    let out = Nfc.normalize("");
    assert_eq!(out, "");
    assert!(matches!(out, Cow::Borrowed(_)));
}

#[test]
fn mixed_ascii_and_decomposed() {
    let out = Nfc.normalize("cafe\u{0301}!");
    assert_eq!(out, "caf\u{e9}!");
}

#[test]
fn hangul_composition() {
    let out = Nfc.normalize("\u{1100}\u{1161}\u{11A8}");
    assert_eq!(out, "\u{AC01}");
}
