# Snaptokens AutoResearch state

Updated: 2026-09-17

This file is the compact mutable context for the next optimization campaign. Stable rules live in [`../AGENTS.md`](../AGENTS.md); complete retained and rejected evidence lives in [`../log.md`](../log.md).

## Status

### Scoped `.st` load campaign (reopened 2026-09-17)

Active root `/Users/namanchetwani/Projects/snaptokens-st-format`, branch
`feat/st-format`. Incoming `.st` tree frozen at `6972e461c8061afd88efb70d0a97acad98c7d822`;
validation-repaired runtime parent `95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71`.
E8 is retained by explicit user request. Root now combines E8 and validated Unigram support. E1 is rejected by Apple v2
encode guards and removed; combined performance gates remain pending. The general
encode champion and `main` are unchanged. No release authorized. The user also explicitly requires `.st` support for both
BPE and Unigram; feature `73f73de` adds validated Unigram snapshots
while preserving existing BPE v1 files, now integrated into the root. This is now part of the active goal.

Evaluator v1 `03c3160` has a scoring defect: log of an arithmetic median differs
from median log ratio for twelve rounds. Evaluator-only v2 is frozen at
`80908bece365e7008498c2e270bb358cc05b4f7b`, branch `eval/st-load-v2`, with unchanged
runtime, inputs, rounds, timers, binaries and bootstrap. Two scorer regression
tests pass. Fresh identical/independent A/A and E1 rebaseline are underway on
Intel, AMD and Apple. The user lowered the point floor to 1.02x on 2026-09-17; confidence above 1.0
and regression guards remain required. V2 model bands must use the stricter
old/new log half-width. V1 results below are historical and cannot retain a
candidate. Rejected versions remain rejected.

- E1 `0cfef16`, String arena: REJECTED by Apple v2 Llama/Mistral scalar and
  DeepSeek batch encode guards (see latest log). Runtime reversed from root.
  Historical v1 ST Intel 1.146238x, AMD 1.120676x,
  Apple 1.121027x; all load-model/format guards pass. Intel encode/RSS guards
  pass. Workspace: 129 unit, 40 integration (9 ignored), 3 binding, 1 doctest;
  all-targets build, fmt, strict Clippy/docs and package creation pass. Full
  package verification and both Python CI matrix environments pass (36 tests
  each). Byte-identical artifact reconstruction, no-default-feature and MSRV
  checks are queued after PMU timing.
- E2 `f59e94f`, bulk integer decode: rejected on Intel JSON 0.955269x,
  eight model bands, despite ST gains on all three CPUs. Restored in `99e209f`.
  PMU same-binary argv0 audit found no significant bias on three JSON models.
- E3 `5503bd1`, allocated inverse map: rejected transfer 0.951866x,
  GLM/Nemotron heap growth/trimming (brk 2,228 versus 28 on GLM). Restored
  in `f8993ba`; complete counter/syscall evidence retained.
- E4 `5444d2b`, combined BMP/length pass: load Intel 1.225615x, AMD 1.168775x,
  but Intel GLM scalar encode 0.890673x falls below frozen 0.908286x band.
  Rejected and restored in `e1536f9` on `perf/st-character-table-pass`.
- E5 `5fa3c23`, decoded-ID scratch reuse: Intel ST 1.013836x,
  CI [0.977321,1.040477], below 1.05x. Rejected and restored in `2cd86ef`.
  Already-running AMD load pool is diagnostic; v2 skip markers installed.
- E6 `a369ab2`, BMP length guard with original allocations: 129 unit tests and
  pre/post transfer parity pass; warm 1.169722x, cycles 1.177214x, instructions
  1.180454x; all twelve models improve. Full v2 pools queued after E1 on all
  three hosts with automatic load-gate rejection before encode guards.
- E7 checksum-only screen: 2.049760x with four workers, 0.970323x with one;
  small first loads regress. No loader edit yet; dependency-feature baseline
  must be controlled before runtime comparison.
- E8 `84a0347`, paired exact hash-tail arithmetic: isolated branch
  `perf/st-paired-hash-tail`; retained there by explicit user request after
  transfer load 1.027395x and cycles 1.037345x. Instructions 0.992121x; full
  CPU-class/encode guards pending. Do not report portable retention yet.

Current combined runtime `f786899` passes the complete package/Python matrix
(129 unit, 41 integration + 9 ignored, 3 binding, 1 doctest; 36 Python per env).
Unigram evaluator `94e35d1` passes full HF parity on T5/UMT5 and 25 inputs;
Intel direct ST versus JSON is 1.157132x (CI 1.135892–1.173646), cached 1.124368x.
This is a two-model format baseline, not promotion. ALBERT normalizers remain
unsupported; MT5 lacks tokenizer.json; NLLB is BPE. Matcher construction is ~60%
of load cycles. E9 `8a600dc` sparse ranked validation is in PMU screen; E10
`ea75ed9` unique-Unigram-vocabulary query removal is queued after it, both isolated.

Task files on pre-existing `snaptokens-bench-20260909-{intel,amd}` in us-central1-a:
`~/st-campaign-20260917`. Do not stop these machines or touch other work.
`~/st-v2-rebaseline-20260917.sh` waits for prior pipeline, then runs v2 AA/load
and guard pools sequentially; statuses under `evidence/v2-*`. Apple pipeline
`/tmp/st-apple-v2-20260917.sh`, results `/tmp/st-apple-campaign-20260917`, frozen
data `/tmp/st-apple-data-20260917`; no competing heavy local work during timing.
All immutable evaluation binaries use Rust 1.98.1 default release with debug=1.

