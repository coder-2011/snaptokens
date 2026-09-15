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

    /// Tokenizes one already-pre-tokenized text slice.
    pub fn tokenize(&self, input: &str) -> Result<Vec<u32>> {
        match self {
            Self::Bpe(bpe) => bpe.tokenize(input),
            Self::Unigram(unigram) => unigram.tokenize(input),
        }
    }

    /// Appends tokens for one text slice to an existing output buffer.
    #[inline(always)]
    pub fn tokenize_into(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.tokenize_into(input, out),
            Self::Unigram(unigram) => unigram.tokenize_into(input, out),
        }
    }

    /// Appends tokens for raw text through the fused byte-level path.
    #[inline(always)]
    pub fn tokenize_into_fused(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.tokenize_into_fused(input, out),
            Self::Unigram(_) => Err("Unigram does not support BPE fused tokenization".into()),
        }
    }

    #[inline(always)]
    pub(crate) fn tokenize_fused_stream(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        use_parallel_cache: bool,
        scan: impl FnOnce(&mut bpe::FusedStream<'_>),
    ) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.tokenize_fused_stream(input, out, use_parallel_cache, scan),
            Self::Unigram(_) => Err("Unigram does not support BPE fused tokenization".into()),
        }
    }

    /// Appends IDs for byte-level-pre-tokenized splits.
    #[inline(always)]
    pub fn tokenize_batch_fused(
        &self,
        buffer: &str,
        splits: &[crate::pre_tokenized::Split],
        out: &mut Vec<u32>,
    ) -> Result<()> {
        match self {
            Self::Bpe(bpe) => bpe.tokenize_batch_fused(buffer, splits, out),
            Self::Unigram(_) => Err("Unigram does not support BPE fused tokenization".into()),
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

    /// Returns safe BPE split-boundary information when the model supports it.
    pub fn bigram_bridge_table(&self) -> Option<&bpe::BigramBridgeTable> {
        match self {
            Self::Bpe(bpe) => bpe.bigram_bridge_table(),
            Self::Unigram(_) => None,
        }
    }

    /// Returns the BPE model when BPE-only fused paths are semantically valid.
    pub(crate) fn bpe(&self) -> Option<&Bpe> {
        match self {
            Self::Bpe(bpe) => Some(bpe),
            Self::Unigram(_) => None,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/models.rs"]
mod tests;
