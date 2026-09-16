use std::{cell::Cell, ops::Range, sync::OnceLock};

use rayon::prelude::*;

const PARALLEL_THRESHOLD: usize = 16;
const PARALLEL_INPUT_BYTES: usize = 16 * 1024;

thread_local! {
    static INNER_PARALLELISM: Cell<bool> = const { Cell::new(true) };
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

/// The end to remove when limiting an encoded sequence.
#[derive(Clone, Copy, Default, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum TruncationDirection {
    /// Keep the final tokens.
    #[serde(alias = "left")]
    Left,
    /// Keep the initial tokens.
    #[default]
    #[serde(alias = "right")]
    Right,
}

impl TruncationDirection {
    /// Returns the spelling used by Python's direction argument.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
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

            chunk_results.map(concat_chunks)
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

            chunk_results.map(concat_chunks)
        })
    }

    /// Skip BPE beyond the retained tokens, but still validate discarded input.
    pub(crate) fn tokenize_with_limit(
        &self,
        max_tokens: usize,
        direction: TruncationDirection,
        mut tokenize: impl FnMut(&str, &mut Vec<u32>) -> Result<(), String>,
        mut validate: impl FnMut(&str) -> Result<(), String>,
    ) -> Result<(Vec<u32>, bool), String> {
        let mut ids = Vec::new();
        let left = direction == TruncationDirection::Left;
        for offset in 0..self.splits.len() {
            let index = if left {
                self.splits.len() - 1 - offset
            } else {
                offset
            };
            let split = &self.splits[index];
            if ids.len() > max_tokens {
                if split.token_id.is_none() {
                    validate(self.split_text(split))?;
                }
                continue;
            }
            let start = ids.len();
            if let Some(id) = split.token_id {
                ids.push(id);
            } else {
                tokenize(self.split_text(split), &mut ids)?;
            }
            // Reversing each piece lets the final reversal restore sequence order.
            if left {
                ids[start..].reverse();
            }
        }
        // One extra token distinguishes actual loss from an exact fit or empty pieces.
        let truncated = ids.len() > max_tokens;
        ids.truncate(max_tokens);
        if left {
            ids.reverse();
        }
        Ok((ids, truncated))
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

/// Concatenate ordered worker output outside the callback-generic encoding methods.
fn concat_chunks(chunks: Vec<Vec<u32>>) -> Vec<u32> {
    let total = chunks.iter().map(Vec::len).sum();
    let mut ids = Vec::with_capacity(total);
    for chunk in chunks {
        ids.extend(chunk);
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_parallelism_scope_restores_state() {
        let enabled = || INNER_PARALLELISM.with(Cell::get);
        assert!(enabled());

        without_inner_parallelism(|| {
            assert!(!enabled());
            without_inner_parallelism(|| assert!(!enabled()));
            assert!(!enabled());
        });
        assert!(enabled());

        let panic = std::panic::catch_unwind(|| {
            without_inner_parallelism(|| panic!("test panic"));
        });
        assert!(panic.is_err());
        assert!(enabled());
    }

    #[test]
    fn from_text_empty() {
        let pts = PreTokenizedString::from_text("");
        assert!(pts.splits().is_empty());
        assert!(pts.buffer().is_empty());
    }

    #[test]
    fn from_text_single_span() {
        let pts = PreTokenizedString::from_text("hello world");
        assert_eq!(pts.splits().len(), 1);
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello world");
        assert_eq!(pts.splits()[0].token_id, None);
    }

    #[test]
    fn new_with_mixed_splits() {
        let buffer = "hello<sep>world".to_string();
        let splits = vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..10,
                token_id: Some(42),
            },
            Split {
                range: 10..15,
                token_id: None,
            },
        ];
        let pts = PreTokenizedString::new(buffer, splits);
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
        assert_eq!(pts.split_text(&pts.splits()[1]), "<sep>");
        assert_eq!(pts.splits()[1].token_id, Some(42));
        assert_eq!(pts.split_text(&pts.splits()[2]), "world");
    }

    #[test]
    fn set_buffer_replaces() {
        let mut pts = PreTokenizedString::from_text("old");
        pts.set_buffer(
            "new text".to_string(),
            vec![Split {
                range: 0..3,
                token_id: None,
            }],
        );
        assert_eq!(pts.buffer(), "new text");
        assert_eq!(pts.split_text(&pts.splits()[0]), "new");
    }

    #[test]
    fn refine_splits_keeps_buffer() {
        let mut pts = PreTokenizedString::from_text("hello world");
        pts.refine_splits(vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..11,
                token_id: None,
            },
        ]);
        assert_eq!(pts.buffer(), "hello world");
        assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
        assert_eq!(pts.split_text(&pts.splits()[1]), " world");
    }

    #[test]
    fn tokenize_text_splits() {
        let pts = PreTokenizedString::from_text("ab");
        let ids = pts
            .tokenize(|text, out| {
                out.extend(text.bytes().map(u32::from));
                Ok(())
            })
            .unwrap();
        assert_eq!(ids, vec![97, 98]);
    }

    #[test]
    fn tokenize_mixed_splits() {
        let buffer = "helloXworld".to_string();
        let splits = vec![
            Split {
                range: 0..5,
                token_id: None,
            },
            Split {
                range: 5..6,
                token_id: Some(99),
            },
            Split {
                range: 6..11,
                token_id: None,
            },
        ];
        let pts = PreTokenizedString::new(buffer, splits);
        let ids = pts
            .tokenize(|text, out| {
                out.push(text.len() as u32);
                Ok(())
            })
            .unwrap();
        assert_eq!(ids, vec![5, 99, 5]);
    }

    #[test]
    fn tokenize_empty() {
        let pts = PreTokenizedString::from_text("");
        let ids = pts
            .tokenize(|_, out| {
                out.push(1);
                Ok(())
            })
            .unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn tokenize_propagates_error() {
        let pts = PreTokenizedString::from_text("x");
        let err = pts.tokenize(|_, _out| Err("boom".to_string())).unwrap_err();
        assert_eq!(err, "boom");
    }
    #[test]
    fn token_limit_skips_merging_but_validates_remaining_pieces() {
        let pts = PreTokenizedString::new(
            "abcdefghi".into(),
            (0..3)
                .map(|i| Split {
                    range: i * 3..i * 3 + 3,
                    token_id: None,
                })
                .collect(),
        );
        for direction in [TruncationDirection::Left, TruncationDirection::Right] {
            for limit in [0, 2, 3, 5, 9, 10] {
                let mut merged = 0;
                let mut validated = 0;
                let (ids, truncated) = pts
                    .tokenize_with_limit(
                        limit,
                        direction,
                        |text, ids| {
                            merged += 1;
                            ids.extend(text.bytes().map(u32::from));
                            Ok(())
                        },
                        |_| {
                            validated += 1;
                            Ok(())
                        },
                    )
                    .unwrap();
                let full: Vec<_> = pts.buffer.bytes().map(u32::from).collect();
                let expected = match direction {
                    TruncationDirection::Left => &full[full.len().saturating_sub(limit)..],
                    TruncationDirection::Right => &full[..limit.min(full.len())],
                };
                assert_eq!(ids, expected);
                assert_eq!(truncated, limit < full.len());
                assert_eq!(merged, (limit / 3 + 1).min(3));
                assert_eq!(merged + validated, 3);
            }
        }
        assert!(
            pts.tokenize_with_limit(
                0,
                TruncationDirection::Right,
                |_, ids| {
                    ids.push(1);
                    Ok(())
                },
                |_| Err("invalid discarded piece".into())
            )
            .is_err()
        );
    }
}