Task-owned `snaptokens-st-pmu-20260917`, us-east1-b, c4-standard-4, has working
cycles/instructions/branch-miss PMU, unsupported generic cache misses, and an
8-hour automatic stop. Evidence and builds under `~/st-campaign-20260917`.
`~/st-e8-pmu-build-20260917.sh` owns current timing; subsequent
`~/st-extra-source-checks-20260917.sh` uses its own source worktree. Source paths
and immutable binaries are recorded; root-owned raw perf captures remain remote.
The user explicitly waived the API-key preflight; local disk currently 12 GiB.
No stale automation found; normal user browsers are preserved.

Next: freeze v2 calibrated bands before interpreting new candidate results;
record remaining E1 diagnostics without retention, then test separate new mechanisms
against the recorded parent. Integrated E8/Unigram is a new combined tree;
component gains cannot establish its performance. E6 remains isolated pending gates.
Historical general-campaign blockers below do not override this scoped campaign.

### Scoped Unigram T5 encoding campaign (reopened 2026-09-15; parallel-pipeline candidates retained)

The user reopened this lane with an end-to-end target of roughly `70x` Hugging Face. A fresh Apple M2 phase-timer diagnostic (temporary source, fully reverted) attributed 61.5% of the pinned 20-input single-document wall to serial work — `121.8 ms` fused WhitespaceSplit+Metaspace walking and `38.3 ms` charsmap normalization against a `100.2 ms` parallel model phase — explaining why every prior matcher/Viterbi micro-candidate moved little. Two candidates are retained on `perf/unigram-json`: (1) partitioned fused encode — large documents cut at whitespace-aligned boundaries and run the word walk plus per-piece Viterbi per partition on the shared pool (`1.5491x`, 20/20 documents faster); (2) raw-text partitioning — with a load-time proof that printable ASCII is charsmap-identity and per-whitespace-byte anchor safety, the charsmap itself runs inside partitions and unchanged partitions borrow instead of copying (`1.0874x`, 16/20). Together they measure `2.0056x` over the prior branch head with every document faster (per-document paired medians, seven counterbalanced cycles); quiet-host local parity runs measure `54.6x`–`62.5x` Hugging Face, and the canonical GCP basis projects to roughly `53x` from the recorded `26.46x` parent. All retentions passed complete-ID Hugging Face parity, new boundary-stress integration tests (combining mark after space, CRLF, ideographic/thin spaces, embedded added tokens, whitespace-free megabyte runs), and an extended `fuzz_unigram` that drives >16 KiB inputs through the partitioned dispatch.

Closed by measurement on 2026-09-15 (see `log.md` for cards and raw evidence): split memoization in three representations (thread-local FlatCache `0.977x`, sharded shared cache `0.711x`, allocation-free 2 MiB direct-mapped table `1.0030x`) — bucketed timing shows short-split Viterbi at `81–266 ns` per occurrence, already at cache-probe parity; partition-granularity tuning (`1.0016x`); and a single streaming automaton scan per partition (`1.0056x`) — per-word iterator/call overhead is smaller than its bucket-derived bound. The earlier matcher/state/output-chain rejections (`0.931x`–`0.973x`) remain in force.

Cross-host confirmation completed on task-owned `snaptokens-unigram-gcp-20260915` (`c4-standard-4`, Intel Xeon Platinum 8581C, us-east1-b): complete-ID parity passed for parent and candidate; the retained head measures `1.448x` over the branch parent (`247.42` versus `170.84 ms` medians, three counterbalanced rounds), `34.93x` Hugging Face end to end on that equal-core canonical basis (`23.96x` parent). The VM is stopped with its evidence preserved on disk; local M2 quiet-host runs measure `54.6x`–`62.5x` against a single-threaded Hugging Face.

The `70x` target is not reached, and the post-partitioning search is measured out on both hosts. Closed on the GCP host with quantitative evidence (`log.md` 2026-09-15 entries): byte-classified walk `1.0230x` (below the 3% floor); the streaming partition scan re-screened at `0.852x` (worse than its neutral M2 result); and a standalone mechanism screen that closes manual double-array traversal (`214.1` versus `199.8 ms` per single-thread stream round), marker-prefix state skipping (saves under 1% because the marker bytes ride hot predicted states), and two-way interleaved scanning (`235.3 ms`, slower — out-of-order execution already overlaps the dependent loads). The automaton scan is at its measured throughput limit on this CPU class; a further material Unigram gain requires a genuinely different segmentation algorithm or more cores, not another traversal, cache, scheduling, or scan-fusion variant. The local per-document paired-median protocol and the GCP counterbalanced rounds are the screening bases for every verdict above; the `fuzz_unigram` independent-reference target and the T5 integration tests remain the required low-cost exactness gates.

A 2026-09-15 follow-up on `perf/unigram-json` at `b3eb482` exhausted two post-partition sequential representations without retaining either: stack-prefixed Metaspace pieces (`0.9768x` sequential paired-median, 7/20 documents faster) and a serial fused walker for `encode_batch` rows that cannot take nested Rayon (`0.9008x` sequential after isolating the new helper so the parallel driver was textually unchanged; noisy batch median `1.0555x`). Sibling Unigram drivers next to the retained parallel path move sequential code generation enough to lose the pinned screen.

