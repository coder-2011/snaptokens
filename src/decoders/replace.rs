use std::borrow::Cow;

use serde_json::Value;

use super::Error;
use crate::normalizers::Replace;

/// Decoder that applies a configured literal or regular-expression replacement.
#[derive(Clone, Debug)]
pub struct ReplaceDecoder {
    inner: Replace,
}

impl ReplaceDecoder {
    /// Builds a replacement decoder from a JSON pattern and replacement text.
    pub fn from_config(pattern: Value, content: String) -> Result<Self, Error> {
        Ok(Self {
            inner: Replace::from_config(pattern, content)?,
        })
    }

    /// Applies the replacement independently to every token string.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            // Reuse the owned token when normalization reports no replacement.
            .map(|token| match self.inner.normalize(&token) {
                Cow::Borrowed(_) => token,
                Cow::Owned(replaced) => replaced,
            })
            .collect()
    }
}
