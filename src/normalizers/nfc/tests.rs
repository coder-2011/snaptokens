use super::*;
use std::borrow::Cow;

#[test]
fn normalization_preserves_borrowed_input_until_composition_is_needed() {
    for (input, expected) in [
        ("", ""),
        ("hello world", "hello world"),
        ("é", "é"),
        ("e\u{301}", "é"),
        ("cafe\u{301}!", "café!"),
        ("\u{1100}\u{1161}\u{11A8}", "각"),
    ] {
        let actual = Nfc.normalize(input);
        assert_eq!(actual, expected, "{input:?}");
        assert_eq!(
            matches!(actual, Cow::Borrowed(_)),
            input == expected,
            "{input:?}"
        );
    }
}