A later structure-derived branch in already-outlined `encode_outer_batch` is retained: Unigram wide batches keep inner parallelism so they take the existing partitioned walker, while BPE still suppresses nested split work. Sequential `encode()` is unchanged (`0.9987x` Intel, `1.0231x` AMD). Wide `encode_batch` of the pinned 20 T5 LongBench contexts is `1.3709x` Intel and `1.3671x` AMD versus the parent batch path (complete-ID parity `5,440,870` tokens; sequential Hugging Face `67.66x` Intel / `80.13x` AMD on 8 cores). A follow-up that moved the Unigram outer `par_iter` onto `bpe_pool` was rejected at `1.0126x` batch on Intel. Matcher/Viterbi micro-representations, split memoization, SIMD/bitmask matchers, streaming partition scans, walk-classification, stack-prefixed pieces, serial-walker siblings, and same-pool outer scheduling are closed. Remaining sequential cost is exact Viterbi; remaining batch headroom versus sequential is ~`7%` same-binary and did not clear the 3% floor. Stop here until a new whole-pipeline profile names a cost outside those lanes.

### PR #17 harness maintenance

PR #17 changes `benchmarks/harness` backend identity bookkeeping and updates its local Snaptokens lockfile entry to `0.2.7`. The full native harness builds in Linux CI using nightly Cargo with `-Z profile-rustflags -Z git=shallow-deps`. This is build validation only: no candidate timing, A/A calibration, or general rebaseline has run for this revision. After landing, freeze the resulting evaluator revision and recalibrate before using it for candidate promotion; historical A/A bands do not validate this changed harness.

The separate `benchmarks/portable/Cargo.lock` still records Snaptokens `0.2.6` against source `0.2.7`. PR #17 does not repair or revalidate that evaluator. The older blocker versions and campaign results below describe their recorded snapshots.

### Current-branch baseline gate

`perf/bpe-miss-scheduling` starts from clean parent `2fff41a84e42bee5bdf195a58888aaefa0002b99`. This is a new Git history, but every tracked file other than `README.md` is byte-identical to the prior `c03b1eb` source tree. Historical source findings may therefore inform a mechanism screen, but every candidate comparison must retain current lockfile, evaluator, manifest, and binary fingerprints. Correctness repair `b11ed21bf1d518be04a691c7e24200e1245ab3b1` disables the generic literal bridge splitter for byte-fallback BPE. Follow-up repair `517a91396692463eaf8c2ab7d474ae9f4f3c4f04` makes a direct whole-piece match legal only when its final decomposition merge produces that token ID. Active branch candidate `dfc251f1277c7a11188f7145b7317604414a02fc` then restores the bridge table after marking unproven byte-fallback identity spellings ineligible and bypassing stale byte-fallback V5 tries.

Candidate `dfc251f` passes the pinned 4.3 MB raw-plus-special Gemma test, all local non-ignored tests, and complete remote Hugging Face IDs for all 32 pinned LongBench documents sequentially and in one batch before and after timing (`24,432,087` characters and `7,160,895` IDs per screen). On the task-owned four-vCPU Intel diagnostic host, eight immutable fresh-process, output-destruction-inclusive sequential parent/candidate pairs yield `3.139765x` geometric throughput with paired 95% interval `[3.088731x, 3.191642x]`; four pairs run parent-then-candidate and four candidate-then-parent. A fresh transfer screen on an independent four-vCPU Intel host derived a different pinned 32-context LongBench input (`24,220,398` characters), selected tokenizer structures from JSON rather than model names, and again passed complete Hugging Face IDs before and after timing. Of thirteen accessible immutable fixtures, only Gemma has BPE byte fallback, no `ignore_merges`, and no ByteLevel pre-tokenizer, so it is the only fixture that reaches the restored splitter. Its eight-pair result is `3.367923x` with paired 95% interval `[3.325491x, 3.410896x]`; structurally ineligible Mistral Nemo is `0.989236x` with `[0.966155x, 1.012868x]`, compatible with its unchanged route. The same runner's Hugging Face context comparisons are roughly `10x` sequential and `8.5–9.0x` batch, but exclude output destruction and are not a promotion metric. These are tokenizer/corpus/shape/CPU diagnostics, not general or competitor-leading speed claims. Experiment 94 evidence is at `~/.cache/snaptokens-exp94-dfc251f-evidence`; transfer evidence is at `~/snaptokens-bpe-transfer` on task-owned host `snaptokens-bpe-transfer-20260914`.

A separate, public-API Tokie rebaseline against the pinned `9b78cc552df73c0fa41451c51e08fe94638fa48c` revision ran the twelve historically eligible warm-repeated `repeated-chat140` cells: GPT-2, GPT-OSS, Mistral Nemo, and Qwen 3 at batches `1`, `32`, and `512`. Current `dfc251f` and Tokie both exactly matched Hugging Face IDs before and after every cell; every cell favored Snaptokens. The geometric mean of the twelve median elapsed-time ratios (`Tokie / Snaptokens`) was `9.347342x` on the same dedicated four-vCPU Intel Xeon 8581C host. The slowest individual median was `5.253542x`; the fastest was `19.194866x`. The independent driver keeps native flat-ragged Snaptokens output and Tokie's public `Encoding` output, including allocation and destruction, but it is not the frozen general evaluator and covers deliberately repeated inputs on one CPU only. Tokie remains inexact on the historical novel-input panels, so this is neither a novel-input nor portable all-competitor claim. Raw rounds, fixture checksums, generated lock SHA-256 `c174e05431496f0b2e0e43eef75fabea7c9e7940441bf209445abeaf84f8f156`, and driver binary SHA-256 `87a77703cda011b8f8c28e9e2d8feb10503155f0daa894247d56c808e4a1adc9` are retained at `~/.cache/snaptokens-exp95-profile-dfc251f-evidence` on that host.

The full portable evaluator remains unavailable as a candidate-promotion gate because its locked competitor still conflicts with current Snaptokens `0.2.5` while this parent declares `0.2.7`; do not repair that evaluator inside a tokenizer candidate. The mandatory local profiling preflight on 2026-09-13 found 17 GB free disk and no stale automation, but no LLM API key, so no local profile ran. A remote four-vCPU GCP diagnostic checkout found the original deterministic Gemma mismatch, then supplied the correctness and scoped timing evidence above. Do not promote `dfc251f` to the general champion or claim portability without the frozen evaluator and Apple/AMD confirmation.

