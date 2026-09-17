use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, value::RawValue};

use crate::{Error, LoadMode, TruncationDirection, models, pre_tokenizers};

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
///
/// Deserializes only from JSON text sources such as `serde_json::from_str`
/// or `from_slice`: the model section borrows its raw text span, which a
/// `serde_json::Value` cannot lend. Render a `Value` to text first, as
/// [`crate::Tokenizer::from_json`] does.
#[derive(Debug, Deserialize)]
pub struct TokenizerJson {
    /// Optional truncation settings restored by the Python wrapper.
    pub truncation: Option<TruncationParams>,
    /// Optional padding settings restored by the Python wrapper.
    pub padding: Option<PaddingParams>,
    /// Added tokens matched before ordinary model tokenization.
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

/// Failure to read configuration or construct its consumer.
#[derive(Debug, thiserror::Error)]
pub enum LoadError<E> {
    /// Reading or decoding the configuration failed.
    #[error(transparent)]
    Load(#[from] Error),
    /// The consumer rejected the decoded configuration.
    #[error(transparent)]
    Construct(E),
}

impl TokenizerJson {
    /// Load typed configuration and construct its consumer before publishing a cache.
    pub fn load_file_with<T, E>(
        path: &std::path::Path,
        mode: LoadMode,
        mut construct: impl FnMut(Self) -> Result<T, E>,
    ) -> Result<T, LoadError<E>> {
        crate::reject_native_sentencepiece_model(path)?;
        match mode {
            LoadMode::JsonOnly => {
                let source = std::fs::read(path).map_err(Error::from)?;
                let config = serde_json::from_slice(&source).map_err(Error::from)?;
                construct(config).map_err(LoadError::Construct)
            }
            LoadMode::TkzCache => crate::tkz::load_or_create(path, construct),
        }
    }
}

/// Which input sequence supplies tokens removed by truncation.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum TruncationStrategy {
    /// Remove tokens from the longer sequence first.
    #[default]
    #[serde(alias = "longest_first")]
    LongestFirst,
    /// Remove tokens only from the first sequence.
    #[serde(alias = "only_first")]
    OnlyFirst,
    /// Remove tokens only from the second sequence.
    #[serde(alias = "only_second")]
    OnlySecond,
}

impl TruncationStrategy {
    /// The spelling used by the Python configuration API.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LongestFirst => "longest_first",
            Self::OnlyFirst => "only_first",
            Self::OnlySecond => "only_second",
        }
    }
}

/// Serialized truncation settings, independent of a binding's supported features.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TruncationParams {
    /// Maximum sequence length, including requested special tokens.
    pub max_length: usize,
    /// Overlap between successive overflow windows.
    #[serde(default)]
    pub stride: usize,
    /// Which sequence to shorten.
    #[serde(default)]
    pub strategy: TruncationStrategy,
    /// Which end of the sequence to discard.
    #[serde(default)]
    pub direction: TruncationDirection,
}

/// How the padding target is selected.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PaddingStrategy {
    /// Pad to the longest sequence in the batch.
    BatchLongest,
    /// Pad to at least this many tokens.
    Fixed(usize),
}

/// Serialized padding settings. Reads both canonical and older shim formats.
#[derive(Clone, Debug, Serialize)]
pub struct PaddingParams {
    /// Which end receives padding tokens.
    pub direction: TruncationDirection,
    /// Padding token ID.
    pub pad_id: u32,
    /// Type ID assigned to padding tokens.
    pub pad_type_id: u32,
    /// Padding token spelling.
    pub pad_token: String,
    /// How to select the padded length.
    pub strategy: PaddingStrategy,
    /// Optional multiple to round the padded length up to.
    pub pad_to_multiple_of: Option<usize>,
}

impl PaddingParams {
    /// Fixed target length, or `None` for batch-longest padding.
    pub fn length(&self) -> Option<usize> {
        match self.strategy {
            PaddingStrategy::BatchLongest => None,
            PaddingStrategy::Fixed(length) => Some(length),
        }
    }
}

impl<'de> Deserialize<'de> for PaddingParams {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Distinguish an absent field from a present null, notably legacy length=null.
        fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
        where
            D: Deserializer<'de>,
            T: Deserialize<'de>,
        {
            T::deserialize(deserializer).map(Some)
        }

        #[derive(Deserialize)]
        struct Fields {
            direction: TruncationDirection,
            pad_id: u32,
            pad_type_id: u32,
            pad_token: String,
            #[serde(default, deserialize_with = "present")]
            strategy: Option<PaddingStrategy>,
            #[serde(default, deserialize_with = "present")]
            length: Option<Option<usize>>,
            pad_to_multiple_of: Option<usize>,
        }

