# Cache experiment results — 2026-09-20

None of the four changes established an encode speedup, so the runtime implementation
remains unchanged. The default hasher in the local long-piece cache was about 2%
slower on warm scalar encoding in this local screen. Atomic locking, boxed shared
keys and the default shared-cache hasher were inconclusive. Boxed keys reduce the
key descriptor from 24 to 16 bytes on this M2, without a demonstrated encode win.

## Comparison on paths affected by each change

Values are throughput changes relative to the parent; positive means faster.
Brackets show paired bootstrap 95% intervals, in percentage points. Shared-cache
rows summarize the two Gemma corpus cells. The local-hasher row summarizes all
nine cells. These different scopes are intentional and explicit; the original
predeclared nine-cell results are also preserved below.

| Change | Cold scalar | Warm scalar | Warm nested batch | Warm ragged batch |
|---|---:|---:|---:|---:|
| Atomic spinlock | +0.20% [-7.68, +10.55] | -1.53% [-3.64, +3.01] | -0.20% [-3.57, +2.30] | +0.40% [-3.08, +5.99] |
| `String` → `Box<str>` (shared) | +2.18% [-8.67, +5.42] | +0.00% [-2.76, +2.24] | +1.12% [-2.63, +3.90] | +0.13% [-2.51, +1.98] |
| Fx → default hasher (local long keys) | +0.67% [-0.93, +2.73] | -2.02% [-2.93, -1.43] | -2.63% [-7.64, +1.77] | -1.90% [-6.16, +5.72] |
| Fx → default hasher (shared) | -1.84% [-11.82, +6.22] | -0.19% [-5.87, +4.12] | -0.73% [-3.78, +2.35] | -0.89% [-3.29, +0.53] |

Cold scalar is the first scalar traversal of each newly loaded tokenizer; pieces
can repeat within that traversal. Warm results use the median of passes 2–6.
Batch calls follow scalar calls, so these are not cold-batch measurements.
Output allocation and destruction are included; model loading is excluded.

The shared-cache intervals all include no difference. The boxed-key cold point
estimate exceeds 2%, but its interval includes losses, so it fails the gate.
The local default-hasher warm scalar result is -2.02%, with a 95% interval of
-2.93% to -1.43%. Eight of its nine cell medians are negative. This is local
comparative evidence favoring the existing Fx hasher, not a portable guarantee.
Both variants use std::collections::HashMap; only the hasher changes.

## Actual cache access

The current direct fused encoder passes `use_shared_cache=false` on short and
long piece misses. This differs from the historical shared-cache experiments
that motivated the initial card. A separate instrumented baseline confirmed zero
shared-cache lookups or inserts for GPT-2, Qwen-3 and GPT-OSS in all selected
cells. They are controls for shared-cache changes, not evidence of a mutex win.

The diagnostic counted the following during one complete six-pass invocation:

| Model / corpus | Shared lookups | Hits | Inserts |
|---|---:|---:|---:|
| Gemma-3 / LongBench | 99,455 | 85,612 | 13,843 |
| Gemma-3 / ShareGPT | 36,052 | 28,546 | 7,506 |
| GPT-2, Qwen-3, GPT-OSS / both corpora | 0 | 0 | 0 |

Counts depend on scheduling and reuse. The diagnostic uses extra atomic counters;
its timing is discarded and its source is never a promotion candidate. It passed
the same Hugging Face parity check before collecting counts. Source and counts
are saved in `raw/diagnostic-source.txt` and `cache-access-counts.json`.

The two-cell Gemma aggregate was added after discovering this call-path limitation.
It is an explanatory subset, not a replacement promotion score. The original
matrix and all raw outcomes are retained without filtering.

## Original predeclared nine-cell summary

| Change | Cold scalar | Warm scalar | Warm nested batch | Warm ragged batch |
|---|---:|---:|---:|---:|
| Atomic spinlock | -0.45% [-3.70, +2.27] | -0.66% [-2.56, +1.31] | +4.44% [-1.90, +13.53] | +0.94% [-4.89, +5.65] |
| `String` → `Box<str>` (shared) | +0.84% [-1.55, +2.57] | -0.15% [-0.95, +0.48] | +9.60% [-2.22, +27.75] | +2.39% [-3.50, +8.54] |
| Fx → default hasher (local long keys) | +0.67% [-0.93, +2.73] | -2.02% [-2.93, -1.43] | -2.63% [-7.64, +1.77] | -1.90% [-6.16, +5.72] |
| Fx → default hasher (shared) | -0.89% [-3.61, +1.43] | -0.21% [-1.49, +0.95] | +0.08% [-11.72, +12.05] | +3.76% [-0.47, +8.21] |

