/// Byte-pair encoding model implementation.
pub mod bpe;
/// SentencePiece-style Unigram model implementation.
pub mod unigram;

use self::bpe::Bpe;
use self::unigram::Unigram;
use crate::json_structs::ModelConfig;

pub(crate) type Result<T> = std::result::Result<T, String>;

/// A supported tokenization model.
// BPE keeps its hot lookup tables inline; boxing it to satisfy the enum-size
// lint would add an unnecessary pointer indirection to every BPE tokenizer.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Model {
    /// A byte-pair encoding model.
    Bpe(Bpe),
    /// A scored SentencePiece-style Unigram model.
    Unigram(Unigram),
}

impl Model {
    /// Builds a model from parsed tokenizer JSON configuration.
    pub fn from_config(config: ModelConfig) -> Result<Self> {
        match config {
            ModelConfig::Bpe(bpe) => Ok(Self::Bpe(*bpe)),
            ModelConfig::Unigram(unigram) => Ok(Self::Unigram(*unigram)),
        }
    }

    /// Appends tokens for one already-pre-tokenized text slice.
    pub(crate) fn tokenize_into(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.append_bpe_ids(input, out),
            Self::Unigram(unigram) => {
                let mut scratch = unigram::ViterbiScratch::default();
                unigram.append_viterbi_ids(input, out, &mut scratch)
            }
        }
    }

    /// Check discarded pieces without populating encode caches.
    pub(crate) fn validate_input(&self, input: &str) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.validate_input(input),
            Self::Unigram(unigram) => {
                let mut discarded = Vec::new();
                let mut scratch = unigram::ViterbiScratch::default();
                unigram.append_viterbi_ids(input, &mut discarded, &mut scratch)
            }
        }
    }

    /// Returns the model-vocabulary text for an ID.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        match self {
            Self::Bpe(bpe) => bpe.id_to_token(id),
            Self::Unigram(unigram) => unigram.id_to_token(id),
        }
    }

    /// Returns the model-vocabulary ID for token text.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        match self {
            Self::Bpe(bpe) => bpe.token_to_id(token),
            Self::Unigram(unigram) => unigram.token_to_id(token),
        }
    }

    /// Returns the number of entries in the model vocabulary.
    pub fn vocab_size(&self) -> usize {
        match self {
            Self::Bpe(bpe) => bpe.vocab_size(),
            Self::Unigram(unigram) => unigram.vocab_size(),
        }
    }

    /// Returns the BPE model when BPE-only paths are semantically valid.
    pub(crate) fn bpe(&self) -> Option<&Bpe> {
        match self {
            Self::Bpe(bpe) => Some(bpe),
            Self::Unigram(_) => None,
        }
    }

    /// Returns the Unigram model when this tokenizer uses Unigram.
    pub(crate) fn unigram(&self) -> Option<&Unigram> {
        match self {
            Self::Bpe(_) => None,
            Self::Unigram(unigram) => Some(unigram),
        }
    }
}
