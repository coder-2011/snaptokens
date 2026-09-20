# Local cache experiments, 2026-09-20

Runtime baseline: `3a0becba4995de38b8e45bedcb1bd9e9e2f4c4a7`.
The commit containing this protocol freezes the evaluator and cards before
runtime edits. All four candidates start from that same commit. This is a
requested local mechanism/target screen, not a general champion campaign.
Historical general evaluator and holdout blockers remain unchanged.

## Fixed evaluator

Use the existing `autoresearch/st-eval` without changing its code, locks, or
six-pass encode timing. Build with Rust 1.97.0, `cargo build --release --locked
--manifest-path autoresearch/st-eval/Cargo.toml`, no RUSTFLAGS, default features,
CARGO_PROFILE_RELEASE_DEBUG=1, CARGO_BUILD_JOBS=2 and one shared owned target.
Copy each executable to an immutable name and record its SHA256 and source SHA.
Rebuild the same source once for independent-build A/A calibration.

Fixtures and extraction hashes are in manifest.json. LongBench-v2: first eight
contexts, each capped at 65,536 UTF-8 bytes. ShareGPT52K: first 64 conversations,
joined as `[speaker]: message` with blank lines, capped at 4,096 UTF-8 bytes.
A truncated UTF-8 suffix is discarded. These are diagnostic subsets, not the
sealed holdout. Models: GPT-2, Qwen-3, GPT-OSS, Gemma-3. Each corpus runs with four
Rayon threads; GPT-OSS/LongBench additionally runs with one. No CPU affinity.

`st-eval check` compares full Hugging Face IDs for JSON, cached ST and direct ST,
scalar/nested/ragged APIs, special tokens on/off, and vocabulary lookup. Run on
both timed corpora and the existing 19-input ST semantic corpus before timing
and again after each pool. A failure stops that candidate before timing.
Use copied JSON fixtures and reuse baseline-generated snapshots for candidates.

Each encode process loads once and executes six scalar/batch/ragged passes.
Output construction and destruction are timed; loading is excluded. Report
first-pass scalar separately as cold-start cache behavior (reuse can occur
within the corpus). Report the median of passes 2–6 separately for each warm
API. Batch/ragged are already warmed by preceding calls: no cold-batch or novel
throughput claim is allowed. All raw passes, including outliers, are retained.

A/A: six alternating same-binary pairs and six alternating independently built
same-source pairs, in every cell. Candidate screen: twelve counterbalanced
parent/candidate pairs per cell, frozen before observing results. Order alternates
by round and cell. No concurrent builds, tests, or profiling during timing.
Do not stop early to retain a candidate. Other user applications remain running;
A/A must reveal noise rather than silently removing competing processes.

## Decision

Compute each cell/API's median log(parent_ns/candidate_ns), then the exponential
mean across equally weighted cells. Keep cold and warm summaries separate.
Use paired bootstrap rounds (10,000 draws, fixed seed 20260920) for 95% intervals.
Local promising result requires >=1.02x and interval above 1.00x, no cell loss
outside its A/A band (max absolute same-source median log ratio, plus uncertainty).
A failed correctness check rejects. Insufficient precision is inconclusive,
not a win. Micro/component observations cannot establish a general speedup.

Atomic lock: focused mutual-exclusion, poison and same-shard eviction stress
must pass before timing; document the unsafe ownership/Acquire-Release proof.
Atomic locking still serializes each shard and is not a lock-free cache.
Box<str>: immutable keys retain exact bytes; String insertion API stays unchanged.
Hasher candidates change only one cache family at a time, retaining full keys
and equality checks; RandomState seeding is outside steady-state timing.

No runtime candidate is promoted to main or feat/st-format by this screen.
Apple/AMD/Intel confirmation, complete release guardrails and sealed holdout
would still be required for a general performance claim.
