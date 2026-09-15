use std::{cell::Cell, ops::Range, sync::OnceLock};

use rayon::prelude::*;

const PARALLEL_THRESHOLD: usize = 16;
const PARALLEL_INPUT_BYTES: usize = 16 * 1024;

thread_local! {
    pub(crate) static INNER_PARALLELISM: Cell<bool> = const { Cell::new(true) };
}

struct InnerParallelismReset<'a> {
    state: &'a Cell<bool>,
    previous: bool,
}

impl Drop for InnerParallelismReset<'_> {
    fn drop(&mut self) {
        self.state.set(self.previous);
    }
}

pub(crate) fn without_inner_parallelism<T>(f: impl FnOnce() -> T) -> T {
    INNER_PARALLELISM.with(|state| {
        let _reset = InnerParallelismReset {
            state,
            previous: state.replace(false),
        };
        f()
    })
}

fn bpe_pool() -> &'static rayon::ThreadPool {
    static POOL: OnceLock<rayon::ThreadPool> = OnceLock::new();
    POOL.get_or_init(|| {
        let n = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(8);
        rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build()
            .expect("failed to build BPE thread pool")
    })
}

/// A byte range in a [`PreTokenizedString`] or an added-token placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Split {
    /// Byte range into the containing buffer.
    pub range: Range<usize>,
    /// If `Some`, this split is an added token and should emit this ID directly
    /// rather than being passed to the model.
    pub token_id: Option<u32>,
}

/// Normalized text divided into pieces that can be tokenized independently.
#[derive(Debug, Clone)]
pub struct PreTokenizedString {
    buffer: String,
    splits: Vec<Split>,
}

impl PreTokenizedString {
    /// Creates one ordinary-text split covering a complete input string.
    pub fn from_text(text: &str) -> Self {
        let splits = if text.is_empty() {
            Vec::new()
        } else {
            vec![Split {
                range: 0..text.len(),
                token_id: None,
            }]
        };
        Self {
            buffer: text.to_string(),
            splits,
        }
    }

    /// Creates an intermediate value from a buffer and its byte-range splits.
    pub fn new(buffer: String, splits: Vec<Split>) -> Self {
        Self { buffer, splits }
    }

    /// Returns the normalized backing text.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Returns the tokenization splits in buffer order.
    pub fn splits(&self) -> &[Split] {
        &self.splits
    }

    /// Returns the text covered by one ordinary split.
    pub fn split_text(&self, split: &Split) -> &str {
        &self.buffer[split.range.clone()]
    }

    /// Replaces the backing text and all splits together.
    pub fn set_buffer(&mut self, buffer: String, splits: Vec<Split>) {
        self.buffer = buffer;
        self.splits = splits;
    }

    /// Replaces splits while retaining the existing backing text.
    pub fn refine_splits(&mut self, splits: Vec<Split>) {
        self.splits = splits;
    }

    /// Tokenizes every split and returns one concatenated ID buffer.
    pub fn tokenize<F>(&self, tokenize_fn: F) -> Result<Vec<u32>, String>
    where
        F: Fn(&str, &mut Vec<u32>) -> Result<(), String> + Sync,
    {
        if self.buffer.len() < PARALLEL_INPUT_BYTES
            || self.splits.len() < PARALLEL_THRESHOLD
            || !INNER_PARALLELISM.with(Cell::get)
        {
            return self.tokenize_sequential(&tokenize_fn);
        }

        let pool = bpe_pool();
        let chunk_size = self.splits.len().div_ceil(pool.current_num_threads());

        pool.install(|| {
            let chunk_results: Result<Vec<Vec<u32>>, String> = self
                .splits
                .par_chunks(chunk_size)
                .map(|chunk| {
                    let mut ids = Vec::with_capacity(chunk.len() * 3);
                    for split in chunk {
                        if let Some(id) = split.token_id {
                            ids.push(id);
                        } else if !split.range.is_empty() {
                            let text = &self.buffer[split.range.clone()];
                            tokenize_fn(text, &mut ids)?;
                        }
                    }
                    Ok(ids)
                })
                .collect();

            let chunks = chunk_results?;
            let total: usize = chunks.iter().map(Vec::len).sum();
            let mut ids = Vec::with_capacity(total);
            for chunk_ids in chunks {
                ids.extend(chunk_ids);
            }
            Ok(ids)
        })
    }

    /// Tokenizes every split using a callback that receives a whole split batch.
    pub fn tokenize_batched<F>(&self, tokenize_fn: F) -> Result<Vec<u32>, String>
    where
        F: Fn(&str, &[Split], &mut Vec<u32>) -> Result<(), String> + Sync,
    {
        if self.buffer.len() < PARALLEL_INPUT_BYTES
            || self.splits.len() < PARALLEL_THRESHOLD
            || !INNER_PARALLELISM.with(Cell::get)
        {
            let mut ids = Vec::with_capacity(self.splits.len() * 2);
            tokenize_fn(&self.buffer, &self.splits, &mut ids)?;
            return Ok(ids);
        }

        let pool = bpe_pool();
        let task_count = pool.current_num_threads().saturating_mul(2);
        let chunk_size = self.splits.len().div_ceil(task_count);

        pool.install(|| {
            let chunk_results: Result<Vec<Vec<u32>>, String> = self
                .splits
                .par_chunks(chunk_size)
                .map(|chunk| {
                    let mut ids = Vec::with_capacity(chunk.len() * 3);
                    tokenize_fn(&self.buffer, chunk, &mut ids)?;
                    Ok(ids)
                })
                .collect();

            let chunks = chunk_results?;
            let total: usize = chunks.iter().map(Vec::len).sum();
            let mut ids = Vec::with_capacity(total);
            for chunk_ids in chunks {
                ids.extend(chunk_ids);
            }
            Ok(ids)
        })
    }

    fn tokenize_sequential<F>(&self, tokenize_fn: &F) -> Result<Vec<u32>, String>
    where
        F: Fn(&str, &mut Vec<u32>) -> Result<(), String>,
    {
        let mut ids = Vec::with_capacity(self.splits.len() * 2);
        for split in &self.splits {
            if let Some(id) = split.token_id {
                ids.push(id);
            } else {
                let text = self.split_text(split);
                if !text.is_empty() {
                    tokenize_fn(text, &mut ids)?;
                }
            }
        }
        Ok(ids)
    }
}
