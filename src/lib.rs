#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Exact, fast tokenization for supported local Hugging Face BPE and Unigram JSON files.
//!
//! `Tokenizer::load_file` loads either a `tokenizer.json` directly or through
//! the optional binary sidecar, selected by [`LoadMode`].
//!
//! ```no_run
//! use snaptokens::{LoadMode, Tokenizer};
//!
//! # fn main() -> Result<(), snaptokens::Error> {
//! let tokenizer = Tokenizer::load_file("tokenizer.json".as_ref(), LoadMode::JsonOnly)?;
//! let ids = tokenizer.encode("hello", false)?;
//!
//! let cached = Tokenizer::load_file("tokenizer.json".as_ref(), LoadMode::TkzCache)?;
//! assert_eq!(cached.decode(&ids, false)?, "hello");
//! # Ok(())
//! # }
//! ```

/// Added-token lookup, matching, and metadata.
pub mod added_tokens;
/// Decoder implementations for token strings.
pub mod decoders;
/// Serde representations of supported `tokenizer.json` sections.
pub mod json_structs;
/// Tokenization models.
pub mod models;
/// Text normalizers.
pub mod normalizers;
/// Post-processors that add special tokens to encoded IDs.
pub mod post_processors;
/// The intermediate text-and-split representation used before the model.
pub mod pre_tokenized;
/// Pre-tokenizers that divide text into model inputs.
pub mod pre_tokenizers;
mod tkz;

use std::{borrow::Cow, path::Path};

use rayon::prelude::*;
use serde_json::Value;

const WIDE_BATCH_TASKS: usize = 4;
const SHORT_BATCH_INPUT_BYTES: usize = 16 * 1024;
const PARALLEL_BATCH_BYTES_PER_THREAD: usize = 2 * 1024;
const PARALLEL_CACHE_BATCH_BYTES: usize = 64 * 1024;
const COMPACT_RAGGED_OUTPUT_BYTES: usize = 8 * 1024 * 1024;
const RAGGED_CHUNKS_PER_WORKER: usize = 8;
const SHORT_OUTPUT_BYTES: usize = 16 * 1024;

fn output_capacity(input_bytes: usize) -> usize {
    if input_bytes <= SHORT_OUTPUT_BYTES {
        return input_bytes.div_ceil(2).saturating_add(16).min(input_bytes);
    }
    input_bytes.div_ceil(3)
}

pub use self::{
    added_tokens::{AddedTokenInfo, AddedTokens},
    json_structs::{
        AddedTokenConfig, DecoderConfig, ModelConfig, NormalizerConfig, PostProcessorConfig,
        PreTokenizerConfig, TokenizerJson,
    },
    models::Model,
    normalizers::{Nfc, Normalizer, Replace},
    post_processors::PostProcessor,
    pre_tokenized::TruncationDirection,
    pre_tokenizers::{ByteLevel, PreTokenizer, Split, SplitBehavior},
};

use self::{
    added_tokens::Segment,
    decoders::Decoder,
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
};

