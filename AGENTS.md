# Snaptokens AutoResearch

This repository optimizes exact BPE tokenization. Performance work is accepted only when it removes measured work without narrowing correctness, portability, or the output contract.

## CRITICAL: never release without explicit approval

Never publish or release a package without the user's explicit approval for that specific release. This covers `cargo publish`, `maturin publish`, `twine upload`, pushing version tags, creating GitHub releases, and manually triggering or approving publish workflows or their environments. Merging code, `--dry-run` checks, and local packaging verification are allowed; the release itself never is. If a task appears to require a release, stop and ask.

## Read order

Before proposing or editing an optimization:

1. Read [`autoresearch/state.md`](autoresearch/state.md).
2. Read the latest relevant retained and rejected entries in [`log.md`](log.md). Do not load the entire ledger when a focused search is enough.
3. Read the source that owns the measured cost and its existing tests.
4. Read the benchmark runner that will judge the candidate.

If `autoresearch/state.md` says the baseline is not frozen, do not benchmark or modify tokenizer code. Freeze the intended tree first and record its commit.

## Non-negotiable invariants

- Return exactly the same token IDs as Hugging Face Tokenizers for every supported tokenizer pipeline.
- Preserve row lengths and ordering for nested and flat-ragged batch APIs.
- Preserve added-token behavior, normalization, pre-tokenization, ByteLevel transformation, byte fallback, `ignore_merges`, post-processing, JSON loading, and direct `.tkz` loading.
- Keep the public API and tokenizer formats unchanged unless the experiment explicitly targets them.
- Keep a portable scalar fallback. Architecture-specific code must be runtime-gated and independently exact.
- Keep output allocation and destruction inside steady-state encode timing when the compared API contract requires them.
- Keep novel-input and repeated-input measurements separate. A full-output cache is not a general novel-input tokenizer speedup.
- Never hide a loss, failed parity check, outlier, unsupported tokenizer, or unmeasured workload.

Correctness is absolute. Throughput is the primary optimized metric. Construction time, peak RSS, and binary size are separate guardrails, not terms in an invented utility score.

## Candidate shape

Every candidate must start from the recorded champion and test one hypothesis. Before editing, write an experiment card in `log.md` containing:

```text
Parent SHA:
Hypothesis:
Measured hot cost:
Invariant that makes the shorter path exact:
Representation being preserved or changed:
Expected winning strata:
Expected adverse strata:
Smallest files that need changing:
Mechanism evidence:
Acceptance rule:
Rejection rule:
```

If the hot cost, invariant, or acceptance rule is unclear, stop and inspect more code. Do not use implementation as a way to discover the question.

Change the smallest possible surface:

- Prefer removing work or improving a representation over adding a new heuristic.
- Do not refactor adjacent code, rename unrelated symbols, reformat untouched sections, add speculative knobs, or broaden public APIs.
- Do not add a dependency, feature, abstraction, cache, `unsafe` block, or model dispatch unless the measured mechanism requires it.
- Keep public API documentation, including Python and command-line help; do not add doc comments to private helpers. Use sparse, single-line comments to explain important abstractions whose purpose or contract is non-obvious, and consequential ordering or edge cases. Do not restate names or nearby code, or require a comment merely because a function or branch exists.
- Add only focused tests for semantic boundaries, fallbacks, or unsafe invariants. Do not duplicate coverage for straightforward branches.
- Explain important, non-obvious unsafe contracts once at the abstraction that owns them. Do not repeat the same proof at every call or narrate nearby guards. Prefer a safe representation or a hoisted check when it measures equivalently.

## General and specialist tracks

The general champion is selected across tokenizer structures, corpora, input shapes, API contracts, and CPU classes.

Structure-derived dispatch is allowed when its guard follows from tokenizer configuration and applies to every tokenizer with that structure. New branches on model names, benchmark corpus content, benchmark environment variables, or published cell sizes are specialist work and cannot promote the general champion.

Specialist experiments must be labeled as such in `log.md`, scored separately, and never included in a general-speed claim unless they later pass the complete general gate.

## Evaluator boundary

Optimization candidates may not change:

- `benchmarks/` code, manifests, corpora, tokenizer artifacts, reference outputs, round counts, thread counts, affinity, or timing boundaries.
- The evaluator version or the script that computes promotion scores.
- Correctness probes or which outputs are materialized.

Evaluator improvements are a separate campaign. Land them first, freeze a new evaluator version, rerun A/A calibration, and rebaseline every candidate. Never mix evaluator and tokenizer changes in one experiment.

Static review must flag any candidate that:

- Detects a model label, corpus, executable, harness, environment variable, or measured input size.
- Reuses a previous request's complete output in the novel-input track.
- Skips normalization, splitting, merging, post-processing, row construction, or output materialization.
- Changes CPU affinity, worker count, warmup, sample count, corpus windows, or timing scope.
- Benchmarks a stale binary, dirty source tree, mismatched lockfile, or unrecorded build command.
- Filters rounds, cells, or outliers after seeing the result.

Reject an evaluator exploit even when it improves the score. Repairing the evaluator invalidates results produced by the vulnerable version.

## Evaluation funnel