### Scoped GPT-2 JSON-load campaign

Current scoped source: `c1acf0d41d8937f7768e71a8cb152b881547235c`. Experiment 32 replaces the temporary heap vector used to reduce vocabulary spellings of up to sixteen Unicode scalars with a fixed stack buffer, restarting the retained heap path before any longer spelling is truncated; Experiment 34 fills the fixed initial-byte-pair table directly from parsed merge rules instead of probing every byte pair through the ranked map; Experiment 35 reuses that exact fixed table for the matching byte-initial pairs during decomposition and retains the CSR lookup for every other pair; Experiment 39 replaces mixed ranked-map slots with equal-total-byte key and packed-payload arrays while retaining the same capacity, hash, linear probe order, sentinels, and hit results; Experiment 40 derives the dense ranked-table width from the valid initial byte-token IDs and keeps the identical CSR fallback outside that smaller exact square. On the immutable GPT-2 JSON GCP Intel screen, the composed scoped improvement over the Experiment 28 parent is `1.559553x`; the Experiment 40 parent/candidate result is `1.029941x` with a one-sided paired t 95% lower bound of `1.022323x`. V4/V5 direct loads improved `1.070194x` and `1.066775x`, cache-backed load improved `1.067417x`, first encode improved `1.027865x`, focused plus full post-timing exactness passed, and binary growth was `0.023811%`. A refreshed test-only profile ranks residual decomposition at `8,160,405 ns` median, parsed merges at `7,677,018 ns`, merge metadata at `4,801,313 ns`, and ranked-map construction at `4,310,264 ns`; its temporary source and test are fully reverted. A later matched GCP comparator finds current GPT-2 JSON loading `1.430171x` faster than Hugging Face Tokenizers `0.22.2` with identical post-load IDs. This is scoped JSON-load evidence only, not a replacement for the frozen general matrix. Full raw evidence and rejected neighbors are in `log.md` Experiments 23--42.

### Scoped local Split allocation experiment

Latest decision: after receiving the complete local result, the user explicitly requested "keep it". Restore the exact `31aabaf` source/test patch as a user-selected scoped retention. This supersedes the rejection decision below but not its measurements or limitations: there is no established speedup, the observed RSS/construction losses remain recorded, and the general champion is unchanged. Restoration uses the existing isolated branch to preserve concurrent uncommitted scanner/Split edits in the main checkout.

The follow-up "keep all changes to the code" authorizes integrating the concurrent pattern-table/regex fallback and the already-landed smaller generic cache as well. Preserve their code while moving the updated Split tests into a separate file and fixing empty pieces from zero-width regex matches. The combined tree passes 36 focused pre-tokenizer tests but has no combined performance measurement. Regex execution errors are still ignored as in the incoming fallback; broad dialect parity and runtime-error propagation remain open limitations. This is user-selected retention, not a general correctness/performance certification.

The user approved the prepared frontend comparison locally on 2026-09-09. Freeze clean parent `b4f2d866051a803bfe90d134ca3664d2ff1937a5` and reuse branch `rust/asm-20260909`. Test only the no-match Split representation described at the top of `log.md`, using the unchanged six-pair frontend runner and its complete output-conversion/destruction boundary. This is scoped allocation work, not a repair or replacement of the blocked general evaluator. No remote worker is needed for this experiment.

The local comparison completed all six pairs and exactness checks. Candidate `31aabaf` is not retained: throughput `1.073261x` has a wide `[0.939172, 1.180894]` interval, with GPT-2 `0.975308x`, higher process RSS and slower GPT-2/Qwen construction. The minimum scoped point floors passed but do not justify accepting those observed costs without a confident benefit. Commit `fbc9080` reverted the candidate source/test patch; all raw results are preserved. Do not claim a local speedup or repeat this noisy pool. Details, stale-build detection before timing, and the durable local evidence path are at the top of `log.md`. No general champion changes.

### Current Rust assembly/allocation investigation

Worker closure verified after the local experiment: GCP reports `snaptokens-rust-asm-20260909` as `TERMINATED`, last stop `2026-09-08T21:50:48.291-07:00` (2026-09-09 04:50:48 UTC). This check issued no stop/delete request and does not establish who stopped it. No further shutdown is needed; the VM/disk and archived evidence are preserved. This supersedes the earlier pending-ownership shutdown note below.

The user resumed normal Rust profiling and representation work on 2026-09-09. Freeze current clean main `66d5b9fc987b853d1ededea3482df2d00cb76dd8` as the diagnostic baseline; it includes the separately retained scoped Python/dtype work recorded at the top of `log.md`. This does not retroactively promote that tree as the general campaign champion below. Work on isolated branch `rust/asm-20260909`; keep new tests in separate test files and merge only accepted changes.

GCP authentication and PMU counters now work. The task owns `snaptokens-rust-asm-20260909`, a four-vCPU `c4-standard-4` with STANDARD PMU, Intel Emerald Rapids, in `ilkb-project-test-project/us-central1-a`. Use the unchanged portable evaluator below for three sequential whole-run counter captures and one sampled callgraph; preserve source/build/input identity. Profiles are diagnostic: the historical runner regroups parity batches and is not sufficient proof of original-batch correctness. Do not change its evaluator or claim a general promotion. Use existing focused exactness tests before experiments; archive evidence and shut down this task's VM afterward.

