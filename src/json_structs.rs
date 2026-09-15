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
    /// The required tokenization-model configuration.
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
    /// SentencePiece's serialized character-rewrite map.
    Precompiled {
        /// Base64-encoded SentencePiece charsmap bytes.
        precompiled_charsmap: String,
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
    /// Removes Unicode whitespace between pre-tokenized pieces.
    WhitespaceSplit,
    /// Replaces SentencePiece spaces with a marker character.
    Metaspace(MetaspaceConfig),
}

/// Compatibility configuration shared by Metaspace pre-tokenizers and decoders.
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceConfig {
    /// One marker character that stands for an ASCII space.
    pub replacement: String,
    /// Legacy Hugging Face spelling for an always/never prepend scheme.
    #[serde(default)]
    pub add_prefix_space: Option<bool>,
    /// Current Hugging Face spelling for the prepend behavior.
    #[serde(default)]
    pub prepend_scheme: Option<MetaspacePrependScheme>,
    /// Whether markers begin a new pre-tokenized piece.
    #[serde(default)]
    pub split: Option<bool>,
}

/// Controls whether Metaspace prepends its marker before ordinary text.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetaspacePrependScheme {
    /// Prepends only to the first original split; unavailable without offsets.
    First,
    /// Does not prepend a marker.
    Never,
    /// Prepends a marker to every ordinary split.
    Always,
}

/// A supported tokenization-model configuration.
#[derive(Clone, Debug)]
pub enum ModelConfig {
    /// A byte-pair encoding model.
    Bpe(Box<models::bpe::Bpe>),
    /// A scored SentencePiece-style Unigram model.
    Unigram(Box<models::unigram::Unigram>),
}

impl<'de> Deserialize<'de> for ModelConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let model_type = value.get("type").and_then(Value::as_str);
        match model_type {
            Some("BPE") => serde_json::from_value(value)
                .map(|bpe| Self::Bpe(Box::new(bpe)))
                .map_err(serde::de::Error::custom),
            Some("Unigram") => serde_json::from_value(value)
                .map(|unigram| Self::Unigram(Box::new(unigram)))
                .map_err(serde::de::Error::custom),
            // Hugging Face's older SentencePiece exports omit `type`; their
            // scored array vocabulary is unambiguous and still accepted by the
            // upstream Unigram deserializer.
            None if value.get("vocab").is_some_and(Value::is_array) => {
                serde_json::from_value(value)
                    .map(|unigram| Self::Unigram(Box::new(unigram)))
                    .map_err(serde::de::Error::custom)
            }
            // Older BPE exports omit `type` too, but their object vocabulary
            // cannot be confused with Unigram's scored array vocabulary.
            None if value.get("vocab").is_some_and(Value::is_object) => {
                serde_json::from_value(value)
                    .map(|bpe| Self::Bpe(Box::new(bpe)))
                    .map_err(serde::de::Error::custom)
            }
            Some(other) => Err(serde::de::Error::custom(format!(
                "unsupported model type: {other}"
            ))),
            None => Err(serde::de::Error::custom(
                "model is missing a type and an Unigram vocabulary",
            )),
        }
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
    /// Reverses a SentencePiece Metaspace pre-tokenizer.
    Metaspace(MetaspaceConfig),
}
