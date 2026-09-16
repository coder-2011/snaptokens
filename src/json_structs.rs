use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{models, pre_tokenizers};

/// One `added_tokens` entry from a Hugging Face tokenizer file.
#[derive(Clone, Debug, Deserialize)]
pub struct AddedTokenConfig {
    /// Numeric ID emitted when this added token matches.
    pub id: u32,
    /// Exact token text to match.
    pub content: String,
    /// Whether matching requires word boundaries on both sides.
    #[serde(default)]
    pub single_word: bool,
    /// Whether matching absorbs whitespace before the token.
    #[serde(default)]
    pub lstrip: bool,
    /// Whether matching absorbs whitespace after the token.
    #[serde(default)]
    pub rstrip: bool,
    /// Whether the content should be matched against normalized
    /// text.
    #[serde(default)]
    pub normalized: bool,
    /// Whether this is a "special" token (e.g. BOS/EOS). All added
    /// tokens are matched regardless of this flag; it only affects
    /// post-processing.
    #[serde(default)]
    pub special: bool,
}

/// The supported top-level contents of a Hugging Face `tokenizer.json` file.
#[derive(Debug, Deserialize)]
pub struct TokenizerJson {
    /// Added tokens matched before ordinary BPE tokenization.
    #[serde(default)]
    pub added_tokens: Vec<AddedTokenConfig>,
    /// Optional text normalizer.
    pub normalizer: Option<NormalizerConfig>,
    /// Optional text pre-tokenizer.
    pub pre_tokenizer: Option<PreTokenizerConfig>,
    /// The required BPE model configuration.
    pub model: ModelConfig,
    /// Optional post-processor for special tokens.
    pub post_processor: Option<PostProcessorConfig>,
    /// Optional decoder for converting token strings back to text.
    pub decoder: Option<DecoderConfig>,
}

/// A supported normalizer configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NormalizerConfig {
    /// Applies normalizers from left to right.
    Sequence {
        /// Nested normalizer configurations.
        #[serde(default)]
        normalizers: Vec<NormalizerConfig>,
    },
    /// Unicode NFC normalization.
    #[serde(rename = "NFC")]
    Nfc,
    /// A literal or regular-expression replacement.
    Replace {
        /// The Hugging Face replacement pattern.
        pattern: Value,
        /// Text substituted for every match.
        #[serde(default)]
        content: String,
    },
}

/// A supported pre-tokenizer configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PreTokenizerConfig {
    /// Applies pre-tokenizers from left to right.
    Sequence {
        /// Nested pre-tokenizer configurations.
        #[serde(default)]
        pretokenizers: Vec<PreTokenizerConfig>,
    },
    /// A GPT-2-style byte-level pre-tokenizer.
    ByteLevel(pre_tokenizers::ByteLevel),
    /// A pattern-based splitter.
    Split(pre_tokenizers::Split),
}

/// A supported tokenization-model configuration.
#[derive(Clone, Debug)]
pub enum ModelConfig {
    /// A byte-pair encoding model.
    Bpe(Box<models::bpe::Bpe>),
}

impl<'de> Deserialize<'de> for ModelConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the large BPE payload once; `RawBpe` preserves the tagged
        // and legacy validation that used to happen before `from_value`.
        models::bpe::Bpe::deserialize(deserializer).map(|bpe| Self::Bpe(Box::new(bpe)))
    }
}

/// A supported post-processor configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PostProcessorConfig {
    /// Applies post-processors from left to right.
    Sequence {
        /// Nested post-processor configurations.
        #[serde(default)]
        processors: Vec<PostProcessorConfig>,
    },
    /// Byte-level post-processing, which does not alter token IDs.
    ByteLevel {
        /// Upstream compatibility setting retained from the JSON format.
        #[serde(default)]
        add_prefix_space: bool,
        /// Upstream compatibility setting retained from the JSON format.
        #[serde(default)]
        trim_offsets: bool,
        /// Upstream compatibility setting retained from the JSON format.
        #[serde(default)]
        use_regex: bool,
    },
    /// A template that inserts configured special-token ID sequences.
    TemplateProcessing {
        /// Template for a single encoded sequence.
        #[serde(default)]
        single: Value,
        /// Pair template, validated but not used by the single-sequence API.
        #[serde(default)]
        pair: Value,
        /// Named special-token definitions referenced by the templates.
        #[serde(default)]
        special_tokens: Value,
    },
}

/// A supported decoder configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum DecoderConfig {
    /// Applies decoder steps from left to right.
    Sequence {
        /// Nested decoder configurations.
        #[serde(default)]
        decoders: Vec<DecoderConfig>,
    },
    /// Reverses GPT-2 byte-level encoding.
    ByteLevel,
    /// Applies a literal or regular-expression replacement.
    Replace {
        /// The Hugging Face replacement pattern.
        pattern: Value,
        /// Text substituted for every match.
        #[serde(default)]
        content: String,
    },
    /// Identity decoder used in some tokenizer configurations.
    Fuse,
    /// Decodes `<0xNN>` fallback byte tokens.
    ByteFallback,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn rejects_unimplemented_pipeline_types() {
        assert!(serde_json::from_value::<NormalizerConfig>(json!({"type": "Lowercase"})).is_err());
        assert!(
            serde_json::from_value::<PreTokenizerConfig>(json!({"type": "Whitespace"})).is_err()
        );
        assert!(serde_json::from_value::<ModelConfig>(json!({"type": "WordPiece"})).is_err());
        assert!(
            serde_json::from_value::<PostProcessorConfig>(json!({"type": "BertProcessing"}))
                .is_err()
        );
        assert!(serde_json::from_value::<DecoderConfig>(json!({"type": "WordPiece"})).is_err());
    }

    #[test]
    fn accepts_legacy_untagged_bpe_model() {
        let model = json!({"vocab": {"a": 0}, "merges": []});
        assert!(serde_json::from_value::<ModelConfig>(model).is_ok());
    }
}
