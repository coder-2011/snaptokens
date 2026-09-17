//! BPE-only Tokenizer encode shortcuts and vocab-safe split insertion.
//!
//! These `Tokenizer` methods are valid only after `Model::bpe()` succeeds.

use std::borrow::Cow;

use rayon::prelude::*;

use super::{BigramBridgeTable, Bpe};
use crate::{
    Error, Tokenizer,
    added_tokens::Segment,
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
    pre_tokenizers::{ByteLevel, FusedSplits, PreTokenizer},
};

impl Tokenizer {
    /// Proves BPE once at the guard; callees receive the `&Bpe` instead of re-deriving it.
    fn fused_byte_level(&self) -> Option<(&Bpe, Option<FusedSplits<'_>>, &ByteLevel)> {
        let bpe = self.model.bpe()?;
        let (splits, byte_level) = self
            .pre_tokenizer
            .as_ref()
            .and_then(PreTokenizer::fused_byte_level)?;
        Some((bpe, splits, byte_level))
    }

    /// Fast fused ByteLevel/split encode that does not build a `PreTokenizedString`.
    pub(crate) fn try_encode_fused_bpe(
        &self,
        input: &str,
        use_parallel_cache: bool,
    ) -> Result<Option<Vec<u32>>, Error> {
        match self.fused_byte_level() {
            Some((bpe, None, byte_level)) if self.normalizer.is_none() => {
                let mut ids = Vec::with_capacity(crate::output_capacity(input.len()));
                self.encode_fused_byte_level_into(
                    bpe,
                    input,
                    byte_level,
                    &mut ids,
                    use_parallel_cache,
                )?;
                Ok(Some(ids))
            }
            Some((bpe, Some(splits), _)) => {
                let mut ids = Vec::new();
                self.encode_fused_split_into(bpe, input, splits, &mut ids, use_parallel_cache)?;
                Ok(Some(ids))
            }
            _ => Ok(None),
        }
    }

    /// Fused ByteLevel encode after ordinary added-token / NFC pre-tokenization.
    ///
    /// The sole caller runs after `try_encode_fused_bpe` returned `None`, so any
    /// fused-`Split` configuration was already consumed there.
    pub(crate) fn encode_fused_bpe_pre_tokenized(
        &self,
        pts: &mut PreTokenizedString,
    ) -> Result<Option<Vec<u32>>, Error> {
        let Some((bpe, _, byte_level)) = self.fused_byte_level() else {
            return Ok(None);
        };
        byte_level.pre_tokenize_fused(pts);
        let ids = pts
            .tokenize_batched(|buf, splits, out| bpe.append_split_bpe_ids(buf, splits, out))
            .map_err(Error::Model)?;
        Ok(Some(ids))
    }

    /// Inserts unbridgeable-bigram splits when BPE merge reachability allows it.
    pub(crate) fn apply_vocab_splits(&self, pts: &mut PreTokenizedString) {
        if self.needs_vocab_splitting
            && let Some(table) = self.model.bpe().and_then(Bpe::bigram_bridge_table)
        {
            split_on_unbridgeable_bigrams(pts, table);
        }
    }

    /// Fused ragged encode on the same BPE-only boundary as scalar encoding.
    pub(crate) fn try_encode_fused_bpe_ragged<S: AsRef<str> + Sync>(
        &self,
        inputs: &[S],
        add_special_tokens: bool,
    ) -> Result<Option<(Vec<u32>, Vec<usize>)>, Error> {
        if add_special_tokens {
            return Ok(None);
        }
        let Some(fused_byte_level) = self.fused_byte_level() else {
            return Ok(None);
        };
        let total_bytes = inputs
            .iter()
            .map(|input| input.as_ref().len())
            .sum::<usize>();
        let parallel_bytes =
            crate::PARALLEL_BATCH_BYTES_PER_THREAD.saturating_mul(rayon::current_num_threads());
        match fused_byte_level {
            (bpe, Some(splits), _) => self
                .encode_fused_split_ragged(bpe, inputs, splits, total_bytes, parallel_bytes)
                .map(Some),
            (bpe, None, byte_level)
                if self.normalizer.is_none()
                    && (inputs.len() == 1 || total_bytes <= parallel_bytes) =>
            {
                self.encode_fused_ragged_serial(bpe, inputs, byte_level, total_bytes)
                    .map(Some)
            }
            (bpe, None, byte_level) if self.normalizer.is_none() && inputs.len() >= 2 => self
                .encode_fused_ragged_parallel(bpe, inputs, byte_level, total_bytes)
                .map(Some),
            _ => Ok(None),
        }
    }

    #[inline(never)]
    fn encode_fused_ragged_serial<S: AsRef<str> + Sync>(
        &self,
        bpe: &Bpe,
        inputs: &[S],
        byte_level: &ByteLevel,
        total_bytes: usize,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        let mut ids = Vec::with_capacity(crate::output_capacity(total_bytes));
        let mut lengths = Vec::with_capacity(inputs.len());
        if self.added_tokens.as_ref().is_none_or(|added_tokens| {
            inputs
                .iter()
                .all(|input| !added_tokens.has_candidate(input.as_ref()))
        }) {
            bpe.append_scanned_bpe_ids("", &mut ids, false, |stream| {
                let mut start = 0;
                for input in inputs {
                    let input = input.as_ref();
                    byte_level.stream_fused(input, stream);
                    let end = stream.output_len()?;
                    lengths.push(end - start);
                    start = end;
                }
                Ok(())
            })?;
            return Ok((ids, lengths));
        }
        for input in inputs {
            let start = ids.len();
            self.encode_fused_byte_level_into(bpe, input.as_ref(), byte_level, &mut ids, false)?;
            lengths.push(ids.len() - start);
        }
        Ok((ids, lengths))
    }