Run the cheapest decisive gate first. A later gate may be skipped only after an earlier gate rejects the candidate.

### 0. Source audit

- The parent and candidate are committed, clean, and identified by full SHA.
- Benchmark inputs, evaluator files, build flags, and dependency locks match.
- The diff contains only the declared mechanism and its focused semantic coverage.
- `cargo fmt --all -- --check` passes before any retained result is reported.

### 1. Exactness

- Run the smallest existing unit or differential tests covering the changed invariant.
- Compare complete candidate token IDs with Hugging Face on every timed input.
- Compare nested and flat-ragged row boundaries when batch code changes.
- Run exactness before timing and again after the timed pool.
- Do not time a candidate after any parity failure.

Unsafe scanner, boundary, cache, or merge changes require the focused reference comparison appropriate to the invariant. Use Miri, fuzzing, or property tests only when the changed boundary warrants them.

### 2. Mechanism screen

Measure the cost named in the card. Depending on the hypothesis, use profiles, instructions, allocations, cache behavior, binary inspection, or contention evidence. A microbenchmark may reject a mechanism but cannot establish a general win.

### 3. Target screen

Run representative cells where the measured cost is material, including the predicted adverse shape. Use immutable parent and candidate binaries, fixed rounds, disjoint warmup, novel rotating inputs, and counterbalanced execution order.

Early stopping is allowed only to reject. A candidate considered for retention must finish the predeclared sample count.

### 4. Development matrix

Score the candidate against its immediate parent, never against Fastokens, Gigatoken, or Hugging Face. Competitors are report context, not the optimization reward.

For each equal-weighted stratum, compute paired candidate/parent throughput ratios. The primary aggregate is:

```text
exp(mean_strata(median_rounds(log(candidate throughput / parent throughput))))
```

Use the A/A calibration recorded in `autoresearch/state.md` to decide whether an apparent gain or regression exceeds noise. The initial campaign target is a `1.02x` point estimate with a paired 95% confidence interval above `1.00x`, plus no structural or CPU-class regression outside its calibrated band.

### 5. Sealed holdout

The holdout must run outside the optimization agent's readable filesystem. It accepts only a committed candidate SHA and returns correctness, aggregate score, guardrail status, and evaluator version. It must not reveal corpora, tokenizer artifacts, seeds, per-cell results, or actionable failing strata.

Use no more than three holdout queries during one 40-candidate campaign. Retire a holdout after its details are disclosed. Repeated querying of an aggregate is still training on the holdout.

### 6. Portability and release

A general candidate must pass fresh-process confirmation on Apple ARM, AMD x86-64, and Intel x86-64. Weight CPU classes equally rather than giving a cloud provider with more instances more influence.

Before promotion, run the complete relevant suite, strict lint and documentation checks, package checks, load and construction measurements, peak RSS, binary size, and the full release matrix. Preserve raw rounds and every observed loss.

## A/A calibration

Before the first campaign and after any evaluator change, compare the champion with itself under the exact candidate protocol. Include identical-binary and independently built same-source comparisons. Record per-cell and aggregate false gains on Apple, AMD, and Intel.

Acceptance and regression bands must come from this observed distribution. Do not loosen a gate after seeing a favored candidate.

## Experiment retention

- Promote only a committed candidate that passes every required gate.
- Treat a combination of two retained changes as a new candidate; composition is not assumed to preserve either win.
- Record failed ideas as carefully as winners, including the target, command, raw result location, and why broader evaluation stopped.
- Revert rejected source changes in full. Remove only scratch files created by that experiment after their durable evidence is recorded.
- Update `autoresearch/state.md` whenever the champion, evaluator, noise band, nearby goal, holdout budget, or ranked headroom changes.

The repository's publication target is `main`. Parallel explorers must use isolated worktrees and must not touch the same files. Only promoted candidates land on `main`; an experiment performed directly on `main` is rejected with `git revert` so its evidence remains auditable.

Never overwrite or discard unrelated user changes. If the working tree contains changes that are not part of the recorded baseline, stop and resolve ownership before benchmarking.

## Autonomous campaign loop

One iteration is:

1. Read current state and relevant prior failures.
2. Profile the champion or select the highest-evidence open cost.
3. Write one experiment card.
4. Cross-review the card for exactness risk, likely adverse cases, and evaluator gaming.
5. Implement the smallest candidate in an isolated committed state.
6. Run gates in order.
7. Retain or reject based only on the predeclared rules.
8. Record the result and compact current state.
9. Continue while the campaign budget and evaluator remain valid.

Stop the campaign when any of these occurs:

- The baseline or evaluator is no longer clean and reproducible.
- A correctness failure cannot be explained and contained.
- An evaluator exploit or information leak is found.
- The holdout-query budget is exhausted.
- Forty candidates or 24 elapsed experiment hours are reached.
- Eight consecutive candidates fail their mechanism screen, indicating that the profile or search lanes need human review.
- Host instability exceeds the recorded A/A band or repeated infrastructure failure prevents trustworthy measurements.

The campaign optimizes useful experiments per hour, not edits per hour. A small, well-rejected experiment is progress; an unmeasured optimization is not.