Prior PMU evidence at `/Users/namanchetwani/.cache/snaptokens-autoresearch/pmu-20260906` profiles historical `b3e8a34`, not current main. Its explicit Snaptokens timer samples point to merge adjacency, front/backing cache lookup and short merges, with substantial unresolved sampling. Obtain current-source attribution before selecting a new production mechanism. No runtime candidate is selected yet; the old campaign limits and missing holdout remain in force.

Experiment 85's **memory-qualified** candidate, `dcbecf21b558d987f8e6932ae984c8bc2a76b978`, is user-authorized to land on `main` as a memory reduction: it shrinks the generic direct BPE cache from 64 MiB to 4 MiB and lowers unchanged four-worker whole-process peak RSS by 236.9 MiB (17.9%) on Gemma and 58.9 MiB (4.7%) on GPT-2, with complete-ID pre/post diagnostic checks passing. Its existing diagnostic timing screen is within the declared adverse guard but excludes output destruction, so this landing does not select a throughput champion or validate portability. Full frozen-evaluator and release gates remain required before any general performance claim. Evidence: `/Users/namanchetwani/.cache/snaptokens-autoresearch/exp85-cache-rss-20260909/snaptokens-exp85-cache-rss-evidence-20260909.tar.gz`, SHA-256 `6efa0a7d76eba001c08eca0ad96878c2ea57c19e1a656e2d3f515fe7cb84bd4a`.

The current-source portable build is blocked before execution: stable Cargo rejects the pinned competitor's nightly-only manifest, and the previously used nightly then rejects `--locked` because the evaluator lock still records Snaptokens 0.2.5 while current source is 0.2.6. No lockfile is changed. In accordance with the user's request not to expand evaluator infrastructure, continue read-only diagnostics through the existing `simple_bench`/`profile_sample` Rust benches with root `Cargo.lock`, Rust 1.97.1, normal release code generation plus debug symbols. This supersedes the unstarted portable capture plan, not the general campaign's acceptance protocol. The simple bench compares complete IDs but excludes output destruction from its internal timer; its timings cannot establish an accepted speedup.

Diagnostic captures are complete: twelve CPU cases (six families, singles/batch32) with 92,344 samples and zero lost samples, and six valid heaptrack cases. Three batch allocation interpreters were OOM-killed despite wrapper success; MiniMax batch was interrupted on discovery and Qwen allocations were not run. Preserve every invalid trace and consult `capture-validity.json` before using summaries. CPU profiles finished before the allocation failures. Source attribution includes warmup, with substantial whole-process unresolved samples. The strongest fresh allocation targets are Gemma's no-match Split vectors and temporary normalization Strings, followed by GPT-OSS ignore-merges encoding Strings. Assembly also motivates testing separate four-byte merge lookup keys; short merge scratch is already stack-resident. Details and limits are at the top of `log.md`. No runtime candidate is implemented or promoted. Scanner refactor `1801ec4`, published concurrently after recording, is outside the frozen diagnostic source. Another user/source build appeared on this task's VM after recording, so do not shut it down until ownership is resolved.

**Evaluator, development manifest, and basic A/A on Apple, AMD, and Intel are frozen. Experiments 72--82 were rejected; the user explicitly resumed the campaign at the fresh attributable-profile rung. No candidate is authorized until that profile exposes a new broad mechanism.**

- Branch: `main`
- Validated source baseline: `24f959b70a3601ce3c3192badfbd88d9567e095c`
- Pre-campaign implementation head: `53e058239049b3c7716a2ea8d68c6a6e258f673f`
- Remote baseline: `origin/main` at `fd8743e`; local history also contains the Kimi commit and baseline commits
- Frozen champion: `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`
- Evaluator version: `portable-paired-v1` at `d9086dffdb6b197e47156c32de5beb16ac89c0d9c90c92297d94d7a9b661763b`
- Development manifest: `dev-v1` at `4927e7d3abbc1e71f7583b6ed05bf662c96a35c0860a6e32e6370e703858caf9`
- A/A calibration: Apple M2, AMD x86-64, and Intel x86-64 basic screens frozen
- Sealed holdout: missing
- Holdout queries used: `0 / 3`

The inherited implementation and benchmark refactors are preserved in commits `791e374` and `24f959b`. Validation passed 176 Rust tests with 9 extended tests ignored, strict workspace Clippy, package construction, the native Python package tests and changed compatibility edges, and portable, long-single, and specialist benchmark builds. The Linux-only full harness was not built locally because its optional IREE dependency began cloning LLVM and reduced free disk toward the campaign safety margin.

Candidate measurement is authorized through the frozen evaluator. Evaluator commit `b3e8a34` validates the flat-ragged API it times before and after timing, and its portable driver compares immutable binaries through fresh-process preflights plus counterbalanced AB/BA cycles. The driver binds binary, source, evaluator, lockfile, toolchain, flags, build, tokenizer, corpus, host, thread, timing, and inner-schedule provenance. Independent static review found the final evaluator patch clean; the portable crate type-checks, and warning-denied Clippy passes with only the pre-existing `manual_is_multiple_of` lint in `benchmarks/shared.rs` allowed. The Linux-only full harness remains unbuilt locally because its IREE dependency requires the large LLVM source tree.

## Campaign objective

Optimize exact, novel-input, steady-state encoding throughput against the immediately preceding Snaptokens champion.

Initial target after A/A calibration:

- Development point estimate: at least `1.09x` candidate/parent.
- Paired 95% confidence interval: strictly above `1.00x`.
- Correctness: exact token IDs and row boundaries in every evaluated cell.
- Regression rule: no tokenizer-structure or CPU-class stratum outside its calibrated A/A regression band.
- Peak RSS: no more than `5%` above the parent.
- Tokenizer construction: no more than `5%` slower than the parent.
- Release binary size: no more than `2%` above the parent.

