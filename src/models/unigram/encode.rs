use std::borrow::Cow;

use rayon::prelude::*;

use super::{Unigram, ViterbiScratch};
use crate::{
    Error, Normalizer, Tokenizer,
    added_tokens::{AddedTokens, Segment},
    pre_tokenized::{self, PreTokenizedString},
    pre_tokenizers::{Metaspace, PreTokenizer},
};

impl Tokenizer {
    fn fused_unigram(&self) -> Option<(&Unigram, &Metaspace)> {
        let unigram = self.model.unigram()?;
        let metaspace = self
            .pre_tokenizer
            .as_ref()
            .and_then(PreTokenizer::fused_metaspace)?;
        Some((unigram, metaspace))
    }

    pub(crate) fn try_encode_fused_unigram(&self, input: &str) -> Result<Option<Vec<u32>>, Error> {
        let Some((unigram, metaspace)) = self.fused_unigram() else {
            return Ok(None);
        };
        // Large eligible Unigram documents partition the raw text before
        // normalization so the charsmap, word walk, and Viterbi all run in
        // parallel; unchanged partitions borrow instead of copying.
        if input.len() >= pre_tokenized::PARALLEL_INPUT_BYTES
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
            return Ok(Some(ids));
        }
        Ok(None)
    }

    pub(crate) fn encode_fused_unigram_pre_tokenized(
        &self,
        pts: &mut PreTokenizedString,
    ) -> Result<Option<Vec<u32>>, Error> {
        let Some((unigram, metaspace)) = self.fused_unigram() else {
            return Ok(None);
        };
        // Large single documents run the fused word walk and Viterbi
        // together per whitespace-aligned partition; the serial fused
        // walker remains the exact path for everything below the gates.
        if pts.buffer().len() >= pre_tokenized::PARALLEL_INPUT_BYTES
            && pre_tokenized::inner_parallelism_enabled()
        {
            let ids = encode_metaspace_normalized_partitions(unigram, metaspace, pts)
                .map_err(Error::Model)?;
            return Ok(Some(ids));
        }
        metaspace.pre_tokenize_after_whitespace(pts);
        let ids = pts
            .tokenize_batched(|buffer, splits, out| {
                unigram.append_split_viterbi_ids(buffer, splits, out)
            })
            .map_err(Error::Model)?;
        Ok(Some(ids))
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
    unigram: &Unigram,
    metaspace: &Metaspace,
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
    unigram: &Unigram,
    metaspace: &Metaspace,
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
    unigram: &Unigram,
    metaspace: &Metaspace,
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
                    let mut ids = Vec::with_capacity(crate::output_capacity(text.len()));
                    let mut word_scratch = String::new();
                    let mut viterbi = ViterbiScratch::default();
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
