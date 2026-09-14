use std::borrow::Cow;

use base64::{Engine as _, engine::general_purpose::STANDARD};

use super::Error;

/// Applies the SentencePiece charsmap embedded in Hugging Face tokenizer JSON.
#[derive(Debug)]
pub struct Precompiled(spm_precompiled::Precompiled);

impl Precompiled {
    /// Decodes and validates one base64-encoded SentencePiece charsmap.
    pub fn from_config(precompiled_charsmap: String) -> Result<Self, Error> {
        let bytes = STANDARD
            .decode(precompiled_charsmap)
            .map_err(|error| Error::Precompiled(error.to_string()))?;
        let charsmap = spm_precompiled::Precompiled::from(&bytes)
            .map_err(|error| Error::Precompiled(error.to_string()))?;
        Ok(Self(charsmap))
    }

    /// Returns a borrowed input when the charsmap makes no textual change.
    pub fn normalize<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let normalized = self.0.normalize_string(input);
        if normalized == input {
            Cow::Borrowed(input)
        } else {
            Cow::Owned(normalized)
        }
    }
}