Apparent batch gains in the atomic and boxed rows are not supported: intervals
cross parity, and most constituent configurations never touch the changed cache.
They must not be cited as shared-cache performance gains.

## Calibration and limits

| A/A comparison | Cold scalar | Warm scalar | Warm nested batch | Warm ragged batch |
|---|---:|---:|---:|---:|
| Same binary | -2.14% [-7.30, +2.94] | +0.74% [-0.72, +9.00] | +7.53% [-19.58, +22.02] | -1.73% [-15.00, +16.48] |
| Independent same-source rebuild | +1.39% [-3.75, +8.71] | -0.63% [-2.67, +8.18] | -4.32% [-20.61, +8.34] | +3.15% [-15.90, +14.23] |

A/A used six pairs per cell for each comparison; each candidate used twelve
counterbalanced pairs per cell. The independently rebuilt executable was byte
identical to the first parent build. Every round and outlier is saved. This busy
8 GiB Apple M2 host could not reliably resolve 2% changes in the sub-millisecond
warm fused calls. No user applications were stopped, no affinity changed and no
cloud machines started. Disk remained above 7 GiB free during the main screen.

RAYON_NUM_THREADS=4 controls the global pool. The existing dedicated BPE pool
still uses eight threads on this host, independently of that variable. Gemma's
large scalar inputs can use this pool; four-wide outer BPE batches disable inner
parallelism. The extra global-one-thread cell is GPT-OSS/LongBench, which bypasses
the shared cache. It is not an uncontended shared-lock benchmark. Thread policy
was unchanged between parent and candidates.

No AMD/Intel, holdout, construction-time, peak-RSS, collision-adversarial corpus,
or full release-matrix conclusion is available. An interval including parity
means no established difference; it does not prove equivalence. The atomic
candidate tests a spinlock per shard, not a lock-free hash table. It can consume
CPU while waiting and does not guarantee fairness. No Miri, Loom or sanitizer
run was performed; stress tests do not exhaust all concurrent executions.

## Source and validation

Frozen runtime: `3a0becba4995de38b8e45bedcb1bd9e9e2f4c4a7`.
Frozen evaluator/cards: `1183b2564971ed49e7d14a1a7a1f3350a4b63751`.
All four runtime variants are independent children of the evaluator/card commit.
The evaluator source, Cargo lockfiles, fixtures and build settings were unchanged
between them. The existing st-eval runner was used without edits.

| Variant | Source SHA | Focused BPE tests |
|---|---|---:|
| Atomic spinlock | `258032ec014adec246633e7d8d8a6e3ca82b2447` | 21 passed |
| `String` → `Box<str>` (shared) | `2740e8dfd58910e7ca2d7e085da456a934a9cd2e` | 18 passed |
| Fx → default hasher (local long keys) | `fc368636a915466f3bcdf12ebaf9a32373eb6cb7` | 18 passed |
| Fx → default hasher (shared) | `1273feea78f869b4a5afb78fb826e8577403a741` | 18 passed |

Formatting and all pre/post exactness checks passed. The check compares complete
Hugging Face token IDs through JSON, cached ST and direct ST, with special tokens
on and off, scalar/nested/ragged output, and vocabulary lookup. It covers both
timed corpora plus the 19-input ST semantic corpus. The atomic tests additionally
cover eight-thread mutual exclusion/publication, poisoning after panic and
concurrent same-shard eviction. Model/source/binary hashes are in `manifest.json`
and `builds.json`; test, build and parity logs are in `raw/`.

`layout.rs` confirms `String`=24 bytes, `Box<str>`=16 bytes, and key/value descriptors
of 48 versus 40 bytes. This is a descriptor-size observation, not measured total
RSS or one allocation saved per entry: both key types still own allocated bytes.

## Reproduction and disposition

The fixed data selection, exact build command, environment, API timing and
acceptance rules are in `protocol.md`, `manifest.json` and `builds.json`.
`run.py` records six-pass timing rows without filtering; `analyze.py` regenerates
the summaries from these committed raw files:

```sh
python3 autoresearch/cache-screen-20260920/analyze.py \
  aa-identical aa-independent atomic boxed long-standard shared-standard
```

Inputs and immutable executables remain in
`/tmp/snaptokens-cache-evidence-20260920/{data,bin}`. Raw evidence and summaries
are committed on `perf/cache-screen-20260920`. Each prototype remains on its
own `perf/cache-*-20260920` branch for inspection. The checked-out result branch
contains the baseline runtime plus evidence only; no candidate or diagnostic
runtime change is retained. `feat/st-format` and main are untouched, and nothing
was pushed or released.
