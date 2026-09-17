> Policy override, 2026-09-17: the user explicitly lowered the scoped point
> floor to 1.02x. Keep the paired confidence interval above 1.00x and all
> correctness/regression/resource guards. A/A uncertainty is reported separately;
> it no longer raises the point floor. Historical thresholds below describe
> their original runs, not the active policy.

# Scoped `.st` construction evaluator v1

This is a separate evaluator campaign for the user-requested `.st` format,
not the frozen general encode champion evaluator. Runtime source is frozen
at `6972e461c8061afd88efb70d0a97acad98c7d822` while this evaluator is established.

`prepare.py` copies the existing twelve-model 2026-09-09 load panel, records
hashes/configuration, and freezes 19 inputs: empty text, whitespace, Unicode,
special tokens, code, JSON, and three lengths from each of four LongBench
contexts. No inputs are selected from candidate results. All copied source
JSON and generated sidecars live outside the original fixture directories.

`st-eval check` compares complete Hugging Face IDs with JSON, cached `.st`,
direct `.st`, and `.tkz` for scalar, nested batch, and flat-ragged batch output,
with and without special tokens. It also compares every vocabulary mapping.
Unsupported input or any mismatch is a failure, not an excluded measurement.

`run.py PARENT CANDIDATE DATA OUTPUT` checks both binaries on every model before
and after twelve paired rounds. Each process loads and drops six tokenizers.
The first load is retained separately; the median of the following five is
the warm-filesystem-page load measurement. AB/BA alternates by whole round.
All raw rounds are retained. The primary score is the equal-model geometric
mean of median paired `.st` ratios; JSON and `.tkz` are guardrails. Confidence
intervals resample complete round indices, preserving correlation across models.
The separate `encode` command times scalar, nested, and ragged materialization
and destruction over the same corpus, with one warmup and five retained passes.

Before a candidate: run identical-binary and independently built same-source
A/A. Predeclared scoped retention floor is 1.03x with paired 95% interval above
1.00x, raised if A/A requires it. No model, JSON/TKZ, encode, or CPU-class
regression outside its A/A band. Keep construction RSS within 5% and binary
size within 2%. Report snapshot size separately; format-size tradeoffs require
an explicit experiment card. Recheck Apple ARM and GCP AMD before declaring
portable improvement. No candidate changes this evaluator or its frozen data.

Build with stable Rust, locked dependencies, default release optimization and
`CARGO_PROFILE_RELEASE_DEBUG=1`; record exact compiler and binary hashes. No
native-CPU flags. Profiling uses the same binary, outside timing runs. This
GCP Intel VM has no hardware cycles/instructions events even as root; use
`sudo perf record -e cpu-clock -F 997 --call-graph dwarf` and heaptrack, retaining
capture status and lost-sample counts. Do not claim PMU instruction evidence.