The target was raised before the first candidate because independently built AMD A/A reached a `1.080819x` upper confidence bound. Do not adjust it after candidate results exist.

## Apple M2 A/A calibration

The frozen `dev-v1` protocol completed two AB-plus-BA cycles for both an identical binary and two independently built same-source binaries. Every fresh-process preflight and post-timing parity check passed, and both failure inventories are empty. Raw rounds, matched pairs, manifests, and summaries are retained under [`results/aa/apple-m2`](results/aa/apple-m2).

| Mode | Point estimate | Paired 95% CI | Whole-cycle scores |
| --- | ---: | ---: | ---: |
| Identical binary | `0.993660x` | `0.984004–1.007034x` | `1.007034x`, `0.984004x` |
| Independent builds | `0.995299x` | `0.995299–0.999761x` | `0.999761x`, `0.998451x` |

Apple alone would permit the original `1.02x` target because it exceeds the observed `1.007034x` false-winner ceiling; the wider cross-CPU target is recorded below. Candidate screening must also keep each tokenizer-family aggregate inside the symmetric log band derived from both Apple A/A modes:

| Tokenizer family | Apple M2 A/A band |
| --- | ---: |
| DeepSeek V3.2 | `0.984405–1.015842x` |
| Gemma 3 | `0.973861–1.026840x` |
| GPT-OSS | `0.978212–1.022273x` |
| GPT-2 | `0.990431–1.009662x` |
| MiniMax M2.1 | `0.992426–1.007632x` |
| Qwen 3 | `0.982963–1.017332x` |

Do not use single-cell medians as regression gates: identical bits produced cell medians from `0.894873x` to `1.293483x`. Shape and contract bands are recorded in `log.md`. The Apple, AMD, and Intel screens together are the frozen basic cross-CPU calibration; they do not replace fresh portability confirmation for a retained candidate.

## x86-64 A/A calibration

The frozen protocol completed identical-binary and independently built same-source modes on an accepted GCP AMD sandbox (`AuthenticAMD family 175 model 17`) and AWS Intel sandbox (`GenuineIntel family 6 model 106`). Modal did not expose model names, so the recorded vendor/family/model tuples are authoritative. Every preflight and post-timing exactness check passed, every failure inventory is empty, and each mode retains 1,440 paired ratios under [`results/aa/amd-x86-64`](results/aa/amd-x86-64) or [`results/aa/intel-x86-64`](results/aa/intel-x86-64).

| CPU class and mode | Point estimate | Paired 95% CI | Whole-cycle scores |
| --- | ---: | ---: | ---: |
| AMD identical binary | `1.009481x` | `1.003042–1.022694x` | `1.003042x`, `1.022694x` |
| AMD independent builds | `1.027463x` | `0.989541–1.080819x` | `0.989541x`, `1.080819x` |
| Intel identical binary | `0.994285x` | `0.990852–0.995594x` | `0.990852x`, `0.995594x` |
| Intel independent builds | `1.000060x` | `0.998447–1.003453x` | `0.998447x`, `1.003453x` |

The AMD independent-build distribution is materially wider than Apple or Intel and raises the aggregate false-winner ceiling to `1.080819x`. The first candidate therefore needs at least a `1.09x` development point estimate, a paired confidence interval above `1.00x`, and no grouped CPU-class regression outside these symmetric log bands:

| Tokenizer family | AMD band | Intel band |
| --- | ---: | ---: |
| DeepSeek V3.2 | `0.963447–1.037939x` | `0.997225–1.002783x` |
| Gemma 3 | `0.974495–1.026172x` | `0.987390–1.012771x` |
| GPT-OSS | `0.970110–1.030811x` | `0.991490–1.008584x` |
| GPT-2 | `0.958358–1.043452x` | `0.992792–1.007261x` |
| MiniMax M2.1 | `0.984243–1.016009x` | `0.995122–1.004902x` |
| Qwen 3 | `0.985236–1.014985x` | `0.990938–1.009145x` |

After reaching the target, promote the candidate, record the new champion, and set another nearby `1–2%` goal instead of declaring the tokenizer finished.

## Primary score

For each stratum, take the median paired throughput ratio across predeclared counterbalanced rounds. Weight strata equally and compute:

```text
exp(mean_strata(median_rounds(log(candidate throughput / parent throughput))))
```

The development strata must cover these independent sources of variation:

- Tokenizer structure: ByteLevel, metaspace/SentencePiece, byte fallback, added-token or normalizer-heavy, and generic fallback or `ignore_merges`.
- Corpus: prose, source code, JSON or tool calls, Unicode-heavy text, and punctuation or newline-heavy text.
- Shape: tiny single input, chat batch, medium document, long document, and skewed ragged batch.
- API contract: nested and flat ragged.
- CPU class: Apple ARM, AMD x86-64, and Intel x86-64.

Do not weight raw cells equally when one structural or host family has more cells. Do not include competitor throughput in the promotion score.

## Benchmark contract

The current benchmark foundation already provides:

- Exact Hugging Face parity around timed work.
- Separate nested and flat-ragged comparisons.
- Balanced implementation ordering.
- Rotating corpus windows and disjoint warmup.
- Timed output allocation and destruction.
- Source, binary, lockfile, tokenizer, corpus, host, thread, and build fingerprints in the Linux matrix runner.
- A five-shape portable screen through [`../benchmarks/portable/src/main.rs`](../benchmarks/portable/src/main.rs).
- The larger reproducible matrix through [`../benchmarks/harness/run-matrix.sh`](../benchmarks/harness/run-matrix.sh).

The visible matrix becomes the development set. It is not a holdout because prior optimization decisions and published thresholds were informed by it.

