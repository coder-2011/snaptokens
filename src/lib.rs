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
pub(crate) mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::{
        collections::HashMap,
        fs,
        path::PathBuf,
        sync::{Mutex, OnceLock},
    };

    struct HfFixture {
        model: &'static str,
        revision: &'static str,
        blake3: &'static str,
    }

    const HF_FIXTURES: &[HfFixture] = &[
        HfFixture {
            model: "Qwen/Qwen3-0.6B",
            revision: "c1899de289a04d12100db370d81485cdf75e47ca",
            blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
        },
        HfFixture {
            model: "Qwen/Qwen3.5-27B",
            revision: "fc05daec18b0a78c049392ed2e771dde82bdf654",
            blake3: "3fd498e7fff0650ca4cf62dccd4e0844ed5ad2f1abcee9adfda013fef06c4220",
        },
        HfFixture {
            model: "zai-org/GLM-4.7",
            revision: "602d01efcdd332c5238ca4bcede555defbe83eb7",
            blake3: "ce097ff5c5a465244c42eab34b0c09e298c17bebaab2fd6bef354d5bd501a683",
        },
        HfFixture {
            model: "deepseek-ai/DeepSeek-V3.2",
            revision: "a7e62ac04ecb2c0a54d736dc46601c5606cf10a6",
            blake3: "43340ce86c8386401d11d3a3fb5a1798ffa696befffc0592a160ea493df6f3ac",
        },
        HfFixture {
            model: "MiniMaxAI/MiniMax-M2.1",
            revision: "cd97f59135f37b2a6bf09356e485d5e4aeb7dc9c",
            blake3: "b5a4e31f8028abc9bf05a2b4049d26ef2b02c132414221f07985fe85abc4a95a",
        },
        HfFixture {
            model: "openai/gpt-oss-120b",
            revision: "b5c939de8f754692c1647ca79fbf85e8c1e70f8a",
            blake3: "0596814de1ca4de2980968802fdb4340e84e39f8ffa67467648e8e606680acb1",
        },
        HfFixture {
            model: "mistralai/Mistral-Nemo-Instruct-2407",
            revision: "04d8a90549d23fc6bd7f642064003592df51e9b3",
            blake3: "5309310c331b5fca7b15536ff8fb20fa1602493f58cfb9aba4d3508d97af2f98",
        },
        HfFixture {
            model: "mistralai/Mistral-Large-3-675B-Instruct-2512",
            revision: "383ffea2c7d60dfd44ca960e8e691709d4fdb9cd",
            blake3: "3d69466e68ef8a541dba19ef2761a1c2e9eb504fa9517607d48984daedbe5887",
        },
        HfFixture {
            model: "Qwen/Qwen3-235B-A22B-Instruct-2507",
            revision: "ac9c66cc9b46af7306746a9250f23d47083d689e",
            blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
        },
        HfFixture {
            model: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
            revision: "9d90cf8fca1bf7b7acca42d3fc9ae694a2194069",
            blake3: "e4bf149b34e0372424d0cd7afba11bbda682d8a3bb29b95a02a7c97e89dc63a2",
        },
        HfFixture {
            model: "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
            revision: "bf77c3174f68ad409e1c2aa60daeb46e32d1c606",
            blake3: "90962b12be7538b0a19316f293747faab874ec037a9d632a5eb3c72456581dca",
        },
        HfFixture {
            model: "nvidia/Qwen3-Nemotron-235B-A22B-GenRM",
            revision: "b127d6abced0dd19b8d1f9678a881bfd30380d4b",
            blake3: "b0cb923fc505fdf0a53f0287654fa26577d3f333d4134350da0a97664b228739",
        },
        HfFixture {
            model: "hoangquan456/Kimi-K2.5",
            revision: "cb77384a429dcec7a14359e25ccdb1b7eb61a6b1",
            blake3: "5ebdd385ae20df5aeb5d6b49ab5e3add2093bc17073a636e9b6c125319d6ee7c",
        },
        HfFixture {
            model: "openai-community/gpt2",
            revision: "607a30d783dfa663caf39e06633721c8d4cfcd7e",
            blake3: "1f9b61de3382db2e111c702730ef4ad5b12788d3c040db87936da6c7f988f861",
        },
        HfFixture {
            model: "google-t5/t5-small",
            revision: "df1b051c49625cf57a3d0d8d3863ed4d13564fe4",
            blake3: "9ce04442387ff177fd52c14e1bd1901ba563d71e956ca2cffd39eeb3ef4bb820",
        },
        HfFixture {
            model: "google/gemma-3-1b-it",
            revision: "dcc83ea841ab6100d6b47a070329e1ba4cf78752",
            blake3: "38e6c65074653102e6e238195e25938a4cb1ea2df4c7c01283d45de480696a11",
        },
    ];

    pub(crate) const HF_MODELS: &[&str] = &[
        "Qwen/Qwen3-0.6B",
        "zai-org/GLM-4.7",
        "deepseek-ai/DeepSeek-V3.2",
        "MiniMaxAI/MiniMax-M2.1",
        "openai/gpt-oss-120b",
        "mistralai/Mistral-Nemo-Instruct-2407",
        "Qwen/Qwen3-235B-A22B-Instruct-2507",
        "Qwen/Qwen3-Coder-480B-A35B-Instruct",
        "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
        "nvidia/Qwen3-Nemotron-235B-A22B-GenRM",
        "hoangquan456/Kimi-K2.5",
    ];

    pub(crate) fn tokenizer_json_path(model: &str) -> anyhow::Result<PathBuf> {
        static VERIFIED: OnceLock<Mutex<HashMap<&'static str, PathBuf>>> = OnceLock::new();
        let fixture = HF_FIXTURES
            .iter()
            .find(|fixture| fixture.model == model)
            .ok_or_else(|| anyhow::anyhow!("missing pinned fixture for {model}"))?;
        let verified = VERIFIED.get_or_init(|| Mutex::new(HashMap::new()));
        if let Some(path) = verified.lock().unwrap().get(model).cloned() {
            return Ok(path);
        }
        let api = hf_hub::api::sync::Api::new()?;
        let repo = hf_hub::Repo::with_revision(
            model.to_string(),
            hf_hub::RepoType::Model,
            fixture.revision.to_string(),
        );
        let path = api.repo(repo).get("tokenizer.json")?;
        let actual = blake3::hash(&fs::read(&path)?).to_hex().to_string();
        anyhow::ensure!(
            actual == fixture.blake3,
            "{model}@{} tokenizer.json hash mismatch: expected {}, got {actual}",
            fixture.revision,
            fixture.blake3,
        );
        verified.lock().unwrap().insert(fixture.model, path.clone());
        Ok(path)
    }

    pub(crate) fn load_tokenizer(model: &str) -> anyhow::Result<Tokenizer> {
        let path = tokenizer_json_path(model)?;
        Ok(Tokenizer::load_file(&path, LoadMode::JsonOnly)?)
    }

    pub(crate) fn load_reference_tokenizer(model: &str) -> anyhow::Result<tokenizers::Tokenizer> {
        let path = tokenizer_json_path(model)?;
        tokenizers::Tokenizer::from_file(path).map_err(|error| anyhow::anyhow!(error))
    }

    /// One oracle for scalar, nested, and ragged APIs, including warm cache calls.
    pub(crate) struct Comparison {
        model: String,
        ours: Tokenizer,
        reference: tokenizers::Tokenizer,
    }

    impl Comparison {
        pub(crate) fn new(model: &str) -> Self {
            Self {
                model: model.into(),
                ours: load_tokenizer(model).unwrap_or_else(|error| panic!("{model}: {error}")),
                reference: load_reference_tokenizer(model)
                    .unwrap_or_else(|error| panic!("{model}: {error}")),
            }
        }

        pub(crate) fn assert_parity(&self, inputs: &[impl AsRef<str> + Sync], special: bool) {
            let expected: Vec<_> = inputs
                .iter()
                .map(|input| {
                    self.reference
                        .encode(input.as_ref(), special)
                        .unwrap()
                        .get_ids()
                        .to_vec()
                })
                .collect();
            for (input, ids) in inputs.iter().zip(&expected) {
                let input = input.as_ref();
                for _ in 0..2 {
                    assert_eq!(
                        self.ours.encode(input, special).unwrap(),
                        *ids,
                        "{} scalar special={special} input={input:?}",
                        self.model
                    );
                }
                for skip in [false, true] {
                    assert_eq!(
                        self.ours.decode(ids, skip).unwrap(),
                        self.reference.decode(ids, skip).unwrap(),
                        "{} decode skip={skip} input={input:?}",
                        self.model
                    );
                }
            }
            assert_eq!(
                self.ours.encode_batch(inputs, special).unwrap(),
                expected,
                "{} batch",
                self.model
            );
            let (ids, lengths) = self.ours.encode_batch_ragged(inputs, special).unwrap();
            assert_eq!(
                lengths,
                expected.iter().map(Vec::len).collect::<Vec<_>>(),
                "{} row lengths",
                self.model
            );
            assert_eq!(
                ids,
                expected.into_iter().flatten().collect::<Vec<_>>(),
                "{} ragged IDs",
                self.model
            );
        }
    }

    pub(crate) const CORPUS: &[&str] = &[
        "",
        " ",
        "  ",
        "\n",
        "\t",
        "\r\n",
        "a",
        "Z",
        "0",
        "!",
        "\u{00e9}",
        "\u{4e2d}",
        "Hello, world!",
        "The quick brown fox jumps over the lazy dog.",
        "A short sentence.",
        "  leading spaces",
        "trailing spaces  ",
        "  both  sides  ",
        "multiple    internal    spaces",
        "tabs\there\tand\tthere",
        "line\none\nline\ntwo",
        "windows\r\nline\r\nendings",
        "mixed\n\ttabs and\r\nnewlines  with  spaces",
        "42",
        "3.14159",
        "1,000,000",
        "0xFF",
        "1e-10",
        "Numbers 1234567890 and mixed ABC123def",
        "Hello!!! How are you???",
        "@user #hashtag $100 %50 ^caret &amp *star",
        "a-b_c.d,e;f:g",
        "(parentheses) [brackets] {braces}",
        "\"double quotes\" 'single quotes' `backticks`",
        "path/to/file.txt",
        "https://example.com/path?q=test&lang=en#section",
        "Special chars: @#$%^&*()_+-=[]{}|;':\",./<>?",
        "caf\u{00e9} r\u{00e9}sum\u{00e9} na\u{00ef}ve",
        "\u{00fc}ber stra\u{00df}e gr\u{00f6}\u{00df}e",
        "se\u{00f1}or ni\u{00f1}o a\u{00f1}o",
        "\u{4f60}\u{597d}\u{4e16}\u{754c}",
        "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}",
        "\u{c548}\u{b155}\u{d558}\u{c138}\u{c694}",
        "\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442} \u{043c}\u{0438}\u{0440}",
        "\u{0645}\u{0631}\u{062d}\u{0628}\u{0627}",
        "\u{0928}\u{092e}\u{0938}\u{094d}\u{0924}\u{0947}",
        "\u{1f600}\u{1f680}\u{2764}\u{fe0f}",
        "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}\u{200d}\u{1f466}",
        "\u{1f1fa}\u{1f1f8}",
        "e\u{0301}",
        "n\u{0303}",
        "a\u{0308}",
        "Hello \u{4e16}\u{754c} \u{041c}\u{0438}\u{0440}!",
        "User123 wrote: \u{4f60}\u{597d}!",
        "fn main() { println!(\"hello\"); }",
        "def foo(x: int) -> str:\n    return str(x)",
        "SELECT * FROM users WHERE id = 1;",
        "if (x > 0 && y < 10) { z = x + y; }",
        "<html><body><p>Hello</p></body></html>",
        "#include <stdio.h>\nint main() { return 0; }",
        "import numpy as np\nx = np.array([1, 2, 3])",
        "{\"key\": \"value\", \"number\": 42, \"array\": [1, 2, 3]}",
        "[{\"id\": 1}, {\"id\": 2}]",
        "aaaaaaaaaa",
        "abababababababab",
        "the the the the the the the the",
        "....",
        "----",
        "    ",
        "\n\n\n\n",
        "This is a longer sentence with various elements: numbers (42, 3.14), \
             symbols (@#$), Unicode (caf\u{00e9}, \u{4f60}\u{597d}), and more.",
        "The year 2024 was notable for advances in AI. Models like GPT-4 and \
             Claude demonstrated remarkable capabilities in reasoning, coding, and \
             multilingual understanding.",
        "a b c d e f g h i j k l m n o p q r s t u v w x y z",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "0123456789",
        "a\nb\nc\n",
        "# Heading\n\n- item 1\n- item 2\n\n```code```",
        "\u{ffff}",
        "\u{0080}",
        "\u{07ff}",
        "\u{0800}",
        "\u{10000}",
        "\u{fffd}",
        "\u{feff}Hello",
        "\u{0000}",
        "abc\u{0000}def",
        "\u{fffe}",
        "\u{fdd0}",
        "\u{200b}\u{200c}\u{200d}",
        "\u{202e}Hello\u{202c}",
        "\u{0001}\u{0002}\u{001f}\u{007f}",
        "\u{0300}",
        "a\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}",
        "\u{e000}\u{f8ff}",
        "\u{01c5}\u{01c8}\u{01cb}",
        "\u{2028}\u{2029}",
        "\u{fff9}\u{fffa}\u{fffb}",
        "\u{d7ff}\u{10ffff}",
        "ab",
        "abc",
        "abcd",
        "aaa",
        "aaaa",
        "aaaaa",
        "**bold** *italic* ~~strikethrough~~ __underline__",
        "```rust\nfn main() {}\n```",
        "> blockquote\n>> nested",
        "| col1 | col2 |\n|------|------|\n| a    | b    |",
    ];

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

    /// Cover ordinary, fused, and nested normalized/added-token error paths.
    #[test]
    fn regex_matching_errors_reach_encode_callers() {
        let split = json!({
            "type":"Split", "pattern":{"Regex":r"z|(?i)(a|b|ab)*(?>c)|a"},
            "behavior":"Isolated", "invert":false
        });
        let byte_level = json!({
            "type":"ByteLevel", "add_prefix_space":false, "use_regex":false
        });
        let first_split = json!({
            "type":"Split", "pattern":{"String":"|"}, "behavior":"Isolated", "invert":false
        });
        for (pre_tokenizer, prefix) in [
            (split.clone(), ""),
            (
                json!({"type":"Sequence", "pretokenizers":[split, byte_level]}),
                "",
            ),
            (
                json!({"type":"Sequence", "pretokenizers":[first_split, split, byte_level]}),
                "<s>",
            ),
        ] {
            let tokenizer = Tokenizer::from_json(json!({
                "model":{"type":"BPE", "vocab":{"a":0,"b":1,"ab":2,"z":3}, "merges":[["a","b"]]},
                "pre_tokenizer":pre_tokenizer,
                "normalizer":{"type":"Replace", "pattern":{"String":"A"}, "content":"a"},
                "added_tokens":[{"id":4,"content":"<s>","special":true,"normalized":false}]
            }))
            .unwrap();
            let input = format!("{prefix}z{}", "Ab".repeat(20));
            for error in [
                tokenizer.encode(&input, false).unwrap_err(),
                tokenizer
                    .encode_batch_ragged(&["zab", &input], false)
                    .unwrap_err(),
            ] {
                assert!(
                    matches!(error, Error::PreTokenizer(pre_tokenizers::Error::Regex(_))),
                    "{error}"
                );
            }
            assert_eq!(tokenizer.encode("zab", false).unwrap(), [3, 0, 1]);
        }
    }

    /// A scan failure must surface even when draining pending BPE work also
    /// fails, matching the error the unfused pipeline reports.
    #[test]
    fn fused_regex_error_precedes_model_error() {
        let tokenizer = Tokenizer::from_json(json!({
            "model":{"type":"BPE", "vocab":{"a":0,"b":1,"ab":2}, "merges":[["a","b"]]},
            "pre_tokenizer":{"type":"Sequence", "pretokenizers":[
                {"type":"Split", "pattern":{"Regex":r"z|(?i)(a|b|ab)*(?>c)|a"},
                 "behavior":"Isolated", "invert":false},
                {"type":"ByteLevel", "add_prefix_space":false, "use_regex":false}
            ]}
        }))
        .unwrap();
        // The matched z is queued before the remaining input exceeds the regex limit.
        let input = format!("z{}", "ab".repeat(20));
        for error in [
            tokenizer.encode(&input, false).unwrap_err(),
            tokenizer.encode_batch_ragged(&[&input], false).unwrap_err(),
        ] {
            assert!(
                matches!(error, Error::PreTokenizer(pre_tokenizers::Error::Regex(_))),
                "{error}"
            );
        }
    }

    #[test]
    fn encode_batch_matches_sequential() {
        let model = "MiniMaxAI/MiniMax-M2.1";
        let ours = load_tokenizer(model).unwrap();

        let inputs = &["Hello, world!", "The quick brown fox", "Test", ""];
        let batch_results = ours.encode_batch(inputs, false).unwrap();

        for (input, batch_result) in inputs.iter().zip(&batch_results) {
            let sequential_result = ours.encode(input, false).unwrap();
            assert_eq!(
                batch_result, &sequential_result,
                "batch mismatch for {input:?}"
            );
        }
    }

    #[test]
    fn encode_batch_ragged_matches_nested() {
        let ours = load_tokenizer("openai-community/gpt2").unwrap();
        let mut inputs = vec!["Hello, world! ".repeat(32); 64];
        inputs[1] = "<|endoftext|>".into();
        inputs[2] = String::new();
        let expected = ours.encode_batch(&inputs, false).unwrap();
        let (actual_ids, actual_lengths) = ours.encode_batch_ragged(&inputs, false).unwrap();

        let expected_lengths = expected.iter().map(Vec::len).collect::<Vec<_>>();
        let expected_ids = expected.into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(actual_lengths, expected_lengths);
        assert_eq!(actual_ids, expected_ids);
    }

    #[test]
    fn batch_special_tokens_match_scalar_and_ragged() {
        let tokenizer = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let inputs = ["hello world", "", "second input"];
        let expected = inputs
            .iter()
            .map(|input| tokenizer.encode(input, true).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(tokenizer.encode_batch(&inputs, true).unwrap(), expected);

        let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, true).unwrap();
        assert_eq!(lengths, expected.iter().map(Vec::len).collect::<Vec<_>>());
        assert_eq!(ids, expected.into_iter().flatten().collect::<Vec<_>>());
    }

    #[test]
    fn newline_partitioned_ragged_matches_sequential() {
        let ours = load_tokenizer("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").unwrap();
        for input in [
            "Text and numbers 123. Unicode: café, 你好. Punctuation?!\r\n\n".repeat(1_200),
            "one long line with no partition boundary ".repeat(2_000),
        ] {
            let expected = ours.encode(&input, false).unwrap();
            let (actual, lengths) = ours.encode_batch_ragged(&[input.as_str()], false).unwrap();
            assert_eq!(lengths, [expected.len()]);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn add_bos_token() {
        let tok = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let bos_id = tok.token_to_id("<s>").expect("<s> not in vocabulary");

        let with_bos = tok.encode("hello world", true).unwrap();
        let without_bos = tok.encode("hello world", false).unwrap();

        assert_eq!(
            with_bos.first().copied(),
            Some(bos_id),
            "first token should be BOS when add_special_tokens=true"
        );
        assert_ne!(
            without_bos.first().copied(),
            Some(bos_id),
            "BOS should be absent when add_special_tokens=false"
        );
        assert_eq!(&with_bos[1..], without_bos.as_slice());

        let tok_q = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let with_flag = tok_q.encode("hello world", true).unwrap();
        let without_flag = tok_q.encode("hello world", false).unwrap();
        assert_eq!(
            with_flag, without_flag,
            "Qwen3 has no BOS post-processor — add_special_tokens should have no effect"
        );
    }

    #[test]
    fn decode_skip_special_tokens() {
        let model = "mistralai/Mistral-Nemo-Instruct-2407";
        let tok = load_tokenizer(model).unwrap();
        let text = "hello world";
        let ids_with = tok.encode(text, true).unwrap();
        let ids_without = tok.encode(text, false).unwrap();
        assert!(
            ids_with.len() > ids_without.len(),
            "expected BOS/EOS from {model}"
        );

        let skipped = tok.decode(&ids_with, true).unwrap();
        assert_eq!(skipped, text);

        let full = tok.decode(&ids_with, false).unwrap();
        assert_ne!(full, text);
        assert!(full.contains(text));
    }

    #[test]
    fn decode_tokens_matches_decode_by_id() {
        let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in &["Hello, world!", "The quick brown fox", "🌍 emoji"] {
            let ids = tok.encode(text, false).unwrap();
            let token_strings: Vec<String> = ids
                .iter()
                .map(|&id| tok.id_to_token(id).unwrap().to_string())
                .collect();
            let via_ids = tok.decode(&ids, false).unwrap();
            let via_tokens = tok.decode_tokens(token_strings).unwrap();
            assert_eq!(via_ids, via_tokens, "mismatch for {text:?}");
        }
    }

    #[test]
    fn encode_is_stable_after_decode() {
        let tok = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in &["hello world", "日本語テスト", "fn foo() {}"] {
            let ids1 = tok.encode(text, false).unwrap();
            let decoded = tok.decode(&ids1, false).unwrap();
            let ids2 = tok.encode(&decoded, false).unwrap();
            assert_eq!(ids1, ids2, "encode not stable after decode for {text:?}");
        }
    }

    #[test]
    fn correctness_gpt2() {
        Comparison::new("openai-community/gpt2").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_minimax_m2_1() {
        Comparison::new("MiniMaxAI/MiniMax-M2.1").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_nemotron() {
        Comparison::new("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_deepseek_v3_2() {
        Comparison::new("deepseek-ai/DeepSeek-V3.2").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_gpt_oss() {
        Comparison::new("openai/gpt-oss-120b").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_qwen3() {
        Comparison::new("Qwen/Qwen3-0.6B").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_mistral_nemo() {
        Comparison::new("mistralai/Mistral-Nemo-Instruct-2407").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_qwen3_nemotron() {
        Comparison::new("nvidia/Qwen3-Nemotron-235B-A22B-GenRM").assert_parity(CORPUS, false);
    }

    #[test]
    fn correctness_kimi_k2_5() {
        Comparison::new("hoangquan456/Kimi-K2.5").assert_parity(CORPUS, false);
    }

    #[test]
    fn model_vocabulary_and_roundtrips() {
        let texts = [
            "Hello, world!",
            "日本語テスト",
            "The quick brown fox jumps over the lazy dog.",
            "fn main() { println!(\"hello\"); }",
            "   leading and trailing spaces   ",
            "line1\nline2\ttabbed",
            "0123456789",
            "🌍🎉✨",
        ];
        for model in HF_MODELS {
            let tokenizer = load_tokenizer(model).unwrap();
            assert!(tokenizer.vocab_size() > 0, "{model}");
            for text in texts {
                let ids = tokenizer.encode(text, false).unwrap();
                assert_eq!(tokenizer.decode(&ids, false).unwrap(), text, "{model}");
            }
            for id in [0, 1, 2, 100, 1000, 10_000] {
                let token = tokenizer.id_to_token(id).expect(model);
                assert_eq!(tokenizer.token_to_id(token), Some(id), "{model} id={id}");
            }
            let payload = vec![100, 200, 300];
            assert_eq!(
                tokenizer.post_process(payload.clone(), false),
                payload,
                "{model}"
            );
        }
    }

    #[test]
    fn long_inputs_match_hugging_face() {
        let input = "The quick brown fox jumps over the lazy dog. Numbers: 42, 3.14, 1000. Code: fn main() {} Unicode: café, 你好. Special: @#$%^&*(). ".repeat(100);
        assert!(input.len() > 8000);
        for model in [
            "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
            "MiniMaxAI/MiniMax-M2.1",
        ] {
            Comparison::new(model).assert_parity(&[&input], false);
        }
    }

    #[test]
    fn cache_consistency() {
        let model = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
        let ours = load_tokenizer(model).unwrap();

        let inputs = &[
            "Hello, world!",
            "The quick brown fox jumps over the lazy dog.",
            "caf\u{00e9} r\u{00e9}sum\u{00e9}",
            "\u{4f60}\u{597d}\u{4e16}\u{754c}",
            "fn main() { println!(\"hello\"); }",
            "a b c d e f g h i j k l m n o p",
            "aaaaaaaaaa bbbbbbbbbb cccccccccc",
        ];

        for &input in inputs {
            let first = ours.encode(input, false).unwrap();
            let second = ours.encode(input, false).unwrap();
            assert_eq!(first, second, "cache inconsistency for {input:?}");
            let third = ours.encode(input, false).unwrap();
            assert_eq!(first, third, "cache inconsistency (3rd call) for {input:?}");
        }
    }

    #[test]
    fn cache_consistency_byte_level() {
        let model = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16";
        let ours = load_tokenizer(model).unwrap();

        let input = "The year 2024 was notable for advances in AI. Models like \
                          GPT-4 and Claude demonstrated remarkable capabilities.";
        let baseline = ours.encode(input, false).unwrap();
        for i in 0..20 {
            let result = ours.encode(input, false).unwrap();
            assert_eq!(result, baseline, "byte-level cache drift on iteration {i}");
        }
    }

    #[test]
    fn limited_encoding_matches_full_prefixes_and_suffixes() {
        use TruncationDirection::{Left, Right};
        let text = "Hello 世界! café 12345\n<|endoftext|> repeated words ".repeat(64);
        for model in [
            "openai-community/gpt2",
            "Qwen/Qwen3-0.6B",
            "zai-org/GLM-4.7",
            "mistralai/Mistral-Nemo-Instruct-2407",
        ] {
            let ours = load_tokenizer(model).unwrap();
            let reference = load_reference_tokenizer(model).unwrap();
            let full = reference
                .encode(text.as_str(), false)
                .unwrap()
                .get_ids()
                .to_vec();
            assert_eq!(ours.encode(&text, false).unwrap(), full);
            for direction in [Left, Right] {
                for limit in [0, 1, 7, full.len(), full.len() + 1] {
                    let (ids, truncated) = ours.encode_with_limit(&text, limit, direction).unwrap();
                    let expected = match direction {
                        Left => &full[full.len().saturating_sub(limit)..],
                        Right => &full[..limit.min(full.len())],
                    };
                    assert_eq!(ids, expected, "{model}: {direction:?} {limit}");
                    assert_eq!(truncated, limit < full.len());
                }
            }
        }
    }

    #[test]
    fn streaming_reconstructs_text_for_single_and_batched_steps() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        for text in [
            "",
            "hello",
            "Hello, world! This is a streaming decode test.",
            "日本語テスト: こんにちは 🌍 — привет мир",
            r#"fn main() { println!("hello"); }"#,
            "The quick brown fox jumps over the lazy dog.",
            "one two three four five six seven eight nine ten",
        ] {
            let ids = tokenizer.encode(text, false).unwrap();
            for chunk_size in [1, 3, ids.len().max(1)] {
                let mut stream = DecodeStream::new(vec![], false);
                let mut output = String::new();
                for chunk in ids.chunks(chunk_size) {
                    if let Some(piece) = stream.step(&tokenizer, chunk.to_vec()).unwrap() {
                        assert!(!piece.is_empty());
                        output.push_str(&piece);
                    }
                }
                assert_eq!(output, text, "chunk_size={chunk_size}");
            }
        }
    }

    #[test]
    fn seeded_stream_emits_only_the_continuation() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let prompt = tokenizer.encode("The capital of France is", false).unwrap();
        let continuation = tokenizer.encode(" Paris.", false).unwrap();
        let mut stream = DecodeStream::new(prompt, false);
        let output: String = continuation
            .into_iter()
            .filter_map(|id| stream.step(&tokenizer, vec![id]).unwrap())
            .collect();
        assert_eq!(output, " Paris.");
    }

    #[test]
    fn streaming_skips_special_and_unknown_ids() {
        let tokenizer = load_tokenizer("mistralai/Mistral-Nemo-Instruct-2407").unwrap();
        let mut ids = tokenizer.encode("hello", true).unwrap();
        ids.insert(1, u32::MAX);
        for skip in [false, true] {
            let mut stream = DecodeStream::new(vec![], skip);
            assert_eq!(stream.step(&tokenizer, vec![u32::MAX]).unwrap(), None);
            assert_eq!(
                stream.step(&tokenizer, ids.clone()).unwrap(),
                Some(tokenizer.decode(&ids, skip).unwrap())
            );
        }
    }

    #[test]
    fn stream_state_remains_bounded_and_rejects_invalid_prefix() {
        let tokenizer = load_tokenizer("Qwen/Qwen3-0.6B").unwrap();
        let ids = tokenizer.encode("word ".repeat(80).trim(), false).unwrap();
        let (mut buffer, mut prefix, mut index) = (vec![], String::new(), 0);
        for id in ids {
            decode_stream_step(
                &tokenizer,
                vec![id],
                false,
                &mut buffer,
                &mut prefix,
                &mut index,
            )
            .unwrap();
        }
        assert!(buffer.len() < 10, "buffer grew to {} entries", buffer.len());
        // The mismatch check requires decoded text longer than the supplied prefix.
        let mut prefix = "Z".to_owned();
        let error = decode_stream_step(
            &tokenizer,
            tokenizer.encode("hello", false).unwrap(),
            false,
            &mut vec![],
            &mut prefix,
            &mut 0,
        )
        .unwrap_err();
        assert!(error.starts_with("Invalid prefix encountered"), "{error}");
    }

    const LONG_BENCH_V2_REVISION: &str = "2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9";

    const GEMMA_LONG_BENCH_INPUT_BLAKE3: &str =
        "23cf94a05e536b67d180de21be65ee2e9753dcc999cabf48da7e434177828379";

    /// Loads the exact LongBench context that exposed the Gemma pipeline mismatch.
    fn gemma_longbench_input() -> anyhow::Result<String> {
        let api = hf_hub::api::sync::Api::new()?;
        let repo = hf_hub::Repo::with_revision(
            "zai-org/LongBench-v2".to_string(),
            hf_hub::RepoType::Dataset,
            LONG_BENCH_V2_REVISION.to_string(),
        );
        let path = api.repo(repo).get("data.json")?;
        let data: Vec<serde_json::Value> = serde_json::from_slice(&fs::read(path)?)?;
        let input = data
            .get(10)
            .and_then(|item| item.get("context"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("LongBench-v2 input 10 has no context"))?
            .to_owned();
        let digest = blake3::hash(input.as_bytes()).to_hex().to_string();
        anyhow::ensure!(
            digest == GEMMA_LONG_BENCH_INPUT_BLAKE3,
            "LongBench-v2 input 10 BLAKE3 mismatch: expected {GEMMA_LONG_BENCH_INPUT_BLAKE3}, got {digest}"
        );
        Ok(input)
    }

    // Runs only with authorized Gemma access because it downloads the pinned 465 MB LongBench fixture.
    #[test]
    #[ignore = "requires authorized Gemma access and downloads the pinned LongBench-v2 fixture"]
    fn gemma_longbench_input_matches_hugging_face() {
        let model = "google/gemma-3-1b-it";
        let hf = load_reference_tokenizer(model).unwrap();
        let ours = load_tokenizer(model).unwrap();
        let input = gemma_longbench_input().unwrap();

        for add_special_tokens in [false, true] {
            let expected = hf
                .encode(input.as_str(), add_special_tokens)
                .unwrap()
                .get_ids()
                .to_vec();
            let actual = ours.encode(&input, add_special_tokens).unwrap();
            let first_difference =
                std::iter::zip(&expected, &actual).position(|(left, right)| left != right);
            assert!(
                actual == expected,
                "{model} LongBench input 10 add_special_tokens={add_special_tokens}: \
                 expected {} IDs, got {}; first differing position: {first_difference:?}",
                expected.len(),
                actual.len(),
            );
        }
    }

    struct ExtendedCorpus {
        longbench: Vec<String>,
        sharegpt: Vec<String>,
    }

    fn extended_corpus() -> &'static ExtendedCorpus {
        static CORPUS: OnceLock<ExtendedCorpus> = OnceLock::new();
        CORPUS.get_or_init(|| {
            let api = hf_hub::api::sync::Api::new().unwrap();

            let lb_repo = api.dataset("zai-org/LongBench-v2".to_string());
            let lb_path = lb_repo.get("data.json").unwrap();
            let lb_data: Vec<serde_json::Value> =
                serde_json::from_str(&fs::read_to_string(lb_path).unwrap()).unwrap();
            let longbench: Vec<String> = lb_data
                .iter()
                .filter_map(|item| {
                    let ctx = item.get("context")?.as_str()?;
                    if ctx.is_empty() {
                        None
                    } else {
                        Some(ctx.to_string())
                    }
                })
                .take(100)
                .collect();

            let sg_repo = api.dataset("RyokoAI/ShareGPT52K".to_string());
            let sg_path = sg_repo.get("sg_90k_part1.json").unwrap();
            let sg_data: Vec<serde_json::Value> =
                serde_json::from_str(&fs::read_to_string(sg_path).unwrap()).unwrap();
            let sharegpt: Vec<String> = sg_data
                .iter()
                .filter_map(|item| {
                    let messages = item.get("conversations")?.as_array()?;
                    let parts: Vec<String> = messages
                        .iter()
                        .filter_map(|msg| {
                            let role = msg
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            let value = msg.get("value").and_then(|v| v.as_str())?;
                            if value.is_empty() {
                                return None;
                            }
                            Some(format!("[{role}]: {value}"))
                        })
                        .collect();
                    if parts.is_empty() {
                        None
                    } else {
                        Some(parts.join("\n\n"))
                    }
                })
                .take(1_000)
                .collect();

            ExtendedCorpus {
                longbench,
                sharegpt,
            }
        })
    }

    fn run_extended(model: &str) {
        let comparison = Comparison::new(model);
        let corpus = extended_corpus();
        for texts in [&corpus.longbench, &corpus.sharegpt] {
            assert!(!texts.is_empty(), "extended corpus is empty");
            for chunk in texts.chunks(10) {
                comparison.assert_parity(chunk, false);
            }
        }
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_minimax_m2_1() {
        run_extended("MiniMaxAI/MiniMax-M2.1");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_nemotron() {
        run_extended("nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_deepseek_v3_2() {
        run_extended("deepseek-ai/DeepSeek-V3.2");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_gpt_oss() {
        run_extended("openai/gpt-oss-120b");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_qwen3() {
        run_extended("Qwen/Qwen3-0.6B");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_mistral_nemo() {
        run_extended("mistralai/Mistral-Nemo-Instruct-2407");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_qwen3_nemotron() {
        run_extended("nvidia/Qwen3-Nemotron-235B-A22B-GenRM");
    }

    #[test]
    #[ignore = "downloads LongBench-v2 and ShareGPT corpora"]
    fn extended_mistral_large() {
        run_extended("mistralai/Mistral-Large-3-675B-Instruct-2512");
    }
}
