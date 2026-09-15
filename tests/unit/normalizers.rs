mod nfc {
    use std::borrow::Cow;

    use crate::normalizers::nfc::*;

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
}

mod replace {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::normalizers::replace::*;

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
}