The missing holdout must live outside the repository and outside the optimization agent's readable filesystem. It should use unseen tokenizer artifacts, corpora, seeds, Unicode mixtures, length distributions, batch distributions, and raggedness patterns while preserving the same exactness and timing contract.

## Architecture map

- [`../src/lib.rs`](../src/lib.rs): public tokenizer construction, encode entry points, batch scheduling, and output contracts.
- [`../src/models.rs`](../src/models.rs): parsed tokenizer pipeline and model-level construction state.
- [`../src/models/bpe.rs`](../src/models/bpe.rs): BPE representations, merge scheduling, scratch, and caches.
- [`../src/pre_tokenizers.rs`](../src/pre_tokenizers.rs): pre-tokenizer representation and dispatch.
- [`../src/pre_tokenizers/scanner.rs`](../src/pre_tokenizers/scanner.rs): fixed scanners, fused streams, classification, and boundary production.
- [`../src/pre_tokenizers/split.rs`](../src/pre_tokenizers/split.rs): exact Split behavior and fallback matching.
- [`../python/src/lib.rs`](../python/src/lib.rs): Python binding and public Python output behavior.
- [`../benchmarks/portable/src/main.rs`](../benchmarks/portable/src/main.rs): local five-shape correctness and timing screen.
- [`../benchmarks/harness/src/main.rs`](../benchmarks/harness/src/main.rs): full matrix benchmark engine.
- [`../benchmarks/harness/run-matrix.sh`](../benchmarks/harness/run-matrix.sh): clean-source, fingerprint, affinity, and result-publication enforcement.
- [`../log.md`](../log.md): complete experiment memory and the source of prior negative evidence.

The central steady-state pipeline is:

```text
input rows
  -> added-token and normalization handling
  -> structural pre-tokenizer or exact fallback
  -> ByteLevel or metaspace transformation when required
  -> initial token IDs
  -> cache probes and exact BPE merges
  -> post-processing
  -> nested or flat-ragged token IDs
```

An optimization may shorten a stage or fuse boundaries, but it may not silently remove a stage that is required by the tokenizer configuration.

## Recent durable evidence

Do not reopen these mechanisms without new profile evidence or a materially different representation:

- Experiment 64 retained removal of unnecessary fused pending-scratch initialization.
- Experiment 65 rejected forced inlining of the Llama ASCII fallback.
- Experiment 66 rejected two-phase short-mask boundary extraction.
- Experiment 67 rejected reusing NEON high-byte masks.
- Experiment 68 rejected sparse-Unicode NFC islands and a follow-up quick-check dependency.
- Experiment 69 retained removal of duplicate DeepSeek ASCII classification.
- Experiment 70 retained the direct Unicode class table in generic fixed scanners.
- Experiment 71 rejected plumbing a hoisted DeepSeek Unicode table view through private helpers.

The ledger contains 70 numbered experiment headings through Experiment 71 because some headings describe measurement attempts or planned/reverted work. Search by subsystem and mechanism rather than assuming the next number identifies unexplored territory.

## Open search lanes

The current champion profile ranks the broad lanes as follows:

1. BPE merge and fused cache resolution dominate the fused tokenizers, but prior short-linear merge and cache-layout representations remain rejected and the current profile does not establish a new exact shortcut.
2. Gemma's generic BPE model closure dominates its encode path, but the prior held-generic-cache representation regressed and no new invariant is selected.
3. Added-token false-positive fallback is closed after exact full-string and candidate-position preflights both failed their cross-model target gates.
4. Exact victim backing is closed after its Intel target screen split sharply by cache population: Qwen and MiniMax improved, while GPT-2 and DeepSeek regressed far outside their frozen bands.
5. Fixed scanner work remains measurable but smaller than cache resolution in every fused profile.

Experiment 75 confirmed that speculative front-cache miss stores are dead but rejected moving them behind the hit branch: the complete Intel target aggregate was `0.968925x`, and all four fused tokenizer families regressed below their frozen bands. Re-profile before changing `FusedStream::flush` or its cache policy again.

Experiment 76 rejected empty/singleton CSR merge-row specialization before candidate code. Those rows covered only `3.12%` of Gemma and `21.41%` of Qwen runtime merge lookups, with singleton rows at `0.67%` and `7.65%` respectively.

Experiment 77 could not establish a `10%` whole-encode ceiling for immutable exact pair retrieval without first implementing the medium representation. Existing exact-hash evidence peaked near `1.02x` with repeated losses, so the lane is held until a profiler can attribute inlined adjacency work directly.

Experiment 78 rejected within-batch exact cold-piece dedup at its first predeclared mechanism cell. Gemma batch 32 had 151 post-cache resolutions and zero duplicates, so it missed the `20%` gate without candidate code.

Experiment 79 rejected carrying ranked merged IDs in heap priority. Gemma and GPT-OSS were non-ranked, while the four ranked fused families reached heap BPE only hundreds to low thousands of times over two MiB because common short pieces already use the ranked stack loop.

Experiment 80 rejected a flat indexed batch task graph before candidate code. Across all six tokenizers and both batch-512 contracts, the impossible perfect-packing removable fraction was only `4.21--28.09%`, below the predeclared `35%` floor in every cell. Complete pre/post Hugging Face IDs and ragged row boundaries passed. Scheduling is closed until a new profile demonstrates removable work outside the current tasks rather than another chunk-count or ownership variation.

Experiment 81 rejected the pinned `mtc-inc-bpe` canonical prefix-state representation before runtime code. Gemma's exact encoded initializer alphabet and surviving Hugging Face rule order returned `ImproperDict` for token `109` (`"\n\n\n"`), so the representation fails a necessary precondition in the required generic family. Do not weaken the atom set, reorder or properize rules, or narrow the general family set to reopen it.