/// An error while loading, constructing, encoding, or decoding a tokenizer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reading a tokenizer JSON file or `.tkz` sidecar failed.
    #[error("failed to read tokenizer files: {0}")]
    Io(#[from] std::io::Error),

    /// Parsing JSON tokenizer data failed.
    #[error("failed to parse tokenizer files: {0}")]
    Json(#[from] serde_json::Error),

    /// A `.tkz` sidecar was malformed or did not match its JSON source.
    #[error("invalid .tkz tokenizer: {0}")]
    Tkz(String),

    /// The file format is recognized but intentionally outside Snaptokens' scope.
    #[error("unsupported tokenizer format: {0}")]
    Unsupported(String),

    /// Constructing or applying a normalizer failed.
    #[error("normalizer error: {0}")]
    Normalizer(#[from] normalizers::Error),

    /// Constructing or applying a pre-tokenizer failed.
    #[error("pre-tokenizer error: {0}")]
    PreTokenizer(#[from] pre_tokenizers::Error),

    /// Constructing or applying a post-processor failed.
    #[error("post-processor error: {0}")]
    PostProcessor(#[from] post_processors::Error),

    /// Constructing or applying a decoder failed.
    #[error("decoder error: {0}")]
    Decoder(#[from] decoders::Error),

    /// The model configuration or tokenization operation was invalid.
    #[error("model error: {0}")]
    Model(String),
}

/// A loaded Hugging Face tokenizer with a supported model and pipeline.
pub struct Tokenizer {
    added_tokens: Option<AddedTokens>,
    normalizer: Option<Normalizer>,
    pre_tokenizer: Option<PreTokenizer>,
    model: Model,
    post_processor: Option<PostProcessor>,
    decoder: Option<Decoder>,
    /// Enables vocab-derived split points for non-ByteLevel pipelines.
    needs_vocab_splitting: bool,
}

/// Selects whether a tokenizer file loads directly from JSON or through `.tkz`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadMode {
    /// Loads and parses a JSON tokenizer file without reading or writing a cache.
    JsonOnly,
    /// Loads a `.tkz` file directly or creates or reuses a sibling sidecar for JSON.
    TkzCache,
}

impl Tokenizer {
    /// Construct a tokenizer from typed configuration without deserializing its model again.
    pub fn from_config(json: TokenizerJson) -> Result<Self, Error> {
        let normalizer = json.normalizer.map(Normalizer::from_config).transpose()?;
        let added_tokens =
            AddedTokens::from_configs_with_normalizer(&json.added_tokens, normalizer.as_ref())
                .map_err(Error::Model)?;
        let pre_tokenizer = json
            .pre_tokenizer
            .map(PreTokenizer::from_config)
            .transpose()?;
        let model = Model::from_config(json.model).map_err(Error::Model)?;
        let post_processor = json
            .post_processor
            .map(PostProcessor::from_config)
            .transpose()?;
        let decoder = json.decoder.map(Decoder::from_config).transpose()?;

        // Vocabulary boundary proofs rely on BPE merge reachability; Unigram
        // always follows the ordinary pre-tokenized model path instead.
        let needs_vocab_splitting = model.bpe().is_some()
            && !pre_tokenizer
                .as_ref()
                .is_some_and(PreTokenizer::contains_byte_level);

        Ok(Self {
            added_tokens,
            normalizer,
            pre_tokenizer,
            model,
            post_processor,
            decoder,
            needs_vocab_splitting,
        })
    }

    /// Builds a tokenizer from the parsed contents of `tokenizer.json`.
    pub fn from_json(json: Value) -> Result<Self, Error> {
        let json: TokenizerJson = serde_json::from_value(json)?;
        Self::from_config(json)
    }

    /// Loads a tokenizer file using the requested JSON or `.tkz` sidecar mode.
    pub fn load_file(path: &Path, mode: LoadMode) -> Result<Self, Error> {
        TokenizerJson::load_file_with(path, mode, Self::from_config).map_err(|error| match error {
            json_structs::LoadError::Load(error) | json_structs::LoadError::Construct(error) => {
                error
            }
        })
    }

    /// Returns the configured normalizer, if the tokenizer has one.
    pub fn normalizer(&self) -> Option<&Normalizer> {
        self.normalizer.as_ref()
    }

    /// Returns the configured pre-tokenizer, if the tokenizer has one.
    pub fn pre_tokenizer(&self) -> Option<&PreTokenizer> {
        self.pre_tokenizer.as_ref()
    }

    /// Returns the configured post-processor, if the tokenizer has one.
    pub fn post_processor(&self) -> Option<&PostProcessor> {
        self.post_processor.as_ref()
    }

    /// Returns the configured model used to encode ordinary text.
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Returns the added-token table, if the tokenizer defines added tokens.
    pub fn added_tokens(&self) -> Option<&AddedTokens> {
        self.added_tokens.as_ref()
    }

    /// Returns the configured decoder, if the tokenizer has one.
    pub fn decoder(&self) -> Option<&Decoder> {
        self.decoder.as_ref()
    }

    /// Encodes one string, applying configured post-processor special tokens when requested.
    pub fn encode(&self, input: &str, add_special_tokens: bool) -> Result<Vec<u32>, Error> {
        self.encode_input(input, add_special_tokens, false)
    }

    /// Encode at most `max_tokens` content tokens and report whether any were discarded.
    ///
    /// Normalization and splitting still inspect the full input. Model merging stops
    /// after the retained prefix or suffix, while discarded pieces are validated.
    /// Special tokens are not inserted; callers must reserve their space separately.
    pub fn encode_with_limit(
        &self,
        input: &str,
        max_tokens: usize,
        direction: TruncationDirection,
    ) -> Result<(Vec<u32>, bool), Error> {
        // Match ordinary encoding: empty input bypasses normalization.
        if input.is_empty() {
            return Ok((Vec::new(), false));
        }
        let mut pts = self.build_pre_tokenized(input);
        if let Some(pre_tokenizer) = &self.pre_tokenizer {
            pre_tokenizer.pre_tokenize(&mut pts)?;
        }
        self.apply_vocab_splits(&mut pts);
        pts.tokenize_with_limit(
            max_tokens,
            direction,
            |text, ids| self.model.tokenize_into(text, ids),
            |text| self.model.validate_input(text),
        )
        .map_err(Error::Model)
    }

    /// Apply the same content-token limit to each input, preserving batch order.
    pub fn encode_batch_with_limit<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        max_tokens: usize,
        direction: TruncationDirection,
    ) -> Result<Vec<(Vec<u32>, bool)>, Error> {
        inputs
            .par_iter()
            .map(|input| self.encode_with_limit(input.as_ref(), max_tokens, direction))
            .collect()
    }

    /// Encodes one string through the configured normalizer, pre-tokenizer, and model.
    fn encode_input(
        &self,
        input: &str,
        add_special_tokens: bool,
        use_parallel_cache: bool,
    ) -> Result<Vec<u32>, Error> {
        if input.is_empty() {
            return if add_special_tokens {
                Ok(self.post_process(Vec::new(), true))
            } else {
                Ok(Vec::new())
            };
        }

        if let Some(ids) = self.try_encode_fused_bpe(input, use_parallel_cache)? {
            return Ok(self.post_process(ids, add_special_tokens));
        }

        let unigram = self.model.unigram();
        let fused_unigram = unigram.zip(
            self.pre_tokenizer
                .as_ref()
                .and_then(PreTokenizer::fused_whitespace_metaspace),
        );

        // Large eligible Unigram documents partition the raw text before
        // normalization so the charsmap, word walk, and Viterbi all run in
        // parallel; unchanged partitions borrow instead of copying.
        if let Some((unigram, metaspace)) = fused_unigram
            && input.len() >= pre_tokenized::PARALLEL_INPUT_BYTES
            && pre_tokenized::inner_parallelism_enabled()
            && !self
                .added_tokens
                .as_ref()
                .is_some_and(AddedTokens::has_normalized)
            && let Some(anchors) = match &self.normalizer {
                None => Some(&ASCII_WS_ANCHORS),
                Some(normalizer) => normalizer.partition_anchor_table(),
            }
        {
            let segments = self.segment_input(input);
            let ids = encode_metaspace_raw_partitions(
                unigram,
                metaspace,
                self.normalizer.as_ref(),
                &segments,
                anchors,
            )
            .map_err(Error::Model)?;
            return Ok(self.post_process(ids, add_special_tokens));
        }

        let mut pts = self.build_pre_tokenized(input);

        if let Some(ids) = self.encode_fused_bpe_pre_tokenized(&mut pts)? {
            return Ok(self.post_process(ids, add_special_tokens));
        }

        if let Some((unigram, metaspace)) = fused_unigram {
            // Large single documents run the fused word walk and Viterbi
            // together per whitespace-aligned partition; the serial fused
            // walker remains the exact path for everything below the gates.
            if pts.buffer().len() >= pre_tokenized::PARALLEL_INPUT_BYTES
                && pre_tokenized::inner_parallelism_enabled()
            {
                let ids = encode_metaspace_normalized_partitions(unigram, metaspace, &pts)
                    .map_err(Error::Model)?;
                return Ok(self.post_process(ids, add_special_tokens));
            }
            metaspace.pre_tokenize_after_whitespace(&mut pts);
        } else if let Some(ref pt) = self.pre_tokenizer {
            pt.pre_tokenize(&mut pts)?;
        }

        self.apply_vocab_splits(&mut pts);

        let ids = match &self.model {
            Model::Unigram(unigram) => pts.tokenize_batched(|buffer, splits, out| {
                unigram.append_split_viterbi_ids(buffer, splits, out)
            }),
            Model::Bpe(bpe) => pts.tokenize(|text, out| bpe.append_bpe_ids(text, out)),
        }
        .map_err(Error::Model)?;

        Ok(self.post_process(ids, add_special_tokens))
    }

    /// Encodes strings in input order, parallelizing substantial batches when useful.
    ///
    /// When `add_special_tokens` is `true`, applies configured post-processor special tokens to
    /// every input.
    pub fn encode_batch<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        add_special_tokens: bool,
    ) -> Result<Vec<Vec<u32>>, Error> {
        let total_bytes = inputs
            .iter()
            .map(|input| input.as_ref().len())
            .sum::<usize>();
        let parallel_bytes =
            PARALLEL_BATCH_BYTES_PER_THREAD.saturating_mul(rayon::current_num_threads());
        if total_bytes <= parallel_bytes
            && (inputs.len() < WIDE_BATCH_TASKS || !self.needs_vocab_splitting)
        {
            return inputs
                .iter()
                .map(|input| self.encode(input.as_ref(), add_special_tokens))
                .collect();
        }

        let outer_tasks = inputs.len().min(rayon::current_num_threads());
        if outer_tasks < 2 {
            return inputs
                .par_iter()
                .map(|input| self.encode(input.as_ref(), add_special_tokens))
                .collect();
        }

        self.encode_outer_batch(inputs, add_special_tokens, outer_tasks)
    }

    /// Encodes a batch into one ID buffer and one token length per input row.
    ///
    /// Concatenating slices of `ids` using `lengths` reconstructs the rows that
    /// [`Self::encode_batch`] would return in the same order, including configured special
    /// tokens when `add_special_tokens` is `true`.
    pub fn encode_batch_ragged<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        add_special_tokens: bool,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        if let Some(encoded) = self.try_encode_fused_bpe_ragged(inputs, add_special_tokens)? {
            return Ok(encoded);
        }

        let rows = self.encode_batch(inputs, add_special_tokens)?;
        let total = rows.iter().map(Vec::len).sum();
        let mut ids = Vec::with_capacity(total);
        let mut lengths = Vec::with_capacity(rows.len());
        for row in rows {
            lengths.push(row.len());
            ids.extend(row);
        }
        Ok((ids, lengths))
    }

    #[inline(never)]
    fn encode_outer_batch<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        add_special_tokens: bool,
        outer_tasks: usize,
    ) -> Result<Vec<Vec<u32>>, Error> {
        let use_parallel_cache = inputs
            .iter()
            .map(|input| input.as_ref().len())
            .sum::<usize>()
            >= PARALLEL_CACHE_BATCH_BYTES;
        inputs
            .par_iter()
            .map(|input| {
                let input = input.as_ref();
                // Unigram inner work already uses `bpe_pool`, so keep inner
                // parallelism and the partitioned walker. BPE still avoids
                // nested split work.
                if (outer_tasks >= WIDE_BATCH_TASKS || input.len() <= SHORT_BATCH_INPUT_BYTES)
                    && self.model.unigram().is_none()
                {
                    pre_tokenized::without_inner_parallelism(|| {
                        self.encode_input(input, add_special_tokens, use_parallel_cache)
                    })
                } else {
                    self.encode_input(input, add_special_tokens, use_parallel_cache)
                }
            })
            .collect()
    }

    /// Replaces the post-processor used by encoding with special tokens.
    pub fn set_post_processor(&mut self, pp: Option<PostProcessor>) {
        self.post_processor = pp;
    }

    /// Replaces the normalizer applied before pre-tokenization.
    pub fn set_normalizer(&mut self, normalizer: Option<Normalizer>) -> Result<(), Error> {
        if let Some(added_tokens) = &mut self.added_tokens {
            added_tokens
                .set_normalizer(normalizer.as_ref())
                .map_err(Error::Model)?;
        }
        self.normalizer = normalizer;
        Ok(())
    }

    /// Applies the configured single-sequence post-processor to token IDs.
    pub fn post_process(&self, ids: Vec<u32>, add_special_tokens: bool) -> Vec<u32> {
        match &self.post_processor {
            Some(pp) => pp.post_process_single(ids, add_special_tokens),
            None => ids,
        }
    }

    /// Decodes token IDs into text.
    ///
    /// If `skip_special_tokens` is true, added tokens marked as special are
    /// omitted. Unknown IDs are ignored to match Hugging Face behavior.
    pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> Result<String, Error> {
        // Unknown IDs are ignored, and added-token spellings take precedence.
        let tokens = ids
            .iter()
            .copied()
            .filter(|&id| !skip_special_tokens || !self.is_special_token(id))
            .filter_map(|id| self.id_to_token(id));
        match &self.decoder {
            Some(decoder) => {
                let mut owned = Vec::with_capacity(ids.len());
                owned.extend(tokens.map(str::to_owned));
                decoder.decode(owned).map_err(Error::Decoder)
            }
            None => Ok(tokens.collect()),
        }
    }

    /// Decode a sequence of token strings back into text.
    ///
    /// Applies the decoder pipeline (e.g. ByteLevel → convert "Ġ" back to " ")
    /// without going through the ID→string lookup.  When no decoder is
    /// configured the tokens are concatenated with no separator.
    pub fn decode_tokens(&self, tokens: Vec<String>) -> Result<String, Error> {
        match &self.decoder {
            Some(dec) => dec.decode(tokens).map_err(Error::Decoder),
            None => Ok(decoders::join_tokens(tokens)),
        }
    }

    /// Decodes every ID row in order.
    pub fn decode_batch(
        &self,
        sentences: &[&[u32]],
        skip_special_tokens: bool,
    ) -> Result<Vec<String>, Error> {
        sentences
            .iter()
            .map(|ids| self.decode(ids, skip_special_tokens))
            .collect()
    }

    /// Returns the token text for an ID, including added tokens.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        if let Some(ref at) = self.added_tokens
            && let Some(s) = at.id_to_token(id)
        {
            return Some(s);
        }
        self.model.id_to_token(id)
    }

    /// Returns the ID for token text, checking added tokens before the model vocabulary.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        if let Some(ref at) = self.added_tokens
            && let Some(id) = at.token_to_id(token)
        {
            return Some(id);
        }
        self.model.token_to_id(token)
    }

    /// Returns the model vocabulary size plus the number of added tokens.
    pub fn vocab_size(&self) -> usize {
        let model_size = self.model.vocab_size();
        let added_size = self.added_tokens.as_ref().map_or(0, |at| at.len());
        model_size + added_size
    }

    /// Returns whether an added token ID is marked as special.
    pub fn is_special_token(&self, id: u32) -> bool {
        self.added_tokens
            .as_ref()
            .is_some_and(|added_tokens| added_tokens.is_special(id))
    }

    /// Applies added-token handling and normalization to construct an intermediate value.
    pub fn build_pre_tokenized(&self, input: &str) -> PreTokenizedString {
        let segments = self.segment_input(input);
        self.build_pre_tokenized_from_segments(input, &segments)
    }

    fn segment_input<'a>(&self, input: &'a str) -> Vec<Segment<'a>> {
        match &self.added_tokens {
            Some(at) => at.split_non_normalized(input),
            None => vec![Segment::Text(input)],
        }
    }

    /// Collects protected tokens and normalized text into the materialized buffer.
    fn build_pre_tokenized_from_segments(
        &self,
        input: &str,
        segments: &[Segment<'_>],
    ) -> PreTokenizedString {
        let normalized_added_tokens = self
            .added_tokens
            .as_ref()
            .filter(|added_tokens| added_tokens.has_normalized());

        if segments.len() == 1
            && let Segment::Text(text) = segments[0]
            && normalized_added_tokens.is_none()
        {
            let normalized = match &self.normalizer {
                Some(n) => n.normalize(text),
                None => std::borrow::Cow::Borrowed(text),
            };
            return match normalized {
                std::borrow::Cow::Borrowed(_) => PreTokenizedString::from_text(text),
                std::borrow::Cow::Owned(s) => {
                    let len = s.len();
                    PreTokenizedString::new(
                        s,
                        vec![PtSplit {
                            range: 0..len,
                            token_id: None,
                        }],
                    )
                }
            };
        }

        let mut buffer = String::with_capacity(input.len());
        let mut splits = Vec::new();

        let Ok(()) = self.for_each_normalized_segment(segments, |segment| {
            let start = buffer.len();
            match segment {
                Segment::Token(id) => {
                    splits.push(PtSplit {
                        range: start..start,
                        token_id: Some(id),
                    });
                }
                Segment::Text(text) => {
                    buffer.push_str(text);
                    splits.push(PtSplit {
                        range: start..buffer.len(),
                        token_id: None,
                    });
                }
            }
            Ok::<(), std::convert::Infallible>(())
        });

        PreTokenizedString::new(buffer, splits)
    }

    /// Emits protected tokens and normalized spans, stopping on the first callback error.
    fn for_each_normalized_segment<E>(
        &self,
        segments: &[Segment<'_>],
        mut emit: impl FnMut(Segment<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        let normalized_added_tokens = self
            .added_tokens
            .as_ref()
            .filter(|added_tokens| added_tokens.has_normalized());
        for segment in segments {
            match segment {
                Segment::Token(id) => emit(Segment::Token(*id))?,
                Segment::Text(text) => {
                    if text.is_empty() {
                        continue;
                    }
                    let normalized = self
                        .normalizer
                        .as_ref()
                        .map_or(Cow::Borrowed(*text), |normalizer| {
                            normalizer.normalize(text)
                        });
                    if let Some(added_tokens) = normalized_added_tokens {
                        for segment in added_tokens.split_normalized(&normalized) {
                            emit(segment)?;
                        }
                    } else {
                        emit(Segment::Text(&normalized))?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// ASCII whitespace bytes usable as partition anchors when no normalizer runs.
const ASCII_WS_ANCHORS: [bool; 128] = {
    let mut table = [false; 128];
    let mut byte = 0;
    while byte < 128 {
        table[byte] = (byte as u8).is_ascii_whitespace();
        byte += 1;
    }
    table
};

/// Bytes of fused word-walk-plus-Viterbi work per parallel partition.
const METASPACE_PARTITION_MIN_BYTES: usize = 16 * 1024;
const METASPACE_PARTITION_MAX_BYTES: usize = 128 * 1024;
const METASPACE_PARTITIONS_PER_WORKER: usize = 6;

/// Target bytes per parallel Unigram partition from pool width and buffer length.
fn metaspace_partition_target(len: usize) -> usize {
    let workers = pre_tokenized::bpe_pool().current_num_threads();
    (len / (workers * METASPACE_PARTITIONS_PER_WORKER).max(1))
        .clamp(METASPACE_PARTITION_MIN_BYTES, METASPACE_PARTITION_MAX_BYTES)
}

/// Cuts `text` at caller-proven offsets so each piece is about `target` bytes.
///
/// `is_safe_cut(cut)` must be true only when `cut` is a character boundary that
/// cannot divide a Metaspace word; the raw and normalized callers each supply
/// the predicate that matches their serial path.
fn push_text_partitions<'a>(
    text: &'a str,
    target: usize,
    mut is_safe_cut: impl FnMut(usize) -> bool,
    partitions: &mut Vec<Segment<'a>>,
) {
    let mut start = 0;
    while text.len() - start > target {
        let mut cut = start + target;
        while cut < text.len() && !is_safe_cut(cut) {
            cut += 1;
        }
        if cut >= text.len() {
            break;
        }
        partitions.push(Segment::Text(&text[start..cut]));
        start = cut;
    }
    if start < text.len() {
        partitions.push(Segment::Text(&text[start..]));
    }
}

/// Normalizes, word-walks, and Viterbi-encodes raw text partitions in parallel.
///
/// Callers prove eligibility: printable ASCII normalizes to itself and each
/// cut sits after an anchor-safe whitespace byte and before printable ASCII,
/// so per-partition normalization and word boundaries match the serial pass.
fn encode_metaspace_raw_partitions(
    unigram: &models::unigram::Unigram,
    metaspace: &pre_tokenizers::Metaspace,
    normalizer: Option<&Normalizer>,
    segments: &[Segment<'_>],
    anchors: &[bool; 128],
) -> Result<Vec<u32>, String> {
    let total_text: usize = segments
        .iter()
        .map(|segment| match segment {
            Segment::Text(text) => text.len(),
            Segment::Token(_) => 0,
        })
        .sum();
    let target = metaspace_partition_target(total_text);

    let mut partitions = Vec::with_capacity(total_text / target + segments.len() + 1);
    for segment in segments {
        match segment {
            Segment::Token(id) => partitions.push(Segment::Token(*id)),
            Segment::Text(text) => {
                let bytes = text.as_bytes();
                push_text_partitions(
                    text,
                    target,
                    |cut| {
                        bytes[cut - 1] < 0x80
                            && anchors[bytes[cut - 1] as usize]
                            && (0x20..0x7f).contains(&bytes[cut])
                    },
                    &mut partitions,
                );
            }
        }
    }

    encode_metaspace_segment_partitions(unigram, metaspace, normalizer, &partitions)
}

/// Runs the fused WhitespaceSplit+Metaspace word walk and per-piece Viterbi
/// in parallel over whitespace-aligned partitions of the normalized buffer.
///
/// A cut directly after an ASCII-whitespace byte is a character boundary that
/// never divides a word, and every Metaspace piece is a function of one word,
/// so concatenating per-partition IDs reproduces the serial fused result.
fn encode_metaspace_normalized_partitions(
    unigram: &models::unigram::Unigram,
    metaspace: &pre_tokenizers::Metaspace,
    pts: &PreTokenizedString,
) -> Result<Vec<u32>, String> {
    let buffer = pts.buffer();
    let target = metaspace_partition_target(buffer.len());

    let mut partitions = Vec::with_capacity(buffer.len() / target + pts.splits().len() + 1);
    for split in pts.splits() {
        if let Some(id) = split.token_id {
            partitions.push(Segment::Token(id));
            continue;
        }
        if split.range.is_empty() {
            continue;
        }
        let text = &buffer[split.range.clone()];
        let bytes = text.as_bytes();
        push_text_partitions(
            text,
            target,
            |cut| bytes[cut - 1].is_ascii_whitespace(),
            &mut partitions,
        );
    }

    encode_metaspace_segment_partitions(unigram, metaspace, None, &partitions)
}

/// Encodes already-cut Token/Text units; `normalizer` runs only on raw slices.
fn encode_metaspace_segment_partitions(
    unigram: &models::unigram::Unigram,
    metaspace: &pre_tokenizers::Metaspace,
    normalizer: Option<&Normalizer>,
    partitions: &[Segment<'_>],
) -> Result<Vec<u32>, String> {
    let chunks = pre_tokenized::bpe_pool().install(|| {
        partitions
            .par_iter()
            .map(|partition| match partition {
                Segment::Token(id) => Ok(vec![*id]),
                Segment::Text(slice) => {
                    let text = match normalizer {
                        Some(normalizer) => normalizer.normalize(slice),
                        None => Cow::Borrowed(*slice),
                    };
                    let mut ids = Vec::with_capacity(output_capacity(text.len()));
                    let mut word_scratch = String::new();
                    let mut viterbi = models::unigram::ViterbiScratch::default();
                    metaspace.for_each_word_piece(&text, &mut word_scratch, |piece| {
                        unigram.append_viterbi_ids(piece, &mut ids, &mut viterbi)
                    })?;
                    Ok(ids)
                }
            })
            .collect::<Result<Vec<Vec<u32>>, String>>()
    })?;

    let total: usize = chunks.iter().map(Vec::len).sum();
    let mut ids = Vec::with_capacity(total);
    for chunk in chunks {
        ids.extend(chunk);
    }
    Ok(ids)
}

/// Rejects the native SentencePiece protobuf boundary before attempting UTF-8 JSON parsing.
pub(crate) fn reject_native_sentencepiece_model(path: &Path) -> Result<(), Error> {
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("model"))
    {
        return Err(Error::Unsupported(
            "native SentencePiece .model files are not supported; export a compatible tokenizer.json"
                .into(),
        ));
    }
    Ok(())
}

/// Stateful incremental decoder that waits for valid UTF-8 before yielding text.
pub struct DecodeStream {
    skip_special_tokens: bool,
    ids: Vec<u32>,
    prefix: String,
    prefix_index: usize,
}

impl DecodeStream {
    /// Starts a stream, optionally seeded with already-decoded prompt IDs.
    pub fn new(ids: Vec<u32>, skip_special_tokens: bool) -> Self {
        Self {
            skip_special_tokens,
            ids,
            prefix: String::new(),
            prefix_index: 0,
        }
    }

    /// Appends IDs and returns newly completed text, if any.
    pub fn step(
        &mut self,
        tokenizer: &Tokenizer,
        token_ids: Vec<u32>,
    ) -> Result<Option<String>, String> {
        decode_stream_step(
            tokenizer,
            token_ids,
            self.skip_special_tokens,
            &mut self.ids,
            &mut self.prefix,
            &mut self.prefix_index,
        )
    }
}

/// Advances explicit incremental-decoder state, retaining incomplete UTF-8 until it resolves.
pub fn decode_stream_step(
    tokenizer: &Tokenizer,
    token_ids: Vec<u32>,
    skip_special_tokens: bool,
    ids: &mut Vec<u32>,
    prefix: &mut String,
    prefix_index: &mut usize,
) -> Result<Option<String>, String> {
    const REPLACEMENT: char = '\u{FFFD}';

    // If the prefix is empty but we already have buffered IDs (e.g. seeded
    // with prompt tokens), prime the prefix before adding the new token.
    if prefix.is_empty() && !ids.is_empty() {
        let s = tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| e.to_string())?;
        if !s.ends_with(REPLACEMENT) {
            *prefix = s;
            *prefix_index = ids.len();
        }
    }

    ids.extend(token_ids);

    let string = tokenizer
        .decode(ids, skip_special_tokens)
        .map_err(|e| e.to_string())?;

    if string.len() > prefix.len() && !string.ends_with(REPLACEMENT) {
        if !string.starts_with(prefix.as_str()) {
            return Err(format!(
                "Invalid prefix encountered while decoding stream. \
                 Expected prefix: '{}', Actual string: '{}'",
                prefix, string,
            ));
        }
        let new_text = string[prefix.len()..].to_string();
        drop(ids.drain(..*prefix_index));
        *prefix = tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| e.to_string())?;
        *prefix_index = ids.len();
        Ok(Some(new_text))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    fn tokenizer_config(fused: bool, normalized_token: bool, normalizer: Value) -> Value {
        let mut alphabet: Vec<_> = tokenizers::pre_tokenizers::byte_level::ByteLevel::alphabet()
            .into_iter()
            .collect();
        alphabet.sort_unstable();
        let vocab: serde_json::Map<_, _> = alphabet
            .into_iter()
            .enumerate()
            .map(|(id, character)| (character.to_string(), json!(id)))
            .collect();
        let mut added_tokens = vec![json!({
            "id": 256, "content": "[e\u{301}?]", "normalized": false,
            "single_word": false, "lstrip": false, "rstrip": false, "special": false
        })];
        if normalized_token {
            added_tokens.push(json!({
                "id": 257, "content": "e\u{301}!", "normalized": true,
                "single_word": false, "lstrip": false, "rstrip": false, "special": false
            }));
        }
        let split = json!({
            "type": "Split", "pattern": { "Regex": "\\s+|\\S+" },
            "behavior": "Isolated", "invert": false
        });
        let first = if fused {
            split
        } else {
            json!({ "type": "Sequence", "pretokenizers": [split] })
        };
        json!({
            "added_tokens": added_tokens,
            "normalizer": normalizer,
            "pre_tokenizer": {
                "type": "Sequence",
                "pretokenizers": [first, {
                    "type": "ByteLevel", "add_prefix_space": false,
                    "trim_offsets": false, "use_regex": false
                }]
            },
            "model": { "type": "BPE", "vocab": vocab, "merges": [] }
        })
    }

    fn assert_encodings_match(
        ours: &Tokenizer,
        reference: &tokenizers::Tokenizer,
        inputs: &[&str],
    ) {
        let expected: Vec<Vec<u32>> = inputs
            .iter()
            .map(|input| reference.encode(*input, false).unwrap().get_ids().to_vec())
            .collect();
        for (input, ids) in inputs.iter().zip(&expected) {
            assert_eq!(&ours.encode(input, false).unwrap(), ids, "{input:?}");
        }
        assert_eq!(ours.encode_batch(inputs, false).unwrap(), expected);
        let (ids, lengths) = ours.encode_batch_ragged(inputs, false).unwrap();
        assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
        assert_eq!(ids, expected.into_iter().flatten().collect::<Vec<_>>());
    }

    #[test]
    fn generic_and_fused_preparation_match_hugging_face() {
        let long = format!("[e\u{301}?]{}[e\u{301}?]", "e\u{301} 中 🦀 ".repeat(300));
        let inputs = [
            "",
            "plain text",
            "e\u{301}!",
            "é!",
            "[e\u{301}?]",
            "[é?]",
            "[e\u{301}?][e\u{301}?]",
            "e\u{301}![e\u{301}?]é!",
            "e\u{301} 中[e\u{301}?]🦀 e\u{301}",
            "erase[e\u{301}?]erase",
            long.as_str(),
        ];
        for normalizer in [
            Value::Null,
            json!({ "type": "NFC" }),
            json!({ "type": "Replace", "pattern": { "String": "erase" }, "content": "" }),
        ] {
            for normalized_token in [false, true] {
                for fused in [false, true] {
                    let value = tokenizer_config(fused, normalized_token, normalizer.clone());
                    let ours = Tokenizer::from_json(value.clone()).unwrap();
                    let reference =
                        tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&value).unwrap())
                            .unwrap();
                    assert_encodings_match(&ours, &reference, &inputs);
                }
            }
        }
    }

    #[test]
    fn normalizer_replacement_matches_fresh_construction() {
        let inputs = ["e\u{301}!", "é!", "[e\u{301}?]", "[é?]", "x", ""];
        for fused in [false, true] {
            let mut ours =
                Tokenizer::from_json(tokenizer_config(fused, true, Value::Null)).unwrap();
            for next in [
                json!({ "type": "NFC" }),
                json!({ "type": "Replace", "pattern": { "String": "!" }, "content": "?" }),
                Value::Null,
            ] {
                let normalizer: Option<NormalizerConfig> =
                    serde_json::from_value(next.clone()).unwrap();
                ours.set_normalizer(normalizer.map(Normalizer::from_config).transpose().unwrap())
                    .unwrap();
                let value = tokenizer_config(fused, true, next);
                let fresh = Tokenizer::from_json(value.clone()).unwrap();
                let reference =
                    tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&value).unwrap()).unwrap();
                assert_eq!(
                    ours.encode_batch(&inputs, false).unwrap(),
                    fresh.encode_batch(&inputs, false).unwrap()
                );
                assert_encodings_match(&ours, &reference, &inputs);
                assert_eq!(ours.encode("[e\u{301}?]", false).unwrap(), vec![256]);
            }
        }
    }

    #[test]
    fn failed_normalizer_replacement_preserves_state() {
        for fused in [false, true] {
            let mut ours =
                Tokenizer::from_json(tokenizer_config(fused, true, json!({ "type": "NFC" })))
                    .unwrap();
            let invalid = Normalizer::Replace(
                Replace::from_config(json!({ "String": "e\u{301}!" }), String::new()).unwrap(),
            );
            assert!(ours.set_normalizer(Some(invalid)).is_err());
            for input in ["é!", "e\u{301}!"] {
                assert_eq!(ours.encode(input, false).unwrap(), vec![257]);
            }
        }
    }
    #[test]
    fn limited_encoding_preserves_normalized_added_token_boundaries() {
        for fused in [false, true] {
            let config = tokenizer_config(fused, true, json!({"type": "NFC"}));
            let ours = Tokenizer::from_json(config.clone()).unwrap();
            let reference =
                tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config).unwrap()).unwrap();
            let inputs = ["", "a", "héllo e\u{301}! [e\u{301}?] 世界", "a\n b\n "];
            for direction in [TruncationDirection::Left, TruncationDirection::Right] {
                for limit in [0, 1, 2, 5, 100] {
                    let batch = ours
                        .encode_batch_with_limit(&inputs, limit, direction)
                        .unwrap();
                    for (input, actual) in inputs.iter().zip(batch) {
                        let full = reference.encode(*input, false).unwrap().get_ids().to_vec();
                        let ids = match direction {
                            TruncationDirection::Left => {
                                full[full.len().saturating_sub(limit)..].to_vec()
                            }
                            TruncationDirection::Right => full[..limit.min(full.len())].to_vec(),
                        };
                        assert_eq!(actual, (ids, full.len() > limit));
                        assert_eq!(
                            ours.encode_with_limit(input, limit, direction).unwrap(),
                            actual
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn limited_encoding_validates_discarded_characters() {
        let ours = Tokenizer::from_json(json!({
            "model": {"type": "BPE", "vocab": {"a":0}, "merges":[]},
            "pre_tokenizer": {"type":"Split", "pattern":{"String":" "}, "behavior":"Removed", "invert":false}
        })).unwrap();
        for direction in [TruncationDirection::Left, TruncationDirection::Right] {
            for input in ["a a z", "z a a"] {
                assert!(ours.encode_with_limit(input, 0, direction).is_err());
            }
            assert_eq!(
                ours.encode_with_limit("a  ", 1, direction).unwrap(),
                (vec![0], false)
            );
            assert_eq!(
                ours.encode_with_limit("   ", 0, direction).unwrap(),
                (vec![], false)
            );
        }
    }

    #[test]
    fn limited_empty_input_bypasses_normalization() {
        let ours = Tokenizer::from_json(json!({
            "model": {"type": "BPE", "vocab": {"a": 0}, "merges": []},
            "normalizer": {"type": "Replace", "pattern": {"String": ""}, "content": "a"}
        }))
        .unwrap();
        assert_eq!(ours.encode("", false).unwrap(), Vec::<u32>::new());
        for direction in [TruncationDirection::Left, TruncationDirection::Right] {
            for limit in [0, 1] {
                assert_eq!(
                    ours.encode_with_limit("", limit, direction).unwrap(),
                    (Vec::new(), false)
                );
            }
        }
    }

    #[test]
    fn byte_fallback_merge_crosses_unicode_boundary() {
        let tokenizer = Tokenizer::from_json(json!({
            "normalizer": null,
            "pre_tokenizer": null,
            "model": {
                "type": "BPE",
                "vocab": {
                    "<unk>": 0,
                    "<0xC3>": 1,
                    "<0xA9>": 2,
                    "<0xAA>": 3,
                    "<0xA9><0xC3>": 4
                },
                "merges": [["<0xA9>", "<0xC3>"]],
                "unk_token": "<unk>",
                "byte_fallback": true
            },
            "post_processor": null,
            "decoder": null
        }))
        .unwrap();

        assert_eq!(tokenizer.encode("éê", false).unwrap(), vec![1, 4, 3]);
        for limit in 0..=4 {
            let full = [1, 4, 3];
            assert_eq!(
                tokenizer
                    .encode_with_limit("éê", limit, TruncationDirection::Right)
                    .unwrap(),
                (full[..limit.min(3)].to_vec(), limit < 3)
            );
            assert_eq!(
                tokenizer
                    .encode_with_limit("éê", limit, TruncationDirection::Left)
                    .unwrap(),
                (full[3usize.saturating_sub(limit)..].to_vec(), limit < 3)
            );
        }
    }

    #[test]
    fn ignore_merges_preserves_piece_semantics() {
        let tokenizer = |ignore_merges| {
            Tokenizer::from_json(json!({
                "normalizer": null,
                "pre_tokenizer": null,
                "model": {
                    "type": "BPE",
                    "vocab": {"a": 0, "b": 1, "c": 2, "d": 3, "ab": 4, "abc": 5},
                    "merges": [],
                    "ignore_merges": ignore_merges
                },
                "post_processor": null,
                "decoder": null
            }))
            .unwrap()
        };

        let merged = tokenizer(false);
        assert_eq!(merged.encode("ab", false).unwrap(), vec![0, 1]);

        let ignored = tokenizer(true);
        assert_eq!(ignored.encode("abc", false).unwrap(), vec![5]);
        assert_eq!(ignored.encode("abd", false).unwrap(), vec![0, 1, 3]);
    }

    #[test]
    fn vocabulary_and_decode_accessors_preserve_added_tokens() {
        let tokenizer = Tokenizer::from_json(json!({
            "model": {"type": "BPE", "vocab": {"a":0, "b":1}, "merges":[]},
            "added_tokens": [
                {"id":2, "content":"<special>", "special":true},
                {"id":3, "content":"<plain>", "special":false}
            ],
            "decoder": {"type":"Fuse"}
        }))
        .unwrap();
        assert_eq!(tokenizer.vocab_size(), 4);
        for (id, text) in [(0, "a"), (1, "b"), (2, "<special>"), (3, "<plain>")] {
            assert_eq!(tokenizer.id_to_token(id), Some(text));
            assert_eq!(tokenizer.token_to_id(text), Some(id));
        }
        let added = tokenizer.added_tokens().unwrap();
        assert_eq!(added.token_to_id("<plain>"), Some(3));
        assert_eq!(added.id_to_token(3), Some("<plain>"));
        assert!(tokenizer.is_special_token(2));
        assert!(!tokenizer.is_special_token(3));
        assert_eq!(tokenizer.decode(&[u32::MAX], false).unwrap(), "");
        assert_eq!(
            tokenizer.decode(&[0, u32::MAX, 2, 1], false).unwrap(),
            "a<special>b"
        );
        assert_eq!(tokenizer.decode(&[0, u32::MAX, 2, 1], true).unwrap(), "ab");
        assert_eq!(
            tokenizer.decode_batch(&[&[], &[0, 1], &[2]], true).unwrap(),
            ["", "ab", ""]
        );
        assert_eq!(
            tokenizer
                .decode_tokens(vec!["a".into(), "b".into()])
                .unwrap(),
            "ab"
        );
    }

    #[test]
    fn post_processing_applies_only_when_requested() {
        for processor in [
            Value::Null,
            json!({
                "type":"TemplateProcessing", "single":[
                    {"SpecialToken":{"id":"<s>","type_id":0}},
                    {"Sequence":{"id":"A","type_id":0}}
                ], "special_tokens":{"<s>":{"id":"<s>","ids":[1],"tokens":["<s>"]}}
            }),
        ] {
            let has_processor = !processor.is_null();
            let tokenizer = Tokenizer::from_json(json!({
                "model":{"type":"BPE","vocab":{"a":0,"<s>":1},"merges":[]},
                "post_processor":processor
            }))
            .unwrap();
            for input in ["", "a"] {
                let raw = tokenizer.encode(input, false).unwrap();
                assert_eq!(tokenizer.post_process(raw.clone(), false), raw);
                let mut expected = if has_processor { vec![1] } else { vec![] };
                expected.extend(&raw);
                assert_eq!(tokenizer.post_process(raw, true), expected);
                assert_eq!(tokenizer.encode(input, true).unwrap(), expected);
            }
        }
    }
}