    fn encode_fused_split_ragged<S: AsRef<str> + Sync>(
        &self,
        bpe: &Bpe,
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
            let capacity = if total_bytes >= crate::COMPACT_RAGGED_OUTPUT_BYTES {
                chunk_bytes.div_ceil(2).saturating_add(16)
            } else {
                chunk_bytes.saturating_add(3)
            };
            let mut ids = Vec::with_capacity(capacity);
            let mut lengths = Vec::with_capacity(chunk.len());
            let use_parallel_cache =
                inputs.len() > 2 && total_bytes >= crate::PARALLEL_CACHE_BATCH_BYTES;
            if chunk
                .iter()
                .all(|input| self.can_encode_fused_split(input.as_ref()))
            {
                bpe.append_scanned_bpe_ids("", &mut ids, use_parallel_cache, |stream| {
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
                        splits.stream_into(input, stream)?;
                        let end = stream.output_len()?;
                        lengths.push(end - start);
                        start = end;
                    }
                    Ok(())
                })?;
                return Ok((ids, lengths));
            }
            for input in chunk {
                let input = input.as_ref();
                let start = ids.len();
                self.encode_fused_split_into(bpe, input, splits, &mut ids, use_parallel_cache)?;
                lengths.push(ids.len() - start);
            }
            Ok((ids, lengths))
        };

        if inputs.len() == 1
            && total_bytes >= crate::PARALLEL_CACHE_BATCH_BYTES
            && self.normalizer.is_none()
        {
            let input = inputs[0].as_ref();
            let workers = rayon::current_num_threads().min(crate::WIDE_BATCH_TASKS);
            if let Some(ranges) = splits.newline_partition_ranges(input, workers)?
                && ranges.len() > 1
            {
                let chunks = ranges
                    .into_par_iter()
                    .map(|range| {
                        let input = &input[range];
                        let mut ids = Vec::with_capacity(crate::output_capacity(input.len()));
                        bpe.append_scanned_bpe_ids("", &mut ids, true, |stream| {
                            Ok(splits.stream_into(input, stream)?)
                        })?;
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
        let chunk_size = inputs
            .len()
            .div_ceil(workers * crate::RAGGED_CHUNKS_PER_WORKER);
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
        bpe: &Bpe,
        inputs: &[S],
        byte_level: &ByteLevel,
        total_bytes: usize,
    ) -> Result<(Vec<u32>, Vec<usize>), Error> {
        let workers = inputs.len().min(rayon::current_num_threads());
        let chunk_size = inputs
            .len()
            .div_ceil(workers * crate::RAGGED_CHUNKS_PER_WORKER);
        let use_parallel_cache = total_bytes >= crate::PARALLEL_CACHE_BATCH_BYTES;
        let chunks = inputs
            .par_chunks(chunk_size)
            .map(|chunk| {
                let chunk_bytes = chunk
                    .iter()
                    .map(|input| input.as_ref().len())
                    .sum::<usize>();
                let mut ids = Vec::with_capacity(crate::output_capacity(chunk_bytes));
                let mut lengths = Vec::with_capacity(chunk.len());
                for input in chunk {
                    let start = ids.len();
                    self.encode_fused_byte_level_into(
                        bpe,
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

    fn encode_fused_byte_level_into(
        &self,
        bpe: &Bpe,
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
                    Segment::Text(text) => {
                        bpe.append_scanned_bpe_ids(text, ids, use_parallel_cache, |stream| {
                            byte_level.stream_fused(text, stream);
                            Ok(())
                        })?
                    }
                }
            }
            return Ok(());
        }

        bpe.append_scanned_bpe_ids(input, ids, use_parallel_cache, |stream| {
            byte_level.stream_fused(input, stream);
            Ok(())
        })
    }

    fn encode_fused_split_into(
        &self,
        bpe: &Bpe,
        input: &str,
        splits: FusedSplits<'_>,
        ids: &mut Vec<u32>,
        use_parallel_cache: bool,
    ) -> Result<(), Error> {
        if !self.can_encode_fused_split(input) {
            let segments = self.segment_input(input);
            bpe.append_scanned_bpe_ids(input, ids, use_parallel_cache, |stream| {
                self.for_each_normalized_segment(&segments, |segment| match segment {
                    Segment::Token(id) => {
                        stream.push_id(id);
                        Ok(())
                    }
                    Segment::Text(text) => {
                        splits.stream_into(text, stream)?;
                        // Queued pieces borrow this normalized span until it is flushed.
                        stream.flush_pending();
                        Ok(())
                    }
                })
            })?;
            return Ok(());
        }

        let normalized = self
            .normalizer
            .as_ref()
            .map_or(Cow::Borrowed(input), |normalizer| {
                normalizer.normalize(input)
            });
        let input = normalized.as_ref();
        bpe.append_scanned_bpe_ids(input, ids, use_parallel_cache, |stream| {
            Ok(splits.stream_into(input, stream)?)
        })?;
        Ok(())
    }

    fn can_encode_fused_split(&self, input: &str) -> bool {
        self.added_tokens.as_ref().is_none_or(|added_tokens| {
            // Normalization can introduce a match absent from the original input.
            !(self.normalizer.is_some() && added_tokens.has_normalized())
                && !added_tokens.has_candidate(input)
        })
    }
}

/// Splits only at byte pairs no vocabulary token can cover.
fn split_on_unbridgeable_bigrams(pts: &mut PreTokenizedString, bigram_table: &BigramBridgeTable) {
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
