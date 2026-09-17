//! Hugging Face exactness tests for the public tokenizer API.
//!
//! In-memory unit tests stay in `src/`. Throughput regression lives in
//! `benchmarks/`, not this crate.

mod common;
mod correctness;
mod extended;
mod pipeline;
mod unigram;
