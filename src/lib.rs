#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Exact, fast tokenization for supported local Hugging Face BPE and Unigram JSON files.
//!
//! `Tokenizer::load_file` loads a `tokenizer.json` directly. To create and
//! reuse the optional binary sidecar, use `Tokenizer::load_file_with_tkz_cache`.
//!
//! ```no_run
//! use snaptokens::Tokenizer;
//!
//! # fn main() -> Result<(), snaptokens::Error> {
//! let tokenizer = Tokenizer::load_file("tokenizer.json".as_ref())?;
//! let ids = tokenizer.encode("hello")?;
//!
//! let cached = Tokenizer::load_file_with_tkz_cache("tokenizer.json".as_ref())?;
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
/// The intermediate text-and-split representation used before BPE.
pub mod pre_tokenized;
/// Pre-tokenizers that divide text into BPE inputs.
pub mod pre_tokenizers;
mod tkz;

use std::{borrow::Cow, fs, path::Path};

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
    pre_tokenizers::{ByteLevel, PreTokenizer, Split, SplitBehavior},
};

use self::{
    added_tokens::Segment,
    decoders::Decoder,
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
    pre_tokenizers::FusedSplits,
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

impl Tokenizer {
    fn build(json: TokenizerJson) -> Result<Self, Error> {
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
        Self::build(json)
    }

    /// Loads and parses a JSON tokenizer file without reading or writing a cache.
    pub fn load_file(path: &Path) -> Result<Self, Error> {
        reject_native_sentencepiece_model(path)?;
        let json: TokenizerJson = serde_json::from_str(&fs::read_to_string(path)?)?;
        Self::build(json)
    }

    /// Loads a `.tkz` file directly or creates or reuses a sibling sidecar for JSON.
    pub fn load_file_with_tkz_cache(path: &Path) -> Result<Self, Error> {
        reject_native_sentencepiece_model(path)?;
        tkz::load_or_create(path)
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

    /// Encodes one string without applying post-processor special tokens.
    pub fn encode(&self, input: &str) -> Result<Vec<u32>, Error> {
        self.encode_with_special_tokens(input, false)
    }

    /// Encodes with optional post-processor special tokens.
    pub fn encode_with_special_tokens(
        &self,
        input: &str,
        add_special_tokens: bool,
    ) -> Result<Vec<u32>, Error> {
        self.encode_with_bpe_cache(input, add_special_tokens, false)
    }

    fn encode_with_bpe_cache(
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

        // Fused ByteLevel streams contain BPE merge/cache operations, which
        // are not a valid implementation of scored Unigram segmentation.
        let fused_byte_level = self.model.bpe().and_then(|_| {
            self.pre_tokenizer
                .as_ref()
                .and_then(PreTokenizer::fused_byte_level)
        });
        let fused_split = fused_byte_level.and_then(|(splits, _)| splits?.single());

        if let Some((None, byte_level)) = fused_byte_level
            && self.normalizer.is_none()
        {
            let mut ids = Vec::with_capacity(output_capacity(input.len()));
            self.encode_fused_byte_level_into(input, byte_level, &mut ids, use_parallel_cache)?;
            return Ok(self.post_process(ids, add_special_tokens));
        }

        if let Some((Some(splits), _)) = fused_byte_level {
            let mut ids = Vec::new();
            self.encode_fused_split_into(input, splits, &mut ids, use_parallel_cache)?;
            return Ok(self.post_process(ids, add_special_tokens));
        }

        // Large eligible Unigram documents partition the raw text before
        // normalization so the charsmap, word walk, and Viterbi all run in
        // parallel; unchanged partitions borrow instead of copying.
        if let Some(unigram) = self.model.unigram()
            && let Some(metaspace) = self
                .pre_tokenizer
                .as_ref()
                .and_then(PreTokenizer::fused_whitespace_metaspace)
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

        let (mut pts, split_applied) = self.build_pre_tokenized_for_encode(input, fused_split);

        if let Some((split, byte_level)) = fused_byte_level {
            if let Some(split) = split
                && !split_applied
            {
                split.pre_tokenize(&mut pts)?;
            }
            byte_level.pre_tokenize_fused(&mut pts);
            let ids = pts
                .tokenize_batched(|buf, splits, out| {
                    self.model.tokenize_batch_fused(buf, splits, out)
                })
                .map_err(Error::Model)?;
            return Ok(self.post_process(ids, add_special_tokens));
        }

        if let Some(metaspace) = self.model.unigram().and_then(|_| {
            self.pre_tokenizer
                .as_ref()
                .and_then(PreTokenizer::fused_whitespace_metaspace)
        }) {
            // Large single documents run the fused word walk and Viterbi
            // together per whitespace-aligned partition; the serial fused
            // walker remains the exact path for everything below the gates.
            if let Some(unigram) = self.model.unigram()
                && pts.buffer().len() >= pre_tokenized::PARALLEL_INPUT_BYTES
                && pre_tokenized::inner_parallelism_enabled()
            {
                let ids =
                    encode_metaspace_partitions(unigram, metaspace, &pts).map_err(Error::Model)?;
                return Ok(self.post_process(ids, add_special_tokens));
            }
            metaspace.pre_tokenize_after_whitespace(&mut pts);
        } else if let Some(ref pt) = self.pre_tokenizer {
            pt.pre_tokenize(&mut pts)?;
        }

        // 2b. Break each text split at unbridgeable byte-pair boundaries.
        //     Split at positions no vocab token can cover across adjacent
        //     bytes. This is provably output-preserving and provides
        //     fine-grained word-level chunking for models that don't use
        //     ByteLevel. The table decodes byte-fallback marker spelling, so
        //     fallback values stay in the ordinary BPE merge path.
        if self.needs_vocab_splitting
            && let Some(table) = self.model.bigram_bridge_table()
        {
            split_on_unbridgeable_bigrams(&mut pts, table);
        }

        let ids = if let Some(unigram) = self.model.unigram() {
            // Metaspace commonly yields one split per word. The Unigram
            // callback owns one workspace per Rayon chunk, never per word.
            pts.tokenize_batched(|buffer, splits, out| {
                unigram.tokenize_splits_into(buffer, splits, out)
            })
        } else {
            pts.tokenize(|text, out| self.model.tokenize_into(text, out))
        }
        .map_err(Error::Model)?;

        Ok(self.post_process(ids, add_special_tokens))
    }

    /// Encodes strings in input order, parallelizing substantial batches when useful.
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
                .map(|input| self.encode_with_special_tokens(input.as_ref(), add_special_tokens))
                .collect();
        }

        let outer_tasks = inputs.len().min(rayon::current_num_threads());
        if outer_tasks < 2 {
            return inputs
                .par_iter()
                .map(|input| self.encode_with_special_tokens(input.as_ref(), add_special_tokens))
                .collect();
        }

        self.encode_outer_batch(inputs, add_special_tokens, outer_tasks)
    }

    /// Encodes a batch into one ID buffer and one token length per input row.
    ///
    /// Concatenating slices of `ids` using `lengths` reconstructs the rows that
    /// [`Self::encode_batch`] would return in the same order.
    pub fn encode_batch_ragged<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        add_special_tokens: bool,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        // Keep ragged batches on the same BPE-only fused boundary as scalar encoding.
        let fused_byte_level = self.model.bpe().and_then(|_| {
            self.pre_tokenizer
                .as_ref()
                .and_then(PreTokenizer::fused_byte_level)
        });
        let total_bytes = inputs
            .iter()
            .map(|input| input.as_ref().len())
            .sum::<usize>();
        let parallel_bytes =
            PARALLEL_BATCH_BYTES_PER_THREAD.saturating_mul(rayon::current_num_threads());
        if !add_special_tokens && let Some((Some(splits), _)) = fused_byte_level {
            return self.encode_fused_split_ragged(inputs, splits, total_bytes, parallel_bytes);
        }

        if !add_special_tokens
            && (inputs.len() == 1 || total_bytes <= parallel_bytes)
            && self.normalizer.is_none()
            && let Some((None, byte_level)) = fused_byte_level
        {
            return self.encode_fused_ragged_serial(inputs, byte_level, total_bytes);
        }

        if !add_special_tokens
            && inputs.len() >= 2
            && self.normalizer.is_none()
            && let Some((None, byte_level)) = fused_byte_level
        {
            return self.encode_fused_ragged_parallel(inputs, byte_level, total_bytes);
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
    fn encode_fused_ragged_serial<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        byte_level: &ByteLevel,
        total_bytes: usize,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        let mut ids = Vec::with_capacity(output_capacity(total_bytes));
        let mut lengths = Vec::with_capacity(inputs.len());
        if self.added_tokens.as_ref().is_none_or(|added_tokens| {
            inputs
                .iter()
                .all(|input| !added_tokens.has_candidate(input.as_ref()))
        }) {
            self.model
                .tokenize_fused_stream("", &mut ids, false, |stream| {
                    let mut start = 0;
                    for input in inputs {
                        let input = input.as_ref();
                        byte_level.stream_fused(input, stream);
                        let end = stream.output_len();
                        lengths.push(end - start);
                        start = end;
                    }
                })
                .map_err(Error::Model)?;
            return Ok((ids, lengths));
        }
        for input in inputs {
            let start = ids.len();
            self.encode_fused_byte_level_into(input.as_ref(), byte_level, &mut ids, false)?;
            lengths.push(ids.len() - start);
        }
        Ok((ids, lengths))
    }

    fn encode_fused_split_ragged<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        splits: FusedSplits<'_>,
        total_bytes: usize,
        parallel_bytes: usize,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        let encode_chunk = |chunk: &[S]| {
            let chunk_bytes = chunk
                .iter()
                .map(|input| input.as_ref().len())
                .sum::<usize>();
            let capacity = if total_bytes >= COMPACT_RAGGED_OUTPUT_BYTES {
                chunk_bytes.div_ceil(2).saturating_add(16)
            } else {
                chunk_bytes.saturating_add(3)
            };
            let mut ids = Vec::with_capacity(capacity);
            let mut lengths = Vec::with_capacity(chunk.len());
            let use_parallel_cache = inputs.len() > 2 && total_bytes >= PARALLEL_CACHE_BATCH_BYTES;
            if chunk
                .iter()
                .all(|input| self.can_encode_fused_split(input.as_ref()))
            {
                self.model
                    .tokenize_fused_stream("", &mut ids, use_parallel_cache, |stream| {
                        let mut start = 0;
                        for input in chunk {
                            let input = input.as_ref();
                            let normalized = self
                                .normalizer
                                .as_ref()
                                .map_or(Cow::Borrowed(input), |normalizer| {
                                    normalizer.normalize(input)
                                });
                            let input = normalized.as_ref();
                            splits.stream_into(input, stream);
                            let end = stream.output_len();
                            lengths.push(end - start);
                            start = end;
                        }
                    })
                    .map_err(Error::Model)?;
                return Ok((ids, lengths));
            }
            for input in chunk {
                let input = input.as_ref();
                let start = ids.len();
                self.encode_fused_split_into(input, splits, &mut ids, use_parallel_cache)?;
                lengths.push(ids.len() - start);
            }
            Ok((ids, lengths))
        };

        if inputs.len() == 1
            && total_bytes >= PARALLEL_CACHE_BATCH_BYTES
            && self.normalizer.is_none()
        {
            let input = inputs[0].as_ref();
            let workers = rayon::current_num_threads().min(WIDE_BATCH_TASKS);
            if let Some(ranges) = splits.newline_partition_ranges(input, workers)
                && ranges.len() > 1
            {
                let chunks = ranges
                    .into_par_iter()
                    .map(|range| {
                        let input = &input[range];
                        let mut ids = Vec::with_capacity(output_capacity(input.len()));
                        self.model
                            .tokenize_fused_stream("", &mut ids, true, |stream| {
                                splits.stream_into(input, stream);
                            })
                            .map_err(Error::Model)?;
                        Ok(ids)
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let token_count = chunks.iter().map(Vec::len).sum();
                let mut ids = Vec::with_capacity(token_count);
                for chunk in chunks {
                    ids.extend(chunk);
                }
                return Ok((ids, vec![token_count]));
            }
        }

        if inputs.len() < 3 || total_bytes <= parallel_bytes {
            return encode_chunk(inputs);
        }

        let workers = inputs.len().min(rayon::current_num_threads());
        let chunk_size = inputs.len().div_ceil(workers * RAGGED_CHUNKS_PER_WORKER);
        let chunks = inputs
            .par_chunks(chunk_size)
            .map(encode_chunk)
            .collect::<Result<Vec<_>, Error>>()?;
        let token_count = chunks.iter().map(|(ids, _)| ids.len()).sum();
        let mut ids = Vec::with_capacity(token_count);
        let mut lengths = Vec::with_capacity(inputs.len());
        for (chunk_ids, chunk_lengths) in chunks {
            ids.extend(chunk_ids);
            lengths.extend(chunk_lengths);
        }
        Ok((ids, lengths))
    }

    #[inline(never)]
    fn encode_fused_ragged_parallel<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        byte_level: &ByteLevel,
        total_bytes: usize,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        let workers = inputs.len().min(rayon::current_num_threads());
        let chunk_size = inputs.len().div_ceil(workers * RAGGED_CHUNKS_PER_WORKER);
        let use_parallel_cache = total_bytes >= PARALLEL_CACHE_BATCH_BYTES;
        let chunks = inputs
            .par_chunks(chunk_size)
            .map(|chunk| {
                let chunk_bytes = chunk
                    .iter()
                    .map(|input| input.as_ref().len())
                    .sum::<usize>();
                let mut ids = Vec::with_capacity(output_capacity(chunk_bytes));
                let mut lengths = Vec::with_capacity(chunk.len());
                for input in chunk {
                    let start = ids.len();
                    self.encode_fused_byte_level_into(
                        input.as_ref(),
                        byte_level,
                        &mut ids,
                        use_parallel_cache,
                    )?;
                    lengths.push(ids.len() - start);
                }
                Ok((ids, lengths))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let token_count = chunks.iter().map(|(ids, _)| ids.len()).sum();
        let mut ids = Vec::with_capacity(token_count);
        let mut lengths = Vec::with_capacity(inputs.len());
        for (chunk_ids, chunk_lengths) in chunks {
            ids.extend(chunk_ids);
            lengths.extend(chunk_lengths);
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
                if outer_tasks >= WIDE_BATCH_TASKS || input.len() <= SHORT_BATCH_INPUT_BYTES {
                    pre_tokenized::without_inner_parallelism(|| {
                        self.encode_with_bpe_cache(input, add_special_tokens, use_parallel_cache)
                    })
                } else {
                    self.encode_with_bpe_cache(input, add_special_tokens, use_parallel_cache)
                }
            })
            .collect()
    }

    fn encode_fused_byte_level_into(
        &self,
        input: &str,
        byte_level: &ByteLevel,
        ids: &mut Vec<u32>,
        use_parallel_cache: bool,
    ) -> Result<(), Error> {
        if let Some(added_tokens) = &self.added_tokens
            && added_tokens.has_candidate(input)
        {
            for segment in added_tokens.split(input) {
                match segment {
                    Segment::Token(id) => ids.push(id),
                    Segment::Text(text) => self
                        .model
                        .tokenize_fused_stream(text, ids, use_parallel_cache, |stream| {
                            byte_level.stream_fused(text, stream)
                        })
                        .map_err(Error::Model)?,
                }
            }
            return Ok(());
        }

        self.model
            .tokenize_fused_stream(input, ids, use_parallel_cache, |stream| {
                byte_level.stream_fused(input, stream)
            })
            .map_err(Error::Model)
    }

    fn encode_fused_split_into(
        &self,
        input: &str,
        splits: FusedSplits<'_>,
        ids: &mut Vec<u32>,
        use_parallel_cache: bool,
    ) -> Result<(), Error> {
        if !self.can_encode_fused_split(input) {
            let segments = self.segment_input(input);
            let normalized_added_tokens = self
                .added_tokens
                .as_ref()
                .filter(|added_tokens| added_tokens.has_normalized());
            self.model
                .tokenize_fused_stream(input, ids, use_parallel_cache, |stream| {
                    for segment in segments {
                        match segment {
                            Segment::Token(id) => stream.push_id(id),
                            Segment::Text(text) => {
                                let normalized = self
                                    .normalizer
                                    .as_ref()
                                    .map_or(Cow::Borrowed(text), |normalizer| {
                                        normalizer.normalize(text)
                                    });
                                if let Some(added_tokens) = normalized_added_tokens {
                                    for segment in
                                        added_tokens.split_normalized(normalized.as_ref())
                                    {
                                        match segment {
                                            Segment::Token(id) => stream.push_id(id),
                                            Segment::Text(text) => {
                                                splits.stream_into(text, stream);
                                                stream.flush_pending();
                                            }
                                        }
                                    }
                                } else {
                                    let text = normalized.as_ref();
                                    splits.stream_into(text, stream);
                                    stream.flush_pending();
                                }
                            }
                        }
                    }
                })
                .map_err(Error::Model)?;
            return Ok(());
        }

        let normalized = self
            .normalizer
            .as_ref()
            .map_or(Cow::Borrowed(input), |normalizer| {
                normalizer.normalize(input)
            });
        let input = normalized.as_ref();
        self.model
            .tokenize_fused_stream(input, ids, use_parallel_cache, |stream| {
                splits.stream_into(input, stream)
            })
            .map_err(Error::Model)?;
        Ok(())
    }

    fn can_encode_fused_split(&self, input: &str) -> bool {
        !self
            .added_tokens
            .as_ref()
            .is_some_and(|added_tokens| added_tokens.has_candidate(input))
    }

    /// Replaces the post-processor used by encoding with special tokens.
    pub fn set_post_processor(&mut self, pp: Option<PostProcessor>) {
        self.post_processor = pp;
    }

    /// Replaces the normalizer applied before pre-tokenization.
    pub fn set_normalizer(&mut self, normalizer: Option<Normalizer>) {
        self.normalizer = normalizer;
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
        let mut tokens = Vec::with_capacity(ids.len());
        for &id in ids {
            if skip_special_tokens
                && let Some(ref at) = self.added_tokens
                && at.is_special(id)
            {
                continue;
            }
            // Match HuggingFace behavior: silently skip unknown IDs (e.g.
            // models like Qwen3-0.6B-FP8 emit IDs in the gap between
            // tokenizer.json's vocab and the embedding matrix). Erroring
            // here would kill streaming generation on a single bad token.
            if let Some(token_str) = self.id_to_token(id) {
                tokens.push(token_str.to_string());
            }
        }

        self.decode_tokens(tokens)
    }

    /// Decode a sequence of token strings back into text.
    ///
    /// Applies the decoder pipeline (e.g. ByteLevel → convert "Ġ" back to " ")
    /// without going through the ID→string lookup.  When no decoder is
    /// configured the tokens are concatenated with no separator.
    pub fn decode_tokens(&self, tokens: Vec<String>) -> Result<String, Error> {
        match &self.decoder {
            Some(dec) => dec.decode(tokens).map_err(Error::Decoder),
            None => Ok(tokens.concat()),
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

    /// Returns the ID for token text, checking added tokens before the BPE vocabulary.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        if let Some(ref at) = self.added_tokens
            && let Some(id) = at.token_to_id(token)
        {
            return Some(id);
        }
        self.model.token_to_id(token)
    }

    /// Returns the BPE vocabulary size plus the number of added tokens.
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

    fn build_pre_tokenized_for_encode(
        &self,
        input: &str,
        fused_split: Option<&Split>,
    ) -> (PreTokenizedString, bool) {
        let segments = self.segment_input(input);
        if matches!(self.normalizer, Some(Normalizer::Nfc(_)))
            && !self
                .added_tokens
                .as_ref()
                .is_some_and(AddedTokens::has_normalized)
            && let Some(split) = fused_split
            && split.supports_ascii_nfc_fusion()
        {
            let capacity = input.len().div_ceil(4).max(segments.len());
            let mut splits = Vec::with_capacity(capacity);
            let mut buffer_len = 0;
            let mut is_ascii = true;

            for segment in &segments {
                match segment {
                    Segment::Token(id) => splits.push(PtSplit {
                        range: buffer_len..buffer_len,
                        token_id: Some(*id),
                    }),
                    Segment::Text(text) => {
                        if !split.append_ascii_splits(text, buffer_len, &mut splits) {
                            is_ascii = false;
                            break;
                        }
                        buffer_len += text.len();
                    }
                }
            }

            if is_ascii {
                let mut buffer = String::with_capacity(buffer_len);
                for segment in &segments {
                    if let Segment::Text(text) = segment {
                        buffer.push_str(text);
                    }
                }
                return (PreTokenizedString::new(buffer, splits), true);
            }
        }

        (
            self.build_pre_tokenized_from_segments(input, &segments),
            false,
        )
    }

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

        for seg in segments {
            match seg {
                Segment::Token(id) => {
                    let start = buffer.len();
                    splits.push(PtSplit {
                        range: start..start,
                        token_id: Some(*id),
                    });
                }
                Segment::Text(text) => {
                    if text.is_empty() {
                        continue;
                    }
                    let normalized = match &self.normalizer {
                        Some(n) => n.normalize(text),
                        None => std::borrow::Cow::Borrowed(*text),
                    };
                    if let Some(added_tokens) = normalized_added_tokens {
                        // Normalized tokens only inspect spans left unmatched by
                        // the raw-token phase, exactly as Hugging Face does.
                        for segment in added_tokens.split_normalized(&normalized) {
                            let start = buffer.len();
                            match segment {
                                Segment::Token(id) => splits.push(PtSplit {
                                    range: start..start,
                                    token_id: Some(id),
                                }),
                                Segment::Text(text) => {
                                    buffer.push_str(text);
                                    splits.push(PtSplit {
                                        range: start..buffer.len(),
                                        token_id: None,
                                    });
                                }
                            }
                        }
                    } else {
                        let start = buffer.len();
                        buffer.push_str(&normalized);
                        splits.push(PtSplit {
                            range: start..buffer.len(),
                            token_id: None,
                        });
                    }
                }
            }
        }

        PreTokenizedString::new(buffer, splits)
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

/// One in-order unit of raw-text partitioned Unigram work.
enum RawPartition<'a> {
    /// An added-token placeholder emitting its fixed ID.
    Token(u32),
    /// A raw text slice cut only at proven-safe whitespace anchors.
    Text(&'a str),
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
    let workers = pre_tokenized::bpe_pool().current_num_threads();
    let target = (total_text / (workers * METASPACE_PARTITIONS_PER_WORKER).max(1))
        .clamp(METASPACE_PARTITION_MIN_BYTES, METASPACE_PARTITION_MAX_BYTES);

    let mut partitions = Vec::with_capacity(total_text / target + segments.len() + 1);
    for segment in segments {
        match segment {
            Segment::Token(id) => partitions.push(RawPartition::Token(*id)),
            Segment::Text(text) => {
                let bytes = text.as_bytes();
                let mut start = 0;
                while text.len() - start > target {
                    let mut cut = start + target;
                    while cut < text.len()
                        && !(bytes[cut - 1] < 0x80
                            && anchors[bytes[cut - 1] as usize]
                            && (0x20..0x7f).contains(&bytes[cut]))
                    {
                        cut += 1;
                    }
                    if cut >= text.len() {
                        break;
                    }
                    partitions.push(RawPartition::Text(&text[start..cut]));
                    start = cut;
                }
                if start < text.len() {
                    partitions.push(RawPartition::Text(&text[start..]));
                }
            }
        }
    }

    let chunks = pre_tokenized::bpe_pool().install(|| {
        partitions
            .par_iter()
            .map(|partition| match partition {
                RawPartition::Token(id) => Ok(vec![*id]),
                RawPartition::Text(slice) => {
                    let normalized = match normalizer {
                        Some(normalizer) => normalizer.normalize(slice),
                        None => Cow::Borrowed(*slice),
                    };
                    let mut ids = Vec::with_capacity(output_capacity(normalized.len()));
                    let mut word_scratch = String::new();
                    let mut viterbi = models::unigram::ViterbiScratch::default();
                    metaspace.for_each_word_piece(&normalized, &mut word_scratch, |piece| {
                        unigram.tokenize_into_with_scratch(piece, &mut ids, &mut viterbi)
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

/// One in-order unit of partitioned fused Unigram work.
enum MetaspacePartition {
    /// An added-token placeholder emitting its fixed ID.
    Token(u32),
    /// A whitespace-aligned byte range of one ordinary split.
    Text(std::ops::Range<usize>),
}

/// Bytes of fused word-walk-plus-Viterbi work per parallel partition.
const METASPACE_PARTITION_MIN_BYTES: usize = 16 * 1024;
const METASPACE_PARTITION_MAX_BYTES: usize = 128 * 1024;
const METASPACE_PARTITIONS_PER_WORKER: usize = 6;

/// Runs the fused WhitespaceSplit+Metaspace word walk and per-piece Viterbi
/// in parallel over whitespace-aligned partitions of the normalized buffer.
///
/// A cut directly after an ASCII-whitespace byte is a character boundary that
/// never divides a word, and every Metaspace piece is a function of one word,
/// so concatenating per-partition IDs reproduces the serial fused result.
fn encode_metaspace_partitions(
    unigram: &models::unigram::Unigram,
    metaspace: &pre_tokenizers::Metaspace,
    pts: &PreTokenizedString,
) -> Result<Vec<u32>, String> {
    let buffer = pts.buffer();
    let bytes = buffer.as_bytes();
    let workers = pre_tokenized::bpe_pool().current_num_threads();
    let target = (buffer.len() / (workers * METASPACE_PARTITIONS_PER_WORKER).max(1))
        .clamp(METASPACE_PARTITION_MIN_BYTES, METASPACE_PARTITION_MAX_BYTES);

    let mut partitions = Vec::with_capacity(buffer.len() / target + pts.splits().len() + 1);
    for split in pts.splits() {
        if let Some(id) = split.token_id {
            partitions.push(MetaspacePartition::Token(id));
            continue;
        }
        if split.range.is_empty() {
            continue;
        }
        let mut start = split.range.start;
        while split.range.end - start > target {
            // Advance to the next position preceded by ASCII whitespace: a
            // char boundary that cannot lie inside any word.
            let mut cut = start + target;
            while cut < split.range.end && !bytes[cut - 1].is_ascii_whitespace() {
                cut += 1;
            }
            if cut >= split.range.end {
                break;
            }
            partitions.push(MetaspacePartition::Text(start..cut));
            start = cut;
        }
        partitions.push(MetaspacePartition::Text(start..split.range.end));
    }

    let chunks = pre_tokenized::bpe_pool().install(|| {
        partitions
            .par_iter()
            .map(|partition| match partition {
                MetaspacePartition::Token(id) => Ok(vec![*id]),
                MetaspacePartition::Text(range) => {
                    let mut ids = Vec::with_capacity(output_capacity(range.len()));
                    let mut word_scratch = String::new();
                    let mut viterbi = models::unigram::ViterbiScratch::default();
                    metaspace.for_each_word_piece(
                        &buffer[range.clone()],
                        &mut word_scratch,
                        |piece| unigram.tokenize_into_with_scratch(piece, &mut ids, &mut viterbi),
                    )?;
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

/// Splits only at byte pairs no vocabulary token can cover.
fn split_on_unbridgeable_bigrams(
    pts: &mut PreTokenizedString,
    bigram_table: &models::bpe::BigramBridgeTable,
) {
    let bytes = pts.buffer().as_bytes();
    let mut new_splits = Vec::with_capacity(pts.splits().len() * 2);

    for split in pts.splits() {
        if split.token_id.is_some() || split.range.is_empty() {
            new_splits.push(split.clone());
            continue;
        }

        let end = split.range.end;
        let mut start = split.range.start;

        for i in (start + 1)..end {
            let prev = bytes[i - 1];
            let cur = bytes[i];

            if !bigram_table.is_bridgeable(prev, cur) && (cur & 0xC0) != 0x80 {
                new_splits.push(PtSplit {
                    range: start..i,
                    token_id: None,
                });
                start = i;
            }
        }

        new_splits.push(PtSplit {
            range: start..end,
            token_id: None,
        });
    }

    pts.refine_splits(new_splits);
}

/// Rejects the native SentencePiece protobuf boundary before attempting UTF-8 JSON parsing.
fn reject_native_sentencepiece_model(path: &Path) -> Result<(), Error> {
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