Experiment 82 rejected exact input-specific independent BPE spans at its first required stage-timing family gate. Gemma had broad source-callable oracle cuts and exact outputs in all ten cells, but even impossible output-informed, zero-cost span discovery improved the isolated production BPE stage only `1.246950x` versus the frozen `2.00x` authorization gate. Qwen and later families did not run. Do not reopen independent-span matching without a materially different end-to-end representation that removes more than cross-boundary merge scheduling.

The 2026-08-20 four-core Intel Gigatoken audit is competitor context only, not candidate evidence. The unchanged portable runner used the frozen six-tokenizer manifest, exact Hugging Face coverage, flat-ragged output, six rounds, four MiB per engine round, and four Rayon workers. Gemma was `9.688x` Snaptokens/Gigatoken, but the five ByteLevel families combined were `0.941x`: batch 1 was `1.535x`, batch 32 `0.674x`, batch 512 `1.252x`, 4 KiB `0.783x`, and 64 KiB `0.728x`. This rules out one global ragged chunk-count change: both the batch-32 and Gigatoken paths are serial, while Snaptokens' 64 KiB fused-Split route already adds newline partitioning and gather work that Gigatoken does not use below its one-MiB chunk floor. Existing Experiments 55, 61, and 80 already reject the obvious parallelization, threshold, and flat-task rewrites. Raw archive: `/tmp/snaptokens-gigatoken-exploratory-4vcpu-20260820/matrix.tar.gz`, SHA-256 `dff3ad2d828cb93aac5de2ef7bd540460d8a3b8f01c541db0c59f369009e90d9`.

No legitimate cheap eighth mechanism screen remains. Cache, heap, exact retrieval, scheduling, deduplication, prefix-state, and independent-span representations are closed by measured evidence; fixed-scanner work is too small to reach the frozen general target. The remaining cross-piece lockstep redesign lacks broad measured reach and is estimated at several development days, so implementing it only to reach the formal eight-failure plateau would violate the cheapest-decisive-gate rule. Resume only after a fresh whole-pipeline profile attributes enough removable time to a materially different broad mechanism. Modal's gVisor sandbox currently rejects `perf_event_open`, while local profiling remains unavailable under the mandatory API-key preflight.

The resumed profile acquisition is currently blocked only by infrastructure authentication: the configured GCP account cannot mint an access token or execute a read-only Compute request, and no application-default or service-account credential is present. Once the user restores noninteractive GCP authentication, use one four-vCPU `c4-standard-4` VM with `--performance-monitoring-unit=standard`; retain its VM/PMU provenance, run the unchanged frozen portable runner sequentially, and collect fixed `perf stat` repetitions plus one `perf record` callgraph. Do not substitute a different host, gprof, or a new benchmark framework.

Construction and `.tkz` loading are a separate objective. Model-specific wins are a separate specialist track. Neither may be folded into the steady-state general score.

The next candidate must use a different representation supported by the current profile; do not tune the rejected cache policy by tokenizer, table size, or input threshold.

## Readiness checklist

- [x] Decide which inherited changes belong in the baseline.
- [x] Commit the intended implementation and benchmark refactors.
- [x] Validate the root, native Python package, benchmark crates, and lockfiles.
- [x] Land and freeze evaluator-only pre/post ragged parity plus immutable A/B support.
- [x] Record the full frozen champion SHA above.
- [x] Freeze a development manifest with tokenizer, corpus, seed, shape, contract, host, thread, build, and round hashes.
- [x] Run identical-binary A/A measurements on Apple M2.
- [x] Run independently built same-source A/A measurements on Apple M2.
- [x] Record grouped and aggregate A/A bands on Apple M2.
- [x] Repeat A/A calibration on AMD and Intel.
- [ ] Install an external sealed holdout that returns only correctness, aggregate score, guardrail status, and evaluator version.
- [ ] Verify an optimization candidate cannot change the evaluator or holdout.
- [x] Profile one representative workload from each structural and shape family.
- [x] Rank the top measured costs by estimated whole-matrix removable time.
- [x] Write and cross-review the first experiment card in `log.md`.

## Campaign budget

- Maximum candidates: `40`
- Maximum elapsed experiment time: `24 hours`
- Maximum sealed holdout queries: `3`
- Plateau trigger: `8` consecutive mechanism-screen failures
- Promotion target: one candidate that clears correctness, development, holdout, portability, and resource gates
- Final checkpoint: eleven consecutive rejected experiments (`72--82`), including seven consecutive mechanism-screen rejections (`76--82`); stopped under the repeated-unproductive-experiments condition because no evidence-backed cheap eighth rung remains

Parallel proposal work may use three independent explorers. Cross-review selects one hypothesis for implementation. The evaluator is deterministic code, not an LLM vote. Promising winners may receive two short exploitation branches, but their combination is evaluated as a new candidate.

Track useful experiments per hour, rejection stage, development-to-holdout false wins, best valid holdout score, human interventions, and infrastructure downtime. The campaign is successful only when its result is reproducible; candidate count alone is not progress.

## Next actions

1. Restore noninteractive GCP authentication, then re-profile the unchanged champion on the predeclared PMU-enabled `c4-standard-4` VM for a cost outside the rejected cache, heap, scheduling, and canonical-prefix representations.
2. Require a quantitative mechanism screen that binds any new representation to generic Gemma and representative fused cache misses before candidate code.
3. Create the external sealed holdout before querying any retained development winner.

Update this file after each champion promotion, evaluator change, A/A recalibration, holdout query, campaign stop, or material change in ranked headroom. Keep it compact; move detailed measurements and failures into `log.md`.
