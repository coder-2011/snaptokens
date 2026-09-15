use std::borrow::Cow;
use std::sync::LazyLock;

use icu_normalizer::{ComposingNormalizer, ComposingNormalizerBorrowed};

static NFC_NORMALIZER: LazyLock<ComposingNormalizerBorrowed<'static>> =
    LazyLock::new(ComposingNormalizer::new_nfc);

/// NFC (Canonical Decomposition, followed by Canonical Composition) normalizer.
///
/// Applies Unicode NFC normalization to the input text. If the input is already
/// in NFC form the original string is returned without allocation.
#[derive(Debug)]
pub struct Nfc;

impl Nfc {
    /// Normalize `input` to NFC form.
    ///
    /// Returns `Cow::Borrowed` when the input is already NFC, avoiding
    /// allocation. Uses ICU4X's `ComposingNormalizer` which finds the
    /// longest already-normalized prefix in a single pass, only allocating
    /// when the suffix actually needs transformation.
    pub fn normalize<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if input.is_ascii() {
            return Cow::Borrowed(input);
        }
        NFC_NORMALIZER.normalize(input)
    }
}

#[cfg(test)]
mod tests {
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
    fn mixed_ascii_and_decomposed() {
        let out = Nfc.normalize("cafe\u{0301}!");
        assert_eq!(out, "caf\u{e9}!");
    }
}