        let fields = Fields::deserialize(deserializer)?;
        let strategy = match fields.strategy {
            Some(strategy) => strategy,
            None => fields
                .length
                .ok_or_else(|| serde::de::Error::missing_field("strategy"))?
                .map_or(PaddingStrategy::BatchLongest, PaddingStrategy::Fixed),
        };
        Ok(Self {
            direction: fields.direction,
            pad_id: fields.pad_id,
            pad_type_id: fields.pad_type_id,
            pad_token: fields.pad_token,
            strategy,
            pad_to_multiple_of: fields.pad_to_multiple_of,
        })
    }
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
///
/// Deserializes only from JSON text sources such as `serde_json::from_str`
/// or `from_slice`; a `serde_json::Value` must be rendered to text first.
/// The borrowed raw span is what keeps the large vocabulary single-pass.
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
        /// Only the two dispatch facts; every other field is skipped unbuilt.
        #[derive(Deserialize)]
        struct ModelProbe<'a> {
            #[serde(rename = "type")]
            tag: Option<Value>,
            #[serde(borrow)]
            vocab: Option<&'a RawValue>,
        }

        // Borrow the model object as its raw text span, scan only the tag and
        // the vocabulary span, then deserialize the large payload once.
        let raw = <&RawValue>::deserialize(deserializer)?;
        let probe: ModelProbe =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        // The vocabulary's first byte settles untagged files: the reference
        // implementation fixes BPE vocabularies as JSON objects and Unigram
        // vocabularies as arrays. A scalar dispatches like a missing one.
        let vocab_shape = probe
            .vocab
            .and_then(|vocab| vocab.get().trim_start().bytes().next());
        let bpe = || {
            serde_json::from_str(raw.get())
                .map(|bpe| Self::Bpe(Box::new(bpe)))
                .map_err(serde::de::Error::custom)
        };
        let unigram = || {
            serde_json::from_str(raw.get())
                .map(|unigram| Self::Unigram(Box::new(unigram)))
                .map_err(serde::de::Error::custom)
        };
        match (probe.tag.as_ref().and_then(Value::as_str), vocab_shape) {
            (Some("BPE"), _) => bpe(),
            (Some("Unigram"), _) => unigram(),
            // Hugging Face's older SentencePiece exports omit `type`; their
            // scored array vocabulary is unambiguous and still accepted by the
            // upstream Unigram deserializer.
            (None, Some(b'[')) => unigram(),
            // Older BPE exports omit `type` too, but their object vocabulary
            // cannot be confused with Unigram's scored array vocabulary.
            (None, Some(b'{')) => bpe(),
            (Some(other), _) => Err(serde::de::Error::custom(format!(
                "unsupported model type: {other}"
            ))),
            (None, _) => Err(serde::de::Error::custom(
                "model is missing a type and an Unigram vocabulary",
            )),
        }
    }
}

/// A supported post-processor configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
        // The model deserializer needs raw JSON text for its zero-copy span.
        assert!(serde_json::from_str::<ModelConfig>(r#"{"type": "WordPiece"}"#).is_err());
        assert!(
            serde_json::from_value::<PostProcessorConfig>(json!({"type": "BertProcessing"}))
                .is_err()
        );
        assert!(serde_json::from_value::<DecoderConfig>(json!({"type": "WordPiece"})).is_err());
    }

    #[test]
    fn accepts_legacy_untagged_bpe_model() {
        let model = json!({"vocab": {"a": 0}, "merges": []}).to_string();
        assert!(serde_json::from_str::<ModelConfig>(&model).is_ok());
    }

    #[test]
    fn accepts_legacy_untagged_unigram_model() {
        let model = json!({"unk_id": 0, "vocab": [["<unk>", 0.0], ["a", 1.0]]}).to_string();
        assert!(matches!(
            serde_json::from_str::<ModelConfig>(&model),
            Ok(ModelConfig::Unigram(_))
        ));
    }

    #[test]
    fn padding_formats_preserve_explicit_strategy_and_reject_invalid_values() {
        let base = json!({"direction":"Left", "pad_id":0, "pad_type_id":0, "pad_token":"[PAD]"});
        for (fields, expected) in [
            (json!({"length": null}), None),
            (json!({"length": 4}), Some(4)),
            (json!({"strategy": {"Fixed": 8}, "length": 4}), Some(8)),
        ] {
            let mut value = base.clone();
            value
                .as_object_mut()
                .unwrap()
                .extend(fields.as_object().unwrap().clone());
            let params: PaddingParams = serde_json::from_value(value).unwrap();
            assert_eq!(params.length(), expected);
            assert!(
                serde_json::to_value(params)
                    .unwrap()
                    .get("length")
                    .is_none()
            );
        }
        for fields in [
            json!({}),
            json!({"strategy": null, "length": 4}),
            json!({"length": -1}),
        ] {
            let mut value = base.clone();
            value
                .as_object_mut()
                .unwrap()
                .extend(fields.as_object().unwrap().clone());
            assert!(serde_json::from_value::<PaddingParams>(value).is_err());
        }
    }
}
