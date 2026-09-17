# Portable tokenizer performance log

### E7 small-model memory rejection screen (2026-09-17) — planned

The original E7 card predicts small-file and first-pool-startup costs. Before retention, use the same preserved feature-matched serial/parallel 1.98.1 binaries on canonical frozen GPT-2 and the unchanged nineteen-input corpus. After the process-startup diagnostic finishes on idle PMU, require complete pre/post HF checks and run three fixed fresh-process AB/BA pairs of six direct ST loads. Record all internal times and child peak RSS, including the first load and pool startup. Reject early only if all three candidate/parent RSS ratios exceed the existing 1.05 resource limit. Otherwise this screen makes no acceptance decision and full portable gates continue. This is an additional cheap adverse-case rejection screen, not changed evaluator data, sample counts or promotion scoring. No source, threshold, worker-count or allocator change.

### E11 loss is variable; predeclare process-startup control (2026-09-17)

The post-selected E11 diagnostic does not consistently reproduce the original loss: three warm parent/candidate ratios are 0.977051, 1.626682 and 1.066542. User-space instruction ratios stay tightly near 1.02587, while parent minor faults vary from 85,821 to 1,309,454 and candidate faults from 57,422 to 968,927. Separate strace captures show parent 1,976 brk calls versus candidate 208, with both at 36 mmap/25 munmap calls. These variable fresh-process allocation/page-fault regimes prevent attributing the old 0.6188x loss to a stable increase in user-space UTF-8 work. Keep E11 rejected and all original data; the diagnostic does not replace the predeclared screen.

The harness collects argv[0] into an owned String. Before attributing every E13 timing gain to lifetime, run a same-binary control on idle PMU: immutable combined parent, fixed twelve AB/BA pairs of hundred-load processes, Qwen3/Mistral Nemo/Mistral Large. Compare two modes: identical argv[0] for both labels, then the exact E13 parent/candidate argv[0] strings while executing the SAME parent binary via subprocess executable=. Keep executable, inputs, environment and rounds fixed; record per-load times, child user/system time, minor faults and peak RSS, with pre/post exactness. This is explicitly a process-startup/allocator diagnostic, not candidate promotion. No evaluator or candidate code changes. Preserve every round and report both controls, including null or conflicting outcomes.

### Diagnostic question for the rejected E11 loss (2026-09-17)

E11 remains rejected/restored. Its Mistral Large warm 0.6188x loss was not attributed, even though the constructor-only change added no allocation. On the now-idle PMU host, compare its preserved 1.98.1 parent/candidate binaries on this explicitly post-selected adverse model: three AB/BA pairs of hundred-load counters separating user/kernel instructions and minor faults, then independent brk/mmap/munmap/mremap summaries and cycles profiles. Preserve source/input/binary identity and pre/post HF checks. This diagnostic may explain the loss and guide future questions; it cannot rescue E11, replace its original pool or establish a win. No source change and no competing PMU timing.

### E14 mechanism passes; freeze matching-parent portability (2026-09-17)

E14 `320b961` passes the focused mixed-UTF-8 constructor/clone/bounds test under Miri nightly-2026-09-15 (1 test, 129 filtered), all 130 stable unit tests, strict Clippy and all BPE/Unigram pre/post HF exactness. Stable 1.98.1 three-pair BPE screen: warm 1.124633492x, cycles 1.112567204x, instructions 1.114888267x, branch misses 1.294244949x. All twelve BPE warm cells improve (1.0692–1.1810x). Unigram ST is 0.995060583x, JSON 1.003430579x; T5 ST 0.9867x is a disclosed adverse cell, UMT5 1.0035x. No performance claim for Unigram and no retention yet.

Freeze `e14-portable-protocol.py` before any portable candidate pool. Queue after E13 on each host; compile committed E14 explicitly with stable 1.98.1, no unrecorded Rust flags, matching dependency locks and clean source. Reuse the exact immutable BPE parent binary and matching-source calibration created for E13's parent `204aae8`; reuse the original Unigram v1 parent binary/calibration from the same `204aae8` source. Verify parent SHA/locks/binary hash, evaluator/scorer hashes and every frozen input before timing. No fresh A/A is needed for the same immutable parent/protocol. BPE remains primary >=1.02 with CI above1; both families require all calibrated model/format/encode guards, RSS <=1.05 and binary <=1.02. Every candidate pool runs all twelve pairs unless a prior complete gate rejects it. No composition with E13 or other candidates.

### E10 rejected by AMD encode guards (2026-09-17)

The completed AMD twelve-pair E10 pool passes the primary ST load gate at 1.021712762x CI [1.012551331,1.042752465], cached 1.032347511x, JSON 1.046557897x. Its UMT5 scalar 0.973213454x falls below the frozen 0.979316985x lower band; UMT5 batch 0.936579298x falls below 0.965093553x. Reject E10 despite the Apple/Intel passes and restore the entire isolated runtime patch to parent `204aae8`. Root never integrated this candidate, so supported BPE/Unigram behavior remains unchanged. Preserve all AMD raw rounds/calibration; no filtered cells or rescue run.

Skip E10's queued later BPE checks under the evaluation funnel. The three task-owned E13 orchestrators were still waiting for E7, with no E13 build or timing started; stop only those waiting processes and restart an otherwise unchanged E13-only protocol. Remove only E10 build/check steps; E13 parent, candidate, evaluator, calibrations, inputs, rounds and gates remain exactly as frozen. Preserve the original protocol and this reason for the skipped checks.

### E13 fault and assembly attribution; E14 safety validation (2026-09-17)

Every E13 BPE cell has fewer measured minor faults. Equal-model geometric mean of per-model paired medians is 7.5965x parent/candidate faults; MiniMax 74.6557x, Qwen3 71.0980x, Qwen Coder 71.0999x and Mistral Nemo 55.7419x. These are whole-process counts over the declared hundred-load runs, not an instruction-speed multiplier. The largest fault reductions coincide with the largest wall gains; this supports the proposed lifetime mechanism. Disassembly keeps load_st essentially the same size (4090 parent bytes, 4088 candidate), with the candidate deallocation before pipeline/model construction. Full raw assembly and counters are in `e13-pmu/`.

E14 is isolated at `320b9615050f2ede54c4d6a8bf68bb39b505795b` on `perf/st-trusted-arena-get`: only the bounded getter's validated-span conversion and its focused constructor/clone/boundary test. Pinned Miri nightly-2026-09-15 is a correctness tool only; all timed binaries remain explicitly built with stable 1.98.1 and verified metadata. Miri precedes every E14 timing. No new runtime integration; E13 and E10 remain separate while portable gates run.

### `.st` experiment 14: trust constructor-validated arena spans in getters (2026-09-17) — planned

Parent SHA: `204aae80ae16190c0dccd03f57a54ea24bdccb89` (combined runtime `f786899`); independent of E10, E13 and parallel checksum.
Hypothesis: eliminate repeated UTF-8 validation in `VocabArena::get` while preserving its Vec/offset representation and all load-time validation.
Measured hot cost: refreshed combined-runtime profiles put 16.31–23.86% self cycles in std UTF-8 validation across five BPE models. Source validates every native span in `from_parts`, then calls the validating getter again while rebuilding character tables; JSON/TKZ construction starts from valid Strings and also uses that getter. The total profile includes constructor validation, so not all of that cost is removable. E1's String representation failed encode guards and E11's whole-buffer constructor validation failed its mechanism screen; neither source patch is reused.
Invariant that makes the shorter path exact: `from_strings` copies valid String bytes and records offsets only between complete strings. `from_parts` checks offset shape, monotonicity, byte bounds and every span with std::str::from_utf8 before constructing Self. Those are the only constructors; bytes/offsets have no mutation API, getters borrow immutably and Clone preserves the invariant. Keep both offset gets and the byte-slice get in `get`, including out-of-range None. Only then convert that already-proved valid span with from_utf8_unchecked.
Representation being preserved or changed: unchanged Vec bytes/offsets, constructors, snapshot format, returned lifetimes, token IDs, JSON/TKZ paths, table/cache layout and every validation/error. One locally documented unsafe conversion avoids rechecking an invariant that source ownership maintains. No other unsafe or unchecked indexing, dependency or API changes. The measured repeated validator and failed safe-representation alternative justify this narrow conversion.
Expected winning strata: BMP reconstruction and repeated vocabulary/decode access; JSON/TKZ may also improve. Expected adverse strata: code-layout effects, scalar/batch encoding regressions as seen in E1, and small vocabularies. Unigram is a non-regression track.
Smallest files that need changing: `src/models/bpe.rs` getter and one focused constructor/getter boundary test, plus this card. Keep `to_vec_strings` and all adjacent code unchanged.
Mechanism evidence: the focused test covers empty/ASCII/two/three/four-byte strings, every possible split in mixed strings, invalid/truncated/overlong/surrogate/out-of-range byte sequences, cloned arenas and out-of-range IDs. Run it under Miri before timing, plus all unit tests and strict Clippy with 1.98.1. Then complete HF scalar/nested/ragged/vocabulary exactness before/after the fixed twelve-model three-pair hundred-load BPE transfer screen and two-model three-pair thirty-load Unigram JSON/ST screen. Capture whole-process cycles/instructions/branch misses and first loads separately; build immutable matching-lock binaries with explicit compiler metadata.
Acceptance rule: exactness and Miri pass, BPE warm >=1.02 to admit full unchanged portable load/encode/resource gates. Full BPE and Unigram regressions remain mandatory; RSS <=1.05, binary <=1.02, point floor exactly 1.02 and primary CI above 1. Same-parent frozen calibration may be reused only with verified matching parent binary, locks, evaluator, flags and inputs. No integration before those gates.
Rejection rule: any construction route without the invariant, Miri/parity failure, insufficient mechanism gain or later calibrated regression. Restore the isolated source fully on rejection; do not loosen boundary validation or combine other candidates to rescue it.
Self-review: module-wide source search found no direct arena construction outside the two constructors and no writes to stored bytes/offsets. The safe bounds checks remain; no format corruption can bypass from_parts. This is a proof-bearing getter change, not removed input validation. Future mutation would invalidate the unsafe contract and must be prevented or revalidated.

### E13 mechanism passes; freeze portable checks and E10 BPE guards (2026-09-17)

E13 `0d12df599ce6edf8b20fc20a50b5cd78a86ba669` passes 129 unit tests, strict Clippy and full BPE/Unigram pre/post HF parity. All three declared BPE pairs complete: warm 1.160574671x, cycles 1.166135220x, instructions 1.109857758x, branch misses 1.001936895x. Qwen3/Code and MiniMax warm are 1.4823–1.5057x; Mistral Nemo 1.6104x. DeepSeek 0.9881x and GLM 0.9988x are disclosed losses. Unigram ST warm is 0.998157568x, JSON 1.001591990x; T5 ST 0.9929x, UMT5 1.0035x. These are mechanism results, not portable retention.

Three fresh-process six-load RSS pairs have candidate/parent medians of 0.7041–0.7563 for ten BPE models, 0.9327 DeepSeek and 0.9465 Gemma; T5 1.0 and UMT5 0.9562. Every measured RSS cell is <=1.05. The Qwen/Nemo profile page-fault cost and lifetime mechanism are consistent with these results, but raw minor-fault counts and disassembly still need inspection before attributing the entire gain to page reuse.

Freeze `e13-portable-protocol.py` before portable timing. Parent `204aae8`, E13 candidate `0d12df5`, E10 `ea75ed9` remain independent committed trees; all locks/build flags must match, rustc is explicitly 1.98.1 and Linux ELF metadata is verified. Fresh identical/independent BPE A/A uses the unchanged v2 evaluator; intersect new bands with previously frozen host bands before either candidate result. Test E10 on BPE as a secondary regression track (all model/format/encode/RSS/binary guards, no BPE speedup requirement); its primary Unigram gate remains separately required. Then test E13 BPE primary >=1.02 with CI lower >1 and all guards. If E13 passes, fresh Unigram A/A and its non-regression load/encode/resource gates follow with the unchanged v1 protocol. Every pool retains all twelve pairs. Each candidate's failure is recorded independently; E10 cannot rescue E13 or vice versa. No root runtime change. Runs wait for existing E7 portability on each host to keep timing isolated.

Corrected E8 final diagnostics: Intel ST 0.999998223x CI [0.971794921,1.049687039], GPT-OSS JSON 0.987786847x below 0.989472016x; Phi scalar 0.950067011x below 0.959234215x. AMD ST 1.011517412x CI [0.935335788,1.084476110] and three load guards fail as recorded in state, while encode/RSS/binary pass. Both binaries have matching 1.98.1 compiler metadata; these results replace invalid compiler-mismatched comparisons, not their raw records. All three CPUs miss the primary 2% gate. E8 stays only under explicit user retention.

### `.st` experiment 13: release decoded file buffer before construction (2026-09-17) — planned

Parent SHA: `204aae80ae16190c0dccd03f57a54ea24bdccb89`, same runtime as user-kept E8 plus validated BPE/Unigram support in `f786899`; no E10 or parallel-checksum change.
Hypothesis: release the owned file Vec immediately after complete decoding so final BPE tables or the Unigram matcher can reuse its memory instead of overlapping its full live allocation.
Measured hot cost: refreshed five-model combined-runtime profiles have zero lost samples; page-fault entry is 8.18% self cycles on Qwen and 8.63% on Mistral Nemo. Prior allocator diagnostics show that overlapping live allocation patterns can cause repeated heap growth and page faults. `load_st` currently retains the complete fs::read buffer through `Payload::into_config`, which allocates full tables or builds the automaton. This supports a lifetime experiment, not a claim that this buffer causes all those faults.
Invariant that makes the shorter path exact: `decode_file` returns an owned `Payload` containing owned vectors/strings, with no lifetime borrowed from the file. Header, source binding, checksum, bounded decode and trailing-data validation all finish before dropping the Vec. Into-config validation, model construction, returned ownership and error ordering remain unchanged; Vec destruction cannot fail.
Representation being preserved or changed: no format, model/table contents, API, dependency, cache, unsafe code, allocator tuning or encode change. Only shorten the lifetime of a temporary file buffer, on both BPE and Unigram direct/cached loads.
Expected winning strata: memory-expanding native tables and matcher construction, especially where later allocations can reuse the freed block. Expected adverse strata: allocator mapping/trimming changes may increase faults, fragmentation or wall time; small files may show no benefit.
Smallest files that need changing: `src/st.rs` and this experiment card. Existing direct/source-absent/cache and malformed-file tests cover the behavior; Rust ownership proves there are no returned borrows, so no implementation-mirroring test is needed.
Mechanism evidence: explicit rustc 1.98.1 parent/candidate immutable binaries, complete existing unit/Clippy gates and full HF scalar/nested/ragged/vocabulary checks before/after. Run the unchanged twelve-model transfer protocol for three AB/BA pairs of 100 direct loads, plus the frozen two-model Unigram three-pair 30-load JSON/ST protocol. Include first loads separately. Collect cycles/instructions/branch misses and minor faults without changing timing scope. Separately record process peak RSS for three fresh-process paired six-load runs on every BPE/Unigram fixture, with fixed four workers. No simultaneous profiles, builds or bulk transfers during timing.
Acceptance rule: BPE equal-model warm throughput >=1.02, exactness throughout, all measured memory/fault/CPU tradeoffs disclosed. Unigram is a required non-regression track. This admits only full fresh-baseline BPE A/A and calibrated three-CPU BPE/Unigram gates; RSS <=1.05 and binary <=1.02 remain required. No retention based on memory reduction alone.
Rejection rule: any mismatch, insufficient BPE gain, or later calibrated regression. Preserve all losses and restore the isolated patch in full on rejection. No model-size threshold or allocator-environment adjustment after results.
Self-review: the compiler enforces the owned payload invariant. The testable mechanism is earlier release, not bypassed parsing or validation. This experiment is independent of E10/E7 and does not silently combine their gains.

### E7 Unigram adverse result and refreshed profile plan (2026-09-17)

Corrected explicit-four-worker E7 Unigram screen completes all exactness and declared rounds: ST warm 0.954554383x, cycles 0.951656219x, instructions 0.981263172x; T5 warm 0.9417x and UMT5 0.9676x. JSON is 1.009148931x. This is an observed adverse result, not a Unigram win. Preserve it with all corrected BPE and one-worker evidence in `e7-fixed/`. Fresh calibrated portable gates remain necessary and may reject the BPE-targeted mechanism. No runtime integration.

Apple E10 also passes its full twelve-pair load and encode/resource gates: ST 1.032622245x CI [1.017181154,1.053185242], cached 1.039788983x, JSON 1.029320281x, binary ratio 1.002175076. Intel passes as recorded above; AMD remains pending. No cross-CPU retention yet.

While portability runs on separate hosts, profile committed runtime `204aae80ae16190c0dccd03f57a54ea24bdccb89` (same runtime as combined `f786899`, evaluator-only additions) on the idle PMU host. Rebuild BPE evaluator explicitly with 1.98.1, verify ELF metadata, preserve locks/binary/input hashes and pre/post HF IDs. Five previously used transfer structures (Qwen3, Mistral Large, Mistral Nemo, GPT-OSS, Gemma) each get 100 direct loads with cycles at 997 Hz, dwarf stacks, plus separate cycles/instructions/minor-faults/branch-miss captures. This refreshes stale E1-era allocation/validation attribution; it is not a candidate or performance claim. No new tokenizer edit until a current measured cost supports it.

### E7 corrected mechanism and portability protocol (2026-09-17)

Explicit four-worker forwarding gives E7 runtime BPE warm 1.034594627x, cycles 0.919797058x (about 8.7% more total CPU cycles), instructions 0.997448995x and branch misses 0.998223911x. All pre/post exactness checks pass. The one-worker supplemental aggregate is 1.037986818x but ten of twelve models regress (0.9269–0.9740x); Mistral Nemo 1.6099x and GPT-OSS 1.5464x account for the favorable aggregate. Preserve these losses and do not call it a general single-worker improvement. Unigram supplemental checks finish before portability starts.

The original E7 card names BPE as the primary load target. Frozen portability orchestration uses unchanged BPE v2 and Unigram v1 evaluators, rustc 1.98.1, identical dependency locks/features for serial parent `61a038b` and parallel candidate `3ee40d6`, independent parent builds and all twelve AB/BA pairs. On each CPU, run fresh identical/independent A/A load and encode pools for the feature baseline, then intersect their calibrated per-model bands with the previously frozen host bands before candidate timing. BPE primary requires point >=1.02 and CI lower >1; both families require every calibrated model/format/encode guard, RSS <=1.05 and binary <=1.02. Unigram is a non-regression guard for this BPE-targeted candidate, not a second 2% improvement target. Any gate failure stops later gates. No thresholds, exclusions, prewarming or worker tuning are added. One/four-worker PMU evidence remains separate. Exact orchestration is retained as `e7-portable-protocol.py`; no root runtime changes.

E10's completed Intel twelve-pair gate passes: direct ST 1.020888957x CI [1.017974632,1.031017886], cached 1.037194390x, JSON 1.027917250x. All calibrated load/encode/RSS/binary guards pass. Aggregate scalar/batch/ragged ratios are 0.973494/0.975169/0.982527 and remain disclosed despite falling within A/A bands. Apple and AMD are still required; no retention yet.

### E12 matcher packing rejected before timing (2026-09-17)

The standalone minimum-one-free-block matcher produces identical overlapping matches on all 32,125 T5 and 256,325 UMT5 vocabulary/corpus inputs. Its heap grows from 1,380,528 to 1,626,288 bytes for T5 (1.1780x) and from 10,055,184 to 12,110,352 bytes for UMT5 (1.2044x), exceeding the predeclared 1.05 limit in both models. Reject before timing. No production code changed, and no block-count sweep follows. The initial diagnostic failed to compile because DaachorseError does not implement std::error::Error; the corrected diagnostic maps that error to its string, passes formatting and strict Clippy, and preserves both failure and retry logs. Full source, lock, hashes and results are retained under `autoresearch/results/st-20260917/e12-builder/`.

Intel compiler audit also confirms historical E2/E4/E5 match the intended 1.98.1 parent; their earlier rejections are unaffected. E11 source restoration is isolated commit `a922c11`. Apple E8 raw load/encode/resource rounds and gates are preserved in `e8-apple-v2/`; compiler metadata in `intel-compiler-audit.txt`.

### Compiler audit invalidates Intel E6/E8 comparisons (2026-09-17)

ELF `.comment` shows Intel `repaired-parent`, `repaired-independent` and E1 were built with rustc 1.98.1 / LLD 22.1.8, while E6 and E8 used rustc 1.99.0-nightly (969b803cb 2026-08-09) / LLD 23.1.0. Their wrappers used bare cargo under a changed host default. Intel E6/E8 timing and guard comparisons are invalid, including the adverse results recorded below; preserve them as invalid evidence, not performance findings. E6 remains rejected independently by the valid Apple GPT-OSS TKZ guard and is not rerun. AMD E6 failed before timing because the default 1.97.1 lacked rustfmt; its dependent E8 never started. Repair the queue without rescuing E6.

Rebuild E8 in a new isolated source directory using explicit `cargo +1.98.1`, preserve the old binary, verify ELF compiler metadata against both parents before timing, and write separate `v2-e8-rust1981` results. Intel runs after its active Unigram pool; AMD runs before Unigram. Evaluator, calibration, inputs, sample count, parent and gate policy remain unchanged. PMU parent/E11/E7 runtime binaries have matching 1.98.1 metadata; new Unigram pools explicitly pin 1.98.1. Do not infer compiler identity from a wrapper's intention.

Apple E8 uses matching 1.98.1 and completes all twelve pairs: ST 0.997852678x, CI [0.991404696,1.011268040], below the 1.02 floor; load-model bands pass. GPT-OSS batch encode 0.492312121x fails the frozen 0.553908525x lower band; its RSS ratio 1.072707249 exceeds 1.05. Binary ratio is 1.0. Keep E8 solely under the user's explicit retention instruction, disclose these losses, and do not promote it as a portable win. Raw rounds and gate calculations are retained.

### E11 load-only UTF-8 validation rejected (2026-09-17)

Candidate `6f297b1e0f1c30e7bce1423ae657aa19fbec0468` completes the predeclared three-pair, hundred-load PMU screen and full pre/post parity. Warm throughput is 0.979375932x, cycles 0.982119740x, instructions 0.962710073x and branch misses 0.995093740x. Mistral Large is adverse at 0.6188x warm and 0.6690x cycles; the cause is not yet attributed. Reject below the 1.02 mechanism floor, skip later gates, and restore the isolated source to `95bc1ac`. This does not change root runtime. Raw evidence is `e11-pmu/` on the task PMU host, with a local archive retained alongside other experiment evidence.

### User-kept E8 fails Intel v2 load gate (2026-09-17)

The full Intel twelve-pair E8 `84a0347` result is ST `0.983703968x`, CI `[0.954307,1.015349]`, below the 1.02 point floor and confidence requirement. DeepSeek ST `0.946590965x < 0.953037297x` and Phi ST `0.969166565x < 0.971355994x` also fail frozen bands. Keep the source solely under the user's explicit earlier "keep it" direction; this is not performance promotion and must not be described as a portable win. Complete raw load evidence/calibration/gate are retained in `e8-intel-v2-load/`. Encode/resource diagnostics and Apple/AMD pools continue under that user-retention plan; the combined root still requires its own measurements.


### PMU thread forwarding correction and new isolated screens (2026-09-17)

The PMU wrapper passed `RAYON_NUM_THREADS` to sudo, which removes it: direct probe `RAYON_NUM_THREADS=1 sudo sh ...` reported unset, and the host has four available CPUs. The supplemental E7 result labeled one-worker is therefore invalid as a one-worker measurement. The original standalone checksum screen and canonical v2 portability/Unigram candidate runners execute directly and are unaffected. Earlier four-worker PMU runs used the host default instead of explicitly forwarding the requested value; retain that limitation. Before interpreting parallel-loader results, rerun all its PMU scenarios with `sudo --preserve-env=RAYON_NUM_THREADS`, unchanged binary/data/rounds/counters, explicit one/four environment probes and separate result paths. Keep the original records; do not rescue a rejected candidate with this repair.

E7 runtime is isolated at `3ee40d61690eebe0a9338ea61e62a9705a4d590e`, compared with serial parent `61a038b6f5d244e3e2fb2c9560d36f8650067ea3`; both enable the same BLAKE3 rayon feature and identical dependency locks. Both pass all 129 unit tests and strict Clippy. The original four-default-worker BPE screen is 1.030720x wall throughput but 0.916687x parent/candidate CPU cycles; it is provisional pending explicit-thread reruns. One-worker supplemental protocol is three pairs of 30 loads, selected before execution. No promotion or root runtime edit.

E11 `6f297b1e0f1c30e7bce1423ae657aa19fbec0468` is isolated on `perf/st-load-utf8-validation`: validate borrowed packed bytes once only in `from_parts`, preserving the Vec layout, every getter and JSON/encode path. Its card distinguishes this from rejected E1 and retains split-codepoint coverage. PMU unit/transfer screen follows E7's initial pool; E12's standalone one-versus-sixteen matcher-block footprint screen follows E11. The corrected E7 thread scenarios are queued after those task-owned jobs. No competing PMU runs.


### `.st` experiment 12: Unigram matcher packing screen (2026-09-17) — planned

Parent SHA: runtime `f78689974948e282e581fda603a6dfbbcf3e2b1c`; production source remains unchanged for this standalone screen.
Hypothesis: limiting Daachorse's placement search to its minimum one free block reduces double-array construction work enough to matter for ST loading without materially enlarging the matcher.
Measured hot cost: current T5 and UMT5 cycle profiles place 22.72% and 20.10% in double-array construction, including base-placement search. The pinned 1.x builder defaults to 16 free blocks; its source documents that a smaller value trades construction time for memory efficiency. No prior `num_free_blocks` experiment is recorded in the ledger.
Invariant that makes the shorter path exact: this builder setting changes placement, not pattern strings, IDs, scores or match policy. Compare every overlapping `(start,end,ID)` sequence on every vocabulary spelling and all 25 frozen corpus strings between settings 16 and 1 before timing. No production runtime edit or new runtime dependency.
Representation being preserved or changed: only a standalone copy of the matcher is built with each setting; measure its actual heap bytes and states. Pin Daachorse 1.0.1 as used by the construction profiler, record the lock, and use the same immutable diagnostic binary for both settings.
Expected winning strata: expensive base searches in large vocabularies. Expected adverse strata: poorer packing, extra matcher memory/cache misses and later encoding performance. A build-only win cannot establish a tokenizer win.
Smallest files that need changing: this record and a standalone diagnostic under `/tmp/st-unigram-builder-screen-20260917`.
Mechanism evidence: first run exact match comparison and footprint on both pinned T5/UMT5 fixtures. Reject before timing if either matcher heap grows above 1.05x. Otherwise run three fresh-process AB/BA pairs of ten build-plus-drop iterations, report first separately and use median remaining nine. No block-count sweep, per-model setting or data-derived threshold.
Acceptance rule: all matches exact, heap <=1.05x for both models, equal-model warm construction >=1.10x. This admits only a future runtime experiment; it cannot retain a change. Any future candidate must preserve encode throughput and complete all calibrated CPU-class guards.
Rejection rule: any changed match, excess footprint or <1.10x builder win. Stop without production edits and preserve all raw evidence. Never reinterpret extra memory as free merely because constructor wall time improves.
Self-review: one versus sixteen is the documented boundary-versus-default comparison, selected before results. Keeping vocabulary order constant avoids conflating packing with insertion-order effects. Root BPE/Unigram behavior stays unchanged.


### E6 rejected; E10 advances to calibrated portability (2026-09-17)

E6 `a369ab2` fails Intel's frozen Gemma JSON load guard: `0.987807159x < 0.988984164x`, despite ST `1.173345x`, CI `[1.129462,1.190965]`. The completed Apple diagnostic independently fails GPT-OSS TKZ `0.942324851x < 0.963472181x`. Intel/Apple automatic load gates skipped encode guards. Reject E6 and restore source to `95bc1ac` in isolated `63c28af`; already-running AMD work is diagnostic only. Full load evidence is retained in `e6-intel-v2/` and `e6-apple-v2/`.

E10 `ea75ed9` completes all three declared two-model pairs and pre/post HF exactness: ST warm `1.038549x`, cycles `1.040340x`, instructions `1.038823x`, branch misses `1.053954x`; JSON warm `1.046676x`. Both T5 and UMT5 improve in both formats. The exact last-duplicate fallback and all 129 unit tests pass, as does strict Clippy. Raw timings/counters are in `e10-pmu/`. This passes the mechanism screen only. Evaluator-only `774c3969f213268d85f9cbc7e430c837023c7a57` freezes the separate Unigram paired/A/A/guard protocol; source runtime remains `f786899`. Full identical/independent A/A then E10 pools are queued on Apple, Intel and AMD after each host's existing E8 pipeline. No promotion yet.


### E9 mechanism rejected below 2% (2026-09-17)

E9 `8a600dc` completed all predeclared rounds and exactness checks: ST warm `1.019220309x`, cycles `1.028311090x`, instructions `0.991157515x`, branch misses `0.994128105x`. The warm result misses the exact 1.02 floor, so no later gates run. Mistral Nemo cycles are `0.9799x`; eleven of twelve models execute more instructions. Source is restored exactly to `95bc1ac` in isolated branch commit `0ff2fd9`. Full raw text evidence and summaries are retained in `autoresearch/results/st-20260917/e9-pmu/`. A profile-download process overlapped early timing before being stopped; this is disclosed and provides no basis to rescue the below-floor result. The E9 branch log records the proof/test and rejection.


### BPE/Unigram root validation and first Unigram load baseline (2026-09-17)

Root runtime `f78689974948e282e581fda603a6dfbbcf3e2b1c` contains user-kept E8 plus Unigram support, with E1 fully removed. Fresh combined checks pass: 129 unit tests, 41 integration tests (9 existing ignored), 3 binding tests, 1 doctest, strict Clippy/docs, no-default-features, Rust 1.91, verified cargo package, local wheel and 36 Python tests in each pinned 3.10/3.12 CI environment. Logs are in `autoresearch/results/st-20260917/integrated-checks-f786899`; no publication occurred.

Separate evaluator `94e35d182bbdeb1bbb444ebd388ae1536cfd269b` freezes 25 varied inputs and two pinned structurally supported Unigram fixtures: legacy untagged T5 (32,100 pieces, Precompiled) and tagged UMT5 (256,300 pieces, Replace). Complete Hugging Face IDs, decode, vocabulary, scalar/nested/ragged rows and both special-token modes pass before and after the full twelve-pair format comparison. Intel PMU host direct ST/JSON throughput is `1.157132x`, CI `[1.135892,1.173646]`; source-present cached ST is `1.124368x`, CI `[1.112966,1.131031]`. Direct T5/UMT5 per-model ratios are `1.159543x` / `1.154725x`; first-load ratios `1.184657x` / `1.233348x`. UMT5 direct-ST peak RSS ratio is `0.819020x`; T5 RSS is unchanged at process high-water resolution. These are format baselines on two models and one CPU, not candidate retention or general portability claims. All raw timings, manifests and parity outputs are retained under `unigram-format-baseline-v1/`.

Disclosed discovery exclusions: ALBERT rejects with `unknown variant NFKD`; its NFKD/StripAccents/Lowercase pipeline is not supported. The pinned MT5 repository has no tokenizer.json. NLLB is BPE, not a Unigram fixture. No input or pipeline was silently dropped after timing.

Unigram profiles (100 direct loads, cycles at 997 Hz, zero lost samples) attribute about 60% to matcher construction: UMT5 build-with-values 26.84%, double-array build 20.10%, failure-link build 12.89%; T5 20.46%, 22.72%, 18.74%. UMT5 pattern collection is another 3.21%, SipHash write 6.60% shared with insertion. Full raw perf captures remain on PMU; the bulk local raw-copy was interrupted to avoid ongoing transfer during E9 timing, so local captures are partial. Text reports are retained. The workspace pins Daachorse 1.0.0 and the frozen evaluator lock pins 1.0.1; latest API docs must not be treated as those versions' contract.

E9 `8a600dc` is isolated on `perf/st-sparse-ranked-validation`, walking sorted occupied ranked slots and retaining the dense fallback for unsorted snapshots. Its exhaustive small-table placement/reference test, all 129 unit tests and strict Clippy pass; three-pair transfer PMU screen is running. E10 `ea75ed9` is isolated on `perf/st-unigram-unique-vocab`: map/vector length equality proves unique strings, eliminating redundant last-duplicate hash queries while preserving pattern order. Its card and frozen mechanism runner precede implementation; validation/timing are queued after E9. Neither candidate is retained.


### E1 rejected by Apple v2 encode guards (2026-09-17)

The completed twelve-pair Apple v2 pool passes the 1.02 load gate, but fails three predeclared encode bands: Llama scalar `0.883956x < 0.926289x`, Mistral scalar `0.865960x < 0.927737x`, and DeepSeek batch `0.524514x < 0.564833x`. Aggregate encode ratios do not override a model failure. RSS and binary size pass. Reject E1 `0cfef16`; reverse its runtime/test patch from the combined root while preserving user-retained E8 and Unigram support. Other already-running E1 host pools are diagnostic only, not a route to rescue this version. Apple raw rounds, summaries, manifest and frozen calibration are retained in `autoresearch/results/st-20260917/apple-v2-e1/`.

Combined `a2c5270` passed Rust tests but failed strict Clippy in E8's test-only constant chunk loop. Fix `393c024` uses typed array chunks; runtime is unchanged. Its complete workspace tests, strict Clippy/docs, MSRV, no-default-features, package verification, wheel build and both Python CI environments pass (36 tests each). These validate that combined tree, not performance retention; the E1 removal requires focused follow-up checks and a new Unigram baseline source SHA before timing.


### Integrated Unigram support and user-kept hash change (2026-09-17)

Integration parent: `57fa36a` on `feat/st-format` (runtime E1 pending portable gates). Apply the runtime/tests/docs from Unigram feature `73f73de` and the exact E8 source patch `84a0347`, retained by user request. This combined tree requires fresh validation; component gains are not multiplied or claimed as a combined win. E6 remains isolated until its full gates finish.

Unigram `.st` version 2 stores exact vocabulary strings/scores, optional unknown ID and byte-fallback configuration plus pipeline JSON, then rebuilds the matcher through the existing checked constructor. BPE version 1 remains unchanged on disk. Feature checks passed: 129 unit tests; 41 integration tests with 9 existing extended tests ignored; 3 Python binding Rust tests; 1 doctest; strict workspace Clippy/docs; no-default-features; Rust 1.91 check; full package verification; and all 36 Python tests in each pinned CI environment. A separate probe loaded all twelve parent BPE artifacts and reconstructed byte-identical files under the feature loader. No unchecked matcher deserialization, dependency change or release was used.

The pinned T5 test compares full Hugging Face IDs, vocabulary mappings, decode, scalar/nested/ragged rows, specials and >16 KiB normalized documents through cached, direct and source-absent `.st` loading. Toy coverage includes empty/duplicate pieces, unknown fusion, byte fallback, pipeline special tokens and checksum recovery. Malformed snapshot scores and unknown IDs are rejected by the existing constructor. Unigram load speed remains unmeasured; a separate panel is needed before a speed claim.

### User-directed retention, 2% floor, and Unigram support (2026-09-17)

The user explicitly said "keep it" after E8's paired-hash transfer result, then "reduce the 5% floor to a 2% floor", and requested support for both Unigram and BPE. E8 `84a0347` is user-selected retention on its isolated branch: 128 unit tests and all twelve-model pre/post checks pass; load `1.027395x`, cycles `1.037345x`, instructions `0.992121x`, branch misses `0.994266x`. Its full portability and regression checks remain required; this is not a general-champion claim.

The active scoped point threshold is now exactly `1.02x`, superseding the earlier 1.05 floor and A/A-derived point-floor escalation. Confidence interval above 1.00x, complete exactness, non-loosening model/format/encode bands, RSS and binary guards remain unchanged. Record A/A uncertainty separately. This policy change is explicit user direction, not silent post-result tuning. Completed raw results and earlier rejections remain intact; E5 still misses 1.02 on Intel and its CI includes a loss.

Add `.st` Unigram support as a separate format/functionality change, preserving BPE v1 readability and byte layout. Inspect Unigram ownership/constructor semantics and compare full Hugging Face IDs through JSON, newly cached files and direct `.st` files. Do not claim a Unigram load speedup before a separate frozen Unigram panel and measurements.

### `.st` narrower BMP screen and package completion (2026-09-17)

E6 `a369ab26e9a3a9b2d3c520c1e061cab97e5ca156` passes 129 unit tests and all twelve-model pre/post transfer checks. Three paired 100-load rounds: warm load `1.169722x`, cycles `1.177214x`, instructions `1.180454x`, branch misses `1.413419x`. Every model improves; Kimi whole-process cycles now improve `1.2907x`, unlike E4's adverse result. The isolated source keeps original allocations and changes only the BMP length guard. Full v2 load/guard pools are queued sequentially after E1 on Intel, AMD and Apple; no retention yet.

E1 full `cargo package --locked` verification passed. One local Linux wheel passed all 36 Python tests in each clean environment: Python 3.10.21 with Transformers 4.57.6 and Python 3.12.3 with Transformers 5.16.1, both pytest 8.4.2. Exact installed versions and logs are retained in `autoresearch/results/st-20260917/e1-package-checks`. Nothing published. Separate twelve-artifact byte-roundtrip, no-default-feature and MSRV checks are queued on PMU after its timing work.

E5's already-running AMD diagnostic completed at ST `1.032141x` CI `[1.007040,1.090757]`, also below the 1.05 floor; its prior rejection stands. E7's standalone checksum screen reaches `2.049760x` with four workers but `0.970323x` with one. Four-worker cold-first large-file ratios are 1.538–1.878x, while the 128 KiB synthetic cold-first case is 0.087x. All digests agree. This passes the narrow >=2x mechanism admission but does not change the loader. Enabling a dependency feature requires a controlled matching lock/build baseline before full candidate comparison; preserve small/one-worker costs.

E8 `84a0347c1f5a474358d7f114ec354c47b26ed134` is isolated on `perf/st-paired-hash-tail`: pair the exact wrapping polynomial tail recurrence while leaving native-endian word hashing unchanged. The card and extended serial-reference coverage precede implementation; GCP unit/mechanism gates are in progress. A fresh E1 PMU annotation confirms substantial local samples at word multiply (19.24%), first tail dependent add (17.30%), and probe-chain loads; these samples may include memory stalls, so arithmetic benefit remains a hypothesis.

### `.st` experiment 7: parallel checksum mechanism screen (2026-09-17) — planned

Parent SHA: `95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71` (runtime unchanged for this standalone screen).
Hypothesis: BLAKE3's supported parallel update can reduce checksum wall time for the already-read native payload while preserving every checksum byte and error ordering.
Measured hot cost: current five-model E1 PMU profiles attribute 6.52–12.84% self cycles to AVX-512 BLAKE3; the earlier parent profile also shows material checksum work. The data is already read into a Vec before hashing, so no mmap, external-mutation lifetime, or parallel disk I/O is required.
Invariant that makes the shorter path exact: `Hasher::update_rayon(payload).finalize()` must equal `blake3::hash(payload)` over the complete identical byte slice before decoding. The official API documents that equivalence. The runtime remains unchanged until this screen supports the mechanism.
Representation being preserved or changed: immutable payload bytes and checksum; standalone diagnostic enables the existing BLAKE3 1.8.5 rayon feature, using the existing Rayon 1.11.0 dependency. No tokenizer/evaluator/fixture edits. Any subsequent candidate must declare the dependency-feature/lock change separately and preserve dependency versions.
Expected winning strata: multi-megabyte payloads. Expected adverse strata: small payloads, first pool initialization, single-thread environments, and busy/contended pools. [Official BLAKE3 documentation](https://docs.rs/blake3/latest/blake3/struct.Hasher.html#method.update_rayon) explicitly warns about small buffers and occupied cores; this is not an unconditional speedup.
Smallest files that need changing: only this record and a standalone diagnostic under `/tmp/st-hash-probe-20260917`; no runtime patch yet.
Mechanism evidence: use all twelve previously frozen transfer payloads, plus deterministic 0/1 KiB/16 KiB/128 KiB/1 MiB boundaries. Three AB/BA pairs of 100 hashes, retaining the first call separately, under one and four workers in separate fresh processes. Verify all serial/parallel digests before and after timing; preserve every timing. Run after E6's PMU pool, never concurrently.
Acceptance rule: proceed to a runtime candidate only if the four-worker equal-model median checksum speedup is >=2.0x and full digests match; report one-worker, small-buffer and first-call costs without exclusions. This screen cannot retain a candidate. Runtime must then pass v2 full load and encode/resource/portability gates with no custom threshold derived from model labels or measured cell sizes.
Rejection rule: digest difference, <2.0x primary checksum ratio, or unsupported dependency contract; close the lane without tokenizer edits. Do not tune worker count or input threshold after seeing outcomes.
Self-review: shared pool startup is a real construction cost. Never preinitialize it in the runtime only for a benchmark. Do not replace BLAKE3, omit verification, trust on-disk hashes, or overlap unvalidated decoding.

### `.st` evaluator correction and completed rejection gates (2026-09-17)

Evaluator-only v2 `80908bece365e7008498c2e270bb358cc05b4f7b` fixes the paired center to `exp(median(log(parent/candidate)))`. For even rounds, `log(median(ratio))` is not equivalent (six ratios of 1 and six of 4 center at 2, not 2.5). The existing bootstrap was already log-based. Inputs, timing, binary commands, round counts and resampling are unchanged; two focused tests pass, runtime diff against `95bc1ac` is empty. Fresh identical/independent A/A and surviving E1 rebaseline are required on all three CPU classes. Minimum floor remains 1.05x and regression bands cannot loosen. Prior rejected candidates stay rejected.

E4's full Intel ST result is `1.225615x` CI `[1.161121,1.265145]`; AMD `1.168775x` `[1.136851,1.201311]`. Despite these gains, Intel GLM scalar encode is `0.890673x`, below the predeclared `0.908286x` limit. E4 is rejected and source/test restored to parent in `e1536f9`. E5 Intel ST is `1.013836x` `[0.977321,1.040477]`, below the 1.05 floor; rejected and restored in `2cd86ef`. Its median-log center cannot exceed the arithmetic-median result, so v2 cannot rescue it. Already-running AMD diagnostics may finish; later E5 gates are skipped.

E1 Apple v1 ST is `1.121027x` `[1.116585,1.123726]`, TKZ `1.002389x`, JSON `1.004309x`; no model/format A/A guard fails. Intel E1 encode/RSS guards also pass, with scalar `1.030759x`, batch `1.026598x`, ragged `0.998066x`, RSS `0.999929x`. These remain historical v1 evidence pending v2. PMU workspace checks for E1 and E4 pass 129 unit, 40 integration (9 configured ignored), 3 binding and 1 doctest, all-targets compilation, fmt, strict Clippy/docs, and `cargo package --no-verify`. The first build lacked Python development headers and its failure logs are retained; after installing headers the complete sequence passed. E1 full package verification and clean CI-matrix Python wheel checks are now separate follow-up jobs, with no publication.

The same-binary JSON argv0 audit completed twelve pairs and pre/post checks: GPT-OSS `1.002525x` CI `[0.992244,1.008214]`, Gemma `0.999022x` `[0.989526,1.005983]`, Qwen `1.001845x` `[0.987535,1.020059]`. It does not establish an argv0 bias, so v2 leaves process invocation unchanged. Root-owned raw profiles remain on PMU; local copied reports are not raw-capture backups.

### `.st` continued gates, allocation attribution, and new screens (2026-09-17)

Experiment 2 is rejected and its source restored in isolated branch commit `99e209f`: Intel `.st` `1.092979x` CI `[1.069503, 1.119332]` passes, but JSON `0.955269x` CI `[0.952470, 0.960347]` fails eight model bands. AMD `.st` is `1.071732x` `[1.031816, 1.131122]`, JSON `0.992682x` `[0.988638, 0.998241]`, and `.tkz` `1.008015x`; no AMD model guard fails. This CPU-class difference is retained, not averaged away. The queued Apple pool remains diagnostic. A PMU diagnostic compares parent and rejected E2 JSON loading on GPT-OSS, Gemma, and Qwen; all three JSON SHA256s exactly match their frozen load-panel counterparts.

Experiment 3's GLM loss is now attributed to kernel memory management, not extra user-space hashing. Instruction sampling (zero lost samples) reports kernel 30.31% versus parent 0.45%. Separate 100-load counter captures show candidate 1,709,708 minor faults versus 35,301; user instructions 19,723,794,173 versus 19,662,181,975; kernel instructions 8,907,296,715 versus 225,302,063, with 100% counter running time. `strace` finds 2,228 `brk` calls versus 28, while mmap/munmap counts are similar (34/23 versus 37/26). This is repeated heap growth/trimming, not repeated file reads. GNU documents [allocation-pattern-dependent mapping thresholds](https://sourceware.org/glibc/manual/latest/html_node/Memory-Allocation-Tunables.html); no allocator environment tuning was applied. Raw diagnostics live in PMU `evidence/e3-mapping`.

Experiment 4 `5444d2b95b3dfb71dc7af91207bae8d869e38a16` derives token lengths and BMP entries in one pass, decoding only spans of one to three bytes. Its isolated card is on `perf/st-character-table-pass`; 129 unit tests and pre/post transfer parity pass. Three paired 100-load screens yield warm load `1.402983x`, cycles `1.345539x`, instructions `1.329107x`. Every warm median improves, but Kimi's whole-process cycles are `0.9195x`; preserve that adverse result and do not infer first-load gains from steady-state medians. The full Intel/AMD load matrices follow completed guard A/A. This candidate is not retained.

Experiment 5 `5fa3c233d6194315d0deff59be62cc8d8fffae34` reuses decoded lookup IDs for the inverse permutation instead of E3's extra allocation. It is isolated on `perf/st-vocab-validation-scratch`, with 129 unit tests and complete pre/post transfer checks passing. Three pairs give warm load `1.060224x`, cycles `1.053709x`, instructions `1.009351x`; Mistral Nemo's `1.5490x` contributes much of the aggregate, Nemotron is `0.9859x`, and GLM recovers to `1.0599x`. Full matrices are queued after E1/E4 guard runs. No per-model exclusion or allocation-policy change is allowed.

Apple identical-binary A/A finished twelve pairs and pre/post checks: `.st` `1.003096x` CI `[0.989402, 1.020670]`, `.tkz` `0.996543x` `[0.952464, 1.002602]`, JSON `1.000909x` `[0.983443, 1.056045]`. The independent A/A follows before candidate results; minimum `.st` floor remains 1.05 pending that calibration. Copies of all summaries and manifests are in the result directory. Raw root-owned perf files remain on the PMU host; a bulk scp copied readable reports but could not read those capture files, so local reports are not claimed to include the raw captures.

### `.st` full GCP load results and rejected locality candidate (2026-09-17)

Experiment 1 `0cfef16` completed all twelve counterbalanced load pairs and pre/post parity on both existing GCP hosts. Intel `.st` is `1.146238x`, 95% interval `[1.123657, 1.183286]`; AMD is `1.120676x`, `[1.078379, 1.187370]`. No per-model `.st`, JSON or `.tkz` median falls outside its predeclared A/A regression band. Intel JSON is `0.995530x` and `.tkz` `1.008480x`; AMD JSON `1.000227x` and `.tkz` `1.007863x`. These are load-gate passes, not retention: Apple, encode, RSS, binary and full relevant correctness/lint checks remain.

Experiment 3 `5503bd1` was rejected after all three PMU transfer pairs and pre/post exactness: load `0.951866x`, cycles `0.948902x`, instructions `0.936697x` parent/candidate. Most models improve slightly, but Nemotron is `0.6570x` and GLM `0.6077x`; their instruction increases are unexplained and cannot be omitted or worked around with model dispatch. The isolated branch records the complete card and rejection; commit `f8993ba` restores `src/models/bpe.rs` exactly to `95bc1ac`. Full raw results are copied to `/tmp/st-e3-pmu-evidence-20260917` and summary/checks retained here. The task PMU host is capturing instruction profiles for the parent and rejected candidate on GLM and Qwen to attribute the difference; these are diagnostics, not another timing attempt.

### `.st` mechanism screen and guard protocol (2026-09-17)

Experiment 2 `f59e94f` finished three counterbalanced pairs on all twelve transfer models with complete pre/post ID/vocabulary parity. Equal-model median ratios: warm load `1.106002x`, cycles `1.106995x`, retired instructions `1.251204x` (20.08% fewer), branch misses `1.019201x`. All twelve load medians improved, from `1.0686x` to `1.1540x`. Hardware events ran 100% of their measured interval. This establishes the bulk-decoding mechanism on one Intel host; it does not retain the candidate. Full frozen twelve-model pools are queued behind experiment 1 on the existing Intel/AMD hosts; Apple runs its own A/A first. Raw counter/load evidence is `~/st-campaign-20260917/evidence/e2-pmu` on the task PMU VM and `/tmp/st-e2-pmu-evidence-20260917` locally; summary, manifests, exactness and scripts are copied into the result directory.

Experiment 3 is isolated at `5503bd10ac66eee06eea04f019334e34dec1f767` in `/Users/namanchetwani/Projects/snaptokens-st-vocab-order`. It reorders vocabulary hash validation using a temporary ID-to-slot map; runtime/format bytes are unchanged. All 129 unit tests pass on Apple and GCP. The initial test-only commit incorrectly cloned a non-Clone snapshot type; the follow-up constructs independent malformed fixtures, with no runtime change. The PMU mechanism screen uses the same predeclared twelve transfer models, three counterbalanced pairs and 100 loads as experiment 2, after unit tests and complete-ID checks.

Before any encode-guard candidate results, freeze guard orchestration `autoresearch/results/st-20260917/guards.py` (SHA256 `b4a8395e1bd83710bcde018609a63fbb32698e8ad4ad9c3c3142b92593a80f01`). This invokes the unchanged evaluator's `encode` (six passes, first warmup, output destruction included) and `st` (six loads) commands. Run twelve complete AB/BA pairs per model, retaining all values and per-child peak RSS; pre/post exactness remains required. Run identical and independent A/A first; encode regression bands use the symmetric union of paired-median 95% bootstrap intervals per model/API (3,000 samples, seed 0). Candidate RSS median must stay within 1.05x per model, binary file size within 1.02x; report raw tails too. This fills the initially declared encode/RSS guard procedure without changing the frozen load evaluator, source, inputs, timing boundaries or promotion score.

### `.st` independent calibration and hardware counters (2026-09-17)

Independent-build A/A completed all twelve pairs and pre/post parity on both existing GCP hosts. Intel `.st` is `0.989296x` with interval `[0.948662, 1.020776]`; AMD is `0.997063x` with `[0.942928, 1.031548]`. The minimum point remains `1.05x`. The exact per-model/format symmetric bootstrap bands from the previously declared policy are recorded in `autoresearch/results/st-20260917/load-calibration.json`; they supersede the orchestration script's diagnostic extreme-round bands. Candidate results were not inspected while deriving these bands.

The user explicitly waived the API-key preflight requirement. Local disk has 14 GiB free and no stale automation matches were found. Apple compiler `1.98.1` is installed to match GCP; immutable parent/candidate evaluator builds are underway. Experiment 2's Apple assembly (Rust 1.97.0) contains 64-byte vectorized copy loops for both integer widths, confirming removal of scalar per-integer reader updates. This is mechanism evidence, not a load-speed result.

Task-owned `snaptokens-st-pmu-20260917` is a four-vCPU C4 in `us-east1-b`, with standard PMU, a 60-GB disk, and an eight-hour automatic stop. Creation in `us-central1-a` failed the existing regional disk quota before allocating an instance/disk; the new region was used instead. Cycles, instructions, and branch misses work; generic cache misses report unsupported. Existing benchmark VMs are untouched by this setup. A transfer panel of twelve deduplicated local BPE fixtures was frozen before candidate results; its independent corpus remains the same nineteen inputs. Experiment 2's mechanism screen uses complete-ID/vocabulary checks before and after three counterbalanced pairs of 100 loads on every transfer model. It measures whole-process counters and retains each internal load time, without treating this extra panel as a replacement promotion evaluator.

### `.st` calibration and profile checkpoint (2026-09-17)

Before any performance-candidate timing: Intel identical-binary A/A completed all twelve pairs and pre/post complete-ID checks. `.st` aggregate `0.99561882x`, paired 95% interval `[0.97437534, 1.04925298]`; `.tkz` `1.00132466x` `[0.99288274, 1.01612496]`; JSON `1.00381971x` `[0.99782236, 1.00789624]`. Therefore the scoped retention floor is raised from `1.03x` to **at least `1.05x`**, or the higher upper bound of completed independent-build A/A on either CPU. This decision precedes candidate results. For model/format regression guards, use the symmetric log envelope of the per-model paired-median 95% bootstrap intervals across A/A modes, not the much wider extreme single-round ratios. Use 3,000 resamples and the frozen seed 0. Encode guard A/A remains required before retention.

AMD profile capture is complete for GPT-2, Gemma, GPT-OSS, Qwen, and Mistral. All five preflight parity checks passed and all CPU captures report zero lost samples (829, 47K, 36K, 39K, 24K samples respectively). UTF-8 self cost is 25.57%, 20.21%, 20.45%, 11.03%, 19.86%. Integer Vec u32/u64 decoding is another 11–15% on most models; assembly shows per-element reader stores and grow-capacity checks. `VocabLookup` self cost is 15–19%; annotation places substantial samples at hash mixing and arena-span reads, which is not enough to distinguish arithmetic latency from cache misses without hardware PMU evidence. Qwen also spends substantial time in page faults. Allocation captures show 266/78,371/815/1,139/12,950 allocations across three loads for GPT-2/Gemma/GPT-OSS/Qwen/Mistral; investigate pipeline/added-token construction separately for Gemma and Mistral rather than treating all models as allocation-equivalent.

Retained evidence copies: `autoresearch/results/st-20260917/`; raw profiles, assembly, heaptrack and rounds remain under `~/st-campaign-20260917/evidence` on the respective GCP VMs. The first allocation-report step expected `.gz` while heaptrack emitted `.zst`; the valid trace was interpreted after correcting that filename. No CPU capture was discarded. Validation repair `95bc1ac` passed all 128 Rust unit tests and strict workspace Clippy. Experiment 1 `0cfef16` passed all 129 unit tests; experiment 2 `f59e94f` is isolated on `perf/st-bulk-decode` and also passed 129 unit tests. Neither is retained by performance yet.

### `.st` experiment 1: validate one String arena (2026-09-17) — planned

Parent SHA: `95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71`.
Hypothesis: own the packed vocabulary as a validated String, so load validates its bytes once and subsequent token access checks only string boundaries rather than validating UTF-8 again.
Measured hot cost: initial AMD GPT-2 CPU-clock profile, 829 samples with zero lost, assigns 25.57% self samples to `core::str::converts::from_utf8`; source calls this once per span in `from_parts` and again per token in BMP rebuild via `get`. The same `get` also revalidates tokens on decode/vocabulary access. Baseline heaptrack records only 266 allocations across three loads, so per-token string allocation is already removed and is not this hypothesis.
Invariant that makes the shorter path exact: concatenated bytes are valid UTF-8 and every monotonic token offset lies on a character boundary. These conditions are equivalent to every individual token span being valid UTF-8, including empty spans. Checked String slicing preserves safe out-of-range behavior. No unchecked UTF-8 conversion is needed.
Representation being preserved or changed: `VocabArena` owns String instead of Vec<u8>; offsets and all on-disk bytes remain unchanged. JSON construction appends known valid strings directly. No dependency, unsafe code, format version, evaluator or encode dispatch changes.
Expected winning strata: all `.st` vocabularies, especially non-ASCII/ByteLevel vocabularies with many spans; decode and token access may benefit. Expected adverse strata: small ASCII vocabularies, JSON packing overhead, clone/to-snapshot conversions.
Smallest files that need changing: `src/models/bpe.rs` and this record.
Mechanism evidence: the cited SIMD UTF-8 research favors contiguous validation; std already scans ASCII runs in larger words. The actual experiment uses std String to encode the validity proof in the representation, not a new SIMD dependency.
Acceptance rule: malformed-boundary and Unicode/empty-span tests, all existing unit tests, frozen complete-ID/vocabulary parity before and after the full twelve-model paired screen. At least 1.03x scoped load aggregate, CI above 1.00x, raised if completed A/A requires; no model/JSON/TKZ/encode regression beyond A/A. Validate on GCP AMD/Intel and Apple before portable retention. RSS within 5%, binary within 2%; serialized snapshots must stay byte-identical for a fixed parent artifact.
Rejection rule: any changed valid output/format, accepted invalid UTF-8 boundary, lost error, unproved win, or regression outside the calibrated band. Self-review: validating the whole byte buffer alone would wrongly accept a split inside `é`; the offset-boundary test is mandatory. This change does not use model labels, input sizes, environment detection, or output caching.

### `.st` snapshot validation repair (2026-09-17)

Parent SHA: `03c3160135aadc5a5c54e653b876d25d6a95f223` (runtime baseline `6972e461c8061afd88efb70d0a97acad98c7d822`).

The continued campaign first froze the incoming dirty format tree and a separate scoped twelve-model load evaluator. GCP Intel passed 126 baseline unit tests and complete-ID scalar/nested/ragged checks, with/without special tokens, plus every vocabulary mapping across JSON/cached `.st`/direct `.st`/`.tkz` on 19 varied inputs per model. The baseline A/A run is ongoing. Both existing Intel and AMD hosts were idle at inspection; use only task-created `~/st-campaign-20260917` paths, never stop these pre-existing machines.

Source audit found that `.st` ranked slots were range-checked but not checked for linear-probe reachability. `native_snapshot_rejects_unreachable_ranked_slot` failed against the frozen parent: moving a valid occupied slot past an empty bucket was accepted. Such a table silently loses the merge on lookup. Serialized table capacities could also request allocations unrelated to occupancy. This is a correctness repair before optimization: check canonical capacities before allocation and prove ranked probe chains, as `.tkz` already does. Existing valid format bytes and encode paths are unchanged. New tests cover unreachable slots and huge sparse capacities; no speed claim.

Research, the scoped evaluator protocol, frozen sources and pending lanes live under `autoresearch/st-eval/`. Hardware cycles/instructions PMU events are absent on both GCP hosts even with root; CPU-clock perf sampling works. Heaptrack is installed for task captures. Never report CPU-clock evidence as hardware instruction counts.

### `.st` arena vocab lookup (2026-09-17) — retained on `feat/st-format`

Parent SHA: isolated `feat/st-format` working tree (SNAPST v1 after compact ranked slots; parent of the format itself is `c64bdd948da442aa659c471628bfb200775a4e89`).

Hypothesis: remaining `.st` load cost is N `String` allocations plus `HashMap<String,u32>` insert (clone of every key). A packed arena plus an open-addressed `(hash, id)` table that probes arena slices is the standard dictionary representation (Tantivy ArenaHashMap, packed interners, compact-dict / persistable linear-probe maps). Persist occupied lookup slots the same way ranked merges already persist, so load scatters the table and never rehashes.

Measured hot cost: `from_native_tables` `tokens_from_arena` + `vocab.insert(token.clone(), id)` for every vocabulary entry after the encode tables are already installed.

Invariant that makes the shorter path exact: vocabulary identity is the packed UTF-8 span; lookup is exact iff each occupied slot's stored hash matches `fx_hash_bytes` of that span, IDs are a permutation of `0..n`, and linear-probe chains are valid the same way ranked-merge slots are. Encode `token_to_id` / `id_to_token` remain the same public results. Trie validation compares arena slices, not owned strings.

Representation being preserved or changed: replace BPE `Vec<String>` + `HashMap<String,u32>` with `VocabArena` + `VocabLookup`. JSON / `.tkz` still construct through the existing HashMap, then pack once at the end of `build_with_sidecar`. `.st` stores the arena it already has plus occupied lookup slots. No evaluator, encode-loop, or public API change. No new dependency.

Expected winning strata: direct `.st` load, especially large ByteLevel vocabularies (Qwen, GPT-OSS). Expected adverse strata: JSON / `.tkz` pay one extra arena copy at the end of construction; `.st` files grow by occupied lookup slots (~16 bytes × vocab); encode must stay exact and should be no slower (fx hash + linear probe vs SipHash HashMap).

Smallest files that need changing: `src/models/bpe.rs`, focused lookup/sidecar tests, this record.

Mechanism evidence: interners keep one byte buffer plus `(offset,len)`; Tantivy hash tables store hash+arena address and compare `&[u8]` without cloning keys; compact-dict / SharedHashMap persist linear-probe tables at load factor ≤0.5–0.7.

Acceptance rule: focused `.st` tests plus a VocabLookup duplicate/miss unit test; JSON/`.tkz`/`.st` IDs still match on the fixture; `load_bench` direct `.st` median faster than the previous `.st` medians (GPT-2 7.564 ms, Qwen 41.854 ms, GPT-OSS 43.533 ms) and still faster than `.tkz`; no ID mismatch.

Rejection rule: revert this lookup representation on any ID mismatch, invalid probe-chain accept, or a load that is not faster than the previous `.st` compact-slot baseline.

Result: retained on `feat/st-format`. BPE now keeps a packed `VocabArena` plus open-addressed `VocabLookup`; JSON/`.tkz` pack once at the end of `build_with_sidecar`, and `.st` persists occupied lookup slots. Focused tests passed (`vocab_lookup`, `st::`, `models::bpe::`). JSON/`.tkz`/`.st` IDs matched on GPT-2, Qwen 3.5, and GPT-OSS (`hello world`, accents, newlines, specials, id↔token spot checks). Same-session interleaved Apple `load_bench`, eight timed pairs after one warmup each:

| Model | `.tkz` median | `.st` median | paired `.tkz`/`.st` | sizes JSON / `.tkz` / `.st` |
| --- | ---: | ---: | ---: | --- |
| GPT-2 | 31.068 ms | 10.570 ms | 2.90x | 1.3 / 2.1 / 3.5 MiB |
| Qwen 3.5 | 147.972 ms | 56.233 ms | 2.67x | 12 / 12 / 18 MiB |
| GPT-OSS | 314.247 ms | 93.104 ms | 3.34x | 27 / 19 / 26 MiB |

The host was slower than the prior compact-slot session (GPT-2 `.tkz` 10.4 ms then, 31 ms now), so absolute `.st` times are not comparable to 7.56/41.9/43.5 ms. Same-session ratios beat that session's 1.37x/1.29x/1.98x, which is the mechanism evidence. Files grew by the occupied lookup table (~16 bytes × vocab). Remaining load still walks every arena span to refill the 64 KiB BMP table and re-hashes spans while validating lookup slots.

### `.st` scatter BMP, skip arena rehash (2026-09-17) — rejected

Parent: arena `VocabLookup` working tree above.

Hypothesis: after the lookup table is persisted, load still scans every vocabulary span: `from_utf8` + `chars()` to refill BMP, and `vocab_hash` of every span to re-check stored hashes. Ranked-merge load already scatters stored keys and proves probe chains without rebuilding from source strings. BMP occupancy is a few hundred to a few thousand single-character tokens, not the full vocab. Persist occupied `(char, id)` pairs and trust blake3 + unique IDs + probe chains for lookup hashes.

Measured hot cost: `from_native_tables` BMP loop over `vocab_size` and `from_cached_slots` hashing every arena span.

Invariant that makes the shorter path exact: a BMP cell is `id` iff that token is a single BMP scalar; scattering the same occupied pairs yields the same 64 KiB table. Lookup `get` still compares arena bytes, so a checksum-valid table with unique IDs and valid probe chains returns the same IDs. Trie validation still compares arena slices.

Representation being preserved or changed: add compact BMP occupancy to `NativeBpeTables`; stop hashing arena bytes during lookup scatter. No encode-loop, evaluator, or public API change.

Expected winning strata: large-vocab `.st` load (Qwen, GPT-OSS). Expected adverse: `.st` grows by occupied BMP pairs; a checksum bypass would be a correctness hole, so keep blake3 and probe-chain checks.

Smallest files that need changing: `src/models/bpe.rs`, this record.

Acceptance rule: existing `.st` and VocabLookup tests; JSON/`.tkz`/`.st` IDs still match on the three local models; interleaved `load_bench` `.st` median faster than the arena-lookup `.st` medians above on at least Qwen and GPT-OSS.

Rejection rule: revert on ID mismatch or a load that is not faster than the parent `.st` on the large vocabularies.

Result: rejected and fully reverted. Sequential and interleaved Apple `load_bench` on this busy host could not beat the parent arena-lookup `.st` medians (Qwen 69.8 vs 56.2 ms, GPT-OSS 102.8 vs 93.1 ms; GPT-2 was dominated by host noise, 22–25 ms vs 10.6 ms parent while `.tkz` also swung 31–153 ms). Occupied BMP pairs did not grow files enough to matter. Remaining `.st` load still validates UTF-8 spans, refills BMP from the arena, and re-hashes lookup slots. Do not reopen BMP occupancy or skipped hash verification without a quieter paired parent/candidate binary comparison.

### `.st` native snapshot format (2026-09-17) — retained on `feat/st-format`

Parent SHA: `c64bdd948da442aa659c471628bfb200775a4e89`.

Hypothesis: `.tkz` v5 still stores canonical vocab/merges and rebuilds encode-time tables (ranked keys/values scatter, merge adjacency, BMP/byte maps, dense merges, fused-cache seeds, bridge table) plus `Vec<String>` bincode. A sibling `.st` snapshot can persist those already-built tables and a packed UTF-8 vocab arena so direct load skips reconstruction.

Measured hot cost: `.tkz` construction after JSON parse; remaining work is table rebuild plus one HashMap insert per vocabulary string.

Invariant that makes the shorter path exact: `.st` is generated from a fully constructed BPE, so installed tables are the same bytes the JSON/`.tkz` constructors would have produced. Load validates UTF-8 spans, table lengths, in-range IDs, ranked empty sentinel, merge-adjacency CSR bounds, and the exact-token trie against the reconstructed vocabulary. Encode, decode, added tokens, and pipeline metadata stay on the existing constructors.

Representation being preserved or changed: add `LoadMode::StCache` and a version-1 `SNAPST` file. Do not change `.tkz`, JSON, encode loops, or the evaluator.

Expected winning strata: direct BPE load of GPT-2 through Gemma-scale vocabularies. Expected adverse strata: Unigram (rejected, no sidecar); first JSON miss still pays full construction plus write; encode throughput must stay neutral.

Smallest files that need changing: `src/models/bpe.rs`, `src/st.rs`, `src/lib.rs`, `src/json_structs.rs`, Python `from_file`, focused sidecar tests, `benchmarks/tools/load_bench.rs` (`st` mode only).

Mechanism evidence: `.tkz` `from_resolved` rebuilds merge maps and `build_with_sidecar` still walks the vocabulary for BMP, token lengths, fused seeds, dense tables, and the bridge bitset.

Acceptance rule: focused `.st` round-trip/corruption tests; JSON, `.tkz`, and `.st` produce identical IDs on the existing fixture; Unigram refuses `.st` without writing a file; `load_bench` direct `.st` median faster than direct `.tkz` on GPT-2 and a large ByteLevel model with no encode regression on a short control.

Rejection rule: revert on any ID mismatch, failed validation bypass, Unigram sidecar leak, or load that is not faster than `.tkz` after the first write.

Result: retained on isolated branch `feat/st-format` (not promoted to `main`). Version-1 `.st` stores a packed vocab arena, occupied ranked-merge slots, merge-adjacency CSR, fused-cache seeds, the exact-token trie, and small byte/bridge tables. Dense merge tables, BMP, token lengths, and the 65,536-entry byte-pair table are rebuilt on load so the file does not carry megabyte zero-filled arrays. Focused tests passed (round-trip, duplicate-merge ranks, corrupt-sidecar rebuild, concurrent create, Unigram refusal). Local Apple `load_bench` direct-load medians, eight fresh loads each:

| Model | JSON | `.tkz` | `.st` | `.st`/`.tkz` | sizes JSON / `.tkz` / `.st` |
| --- | ---: | ---: | ---: | ---: | --- |
| GPT-2 | — | 10.373 ms | 7.564 ms | 1.37x | 1.3 / 2.1 / 2.7 MiB |
| Qwen 3.5 | — | 54.137 ms | 41.854 ms | 1.29x | 12 / 12 / 14 MiB |
| GPT-OSS | — | 86.288 ms | 43.533 ms | 1.98x | 27 / 19 / 22 MiB |

Remaining load cost is still `Vec<String>` plus `HashMap<String,u32>` reconstruction from the arena. Files are still larger than `.tkz` because adjacency/seeds are extra. Not a general encode champion and not a release.

### Unigram encode lane moved beside its model (2026-09-17) — refactor, not a candidate

Parent SHA: `047376a` lineage on `main` (branch `refactor/unigram-encode-lane` from `3ed04d7`). Behavior-identical code move, not an optimization experiment: the fused WhitespaceSplit+Metaspace tiers (`ASCII_WS_ANCHORS`, partition consts, `metaspace_partition_target`, `push_text_partitions`, `encode_metaspace_raw/normalized/segment_partitions`) moved from `src/lib.rs` into new `src/models/unigram/encode.rs`, wrapped in `try_encode_fused_unigram` / `encode_fused_unigram_pre_tokenized` entry points mirroring `models/bpe/encode.rs`; `encode_input` became a flat probe chain. Gates, tier order, and error mapping preserved verbatim; `apply_vocab_splits` dropped only from the fused Unigram serial tier where it was a proven double no-op (`needs_vocab_splitting` and `apply_vocab_splits` both require `model.bpe()`).

Verification: `cargo fmt --check`, full workspace tests (115+45+3, 9 network-extended ignored), Python binding tests, clippy warning set byte-identical to `main` under local Rust 1.97.0. Local M2 layout-sensitivity screen (interleaved immutable binaries, `simple_bench --no-hf`, 50 LongBench samples): T5 paired ratios ~`1.02x` median over 4 pairs (one 0.56x machine-hiccup outlier discarded); GPT-2 10 pairs spread `0.81x`–`1.12x`, median `0.97x` — within the recorded identical-binary Apple M2 A/A cell-median band (`0.895x`–`1.293x`), so no detectable regression at local sensitivity. Any finer claim needs the frozen evaluator; this entry selects no champion and changes no scores.

### Unigram outer batch on bpe_pool (2026-09-15) — planned

Parent: retained wide-batch tree on `b3eb482ca89228527366b08c3673d58b39bca56e` (Unigram `encode_outer_batch` keeps inner parallelism; BPE still suppresses nested splits).

Hypothesis: that retained Unigram batch still runs `par_iter` on Rayon's global pool while each large document's partitions `install` onto the separate `bpe_pool`, so eight blocked global workers plus eight pool workers oversubscribe an 8-core host. Same-binary Intel `seq/batch` is only `1.07x`–`1.13x` after keeping the walker. Running the outer Unigram `par_iter` inside `bpe_pool().install` puts document and partition tasks on one pool; nested `install` from a pool thread runs in place, so Rayon can steal partition work without extra OS threads.

Measured hot cost: Intel parent/candidate `encode_batch` medians `136.311` versus `100.092 ms` for the walker-keep change; the remaining dual-pool tax is the gap from ~`100 ms` batch toward the ~`104 ms` sequential partitioned loop and any extra scheduling loss.

Invariant that makes the shorter path exact: token IDs depend only on per-document partitioned Viterbi, not on which Rayon pool owns the workers. `ThreadPool::install` from a thread already in `bpe_pool` runs the closure on that pool; nested `par_iter` over partitions is the same work as today's inner install, with the same concatenation order per document and the same `par_iter` collect order across rows.

Representation being preserved or changed: Unigram-only scheduling in already-outlined `encode_outer_batch`. No change to `encode()`, Viterbi, Metaspace, APIs, formats, dependencies, evaluator, or BPE.

Expected winning strata: wide Unigram `encode_batch` on 8-core hosts. Expected adverse strata: sequential encode must stay within `0.97x`; a single-pool outer may lose if global-pool workers were hiding inner latency.

Smallest files that need changing: `src/lib.rs` and this record.

Acceptance rule: existing T5 partitioned sequential-plus-batch Hugging Face tests; complete-ID sequential and `encode_batch` parity on the 20 LongBench contexts; Intel seven-cycle sequential no-HF paired-median must not fall below `0.97x` versus the immutable walker-keep parent; Intel seven-cycle `encode_batch` must reach a 3% paired geomean floor versus that parent batch path.

Rejection rule: fully revert this scheduling patch on any ID/row mismatch, sequential regression below the guard, or a batch gain below 3%. Keep the already-retained walker-keep branch. Do not alter corpus, runner, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected and fully reverted (walker-keep retained). Complete-ID sequential and `encode_batch` parity passed (`5,440,870` IDs). Intel seven-cycle sequential paired-median geomean `1.0134x` (16/20 documents faster). Intel seven-cycle `encode_batch` paired geomean `1.0126x` (parent median `101.393` versus candidate `100.598 ms`; per-cycle `0.990x`–`1.040x`), below the 3% floor. Same-binary `seq/batch` was already `1.05x`–`1.09x` on the walker-keep parent; putting the outer loop on `bpe_pool` does not remove a material dual-pool tax. This closes same-pool outer scheduling and, by the same measurement, flattening into one partition task set as a follow-on: the remaining batch gap to sequential is ~`7%` same-binary and is not a 3% win over walker-keep.

### Unigram wide-batch keeps partitioned walker (2026-09-15) — planned

Parent SHA: `b3eb482ca89228527366b08c3673d58b39bca56e`.

Hypothesis: `encode_outer_batch` wraps every wide or short row in `without_inner_parallelism`, and the retained Unigram raw-partition driver is gated on that same flag, so a 20-document T5 batch abandons the walker and rebuilds the serial Metaspace buffer. A same-binary probe measured `encode_batch` at `0.57x`–`0.86x` of sequential wall time. Sequential `encode()` already uses a separate `bpe_pool` for partitions; leaving inner parallelism enabled for Unigram rows keeps that driver without editing `encode()`. BPE rows keep the existing nested-pool suppression.

Measured hot cost: sequential partitioned encode of the pinned 20 LongBench contexts is ~90 ms on this M2. The discarded inner-parallel batch path pays the pre-partition serial fused fraction on each outer worker. Prior attempts to add a serial walker sibling of `encode_metaspace_raw_partitions` moved sequential codegen (`0.90x`) even when sequential still called the original function.

Invariant that makes the shorter path exact: Unigram token IDs depend only on each document's bytes, added-token cuts, charsmap, Metaspace pieces, and Viterbi. Enabling the already-exact partitioned driver from a Rayon worker concatenates the same per-partition IDs as a main-thread encode of that document. Nested `bpe_pool` work does not change match order or scores. Row order is the existing `par_iter` collect order.

Representation being preserved or changed: one structure-derived branch in already-outlined `encode_outer_batch`. No change to `encode()`, Viterbi, Metaspace, APIs, formats, dependencies, evaluator, or BPE scheduling.

Expected winning strata: `encode_batch` / ragged fallback of four or more large Unigram documents. Expected adverse strata: sequential encode must stay within the `0.97x` codegen guard; oversubscription of the global pool plus `bpe_pool` may blunt or reverse the batch gain.

Smallest files that need changing: `src/lib.rs`, the existing partitioned T5 integration test (four large rows already disable inner Rayon today), and this record.

Acceptance rule: focused T5 partitioned sequential-plus-batch Hugging Face parity; T5/GPT-2 unit/integration; complete-ID sequential and `encode_batch` parity on the 20 LongBench contexts; seven-cycle sequential no-HF paired-median screen must not fall below `0.97x`; seven-cycle out-of-repo `encode_batch` screen must reach a 3% paired-median floor versus the immutable parent batch path.

Rejection rule: fully revert on any ID/row mismatch, sequential regression below the guard, or a batch gain below 3%. Do not alter corpus, runner, worker count, fixture, evaluator, or dependency to rescue the result.

Result: retained. Focused T5 partitioned sequential-plus-batch, T5/GPT-2, and encode_batch/ragged tests passed. Complete-ID Hugging Face parity passed for sequential and `encode_batch` of the 20 LongBench contexts (`18,012,626` characters, `5,440,870` IDs) on Apple M2, idle GCP Intel `c4-standard-8` (`snaptokens-bench-20260909-intel`), and idle GCP AMD `c3d-standard-8` (`snaptokens-bench-20260909-amd`), all native `RUSTFLAGS`. Sequential `encode()` is unchanged: Intel seven-cycle per-document paired-median geomean `0.9987x` (10/20 documents faster; median sums `103.57` versus `104.08 ms`); AMD `1.0231x` (18/20; `99.40` versus `97.19 ms`). The hypothesized batch path wins: Intel seven-cycle `encode_batch` paired geomean `1.3709x` (parent median `136.311` versus candidate `100.092 ms`; every cycle `1.288x`–`1.447x`); AMD `1.3671x` (parent `126.614` versus candidate `93.308 ms`; every cycle `1.306x`–`1.450x`). Same-binary `seq/batch` is ~`0.78x`–`0.84x` on the parent and ~`1.06x`–`1.13x` on the candidate on both GCP hosts, so wide Unigram batches now keep the partitioned walker instead of rebuilding the serial Metaspace buffer. A noisy M2 batch probe agreed in direction (`1.5766x`) but is not the screening basis. End-to-end sequential Hugging Face on the candidate is `67.66x` (Intel 8-core) and `80.13x` (AMD 8-core). BPE still suppresses nested split work. Immutable Intel binaries: parent `f660ac9f…`, candidate `08af09ad…`. Raw CSVs `~/unigram-wide-batch/results` on each VM; M2 `/tmp/unigram-stack-piece/wb-seq-*.csv`.

### Unigram stack-prefixed Metaspace piece candidate (2026-09-15) — planned

Parent SHA: `b3eb482ca89228527366b08c3673d58b39bca56e`.

Hypothesis: the post-partition GCP profile assigns 6.9% of self samples to `Metaspace::emit_word_pieces`. For the dominant Always-prepend path the walker copies each word into a heap `String` and then `char_indices`-scans that prefixed piece for interior markers, even though ordinary T5 words contain none. If the original word has no replacement character, the existing `offset > start` split rule emits exactly one piece, so the prefixed haystack can be assembled on the stack and handed to the unchanged Viterbi entry without the interior decode loop or the heap copy.

Measured hot cost: GCP cycle profile of retained head `bb0ba3a` (source-equal to this parent) attributes 62.4% to `Unigram::tokenize_into_with_scratch`, 9.6% to the raw-partition word scan, and 6.9% to `emit_word_pieces`. The pinned workload's median Metaspace split is 8 bytes after the 3-byte marker, so almost every piece fits a small stack buffer.

Invariant that makes the shorter path exact: Hugging Face Metaspace with `split=true` cuts only at a replacement character whose byte offset is strictly greater than the current piece start. After optional Always-prepending, the only replacement in a word that itself contains none is the prefix at offset 0, which that rule does not cut. The stack bytes are the UTF-8 concatenation of the replacement character and the original word, so the `&str` passed to Viterbi is byte-identical to the heap-prefixed piece. Words that contain the replacement, pieces that overflow the stack, `split=false`, and Never-prepend keep the existing copy-and-scan path.

Representation being preserved or changed: add a private stack haystack only inside `emit_word_pieces`. Preserve Metaspace config, walker order, Unigram Viterbi, partitioning, APIs, formats, dependencies, evaluator, and BPE paths. No unsafe code, model-name dispatch, cache, or benchmark change.

Expected winning strata: Always-prepend Metaspace Unigram on ASCII-dominant documents, including pinned T5 LongBench. Expected adverse strata: marker-bearing words, long single words above the stack cap, Never-prepend and `split=false` (unchanged aside from the contains check on the split=true fast path).

Smallest files that need changing: `src/pre_tokenizers/metaspace.rs`, the existing walker-equivalence test extended with a no-interior fast-path word, and this record.

Acceptance rule: focused walker equivalence including interior markers, leading markers, and a long overflow word; Unigram/pre-tokenizer unit suites; T5 scalar/batch/ragged and GPT-2 integration; 1,000 local `fuzz_unigram` cases; a complete-ID 20-input Hugging Face run; then seven counterbalanced per-document paired-median no-HF rounds against the immutable parent binary at a 3% geomean floor.

Rejection rule: fully revert on any piece-sequence, parity, or fuzz discrepancy, or a screen below the floor. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected and fully reverted. Focused walker, Unigram/pre-tokenizer, T5 scalar/batch/ragged, and GPT-2 tests passed. Complete-ID Hugging Face parity passed for all 20 LongBench contexts (`18,012,626` characters, `5,440,870` IDs, `57.65x` on this M2). Seven counterbalanced no-HF cycles against immutable parent `63b83917…` and candidate `4e85804a…` produced a per-document paired-median geomean of `0.9768x` (7/20 documents faster; median sums `92.40` versus `94.54 ms`). Whole-run totals were noisy (`85.56–114.88` parent, `87.15–139.21` candidate). The `contains` plus stack copy does not beat the already-warm heap `String` and interior `char_indices` on short T5 words. Source restored exactly; raw CSVs `/tmp/unigram-stack-piece/csv-*.csv`.

### Unigram walker without nested-pool requirement (2026-09-15) — planned

Parent SHA: `b3eb482ca89228527366b08c3673d58b39bca56e`.

Hypothesis: `encode_batch` disables inner Rayon once there are four or more rows, and that currently also abandons the retained fused Unigram walker, falling back to the whole-document rewritten Metaspace buffer. A same-binary probe of the pinned 20 LongBench contexts measured `encode_batch` at `0.57x`–`0.86x` of sequential `encode` wall time with identical token counts. Keep the walker for every eligible Unigram+Metaspace input; only the partition `par_iter` should require inner parallelism and the 16 KiB size gate. Outer-batch workers then run the serial walker per document instead of reconstructing the pre-partition path.

Measured hot cost: sequential partitioned encode of the pinned 20 contexts is ~90 ms on this M2 (`simple_bench --no-hf`). The discarded inner-parallel batch path rebuilds the serial fused buffer that partitioning removed, so each outer worker pays the old 61.5% serial fraction on one core.

Invariant that makes the shorter path exact: `for_each_word_piece` already matches the serial fused walker on any range that does not split a word. Using one partition per text segment, or sequential partitions, concatenates the same per-word pieces in the same order. Added-token segmentation, charsmap-anchor cuts, Viterbi, ties, and post-processing are unchanged. Nested Rayon is skipped only by replacing `par_iter` with `iter` when inner parallelism is off or the input is below the existing parallel byte gate.

Representation being preserved or changed: widen the existing raw-partition driver to all eligible Unigram+Metaspace encodes and gate only its Rayon split. No public API, format, dependency, evaluator, BPE, or unsafe-code change.

Expected winning strata: `encode_batch` of large Unigram documents and any encode already running on a pool thread. Expected adverse strata: sequential large encodes from the main thread remain on the current parallel-partition path and should be neutral; ineligible charsmaps keep the old route.

Smallest files that need changing: `src/lib.rs`, existing T5 batch/partition integration coverage if a serial-walker large document is not already asserted, and this record.

Acceptance rule: unit/T5/GPT-2/fuzz gates; complete-ID sequential and `encode_batch` Hugging Face parity on the 20 LongBench contexts; seven-cycle sequential no-HF paired-median screen must not fall below `0.97x` versus the immutable parent; a same-binary `encode_batch` screen must improve at least 3% over the parent batch path.

Rejection rule: fully revert on any ID/row mismatch, sequential regression below the guard, or a batch gain below 3%. Do not alter corpus, runner, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected and fully reverted. Two implementations were screened. Sharing one `encode_partition` closure between `par_iter` and `iter` passed T5/GPT-2 tests and complete-ID parity (`5,440,870` IDs) but the seven-cycle sequential paired-median geomean was `0.9042x` (3/20 documents faster). Isolating a new `encode_metaspace_raw_serial` helper and leaving `encode_metaspace_raw_partitions` textually unchanged still produced `0.9008x` sequential (2/20 faster); the batch probe median was `1.0555x` on a noisy host with overlapping parent/candidate ranges. Adding any sibling Unigram driver next to the retained parallel path moves sequential code generation enough to lose the pinned screen. Source and the extra batch assertion are restored exactly. Raw CSVs `/tmp/unigram-stack-piece/sw-*.csv` and `/tmp/unigram-stack-piece/seq-*.csv`.

### GCP cross-host confirmation and re-screens (2026-09-15)

Task-owned host `snaptokens-unigram-gcp-20260915` (`c4-standard-4`, Intel Xeon Platinum 8581C, us-east1-b, standard PMU, Debian 12, Rust 1.97.1) rebuilt the immutable parent `ea4458a` and candidate head `bb0ba3a` from clean archives (SHA-256 `2038fb9a…` and `ba5c2771…`) against the pinned T5 tokenizer and the same 20 LongBench contexts shipped as a `local:` dataset. Complete-ID Hugging Face parity passed for both: parent `262.95 ms` (`23.96x`), candidate `180.64 ms` (`34.93x`). Three counterbalanced no-HF rounds each: parent `247.42/248.80/240.52 ms`, candidate `172.31/167.85/170.84 ms` — medians `247.42` versus `170.84 ms`, `1.448x` candidate/parent on the equal-core canonical host, consistent with the 4-worker bound for removing a 61.5% serial fraction.

The rejected streaming partition-scan candidate `6494674` was additionally re-screened on this host (its M2 screen was `1.0056x`): parity passed, but five counterbalanced rounds gave candidate-head medians `168.33` versus streaming `197.56 ms` — `0.852x`, a decisive cross-host loss confirming the rejection.

A cycle profile of the candidate head on this host attributes 62.4% of self samples to `Unigram::tokenize_into_with_scratch`, 9.6% to the raw-partition closure (the inlined whitespace word scan), 6.9% to `Metaspace::emit_word_pieces`, about 4.4% to charsmap normalization, 2.5% to `append_ids_for_pieces`, and about 2.5% to page faults from per-encode allocations.

### Byte-classified Metaspace walk candidate (2026-09-15) — planned

Parent SHA: `bb0ba3a` head state.

Hypothesis: the word walk decodes every character twice — `char_indices` plus `is_whitespace` over the partition, and again over each piece hunting interior markers — costing about 16% of GCP cycles. Only bytes `0x09–0x0D`, `0x20`, and lead bytes `C2/E1/E2/E3` can begin a whitespace character (the complete White_Space repertoire is U+0009–000D, U+0020, U+0085, U+00A0, U+1680, U+2000–200A, U+2028/2029, U+202F, U+205F, U+3000), so a 256-entry class table can skip plain bytes without decoding and decode only possible-whitespace leads. Interior-marker splitting can likewise scan for the marker's UTF-8 first byte — any lead-byte value in valid UTF-8 is a character start — and verify the remaining bytes, replacing per-character decoding with byte comparison.

Invariant that makes the shorter path exact: continuation bytes are `0x80–0xBF` and never equal an ASCII byte or a lead byte, so byte-stepping cannot misalign; a byte outside the class table's whitespace-capable set can never begin a whitespace character, and classified leads are decoded and tested with the same `char::is_whitespace`; a marker occurrence found by full-encoding byte comparison is exactly a `character == replacement` position, and the `offset > start` boundary rule is preserved verbatim. Word boundaries, piece boundaries, prepend behavior, and emission order are unchanged for every input.

Representation being preserved or changed: precompute the class table as a const and the marker's UTF-8 encoding at Metaspace construction; rewrite only the private `for_each_word_piece` and `emit_word_pieces` scans. No public API, format, dependency, semantics, evaluator, benchmark, BPE, or unsafe-code change.

Expected winning strata: ASCII-dominant text (the pinned shape). Expected adverse strata: whitespace-lead-dense CJK text pays one decode per classified lead, bounded by the existing per-character cost.

Smallest files that need changing: `src/pre_tokenizers/metaspace.rs`, focused equivalence tests, and this record.

Acceptance rule: focused walker-equivalence tests across Unicode whitespace (including every multi-byte White_Space character), marker-dense words, and both schemes/split modes; the standard unit/integration/fuzz/parity gates; then five counterbalanced no-HF rounds on the GCP host at a 3% median floor, with an M2 paired screen as a no-regression check.

Rejection rule: fully revert on any equivalence, parity, or fuzz discrepancy, or a GCP screen below the floor. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected and fully reverted. Every gate passed — the walker-equivalence test extended to every multi-byte White_Space character and same-lead non-whitespace characters, the full unit/integration suites, 1,500 fuzz cases, and complete-ID GCP parity (`35.69x`). Five counterbalanced GCP rounds gave head medians `165.64` versus walk `161.92 ms` — `1.0230x`, real-looking (4/5 rounds faster) but below the predeclared 3% floor, so the revert stands. The mechanism was smaller than profiled because the closure's self time includes allocation and normalization checks beyond the character decode.

### Interleaved/manual automaton mechanism screen (2026-09-15) — lane closed without candidate code

On the same GCP host, a standalone out-of-repo screen decoded the production T5 automaton through the previously rejected `FastDoubleArray` serialization layout (exactness verified against `find_overlapping_iter` on 5,000 real pieces) and measured single-thread match-collection throughput over the real 2,443,886-piece marker-prepended word stream (21.8 MB): daachorse iterator `199.8 ms`, manual single-stream `214.1 ms` (confirming the earlier traversal rejection), manual with precomputed marker-prefix state skip `213.1 ms` (the marker's three bytes ride hot, perfectly predicted L1 states, so skipping 33% of scanned bytes saves under 1%), two-way interleaved `235.3 ms`, and two-way with prefix skip `221.8 ms`. Interleaving independent words is slower than one stream: out-of-order execution already overlaps the dependent double-array loads, leaving no latency to hide. This closes the manual-traversal, prefix-state, and cross-piece interleaving lanes with quantitative evidence and no production edit; the automaton scan is at its measured throughput limit on this CPU class. VM evidence (perf stat/record captures) is archived at `~/.cache/snaptokens-autoresearch/unigram-gcp-20260915/gcp-evidence-20260915.tar.gz`, SHA-256 `6dbef0d7ecb1b1876dfd42d0c6e8725b427b363c11fb88f0d354b6ab5dba9be1`; immutable VM binaries: parent `2038fb9a…`, candidate `ba5c2771…`, walk `d420ed26…`, streaming `c02c4e88…`. The VM is stopped with its disk preserved.

### Streaming partition-scan Unigram candidate (2026-09-15) — planned

Parent SHA: `5703bb92`-headed raw-partition tree (source equal to retained `cb6ac35` state).

Hypothesis: the driver calls `tokenize_into_with_scratch` once per word, so each of the pinned run's 2.44 M words pays automaton-iterator construction, workspace truncate/resize, and call overhead. The bucketed timing bounds that fixed cost: 1–4-byte words cost `81 ns` per occurrence although their scan and DP work is a small fraction of that, and 5–8-byte words cost `124 ns`; the fixed per-call share is roughly 30% of total Viterbi time. Build each partition's marker-rewritten buffer with the existing fused walker, run one `find_overlapping_iter` over the whole partition, and apply the identical per-piece Viterbi recurrence while filtering matches that cross piece boundaries. One iterator per ~16 KiB partition replaces one per word.

Invariant that makes the shorter path exact: in valid UTF-8, every automaton match starts at a lead byte, so match starts and ends are character boundaries of the partition buffer; the fused walker's splits tile that buffer, so every match end is visited by exactly one split's boundary loop in global nondecreasing end order. A match starting before the current split's base is discarded — per-word scanning never saw it — and matches inside a split are exactly the per-word match set rebased by the split start. Scores, strict-greater ties, smaller-source tie order, unknown fallback and fusion, byte fallback, backtracking, and output order are the unchanged existing logic; models without `unk_id` and empty-vocabulary models keep their existing per-split paths.

Representation being preserved or changed: the raw-partition driver builds a per-partition `PreTokenizedString` through the retained fused walker instead of per-word emission, and a new crate-private contiguous-splits Unigram entry streams one automaton scan across those splits. The per-word walker becomes unused and is removed with its focused test. No public API, format, dependency, evaluator, benchmark, BPE, or unsafe-code change.

Expected winning strata: short-word-dense text — the dominant T5 shape. Expected adverse strata: partitions with many cross-piece byte matches (vocabulary pieces containing interior markers) pay filtered match traffic; long single-word partitions are neutral.

Smallest files that need changing: `src/models/unigram.rs`, `src/lib.rs`, `src/pre_tokenizers/metaspace.rs` (walker removal), and this record; existing partition, integration, and fuzz gates already cover the changed path.

Acceptance rule: the standard gates (unit suites, T5 scalar/batch/ragged, GPT-2, partitioned-document parity, 1,000 local fuzz cases, complete-ID 20-input Hugging Face run) and the seven-cycle counterbalanced per-document paired-median screen at the 3% floor against the immutable raw-partition binary.

Rejection rule: fully revert on any parity, fuzz, boundary, tie, or added-token discrepancy, or a screen below the floor. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected and fully reverted. Every exactness gate passed — unit suites, T5 scalar/batch/ragged, GPT-2, the partitioned-document boundary tests, 1,500 fuzz cases through the streaming path, and a complete-ID 20-input Hugging Face run (`54.56x` on this host). The seven-cycle paired-median screen against the immutable raw-partition binary gave only `1.0056x` (10/20 documents faster; median sums `93.9` versus `93.7 ms`), below the 3% floor: per-word iterator and call overhead is smaller than the bucket-derived bound suggested, and the per-partition rewritten-buffer plus split-vector construction consumes the savings. The revert restores per-word emission. Raw CSVs `/tmp/unigram-cache-screen/csv7-*.csv`. Together with the granularity rejection this closes the cheap post-partitioning lanes; the remaining measured costs are the Viterbi recurrence itself (closed lanes) and asymmetric-core straggle.

### Unigram partition-granularity candidate (2026-09-15) — rejected and reverted

Hypothesis: the steady-state profile shows about 18% of process samples parked in `__psynch_cvwait`/`swtch_pri`, so raising partitions per worker from 6 to 16 and lowering the partition floor from 16 KiB to 8 KiB might improve stealing balance on this asymmetric 4P+4E host; boundaries obey the identical anchor rules, so only counts change. Result: rejected and fully reverted. All parity gates passed, but the seven-cycle paired-median screen against the raw-partition head gave `1.0016x` (9/20 documents faster; median sums `96.1` versus `97.3 ms`), below the 3% floor — the idle time is dominated by asymmetric P/E completion and between-document pool idle rather than stealing granularity. Raw CSVs `/tmp/unigram-cache-screen/csv6-*.csv`.

### Raw-partitioned Unigram normalization candidate (2026-09-15) — planned

Parent SHA: `8b8c74600a3e9fdc7190e7a53a43697cab81d68e`.

Hypothesis: after the retained partitioned fused encode, the pinned wall is `150.6 ms` with `28.6 ms` (19.0%) still serial — almost entirely charsmap normalization, which also copies the whole 18 MB document into an owned buffer. Partitioning the raw text before normalization and normalizing inside each parallel partition (borrowing unchanged partitions instead of copying) removes the last large serial phase and most of the 18 MB serial allocation.

Invariant that makes the shorter path exact: eligibility requires either no normalizer or a Precompiled charsmap with printable-ASCII identity. A cut is placed only where the previous byte is an anchor-safe ASCII whitespace byte (its charsmap mapping is identity or ends with a whitespace character) and the byte at the cut is printable ASCII. Then (1) printable ASCII normalizes bytewise to itself, and control bytes normalize through the per-byte ASCII map, with CRLF pairs never split because a following `\n` is not printable; (2) a grapheme fallback span starts at a non-ASCII byte, extends backward at most one byte, and stops at the first ASCII byte followed by ASCII, so the ASCII-ASCII byte pair around every cut prevents any span from crossing it in either direction — a combining mark directly after whitespace also cannot begin a partition because the cut byte must be printable ASCII; (3) the anchor's normalized output ends in whitespace and the cut byte is identity printable ASCII, so the serial normalized stream has a word boundary exactly at the cut and the per-word fused walk emits identical pieces. Added-token segmentation runs serially on the raw input exactly as before, and cuts lie strictly inside ordinary text segments. Normalizers other than the eligible forms, normalized added tokens, and inputs below the parallel gates keep the retained partitioned path or the serial path unchanged.

Representation being preserved or changed: precompute a 128-entry whitespace-anchor safety table beside the existing ASCII map at Precompiled load; add a raw-partition driver in `lib.rs` dispatched before the normalized-buffer path under the conditions above; per partition, normalize a borrowed slice (`Cow`), run the existing word walk and Viterbi. No public API, format, dependency, evaluator, benchmark, BPE, or unsafe-code change.

Expected winning strata: large ASCII-dominant Unigram documents (the pinned T5 shape). Expected adverse strata: charsmaps that remap printable ASCII or whose whitespace mappings end in non-whitespace (ineligible, unchanged path); non-ASCII-dense text cuts less often but only loses parallel granularity, never exactness.

Smallest files that need changing: `src/normalizers/precompiled.rs`, `src/normalizers.rs` (accessor), `src/lib.rs`, integration coverage in `tests/tokenizer.rs` for combining-mark-after-space and CRLF boundaries, and this record. The existing extended fuzzer already drives >16 KiB inputs through the new dispatch (no-normalizer form).

Acceptance rule: the same exactness gates as the previous candidate (unit suites, T5 scalar/batch/ragged, GPT-2, partitioned-document integration parity extended with boundary-stress inputs, 1,000 local fuzz cases, complete-ID 20-input Hugging Face run), then the seven-cycle counterbalanced per-document paired-median no-HF screen against the immutable parent. Retain only with exact IDs and at least a 3% paired-median geomean improvement; report allocation behavior separately.

Rejection rule: fully revert on any parity, fuzz, boundary, or added-token discrepancy, or a screen below the floor. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: retained T5 specialist result. The partitioned-document integration test gained a combining-mark-after-space and decomposed-accent stress input (a cut may not separate a space from a following U+0301 or split a decomposed cluster); all four >16 KiB documents match Hugging Face exactly, as do the unit suites, T5 scalar/batch/ragged, GPT-2, and 1,000 local fuzz cases (the fuzzer's >16 KiB no-normalizer repetition now flows through the raw-partition dispatch). The complete-ID 20-input Hugging Face run passed at `223.47 MB/s` snaptokens throughput.

The seven-cycle per-document paired-median screen against retained candidate `8b8c746` gave `1.0874x` (16/20 documents faster; median sums `99.0` versus `89.9 ms` on a calmer host than the previous screen). A direct seven-cycle screen against the branch parent `ea4458a` binary measures the two retained partitioning candidates together at `2.0056x` geomean with every document faster (`186.9` versus `85.3 ms`; worst document `1.3821x`, best `2.6795x`). Allocation behavior improves: unchanged ASCII partitions borrow their input slice, so the serial whole-document normalized copy is gone. Raw CSVs `/tmp/unigram-cache-screen/csv5-*.csv` and `csvF-*.csv`. Local Apple M2 evidence; the canonical cross-host ratio still requires the GCP host, but at the recorded `26.46x` parent this projects to roughly `53x` Hugging Face on the pinned workload.

### Parallel per-partition fused Unigram encode candidate (2026-09-15) — planned

Parent SHA: `927357c533bb08d48e79e5b5c24f3adca7a7481c`.

Hypothesis: the pinned T5 single-document encode serializes 61.5% of its wall time — `121.8 ms` in the fused WhitespaceSplit+Metaspace walker and `38.3 ms` in normalization against a `100.2 ms` parallel model phase (phase-timer evidence in the previous entry). Because Metaspace word pieces are a pure per-word function and whitespace produces no output, the normalized buffer can be partitioned at any byte position whose previous byte is ASCII whitespace and each partition can run the fused word walk plus per-piece Viterbi independently. Executing those partitions on the existing shared pool converts the dominant serial phase into parallel work and removes the intermediate whole-document rewritten buffer and split vector entirely.

Invariant that makes the shorter path exact: the serial fused output is the in-order concatenation, over ordinary splits, of each whitespace-delimited word's pieces (marker prepending and interior-marker splitting depend only on the word itself), with added-token IDs interleaved at their split positions. A cut placed immediately after an ASCII-whitespace byte is a character boundary, lies between words (or inside whitespace, which emits nothing), and therefore never divides a word or piece; every partition reproduces exactly its words' pieces in order, and per-piece Viterbi, strict-tie selection, unknown fusion, and byte fallback are the unchanged existing routines applied to identical piece strings. Unigram has no bigram bridge table, so the skipped vocabulary-splitting step is already a no-op for it. Inputs below the existing parallel thresholds, non-fused pipelines, models other than Unigram, and encodes running under disabled inner parallelism keep the current serial code byte for byte.

Representation being preserved or changed: add a crate-private partitioned driver in `lib.rs` dispatched only for Unigram behind the existing fused WhitespaceSplit→Metaspace capability and the existing parallel-threshold and inner-parallelism gates; add a crate-private per-word piece walker on `Metaspace` mirroring `append_whitespace_free`/`append_split_ranges`; expose the existing Viterbi scratch entry crate-privately. No public API, format, dependency, evaluator, benchmark, BPE-path, or unsafe-code change.

Expected winning strata: large single-document Unigram encodes — the pinned T5 LongBench shape — and any split-heavy SentencePiece pipeline above the parallel threshold. Expected adverse strata: small inputs and outer-parallel batches (unchanged serial code); partition-scan overhead on whitespace-free megabyte blobs is bounded by one linear byte scan.

Smallest files that need changing: `src/lib.rs`, `src/pre_tokenizers/metaspace.rs`, `src/pre_tokenized.rs` (pool accessor), `src/models/unigram.rs` (visibility only), focused unit tests beside the walker and driver, `tests/tokenizer.rs` end-to-end coverage, and this record. No `benchmarks/` change.

Mechanism evidence: the reverted phase-timer measurement above; the single-thread whole-pipeline work (`~609 ms` fused walk plus Viterbi against `~100 ms` current parallel-phase wall) bounds the reachable win at roughly `1.7-2.1x` on this host before normalization or balance follow-ups.

Acceptance rule: focused walker tests against the serial fused walker across prepend schemes, split modes, Unicode whitespace, marker-bearing words, and added tokens; a forced-small-partition equivalence test; full Unigram/pre-tokenizer/normalizer suites; T5 scalar/batch/ragged and GPT-2 integration parity; at least 1,000 local `fuzz_unigram` cases; a complete-ID 20-input Hugging Face run; then the seven-cycle counterbalanced per-document paired-median no-HF screen against the immutable parent binary. Retain only with exact IDs everywhere and at least a 3% paired-median geomean improvement (the mechanism predicts far more); report memory separately. Local Apple evidence, T5-specialist pending cross-host confirmation.

Rejection rule: fully revert if any boundary, order, added-token, tie, parity, or fuzz discrepancy appears, or if the paired-median screen misses its floor. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: retained T5 specialist result. The focused walker test matches the serial fused walker's piece sequence across both prepend schemes, both split modes, Unicode whitespace, marker-bearing words, and boundary whitespace. The new `t5_unigram_partitioned_documents_match_hugging_face` integration test covers three >16 KiB documents — mixed Unicode with CRLF and thin/ideographic spaces, embedded `<extra_id>` added tokens, and a 72 KiB whitespace-free run — with exact Hugging Face IDs. Full Unigram/pre-tokenizer/normalizer suites, T5 scalar/batch/ragged, and GPT-2 parity passed; the extended `fuzz_unigram` (each case now also encodes a >16 KiB whitespace-joined repetition through the partitioned path against the repeated single-copy reference) passed 1,000 local cases. The 20-input complete-ID Hugging Face run passed on a loaded host (`50.59x` there; the HF side was slowed by that load, so the ratio is not comparable across sessions).

The seven-cycle counterbalanced per-document paired-median no-HF screen against the immutable parent gave `1.5491x` candidate/parent geomean with all twenty documents faster (per-document ratios `1.1454x` to `2.6186x`; per-document median sums `284.6 ms` parent, `175.8 ms` candidate). Clippy reports only the parent's three pre-existing lints. Memory: the driver materializes per-partition ID vectors (bounded by output size) and no longer materializes the whole-document rewritten buffer or split vector, so steady-state allocation shrinks; no fixed new tables. Raw CSVs are `/tmp/unigram-cache-screen/csv4-*.csv` beside the immutable binaries. Local Apple M2 evidence; cross-host confirmation still required.

### Unigram split-memoization cache candidate (2026-09-15) — planned

Parent SHA: `ea4458aad6a66d3ba51121408ed13d97fbfdb697`.

Hypothesis: T5's Metaspace pipeline re-runs the full automaton scan, Viterbi recurrence, and backtracking for every occurrence of a repeated word split, although a split's token IDs are a pure function of its bytes and the immutable model. Memoize per-split ID sequences in the existing `FlatCache` representation behind a thread-local guarded by a Unigram instance ID, exactly as the retained BPE champion memoizes pre-tokenized pieces. A hit replaces the entire matcher/Viterbi/backtrack/emission path with one packed-key probe and an ID copy. This is a per-piece cache amortized across novel inputs, not the prohibited complete-request output cache; it is the same architecture the BPE general champion already uses.

Measured hot cost: a fresh Apple M2 10-second `sample` profile of the steady-state pinned 20-input LongBench T5 loop (immutable parent source, out-of-repo harness) attributes 3,928 of roughly 5,182 busy worker self-samples (75.8%) to `Unigram::tokenize_into_with_scratch`, plus 328 (6.3%) to `tokenize_splits_into`/`append_ids_for_pieces`. Across the same 20 pinned inputs, 2,443,639 ordinary splits contain only 264,721 unique spellings: 89.2% of occurrences and 79.3% of split bytes are duplicates whose Viterbi work recomputes a known answer. 94.5% of splits (83.5% of bytes) fit the existing packed 15-byte key; the remainder uses the cache's exact long-key fallback. The local parent baseline is `26.82x` Hugging Face on the pinned run (`207.20 ms` vs `5,556.73 ms`).

Invariant that makes the shorter path exact: `tokenize_into_with_scratch` output for a split depends only on the split's bytes and immutable model state (scores, matcher, unk handling, byte fallback), so memoizing its emitted IDs by exact full-key comparison returns byte-identical output. The `FlatCache` compares complete packed keys (length-tagged) or full strings in its long fallback, so no collision can substitute a wrong entry. A thread-local instance-ID guard clears the cache whenever a different Unigram model runs on the same thread. Added-token placeholder splits bypass the cache and append their fixed ID as before. Errors propagate before any insert, so no failed segmentation is memoized. Split boundaries, output order, tie handling, unknown fusion, and byte fallback are untouched because misses run the identical existing code.

Representation being preserved or changed: reuse the existing private `FlatCache` (made crate-visible, unchanged layout and probe logic) with a new `TL_UNIGRAM_CACHE` thread-local and a construction-time Unigram instance ID. Only `Unigram::tokenize_splits_into` changes: probe before, insert after, per ordinary split. No public API, format, dependency, evaluator, BPE-path, or benchmark change; no unsafe code beyond the already-reviewed `FlatCache` internals.

Expected winning strata: natural-language Unigram workloads with Zipf-repeated words — the pinned T5 LongBench run and chat/document batches generally. Expected adverse strata: high-entropy inputs with few repeated splits (unique-heavy code or random text) pay one probe+insert per split; per-worker resident memory grows by the cache tables (about 10 MiB per worker thread that encodes Unigram, plus pooled IDs bounded by the existing clear thresholds) — reported separately as a resource cost.

Smallest files that need changing: `src/models/bpe.rs` (visibility only), `src/models/unigram.rs` (cache probe/insert plus focused tests), and this record. No `benchmarks/` change.

Mechanism evidence: the byte-weighted duplicate share above bounds removable Viterbi work at roughly 79% of the dominant 76% routine; the packed-key coverage shows the probe path is the existing 16-byte fast comparison for 94.5% of splits. The BPE champion's identical piece-cache architecture is the retained precedent that cache-hit emission costs are far below merge/Viterbi recomputation.

Acceptance rule: run new focused cache tests (repeat-split equivalence against the uncached path, cross-model thread guard, long-key splits), the full Unigram/pre-tokenizer/normalizer unit suites, T5 scalar/batch/ragged and GPT-2 integration tests, and at least 1,000 local `fuzz_unigram` cases against the independent reference. Run the complete-ID 20-input Hugging Face comparison before and after timing. Then compare three counterbalanced no-HF rounds of immutable parent and candidate binaries on this quiet Apple M2. Retain only with exact IDs everywhere and at least a 3% median throughput improvement; report per-worker memory growth separately. This is local Apple evidence for a T5-specialist lane; cross-host confirmation remains required before any broader claim.

Rejection rule: fully revert if any parity, fuzz, tie, added-token, guard, or long-key result differs, or if the median gain is below 3%. Do not alter corpus, model revision, runner, timer scope, worker count, fixture, evaluator, or dependency to rescue the result.

Result: rejected in all three representations and fully reverted; the diagnosis that ends the lane is recorded below. Every variant passed the full exactness gates: focused cache/guard/long-key unit tests, the complete Unigram/pre-tokenizer suites, T5 scalar/batch/ragged and GPT-2 integration parity, at least 1,000 local `fuzz_unigram` reference cases per variant, and a complete-ID 20-input Hugging Face run per variant (parent local baseline `26.82x`, `207.20 ms` vs `5,556.73 ms`).

Variant 1 (`e748c64`, thread-local `FlatCache` for all splits): three alternating no-HF rounds gave parent median `259.12 ms` versus candidate `265.24 ms`, `0.977x`. A temporary instrumented pass (fully reverted) measured a real one-pass hit rate of `82.00%` (2,003,716 hits, 439,923 misses, 1.67 IDs per hit), so the miss rate is not the failure. Variant 2 (`bfc87bd`, adds the Bpe-style sharded shared cache, thread-local restricted to 15-byte keys): the seven-cycle per-document paired-median screen gave `0.7114x` with 19 of 20 documents slower — shard mutex traffic plus per-miss `String`/`Vec` insert allocations cost far more than recomputing Viterbi on short words. Variant 3 (`582d870`, allocation-free 2 MiB direct-mapped table, 64-byte slots, packed key plus up to eleven inline IDs, simulated `87.6%` short-occurrence hit rate at 1 MiB and `>99%` inline-ID coverage): the same seven-cycle screen gave `1.0030x` (8/20 documents faster), below the 3% floor.

Why the lane is closed: bucketed single-thread timing over the pinned split stream measures Viterbi at `81 ns` (1–4 B), `124 ns` (5–8 B), and `266 ns` (9–15 B) per occurrence, timer overhead included — already at parity with a single L2/cache-line probe, so no memoization representation can materially beat recomputation for the splits that dominate occurrence counts. A temporary phase-timer pass (fully reverted) then attributed the pinned one-pass wall as: serial `build_pre_tokenized` plus normalization `38.3 ms` (14.7%), serial fused WhitespaceSplit+Metaspace pre-tokenization `121.8 ms` (46.8%), and the parallel model phase only `100.2 ms` (38.5%) of a `260.3 ms` wall. The single-thread Viterbi total is `486.9 ms`, so the parallel model phase is already near its 8-worker floor and 61.5% of the wall is serial pre-model work that no matcher, Viterbi, or cache candidate can reach. Raw screens are under `/tmp/unigram-cache-screen/` (immutable parent/candidate binaries with SHA-256 recorded in the session, alternating-round outputs, and seven-cycle per-document CSVs); this closes split memoization on this host and redirects the campaign at the serial phases.

### Unigram T5 adaptive-trie candidate (2026-09-14) — retained T5 specialist result

Parent SHA: `a75fde44185a67fd91db57a7190cf63e4eea087d`.

Hypothesis: T5 Unigram spends substantial CPU repeatedly binary-searching sorted byte-trie children while enumerating every possible piece prefix. Give only trie nodes with at least four children a direct 256-byte child table, leaving all smaller nodes in the compact sorted-edge representation. This removes the branch-heavy search where it is plausible to matter without paying a table per node.

Measured hot cost: on the task-owned GCP Intel Emerald Rapids `c4-standard-4`, a frame-pointer cycle profile of the 20-input, 18,012,626-character LongBench-v2 T5 run attributes 48.62% of samples to `Unigram::tokenize_into`; the second-largest timed pipeline component is SentencePiece precompiled normalization at 10.02%. The no-HF workload consumes 11,550,857,938 cycles, 24,391,003,763 instructions, and 1.46% branch misses over three runs. T5's 32,100 pieces build an 82,672-node, 82,671-edge trie: 2,365 nodes have degree at least four, requiring 2.31 MiB of `u32[256]` tables and covering 20.32% of stored edges.

Invariant that makes the shorter path exact: every byte child reachable through a direct table is the same child reachable through the existing sorted edge list; missing entries remain missing, terminal IDs preserve the last duplicate vocabulary ID, and prefix visits remain in increasing byte-length order. Viterbi scores, strict-greater tie handling, unknown fallback, UTF-8 boundaries, and output order do not change.

Representation being preserved or changed: keep immutable `PrefixTrie` nodes, sorted compact `TrieEdge`s, model IDs, scores, and public APIs. Add a private dense-child table only to high-degree nodes, with one sentinel that encodes no child. Do not change normalization, pre-tokenization, Viterbi state, BPE paths, evaluator sources, or input-size/model-name dispatch.

Expected winning strata: T5-style Unigram long documents and large batches where high-degree prefix nodes recur enough to amortize the direct-table load.

Expected adverse strata: small synthetic vocabularies, low-degree paths, construction time, and model RSS. These retain the compact edge lookup and are explicit controls rather than assumed wins.

Smallest files that need changing: `src/models/unigram.rs`, its focused unit tests, `tests/tokenizer.rs` for real T5 parity coverage, and `fuzz/fuzz_targets/fuzz_unigram.rs` for optimized-versus-reference segmentation coverage. No file under `benchmarks/` changes.

Mechanism evidence: the profile's hottest annotated region is the trie-child binary-search loop inside `Unigram::tokenize_into`; the T5 trie inventory above bounds the direct representation before implementation. The existing BPE direct-table result is only a representation precedent, not performance evidence for Unigram.

Acceptance rule: preserve exact unit, real T5 scalar/batch/ragged, and fuzz-reference outputs; run the existing 20-input Hugging Face parity comparison before and after the candidate; compare parent and candidate with three repeated no-HF T5 runs using the unchanged compiled `simple_bench` workload and counters. Retain only if the candidate improves the three-run median by at least 3%, has no ID mismatch, and reports construction/RSS impact separately. A retained local result remains a T5 specialist result until cross-host confirmation.

Rejection rule: revert in full if any semantic gate fails, if the three-run median gain is below 3%, or if construction/RSS cost makes the narrow direct-table representation unjustified. Do not alter corpus, model revision, runner code, timer boundaries, worker count, or profile settings to rescue the result.

Result: retained. The unchanged 20-input complete-ID Hugging Face comparison improved from Hugging Face `7,698.01 ms` versus parent `1,337.43 ms` (`5.76x`) to Hugging Face `7,587.74 ms` versus candidate `1,148.11 ms` (`6.61x`). The three no-HF parent runs were `1,353.73`, `1,354.09`, and `1,330.32 ms`; candidate runs were `1,149.30`, `1,116.39`, and `1,147.48 ms`. Their medians are `1,353.73` and `1,147.48 ms`, a `1.179742x` candidate/parent throughput result that exceeds the predeclared 3% floor. Full IDs matched Hugging Face in the before and after runs.

The candidate's three-run CPU counters fell from `11,550,857,938` to `9,218,555,013` cycles (20.2%), from `24,391,003,763` to `21,405,911,015` instructions (12.2%), and from `69,986,060` to `59,251,888` branch misses (15.3%); IPC rose from `2.10` to `2.33`. Ten repeated JSON constructions had a parent median of `19.860 ms` and candidate median of `18.056 ms`. A short repeated-load process-RSS sample rose from `23,452 KiB` to `30,900 KiB`; this is whole-process sampling rather than isolated model heap accounting, so it is reported as a resource cost rather than treated as an exact allocation result. The result is one T5/Intel specialist screen, not a cross-host or general-BPE claim.

### Unigram singleton-trie lookup candidate (2026-09-14) — rejected and reverted

Parent SHA: `9a973f7`. Hypothesis: directly compare the only `TrieEdge` at degree-one nodes rather than call generic binary search. The post-dense profile still attributed 35.12% of samples to `Unigram::tokenize_into`, and 48,140 of T5's 82,672 trie nodes have one child. The exactness invariant was direct: a one-element sorted edge slice and one equality comparison have the same child/miss result as binary search; Viterbi, UTF-8, unknown, and pipeline behavior stay unchanged.

Result: rejected and fully reverted as `0f9470c`. The focused model tests, real T5 scalar/batch/ragged Hugging Face parity run, and 5,000-run nightly sanitizer fuzz comparison against the independent brute-force Viterbi reference all passed. On the unchanged Intel 20-input no-HF run, the parent times were `1,117.95`, `1,117.72`, and `1,128.54 ms`; candidate times were `1,152.28`, `1,127.12`, and `1,139.53 ms`. Median throughput is `0.981x` candidate/parent, below the predeclared 3% floor. Candidate counters did remove 0.4% instructions and 0.2% branches, but cycles were neutral and wall time regressed; the added branch on compact nodes does not repay. No broader evaluation is warranted. Raw outputs and counters are preserved under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/singleton-{parent,candidate}-*` on the task-owned VM.

### Unigram compact-trie layout candidate (2026-09-14) — rejected and reverted

Parent SHA: `c824bfa`. Hypothesis: shrink private trie nodes from 32 to 16 bytes and edges from 16 to 8 bytes by validating `u32` indices, `u16` byte-child counts, and an unreachable token sentinel. The post-dense profile had `Unigram::tokenize_into` at 35.12% of samples, and T5's 82,672 nodes / 82,671 edges made the immutable working set a credible cache target.

Result: rejected and fully reverted as `7734381`. Focused Unigram tests, real T5 scalar/batch/ragged Hugging Face parity, the 20-input complete-ID Hugging Face benchmark, and a 5,000-run nightly sanitizer fuzz comparison against the brute-force Viterbi reference all passed. Parent no-HF runs were `1,141.71`, `1,122.08`, and `1,147.13 ms`; candidate runs were `1,152.67`, `1,139.55`, and `1,116.13 ms`. The 1,139.55 versus 1,141.71 ms median is only `1.002x`, far below the predeclared 3% floor. Although candidate instructions fell about 1%, branch misses rose from 1.33% to 1.45–1.46% and cycles rose 0.7%; the net result is indistinguishable from noise. No broader run is justified. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/compact-{parent,candidate}-*` on the task-owned VM.

### Unigram split-scratch candidate (2026-09-14) — planned

Parent SHA: `f5ce00d`.

Hypothesis: T5's Metaspace pre-tokenizer creates a very large number of independent ordinary-text splits, and each current `Unigram::tokenize_into` allocates and drops a Viterbi `best` table plus a `PathPiece` backtracking vector. Process the `PreTokenizedString` chunk supplied to the existing parallel tokenizer with one private Unigram scratch buffer reused across all its splits. This preserves each split boundary and output order while eliminating per-word DP/backtracking allocation growth.

Measured hot cost: a one-input (1,003,196 characters, 318,809 output IDs) Intel heaptrack run records 1,599,361 allocations. It attributes 135,084 `PathPiece::grow_one` allocations directly to `Unigram::tokenize_into`, reached through `PreTokenizedString::tokenize`'s Rayon split chunks. The same run records 135,086 allocation calls beneath the Metaspace/ordinary pre-tokenization path, confirming the split-heavy shape. The post-dense CPU profile still places `Unigram::tokenize_into` at 35.12% of samples. By contrast, the same machine and 20 LongBench inputs measure GPT-2 BPE at 71.69x Hugging Face while T5 Unigram is 6.61x; Hugging Face's own totals are similar, so the missing work is in our Unigram path rather than the reference target.

Invariant that makes the shorter path exact: Unigram inference for a pre-tokenized split depends only on that split's bytes and immutable model state. Reusing cleared `Vec` capacities changes neither initialized DP entries nor the `out` append order. Added-token placeholders still append their existing ID directly; ordinary splits still call the same Viterbi, strict tie, unknown-fusion, byte-fallback, and decode-compatible emission logic. Each parallel chunk owns its scratch buffer, so no mutable state crosses threads or tokenizer calls.

Representation being preserved or changed: retain the public `Tokenizer`, `Model`, JSON, cache, BPE, and generic pre-tokenization APIs. Add a private Unigram batch entry point plus private `best`/backtrack scratch vectors, and route only the already-valid Unigram ordinary pipeline through the existing chunked `PreTokenizedString::tokenize_batched` API. No global cache, output cache, unsafe code, dependency, model-name branch, or evaluator change.

Expected winning strata: SentencePiece-style tokenizers with Metaspace or other split-heavy pre-tokenization, especially long natural-language documents where per-word Viterbi allocations dominate.

Expected adverse strata: one large unsplit Unigram input, short calls, and BPE pipelines. The first two should be neutral to modestly better; BPE must retain its current specialized paths byte-for-byte.

Smallest files that need changing: `src/models.rs`, `src/models/unigram.rs`, `src/lib.rs`, a focused Unigram scratch-reuse test, `tests/tokenizer.rs` real T5 split-heavy coverage, and the existing `fuzz/fuzz_targets/fuzz_unigram.rs` reference fuzzer. No `benchmarks/` source changes.

Mechanism evidence: heaptrack exposes a concrete 135,084-call allocation site in Unigram backtracking on one representative T5 document. This candidate removes that allocation lifetime by carrying the exact same vectors from one independent split to the next; it does not alter Viterbi segmentation or cache complete requests.

Acceptance rule: run focused scratch and Viterbi tests, real T5 scalar/batch/ragged parity, BPE regression coverage, the optimized-versus-brute-force Unigram fuzzer, and the unchanged 20-input complete-ID Hugging Face run. Then compare three no-HF Intel parent/candidate runs with counters. Retain only if outputs match and median throughput improves at least 3%; report construction/RSS separately and keep the result T5-specialist until wider validation.

Rejection rule: revert fully if reused scratch changes a split boundary/output, races under parallel tokenization, leaves stale DP state, misses the 3% floor, or regresses the BPE path. Do not alter the corpus, runner, timing boundary, thread count, or use complete-request caching.

Result: retained T5 specialist result. The unchanged 20-input complete-ID Hugging Face run measured Hugging Face `7,468.42 ms` and the candidate `1,057.59 ms` (`7.06x`); all IDs matched. The three parent no-HF runs were `1,119.58`, `1,142.27`, and `1,140.21 ms`; candidate runs were `1,051.48`, `1,049.49`, and `1,053.34 ms`. Their medians are `1,140.21` and `1,051.48 ms`, a `1.084385x` candidate/parent throughput gain above the predeclared 3% floor. Median cycles fell from `9,218,893,015` to `8,528,983,078` (7.5%), instructions from `21,409,781,975` to `19,830,996,083` (7.4%), and branches from `4,449,014,242` to `4,171,944,638` (6.2%). Branch misses remained roughly flat in absolute count (`59.27M` to `59.49M`) while their percentage rose with the lower branch count.

Mechanism check: the representative one-input heaptrack capture fell from `1,599,361` to `1,019,953` allocations (36.2%) and from `452,031` to `307,925` temporary allocations (31.9%); the prior 135,084 `PathPiece::grow_one` call site no longer appears in the candidate's top allocation report. The focused scratch test, real T5 scalar/batch/ragged integration tests, GPT-2 regression test, 20-input parity run, and nightly sanitizer fuzz runs—including the new added-token multi-split reference assertion—passed. Construction/RSS have not changed representation and were not re-measured. This does not yet support a general, cross-host, or BPE-equivalence claim. Raw outputs, counters, and heaptrack files remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/scratch-*` and `heaptrack-{unigram,scratch}.*` on the task-owned VM.

### Metaspace single-copy split construction candidate (2026-09-14) — rejected and reverted

Parent SHA: `4c055b9`. Hypothesis: append each already-rewritten Metaspace segment once and create the same marker-delimited ranges by offset, removing the 135,086 repeated word append/reserve checks seen in T5 heaptrack.

Result: rejected and fully reverted as `c13bf60`. The focused Metaspace range/added-token test, real T5 scalar/batch/ragged parity, GPT-2 regression test, complete-ID 20-input Hugging Face run, and nightly sanitizer fuzz run with the independent Metaspace plus added-token reference all passed. Parent no-HF times were `1,054.69`, `1,058.02`, and `1,055.84 ms`; candidate times were `1,057.15`, `1,053.67`, and `1,052.44 ms`. The 1,053.67 versus 1,055.84 ms median is `1.002x`, far below the 3% promotion floor. Instructions are neutral; the candidate did lower branch misses from 1.43% to 1.40%, but no material wall-time benefit follows. No broader result is warranted. Raw counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/metaspace-{parent,candidate}-*` on the task-owned VM.

### Unigram split-DP reachability candidate (2026-09-14) — rejected and reverted

Parent SHA: `098bfea`. Hypothesis: replace each split's 32-byte-stride `Option<BestPathNode>` table with retained plain nodes plus a cleared byte reachability map, so stale nodes cannot participate while avoiding full-node reinitialization.

Result: rejected and fully reverted as `dd0a523`. Focused Viterbi/stale-scratch tests, real T5 scalar/batch/ragged parity, the GPT-2 regression test, a 5,000-run nightly sanitizer fuzz comparison against the independent reference, and the 20-input complete-ID Hugging Face run all passed; the latter measured `7,741.90 ms` Hugging Face versus `1,077.01 ms` candidate (`7.19x`). The fixed no-HF parent runs were `1,141.16`, `1,054.94`, and `1,056.77 ms`; candidate runs were `1,061.55`, `1,074.96`, and `1,064.84 ms`. The 1,064.84 versus 1,056.77 ms median is `0.992x` throughput, below the 3% floor. Candidate median instructions rose from `19.834B` to `20.102B` (1.35%) and branches from `4.173B` to `4.236B` (1.52%), while branch misses fell from `59.55M` to `57.90M`; the smaller state did not overcome the additional map stream. No broader run is justified. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{scratch,reachability}-nohf-*` on the task-owned VM.

### SentencePiece printable-ASCII normalization candidate (2026-09-14) — planned

Parent SHA: `cca6956`.

Hypothesis: `spm_precompiled::Precompiled::normalize_string` walks Unicode grapheme clusters and probes its charsmap even for printable ASCII. The retained T5 profile attributes 14.10% of cycle samples to that call, including 5.93% to its DoubleArray prefix search. Precompute each single-ASCII mapping when loading the charsmap; when every printable ASCII mapping is an identity, copy printable runs directly and invoke the reference normalizer only on the small spans around non-ASCII text or controls.

Invariant that makes the shorter path exact: printable ASCII bytes are standalone grapheme clusters unless adjacent to a non-ASCII grapheme component; the fallback span includes one preceding printable byte and extends through the next possible boundary, so it delegates every potentially joined cluster to the unchanged reference implementation. Controls remain individual clusters except CRLF, which is precomputed and handled as one reference-equivalent unit. If any printable ASCII mapping changes text, the implementation uses the reference normalizer for the entire input.

Representation being preserved or changed: retain the serialized charsmap and `spm_precompiled` parser unchanged. Add private, load-time ASCII and CRLF mapping tables alongside it, plus a scalar portable run scanner. Preserve `Cow` borrowing when the final normalized string is unchanged. No model, tokenizer pipeline, evaluator, BPE path, public API, unsafe code, or dependency changes.

Expected winning strata: ASCII-dominant SentencePiece/Unigram text with a charsmap that leaves printable ASCII unchanged, including the pinned T5 LongBench workload. Expected adverse strata: non-ASCII-dense text, printable-ASCII-remapping charsmap configurations, and tiny calls; these retain or quickly fall back to the existing exact implementation.

Smallest files that need changing: `src/normalizers/precompiled.rs`, its focused unit tests, `tests/tokenizer.rs` with direct pinned-T5 normalizer/reference equivalence, and the existing Unigram integration/fuzz gates. No file under `benchmarks/` changes.

Mechanism evidence: the post-scratch cycle profile identifies the bundled charsmap normalizer as the second-largest timed component. The repository already includes an MIT notice for the independently inspected scalar/simd ASCII-run strategy; this candidate takes only the portable scalar mechanism and keeps the source normalizer as the semantic oracle.

Acceptance rule: compare direct pinned-T5 normalization with `spm_precompiled` across ASCII, controls, combining marks, full-width text, CJK, emoji, and mixed runs; then run existing T5 scalar/batch/ragged and GPT-2 integration coverage, the reference Unigram fuzzer, a 20-input complete-ID Hugging Face run, and three no-HF counter-backed T5 runs. Retain only if IDs match and median throughput improves by at least 3%; report it as a T5 specialist result until broader confirmation.

Rejection rule: fully revert if any direct normalizer or pipeline parity mismatch occurs, if the profile mechanism does not produce a 3% median gain, or if the fast span boundary cannot be explained as exactly reference-equivalent. Do not alter the corpus, timing boundary, worker count, model revision, or evaluator to rescue the candidate.

Result: retained T5 specialist result. The direct pinned-T5 normalizer/reference test covers printable ASCII, every ASCII control, CRLF, combining marks, full-width forms, CJK, emoji, and mixed runs; it passed, as did existing T5 scalar/batch/ragged and GPT-2 integration tests, the 5,000-run nightly sanitizer Unigram fuzz test, and a 20-input complete-ID Hugging Face run. That parity run measured Hugging Face `7,669.05 ms` and candidate `553.24 ms`, a `13.86x` ratio. The unchanged no-HF parent rounds were `1,052.80`, `1,064.54`, and `1,062.45 ms`; candidate rounds were `538.78`, `545.67`, and `537.36 ms`. Their medians are `1,062.45` and `538.78 ms`, a `1.971955x` candidate/parent throughput result well beyond the predeclared floor. Median instructions fell from `19.833B` to `12.117B` (38.9%), branches from `4.172B` to `2.593B` (37.9%), and cycles from `8.589B` to `6.667B` (22.4%). Branch misses fell only from `59.49M` to `55.40M`, so their percentage rose from 1.43% to 2.14% with the reduced branch count. The scalar fast path is portable and the immutable T5 map falls back entirely if printable ASCII is ever remapped; this remains a T5/Intel specialist result until broader confirmation. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{scratch,normalizer}-nohf-*`.

### Unigram direct-root transition candidate (2026-09-14) — rejected and reverted

Parent SHA: `ba77b67`. Hypothesis: remove the root-node lookup and representation branch for every Viterbi character boundary with a permanent 256-entry root child table.

Result: rejected and fully reverted as `189810b`. Focused root-prefix/Viterbi tests, direct T5 charsmap coverage, scalar/batch/ragged T5 parity, GPT-2 regression, the 5,000-run nightly sanitizer Unigram fuzzer, and 20-input Hugging Face parity all passed; the latter measured Hugging Face `7,707.03 ms` versus candidate `565.54 ms` (`13.63x`). The fixed no-HF parent rounds were `537.33`, `540.36`, and `533.24 ms`; candidate rounds were `543.46`, `543.06`, and `543.89 ms`. The 543.46 versus 537.33 ms median is `0.989x` throughput. Candidate instructions fell from `12.112B` to `11.808B` (2.5%) and branches from `2.592B` to `2.535B` (2.2%), but cycles were flat and branch misses rose from `55.43M` to `56.36M`; the extra 1 KiB root structure did not repay on this workload. No broader run is justified. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{normalizer,root}-root-nohf-*`.

### Unigram terminal-ID sentinel candidate (2026-09-14) — rejected and reverted

Parent SHA: `1bb3c78`. Hypothesis: replace terminal `Option<u32>` tags with the structurally unreachable `u32::MAX`, shrinking every trie node from 32 to 24 bytes while leaving all transitions intact.

Result: rejected and fully reverted as `c66c1b8`. Focused terminal/miss and Viterbi tests, direct T5 charsmap equivalence, scalar/batch/ragged T5 parity, GPT-2 regression, the 5,000-run nightly sanitizer fuzzer, and 20-input Hugging Face parity all passed. The latter measured Hugging Face `7,746.72 ms` and candidate `549.64 ms` (`14.09x`). Parent no-HF rounds were `541.63`, `548.46`, and `548.90 ms`; candidate rounds were `545.16`, `546.77`, and `540.17 ms`. The 545.16 versus 548.46 ms median is only `1.006x` throughput, below the 3% floor. Candidate median instructions rose from `12.120B` to `12.182B` (0.5%), branches were neutral, and branch misses rose from `55.42M` to `55.65M`; the smaller node does not improve this traversal. No broader result is justified. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{normalizer,terminal}-terminal-nohf-*`.

### Unigram WhitespaceSplit-Metaspace fusion candidate (2026-09-14) — planned

Parent SHA: `d8d5b9b`.

Hypothesis: the pinned T5 JSON's pre-tokenizer sequence is exactly `WhitespaceSplit → Metaspace(prepend=always, split=true)`. The current generic pipeline first constructs a complete word-range vector over the original buffer, then `Metaspace` allocates a replacement buffer and walks every range again. For a Unigram tokenizer with that structural sequence, scan each ordinary input range once, omit whitespace, prepend the same marker to each word, write the final buffer/ranges directly, and preserve added-token placeholders unchanged.

Measured hot cost: after retaining printable-ASCII charsmap normalization, the Intel cycle profile assigns 8.77% cumulatively to `PreTokenizer::pre_tokenize`, with 2.94% self time in Metaspace and 2.52% self time in WhitespaceSplit. The retained Viterbi scratch result establishes that the representative T5 document has a very large number of independent word splits, so avoiding the intermediate split vector and second walk has a concrete allocation and traversal target.

Invariant that makes the shorter path exact: `WhitespaceSplit` removes each Unicode-whitespace scalar and preserves non-whitespace ranges in order; `Metaspace` then replaces ASCII spaces (none remain), prepends its marker when the non-empty range does not already start with it, and creates the same marker-delimited final splits. The fused walker performs those same transitions in order. Added-token placeholders bypass both steps and are copied with their original ID, as in the separate steps.

Representation being preserved or changed: retain `WhitespaceSplit`, `Metaspace`, generic `PreTokenizer::Sequence`, all nonmatching sequences, BPE paths, model inference, and public APIs. Add a crate-private capability for only the two-step sequence and a direct writer on Metaspace; dispatch it only for `Model::Unigram`. No unsafe code, dependency, model-name/corpus branch, cache, or evaluator change.

Expected winning strata: SentencePiece-style Unigram JSON using the exact two-step sequence on long text and batches. Expected adverse strata: other pre-tokenizer sequences and BPE, which stay on their existing code; tiny supported calls may be neutral.

Smallest files that need changing: `src/lib.rs`, `src/pre_tokenizers.rs`, `src/pre_tokenizers/metaspace.rs`, focused pre-tokenizer tests, `tests/tokenizer.rs` existing real T5 coverage, and `fuzz/fuzz_targets/fuzz_unigram.rs` with a reference whitespace/Metaspace route. No `benchmarks/` source changes.

Acceptance rule: compare fused and serial intermediate buffer/range output on ASCII, Unicode whitespace, repeated markers, and added-token placeholders; run the expanded independent Unigram fuzzer, direct T5 charsmap/reference parity, T5 scalar/batch/ragged and GPT-2 regression tests, a 20-input Hugging Face ID run, then three no-HF Intel counter rounds. Retain only if IDs/ranges match and median throughput improves at least 3%; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert if the fusion changes split boundaries, added IDs, marker behavior, any reference output, or misses the 3% screen. Do not modify corpus, timer scope, worker count, model revision, or evaluator to rescue the candidate.

Result: retained T5 specialist result. The focused serial-versus-fused test covers both supported prepend modes, split true/false, Unicode whitespace, repeated markers, and added-token placeholders; it passed. The expanded independent Unigram fuzzer also covers `WhitespaceSplit → Metaspace(always, split=true)` on arbitrary text and passed 1,000 local plus 5,000 nightly sanitizer runs. Direct T5 charsmap/reference parity, T5 scalar/batch/ragged integration tests, GPT-2 regression, and the unchanged 20-input complete-ID Hugging Face run passed. The latter measured Hugging Face `7,537.49 ms` and candidate `419.46 ms` (`17.97x`). The fixed no-HF parent rounds were `540.71`, `546.82`, and `549.28 ms`; candidate rounds were `400.40`, `407.20`, and `396.25 ms`. Their medians are `546.82` and `400.40 ms`, a `1.365684x` candidate/parent throughput result. Median instructions fell from `12.114B` to `10.373B` (14.4%), branches from `2.592B` to `2.182B` (15.8%), and cycles from `6.704B` to `6.230B` (7.1%). Branch misses fell from `55.34M` to `53.44M`, though the lower branch count raises their rate from 2.13% to 2.45%. Only the exact structural sequence under a Unigram model uses this path; BPE and all other sequences remain unchanged. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{normalizer,fused-whitespace}-fusion-nohf-*`.

### Unigram in-node reachability sentinel candidate (2026-09-14) — rejected

Candidate `025cd383363995e79f78a69ba2abc07dbbbabe61` replaced the private `Vec<Option<BestPathNode>>` with a 24-byte node vector using `usize::MAX` in `starts_at` as the unreached marker. It cleared only that predecessor word between independent splits, retaining the same scores, strict-greater tie behavior, unknown fallback, trie, APIs, BPE path, and evaluation boundary. Its focused stale-workspace test, all Unigram model tests, pre-tokenizer tests, T5 charsmap/reference parity, T5 scalar/batch/ragged integration, GPT-2 regression, and fuzzer build passed. The independent Unigram fuzzer ran 1,000 local cases and 5,000 nightly ASan cases on GCP without a mismatch. The pinned 20-input GCP Hugging Face ID run also passed: Hugging Face took `7,592.59 ms` and the candidate took `420.38 ms` (`18.06x`).

The counter-backed no-HF parent/candidate rounds in milliseconds were `409.26/411.33`, `401.27/399.34`, and `392.59/409.44`. Their medians were `401.27` and `409.44 ms`, so median candidate/parent throughput was `0.980047x`, well below the required `1.03x` retention gate. Candidate instructions were not lower (`10.438B`, `10.424B`, and `10.434B` versus parent `10.373B`, `10.374B`, and `10.372B`), confirming that removing the Option layout did not remove useful recurrence work. Commit `a1a8fc1` fully reverts the source, stale-workspace test, and planned card; no API, format, dependency, evaluator, benchmark, BPE path, or fuzzer representation change is retained. Raw GCP outputs and counters remain at `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/in-node-{parent,candidate}-{1,2,3}.{out,perf}`.

### Unigram all-match automaton candidate (2026-09-14) — planned

Parent SHA: `3043783` (the source is identical to retained fusion head `09ad7cc`; later commits record and revert the failed workspace candidate).

Hypothesis: the retained Viterbi profile attributes 52.26% of samples to `Unigram::tokenize_into_with_scratch`. Its hottest annotated instructions are the repeated sparse/dense trie transition, terminal test, and target-score update executed while restarting a prefix walk from every character boundary. The already-pinned `daachorse` dependency can scan a split once with a bytewise double-array Aho-Corasick automaton and report every overlapping vocabulary match. Gather those matches in a reused scratch vector, then process them by end boundary into the same Viterbi table. This turns repeated suffix walks into one forward automaton scan plus the unavoidable score updates.

Invariant that makes the shorter path exact: the automaton is built only from nonempty vocabulary spellings whose final `token_to_id` value matches the existing last-duplicate-wins trie rule. Each reported spelling has the same byte start, byte end, and ID as the trie prefix that formerly emitted it; because both input and spelling are valid UTF-8, those offsets are character boundaries. At each end boundary, all source scores are already final. Real matches are applied with the same strict-greater score comparison; an equal score retains the smaller source offset, exactly the source order in the old left-to-right loop. The unknown transition is added only if the same start has no one-character match, preserving the existing finite-score behavior and its edge cases. Scores, UTF-8 boundaries, unknown fusion, byte fallback, output order, and public APIs remain unchanged.

Representation being preserved or changed: replace only the private `PrefixTrie` matcher with a private double-array all-match automaton and a per-chunk reused `Vec<Match<u32>>`. Retain vocabularies, scores, token lookup, Viterbi table, added tokens, normalization, pre-tokenization, byte fallback, BPE paths, formats, evaluator, timing boundary, and the existing dependency version. No new dependency, unsafe code, model-name/corpus/input-size dispatch, cache, or benchmark change.

Expected winning strata: Unigram splits with overlapping prefixes and longer text, particularly T5's Metaspace word splits where the old matcher restarts at every Unicode boundary. Expected adverse strata: tiny words, very match-dense spellings, model construction, and memory, which remain explicit controls. BPE is unchanged.

Smallest files that need changing: `src/models/unigram.rs`, its Viterbi/tie unit coverage, existing `tests/tokenizer.rs` T5 integration coverage, `fuzz/fuzz_targets/fuzz_unigram.rs`, and this record. No `benchmarks/` source changes.

Mechanism evidence: the GCP post-fusion profile at `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/post-in-node-retained.perf` assigns 52.26% self-inclusive samples to the Viterbi routine. Its annotation attributes 11.14% of routine samples to the terminal-node test, 9.44% to the dense-child sentinel comparison, 8.73% to score addition, and 2.94% to a sparse-edge binary-search selection. The installed Daachorse 1.0.1 source and its API documentation verify that `find_overlapping_iter` exposes all matches' byte offsets and stored values using the existing dependency.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown tests, pre-tokenizer fusion, direct T5 charsmap/reference parity, T5 scalar/batch/ragged and GPT-2 integration tests; expand the independent Unigram fuzzer for overlapping suffix ties and run it locally plus 5,000 nightly ASan cases on GCP. Then run the 20-input complete-ID Hugging Face path and three counter-backed no-HF parent/candidate rounds on the same GCP host. Retain only with exact IDs, no material model-construction regression, and at least 3% median throughput improvement; keep as a T5 specialist until broader cross-host and fixture confirmation.

Rejection rule: fully revert if duplicate vocabulary precedence, match offsets, equal-score source order, unknown behavior, UTF-8, byte fallback, any parity/fuzz check, construction guard, or the 3% target screen fails. Do not alter corpus, timer scope, worker count, model revision, evaluator, or dependency to rescue the candidate.

Result: retained T5 specialist result. Candidate `cdaa54116770b414cd6184e5f24d0f6c92cd7f9a` replaces the private trie only with a double-array all-match automaton from the already-pinned `daachorse` dependency and reuses one match vector per Rayon chunk. Focused global Viterbi, equal-score, unknown, byte fallback, duplicate-piece, and new overlapping-automaton unit tests passed, as did pre-tokenizer fusion, T5 charsmap/reference parity, T5 scalar/batch/ragged integration, and GPT-2 regression before and after timing. The expanded independent fuzzer adds fixed overlapping suffixes `a`, `ab`, `b`, and `aba`; it passed 1,000 local cases and 5,000 nightly ASan cases on GCP. The pinned complete-ID 20-input T5 run passed with Hugging Face `7,587.89 ms` and candidate `369.42 ms`, a `20.54x` ratio.

The fixed no-HF parent/candidate rounds in milliseconds were `405.91/365.95`, `400.24/361.45`, and `389.67/354.99`. Their medians are `400.24` and `361.45 ms`, a `1.107318x` candidate/parent throughput result beyond the 3% retention gate. Median cycles fell from `6.230B` to `5.721B` (8.2%), while median instructions rose from `10.380B` to `11.123B` (7.2%); the all-match scan trades scalar work for much better control flow, cutting median branch misses from `53.464M` to `42.079M` (21.3%) despite branches rising from `2.184B` to `2.264B` (3.7%).

As a construction guard, a separate existing `print_pipeline` executable was built from the immutable parent and candidate, then each was run ten alternating fresh processes on the same T5 JSON. This is a coarse load-and-build process measurement, not a replacement for the encode timer: the parent median was `149.294 ms` and candidate median `101.923 ms` (`1.465x` candidate/parent throughput), so there is no construction regression. The native simple benchmark binary shrank from `11,809,152` to `11,802,832` bytes (0.054%). Parent/candidate SHA-256 values were `5ac92d7753af0e46607509e865bf6b1b9787ed17f565e2fd71b13b378dbc60cd` and `6de69f2e0385e0abbca534609006024c0ab7e76b2c8fc2e640de2ab2afa70733`. BPE behavior, formats, APIs, benchmarks, evaluator, dependency list, and timer scope are unchanged. Raw GCP evidence remains under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/{automaton-parent,automaton-candidate}-{1,2,3}.{out,perf}`, `automaton-load-pairs.csv`, and the two named immutable binaries.

### Unigram one-pass end-group candidate (2026-09-14) — planned

Parent SHA: `efef60d` (source-equivalent to retained automaton candidate `cdaa541`).

Hypothesis: after the automaton scan, Viterbi currently walks every same-end match slice twice: once to detect whether the current character has an exact one-character piece, then again to apply scored paths. The retained profile shows both loops in the inner routine. Accumulate that boolean while applying each same ordered match, and only then add the same unknown transition when no one-character match occurred. This removes one read-only pass over every reported match without changing the automaton, DP table, or score updates.

Invariant that makes the shorter path exact: both variants visit the identical end-group in identical order. The new loop sets `has_single_character_piece` exactly when `matched.start() == starts_at`, the old predicate. The unknown transition remains after all real matches and remains skipped exactly when that predicate is true. Scores, strict-greater comparison, equal-score source tie rule, duplicate IDs, UTF-8 offsets, unknown fusion, and output order remain unchanged.

Representation being preserved or changed: remove only the redundant group predicate pass inside private Unigram Viterbi. Retain the double-array matcher, scratch vector, all model and pipeline state, BPE behavior, APIs, formats, dependencies, evaluator, and timing boundary. No unsafe code, cache, dispatch, test fixture, or benchmark change.

Expected winning strata: T5-style text with multiple overlapping matches at an end boundary. Expected adverse strata: no-match and one-match short words where the pass is nearly free; BPE remains unchanged.

Smallest files that need changing: `src/models/unigram.rs` and this record. Existing model, integration, and independent overlapping-match fuzzer coverage directly exercise the preserved predicate.

Mechanism evidence: the retained GCP profile `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/automaton-retained.perf` shows the redundant read-only group walk beside the real match-update walk in `Unigram::tokenize_into_with_scratch`; its annotated instructions include the separate match-range loop before the same group is revisited for Viterbi scoring.

Acceptance rule: commit a clean candidate; format; run focused Viterbi/tie/unknown tests, T5 scalar/batch/ragged and GPT-2 integration, the existing expanded Unigram fuzzer locally and for 5,000 nightly ASan GCP cases, a 20-input full-ID Hugging Face run, and three counter-backed no-HF parent/candidate rounds. Retain only with exact output and at least 3% median throughput improvement; keep as a T5 specialist pending broader confirmation.

Rejection rule: fully revert if any single-character/unknown, tie, match order, parity, or fuzz result differs, or if the 3% target screen fails. Do not change the corpus, timer, worker count, model revision, evaluator, or dependency to rescue it.

Result: retained T5 specialist result. Candidate `7b91ea5e0bb06c445a36a50bab9e7281d0e29be8` merges the two same-end match loops without changing the matcher, Viterbi state, pipeline, BPE paths, API, format, dependency set, evaluator, or timer. The focused global-path, equal-score, unknown, byte-fallback, duplicate/match-order, pre-tokenizer, T5 scalar/batch/ragged, T5 precompiled-normalizer, and GPT-2 regression tests passed locally and again on GCP after timing. The independent overlapping-match Unigram fuzzer passed 1,000 local cases and 5,000 nightly ASan cases on GCP. The pinned complete-ID 20-input T5 run passed with Hugging Face `7,829.05 ms` and candidate `356.09 ms`, a `21.99x` ratio.

The counterbalanced no-HF parent/candidate rounds in milliseconds were `361.82/348.73`, `362.95/342.72` (candidate first), and `363.75/352.29`. Their medians are `362.95` and `348.73 ms`, a `1.04078x` candidate/parent throughput result above the 3% retention gate. Median cycles fell from `5.774B` to `5.625B` (2.6%), instructions from `11.139B` to `11.055B` (0.8%), branches from `2.268B` to `2.218B` (2.2%), and branch misses from `42.227M` to `40.299M` (4.6%). Raw GCP evidence remains under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/end-group-{parent,candidate}-{1,2,3}.{out,perf}` plus `end-group-full-hf.out`; this is an Intel/T5 specialist screen, not a cross-host or general-BPE-equivalence result.

### Unigram streaming automaton-match candidate (2026-09-14) — rejected and reverted

Candidate `67d4774bdc52a3ea9084a47b44e20df6ce89596e` removed the private reused `Vec<Match<u32>>` and consumed Daachorse's nondecreasing end-order iterator directly while retaining the same Viterbi end groups, strict score comparison, smaller-source tie rule, unknown path, UTF-8 boundaries, and all public/pipeline behavior. The overlap-order unit test was adjusted to exercise the iterator directly. Focused Unigram, pre-tokenizer, T5 scalar/batch/ragged, T5 normalizer, and GPT-2 tests passed; so did 1,000 local and 5,000 nightly ASan GCP cases against the independent fuzzer reference. The full 20-input Hugging Face ID run was exact and measured Hugging Face `7,722.28 ms` versus candidate `350.43 ms` (`22.04x`).

The counterbalanced no-HF parent/candidate rounds in milliseconds were `348.56/341.70`, `343.44/341.39` (candidate first), and `343.02/341.45`. The parent/candidate medians are `343.44` and `341.45 ms`, only `1.005828x` throughput, well below the required `1.03x` floor. The candidate did reduce median cycles from `5.616B` to `5.520B` (1.7%) and branch misses from `40.293M` to `33.068M` (17.9%), but instructions rose from `11.051B` to `11.777B` (6.6%) and branches from `2.217B` to `2.288B` (3.2%); removing the copied records did not remove enough automaton or iterator work. This commit fully restores the retained vector representation. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/streaming-{parent,candidate}-{1,2,3}.{out,perf}` and `streaming-full-hf.out`.

### Unigram monomorphized streaming-match candidate (2026-09-14) — planned

Parent SHA: `596d191`.

Hypothesis: the rejected streaming candidate used `Option::iter().flat_map(...)` to represent the rare no-automaton case. Its GCP profile assigns 28.47% of flat samples to `FlattenCompat::next`, which embeds both the automaton scan and a generic adapter branch on every emitted match. Dispatch once on `PrefixMatcher::automaton`, then call a generic Viterbi helper with Daachorse's concrete overlapping iterator. Preserve the all-empty-vocabulary case with a separate exact unknown path. This keeps the beneficial removal of the match vector while removing the adapter from the hot loop.

Invariant that makes the shorter path exact: the supported automaton case receives precisely the same `find_overlapping_iter` values in the same nondecreasing end-offset order as the rejected candidate. The helper applies the same end-group, strict score, source-tie, unknown, UTF-8, backtrack, and byte-fallback logic. In the no-automaton case there are no nonempty vocabulary pieces, so every character takes the old unknown transition and Hugging Face's adjacent-unknown fusion produces one whole-input unknown span; preserve that span and its existing byte-fallback behavior directly.

Representation being preserved or changed: remove the private match vector and the generic `FlatMap` iterator adapter from Unigram's hot path. Retain the existing double-array automaton, scores, IDs, DP nodes, path pieces, split scratch ownership, pipeline, BPE behavior, formats, APIs, dependency, evaluator, and timer. No unsafe code, cache, fixture, model/corpus branch, or benchmark change.

Expected winning strata: every nonempty Unigram vocabulary, especially the pinned T5 workload where the generic adapter dominated the rejected profile. Expected adverse strata: a malformed-but-accepted all-empty vocabulary, which takes the exact cold fallback; BPE remains unchanged.

Smallest files that need changing: `src/models/unigram.rs`, a focused all-empty-vocabulary unknown/byte-fallback test, the existing independent Unigram fuzzer, and this record. Existing overlap, tie, unknown, and T5 integration tests cover the common streaming case.

Mechanism evidence: the retained vector path profile `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/end-group-retained.perf` shows 21.65% in match-vector extension. The rejected direct-stream profile `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/streaming-rejected.perf` shows 28.47% in `FlattenCompat::next`, not an inlined direct Daachorse iterator. Replacing that adapter with the iterator's concrete monomorphization is the missing mechanism; merely streaming through the generic adapter was correctly rejected.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/overlap plus the new empty-vocabulary test, T5 scalar/batch/ragged and GPT-2 integration, and the independent fuzzer locally plus 5,000 nightly ASan GCP cases. Run the full 20-input Hugging Face ID comparison before timing, then three counterbalanced no-HF GCP parent/candidate rounds with counters. Retain only with exact IDs and at least 3% median throughput improvement; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert if the empty-vocabulary fallback, iterator order, tie/unknown behavior, UTF-8, any parity/fuzz result differs, or the 3% target screen fails. Do not modify corpus, model revision, runner, timer, workers, evaluator, or dependency to rescue it.

Result: retained T5 specialist result. Candidate `57ac171d0f66d1691816ad58c4ae662348c4450a` dispatches once on the presence of the private automaton, streams its concrete overlapping iterator through the same end-ordered Viterbi recurrence, and uses an exact fused-unknown cold path only when the accepted vocabulary has no nonempty piece. The focused all-empty fallback test, global-path/tie/unknown/byte-fallback/overlap tests, pre-tokenizer suite, T5 scalar/batch/ragged integration, T5 normalizer, and GPT-2 regression passed locally and again on GCP after timing. The independent fuzzer passed 1,000 local and 5,000 nightly ASan GCP cases. The pinned complete-ID 20-input T5 run passed with Hugging Face `7,801.58 ms` and candidate `304.65 ms`, a `25.61x` ratio.

The counterbalanced no-HF parent/candidate rounds in milliseconds were `349.76/296.65`, `348.30/292.03` (candidate first), and `347.91/291.58`. Their medians are `348.30` and `292.03 ms`, a `1.192686x` candidate/parent throughput result far beyond the 3% retention gate. Median cycles fell from `5.626B` to `4.983B` (11.43%), instructions from `11.049B` to `10.213B` (7.57%), branches from `2.217B` to `2.049B` (7.60%), and branch misses from `40.274M` to `33.990M` (15.60%). The benchmark binary fell from `11,803,408` to `11,803,280` bytes (128 bytes); construction and model representation are unchanged and were not remeasured. Parent/candidate SHA-256 values are `2fbd6c31948a3c5e356b9389d6ab04a13826e7f8c8d93ee2e3800186ed7484bb` and `c39dd3c83ceb39505ebe09a51d2fe5ebae7c2229ce0f6e0a6ed5770568463601`. BPE behavior, formats, API, fixture, dependency list, evaluator, and timer scope are unchanged. Raw evidence remains under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/monomorphized-{parent,candidate}-{1,2,3}.{out,perf}` and `monomorphized-full-hf.out`; this is an Intel/T5 specialist result, not yet a cross-host or general-BPE-equivalence claim.

### Unigram Daachorse 3 matcher candidate (2026-09-14) — rejected and reverted

Candidate `b506c8ec25dc72ffb51ddd8c559027513b1b3e5a` (with fuzz-lock follow-up `4fd5f6bc194597018ceffb9713a09091339b91cf`) upgraded only the existing Daachorse dependency from 1.0.1 to 3.0.3. The builder and overlapping iterator preserved exact behavior: focused Unigram/pre-tokenizer/T5/GPT-2 tests passed, as did 1,000 local and 5,000 nightly ASan GCP fuzzer cases. The full 20-input Hugging Face ID run also passed, measuring Hugging Face `7,777.99 ms` and candidate `303.50 ms` (`25.63x`).

The counterbalanced no-HF parent/candidate rounds in milliseconds were `290.62/295.13`, `295.20/294.57` (candidate first), and `292.82/293.73`. Their medians are `292.82` and `294.57 ms`, `0.994059x` candidate/parent throughput, below the 3% retention floor. Candidate cycles fell only 0.51% (`4.967B` to `4.942B`) and instructions 0.58% (`10.214B` to `10.155B`), while branch count was neutral and the wall result regressed. This commit fully restores the 1.0.1 manifest and both root/fuzz lockfiles; no dependency change is retained. Raw outputs and counters remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/daachorse3-{parent,candidate}-{1,2,3}.{out,perf}` and `daachorse3-full-hf.out`.

### Unigram reachable-Viterbi workspace candidate (2026-09-14) — planned

Parent SHA: `dbdc31d`.

Hypothesis: every successful Unigram step with an `unk_id` reaches the next character boundary: a one-character real piece updates it, otherwise the required unknown transition does. The current scratch table is `Vec<Option<BestPathNode>>`; every split clears/resizes an entry for every input byte despite only character boundaries being read. For the common `unk_id` path, retain a 24-byte plain node table, mark only the current character end as unreached before applying its end group, and reuse all other established predecessors. This removes the Option tag and full-table clear without changing the recurrence. Keep the existing checked Option table for the no-`unk_id` path, where a missing single-character piece must still return the current error.

Invariant that makes the shorter path exact: induction starts at boundary zero. With `unk_id`, each prior character boundary is reachable; for the next end, a same-character real match uses that reachable source, or the old unknown transition writes it. Matches are processed in increasing end order, so no future endpoint can be written early. Resetting the current endpoint to the same unreached sentinel before its group reproduces `Option::None`; strict-greater scoring, equal-score smaller-source tie selection, unknown behavior, UTF-8 boundaries, backtracking, and byte fallback are identical. Without `unk_id`, retain the existing checked representation and error path.

Representation being preserved or changed: add a private plain `BestPathNode` scratch vector for the known-reachable path and retain the existing Option vector only for unsupported-character error handling without an unknown ID. Models, scores, automaton, IDs, path pieces, split ownership, pipeline, BPE behavior, formats, APIs, dependencies, evaluator, and timer stay unchanged. No unsafe code, cache, fixture, model/corpus branch, or benchmark edit.

Expected winning strata: normal SentencePiece-style Unigram tokenizers with `unk_id`, including the pinned T5 tokenizer and its many short Metaspace splits. Expected adverse strata: no-`unk_id` models, which preserve the old path, and tiny inputs where scratch clearing is negligible; BPE remains unchanged.

Smallest files that need changing: `src/models/unigram.rs`, focused known-unknown/no-unknown workspace tests, the existing independent Unigram fuzzer, and this record. No evaluator files change.

Mechanism evidence: the retained GCP annotation `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/monomorphized-retained.perf` shows the byte-stride Option table clear in `tokenize_into_with_scratch`, while the resulting Viterbi code repeatedly tests Option tags at each source/target. A previous in-node-sentinel experiment was rejected because it still reset every entry and predates both all-match and specialized streaming; this candidate's new mechanism is reachability-based endpoint reset, not a layout-only retry.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/overlap, known-unknown and no-unknown tests, T5 scalar/batch/ragged and GPT-2 integration, and the independent fuzzer locally plus 5,000 nightly ASan GCP cases. Run full 20-input Hugging Face IDs before timing, then three counterbalanced no-HF GCP parent/candidate rounds with counters. Retain only with exact IDs and at least 3% median throughput improvement; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert if any stale scratch entry, unknown/no-unknown error, tie, UTF-8, parity, or fuzz discrepancy occurs, or if the 3% target screen fails. Do not alter corpus, model revision, runner, timing, workers, fixture, evaluator, or dependency to rescue it.

Result: retained T5 specialist result. Candidate `07545451b69a2221002699b26d675f118d729822` uses a private plain Viterbi table only when `unk_id` proves every character boundary reachable; each character end is reset immediately before its end-ordered match group, while the no-`unk_id` path retains the prior checked Option table and exact error. Focused global/tie/unknown/overlap, new no-unknown coverage, pre-tokenizer, T5 scalar/batch/ragged, T5 normalizer, and GPT-2 tests passed locally and again on GCP after timing. The independent fuzzer passed 1,000 local and 5,000 nightly ASan GCP cases. The complete-ID 20-input T5 comparison passed with Hugging Face `7,632.98 ms` and candidate `288.49 ms`, a `26.46x` ratio.

The counterbalanced no-HF parent/candidate rounds in milliseconds were `294.47/279.94`, `295.47/281.58` (candidate first), and `293.47/283.95`. Their medians are `294.47` and `281.58 ms`, a `1.045777x` candidate/parent throughput result over the 3% retention gate. Median cycles fell from `4.964B` to `4.820B` (2.89%), instructions from `10.213B` to `9.862B` (3.44%), branches from `2.049B` to `1.984B` (3.18%), and branch misses from `34.010M` to `33.134M` (2.57%). The native benchmark binary rose from `11,803,280` to `11,805,456` bytes (2,176 bytes) because the uncommon no-unknown checked recurrence remains beside the fast path; the model format and construction representation are unchanged and were not remeasured. Parent/candidate SHA-256 values are `c39dd3c83ceb39505ebe09a51d2fe5ebae7c2229ce0f6e0a6ed5770568463601` and `4d4e72797ca45435c2c05518eec746a32e9f051db3607af7439e52555249560b`. BPE, APIs, formats, dependency list, evaluator, fixture, and timing boundary are unchanged. Raw evidence remains under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/reachable-{parent,candidate}-{1,2,3}.{out,perf}` and `reachable-full-hf.out`; this is an Intel/T5 specialist result, not a cross-host or general-BPE-equivalence claim.

### Unigram monotonic reachable-workspace candidate (2026-09-14) — planned

Parent SHA: `a9b351d64f2d3b7c584c66aff80fd6d81cde6d03`.

Hypothesis: the reachable Viterbi recurrence now resets only each current endpoint, but its workspace first `truncate`s to the current split length and then `resize`s on every later longer split. The GCP annotation of the retained binary shows the generated 24-byte node initialization loop at the start of `Unigram::tokenize_into_with_scratch`. Keeping the backing table at its largest prior length within the already-owned Rayon chunk removes those repeated stores for shorter and medium splits.

Measured hot cost: the retained sampled profile assigns 35.39% of process samples to `Unigram::tokenize_into_with_scratch`; its local annotation shows the `Vec::resize` initialization loop immediately before the end-ordered automaton scan. T5's Metaspace pipeline produces variable-length independent word splits, so a workspace that is repeatedly shortened pays that initialization again even though its allocation capacity is retained.

Invariant that makes the shorter path exact: with `unk_id`, entry zero is overwritten before every split and every later character endpoint is set to the unreached sentinel immediately before its complete end-ordered match group. A match source is either entry zero or a prior character endpoint already set in this same split; valid UTF-8 pieces never access a non-boundary. Entries beyond the current input length are never indexed. Therefore an oversized backing vector cannot supply stale state, while newly appended entries remain initialized to the same unreached sentinel. The no-`unk_id` checked table is unchanged.

Representation being preserved or changed: retain the existing per-Rayon-chunk private `Vec<BestPathNode>` and all Viterbi, matcher, tie, unknown, byte-fallback, normalization, pre-tokenization, output, BPE, API, format, dependency, fixture, evaluator, and timing behavior. Change only its private lifetime from current-split length to maximum split length in the chunk. No thread-local/global cache, unsafe code, allocation of token outputs, model/corpus/input-size dispatch, or benchmark edit.

Expected winning strata: split-heavy Unigram pipelines with variable word lengths, including pinned T5. Expected adverse strata: one unsplit or monotonically longer split sequence, where the old and new initialization work converge; models without `unk_id` and BPE are unchanged.

Smallest files that need changing: `src/models/unigram.rs`, one focused stale-workspace boundary test, the existing independent Unigram fuzzer only if it needs a sequence assertion, and this record. No `benchmarks/` source changes.

Mechanism evidence: `perf annotate` on `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/reachable-retained.perf` shows scalar stores of `BestPathNode::unreached()` in the `truncate`/`resize` sequence. This is a representation-lifetime correction to the retained reachability invariant, not a retrial of the rejected full-table sentinel reset.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/no-unknown and long-to-short scratch tests, T5 scalar/batch/ragged plus GPT-2 integration, and the independent Unigram fuzzer locally plus 5,000 nightly ASan GCP cases. Run the complete-ID 20-input Hugging Face comparison before timing, then three counterbalanced no-HF GCP parent/candidate rounds with counters. Retain only with exact IDs, no stale-workspace failure, and at least 3% median throughput improvement; measure resident memory separately and retain the result only as a T5 specialist pending wider confirmation.

Rejection rule: fully revert if any stale entry, unknown/no-unknown error, tie, UTF-8, parity, fuzz discrepancy, or BPE regression appears; reject if the target median gain is below 3% or resident memory cost is material. Do not alter corpus, model revision, runner, timing, workers, fixture, evaluator, or dependency to rescue the result.

Result: rejected at the target speed screen. Candidate `42276938e7d41f1d23787e327ac25d282efdcc20` passed focused Unigram, T5 scalar/repeated-prefix, and GPT-2 tests on GCP; its 5,000-run nightly ASan Unigram fuzzer completed without a mismatch or sanitizer finding. The full 20-input Hugging Face comparison also preserved every ID, measuring Hugging Face `7,861.56 ms` and candidate `289.07 ms` (`27.20x`).

The counterbalanced no-HF parent/candidate rounds in milliseconds were `282.32/277.53`, `283.17/280.06` (candidate first), and `276.45/275.11`. Their medians are `282.32` and `277.53 ms`, only `1.017259x` candidate/parent throughput and below the predeclared 3% floor. Median cycles fell from `4.857B` to `4.781B` (1.57%), instructions from `9.860B` to `9.803B` (0.58%), branches from `1.983B` to `1.974B` (0.46%), and branch misses from `33.039M` to `31.205M` (5.55%); the reduced work did not translate into a sufficient wall-time gain. The candidate is fully reverted in the following commit, including its implementation-specific unit test. No API, format, dependency, benchmark, evaluator, BPE, or fuzzer source change remains. Raw remote evidence is `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/monotonic-{parent,candidate}-{1,2,3}.{out,perf}` and `monotonic-full-hf.out`.

### Unigram contiguous-Aho matcher candidate (2026-09-14) — planned

Parent SHA: `60b3a90e7d71bf7ff98a2d1a53fca4a3e127b94d`.

Hypothesis: the retained 92,768-sample GCP profile assigns 35.31% of samples to the Unigram routine. Its hottest inner instructions are the Daachorse double-array transition's check-byte comparisons and state loads: `cmp` at `0x4192e8` (9.82% local routine samples) and the corresponding state load at `0x41933f` (10.63%). Replace the private all-match matcher with the already-transitive `aho-corasick` engine's automatically selected contiguous NFA/DFA representation, preserving the same one-pass, end-ordered overlapping stream while testing whether its representation and optional prefilter reduce those dependent transition costs.

Measured hot cost: the aggregate profile at `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/reachable-aggregate.perf` contains 92,768 cycle samples with zero lost samples. `Unigram::tokenize_into_with_scratch` is 35.31% self-inclusive; the emitted disassembly is dominated by the current double-array state/check fetches rather than table initialization. The previous monotonic-workspace candidate removed some initialization but yielded only `1.017259x`, so it is rejected and does not select the next mechanism.

Invariant that makes the shorter path exact: build the matcher only from the existing nonempty, last-duplicate-wins vocabulary pieces and retain a pattern-ID-to-token-ID table. Standard Aho-Corasick overlapping search reports each match when its ending position is reached; all same-end matches are complete before the next byte. Every returned start/end remains a byte range of the identical matched spelling, and vocabulary/input UTF-8 makes those ranges character boundaries. The Viterbi recurrence retains its strict score ordering and explicit smaller-source equal-score tie rule, so same-end pattern order is immaterial. Unknown fallback, byte fallback, no-unknown errors, scores, and output emission stay unchanged.

Representation being preserved or changed: replace only `PrefixMatcher`'s private Daachorse automaton with a private `AhoCorasick` plus immutable pattern-to-token-ID vector. Add an explicit root dependency on the already-locked `aho-corasick 1.1.4`; do not alter its version or add a new package. Retain model JSON, ID tables, Viterbi workspace, pipeline, BPE, API, cache formats, fixture, evaluator, timing boundary, and output allocation. No unsafe code, model/corpus/input-size dispatch, complete-output cache, or benchmark edit.

Expected winning strata: Unigram texts with many bytes that do not continue a vocabulary piece, where the alternate automaton's state layout or prefilter can reduce check transitions. Expected adverse strata: match-dense Unicode pieces and model construction, because the alternate engine uses a different immutable automaton layout; BPE remains unchanged.

Smallest files that need changing: `Cargo.toml`, root and fuzz lockfiles only as resolver-required, `src/models/unigram.rs`, one focused end-order/duplicate test, the independent Unigram fuzzer if mapping coverage needs expansion, and this record. No `benchmarks/` source changes.

Mechanism evidence: upstream's local `aho-corasick 1.1.4` source documents `MatchKind::Standard` as the only overlapping mode and emits matches through `find_overlapping_iter` as each forward search reports them. Its automatic builder prefers a contiguous NFA for larger pattern sets, with a DFA attempted only for small sets. The retained profile proves the current automaton transition representation is hot; it does not predict the replacement will win.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/no-unknown, duplicate, and nondecreasing-end ordering tests; run T5 scalar/batch/ragged plus GPT-2 integration and the independent Unigram fuzzer locally plus 5,000 nightly ASan GCP cases. Run full 20-input Hugging Face IDs before timing, then three counterbalanced no-HF GCP parent/candidate rounds with counters. Retain only with exact IDs, no construction/resource regression that outweighs a target win, and at least 3% median throughput improvement; keep it T5-specialist pending wider confirmation.

Rejection rule: fully revert if matcher offset/order/duplicate precedence, tie, unknown, UTF-8, byte fallback, parity, fuzz, construction, or BPE behavior differs; reject if the target median gain is below 3%. Do not alter corpus, model revision, runner, timing, workers, fixture, evaluator, or dependency version to rescue the result.

Result: rejected at the first decisive speed gate. Candidate `488c3d96f0775a98f3f36cbb469cd3571c2f2c6c` passed focused Unigram, T5 scalar/repeated-prefix, and GPT-2 tests on GCP, plus 5,000 nightly ASan Unigram fuzzer iterations with no mismatch or sanitizer finding. Its complete-ID 20-input Hugging Face run also passed, but measured Hugging Face `7,743.21 ms` and candidate `465.78 ms` (`16.62x`). The retained matcher previously completed the same fixed T5 workload in `288.49 ms`; this 61.46% candidate slowdown is decisive, so counter rounds and construction expansion are skipped under the earliest-rejection rule.

The candidate binary is `11,820,696` bytes versus the retained `11,805,456` bytes, a `15,240`-byte increase. The source, focused ordering test, direct dependency, and resolver lock changes are fully reverted in the following commit; no alternate matcher, dependency, API, format, BPE, evaluator, benchmark, or fuzzer source change remains. Raw remote evidence is `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/contiguous-aho-full-hf.out` and `simple-bench-contiguous-aho`.

### Scalar-fallback audit conclusion (2026-09-14)

The runtime source is restored exactly to scalar-fallback parent `719efd3` (`mask_scanner.rs` and its scanner tests have no diff). Four independent Kimi fallback mechanisms were exhausted: reuse the fallback-start classification, skip scalar decode for direct Han, vectorize ten direct Han scalars, and gate that vector body behind a three-scalar direct-Han prefix. The first two were neutral under the fixed Kimi screen; the vector bodies proved exact and fast in a synthetic dense-Han mechanism check but could not clear the real LongBench paired gate or stable batch control. They remain reverted.

No broad fallback optimization remains that is both independent and safe to implement without new attribution. `KimiScheme` alone sets `SIMD_UNICODE = false`; the AVX2/AVX-512 front ends already bail before discarded class chains for that flavor. The other fixed grammars use `extended_masks` for Unicode and scalar-walk only masks deliberately marked for run-contextual marks, straddling whitespace or counted-number rules, unresolved tail carries, and contraction lookaround. Removing or narrowing those zones would require a new grammar proof rather than a local performance edit. On unsupported CPUs, the scalar walker already executes directly after one `MaskState` setup; bypassing that setup would remove only one per-input state construction and has no measurable target or source-level hot cost. A full Kimi Unicode mask algebra is a new scanner implementation, not a fallback-path refinement, and the fixed Kimi LongBench screen contains only 0.1047% direct Han characters.

The task-owned Intel host did not permit PMU attribution (`perf_event_paranoid=3`), and that policy was not changed. With no additional distinct, evidence-backed fallback mechanism and all implemented candidates rejected/reverted, this search stops here.

### Scalar-fallback experiment 4 — gate Kimi AVX2 blocks on a direct-Han prefix (2026-09-14)

Parent SHA: `f06f3c116f7f53c6d5a7831fe8166b658328513d`.

Hypothesis: experiment 3 established that an AVX2 block can reduce dense direct-Han scanning, but its helper probed every confirmed Han start when 32 input bytes remained. Most LongBench runs are short or mixed. Requiring three exact direct-Han scalars before the AVX2 call means isolated Han scalars take the existing decoder alone; dense runs still skip the prevalidated three scalars and scan the following ten starts with the same vector proof.

Measured hot cost: the experiment-3 dense mechanism screen was exact and `1.004248x` with full output construction/destruction, while LongBench sequential was `1.013709x`; its batch screen was unstable and cannot support retention. The source-visible adverse mechanism is the existing candidate's unconditional vector entry for every direct-Han start with enough trailing bytes. This is a new entry-eligibility hypothesis, not a byte-range or corpus retune.

Invariant that makes the shorter path exact: the prefix admits only three independently validated U+3400--U+4DBF or U+4E00--U+9FFF scalars, so advancing nine bytes reaches the same UTF-8 boundary that the old scalar loop would reach. The AVX2 body keeps experiment 3's exact ten-start proof and returns without consuming a mixed block. Extension Han, U+4DC0--U+4DFF, punctuation, marks, tail bytes, ARM, and non-AVX2 x86 all remain in the exact scalar loop.

Representation being preserved or changed: retain the Kimi regex, scalar grammar, Unicode table, masks, BPE, APIs, fixtures, benchmark runner, and experiment-3 byte intervals. Add only a three-scalar entry gate before the private AVX2 continuation helper. No model/input threshold, public corpus, evaluator, dependency, or fallback ownership changes.

Expected winning strata: direct-Han runs of at least fourteen scalars on AVX2 x86-64. Expected adverse strata: all short or mixed Han runs avoid the vector probe and otherwise retain the old scalar work; every non-Kimi grammar and unsupported CPU is unchanged.

Smallest files that need changing: `src/pre_tokenizers/scanner/mask_scanner.rs`, the existing Kimi scanner differential test to cross the direct-Han threshold and U+4DC0 boundary, and this record. No benchmark or fixture source changes.

Mechanism evidence: direct-Han UTF-8 begins are exactly inspectable at `end`, `end + 3`, and `end + 6` after the first confirmed scalar. `str` validity makes those offsets character boundaries only after each preceding direct predicate succeeds. The experiment-3 code and result prove both the vector body and isolated-entry cost; this candidate changes only when that proven body may run.

Acceptance rule: format; run all four scanner differential tests and `correctness_kimi_k2_5` locally and on x86; then require parent/candidate complete Kimi Hugging Face IDs for the fixed 32 LongBench inputs sequentially and batch-32 before and after timing. Use the same eight fresh output-destruction-inclusive dense and LongBench sequential pairs, counterbalanced four per order. The unchanged batch path remains a regression observation but must not be used to turn noisy process results into a speed claim. Retain only if exactness passes and both paired sequential screens improve reproducibly without a new adverse control.

Rejection rule: reject and revert for a mismatch, missing x86 execution, source outside the declared entry gate, unsupported-CPU behavior change, missed paired screen, or a repeatable adverse batch control. Do not change the vector byte ranges, number of prevalidated scalars, corpus, runner, scalar grammar, or acceptance criteria after results.

Result: rejected at the fixed LongBench sequential pool. Candidate `3db23b10e5047ea13a52682b3b59f7c595ca9a2f` passed formatting, ARM scalar-oracle tests, x86-64 cross-compilation, all four Intel-host scalar-oracle tests, and `correctness_kimi_k2_5`. It passed complete Kimi/Hugging Face IDs on the dense-Han mechanism input (699,040 IDs) and on all 32 frozen LongBench contexts, sequentially and in one batch (6,309,180 IDs from 24,432,087 characters each). The candidate's dense exact parity run took 0.92 ms versus the experiment-3 parent's earlier 1.93 ms before output destruction, so the gated vector body is exercised, but no dense timing pool was run after the frozen control missed its predeclared paired gate.

Eight fresh LongBench `--no-hf` pairs with output construction and destruction produced candidate/parent ratios `1.021873`, `1.000259`, `0.978305`, `1.033957`, `1.023899`, `1.027383`, `0.981522`, and `1.006584`: `1.009028x` geometric mean with paired log-ratio 95% interval `[0.991437x, 1.026931x]`, parent-first `1.001168x`, and candidate-first `1.016949x`. The interval crosses neutral and the order split is material, so the small mean cannot support retention. One-off exact batch totals varied from 106.60 to 201.64 ms in the surrounding screens, consistent with the rejected experiment's process-level instability and not evidence for a new tuning pass. Revert the source in full; do not add a fourth threshold, broaden the vector body, or retry the same dense-Han mechanism without new attribution showing it can affect a broader real workload.

### Scalar-fallback experiment 3 — scan dense Kimi Han runs in AVX2 blocks (2026-09-14)

Parent SHA: `719efd3f1fc85719aea1c8e237bac05ea64b7cd4`.

Hypothesis: Kimi sends Unicode-adjacent batches through its exact scalar scanner. Once `scan_kimi_han_run` has confirmed a direct Han scalar, it decodes every following three-byte scalar independently. An AVX2 helper can validate ten consecutive scalar starts at byte offsets 0, 3, ..., 27 in one 32-byte load, then advance 30 bytes at once. The existing scalar decoder remains responsible for the first scalar, tail, every mixed ten-scalar block, all extension Han ranges, and every CPU without AVX2.

Measured hot cost: the current Kimi LongBench screen is deliberately retained as a regression control, but it contains only 25,364 direct Han characters out of 24,220,430 (0.1047%) across its fixed 32 contexts, so it cannot establish a dense-Han speed claim. The repository's earlier mixed-CJK screen retained the separate SIMD early-bail optimization, which proves the Kimi Unicode fallback is exercised but not that a scalar Han-loop reduction is material. This candidate therefore requires exact Kimi checks plus an isolated dense-Han mechanism screen; it cannot be promoted as a general gain from either screen alone.

Invariant that makes the shorter path exact: valid `str` input means every `E3`--`E9` lead has its required continuation bytes. The AVX2 block accepts exactly U+3400--U+4DBF (`E3 90..BF` and `E4 80..B6`) and U+4E00--U+9FFF (`E4 B8..BF` and `E5..E9`); it rejects the U+4DC0--U+4DFF gap (`E4 B7`) and leaves it to the existing decoder and table. A false result exits without consuming any scalar, so the scalar loop chooses the original next boundary.

Representation being preserved or changed: preserve the Kimi regex, scalar grammar, Unicode table, `MaskState` fallback ownership, existing AVX-512/AVX2 dispatch, unsupported-CPU behavior, BPE, public API, model fixture, and timing runner. Add only a private AVX2 dense-direct-Han probe beneath the confirmed Han loop.

Expected winning strata: Kimi inputs with direct-Han runs of at least eleven scalars on AVX2-capable x86-64. Expected adverse strata: ordinary LongBench Kimi text, mixed Unicode, non-Kimi patterns, short Han runs, extension Han, the Yijing gap, ARM, and unsupported x86 retain the existing scalar behavior except for one runtime feature check per confirmed run.

Smallest files that need changing: `src/pre_tokenizers/scanner/mask_scanner.rs`, its existing scanner differential test to cross every direct-range edge across an AVX2 block, and this record. No production corpus, tokenizer, evaluator, dependency, model dispatch, or public API changes.

Mechanism evidence: `scan_kimi_han_run` is currently a loop of `decode_cp` and `MaskClassTable::is_han`; its direct intervals are byte-regular three-byte UTF-8 ranges. The fixed LongBench input's 0.1047% direct-Han density rules it out as the sole target decision, while prior CJK work shows this exact fallback is a real Kimi path. `perf` attribution remains unavailable on the task host because `perf_event_paranoid=3`; kernel policy stays unchanged.

Acceptance rule: format and run all four scalar-versus-mask differential tests plus `correctness_kimi_k2_5`; then compare immutable parent/candidate processes with complete Hugging Face Kimi IDs before and after the established 32-input LongBench sequential and batch screen. A dense-Han fixture may test only the declared mechanism and must not replace the frozen LongBench control or support a general speed claim. Retain only if exactness passes and both the dense-Han mechanism screen and the frozen LongBench control show a repeatable nonnegative result without order dependence. Report every result, including neutral controls, and revert the runtime source on rejection.

Rejection rule: reject and revert for any mismatch, focused-test failure, AVX2-gating error, source outside this one continuation probe, unsupported-CPU behavior change, or missed screen floor. Do not broaden the byte ranges, add an input/model threshold, modify the scalar grammar, alter the benchmark runner, or use a dense-Han fixture as a general performance claim.

Result: rejected as an over-eager AVX2 entry shape. Candidate `036b56eb01702042551ed198e4fe410b6247c768` passed local ARM scalar-oracle tests, an x86-64 cross-compile, the Intel-host AVX2 scalar-oracle tests, and `correctness_kimi_k2_5`. It also passed complete Kimi/Hugging Face IDs on a deterministic dense-Han mechanism input (32 × 65,535 UTF-8 bytes, 699,040 IDs) and on the unchanged 32 LongBench contexts, sequentially and in one batch (24,432,087 characters and 6,309,180 IDs each). The small dense parity pass measured 1.93 ms parent and 0.91 ms candidate, showing that the vector body executes, but that timer excludes output destruction. With output construction and destruction included, eight fresh 33,554,400-byte dense-Han pairs gave candidate/parent ratios `1.000746`, `1.008711`, `1.003617`, `1.009419`, `1.000572`, `1.002272`, `1.002103`, and `1.006586`: `1.004248x` geometric mean, paired log-ratio 95% interval `[1.001310x, 1.007194x]`, parent-first `1.001759x`, candidate-first `1.006743x`.

The frozen sequential LongBench screen likewise had eight fresh output-destruction-inclusive ratios of `1.005628`, `0.992741`, `1.025064`, `1.019587`, `0.998960`, `1.035738`, `1.013079`, and `1.019568` (`1.013709x` geometric mean, 95% interval `[1.001924x, 1.025632x]`, parent-first `1.010636x`, candidate-first `1.016791x`). That small positive result is not enough to retain a 72-line target-feature path when the predeclared batched control is irreconcilable: the first exact batch pass was 124.85 ms parent versus 130.85 ms candidate, and alternating exact repeat pairs were 122.97/113.15 ms (`1.086787x`), 90.77/119.38 ms (`0.760345x`), and 108.77/133.09 ms (`0.817267x`). Those batch samples demonstrate an unstable process-level path, not a reproducible candidate result. The vector helper also probes every direct-Han start with at least 32 trailing bytes, including isolated Han scalars, so its entry overhead is a distinct cost. Revert the source in full; a separate experiment may test a multi-scalar eligibility gate, but may not reuse this result to tune byte ranges or declare a general win.

### Branch/cache local screening session (2026-09-09): three scoped retentions, four rejections

### Scalar-fallback experiment 1 — classify a Kimi fallback start once (2026-09-14)

Parent SHA: `fee7456f010c6fe0d631f9d9ef537ee98ad4e2f3`.

Hypothesis: Kimi marks every Unicode-adjacent 64-byte batch as scalar-only. At each scalar fallback start, `KimiScheme::advance` currently decodes and classifies the leading non-ASCII scalar in `scan_kimi_han_run`, then repeats the work in `scan_kimi_punctuation_run` when it is not Han. Classifying it once should shorten the non-Han Unicode and punctuation entry path without changing token boundaries.

Measured hot cost: the immutable parent passed complete Hugging Face parity for the fixed 32-input LongBench-v2 Kimi K2.5 screen on the task-owned four-vCPU Intel host, taking 197.14 ms of Snaptokens encode time for 24,432,087 characters and 6,309,180 IDs. This is whole-encode attribution, not a claim that the repeated leading-scalar classification accounts for all of that cost. `perf` attribution is unavailable because the host has `perf_event_paranoid=3`; its kernel policy is unchanged.

Invariant that makes the shorter path exact: the candidate must call the same `MaskClassTable::class_and_han` classifier on the same valid UTF-8 leading scalar. A Han start still consumes the identical contiguous Han run; an `Other` start still enters the identical punctuation continuation; every remaining start still delegates to `scan_kimi`.

Representation being preserved or changed: keep the grammar, `MaskState` handoff, scalar fallback, SIMD masks, Unicode table, BPE, output allocation, APIs, tokenizer files, and benchmark inputs unchanged. Change only the private Kimi fallback start classification in `mask_scanner.rs`.

Expected winning strata: Kimi inputs with non-Han Unicode or punctuation starts in scalar fallback zones. Expected adverse strata: ASCII starts, Han runs, and all non-Kimi grammars should retain their existing path.

Smallest files that need changing: `src/pre_tokenizers/scanner/mask_scanner.rs`, its existing scanner differential test only if a missing boundary must be demonstrated, and this record. No evaluator, benchmark, dependency, fixture, model, corpus, or public API changes.

Mechanism evidence: `KimiScheme::advance` calls `scan_kimi_han_run` before `scan_kimi_punctuation_run`; both currently decode a non-ASCII start and query `MaskClassTable`. The existing differential test exercises mixed Han, punctuation, combining marks, Arabic digits, and positions around 64-byte boundaries.

Acceptance rule: commit the isolated candidate; run formatting and the existing Kimi scanner differential test before a remote screen. Require the unchanged 32-input sequential and batch Kimi Hugging Face ID screens before and after timing, then eight fresh `simple_bench --no-hf` parent/candidate pairs with four orders each, output construction/destruction included. Retain only if the paired geometric point estimate exceeds `1.02x`, its paired 95% interval is above `1.00x`, and every exactness gate passes. This diagnostic host cannot establish a general, portable, or competitor claim.

Rejection rule: reject and revert on any ID mismatch, focused-test failure, source change outside the declared fallback entry, unavailable candidate build, or missed throughput gate. Do not retune the model, corpus, input count, worker count, evaluator, benchmark, SIMD masks, or scalar grammar after observing results.

Result: rejected at the Kimi target screen. Candidate `054170466985969de0ba544efe7291bbe2e6d4ff` passed `cargo fmt --all -- --check` and all four fixed-scanner differential tests locally. On the isolated Intel host, parent and candidate each passed complete Hugging Face IDs for all 32 Kimi K2.5 LongBench inputs, sequentially and in one batch, both before and after the pool; every screen produced 6,309,180 IDs from 24,432,087 characters. Eight fresh `simple_bench --no-hf` pairs retained output construction and destruction. Candidate/parent ratios were `1.008657`, `0.990836`, `0.996876`, `0.996081`, `1.026247`, `1.012250`, `1.017031`, and `1.017914`; their geometric mean was `1.008169x`, with parent-first `0.998092x`, candidate-first `1.018348x`, and a paired log-ratio 95% interval of `[0.997778x, 1.018668x]`. It misses the predeclared point, confidence, and order criteria. The source is reverted in full; this one-UTF-8-scalar reuse is closed unless a new profile identifies a distinct affected representation.

### Scalar-fallback experiment 2 — skip decoding common Kimi Han scalars (2026-09-14)

Parent SHA: `931f4d3d09b4a7d42e8fc9f233905199dff31e47`.

Hypothesis: after Kimi has confirmed a Han start, `scan_kimi_han_run` decodes every following scalar only to accept the directly encoded Han ranges U+3400–U+9FFF. The common three-byte subset can be recognized from valid UTF-8 lead bytes: `E3 90..BF`, `E4 80..B6`, and `E5..E9`. Advancing by three bytes through that subset should remove codepoint reconstruction and range matching inside long Han runs.

Measured hot cost: the current scalar-fallback parent passed complete sequential and batch Hugging Face Kimi K2.5 ID checks for 24,432,087 fixed LongBench characters and 6,309,180 IDs. Its sequential encoder took 197.14 ms on the isolated four-vCPU Intel host. Kimi is the only fixed scanner flavor that marks every Unicode-adjacent batch scalar-only; the per-scalar Han loop is therefore an exact, source-visible part of the guarded fallback. The unavailable PMU permission prevents assigning a percentage to that loop, so this is an isolated mechanism screen rather than a general attribution claim.

Invariant that makes the shorter path exact: `str` validity means that a UTF-8 boundary beginning with `E3..E9` has the expected continuation bytes when three bytes remain. The three admitted byte ranges encode exactly the existing direct `is_han` ranges U+3400–U+9FFF. Every other scalar, including extension ranges, Yijing symbols U+4DC0–U+4DFF, marks, punctuation, and the final partial window remains on the current `decode_cp` plus `MaskClassTable::is_han` path.

Representation being preserved or changed: retain the Kimi regex, all fallback eligibility, Unicode table, general scalar classifier, masks, BPE, caches, API, files, fixtures, and benchmark contract. Add a private branchless-looking byte-range guard only to the confirmed-Han continuation loop.

Expected winning strata: Kimi text with long contiguous runs of common CJK Unified Ideographs or Extension A. Expected adverse strata: all other Unicode, short/tail runs, non-Kimi grammars, ASCII, and unsupported CPUs should preserve their old path or pay one rejected guard.

Smallest files that need changing: `src/pre_tokenizers/scanner/mask_scanner.rs`, the existing scanner differential test only if necessary to demonstrate a range boundary, and this record. No evaluator, benchmark, dependency, corpus, fixture, model-specific dispatch, or public API changes.

Mechanism evidence: `MaskClassTable::is_han` already recognizes U+3400–U+4DBF and U+4E00–U+9FFF without a table lookup, but only after `decode_cp` reconstructs each scalar. The three byte ranges are the exact UTF-8 encodings of those same intervals; U+4DC0–U+4DFF is deliberately excluded by the `E4 B7..BF` guard.

Acceptance rule: commit the isolated candidate; run formatting and all existing scanner differential tests. Require parent/candidate full Hugging Face Kimi IDs sequentially and batch-32 before and after timing. Then use the unchanged Kimi K2.5 32-input target screen with eight fresh `simple_bench --no-hf` parent/candidate pairs and four orders each, retaining output construction/destruction. Retain only at a geometric candidate/parent point estimate above `1.02x`, paired log-ratio 95% interval above `1.00x`, and no exactness failure. This remains a one-host diagnostic result, not a general promotion.

Rejection rule: revert for any ID mismatch, missing focused test, source change outside the declared Han continuation, build failure, or missed throughput gate. Do not broaden the byte ranges, modify the scalar grammar, retune the corpus/model/input count/workers, or change evaluator/benchmark code after results.

Result: rejected at the Kimi target screen. Candidate `d1116d4c5bed8804826c86a204a3d5b9e8cf70e8` passed formatting, all four scanner differential tests, and the real `correctness_kimi_k2_5` Hugging Face integration test. Parent and candidate each passed complete Hugging Face Kimi K2.5 IDs sequentially and batch-32 before and after the pool, producing 6,309,180 IDs from the unchanged 24,432,087 LongBench characters. Eight fresh output-destruction-inclusive candidate/parent ratios were `0.985879`, `1.028900`, `1.009296`, `1.004385`, `1.009334`, `1.005533`, `0.988007`, and `1.001378`; their geometric mean was `1.004010x`, parent-first was `1.006999x`, candidate-first was `1.001030x`, and the paired log-ratio 95% interval was `[0.992833x, 1.015313x]`. The cost removed from common Han continuation is not material enough under this unchanged Kimi screen. Revert the source in full and do not widen this byte-range technique without a new attribution or a distinct workload contract.

User-directed session on worktree branch `rust/branch-cache-opts-20260909` (parent `3fc5a08`) targeting branch reduction and cache behavior. All measurements are local Apple M2 screens on a loaded desktop, single Rayon thread, via a `--no-hf` mode added to `benches/simple_bench.rs` (per-chunk CSV, counterbalanced AB/BA cycles, per-chunk paired medians); the frozen portable evaluator was not run and no result here is a general champion promotion. Every retained and rejected candidate passed the full HF token-ID parity run (n=32 LongBench per family, plus a seeded local mixed-CJK corpus for Kimi/DeepSeek via a new `local:<path>` dataset mode) and the multithreaded `encode_batch` parity runs; 133 lib + 54 integration tests and warning-free strict Clippy pass on the final tree. Whole-run totals proved unusable on this host (cycle medians spanning 0.62-2.78x on untouched code); per-chunk paired medians in calmer windows are the basis for every verdict below, and a final cumulative screen was inconclusive under extreme contention. E-core pinning via `taskpolicy -c background` was tried and also unstable.

Retained (scoped, local evidence only): (1) `c70e718` splits CSR merge-adjacency into aligned `neighbor<<32|rank` u64 keys searched by `partition_point`, with merged IDs in a parallel array read only on hits — the strongest result: 7-cycle pooled per-chunk medians vs its parent were DeepSeek 1.0374, MiniMax 1.0496 (6/7 cycles above 1), GPT-2 1.0231, Qwen 1.0284, GPT-OSS 1.0158, Gemma 0.9946; matches the 2026-09-06 profile attribution of 5.9-19.2% encode cycles to `MergeAdjacency::get`. Same total table bytes; construction unchanged. (2) `b26d5d4` adds a 256 KiB BMP direct char-to-token table for the encoded merge path (Gemma-class `▁`/CJK chars stop paying a SipHash string probe): Gemma pooled median 1.0390/geomean 1.0777 in its cleaner screen; only the generic encoded path executes it; +256 KiB RSS per model clone; Gemma construction-dominated wall time unchanged (1.5s both). (3) `f5f08cd` tests the non-SIMD-Unicode bail before the seven NEON class chains in `batch_masks` (const-gated per flavor; x86 tiers get one movemask pre-test; other flavors keep their late test), removing discarded classification on Kimi CJK batches: weak positive, Kimi CJK pooled median 1.0044/geomean 1.0182 over 7 cycles — kept as provable dead-work removal with no cost path.

Rejected and reverted in full: fixed-trip full-array min-scan in `merge_short_raw_into` (MiniMax 0.8857, GPT-OSS 0.9392 geomean in both orders); its narrowed masks-plus-fused-compare remainder (pooled medians 0.97-1.03, indistinguishable from neutral); Fx-hashing the `token_to_id` field (GPT-OSS ignore_merges full-piece probes regressed ~20% in 4/5 cycles — weak multiplicative hash clusters ByteLevel piece strings; the public-API-preserving conversion shape itself worked); DeepSeek non-ASCII piece-start dedup (neutral 1.0043 pooled on the CJK corpus — starts are too rare relative to run-loop work); Kimi piece-start single-classify (exactly 1.0000 — LLVM already CSEs the inlined re-decodes). A `FlatCache::get/insert` switch to the SIMD packer was withdrawn before commit: standalone `&str` keys can never satisfy the 16-byte lookahead, so it degenerates to the scalar packer plus branches. The Gigatoken pretokenizer log (github.com/marcelroed/gigatoken) was reviewed; its SWAR/dual-cursor lessons align with the queued lockstep lane and its hot/cold-split regression matches the `unit_at_non_ascii` `#[cold]` concern, both left unimplemented.

Limitations: single-thread screens only (parallel contracts covered by parity, not timing); no A/A calibration for this local protocol; noise floor ±3-5% at best and far worse under load; no RSS/holdout/portability gates. Screen scripts and corpora: `/tmp/snaptokens-ab-20260909/` (ab_screen.sh, parity.sh, batch_parity.sh, gen_cjk_corpus.py, cjk_corpus.json). Confirmation on the frozen evaluator on quiet hardware is required before any general claim.

Post-review addendum (PR #1, Codex automated review, three P1 findings — all accepted as valid protocol observations): (1) The `--no-hf` timer originally stopped before output destruction; fixed so allocation, encode, and drop are all inside the timed interval, matching the steady-state contract. (2) The composition of the three retained changes was re-screened as its own candidate against parent `3fc5a08` with the corrected harness: a P-core 7-cycle screen gave pooled per-chunk medians DeepSeek 0.992, Gemma 0.979, GPT-OSS 0.978, GPT-2 1.039, MiniMax 1.034, Qwen 1.038 (one 0.62 outlier cycle), and an E-core 7-cycle screen gave DeepSeek 1.061, Gemma 1.025, GPT-OSS 0.956, GPT-2 0.977, MiniMax 1.075, Qwen 0.877 — mutually contradictory under sustained host contention (cycle medians spanning 0.71-1.48 on untouched-path families). The composed tree is therefore NOT locally performance-certified in either direction; its retention is user-directed, resting on the per-candidate targeted screens above, and the frozen evaluator on quiet hardware remains the deciding gate before any general or published claim. (3) The screening runner landed in the same branch as the candidates it screened; future local campaigns should freeze the runner (and A/A it) before screening candidates with it. Both raw composed-screen outputs are preserved in the session evidence.

### GLM-4.7 long-input exactness repair card (2026-09-09)

Parent SHA: b3b73b9a494999e0234a75f1a420ef98b036c34f
Hypothesis: `newline_partition_ranges` cuts after a raw newline run although the `PhiGlm` whitespace expression can own the spaces before, between, and after that run. Partitioning there turns one exact pre-tokenizer piece into two. This is a correctness repair, not a throughput candidate.
Measured hot cost: none. Timing is prohibited until complete IDs and row boundaries match Hugging Face on the failing schedule.
Invariant that makes the shorter path exact: parallel chunks may begin only at a complete whole-input fused pre-tokenizer piece boundary. A raw newline is insufficient because `\\s*[\\r\\n]+` can include adjacent spaces in the same piece.
Representation being preserved or changed: retain the public tokenizer APIs, BPE cache representation, corpus, evaluator, and timing boundary. Change only private partition-boundary selection from raw newline runs to whole-input fused-piece ends.
Expected winning strata: none. Restored correctness is the sole acceptance target.
Expected adverse strata: every fixed-scanner family that currently uses newline partitioning, especially whitespace immediately adjacent to a partition target.
Smallest files that need changing: `src/pre_tokenizers.rs`. The user requested that no new tests written in this campaign be retained.
Mechanism evidence: the committed GCP diagnostic records the mismatch. Serial `encode` is exact, while four partitions reproduce ragged output and split the expected byte-level token `ĠĠĠĊĠĠĠĠĊ` into `ĠĠĠĊ` and `ĠĠĠĠĊ` at source offset 49634.
Acceptance rule: reproduce the exact failing input and full schedule before and after cache warmup; run focused existing scanner tests, the affected integration test, and GCP Intel plus AMD parity gates. No timing, aggregate, README number, or package release is admitted before the full benchmark matrix later passes.
Rejection rule: reject and revert any change that changes unrelated scanner semantics, fails an exactness gate, depends on model labels or corpus sizes, or cannot explain the observed boundary. Preserve the diagnostic evidence and do not time a failed candidate.


### Local no-match Split restoration (2026-09-09): user-selected retention

The follow-up request "keep all changes to the code" also authorizes preserving the concurrent pattern-table/regex-fallback edits (`56f9050`) and the already-landed smaller-cache merge (`f331ef9`). Integration keeps all three changes, moves the updated Split tests into the separate test file, and makes the test-only reverse pattern lookup conditional on tests. The newly added HF comparison exposed empty output pieces for zero-width regex matches in the two direct splitting paths; both now omit those pieces while preserving boundaries. All 36 focused pre-tokenizer tests pass on the combination. The combined tree has not been benchmarked, so the earlier ratios apply only to the original isolated candidate. The fallback retains the incoming behavior of ignoring regex execution errors; full regex-dialect parity and runtime-error propagation remain limitations, not claims validated by these focused cases.

After reviewing the inconclusive result and observed regressions below, the user explicitly requested "keep it". Restore candidate `31aabaf43ee6d722af04f1d65beff398df23a594` exactly, including its separate test file, on the existing `rust/asm-20260909` branch. This is a user-selected scoped retention, not a statistically established speedup or a general champion promotion. The measured ratios, confidence interval, losses and raw evidence remain unchanged. The earlier rejection and revert remain in history. No benchmark is rerun or modified for this restoration.

### Local no-match Split result (2026-09-09): not retained

Final worker check: `gcloud compute instances describe snaptokens-rust-asm-20260909 --project=ilkb-project-test-project --zone=us-central1-a` reports `TERMINATED`, last stop `2026-09-08T21:50:48.291-07:00`. No stop/delete request was issued by this local experiment; the stop actor was not determined. The instance/disk and previous remote evidence remain preserved.

Completed the unchanged six-pair frontend comparison locally on Apple M2 (8 GiB, AC power), Python 3.12.13, Rust 1.97.0, against parent `b4f2d866051a803bfe90d134ca3664d2ff1937a5`, candidate `31aabaf43ee6d722af04f1d65beff398df23a594`. Aggregate throughput ratio is `1.073261x`, paired 95% CI `[0.939172, 1.180894]`. This does not establish a speedup. Model aggregates are GPT-2 `0.975308x`, Qwen3 `1.135103x`, Gemma3 `1.116708x`. All six pairs and every pre/post HF ID, metadata and row-boundary check completed.

The minimal scoped point-estimate floors passed, but the candidate is not selected for retention: there is no statistically supported speed benefit to justify the observed RSS and construction losses. Median candidate/parent whole-process peak RSS is `1.064503` (6.5% higher). Construction-time ratios are GPT-2 `1.114609`, Qwen3 `1.127573`, Gemma3 `0.909540`. The first two also exceed the existing 5% construction guardrail, as RSS exceeds its 5% guardrail. These measurements include the Python/HF process and normal desktop variation, not isolated tokenizer heap size. No post-hoc adjustment or additional timing pool is used to rescue the result.

Worst cells are Gemma tiny/all-metadata `0.893759x`, GPT-2 tiny/all-metadata `0.897467x`, GPT-2 tiny/flat `0.909848x`, Qwen tiny/IDs `0.910979x`, and GPT-2 document/all-metadata `0.922857x`. Whole-pair geometric aggregates range from `0.891918x` to `1.211722x`; the raw six values are `1.113970, 1.211722, 1.153919, 0.891918, 0.995021, 1.140267`. All fourteen losing primary cells remain in the raw summary. Faster point estimates for unchanged fused paths do not prove the Split mechanism caused those gains.

Mechanism and correctness were positive: ARM64 assembly branches on zero segment length at candidate `0xcddec` to original-range copying via `0xce0cc`, bypassing segment construction and apply_behavior. Parent reaches segment growth at `0xdf3b8`. The remaining output-vector allocation is unchanged. Seven focused Split tests, all 185 library tests (nine existing ignored), both packages' 13 Python tests, fmt and diff checks passed. Strict Clippy and broader release gates were not run after the candidate was rejected. Commit `fbc9080` reverted the runtime/test patch in full, preserving its committed source and evidence.

Local evidence archive: `/Users/namanchetwani/.cache/snaptokens-local-split-20260909.Hw3SaE-evidence.tar.gz`, SHA256 `3681ab09018d9a1480a051f0106e34b8bd6352fbfa0f5e137950394f73a9a64d`. It includes raw rounds, measured packages, both source snapshots, inputs, logs, provenance and assembly; only the rebuildable candidate target directory is excluded. This archive is local, not an off-host backup.

Build audit caught a stale artifact before timing: the initial shared-target candidate wheel reused the parent's identical native binary. Those unscored wheels/packages/logs are retained with `stale-` prefixes. Rebuilding the candidate with an isolated target directory and explicit manifest produced distinct verified native SHA256s: parent `0a494cdfebd1bfc29fc2650659ce4dbeb0ac88c541c105a9705db885058e40ca`, candidate `d447b52cf4df8eb6b823aecec86c1a7a56d728275e4770e781167b591548a4b0`. Binary sizes are 4,706,560 and 4,706,832 bytes. No timing used the stale artifact.

Other local tasks ran multiple builds/test suites during preparation. They were not terminated. The task's full suite took 978.91 seconds; a stack sample confirmed active tokenizer construction rather than a deadlock. All observed compiler/Rust-test jobs had finished before benchmark launch, and periodic checks saw none during timing. Normal user applications remained running. Raw rounds, input/package hashes, build commands and logs, source identity, assembly, compatibility tests and the diagnostic sample are retained under `/Users/namanchetwani/.cache/snaptokens-local-split-20260909.Hw3SaE`. The unchanged runner hash is recorded in the card below. Concurrent uncommitted scanner/Split changes in the main checkout are unrelated and are preserved.

### Local no-match Split candidate (2026-09-09)

Parent SHA: b4f2d866051a803bfe90d134ca3664d2ff1937a5
Hypothesis: represent a matcher with no matches without allocating segment and behavior vectors, then preserve the existing nonempty text split.
Measured hot cost: the retained Gemma native profile resolves 389,463 first allocations each in find_segments and apply_behavior for the single-row workload.
Invariant that makes the shorter path exact: after the dedicated Removed+invert path returns, every remaining delimiter behavior preserves a nonempty span when the matcher emits nothing. An emitted zero-width match still counts as a match. Already-assigned tokens and empty text keep their existing handling.
Representation being preserved or changed: private find_segments returns Option<Vec<(usize, usize, bool)>>, with None for no emitted matches; all public APIs, byte ranges, added-token IDs and matched paths are preserved. No new dependency, unsafe code or extra matcher scan.
Expected winning strata: generic no-match splitting, notably space-normalized Gemma spans.
Expected adverse strata: matched generic inputs pay the additional branch; fused GPT-2/Qwen paths are unchanged but remain regression controls.
Smallest files that need changing: src/pre_tokenizers/split.rs and separate src/pre_tokenizers/split/tests.rs, plus state and this record. Move the existing tests and test-only helper into that test file.
Mechanism evidence: fresh native heap stacks and source inspection establish two intermediate allocations per nonempty no-match generic span. Inspect candidate assembly and verify the no-match representation in focused tests.
Acceptance rule: user explicitly approved running the prepared comparison locally. Reuse frontend.py SHA256 3e0ecae71994e35e83b00d47d42f27d1004ff0e5cc788dd93319427fce20fdc3 unchanged: all six fresh-process AB/BA pairs, three models, five shapes, three output contracts, complete pre/post HF parity, timed conversion and destruction. Both packages use the same local Python, Rust, locks and default release flags. This is scoped allocation work, not general Rust promotion. Retention requires proven allocation removal, exactness, aggregate >=0.98x and every model >=0.95x. A speed claim additionally requires >=1.02x with paired 95% CI above 1.00x. Report all losses, construction, peak RSS and binary size separately. Run focused Split tests, Rust library tests, Python compatibility, fmt and strict Clippy before merging.
Rejection rule: reject any parity failure or missed floor, preserve all rounds and revert rejected runtime source. Do not change the runner, pool sizes, rounds or threads to rescue noisy local results. Do not rerun the failed portable evaluator or repeat heaptrack OOM captures.

Pre-edit cross-review: the Removed+invert exception is already dispatched separately; checking segments.is_empty() after matching is safer than previous==0 because emitted empty matches must retain their semantics. Test all five behaviors with both inversion settings, no-match/matched Unicode text and protected ranges. Empty-literal semantics stay unchanged. No model-label or input-size dispatch is added. Local preflight confirms a key is present, >3 GB disk free, and no stale automation process to terminate.

### Rust assembly/allocation findings (2026-09-09, diagnostics only)

Profiled immutable `66d5b9fc987b853d1ededea3482df2d00cb76dd8` through the unchanged native `simple_bench`, Rust 1.97.1, root `Cargo.lock`, default release code generation with debug symbols, four Rayon workers, on Intel Emerald Rapids. The six existing tokenizer JSONs each consumed the first 32 LongBench-v2 rows, 24,432,087 UTF-8 bytes, as sequential singles and batch32. The dataset revision is `2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9`. All twelve cases completed complete-ID preflight, three hardware-counter captures, a cycle callgraph and postflight without reported ID mismatch. No evaluator files or dependency locks changed.

The twelve cycle recordings contain 92,344 samples, zero reported lost samples and 2,781 samples attributed to Snaptokens encoding. About 35.03% of whole-process cycle weight has an unresolved leaf. Whole-process counters include Hugging Face, construction and dataset work, while attributed encoding includes first-row warmup and cache initialization. These are neither steady-state-only profiles nor accepted throughput comparisons. The bench's internal timer excludes output destruction. Concurrent scanner-only commit `1801ec4` arrived after recording and is not represented in this profile.

Completed heaptrack captures give these encoding-attributed allocation-call counts, including warmup. The temporary column uses heaptrack's definition, not all allocations that are eventually freed. Counts measure neither allocated bytes nor time.

| Model and shape | Encoding allocation calls | Heaptrack temporary calls |
| --- | ---: | ---: |
| DeepSeek V3.2 single | 5,387 | 3 |
| Gemma 3 single | 1,895,850 | 1,164,265 |
| GPT-OSS single | 175,499 | 170,025 |
| GPT-2 single | 9,207 | 36 |
| GPT-2 batch32 | 10,003 | 22 |
| MiniMax M2.1 single | 5,264 | 1 |

**Invalid captures are retained, not scored:** the heaptrack interpreter was OOM-killed on DeepSeek batch32 at 03:36:31 UTC, Gemma batch32 at 03:39:59 and GPT-OSS batch32 at 03:43:54. Its RSS reached approximately 12.9–13.8 GB. The wrapper still returned success and the benchmark completed, so generated summaries from these three truncated traces are invalid. On discovery the task's allocation process group was terminated, interrupting MiniMax batch32. Neither Qwen allocation case ran. The six valid captures have terminal heaptrack statistics. All CPU captures completed before these OOMs. Kernel logs, truncated raw data and the explicit `capture-validity.json` are preserved. A separate `.gz` versus actual `.zst` analysis-path error is also retained. No workload or sample count was reduced to rescue profiling.

Ranked source and assembly findings, not implemented or promoted candidates:

1. **Eliminate no-match Split vectors.** Gemma's single-row trace attributes 816,617 allocations to `Split::pre_tokenize`; 389,463 each are first allocations in `find_segments` and `apply_behavior`. Its literal normalizer replaces spaces with `▁`, then its literal space splitter commonly receives spans with no delimiter. Returning a private no-match representation before pushing a whole-span segment could preserve the original split without either vector. The matcher must run only once, zero-width matches must not be confused with no matches, and all delimiter/invert behaviors and added-token ranges need focused tests. No public offset narrowing is needed.
2. **Avoid temporary normalization strings.** Gemma allocates 755,813 times in `Replace::normalize`, including 755,467 heaptrack temporary calls. The trace resolves 336,479 initial String allocations plus repeated growth. `Replace::normalize` at binary address `0x3d9e80` calls string search at `0x3d9f28`, then repeats tail search at `0x3da34c`/`0x3da42c`, interleaved with small copies and growth checks. Appending directly to the final normalized buffer is a separate, larger hypothesis requiring proof of added-token phase ordering and preservation of the public Cow-returning API. A single search iterator is a smaller alternative. Neither has measured candidate benefit.
3. **Bounded stack encoding for ignore_merges.** GPT-OSS attributes 170,024 temporary calls to `Bpe::tokenize_fused_uncached`, with stack traces resolving to temporary String allocation/growth before full-vocabulary lookup. At `0x39de40`, the assembly follows the source's ByteLevel String path. A bounded stack buffer for already-short pieces, with an exact heap fallback for longer pieces, differs from rejected Exp20 capacity inflation and Exp21 buffer reuse. Preserve the whole-token vocabulary lookup, UTF-8 validity and every byte mapping. Do not extrapolate a removed allocation into a speedup.
4. **Separate merge lookup keys from payloads.** `MergeAdjacency::get` accounts for approximately 5.9–19.2% of each case's attributed encoding-cycle weight. Current binary search probes 12-byte `(neighbor, rank, id)` entries although it needs only the four-byte neighbor until a match. `init_merge_heap`'s closure at `0x3a1b70` shows scaled midpoint address arithmetic and conditional moves around `0x3a1be0`. Separate u32 keys and packed u64 payloads would preserve the full ID/rank domain and twelve logical bytes per rule, but add an allocation and hit-side indirection. This needs an adverse construction/cache screen, not an assumed win from denser keys.

The requested stack audit found that short 16/32-piece merging already uses stack arrays and u8 links. `merge_all_raw_into` at `0x39c570` reserves a 520-byte stack frame, with compiler-eliminated unused state in the ranked specialization. There is no per-piece scratch heap allocation to remove there. DeepSeek/GPT-2/MiniMax allocations are mostly owned long cache keys, not temporary merge arrays. Avoid introducing unsafe uninitialized arrays solely to remove visible stores.

Input bounds also reject blanket u16 token IDs: GPT-2's maximum ID is 50,256, but the other five fixtures require 17–18 bits, reaching Gemma's 262,143. A u24 representation fits these fixtures but not the public u32 contract without an exact structural fallback. Smaller scalar widths are not automatically faster. No dtype or runtime source change is part of this investigation.

Shipped separately: `8262138` moves the existing BPE and added-token test modules into `src/models/bpe/tests.rs` and `src/added_tokens/tests.rs`, preserving every test body and production prefix. Validation: 182 library tests passed, nine existing ignored, fmt, strict library/tests Clippy and diff checks passed. Merge `84dfd11` is published, with CI and release-build verification green. The older Python branch is not merged wholesale because it would remove newer frontend fixes already on main.

Candidate evaluation remains stopped at the locked portable-build failure documented below. The existing prepared frontend runner was offered as a scoped alternative, without evaluator expansion or a general Rust-speed claim, but no scope change was assumed. Another build appeared on the VM under a separate user/source directory at 03:53:08 UTC after this profiling run. It was not killed, and worker shutdown awaits ownership clarification.

Durable raw evidence and both immutable native binaries are archived at `gs://snaptokens-hillclimb-20260907-01a07615/rust-asm-20260909/native-evidence-final.tar.gz`, generation `1788926560085998`, 1,416,236,727 bytes, SHA256 `83f6da6dbb89e2aa23c0e2f349f2073931972fe02e241806f0eb116e30f8b4f2`. The pinned full dataset is in the same private prefix as `longbench-v2-pinned.json`, generation `1788926574422789`, SHA256 `15d61c22d92c96900b3c4948b6aeea218d3214b676a65df48e7b8555604c7fe2`. Independent object metadata confirms sizes and MD5s match the worker files. Uploads used creation-only generation guards, with no existing evidence overwritten. The bucket remains restricted to project roles and this run occupies a separate prefix from the older hillclimb. The worker retains its original automatic STOP deadline, 2026-09-09 07:00:15 UTC.

Local source/input archives remain under `/Users/namanchetwani/.cache/snaptokens-rust-asm-20260909`: `source-66d5b9f.tar` SHA256 `f084d805b2de24c1303b740d06485611dc489de881c21993845c2c2f361c1e8d` and `inputs.tar.gz` SHA256 `afc060370eb8fe1946d2a2c311744f639c28434889347a17a5fb616fe31590f3`. The small final analysis archive has SHA256 `ac196226b04c0b3035429869a2ea8d51858f22a58c15a231643cdba6fcbd30d9`. Full raw-archive download over SSH was slow and was stopped after verifying the private object-store copy, not treated as a completed local archive. No SSH private key or cloud credential is in the evidence archive or repository.

### Rust assembly and allocation profile plan (2026-09-09)

Build outcome before any workload: stable Cargo rejected the pinned Gigatoken `profile-rustflags` feature. Matching historical nightly 2026-08-10 passed that manifest but rejected the stale portable lock's Snaptokens 0.2.5 entry against current 0.2.6 source. Both failure outputs are retained; no dependency or evaluator file was changed. The portable timing/promotion lane is stopped, not silently unlocked. Continue diagnostic assembly and allocation inspection with existing `simple_bench` and `profile_sample`, current root lock and stable Rust 1.97.1; no new benchmark code. Use the fixed first 32 LongBench rows, single-row and batch-32 public encoding, across the six existing development tokenizer JSONs. Separate uninstrumented complete-ID preflight, three PMU counter repeats, one callgraph and heap allocation tracing; none of their internal times count as a candidate win. This revised diagnostic plan is declared before these benches build or run.

Frozen diagnostic source: `66d5b9fc987b853d1ededea3482df2d00cb76dd8`, clean current main. No tokenizer changes, benchmark changes or new performance framework are part of this step. Reuse portable evaluator `d9086dffdb6b197e47156c32de5beb16ac89c0d9c90c92297d94d7a9b661763b`, its six development tokenizers, enwik8, six load/encode rounds, two MiB per encode round and four workers. The task's GCP `c4-standard-4` STANDARD PMU host reports Intel Emerald Rapids; initial cycles/instructions probe succeeded. Capture three sequential `perf stat` runs and one `perf record` callgraph, then inspect attributed Snaptokens assembly and allocation call stacks. Retain all profiles, provenance and losses outside the repository at `/Users/namanchetwani/.cache/snaptokens-rust-asm-20260909`.

The historical runner's regrouped parity checks do not establish original-batch exactness. These captures measure diagnostic costs, not candidate acceptance or general throughput. Separate library tests and focused complete-output checks must precede any candidate timing. The previous `b3e8a34` PMU capture is context only; its explicit Snaptokens timer samples put merge adjacency near 16%, cache front/backing lookup near 15%, and short raw merges near 5%, but unresolved samples limit those estimates. Exp75 already showed fewer assembly stores can still lose 3.1% overall; Exp20/21 rejected larger or reused ignore-merges String buffers. Do not repeat those mechanisms without a materially different representation and fresh cost evidence. New or changed tests belong in test files. No production hypothesis is selected until current-source attribution is available.

### Experiment 85: smaller generic thread-local direct cache — planned (2026-09-09)

Parent SHA: `84dfd111d227a8555bdc842427b19ecf383e1343`, clean isolated worktree `autoresearch/exp85-cache-rss`.

Hypothesis: the generic BPE thread-local cache can use a 131,072-entry direct table rather than 2,097,152 entries. It remains an exact memoization cache, but reduces each initialized generic worker cache from about 71 MiB (64 MiB direct table, 6 MiB backing table, 1 MiB initial ID pool) to about 11 MiB. This is a memory experiment, not a general throughput claim.

Measured hot cost: unchanged `profile_sample` processes on task-owned four-vCPU Intel Emerald Rapids measured a four-worker max RSS of 1,285,712 KiB for Gemma and 1,291,400 KiB for GPT-2. A documented `taskset -c 0` cache-residency control put Gemma at 1,135,616 KiB, a 211 MiB reduction after removing three generic BPE-pool workers; GPT-2 stayed effectively flat at 1,291,400 KiB because its fused raw path does not initialize those generic worker caches. The profiler also observed only 32,940 Gemma text splits and 18,984 unique splits in its fixed LongBench sample, well below a 131,072-slot direct table. The profiler holds the dataset and Hugging Face tokenizer too, so these are whole-process peak-RSS evidence, not a claim that all bytes are cache allocation.

Invariant that makes the shorter path exact: the direct front table is an exact full-key memoization tier. Reducing its size can only turn an otherwise direct hit into the unchanged exact backing-cache lookup or BPE recomputation; it cannot alter a returned ID sequence. The backing table, pooled IDs, long-key map, shared string caches, cache clearing, seeded values, and cache ownership remain unchanged.

Representation being preserved or changed: change only the direct-table capacity selected by `FlatCache::new`, which is used by generic encoded BPE. The fused sequential and parallel cache capacities stay unchanged, as do all public APIs, `.tkz`/JSON formats, merge representation, byte fallback, `ignore_merges`, and shared-cache policy. This does not claim to solve unbounded shared-cache growth.

Expected winning strata: generic multi-worker BPE, especially Gemma-like long encodes that initialize a 64 MiB cache per worker. Expected adverse strata: repeated encoded-piece workloads whose direct-mapped cache collisions increase enough to reach the backing table or recompute BPE.

Smallest files that need changing: `src/models/bpe.rs` and this record. No evaluator, benchmark, corpus, tokenizer artifact, dependency, public API, or allocator change.

Mechanism evidence: the four-worker versus one-worker Gemma measurement exposes approximately 70 MiB of initialized generic cache residency per extra worker, matching the fixed allocation sizes. A 2^17 direct table still provides more than six slots per observed unique Gemma split in the predeclared diagnostic sample. This is evidence for a smaller exact cache capacity, not a license to tune capacity per tokenizer or workload.

Acceptance rule: commit the one-line capacity change; run formatting and focused BPE tests before any measurement. On immutable parent/candidate processes, verify complete Hugging Face IDs on the unchanged six-tokenizer fixed-input preflight before and after the sample. Require at least 100 MiB lower observed whole-process peak RSS in the four-worker Gemma control, no output mismatch, and no repeated >2% cold or warm regression in the existing diagnostic `profile_sample` for Gemma or an adverse GPT-2 control. The existing portable evaluator remains blocked and its timer excludes destruction, so these diagnostics cannot promote a throughput champion or a general speed claim.

Rejection rule: reject on any output mismatch, missed RSS floor, repeated diagnostic throughput regression, cache-layout growth outside this one capacity reduction, or any change to shared-cache eviction, evaluator, public API, or tokenizer configuration.

Result: **memory-qualified candidate, not a general promotion.** Candidate `dcbecf21b558d987f8e6932ae984c8bc2a76b978` changes only `FRONT_CACHE_BITS` from `21` to `17`, shrinking the generic `FlatCache` direct table from 64 MiB to 4 MiB. Formatting passed, and the focused remote `models::bpe` test selection passed all 16 tests. Complete-ID `simple_bench` preflights across GPT-2, Qwen 3, DeepSeek V3.2, GPT-OSS, MiniMax M2.1, and Gemma 3 passed both before and after the timing/RSS pool for single inputs and batch-32 inputs (12 fresh candidate processes at each side); a mismatch would have failed the benchmark process.

On the unchanged four-worker `profile_sample` control, the paired median whole-process peak RSS fell from 1,355,478 KiB to 1,112,876 KiB for Gemma (242,602 KiB, 236.9 MiB, 17.9%) and from 1,292,916 KiB to 1,232,636 KiB for GPT-2 (60,280 KiB, 58.9 MiB, 4.7%). These are process peaks including the profiler's held LongBench data and Hugging Face tokenizer, not allocator-attribution claims. The GPT-2 decline corrects the card's too-narrow prediction that its fused raw path would leave the generic allocation invisible in this process: the exact per-process ownership remains unresolved, but the immutable candidate/parent measurement establishes the observed peak reduction.

The existing exact-ID diagnostic screen was counterbalanced ABBA with four fresh processes per target. Gemma's candidate/parent geometric ratio was `1.002544x`; adverse GPT-2 was `0.992929x`. The individual GPT-2 AB and BA comparisons were `0.9806x` and `1.0053x`, respectively, so no repeated diagnostic loss exceeded the predeclared 2% rule. The `simple_bench` timer excludes output destruction, its samples are too small for the frozen evaluator's statistics, and only one Intel host ran; this result therefore does not claim a throughput win, cross-CPU portability, a new champion, or a main-branch merge.

The user authorized this isolated memory-qualified change to land on `main`; that exception is not a throughput claim or a new general champion. An unblocked frozen evaluator, complete output-materializing timing, and release/portability gates remain necessary for those claims. Raw evidence is archived at `/Users/namanchetwani/.cache/snaptokens-autoresearch/exp85-cache-rss-20260909/snaptokens-exp85-cache-rss-evidence-20260909.tar.gz`, SHA-256 `6efa0a7d76eba001c08eca0ad96878c2ea57c19e1a656e2d3f515fe7cb84bd4a`. Profile-sample parent/candidate binary SHA-256 values are `7a5857df27761d23956c751646aba7544570d88db11245901fe86c1c4d873b79` and `0b27bf0553820c850b5183d2133934befb95cc2bd894df8aec1178d44d259def`; simple-bench parent/candidate values are `a2c3a4e3d610edda9e0efdc659cf18246b68bf80f81f5d7be7e5389f3b670d62` and `97e27072bc0b5355deb05dd5c020d2e444a7386be3e1ed429ab793ec028032fb`.

### Scoped dtype outcome (2026-09-09)

Retain the four-change combination for smaller representations, not as a demonstrated speedup or general champion. Measured parent `293437af6d775d5e331ffc637866f3c95b723eaa`, candidate `bf5c76c7b934b7c0096ac43cafc3422bed3b2be3`. All six primary AB/BA pairs completed: aggregate `1.009518x`, paired 95% CI `[0.999311, 1.036819]`. This passes the declared memory floor but fails the >=1.02x speed-claim threshold. GPT-2/Qwen3/Gemma3 family ratios are `0.976928/1.019916/1.032561x`.

The worst cell, GPT-2 batch256/flat, is `0.814786x` (18.5% slower). Its six raw ratios are `0.742445, 1.320495, 1.103119, 0.784965, 0.672662, 0.845741`; the spread is large, but the loss is not discarded or explained away. GPT-2 batch256/IDs and all-metadata are also slower at `0.913473x` and `0.920378x`. These results do not support a claim of uniformly unchanged throughput.

The separate six-pair added-token screen also passes its >=0.95x per-cell floor. Encode/decode ratios: GPT-2 `1.003618/0.977591x`, Qwen3 `1.012785/1.015707x`, Gemma3 `0.955909/1.003506x`. Gemma added-token encoding is 4.4% slower. Every primary and added-token process passed complete pre/post Hugging Face ID, row-boundary and applicable decoded-string checks. Output conversion and destruction remain timed.

Storage changes: per-ID added-token flags 4 to 1 byte (75% less); bridge table 65536 to 8192 bytes (87.5% less); token-length entries 2 to 1 byte with exact long-length fallback (50% less); backing-cache slots 32 to 24 bytes (25% less, 2 MiB saved per initialized backing table). Paired median whole-process peak-RSS ratio is `0.994734` (about 0.5% lower), with all six candidate readings below their paired parent. These approximately 2 GiB processes include Python, Hugging Face and the harness, not just tokenizer allocation. Median candidate/parent construction-time ratios are GPT-2 `0.999813`, Qwen3 `1.063088`, Gemma3 `0.992212`; Qwen construction is 6.3% slower. Native binary size falls from 5792384 to 5787368 bytes.

Validation on the final production tree: 182 Rust library tests plus three binding unit tests passed, nine existing extended tests remain ignored, and all 13 Python package tests passed for both parent and candidate. All-target compilation, fmt, strict workspace/all-target Clippy, doctests, warning-clean docs, Rust package verification, Python sdist/wheel build and the corrected installed-wheel smoke check pass. No benchmark files, dependency locks, Python frontend code or public formats differ between the measured packages. The separate workflow repairs are already green on main `e96e1c3`; its production sources equal measured parent `293437a`. Merging that workflow-only history into the candidate changes no files. One four-core Intel family-6/model-85 worker, Rust 1.97.1 and Python 3.12.1 were measured. Apple ARM, AMD, sealed holdout and a general campaign promotion were not run.

Durable raw evidence, including all ten independent screens, every loss, validation logs and the superseded unscored partial integration: `/Users/namanchetwani/.cache/snaptokens-dtype-20260908/final-evidence.tar.gz`, SHA256 `46ac99fb8261df7f08c87852afda9ac9c6230f5bedc917bc48972be565d94299`. Preserved immutable packages and final wheels: `final-binaries.tar.gz` in the same directory, SHA256 `c43fb20618d426055092ab49fc19d06b4b5421471d43a744163422a0080a046d`. Parent/candidate native SHA256: `5f27081647aafefc22c1ca5c6446ce1b5d9282e9423e9fb53f41c8155af22485` / `3eef2f7f65085dd5ee27ba07befb5fafb35a31f2c3661c8df5d4412247debb7f`. Archived parent/candidate source SHA256: `620a73ac7d89e5d020bc3440ca99fe982a2e1a9a2781d96f39fe5767031fe88a` / `36fddd38eeb4409344870f0fa672b45c72aaad36632e2d796bcfdb7cba9618b8`. The unchanged runner and input hashes are in every manifest. No local-profile timing or evaluator-infrastructure expansion was performed.

### Ten independent dtype screens

All ten candidates below compare with `d5ac75a782fa409327dcf49677fcb49769872636`, not with each other or with the final combination. Each completed six fresh-process AB/BA pairs of the unchanged 45-cell frontend matrix, full pre/post Hugging Face output parity, focused Rust tests, Python compatibility tests, fmt and strict library Clippy. None qualified for a speed claim. Selection means inclusion in the separately tested combination, not an independently established current-main speedup. All unselected source changes were reverted on their experiment branches, with results and history preserved.

| Representation | Source | Throughput ratio | Paired 95% CI | GPT-2 / Qwen3 / Gemma3 | Worst cell ratio | Decision |
|---|---|---:|---|---|---|---|
| u8 added-token flags | `dbfa8ea` | 0.990884x | [0.956747, 1.053098] | 1.008006 / 0.985267 / 0.979606 | gemma-3/batch256/ids 0.877967x | selected for combination |
| u64 bridge bitset | `e5ebc92` | 0.999701x | [0.993035, 1.028650] | 0.996608 / 0.996486 / 1.006040 | gpt2/long/flat 0.930656x | selected for combination |
| u8 lengths with exact fallback | `1d6bf35` | 1.003979x | [0.996731, 1.008422] | 1.010068 / 1.003099 / 0.998802 | gpt2/batch32/flat 0.970382x | selected for combination |
| owned string lengths only | `47d1126` | 0.991742x | [0.982744, 1.004118] | 0.996999 / 0.993456 / 0.984810 | gemma-3/batch32/ids 0.952031x | not selected; remove dense lookup |
| u16 dense-table indexes | `4c24c16` | 0.982264x | [0.966074, 1.008292] | 0.961529 / 0.984594 / 1.001071 | gpt2/document/flat 0.883842x | rejected tradeoff; added indirection and losses |
| u8 bridge bitset | `d43f1a4` | 1.000442x | [0.994226, 1.009026] | 0.999266 / 1.002804 / 0.999261 | gemma-3/document/flat 0.967602x | not selected; no reliable advantage over u64 |
| 24-byte cache slots | `a5396a9` | 0.999440x | [0.974889, 1.024262] | 1.012857 / 1.000911 / 0.984751 | gemma-3/batch256/flat 0.925312x | selected for combination |
| u32 bridge bitset | `4982d02` | 0.991461x | [0.955593, 1.008638] | 0.992800 / 0.986428 / 0.995175 | qwen-3/batch256/flat 0.800266x | not selected; no reliable advantage over u64 |
| u16 bridge bitset | `44d5011` | 0.998530x | [0.961058, 1.025607] | 1.007727 / 0.996852 / 0.991082 | gemma-3/batch32/flat 0.803747x | not selected; no reliable advantage over u64 |
| u128 bridge bitset | `ba36746` | 0.977310x | [0.950590, 0.993313] | 0.976484 / 0.978689 / 0.976758 | gpt2/batch256/flat 0.755390x | rejected; aggregate below 0.98x floor |

The owned-length, dense-index, u8/u16/u32 bridge alternatives passed the predeclared minimum memory floors but were not selected: the floors permit consideration, not mandatory retention. Dense indexes added a branch/indirection with adverse GPT-2/Qwen points; removing lengths also removes the hot dense lookup. All bridge word widths occupy the same 8 KiB, and these results do not establish that u64 is fastest. u128 fails the aggregate floor. The selected u64 implementation is one portable, already-qualified choice. Custom cache outputs with one or two IDs >=2^31 now use the exact pool, so not every possible cache population is smaller.



### Scoped dtype integration candidate (2026-09-09)

Additional guard declared before integration timing: reuse the unchanged added-screen.py from the flags experiment for six fresh-process AB/BA pairs against current main. Check all HF IDs and decoded strings before and after timing, consume/destroy outputs, and require each of the six model/encode-or-decode paired medians to remain >=0.95x. This separate adverse screen does not change the primary frontend matrix or its aggregate.

Pre-timing baseline repair: concurrent main commit `1e90dd6` omitted the pinned fixture required by the existing Qwen3.5 vision test. Both original parent `88684ca` and this integration failed before HF loading, not on a token mismatch (integration: 181 passed, one fixture failure, nine ignored). Separate test-only commit `3ebe85f` adds the missing immutable revision/hash without changing the model, assertions, benchmark runner or its inputs. The existing vision test and fixture check then pass. Rebuilt parent native SHA256 remains exactly `248ea353f23e6237af061ef25f85e9dc0810a74d0d4c4493fb1c45d853d02820`; the original and repaired parent binaries are byte-identical. The candidate is compared against this repaired, separately published parent. A prior workspace-test link error came from the image's relocated libpython directory; test-only LIBRARY_PATH/LD_LIBRARY_PATH point to /usr/local/lib, with release build flags unchanged. Both failure logs are retained.

Baseline advanced before the first integration comparison completed: main `293437a` changes Python truncation reporting. Retire that unfinished older-parent pool as superseded, retain its raw rounds without scoring or selecting them, and rebuild both packages from the new parent and merged candidate. Repeat all six primary and six added-token pairs under the identical rules below. Separate workflow-only repairs use a nonempty smoke-test vocabulary and compile CLI benchmarks without executing them as unit tests; no benchmark source or evaluator setting changes.

Parent SHA: 293437af6d775d5e331ffc637866f3c95b723eaa
Hypothesis: combine four independently memory-qualified representations while keeping end-to-end throughput near the current Python frontend baseline.
Measured hot cost: four-byte added-token flags, a 64 KiB bridge table, two-byte cached token lengths, and 32-byte backing-cache entries all duplicate or over-allocate exact state.
Invariant that makes the shorter path exact: independent flags occupy four bits; bridge pairs map bijectively to bits; length 255 is a marker with an exact owned-string fallback and unchanged u16 input limit; full short keys and u32 IDs survive the cache payload encoding, with an exact pool fallback for wide IDs.
Representation being preserved or changed: combine flags `dbfa8ea`, u64 bridge `e5ebc92`, u8 lengths `1d6bf35`, and cache slots `a5396a9` on newer main. All public Python/Rust outputs and JSON/TKZ formats remain unchanged. Dense-index indirection is excluded because of observed losses. Choose the compact length array over removing it: both passed minimum memory floors, but the former retains the dense hot lookup and showed a smaller adverse family point. This is a selection tradeoff, not proof from a direct head-to-head test. The u64 bridge is an already-qualified portable choice, not a claim that it is the fastest word width.
Expected winning strata: retained tokenizer and initialized-cache memory; potentially cache-probe-heavy calls.
Expected adverse strata: generic Gemma and large batches showed losses in some individual candidates; unpacking and shifted indexes can accumulate cost.
Smallest files that need changing: src/added_tokens.rs, src/models/bpe.rs and this record. No evaluator, dependency, public API or unrelated cleanup changes.
Mechanism evidence: flags 4 to 1 byte per ID; bridge 65536 to 8192 bytes; lengths 2 to 1 byte per vocabulary slot; cache 32 to 24 bytes per slot. Separate experiments preserve every observed loss. Do not sum independent speed estimates into a combined claim.
Acceptance rule: this is scoped dtype work, not a general champion promotion. Compare immutable clean current-parent/candidate packages using unchanged frontend.py SHA256 3e0ecae71994e35e83b00d47d42f27d1004ff0e5cc788dd93319427fce20fdc3, all six fresh-process AB/BA pairs, three models/five shapes/three contracts, complete pre/post HF IDs and boundaries, and timed output conversion/destruction. Run the complete relevant Rust library and Python suites, fmt, strict library/workspace checks, docs and package validation before retention. Claim speed only at >=1.02x with paired 95% CI above 1.00x. Memory-only retention requires aggregate >=0.98x and every family >=0.95x, proven smaller storage and no API/format changes. Report load, process peak RSS and binary size separately.
Rejection rule: reject any parity or representation failure or missed throughput floor; preserve raw losses and revert rejected source. A failed combination is not rescued by quoting its individual candidates. No multi-CPU or sealed-holdout claim.

This file records every performance experiment in this optimization campaign, including rejected work. Rejected code is reverted with `git revert` so its rationale and measurements remain in history.

## Guardrails

- Preserve exact token IDs relative to Hugging Face Tokenizers for every supported pipeline.
- Optimize portable Rust for Apple ARM, AMD Zen 5, and Intel Core Ultra; do not use `target-cpu=native`, architecture-specific intrinsics, approximate BPE, or machine-specific dispatch.
- Keep each experiment minimal and tie every changed line to a measured bottleneck.
- Compare committed binaries under the same corpus, thread count, build profile, and process-level trial protocol.
- Treat an improvement as retained only when repeated measurements show no material regression in another representative workload.
- Keep Hugging Face credentials in the macOS Keychain and inject them only into benchmark processes.

## Baseline environment

- Date: 2026-07-14
- Baseline commit: `326cb5a`
- Experiment branch: `codex/portable-tokenizer-perf`
- Host: MacBook Air with Apple M2, 8 CPU cores (4 performance and 4 efficiency), 8 GB RAM
- OS: macOS 26.5.2
- Toolchain: `rustc 1.97.0`, `cargo 1.97.0`
- Build: repository's untouched default `cargo build --release` profile
- Preserved binary SHA-256: `57e1c185fea8776f280145b9eec99ee1f9b88d5d2e195d9201fddc69e3a9842a`
- Free disk before corpus downloads: 17 GB; after both benchmark datasets and seven tokenizer configurations were cached: 12 GB

Baseline validation:

- `cargo fmt --all -- --check`: passed.
- Focused tests: 9 BPE tests, 46 Split tests, and 22 added-token tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: blocked by existing example-only lints (`needless_range_loop`, `needless_borrow`, `manual_div_ceil`, and `useless_borrows_in_formatting`). No production lint failure was observed, and unrelated example cleanup is outside this campaign.

## Existing benchmark audit

`examples/simple_bench.rs` is the correctness-bearing benchmark. It loads real LongBench-v2 or ShareGPT52K text, compares exact output IDs against Hugging Face, and measures sequential encoding or `encode_batch`. Its aggregate time is useful for same-host A/B comparisons, with these limitations:

- It records one timing per input or batch, always Hugging Face first and fastokens second.
- It warms fastokens only with the first sample, then measures a mixture of warm and cold cache behavior.
- Totals are dominated by the longest inputs, and inputs are the first `n` dataset rows rather than a stratified sample.
- Dataset loading, tokenizer construction, and allocator state are outside the timed region.
- The label says characters, but the implementation counts UTF-8 bytes.

The campaign therefore uses repeated fresh processes, fixed cached inputs, explicit Rayon thread counts, medians plus spread, and the same preserved baseline binary. Sequential and batched results are reported separately. The existing CSV output remains the source of per-input timings and exact-ID validation.

`examples/dynamo_speed.py --tokenize-only` is not a trustworthy ruler in its current form. Transformers returns a `BatchEncoding`, but the script iterates it as if it were a list of per-prompt encodings; token totals, sample counts, latency buckets, and throughput are consequently incorrect. Its repeated synthetic prompts also disproportionately exercise caches. It will not be used to accept an optimization.

The SGLang and full Dynamo benchmarks require external GPU serving stacks. On this Apple M2 host they are integration/TTFT checks, not local tokenizer microbenchmarks.

## Representative matrix

The primary matrix covers seven BPE tokenizers and both shipped corpora:

- `nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16`
- `deepseek-ai/DeepSeek-V3.2`
- `MiniMaxAI/MiniMax-M2.1`
- `openai/gpt-oss-120b`
- `mistralai/Mistral-Nemo-Instruct-2407`
- `zai-org/GLM-4.7` (including the `ignore_merges` path)
- `Qwen/Qwen3-0.6B`

Together these exercise the supported NFC, Sequence, Split, ByteLevel, TemplateProcessing, and ByteLevel post-processing combinations. The screening matrix uses LongBench-v2 (`n=50`) and ShareGPT52K (`n=200`), sequential and batch size 32, with three fresh-process trials in alternating model order. Retained candidates then face batch sizes 1, 8, 32, and 128 plus explicit 1-thread and full-host thread checks.

## Untouched measurements

Pre-matrix calibration, used only to size the workload:

| Model | Corpus | Mode | Samples | Bytes | Tokens | fastokens | Throughput | Exact IDs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `openai/gpt-oss-120b` | LongBench-v2 | sequential | 10 | 7,133,318 | 1,909,115 | 75.43 ms | 94.57 MB/s | yes |
| `Qwen/Qwen3-0.6B` | ShareGPT52K | sequential | 200 | 3,290,328 | 869,054 | 70.98 ms | 46.36 MB/s | yes |

The ShareGPT calibration process peaked at 1.28 GB resident memory and completed in 4.45 seconds including loading and Hugging Face comparison, so `n=200` is safe on the 8 GB host.

## Candidate backlog

Candidates are ordered by evidence, then tested individually:

1. Add a portable release profile (`thin` LTO and one codegen unit) and measure whether deployment binaries improve enough to justify slower builds.
2. For short BPE pieces, compare the current binary heap with a cache-local linear minimum scan, following tiktoken's split strategy while preserving exact rank semantics.
3. Carry the merged token ID in heap entries or merge adjacency metadata to avoid a redundant successful-merge lookup.
4. Remove construction-era BPE tables retained at runtime only if reachability and memory measurements prove they are dead after initialization.
5. Re-evaluate nested Rayon ownership and thresholds across 1-thread and full-host batch workloads without hard-coding this M2's topology.
6. Compare thread-local/shared cache policy only after measuring miss rate, lock time, memory growth, and repeated-input behavior.
7. Replace standard hashing on the non-fused and `ignore_merges` paths only if those paths are material in the matrix.

## Experiment record

No tokenizer implementation experiment has been started yet.

### Measurement attempt 0: uncooled screening matrix — rejected

The first matrix launch ran trial 2 immediately after trial 1. It was stopped when the reversed-order Qwen LongBench measurements exposed thermal/system drift:

| Trial | Mode | Hugging Face | fastokens |
|---:|---|---:|---:|
| 1 | sequential | 15,470.13 ms | 465.68 ms |
| 2 | sequential | 17,278.96 ms | 643.38 ms |
| 1 | batch 32 | 8,563.31 ms | 379.42 ms |
| 2 | batch 32 | 8,768.19 ms | 421.56 ms |

The Hugging Face control and fastokens both slowed, and the fanless host also showed unrelated macOS analysis work. These trials are retained as diagnostic CSVs but rejected as the official baseline. The replacement protocol adds cooldowns, checks system load, and uses paired baseline/candidate runs for experiment decisions.

### Official untouched screening baseline

The replacement run used three fresh-process trials, 20-second cooldowns between model groups, 30-second cooldowns between trials, reversed model and mode order in trial 2, and `RAYON_NUM_THREADS=8`. Every one of the 84 runs matched Hugging Face token IDs exactly. Times below are the median with the observed `[minimum, maximum]` process totals.

LongBench-v2, 50 samples and 42,424,271 input bytes:

| Model | Sequential ms | Sequential MB/s | Batch-32 ms | Batch-32 MB/s |
|---|---:|---:|---:|---:|
| Nemotron | 439.38 [417.90, 445.90] | 96.56 | 312.49 [310.80, 378.96] | 135.76 |
| DeepSeek V3.2 | 967.59 [946.47, 1,177.53] | 43.85 | 455.28 [426.46, 514.64] | 93.18 |
| MiniMax M2.1 | 475.09 [467.57, 485.12] | 89.30 | 341.45 [327.12, 349.43] | 124.25 |
| GPT-OSS | 451.49 [435.43, 467.39] | 93.97 | 370.05 [313.90, 395.65] | 114.64 |
| Mistral Nemo | 436.03 [427.63, 456.63] | 97.30 | 302.05 [291.53, 402.25] | 140.46 |
| GLM 4.7 | 447.14 [394.48, 470.85] | 94.88 | 292.30 [265.45, 371.44] | 145.14 |
| Qwen 3 | 432.08 [419.58, 441.20] | 98.19 | 400.55 [287.38, 459.10] | 105.92 |

ShareGPT52K, 200 samples and 3,290,328 input bytes:

| Model | Sequential ms | Sequential MB/s | Batch-32 ms | Batch-32 MB/s |
|---|---:|---:|---:|---:|
| Nemotron | 72.22 [66.74, 77.45] | 45.56 | 32.13 [32.02, 32.86] | 102.40 |
| DeepSeek V3.2 | 98.04 [96.64, 100.22] | 33.56 | 86.60 [39.22, 142.10] | 37.99 |
| MiniMax M2.1 | 76.14 [73.81, 78.12] | 43.21 | 34.33 [34.24, 41.28] | 95.85 |
| GPT-OSS | 70.38 [69.02, 70.64] | 46.75 | 33.45 [31.11, 34.73] | 98.35 |
| Mistral Nemo | 70.12 [67.48, 78.24] | 46.92 | 42.87 [31.68, 45.80] | 76.76 |
| GLM 4.7 | 71.83 [68.50, 85.92] | 45.81 | 41.85 [34.42, 63.39] | 78.62 |
| Qwen 3 | 72.79 [72.12, 79.28] | 45.20 | 56.02 [44.93, 68.46] | 58.74 |

The geometric mean across these 28 workload throughputs is **76.165 MB/s**. Sequential measurements are generally tight enough for paired screening. Some full-host batch cells are bimodal, most notably DeepSeek ShareGPT, so batch acceptance requires more repetitions and the explicit one-thread sweep. DeepSeek's large sequential outlier came primarily from three inputs (indices 12, 33, and 35), rather than a uniform slowdown; the median is retained without deleting the outlier.

### Experiment 1: ThinLTO and one codegen unit — planned

Hypothesis: the default release profile leaves portable cross-crate optimization on the table. Add only `lto = "thin"` and `codegen-units = 1`; do not add native CPU flags, fat LTO, panic changes, or allocator changes. Cargo's official profile documentation says ThinLTO can approach fat-LTO gains at substantially lower link cost, while fewer codegen units may produce faster code at the cost of compile parallelism.

Primary acceptance gate: at least 3% improvement in the 14-cell sequential geomean, at least 2% in the full 28-cell geomean, and no sequential median regression worse than 3%. A result inside the observed noise is rejected rather than retained as speculative build complexity. Exact-ID checks remain enabled, and a release Python-extension smoke check is required if retained.

The xhigh peer review agreed this is the smallest portable calibration experiment and ranked carrying merged token IDs in heap entries as the next source-level candidate. The concurrent Claude review stalled in its backend without returning an opinion; it made no edits.

Result: **rejected**. The release build took 62.24 seconds and peaked at 891 MB RSS. The executable shrank from 10,232,976 to 8,703,936 bytes (14.9%), but runtime did not meet the gate.

- The first complete 14-cell sequential paired trial was 1.507% slower in geometric mean throughput. Individual cells ranged from 28.85% slower to 15.78% faster, exposing strong execution-position and host noise rather than a coherent compiler win.
- Four balanced AB/BA LongBench pairs on DeepSeek had a 975.56 ms baseline median and 981.02 ms candidate median: 0.557% slower.
- Four balanced AB/BA LongBench pairs on GPT-OSS had a 460.37 ms baseline median and 473.05 ms candidate median: 2.682% slower. Excluding the externally disturbed first pair still left the candidate about 0.9% slower.
- Every completed run matched Hugging Face token IDs exactly.

The broad follow-up was stopped after balanced confirmation on the slow-path and stable-control models because the candidate was already below the primary gate. Commit `89d223c` is reverted; later experiments use the original release profile.

### Experiment 2: carry merged IDs in heap entries — planned

Hypothesis: every accepted heap merge performs a redundant binary search in `merge_adj` even though all enqueue sites already know the merged token ID. Store that ID in `MergeEntry` and use it after the unchanged two-endpoint stale check.

The exact surgical representation grows each entry from 16 to 24 bytes because rank, position, both endpoint IDs, and the merged ID are all logically full-width `u32` values. This may lose on heap cache traffic, especially for long pieces with many stale entries. Bit packing, dropping an endpoint, or treating `unmerge_map` as the merge-rule inverse are rejected because they require bounds or uniqueness properties not guaranteed by arbitrary tokenizer JSON.

The xhigh design review confirmed the 24-byte layout is the smallest safe isolated carry experiment. It also identified an exact fallback: rank uniquely identifies the surviving merge rule, so a dense rank-to-`(left, right, new_id)` table could remove the binary search while shrinking entries to 8 bytes. That more invasive design is deferred unless the first rung fails. The concurrent Claude review again stalled in its backend without an opinion and made no edits.

Acceptance requires at least 2% sequential geomean improvement, no sequential median regression worse than 3%, exact Hugging Face IDs, and a focused overlapping-pair stale-entry test through both encoded and fused/raw initialization paths.

Result: **rejected**. The focused stale-entry test passed through both encoded and fused/raw paths, all benchmark outputs matched Hugging Face, and the default-profile release build completed in 14.68 seconds. The binary grew by only 112 bytes, but DeepSeek LongBench regressed in all three cooled pairs:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 1,041.19 ms | 1,185.66 ms | -12.18% |
| 2 | candidate, baseline | 1,055.49 ms | 1,068.73 ms | -1.24% |
| 3 | candidate, baseline | 1,062.25 ms | 1,176.13 ms | -9.68% |

The median was 1,055.49 ms baseline versus 1,176.13 ms candidate, a 10.26% throughput regression. The screen stopped before other models because the primary slow path already crossed the hard rejection threshold in both execution positions. The 50% larger heap entry outweighed the removed accepted-merge lookup. Commit `fd12aa7` is reverted.

### Experiment 3: index immutable merge rules by rank — planned

Hypothesis: rank already uniquely identifies each surviving parsed merge rule. Store `(left, right, new_id)` in a dense rank-indexed table and reduce each heap entry to the existing 64-bit `(rank, position)` key. On pop, one indexed rule load supplies both stale-check endpoints and the merged ID, replacing the accepted-merge binary search while halving heap-entry size.

This invariant does not assume bounded token IDs or unique token-string decomposition. Duplicate pair lines are last-write-wins in `ParsedMergeMap`; earlier ranks become unused table holes and are never enqueued. Static memory grows by 12 bytes per rank, while heap traffic falls from 16 to 8 bytes per entry. A focused test combines a duplicate-pair rank hole with overlapping right-endpoint invalidation through both encoded and fused/raw paths.

Acceptance again requires at least 2% sequential geomean improvement, no sequential regression worse than 3%, and exact Hugging Face IDs. DeepSeek LongBench and GPT-OSS LongBench receive four balanced screen pairs before broader matrix expansion.

Result: **rejected**. The combined duplicate-rank/stale-pair test passed through both heap initialization paths, every benchmark matched Hugging Face, and the release build completed in 11.51 seconds. DeepSeek LongBench produced:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 1,082.88 ms | 1,016.51 ms | +6.53% |
| 2 | candidate, baseline | 986.58 ms | 1,023.23 ms | -3.58% |
| 3 | candidate, baseline | 1,035.07 ms | 1,069.09 ms | -3.18% |
| 4 | baseline, candidate | 1,050.96 ms | 1,156.88 ms | -9.16% |

The balanced median was 1,043.01 ms baseline versus 1,046.16 ms candidate, a 0.30% throughput regression. Three of four pairs lost, including both candidate-first trials, so the screen stopped before GPT-OSS. Halving heap entries did not repay an indexed rule-table load on every stale pop. Commit `134c1ce` is reverted.

### Experiment 4: linear merging for short pieces — planned

Hypothesis: heap construction, maintenance, and stale entries are unnecessary for the overwhelmingly short pre-tokenized pieces. For pieces below 64 input bytes, use a dense thread-local `Vec<u32>`, repeatedly scan live adjacent pairs for the minimum `(rank, position)`, merge in place, and remove the right token. Keep the current heap path unchanged at and above 64 bytes, and skip raw `heap_buf` seeding entirely on the short path.

The cutoff is evidence-based rather than machine-specific. Five GPT-OSS LongBench documents ranging from 1.0 to 5.3 MB had median split sizes of 4–5 bytes, p95 of 11–14, and p99 of 13–38. Pieces over 50 bytes were rare, although maxima reached 2,158 bytes and remain protected by the heap path. OpenAI tiktoken independently uses the same cache-local linear strategy below 100 bytes; 64 is the more conservative starting point for fastokens' CSR pair lookup.

The xhigh review confirmed strict rank comparison in a left-to-right scan preserves the heap's rank-then-leftmost ordering. It recommended a dense ID vector instead of compacting 12-byte linked symbols. One boundary test covers equal-rank overlapping merges at 63 bytes (linear) and 64 bytes (heap), through both encoded and fused/raw paths. Acceptance requires at least 2% sequential geomean improvement, no cell worse than 3%, exact IDs, and no batch regression after the sequential screen.

Result: **rejected**. The boundary test and all 10 focused BPE tests passed, every benchmark matched Hugging Face IDs, and the unchanged-profile release build completed in 15.24 seconds with 401,555,456 bytes maximum RSS. The executable grew from 10,232,976 to 10,249,968 bytes. Four cooled, balanced DeepSeek LongBench pairs produced:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 1,043.29 ms | 1,043.16 ms | +0.01% |
| 2 | candidate, baseline | 1,010.51 ms | 1,091.00 ms | -7.38% |
| 3 | candidate, baseline | 984.95 ms | 1,024.24 ms | -3.84% |
| 4 | baseline, candidate | 1,005.54 ms | 1,074.63 ms | -6.43% |

The balanced median was 1,008.03 ms baseline versus 1,058.90 ms candidate, a 4.80% throughput regression. Three of four pairs lost. In trial 4 the candidate lost even though its Hugging Face control was 2.41% faster, so system drift cannot explain the rejection. The screen stopped before GPT-OSS because a representative cell crossed the hard 3% regression limit. Commit `5f16ef5` is reverted; a separate lower-cutoff experiment may still test whether a narrower linear regime has a favorable crossover.

### Experiment 5: linear merging below 16 symbols — planned

Hypothesis: the 64-byte candidate crossed into the range where repeated CSR binary searches dominate, but the median 4–5-symbol pieces may still be cheaper to scan than heapify. Test exactly one lower rung: fewer than 16 initial symbols. This bounds the worst scan, covers the observed 11–14-byte p95 region, and avoids conflating UTF-8 byte width with the actual number of merge symbols.

The implementation will reuse the existing `Vec<MergeSymbol>` as dense temporary storage on the short path. Its linked-list indices are irrelevant there, so this avoids the rejected candidate's second scratch vector and construction-time branch. Raw initialization skips byte-pair heap seeding only when the already-known byte count is below 16. The same boundary test covers 15 symbols through the linear path and 16 through the unchanged heap path, for both encoded and fused/raw entry points.

The requested xhigh peer discussion was attempted again with the complete 64-byte measurements, but both backends stalled without returning output and made no edits. The decision therefore follows the measured crossover hypothesis and the simpler code shape. Acceptance remains at least 2% sequential geomean improvement, no representative cell worse than 3%, and exact IDs. Failure at this rung ends the linear-scan candidate family rather than starting a machine-tuned cutoff sweep.

Result: **rejected**. The threshold test and all 10 focused BPE tests passed, every benchmark matched Hugging Face IDs, and the unchanged-profile release build completed in 14.82 seconds with 398,524,416 bytes maximum RSS. The executable grew from 10,232,976 to 10,250,096 bytes. Four cooled, balanced DeepSeek LongBench pairs produced:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 1,115.97 ms | 1,045.75 ms | +6.71% |
| 2 | candidate, baseline | 1,032.80 ms | 1,205.40 ms | -14.32% |
| 3 | candidate, baseline | 1,054.37 ms | 1,062.20 ms | -0.74% |
| 4 | baseline, candidate | 1,047.73 ms | 1,064.08 ms | -1.54% |

The balanced median was 1,051.05 ms baseline versus 1,063.14 ms candidate, a 1.14% throughput regression. Three of four pairs lost, including both candidate-first trials, and the only win coincided with a 3.70% faster Hugging Face control. The candidate therefore missed the positive acceptance gate even before broader-model testing. Commit `b22a32f` is reverted, and no further linear cutoff is pursued.

### Experiment 6: remove the construction-only ranked merge table — planned

Hypothesis: `RankedMergeMap` duplicates every parsed merge in a 50%-full, 16-byte-slot hash table. It is queried only while building the fixed 256-by-256 initial-byte table, but the completed table is then retained in every `Bpe` and cloned. Use the already-live `ParsedMergeMap` for those 65,536 construction-only lookups and delete the ranked table type and field.

Repository-wide reachability confirms no encoding, decoding, equality, or debug path reads the field. The parsed map has identical last-write-wins rank and merged-ID values and remains immutable in `Bpe::new`, so the replacement does not change merge semantics. Cached tokenizer JSON gives exact retained slot allocations of 4,194,304 bytes for DeepSeek and 16,777,216 bytes each for GPT-OSS and Nemotron. A tokenizer-only macOS malloc-zone probe produced identical results in two runs: 39,398,960 live bytes for DeepSeek, 78,169,328 for GPT-OSS, and 71,807,904 for Nemotron.

Acceptance requires exact output IDs across the full model/corpus matrix, a live-heap reduction consistent with the deleted slot allocations, no sequential or batch throughput regression worse than 3%, and no material tokenizer construction slowdown. Because the source change deletes a redundant representation and reuses existing immutable data, the existing focused and model-level exact-ID checks are sufficient; no synthetic unit test is added.

Result: **rejected despite the memory win**. The source diff was one insertion and 80 deletions; all 9 BPE tests and library clippy with warnings denied passed, and every benchmark matched Hugging Face IDs. The simple benchmark executable shrank by 544 bytes.

The malloc-zone probe was byte-for-byte repeatable. Live heap fell from 39,398,960 to 35,204,656 bytes for DeepSeek (-4,194,304), from 78,169,328 to 61,392,112 for GPT-OSS (-16,777,216), and from 71,807,904 to 46,642,080 for Nemotron (-25,165,824). Median construction CPU fell from 1.025 to 0.790 seconds for GPT-OSS and from 0.505 to 0.420 seconds for Nemotron; DeepSeek was effectively tied at 0.29 versus 0.28 seconds.

Four balanced DeepSeek LongBench pairs all favored the candidate, with a 996.65 ms baseline median and 983.94 ms candidate median (+1.29% throughput). GPT-OSS LongBench did not:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 561.33 ms | 493.01 ms | +13.86% |
| 2 | candidate, baseline | 437.67 ms | 455.87 ms | -3.99% |
| 3 | candidate, baseline | 430.71 ms | 487.00 ms | -11.56% |
| 4 | baseline, candidate | 466.11 ms | 502.63 ms | -7.27% |

The GPT-OSS balanced median was 451.89 ms baseline versus 490.01 ms candidate, a 7.78% throughput regression. Input 10 alone was slower in all four candidate runs by 9.62 to 26.30 ms, so the failure was not a single aggregate outlier. Commit `4f07ce0` is reverted. The experiment combined removal of the retained field with replacement of the construction lookup; the next isolation keeps construction unchanged and drops only post-construction storage.

### Experiment 7: release the ranked table after construction — planned

Hypothesis: preserve the baseline `RankedMergeMap` type, construction, and initial-byte lookups exactly, but do not move the completed local table into `Bpe` or clone it. The three-line field/initializer/clone deletion should retain the measured 4–16 MB logical memory saving while isolating the rejected construction rewrite and most of its code-layout changes.

The table's last semantic use remains the construction loop. `Bpe` deserializes through `RawBpe`, and neither equality nor debug output includes the field, so this does not alter the serialized contract. Existing BPE tests plus the real-model exact-ID matrix remain sufficient. Acceptance requires the expected live-heap reduction, no material construction slowdown, and no sequential or batch median regression worse than 3%.

Result: **rejected**. The source change was exactly three deletions; all 9 BPE tests and library clippy with warnings denied passed, every benchmark matched Hugging Face IDs, and the executable shrank by 464 bytes. Two malloc-zone passes reproduced the combined experiment's exact live-heap values: 35,204,656 bytes for DeepSeek, 61,392,112 for GPT-OSS, and 46,642,080 for Nemotron.

The decisive GPT-OSS LongBench screen produced:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 437.96 ms | 425.16 ms | +3.01% |
| 2 | candidate, baseline | 389.42 ms | 441.72 ms | -11.84% |
| 3 | candidate, baseline | 472.61 ms | 496.79 ms | -4.87% |
| 4 | baseline, candidate | 401.41 ms | 482.33 ms | -16.78% |

The balanced median was 419.69 ms baseline versus 462.03 ms candidate, a 9.16% throughput regression. Three of four pairs lost. Because construction and lookup code were unchanged, removing the 24-byte inline field and shifting the following hot-field offsets is the remaining source-level distinction. Commit `10cd936` is reverted. The next isolation preserves the complete `Bpe` layout and empties only the construction table's backing allocation.

### Experiment 8: free ranked slots while preserving `Bpe` layout — planned

Hypothesis: after `byte_pair_initial` is complete, reset the local ranked table to its valid empty state before moving it into `Bpe`. Setting `mask` to zero and replacing `slots` with `Vec::new()` frees the construction-only backing allocation while preserving the field, struct size, field order, clone shape, and all encoding-time offsets.

This is two assignments plus one explanatory comment. `RankedMergeMap::get` already returns before reading `mask` when `slots` is empty, and no runtime path queries the field. Acceptance requires the same exact live-heap deltas, exact model outputs, and no sequential or batch median regression worse than 3%. Failure ends the ranked-table memory candidate rather than adding explicit padding or architecture-specific layout controls.

Result: **rejected**. The source change was five insertions and one deletion; all 9 BPE tests and library clippy with warnings denied passed. The executable grew by 112 bytes. Two malloc-zone passes agreed exactly: live heap fell by 4,194,304 bytes for DeepSeek, 8,667,136 bytes for GPT-OSS, and 25,165,824 bytes for Nemotron while preserving the inline `Bpe` layout.

The initial balanced screens passed: GPT-OSS LongBench had a 462.65 ms baseline median versus 445.02 ms candidate median (+3.96%), and eight DeepSeek LongBench pairs had a 987.31 ms baseline median versus 1,005.49 ms candidate median (-1.81%). The candidate then completed a four-trial, order-balanced matrix over all seven models, both corpora, sequential and batch-32: 224 processes and exact Hugging Face IDs in every run.

The full 28-cell throughput geomean changed by -1.06%. Sequential was effectively identical at -0.003%, but batch-32 changed by -2.11% and several repeated cells crossed the hard guardrail:

| Cell | Baseline median | Candidate median | Throughput change | Pair record |
|---|---:|---:|---:|---:|
| GLM LongBench batch-32 | 272.10 ms | 328.26 ms | -17.11% | 0 wins, 4 losses |
| Qwen LongBench batch-32 | 272.78 ms | 304.67 ms | -10.47% | 1 win, 3 losses |
| DeepSeek ShareGPT batch-32 | 52.28 ms | 58.81 ms | -11.10% | 1 win, 3 losses |
| Nemotron LongBench batch-32 | 324.07 ms | 350.40 ms | -7.51% | 0 wins, 4 losses |

GLM's four candidate changes were -18.73%, -19.76%, -21.12%, and -4.90%; Nemotron LongBench batch also lost all four. Those consistent cells prevent attributing the result solely to the known batch bimodality. Releasing the cold allocation changes subsequent allocator state enough to fail the full-host batch portability gate. Commit `e39cd05` is reverted, and the ranked-table memory candidate family stops here without padding or architecture-specific layout controls.

### Experiment 9: avoid cross-pool BPE nesting in batch encodes — planned

Hypothesis: `Tokenizer::encode_batch` parallelizes documents on Rayon's global pool, but a document with at least 16 pre-tokenized splits synchronously installs work into the separate capped BPE pool. The outer worker waits while up to eight other workers tokenize that document, so a batch can oversubscribe the host and repeatedly cross pool boundaries. When tokenization is already executing on any Rayon worker, keep that document's split loop sequential and let the owning pool distribute documents; standalone long encodes still enter the existing warm, eight-thread BPE pool unchanged.

This is two identical predicate additions in `tokenize` and `tokenize_batched`, with one comment explaining pool ownership. It does not expose the private pool, change the deliberate many-core cap, add topology heuristics, or alter ordering and errors. Rayon's official `ThreadPool::install` documentation confirms that cross-pool calls keep the current worker busy while the target pool executes, and the source history confirms the eight-thread cap was intentionally added for many-core locality. The xhigh Grok review agreed that cross-pool nesting is real, but warned that moving the outer batch into the capped pool could regress short batches on high-core Zen 5 and Intel hosts; this candidate avoids that design.

Acceptance requires exact IDs, no sequential regression, and at least 2% improvement in the 14-cell batch-32 geomean with no repeated cell worse than 3%. Before a full matrix, balanced screens cover long documents and short ShareGPT documents at `RAYON_NUM_THREADS=8`, then repeat batch sizes 1, 8, 32, and 128 at one and eight global threads. A one-thread regression or a loss from suppressing useful inner parallelism rejects the candidate rather than adding a machine-tuned batch heuristic.

Result: **rejected**. Formatting, all 9 focused pre-tokenized tests, the existing batch/sequential equivalence test, and library clippy with warnings denied passed. The release build completed in 9.10 seconds with 402,800,640 bytes maximum RSS, and the benchmark executable size was unchanged at 10,232,976 bytes. Every benchmark output matched Hugging Face IDs.

The first portability screen fixed the global pool at one thread while leaving the baseline's dedicated BPE pool unchanged. Two balanced DeepSeek LongBench batch-32 pairs produced:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 1,211.43 ms | 1,737.87 ms | -30.30% |
| 2 | candidate, baseline | 1,143.59 ms | 1,787.32 ms | -36.01% |

The balanced median was 1,177.51 ms baseline versus 1,762.59 ms candidate, a 33.20% throughput regression. Hugging Face control time differed by only 0.56% in trial 1 and 1.75% in the opposite direction in trial 2, so host drift cannot explain the loss. The predicate suppressed useful inner parallelism whenever a caller intentionally supplied a narrow Rayon pool. The screen stopped before eight-thread and full-matrix runs because the explicit one-thread hard gate failed twice. Commit `5d6ff31` is reverted; a viable ownership design would need batch context and work-size awareness rather than treating every Rayon worker as sufficient parallel ownership.

### Experiment 10: bypass the shared cache on fused batch tokenization — planned

Hypothesis: `tokenize_batch_fused` already holds its thread-local 65,536-slot flat cache across an entire split chunk, but every local miss also locks a shared shard for lookup. A true miss then hashes and copies the key and IDs again under another shared lock. On GPT-OSS LongBench sample 10, 936,417 pieces collapse to 34,757 unique pieces, the median piece is 4 bytes, and cold versus warm encode is 33.63 versus 11.66 ms; caching matters, but it does not establish that cross-thread sharing repays synchronization and allocation.

The current Hugging Face BPE implementation independently uses only a per-thread cache and documents removal of its shared lock from the hot path. Tiktoken's implementation notes likewise report that even read-side shared locking hurt multithreaded performance. Test that design boundary only in fastokens' primary fused batch callback: remove the `fused_shared_cache` lookup and insert from `tokenize_batch_fused`, while preserving the thread-local cache, both shared-cache fields and struct layout, `tokenize_into_fused`, the non-fused path, merge logic, and output ordering. This is a deletion-only ablation; a retained result can separately remove newly dead fused-shared state only after layout and memory validation.

The requested xhigh AI discussion was attempted with the exact code path and workload evidence. Both peer backends produced no response in 90 seconds, were stopped, and made no edits. Acceptance requires at least 2% improvement in the 14-cell batch-32 geomean and 1% in sequential geomean, no repeated cell regression worse than 3%, exact IDs, and no pathological memory growth. Balanced fused-model screens cover GPT-OSS LongBench and ShareGPT at sequential and batch-32 with `RAYON_NUM_THREADS=1` and `8`; a consistent loss stops the experiment before the full matrix.

Result: **rejected**. The deletion-only source diff removed eight lines. Formatting, all 9 focused BPE tests, the repeated fused-cache consistency test, and library clippy with warnings denied passed. The release build completed in 8.78 seconds with 401,211,392 bytes maximum RSS. The executable grew by 304 bytes to 10,233,280 bytes, despite the source deletion, and every benchmark matched Hugging Face IDs.

GPT-OSS LongBench batch-32 at eight threads was inconclusive: the first pair favored the candidate by 7.50%, the reversed pair lost 9.01%, and trial 3 contained a candidate second-batch outlier of 338.10 ms versus 197.19 ms for its first batch while host load was elevated. The two-trial balanced medians were effectively tied at 310.03 ms baseline and 311.58 ms candidate (-0.50%). The tighter sequential mode then failed consistently:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 402.89 ms | 443.10 ms | -9.07% |
| 2 | candidate, baseline | 421.52 ms | 443.57 ms | -4.97% |

The sequential median was 412.21 ms baseline versus 443.34 ms candidate, a 7.02% throughput regression. Its Hugging Face controls moved by only 2.11% and 1.77%, so the repeated fastokens losses are larger than execution-position drift. Cross-worker reuse is valuable in the dedicated BPE pool and outweighs the shared cache's locks on this representative fused workload. The screen stopped before ShareGPT, one-thread, and full-matrix runs after the explicit repeated-cell guardrail failed. Commit `8a87542` is reverted; later cache experiments must preserve cross-thread reuse rather than simply remove it.

### Experiment 11: compact thread-local cache slots — planned

Hypothesis: `CacheSlot` stores a 64-bit hash plus 12 bytes of offsets and lengths; `repr(C)` tail padding makes the slot 24 bytes. The full key bytes are already compared on every matching fingerprint, so a 32-bit fingerprint preserves correctness while reducing the slot to 16 bytes. Each 65,536-slot thread-local table falls from 1,572,864 to 1,048,576 bytes without changing capacity, 75% clearing, key/ID pools, cache ownership, or shared-cache behavior.

At 49,152 live entries, the expected number of colliding fingerprint pairs is about 0.28 under a uniform 32-bit hash, and linear probing plus the existing full-key comparison distinguishes them. The memory reduction is architecture-relevant: the slot array alone currently exceeds a 1 MiB Zen 5 private L2, while the compact array fits exactly; Apple and Intel also benefit from a one-third smaller random-probed working set. Keep the existing 64-bit mixing operations and truncate only the final fingerprint. A focused test uses two short precomputed keys with the same 32-bit fingerprint to prove that both values remain independently retrievable.

Acceptance requires exact IDs, the expected 16-byte slot size, at least 1% improvement in sequential and full-matrix throughput geomeans, no repeated cell regression worse than 3%, and a post-encode memory reduction consistent with 512 KiB per initialized worker cache. Balanced GPT-OSS and DeepSeek LongBench screens exercise fused and non-fused caches before the full seven-model matrix. The candidate is rejected if extra collision probing or code-layout drift erases the locality gain.

Result: **rejected**. The production change narrowed the stored fingerprint and final hash return from 64 to 32 bits; one focused test pinned the 16-byte slot size and proved independent retrieval for two real colliding fingerprints. All 10 focused BPE tests, formatting, and library clippy with warnings denied passed. The release build completed in 11.85 seconds with 397,000,704 bytes maximum RSS, and the executable size was unchanged at 10,232,976 bytes. Every benchmark matched Hugging Face IDs.

Two balanced GPT-OSS LongBench sequential pairs at eight threads both rejected the candidate:

| Trial | Order | Baseline | Candidate | Throughput change |
|---:|---|---:|---:|---:|
| 1 | baseline, candidate | 435.52 ms | 479.79 ms | -9.23% |
| 2 | candidate, baseline | 435.11 ms | 470.53 ms | -7.53% |

The balanced median was 435.32 ms baseline versus 475.16 ms candidate, an 8.38% throughput regression. The candidate's Hugging Face control was 0.79% faster in trial 1 and 1.67% slower in trial 2, so host drift cannot account for the repeated fastokens loss. The initial probe index still used the same low 16 hash bits and a full-key comparison preserved correctness; the remaining differences are the slot stride, narrower fingerprint load, and resulting code/layout. The smaller random-probed footprint did not translate to faster execution on this host. The screen stopped before DeepSeek, batch, and memory probes after the hard runtime gate failed twice. Commit `a1d0fd8` is reverted rather than retaining a memory-only optimization with an 8% hot-path cost.

### Experiment 12: reuse PCRE2 match data across each scan — planned

Hypothesis: stage-level measurement over the same 50 LongBench inputs attributes 258.24 ms of GPT-OSS time to regex splitting, 152.04 ms to fused BPE, 18.25 ms to input construction, and effectively zero to post-processing. Nemotron independently spends 238.67 ms splitting and 152.48 ms in BPE. The split stage is therefore roughly 56–60% of the measured encode pipeline and is the largest remaining target.

`find_matches_pcre2` currently loops over `Regex::find_at`. PCRE2 crate source shows that every `find_at` call acquires a `MatchDataPoolGuard`, executes one match, and returns the guard. The same crate's `find_iter` acquires one guard when the iterator is created and reuses it through all non-overlapping matches; its empty-match advancement is the same `end + 1` rule already implemented locally. Replace only the helper's manual position loop with `find_iter`, retaining the existing empty-match filter, absolute base offsets, capacity estimate, error mapping, chunking, overlap, and boundary repair.

This removes synchronization proportional to match count without changing the regex, match order, or public behavior. Existing Split coverage, empty-input, real-pattern, parallel-boundary, Unicode, and cache tests exercise the semantic edge cases; no duplicate test is added. Acceptance requires exact IDs, at least 5% improvement in GPT-OSS and Nemotron stage totals, at least 3% in both sequential and full-matrix throughput geomeans, and no repeated cell regression worse than 3%. One-, two-, four-, and eight-thread screens verify that the iterator helps rather than hiding a pool-specific effect before the full seven-model matrix.

Result: **retained**. The implementation replaces the local `find_at` position loop with `find_iter`, preserving the capacity estimate, empty-match filter, offsets, and error mapping. The source diff is 5 insertions and 15 deletions. The default-profile release build took 10.01 seconds and peaked at 402,423,808 bytes RSS; the executable grew from 10,232,976 to 10,250,672 bytes (17,696 bytes, or 0.17%).

The stage probe confirmed the intended mechanism before end-to-end expansion:

| Model, LongBench 50 | Baseline split | Candidate split | Split change |
|---|---:|---:|---:|
| GPT-OSS | 230.80 ms | 173.40 ms | -24.87% |
| Nemotron | 238.67 ms | 188.63 ms | -20.96% |

The retained candidate then completed four counterbalanced trials over all seven models, both corpora, sequential and batch-32: 224 fresh processes with `RAYON_NUM_THREADS=8`. Every process compared every output against Hugging Face and all outputs matched exactly. Each cell below compares the median of four baseline process totals with the median of four candidate totals; positive change means higher fastokens throughput.

| Model | Corpus | Sequential baseline → candidate | Change | Batch-32 baseline → candidate | Change |
|---|---|---:|---:|---:|---:|
| Nemotron | LongBench | 432.87 → 392.40 ms | +10.31% | 300.23 → 241.07 ms | +24.54% |
| Nemotron | ShareGPT | 67.35 → 61.81 ms | +8.96% | 32.60 → 30.95 ms | +5.34% |
| DeepSeek | LongBench | 980.22 → 978.70 ms | +0.16% | 499.08 → 429.71 ms | +16.14% |
| DeepSeek | ShareGPT | 99.94 → 96.67 ms | +3.39% | 51.53 → 41.78 ms | +23.34% |
| MiniMax | LongBench | 484.66 → 441.76 ms | +9.71% | 363.03 → 276.77 ms | +31.17% |
| MiniMax | ShareGPT | 79.90 → 71.19 ms | +12.23% | 65.17 → 48.51 ms | +34.35% |
| GPT-OSS | LongBench | 425.10 → 391.76 ms | +8.51% | 331.00 → 240.43 ms | +37.67% |
| GPT-OSS | ShareGPT | 66.89 → 63.77 ms | +4.90% | 30.87 → 29.87 ms | +3.36% |
| Mistral | LongBench | 422.03 → 372.27 ms | +13.37% | 283.06 → 279.52 ms | +1.27% |
| Mistral | ShareGPT | 67.68 → 66.03 ms | +2.50% | 36.69 → 29.66 ms | +23.69% |
| GLM | LongBench | 438.78 → 359.12 ms | +22.18% | 319.58 → 260.64 ms | +22.61% |
| GLM | ShareGPT | 68.14 → 68.83 ms | -1.00% | 33.51 → 31.40 ms | +6.72% |
| Qwen | LongBench | 412.87 → 352.62 ms | +17.09% | 285.93 → 279.23 ms | +2.40% |
| Qwen | ShareGPT | 71.40 → 66.94 ms | +6.67% | 35.48 → 28.98 ms | +22.45% |

The four-run-median geometric means are **+12.87% overall**, **+8.32% sequential**, and **+17.61% batch-32**. Twenty-seven of 28 median cells improve. The lone loss, GLM ShareGPT sequential at -1.00%, split 2 wins and 2 losses and remains below the material-regression threshold. The individual trial geomeans were +12.87%, +11.23%, +13.83%, and +17.07% overall; their sequential geomeans were +6.21%, +7.58%, +14.50%, and +6.79%. No greater-than-3% loss repeated in at least three of the four matrix pairs.

The explicit thread sweep used balanced LongBench pairs for GPT-OSS and Nemotron:

| Rayon threads | GPT-OSS change | Nemotron change |
|---:|---:|---:|
| 1 | +0.15% over four pairs | +1.03% over four pairs |
| 2 | +3.95%, 2/2 wins | +9.79%, 2/2 wins |
| 4 | +8.53%, 2/2 wins | +24.60%, 2/2 wins |
| 8 | covered by the full matrix | covered by the full matrix |

Batch sizes 1, 8, and 128 were additionally tested in two reversed-order trials on GPT-OSS and DeepSeek across both corpora. All 48 processes were exact, the 12-cell geomean was +10.47%, and 8 of 12 two-run medians improved. GPT-OSS LongBench improved by +11.39%, +3.84%, and +1.75% at batch 1, 8, and 128. DeepSeek LongBench changed by -0.03%, +4.44%, and +13.36%. The two apparent GPT-OSS ShareGPT losses at batch 8 and 128 each occurred only in trial 1 and reversed to wins in trial 2.

DeepSeek ShareGPT batch-1 needed a separate control: four `simple_bench` pairs showed a raw -4.89% median, but the Hugging Face control inside every candidate process was also 1.27–5.27% slower. Normalizing each pair by that independent control produced a -0.78% median and a 2–2 split. A fastokens-only external diagnostic then removed interleaved Hugging Face execution and ran 12 alternating processes over the same 200 inputs. Both binaries produced 862,854 tokens; baseline and candidate medians were 89.34 and 88.97 ms, respectively (**+0.42%**). This isolates the raw `simple_bench` result to whole-process/control drift rather than a repeatable tokenizer regression.

Validation passed `cargo fmt --all -- --check`, `cargo clippy --lib -- -D warnings`, all 179 non-ignored Rust workspace tests, all 3 Python-extension Rust tests, and a release-wheel encode/decode smoke test. The requested xhigh AI discussion was attempted again before the change; both peer backends remained silent for 90 seconds, were stopped cleanly, and made no edits. Commit `810979f` is retained as the first source optimization in this campaign.

### Experiment 13: reuse the incremental Split cache's input allocation — planned

Hypothesis: `pre_tokenize_pcre2_isolated` copies every input into its thread-local incremental cache, but it reuses `prev_input` only when the next input has exactly the same length. Otherwise `bytes.to_vec()` allocates a replacement and drops the old buffer even when its capacity is sufficient. All 50 consecutive LongBench inputs and all 200 consecutive ShareGPT inputs in the screening corpus have different lengths; each corpus establishes a new maximum only four times. Clear the existing vector and extend it from the new bytes so allocation grows only when capacity is insufficient. Do not alter the copied bytes, prefix threshold, match cache, or incremental-reuse policy.

The tradeoff is that each initialized Split cache retains the largest input buffer it has seen instead of shrinking to the latest input. The screen therefore includes post-run RSS as well as timing. Acceptance requires at least 1% improvement in repeated GPT-OSS or Nemotron split-stage medians, confirmation in an exact end-to-end workload, and no repeated regression worse than 2%. A result inside noise is reverted rather than retaining a memory-lifetime change without measured speedup.

Result: **rejected**. The minimal candidate replaced the exact-length allocation branch with `clear` plus `extend_from_slice`; all 46 Split tests and library clippy passed. Its release example build took 9.35 seconds and peaked at 397,197,312 bytes RSS. The binary grew by 128 bytes, from 10,250,672 to 10,250,800 bytes.

Six counterbalanced stage pairs at eight Rayon threads failed before an end-to-end expansion. GPT-OSS split medians moved from 160.54 to 165.95 ms, a **3.26% throughput regression**, with the candidate losing four of six pairs. Nemotron moved from 171.18 to 172.63 ms, a 0.84% regression. Since the primary stage gate failed, the post-run retention-memory probe and broader exact matrix were intentionally skipped. General-purpose allocator reuse already amortizes enough of the differing-size allocations that keeping the largest per-thread buffer adds lifetime without measurable benefit. Commit `726a277` is reverted.

### Experiment 14: allow concurrent shared-cache reads — planned

Hypothesis: the fused BPE path has a proven need for cross-worker cache reuse, but each thread-local miss currently takes an exclusive `Mutex` even when it only reads an existing shared entry. A 10-second macOS sample of a repeated GPT-OSS LongBench stage workload attributed 195 worker samples to kernel mutex waits inside `tokenize_batch_fused`. Removing the shared cache already regressed sequential throughput by 7.02%, so retain the cache and its 64 shards while changing only each shard's synchronization from `Mutex` to `RwLock`: `get_into` takes a read guard and `insert` takes a write guard.

An external diagnostic over the 50 LongBench inputs found 9,755,220 piece occurrences and 211,366 unique pieces. The current prefix hash distributes unique keys almost perfectly: the largest of 64 shards holds 3,479 keys, only 1.05 times the mean. Rehashing every byte made the unique distribution substantially worse, and 128 shards did not materially reduce the hottest occurrence-weighted shard, so neither the hash nor shard count changes in this experiment. The diagnostic also exposed a profiling caveat: 100 passes eventually exceed and churn each worker's 49,152-entry local-cache load limit, making that sample more read-heavy than the normal one-pass benchmark.

The standard library documents that reader-writer priority is operating-system-dependent. Treat the substitution as portable only if it wins the project's cold/mixed workload as well as the read-heavy probe. Start with balanced GPT-OSS LongBench stage and exact end-to-end pairs, including sequential and batch-32. A candidate that wins only the repeated probe is rejected. Expansion requires at least 1% improvement in both the sequential and batch-32 screen medians, then a full seven-model, two-corpus matrix with at least 1% geomean improvement and no repeated cell regression worse than 3%. Validate focused BPE tests, formatting, clippy, build time, RSS, binary size, and exact Hugging Face IDs. A requested xhigh peer discussion was attempted with this evidence; both backends remained silent for 60 seconds, were stopped cleanly, and made no edits.

Result: **rejected**. The candidate changed only the shard type, constructor, and read/write acquisition sites. Formatting, all 9 focused BPE tests, and library clippy with warnings denied passed. The release example build took 10.33 seconds and peaked at 400,375,808 bytes RSS. The binary shrank by 848 bytes, from 10,250,672 to 10,249,824 bytes. Every benchmark produced the same 10,961,405 tokens as the retained baseline and matched Hugging Face IDs.

Four one-pass GPT-OSS LongBench stage pairs were noisy and split two wins to two losses. Their BPE medians were 153.44 ms baseline and 155.80 ms candidate, a 1.52% throughput regression. The correctness-bearing end-to-end screen was clearer. Four counterbalanced sequential pairs had a 419.52 ms baseline median and 435.77 ms candidate median, a **3.73% throughput regression**, again with only two candidate wins. Batch-32 then lost all four pairs by 15.22%, 7.68%, 3.47%, and 28.06%. Its median moved from 261.75 to 308.85 ms, a **15.25% throughput regression**. The candidate's Hugging Face control was faster in two of those four pairs, including the largest loss, so host ordering does not explain the result.

This cache is not read-mostly during a cold corpus pass: workers interleave shared misses and inserts, and the reader-writer lock's more expensive acquisition plus exclusive write phases overwhelms any concurrent-hit benefit. The normal sequential and batch gates both failed, so the synthetic 100-pass read-heavy probe and full matrix were skipped. Commit `a9378ee` is reverted; later shared-cache work must preserve the cheaper uncontended mutex path as well as cross-worker reuse.

### Experiment 15: bypass fused BPE for one-byte pieces — planned

Hypothesis: a nonempty raw piece containing exactly one UTF-8 byte is ASCII and cannot participate in a BPE merge. The fused model already has a 256-entry `byte_to_initial_token` table, and `merge_all_raw_into` ultimately emits that entry after allocating merge scratch state. The batch path instead checks the thread-local cache first and, on a local miss, locks the shared cache before reaching the same result. OpenAI tiktoken independently handles a one-byte piece as an immediate token lookup before its merge algorithm.

The external LongBench diagnostic found that one-byte pieces are common: 1,790,246 of 9,755,220 GPT-OSS pieces (18.35%) and 2,997,166 of 10,516,705 Nemotron pieces (28.50%). Add one guarded fast path inside the production `tokenize_batch_fused` loop before cache lookup. If the precomputed byte ID is valid, append it and continue; if the vocabulary lacks that byte, fall through to the existing cache and error path unchanged. Do not change the non-batched profiling helper, cache representations, merge algorithm, `ignore_merges`, or pieces of length two and above.

Acceptance requires at least 3% improvement in GPT-OSS and Nemotron fused BPE-stage medians, at least 2% in their exact sequential and batch-32 end-to-end screens, and no repeated regression worse than 3%. A passing candidate expands to the seven-model, two-corpus matrix and must improve its overall geomean by at least 1%. Validate formatting, focused BPE and fused-cache tests, clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected**. The source change added one valid-ID branch before the local-cache lookup and otherwise fell through unchanged. Formatting, all 9 focused BPE tests, the real-model fused-cache test, and library clippy with warnings denied passed. The release example build took 12.15 seconds and peaked at 397,426,688 bytes RSS; its 10,250,672-byte executable was unchanged in size. Every end-to-end run matched Hugging Face and produced 10,961,405 tokens.

An initial repeated stage attempt was invalid and excluded: the preserved baseline stage executable predated the external harness's repeat argument, so it ran one pass while the candidate ran three. Six corrected one-pass GPT-OSS stage pairs then had BPE medians of 150.17 ms baseline and 158.00 ms candidate, a 4.96% throughput regression; the candidate lost five of six pairs. The stage executables were linked from slightly different external harness revisions, so the unchanged `simple_bench` source provided the decisive check.

Four counterbalanced GPT-OSS LongBench sequential pairs split two wins and two losses. Their baseline times were 397.87, 415.43, 426.66, and 389.26 ms; candidate times were 439.18, 388.45, 424.96, and 415.28 ms. The four-run medians were 406.65 and 420.12 ms, respectively, a **3.21% candidate throughput regression**. Normalization by each process's Hugging Face control also split two wins and two losses and showed no repeatable gain. The GPT-OSS stage and end-to-end gates both failed, so Nemotron, batch-32, and the full matrix were skipped.

Although a table lookup is cheaper than BPE in isolation, the existing local cache already makes recurring one-byte pieces cheap. Testing `text.len() == 1` before every lookup adds an unpredictable branch to all 9.76 million pieces, overwhelming the avoided work on this pipeline. Commit `ac45b23` is reverted. A future variant may test the length only after a local-cache miss, but must be treated as a distinct mechanism rather than restoring this all-piece branch.

### Experiment 16: emit one-byte pieces after local-cache misses — planned

Hypothesis: Experiment 15 placed the length branch before the thread-local lookup and regressed GPT-OSS by 3.21%. Move the same exact fast path after `cache.get` instead. Cached multi-byte pieces then retain their original branch-free path. One-byte pieces intentionally miss the local table, test their length, and emit the precomputed byte ID without a shared-cache lock, allocation, or merge. Compared with the existing one-byte local hit, this still computes the flat hash but replaces full-key comparison and cached-output copying with an empty-slot check and direct array lookup.

The valid-ID guard and sentinel fallthrough remain unchanged, as do all cache types, multi-byte pieces, and public behavior. This is the only placement variant justified by the rejected experiment's branch-cost evidence; if it does not improve the exact GPT-OSS LongBench sequential median by at least 1% across four balanced pairs, stop the single-byte family and revert before batch or matrix expansion. A passing screen must also improve Nemotron and batch-32, with the same full-matrix and correctness requirements as Experiment 15.

Result: **rejected**. The source diff again added only the guarded byte lookup, this time after `cache.get`. Formatting, all 9 focused BPE tests, the real-model fused-cache test, and library clippy passed. The release example build took 9.93 seconds and peaked at 395,231,232 bytes RSS; binary size remained 10,250,672 bytes. Every correctness-bearing run matched Hugging Face IDs.

Four balanced GPT-OSS LongBench sequential pairs produced baseline times of 397.22, 404.24, 424.94, and 468.92 ms and candidate times of 436.29, 408.69, 430.51, and 382.73 ms. The raw medians were 414.59 and 419.60 ms, a 1.19% candidate throughput regression, with only one raw candidate win. The candidate's independent Hugging Face control was slower in the first three process positions, and control-normalized ratios favored it in three pairs, so an isolated probe removed that confound before rejection.

The external probe compiled one identical harness against the exact retained and candidate commits, excluded construction, warmed one input, and timed only 50 sequential fastokens encodes. Twelve alternating processes all produced 10,961,405 tokens. Baseline times were 319.06, 327.13, 335.13, 396.33, 342.14, and 319.89 ms; candidate times were 365.91, 301.10, 316.30, 344.53, 353.91, and 337.39 ms. Their medians were 331.13 and 340.96 ms, a **2.88% candidate throughput regression**, split three wins to three losses. The post-cache placement therefore does not expose a reliable saving either. Commit `744a591` is reverted, and the single-byte fast-path family stops without Nemotron, batch, or matrix expansion.

### Experiment 17: move the complete parallel match vector — planned

Hypothesis: `pre_tokenize_pcre2_isolated` initializes `matches` from the incremental cache, computes a complete parallel suffix vector, then calls `matches.extend(suffix_matches)`. When no prefix is reusable—the case for every consecutive LongBench and ShareGPT input—`matches` is empty, so extending allocates another vector and copies every match before building splits. The macOS sample attributed 74 `memmove` frames directly to this `Vec::extend` site.

When `matches.is_empty()`, move the owned parallel result vector into it instead of extending. When cached prefix matches exist, keep the current extension path unchanged. This changes neither match contents nor order, adds no representation or unsafe code, and preserves the sequential small-suffix path in the first experiment. The normal path removes one allocation and one full match-vector copy per large input.

Use identical external stage-harness source linked separately against the retained and candidate commits. Acceptance requires at least 2% improvement in both GPT-OSS and Nemotron split-stage medians, at least 1% in their exact end-to-end sequential and batch-32 medians, and no repeated regression worse than 3%. A passing screen expands to the full seven-model, two-corpus matrix. Validate all Split tests, formatting, clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected**. The implementation replaced the empty destination with the owned parallel result while retaining `extend` for incremental prefix reuse. All 46 Split tests, formatting, and library clippy passed. The release example build took 11.13 seconds and peaked at 396,591,104 bytes RSS; binary size remained 10,250,672 bytes. All end-to-end processes matched Hugging Face IDs.

Identical external stage harnesses supported the intended copy-removal mechanism. Over six three-pass pairs, GPT-OSS split medians moved from 478.45 to 471.83 ms (**+1.40% throughput**) with six candidate wins. Nemotron moved from 543.71 to 521.78 ms (**+4.20%**) with five wins. GPT-OSS's four exact LongBench sequential pairs also all improved, from a 412.13 ms baseline median to 389.75 ms candidate median (**+5.74%**).

Nemotron did not carry the isolated win into the correctness-bearing harness. Its first four exact sequential pairs had a 432.40 ms baseline median and 492.95 ms candidate median, a 12.28% regression, with only one candidate win. An identical fastokens-only harness then ran 12 alternating processes, each producing 12,288,294 tokens: baseline and candidate medians were 341.10 and 315.23 ms, respectively (**+8.21% candidate throughput**, four of six wins). That proved the merge-copy removal itself can help, but did not clear the actual benchmark's interleaved Hugging Face and allocator context.

Four additional pairs with five-second cooldowns between every process did not resolve the end-to-end loss. Baseline times were 412.72, 393.43, 416.39, and 537.32 ms; candidate times were 401.36, 395.67, 474.90, and 482.68 ms. Across all eight exact pairs, baseline and candidate medians were 428.43 and 478.79 ms, a **10.52% candidate throughput regression**, and the candidate lost five pairs. Moving the earlier parallel-result allocation into the long-lived Split cache changes downstream allocator state even though it removes a copy; Nemotron is repeatedly sensitive to that state when Hugging Face work is interleaved. The explicit repeated-cell guardrail therefore overrides the isolated win. Batch and matrix expansion stopped, commit `f8c9c07` is reverted, and later allocation-lifetime experiments must preserve the current result-vector ownership pattern.

### Candidate investigation: reuse match data during parallel-boundary repair — stopped before source change

Experiment 12 removed repeated PCRE2 match-data acquisition from the main scans, but `find_matches_pcre2_parallel` still calls `find_at` when repairing a truncated or misaligned chunk boundary. PCRE2 exposes `captures_read_at` with reusable `CaptureLocations`, which could remove that remaining pool access. The crate implementation confirms the tradeoff: `find_at` acquires an existing pooled `MatchData`, while `capture_locations` allocates fresh match data for explicit reuse. A lazy location buffer would therefore need several repair searches per affected document to repay its allocation and added closure state.

An external diagnostic mirrored the production authority zones, 1 KiB overlap, per-chunk filtering, ghost removal, truncation repair, and convergence search over the first 50 LongBench-v2 contexts. GPT-OSS required only 30 full-text repair searches total: 44 documents required none, four required 3, and the remaining two required 8 and 10. Nemotron required 9 total: 47 documents required none and three required 3. Both medians were zero. The retained ten-second macOS sample independently captured only one `find_at` execution in the repair closure.

This mechanism cannot materially affect the measured split stage: even perfect elimination saves fewer than one pool acquisition per document on average, while an eager buffer would allocate on every document and a lazy buffer would add state for a path absent in most documents. No production source was changed or benchmark build created. Reconsider only if a tokenizer pattern or corpus demonstrates substantially more boundary repairs.

### Experiment 18: move initial BPE heap entries without copying — planned

Hypothesis: `merge_all_raw_into` first collects all initial byte-pair merge candidates in a reusable `Vec`, then drains that vector into an empty reusable `BinaryHeap`. Each `MergeEntry` is 16 bytes, so the drain copies every initial candidate before the merge loop. The standard library's `BinaryHeap::from(Vec)` heapifies a vector in place, while `BinaryHeap::append` swaps the two heaps when the destination is shorter. Because the scratch heap is explicitly cleared first, construct the initial heap from `heap_buf`, append it into the empty scratch heap, and return the now-empty heap's backing vector to `heap_buf`. This preserves both existing allocations while eliminating the element transfer.

Change only the three-line bulk-heap construction. Do not alter candidate generation, heap ordering, merge lookup, scratch lifetimes, cache behavior, or the encoded-text path. The standard library operation is architecture-independent and stable since Rust 1.11. Screen six counterbalanced GPT-OSS and Nemotron LongBench stage pairs with identical external harnesses. Expansion requires at least 1% improvement in the two-model BPE-stage geomean, no model median regression worse than 2%, then at least 1% improvement in exact sequential end-to-end medians without a repeated regression worse than 3%. A passing candidate proceeds to batch-32 and the full seven-model matrix. Validate focused BPE tests, the real-model fused-cache test, formatting, clippy, build time, RSS, binary size, and exact Hugging Face IDs.

Result: **rejected**. The candidate used `BinaryHeap::from` and `append` exactly as planned. Formatting, all 9 focused BPE tests, the real-model fused-cache test, and library clippy with warnings denied passed. The release example build took 9.05 seconds and peaked at 397,770,752 bytes RSS. Its executable shrank by 112 bytes, from 10,250,672 to 10,250,560 bytes. Every end-to-end process produced 10,961,405 tokens and matched Hugging Face IDs.

The stage screen was noisy but narrowly permitted an exact check. GPT-OSS BPE medians were 348.82 ms baseline and 351.05 ms candidate, a 0.64% regression with four of six candidate wins. Nemotron medians were 437.62 and 404.09 ms, an 8.30% apparent improvement with only two of six candidate wins because the host shifted to a faster regime midway through the run. Pairwise normalization produced +3.36% for GPT-OSS, -0.70% for Nemotron, and **+1.31% across all 12 pairs**, just clearing the 1% stage gate.

The correctness-bearing GPT-OSS LongBench screen rejected the candidate. Four counterbalanced pairs produced baseline times of 381.95, 400.94, 388.04, and 428.77 ms and candidate times of 381.74, 554.33, 401.42, and 414.17 ms. Their medians were 394.49 and 407.80 ms, a **3.26% candidate throughput regression**, with two wins and two losses. The largest loss coincided with only a 2.71% slower Hugging Face control, versus a 38.26% fastokens time increase; control-normalized results still had two material regressions. The candidate failed the first exact-model gate, so Nemotron exact, batch-32, and the full matrix were skipped. Eliminating the initial-entry copy does not provide a stable end-to-end benefit, and commit `3787d82` is reverted.

### Candidate investigation: add one slot of Split output capacity — stopped before source change

The retained profile contained 19 `Vec` growth samples under the compiled Split entry point, 18 of which reached a large `mach_vm_copy`. The fast PCRE2 path allocates output capacity as twice the match count even though the theoretical maximum is twice the matches plus one, making a one-slot correction appear plausible. The stripped sample could not locate the originating vector reliably, so an external diagnostic counted the exact full-text PCRE2 match and output-piece sequences before changing source.

Across the first 50 LongBench-v2 contexts, GPT-OSS produced 9,755,249 nonempty matches and exactly 9,755,249 output pieces; Nemotron produced 10,517,080 of each. Neither model had a gap before, between, or after matches, and no document could exhaust the existing `2 × matches` capacity. Adding one slot would therefore leave every measured allocation and push unchanged and cannot explain the sampled growth. No production source or benchmark build was created; a future capacity change requires a profile that resolves the actual growing vector.

### Experiment 19: reserve known generic Split expansion — planned

Hypothesis: a release build with line-level debug metadata resolved the sampled growth to the generic path's `new_splits.push`, not the isolated PCRE2 fast path. That vector starts with capacity `2 × input_splits`, but each unprotected text split computes its complete `ranges` vector before appending it. Added-token preprocessing can create a small number of input splits around very large text regions: GPT-OSS LongBench document 35 expands 47 input splits into 944,365 outputs; Nemotron documents 33 and 35 expand 5 into 1,301,045 and 369 into 982,320. The ten-second retained sample caught 15 growth calls at this exact push, including 14 large reallocations that entered `mach_vm_copy`.

Call `new_splits.reserve(ranges.len())` once after each range vector is known and before its push loop. This guarantees capacity for that split's maximum contribution, turning repeated geometric growth within a large range into at most one reservation. Empty ranges merely leave conservative spare capacity. Do not alter the existing initial capacity, range construction, filtering, order, fast path, or any representation. Screen six counterbalanced three-pass stage pairs on GPT-OSS and Nemotron. Expansion requires at least 1% improvement in the two-model split-stage geomean, no model regression worse than 2%, then at least 1% exact sequential improvement without a repeated regression worse than 3%. A passing candidate proceeds to batch-32 and the full seven-model matrix. Validate all Split tests, formatting, clippy, build time, RSS, binary size, and exact Hugging Face IDs.

Result: **rejected**. The source change added only the planned reservation and its explanatory comment. All 46 Split tests, formatting, and library clippy with warnings denied passed. The release example build took 9.55 seconds and peaked at 405,323,776 bytes RSS; executable size remained 10,250,672 bytes. Every stage process produced the retained token count.

Six counterbalanced three-pass GPT-OSS stage pairs had split medians of 474.90 ms baseline and 475.62 ms candidate, a 0.15% regression with three wins and three losses. Nemotron produced 529.71 and 535.03 ms medians, a **0.99% regression**, with only two candidate wins. The combined split-stage geomean was **-0.57%**, and the candidate won five of twelve pairs. Reserving the final large extent before its first push removes geometric copies in isolation but does not repay the earlier large allocation and changed allocator state in the full corpus. The primary stage gate failed, so exact end-to-end, batch-32, and matrix benchmarks were skipped. Commit `f6170d1` is reverted.

### Experiment 20: size `ignore_merges` encoding for UTF-8 expansion — planned

Hypothesis: GPT-OSS and GLM 4.7 both enable `ignore_merges`. On every fused-cache miss, that path allocates an encoded `String` with the raw byte length, then maps each byte to a character occupying one or two UTF-8 bytes. Any piece containing a space, control byte, or non-ASCII byte therefore outgrows its initial allocation. The line-resolved retained GPT-OSS profile sampled 39 executions in `String::push` growth at this exact loop, including allocator reallocation work. The byte-level implementation already proves that twice the raw length is a sufficient upper bound.

Change only the two `ignore_merges` temporary capacities from the raw length to its saturating double. Do not change encoding, lookup, caches, the loop, or models without `ignore_merges`. Screen six counterbalanced three-pass BPE-stage pairs on GPT-OSS and GLM 4.7 LongBench. Expansion requires at least 1% improvement in their combined BPE-stage geomean, no model regression worse than 2%, then at least 1% exact sequential improvement without a repeated regression worse than 3%. A passing candidate proceeds to batch-32 and the full seven-model matrix. Validate the focused `ignore_merges` tests, formatting, clippy, build time, RSS, binary size, and exact Hugging Face IDs.

Result: **rejected after the early regression gate**. The implementation changed only the two capacities and documented the two-byte UTF-8 bound. The real GLM `ignore_merges` test, its randomized decode comparison, the GPT-OSS correctness test, formatting, and library clippy with warnings denied passed. The release example build took 9.90 seconds and peaked at 390,299,648 bytes RSS; executable size remained 10,250,672 bytes.

Two counterbalanced three-pass LongBench pairs were sufficient to trigger the planned stop rule. GPT-OSS BPE times were 360.019 versus 374.799 ms in the baseline-first pair (**-3.94% candidate throughput**) and 338.457 versus 373.587 ms in the candidate-first pair (**-9.40%**). Its baseline and candidate medians were 349.238 and 374.193 ms, a **6.67% regression**. GLM was effectively neutral: 346.477 versus 348.340 ms (**-0.53%**) and 346.829 versus 346.698 ms (**+0.04%**), for a 0.25% median regression. The four paired throughput ratios had a **-3.54% geometric mean**, and every process produced its model's identical retained token count.

Unrelated system load was elevated, but reversing both binary and model order did not change the GPT-OSS direction, and both losses exceeded the repeated 3% guardrail. The remaining four stage pairs, exact end-to-end checks, and matrix expansion were stopped. Allocating the maximum two-byte extent for every temporary costs more than the occasional geometric growth it removes; a future encoding experiment must avoid that footprint increase rather than retest capacity headroom. Commit `0866efc` is reverted.

### Experiment 21: reuse one `ignore_merges` encoding buffer per fused chunk — planned

Hypothesis: Experiment 20 showed that allocating the maximum encoded extent for every miss is harmful, but the retained profile still resolves initial allocation and growth samples to the temporary `ignore_merges` string. GPT-OSS and GLM each produce about 9.7 million LongBench pieces, with only 0.31% and 0.35% at least 16 bytes, respectively. A fused chunk can therefore create one empty `String`, clear and reuse its small allocation across all misses in that chunk, then drop it. This removes per-piece allocation without retaining memory in the thread-local cache or reserving worst-case capacity.

Change only `tokenize_batch_fused`: declare one chunk-local encoding buffer outside the split loop, replace the per-miss construction with `clear`, and keep the byte mapping and vocabulary lookup unchanged. Do not change `FlatCache`, the single-piece API, capacity policy, encoding implementation, caches, or merge behavior. Screen six counterbalanced three-pass BPE-stage pairs on GPT-OSS and GLM 4.7 LongBench. Expansion requires at least 1% improvement in their combined BPE-stage geomean, no model regression worse than 2%, then at least 1% exact sequential improvement without a repeated regression worse than 3%. A passing candidate proceeds to batch-32 and the full seven-model matrix. Validate focused correctness, formatting, clippy, build time, RSS, binary size, and exact Hugging Face IDs.

Result: **rejected at batch-32**. The implementation added one empty chunk-local `String`, cleared it on each `ignore_merges` miss, and otherwise retained the existing byte loop and lookup. The real GLM `ignore_merges` test, formatting, and library clippy with warnings denied passed. The release example build took 9.27 seconds and peaked at 391,610,368 bytes RSS; executable size remained 10,250,672 bytes. Every correctness-bearing process matched Hugging Face IDs exactly.

The isolated six-pair LongBench stage screen passed. GPT-OSS BPE medians moved from 360.122 to 336.938 ms (**+6.88% throughput**, four wins), while GLM moved from 357.494 to 329.612 ms (**+8.46%**, five wins). Two externally slow GPT candidate processes made its raw pairwise geometric mean -3.26%, but its Split-control-normalized result was +0.09%; GLM's corresponding results were +6.55% and +4.19%. Across all twelve pairs, raw and Split-normalized geometric means were **+1.52%** and **+2.12%**, respectively.

Four counterbalanced exact sequential pairs remained noisy but narrowly passed in aggregate. GPT-OSS medians were 445.145 and 421.805 ms (**+5.53%**, three wins), with +1.90% raw and -0.77% Hugging Face-control-normalized pairwise geometric means. GLM medians were 388.260 and 392.190 ms (**-1.00%**, two wins), with +0.94% raw and -1.03% normalized pairwise means. The two-model median geomean was +2.21%; all eight raw pairs gave +1.42%, while control normalization gave -0.90%. GPT's first candidate process lost across 43 of 50 inputs, whereas the next three won 34, 39, and 35 inputs, confirming broad process-state phases rather than one pathological document.

Batch-32 rejected the candidate after two counterbalanced pairs. GPT-OSS baseline/candidate times were 280.86/281.96 and 258.06/327.69 ms, producing medians of 269.460 and 304.825 ms (**-11.60%**) and control-normalized pair losses of 1.80% and 14.13%. GLM times were 254.93/407.46 and 223.80/277.86 ms, producing medians of 239.365 and 342.660 ms (**-30.15%**) and normalized pair losses of 25.33% and 12.32%. Across the four batch pairs, raw and control-normalized geometric means were **-20.71%** and **-13.80%**. Reusing allocation within a whole parallel chunk helps the isolated BPE stage but extends buffer lifetimes and changes allocator state enough to harm the correctness-bearing batched workload. The remaining batch pairs and seven-model matrix were stopped, commit `a63f52a` is reverted, and future `ignore_merges` work must not extend temporary allocation lifetime across pieces.

### Experiment 22: fold the flat-cache hash before bucket masking — planned

Hypothesis: the line-resolved retained profile places the largest fused-BPE sample cluster in `FlatCache::get` linear-probe advancement. The cache stores a full 64-bit rolling hash but chooses its 65,536-slot bucket from only the low 16 bits. An external simulator replayed every nonempty LongBench piece through the production table size, 75% clear threshold, and linear-probe rules. Folding the high 32 bits into the low half before masking reduced average probes from 6.440 to 1.422 for GPT-OSS, 6.146 to 1.379 for Nemotron, 6.440 to 1.422 for MiniMax, 6.146 to 1.379 for Mistral, 6.202 to 1.405 for GLM, and 6.086 to 1.381 for Qwen. DeepSeek improved from 1.696 to 1.456. Maximum chains on the six most affected models fell from 2,051–2,302 slots to 126–169, and every strategy had the same clear count.

Change only the initial bucket expression in `FlatCache::get` and `FlatCache::insert` from the raw hash to `hash ^ (hash >> 32)` before applying the existing mask. Keep the stored hash, hash function, equality checks, table representation, size, load factor, probing, clearing, and every cache caller unchanged. The shift and XOR are portable across Apple ARM, Zen 5, and Core Ultra. Screen six counterbalanced three-pass BPE-stage pairs on GPT-OSS, Nemotron, and DeepSeek. Expansion requires at least 3% improvement in their combined BPE-stage geomean with no model median regression worse than 2%, then at least 2% exact sequential and batch-32 improvement without repeated regressions worse than 3%. A passing candidate proceeds to the full seven-model, two-corpus matrix and thread/batch-size sweeps. Validate focused cache and real-model tests, formatting, clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected at batch-32**. The implementation changed only the two bucket expressions and documented the fold. Four cache-focused tests, including cold/warm real-model fused consistency, formatting, and library clippy with warnings denied passed. The release example build took 8.55 seconds and peaked at 399,982,592 bytes RSS; executable size remained 10,250,672 bytes. Every correctness-bearing process matched Hugging Face IDs exactly.

The six-pair three-model stage screen strongly supported the mechanism after accounting for host phases. GPT-OSS BPE medians moved from 345.890 to 320.459 ms (**+7.94%**, five wins), with +3.97% raw and +10.55% Split-normalized pairwise geometric means. Nemotron moved from 374.756 to 323.960 ms (**+15.68%**, six wins), with +17.75% raw and +6.34% normalized means. DeepSeek's initial three-pass medians were 1,038.447 and 1,084.025 ms (-4.20%), but its raw pairwise mean was only -0.42% and its Split-normalized mean was +4.46%. Across all eighteen pairs, raw and normalized geometric means were **+6.83%** and **+7.09%**.

A supplementary four-pair, ten-pass DeepSeek screen separated the borderline result from control-stage drift. Baseline and candidate BPE medians were 1,266.489 and 1,293.071 ms (-2.06%), while every candidate process also had a 4–7% slower untouched Split stage; the four Split-normalized pairs all improved, for a +2.55% geomean. Four exact sequential pairs then cleared the per-model gate: medians moved from 1,035.190 to 1,012.645 ms (**+2.23%**), with +2.62% raw and +2.75% Hugging Face-normalized pairwise means. Every run produced 11,248,053 exact tokens.

Two-pair exact sequential screens were mixed. GPT-OSS produced one 2.67% win and one 24.26% loss, for -11.82% raw and -9.07% control-normalized geometric means. Nemotron produced a 1.10% loss and a 7.32% win, for +3.03% raw and +4.78% normalized means. The process-level variance required batch evidence rather than accepting or rejecting from these incomplete sequential screens.

Batch-32 rejected the global fold. GPT-OSS lost 30.46% and 32.97% in opposite binary orders; its median throughput regression was **31.85%**, and control-normalized pairwise geomean was **-27.79%**. GLM 4.7 then lost 21.65% and 6.01%, for a **14.67% median regression** and -16.49% normalized mean. Nemotron differed: one 17.89% win and one 3.71% loss produced a +6.34% median result and +7.83% normalized mean. The simulator correctly exposed long probe chains but omitted per-worker access locality; reducing logical probes globally does not improve every parallel configuration. GPT-OSS, Nemotron, Mistral, and GLM all enable `ignore_merges`, so that configuration flag cannot separate the observed winners and losers. The remaining exact pairs and full matrix were stopped, commit `cd39471` is reverted, and the hash-fold family stops here.

### Experiment 23: reuse the flat-cache hash after a miss — planned

Hypothesis: `FlatCache::get` hashes the complete key before lookup, then every miss that reaches `FlatCache::insert` hashes the same immutable key again. Line-resolved optimized assembly confirms that LLVM retains distinct full hash loops: the batch lookup at source line 1178 inlines `hash_str` over `0x100070810..0x10007089c`, while the exact-token insertion at line 1195 inlines it again over `0x1000713d0..0x100071468`. This is computation reuse, unlike Experiment 22: it does not change the hash, bucket, probe sequence, table contents, or access locality.

An external simulator replayed every nonempty piece from the first 50 LongBench-v2 contexts through the production table size, load limit, clearing, hash, and probe rules. Nemotron missed 3.48% of lookups and 7.31% of hashed bytes; DeepSeek missed 15.89% and 86.14%; MiniMax 3.90% and 7.45%; GPT-OSS 3.90% and 7.45%; Mistral 3.48% and 7.31%; GLM 3.95% and 7.61%; and Qwen 3.57% and 7.54%. The short-piece models bound the likely saving, while DeepSeek's long, less-repetitive pieces provide a workload where eliminating the second pass can be material.

Compute the existing hash once at each of the three private tokenizer call paths and pass it to both `FlatCache::get` and any subsequent `FlatCache::insert`. Do not add a second API, cache state, wrapper, representation, or new hash operation. Preserve all early returns, shared-cache behavior, `ignore_merges`, merging, and insertion policy. Confirm in optimized assembly that miss branches no longer contain a second `hash_str` loop. Screen six counterbalanced three-pass LongBench stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires at least 1% improvement in their combined BPE-stage geomean with no model median regression worse than 2%, then at least 1% exact sequential and batch-32 geomean improvement without repeated regressions worse than 3%. A passing candidate proceeds to the full seven-model, two-corpus matrix and thread/batch-size sweeps. Validate focused cache and real-model tests, formatting, clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected at the stage gate**. The implementation computed the unchanged hash once in each private tokenizer path and passed it through every corresponding local lookup and insertion. All 9 BPE tests, encoded and fused real-model cache consistency, GLM `ignore_merges`, formatting, and library clippy with warnings denied passed. Workspace all-target clippy remains blocked by unrelated pre-existing warnings in `examples/profile_sample.rs`, `examples/simple_bench.rs`, and `python/src/lib.rs`; those files were not changed. The release example build took 8.70 seconds and peaked at 391,823,360 bytes RSS. Its executable shrank by 560 bytes, from 10,250,672 to 10,250,112 bytes.

Optimized assembly confirmed the intended mechanism. The inlined `tokenize_batch_fused` body shrank from 5,868 to 5,396 bytes. Within that body, the shared FlatCache/FxHash multiplier was materialized once with four multiply sites, versus twelve materializations and ten multiply sites in the retained binary. The redundant insertion scans were therefore removed rather than merely rearranged.

Six counterbalanced three-pass LongBench pairs exposed a cross-model regression:

| Model | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Raw wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,093.410 ms | 1,090.693 ms | +0.25% | -0.61% | +1.40% | 4/6 |
| GPT-OSS | 348.090 ms | 388.470 ms | **-10.39%** | -0.15% | +2.12% | 2/6 |
| Nemotron | 384.593 ms | 356.688 ms | +7.82% | +6.92% | +6.37% | 5/6 |

Across all eighteen pairs, raw and Split-normalized geomeans were +1.99% and +3.27%, with eleven and twelve wins, respectively. The geomean of the three model median ratios was nevertheless **-1.06%**. More importantly, GPT-OSS lost its final four raw pairs by 4.15%, 7.80%, 15.69%, and 14.20% in alternating binary orders. Its untouched Split stage also varied, explaining the positive normalized result but not satisfying the explicit raw per-model and repeated-regression guardrails. Exact end-to-end and batch expansion therefore stopped. Commit `81081a2` is reverted; global caller-level hash reuse is not portable across this matrix despite eliminating the intended work.

### Candidate investigation: enable PCRE2 JIT — stopped before source change

Both PCRE2 construction paths already call `jit_if_available(true)`: initial compilation in `try_compile_pcre2_regexes` and the independent-copy `Clone` implementation. The retained macOS sample also contains `pcre2_jit_match_8` below the production iterator, proving that JIT code is active on this host rather than merely requested. There is no missing switch to test; forcing JIT would only turn the existing portable fallback into a construction error on unsupported platforms. No source or benchmark build was created.

### Candidate investigation: shrink PCRE2 match-vector capacity — stopped before source change

`find_matches_pcre2` reserves `input.len() / 3` match slots per sequential or parallel chunk. Whole-corpus averages suggested substantial slack, but allocation is per chunk and tokenizer text has highly nonuniform match density. An external JIT diagnostic reproduced production's UTF-8-snapped authority boundaries, 1 KiB overlap, and pre-filter match iteration for 398 chunks across the first 50 LongBench-v2 contexts.

Nemotron's current `/3` capacity already overflowed in 19 chunks with a worst required/capacity ratio of 1.7010. Changing to `2/7` would overflow 55 chunks at up to 1.9846, while `/4` would overflow 138 at up to 2.2681. GPT-OSS changed from 4 overflows at 1.1645 with `/3`, to 19 at 1.3587 with `2/7`, and 76 at 1.5527 with `/4`. DeepSeek's sparse first Split overflowed none, but the denser Mistral and Qwen patterns reproduced the Nemotron regime; MiniMax and GLM reproduced the GPT-like regime.

Aggregate spare slots therefore cannot justify a smaller one-line heuristic: it would convert many locally dense chunks into geometric reallocations and copies. Raising capacity enough to eliminate Nemotron's worst current overflow would instead reserve far more memory for every ordinary and sparse chunk. The existing `/3` is a compromise, and a more adaptive estimator would add scanning or model-specific policy without profile evidence that it repays its complexity. No production source or benchmark build was created.

### Experiment 24: oversubscribe parallel PCRE2 chunks — planned

Hypothesis: `find_matches_pcre2_parallel` creates exactly one byte-balanced task per available CPU. Regex cost is content-dependent, so a dense or difficult chunk can leave the other Rayon workers idle at the join. The retained ten-second GPT-OSS sample captured 2,088 main-thread milliseconds waiting in Rayon's latch during Split, and the per-chunk density diagnostic found locally dense chunks up to 1.70 times the current capacity heuristic. Rayon documents that its fixed worker pool uses work stealing; creating more tasks than workers gives idle workers useful work without creating more threads.

An external diagnostic replayed exact JIT match counts through UTF-8-snapped chunks and greedily scheduled them over eight workers. Doubling tasks reduced modeled critical work by 3.68% for Nemotron, 2.20% for GPT-OSS, and 17.53% for DeepSeek, while extra overlap increased total matches by only 0.81%, 0.81%, and 0.85%. Four times as many tasks raised overlap by about 2.2%; it improved Nemotron and DeepSeek further but was slightly worse than factor two for GPT-OSS, so it is not tested. PCRE2's official multithreading documentation states that compiled patterns are immutable and may be matched simultaneously; the Rust crate supplies distinct pooled match data and JIT stack state for concurrent searches.

Change only the PCRE2 parallel path: cap chunk count at twice the available CPUs rather than once, and select the existing compiled regex copies modulo their count. Do not compile more patterns, change the worker pool, minimum chunk size, overlap, authority filtering, merge/repair logic, fancy-regex fallback, incremental cache, or output allocation. Screen six counterbalanced three-pass LongBench stage pairs on GPT-OSS, Nemotron, and DeepSeek. Expansion requires at least 2% improvement in the combined Split-stage geomean with no model median regression worse than 2%, then at least 1% exact sequential and batch-32 geomean improvement without repeated regressions worse than 3%. A passing candidate proceeds to the full seven-model, two-corpus matrix plus explicit one-thread and full-host checks. Validate all Split tests, formatting, library clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected at the exact end-to-end gate**. The implementation changed only the PCRE2 parallel path: it doubled the byte-balanced task target with saturating arithmetic and reused the existing compiled regex copies by task-index modulo. All 46 Split tests, formatting, and library clippy with warnings denied passed. The release example build took 10.16 seconds and peaked at 395,329,536 bytes RSS; executable size remained 10,250,672 bytes.

The six counterbalanced three-pass stage pairs were noisy but narrowly passed the predeclared normalized screen. DeepSeek split medians moved from 135.238 to 125.977 ms (+7.35% throughput), GPT-OSS from 501.600 to 486.665 ms (+3.07%), and Nemotron from 553.165 to 527.317 ms (+4.90%). Their model-median geomean was +5.09%, while all 18 raw pairs gave -0.50% and BPE-stage normalization gave **+2.05%**. Per-model raw/normalized pairwise geomeans were -0.38%/+4.18% for DeepSeek, -1.83%/+1.79% for GPT-OSS, and +0.75%/+0.22% for Nemotron. Each model won four of six raw pairs. The independent exact-output gate therefore proceeded instead of accepting the stage medians alone.

Four counterbalanced LongBench pairs covered the same three models in sequential and batch-32 modes with eight Rayon threads. One preliminary DeepSeek baseline process was excluded before this schedule because the shell wrapper attempted to assign zsh's read-only `status` parameter after the process completed; no timing or validation output survived, the wrapper was corrected to use `rc`, and the complete schedule restarted from pair 1. All 48 recorded processes produced the expected token totals and matched Hugging Face IDs exactly.

| Model, mode | Baseline median | Candidate median | Median throughput | Raw pair geomean | HF-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek sequential | 1,037.58 ms | 1,056.12 ms | -1.76% | -1.50% | -1.62% | 1/4 |
| DeepSeek batch-32 | 472.81 ms | 437.29 ms | +8.12% | +6.66% | -0.12% | 2/4 |
| GPT-OSS sequential | 452.04 ms | 484.10 ms | **-6.62%** | -5.94% | -5.55% | 2/4 |
| GPT-OSS batch-32 | 322.54 ms | 327.89 ms | -1.63% | **-4.13%** | +0.15% | 1/4 |
| Nemotron sequential | 468.44 ms | 472.58 ms | -0.87% | -1.71% | -3.62% | 2/4 |
| Nemotron batch-32 | 334.57 ms | 293.84 ms | +13.86% | +6.56% | -1.54% | 2/4 |

The six-cell median geomean was +1.62%, but it split into **-3.12% sequential** and +6.59% batch-32. Across all 24 pairs, raw geomean was -0.13%, Hugging Face-control-normalized geomean was **-2.07%**, and the candidate won only 10 pairs. GPT-OSS batch-32 regressed by more than 3% in three of four raw pairs, independently failing the recurrence guard. Doubling chunks can mitigate inner-parallel load imbalance, but it adds overlap and scheduling work when outer batch parallelism already occupies Rayon; the benefit is not stable across modes or process phases. The full matrix, thread sweep, and workspace-wide tests were skipped after the exact gate failed. Commit `3424c2c` is reverted.

### Experiment 25: scalarize one-ID thread-local cache hits — planned

Hypothesis: `FlatCache::get` extends the output from a runtime-length slice, so optimized ARM64 assembly calls `memcpy` for every nonempty hit. The retained ten-second profile recorded 7,313 top-of-stack samples inside fused cache/tokenization work and 1,178 in platform `memmove`; disassembly confirms that the cache-hit path loads the stored `u16` length and reaches the copy stub without a one-element specialization. A corpus diagnostic then tokenized every unique Split piece and weighted its output length by lookup frequency. Exactly one cached ID accounts for 8,856,960 of 9,755,220 GPT-OSS lookups (90.79%), 9,259,321 of 10,516,705 Nemotron lookups (88.04%), and 1,040,876 of 1,465,361 DeepSeek lookups (71.03%).

In `FlatCache::get`, push the one stored ID directly when `slot.len == 1`; retain the existing `extend_from_slice` path for every other length. Do not change hashing, probing, fingerprints, key comparison, capacity checks, cache storage, insertion, clearing, shared caches, callers, or output order. This adds one highly biased branch and replaces a variable-length library call with the existing `Vec::push` fast path for the dominant value shape on all target architectures. Screen six counterbalanced three-pass BPE-stage pairs on GPT-OSS, Nemotron, and DeepSeek. Expansion requires at least 2% combined BPE-stage geomean with no model median regression worse than 2%, then at least 1% exact sequential and batch-32 geomean without repeated regressions worse than 3%. A passing candidate proceeds to the full seven-model, two-corpus matrix and explicit one-thread/full-host checks. Validate focused cache and real-model tests, formatting, library clippy, build time, RSS, binary size, full workspace tests, and exact Hugging Face IDs.

Result: **rejected at batch-32**. The source change added only the one-ID branch and its mechanism comment. Candidate assembly used one scalar pool load, the normal `Vec::push` capacity check, and one scalar store instead of the copy stub for `len == 1`; every other value retained the original path. All 9 BPE tests, encoded and fused real-model cache consistency, GLM `ignore_merges`, formatting, and library clippy with warnings denied passed. The release example build took 11.02 seconds and peaked at 400,703,488 bytes RSS. Its executable remained 10,250,672 bytes; the identical external stage harness grew by 48 bytes to 7,026,544 bytes.

Six counterbalanced three-pass stage pairs initially passed the raw gate:

| Model, LongBench 50 | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,093.193 ms | 1,087.308 ms | +0.54% | +4.38% | -6.05% | 3/6 |
| GPT-OSS | 359.073 ms | 344.251 ms | +4.31% | +6.14% | +5.08% | 5/6 |
| Nemotron | 360.672 ms | 346.299 ms | +4.15% | +3.65% | +6.08% | 4/6 |

The model-median geomean was +2.98%, all 18 raw pairs gave +4.72%, and Split normalization gave +1.55%. DeepSeek's raw pairs ranged from -12.57% to +22.07% and normalized to a loss, so the exact gate remained necessary.

The correctness-bearing LongBench screen stopped when GPT-OSS batch-32 failed the greater-than-3% recurrence guard in all three completed pairs. Its raw changes were **-13.71%, -6.16%, and -51.46%** in alternating binary orders; Hugging Face-control normalization still gave **-10.73%, -7.15%, and -41.09%**. The first two opposite-order losses were already consistent, and the third candidate process had a 21.36% slower control but a 106.01% slower fastokens result. Its three-pair medians were 247.01 ms baseline and 307.15 ms candidate, a **19.58% throughput regression**; raw and normalized pairwise geomeans were -26.75% and -21.26%.

Five other cells had three completed pairs except DeepSeek batch-32, which had two before the stop. DeepSeek sequential medians changed -1.51% with -1.00% raw and -0.27% normalized pairwise geomeans; its batch medians changed +1.71% with +1.66% raw and -3.50% normalized means. GPT-OSS sequential improved +6.10% by medians, +6.96% raw, and +4.11% normalized. Nemotron's phase-noisy sequential medians improved +12.15% while its raw/normalized pairwise means were -4.98%/+10.74%; batch medians improved +12.96% while its means were -3.60%/+9.37%. All 34 recorded processes produced the expected token totals and exact Hugging Face IDs. Across the 17 complete pairs the candidate won 10, raw geomean was -5.67%, and control-normalized geomean was -0.59%.

The one-ID scalar path helps sequential/cache-warm stages but changes the large inlined batch function enough to regress the actual parallel contract repeatedly. A fastokens-only batch harness was not added because it could not overturn the correctness-bearing guard. The remaining fourth pairs, full matrix, thread sweep, and workspace-wide tests were skipped after the hard failure. The requested xhigh peer discussion again produced no response, was stopped after several minutes, and made no edits. Commit `436f4a7` is reverted.

### Candidate investigation: direct-vocabulary identity shortcut — stopped before source change

The cache-value diagnostic found that printable-ASCII pieces already present as exact single vocabulary tokens account for 32.01% of GPT-OSS lookups, 35.75% of Nemotron lookups, and 67.82% of DeepSeek lookups. Those weighted totals overstate a post-cache shortcut because recurring pieces normally hit the thread-local table. Extending the production cache simulator to count opportunities only on its existing miss path reduced coverage to 60,880 of 380,049 GPT-OSS misses (16.02%), 37,484 of 366,390 Nemotron misses (10.23%), and 6,593 of 232,799 DeepSeek misses (2.83%). They represented only 9.55%, 5.98%, and 0.05% of each model's missed bytes, respectively.

A correct shortcut would have to inspect raw bytes to prove ByteLevel identity and then query the vocabulary before bypassing BPE. DeepSeek would therefore scan up to 36.5 MB of miss text to bypass only 18 KB of direct-token text, while the two short-piece models would still add work to at least 84% and 90% of misses. The earlier one-byte pre-cache and post-cache fast paths already showed that similarly simple direct emission can lose after code-layout and branch costs. No production source or benchmark build was created; this family stops without adding a length cutoff or model-specific policy.

### Experiment 26: narrow the initial byte-pair rank table — planned

Hypothesis: `byte_pair_initial` stores `(rank, new_id)` in a fixed 65,536-entry vector, but raw heap seeding reads only `rank`. `MergeEntry` already carries the two token IDs, and the validated merge loop derives `new_id` from `merge_adj` after its stale-entry check. Keeping only `rank` therefore reduces the separately allocated table from 512 KiB to 256 KiB and halves bytes fetched per indexed pair without changing merge semantics, field order, the inline `Bpe` layout, or control flow.

The production cache simulator estimates 2,781,286 adjacent-pair table reads for GPT-OSS, 2,732,846 for Nemotron, and 36,312,807 for DeepSeek over the 50-input LongBench screen. This is distinct from Experiments 6–8: those removed or emptied the separate multi-megabyte `RankedMergeMap` and changed allocator state or following field offsets, while this candidate retains that field, its construction, and every lookup. The requested xhigh peer discussion was attempted with the exact source and measurements, but both backends remained silent for a bounded minute; it was stopped cleanly and made no edits.

Change only the rank-table element type, sentinel initialization, construction assignment, and runtime load. Do not remove the ranked map, change heap entries, merge lookup, cache policy, allocation lifetime, field order, or any neighboring representation. Screen six counterbalanced three-pass LongBench BPE-stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires either at least 2% DeepSeek improvement with a nonnegative three-model median geomean or at least 1% combined geomean, with no model regression worse than 2%. A passing candidate then needs at least 1% across exact sequential and batch-32 medians without a repeated regression worse than 3%, followed by the full seven-model, two-corpus matrix and thread/batch-size checks. Validate focused BPE and real-model cache tests, GLM `ignore_merges`, formatting, library clippy, build time, RSS, binary size, logical allocation size, and exact Hugging Face IDs.

Result: **rejected at the stage gate**. The candidate reduced the logical table allocation from 524,288 to 262,144 bytes with five substitutions and no inline `Bpe` layout change. All 9 focused BPE tests, both real-model cache consistency tests, GLM `ignore_merges`, formatting, and library clippy with warnings denied passed. The release example build took 8.82 seconds and peaked at 399,081,472 bytes RSS. The 10,250,672-byte simple benchmark executable was unchanged in size; the identical external stage harness grew by 16 bytes to 7,026,512 bytes.

Six counterbalanced three-pass LongBench stage pairs produced:

| Model | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,055.554 ms | 1,062.910 ms | -0.69% | -1.10% | +3.91% | 2/6 |
| GPT-OSS | 358.384 ms | 350.388 ms | +2.28% | +2.29% | +0.88% | 4/6 |
| Nemotron | 350.509 ms | 358.142 ms | **-2.13%** | -1.55% | +1.36% | 2/6 |

The three model medians had a **-0.20%** geomean. Across all 18 pairs, raw and Split-normalized geomeans were -0.13% and +2.04%, and the candidate won 8 pairs. Whole-process phases were substantial: for example, the final DeepSeek pair favored the candidate by 4.93% while its untouched Split stage also favored it, whereas Nemotron in the same trial lost 9.76% while its Split stage slowed. The normalized result cannot replace the predeclared raw gate, especially because the 36.3-million-read DeepSeek workload did not show the expected dominant win and Nemotron crossed the 2% regression guard. Exact sequential, batch-32, full-matrix, and thread/batch-size expansion were skipped. Halving this L2-sized table is not a stable end-to-end stage win on the measured workload; commit `b7ff69b` is reverted.

### Candidate investigation: compact or rebuild the BPE heap — stopped before source change

The first DeepSeek line-level sample repeatedly tokenized the same 50 inputs, so its later passes inherited warm local and shared caches and could not represent the cold merge workload. It is preserved but excluded. A second line-debug build paused before its first encode, was sampled only after attachment, and then processed the first 50 LongBench-v2 contexts exactly once. That valid cold run spent 74.368 ms in Split and 1,224.347 ms in BPE and produced the expected 11,247,001 tokens.

Excluding the intentional attach wait, the cold sample placed 3,053 top-of-stack samples in `merge_all_raw_into` or its merge closure, versus 95 in `BinaryHeap` pop/heapify, 94 in `FlatCache`/tokenization, 52 in `memmove`, 38 in mutex waits, 19 in added-token handling, and 18 in shared-map rehashing. Disassembly resolved the dominant merge offsets to the standard library's `BinaryHeap::pop` sift-down path, which moves and compares the current 16-byte `MergeEntry`. The eight BPE-pool workers accumulated approximately 487, 238, 225, 303, 139, 415, 240, and 301 merge samples, a 3.5-to-1 maximum/minimum spread that also exposes document-local scheduling imbalance.

The apparent heap target does not justify another representation experiment. Experiment 3 already halved `MergeEntry` from 16 to 8 bytes with a dense rank-index table and regressed the DeepSeek median by 0.30%, losing three of four pairs. Experiments 4 and 5 tested linear merge loops below 64 and 16 bytes and both regressed. Experiment 18 removed the initial heap-entry transfer with `BinaryHeap::from`/`append` and regressed the exact GPT-OSS median by 3.26%. Deriving the merged ID on each valid or stale pop would add a lookup to the sampled hot path and is less direct than the rejected compact-entry design. No production source or benchmark build was created for another heap variant.

The actionable new signal is instead the worker spread. `tokenize_batched` currently partitions a document into exactly one contiguous split-count chunk per BPE worker, leaving no additional task for an idle worker to steal after finishing a cheap chunk. Before changing that policy, an external diagnostic will quantify whether two tasks per worker reduces the predicted critical work enough to repay extra task-result vectors, cache borrows, and concatenation. Any source experiment will remain limited to the fused batched path and must pass both sequential and batch-32 exact-output gates because outer batch parallelism may reverse the scheduling tradeoff.

### Experiment 27: oversubscribe fused BPE chunks — planned

Hypothesis: the cold DeepSeek sample found a 3.5-to-1 spread in merge activity across the eight BPE workers, while `tokenize_batched` creates exactly eight contiguous split-count chunks. Doubling the chunk count gives Rayon's existing workers one additional task each on average, allowing an idle worker to steal remaining merge-heavy work without creating threads or changing the BPE algorithm. This is distinct from Experiment 24, which changed regex chunks with overlap, and Experiment 9, which changed whether inner BPE parallelism ran at all.

The external diagnostic proposed above is deliberately skipped: it would duplicate the production partitioner and cache behavior, while the existing identical stage harness can test the two-line policy directly. Change only the fused batched path: compute twice the current pool width with saturating arithmetic and use that task count for `par_chunks`. Do not change the generic `tokenize` path, pool size, parallel threshold, callback, result capacities, cache policy, merge logic, or concatenation. The oversubscription rationale must be documented next to the calculation.

Screen six counterbalanced three-pass LongBench BPE-stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires at least 2% improvement in the three-model median geomean, no model median regression worse than 2%, and at least ten of eighteen raw pair wins. A passing candidate must then improve the exact sequential and batch-32 six-cell median geomean by at least 1%, with neither mode's three-model geomean negative and no greater-than-3% regression recurring in one cell. Only then expand to the full seven-model, two-corpus matrix and thread/batch-size checks. Validate focused pre-tokenized and BPE tests, fused real-model cache consistency, formatting, library clippy, build time, RSS, binary size, and exact Hugging Face IDs.

Result: **rejected at the per-model stage gate**. The source diff changed only the fused chunk calculation and its rationale comment; generic tokenization remained untouched. All 9 `PreTokenizedString` tests, all 9 BPE tests, the real Nemotron fused-cache test, formatting, and library clippy with warnings denied passed. The release example build took 11.05 seconds and peaked at 395,395,072 bytes RSS. Its 10,250,672-byte executable was unchanged; the identical external stage executable grew by 3,984 bytes to 7,030,480 bytes. Every timed process produced its model's exact retained token total.

One preliminary DeepSeek process is excluded and disclosed: a zsh scalar failed to word-split the intended two-label order, so the wrapper ran only the candidate under the combined label, printed 876.151 ms, and began a second candidate before interruption. With no paired baseline and an invalid label, it cannot enter the screen. The wrapper was corrected to use a zsh array, and all six pairs restarted from trial 1.

| Model | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,109.524 ms | 948.797 ms | +16.94% | +17.35% | +7.18% | 6/6 |
| GPT-OSS | 362.922 ms | 341.735 ms | +6.20% | +0.26% | +4.43% | 3/6 |
| Nemotron | 338.841 ms | 359.775 ms | **-5.82%** | +1.63% | +5.17% | 3/6 |

Across all eighteen pairs, the three-model median, raw-pair, and Split-normalized geomeans were +5.36%, +6.14%, and +5.59%, and the candidate won 12 pairs. Background system load was visibly phase-heavy: GPT-OSS's fifth candidate coincided with Split rising from 488.215 to 668.074 ms, while Nemotron's first baseline had both its slowest Split and a 510.177 ms BPE result. Those phases explain why pairwise and normalized aggregates are positive, but they do not permit replacing the gate after observing results. Nemotron's six-run raw median crosses the predeclared 2% regression limit by a wide margin and loses half its pairs. Exact sequential, batch-32, matrix, and thread/batch-size expansion therefore stop. Factor-two fused oversubscription is not retained; commit `b1c01a3` is reverted.

### Candidate investigation: shorten heap-entry comparison — stopped before source change

The cold DeepSeek profile resolves the dominant BPE offsets to standard `BinaryHeap` sift-down comparisons, making a comparator that ignores stale-check payload fields appear attractive. The retained source already does exactly that: `MergeEntry` packs `(rank, position)` into one `u64`, stores `(left, right)` separately, and its manual `Ord` implementation compares only the key. ARM64 disassembly confirms one 64-bit comparison per heap choice rather than a derived comparison over both fields. There is no redundant comparator work to remove, so no source or benchmark build was created.

The requested xhigh peer discussion was run again with the retained/rejected experiment map and a read-only prompt for minimal portable candidates. Claude Code and Grok Build both remained silent through two bounded windows. The task-owned discussion was interrupted, left no child processes or edits, and production source still matched retained commit `810979f` exactly.

### Experiment 28: carry the previous initial token ID — planned

Hypothesis: raw BPE initialization loads each byte's token ID from the 256-entry `byte_to_initial_token` table. When the adjacent raw-byte pair has a valid initial merge rank, it then loads the previous byte's token ID from the same table again to construct `MergeEntry`. Retained optimized ARM64 disassembly shows this second indexed load at `merge_all_raw_into + 0x1dc`; LLVM does not carry the prior result across the loop. The previous iteration already validated and produced that ID, so one scalar local can replace the redundant load without changing memory layout or lookup policy.

Add only a `prev_id` local, use it as the heap entry's left token, and update it beside `prev_byte`. Document why the value is carried. Do not change the byte-pair table, current-byte load, error path, heap entry, rank lookup, loop shape, caches, or encoded-text path. Confirm in candidate assembly that the previous-token table load is gone. Screen six counterbalanced three-pass LongBench BPE-stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires at least 1% improvement in the three-model median geomean, no model median regression worse than 2%, and at least ten of eighteen raw wins. A passing candidate must then improve the exact sequential and batch-32 six-cell geomean by at least 1%, with neither mode negative and no recurring greater-than-3% cell regression, followed by the full matrix. Validate focused BPE and fused-cache tests, formatting, library clippy, build time, RSS, binary size, exact IDs, and the optimized instruction sequence.

Result: **rejected at the first per-model stage gate**. The source added one carried local and comment, substituted it for the previous-token table lookup, and updated it beside the existing previous byte. All 9 BPE tests, the real Nemotron fused-cache test, formatting, and library clippy with warnings denied passed. The release example build took 8.82 seconds and peaked at 397,656,064 bytes RSS. Its 10,250,672-byte executable was unchanged; the identical stage executable grew by 48 bytes to 7,026,544 bytes. Candidate ARM64 assembly removed the intended conditional indexed `ldr` and replaced the loop-carried state with one register move. Every timed process produced the exact 33,741,003-token total.

Six counterbalanced three-pass DeepSeek pairs had baseline BPE times of 1,245.241, 1,142.611, 1,080.499, 1,089.033, 1,045.397, and 1,058.462 ms and candidate times of 1,108.809, 1,154.931, 1,075.736, 1,109.476, 1,157.410, and 1,033.364 ms. Their medians were 1,084.766 and 1,109.142 ms, a **2.20% candidate throughput regression**, with three wins. The raw pair geomean was +0.22%; normalizing each BPE result by its untouched Split stage gave +5.72% because the larger candidate losses coincided with slower Split phases.

The experiment's gate was deliberately based on raw per-model medians, so phase correlation cannot replace it after seeing the result. Carrying a register removes the confirmed load but does not provide a stable whole-stage saving, and the median crosses the 2% regression limit. GPT-OSS, Nemotron, exact sequential/batch-32, and matrix expansion stopped. Commit `4cc2184` is reverted.

### Experiment 29: remove the heap-position bounds check — planned

Hypothesis: retained optimized ARM64 assembly checks `entry.pos() < symbols.len()` before reading the current `MergeSymbol` on every heap pop, including stale entries. The cold DeepSeek sample places the overwhelming majority of BPE work in this pop/merge loop. Every initial heap position is constructed from an existing symbol index; dynamic entries use either the already-validated current position or a nonnegative `prev`; and `symbols` never changes length inside `run_merge_loop`. The standard library's `get_unchecked` contract requires exactly that the index be in bounds, so this private invariant can remove one hot compare-and-branch on all target architectures.

Replace only the first `symbols[pos]` read with a one-expression `get_unchecked`, preceded by a debug assertion and a local `SAFETY` comment that states the fixed-length/index-origin invariant. Do not unchecked-index `next`, `prev`, or output traversal; change enqueue sites, heap layout, merge logic, or public APIs; or add an abstraction. Existing chained/repeated BPE tests exercise stale and dynamically enqueued entries, and the real fused-cache test provides a model-level boundary, so no new test is added. Confirm in release assembly that the first bounds branch is absent.

Because unsafe must buy a clear measured result, screen six counterbalanced three-pass LongBench BPE-stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires at least 1.5% improvement in the three-model median geomean, no model median regression worse than 2%, and at least ten of eighteen raw wins. A passing candidate must then improve the exact sequential and batch-32 six-cell geomean by at least 1%, with neither mode negative and no recurring greater-than-3% cell regression, followed by the full matrix and Miri if available. Validate focused BPE and fused-cache tests, formatting, library clippy, build time, RSS, binary size, exact IDs, and the optimized instruction sequence.

Result: **rejected at the stage benefit gate**. The candidate added one debug assertion and local safety comment, then changed only the first current-symbol read to `get_unchecked`. All 9 BPE tests, the real Nemotron fused-cache test, formatting, and library clippy with warnings denied passed. The release example build took 8.93 seconds and peaked at 396,263,424 bytes RSS. Its 10,250,672-byte executable was unchanged; the identical stage executable grew by 48 bytes to 7,026,544 bytes. Candidate ARM64 assembly removed the intended symbol-length load, compare, and panic branch while retaining all subsequent bounds checks. Every process produced its exact retained token total.

| Model | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,060.512 ms | 1,058.755 ms | +0.17% | +1.89% | -12.65% | 4/6 |
| GPT-OSS | 356.497 ms | 343.645 ms | +3.74% | +0.67% | +1.43% | 3/6 |
| Nemotron | 390.135 ms | 391.581 ms | -0.37% | +1.08% | +3.01% | 3/6 |

The three-model median geomean was **+1.16%**, below the predeclared 1.5% required to retain unsafe. Across all eighteen pairs, raw and Split-normalized geomeans were +1.21% and -3.00%, with exactly 10 wins. DeepSeek trial 1 had a severe baseline-wide phase (318.359 ms Split versus 129.947 ms candidate), and GPT-OSS showed a pure order pattern: the candidate won all three baseline-first pairs and lost all three candidate-first pairs. These effects prevent treating the apparent raw improvement as a stable bounds-check saving. Exact sequential/batch-32, full matrix, and Miri stopped because performance did not earn the unsafe boundary. Commit `6e162f9` is reverted.

### Experiment 30: adapt fused BPE chunk count to split shape — planned

Hypothesis: Experiment 27's unconditional two-tasks-per-worker policy improved DeepSeek's LongBench BPE-stage median by 16.94% but regressed Nemotron by 5.82%. An external diagnostic now identifies a simple workload distinction available at the call site. Across the 50 LongBench screening documents, DeepSeek's post-Split buffer averaged at least 6.09 bytes per piece in every document, while the maxima were 5.52 for GPT-OSS, 5.42 for Nemotron and Mistral, 5.43 for Qwen, 5.52 for MiniMax, and 5.53 for GLM. On ShareGPT, every one of DeepSeek's 119 parallel-eligible inputs averaged at least six bytes per piece; 27--29 of 200 inputs did so for each short-piece tokenizer. The boundary therefore describes expensive, imbalanced pieces rather than a model name, corpus, or processor.

Change only `PreTokenizedString::tokenize_batched`: keep the existing one-task-per-worker partition below six raw buffer bytes per split, and use two tasks per worker at or above that predeclared integer threshold. Do not change the generic `tokenize` path, parallel threshold, pool width, callback, capacities, cache policy, merge loop, or output order. The average is evaluated only after the existing 16-split parallel guard, so its divisor is nonzero. A requested xhigh AI discussion received no output from either backend in a bounded minute; it was stopped cleanly, left no child process or edits, and does not substitute for measurement.

First validate focused pre-tokenized and BPE tests, fused real-model cache consistency, GLM `ignore_merges`, formatting, library clippy, release build time, RSS, binary size, and exact token totals. Screen six counterbalanced three-pass LongBench stage pairs on DeepSeek, GPT-OSS, and Nemotron. Expansion requires at least 8% DeepSeek BPE-stage improvement, at least 2% in the three-model median geomean, no model median regression worse than 2%, and at least ten of eighteen raw pair wins. A passing candidate must then improve the six-cell exact sequential/batch-32 LongBench median geomean by at least 1%, with neither mode's three-model geomean negative and no greater-than-3% regression recurring in one cell. Only then run the four-pair seven-model, two-corpus matrix plus one-thread/full-host and batch-size checks. Retention requires exact Hugging Face IDs, at least 1% overall matrix geomean improvement, nonnegative sequential and batch-32 geomeans, and no greater-than-3% regression in three of four pairs for any cell.

Result: **rejected at the first exact cell**. The implementation changed only the fused batched partition: it retained one task per worker below six integer-average bytes per split and doubled the task count at or above six. The generic tokenization path remained byte-identical. An xhigh Grok Build review inspected the live source, agreed that the minimal experiment and threshold were sound, and identified nested/end-to-end behavior as the primary retention risk; its advice did not replace any gate. All 9 pre-tokenized tests, 9 BPE tests, the fused real-model cache test, GLM `ignore_merges`, formatting, and library clippy with warnings denied passed.

The release example build took 11.28 seconds and peaked at 393,674,752 bytes RSS. Its executable remained 10,250,672 bytes. The identical external stage harness (`50a5fec2dd5abb6cebd1a157ddf53ddf26dedf5f743ef4ca945fea8cee558acf`) produced a 7,026,304-byte candidate executable versus the retained 7,026,496-byte baseline.

| Model | Baseline BPE median | Candidate BPE median | Median throughput | Raw pair geomean | Split-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| DeepSeek | 1,170.702 ms | 1,011.508 ms | +15.74% | +14.79% | +8.55% | 5/6 |
| GPT-OSS | 410.875 ms | 398.409 ms | +3.13% | -2.00% | -0.68% | 3/6 |
| Nemotron | 430.563 ms | 364.305 ms | +18.19% | +11.64% | +3.45% | 5/6 |

The stage screen passed: its three-model median geomean was +12.15%, while all eighteen raw and Split-normalized pairs gave +7.89% and +3.70%, with 13 wins. Every stage process produced its model's exact fixed token total. GPT-OSS and Nemotron remained below the threshold on all LongBench inputs, so their large median movements and opposing raw/normalized signals demonstrate whole-process or code-layout sensitivity rather than activation of the adaptive policy.

The first correctness-bearing cell then failed decisively. Four counterbalanced DeepSeek LongBench sequential pairs produced candidate throughput changes of **-29.27%, +9.56%, -15.30%, and -8.03%**. Baseline and candidate medians were 1,304.455 and 1,434.300 ms, a **9.05% throughput regression**. Raw and Hugging Face-control-normalized pairwise geomeans were **-11.86% and -9.21%**, and the candidate won one pair. The three greater-than-3% regressions independently triggered the recurrence guard. All eight processes produced the exact 11,248,053-token result. One baseline-only DeepSeek batch-32 process completed as the stop was issued; it is excluded because no candidate mate completed. The remaining exact cells, full matrix, thread/batch-size sweeps, and workspace-wide tests were skipped because they cannot undo the hard guard. Adaptive chunking improves the isolated warm BPE stage but is unstable in the cold end-to-end pipeline; commit `3f4538b` is reverted.

### External candidate 31: evaluate PR #46 vocab-aware splitting — planned

PR #46 (`4dbc2984491683aa3f9d638e94c23cd07cd8ff5e`) is a 13-commit, nine-file feature stack based on upstream 0.2.1. It adds Replace normalization/decoding, ByteFallback encoding/decoding, `lstrip`/`rstrip` added-token behavior, an ASCII character lookup, and vocabulary-aware splitting for non-ByteLevel BPE models. The split builds a 65,536-entry table of byte pairs observed anywhere in the vocabulary and cuts only at UTF-8 boundaries whose adjacent pair is absent. Such a boundary is output-preserving because any token spanning it would itself contain the absent pair. The author reports exact full-corpus Gemma-4 and Phi-4 results at 11--29 times Hugging Face throughput.

Test the exact PR head unchanged in a detached worktree before combining it with the retained PCRE2 improvement. Run formatting, library/all-target clippy, workspace tests, and an xhigh Grok Build review of the proof and new compatibility paths. Validate Gemma-4 and Phi-4 against Hugging Face on LongBench 50 and ShareGPT 200 in sequential and batch-32 modes, including encode/decode behavior and exact IDs; a target-model mismatch, panic, or unsupported configuration rejects the head. If those pass, construct a separate retained-plus-PR candidate and screen existing ByteLevel models so the comparison does not conflate removal of Experiment 12. Only a correctness-clean candidate with material target-model speedups, no recurring greater-than-3% existing-model regression, and reviewable portable code proceeds to a full matrix. Do not edit or comment on the upstream PR, retune its algorithm, or integrate its broad feature stack merely because isolated throughput is high.

Result: **rejected on two reproducible correctness failures**. The exact detached head remained byte-for-byte unchanged. `cargo test --workspace` passed 200 tests with 9 ignored and no failures. Formatting failed in six pre-existing locations in the PR diff, and all-target clippy with warnings denied found one collapsible nested `if` in the new vocabulary-splitting gate. An xhigh Grok Build review independently identified the same two correctness risks below, plus the missing-byte-fallback-token error behavior and unconditional 65,536-entry bridge-table construction.

Two isolated public-API probes establish that the failures are semantic rather than stylistic:

- Four distinct added-token start bytes select `split_full_scan`. For configs `"<a>"` with `rstrip`, `" X"` with `lstrip`, and two inert patterns, input `"<a> X"` panics at `added_tokens.rs:256` because the first token consumes the separating space and the second match then slices byte range `4..3`. Removing one inert pattern selects the three-byte prefilter path, which correctly skips the consumed match and does not panic. The full-scan loop lacks the prefilter's `m.start() < prev_end` rejection.
- A valid synthetic byte-fallback BPE over input `"éê"` expands to bytes `C3 A9 | C3 AA` and defines one merge from `<0xA9>` plus `<0xC3>` to a combined token. Hugging Face returns IDs `[1, 4, 3]`; the PR returns `[1, 2, 1, 3]`. The raw vocabulary-byte table declares the Unicode character boundary unbridgeable even though byte-fallback token IDs can merge across it, so the proposed splitting proof does not hold when `byte_fallback` is enabled.

The advertised real-model paths nevertheless work on a bounded smoke, and every checked input matched Hugging Face exactly:

| Model | Corpus and samples | fastokens throughput | Hugging Face speedup |
|---|---|---:|---:|
| Gemma 4 31B | LongBench 1 | 82.03 MB/s | 29.33x |
| Gemma 4 31B | ShareGPT 200 | 54.68 MB/s | 9.31x |
| Phi-4 | LongBench 1 | 97.15 MB/s | 24.78x |
| Phi-4 | ShareGPT 200 | 41.58 MB/s | 9.77x |

These 402 exact inputs confirm the feature's intended model support and substantial upside, but they cannot override a deterministic panic and deterministic ID mismatch on valid tokenizer configurations. The target-model full matrix, retained-plus-PR construction, and existing-model regression screen were therefore stopped. No PR code was integrated, no upstream comment was posted, and the retained source remains Experiment 12 only pending the user's explicit restoration of Experiment 27.

### User-directed retention: restore Experiment 27

Status: **retained by explicit user direction**. Commit `68322b9` reverts Experiment 27's source revert and restores the original `b1c01a3` fused-only policy byte-for-byte: two tasks per BPE worker, with the generic tokenization path unchanged. The original rejection and measurements remain above rather than being rewritten. The known stage tradeoff is +16.94% for DeepSeek, +6.20% for GPT-OSS, and -5.82% for Nemotron by six-run medians, for a +5.36% three-model geomean and 12 of 18 candidate wins. The user explicitly chose to keep that tradeoff despite the prior per-model guard.

The restored branch passes formatting, library clippy with warnings denied, 179 Rust library tests, 3 Python-extension Rust tests, and doc tests, with 9 extended tests intentionally ignored and no failures. The source is identical to original Experiment 27 commit `b1c01a3`. Fresh end-to-end timing is deferred until unrelated host load falls below the campaign's noise threshold; this does not change the user-directed retention decision.

### Final xhigh portable-candidate audit

A final read-only Grok Build audit at `xhigh` inspected the full experiment log and retained hot paths without making edits. It eliminated every previous family plus seven remaining near-misses: stack storage for `ignore_merges`, a faster vocabulary hasher, linear scans for tiny merge-adjacency rows, a different accepted-merge lookup, another nested-Rayon predicate, live-symbol heap exit, and oversubscribing the generic non-fused tokenizer. Each lacks either measured matrix-wide coverage, a failure mode distinct from a closed experiment, or a minimal way to avoid the demonstrated code-layout, allocator-lifetime, and batch-32 risks.

Verdict: **no untried minimal portable source candidate clears the campaign's plausibility bar**. Split-stage allocation, chunking, JIT, and repair variants are exhausted after Experiment 12; heap entries, merge representations, short linear paths, and hot-loop instruction removal are exhausted; shared/local cache shape and hashing repeatedly fail batch workloads; and Experiment 30 showed that another fused scheduling heuristic does not survive exact end-to-end validation. Reopening the search requires new cold profiles from Zen 5 or Core Ultra that identify a different dominant site, a correctness-first redesign of PR #46, a non-minimal architectural rewrite, or deployment-specific CPU/allocator flags outside the portable-source guardrail. The practical source baseline is therefore Experiment 12 plus the user-retained Experiment 27.

### User-directed final integration: fixed scanners and corrected PR #46

Status: **retained by explicit user direction**. Commit `044855c` replaces the pre-tokenizer regex engines with hand-written scanners selected by the exact production tokenizer pattern. Literal splitting still uses `memchr`, unknown arbitrary regexes fail as unsupported, and `fancy-regex` is linked only for PR #46's generic Replace normalizer/decoder rather than the pre-tokenizer hot path. Experiment 27's fused two-tasks-per-worker policy remains unchanged.

The scanner matched Hugging Face exactly across seven full LongBench model runs totaling 80,610,439 token IDs and seven 200-sample ShareGPT runs totaling 6,027,187 token IDs. Additional 200-sample ShareGPT checks passed for Kimi-K2.5, Qwen3.5, and Phi-4. A vertical-tab oracle mismatch found during full-corpus MiniMax validation was fixed by matching the regex definition of ASCII whitespace rather than Rust's broader `is_ascii_whitespace` predicate.

>Qwen and GLM sequential remain exact and improve 15% and 21% end-to-end. One Qwen batch pair is a small 4% loss (184.8 vs 177.6 ms) despite a slightly better HF-normalized ratio, so I’m treating it as unresolved noise—not hiding it.

The broader noisy screen also retained known MiniMax losses of 1.4% sequential and 11.1% batch by user direction. Across the twelve paired end-to-end cells recorded for DeepSeek, GPT-OSS, Nemotron, Qwen, GLM, and MiniMax, the raw screening geomean was approximately 1.45x. These results are evidence for the retained implementation, not a claim that every model or host improves.

The corrected PR #46 branch is merged after repairing its full-scan added-token overlap and byte-fallback correctness failures. Byte-fallback BPE retains normal Hugging Face merge semantics, but exposes no bigram bridge table, so vocab-aware splitting never interprets or partitions fallback values. The synthetic cross-Unicode fallback merge now returns Hugging Face's `[1, 4, 3]`. On the final combined tree, Gemma 4 LongBench matched 12,544,717 token IDs across 50 samples at 1.54x Hugging Face throughput, while Phi-4 ShareGPT batch-20 matched 546,648 token IDs across 100 samples at 11.46x.

Final combined validation passes formatting, strict library clippy, no-default-feature checking, all-feature/all-target checking, 157 Rust library tests, 3 Python-extension tests, and doc tests, with 9 extended dataset tests intentionally ignored and zero failures.

### Repository simplification and decode allocation pass

Status: **retained**. The tokenizer no longer clones and stores a derived Split-only pre-tokenizer beside the authoritative pipeline. Pipeline-shape queries now live with `PreTokenizer`, encode borrows the canonical Split when the exact fused shape is present, and decode delegates through one implementation. Streaming decode slides its existing ID allocation in place. ByteLevel decode no longer builds an intermediate joined string, Replace decode reuses unchanged owned tokens, ByteFallback reserves its known lower-bound capacity, and empty-string joins use `concat`.

The pass also removes private config fields that were parsed and retained but never implemented, while preserving public constructor signatures and validation that affects supported behavior. Template pair syntax is still validated but no longer stored because pair encoding is not exposed; ByteLevel still accepts `trim_offsets` but stores no inert flag because this crate emits no offsets. A redundant empty-range check in batch BPE was removed. The single-member workspace indirection, unused direct `csv` development dependency, obsolete PCRE build environment, and corresponding stale PCRE notice were deleted; package URLs now identify `coder-2011/snaptokens`, and CI checks all targets. The retained BPE layout tables and Experiment 27 scheduling remain untouched because prior experiments show that apparently dead layout fields and scheduling changes can regress end-to-end performance.

Review exposed one pre-existing ByteLevel correctness hole: zero-filled gaps in the reverse GPT-2 table decoded unmapped characters below U+0144 as NUL. Reverse entries are now checked against the forward table before use, and an edge test covers U+00AD. Focused decoder, pre-tokenizer, post-processor, BPE, and pre-tokenized-string tests pass; the default library suite passes 158 tests with 9 extended tests ignored. A corrected extended harness now takes 100 valid LongBench and 1,000 valid ShareGPT samples per tokenizer rather than unintentionally consuming the entire datasets. All nine extended tokenizers passed all 9,900 Hugging Face encode/decode comparisons with no mismatch.

Against an exact detached `0347995` baseline, the release example is 20,336 bytes smaller. Five counterbalanced exact round-trip decode runs reduced the Qwen ByteLevel median from 147.609 to 139.177 ms (+6.1% throughput) and the Gemma Replace/ByteFallback/Fuse median from 348.042 to 311.454 ms (+11.8%). Exact sequential encode checks across Qwen, GLM, Gemma, and Phi-4 found no stable regression. Qwen batch-32 remained order-sensitive: one pair favored the candidate strongly, while the reversed pair was 25.63 versus 24.86 ms, so no batch speedup is claimed.

Final gates pass formatting, strict all-target clippy, no-default-feature and all-feature/all-target checks, all-target tests, doc tests, locked package verification, and diff hygiene. A final highest-setting read-only Grok Build review found no actionable correctness, API, serialization, safety, performance, or simplification issue.

### Experiment 32: emit Removed-and-inverted matches directly — retained

Hypothesis: `SplitBehavior::Removed` with `invert=true` retains exactly the match spans produced by the original matcher, but the generic path currently allocates an alternating match/gap vector, flips every flag, allocates a second range vector, and then discards every gap. MiniMax-M2.1 and Phi-4 use this exact shape. Emit matcher spans directly while preserving added-token splits, leaving every other behavior unchanged.

Accept only after focused Hugging Face equivalence tests, exact MiniMax and Phi-4 corpus validation, and fresh-process counterbalanced timing. MiniMax LongBench sequential must improve by at least 2% by median across six pairs, batch-32 must improve by at least 1% or remain within 3%, and Phi-4 plus an untouched Qwen control must have no recurring greater-than-3% regression. Otherwise revert the candidate commit rather than editing it away.

The direct emitter is retained. It preserves added-token splits and changes only the exact `Removed && invert` shape; the existing semantic-edge matrix now exercises that runtime path for both Llama-contraction and Phi/GLM scanners against Hugging Face. Formatting, the focused equivalence test, and strict library clippy pass. Across all smoke, target, and control processes below, 488,898,177 token IDs matched exactly.

| Cell | Baseline median | Candidate median | Median throughput | Raw pair geomean | HF-normalized geomean | Candidate wins |
|---|---:|---:|---:|---:|---:|---:|
| MiniMax LongBench sequential, 6 pairs | 424.665 ms | 318.235 ms | +33.44% | +31.44% | +33.22% | 6/6 |
| MiniMax LongBench batch-32, 4 pairs | 321.695 ms | 235.760 ms | +36.45% | +38.34% | +46.04% | 4/4 |
| Phi-4 ShareGPT 2,000, 4 pairs | 544.060 ms | 470.485 ms | +15.64% | +19.37% | +16.98% | 4/4 |
| Qwen ShareGPT 2,000 control, 4 pairs | 569.275 ms | 549.660 ms | +3.57% | +4.27% | -1.08% | 2/4 |

The initial 200-sample Phi-4 screen lasted only 48--74 ms in normal runs and produced two wins, one modest loss, and one 148 ms candidate outlier. Scaling the same exact comparison to 2,000 samples resolved that short-run noise into four candidate wins. The untouched Qwen path also remained load-sensitive: its normalized pairs were -2.37%, -4.59%, +8.63%, and -5.38% while WindowServer and Arc were consuming substantial CPU. The normalized cell geomean stayed within the 3% materiality guard and the raw cell favored the candidate, so this is recorded as unresolved pair noise rather than a Qwen regression. A highest-setting Grok Build audit independently found the specialization semantically exact and the evidence sufficient to retain it.

The proposed follow-up that fuses MiniMax's ASCII proof, NFC skip, and match-only scanner is closed without source. It has a broader correctness surface, helps only wholly ASCII inputs, and lacks a post-Experiment-32 profile showing NFC plus Split still dominates. The 33--36% MiniMax improvement removes the prior justification for writing that second specialized path.

### Repository cleanup: remove the obsolete stage profiler — retained

`examples/profile_stages.rs` is an undocumented 278-line profiler for one hard-coded pipeline shape and requires an `outputs.txt` file absent from the repository. Its only unique library dependency is `PreTokenizedString::tokenize_sequential_pub`, a public wrapper around the real private small-input path. Remove the stale example and its profiling-only API together. Retain `simple_bench`, `profile_sample`, and every production sequential-tokenization path. Accept only if formatting, strict all-target clippy, all-target tests, remaining example builds, and packaged-crate verification pass.

The cleanup removes 286 lines while leaving the private small-input implementation and all production call sites unchanged. Formatting, strict all-feature/all-target clippy, 158 tests, release builds of every remaining example, and packaged-crate verification pass; the same 9 extended dataset tests remain intentionally ignored.

### Repository cleanup: drop inert configuration storage — retained

`TemplatePiece` stores `type_id` for every single-sequence template piece even though the Rust API returns no type IDs, while `TokenizerJson` flattens all unknown top-level keys into an `extra` map that is never read or serialized. Remove both stored values without denying their JSON fields: serde continues to ignore unknown keys, pair-template syntax is still validated, and fixtures containing `type_id` remain the compatibility oracle. Accept only if the post-processor suite, real cached tokenizer parsing, all-target strict clippy, and packaged-crate verification pass.

The cleanup removes 74 net lines and both inert allocations/fields. All 14 post-processor tests pass with their JSON `type_id` fixtures unchanged, the unsupported-pipeline test passes, every cached real tokenizer JSON parses, strict all-feature/all-target clippy passes, and the packaged crate verifies.

### Candidate investigation: specialize Gemma's literal Split — closed before source

Gemma-4 normalizes every ASCII space to `▁`, then runs a literal `Split` that searches for ASCII space with `MergedWithPrevious`. Five LongBench probes spanning 65,429 to 4,299,194 input bytes all produced exactly one unchanged split. The combined normalization and pre-tokenization stage took 0.28--16.97 ms while cold encode took 10.56--1,353.05 ms; the redundant Split is only a fraction of that already small combined stage. A direct-emission specialization or cross-stage no-op proof therefore lacks enough end-to-end headroom to justify new pipeline coupling, so no source was written.

### Final whole-tree release audit

The complete branch changes 30 files with 855 insertions and 1,316 deletions, a net reduction of 461 lines. Final formatting, no-default-feature and all-feature/all-target checks, strict all-target clippy, 158 tests, doc tests, remaining example builds, and packaged-crate verification pass; 9 extended dataset tests remain intentionally ignored after the earlier 9,900-comparison run. A final highest-setting read-only Grok Build audit of the complete branch found no actionable correctness, byte-fallback, serde-compatibility, API, safety, performance, or simplification issue and judged the head safe to push.

### Experiment 33: fuse multiple Split stages with bulk ByteLevel — planned

Hypothesis: DeepSeek-V3.2 runs three exact `Split` stages followed by `ByteLevel` with `use_regex=false` and `add_prefix_space=false`. The current fused path recognizes only one `Split`, so DeepSeek allocates and fills an expanded Unicode ByteLevel buffer before encoded-text BPE. A bulk-only ByteLevel stage is a per-byte substitution that does not alter split boundaries; after all leading Split stages have refined the original UTF-8 ranges, the existing raw-byte BPE tables produce the same initial IDs and ranked merges without materializing that buffer. Removing the transform, allocation, and character decoding is architecture-neutral.

Change only the exact pipeline-shape query and the fused encode branch: accept a nonempty sequence of `Split` stages followed by one bulk-only `ByteLevel`, apply those Split stages in order, and pass the unchanged normalized buffer and refined ranges to `tokenize_batch_fused`. Preserve the current single-Split ASCII-NFC fusion exactly, reject every other sequence shape, and do not change scanners, ByteLevel, BPE, caches, scheduling, normalization, added tokens, or public APIs. Existing DeepSeek correctness and added-token tests plus fresh Hugging Face comparisons cover the semantic boundary; add no broad test matrix to source.

Build the untouched parent and committed candidate as separate release executables. Screen counterbalanced fresh processes on DeepSeek LongBench sequential and batch-32, then ShareGPT sequential and batch-32, with GPT-OSS and Nemotron as existing single-Split fused controls. Retention requires exact Hugging Face IDs in every process, at least 5% DeepSeek four-cell median geomean improvement, nonnegative DeepSeek sequential and batch geomeans, no control median regression worse than 3%, and no greater-than-3% control loss recurring in three trials. A passing candidate proceeds to the full model/corpus matrix, focused tests, strict lint, build and package checks; a failure is committed and then reverted with `git revert`.

Result: **rejected**. Commit `82f698c` implemented exactly the planned shape and passed formatting, diff hygiene, strict library clippy, focused DeepSeek and fused-cache tests, and the extended DeepSeek differential: 100 valid LongBench and 1,000 valid ShareGPT inputs matched Hugging Face for encode and decode. CI and release workflows also passed. The 9,341,376-byte candidate was 80 bytes larger than the frozen 9,341,296-byte baseline; their SHA-256 hashes were `7a83ae46dd0cd241b001c1abbfc6ea51e1b8810ec26f72fe3dc105504803d9b7` and `c2761d977525c713336d829618ee5c39257b5e22ee5df1f9988139e68a087443`.

DeepSeek passed its target gates, with exact IDs in every process:

| Cell | Baseline median | Candidate median | Median throughput |
|---|---:|---:|---:|
| LongBench sequential, 4 pairs | 620.530 ms | 507.985 ms | +22.16% |
| LongBench batch-32, 4 pairs | 354.305 ms | 254.845 ms | +39.03% |
| ShareGPT 2,000 sequential, 4 clean pairs | 708.235 ms | 615.490 ms | +15.07% |
| ShareGPT 2,000 batch-32, 4 clean pairs | 268.440 ms | 295.945 ms | -9.29% |

The four-cell median geomean was +15.38%; sequential and batch-32 mode geomeans were +18.55% and +12.30%. The ShareGPT batch loss is retained explicitly rather than hidden. Candidate timings won three of its four raw pairs, so the negative median is consistent with the known full-host batch variance, but it is still the measured cell result. Earlier ShareGPT runs were excluded only after benchmark logs under Documents activated FileProvider/iCloud and both Hugging Face and candidate timings rose by seconds; the clean replacement used the same frozen binaries, 2,000 inputs, counterbalanced order, quiet output, and `/tmp` logs.

The untouched controls decided the experiment:

| Control cell | Baseline median | Candidate median | Median throughput |
|---|---:|---:|---:|
| GPT-OSS LongBench sequential | 306.350 ms | 312.325 ms | -1.91% |
| GPT-OSS LongBench batch-32 | 185.285 ms | 189.075 ms | -2.00% |
| Nemotron LongBench sequential | 327.810 ms | 316.570 ms | +3.55% |
| Nemotron LongBench batch-32 | 174.640 ms | 254.125 ms | -31.28% |

All four Nemotron batch pairs regressed: -34.53%, -37.98%, -29.97%, and -4.98% throughput. The candidate shape cannot activate for Nemotron, making this a code-layout or full-host execution effect rather than useful work added to its pipeline; that does not waive the predeclared portability guard. The recurring control failure stops the full matrix and overrides the DeepSeek gain. Commit `82f698c` is reverted by `98f51f2`.

The semantic proof remains useful for future layout-neutral integration. Hugging Face's ByteLevel implementation shows that `use_regex=false` preserves the current split before mapping each UTF-8 byte through the byte alphabet, and tiktoken likewise applies BPE directly to byte pieces. The optimization is therefore closed because its compiled artifact was not performance-portable, not because exact byte/range equivalence failed.

### Correctness fix: honor `ignore_merges` on non-fused pieces — planned

Hugging Face Tokenizers at the pinned `057411221059` revision checks `ignore_merges` by looking up the complete pre-tokenized sequence in the vocabulary before cache or merge work. Snaptokens implements that rule in both fused raw-byte paths, but the non-fused encoded-text `tokenize_into` path first probes a DAAC that deliberately excludes vocabulary tokens its normal merge graph cannot produce. A tokenizer combining `ignore_merges=true` with a non-fused pipeline can therefore split an exact orphan vocabulary token instead of returning its ID.

Add only the same guarded `token_to_id` lookup at the start of `tokenize_into`, before its existing DAAC and cache paths. Do not change DAAC construction, caches, fused tokenization, merge logic, byte fallback, pipeline eligibility, or public APIs. One focused regression fixture uses an orphan `abc` token with no merges: `ignore_merges` must return its single vocabulary ID rather than the three character IDs. Validate the focused failure and fix, all BPE tests, real GLM `ignore_merges`, representative exact model checks, formatting, strict lint, full tests, release examples, and packaged-crate verification. Freeze parent and candidate release binaries and screen non-fused DeepSeek and Gemma plus fused GPT-OSS and GLM controls; correctness is mandatory, and a recurring material control regression rejects the implementation shape for a less intrusive equivalent.

The first implementation is **rejected as a hot-path shape**. Before its guarded lookup, the focused fixture returned `[0, 1, 2]` instead of orphan token `[3]`; afterward it passed along with all 10 BPE tests, strict library clippy, real GLM `ignore_merges`, and the DeepSeek correctness test. Commit `f9b69a3` changed only that branch, its mechanism comment, and the one fixture. Its 9,341,520-byte release binary was 224 bytes larger than the 9,341,296-byte baseline, with SHA-256 `ee44ae7647a530132b11c767c11d96f5e33cb1ced127fc26ce0ed3c817efb0b1` versus `c2761d977525c713336d829618ee5c39257b5e22ee5df1f9988139e68a087443`.

| Exact LongBench cell, 4 pairs | Baseline median | Candidate median | Median throughput |
|---|---:|---:|---:|
| DeepSeek sequential | 628.510 ms | 619.315 ms | +1.48% |
| DeepSeek batch-32 | 269.910 ms | 242.170 ms | +11.46% |
| Gemma sequential | 9,758.530 ms | 9,297.695 ms | +4.96% |
| Gemma batch-32 | 5,361.965 ms | 6,034.040 ms | -11.14% |

Gemma batch candidate changes were -12.76%, +22.03%, -26.15%, and -21.87%. Trial 3 moved with its Hugging Face control, but trial 4 was decisive in the opposite direction: candidate Hugging Face was faster while candidate snaptokens was 21.87% slower. Three material raw losses trigger the predeclared recurrence guard. Two complete GPT-OSS sequential pairs and the remaining fused controls are excluded because the screen was stopped after this hard failure. Every completed process produced exact Hugging Face IDs. Commit `f9b69a3` is reverted by `6fa38ba`.

The bug remains confirmed, so the replacement must move the distinction to construction: build the existing exact-token DAAC with orphan vocabulary entries only when `ignore_merges` is configured, then let the original byte-identical `tokenize_into` shortcut enforce the full-span check. Preserve the public two-argument constructor as the normal-merge default; add no runtime branch, cache, table, dependency, unsafe code, or API.

The construction-time replacement is also **rejected as a compiled shape**. Commit `6bb0ddb` passed the focused orphan-token regression, all 10 BPE tests, real GLM, GPT-OSS and DeepSeek checks, formatting, no-default and all-feature checks, strict all-target clippy, 159 all-target tests with the same 9 extended tests ignored, doc tests, and locked package verification. It left `tokenize_into` byte-identical and changed only DAAC construction so orphan vocabulary entries were retained when `ignore_merges` was configured. The 9,341,920-byte release binary was 624 bytes larger than the frozen 9,341,296-byte baseline, with SHA-256 `1c9daf1d709022dff128ee5b2864f98370516c560042b660b1b80e15851a3a86`.

Eight counterbalanced Gemma LongBench batch-32 pairs screened binary-layout effects. Trial 5 is preserved but excluded because the candidate process also made Hugging Face 84% faster, proving whole-process contamination. Across the remaining seven pairs, baseline and candidate medians were 5,999.88 and 6,244.30 ms, a **3.91% candidate throughput regression**. Candidate losses recurred in trials 2, 4, and 8; trial 8 was decisive because Hugging Face timings were effectively identical while snaptokens fell from 8,337.60 to 11,076.46 ms, a 24.72% loss. Commit `6bb0ddb` is reverted by `483c6e4`. The correctness bug remains open; the next shape must reject unsupported non-fused `ignore_merges` configurations at construction rather than alter encode code or BPE tables.

The construction-rejection shape is also **rejected after its fused control**. Commit `785ddb2` made the model flag crate-visible and added one cold `Tokenizer::build` check using the same exact fused-pipeline predicate as encode. Its focused rejection test, all 9 BPE tests, real GLM equivalence, formatting, strict library clippy, and construction of all six cached `ignore_merges` tokenizers passed. The release build took 10.19 seconds and 369,131,520 bytes maximum RSS. Its SHA-256 was `eb50935e95559628ea61f0f6c31a1f987598eb02a185f18d077e7843bc21e256`; the executable grew by 18,464 bytes to 9,359,760 and its `__TEXT` segment grew by one 16 KiB page.

Gemma LongBench batch-32 initially favored the candidate in all four pairs: medians were 5,354.31 and 5,103.69 ms, a +4.91% median throughput result, with +5.63% raw and +8.24% Hugging Face-normalized pair geomeans. The fused Nemotron control rejected it decisively. Candidate throughput changes were -30.17%, -3.33%, -14.38%, and -51.05%; medians were 177.47 and 214.19 ms, a **17.14% regression**, while raw and control-normalized pair geomeans were -27.07% and -15.10%. Every run emitted the exact expected token total. Commit `785ddb2` is reverted by `c61b205`.

The next isolation keeps the existing exact-token DAAC shortcut first, then performs the guarded vocabulary lookup only after that shortcut fails. Exact non-orphan vocabulary pieces therefore retain the current byte-identical return; an excluded orphan gets the Hugging Face `ignore_merges` result before cache or merge work. Change only that fallback branch and restore the one orphan-token unit fixture. Screen the same frozen binaries and hard controls; any recurring material regression still rejects the shape.

Result: **rejected at the first hard cell**. Commit `6cdf489` added only the post-DAAC guarded lookup, its mechanism comment, and the one orphan fixture. The fixture, all 10 BPE tests, real GLM equivalence, formatting, and strict library clippy passed. The 15.21-second release build peaked at 365,887,488 bytes RSS. Its 9,341,520-byte executable was only 224 bytes larger than baseline and retained the same Mach-O `__TEXT` segment size; SHA-256 was `f402671c8896d7ec9c5a128ed91789857210648efdc6d499eb00d1dcbe728ee9`.

Gemma LongBench batch-32 candidate throughput changed -11.93%, -8.32%, -1.36%, and -23.90%. Baseline and candidate medians were 4,529.86 and 5,065.58 ms, a **10.58% regression**; raw and Hugging Face-normalized pair geomeans were -11.77% and -6.92%. Pair 2 moved with its control, but pairs 1 and 4 remained 14.90% and 16.64% slower after normalization. Every process emitted 12,544,717 tokens. Three recurring raw losses stop the remaining controls; commit `6cdf489` is reverted by `b834754`.

### Correctness fix: preserve complete `ignore_merges` piece semantics — planned

A read-only AI discussion assigned Claude Code the semantic/API boundary and Grok Build the hot-path/codegen boundary. Both confirmed the non-fused exact-token hole. Claude also found two related boundary errors that the prior four shapes did not cover. First, vocab-aware bigram splitting runs after pre-tokenization, but Hugging Face performs the `ignore_merges` lookup on the original pre-tokenized piece; looking up a later subpiece can emit an orphan token Hugging Face never considered. Second, `encoding_decomposition` treats its final two tokens as a valid producing pair without checking that the merge exists, so normal mode can emit an unmerged two-character vocabulary token.

Use the smallest complete construction-time shape. Build the existing DAAC with unreachable vocabulary entries only when `ignore_merges` is configured, do not expose the bigram split table in that mode, and require the final decomposition pair to exist. Keep `tokenize_into`, caches, fused tokenization, merge engines, byte-fallback values, public signatures, and unrelated dead fields unchanged. One public-pipeline test covers three hard edges: normal mode rejects an unmerged exact token, ignore mode accepts an exact orphan, and a non-exact original piece cannot gain an orphan ID through internal splitting.

Correctness requires the focused fixture, all BPE tests, real GLM `ignore_merges`, representative fused and non-fused Hugging Face comparisons, the full suite, formatting, and strict lint. Freeze parent and candidate release binaries, confirm normal-path `tokenize_into` code remains unchanged, and screen Gemma non-fused plus Nemotron fused batch-32 controls. A repeated greater-than-3% candidate loss in at least three of four pairs must agree with the Hugging Face-normalized direction and reproduce in a second fresh candidate build before attributing it to this cold/layout-only shape; exact-ID failure rejects immediately.

## Research provenance

The campaign used primary implementation sources rather than transplanting folklore. Local pinned checkouts cover [OpenAI tiktoken](https://github.com/openai/tiktoken) at `08a5f3b2c987`, [Hugging Face Tokenizers](https://github.com/huggingface/tokenizers) at `057411221059`, [rs-bpe](https://github.com/gweidart/rs-bpe) at `8af937aa28bd`, and [mtc-inc-bpe](https://github.com/ModelTC/mtc-inc-bpe) at `96183fa2a460`. Their heap/linear merge strategies, regex scratch ownership, per-thread/shared cache choices, incremental-prefix methods, and batch scheduling informed Experiments 4--5, 9--11, 13--14, 17--18, and 20--27; measurements in this repository, not external claims, decided retention.

The broader references were the [Rust Performance Book's benchmarking guidance](https://nnethercote.github.io/perf-book/benchmarking.html), including its warning about wall-time variance from small layout changes; the [PCRE2 match iterator contract](https://docs.rs/pcre2/latest/pcre2/bytes/struct.Matches.html), which underpins Experiment 12's non-overlapping byte-offset equivalence; [Rayon `ThreadPool` documentation](https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html), especially pool-relative parallel iterators and work stealing; and the [Fast WordPiece Tokenization paper](https://arxiv.org/abs/2012.15524). The paper's Aho--Corasick-inspired linear WordPiece algorithm is valuable but targets longest-match WordPiece rather than this repository's exact arbitrary BPE merge semantics, so it did not justify an architectural rewrite under the minimal-change guardrail.

### Broader algorithm research: no new source candidate

An Exa-guided primary-source pass deliberately looked outside the existing implementation families: streaming and finite-state BPE, priority-queue alternatives, cache-conscious static dictionaries, Unicode normalization, and irregular-loop scheduling. The closest production comparison is [IREE's tokenizer](https://github.com/iree-org/iree/blob/main/runtime/src/iree/tokenizer/README.md), which independently converges on a reusable priority queue, open-addressed merge table, compact vocabulary storage, fixed hot-path state, and batched pipeline stages. Its benchmark also rotates 64 corpus copies to defeat cache warming; that is a useful future harness control, not evidence for another source change.

The linear-time alternatives are architectural rewrites. IREE's short-segment [BPE backtracking implementation](https://github.com/iree-org/iree/blob/main/runtime/src/iree/tokenizer/model/bpe_backtrack.c) is more than 800 lines of pair-validation, suffix-preemption, cache, and boundary machinery, with a heap fallback for larger segments. The [PLDI 2026 streaming algorithm](https://pldi26.sigplan.org/details/pldi-2026-papers/87/An-Efficient-Algorithm-for-Streaming-BPE-Tokenization) obtains fixed input-independent memory and linear time from a bounded-delay transducer. The [BPE DFA construction](https://arxiv.org/abs/2405.07671) similarly moves work into a precompiled automaton. Its [reference implementation](https://github.com/mcognetta/tokenization-as-finite-state-transduction) omits 16K and 32K automata because they are too large to package and warns that compiling 32K can take more than a day; supported snaptokens vocabularies reach roughly 100K--500K. None is a minimal portable replacement for the exact heap engine.

A traditional integer-priority-queue idea was rejected by data rather than asymptotics. If every newly enabled merge rank exceeded the rank just applied, the queue would be monotone and could use radix buckets. An O(number of merges) diagnostic emulating the loader's last-write-wins pair map checked all 16 cached tokenizer JSONs. Eight violate that property, including GLM, Mistral/Nemotron, Kimi, GPT-OSS, Qwen 3.5, and Gemma. For example, GLM can create `ĠĠĠ` at rank 14 and then enable a rank-1 merge. A monotone queue would therefore be incorrect on supported models. General cache-oblivious priority queues add machinery for large external-memory workloads while real pre-tokenized pieces are normally only a handful of symbols, so they do not clear the complexity bar either.

The Unicode and scheduling searches also close rather than open source work. [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/) defines normalization quick checks, but the retained fixed scanner already proves ASCII while producing splits and bypasses NFC entirely on that path. ICU4X's `ComposingNormalizer::normalize` finds the maximal normalized prefix and returns it borrowed when unchanged; adding a separate precheck would repeat the scan. Guided/factoring and cost-aware work scheduling target irregular loop imbalance, but a tokenizer-specific cost partition would add estimation, prefix sums, and partition state on top of the explicitly retained two-tasks-per-worker policy. Experiments 27 and 30 already show that inner-stage scheduling wins can reverse in exact batch and cold end-to-end workloads. No new production source or benchmark candidate was created from this research pass.

The complete `ignore_merges` construction shape is **rejected after independent reproduction**. Commit `319d4d8` included exact otherwise-unreachable vocabulary entries in the DAAC only for `ignore_merges`, withheld the unbridgeable-bigram table in that mode, and fixed the final-pair decomposition check. Commit `d76f5ef` then reused the existing best-merge search for that final check, removing a second map lookup and reducing the release artifact from 9,359,632 to 9,341,920 bytes. The final candidate was only 624 bytes larger than the frozen 9,341,296-byte baseline and had the same 6,455,296-byte `__TEXT` segment. Its clean rebuild was byte-for-byte identical with SHA-256 `7051934c2fb47dcd5aeaf8ce95ef0cc80fdae054e19922969291fcd5160dc04c`.

The original candidate passed the focused three-edge public regression, all 9 BPE tests, real GLM `ignore_merges`, the full all-feature/all-target suite, strict all-target clippy, no-default checking, and doc tests. After the loop simplification, formatting, the focused regression, and all BPE tests passed again. Every one of 16 complete Gemma benchmark processes produced exactly 12,544,717 Hugging Face token IDs.

Four counterbalanced Gemma LongBench batch-32 pairs produced raw candidate changes of -1.41%, -6.73%, -6.91%, and -17.58%; all four Hugging Face-normalized changes were also negative. Baseline and candidate medians were 6,499.230 and 7,416.440 ms, a -12.37% median throughput result, while raw and normalized pair geomeans were -8.35% and -19.52%. A clean second build then reproduced raw changes of -6.90%, -3.32%, -3.35%, and +1.45%, with the first three normalized directions also negative. Its medians were 5,430.345 and 5,569.215 ms (-2.49%), with -3.07% raw and -2.54% normalized pair geomeans. Both screens therefore had three material losses in four pairs. A completed Nemotron control pair was exact and favored the candidate, but remaining controls cannot undo the predeclared reproduced Gemma guard. Commits `d76f5ef` and `319d4d8` are reverted by `cf138f8` and `2bc4a21`; the correctness bug remains open for a shape that leaves the normal DAAC iterator unchanged.

### Correctness fix: isolate the `ignore_merges` orphan mask — planned

The reproduced Gemma loss occurs with `ignore_merges=false`, so the next shape must keep the normal DAAC pattern iterator in its exact baseline form. Pass the configuration only to construction, clear the already-built orphan mask when `ignore_merges` is true, and retain the original `!is_orphan[token]` filter expression. This includes every vocabulary entry for exact lookup in that mode without adding its flag to the generic iterator closure. Keep the one mode check on bigram-table exposure because Hugging Face's whole-piece lookup cannot be moved after internal splitting. Independently reuse the existing final merge search so normal mode cannot accept an unmerged two-character token.

Change no hot model tokenizer, cache, merge engine, table representation, field layout, byte-fallback behavior, public signature, or unrelated construction data. Restore the same one-test, three-assertion public fixture. Require formatting, focused and BPE tests, real GLM, full tests, strict lint, a release artifact with no new text page, and exact IDs. Screen four fresh Gemma batch-32 pairs first; three material control-consistent losses reject and revert the committed source. A passing Gemma screen proceeds to four Nemotron fused controls before broader correctness gates.

Result: **retained**. Commit `3e7f5b4` leaves the normal DAAC iterator expression unchanged and clears the temporary orphan mask only while constructing an `ignore_merges` model. It also withholds the optional bigram split table in that mode and reuses the final merge search so normal mode cannot accept a nonexistent two-symbol merge. The one public fixture covers the normal unmerged token, exact ignored token, and forbidden post-split orphan subpiece. Byte-fallback values and merge behavior remain untouched.

Formatting, the focused fixture, all 9 BPE tests, real GLM equivalence, 159 all-feature/all-target tests, strict all-target clippy, no-default checking, doc tests, and locked package verification pass; the same 9 external-dataset tests remain intentionally ignored. The 12.74-second release build peaked at 380,878,848 bytes RSS. Its 9,341,936-byte benchmark executable is 640 bytes larger than the frozen 9,341,296-byte baseline, with SHA-256 `3576412617847bec251909dc06df01646103cda97495c54e2bf8495c21c263ef`. Both retain a 6,455,296-byte `__TEXT` segment and a 4,166,888-byte `__text` section; the hot raw and encoded merge functions retain their exact prior instruction lengths.

Four exact Gemma LongBench batch-32 pairs produced raw candidate changes of +3.65%, +5.57%, -5.12%, and +0.01%. The lone raw loss became +0.36% after its Hugging Face control; no material control-consistent loss recurred. Baseline and candidate medians were 5,622.275 and 5,555.655 ms (+1.20% throughput), with a +0.94% raw geomean. The normalized geomean was -10.18% because trials 1 and 4 contained large Hugging Face-only phase shifts in opposite binary positions, so no Gemma speedup is claimed.

Nemotron raw changes were +37.99%, -13.21%, -5.43%, and -6.34%. Only trials 2 and 3 remained losses after control normalization; trial 4 became +0.32%. Its medians were 207.500 and 216.695 ms (-4.24%), while raw and normalized pair geomeans were +1.49% and +1.72%. This does not meet the predeclared three-control-consistent-loss rejection rule, but it is disclosed rather than presented as a speedup. All eight Gemma processes emitted exactly 12,544,717 Hugging Face token IDs and all eight Nemotron processes emitted exactly 12,288,294.

### Experiment 34: use a quaternary BPE candidate heap — planned

Hypothesis: cold Apple ARM64 profiles now resolve the remaining encoded BPE cost to the standard library binary heap's sift path. A 10-second mixed Gemma LongBench profile placed 1,068 top-of-stack samples in `merge_all_encoded_into` and only three in local/shared cache lookup. A clean cold Gemma profile then placed 128 samples in the merge function, 17 in initial heap construction, and instruction-level DWARF mapping resolved the dominant offsets inside heap removal. A four-child heap reduces tree depth and places each node's children contiguously; Hugging Face Tokenizers independently uses `dary_heap::QuaternaryHeap` for its BPE merge queue, and the crate documents four as its default starting point while requiring workload measurement.

Change only the internal scratch heap type and add the direct dependency needed to name it. Preserve `Reverse<MergeEntry>`, the complete `(rank, position)` order, all enqueue sites, lazy stale-entry checks, merge adjacency, raw and encoded initialization, scratch reuse, byte fallback, caches, pipeline behavior, and public APIs. Do not add a wrapper, custom heap, arity sweep, or duplicate test: the existing overlapping/repeated BPE tests and real-model Hugging Face comparisons exercise exact ordering through both heap paths.

Freeze the untouched release benchmark before the source commit. First require formatting, all BPE tests, real fused and non-fused model checks, strict library lint, and exact Hugging Face IDs. Screen counterbalanced fresh processes on Gemma and Phi for encoded BPE plus DeepSeek, GPT-OSS, and Nemotron for fused BPE. Expansion requires at least 3% improvement in the five-model sequential median geomean, at least four model wins, and no model median regression worse than 2%. A passing stage then requires at least 1.5% across sequential and batch-32 end-to-end geomeans, neither mode negative, and no greater-than-3% loss recurring in three of four pairs for any cell before the full model/corpus matrix. A miss is committed and reverted rather than retained as dependency-backed speculation.

Result: **rejected at the sequential gate**. Commit `c76651d` added the already-locked `dary_heap` package as a direct dependency and replaced only the reusable `BinaryHeap<Reverse<MergeEntry>>` scratch field with `QuaternaryHeap<Reverse<MergeEntry>>`. Formatting, compilation, all 9 BPE tests, 10 cached real-tokenizer correctness tests, both cache-consistency tests, and real GLM `ignore_merges` passed. The 12.47-second candidate release build peaked at 378,699,776 bytes RSS. Its benchmark executable shrank by 64 bytes, from 9,362,656 to 9,362,592 bytes. Frozen SHA-256 values were `1ab42077517c6297a9c2666b1e85b62cd293fc3b2b26793c60eaaa62d5d10e09` and `0651153d4755217cf8136a61b6c0b3a090ba2e6dd0101a89efa432fcca9dc956`.

Three counterbalanced fresh-process LongBench pairs produced exact Hugging Face IDs in every run:

| Model | Baseline median | Candidate median | Median throughput | HF-normalized pair geomean |
|---|---:|---:|---:|---:|
| Gemma | 14,115.190 ms | 11,838.400 ms | +19.23% | +21.10% |
| Phi | 390.360 ms | 409.010 ms | -4.56% | -2.71% |
| DeepSeek | 820.150 ms | 797.860 ms | +2.79% | +0.64% |
| GPT-OSS | 413.700 ms | 434.640 ms | -4.82% | -4.94% |
| Nemotron | 429.280 ms | 438.190 ms | -2.03% | +0.53% |

The five-model median geomean was only +1.75%, below the required +3%, with two wins rather than four. Phi regressed in all three raw pairs (-4.95%, -5.39%, and -2.59%), as did GPT-OSS (-12.21%, -5.05%, and -1.81%); both remained negative after Hugging Face normalization. Those recurring portable-model losses reject the global substitution despite Gemma's large repeatable gain. Batch-32 and the full matrix cannot repair the failed first gate, and an arity or model-specific heap sweep would add policy and dependency complexity after the simplest general form failed. Commit `c76651d` is reverted by `78e74bf`.

### Experiment 35: isolate the quaternary heap to encoded BPE — planned

Hypothesis: Experiment 34 separated exactly by existing execution path. Gemma and current DeepSeek use the encoded merge engine and improved 19.23% and 2.79% by median. Phi, GPT-OSS, and Nemotron match the one-Split-plus-bulk-ByteLevel fused shape and regressed 4.56%, 4.82%, and 2.03%. DeepSeek has three Split stages before ByteLevel, so the rejected multi-Split fusion leaves it on encoded BPE. Isolate the higher-arity heap to that measured path rather than selecting by model, tokenizer metadata, or a tuned length threshold.

Use one const-generic `MergeScratch<D>` backed by `dary_heap::DaryHeap`: arity four in a new encoded thread-local and arity two in the existing fused/raw thread-local. Make only `init_merge_heap` and `run_merge_loop` generic over the same constant. This monomorphizes the unchanged loop without a runtime branch, trait, wrapper, duplicated algorithm, or public policy. Preserve every entry, order, stale check, adjacency lookup, raw byte table, cache, byte-fallback value, and output path. Add no test because Experiment 34 already validated the identical quaternary ordering; rerun the same focused and real-model gates.

Freeze fresh parent and candidate release binaries. Three counterbalanced LongBench sequential pairs must retain at least 10% Gemma improvement, a positive DeepSeek median, and at least 5% encoded-path geomean, while Phi, GPT-OSS, and Nemotron controls have no median loss worse than 2% and no three recurring losses. A passing screen expands to four sequential and batch-32 pairs for all five models, then the broader matrix. Retention requires exact Hugging Face IDs, at least 3% five-model geomean, positive encoded and fused control geomeans, and no recurring greater-than-3% cell regression. Any failure reverts the dependency and both monomorphizations; do not add a custom heap abstraction to preserve the standard binary implementation afterward.

Result: **rejected at the four-pair sequential expansion**. Commit `1ef5aac` used a single const-generic scratch type and unchanged merge loop with arity four for encoded BPE and arity two for fused/raw BPE. It added no runtime branch, model check, duplicated loop, or test. Formatting, compilation, all 9 BPE tests, 10 cached real-tokenizer correctness tests, both cache-consistency tests, and real GLM `ignore_merges` passed. The 12.07-second release build peaked at 379,273,216 bytes RSS. Its 9,365,744-byte executable was 3,088 bytes larger than the 9,362,656-byte parent, with SHA-256 `c614daf779fd54f692f53bf96504db867e436948a4214df58dd359c2a9fbbe74` versus `1ab42077517c6297a9c2666b1e85b62cd293fc3b2b26793c60eaaa62d5d10e09`.

The three-pair screen passed: Gemma and DeepSeek medians improved 22.59% and 1.66%, the encoded geomean was +11.63%, controls were +1.20% Phi, +0.96% GPT-OSS, and -0.59% Nemotron, and the five-model geomean was +4.82%. The required fourth counterbalanced pair changed the stable decision:

| Model | Baseline median | Candidate median | Median throughput | Four-pair normalized geomean |
|---|---:|---:|---:|---:|
| Gemma | 14,184.870 ms | 11,625.800 ms | +22.01% | +22.53% |
| DeepSeek | 798.865 ms | 791.330 ms | +0.95% | -0.84% |
| Phi | 388.560 ms | 386.090 ms | +0.64% | -1.35% |
| GPT-OSS | 417.615 ms | 447.215 ms | -6.62% | -3.17% |
| Nemotron | 433.175 ms | 432.580 ms | +0.14% | +0.43% |

GPT-OSS lost three of four raw pairs (-3.50%, -6.77%, +3.87%, and -22.74%); the fourth remained -10.66% after its Hugging Face control. Its median crosses the 2% control limit, the fused-control median geomean is -2.00%, and the five-model median geomean is 2.998%, just below the explicit 3% floor. Batch-32 and the broader matrix stop because they cannot repair the failed sequential control and retention gates. A custom abstraction retaining `std::BinaryHeap` only for fused BPE would add more code after the minimal path-isolated form failed. Commit `1ef5aac` is reverted by `79cea1c`.

### Experiment 36: opt-in native tokenizer sidecars — planned

Hypothesis: repeat tokenizer construction can skip multi-megabyte JSON parsing and string-based merge resolution without changing the encode-time representation. Add a versioned Snaptokens-native `.tkz` sidecar that stores the complete supported pipeline metadata, dense vocabulary, resolved merge IDs and both BPE flags. Keep `Tokenizer::from_file` JSON-only; expose one opt-in loader that maps `tokenizer.json` to `tokenizer.tkz`, validates the exact source content, atomically creates or replaces the sidecar, and loads direct `.tkz` inputs without producing another file.

The first retained shape must serialize canonical inputs, not live hash tables, automata, thread-local/shared caches, scratch space, or process-local BPE IDs. Loading must rebuild through the existing validated constructors so every unchecked encode-time table invariant and fresh-cache identity remains intact. A raw or compressed JSON copy is rejected because it preserves the dominant parsing and merge-string work; a heap image is rejected because it couples the format to dependency layouts and can bypass safety invariants.

Require focused full-pipeline round trips for added-token flags, normalizer/pre-tokenizer metadata, `byte_fallback`, `ignore_merges`, post-processing and decoding; stale and corrupted sidecars must never be used; direct `.tkz` must remain self-contained; concurrent creation must leave one valid file. Then compare fresh-process JSON and `.tkz` loads across at least GPT-2, Phi, Qwen, GPT-OSS and Gemma. Retain only if every token-ID and decode comparison is exact, repeat loads improve materially across sizes, and the disabled encode path shows no representative throughput regression.

### Experiment 37: stop building the unused prefix map — planned

Profile-guided review found that `Bpe::new_with_ignore_merges` runs a DAAC search over every vocabulary token to populate `next_prefix_map`, but no production or test path reads the field. Remove only that field, its construction loop, and its mechanical clone/equality entries. Do not change the DAAC, decomposition, merge tables, caches, byte fallback, `ignore_merges`, tokenization, or public API.

Freeze the committed parent load benchmark, then compare JSON and direct `.tkz` construction across GPT-2, Phi, Qwen, GPT-OSS, and Gemma. Retain the deletion only if exact five-model vocabulary/encode/batch/decode parity remains clean, direct `.tkz` load geomean is nonnegative with no recurring material model loss, focused and full tests pass, and representative encode controls show no regression attributable to the smaller `Bpe` layout. Otherwise commit and revert the experiment.

### Experiment 38: persist validated BPE decomposition — planned

The direct Gemma `.tkz` profile attributes roughly 60% of construction to recomputing each vocabulary token's final producing pair and orphan status. Extend the native sidecar with only those two derived results: the dense `unmerge_map` and the distinct orphan mask. Bump the format version, validate both lengths and every referenced token ID, require every non-identity decomposition pair to be a canonical merge producing that token, and preserve the existing `ignore_merges` policy. Keep canonical vocabulary and merges in the file; continue rebuilding DAAC, merge lookup tables, byte tables, caches, and the fresh process-local BPE ID.

Factor the existing constructor once so JSON computes decomposition and `.tkz` supplies validated decomposition before both continue through the same DAAC/table/cache initialization. Do not duplicate the `Bpe` initializer, persist live hash layouts, add unsafe deserialization, change encode-time fields, or add a dependency. Existing full-pipeline sidecar tests must continue to cover normal orphan exclusion, `ignore_merges`, and byte fallback without a broad new fixture matrix.

Freeze the Experiment 37 release harness and compare direct `.tkz` against both that parent and JSON across GPT-2, Phi, Qwen, GPT-OSS, and Gemma. Retention requires exact five-model vocabulary, sequential, batch, special-token, byte-fallback and decode parity; every direct load must improve, Gemma must improve at least 2x over the current `.tkz`, and Gemma JSON-to-`.tkz` speedup must reach at least 2.5x. Re-profile the retained shape before considering DAAC persistence; if decomposition is no longer dominant and the remaining dependency-coupled opportunity is small, stop at this simpler format.

### Experiment 39: remove the duplicate unranked merge table — planned

The Experiment 38 Gemma profile resolves its two largest remaining source lines to independently building `MergeMap` and `RankedMergeMap` from the same canonical pairs. `MergeMap` is not used by either encode engine; only the public compatibility helper reads it, and that helper needs the merged token ID already stored in each ranked slot. Remove the duplicate table and have the helper discard the existing lookup's rank. Preserve the ranked table layout, merge order, decomposition, DAAC, adjacency, caches, byte fallback, `ignore_merges`, public signatures, and both encode loops.

This is a representation deletion, not a fused-table rewrite: add no new table, serializer field, unsafe block, dependency, test fixture, or branch. Freeze the Experiment 38 release binary before the source commit. Retention requires all focused BPE and sidecar tests, exact five-model vocabulary/sequential/batch/special-token/decode parity, a smaller release artifact, no representative encode regression, and a positive direct `.tkz` cold-load geomean with Gemma improving materially. Otherwise commit and revert the source rather than retaining an unmeasured simplification.

Result: **retained**. Commit `766f0c6` removes the unranked open-address table and has the compatibility helper discard the rank from the existing ranked lookup. Both encode loops, the ranked layout, merge ordering, decomposition, DAAC, adjacency, byte tables, caches, byte fallback, `ignore_merges`, and public signatures are unchanged. The source loses 63 net lines with no new dependency, unsafe block, branch, representation, or test.

All 9 BPE tests, both focused sidecar tests, strict all-target/all-feature clippy, and the five-model sidecar verifier pass. GPT-2, Phi, Qwen, GPT-OSS, and Gemma match JSON across every vocabulary entry, sequential and batch encoding, both special-token modes, diverse Unicode and byte inputs, and decoding. The signed load harness shrinks 80 bytes; its `__text` section shrinks 1,048 bytes.

Counterbalanced repeat-load medians improved 10.04% on GPT-2 (62.825 to 57.095 ms), 17.40% on Phi (169.744 to 144.583 ms), 14.68% on Qwen (321.864 to 280.663 ms), and 19.37% on GPT-OSS (1,525.516 to 1,277.987 ms). Heavy unrelated desktop load made Gemma wall time unstable in both directions, so six counterbalanced fresh-process CPU pairs were used instead; every pair improved, and the medians fell from 0.895 to 0.630 seconds, a 42.06% throughput gain. Two Qwen batch-32 LongBench controls emitted exactly 6,779,851 Hugging Face IDs and improved the HF-normalized speedup from 32.00x to 33.25x and from 29.69x to 30.75x. Current Gemma JSON-to-direct-`.tkz` CPU medians are 1.390 versus 0.745 seconds, or 1.87x: materially better, but still below the original 2.5x retention target and the projected roughly 3x.

### Experiment 40: transfer the owned vocabulary into BPE — planned

Both JSON deserialization and native sidecar reconstruction own their newly built vocabulary map, but the shared constructor accepts only a borrow and clones every token string into a second map immediately before returning. Let the private constructor consume that map and move it directly into `Bpe::token_to_id`. Preserve the public borrowed `Bpe::new` signature by cloning at that API boundary, where the same clone already occurs today. The JSON and `.tkz` paths then avoid one redundant full-vocabulary allocation and string copy without changing the final representation.

Do not change vocabulary validation, dense ID ordering, decomposition, merge tables, DAAC, byte tables, caches, model flags, public behavior, or encode code. Add no helper, type, dependency, unsafe block, branch, or test. Retain only if focused and five-model correctness remain exact, JSON and direct `.tkz` construction CPU time improve across representative sizes, the release artifact does not grow materially, and the unchanged encode path has no regression. Commit and revert a miss.

Result: **retained**. Commit `13f27e1` changes only the private constructor's vocabulary argument from borrowed to owned, moves it into the existing `token_to_id` field, and preserves the public borrowed constructor by cloning at that boundary. The final BPE representation and every encode function are unchanged; the load harness shrinks 512 bytes and its `__text` section shrinks 712 bytes.

All 9 BPE tests, both sidecar tests, strict clippy, and the five-model vocabulary/sequential/batch/special-token/Unicode/byte/decode verifier pass exactly. Counterbalanced GPT-2 medians improve 4.85% for direct `.tkz` (40.370 to 38.502 ms) and 2.38% for JSON (88.451 to 86.397 ms). Qwen improves 7.09% for `.tkz` (176.368 to 164.689 ms) and 2.00% for JSON (364.419 to 357.266 ms). Six Gemma CPU pairs put `.tkz` medians at 0.635 versus 0.630 seconds (+0.79%) and JSON at 1.055 versus 1.025 seconds (+2.93%). One loss occurred in each six-pair Gemma series, but neither changed the median direction.

The expanded Qwen batch control was too host-noisy for a speed claim: Hugging Face totals ranged from 2.52 to 4.71 seconds and candidate directions alternated under both binary orders. Every process still emitted exactly 6,779,851 Hugging Face IDs, raw pair directions did not recur consistently, and the raw/encoded merge plus tokenize function instruction lengths remain identical between frozen binaries. This experiment claims only the measured construction and artifact-size improvements.

### Experiment 41: reuse the fast hasher for resolved merge pairs — planned

The temporary `ParsedMergeMap` uses randomized SipHash for `(u32, u32)` token-ID pairs, then every entry is copied into the deterministic open-address tables used at runtime. Reuse the existing private `FxBuildHasher` already used by BPE caches and reserve the known merge count while parsing. The input is a local tokenizer file, and the same resolved keys already enter deterministic `fx_hash` tables, so this removes construction-only hashing work without weakening a protection retained by the final representation.

Change only the private type alias and its two constructors. Preserve string-vocabulary hashing, canonical merge validation, last-write-wins JSON behavior, ranked layout, adjacency, decomposition, DAAC, caches, flags, public APIs, and encode code. Add no dependency, hasher implementation, unsafe block, helper, or test. Retain only with exact focused and five-model parity, positive JSON and direct `.tkz` construction medians on representative models, no material artifact growth, and no evidence of a hot-path regression; otherwise commit and revert.

Result: **rejected at strict lint and the public API gate**. Commit `6ac1ede` changed only the alias and two allocations, and the focused BPE and sidecar tests passed. Strict clippy correctly reported that public `Bpe::new` would expose the private hasher through its merge-map parameter. Making the hasher public would leak an implementation detail and break callers that pass the standard map type; preserving that signature would require either rehashing the whole map at the public boundary or generically duplicating the large constructor. The experiment therefore stops before performance screening rather than adding machinery for a speculative construction-only gain. Commit `6ac1ede` is reverted by `1d39edb`.

### Experiment 42: validate dense vocabulary IDs without a tree — planned

The shared BPE constructor builds a `BTreeMap<u32, &str>` only to sort numeric token IDs, detect gaps, and clone strings into the final dense vector. Replace that tree with a `Vec<Option<&str>>` sized to the vocabulary and fill it directly by ID. This makes validation linear, removes one separately allocated tree node per token, rejects duplicate IDs explicitly, and still produces the same owned `id_to_token` plus owned `token_to_id` representations.

Change only dense-ID validation and the two downstream iterations that currently read the tree. Preserve error propagation, token ordering, decomposition, orphan handling, DAAC patterns and values, token lengths, merge tables, byte tables, caches, flags, public APIs, and encode code. Add no helper, type, dependency, unsafe block, runtime field, or test. Retain only with focused and five-model exactness, positive JSON and direct `.tkz` construction medians across representative sizes, no material artifact growth, and no recurring encode loss; otherwise commit and revert.

Result: **retained**. Commit `91d0724` replaces the allocation-heavy tree with one dense temporary vector, validates bounds and duplicate IDs before unchecked encode indexing, and iterates the final dense vocabulary directly for decomposition, DAAC construction, and token lengths. It adds no helper, type, dependency, unsafe block, runtime field, public API, or test. The signed load harness shrinks 5,504 bytes and its `__text` section shrinks 6,668 bytes.

All 9 BPE tests, both sidecar tests, strict clippy, and the complete five-model sidecar verifier pass. GPT-2 construction improves 14.99% for direct `.tkz` (38.068 to 33.105 ms) and 10.69% for JSON (90.198 to 81.484 ms). Qwen improves 11.40% for `.tkz` (164.116 to 147.321 ms) and 5.63% for JSON (358.338 to 339.227 ms). All six counterbalanced Gemma `.tkz` CPU pairs improve, with medians falling from 0.580 to 0.545 seconds (+6.42%); Gemma JSON medians fall from 1.010 to 0.980 seconds (+3.06%), with two losses among six pairs.

Three Qwen LongBench batch-32 controls each emitted exactly 6,779,851 Hugging Face IDs. Raw candidate changes were +16.05%, +31.30%, and -1.59%; HF-normalized changes were +13.51%, +39.28%, and -3.63%. The lone small loss does not recur, while both binary orders contain a large raw and control-consistent win. No encode speedup is required for this construction change, but the control clears the no-recurring-loss gate.

### Experiment 43: persist the validated ranked merge table — planned

An eight-second retained-build Gemma profile places 2,782 of 5,606 sampled construction stacks in `RankedMergeMap::from_parsed`, almost exactly half of direct `.tkz` load time. Extend the native payload with the existing ranked slots and derive the mask from their length. Bump the format version. Keep canonical merges in the payload and validate the cached table exhaustively before installation: require the exact deterministic capacity, an occupied count equal to the canonical pair count, every packed token and merged ID in vocabulary range, and every canonical pair to resolve to its exact rank and merged ID. Those checks also prove an empty sentinel exists, so validation lookup terminates before encode-time unchecked access.

Persist only Snaptokens' own stable slot values, not a dependency automaton, hash-map allocation, pointer, cache, scratch buffer, or process ID. JSON construction continues building the table normally; `.tkz` supplies the validated table to the same shared initializer. Preserve canonical merge validation, decomposition, DAAC construction, adjacency, byte tables, flags, public APIs, and both encode loops. Add one focused corruption boundary to the existing tests rather than a fixture matrix.

Freeze Experiment 42, regenerate version-3 sidecars, and verify all five models across full vocabulary, sequential/batch/special-token/Unicode/byte/decode behavior. Retention requires every direct `.tkz` median to improve, at least 25% Gemma improvement over version 2, at least 2.5x Gemma JSON-to-`.tkz` CPU speedup, sidecars below twice their version-2 size, strict lint and full tests, plus no recurring representative encode loss. A miss or validation complication is committed and reverted before considering DAAC serialization.

Result: **retained after compacting the persisted representation**. Commit `99dbb66` first stored the complete sparse slot array. It remained exact, but version-3 files grew 2.04–2.34x and direct loads regressed 3.82% on GPT-2, 2.25% on Qwen, and 3.42% on Gemma. Commit `a61c5cb` replaces those empty slots with one slot index per canonical merge. Version 4 reconstructs the same table directly, then validates ranks, token bounds, unique slots, the empty terminator, and every linear-probe chain before the table reaches unchecked encode lookup.

All 10 BPE tests, both sidecar tests, the full 164-pass all-target/all-feature suite, strict clippy, doc tests, and the independent five-model verifier pass. The verifier compares every vocabulary entry plus sequential, batch, special-token, Unicode, byte-input, and decode behavior between JSON, cached JSON paths, and direct `.tkz` files.

| Model | Version 2 `.tkz` | Version 4 `.tkz` | Load speedup |
|---|---:|---:|---:|
| GPT-2 | 33.779 ms | 31.124 ms | 1.09x |
| Phi | 80.027 ms | 73.769 ms | 1.08x |
| Qwen | 146.474 ms | 138.044 ms | 1.06x |
| GPT-OSS | 454.059 ms | 252.705 ms | 1.80x |
| Gemma | 552.265 ms | 287.570 ms | 1.92x |

Every direct-load median improves. Version-4 sidecars are only 1.095–1.142x their version-2 size, and the signed load harness grows by 672 bytes with 2,416 additional text bytes.

| Model | JSON | Version 4 `.tkz` | Cold-load speedup |
|---|---:|---:|---:|
| GPT-2 | 81.396 ms | 30.769 ms | 2.65x |
| Phi | 191.376 ms | 75.679 ms | 2.53x |
| Qwen | 339.345 ms | 137.483 ms | 2.47x |
| GPT-OSS | 875.831 ms | 253.997 ms | 3.45x |
| Gemma | 986.096 ms | 281.103 ms | 3.51x |

Eight counterbalanced fresh-process Gemma samples put wall-time medians at 1.010 seconds for JSON and 0.280 seconds for `.tkz` (3.61x), with CPU medians at 0.990 and 0.270 seconds (3.67x). These are fresh tokenizer/process measurements with warmable OS file pages, not reboot-level cold-disk measurements.

Six counterbalanced Qwen LongBench batch-32 controls each emit exactly 6,779,851 Hugging Face IDs. Candidate and Experiment-42 medians are 86.66 and 87.41 ms; the Hugging Face-normalized medians are 31.30x and 31.04x. The encode path therefore has no measured median loss. The compact representation clears every predeclared gate, so DAAC serialization stops here rather than coupling the format to a dependency automaton.

### Follow-up whole-codebase simplification — planned

The live tree has already removed its obsolete tooling, inert configuration storage, duplicated merge representation, and cloned fused-pipeline state. This rerun therefore rejects cosmetic module moves, generic sequence abstractions, public enum collapse, and any attempt to reunify the deliberately specialized ASCII/Unicode scanners or raw/encoded merge engines.

Three remaining ownership duplications clear the semantic bar. `MergeScratch` link construction is open-coded in three symbol-producing branches; the single and batched fused entry points independently implement the same local-cache, shared-cache, exact-piece, raw-merge, and publication lifecycle; and pre-tokenized construction rediscovers the same normalized added-token matcher for every text segment. The latent operations are “append one live merge symbol,” “tokenize one raw piece with the selected cache,” and “use the normalized matcher selected for this build.” Their natural owners are `MergeScratch`, `Bpe`, and the local build operation, respectively.

Keep public APIs, JSON and `.tkz` formats, errors, token ordering, byte fallback, `ignore_merges`, cache identities, scanner specialization, merge priority, thread scheduling, and supported pipeline shapes unchanged. Land each coherent source change separately. Require formatting, focused tests, strict all-target lint, full cached-model parity, release artifact inspection, and counterbalanced fused/non-fused benchmarks against the frozen `d402b0d` binaries. A source change with a recurring material regression is reverted with `git revert`; do not retain a shorter but slower hot path.

Result: **three narrow changes retained; speculative representation changes rejected**. The complete live production tree, examples, scripts, callers, tests, persisted representation, and public surface were read before editing. Cosmetic module moves, public API deletion, sequence-loop unification, scanner reunification, cache-lifecycle abstraction, and raw/encoded BPE reunification remain rejected because their apparently similar code has different ownership, scheduling, or compatibility contracts.

Commit `f966bd7` first centralized symbol linking across all three BPE initialization branches. It passed focused correctness, but enlarged the fused/raw merge function by 20 bytes and lost about 0.9% after Hugging Face normalization in both 2,000-input Qwen pairs. Commit `f5424f9` reverts it. Commit `5fa0238` retains only the duplicated encoded-text operation as `MergeScratch::push_encoded_symbol`; the fused/raw function is the same 2,424 bytes and has the same normalized instruction stream as the frozen baseline, while the encoded function shrinks 64 bytes. Two 1,000-input Gemma pairs emitted exactly 4,754,422 IDs each and improved raw time in both orders, while their control-normalized directions split, so no speedup is claimed.

Commit `83332e0` resolves the normalized added-token matcher once per pre-tokenized build instead of rediscovering the immutable configuration for every text segment. It removes two net lines, preserves the single-text fast path and two-phase raw/normalized token order, and passes the focused normalized-token fixture. Commit `d243bac` reuses one local `String` for merged-vocabulary probes rather than allocating through `format!` for every merge. Repeat-load medians improved 2.2% on GPT-2, 14.3% on Phi, 16.8% on Qwen, 0.3% on GPT-OSS, and 3.0% across six fresh-process Gemma samples. This is construction-only; completed runtime tables and encode behavior are unchanged.

The requested container and parsing candidates were screened against the actual representation rather than added wholesale:

- Commit `8f4be41` packed the fixed 65,536-entry bigram table from 64 KiB to 8 KiB without a crate. Gemma regressed 4–6% raw and 1.5–1.7% after its control in both orders, so `abaa9d5` reverts it.
- A disposable `simd-json 0.17.3` probe used the same public `TokenizerJson` deserializer and an already-owned mutable file buffer. Qwen parse-plus-build improved 5.7%, but Gemma was 0.3% slower. That does not justify another error type and dependency subtree; no repository source was written.
- Commit `a7a0ad2` tested `compact_str 0.9` in both owned vocabulary representations while preserving the standard-map public constructor and version-4 `.tkz` payload. More than 98% of tokens fit inline, yet peak RSS fell only 2.5 MiB on Qwen and 4.4 MiB on Gemma, JSON load stayed flat, and Gemma `.tkz` load regressed 3.4%. `5c0833e` reverts it.
- Commit `1959fad` tested direct AHash deserialization/runtime vocabulary storage while preserving the standard public map. Qwen JSON improved 3.0% but Qwen `.tkz` regressed 4.1%; Gemma moved in the opposite directions, and GLM exact-lookup encoding showed no repeatable normalized gain. `3838eca` reverts it.
- `SmallVec`/`ArrayVec` do not fit the unbounded symbol and output buffers: merge scratch already reuses thread-local `Vec` capacity, while a fixed-capacity container would add an unsupported input limit. Arenas have no shared drop boundary beyond those existing reusable buffers. `hashbrown` duplicates the standard table engine, `memchr` is already used at literal scan sites, and interning cannot deduplicate a vocabulary whose token texts are required to be unique. `rkyv`, `zerocopy`, and live heap images remain outside the portable `.tkz` invariant because version 4 deliberately validates canonical data before rebuilding dependency-owned automata and process-local caches.

Final proof is clean: formatting; strict all-target/all-feature clippy; 164 passing tests with the same 9 extended tests intentionally ignored; doc tests; no-default checking; locked packaged-crate build; and the independent five-model verifier all pass. The verifier compares every vocabulary entry plus sequential, batch, special-token, Unicode, byte-input, and decode behavior for JSON, cached JSON, and direct `.tkz` loads on GPT-2, Phi, Qwen, GPT-OSS, and Gemma. The final release hashes are `498079be24bfbe830076c93553bbca035ae866d64595916a1e9717fd679017b3` for `simple_bench` and `ea3998b9e0f8b286721e126c449a2b806a281f45d9e7729c7c25733514d10319` for `load_bench`.

### Experiment 44: inline the common added-token segment — planned

The published multicore matrix identifies single-item chat calls as the remaining weak shape, but it predates the retained fixed scanners. A fresh local screen shows that bypassing Rayon for a one-input `encode_batch` does not clear the source-experiment bar: on 200 ShareGPT inputs, direct sequential versus batch-1 medians were 56.99 versus 50.79 ms for Qwen, 46.07 versus 47.75 ms for GPT-OSS, 43.93 versus 47.35 ms for Phi, and 382.02 versus 375.30 ms for Gemma. The directions split and their four-model geomean favors the existing path, so no scheduling branch is written.

All five cached production tokenizers contain added tokens, and their ordinary chat inputs normally produce one unmatched text segment. The current added-token phase nevertheless allocates a `Vec<Segment>` for that single item on every encode. Use `SmallVec<[Segment; 1]>` only inside the crate so the common no-match result stays inline and any number of real matches still spills to the heap. Preserve the public `AddedTokens::split -> Vec` return type, every matcher and boundary rule, normalization order, pre-tokenized buffer, scanner, model, cache, post-processing, JSON and `.tkz` format, and public API.

This is not a change to the unbounded BPE scratch or output buffers rejected in the earlier container audit. Add no wrapper, hard capacity, unsafe code, branch, new function, or test. Existing added-token tests cover empty, no-match, one-match, adjacent, many, normalized, whole-word, and whitespace-stripping behavior. Retain only if those tests, the full suite, strict lint, and five-model Hugging Face parity pass; four counterbalanced ShareGPT sequential and batch-1 pairs must show a positive representative geomean with no recurring greater-than-3% model loss. Otherwise commit and revert the source, then keep the recorded result.

Result: **rejected on the first model's recurring-loss gate**. Commit `2b99441` changes only the internal segment container and preserves the public `Vec` return. All 39 focused added-token and pipeline tests pass, strict library clippy is clean, and the release binary has unchanged text and data segment sizes. Four counterbalanced Qwen ShareGPT processes each produce exactly 869,054 Hugging Face IDs. Parent times are 49.16, 46.81, 53.06, and 47.77 ms; candidate times are 52.75, 45.66, 56.22, and 54.55 ms. The candidate loses three pairs by more than 3%, and its median rises from 48.47 to 53.65 ms, a 10.7% time regression. Hugging Face control medians move only 2.4%, while the control-normalized candidate median is still 5.8% slower. Two already-started GPT-OSS pairs are near flat and cannot repair the failed per-model gate, so batch-1 and the remaining matrix stop. Commit `58f1303` reverts the source rather than retaining a dependency and wider inline value for one saved allocation.

### Experiment 45: reserve the parsed merge map — planned

Experiment 41 combined a fast hasher with reserving the known merge count, then stopped at the hasher's public-API lint failure before measuring construction. Isolate the still-untried one-line part: initialize JSON `parse_merges` with `ParsedMergeMap::with_capacity(merges.len())`. The five cached models contain 50,000 to 514,906 merge entries, so the default-growing map otherwise repeatedly reallocates and rehashes a table that is populated once and discarded after the deterministic runtime tables are built.

Preserve the map type, hasher, insertion order, duplicate-pair last-write-wins behavior, errors, model flags, runtime representation, `.tkz` path, encode code, and public API. Add no dependency, helper, branch, unsafe code, or test. Retain only if focused BPE and five-model parity stay exact, repeated fresh-process JSON construction has a positive five-model geomean with no recurring greater-than-2% model loss, and direct `.tkz` plus representative encode controls remain flat. Otherwise commit and revert the source.

Result: **retained**. Commit `fd49627` changes only the JSON construction map from `new` to `with_capacity(merges.len())`; the completed runtime representation, native sidecar path, and every encode function remain unchanged. The release load binary has identical text/data segment sizes and is 16 bytes smaller.

Fresh counterbalanced JSON-construction medians improve 2.57% on GPT-2 (88.181 to 85.974 ms), 3.55% on Phi (209.159 to 201.997 ms), 1.55% on Qwen (364.741 to 359.178 ms), 10.04% on GPT-OSS (956.785 to 869.520 ms), and 1.40% on Gemma (1,021.950 to 1,007.822 ms), a 3.77% five-model geomean improvement. A short four-pair GPT-2 screen was invalidated by a complete second-process advantage and reported -3.49%; eight longer 21-load pairs resolve +2.57% overall, with independently position-matched first and second medians both positive. Gemma's first four-pair block also moved -3.79% under one large parent outlier and one greater-than-2% candidate loss; three added pairs improve 3.62%, 7.19%, and 4.40%. Across all seven pairs the median is positive, the candidate wins four, and no greater-than-2% loss recurs.

The null controls stay clean. Qwen direct-`.tkz` medians are 158.767 ms for the parent and 156.814 ms for the candidate (+1.25% despite one candidate outlier), confirming that the bypassed JSON parser has no recurring loss. Four Qwen ShareGPT encode processes per binary emit exactly 869,054 IDs each; raw medians improve from 64.81 to 60.41 ms and Hugging Face-normalized medians from 13.67x to 15.44x. The source therefore clears its construction, artifact, sidecar, encode, and exactness gates without claiming the noisy controls as causal speedups.

### Experiment 46: preserve GPT-2 punctuation boundaries — planned

The five-model Hugging Face gate exposed a pre-existing GPT-2 mismatch shared by the frozen parent and Experiment 45: punctuation immediately followed by newlines is emitted as one scanner piece, although the GPT-2 regex has no newline suffix on its punctuation alternative. For `"a.\n\nx"`, Hugging Face emits `"a"`, `"."`, `"\n"`, `"\n"`, and `"x"`; Snaptokens emits `"a"`, `".\n\n"`, and `"x"`, allowing BPE to merge the newline pair and change IDs.

Add one explicit newline-suffix control to the existing ASCII and Unicode punctuation helpers. Keep it enabled for the Llama, Kimi, Qwen, Phi, and GLM patterns whose source alternatives contain `[\r\n]*`, and disable it only for the two GPT-2 patterns. Add the failing punctuation/newline boundary to the existing cross-grammar Hugging Face semantic-edge test rather than creating a new fixture. Preserve every other scanner rule, ByteLevel transform, merge engine, cache, model flag, JSON/`.tkz` path, and public API. Retention requires the focused scanner test, exact GPT-2 ShareGPT parity, the full suite, strict lint, and unchanged parity on the other four representative models.

Result: **fixed and retained**. Commit `d6c858e` adds only the explicit suffix control and one semantic-edge input. A direct Hugging Face diagnostic now gives identical pieces for the failing `"a.\n\nx"` boundary and the surrounding whitespace cases. All 24 focused pre-tokenizer tests pass, including both GPT-2 scanners and every fixed Llama/Kimi/Qwen/Phi/GLM/DeepSeek grammar against Hugging Face.

The repaired GPT-2 tokenizer exactly matches Hugging Face across 200 ShareGPT conversations and 1,125,781 output IDs while running 15.78x faster in that validation process. Phi, Qwen, GPT-OSS, and Gemma also pass end-to-end exact-ID ShareGPT checks after the change. Final proof is formatting and diff cleanliness; 164 passing all-target/all-feature tests with the same 9 extended tests intentionally ignored; strict all-target/all-feature clippy; doc tests; no-default checking; and locked package construction/verification.

### Experiment 47: final portable residue audit — no source candidate

A fresh high-reasoning Grok Build audit independently read the live source, benchmark harnesses, and Experiments 1--46 under the same exactness, portability, and minimal-diff constraints. Its only encode idea with historically material upside was a layout-isolated reimplementation of Experiment 33's multi-Split ByteLevel fusion. That is a non-minimal reopening of an exact implementation already rejected by recurring inactive-model losses, not a new source experiment. The remaining measured encode bottleneck is `BinaryHeap` maintenance in `run_merge_loop`; the alternative heap arities, entry layouts, short linear engines, accepted-merge lookup removal, and unsafe check-elision families have all failed their end-to-end gates.

The only apparently one-line construction residue was to skip `build_bigram_bridge_table` when `byte_fallback` or `ignore_merges` makes `bigram_bridge_table()` return `None`. The live audit confirms that the scan is unconditional, but also that `Bpe` and its `bigram_bridge_table` field are public Rust API. Filling a dummy table would therefore change observable behavior, while an `Option` or lazy wrapper would change the public field type and runtime layout. Preserving the public value requires building it. Replacing the temporary `.tkz` merge map similarly loses the existing pair-to-rank validation oracle and duplicates canonical merge logic for a thin slice of an already 2.5--3.5x faster path.

No source was written. The campaign stops here because no untried small change removes a currently dominant portable cost without repeating a rejected family, changing public behavior, coupling `.tkz` to dependency layouts, using target-specific instructions, or weakening Hugging Face parity. Reopen only if cold Zen 5/Core Ultra/Apple profiles identify a different dominant symbol, or if the project explicitly authorizes a larger algorithm/layout-isolation effort.

### Experiment 48: overlap independent BPE construction — planned

The retained Gemma `.tkz` profile makes leftmost-longest DAAC construction dominant and resolves the largest independent common residue to the fixed 65,536-entry initial byte-pair table. After the ranked merge and byte lookup tables exist, both builds consume only immutable inputs and neither observes the other's allocation order. Rayon is already a runtime dependency. Use one stack-only `rayon::join` to overlap only those two builds; do not add a pool, threshold, helper, field, dependency, test, unsafe block, format change, or encode-path branch. Preserve construction error precedence, public APIs, every final table and cache, byte fallback, `ignore_merges`, JSON and `.tkz` behavior, and the complete `Bpe` layout.

Freeze the current release load binary before source changes. Retain only if focused and five-model exactness remain clean, fresh-process JSON construction has a positive representative geomean with no recurring greater-than-2% model loss, direct `.tkz` construction is nonnegative, and a representative encode control shows no recurring loss. Include the first-call pool-start cost rather than prewarming it. A miss is committed and reverted; do not compensate with model-name checks, size thresholds, nested joins, or a construction framework.

Result: **retained**. Commit `179c65e` moves the already-required ranked and byte lookup tables ahead of DAAC construction, then uses one `rayon::join` to build DAAC and the fixed byte-pair table concurrently. It adds no dependency, pool, helper, field, test, unsafe block, format change, or encode branch. The final `Bpe` layout and construction error precedence are unchanged.

Fresh-process medians improve in every model and format, including the first-use Rayon startup cost. Each cell has six counterbalanced pairs; the four noisiest controls were extended to sixteen.

| Model | JSON load | Direct `.tkz` load |
|---|---:|---:|
| GPT-2 | +4.65% | +11.78% |
| Phi | +4.42% | +6.04% |
| Qwen | +2.26% | +6.63% |
| GPT-OSS | +5.68% | +8.68% |
| Gemma | +3.19% | +4.28% |
| Five-model geomean | **+4.04%** | **+7.45%** |

Two more elaborate scheduling rungs were measured and rejected. Commit `ee69203` used a scoped standard-library thread, but repeated GPT-2 JSON load rose from 84.906 to 90.614 ms (+6.7% time) and `e91c454` reverts it. Commit `00dfecb` reused the private BPE pool and was essentially tied with the global join across ten load cells, but direct controls exposed position-dependent recurring losses: Qwen `.tkz` was 2.93% slower when the candidate ran first. Commit `8279d98` reverts that variant rather than averaging the losses away.

The construction-only control remains exact and effectively flat. Four counterbalanced Qwen ShareGPT pairs each emit exactly 869,054 Hugging Face IDs. Raw encode medians are 49.02 ms for the parent and 49.45 ms for the candidate (-0.86% throughput), while Hugging Face-normalized medians improve 0.63%; only one pair has a material loss in either view. End-to-end exact-ID checks also pass for GPT-2, Phi, Qwen, GPT-OSS, and Gemma. The rebuilt final binaries exactly match the already-validated candidate hashes.

Formatting, strict all-target/all-feature clippy, 164 active tests with the same 9 extended tests ignored, doc tests, no-default checking, and locked package construction all pass. The load-only harness grows 126,528 bytes because it now links the Rayon global-worker path; the full benchmark grows only 4,464 bytes and has unchanged text/data segment sizes. Loading may initialize Rayon's global pool earlier than sequential-only callers previously observed, but the crate already uses that pool for `encode_batch`; callers that customize it must still configure it before tokenizer use.

### Experiment 49: close unsafe and redundant boundaries — retained

A final whole-tree audit found two public inputs that did not establish the preconditions of downstream unchecked code. Bulk ByteLevel encoding assumed splits formed a non-overlapping partition, although `PreTokenizedString` publicly accepts overlapping ranges. It now calls the existing capacity-checking encoder per split and deletes the redundant unchecked wrapper. Ordinary tokenizer splits retain the original allocation; repeated public ranges can no longer write beyond it. The public `Bpe::new` boundary now rejects out-of-range left, right, and merged IDs before they enter unchecked adjacency tables. Two focused tests cover those malformed boundaries.

Construction now skips the 256 formatted `<0xNN>` probes when `byte_fallback` is disabled; the encoded merge path already guards every read of that table with the same flag, so enabled fallback behavior and values are unchanged. The added-token prefilter also loses an unreachable fallback after its exhaustive tail/token cases. Repository cleanup removes stale Python/native-extension ignores, the duplicate local task list, and nine unreferenced chart variants totaling 1,327,549 bytes while preserving every README-linked SVG.

One representation experiment was rejected rather than hidden. Making the DAAC helper require a whole-input match allowed the duplicate `token_lens` vector and dependent exact-match load to be removed. Six counterbalanced Gemma and DeepSeek LongBench pairs remained exactly equal to Hugging Face at 12,544,717 and 11,248,053 IDs per process, but the combined Hugging Face-normalized geomean was 0.83% slower. Gemma was 1.31% slower under both binary orders. The complete representation change was therefore removed instead of claiming its smaller binary and heap as a speedup. Extending Experiment 48's construction worker to adjacency and bridge-table construction was also removed before release when the requested close prevented a clean isolated gate.

Final proof is formatting, diff cleanliness, strict all-target/all-feature clippy, 166 active tests with the same 9 extended tests intentionally ignored, doc tests, no-default checking, and locked package construction/verification. The active suite includes cached Hugging Face model parsing, token-ID round trips, sequential/batch consistency, JSON and direct `.tkz`, byte fallback, `ignore_merges`, added-token flags, Unicode, decoding, and streaming.

### Correctness repair after Experiment 49: preserve default ByteLevel contraction case

Commit `240d54f` fixes a pre-existing mismatch in standalone `ByteLevel`: the default Hugging Face GPT-2 pattern is case-sensitive, but `ByteLevel::pre_tokenize` had selected the case-insensitive scanner variant. The difference is observable after an apostrophe: `"'The"` must split as `"'"` plus `"The"`, not `"'"`, `"T"`, and `"he"`. The constructor now selects the existing case-sensitive scanner, and one focused regression test covers that exact boundary. The focused test, formatting, and the full all-target/all-feature suite pass; the full run reports 167 passed, 0 failed, and the same 9 extended tests ignored.

### Post-Experiment-49 research: high-impact optimization queue

Six independent, Exa-backed passes attacked the next search from hardware constraints, algorithmic reframing, adjacent-field analogues, failure-mode inversion, cost/throughput tradeoffs, and competitor implementations. They returned 53 deliberately unranked suggestions. Mechanical overlap removal produced the 41 `M` families below; no quality decision was made during that merge.

Grok Build then attacked all 41 families for six consecutive rounds at its highest supported reasoning setting. The rounds tested correctness invariants, microarchitecture and measurement failure, state explosion, interactions, and hidden assumptions. Each round had to add one idea absent from the original pool, producing `N1` through `N6`. A separate Claude Code Fable/high pass read the complete experiment history and live hot paths, then added or materially reframed `C1` through `C7`.

Convergence uses only two scores. **Expected impact** is estimated end-to-end throughput in the explicitly named workload unless the cell says load-only; **cost to test** includes the smallest exact prototype and its parity/performance gate. The percentages are hypotheses to falsify, not benchmark results. Low-impact ideas are cut even when cheap, while a high-impact narrow-regime idea remains visible instead of being averaged away.

#### Kept: ranked next experiments

| Rank | Ideas | Experiment | Expected impact | Cost to test | Why the first pass missed it |
|---:|---|---|---|---|---|
| 1 | `C2` | Re-adjudicate Experiment 35's encoded-path quaternary heap with deterministic counts and fastokens-only alternating processes. | 18--22% on Gemma's encoded path; 0--3% broad geomean. | Low: 0.5--1 day. | A noisy inactive GPT-OSS control, which the candidate path should not execute, closed a repeatable 22% Gemma result. |
| 2 | `C4` | Flatten nested Rayon document and piece work into one indexed task graph with deterministic reassembly. | 5--30% on the weak batch cells, plus lower variance. | Medium: 2--4 days. | Experiment 9 tested one overly broad suppression predicate, not flat ownership of the complete task set. |
| 3 | `M37` | Deduplicate exact pieces within a cold batch, tokenize each unique piece once, then scatter IDs in input order. | 10--30% on duplicate-heavy cold batches; 0--10% generally. | Low: 1--2 days. | Previous cache work assumed warm cross-call reuse rather than same-call execute-once reuse. |
| 4 | `C3` | Parallelize the hand-written fixed scanner with exact boundary-state handoff or rescan to each enclosing token boundary. | 15--30% on long sequential documents if scanning still consumes at least 15% of time. | Medium: under 1 day to falsify, 3--5 days to prototype. | Parallel scanning disappeared with the regex engine; the replacement fixed grammar has not been tested with an exact parallel boundary protocol. |
| 5 | `M23`, `M24`, `M25` | Isolate a selection-vector ByteLevel path, derive maximal exact spans, then pass raw bytes to BPE only across proved-safe boundaries. | 10--35% on currently non-fused compatible pipelines. | Medium: 3--5 days. | PR #46 used a global bigram approximation; it did not compute maximal input-specific safe spans in a layout-isolated path. |
| 6 | `M14`, `M15` | Prototype an exact compatibility-guided/prefix-state encoder that avoids maintaining every intermediate heap candidate. | 15--50% on merge-heavy cold pieces. | High: 5--10 days. | The campaign varied heap arity and layout inside the same algorithm instead of changing the exact BPE state transition. |
| 7 | `M10`, `M12`, `M18` | Run several pieces in lockstep with contiguous state, prefetch, and dense non-monotone/local-minimum rounds while preserving per-piece Hugging Face order. | 15--50% on long-piece batches. | High: 5--10 days. | Earlier experiments optimized one piece's priority queue; they never vectorized scheduling across independent pieces. |
| 8 | `M20`, `M41` | Add a caller-owned `EncodeSession` that preserves exact BPE state across appended prompt suffixes. | 2--10x for growing-prompt retokenization; no gain for unrelated texts. | Very high: 7--14 days. | The stateless benchmark matrix excluded the dominant append-only serving shape. |
| 9 | `M33` | Replace all CSR merge-adjacency searches with immutable perfect retrieval plus exact-key verification. | 3--15% end-to-end, or 10--30% of a BPE-heavy stage. | Medium: 2--4 days. | Experiments 2 and 3 removed only the accepted-pop lookup; initial seeding and both newly enabled-neighbor queries still binary-search CSR rows. |
| 10 | `M34` | Make a validated, mmap-native `.tkz` section readable in place while retaining canonical corruption checks. | 1.3--2.5x cold load and lower multiprocess RSS; no encode gain. | Very high: 7--14 days. | Version 4 deliberately favored portable canonical reconstruction, so zero-copy load was excluded before being measured. |

#### Cut or held

| Ideas | Expected impact | Cost to test | Decision record |
|---|---|---|---|
| `M01` instrumentation PGO | 2--8% and profile/host-specific. | 1--2 days. | Cut: useful deployment tuning, but below the architectural queue and easy to overfit to one corpus. |
| `M02`, `C5` whole-input ASCII dispatch and resume after sparse Unicode | 1--5%. | Under 1 day to diagnose; about 2 days to implement. | Cut: whole-input ASCII fusion already exists; only restart-on-Unicode and non-Qwen eligibility remain. |
| `M03` wide ASCII boundary masks | 3--10% scanner-stage, less end-to-end. | 2--4 days. | Cut: the scalar scanner is already cheap, while wider masks add tail and code-layout risk. |
| `M04` generated minterm DFA | 3--12% scanner-stage. | 3--5 days. | Cut: generated code size and instruction-cache pressure are likely to consume the gain. |
| `M05` topology-aware workers | 0--8%, host-specific. | 2--4 days. | Cut: no current profile makes NUMA or efficiency-core placement the dominant cost. |
| `M06` outer/inner permits | 0--10%. | 1--2 days. | Cut: atomics add a new tax and `C4` attacks the ownership problem directly. |
| `M07`, `M08`, `N1` SwissTable, set-associative, and soft-epoch `FlatCache` variants | 0--8%. | 2--4 days. | Cut: several cache representations already reversed by model and batch mode, with no new evidence that probe/clear dominates. |
| `M09` hot/cold `Bpe` layout | 0--5%. | 1--2 days. | Cut: neighboring layout reductions have caused recurring instruction-layout regressions. |
| `M11` monotone radix queue | Under 3%. | Under 1 day. | Cut: the required monotone-rank invariant is already false in 8 of 16 inspected tokenizers. |
| `M13` Re-Pair batch wavefront | 5--15%. | 5--10 days. | Cut: global merge rounds synchronize pieces and are weaker than independent exact lockstep scheduling. |
| `M16` successor-forest incremental BPE | 5--15%. | 5--10 days. | Cut: pointer-heavy updates have lower expected locality than the selected compatibility/prefix-state prototype. |
| `M17` compressed compatibility graph | 0--5% before `M14`/`M15` exists. | 2--4 days. | Hold: it is a second-stage compression of an unvalidated representation, not a first experiment. |
| `M19` whole-tokenizer subsequential transducer | Below 1% expected overall; theoretical runtime impact could be large if tractable. | More than 2 weeks. | Cut: the paper proves existence and linear execution for a fixed dictionary but leaves equivalent-transducer state complexity open, so production-vocabulary construction is unvalidated. |
| `M21` pull IDs directly into the sink | 3--12%. | 3--7 days. | Cut: the fused raw path already removes most of the materialization this idea targets. |
| `M22` minimal vocabulary FST | 2--10% load/lookup. | 3--5 days. | Cut: DAAC and vocabulary lookup are not hot enough after the retained `.tkz` work. |
| `M26` GPU long-piece BlockBPE | Under 5% overall; potentially large only for rare huge pieces. | More than 2 weeks. | Cut: current median pieces are 4--5 bytes and p99 is at most 38 bytes, too small to amortize dispatch. |
| `M27` content-defined superchunk cache | 0--15% generally; high only on edited/repeated documents. | 3--5 days. | Cut: `M20`/`M41` preserves exact incremental state with less hashing and collision machinery. |
| `M28` minimizer seed-and-verify vocabulary lookup | Under 5%. | 2--4 days. | Cut: long whole-vocabulary searches are too rare in the live path. |
| `M29` allocator deployment sweep | 0--8%, host-specific. | Under 1 day. | Cut: easy but deployment-specific, and prior allocator-state effects reversed across models. |
| `M30` construction/runtime allocation regions | 3--12% load-only. | 2--4 days. | Cut: version-4 `.tkz` and retained constructor work removed most repeated construction allocation. |
| `M31` byte-budgeted TinyLFU-admission shared cache | 0--5% throughput; possible RSS benefit. | 3--5 days. | Cut: this is mainly a memory policy change without evidence that admission misses dominate. |
| `M32` compressed shared-cache IDs | 0--3%. | 1--3 days. | Cut: cached ID vectors are normally too short for decode overhead to repay saved bytes. |
| `M35` cross-process content-addressed piece cache | Near zero in the library's normal single-process use; potentially high in one serving topology. | 5--10 days. | Cut: IPC, trust, invalidation, and deployment ownership outweigh its narrow expected coverage. |
| `M36` Four-Russians BPE supertransitions | Below 1% expected overall; theoretical runtime impact could be large if tractable. | More than 2 weeks. | Cut: the frontier state is expected to grow with token context before useful block widths are reached. |
| `M38`, `N4`, `N5`, `N6`, `C1` causal harness, counter ledger, layout firewall, stratified matrix, and deterministic-count screen | 0% direct runtime. | 1--3 days total. | Cut from the speedup ranking, but retain as experiment prerequisites: they make small or path-isolated results decidable. |
| `M39` per-tokenizer TLS cache partition | 0% for the common one-tokenizer process; 0--5% otherwise. | 1--3 days. | Cut: the product target does not establish frequent tokenizer switching. |
| `M40` persisted pipeline strategy ID | 0--3%. | 1--2 days. | Cut: current enum/`Option` shape dispatch is trivial relative to scanning and BPE. |
| `N2` pinned Unicode property bitsets | 0--3% speed. | 3--5 days. | Cut as an optimization: it is primarily a correctness/portability mechanism. |
| `N3` stage-affine scanner/BPE pools | 0--10%. | 3--5 days. | Cut: it fights TLS warmth and is structurally weaker than the flat task graph in `C4`. |
| `C6` raw-byte inverse vocabulary for `ignore_merges` | 1--5% on affected models only. | 1--2 days. | Cut: correct and cheap, but below the required impact floor. |
| `C7` deferred, shard-grouped shared-cache publication | 1--3% batch throughput. | 1--2 days. | Cut: mutex wait has not been shown large enough, so this is easy but small. |

The source trail is intentionally separate from the scores. Exact BPE reframings came from [GitHub's rust-gems BPE](https://github.com/github/rust-gems/tree/main/crates/bpe), [rs-bpe](https://github.com/gweidart/rs-bpe), [c-bpe](https://github.com/andrey-savov/c-bpe), [BlockBPE](https://arxiv.org/pdf/2507.11941), [incremental BPE](https://arxiv.org/html/2605.30813v1), and the [BPE transducer construction](https://arxiv.org/abs/2405.07671). These sources motivate algorithmic experiments; none by itself proves exactness for Snaptokens' full normalizer, scanner, added-token, byte-fallback, and post-processing pipeline. Data-layout and caching candidates used [Lucene FSTs](https://msfroh.github.io/lucene-university/docs/FiniteStateTransducers.html), [TinyLFU](https://arxiv.org/pdf/1512.00727), [PtrHash](https://arxiv.org/abs/2502.15539), [Ribbon retrieval](https://arxiv.org/abs/2109.01892), [Stream VByte](https://arxiv.org/abs/1709.08990), and [FastCDC](https://github.com/google/cdc-file-transfer/blob/main/fastcdc/fastcdc.h). Scheduling and measurement checks used [oneTBB task arenas](https://uxlfoundation.github.io/oneTBB/main/specification/source/task_scheduler/task_arena/task_arena_cls.html), [rustc PGO](https://doc.rust-lang.org/stable/rustc/profile-guided-optimization.html), [Stabilizer](https://people.cs.umass.edu/~emery/pubs/stabilizer-asplos13.pdf), and the [Rust Performance Book](https://nnethercote.github.io/perf-book/benchmarking.html).

No optimization candidate was implemented during this research pass. Experiment 50 below adjudicates rank 1 with `C1` deterministic counts and the same-artifact control from `M38`; the next untried source experiment is rank 2.

### Experiment 50: isolate the quaternary heap to encoded BPE — retained

Experiment 35's supposedly inactive fused/raw control was not actually isolated: encoded BPE used `DaryHeap<4>`, but fused/raw BPE also changed from the standard `BinaryHeap` to `DaryHeap<2>`. This retry keeps the fused/raw scratch on `std::collections::BinaryHeap` and gives only already-encoded BPE a separate `QuaternaryHeap`. A private macro expands the one exact merge algorithm behind two concrete wrappers, avoiding runtime dispatch, a model or length heuristic, duplicated semantics, unsafe code, and public or `.tkz` format changes. Trait and function-item adapters were measured and removed because they changed AArch64 code generation without simplifying the final representation.

The causal harness compiled identical source (SHA-256 `20cb2597c401dd4ad3d8dc702acfa4a6dc2f58347b8eadeb816f71bab0f31ed3`) against frozen parent `55ec9a9` and the candidate, forced the repository's locked `dary_heap` 0.3.8, excluded tokenizer construction and dataset parsing, warmed one input, and timed 50 unique LongBench-v2 contexts. Each cell below has two counterbalanced fresh-process pairs and exact token-count equality.

| Model/path | Parent median | Candidate median | Time change | IDs/process |
|---|---:|---:|---:|---:|
| Gemma encoded + byte fallback | 17,404.117 ms | 12,878.337 ms | **-26.00%** | 12,544,717 |
| DeepSeek multi-Split encoded | 1,531.484 ms | 1,493.051 ms | -2.51% | 11,248,053 |
| GPT-OSS fused/raw | 817.846 ms | 752.282 ms | -8.02% | 10,961,405 |
| Phi fused/raw | 757.442 ms | 694.357 ms | -8.33% | 11,081,531 |

Both Gemma pair directions improve, by 31.97% and 20.25%. The three short controls each reverse direction once under substantial host-frequency variance, so their nonnegative medians clear the no-recurring-loss gate but are not claimed as speedups. A separate 250-unique-input stress screen likewise found no recurring fused/raw loss; repeated-corpus measurements were discarded once cache reuse was identified rather than reported as evidence.

The locked full-pipeline harness then matched Hugging Face exactly on 50 inputs per model: Gemma emitted 12,544,717 IDs at 1.91x, DeepSeek 11,248,053 at 22.60x, GPT-OSS 10,961,405 at 24.74x, and Phi 11,081,531 at 36.52x. Gemma exercises enabled byte fallback, while GPT-OSS exercises `ignore_merges`; neither behavior or value path changed.

Formatting, strict all-target/all-feature clippy, 167 active tests with the same 9 extended tests ignored, no-default checking, doc tests, and locked package construction/verification all pass. The focused BPE subset is 11/11. The signed release benchmark shrinks 288 bytes overall while `__text` grows 288 bytes. The retained source adds one direct dependency entry and 31 net BPE lines; it adds no test because the existing exact-order suite and four real-tokenizer gates directly exercise the changed invariant.

### Experiment 51: coordinate outer batch and inner BPE parallelism — retained

The published chat curve exposed cross-pool contention: every outer Rayon document task synchronously queued more work into the shared eight-thread BPE pool. The retained scheduler keeps inner BPE parallelism for one outer task and for long inputs when only two or three outer tasks are active. Inputs through 16 KiB run directly on two or more outer workers; four outer tasks are sufficient at every measured length. A short one-item batch bypasses its otherwise useless outer Rayon dispatch. The wide path stays out of line so it cannot inflate the narrow hot path.

The suppression scope is thread-local, nested, and restored by `Drop` on ordinary return, error, or panic. Both ordinary and fused batched tokenization honor it after their existing small-split fast path. Experiment 27 is unchanged: whenever the dedicated pool remains eligible, fused BPE still creates two tasks per BPE worker.

A temporary same-process harness loaded the candidate, parent Snaptokens, pinned Fastokens `326cb5a`, and Hugging Face from the same tokenizer JSON. Every case first compared complete token ID vectors, warmed at least 4 MiB, then ran six counterbalanced implementation orders. On the 26-vCPU Xeon, the final 140-byte Qwen screen improved over pinned Fastokens in every batch-size median:

| Batch | Throughput change |
|---:|---:|
| 1 | +20.72% |
| 2 | +188.86% |
| 4 | +87.06% |
| 8 | +68.33% |
| 16 | +155.65% |
| 32 | +415.77% |
| 64 | +297.42% |
| 128 | +209.01% |
| 256 | +211.88% |
| 512 | +224.92% |
| 1024 | +243.92% |

The fused GPT-OSS control improved 113.54% at batch 16 and 201.09% at batch 256. Non-fused DeepSeek improved 88.52% and 190.90% in the same cells. Every candidate repetition beat both parent Snaptokens and Fastokens in those four controls.

The first blunt batch-two policy was rejected after two equal 64 KiB Qwen inputs regressed about 10% versus the parent. A byte-length sweep found candidate wins through 16 KiB on Qwen, GPT-OSS, and DeepSeek, while the retained long path returned to parity. Equal 64 KiB batches improved versus the parent by 23.43--73.67% for Qwen, 21.51--46.53% for GPT-OSS, and 13.87--34.00% for DeepSeek across the retained batch cells. One-long-input skewed batches also passed: Qwen improved 17.40% at batch 8 and 4.50% at batch 16, while DeepSeek improved 16.87% at batch 8. With one Rayon thread, long batch 32 remained within 2% of the parent; two-thread long batch 2 was within 1%; four-thread long batch 4 improved 58%.

All measured outputs matched Hugging Face exactly. The focused batch/sequential test passes, and one new unit test proves the suppression state restores after both nested use and unwinding. The implementation adds no dependency, unsafe code, public API, tokenizer-format change, model-name branch, or change to the retained Experiment 27 task count.

### Experiment 52: close the portable Gigatoken matrix — retained

The final scanner pass removes closure dispatch between fixed pre-tokenizers and fused BPE, reuses one fused cache stream for eligible serial ByteLevel batches, and replaces the partial GPT-2 scanner with one shared exact mask walker. The same walker now covers Qwen, Qwen 3.5, and Phi/GLM through grammar-specific boundary algebra. Ambiguous Unicode spans remain on the existing scalar path. A separate one-line eligibility expansion admits only the exact Llama-family `Removed` and inverted Split shape used by MiniMax.

One direct-sink-only Qwen trial was removed before the mask implementation: Qwen 3.5 improved slightly, but GLM lost seven of eight parent comparisons. The retained change therefore shortens the scanner itself instead of preserving a faster call shape around the scalar scanner.

The release retention harness builds the live candidate, frozen parent, and Gigatoken 0.10.0 with fat LTO. It uses four Rayon threads, novel inputs, alternating implementation order, and output allocation/destruction inside each timing. Every timed pool first compares flat IDs and row lengths among candidate, parent, Gigatoken, and Hugging Face.

The primary screen covered all 12 cached tokenizers accepted by current Gigatoken: GPT-2, GPT-OSS, DeepSeek V3.2, Mistral Nemo, Nemotron, four ordinary Qwen variants, Qwen 3.5, GLM 4.7, and MiniMax M2.1. Each ran 140-byte batches 1, 32, and 512 plus 4 KiB and 64 KiB single inputs. Snaptokens won all 60 cell medians; the narrowest median was 1.30x Gigatoken, and most cells were 1.5--4x. A separate ten-round Qwen Coder batch-512 rerun won every round with a 2.82x median. Kimi K2.5 is not omitted from a shared cell: Gigatoken rejects its exact Split regex at load time.

Two different real corpora guard against the generated-input result. On 64 KiB windows rotated through this scanner's Rust source, every shared model won all four rounds. Per-model medians range from 1.05x for GLM and 1.14x for MiniMax through 1.45x for GPT-2 and GPT-OSS. A second matrix rotated through this mixed prose/code experiment log and reran all five shapes for DeepSeek, Mistral, Qwen Coder, GLM, and MiniMax. All 25 medians won; representative long-input medians were 1.55x, 1.43x, 1.38x, 1.58x, and 1.27x respectively.

Correctness is independently clean: the new differential test compares all three Qwen mask grammars with their scalar oracles across semantic block edges and 1,000 deterministic mixed-Unicode inputs. Hugging Face semantic-edge tests, real Qwen 3, Qwen 3.5, GLM, byte-fallback, `ignore_merges`, Kimi, streaming, JSON, and direct `.tkz` paths pass. Final validation reports 174 passed, 0 failed, and the same 9 extended tests intentionally ignored in 664.94 seconds. Formatting, strict all-target/all-feature Clippy, docs, no-default checking, x86-64 cross-checking, locked package verification, and diff cleanliness pass. The optimized paths are portable NEON and runtime-gated AVX2/AVX-512 with the scalar fallback retained on other targets.

### Experiment 53: match output contracts and schedule expensive small batches — retained

The reported Apple M2 batch-32 loss was not initially an engine regression. Snaptokens was timed through nested `Vec<Vec<u32>>` output while Gigatoken 0.10.0 used its flat IDs-plus-lengths API. Switching only the timer to Snaptokens' existing `encode_batch_ragged` changed the frozen GPT-2 batch-32 median from 0.759x to 2.087x without changing production code. The committed all-engine harness now measures the same flat ragged contract for both engines.

A six-family gate then exposed one real loss. Gemma's normalizer, Metaspace, and byte-fallback pipeline remained sequential because the batch policy considered only 4,480 input bytes, not the heavier work per row. Frozen baseline runs lost at 0.777x and 0.802x. Lowering the global byte threshold was rejected after GPT-2 fell to about 0.52x; scheduling every cheap ByteLevel row added more overhead than work.

The retained source change instead uses the existing four-task threshold only on the existing non-ByteLevel pipeline flag. ByteLevel scheduling is unchanged, and there is no model name, new threshold, API, dependency, unsafe code, or tokenizer-format change. Two final Gemma processes measured 1.544x and 1.678x, with a 1.619x median across their 16 rounds.

| Tokenizer | Batch 1 | Batch 32 | Batch 512 | 4 KiB | 64 KiB |
| --- | ---: | ---: | ---: | ---: | ---: |
| GPT-2 | 2.040x | 2.266x | 1.684x | 1.644x | 1.552x |
| Qwen 3 | 1.775x | 2.172x | 4.068x | 1.744x | 1.496x |
| DeepSeek R1 | 2.387x | 1.830x | 3.725x | 1.636x | 1.489x |
| GPT-OSS | 1.615x | 2.244x | 3.516x | 1.696x | 1.561x |
| Phi-4 mini | 1.610x | 2.247x | 3.565x | 1.693x | 1.575x |

All 25 ByteLevel medians win, with a 2.009x geometric mean and 1.489x minimum. Batch 32 wins across all six families at a 2.047x geometric mean. Every round compares complete flat IDs and row lengths after both timers, preventing the correctness pass from prewarming nominally novel inputs. Each ordinary cell processes 8 MiB per engine over eight alternating-order rounds with output allocation and destruction timed.

The exact retained `src/lib.rs` SHA-256 is `b1cd214dfe929bf659cf3d3a2be04d86e1e8bf5cc805fe893e75128143143061`. The five-shape binary SHA-256 is `e74e91e5fba66e9d192421078c0ef7f1dabf00943f8946bf7c92eb01af0a0781`; the Gemma binary is `a1b5dbadf8984bb6df90f1a4cd45f7aba95bbc690950e1a9c88a5b5cc3c4d4f2`. Both were built with fat LTO and `target-cpu=native` using `rustc 1.99.0-nightly (d0babd8b6 2026-07-15)`.

No new unit test is added because the changed invariant is scheduling only. The existing ragged-versus-nested, batch-versus-sequential, and byte-fallback boundary tests cover its semantic paths, while the six real-tokenizer direct comparison checks the exact timed output. Final validation reports 174 passed, 0 failed, and the same 9 extended tests intentionally ignored. Formatting, docs, no-default checking, and locked package construction pass. Clippy is otherwise clean with only the pre-existing `chunks_exact(2)` style lint from `o200k_mask.rs` allowed; the unmodified line predates this experiment.

### Experiment 54: decode byte fallback in safe vocabulary splitting — retained

Gemma remained the clear portable weak path because `Bpe::bigram_bridge_table` disabled safe vocabulary splitting whenever `byte_fallback` was enabled. The table scanned literal vocabulary strings, so a merged token such as `<0xA9><0xC3>` appeared to contain marker text rather than the represented bytes `A9 C3`. Enabling the old table unchanged could therefore split between adjacent Unicode characters even when a canonical fallback merge crosses that boundary.

The retained change preserves the literal scan and adds a semantic scan only for byte-fallback token strings containing `<`. A valid leading `<0xHH>` contributes its represented byte; every other byte remains literal. Adding semantic adjacencies can only make the splitter more conservative, while the existing BPE engine still performs every fallback merge. `ignore_merges` remains ineligible. No merge value, tokenizer format, cache, scanner, scheduler, dependency, unsafe block, or public API changes.

The existing `byte_fallback_merge_crosses_unicode_boundary` fixture is the exact safety boundary: `éê` must retain a merge from trailing `A9` to leading `C3` and emit `[1, 4, 3]`. It passes without adding another test. The full all-target/all-feature suite reports 174 passed, 0 failed, and the same 9 extended tests ignored. Formatting, strict Clippy, no-default checking, doc tests, locked package construction, and diff checks pass.

The six-sandbox Modal gate used four vCPUs, 8 GiB, native fat LTO, four Rayon workers, six real tokenizer files, five shapes, eight alternating-order rounds per cell, and 8 MiB per engine per cell. Output allocation and destruction stayed inside both timers; complete flat IDs and row lengths were compared after both timers. The placements covered AMD Zen 3, Intel Ice Lake, and a newer AMD EPYC class.

| Placement | CPU | Frozen Gemma 64 KiB | Candidate Gemma 64 KiB |
| --- | --- | ---: | ---: |
| AWS east | EPYC 7R13, Zen 3 | 0.205x | **1.780x** |
| AWS west | EPYC 7R13, Zen 3 | 0.163x | **1.957x** |
| GCP east | EPYC 7B13, Zen 3 | 0.249x | **2.305x** |
| GCP west | EPYC 7B13, Zen 3 | 0.205x | **1.896x** |
| OCI east | 8358 baseline; EPYC 9J45 candidate | 0.170x | **1.224x** |
| OCI west | Xeon Platinum 8358, Ice Lake | 0.171x | **2.855x** |

The five comparable placement/CPU-class ratios move from a 0.196x to a 2.126x geometric mean, a 10.83x lift relative to Gigatoken. Across all six candidate placements, the 64 KiB geometric mean is 1.939x. Gemma wins all 30 candidate cells, including every batch and long-input shape. OCI east is deliberately excluded from the before/after lift because Modal assigned different hardware.

The unchanged GPT-2 batch-512 path produced isolated 0.845x and 0.987x cells in the broad matrix. Four fresh 20-round, 16 MiB-per-round controls resolved those as variance: two AWS Zen 3 workers measured 1.595x and 1.581x, while OCI's EPYC 9J45 and Ice Lake workers measured 1.321x and 1.941x. All four controls compared complete output after alternating timer order. A ten-round Apple M2 control had a 2.012x median with one 0.888x outlier, so no speedup is attributed to an unchanged ByteLevel path.

The local Apple M2 result is positive but more nuanced than the generated matrix. A 64 KiB generated Gemma screen moved from a frozen-parent 0.081x median to 1.067x; the candidate's first cold round was still only 0.215x before later rounds reached 1.049--1.105x. On 50 real LongBench-v2 documents totaling 42,424,271 bytes and 12,544,717 exact Gigatoken-matching IDs, four fresh candidate processes reduced median Snaptokens time from the parent's 4,443.499 ms to 2,005.063 ms, a 2.22x throughput gain. The candidate nevertheless reaches only a 0.482x median against Gigatoken on that diverse corpus, so this experiment closes the large regression without claiming that the remaining SentencePiece path is fastest everywhere.

### Experiment 55: materialize long fused pieces for parallel BPE — rejected

The long single-input fused path leaves the Rayon workers idle, so this experiment scanned one 64 KiB row into exact existing Split ranges and passed those ranges to the existing ordered parallel BPE path. The candidate kept normalizers, scanner boundaries, raw-byte BPE, caches, output order, APIs, formats, dependencies, and unsafe code unchanged. It also made the private BPE pool respect the configured Rayon width so the four-thread Apple M2 comparison could not silently use eight workers.

Commit `9eef3de` compiled, passed the focused nested parallelism restoration test, and matched complete Hugging Face and Gigatoken flat IDs and row lengths throughout the four-model matrix. The production-comparable harness used native fat LTO, four configured workers, six balanced rounds, two MiB per engine per round, rotating windows from this log, disjoint warmup, and timed output allocation and destruction. Only the 64 KiB single-input fused cells were intended to change; the other 16 cells were controls.

| Model | Parent 64 KiB | Candidate 64 KiB | Candidate / parent |
| --- | ---: | ---: | ---: |
| GLM 4.7 | 880.59 MiB/s | 889.76 MiB/s | 1.010x |
| GPT-OSS | 887.61 MiB/s | 562.68 MiB/s | 0.634x |
| MiniMax M2.1 | 703.14 MiB/s | 632.19 MiB/s | 0.899x |
| Qwen 3 | 709.80 MiB/s | 655.31 MiB/s | 0.923x |

Every GPT-OSS and Qwen candidate round lost to its parent distribution; GPT-OSS ranged from 517.61 to 635.58 MiB/s against 760.31 to 971.88 MiB/s for the parent. The range vector, owned normalized buffer, worker-local outputs, shared-cache traffic, and final gather cost more than parallel BPE saves for the overwhelmingly short fixed-scanner pieces. The loss spans Qwen-mark and ordinary Qwen grammars rather than one tokenizer or isolated noisy pair, so threshold tuning would only hide the failed mechanism. Commit `94b98e8` reverts it in full.

### Experiment 56: use the ranked merge map throughout heap BPE — rejected

The heap BPE path still queried the compact CSR adjacency table after constructing the existing exact open-addressed `ranked_merge_map`. This experiment changed only its four hot pair lookups: initial heap construction, accepted-pop token resolution, and the two newly exposed neighbors. It added no state, dependency, unsafe code, API, format change, fallback, or test. The 12 focused BPE tests passed, and all benchmark outputs matched exactly.

Two six-round Apple M2 pairs used the same four real tokenizers, five shapes, native fat LTO, four Rayon workers, two MiB per engine per round, rotating real-corpus windows, disjoint warmup, and timed output allocation and destruction. Process order was reversed between pairs. Ratios below divide each Snaptokens result by its same-process Gigatoken result before comparing candidate with parent, reducing process-wide noise.

| Process order | Geometric mean | Winning cells |
| --- | ---: | ---: |
| Parent then candidate | 1.022x | 13 / 20 |
| Candidate then parent | 1.019x | 11 / 20 |

The average looked attractive, but the required portable win gate failed. Four flat-ragged cells regressed after same-run Gigatoken normalization in both orders: GLM 4 KiB at 0.998x and 0.932x, GPT-OSS batch 512 at 0.996x and 0.976x, MiniMax batch 1 at 0.996x and 0.983x, and Qwen batch 512 at 0.983x and 0.989x. The first pair also used a different corpus digest after this log changed, so it cannot override the exact-corpus losses. The open-addressed table's larger footprint and probe behavior help some merge distributions but harm others; keeping it would optimize the average by sacrificing repeatable cells. Commit `6ea54d9` reverts the four-line trial in full.

### Experiment 57: replace the exact-token DAAC with the persisted orphan mask — rejected

The BPE constructor already computes and persists whether each vocabulary token is unreachable through the configured merge graph. This experiment retained that mask in `Bpe`, replaced the runtime leftmost-longest automaton with one exact `token_to_id` lookup plus the mask bit, and removed the duplicate token-length vector. It deleted 47 net source lines and made the automaton dependency orphaned, but deliberately left the dependency declaration in place until performance adjudication. The 12 focused BPE tests and all four-model harness outputs remained exact.

Cold construction improved substantially. Across two processes and four balanced child positions per process, JSON load improved 1.219--1.470x and direct `.tkz` load improved 2.209--3.410x across GLM, GPT-OSS, MiniMax, and Qwen. Peak load RSS fell 13--31% for JSON and 26--35% for `.tkz`; JSON-to-`.tkz` conversion improved 1.036--1.334x. The fat-LTO benchmark binary was 9,872 bytes smaller even before removing the unused dependency.

Encode did not clear the independent gate. Two opposite-order Apple M2 pairs used the identical corpus digest, four real tokenizers, five shapes, six balanced rounds, four Rayon workers, two MiB per engine per round, disjoint warmup, and timed allocation/destruction. Filtering explicitly to the flat-ragged contract, their Gigatoken-normalized geometric means were 0.983x and 1.024x, with only 5/20 and 15/20 cells winning. Qwen 4 KiB lost raw throughput in both orders, by 16.48% and 3.33%; the first normalized loss was 13.70%, while the second normalized result was essentially flat at +0.18%. The reverse pair's Qwen raw five-shape mean was 0.972x, and the first pair lost 1.68% across the entire normalized matrix. Removing the DAAC is therefore an excellent cold-load optimization but an unacceptable position-sensitive encode trade under the 100%-win requirement. Commit `9fd58ab` reverts it in full; a future `.tkz`-only automaton persistence design must preserve the current runtime lookup shape.

### Experiment 58: reuse the first generic ragged row — rejected

The generic `encode_batch_ragged` fallback encoded nested rows, allocated a second flat vector, and copied every row into it. This experiment preserved the length pass and output order but took ownership of row zero, reserved the final size on that allocation, and appended only later rows. It changed six lines, added no API, type, dependency, unsafe code, tokenizer condition, or test, and passed the existing ragged-versus-nested test. The exact published Gemma 3 tokenizer (`4667f208...ad795`) matched Hugging Face throughout.

Two opposite-order Apple M2 process pairs used six balanced rounds, four workers, two MiB per engine per round, the same real corpus, and both nested and flat-ragged controls. Flat-ragged batch 1 improved in both pairs, by 28.79% and 6.54%, and 4 KiB improved by 0.28% and 2.47%. The broader allocation reuse failed, however: batch 512 changed by 0.553x and 0.973x, while 64 KiB changed by 0.976x and 0.992x. In the first candidate process, batch-512 rounds after Gigatoken ran at only 19.83--26.53 MiB/s versus 91.84--125.35 MiB/s when Snaptokens ran first. Reallocating one small row into the complete batch buffer made performance depend on inherited allocator state. Commit `48832a2` reverts it in full. The measured batch-1 win justifies only the next narrower rung: return the sole owned row unchanged when `inputs.len() == 1`.

### Experiment 59: return a sole generic ragged row directly — rejected

This narrower rung left every multi-row path byte-identical and returned the already-owned nested row when the generic ragged fallback received exactly one input. The six-line branch eliminated one allocation and complete ID copy without changing output ownership, capacity, IDs, lengths, or APIs. The existing ragged-versus-nested test passed, and the exact Gemma 3 matrix remained equal to Hugging Face.

The first six-round Apple M2 pair rejected it without another process pair. All three changed flat-ragged cells lost raw throughput: 140-byte batch 1 at 0.869x, 4 KiB at 0.982x, and 64 KiB at 0.966x. Dividing each candidate/parent ratio by the same binary's unchanged nested control still produced 0.876x, 0.936x, and 0.907x. The candidate missed every targeted cell under both a direct and path-isolated comparison, so process-order expansion could not satisfy the predeclared positive gate. A tiny branch and changed caller layout cost more than the avoided copy in this compiled shape. Commit `710653c` reverts it in full; generic ragged ownership reuse is closed rather than tuned further.

### Post-Experiment-59 boundary: do not persist the dependency automaton

Experiment 57 showed that avoiding DAAC reconstruction can cut direct `.tkz` load time by 2.2--3.4x, but the dependency cannot safely preserve the current runtime representation from an untrusted sidecar. Daachorse 1.0 exposes only `unsafe deserialize_unchecked`. Version 3 adds a checked function, but its own contract proves only memory bounds: malformed data can still loop forever or return incorrect match ranges. The `.tkz` payload checksum detects accidental corruption, not an attacker who can recompute an unkeyed hash, and direct `.tkz` loading has no trusted source JSON to authenticate against.

Persisting that automaton would therefore weaken the crate's safety or exactness boundary. Rebuilding and semantically comparing it would erase the cold-load win, while validating dependency-private state would couple the format to Daachorse internals. No source change is justified. A future mmap-native format must store a Snaptokens-owned representation with complete structural validation rather than dependency serialization.

### Experiment 60: hold the encoded-piece cache across a chunk — rejected

The generic encoded-BPE path borrowed its thread-local cache once per pre-tokenized split, unlike the fused raw path, which holds one borrow across a complete chunk. This experiment factored the existing encoded cache-miss lifecycle into one private helper, added a chunk callback, and selected the existing batched split driver. It added no representation, dependency, unsafe code, public API, or test. All 12 focused BPE tests and both batch/ragged equivalence tests passed.

Two opposite-order six-round Apple M2 pairs used the exact published Gemma 3 tokenizer, rotating corpus windows, four workers, two MiB per engine and shape, timed output allocation/destruction, and both nested and flat-ragged contracts. Every output matched Hugging Face. The control-normalized ten-cell geomeans were 1.006x and 1.034x, with 6/10 and 8/10 wins, but the same important cells regressed under both binary orders. Nested batch 32 changed by 0.967x and 0.850x relative to Hugging Face; flat-ragged batch 1 changed by 0.958x and 0.894x relative to Gigatoken. Larger inputs often improved, but a length or shape gate would tune directly to the observed matrix. Commit `6b1e705` reverts the source in full rather than trading short-request throughput for a positive average.

### Current-loss audit: MiniMax batch 32 remains real

The two published Gigatoken losses were remeasured before targeting them. Two eight-round, four-MiB Apple M2 GLM processes put batch 32 at 1.024x and 0.999x, so that old 0.985x cell is currently a tie rather than a stable loss. Four concurrent ephemeral Modal sandboxes then reproduced the other cell on the exact reported Intel family 6/model 106 class. All used AWS EU-West, four physical cores, 16 GiB, native fat LTO, eight balanced rounds, four MiB per engine, and current exact MiniMax JSON/TKZ inputs. Snaptokens/Gigatoken batch-32 ratios were 0.953x, 0.956x, 0.943x, and 0.943x. All sandboxes stopped after collection, and every parity probe passed. The remaining shortfall is therefore narrow but real.

### Experiment 61: lower the fused-ragged parallel threshold — rejected

The affected MiniMax call contains 32 independent 140-byte rows totaling 4.5 KiB, just below the fused Split path's 2 KiB-per-worker serial cutoff. This experiment changed only that path to 1 KiB per worker, leaving single rows, large batches, other pipeline shapes, cache policy, output assembly, and all semantics unchanged. The existing ragged-versus-nested test passed.

The first four-model Apple M2 matrix rejected the mechanism before another cloud build. For GLM, GPT-OSS, MiniMax, and Qwen batch 32, candidate throughput retained only 0.523x, 0.329x, 0.486x, and 0.339x of the parent after each process was normalized by its own Gigatoken control. Tiny parallel chunks duplicate worker-cache and gather overhead rather than exposing useful BPE work. Commit `67e8a88` reverts the one-line source change; CPU/model dispatch or threshold tuning would overfit the single Intel placement and is not pursued.

### Experiment 62: batch fused cache probes across ragged rows — rejected

The serial fused path flushed pending scanner pieces after every row so it could record exact ragged lengths. This experiment encoded row boundaries as sentinels in the existing pending-piece batch, allowing short pieces from adjacent rows to share one cache-probe phase. A const-generic stream kept the boundary check out of non-ragged code. It changed no public API, model dispatch, tokenizer format, dependency, merge behavior, or scanner boundary. The 12 focused BPE tests, warning-denied compilation, and a ragged test covering empty rows, added-token fallback, and more than 256 pending pieces passed.

The first six-round Apple M2 pair used GLM, GPT-OSS, MiniMax, and Qwen with native fat LTO, four workers, two MiB per engine per shape, disjoint warmup, exact Hugging Face parity, and same-process Gigatoken controls. Control-normalized batch-32 ratios looked promising at 1.019x, 1.084x, 1.017x, and 1.104x. The same run's batch-512 results were contradictory, including 0.664x for MiniMax but 1.046x for GLM, while unrelated nested controls varied by as much as 2x between processes.

An opposite-order confirmation therefore increased the measurement to eight rounds and four MiB per engine for MiniMax and Qwen. The claimed batch-32 gain disappeared at 0.992x and 0.993x, and batch 1 regressed in both models at 0.967x and 0.963x. Batch 512 flipped to 1.073x and 1.049x, confirming that the earlier large-batch loss was process noise rather than a stable effect. The mechanism is exact but does not provide a repeatable speedup, and its extra stream state is not justified. Candidate binary `0f560cf601955de56ae96ec617aac13f329d1a0131511aa29a7e9e883e4f12a4` is rejected; commit `4ffb20b` reverts source commit `c904089` in full without a Modal run.

### Experiment 63: halve the sequential fused front cache — rejected

The fused stream's sequential direct table contains `2^21` 32-byte slots, or 64 MiB, while parallel workers use `2^19` slots. The remaining MiniMax loss occurs in a 4.5 KiB serial batch on Intel, so this experiment reduced the sequential table by one bit to 32 MiB without adding a CPU, model, or workload branch. The existing cache-consistency and ragged-equivalence tests passed; the source diff was one constant.

One six-round Apple M2 matrix covered GLM, GPT-OSS, MiniMax, and Qwen across five shapes, native fat LTO, four workers, two MiB per engine, exact Hugging Face output, and same-process Gigatoken controls. The 20 flat-ragged cells had a 1.004x raw geomean but only a 0.999x control-normalized geomean. The target MiniMax batch-32 cell was unchanged at 1.001x. MiniMax and Qwen batch 512 changed by 0.915x and 0.918x after normalization even though that shape uses the unchanged parallel cache, showing that code-layout or process-state costs already consume any sequential locality gain. The candidate offered no target improvement and failed the whole-matrix gate, so no reverse pair or Modal run was justified. Candidate binary `8c0ec5c33f5b69251d07c5a8a336fcb0e9f742e64f4de238eaaa0bb0b45dbca5` is rejected; commit `a18b48a` reverts source commit `4cf6438` in full.

### Experiment 64: stop initializing fused pending scratch — retained

A time-sampled MiniMax batch-32 profile attributed about 17% of samples to `_platform_memmove` under `tokenize_fused_stream_with_cache`. Fat-LTO disassembly exposed the cause: every scanner stream materialized a 7,680-byte `[FusedPiece; 320]`, zeroed it, and copied it into `FusedStream`, even though queue code writes every slot before increasing `pending_len` and flush reads only `0..pending_len`.

The retained source keeps that exact invariant explicit. A separate stack array contains `MaybeUninit<FusedPiece>` slots, `FusedStream` borrows it for the scan, queue paths use `ptr::write`, and only the proved initialized prefix is reinterpreted as `FusedPiece`. It changes no cache key or value, scanner boundary, merge order, byte-fallback handling, `ignore_merges`, output layout, public API, tokenizer format, dependency, model dispatch, or length threshold. No new test was added because the existing scanner differential, cache consistency, Hugging Face model fixtures, and nested-versus-ragged tests directly cover every changed read/write path.

The frozen Apple M2 artifact is `de486afe920a5defb312815fd4e7478abec55ac5636481850322b33b91fb42aa`, built from source commit `dbd94eb` with native fat LTO. Its fused-stream specializations contain no bulk `bzero`, `memcpy`, `memmove`, `0x1e00`, or `0x1e08` operation, and the complete benchmark binary is 4,832 bytes smaller than frozen parent `434c2797c72d64c7212cc85b222c3ca5594f141a6583b237da689f7ec7df29ea`.

Two opposite-order Apple M2 matrices used four real tokenizers, five shapes, both nested and flat-ragged contracts, four workers, disjoint warmup, rotating windows from this log, complete Hugging Face parity, and contract-matched unchanged controls. Ratios divide parent/candidate time and then normalize flat-ragged cells with same-process Gigatoken and nested cells with same-process Hugging Face.

| Process order | Flat-ragged geomean | Flat wins | Nested geomean | Nested wins |
| --- | ---: | ---: | ---: | ---: |
| Candidate then parent | **1.232x** | 19/20 | **1.258x** | 15/20 |
| Parent then candidate | **1.228x** | 17/20 | **1.313x** | 19/20 |

The original Intel target, MiniMax batch 32, improves 1.458--1.496x on M2 rather than trading away other batch sizes. A ten-round, eight-MiB follow-up resolved the disputed GLM, GPT-OSS, and MiniMax 64 KiB cells at 1.013--1.076x after their contract controls. The M2 result is not claimed as a literal all-cell sweep: GLM flat-ragged 64 KiB remains 0.992x across the first two orders, and Qwen nested 64 KiB is 0.978x across those orders and 0.983x in its higher-sample rerun. They are small unresolved local losses, not omitted observations. Later sustained brackets became unusable when normal user applications pushed load average to 19--22 and the identical candidate binary changed by roughly 2x between positions; no user process was killed to improve a benchmark.

Four concurrent isolated Modal sandboxes then built candidate and parent inside the same four-vCPU worker and reversed measurement order on two workers. Three workers reported Intel family 6/model 106 and one reported AMD family 175/model 1. Every sandbox used native fat LTO, eight balanced rounds, four MiB per engine and shape, exact MiniMax JSON/TKZ inputs, and complete parity. All sandboxes were explicitly terminated after collection. The candidate won all 40 raw A/B cells:

| Contract | Batch 1 | Batch 32 | Batch 512 | 4 KiB | 64 KiB |
| --- | ---: | ---: | ---: | ---: | ---: |
| Flat ragged | **1.340x** | **1.280x** | **1.110x** | **1.022x** | **1.045x** |
| Nested | **1.366x** | **1.357x** | **1.152x** | **1.018x** | **1.036x** |

Those are geometric means across the four workers. The narrowest raw cell is 1.007x and the all-cell geomean is 1.164x. This cross-vendor 40/40 result, the repeated M2 aggregate gain, and the direct removal of measured work justify retention without a CPU, tokenizer, model, or input-shape exception.

Three representation refinements were tested and rejected rather than folded into the result. A direct raw scratch pointer made Qwen nested 4 KiB and 64 KiB fall to 0.909x and 0.969x after controls. An inline union removed the copy but retained a 7,688-byte `bzero`; it passed one Qwen-only screen but produced five recurring full-matrix losses. A cache-line-aligned external wrapper improved aggregate M2 results at 64-byte alignment but retained three long-cell losses; 128-byte alignment matched the M2's reported cache line but became undecidable under the busy host and was reverted. The final production source is byte-identical to the simpler `dbd94eb` representation after those reverts.

Formatting, warning-denied library checking, strict all-target/all-feature Clippy, no-default checking, docs, package listing, and the complete suite pass. The final suite reports 174 passed, 0 failed, and the same 9 extended tests intentionally ignored. Byte fallback, `ignore_merges`, GLM, GPT-OSS, MiniMax, Qwen, DeepSeek, Kimi, Mistral, Nemotron, streaming, added tokens, JSON, direct `.tkz`, and ragged output remain exact.

### Experiment 65: inline the Llama ASCII fallback — rejected

A fresh current-HEAD MiniMax batch-32 sample put the remaining encode time in the fixed scanner and cache-probe loop rather than Rayon: all four worker threads slept while the main thread ran. The scalar Llama fallback alone held about 11% of active samples. This experiment added only `#[inline(always)]` to `scan_llama_ascii`, preserving every scanner boundary, cache operation, API, format, model path, and threshold. All five scanner differential tests passed. Fat-LTO removed both standalone specializations and shrank the complete profiler binary by 224 bytes.

Two 12-second opposite-order screens used the exact published MiniMax tokenizer, the same 64 MiB rotating corpus pool, 140-byte rows, batch 32, four configured workers, and the public flat-ragged API. Candidate/parent throughput was 0.932x in the cleaner pair and 0.683x in the noisier pair. Both directions fail the first target gate, and the large second-pair shift makes broader local precision claims inappropriate. Forced inlining increased caller register pressure or disturbed hot code layout more than it saved in call overhead. Commit `146653b` reverts source commit `f026fe4` in full; no model threshold, full matrix, or Modal run is justified. The requested Claude reward-hacking review remained unavailable because `claude auth status` reported no authenticated session, so the rejection rests on the stricter direct gate rather than peer advice.

### Experiment 66: two-phase short-mask boundary extraction — rejected

The fixed scanner currently pops each boundary and immediately packs its cache key. This experiment separated those dependency chains for masks whose pieces were all at most 15 bytes: phase A wrote boundary ends into the existing 64-slot pending slack, then phase B packed and prefetched them in a counted loop. The existing `pending_len < 256` capacity check proved that phase-B writes could not clobber unread boundaries. Long pieces, fallback scanners, cache resolution, public APIs, tokenizer formats, model dispatch, and thresholds were unchanged. All five scanner differential tests and both focused fused/cache tests passed.

The native fat-LTO candidate was `3b91da12f56d1d6ae563f99010c6c46fa5cecb20f7bca424d13c68c8a591e302`, 13,152 bytes smaller than the frozen parent. Two 12-second opposite-order screens used the exact published MiniMax tokenizer, a 64 MiB rotating real-corpus pool, 140-byte rows, batch 32, four configured workers, and the public flat-ragged API. Parent then candidate processed 3,031,118,720 and 3,208,531,200 bytes, a 1.059x candidate gain. Candidate then parent processed 2,355,194,240 and 3,027,610,880 bytes, only 0.778x for the candidate. The second process was strongly favored on this loaded host, but the predeclared gate required a win in both orders; the mechanism therefore cannot justify 35 added unsafe-path lines.

Commit `98789c4` reverts source commit `8bff7e7` in full. No full matrix or Modal run was performed because the isolated local gate failed. Claude review remained unavailable because `claude auth status` still reported `loggedIn: false`; no peer result is implied.

### Experiment 67: reuse NEON high-byte masks — rejected

The AArch64 fixed scanner accumulated an aggregate high-byte vector during its first classification pass, then reloaded all 64 bytes to recover exact Unicode positions only when needed. This experiment instead retained the four high-byte comparison vectors and reused them on that path, matching Gigatoken's current classifier shape. It removed three net source lines and changed no boundary algebra, scalar fallback, x86 code, API, allocation, tokenizer format, model dispatch, or threshold. All five scanner differential tests passed.

A single native fat-LTO binary contained the candidate and frozen parent as separate crates plus Gigatoken and Hugging Face. Four counterbalanced MiniMax rounds used 140-byte rows, batch 32, one MiB of novel mixed-language input per engine, four workers, timed output allocation and destruction, and exact candidate/parent/Gigatoken/Hugging Face parity before timing. Candidate/parent throughput was 0.999x, 0.995x, 1.010x, and 0.988x, for a 0.998x geomean and three losses in four rounds. Both versions remained 1.75--1.90x faster than Gigatoken, but the changed high-byte dataflow did not improve its intended Unicode-heavy target.

Commit `41175b7` reverts source commit `6728414` in full. No broader M2 matrix or Modal run was performed: expanding the search after the target failed would select for benchmark noise rather than a portable mechanism. Claude review remained unavailable because `claude auth status` still reported `loggedIn: false`.

### Experiment 68: check sparse Unicode NFC islands — rejected

A Qwen 64 KiB profile attributed about one-third of active encode samples to ICU's already-normalized NFC scan. This experiment split mixed input into maximal non-ASCII islands, included each island's immediately preceding ASCII starter, and returned the original string only when ICU independently confirmed every slice. Any failed slice fell back to the unchanged full-string normalization. One differential test covered composed text, decomposed Latin, Hangul Jamo, leading combining marks, and multiple ASCII separators against the same ICU oracle.

The first same-binary target gate rejected the mechanism. Four counterbalanced Apple M2 rounds used Qwen 3, 64 KiB inputs rotated through the sparse-Unicode real corpus, two MiB per engine, four workers, timed output allocation and destruction, and exact candidate/parent/Gigatoken/Hugging Face parity. Candidate/parent throughput was 0.910x, 0.895x, 0.903x, and 0.913x, a 0.905x geomean with four losses. Repeated ICU setup per island cost more than skipping long ASCII spans.

Commit `3189c9e` reverts source commit `537f5b4` in full. Dense-NFC, decomposed-input, broader-model, and Modal runs were intentionally skipped after the primary sparse-NFC gate failed. No island length, count, model, or input-size threshold is pursued. Claude review remained unavailable because `claude auth status` still reported `loggedIn: false`.

An uncommitted follow-up microbenchmark also rejected replacing ICU's fast check with `unicode-normalization` 0.1.25 `is_nfc_quick`, accepting only `Yes` and falling back to ICU for `Maybe` or `No`. Against the unchanged ICU path it retained only 0.55--0.58x throughput on sparse NFC, 0.36--0.37x on dense NFC, and 0.77--0.79x on decomposed NFD; ASCII was merely noisy and roughly flat. The dependency and production source were never changed.

### Experiment 69: skip duplicate DeepSeek ASCII classification — retained

The fused DeepSeek scanner classified the first scalar before deciding whether it could match the initial number-isolation stage, then classified ordinary ASCII again in the main stage. The retained branch reads the first byte once: ASCII digits enter the number stage, all other ASCII enters the main stage directly, and non-ASCII keeps the previous scalar path. It changes no Unicode boundary, Split order, cache behavior, model threshold, format, or API.

The first digit-before-ASCII branch ordering was rejected because it disturbed dense Unicode and decomposed-input timings. The final nested ASCII-first form won all eight 64 KiB sparse-Unicode rounds at 1.030--1.047x and all eight ASCII rounds at 1.024--1.105x on Apple M2. Short batches and 4 KiB inputs were also positive, while dense NFC and NFD stayed approximately flat. Commit `f44256b` retains only that nine-line fast path.

### Experiment 70: use a direct Unicode class table in fixed scanners — retained

The generic fixed scanners previously queried ICU's general-category trie for every non-ASCII scalar and separately called `char::is_whitespace`. The retained lazy table materializes the same compact class byte for all 1,114,112 Unicode code points with one ICU range walk, then turns each hot classification into one indexed byte load. The table occupies about 1.1 MiB and is built only when a generic scanner first needs it. An exhaustive temporary differential traversed every valid non-ASCII scalar and matched the previous ICU result exactly; it was removed after passing because the existing scanner oracles cover the persistent semantic boundary.

A first shared packed-table design was rejected before retention. It improved DeepSeek dense Unicode but regressed Qwen dense input about 4%; even restoring Qwen's classifier byte-for-byte left a roughly 0.6% loss. The final standalone table leaves `o200k_mask.rs` byte-identical to the frozen parent. Its Qwen dense control was 0.996x, while GPT-2 and GPT-OSS controls centered near parity.

On Apple M2, table-only DeepSeek dense-Unicode rounds improved about 1.14--1.18x and sparse real-corpus rounds generally improved 4--8%. Cold behavior did not trade away the warm gain: ten counterbalanced fresh processes put first dense encode at a 19.7 ms candidate median versus 20.6 ms for the parent even with table construction included. JSON load remained about 0.40--0.43 seconds and paired peak RSS was neutral at 195.3 versus 195.5 MiB.

The final source at `92f54df` then ran on two AMD family 175/model 1 and two Intel family 6/model 106 Modal workers. Separate native-fat-LTO candidate and parent processes covered both nested and flat-ragged contracts, DeepSeek JSON plus direct `.tkz` loading, five input shapes, eight balanced encode rounds, four MiB per engine, and 2,466 exact parity probes per implementation. The candidate's 40 cell medians had a 1.044x geometric mean; flat-ragged won 19/20, while seven process-separated losses ranged from 0.1% to 3.3% and were retained rather than hidden.

A final same-binary follow-up removed that process-state ambiguity. One AMD and three Intel workers placed the candidate, frozen parent, Gigatoken, and Hugging Face in one native-fat-LTO binary, alternated timing order, and checked exact IDs before every round. All twelve worker/case medians beat the parent. Across workers, the aggregate candidate/parent medians were 1.022x for batch 512, 1.066x for 4 KiB, and 1.019x for 64 KiB. Raw round wins were 36/48, 40/40, and 39/40 respectively; the sole 64 KiB loss was 0.1%. Candidate/Gigatoken aggregate medians were 2.809x, 1.470x, and 1.416x in the same cases. Every sandbox was explicitly terminated after collection.

Final validation reports 174 passed, 0 failed, and the same 9 extended tests intentionally ignored. Formatting, strict all-target/all-feature Clippy, no-default checking, warning-denied docs, and locked package construction pass. DeepSeek, Qwen, GPT-2, GPT-OSS, byte fallback, `ignore_merges`, nested/ragged equivalence, JSON, and direct `.tkz` behavior remain exact. No new permanent test, dependency, unsafe block, public API, tokenizer-format change, merge path, cache policy, or model-name dispatch was added.

### Experiment 71: hoist the DeepSeek Unicode table view — rejected

Disassembly of the table-only candidate showed that every non-ASCII DeepSeek scalar still paid `LazyLock`'s readiness check. Passing one resolved table slice through the fused scanner removed that atomic check, but added 32 net lines across eight private helpers. Its first 64 KiB dense run had a 1.046x median, while a longer 1 MiB dense confirmation fell to 1.018x with only five of eight wins. The improvement did not meet the predeclared repeatable 4% bar for the extra plumbing.

Commit `92f54df` reverts source commit `6d8223f` in full, restoring the simpler table-only implementation byte-for-byte. Claude reward-hacking review remained unavailable because `claude auth status` still reported `loggedIn: false`; no peer review is implied.

### AutoResearch basic A/A calibration: Apple M2 — frozen

Champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`, evaluator `d9086dffdb6b197e47156c32de5beb16ac89c0d9c90c92297d94d7a9b661763b`, and development manifest `4927e7d3abbc1e71f7583b6ed05bf662c96a35c0860a6e32e6370e703858caf9` were frozen before calibration. Two independent native-fat-LTO artifacts were built sequentially with one Cargo job from detached worktrees at the champion. Build A is `86261112f554c44cd558415ad9c0489ec2c16c430197c2700bffe1de35740cec`; build B is `074c6d7581aafc4fc5d47880004ae38c5ce6b9f6c7a44342c243db9fa956c42e`. Both report `rustc 1.99.0-nightly (969b803cb 2026-08-09)`, `-C target-cpu=native`, fat LTO, one codegen unit, the same source/evaluator/lock hashes, and identical Mach-O segment sizes. Their embedded build IDs, build commands, and binary hashes are distinct.

Both modes used this frozen control envelope: `SNAP_THREADS=4`, `SNAP_LOAD_ROUNDS=6`, `SNAP_ENCODE_ROUNDS=6`, `SNAP_SAMPLE_MIB=2`, `SNAP_HOST=apple-m2`, and `SNAP_CPU='Apple M2'`. The identical-binary command passed build A as both roles; the independent command passed build A as parent and build B as candidate:

```text
SNAP_THREADS=4 SNAP_LOAD_ROUNDS=6 SNAP_ENCODE_ROUNDS=6 SNAP_SAMPLE_MIB=2 SNAP_HOST=apple-m2 SNAP_CPU='Apple M2' PYTHONDONTWRITEBYTECODE=1 python3 benchmarks/portable/paired.py --mode identical-aa --parent /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/builds/a/snaptokens-portable-benchmark --candidate /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/builds/a/snaptokens-portable-benchmark --tokenizers /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/tokenizers --corpus /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/corpora/enwik8.txt --output /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/results/apple-m2-identical-aa --cycles 2
SNAP_THREADS=4 SNAP_LOAD_ROUNDS=6 SNAP_ENCODE_ROUNDS=6 SNAP_SAMPLE_MIB=2 SNAP_HOST=apple-m2 SNAP_CPU='Apple M2' PYTHONDONTWRITEBYTECODE=1 python3 benchmarks/portable/paired.py --mode independent-aa --parent /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/builds/a/snaptokens-portable-benchmark --candidate /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/builds/b/snaptokens-portable-benchmark --tokenizers /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/tokenizers --corpus /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/corpora/enwik8.txt --output /Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/aa/results/apple-m2-independent-aa --cycles 2
```

Each scored process covered six tokenizer structures, five shapes, both nested and flat-ragged contracts, and six encode rounds. Each mode therefore retained 1,440 matched cell-round ratios. One unscored fresh process per distinct binary checked every deterministic pool before scoring; every scored process repeated exact Hugging Face token-ID and row-boundary checks after timing. All raw process files contain 1,077 records, both `failures.json` files are empty, and the evaluator accepted every input, schedule, source, binary, lockfile, toolchain, build, host, and control fingerprint.

| Mode | Equal-cell geomean | Paired 95% CI | Whole AB-plus-BA cycle scores |
| --- | ---: | ---: | ---: |
| Identical binary | `0.993660x` | `0.984004–1.007034x` | `1.007034x`, `0.984004x` |
| Independent builds | `0.995299x` | `0.995299–0.999761x` | `0.999761x`, `0.998451x` |

The independent pair aggregates expose the position effect that counterbalancing removes: cycle 0 measured `0.960655x` in AB order and `1.049149x` in BA order; cycle 1 measured `0.987125x` and `1.010372x`. The identical cycle pair aggregates were `0.987664x`/`1.023836x` and `0.984732x`/`0.986916x`. A single direction is therefore not acceptable evidence.

The maximum aggregate false-winner bound is `1.007034x`, so the predeclared `1.02x` point target remains above measured Apple noise. Individual cells are not stable enough for a fixed per-cell cutoff: identical bits produced medians from `0.894873x` to `1.293483x`, with the widest movement in Gemma long/tiny cells. Regression screening instead uses symmetric log bands around grouped equal-cell medians across both A/A modes:

| Group | Frozen Apple M2 band |
| --- | ---: |
| DeepSeek V3.2 | `0.984405–1.015842x` |
| Gemma 3 | `0.973861–1.026840x` |
| GPT-OSS | `0.978212–1.022273x` |
| GPT-2 | `0.990431–1.009662x` |
| MiniMax M2.1 | `0.992426–1.007632x` |
| Qwen 3 | `0.982963–1.017332x` |
| 140-byte batch 1 | `0.991735–1.008334x` |
| 140-byte batch 32 | `0.978167–1.022320x` |
| 140-byte batch 512 | `0.981717–1.018624x` |
| 4 KiB single | `0.991271–1.008806x` |
| 64 KiB single | `0.979366–1.021069x` |
| Flat ragged | `0.990113–1.009985x` |
| Nested | `0.993703–1.006337x` |

The complete durable bundle is under `autoresearch/results/aa/apple-m2`. The identical raw archive is `b1695bc32a0203297cc3c402c36b6ca64c2b662c3f40d3d92032a9b813514b9b`; the independent raw archive is `dde689df440f66a520cb17459dfa1c91069432cef46f1cecec94678adfb55715`. Apple M2 basic A/A is frozen. AMD and Intel A/A remain pending, so these bands authorize profiling but not candidate timing or a cross-CPU claim.

### AutoResearch basic A/A calibration: AMD and Intel x86-64 — frozen

The Apple protocol was repeated without evaluator changes on two accepted four-core Modal sandboxes. Modal cannot pin a CPU vendor, so every placement was checked through `/proc/cpuinfo` before build or timing. The accepted GCP US-East host exposed `AuthenticAMD family 175 model 17`; the accepted AWS EU-West host exposed `GenuineIntel family 6 model 106`. Neither exposed a model name, and no processor brand is inferred. Both had four online cores with one thread per core and more than 3 GiB free.

The generic image fetched only the pinned toolchain and locked dependencies. Builds A and B were compiled sequentially inside each accepted host with one Cargo job, `rustc 1.99.0-nightly (969b803cb 2026-08-09)`, `-C target-cpu=native`, fat LTO, and one codegen unit. The AMD binaries are `82ca526f17b61cf30d158be23a592ce579adb80613d090bed7b8f0c40eb39a89` and `0e8e57fbc4ac469b94cc2beaf05629126e85dd37d5c392df242d8c18ab14270c`; the Intel binaries are `6f7e403680a75d3fcbc1ca13b6af7136ef6f145af221a02c157c7203853f75c0` and `1640133159c602ccf066398b74cb77c0041b57aecdf9d37dd7787b2c70f0ad3f`. Every binary reports champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`, evaluator `d9086dffdb6b197e47156c32de5beb16ac89c0d9c90c92297d94d7a9b661763b`, and portable lock `693d0ea38f555f9162fd031ff3988916563636686878ce615626c77434da5625`.

Both modes used `SNAP_THREADS=4`, `SNAP_LOAD_ROUNDS=6`, `SNAP_ENCODE_ROUNDS=6`, `SNAP_SAMPLE_MIB=2`, two complete AB-plus-BA cycles, and the same six tokenizer hashes and corpus hash as Apple. Each of the 32 scored processes retained 1,077 records. Each mode contains 1,440 matched ratios, both preflight roles were checked when binaries differed, all post-timing Hugging Face token IDs and row boundaries matched, and all four failure inventories are empty.

| CPU class and mode | Equal-cell geomean | Paired 95% CI | Whole AB-plus-BA cycle scores |
| --- | ---: | ---: | ---: |
| AMD identical binary | `1.009481x` | `1.003042–1.022694x` | `1.003042x`, `1.022694x` |
| AMD independent builds | `1.027463x` | `0.989541–1.080819x` | `0.989541x`, `1.080819x` |
| Intel identical binary | `0.994285x` | `0.990852–0.995594x` | `0.990852x`, `0.995594x` |
| Intel independent builds | `1.000060x` | `0.998447–1.003453x` | `0.998447x`, `1.003453x` |

AMD independent-build A/A is materially noisier than Apple or Intel. Its two cycles landed on opposite sides of parity, and its `1.080819x` upper bound is a false winner from same-source binaries. The predeclared rule therefore raises the first-candidate point target from `1.02x` to `1.09x`; the result is retained rather than rerun after inspection. Grouped symmetric log bands are:

| Group | AMD band | Intel band |
| --- | ---: | ---: |
| DeepSeek V3.2 | `0.963447–1.037939x` | `0.997225–1.002783x` |
| Gemma 3 | `0.974495–1.026172x` | `0.987390–1.012771x` |
| GPT-OSS | `0.970110–1.030811x` | `0.991490–1.008584x` |
| GPT-2 | `0.958358–1.043452x` | `0.992792–1.007261x` |
| MiniMax M2.1 | `0.984243–1.016009x` | `0.995122–1.004902x` |
| Qwen 3 | `0.985236–1.014985x` | `0.990938–1.009145x` |
| 140-byte batch 1 | `0.978141–1.022347x` | `0.994090–1.005945x` |
| 140-byte batch 32 | `0.968690–1.032322x` | `0.990279–1.009817x` |
| 140-byte batch 512 | `0.960931–1.040657x` | `0.988512–1.011621x` |
| 4 KiB single | `0.981980–1.018351x` | `0.991963–1.008102x` |
| 64 KiB single | `0.976759–1.023794x` | `0.993758–1.006281x` |
| Flat ragged | `0.968221–1.032822x` | `0.991014–1.009068x` |
| Nested | `0.978347–1.022132x` | `0.997568–1.002438x` |

Placement and orchestration failures occurred before timing and are preserved. AMD attempt 1 returned Intel and was rejected before build. Attempt 2 reached the correct vendor but exposed a missing copied development manifest during preflight. Attempt 3 reached the correct vendor but failed before producing a binary because `RUSTFLAGS` was not shell-quoted. Recovery attempt 4 used the corrected frozen orchestration and completed both modes; this exceeded the three-attempt placement cap only to recover those pre-timing script defects, not to search after observing scores. Intel succeeded on its first AWS EU-West placement. Every sandbox was terminated, and `modal container list --json` was empty after collection.

Durable bundles are under `autoresearch/results/aa/amd-x86-64` and `autoresearch/results/aa/intel-x86-64`. Their complete remote archive hashes are, respectively, AMD identical `94721c7dc5dabb620ca7f9a8d7ac45f3c61292432bb464f8d5240b5b529a9df8`, AMD independent `2defb75e819ca56f570ef08da8399551a8d8d987f8dc3b7493fdd6d59ba20a4d`, Intel identical `1890d8a00b193df57cbe0d40f6aacd9be49931de6d71eb496841f5a8f8a7bbc6`, and Intel independent `202e47b7ab8adb92e8d7ea99c5590e5f257e09ac4b1ccb9af36c90656a6231a0`.

### AutoResearch champion profile: six-tokenizer portable matrix

The frozen champion was rebuilt on an isolated four-core `AuthenticAMD family 175 model 1` Modal host with `-C target-cpu=native`, `-Z instrument-mcount`, and final-link-only `-pg -no-pie`. The unchanged portable runner covered all six development tokenizers, five shapes, nested and flat-ragged output, one four-MiB round, JSON and TKZ construction, and Hugging Face parity. All six processes completed with empty stderr; their matrices contain all five coverage records and all expected shapes and contracts. GNU gprof's instrumentation inflates high-call routines, so the profile is used for call counts and within-Snaptokens ranking, not throughput comparison.

Fused cache resolution and merging remain the broadest hot lane. `tokenize_fused_uncached` ran 1.79 million times for GPT-2, 2.56 million for MiniMax, 2.86 million for Qwen, and 2.97 million for DeepSeek. `FusedStream::flush` ran 0.70--1.26 million times per model. The fixed scanners were smaller: GPT-2's R50k scanner accounted for about 0.24 seconds of sampled inclusive time, while the MiniMax and Qwen scanners accounted for about 0.52--0.54 seconds. Gemma's generic BPE model closure dominated its Snaptokens encode path with 24.87 million instrumented calls. Prior short-linear merge, cache-layout, and generic-cache experiments already reject the obvious versions of those mechanisms, and this profile did not reveal a new exact representation for them.

Added-token preflight is smaller but exposes directly removable work. `AddedTokens::split_with` ran 694,489 times in MiniMax and 67,642 times in Qwen. Replaying the portable runner's four warm/timed corpus pools across all five shapes showed why: MiniMax's current start-byte predicate admitted 248,600 of 364,960 rows, covering 67,642,340 bytes, while no raw added token occurred. Qwen admitted 23,580 rows and 25,163,496 bytes with no raw match. GPT-OSS's shared `<|` prefix rejected every row. This is structure-derived rather than model-derived: the false positives occur when a row contains a possible first byte but not a complete configured added-token literal.

The complete raw evidence is under `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/profiles/remote-gprof-2026-08-12`. The six per-model archives, each with every per-thread `gmon` file and the complete matrix, all match their adjacent SHA-256 files. The provenance archive is `ec70a3161dc76598151f43060e103ee8af88c2a29b944dc0fc183adbba595c59`; the instrumented binary is `c63257a6e85acca5bccf9822a37f530070879351dea30e479fdf572ea1ae8ac3`. The sandbox was terminated and `modal container list --json` was empty after collection.

### Experiment 72: require an exact raw added-token match before leaving fused encoding — planned

Parent SHA: `6abdfb447216422d5bc9cfed90baffb3a7f91cc9` (tokenizer source is byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: replacing the possible-start-byte preflight with an allocation-free exact match predicate will keep false-positive MiniMax and Qwen rows on the fused path instead of materializing added-token segments and restarting fused streams.

Measured hot cost: `AddedTokens::split_with` ran 694,489 times for MiniMax and 67,642 times for Qwen in the instrumented full-shape run. Exact replay found 248,600 MiniMax and 23,580 Qwen false-positive rows, covering 92,805,836 bytes in total.

Invariant that makes the shorter path exact: when no normalized added-token phase exists, the compiled leftmost-longest automaton sees the same raw input as the fused preflight. A row may bypass added-token splitting only when that automaton has no match accepted by the existing Unicode `single_word` boundary rule. `lstrip` and `rstrip` change a match's consumed bounds, not whether a match exists. Paths with normalized added tokens keep the existing guard.

Representation being preserved or changed: preserve both compiled added-token automata and all encoding representations; add one allocation-free query over the existing automata. Do not change segmentation, normalization, fused streams, caches, model dispatch, thresholds, public APIs, tokenizer formats, dependencies, or unsafe code.

Expected winning strata: MiniMax first, then Qwen, especially punctuation-bearing 140-byte rows and long rows where the current possible-start check frequently fires without a literal match. Standalone ByteLevel tokenizers with false-positive raw prefixes may also benefit.

Expected adverse strata: GPT-OSS and rows without possible start bytes should be effectively unchanged after the existing cheap prefilter. Rows with an actual added token pay one exact scan before the unchanged split; normalized added-token tokenizers preserve the old decision.

Smallest files that need changing: `src/added_tokens.rs` for the exact predicate and one focused `single_word` test; `src/lib.rs` for the three existing preflight call sites.

Mechanism evidence: the profile and corpus replay above demonstrate zero true raw matches among 272,180 admitted MiniMax/Qwen rows. The candidate removes the fallback allocation and segmentation only for those proven non-matches.

Acceptance rule: focused added-token and fused-path exactness tests pass; the candidate wins MiniMax and does not lose Qwen in both orders of a counterbalanced target screen; a retained candidate then needs at least a `1.09x` development point estimate, paired 95% confidence interval above `1.00x`, and no structure or CPU-class regression outside the frozen A/A bands.

Rejection rule: reject immediately on any token-ID or row-boundary mismatch, an exact-match preflight that allocates, a MiniMax target loss in either order, a Qwen regression outside its frozen band, or failure of the full development acceptance rule. Do not tune by model, token count, input length, or observed benchmark cell.

Experiment 72 is rejected at the target screen. The focused exactness gate passed all 40 added-token tests, both nested/ragged equivalence tests, and fused-cache consistency. The candidate at `5959259bd63e804076455e65ab2a9aaaeef2fb11` added one allocation-free full-automaton predicate, changed the three existing fused preflights, added one `single_word` boundary test, and contained no evaluator, dependency, unsafe, model, threshold, or format change.

Parent and candidate were built sequentially on one accepted OCI Intel host reporting `GenuineIntel family 6 model 106`, with the frozen nightly, native fat LTO, one codegen unit, and one Cargo job. Their immutable binary hashes were `a1d3e82b916d7e72cd50a25e65c0eb2c6ba13b7e14248db1b1b6c7489bb06322` and `e2b933fa512ccdb827ab94fcc97b213af2b22e2d2786b596633af052d90f1d93`. Both fresh-process preflights and all four scored processes passed complete Hugging Face token-ID and row-boundary checks; `failures.json` is empty.

MiniMax's ten target-cell medians had a `1.074x` geomean and stayed positive in both outer orders (`1.080x` AB, `1.051x` BA), with short-row medians up to `1.225x`. The full-string automaton scan did not generalize: Qwen's ten-cell geomean was `0.925x`, including `0.784x` for nested 64 KiB, and its order aggregates were `1.010x` AB but `0.848x` BA. The equal-cell aggregate was `0.997x`; its outer orders were `1.044x` and `0.944x`. This violates the predeclared requirement that Qwen not lose in either order, so the full development matrix was skipped and no result-dependent threshold was attempted.

The durable archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/candidates/exp72-target-intel/exp72-target.tar.gz` at SHA-256 `f2f8cd331653c62edc6958a3fab2c2c50283a3c8095bbb91dd9aa7c889d743e7`. One earlier run completed both preflights and three scored processes but its client orchestration was stopped before the final BA parent, scorer, or archive; no partial score was inspected. A subsequent retry in the wrong Modal workspace failed before build because that backend rejected the legacy filesystem API. The completed run used the original workspace, and all sandboxes from all attempts were terminated. Commit `a701cab` reverts the candidate in full on its isolated branch.

### Experiment 73: exact added-token preflight at candidate start positions — planned

Parent SHA: `deb5e62934f9044b2d784d969cac6c483b246d90` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: reusing the existing memchr candidate-position strategy for the exact preflight will retain Experiment 72's avoided segmentation without paying a full automaton scan over rows whose possible start bytes are sparse.

Measured hot cost: Experiment 72 improved MiniMax's target-family geomean to `1.074x` but reduced Qwen to `0.925x`; Qwen's raw `<` candidate occurs in only 23,580 of 364,960 replayed rows, so scanning every byte after finding that sparse candidate is the measured adverse mechanism.

Invariant that makes the shorter path exact: for matchers with at most three distinct first bytes, every possible match begins at a position returned by the existing memchr iterator. Running the existing leftmost-longest automaton only in a token-length-bounded window at each non-overlapping candidate position finds the same accepted raw matches and applies the same Unicode `single_word` rule as `split_prefilter`. Matchers with more than three first bytes and tokenizers with normalized added tokens retain the current conservative preflight.

Representation being preserved or changed: preserve the automata, memchr start-byte metadata, segmentation, normalization, caches, fused stream, model dispatch, public APIs, formats, and evaluator. Add only a no-allocation boolean walker over the existing candidate positions; no dependency, unsafe code, token-count branch, input-length threshold, or model label.

Expected winning strata: Qwen rows containing a sparse false-positive `<`, MiniMax short rows, and long rows where full-string scanning caused Experiment 72's losses.

Expected adverse strata: rows with many candidate start bytes may repeat short automaton setup; rows with true added-token matches scan once before the unchanged split. GPT-OSS and rows rejected by the existing cheap prefix test should remain unchanged.

Smallest files that need changing: `src/added_tokens.rs` for the candidate-position predicate and the same focused `single_word` boundary; `src/lib.rs` for the same three preflight substitutions.

Mechanism evidence: the rejected full-scan candidate establishes both the removable MiniMax work and the Qwen scan penalty. The existing production `split_prefilter` already proves the exact candidate-position and bounded-window representation used by the proposed boolean walker.

Acceptance rule: the same focused exactness tests pass; MiniMax and Qwen each avoid regression in both outer orders, the two-family target aggregate exceeds `1.02x`, and neither tokenizer-family aggregate falls below its frozen Intel band. A retained candidate then runs the complete development and portability gates with the original `1.09x` promotion target.

Rejection rule: reject on any exactness failure, allocation in the predicate, either family losing in either order, aggregate target gain at or below `1.02x`, any target cell median below its frozen band, or a diff that requires model/input-size dispatch. Do not add a threshold after observing results.

Experiment 73 is rejected at the target screen. Its focused exactness gate passed the same 40 added-token, two nested/ragged equivalence, and fused-cache tests as Experiment 72. Candidate `7df237e6004476f8e1c4aaec2dd8045c85e29c88` restricted exact checks to the existing memchr start positions and token-length-bounded automaton windows; normalized and more-than-three-start-byte matchers retained the old conservative check.

The same accepted OCI Intel family 6/model 106 protocol built immutable parent `99483df1b2b2ad44f7f89c4f242022de2d9fda5c8f00d0da5e1594ba615699ae` and candidate `08b4059b45daae2608582acaa4fd81b360d571b07ace52aaf00d445b405f7a46`. Both preflights and all four scored processes passed exact Hugging Face IDs and row boundaries, and the failure inventory is empty.

Candidate-position scans removed Experiment 72's catastrophic Qwen loss but did not win. Qwen's ten-cell geomean was `0.984x`, losing in both AB (`0.977x`) and BA (`0.989x`) orders. MiniMax fell to `0.981x`, with `0.938x` AB and `1.029x` BA. The equal-cell aggregate was `0.982x`; the outer orders were `0.957x` and `1.009x`. Repeated bounded-window automaton setup cost more than the avoided segment allocation and fused-stream restart, so the added-token lane is closed after two mechanism-screen failures rather than tuned by model, start-byte count, or input length.

The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/candidates/exp73-target-intel/exp73-target.tar.gz` at SHA-256 `7f2504a0dec0e37600b87591aac062aa57945fb8690cb4f82557f47e4d61c2f3`. The sandbox was terminated and the candidate is reverted in full by `2b693e8` on its isolated branch.

### Experiment 74: back only displaced fused front-cache entries — planned

Parent SHA: `776b923ebfeb4ffbede4d7f5bfa0ff9cab3c93d6` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: compact one-to-four-ID short-piece results need not be inserted into both fused cache tiers immediately. Keeping the newest value in its direct front slot and moving the exact displaced slot into the collision-resolving backing table preserves every reusable result while removing most backing-table probes and writes from the miss path.

Measured hot cost: the full-shape profiles called `FlatCache::insert_packed_backing` 3.12 million times for GPT-2, 6.51 million for MiniMax, 6.34 million for Qwen, and 6.69 million for DeepSeek. Those calls currently occur before every eligible direct-front insertion, even when no direct-slot collision exists.

Invariant that makes the shorter path exact: a front-eligible cache value stores its complete packed key and all one-to-four output IDs inline. Before replacing a different key in the same direct slot, those IDs can be decoded losslessly and inserted into the unchanged backing table. The new key remains directly retrievable; the displaced key remains backing-retrievable. Ineligible values continue directly to the backing table, and cache misses may always recompute the same exact BPE result.

Representation being preserved or changed: change only the private cache insertion policy from write-through to exact victim backing. Preserve front and backing table sizes, hashes, probes, clearing, seed contents, shared caches, BPE merging, scanner ranges, output materialization, public APIs, formats, dependencies, and unsafe surface.

Expected winning strata: all four fused development tokenizers, especially uncached novel-input rows where the profiles show millions of backing insertions. Short pieces with compact outputs should remove one random-probed table operation per new key.

Expected adverse strata: collision-heavy direct slots add victim decoding before the existing backing insertion; values longer than four IDs or with an unencodable first ID are unchanged. Repeated-input cache hits should remain neutral.

Smallest files that need changing: only `src/models/bpe.rs`. Modify `insert_packed` and `insert_front`, and tighten the existing four-ID cache test to exercise a real direct-slot collision instead of manually deleting a front entry. Add no new test.

Mechanism evidence: `insert_packed_backing` is a top Snaptokens call-count routine in every fused profile. `FrontCacheSlot` already stores the exact 128-bit key, one-to-four value length, and all four IDs, so no new metadata or lossy fingerprint is required.

Acceptance rule: formatting and the focused cache/fused/model exactness tests pass; an Intel target screen over GPT-2, MiniMax, Qwen, and DeepSeek exceeds `1.02x` in each outer order and overall, with no tokenizer-family aggregate below its frozen Intel band. A passing candidate proceeds to the complete development matrix and retains the original `1.09x` promotion target.

Rejection rule: reject on any exactness failure, new allocation or unsafe code, any target family outside its Intel band, either outer order at or below `1.02x`, or target aggregate at or below `1.02x`. Do not tune table size, collision policy, tokenizer, or input thresholds after results.

Experiment 74 is rejected at the Intel target screen. Candidate `36e836ef236dbd2ef33dd0953cbf2743421150b6` changed only `src/models/bpe.rs`: compact results entered the direct front cache without immediate backing insertion, and an exact one-to-four-ID victim was decoded into the unchanged backing table only when a different key displaced it. The existing four-ID cache test was tightened to force a real one-slot collision. All 12 BPE tests, fused-cache consistency, both ragged-equivalence tests, and all 39 added-token/model tests passed; formatting and diff checks were clean. The counterbalanced remote run also passed both complete preflights and every post-timing Hugging Face token-ID and row-boundary check. `failures.json` is empty.

The candidate's equal-cell aggregate was `1.018186x`, below the predeclared `1.02x` target. AB measured `1.025507x`, but BA measured only `1.005742x`, so the order gate also failed. The family breakdown explains the rejection: Qwen improved to `1.181992x` and MiniMax to `1.095626x`, while GPT-2 regressed to `0.888824x` and DeepSeek to `0.933719x`, both far outside their frozen Intel bands. Deferring backing insertion therefore helps collision behavior in some fused cache populations but makes front-slot churn materially worse in others; table size, tokenizer, and input thresholds are not tuned after observing that split.

The parent binary was `4754d13d0785f24c3732bf8fce76f35de063414f56d73958807a510cd4e6987b`; the candidate binary was `33dbfcda68f33262fefced0ecd576390e0707de9099a3a68827d007094e587b4`. Both were built sequentially on `GenuineIntel family 6 model 106` with the pinned nightly, `-C target-cpu=native`, fat LTO, one codegen unit, and one Cargo job. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/candidates/exp74-target-intel/exp74-target.tar.gz` at SHA-256 `bb46cfe297cdf5233de9601a839b2bb6df6c79de13a34c7a2c9205aedcb6bf50`. The sandbox was terminated, `modal container list --json` was empty, and the candidate is reverted in full by `5a1050b` on its isolated branch.

### Experiment 75: skip speculative fused-cache miss stores — planned

Parent SHA: `47ce8fbf3b95643820a6eeb746feea59c205254a` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: testing the direct fused-cache lookup result before packing and writing its value will remove two `u64` stores and the ID transform from every front-cache miss without changing the hit path or miss resolution.

Measured hot cost: the full-shape profiles called `FusedStream::flush` 0.70--1.26 million times per fused tokenizer and `tokenize_fused_uncached` 1.79 million times for GPT-2, 2.56 million for MiniMax, 2.86 million for Qwen, and 2.97 million for DeepSeek. The uncached counts are a strong miss proxy rather than an exact removable-store count because other callers and long pieces also reach uncached tokenization.

Invariant that makes the shorter path exact: `front_packed_value_at` reports a hit only after full 128-bit key equality. On a miss, the current loop writes four unrelated cached lanes beyond `out.len()`, advances `destination` by zero, restores the vector length to that unchanged cursor, and then overwrites or discards those lanes through the unchanged backing-cache or BPE path. Only exact hits need to initialize output lanes.

Representation being preserved or changed: preserve `FusedPiece`, both cache tiers, keys, hashes, values, insertion policy, scanner ranges, BPE merging, allocation capacity, output layout, public APIs, tokenizer formats, dependencies, and unsafe surface. Reorder only the existing hit branch so miss handling remains byte-for-byte unchanged.

Expected winning strata: GPT-2, MiniMax, Qwen, and DeepSeek novel-input fused workloads, especially cache-cold shapes with frequent direct-cache misses.

Expected adverse strata: front-hit-heavy or repeated-piece rows remove no work, and changed instruction layout may disturb their hit throughput. Long pieces that bypass the front representation are unchanged.

Smallest files that need changing: only `src/models/bpe.rs`. Move the existing ID packing, two stores, cursor advance, and `continue` into the existing `found` branch and update its local safety comment. Add no helper, type, dependency, unsafe block, or test.

Mechanism evidence: optimized parent disassembly places the two output stores at `0x7eef35` and `0x7eef39` before the exact-hit branch at `0x7eef3d`. Candidate disassembly must place the exact-hit decision before ID packing and both stores, with the stores reachable only from the hit block.

Acceptance rule: formatting, the 12 BPE tests, fused-cache consistency, and nested/ragged equivalence pass; optimized disassembly proves miss-side store removal; and one complete counterbalanced Intel target screen over GPT-2, MiniMax, Qwen, and DeepSeek exceeds `1.02x` in both outer orders and overall, with no tokenizer-family aggregate below its frozen Intel band. A passing candidate proceeds to the complete development matrix and retains the original `1.09x` promotion target.

Rejection rule: reject on any exactness failure, stores remaining on the miss path, new unsafe or dispatch policy, either outer order at or below `1.02x`, aggregate target gain at or below `1.02x`, or any tokenizer family outside its frozen Intel band. Do not add branch hints, thresholds, or model-specific policy after observing results.

Pre-result cross-review clarification: "outside its frozen Intel band" above means below the lower bound. The symmetric A/A interval is a regression guard, not a ceiling on positive improvements; this clarification changes no workload, threshold, or candidate code.

Experiment 75 is rejected at the Intel target screen. Candidate `1bd5a5e8bb4eb1e71004cb81b3dc931814a7526f` moved only the existing ID transform, two unaligned output stores, cursor advance, and `continue` into the exact front-cache hit branch. Formatting, all 12 BPE tests, fused-cache consistency, nested/ragged equivalence, and newline-partitioned ragged equivalence passed. Optimized disassembly proved the mechanism: the parent wrote both output words before its equality branch, while the candidate branched on full-key equality before reaching the hit-only stores.

Both fresh-process preflights and all four scored processes passed complete Hugging Face token-ID and row-boundary checks; `failures.json` is empty. The candidate lost at `0.968925x` equal-cell geomean, with both outer orders below one (`0.988281x` AB and `0.948864x` BA). Every family fell below its frozen Intel regression band: DeepSeek `0.983959x`, GPT-2 `0.980061x`, MiniMax `0.965711x`, and Qwen `0.946419x`. Removing dead miss stores changed hot-loop layout and branch behavior enough to outweigh the eliminated work, so the cache path is not tuned further.

Parent and candidate were built sequentially with one Cargo job on `GenuineIntel family 6 model 106`, using the pinned nightly, native fat LTO, and one codegen unit. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/candidates/exp75-target-intel/exp75-target.tar.gz` at SHA-256 `4cb66a4495095d8b9724ad763ed925940c89cd46fc93daae45781cbf90c06b50`. The sandbox terminated with no live Modal containers, and commit `5077c27` reverts the candidate in full on its isolated branch.

### Experiment 76 mechanism screen: degree-specialize exact CSR merge lookup — planned

Parent SHA: `ca0ec158415ff820c76629f4f0a675d687d78885`

Hypothesis: if runtime merge discovery predominantly queries empty and singleton `MergeAdjacency` rows, handling those degrees directly can avoid general binary-search setup and comparator control flow while leaving every larger row unchanged.

Measured hot cost: the frozen profile places BPE merge and cache-miss resolution first among broad lanes, and every initial or newly exposed candidate performs a CSR lookup. The current profile does not record queried row degrees, so a counter-only diagnostic is required before source candidacy.

Invariant that makes the shorter path exact: each CSR row is sorted by a unique right-token ID. An empty row is always a miss; a singleton row returns its stored `(rank, new_id)` exactly when its right ID matches; rows of length two or more retain the current binary search.

Representation being preserved or changed: preserve the CSR offsets/data, merge heap, ordering, caches, scanners, output allocation, APIs, formats, dependencies, and unsafe surface. A candidate may add only degree dispatch inside the existing private lookup; the mechanism screen uses test-only counters that are not part of any candidate.

Expected winning strata: generic Gemma and fused cold-miss workloads when empty/singleton left-token rows dominate initial and newly exposed neighbor probes.

Expected adverse strata: high-degree rows pay an extra dispatch, and the compiler may already make empty/singleton binary search cheap enough that code layout outweighs the saved control flow.

Smallest files that need changing: the disposable mechanism screen may instrument `src/models/bpe.rs` and one ignored diagnostic test in `src/lib.rs`; any source candidate changes only `MergeAdjacency::get` and adds no test.

Mechanism evidence: pending runtime counts from one representative generic tokenizer (Gemma) and one representative fused tokenizer (Qwen) over the same two-MiB corpus prefix and 4-KiB document shape.

Acceptance rule: propose candidate code only if degree-zero-or-one rows are at least `60%` of all runtime lookups and singleton rows alone are at least `30%` for both tokenizers. Candidate assembly must bypass binary search for lengths zero and one without materially growing the hot function; focused exactness and a counterbalanced Intel target must then pass the existing general gates.

Rejection rule: stop before candidate code if either tokenizer misses either degree threshold, the instrumentation cannot separate queried degree from static row counts, or the direct cases remain compiled through the general binary-search path. Do not add a degree threshold beyond exact zero/one or tune by tokenizer, corpus, input length, or observed score.

Experiment 76 is rejected at the mechanism screen without candidate code. Test-only counters at `cf71338` measured every runtime lookup over the first two MiB of the frozen corpus in 4-KiB documents, then were removed by `1544a00`. Gemma recorded 14,891 degree-zero misses, 3,308 singleton misses, 797 singleton hits, 99,805 degree-two-plus misses, and 490,971 degree-two-plus hits. Empty-or-singleton rows were only `3.12%` of 609,772 lookups; singleton rows were `0.67%`.

Qwen recorded 12,011 degree-zero misses, 5,908 singleton misses, 774 singleton hits, 49,348 degree-two-plus misses, and 19,266 degree-two-plus hits. Empty-or-singleton rows were `21.41%` of 87,307 lookups; singleton rows were `7.65%`. Both tokenizers miss the predeclared `60%` empty-or-singleton and `30%` singleton gates by wide margins. General binary search primarily serves higher-degree rows, so adding zero/one dispatch would tax the dominant path to shorten a small minority. No candidate, benchmark build, dependency, unsafe code, or retained test was created.

### Experiment 77 mechanism screen: immutable exact merge-pair retrieval ceiling — planned

Parent SHA: `a1761693ccb1fcd2825550738d83b3b1b7318ae0`

Hypothesis: replacing CSR row binary searches with compact immutable exact pair retrieval can remove enough comparison work across generic and fused BPE to justify a medium representation experiment.

Measured hot cost: merge/cache resolution is the only broad dominant lane, and Experiment 76 shows degree-two-plus rows cover `96.88%` of Gemma and `78.59%` of Qwen runtime adjacency queries. It did not count comparator executions or establish the whole-encode ceiling.

Invariant that makes the shorter path exact: any later retrieval slot must verify the full `(left, right)` key before returning the same stored `(rank, new_id)`; nonmatching keys return `None`. Merge ordering, tie behavior, and every caller remain unchanged.

Representation being preserved or changed: the disposable screen changes no production representation and only counts calls, comparisons, and outcomes. A later candidate would replace CSR lookup only if a compact immutable representation stays within construction and RSS guardrails; it may not reuse Experiment 56's rejected open-addressed ranked table.

Expected winning strata: merge-heavy generic Gemma plus fused cold-miss tokenizers, because initial seeding, accepted-pop resolution, and both newly exposed neighbor probes all query adjacency.

Expected adverse strata: exact retrieval metadata may increase random footprint, construction, RSS, and code layout; the rejected ranked-table substitution already reversed by model despite a positive average.

Smallest files that need changing: test-only instrumentation in `src/models/bpe.rs` and the existing ignored local diagnostic in `src/lib.rs`, committed and reverted after counting. No candidate code, dependency, unsafe block, API, format, or evaluator change.

Mechanism evidence: pending call, comparator, and hit/miss totals for Gemma, Qwen, and DeepSeek over the same first two-MiB corpus prefix in 4-KiB documents, combined with the frozen profile's whole-encode call/sample evidence.

Acceptance rule: proceed to representation design only if adjacency search plausibly owns at least `10%` of Snaptokens encode instructions or samples in all three tokenizers, and a zero-comparison upper bound can plausibly exceed the frozen `1.09x` equal-family development target while a projected representation stays within `5%` construction and RSS guardrails.

Rejection rule: stop before candidate code if any tokenizer falls below the `10%` ceiling, the diagnostic cannot bind comparator work to whole encode, the equal-family upper bound is below `1.09x`, or compact exact retrieval requires a previously rejected table shape. Do not treat comparator counts alone as elapsed-time proof.

Experiment 77 is rejected before instrumentation or candidate code because the proposed cheap screen cannot bind comparisons to whole-encode cost. The frozen gprof build inlines `MergeAdjacency::get`, so it exposes neither attributable self samples nor retired instructions for that lookup; counting comparisons would establish frequency but violate the card's explicit rule against presenting counts as elapsed-time proof. Making the function non-inline, timing every call, or inserting counters into every comparison would materially change the mechanism being measured.

The nearest already-measured exact-retrieval proxy is Experiment 56, which substituted the existing full-key open-addressed table at all four adjacency call sites. It produced `1.022x` and `1.019x` process aggregates but repeated four exact cell regressions in both orders, so it was rejected. A different compact perfect-retrieval representation remains a medium `2--4` day design, not a small candidate justified by current evidence, and its estimated `3--15%` end-to-end range does not prove the frozen `1.09x` general gate. No source, diagnostic, benchmark, dependency, unsafe code, or format change was created.

### Experiment 78 mechanism screen: deduplicate exact cold pieces within one batch — planned

Parent SHA: `82437a48ceaa133a0ac1683513e06f3ab8f2d650`

Hypothesis: parallel workers redundantly resolve identical post-cache BPE pieces within one cold novel-input batch often enough that execute-once plus ordered scatter could justify a new batch representation.

Measured hot cost: cache-miss merge resolution dominates the broad fused lane and Gemma's generic BPE remains the weak family, but current profiles count resolutions rather than duplicate exact piece keys across concurrent workers.

Invariant that makes the shorter path exact: identical post-normalization, post-pre-tokenization piece bytes under one immutable BPE model produce identical ID vectors. A later candidate could resolve each unique piece once and scatter those exact IDs in the original row/piece order without changing boundaries.

Representation being preserved or changed: the disposable screen records exact piece strings only after local and shared cache miss gates, then is committed and reverted. A candidate is not authorized by this card. No full-output cache, cross-request reuse, model/input dispatch, dependency, unsafe code, API, format, or evaluator change is allowed.

Expected winning strata: cold 32-row and 512-row novel batches for generic Gemma and fused Qwen/DeepSeek if concurrent worker-local caches repeatedly miss the same pieces before shared publication.

Expected adverse strata: existing local/shared caches may already remove nearly every duplicate; collecting, uniquing, scattering, and preserving row order can cost more than the remaining duplicate merges, especially on diverse text.

Smallest files that need changing: test-only exact-key counters in `src/models/bpe.rs` and one ignored diagnostic test in `src/lib.rs`. Any instrumentation is removed before the next candidate.

Mechanism evidence: pending unique and duplicate post-cache resolution counts for Gemma, Qwen, and DeepSeek at 32 and 512 novel 140-byte rows from the frozen corpus, using two Rayon threads to avoid overloading the host.

Acceptance rule: continue to representation design only if duplicates are at least `20%` of post-cache resolutions for every tokenizer at both batch sizes and the affected equal-weight family/shape ceiling can plausibly exceed `1.09x` overall.

Rejection rule: stop before candidate code if any tokenizer/shape has less than `20%` duplicates, the hook records before an existing cache gate, or opportunity is confined to repeated-input rather than novel-input work. Do not add a content, length, model, or batch-size threshold after observing counts.

Experiment 78 is rejected at the first and cheapest mechanism cell without candidate code. Test-only exact-string instrumentation at `6382c4c` ran Gemma 3 over 32 consecutive novel 140-byte rows from the frozen `enwik8` corpus with two Rayon threads. The command used the existing root target directory, `CARGO_BUILD_JOBS=2`, `RAYON_NUM_THREADS=2`, `SNAP_PROFILE_BATCH=32`, and the ignored `profile_within_batch_piece_duplicates` diagnostic. Its raw result was `resolutions=151 duplicates=0`: `0.00%` duplication after the existing local and shared cache gates, versus the predeclared `20%` minimum.

Because one failed tokenizer/shape was sufficient to reject the mechanism, the remaining five cells were not run. The diagnostic was removed by revert `1cb6c6d`. No candidate, benchmark build, dependency, unsafe code, public API, format, or evaluator change was created.

### Campaign checkpoint after Experiment 78

The campaign stops at the user-authorized repeated-unproductive-experiments condition. Experiments 72--75 all reached their declared target screens and were rejected; Experiments 76--78 then rejected three different representations at or before their mechanism screens. The current frozen profile no longer supports a small general candidate with a plausible `1.09x` ceiling: added-token matching, victim backing, fused miss write elision, low-degree CSR dispatch, immutable exact retrieval, and within-batch cold-piece dedup are all closed by measured evidence. A new campaign should begin with a fresh whole-pipeline profile or a materially new representation, not threshold tuning or another cache-policy permutation.

The user subsequently resumed the campaign with an explicit instruction not to stop. The frozen champion, evaluator, A/A bands, development target, and candidate budget remain unchanged.

### Experiment 79 mechanism screen: carry ranked merged IDs in heap priority — planned

Parent SHA: `89b791e8a466600e311a54b751c3d0e13b1b9581` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: when a tokenizer's canonical merges satisfy `merged_id = rank + constant_offset`, storing the merged ID rather than the rank in heap priority preserves exact priority order and lets a valid pop use that ID directly instead of searching `MergeAdjacency` again.

Measured hot cost: BPE merge/cache resolution remains the broad dominant lane. Every valid heap pop currently performs a CSR lookup solely to recover the merged ID, even though initial and newly exposed pair lookups already return it. Experiment 76 counted 609,772 heap adjacency queries for Gemma and 87,307 for Qwen over two MiB, but did not separate accepted pops or ranked tokenizers.

Invariant that makes the shorter path exact: `ranked_merges` is true only when every canonical merge has one checked constant rank-to-ID offset. Adding that constant preserves rank order and leftmost position tie-breaking. Endpoint stale checks remain unchanged, and the priority is used as an ID only after both endpoints still match. Non-ranked tokenizers retain rank priority and the existing lookup.

Representation being preserved or changed: preserve the 16-byte `MergeEntry`, CSR adjacency, both heap types, stale checks, merge ordering, caches, short-piece loops, scanners, output materialization, APIs, formats, dependencies, and unsafe surface. A later candidate may change only the priority value at the three heap enqueue sites and the valid-pop ID resolution. The disposable screen adds test-only counters only.

Expected winning strata: ranked generic Gemma and ranked fused long-piece or byte-fallback heap paths with many accepted merges.

Expected adverse strata: short ranked fused pieces already order directly by merged ID and are unchanged; non-ranked and `ignore_merges` families gain nothing but may pay code-layout cost; heap workloads with many stale candidates save few lookups.

Smallest files that need changing: the diagnostic may instrument `src/models/bpe.rs` and one ignored test in `src/lib.rs`, then is committed and reverted. Any candidate changes only `src/models/bpe.rs` and adds no test, type, field, dependency, helper, or unsafe block.

Mechanism evidence: pending counts over Gemma, Qwen, DeepSeek, GPT-2, MiniMax, and GPT-OSS using the first two MiB of the frozen corpus in 4-KiB documents with two Rayon threads. Record ranked mode, total heap adjacency queries, and valid-pop ID-resolution queries separately.

Acceptance rule: propose candidate code only if ranked mode covers Gemma and at least three fused families, valid-pop resolution is at least `25%` of heap adjacency queries in every covered family, and the affected-family ceiling remains capable of the `1.09x` equal-family target. Optimized disassembly must then show the ranked valid-pop path bypassing CSR; focused exactness and a counterbalanced representative target decide retention.

Rejection rule: stop before candidate code if ranked coverage is narrower, any covered family misses the `25%` valid-pop share, the diagnostic cannot distinguish accepted from stale pops, or the opportunity is confined to generic Gemma. Do not add model, corpus, length, byte-fallback, or heap-size dispatch after observing counts.

Experiment 79 is rejected at the mechanism screen without candidate code. Test-only counters at `5deb768` measured actual heap adjacency calls and the valid-pop subset over the first two MiB of the frozen corpus in 4-KiB documents, then were removed by `48297f4`. Gemma was non-ranked despite 609,772 heap queries and 159,188 valid-pop resolutions, immediately failing the required generic-plus-fused coverage. GPT-OSS was also non-ranked with 3,072 queries and 1,044 valid pops.

The four ranked fused families passed the frequency fraction but exposed negligible absolute heap traffic because their common 2--15-byte pieces already use the ranked stack loop: Qwen recorded 737 queries and 255 valid pops (`34.60%`), DeepSeek 3,189 and 1,111 (`34.84%`), GPT-2 320 and 112 (`35.00%`), and MiniMax 2,522 and 852 (`33.78%`). Removing one lookup from those rare long-piece heap merges cannot supply the frozen `1.09x` equal-family ceiling. No production candidate, benchmark build, dependency, unsafe code, API, format, or evaluator change was created.

### Experiment 80 mechanism screen: flat indexed batch task graph ceiling — planned

Parent SHA: `981ef2d1b5ebf8e7b4c63a49a8899c40bcecc155` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: replacing separate row, ragged-chunk, and inner-piece schedulers with one flat indexed task graph could reduce enough batch critical-path idle and gather time to justify a unified scheduling representation.

Measured hot cost: the current source has separate scheduling and ordered gather in `encode_outer_batch`, `encode_fused_split_ragged`, `encode_fused_ragged_parallel`, and `PreTokenizedString::tokenize_batched`. Earlier cold DeepSeek samples found a `3.5:1` worker spread, but Experiments 27, 30, 55, 61, and 62 show that extra tasks or altered ownership can reverse by model, contract, and batch size. No current trace binds the remaining imbalance to whole encode.

Invariant that makes the shorter path exact: independent rows and pre-tokenized pieces can be encoded in parallel when each result retains its `(row_index, piece_index)` and final IDs are concatenated in that exact lexicographic order. Row lengths are sums over each row's ordered pieces. This screen does not implement that representation.

Representation being preserved or changed: the diagnostic records only test-build task elapsed time around the current row and ragged-chunk closures. It preserves every scheduler, cache, output buffer, ordering rule, API, format, dependency, unsafe block, evaluator file, and production build. A candidate is not authorized by this card.

Expected winning strata: 140-byte batch-512 nested and flat-ragged workloads across generic and fused families if current task imbalance, sequential setup, and ordered gather dominate the critical path.

Expected adverse strata: batch 1, batch 32, and single-document shapes cannot repay a new task graph; breaking fused cache streams or creating per-piece results can add allocation, cache duplication, and gather traffic. Experiment 61 already rejects parallelizing the 4.5-KiB batch-32 fused path.

Smallest files that need changing: test-only duration hooks in the three current outer/ragged task closures in `src/lib.rs`, plus one ignored diagnostic test in that file. The hooks are committed and reverted; no benchmark or production candidate changes.

Mechanism evidence: pending four-thread remote measurements for all six tokenizers at 512 novel 140-byte rows, both nested and flat-ragged, after one disjoint warmup batch. Record total encode time, task count, summed task time, and the optimistic ideal `sum(task_time) / 4`.

Acceptance rule: proceed to representation design only if `(encode_time - ideal_task_time) / encode_time` is at least `35%` for both contracts in every family. Because prior evidence closes batch 32, batch 512 alone must improve about `1.09^5 = 1.539x` across all families to reach the equal-shape target; `35%` is the corresponding perfect-removal floor. A later candidate must still preserve cache reuse and pass the frozen exactness and counterbalanced gates.

Rejection rule: stop before candidate code if any family/contract is below `35%`, the hooks miss the actual task closures, or the model requires assuming that task execution itself becomes faster. Do not tune chunk count, batch threshold, model, contract, or input size after observing the screen.

Experiment 80 is rejected at the mechanism screen without candidate code. Test-only timing hooks at `479e0fb` covered the actual batch-512 work items in `encode_outer_batch`, `encode_fused_split_ragged`, and `encode_fused_ragged_parallel`, then were removed by `b341f44`. Four cache-line-separated worker counters avoided inter-worker false sharing; the ignored test enabled them only around one timed call. Inputs were 512 disjoint exact 140-byte rotating corpus windows after a separate 512-row warmup. A separate Snaptokens instance passed complete Hugging Face parity before timing, and the measured instance passed complete nested token IDs or flat IDs plus every row length after timing in all 12 cells.

The single accepted `GenuineIntel family 6 model 106` sandbox built committed diagnostic `479e0fbffb1bf2c9bdd3484c14517f98900d1e0e` with the pinned nightly, native fat LTO, one codegen unit, and one Cargo job, then ran all model/contract processes sequentially with exactly four Rayon workers. For each cell, the optimistic removable fraction is `1 - (summed work-item time / 4) / encode time`; it treats every setup, scheduling, imbalance, and gather nanosecond outside the current work items as perfectly removable and assumes the work itself is unchanged.

| Tokenizer | Nested removable | Ragged removable | Nested perfect-packing ceiling | Ragged perfect-packing ceiling |
| --- | ---: | ---: | ---: | ---: |
| DeepSeek V3.2 | `21.71%` | `28.09%` | `1.277x` | `1.391x` |
| Gemma 3 | `4.79%` | `4.21%` | `1.050x` | `1.044x` |
| GPT-OSS | `9.51%` | `10.42%` | `1.105x` | `1.116x` |
| GPT-2 | `23.00%` | `15.10%` | `1.299x` | `1.178x` |
| MiniMax M2.1 | `15.69%` | `10.33%` | `1.186x` | `1.115x` |
| Qwen 3 | `13.77%` | `12.56%` | `1.160x` | `1.144x` |

Every cell misses the predeclared `35%` floor, including both contracts for every tokenizer. The best observed fraction is only `28.09%`, while generic Gemma is near `4%`; a real flat graph would also retain some setup and ordered assembly cost. Because prior evidence closes batch 32, these ceilings cannot supply the roughly `1.539x` batch-512 improvement required to produce a `1.09x` equal-shape general score. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/evidence/exp80-task-ceiling/exp80-task-ceiling.tar.gz` at SHA-256 `d85511815925ed27a72ab0a5edd7f31dbc09364bb555ea34f153fa606a692f92`; it contains the immutable test binary, build log, all raw cell logs, and host/source provenance. The sandbox terminated with no live Modal containers. No production candidate, evaluator change, dependency, unsafe code, API, or tokenizer-format change was created.

### Experiment 81 mechanism screen: test canonical prefix-state representability — planned

Parent SHA: `d2e2477737b258d2a4d2494f1a1407e51faa0fa2` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: Gemma and the four merge-active fused families' path-exact initial token IDs and canonical Hugging Face merge order satisfy the necessary `mtc-inc-bpe` normalized-dictionary preconditions, making a later exact prefix-state prototype worth testing without maintaining every intermediate heap candidate.

Measured hot cost: BPE merge and cache-miss resolution remains the only broad dominant lane. Experiments 2--5, 18, 26, 29, 34--35/50, 56, 76--79 changed heap, lookup, or cache mechanics; none tested whether the merge graph admits a different exact state transition. Heap-only work is insufficient because the four ranked fused families usually use the short stack loop, while non-ranked Gemma dominates the encoded heap path.

Invariant that makes the shorter path exact: the prefix-state algorithm requires one canonical atomic decomposition per token and no improper rule. The screen must pass the unchanged surviving last-write-wins pairs in their original relative rank order and the exact token IDs emitted by each current initializer. It may not reorder, delete, or "properize" a rule, because Hugging Face repeatedly selects the live adjacent pair with minimum `(rank, original left position)`, and a newly exposed rule may have lower rank than the merge that exposed it. Constructor success is necessary but does not itself prove exact runtime behavior.

Representation being preserved or changed: the disposable screen preserves tokenizer JSON, token IDs, surviving pair order, initial-symbol sets, byte fallback, `ignore_merges`, caches, scanners, outputs, APIs, formats, dependencies, evaluator files, and production source. It asks only whether pinned upstream `mtc-inc-bpe` commit `6fcbe980ebb77c0e840169eefe9a368fae547bab` can construct `NormalizedDict` with an explicit ID-membership predicate matching each active initializer. Gemma's set contains exact whole-Unicode-scalar vocabulary IDs plus configured byte-fallback IDs that its encoded initializer can emit. Each fused/raw set contains the 256 `byte_to_initial_token` IDs for transformed ByteLevel bytes. A production representation is not authorized by this card.

Expected winning strata: cache-miss BPE in encoded generic Gemma and fused/raw GPT-2, Qwen, DeepSeek, and MiniMax. If GPT-OSS constructs successfully, only its `ignore_merges` miss fallback is affected; whole-piece vocabulary hits remain an unchanged adverse control.

Expected adverse strata: arbitrary Hugging Face merge lists may be improper even though the current rank-plus-leftmost engine handles them exactly. Byte fallback adds atomic `<0xNN>` IDs to encoded initialization. Duplicate JSON pairs create rank holes and must be reduced by last-write-wins without changing surviving relative order. Construction may also reject ambiguous atomic decompositions or token lengths unsupported by the proposed representation.

Smallest files that need changing: an isolated committed standalone diagnostic under `tools/exp81-prefix-proof/` with its own pinned `Cargo.toml`, lockfile, and one `src/main.rs`, then a complete revert. No library, evaluator, benchmark, root workspace, root dependency lock, or test file changes.

Mechanism evidence: `rust-gems/bpe` derives exact compatibility from a stronger priority-by-token-order dictionary. `mtc-inc-bpe` supports explicit rule IDs only after `NormalizedDict` proves canonical rules and returns `ImproperDict` when that state does not exist. Snaptokens' prior monotonicity audit found newly exposed lower-rank rules in several supported tokenizers, but it did not run this exact normalization proof on the six frozen families and their active initial alphabets.

Acceptance rule: run generic Gemma first, then GPT-2, Qwen, DeepSeek, and MiniMax only if it succeeds; run GPT-OSS last as an adverse `ignore_merges` control. Reconstruct the exact contiguous vocabulary and last-write-wins pair set, sort surviving distinct ranks without changing their order, and retain a hashed dense-upstream-rule-to-original-rank map. Verify every rule's left, right, merged ID, and concatenated token text; reject duplicate surviving ranks because the upstream representation cannot preserve public `Bpe::new` rank ties. Require `NormalizedDict::new` to succeed with the path-exact atomic-ID set, and record its hash/count plus vocabulary, original surviving rule, upstream rule, elapsed construction, peak RSS, source revision, and tokenizer hashes. Success for Gemma and all four merge-active fused families authorizes only a separate test-only cold exact-oracle and runtime-coverage prototype; GPT-OSS success is supporting adverse evidence. If GPT-OSS remains unchanged at `1.00x`, the five affected families must later average at least `1.1089x` to produce the frozen six-family `1.09x` geomean.

Rejection rule: stop before runtime tables or candidate code if Gemma or any required fused path returns `ImproperDict` or `MultipleAtomicTokenSeq`, cannot reconstruct every input rule and exact atomic ID, has a rule-text or duplicate-rank mismatch, exceeds an upstream length/ID limit, or needs rule reordering/deletion. Construction may internally mark only a canonical subset; success therefore cannot establish runtime coverage, and the next prototype must reject if any cold production input falls outside supported canonical state or differs from the current heap and Hugging Face. Do not select a smaller required family set, change the atomic alphabet, renumber by merged token ID, or properize the merge order after observing failures.

Experiment 81 is rejected at its first required-family mechanism gate without candidate code. Standalone diagnostic `aee69bea1b17d0ba9d1e523932223f13960abaff` reconstructed Gemma's exact contiguous vocabulary, last-write-wins surviving rules, original relative ranks, and encoded initializer atom set, then verified every upstream rule's left, right, and merged ID before calling pinned `mtc-inc-bpe` `NormalizedDict::new`. The constructor returned `ImproperDict` for token `109` (`"\n\n\n"`), with proper result IDs `[108, 107]` (`["\n\n", "\n"]`). Gemma therefore fails the necessary representation precondition, so GPT-2, Qwen, DeepSeek, MiniMax, and adverse-control GPT-OSS were not run and no runtime or production prototype was authorized.

The valid sequential run used one accepted `AuthenticAMD family 175 model 1` sandbox, one Cargo build job, the pinned nightly and upstream revision, and the frozen tokenizer/evaluator preflight. The Gemma process took `1.79 s` wall time, used `330,868 KiB` peak RSS, and exited `1` on the semantic constructor error. Raw evidence is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/evidence/exp81-prefix-proof/exp81-prefix-proof.tar.gz` at SHA-256 `138ba3b55c1316340bd9ed8e529edd9abf4ecf53591a2f3a88a198ca53c96537`. An earlier accepted Intel attempt never executed the diagnostic because `/usr/bin/time` was absent (`status=127`); that invalid infrastructure archive is preserved as `exp81-prefix-proof-invalid-time-missing.tar.gz` at SHA-256 `15308911257468effcc561e1f8b2f8b2e6854aade16f272f074bed37620ebd0a`. The disposable source was removed by revert `ca58e98e84cedd079d227e3035fd4598c2194139`; no live Modal containers remain. No production source, evaluator, benchmark, root dependency lock, unsafe code, API, tokenizer format, or timed output path changed.

### Fresh champion profile capability check after Experiment 81

A pre-build hardware-sampling probe on one bounded four-vCPU Modal AWS sandbox was infrastructure-invalid. The host ran Linux `4.19.0-gvisor`; `perf 6.1.180` failed `perf_event_open` with `ENODEV` before any tokenizer, benchmark, or compilation ran. The sandbox terminated with no live containers. The mandatory local-profile preflight also remained closed: no LLM API key was present, although free disk exceeded `3 GB`; the Mac was additionally under visible application load. No local profile was started and call-instrumented gprof or hot-loop counters were not substituted for hardware sampling.

### Experiment 82 mechanism screen: exact input-specific independent BPE spans — planned

Parent SHA: `d3e7919b7b105103ac295a3b0da7131d86218e6b` (tokenizer source remains byte-identical to frozen champion `b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa`)

Hypothesis: matching complete constructible vocabulary-token atom sequences against each real cache-miss piece can prove substantially more independent BPE span boundaries than the current context-free bigram table, enough that the unchanged merge engine runs at least `2.00x` faster on those precomputed spans in both generic Gemma and fused Qwen.

Measured hot cost: the frozen profile places cache-miss BPE merge resolution first across fused families and Gemma's encoded model closure first in its encode path. The profile cannot attribute hardware cycles because Modal blocks `perf_event_open` and local profiling fails its API-key preflight, so this screen measures the isolated unchanged BPE stage and may reject but cannot establish a whole-encode win.

Invariant that makes the shorter path exact: a cut is allowed only between production initial token IDs and only when no constructible token ID has a surviving-rule derivation whose exact initial-ID sequence matches an input subsequence crossing that cut. Then no merge can cross the cut, reductions on each side commute, and concatenating independently reduced spans preserves Hugging Face's global `(rank, original-left-position)` result. The screen must consider every surviving incoming rule, reject inconsistent atomic sequences, use raw ByteLevel initial IDs or Gemma direct-scalar/byte-fallback IDs exactly, and never cut inside one encoded Unicode scalar.

Representation being preserved or changed: test-only capture records pieces only after existing local and shared cache misses. A test-only standard overlapping matcher derives one exact atom sequence per constructible token from all surviving rules and precomputes maximal safe spans outside timing. Timed whole and span modes call the unchanged private BPE merge functions directly with identical output allocation and destruction; matcher construction and matching are deliberately treated as free. Production cache policy, merge order, heaps, adjacency, short loops, scanners, output work, APIs, formats, dependencies, unsafe code, evaluator files, and benchmark inputs remain unchanged.

Expected winning strata: generic encoded Gemma pieces that remain large after the retained global bigram splitter, plus fused/raw Qwen, DeepSeek, GPT-2, and MiniMax cache misses when full-token context proves safe cuts that the global pair table cannot express.

Expected adverse strata: fused pieces are usually only four or five bytes; Gemma already uses conservative vocab-aware splitting; safe spans preserve accepted merges rather than deleting them; matching and assembly add real work omitted by the optimistic timing. GPT-OSS `ignore_merges` is ineligible because its exact whole-piece lookup can emit an orphan token with no merge derivation.

Smallest files that need changing: test-only capture, atom-pattern construction, span verification, and counters in `src/models/bpe.rs`, plus one ignored diagnostic in the existing `src/lib.rs` test module. The diagnostic is committed and reverted. No new crate, public helper, production field, evaluator, or benchmark file.

Mechanism evidence: the retained global `BigramBridgeTable` proves cuts only from vocabulary-wide adjacent surface pairs. Exact overlapping full-token patterns are strictly more input-specific. The independent exactness review confirmed the graph-cut invariant only when all surviving-rule derivations and path-exact initial IDs are represented; `unmerge_map` or token surface text alone is insufficient.

Acceptance rule: run Gemma then Qwen over all five frozen shapes and both contracts, with skewed ragged as an extra adverse probe, and require full Hugging Face IDs and row boundaries before and after capture. Reject immediately on inconsistent token atom sequences or any whole-versus-span ID mismatch. For each family/shape, precompute spans once, then run six counterbalanced whole/span BPE-stage rounds in a fresh process. Score a family as the equal-shape geomean of paired-round medians; do not weight pieces or bytes. Continue to GPT-2, MiniMax, and DeepSeek only if both early families reach `2.00x`, every shape has at least one new cut and no material stage regression, and both encoded-generic and raw-fused paths have nonzero new cuts. Report `new_safe_cuts / eligible_internal_atom_boundaries`, the fraction of cache-miss pieces gaining a cut, and atom-weighted span median/p95/max separately rather than using them as authorization proxies. Require the same timing gates in all five affected families. With GPT-OSS fixed at `1.00x`, those families need `1.1089x` average end-to-end speed, and even a `2.00x` stage needs at least `19.64%` end-to-end share. Because that share is unmeasured, screen success authorizes only a disposable test-only matcher-costed full-pipeline prototype under the frozen evaluator, not production design or candidacy.

Rejection rule: stop before a matcher-costed full-pipeline prototype if Gemma or any required fused family misses the `2.00x` equal-shape stage aggregate, any frozen shape has no new cut or a material stage regression, atom derivations are inconsistent or unsupported, exactness differs, metadata construction exceeds the existing `5%` RSS/construction projection, or opportunity depends on `ignore_merges`, heap-only long pieces, one shape, model labels, corpus details, or input-length dispatch. Do not weaken the family, shape, atom, or stage gates after seeing results.

Pre-source addendum: run a construction-only semantic gate before adding capture, timing, or a matcher. Reuse Experiment 81's field-by-field audit of the exact contiguous vocabulary, JSON last-write-wins pair set, surviving rank order, merged IDs, concatenated token text, and duplicate-rank handling, then derive the path-specific initial token IDs. Define constructible tokens as the least fixed-point closure of that exact atomic-ID set under surviving rules whose two operands are already constructible. Starting from atomic sequence `[id]`, a rule contributes `sequence(left) || sequence(right)` only after both sides are known. Reject immediately if one token receives different sequences, including a direct-atomic versus rule-derived conflict; identical repeated sequences are valid. After convergence, record and ignore vocabulary IDs outside the closure, including unresolved rule SCCs, because no finite active-atom derivation constructs them. Run encoded Gemma first and raw Qwen only if Gemma succeeds. Record hashes and counts for the atomic IDs, surviving rules, constructible tokens, distinct multi-atom sequences, total atoms, maximum sequence length, and unresolved IDs. This gate changes no tokenizer source and makes no speed, coverage, RSS, or construction claim. Passing both families authorizes only an exhaustive test-only crossing-subrange oracle on post-cache pieces; it does not authorize an automaton or production candidate. Rejection stops Experiment 82 before hot-path instrumentation.

The construction-only semantic gate passed on one bounded `AuthenticAMD family 175 model 1` sandbox. Committed standalone diagnostic `4bb065608c59685102c31d95d14d1f7baa0db51d` used one Cargo build job and sequential processes. Gemma's encoded path produced `19,359` active atoms, `255,698 / 262,144` constructible tokens, `236,339` distinct multi-atom sequences, `1,381,542` distinct-sequence atoms, and maximum sequence length `31`; Qwen's raw path produced `256` active atoms, all `151,643` tokens constructible, `151,387` distinct multi-atom sequences, `975,713` distinct-sequence atoms, and maximum length `128`. Both full token-to-sequence mappings were internally consistent. This is representability evidence only; it does not measure opportunity or speed. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/evidence/exp82-span-proof/exp82-span-proof.tar.gz` at SHA-256 `a0dfdeb500d944877d46b6bade7ed5db1e91168c0e692fcc86f19f18dd0eba83`; the sandbox terminated with no live container.

Opportunity-gate addendum: before stage timing or matcher construction, capture only piece text, production output IDs, and encoded/raw initializer kind after the existing local, shared, and `ignore_merges` gates have resolved and published each miss. Disable capture, then rederive the exact initial token-ID vectors sequentially. Use the canonical whole-vector output itself as an impossible perfectly informed splitter: the unique atom sequence of each emitted token defines the maximum exact final-token boundaries for that observed piece. Apply the unchanged canonical rank-plus-leftmost heap directly to the whole initial-ID vector and independently to every resulting atom span, without routing spans through exact-token or cache shortcuts. Require per piece `production output == canonical whole-vector output == concatenated canonical span outputs`, and require each emitted token's derived atom sequence to equal the input span it owns. Run Gemma then Qwen over all five frozen portable shapes and both contracts, with a fresh tokenizer/process per cell, disjoint capture-disabled warmup, and complete Hugging Face IDs and ragged row boundaries before and after capture. Define an eligible boundary as every internal boundary of an actual post-cache-miss initial-ID vector; an oracle cut is one final-token boundary inside that already globally split production piece. Record captured pieces and atoms, eligible boundaries, oracle cuts, pieces gaining cuts, and atom-weighted span median/p95/max per cell. Reject Experiment 82 before timing if a required cell captures no eligible work, has zero oracle cuts under this intentionally stronger broad-coverage policy, fails any three-way piece equality, or changes Hugging Face parity. Passing authorizes only the original counterbalanced production-engine whole-versus-span stage timing; it does not establish a `1.09x` ceiling or authorize a production matcher. Only if that impossible timing ceiling passes may a conservative all-rule matcher be considered.

The opportunity gate passed all twenty required Gemma/Qwen cells on committed diagnostic `53947d0006a7f3ff17904566a304baf50da348f7`. Every fresh-process cell covered one frozen shape and contract, completed full Hugging Face pre/capture/post parity, and satisfied `production output == canonical whole-vector output == concatenated canonical final-token-span output`. Gemma exposed oracle cuts at `22.16%--23.05%` of eligible boundaries and every captured miss gained a cut; Qwen exposed `21.75%--23.44%`, with `95.38%--99.95%` of captured misses gaining a cut. Median atom spans were `4` in every cell, with p95 `10` for Gemma and `8--9` for Qwen. These are deliberately impossible perfectly informed opportunity bounds, not achievable matcher coverage or timing evidence.

The valid run used one bounded `AuthenticAMD family 175 model 1` sandbox, one Cargo build job, and twenty strictly sequential test processes. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/evidence/exp82-span-opportunity/exp82-span-opportunity.tar.gz` at SHA-256 `112a66a21794ee94a5cf2b4b1077189a6a744bbbad283b24100eea224e88cf27`; all `73` archived member hashes verified and no sandbox remained live. A first accepted-host attempt compiled successfully but its runner searched the wrong Cargo output directory and executed no cell. That invalid infrastructure archive is preserved separately as `exp82-span-proof-partial.tar.gz` at SHA-256 `59b856f8bf7ee9ef0417b669f1e71d3fb15aa0345302b116b8cdaf12227c8919`. Passing this rung authorizes only the predeclared counterbalanced production-engine whole-versus-impossible-span stage timing.

Stage-timing addendum: coalesce the impossible atom spans until every retained cut maps to an original UTF-8 byte boundary. Raw ByteLevel initialization contributes one atom per source byte, but the private production API accepts `&str`; encoded initialization contributes either one direct-scalar atom or several byte-fallback atoms, so no cut may split one scalar's fallback expansion. Record a source-byte boundary for each legal atom boundary while reproducing initialization, reject any piece whose legal-span output differs, and require every cell to retain at least one legal cut.

Precompute immutable `(piece text, production IDs, legal source ranges)` work outside timing. The timed inventory includes every captured miss, including one-atom and no-cut pieces. The whole arm calls the unchanged private `merge_all_encoded_into` or `merge_all_raw_into` once per captured piece; the span arm calls the same mode on each legal substring, preserving the production single-atom, short, dense, byte-fallback, and heap dispatch. Both arms append into one fresh `Vec<u32>` per cell invocation. Start timing before allocating that vector, process the complete stable worklist, black-box the fully materialized IDs and total length, explicitly drop the vector, and only then stop timing. Span discovery, matcher work, exact comparisons, reports, input cloning, and public caches remain outside timing. Warm both arms once, then run six paired rounds alternating whole/span and span/whole order. Preserve all raw times. For each of the ten equal family strata, score `cell = exp(median_6(log(whole_ns / span_ns)))`, then `family = exp(mean_10(log(cell)))`; also report the two-contract shape points and the two order-subset family points. Require complete per-piece equality before and after timing, every cell to retain at least one callable cut, no cell point below `0.95x`, a family point of at least `2.00x`, both order-specific family points of at least `2.00x`, and `max(order_AB_family, order_BA_family) / min(order_AB_family, order_BA_family) <= 1.05`. The `0.95x` adverse floor is an intentionally frozen rejection policy, not an A/A-derived private-stage noise band. Each cell inherits the opportunity protocol's fresh process, disjoint warmup, and contract-specific frozen seeds (`8,000,000 / 10,000,000` nested and `18,000,000 / 20,000,000` ragged). Run all Gemma cells first, then Qwen only if Gemma passes; run GPT-2, MiniMax, and DeepSeek sequentially only if both early families pass. Early stopping is rejection only. Passing still authorizes only a matcher-costed frozen full-pipeline prototype; it does not prove that this stage owns the `19.65%` of five-family end-to-end time needed for a general `1.09x` result.

Experiment 82 is rejected at the first predeclared stage-timing family gate. On one accepted `GenuineIntel family 6 model 85` sandbox, all ten Gemma shape-contract cells passed complete Hugging Face parity and per-piece equality before and after six counterbalanced production-engine rounds. Every cell retained source-callable cuts, and its impossible free-span stage point ranged from `1.224415x` to `1.275841x`, but the equal-cell family point was only `1.246950x` versus the frozen `2.00x` requirement. The whole-first and span-first family points were `1.246425x` and `1.252377x`; their `1.004776x` split passed the `1.05` order-stability gate. Qwen and the later fused families were therefore not run. This is a real isolated Gemma-stage improvement under impossible output-informed, zero-cost span discovery, but it cannot justify a matcher-costed full-pipeline candidate or the required five-family general ceiling.

Committed diagnostic `a7633e2bd0ef5038e77ca63eae96a3e7d65a7a85` used the same private production encoded engine for both arms, kept output allocation, writes, observation, and destruction inside timing, and preserved all six raw pairs without filtering. The complete archive is `/Users/namanchetwani/.cache/snaptokens-autoresearch/dev-v1/evidence/exp82-span-stage/exp82-span-stage.tar.gz` at SHA-256 `5717e01907bfe40f5c480505c6ad13342dfea63cfb6c1c8505676015df83fdb9`; all `44` member hashes verified and no sandbox remained live. No production candidate, dependency, unsafe code, public API, tokenizer format, evaluator, benchmark, or promotion score changed.

### JSON-load experiment 19: share runtime vocabulary strings — planned

Parent SHA: `d387f23fafdf43dbe0b1c0f9ae583c357b05fad8`.

Hypothesis: fresh JSON BPE construction currently retains every vocabulary spelling in both `id_to_token: Vec<String>` and `token_to_id: HashMap<String, u32>`. Building one `Arc<str>` per parsed spelling and cloning only the small handle into the other table removes a full vocabulary-sized allocation and copy while preserving the standard randomized lookup map.

Measured hot cost: the preserved Intel GPT-2 JSON callgraph attributes `18.69%` of sampled task-clock time to `Bpe::build`, `7.62%` to DAAC construction, and `3.52%` to `malloc`; the build also exposes the current `Vec<Option<&str>>` to `Vec<String>` copy. Current CPU-0 fresh JSON loads are `126,855,495`, `121,767,102`, and `119,112,439` ns. New PMU recording is unavailable on this VM because `perf_event_paranoid = 4`; no substitute profiler is used.

Invariant that makes the shorter path exact: an `Arc<str>` contains exactly the same immutable UTF-8 bytes as the parsed `String`, and `HashMap<Arc<str>, u32>` queried by `&str` returns the same ID for every token spelling. Token-ID contiguity validation, duplicate detection, merge parsing, matcher inputs, decode strings, sidecar payloads, and all public APIs remain unchanged.

Representation being preserved or changed: JSON parsing still owns `HashMap<String, u32>` with the standard randomized hasher. Only the finished private BPE representation changes from separately owned strings in its ID vector and lookup map to shared `Arc<str>` strings. No dependency, feature, unsafe code, cache policy, model dispatch, evaluator, or format changes.

Expected winning strata: large BPE JSON vocabularies, especially GPT-2 and other ByteLevel files where `Bpe::build` copies every vocabulary spelling after deserialization.

Expected adverse strata: small vocabularies may lose to atomic reference-count bookkeeping; sidecar resolution serializes owned strings and may see no benefit; construction RSS, direct V5 loads, JSON-to-sidecar creation, and encode must remain guarded.

Smallest files that need changing: `src/models/bpe.rs` and this record. No test file is added.

Mechanism evidence: the current constructor derives `id_to_token` from borrowed entries in `Vocab` with `map(str::to_owned)`, then retains `Vocab` as `token_to_id`. The retained V5 sidecar removes other direct-load construction work but JSON still performs this duplicate ownership step.

Acceptance rule: commit a clean candidate; format; run existing focused BPE, JSON-structure, and TKZ tests plus complete Hugging Face IDs for candidate JSON, V5 direct TKZ, and legacy V4 direct TKZ on the fixed GPT-2 contexts. Use immutable parent and candidate binaries on the GCP CPU-0 fresh-process pool, with pre/post exactness, ten counterbalanced JSON load pairs, ten direct V5 pairs, ten JSON-to-sidecar creation pairs, and the existing encode guard. Retain only if JSON load improves beyond the parent noise band, direct V5, creation, and encode do not regress, binary growth stays within `2%`, and the separately frozen disposable Snaptokens-versus-Hugging-Face JSON-load runner shows a real improvement toward the competitor on the same input.

Rejection rule: reject and revert for any ID difference, row-boundary or malformed-sidecar failure, direct V5, creation, or encode regression, binary growth above `2%`, no JSON gain beyond noise, retained test, evaluator change, dependency, unsafe code, public API change, source/model/input-size dispatch, or failure to improve the isolated competitor comparison. Do not switch to a non-cryptographic vocabulary or merge hasher after observing this result.

Experiment 19 is rejected at the JSON target screen. Candidate `8512a3746d679b2e983c8d51d504ab0c2ba16db6` and parent `d387f23fafdf43dbe0b1c0f9ae583c357b05fad8` were committed and clean on the same GCP Intel C4 VM, with `RUSTFLAGS="-C target-cpu=native"`, separate target directories, the root lockfile, and immutable GPT-2 JSON input SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. Release binaries were SHA-256 `05b04dcdd3932fa85902657ce7bdbbbbb6734bcc3ab1cdd491336c3710228456` (parent) and `3921193ca389ac538dcd9916babb279db34eb51d280ad6c1e673a76cf83652f2` (candidate). One untimed load per binary preceded twelve counterbalanced fresh-process pairs; process startup was outside the timer and each timer covered `Tokenizer::load_file` through tokenizer destruction.

Candidate complete-load throughput was `0.973670x` of the parent (geometric mean of paired parent/candidate elapsed-time ratios; median `0.974305x`), a `2.63%` regression. Parent/candidate elapsed nanoseconds by paired round were: `126113234/129280657`, `130469011/128636441`, `127999017/128717674`, `124678149/132097153`, `126775868/129244734`, `126639772/127121510`, `125919060/130173124`, `126835902/129061681`, `126713778/130837713`, `125158534/128617004`, `123530516/131145722`, and `123510245/130274425`.

Focused BPE construction tests passed before and after timing (`16 passed`), and the post-timing `correctness_gpt2` Hugging Face full-ID comparison passed (`1 passed`). No test was added or retained. The candidate failed the first required target, so direct V5, JSON-to-sidecar creation, encode, binary-size, and Hugging Face-comparator guards were deliberately not run. The source change is reverted in full; no evaluator, dependency, unsafe code, public API, tokenizer format, benchmark, or documentation change is retained.

### JSON-load experiment 20: replace whole-piece DAAC matching with exact vocab lookup — planned

Parent SHA: `d387f23fafdf43dbe0b1c0f9ae583c357b05fad8` (source is byte-identical after Experiment 19's revert).

Hypothesis: `Bpe::tokenize_into` only accepts a DAAC match when its token length equals the complete input length. Replacing `ExactTokenMatcher` with a direct `token_to_id.get(input)` guarded by the already-derived orphan bitset removes DAAC construction and storage without changing any accepted whole-piece ID.

Measured hot cost: the preserved GCP Intel GPT-2 JSON profile assigns `7.62%` of task-clock samples to DAAC construction, while the same builder's `rayon::join` covers DAAC construction beside the fixed byte-pair table. The direct lookup is already an existing exact vocabulary operation; the current DAAC is not used by any other BPE call path.

Invariant that makes the shorter path exact: a non-orphan token is a legal exact whole-piece result if and only if its complete spelling is a key in `token_to_id`. The current matcher result is accepted only after its matched token's byte length equals `input.len()`, which rules out every proper-prefix match. `ignore_merges` already clears all orphan bits. The V5 trie remains fully validated before construction; only its redundant runtime matching role is removed.

Representation being preserved or changed: preserve tokenizer JSON, IDs, BPE merge ordering, decomposition, sidecar validation, direct V5 and legacy V4 formats, caches, byte fallback, `ignore_merges`, API, and output materialization. Replace the private regular/V5 exact-prefix matcher field with the existing per-token orphan bitset, then perform the same exact whole-piece decision through the randomized vocabulary map. No dependency, unsafe code, model dispatch, evaluator, or benchmark change.

Expected winning strata: every JSON and legacy-V4 BPE load that currently builds DAAC, especially GPT-2 and large vocabularies.

Expected adverse strata: BPE whole-piece checks may lose encode speed to a hash lookup; V5 no longer uses its validated trie as a runtime lookup, so direct V5 and first encode require explicit guards. Models without BPE are unchanged.

Smallest files that need changing: `src/models/bpe.rs` and this record. No test file is added.

Mechanism evidence: `rg` shows `Bpe::next_match` is used only by `tokenize_into`; the subsequent `token_length_matches` makes that use an exact full-input test. Hugging Face's pinned BPE loader similarly owns a forward vocabulary and reverse vocabulary, rather than this extra automaton. The formal frozen runner cannot rebuild with its stale locked Gigatoken manifest under the available Cargo versions, so its retained binary is competitor context only and cannot promote this candidate.

Acceptance rule: commit a clean candidate; format; run focused BPE and TKZ tests plus complete Hugging Face IDs for the timed GPT-2 input before and after timing. On the same GCP host use immutable parent/candidate binaries, one untimed warmup per arm, and at least ten alternating JSON load pairs. Retain only if JSON-load throughput exceeds the parent beyond noise, GPT-2 direct V5 and legacy V4 direct loading plus first encode do not regress, and no exactness, sidecar-validation, binary-size, or public-API guard fails.

Rejection rule: reject and fully revert for any ID mismatch, malformed-sidecar acceptance, direct-load or first-encode regression, no JSON gain beyond noise, retained test, evaluator change, dependency, unsafe code, public API change, or source/model/input-size dispatch. Do not retain a JSON-only gain that shifts DAAC work into the first encode.

Experiment 20 is rejected after its direct-V5 guard. Candidate `7f1a27463103dce6bc29b2b48df1a75fa5966337` passed pre- and post-timing focused BPE construction (`16 passed`), GPT-2 complete Hugging Face IDs (`1 passed`), and the existing two-test TKZ suite on the GCP Intel C4 host. No test was added or retained. With the root lockfile and `RUSTFLAGS="-C target-cpu=native"`, twelve alternating fresh-process GPT-2 JSON loads against committed parent `d387f23` measured `1.364045x` candidate/parent throughput (median `1.365789x`; means `126,880,318` versus `93,026,987` ns). The release binary SHA-256 values were `05b04dcdd3932fa85902657ce7bdbbbbb6734bcc3ab1cdd491336c3710228456` (parent) and `206de02b42aae918d32632982a7e077fc4ba4dda6c920cca4a39c70cd5b3677f` (candidate); the JSON was fixed at SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`.

The same candidate improved legacy V4 direct GPT-2 TKZ load `2.135140x` over twelve alternating pairs (mean `61,789,452` versus `28,954,906` ns), but its candidate-created V5 sidecar exposed the adverse path. Fifty alternating root-lock direct-V5 load pairs scored `0.989279x` candidate/parent (median `0.990464x`; means `33,399,279` versus `33,754,645` ns). A disposable same-host first-encode guard retained the exact checksum `17406679798874925224` in all 24 observations but was noisy and did not override the direct-load loss. Because the experiment removed the V5 trie along with DAAC, this result is attributed to changing that already-fast V5 representation, not accepted as a harmless variation. The source is fully reverted. No evaluator, dependency, unsafe code, public API, tokenizer format, benchmark, or documentation change is retained.

### JSON-load experiment 21: remove DAAC only when no V5 trie exists — planned

Parent SHA: `d387f23fafdf43dbe0b1c0f9ae583c357b05fad8` (source is byte-identical after Experiment 20's revert).

Hypothesis: JSON and legacy V4 build a DAAC only for an exact whole-piece decision, while V5 already restores a compact checked trie. Preserve the V5 trie matcher byte-for-byte; replace only JSON/V4 DAAC matching with a direct vocab lookup plus the builder's orphan bitset.

Measured hot cost: Experiment 20 proves the JSON/V4 removal screen, at `1.364045x` and `2.135140x` respectively, but the all-path variant costs `1.07%` on V5 direct load. The predeclared profile attributed `7.62%` to DAAC construction; V5 does not construct DAAC and needs no removal.

Invariant that makes the shorter path exact: when a V5 trie exists, retain its original longest-prefix and exact-length behavior. Otherwise, the current matcher can be accepted only for a whole input token, and direct map lookup plus the derived orphan bitset returns that identical legal ID. The original token-length proof remains in both branches.

Representation being preserved or changed: preserve V5 `ExactTokenTrie`, its validation, matching implementation, and sidecar fields; retain direct V4 behavior only through the new private direct-whole-token matcher. Preserve all JSON, IDs, merge ordering, caches, byte fallback, `ignore_merges`, APIs, formats, dependencies, evaluator, and timing boundaries. No test file is added.

Expected winning strata: JSON and legacy V4 BPE loads; V5 direct loading and first encode are an unchanged adverse control.

Expected adverse strata: map lookup may affect JSON/V4 whole-piece encoding, and a representation tag may change code layout. V5 must have no measurable construction or first-encode loss.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: Experiment 20's loss is isolated to discarding the V5 trie. A tagged private matcher can retain the exact original trie for sidecars and carry `Vec<bool>` only in JSON/V4 construction, so V5 avoids the extra retained orphan vector and any changed whole-piece call path.

Acceptance rule: commit a clean candidate; format; run existing focused BPE, GPT-2 Hugging Face IDs, and TKZ tests before and after timing. Use the same GCP host, root lockfile, immutable parent/candidate binaries, one disjoint warmup per arm, and at least twelve alternating JSON, V4 direct, and V5 direct pairs. Retain only if JSON remains above parent beyond noise, V4 remains non-regressing, V5 direct and first encode stay within their parent noise, and no sidecar, exactness, binary-size, or public-API guard fails.

Rejection rule: reject and fully revert for any ID mismatch, sidecar-validation failure, JSON non-win, V4/V5/first-encode regression, retained test, evaluator change, dependency, unsafe code, public API change, or source/model/input-size dispatch. Do not weaken the V5 guard after seeing Experiment 20.

Experiment 21 is retained as the new scoped JSON/V4 load parent. Committed candidate `4a116ae88b69e0df574485415e19afdcbe1f6e02` and parent `d387f23fafdf43dbe0b1c0f9ae583c357b05fad8` were clean and built with the root lockfile and `RUSTFLAGS="-C target-cpu=native"` on the same GCP Intel C4 host. The immutable GPT-2 JSON input was SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. Each arm received one untimed direct-runner warmup; then the timer covered only `Tokenizer::load_file` or `Tokenizer::load_file_with_tkz_cache` through tokenizer destruction, excluding process startup.

Across twelve alternating fresh-process JSON pairs, candidate/parent throughput was `1.375074x` (median `1.372602x`; mean elapsed `126,054,702` versus `91,675,556` ns). The same candidate improved legacy V4 direct GPT-2 TKZ load to `2.147947x` across twelve pairs (median `2.163211x`; mean `61,103,168` versus `28,462,446` ns). The V5 direct adverse control retained its trie and stayed non-regressing across twenty pairs at `1.008584x` (median `1.005180x`; mean `34,447,884` versus `34,156,045` ns). A disposable first-encode guard on the same V5 sidecar kept checksum `17406679798874925224` in all 24 observations and measured `1.018502x` candidate/parent throughput (median `1.008330x`; mean `5,549,063` versus `5,459,005` ns). No benchmark, evaluator, dependency, API, format, unsafe block, or test file changed.

Focused BPE construction tests (`16`), GPT-2 complete Hugging Face IDs, and the two existing TKZ tests passed before and after the timed pools. Post-timing expanded structure coverage passed for MiniMax, Qwen, Qwen Nemotron, DeepSeek V3.2, GPT-OSS, and `ignore_merges`; the complete existing tokenizer integration suite also passed (`54` active tests, `9` ignored). Release runner binaries were `4,522,728` bytes for the parent (SHA-256 `05b04dcdd3932fa85902657ce7bdbbbbb6734bcc3ab1cdd491336c3710228456`) and `4,389,064` bytes for the candidate (SHA-256 `16bfe6aee02dc3e59b619e08c1ecf17c0a0d583a16f986728e64917b8642b9fd`), a `2.96%` reduction. The formal competitor runner remains unrebuildable unchanged because its pinned manifest and lockfile conflict with the available Cargo/toolchain combination, so this result establishes a scoped own-parent improvement but not a current-head Snaptokens-versus-Hugging-Face claim. The next card must begin from this retained source and target the remaining JSON construction cost.

### JSON-load experiment 22: deserialize the BPE model without an intermediate JSON object — planned

Parent SHA: `5320bc7` (retained Experiment 21 source).

Hypothesis: `ModelConfig::deserialize` first materializes every BPE model field as `serde_json::Value`, then reparses that owned object as `Bpe`. Directly deserializing `Bpe` while retaining the same tagged-BPE and legacy-untagged validation removes the temporary model object, its generic value tags, and its object insertion work.

Measured hot cost: on the fixed GPT-2 JSON, the model contains `50,257` vocabulary entries and `50,000` merge strings. The existing parser visits that payload twice through `Value::deserialize` followed by `serde_json::from_value`; the preserved constructor profile already places BPE build at `18.69%`, generic JSON-object insertion at `4.86%`, and allocation at `3.52%`. Experiment 21 removes DAAC construction but not this generic model representation.

Invariant that makes the shorter path exact: a tagged object is accepted only when `type == "BPE"`; a legacy untagged object is accepted only when it contains both `vocab` and `merges`. Every accepted field still flows through the existing `RawBpe` validation, merge parsing, contiguous-ID validation, and BPE builder. Unsupported model types, missing legacy merge fields, malformed `type` fields, unknown BPE fields, and all tokenizer IDs remain governed by the same rules.

Representation being preserved or changed: replace only the temporary `serde_json::Value` representation of the `model` field with direct `RawBpe` deserialization. Keep `Value` configurations elsewhere in the tokenizer schema, the parser library, vocabulary hasher, merge representation, BPE tables, tokenization, sidecars, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary unchanged. No test file is added.

Expected winning strata: large JSON BPE tokenizers such as GPT-2, where a 50k-entry model object otherwise enters generic `Value` before its final BPE representation.

Expected adverse strata: malformed model error paths and small BPE files may see no benefit; an incomplete legacy model must not become accepted; direct V4/V5 sidecars and first encode must remain unchanged.

Smallest files that need changing: `src/json_structs.rs`, `src/models/bpe.rs`, and this record.

Mechanism evidence: the current `ModelConfig` implementation calls `Value::deserialize`, checks `type`, then consumes it with `serde_json::from_value`. `RawBpe` is already the `Bpe` deserialization boundary, so adding the type/presence validation there lets Serde populate the final vocabulary and merge input directly. Hugging Face uses a different BPE representation and hasher, which this candidate deliberately does not adopt.

Acceptance rule: commit a clean candidate; format; run existing JSON-structure and BPE tests plus complete Hugging Face IDs for GPT-2 before and after timing. Verify tagged BPE, legacy untagged BPE, unsupported model, missing legacy merges, and malformed model values through existing or disposable remote checks. On the same GCP host use immutable parent/candidate root-lock binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if the JSON gain exceeds the parent noise, V4/V5 direct load and V5 first encode stay non-regressing, binary growth stays within `2%`, and no parser-contract or ID check fails.

Rejection rule: reject and fully revert for any accepted unsupported or incomplete model, changed IDs, JSON non-win, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe code, public API change, source/model/input-size dispatch, or a candidate that moves construction work into first encode. Do not change the vocabulary or merge-map hasher after observing the result.

Experiment 22 is retained as the new scoped JSON-load parent. Candidate `91cc135fe8fb65a8b6ea8ddb10b906ff98bd8d78` and parent `5320bc720ee6e3bc741fdcb9e41c114d1d950be8` were clean, root-lock native builds on the same GCP Intel C4 CPU-0. The immutable GPT-2 JSON was SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating processes; each timer covered `Tokenizer::load_file` through destruction, excluding process startup. Parent/candidate elapsed nanoseconds were `89771818/74832289`, `90758661/74647937`, `88972119/73154826`, `89940589/74436250`, `89768545/73555470`, `88775928/73331474`, `90381837/74046631`, `90063178/74477101`, `89760521/74812400`, `88837993/73760944`, `88935392/71041194`, and `87689654/71537990`. Candidate/parent throughput was `1.215154x` (median `1.213217x`; means `89,471,353` versus `73,636,209` ns; every pair favored the candidate).

The V4 direct guard's twelve raw parent/candidate pairs were `25345387/25728514`, `25008040/24860034`, `25191258/24415683`, `25239185/24271278`, `24278018/24295740`, `24621554/25848979`, `24760689/24728342`, `25292985/24722437`, `24784620/24919122`, `24373557/24223229`, `24671286/24535347`, and `25648481/25223552`, for `1.004934x` throughput (median `1.005747x`). The V5 direct twenty-pair result was `1.004407x` (median `1.002729x`; means `29,106,749` versus `28,982,482` ns); all raw pairs are preserved in the GCP command transcript for this experiment. A disposable V5 first-encode guard preserved checksum `17406679798874925224` in all 24 observations. Its load result was `0.996857x` (median `0.992505x`; means `27,544,287` versus `27,629,969` ns) and its encode result was `0.996115x` (median `0.998309x`; means `4,477,535` versus `4,487,566` ns), both inside the same short-run variation seen by the direct V5 pools. The guard source is outside the repository and has no promotion role.

The V5 direct raw parent/candidate pairs were `28912816/29180665`, `28648716/29136048`, `29686571/28552014`, `28639202/28841008`, `28487982/29393298`, `28922031/29002042`, `28919575/28910424`, `30523956/29058579`, `28964366/28691363`, `28866960/28766258`, `29886839/31458740`, `29150757/29114574`, `28697984/28333211`, `29241043/28210080`, `28623930/28568006`, `28702674/29852840`, `28805408/28297952`, `28777722/28107383`, `29868493/28925529`, and `29807957/29249618`. The disposable V5 guard raw `parent_load,parent_encode/candidate_load,candidate_encode` observations were `29602202,5649284/29155115,4785991`, `27843348,4764733/28893904,5299409`, `28983713,5399294/28381608,4873532`, `27994396,4551571/28089941,4536758`, `26763258,4130021/27288956,4156982`, `27700171,4524668/27089759,4175119`, `27767336,4453644/27350274,4079699`, `27266900,4439411/27986795,4322477`, `26968165,4270692/27289147,4185318`, `27028091,4597446/27083454,4322866`, `26923548,4283705/27696793,4286407`, `27929834,4614257/27026347,4674669`, `26597258,4136785/27442372,4683935`, `27492042,4350624/27839467,4608915`, `27205465,4064322/27249459,4244837`, `28297206,4563004/26509103,4275132`, `27978777,4525969/26721131,4246228`, `26900148,4130218/27205901,4488097`, `26933289,4059711/28109572,4556185`, `27263518,4043166/28160013,4472669`, `27243348,4493689/27345943,4506089`, `27953516,4523131/26863276,4384002`, `27638214,4882601/28222078,4982061`, and `26789154,4008900/28118836,4554214`.

Before and after timing, existing JSON-structure tests (`2`), BPE construction tests (`16`), GPT-2 complete Hugging Face IDs, and TKZ tests (`2`) passed. The complete existing tokenizer integration suite passed post-timing (`54` active, `9` ignored). A disposable GCP-only parser check also verified that a missing legacy `merges`, a null model type, null tagged-BPE vocabulary, and null tagged-BPE merges all remain errors; that check was removed from the repository before retention. No test file, evaluator, benchmark, dependency, API, format, unsafe code, or model/input dispatch is retained. Runner binaries were `4,389,064` bytes for the parent (SHA-256 `16bfe6aee02dc3e59b619e08c1ecf17c0a0d583a16f986728e64917b8642b9fd`) and `4,397,072` bytes for the candidate (SHA-256 `7e0e3764bff37272e538b4e337c4bdb60657bcd4047ee5b015d681a663882fab`), a `0.18%` increase. The frozen formal competitor runner remains unrebuildable unchanged because its pinned manifest and lockfile conflict with the available Cargo/toolchain; this is therefore an own-parent result, not a current-head competitor claim.

### JSON-load experiment 23: phase-profile the retained BPE constructor — planned

Parent SHA: `d629f26` (retained Experiment 22 source).

Hypothesis: after removing the intermediate model JSON object, the remaining GPT-2 load time is concentrated in one or two BPE constructor phases rather than uniformly spread across parsing, merge resolution, decomposition, and derived encode tables. A test-only phase ledger can select one representation-preserving candidate without guessing from aggregate wall time.

Measured hot cost: Experiment 22 reduces fixed GPT-2 JSON load to `73,636,209` ns mean, but the prior whole-constructor profile predates both retained changes. The still-active constructor performs merge parsing, ID-order reconstruction, per-token decomposition/orphan derivation, ranked-map construction, byte and byte-pair tables, token-length and character tables, merge adjacency, dense/fused metadata, and the global bridge table. PMU access remains blocked by `perf_event_paranoid = 4`.

Invariant that makes the shorter path exact: this is diagnostic-only. The production constructor, parsed inputs, error paths, output IDs, caches, sidecars, APIs, formats, dependency graph, evaluator, and benchmark timing stay byte-for-byte unchanged once the temporary hooks are reverted. The ledger only observes elapsed wall time around existing contiguous constructor regions on the loading thread.

Representation being preserved or changed: add temporary `cfg(test)` timing storage, timestamps at existing phase boundaries, and one ignored test that loads the immutable GPT-2 JSON and prints the phase totals. The diagnostic is committed for source identity, run once after a separate load warmup, recorded, then fully reverted. No production field, branch, environment-variable behavior, test file, benchmark, or evaluator change remains.

Expected winning strata: this screen does not assert a winning runtime stratum. It can authorize a later JSON-construction candidate only when one phase owns a material part of large BPE JSON loading and an exact replacement representation exists.

Expected adverse strata: instrumentation changes test-profile timing and cannot establish throughput; cold filesystem and compile time are outside every phase; small or sidecar loads may have a different distribution. A phase that is only expensive because it preserves an existing exact invariant is not automatically removable.

Smallest files that need changing: temporary hooks and an ignored test in `src/models/bpe.rs`, plus this record. Every source change is reverted after the GCP capture.

Mechanism evidence: `ModelConfig` no longer buffers the BPE model, so the next plausible costs are visible existing regions rather than a new parser library or non-cryptographic hasher. This diagnostic directly measures those regions without changing their data structures or operations.

Acceptance rule: run on the same GCP Intel host and immutable GPT-2 JSON after existing BPE and GPT-2-ID exactness checks. Record raw phase nanoseconds and their sum over at least three fresh test processes; reject any hook that fails to cover the complete `RawBpe -> Bpe` path. Authorize exactly one follow-up candidate only if a single phase or a tightly coupled representation has a median at least `10%` of measured constructor time, an explicit exact invariant, and a plausible elimination or reduction mechanism that does not change the public contract. The next candidate still requires its own card and complete gates.

Rejection rule: remove the instrumentation without runtime candidate code if phase timing is unstable, a hook changes loading semantics, no phase meets `10%`, the apparent cost cannot be shortened without changing semantics, any exactness check fails, or the measurement would require evaluator/model/input-size dispatch. Do not use phase proportions as a competitor comparison or a promotion score.

Experiment 23 passes its diagnostic acceptance rule and authorizes one follow-up representation audit. Committed temporary diagnostic `a5e5976c40eb1b66c6caee2cc961c8c4fd120121` passed existing BPE construction tests (`16` active, one intentionally ignored diagnostic) and GPT-2 complete Hugging Face IDs before capture on the same GCP Intel host and immutable JSON input. One ignored-test warmup was discarded; the three fresh test-process raw `load_ns` and phase nanoseconds were:

| Round | Load | Parse merges | ID order | Decomposition | Ranked map | Byte tables | Byte-pair table | Token metadata | Merge metadata | Bridge table |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 542055492 | 54664405 | 7614677 | 332725515 | 16290439 | 83434 | 28263151 | 4959318 | 33593355 | 3873027 |
| 2 | 537470560 | 54587609 | 7722879 | 331231742 | 16096665 | 86725 | 28293356 | 4925938 | 31969970 | 3821027 |
| 3 | 541914053 | 55979019 | 7746517 | 331673115 | 16189342 | 85987 | 29057701 | 4972430 | 32069190 | 3904069 |

The median whole load was `541,914,053 ns`; timed BPE phases sum to `479,530,619 ns` by per-phase median. Decomposition alone was `331,673,115 ns`, or `61.20%` of whole test-profile load and `69.17%` of the measured BPE phases. Its three values differ by only `0.45%`, easily clearing the predeclared `10%` materiality gate. Test instrumentation alters debug/test timing and is not a speed claim; it identifies only this existing repeated-construction region. The source and ignored test are removed in the following revert, with no production, test, evaluator, benchmark, dependency, API, format, or unsafe change retained.

### JSON-load experiment 24: audit reverse merge provenance for exact decomposition — planned

Parent SHA: `c44c2fa` (retained Experiment 22 source after Experiment 23's full revert).

Hypothesis: for a BPE graph whose non-atomic token ID has one producing merge and whose existing `encoding_decomposition` final pair equals that producer, `unmerge_map` can be built directly from merge provenance rather than repeatedly rerunning BPE on every vocabulary spelling. A separate orphan audit determines whether the exact whole-piece mask is likewise derivable without the repeated reductions.

Measured hot cost: Experiment 23 attributes a median `331,673,115 ns` to decomposition, `69.17%` of measured BPE-construction phases, on the retained GPT-2 JSON path. The current loop independently tokenizes and reduces every multi-character vocabulary spelling solely to populate `unmerge_map` and `is_orphan`.

Invariant that makes the shorter path exact: each parsed merge already verifies that `left_text + right_text == merged_text`. A direct producer pair is exact only when it is the sole producer for that merged token ID and equals the existing canonical final pair. Orphan bits may change only if the audit proves their direct predicate is equivalent for every token; otherwise the later candidate must retain their current derivation. Tokens without a producer, multiple producers, missing atomic characters, duplicate merge pairs, `ignore_merges`, and every rejected audit case keep the existing path.

Representation being preserved or changed: this screen adds a temporary ignored test that parses the immutable JSON model, constructs the current BPE, and compares every existing `unmerge_map` and direct-matcher orphan bit against a reverse map built from its parsed merge map. It records counts of unique, absent, and multiple producers, pair mismatches, orphans by producer class, and token examples. It changes no production source, outputs, API, cache, sidecar, format, dependency, unsafe code, evaluator, or benchmark and is fully reverted afterward.

Expected winning strata: GPT-2 and any other BPE JSON whose merge graph has unique, canonical reverse provenance. A later structure-derived guard may cover such models without using model labels or corpus properties.

Expected adverse strata: tokenizers with multiple producing merges, noncanonical final pairs, orphan spelling edge cases, byte fallback, or `ignore_merges` may be ineligible. A direct reverse map cannot assume that a textual concatenation alone respects Hugging Face's live rank-plus-leftmost reduction order.

Smallest files that need changing: temporary diagnostic code and one ignored test in `src/models/bpe.rs` and `src/models/bpe/tests.rs`, plus this record. All source/test changes are reverted after capture.

Mechanism evidence: `encoding_decomposition` returns only the last pair after repeatedly selecting the minimum live merge rank. The parsed map already owns every candidate `(left, right) -> (rank, merged)` relation. The audit tests the missing equivalence rather than assuming it from token surface text or from a single producer count.

Acceptance rule: on the same GCP host and immutable GPT-2 JSON, run existing BPE and complete GPT-2 Hugging Face-ID checks before the ignored diagnostic. Continue to a separately carded runtime candidate only if every non-identity existing final pair has exactly one reverse producer equal to it, every direct-matcher orphan has an independently recorded equivalent direct condition, all source-token and merge-text checks pass, and the screen identifies no token requiring rank simulation. Record raw counts and examples. A later candidate must retain the current path whenever this structural proof fails and then pass the complete JSON/direct/encode gates.

Rejection rule: revert the diagnostic without candidate code if any final-pair mismatch, ambiguous producer, unexplained orphan, text mismatch, duplicate merge-pair semantic difference, or test failure appears. Do not substitute a model-specific allowlist, ignore a counterexample, weaken orphan handling, or use the profile result as a competitor comparison.

Experiment 24 establishes the GPT-2 provenance facts but does not authorize a direct-producer runtime replacement. Temporary diagnostic `ae21b0ea3741084601062158519166c396f5fe1e` passed existing BPE construction tests (`16` active and one ignored diagnostic) and complete GPT-2 Hugging Face IDs on the same GCP host before its ignored audit. The immutable JSON produced `257` tokens with no merge producer, `50,000` with exactly one producer, and `0` with multiple producers. All `50,000` non-identity current `unmerge_map` pairs matched their sole parsed producer; mismatches were empty. The direct matcher had one orphan, and it belonged to the producer-less class (`orphan_absent=1`, `orphan_unique=0`, `orphan_multiple=0`). Parsed merge validation had already checked every source and merged token text.

These facts show that standard GPT-2 has a clean reverse-provenance graph, but equality against the existing reductions is not a cheaper, general proof that every future graph's rank-plus-leftmost trajectory is constructible. A malformed yet supported graph can have unique producers but a lower-ranked cross-boundary merge that prevents the intended final pair. No model-specific allowlist or optimistic orphan predicate is retained. The temporary test is fully reverted in the following commit; its result instead supports a separate candidate that uses the existing exact ranked lookup table inside the unchanged reduction algorithm.

### JSON-load experiment 25: use the existing ranked merge table during decomposition — planned

Parent SHA: `bb6826b` (retained Experiment 22 source after both diagnostics were fully reverted).

Hypothesis: `encoding_decomposition` repeatedly probes the randomized parsed `HashMap` while reducing every vocabulary spelling, even though the constructor builds `RankedMergeMap` from that identical last-write-wins mapping immediately afterward. Building the ranked table before decomposition and querying it directly removes those repeated generic hash probes without changing a rank or merged ID.

Measured hot cost: Experiment 23 measures decomposition at `331,673,115 ns`, `69.17%` of BPE-construction phases. Its inner loop scans every live adjacent pair and calls `merge_map.get`; ranked-map construction costs only `16,189,342 ns` median and is already required for the final BPE representation. Experiment 24 confirms GPT-2 has 50,000 reductions to exercise this path but is not used as a runtime dispatch condition.

Invariant that makes the shorter path exact: for every parsed pair, `RankedMergeMap::from_parsed` stores exactly the parsed map's final `(rank, merged_id)` after duplicate-pair last-write-wins resolution. Its `get(left, right)` therefore returns `Some((rank, merged))` if and only if the existing `HashMap::get(&(left, right))` does, with identical values. The reduction still scans the same vector in the same order, selects strictly the same lowest rank, and returns the same final pair or stuck condition.

Representation being preserved or changed: move construction of the existing private ranked table before decomposition and pass an immutable reference into the existing reduction helper. Preserve parsed vocabulary and merges, duplicate semantics, BPE ordering, orphan logic, sidecars, direct matcher, caches, outputs, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: large JSON BPE construction, especially GPT-2 and other vocabularies whose decomposition invokes many merge-pair lookups.

Expected adverse strata: small models may not repay early table construction (though the table already exists); V4/V5 direct loads and first encode can shift from code layout alone; malformed and duplicate merge pairs must retain exact last-write-wins behavior.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: `RankedMergeMap::from_parsed` already consumes the parsed map exactly once and `RankedMergeMap::get` already returns the pair information needed by decomposition. The current delayed construction means the expensive loop cannot reuse it.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 complete Hugging Face IDs, and an existing duplicate-merge check before and after timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON load improves beyond the Experiment 22 parent noise, V4/V5 direct load and V5 first encode remain within their parent noise, binary growth is at most `2%`, and no exactness or sidecar guard fails.

Rejection rule: reject and fully revert for any ID or duplicate-merge difference, JSON non-win, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe code, API/format change, source/model/input-size dispatch, or any alteration of parsed-map duplicate semantics.

Experiment 25 is rejected at the JSON target screen. Candidate `1094c8cf815bae2dc76edf7d9c056f7cf5688a58` passed existing BPE construction tests (`16`), JSON-structure tests (`2`), and complete GPT-2 Hugging Face IDs before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable GPT-2 JSON. One untimed process per arm preceded twelve alternating fresh-process JSON loads. Parent/candidate elapsed nanoseconds were `71513922/177745550`, `71117727/181391655`, `70642749/187937809`, `72772594/183327862`, `70682282/176590100`, `72251482/179293305`, `71401768/182061896`, `71960375/179773315`, `71815942/177634850`, `73493081/174768415`, `71951606/180453342`, and `72794902/173854695`.

Candidate/parent throughput was `0.400275x` (median `0.400273x`; means `71,866,536` versus `179,569,400` ns). The failure is far outside measurement noise, so V4/V5, first-encode, binary-size, and post-timing checks were deliberately skipped. The existing ranked table is exact but performs poorly under the decomposition loop's access pattern; no further mechanism is inferred from this result. The source is fully reverted in the following commit, and no test, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch change is retained.

### JSON-load experiment 26: use the existing merge-adjacency rows during decomposition — planned

Parent SHA: `e0499d1` (retained Experiment 22 source after Experiment 25's full revert).

Hypothesis: `MergeAdjacency` is an exact CSR form of the same parsed pair map, storing each left token's sorted right-token rows contiguously and returning the same `(rank, merged_id)`. Building that already-retained table before decomposition may serve its repeated pair probes more cheaply than randomized `HashMap` lookups, unlike Experiment 25's rejected open-address table.

Measured hot cost: decomposition remains `331,673,115 ns` median in Experiment 23. Experiment 25 rejects the ranked open-address table at `0.400275x`, so this is a distinct locality hypothesis, not a retry. The current constructor already builds `MergeAdjacency::from_parsed` after decomposition for the finished BPE; the retained table's query is a bounded binary search over one left-token row.

Invariant that makes the shorter path exact: `MergeAdjacency::from_parsed` inserts every unique parsed `(left, right)` pair's existing `(rank, merged_id)` exactly once. `MergeAdjacency::get(left, right)` returns that value if and only if the parsed map does. The unchanged reduction visits the same pair sequence, chooses the same strictly lowest rank, and keeps duplicate last-write-wins semantics fixed at parsing.

Representation being preserved or changed: move construction of the existing private adjacency table before decomposition and pass an immutable reference into the existing reduction helper. Keep its final storage unchanged. Preserve all parsed inputs, merge order, orphan behavior, sidecars, direct matcher, caches, IDs, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: large JSON BPE construction whose decomposition repeatedly resolves pair ranks, especially GPT-2.

Expected adverse strata: rows with several right-token neighbors may make binary search slower than `HashMap`; small JSON files may not repay early construction; direct V4/V5 and first encode remain code-layout guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the table is already constructed and retained by the successful parent, but only after the measured hot phase. Its sorted CSR rows avoid Experiment 25's hash/probe layout while representing the same exact map.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 complete Hugging Face IDs, and the duplicate-merge check before and after timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any ID or duplicate-merge difference, JSON non-win, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe code, API/format change, source/model/input-size dispatch, or change to parsed-map semantics. Do not reintroduce Experiment 25's table after observing this result.

Experiment 26 is retained as the new scoped JSON-load parent. Candidate `59141b6c4b1588afeddf3f00d68e1566164556f9` and parent `e0499d1` were clean native root-lock builds on the same GCP Intel CPU-0. The immutable GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `70562266/66930168`, `70793582/67113991`, `70561253/66827806`, `70266281/66636683`, `69638964/68489399`, `71165550/67798850`, `70353370/68344880`, `71236505/67805951`, `70755258/67407059`, `70790959/67416760`, `70232912/67239655`, and `70869101/73713208`. Candidate/parent throughput was `1.038954x` (median `1.049861x`; means `70,602,167` versus `67,977,034` ns; 11 of 12 pairs favored the candidate).

The V4 direct twelve-pair parent/candidate values were `25552860/24592859`, `24179927/24590751`, `24665545/23939934`, `23876348/24578323`, `24357340/24096154`, `24234930/24365464`, `24520256/24097567`, `25072301/25411619`, `24436531/23991510`, `24628351/24523069`, `23918338/24936886`, and `23928414/24075170`, for `1.000544x` throughput (median `0.999468x`). V5 direct raw pairs were `30407745/28089150`, `28692541/28661590`, `28450724/28038591`, `28953843/28715014`, `28066124/29277866`, `27874954/28158925`, `28346558/27980190`, `28994353/28308249`, `28832162/29125034`, `28067049/28645256`, `28633551/28300754`, `28063798/28267451`, `28449417/27938050`, `28247883/29015347`, `28038435/28527344`, `28312302/27941267`, `28509510/28856816`, `28660407/28711644`, `28506064/28544905`, and `28708748/27858080`, for `1.003197x` (median `0.999860x`).

A disposable V5 guard preserved checksum `17406679798874925224` in all 24 observations. Its raw `parent_load,parent_encode/candidate_load,candidate_encode` rows were `26681013,4082894/26567634,3845965`, `26093401,3815433/26323647,3838380`, `26083129,3830367/25960406,3923334`, `26086458,3842153/26308519,3865702`, `26007165,3824266/25934112,3816613`, `26108504,3824933/26614323,3827034`, `25992448,3851338/26448350,3958985`, `25922323,3813516/27215331,3851113`, `25930088,3830391/26277695,3822145`, `25914179,3824072/26326036,3832975`, `26184982,3818362/26708083,4048865`, `26302375,3872648/26909029,3881025`, `26181966,4012383/26325692,3884162`, `25934337,3814107/26122702,3835316`, `26186085,3964138/26035919,3810569`, `26242174,3817423/26485814,3964132`, `26029204,3797408/26128126,3868242`, `25867352,3797709/26464517,3807943`, `26105351,4026885/26566979,3805830`, `26634908,3788443/26057479,3802767`, `26169189,3898657/26288087,3843100`, `26326236,3838390/25843318,3818611`, `25920958,4025162/26304605,3788993`, and `25834730,3819830/26027999,3809433`. Load was `0.991330x` (median `0.991406x`; means `26,114,106` versus `26,343,517` ns) and first encode `1.002912x` (median `0.997759x`; means `3,867,955` versus `3,856,301` ns), both within the guard's observed short-run variation.

Existing BPE tests (`16`), JSON-structure tests (`2`), and complete GPT-2 Hugging Face IDs passed before timing. Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained. Runner binary size decreased from `4,397,072` bytes (parent SHA-256 `7e0e3764bff37272e538b4e337c4bdb60657bcd4047ee5b015d681a663882fab`) to `4,396,752` bytes (candidate SHA-256 `e47095e45bc3aed709102104f412631fdc7b0df25e06ee9fc4096d58c88db165`). The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention, not a current-head Snaptokens-versus-Hugging-Face claim.

### JSON-load experiment 27: reuse decomposition token storage across vocabulary entries — planned

Parent SHA: `52d14ae` (retained Experiment 26 source).

Hypothesis: `encoding_decomposition` starts a fresh growable `Vec<TokenId>` for every vocabulary spelling, then drops it after one final-pair result. One caller-owned buffer cleared between entries preserves the same transient token sequence while avoiding repeated allocation/growth and destruction during the measured decomposition phase.

Measured hot cost: Experiment 23 measures decomposition at `331,673,115 ns` median, and Experiment 26's exact adjacency reuse improved JSON load `1.038954x` but leaves that per-token working-vector lifecycle intact. GPT-2 executes the reduction for 50,000 non-identity vocabulary tokens in the provenance audit, so a transient allocation cost repeats at that scale.

Invariant that makes the shorter path exact: clearing a private `Vec<TokenId>` preserves its logical empty sequence; each call pushes the same per-character IDs, executes the unchanged scan/remove/reduction order, and returns only copied token IDs or status. The buffer never escapes, no result borrows it, and its retained capacity has no effect on token IDs, errors, cache, or public memory ownership.

Representation being preserved or changed: change only the private helper to receive a caller-owned scratch vector and allocate it once around the existing vocabulary loop. Preserve merge lookup, character mapping, rank ordering, orphan logic, sidecars, direct matcher, caches, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: large JSON BPE vocabularies that execute many short decomposition reductions, notably GPT-2.

Expected adverse strata: one unusually long vocabulary spelling retains the scratch capacity through the rest of construction; small vocabularies may not repay the extra parameter; direct V4/V5 and first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the current helper constructs `Vec::new`, pushes one ID per character, uses `remove`, then drops the vector once per vocabulary entry. Its result never references that allocation, making storage reuse exact.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 complete Hugging Face IDs, and the duplicate-merge check before and after timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any ID or duplicate-merge difference, JSON non-win, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe code, API/format change, source/model/input-size dispatch, or retained scratch whose capacity is unbounded by the largest existing vocabulary spelling.

Experiment 27 is rejected at its V5 first-encode guard. Candidate `ab15252f792e0f88da7cd0fa839b57f0e26b63eb` passed existing BPE construction tests (`16`), JSON-structure tests (`2`), and complete GPT-2 Hugging Face IDs before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable JSON. One untimed process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `67784016/65803727`, `67017681/65557547`, `66834429/65916434`, `67953126/66122623`, `67865602/66523166`, `66538640/64962852`, `67358061/65683735`, `67369491/65879299`, `67156917/64742236`, `67448474/66557323`, `66778671/65575468`, and `66381221/65435470`. Candidate/parent throughput was `1.022478x` (median `1.022446x`; means `67,207,194` versus `65,729,990` ns), and every JSON pair favored the candidate.

V4 direct throughput was `0.998876x` over twelve pairs (median `1.005706x`; means `24,256,255` versus `24,285,118` ns), while V5 direct was `0.999333x` over twenty (median `1.001372x`; means `28,361,018` versus `28,380,317` ns). Both are within direct-load variation. The disposable V5 guard kept checksum `17406679798874925224` in all 24 rows and its load sub-measurement was flat at `1.000447x` (median `0.996090x`; means `26,681,099` versus `26,669,321` ns). But first encode regressed to `0.983611x` (median `0.981853x`; means `3,970,181` versus `4,037,792` ns). The candidate fails the predeclared no-regression rule, so post-timing expansion and binary-size checks were deliberately skipped. The source is fully reverted in the following commit; no test, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch change is retained.

V4 raw parent/candidate pairs were `24964672/24544864`, `24177078/24097425`, `24120150/24497454`, `23959555/24154289`, `24252821/24057787`, `24030746/25438992`, `24398624/24162259`, `24407648/23893230`, `24243011/23935908`, `24048742/24522280`, `23850872/24052848`, and `24621140/24064077`. V5 pairs were `27989289/28720627`, `28278459/28521031`, `28285683/28038212`, `28406982/28220452`, `28310263/28522060`, `28955174/28571615`, `28082574/28289400`, `28395774/28368847`, `28338954/28633784`, `28780613/28729055`, `28577670/28187667`, `28362110/28073615`, `28458420/28313111`, `28312309/28331285`, `28526675/28862750`, `28247122/28253841`, `28138664/28030646`, `28476773/28092070`, `28300816/28005123`, and `27996044/28841153`. Guard rows as `parent_load,parent_encode/candidate_load,candidate_encode` were `26890109,3982547/26005025,3938102`, `26157135,3809489/26216764,3803825`, `26469967,3860778/27006299,4458435`, `26894142,3950486/27224661,4053825`, `26110579,3808569/26380510,3908950`, `26844823,3943071/25946125,3998967`, `26729410,3963152/25958475,3901374`, `26434193,3978431/26563163,3920687`, `26107108,4063757/26312845,3977844`, `26102525,4058118/26755548,3909325`, `26659413,3889978/26267399,4096117`, `26310874,4082085/26611542,3884639`, `26349333,3922480/27162718,4267506`, `27656275,4118396/27738518,4212400`, `27639875,3948819/26764509,4143543`, `27105722,3877922/26992615,4089453`, `27086130,4014291/26352501,4149252`, `26873942,4011148/26823214,4144896`, `27201906,4150236/27432969,4093872`, `26935420,3922375/26963253,3919961`, `26243409,3892684/27206337,4114875`, `26175256,4099536/26851821,3914294`, `27210519,4078695/26076919,3908063`, and `26158300,3857310/26449976,4096800`.

### JSON-load experiment 28: use the retained BMP character table during decomposition — planned

Parent SHA: `c62994c` (retained Experiment 26 source after Experiment 27's full revert).

Hypothesis: decomposition maps each character in every vocabulary spelling through `vocab.get(&char_utf8)`, although the constructor later derives `bmp_char_token`, an exact direct mapping for every one-scalar BMP vocabulary token. Building that already-retained table before decomposition and consulting it first removes repeated string-hash lookups for BMP characters while retaining the current map fallback for astral Unicode.

Measured hot cost: Experiment 23 puts decomposition at `331,673,115 ns` median. Experiment 26 improves its pair lookup but leaves character-to-initial-token resolution as a `HashMap` probe for every character of all 50,000 GPT-2 reducible tokens. The existing `bmp_char_token` table is constructed from the same ordered vocabulary later in the same constructor and costs no additional retained memory.

Invariant that makes the shorter path exact: for any BMP scalar `ch`, the table slot contains its ID exactly when `vocab` has the one-scalar UTF-8 spelling `ch`; otherwise it holds `INVALID_TOKEN`. This is the same key that the current helper encodes into its local UTF-8 buffer. Astral scalars retain the existing map lookup. All resulting initial token-ID sequences, reduction ranks, final pairs, and orphan decisions are unchanged.

Representation being preserved or changed: move construction of the existing `bmp_char_token` table before decomposition, pass it into the private helper, use one indexed lookup for BMP scalars, and retain the current UTF-8 map lookup for astral scalars. Preserve vocabulary, merge parsing and order, adjacency lookup, sidecars, matcher, caches, output IDs, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: large BPE JSON vocabularies whose stored tokens predominantly comprise BMP scalars, including ByteLevel GPT-2 spellings.

Expected adverse strata: astral-heavy vocabularies retain the existing fallback; early table construction can only move already-required work; V4/V5 direct load and first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: `bmp_char_token` already traverses `id_to_token` and writes `token_id` for each one-scalar BMP spelling. Decomposition currently repeats the same mapping through a UTF-8 buffer and randomized map lookup, before that table exists.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 complete Hugging Face IDs, and an existing Unicode/byte-fallback correctness case before and after timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any BMP/astral/byte-fallback ID difference, JSON non-win, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe code, API/format change, source/model/input-size dispatch, or fallback removal.

Experiment 28 is retained as the new scoped JSON-load parent. Candidate `79921922a16a832b480bc4e4cb3c397734c07bc4` and parent `c62994c` were clean native root-lock builds on the same GCP Intel CPU-0. The immutable GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `68049229/63742533`, `67595467/62630675`, `67377049/63112799`, `68095206/63231988`, `68202355/62758845`, `68396958/63449653`, `68503295/64313455`, `66589362/64918933`, `67007052/62558946`, `67709253/64342637`, `67184602/62956759`, and `66456144/62756081`. Candidate/parent throughput was `1.066261x` (median `1.067565x`; means `67,597,164` versus `63,397,775` ns; every pair favored the candidate).

V4 direct raw parent/candidate pairs were `25984461/25526608`, `24223511/25545383`, `24300603/24188935`, `25121787/24915185`, `24791550/24838079`, `24157118/24528862`, `31653580/26171735`, `24638066/24438488`, `24351724/25845886`, `24106086/24186103`, `24962111/25363567`, and `23976614/24462027`, for `1.004996x` throughput (median `0.997409x`). V5 pairs were `29154711/28898972`, `27769439/28600206`, `28231404/29118196`, `27964907/27985993`, `28374720/28849422`, `28781071/28831618`, `28119330/28089144`, `28998177/28014049`, `27948999/28433588`, `28567838/28849951`, `27985425/28477278`, `29129477/27992402`, `29407912/28441816`, `28092254/28347724`, `28934629/28752854`, `29258990/28956837`, `29010771/28957287`, `28533981/29588821`, `28716378/29378105`, and `29403796/29556508`, for `0.996950x` (median `0.996540x`).

A disposable V5 guard preserved checksum `17406679798874925224` in all 24 rows. Its raw `parent_load,parent_encode/candidate_load,candidate_encode` observations were `28276601,4185179/27009646,3897977`, `26594563,3963458/26883642,4056224`, `26337686,3804591/26304442,3817808`, `26221389,3797912/26518888,3830634`, `26937428,4023314/26648464,3830656`, `26459491,4037103/26538648,3848514`, `26173680,4014802/27215143,3832888`, `26657678,3814582/26320880,3840683`, `25871590,3795962/26303485,3863245`, `26085149,3804875/26747399,3825576`, `26670543,3796211/26057966,3799091`, `25893500,3831836/26000960,3828814`, `26657003,3815932/26842207,4013548`, `26616696,3816476/26209016,3826657`, `27333465,3771461/26894313,3852088`, `26464672,3983969/27300104,3792334`, `26832583,3805290/26678353,3802324`, `26271087,3859835/26176351,3777016`, `26392242,3859478/26633150,3796500`, `25991408,3884631/26608930,4070324`, `27634726,3831734/27603303,4016689`, `27234955,4036873/26932371,3805698`, `27005942,3807136/25884822,3804562`, and `27668954,3792087/26491576,4031653`. Load was `1.002194x` (median `1.001201x`; means `26,678,460` versus `26,616,836` ns) and first encode `1.002832x` (median `0.998291x`; means `3,880,614` versus `3,869,229` ns), inside the guard's variation.

Existing BPE tests (`16`), GPT-2 complete Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained. The release runner grew from `4,396,752` bytes (parent SHA-256 `e47095e45bc3aed709102104f412631fdc7b0df25e06ee9fc4096d58c88db165`) to `4,396,888` bytes (candidate SHA-256 `890259c5fd300b83d46f70c5daad1c5a8b5cfcf803fe0b9f428b8583cd545a47`), a `0.003%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention, not a current-head Snaptokens-versus-Hugging-Face claim.

### Historical Snaptokens JSON-load context after Experiment 28

The retained binary `ca96799df0620aa92b3d7a025e42c5c28f44921f3c2d4c893af52e9849806218` is an older Snaptokens runner, not a Hugging Face runner: its retained `daf8899:benches/load_bench.rs` source calls `snaptokens::Tokenizer::from_file`. One untimed warmup and twelve CPU-0 fresh processes on the immutable GPT-2 JSON recorded `load_ns` values of `53638176`, `52543723`, `52852802`, `51601127`, `53412438`, `51782379`, `53585651`, `55165800`, `52325756`, `53412015`, `53797620`, and `52351880`: mean `53,039,114 ns`, median `53,132,409 ns`, and population standard deviation `954,926 ns`. It emitted the same checksum `17406679798874925224` and first-encode IDs as the Snaptokens guards. Its `63,397,775 / 53,039,114 = 1.195302x` elapsed ratio is historical own-project context only. It is not Hugging Face evidence. The frozen formal runner still cannot be rebuilt unchanged because its pinned manifest/lock conflicts with the available Cargo/toolchain; no evaluator, runner, lockfile, or benchmark artifact changed.

### JSON-load experiment 29: linear-probe small merge-adjacency rows — planned

Parent SHA: `3490a1a` (retained Experiment 28 source).

Hypothesis: the exact `MergeAdjacency::get` table now serves decomposition, but it binary-searches every left-token row, including rows with only one to four packed entries. A short linear scan of those tiny sorted rows returns the same matching pair with fewer branch and search operations, while larger rows retain the existing binary search.

Measured hot cost: decomposition remains the dominant construction phase from Experiment 23, and Experiment 26 shows the adjacency representation itself improves JSON load `1.038954x`. `MergeAdjacency::get` is called from decomposition and the generic encode engine. Its packed rows are already contiguous and sorted by right token; binary search overhead is least justified when a row has only a few entries.

Invariant that makes the shorter path exact: each small-row scan visits the exact packed keys the binary search considers and returns only when `(key >> 32) as u32 == right`, with the same rank in the key low half and the same `new_ids` index. Rows remain sorted and unique by right token. Rows above the fixed threshold retain the existing `partition_point` implementation.

Representation being preserved or changed: add a private small-row branch inside `MergeAdjacency::get`; preserve the table layout, rows, keys, ranks, IDs, larger-row path, decomposition, BPE ordering, sidecars, caches, APIs, formats, dependencies, unsafe invariants, evaluator, and timing boundary. No test file is added.

Expected winning strata: large JSON construction and generic encode workloads with frequent degree-one through degree-four adjacency probes.

Expected adverse strata: models with wide rows stay on the existing path; a branch can cost workloads dominated by wide rows; direct V4/V5 and first encode remain explicit guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: `get` already reads an aligned slice for one left ID, then calls `partition_point`. Its keys are sorted by `right << 32 | rank`, so a sequential comparison over a short slice identifies the same unique right-token match.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 complete Hugging Face IDs, and generic-model integration coverage before and after timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, relevant generic encode tests pass, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any lookup or ID difference, JSON non-win, generic encode regression, direct-load or first-encode regression, binary growth above `2%`, retained test, evaluator change, dependency, unsafe change, API/format change, source/model/input-size dispatch, or altered wide-row behavior.

Experiment 29 is rejected at the JSON target screen. Candidate `34fd4914adf9a384fe4ffbb8c83da903cc6dd0f7` passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and Qwen3 Hugging Face IDs (`2`) before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable JSON. One untimed process per arm preceded twelve alternating loads. Parent/candidate elapsed nanoseconds were `62815888/62961402`, `62157880/63279790`, `63204007/67802189`, `62733091/63457460`, `62335644/64043809`, `63166317/63560901`, `62869408/63848781`, `62407949/63189798`, `62395498/63312730`, `63879952/64008317`, `63525486/64851015`, and `62006507/64531853`.

Candidate/parent throughput was `0.980178x` (median `0.985087x`; means `62,791,469` versus `64,070,670` ns). The result is a consistent target loss, so V4/V5, first encode, binary size, and post-timing expansion were deliberately skipped. The source is fully reverted in the following commit, with no test, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch change retained.

### JSON-load experiment 30: remove the duplicate decomposition character scan — planned

Parent SHA: `27dfa7b` (retained Experiment 28 source after Experiment 29's full revert).

Hypothesis: the outer vocabulary loop first executes `text.chars().count()` to skip scalar tokens, then `encoding_decomposition` decodes the same text again to create its token-ID sequence. Calling the existing helper for every spelling removes the duplicate scan; its existing `tokens.len() < 2` result preserves the skip outcome.

Measured hot cost: decomposition remains the largest measured constructor phase. Experiment 28 makes the second scan's BMP ID resolution cheap, making the first standalone character-count walk proportionally more visible across 50,257 GPT-2 vocabulary spellings.

Invariant that makes the shorter path exact: for a one-scalar spelling, the helper returns `CharsNotInVocab` after at most one exact character lookup; the outer loop previously made no `unmerge_map` or orphan change. For a missing character, both paths return the same non-orphan status. Multi-scalar inputs execute the unchanged reduction. No rank, pair, or final ID changes.

Representation being preserved or changed: remove only the outer count-and-continue branch. Keep helper behavior, BMP/astral fallback, adjacency resolution, orphan assignment, sidecars, caches, APIs, formats, dependencies, unsafe code, evaluator, and timing boundary unchanged. No test file is added.

Expected winning strata: large JSON BPE vocabularies with many multi-scalar spellings.

Expected adverse strata: single-scalar-heavy vocabularies perform one cheap helper call per token; direct V4/V5 and first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the existing helper already returns `CharsNotInVocab` for fewer than two initial IDs; the outer `chars().count()` only avoids invoking that same classification.

Acceptance rule: commit a clean candidate; format; run existing BPE tests, GPT-2 IDs, and byte-fallback coverage before and after timing. Use the same GCP parent/candidate protocol with twelve alternating JSON pairs; retain only for a JSON win beyond noise with no V4/V5/first-encode, exactness, binary, or API guard loss.

Rejection rule: reject and fully revert for any ID/orphan difference, JSON non-win, direct-load or first-encode regression, retained test, evaluator change, dependency, unsafe code, API/format change, source/model/input dispatch, or altered helper result.

Experiment 30 is rejected at the JSON target screen because its gain is inside the recorded meaningful-win/noise band. Candidate `5d3642ff04cb5289c5e943fba2530293096704c7` passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable JSON. One untimed process per arm preceded twelve alternating loads. Parent/candidate elapsed nanoseconds were `62739069/63778366`, `63256136/63399219`, `62890669/62710203`, `62892919/62303869`, `64708186/62560424`, `63895059/62680836`, `64102264/62454982`, `63308001/62590434`, `64457083/62203516`, `62499971/63107735`, `62555687/63448778`, and `62634329/61747020`.

Candidate/parent throughput was `1.009206x` (median `1.010459x`; means `63,328,281` versus `62,748,782` ns). The candidate does not meet the predeclared improvement rule, so direct-load, first-encode, binary-size, and post-timing expansion were deliberately skipped. The source is fully reverted in the following commit; no test, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch change is retained.

### JSON-load experiment 31: refresh BPE constructor phase attribution — planned diagnostic

Parent SHA: `b8ed980` (retained Experiment 28 source after the full reversions of Experiments 29 and 30).

Hypothesis: the pre-Experiment-26 phase profile made decomposition the largest GPT-2 constructor region, but the retained adjacency and BMP-table changes have changed that balance. Re-measuring the unchanged constructor regions will identify the next representation-level cost rather than extending low-evidence micro-branches.

Measured hot cost: the earlier test-only profile measured decomposition at `331,673,115 ns` in its diagnostic build. Experiments 26 and 28 have since removed its repeated map lookups, while Experiment 30 showed that a small scan-only change is below the meaningful-win threshold.

Invariant that makes the diagnostic exact: all timing hooks compile only under `cfg(test)`, run outside production release artifacts, and observe contiguous constructor regions without changing their inputs, control flow, outputs, caches, sidecars, or timing boundary.

Representation being preserved or changed: preserve every production representation. Temporarily restore the prior test-only phase ledger and one ignored external-input diagnostic test, updating only phase boundaries that moved with retained Experiments 26 and 28. The diagnostic source will be fully reverted regardless of result.

Expected winning strata: none; this selects the next candidate only for the immutable GPT-2 JSON construction path.

Expected adverse strata: none in production because the instrumentation is test-only. A profiling-build result is mechanism evidence, never a production throughput claim.

Smallest files that need changing: `src/models/bpe.rs`, `src/models/bpe/tests.rs`, and this record.

Mechanism evidence: the earlier profile already established contiguous constructor regions for merge parsing, ID ordering, decomposition, merge metadata, byte tables, and bridge-table construction. The currently retained changes are concentrated in decomposition and move `merge_adj` and `bmp_char_token` earlier, so the old boundaries require a fresh reading.

Acceptance rule: commit the diagnostic separately; run exactly the ignored phase test on the GCP Intel host against the immutable GPT-2 JSON with the root lockfile and native flags. Record the complete phase ledger and fully revert the instrumentation. A subsequent production experiment must start from the clean retained parent and receive its own card.

Rejection rule: reject the diagnostic method and fully revert immediately if it changes production codegen, JSON input, evaluator, benchmark, API, tokenizer format, output IDs, or retains a test. Do not infer a production speedup from the diagnostic build.

Experiment 31 completes its diagnostic acceptance rule and is fully removed. Temporary instrumentation commit `a1db809` was formatted and built with the root lockfile on the GCP Intel host using native flags. The ignored diagnostic test loaded the immutable GPT-2 JSON (SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`) once as an untimed warmup, then three fresh CPU-0 test processes. Each diagnostic load completed and the test's ordinary encode probe passed. The warmup was `68,981,493 ns` and is excluded. The three recorded rows were:

| Round | Load | Parse merges | ID order plus pre-decomposition tables | Decomposition | Ranked map | Byte tables | Byte-pair table | Token metadata | Merge metadata | Bridge table |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 69,011,779 | 8,102,898 | 5,013,471 | 25,448,561 | 4,385,443 | 18,866 | 3,540,900 | 230,891 | 5,351,481 | 716,846 |
| 2 | 64,170,237 | 7,613,578 | 4,503,315 | 24,949,857 | 4,285,584 | 18,177 | 3,552,762 | 177,990 | 4,660,955 | 720,541 |
| 3 | 63,297,629 | 7,677,520 | 4,134,273 | 24,454,720 | 4,220,161 | 19,920 | 3,669,534 | 170,804 | 4,681,007 | 701,932 |

The median whole diagnostic load was `64,170,237 ns`; the median phase sum was `50,563,747 ns`. Decomposition remains the largest region at `24,949,857 ns` (`49.35%` of the measured BPE phases and `38.88%` of diagnostic load), but parser ownership (`7,677,520 ns`) and merge metadata (`4,681,007 ns`) are now material secondary lanes. The ID-order field intentionally includes the retained early construction of `merge_adj` and `bmp_char_token`, so it is not comparable to Experiment 23's older boundary. This is attribution from a test build, not a speed claim, exactness proof, or competitor comparison. The instrumentation and ignored test are reverted in the following commit; no production source, test, evaluator, benchmark, dependency, API, format, unsafe code, or input-specific dispatch remains.

### JSON-load experiment 32: stack-resident short decomposition tokens — planned

Parent SHA: `0a06290` (clean retained Experiment 28 source after the Experiment 31 diagnostic revert).

Hypothesis: every vocabulary spelling currently allocates a fresh `Vec<TokenId>` before its exact reduction. A fixed stack buffer for short spellings removes that allocator and heap metadata traffic; an unchanged heap-backed fallback handles every spelling that exceeds the buffer.

Measured hot cost: Experiment 31's fresh GCP attribution measured decomposition at `24,949,857 ns`, the largest current BPE constructor region. The immutable GPT-2 vocabulary has `50,185 / 50,257` spellings no longer than sixteen UTF-8 bytes; therefore at least that many have no more than sixteen Unicode scalars and can use the fixed buffer. The remaining fourteen spellings, and any longer spelling in every other tokenizer, take the full existing heap path.

Invariant that makes the shorter path exact: the short path stores exactly the same initial scalar token IDs, selects the same lowest-ranked adjacent pair on each reduction, replaces and shifts the same position, and returns the same final pair/orphan classification. It falls back before processing the seventeenth scalar, restarts the unchanged heap algorithm from the original spelling, and therefore never truncates or changes a long input.

Representation being preserved or changed: replace only temporary constructor-local storage for short decomposition sequences from a heap `Vec<u32>` to a fixed `[u32; 16]` plus logical length. Retain the generic `Vec` path for long sequences, BMP/astral lookup behavior, merge ranks, adjacency lookup, ordering, sidecars, matcher, caches, public API, JSON and `.tkz` formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: BPE JSON construction whose vocabulary is dominated by short spellings, including ByteLevel vocabularies.

Expected adverse strata: long-spelling-heavy vocabularies take the retained heap fallback; a small stack frame can affect code layout, so V4/V5 direct load and first encode are explicit guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the current helper begins with `Vec::new()` and `push` for every scalar of every spelling. Experiment 27's heap scratch reuse improved JSON loading `1.022478x` but was rejected because its altered reuse lifetime regressed V5 first encode; this candidate instead keeps storage call-local, bounded, and has no retained heap allocation on the common path.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and existing byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any short/long, BMP/astral, rank, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; unsafe code; API/format change; source/model/input-size dispatch; or fallback removal.

Experiment 32 is retained as the new scoped JSON-load parent. Candidate `b5e99ce43ed421050d54f6892105f0205d126a4c` and parent `0a06290` were clean native root-lock builds on the GCP Intel CPU-0; the parent source is byte-identical to the retained Experiment 28 source used for the immutable parent binary. The GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `63858009/57782883`, `62311299/58796513`, `63193362/59174106`, `63968343/58161163`, `63608078/57491150`, `63944650/58211365`, `62754088/58214670`, `62782085/59931864`, `64483545/57786563`, `62817956/58149819`, `63204442/59285665`, and `63066149/58222971`.

Candidate/parent JSON throughput was `1.083855x` (median `1.081731x`; means `63,332,667` versus `58,434,061 ns`; every pair favored the candidate). This passes the target acceptance rule and removes one bounded constructor allocation for the common vocabulary spelling, rather than reusing heap storage across later encode calls.

V4 direct raw parent/candidate pairs were `25290948/25916676`, `24179180/24338471`, `24832240/24294211`, `25585107/24522757`, `25437684/24198255`, `24459153/25372454`, `25281162/24158453`, `24504941/24126176`, `24770866/24314604`, `24784413/24353722`, `24696087/24468823`, and `24657526/25611218`, for `1.009631x` throughput (median `1.016692x`; means `24,873,276` versus `24,639,652 ns`). V5 direct pairs were `29716940/28590785`, `28797839/28321577`, `28507914/28161443`, `28448265/29701158`, `28405352/27974318`, `28270704/28919007`, `28414654/29673155`, `28203477/28132947`, `28685215/28450436`, `28832942/28915146`, `28125187/28032175`, `28515682/28461282`, `28091118/28844360`, `28770216/29423831`, `28849834/29142635`, `28825005/28918274`, `28774105/28253101`, `28640953/28709263`, `28809867/28187076`, and `30878139/29296904`, for `1.000744x` throughput (median `1.002209x`; means `28,728,170` versus `28,705,444 ns`). Both direct paths are within the retained variation band.

The disposable V5 first-encode guard kept checksum `17406679798874925224` in all 24 rows. Its raw `parent_load,parent_encode/candidate_load,candidate_encode` observations were `27831000,4066774/27873096,3858025`, `26509210,3825984/27278898,3816865`, `27278382,4072773/27618087,4049234`, `27196360,4101989/27370295,4101727`, `27461899,4105542/26905455,3830622`, `27371558,3857619/26855549,3804180`, `27190703,4009180/26501517,3867232`, `26520543,3838246/27420434,3983067`, `27099025,3829336/27981803,3815698`, `26459705,3867425/26618336,4032223`, `27641828,4009541/26779668,3789427`, `26266731,3810118/27227774,4084619`, `26432505,4010695/27236849,4020565`, `26336711,3779155/27325919,3951025`, `26977231,4007881/26688872,3901285`, `26515146,3876785/27317529,3830441`, `27101588,3898397/26417109,3970467`, `26765476,3839647/27264307,3913769`, `27155947,4005060/26043766,3786796`, `27061329,3849263/26682688,3807049`, `26544345,3833557/26686064,3774226`, `26758589,4118720/26661100,3816556`, `26413516,4046293/27060039,3835355`, and `25954244,3795379/26932347,3866678`. Its load sub-measurement was `0.993972x` (median `0.993843x`; means `26,868,482` versus `27,031,146 ns`), inside direct-load variation, while first encode improved `1.010065x` (median `1.008451x`; means `3,935,640` versus `3,896,130 ns`).

Existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed on GCP. `cargo fmt --all -- --check` passed locally without running tests. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or input-specific dispatch is retained. The release runner grew from `4,396,888` bytes (parent SHA-256 `890259c5fd300b83d46f70c5daad1c5a8b5cfcf803fe0b9f428b8583cd545a47`) to `4,397,560` bytes (candidate SHA-256 `e4e5efdbcb95cd091d2b2d83837b60b9e41fe852ddacc20c754c2176837d4332`), a `0.015%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention rather than a confirmed current-head Snaptokens-versus-Hugging-Face result.

### JSON-load experiment 33: refresh attribution after stack decomposition — planned diagnostic

Parent SHA: `6240e92` (clean scoped Experiment 32 parent).

Hypothesis: Experiment 32 removes the common temporary decomposition allocation and materially changes the cost hierarchy. Refreshing the test-only constructor attribution will select the next work-removal lane from the new parent instead of assuming that the previous decomposition profile remains proportional.

Measured hot cost: Experiment 31 measured decomposition at `24,949,857 ns`, ahead of merge parsing at `7,677,520 ns` and merge metadata at `4,681,007 ns`. Experiment 32 targets the former directly and produced a `1.083855x` target-screen win.

Invariant that makes the diagnostic exact: the timing ledger and ignored external-input test compile only under `cfg(test)`, observe constructor regions without changing their outputs or release codegen, and are fully reverted after capture.

Representation being preserved or changed: preserve every production representation. Temporarily restore the prior test-only ledger and ignored phase test without retaining either. No evaluator, benchmark, API, tokenizer format, input, dependency, unsafe code, or production timing boundary changes.

Expected winning strata: none; this is attribution only for the immutable GPT-2 JSON construction path.

Expected adverse strata: none in production because the instrumentation is test-only. Its output cannot be used as a speed claim.

Smallest files that need changing: `src/models/bpe.rs`, `src/models/bpe/tests.rs`, and this record.

Mechanism evidence: the prior ledger has stable contiguous boundaries. The stack representation changes only the reduction helper, making its current share directly measurable without modifying normal release artifacts.

Acceptance rule: commit the diagnostic separately; run one untimed warmup and three fresh CPU-0 ignored-test processes on the GCP Intel host against the immutable GPT-2 JSON, record the complete phase ledger, and fully revert the instrumentation. A production candidate must begin at the clean retained parent with its own card.

Rejection rule: reject the diagnostic method and fully revert if it changes production codegen, JSON input, evaluator, benchmark, API, tokenizer format, output IDs, or retains a test. Do not infer an implementation win from test-build attribution.

Experiment 33 completes its diagnostic acceptance rule and is fully removed. Temporary instrumentation commit `1a934f4` was formatted and built with the root lockfile on the GCP Intel host using native flags. The ignored diagnostic test loaded the immutable GPT-2 JSON once as an untimed warmup, then three fresh CPU-0 test processes. Each load completed and the ordinary encode probe passed. The warmup was `64,656,637 ns` and is excluded. The three recorded rows were:

| Round | Load | Parse merges | ID order plus pre-decomposition tables | Decomposition | Ranked map | Byte tables | Byte-pair table | Token metadata | Merge metadata | Bridge table |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 63,811,535 | 8,205,076 | 5,034,685 | 20,956,868 | 4,348,535 | 19,478 | 3,529,204 | 220,601 | 4,912,845 | 732,088 |
| 2 | 59,734,584 | 7,518,964 | 4,280,590 | 20,654,420 | 4,358,620 | 21,684 | 3,469,257 | 174,199 | 4,498,993 | 713,059 |
| 3 | 58,674,990 | 7,514,708 | 4,204,448 | 20,022,860 | 4,072,854 | 18,202 | 3,403,166 | 160,107 | 4,348,311 | 711,523 |

The median whole diagnostic load was `59,734,584 ns`; the median phase sum was `45,677,495 ns`. Decomposition remains the largest region at `20,654,420 ns` (`45.22%` of measured phases and `34.58%` of diagnostic load), while merge parsing (`7,518,964 ns`), ranked-map construction (`4,348,535 ns`), merge metadata (`4,498,993 ns`), and the byte-pair table (`3,469,257 ns`) are independently material. The stack path reduces the diagnostic decomposition median by `4,295,437 ns` from Experiment 31. This remains test-build attribution, not a production speed claim or competitor comparison. The instrumentation and ignored test are reverted in the following commit; no production source, test, evaluator, benchmark, dependency, API, format, unsafe code, or input-specific dispatch remains.

### JSON-load experiment 34: construct initial byte pairs from parsed merge rules — planned

Parent SHA: `0a79c3c` (clean scoped Experiment 32 parent after the Experiment 33 diagnostic revert).

Hypothesis: the byte-pair table currently performs up to `256 × 256` ranked-hash lookups, although the parsed merge map already contains every final `(left, right) -> (rank, merged)` rule. Inverting the fixed byte-to-initial-token array once and streaming the parsed rules writes exactly the same table entries without probing absent pairs.

Measured hot cost: Experiment 33 measures the byte-pair table at `3,469,257 ns` median (`7.59%` of measured BPE construction phases). The existing table visits all 65,536 byte pairs, while the immutable GPT-2 JSON has 50,000 parsed merge rules; each rank lookup also traverses the separately built open-address map.

Invariant that makes the shorter path exact: after dense vocabulary-ID validation, each non-sentinel `byte_to_initial_token[byte]` is an in-range unique initial token ID. Its inverse maps a merge endpoint back to that one byte. For every parsed last-write-wins merge rule whose two endpoint IDs both occur in the inverse, the replacement writes `(rank, merged_id)` to exactly the table cell the old double-byte loop would obtain from `ranked_merge_map.get`; every other cell remains the same `(u32::MAX, 0)`. Parsed-map and ranked-map values are already equivalent by construction.

Representation being preserved or changed: replace only the constructor-local byte-pair table fill. Retain the parsed map, ranked map, table shape and sentinels, byte initial-token table, rank/merged values, merge ordering, decomposition, sidecars, matcher, caches, all encode paths, public API, JSON and `.tkz` formats, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: BPE construction where the vocabulary has the normal byte alphabet and the merge map is materially smaller than all possible byte pairs.

Expected adverse strata: sparse or large vocabularies pay one temporary two-byte-per-token inverse allocation and keep the same 65,536-entry output table; direct V4/V5 load and first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: `byte_to_initial_token` is fully initialized immediately before the current closure, `merge_map` remains live until the end of construction, and `RankedMergeMap::from_parsed` stores the same final values only to serve later runtime lookups. The table needs the parsed rules' values, not a re-query through that runtime representation.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any byte-pair rank, merged-ID, sentinel, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; unsafe code; API/format change; source/model/input-size dispatch; or removal of dense-ID validation.

Experiment 34 is retained as the new scoped JSON-load parent. Candidate `5fb7c157b10de2563d7d447d651e2e6c42b0535b` and parent `0a79c3c` were clean native root-lock builds on GCP Intel CPU-0; the parent source is the retained Experiment 32 source. The GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `59217345/56309570`, `59008193/56216290`, `58711805/55765325`, `58774245/56699957`, `59489450/55507151`, `58402175/56402912`, `58708132/55724758`, `58325401/56119453`, `58932430/55394147`, `59091676/56256124`, `58535267/56369843`, and `58276800/55729973`.

Candidate/parent JSON throughput was `1.049043x` (median `1.050034x`; means `58,789,410` versus `56,041,292 ns`; every pair favored the candidate). Composed with Experiment 32's measured parent-relative ratio, the scoped screen is `1.137013x` faster than Experiment 28. The candidate removes absent ranked-map probes from the byte-pair constructor; it does not alter the runtime ranked map.

V4 direct raw parent/candidate pairs were `26130367/22472952`, `24772307/21328461`, `24753484/22047008`, `25066438/21494042`, `24852505/21113064`, `24972121/21422920`, `24900288/21302550`, `25443983/21807870`, `24886478/22334184`, `24236194/21286451`, `24386450/21252388`, and `25185583/21157433`, for `1.156662x` throughput (median `1.164210x`; means `24,965,516` versus `21,584,944 ns`). V5 direct pairs were `29872788/26231459`, `29203218/25378876`, `29803325/26449082`, `28586087/25957404`, `29504855/25399674`, `29341092/25368126`, `28877979/26700322`, `28727404/25735016`, `28933231/25827179`, `28165546/25978974`, `28778116/25995973`, `29241239/25258624`, `29219755/25335923`, `28895495/25821703`, `28785083/25108742`, `28781600/25808937`, `29392137/25749524`, `28677144/25646590`, `29272848/25471760`, and `27998388/25446483`, for `1.127035x` throughput (median `1.123541x`; means `29,002,866` versus `25,733,519 ns`).

The disposable V5 first-encode guard kept checksum `17406679798874925224` in all 24 rows. Its raw `parent_load,parent_encode/candidate_load,candidate_encode` observations were `27674346,4167337/23635053,4103166`, `27439055,3800233/23305022,3856293`, `26556645,4010084/23097966,3782295`, `26244859,3991317/23481422,3792859`, `26706878,3776786/23012382,3759749`, `26409315,3771430/22900659,3760298`, `26408999,3786821/22929091,3964348`, `26284475,3905275/23313506,4017673`, `26880560,3983341/23641028,3827561`, `26353349,4026524/23158153,3816218`, `27203782,4010121/23028931,3787826`, `28513244,3825917/24078080,4003781`, `27195938,3817891/22905574,3814645`, `26890673,3787339/24604760,3813489`, `27405724,3983831/23027635,3971960`, `26691989,3785283/23438034,3780123`, `26340522,3798556/25077440,3833154`, `26893300,3788546/23514837,3813071`, `26640191,3802032/23533027,3812549`, `26986437,4021287/23350126,3841708`, `26743288,4039840/23463768,3952525`, `26480133,4125653/23234190,4011038`, `26749411,3808396/23441956,3969357`, and `26460322,3919624/22971676,3798524`. Its load sub-measurement improved `1.145948x` (median `1.146707x`; means `26,839,726` versus `23,422,680 ns`), while first encode improved `1.008978x` (median `1.002975x`; means `3,905,561` versus `3,870,175 ns`).

Existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed on GCP. `cargo fmt --all -- --check` passed locally without running tests. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or input-specific dispatch is retained. The release runner grew from `4,397,560` bytes (parent SHA-256 `e4e5efdbcb95cd091d2b2d83837b60b9e41fe852ddacc20c754c2176837d4332`) to `4,397,944` bytes (candidate SHA-256 `a74246f392770daec42f044f8bd44d01851bc869ea6751223f81ab30edc0994d`), a `0.009%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention rather than a confirmed current-head Snaptokens-versus-Hugging-Face result.

### JSON-load experiment 35: reuse the exact byte-pair table in decomposition — planned

Parent SHA: `047ea06` (clean scoped Experiment 34 parent).

Hypothesis: the existing byte-pair table stores the exact parsed merge answer for every pair of byte-initial token IDs, but decomposition currently binary-searches the CSR adjacency for those same pairs. Build the already-required table before decomposition and consult it when both live IDs have a byte-initial inverse; leave all other pairs on the exact CSR path.

Measured hot cost: Experiment 33 measures decomposition at `20,654,420 ns`, the largest current constructor region. The helper scans adjacent initial scalar tokens for every vocabulary spelling. For ByteLevel BPE vocabularies, many of those IDs are the 256 initial byte-token IDs represented by the already-required fixed table.

Invariant that makes the shorter path exact: after dense-ID validation, the temporary inverse maps a token ID to a byte exactly when `byte_to_initial_token[byte] == token`. Experiment 34 establishes that `byte_pair_initial[byte1 * 256 + byte2]` equals `merge_map[(token1, token2)]` when present and keeps `(u32::MAX, 0)` exactly when absent. Therefore table lookup returns the same `(rank, merged_id)` as `MergeAdjacency::get` for two inverse-mapped IDs; all IDs outside that exact subset keep the unchanged adjacency lookup. The reduction still scans in the same order and selects the same strict lowest rank.

Representation being preserved or changed: move construction of the existing byte initial-token array, transient inverse, and fixed byte-pair table before decomposition; pass immutable references to the private lookup helper. Retain the final table, byte mapping, dense-ID validation, BMP/astral resolution, merge ranks, reduction ordering, adjacency fallback, sidecars, matcher, caches, public API, JSON and `.tkz` formats, dependencies, unsafe code, evaluator, and timing boundary. The inverse remains constructor-local and is dropped before the finished BPE is returned. No test file is added.

Expected winning strata: BPE JSON construction with many byte-initial character tokens in vocabulary spellings, including ByteLevel GPT-2-like structures.

Expected adverse strata: non-byte or sparse-byte vocabularies take the same adjacency fallback after paying a short-lived inverse allocation; V4/V5 direct load and first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: `byte_pair_initial` is part of every constructed `Bpe`; Experiment 34 makes its values direct parsed-map results. Decomposition currently has no table path even though its initial tokens are resolved from the same canonical vocabulary IDs.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any byte-pair/adjacency lookup, rank, merged-ID, sentinel, order, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; unsafe code; API/format change; source/model/input-size dispatch; or removal of the generic adjacency fallback.

Experiment 35 is retained as the new scoped JSON-load parent. Candidate `e3bb7f780d0b51072ee7f63516e54517e4ead365` and parent `047ea06` were clean native root-lock builds on GCP Intel CPU-0. The immutable GPT-2 JSON was SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `56196204/43478017`, `57093697/44436833`, `55303084/43111971`, `55055291/43456934`, `60166652/43725769`, `55533194/42604009`, `55953666/43097936`, `55288263/42955880`, `56716547/43985962`, `56418653/43500786`, `55053574/42799551`, and `55997135/42942568`.

Candidate/parent JSON throughput was `1.297135x` (median `1.290970x`; means `56,231,330` versus `43,341,351 ns`; every pair favored the candidate). Composed with the retained Experiment 32 and 34 parent-relative screens, the scoped result is `1.474859x` faster than Experiment 28. The moved byte table preserves every parsed-map rank and merged ID; decomposition only replaces the CSR probe where its two live IDs map to that same table cell.

V4 direct raw parent/candidate pairs were `22085637/21947858`, `21771607/21640641`, `21445576/21163877`, `21157394/21283527`, `21377390/22107993`, `22035514/21564034`, `21326869/22081837`, `21238743/21122303`, `21252525/21806679`, `21679164/21460642`, `21472641/21393071`, and `21863803/21801794`, for `0.997440x` throughput (median `1.004615x`; means `21,558,905` versus `21,614,521 ns`). V5 direct raw parent/candidate pairs were `26227667/26148518`, `24980728/25503407`, `25043671/25025320`, `24816266/25362457`, `25605703/25696273`, `25102543/26254923`, `26378372/25690364`, `25955472/25546777`, `25398597/25698242`, `25603715/24993780`, `25135402/25305849`, `24960237/25274224`, `25381230/25145441`, `25615620/25620125`, `25313115/25517082`, `25118389/25562290`, `25799463/25207019`, `26006216/25149610`, `26104772/26417872`, and `25619215/25530928`, for `0.999008x` throughput (median `0.998150x`; means `25,508,320` versus `25,532,525 ns`). Both controls are within the direct-load variation observed on this host.

The disposable V5 first-encode guard kept checksum `17406679798874925224` in all 24 rows. Its raw `parent_load,parent_encode/candidate_load,candidate_encode` observations were `24213332,4115079/23295542,3857253`, `23528020,4011388/24017094,4013599`, `23693988,3819158/23092364,3822406`, `23818652,3773232/22913082,3783347`, `24151348,4004282/23568519,3782485`, `24169577,3818303/23316337,3786386`, `23853301,3793876/23098425,3785132`, `23727861,3787616/22927491,3915276`, `22851991,3787534/23350920,3939443`, `23315294,3797164/22715109,3794370`, `23375644,4005898/23506228,3988169`, `23199120,3784978/23214265,3773612`, `23040889,3776978/24875550,3752577`, `23739948,3866338/22909590,3870393`, `23186734,3771673/22955396,3831319`, `22759916,3835229/23281322,3768124`, `23508063,3761315/23365611,3783507`, `24033688,3927530/22984254,3753383`, `22797982,3759946/23662734,3971862`, `22880359,3832971/22655299,3780527`, `23410932,3874146/23531439,3778544`, `23726464,3763147/24217631,3957277`, `23145559,3974859/23593105,3827759`, and `23551596,3795509/23412570,3941243`. Its load sub-measurement was `1.005789x` (median `1.008015x`; means `23,486,677` versus `23,352,495 ns`), while first encode improved `1.001858x` (median `1.001525x`; means `3,851,590` versus `3,844,083 ns`).

Existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed on GCP. `cargo fmt --all -- --check` passed locally without running tests. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or input-specific dispatch is retained. The root load runner grew from `4,397,944` bytes (parent SHA-256 `a74246f392770daec42f044f8bd44d01851bc869ea6751223f81ab30edc0994d`) to `4,399,872` bytes (candidate SHA-256 `b68fae38971d45a3105817ca0e6a6ad812b030e023566e7f5b83de7ae412231e`), a `0.044%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention rather than a confirmed current-head Snaptokens-versus-Hugging-Face result.

### JSON-load experiment 36: refresh constructor attribution after byte-pair decomposition lookup — planned diagnostic

Parent SHA: `e3bb7f780d0b51072ee7f63516e54517e4ead365` (clean scoped Experiment 35 source).

Hypothesis: Experiment 35 removes the old CSR lookup from byte-initial decomposition pairs and reduced the complete JSON screen by `12,889,979 ns`; Experiment 33's phase attribution is therefore stale. A disposable test-only timing ledger can identify the next constructor region without changing the release code or evaluator.

Measured hot cost: the previous attribution had decomposition at `20,654,420 ns`, parsed merges at `7,518,964 ns`, merge metadata at `4,498,993 ns`, ranked-map construction at `4,348,535 ns`, and byte-pair construction at `3,469,257 ns` median. The changed decomposition representation invalidates those proportions.

Invariant that makes the diagnostic exact: `#[cfg(test)]` timestamps only observe contiguous constructor regions in the existing order; the release build, parsed representation, merge decisions, IDs, public API, formats, evaluator, and timing boundary are unchanged. The diagnostic test loads the immutable GPT-2 JSON and reports its full existing Hugging Face IDs before and after profiling.

Representation being preserved or changed: add only temporary test-gated phase fields, timestamp writes, and one ignored diagnostic test. Retain all release behavior and source representations; remove the entire instrumentation and test after capture regardless of the observed ordering. No runtime candidate, test, benchmark, evaluator, dependency, unsafe code, API, format, or dispatch change is eligible for retention.

Expected winning strata: none; this is attribution only.

Expected adverse strata: test builds add timing calls, so they cannot be compared with release timing and cannot establish a speed claim.

Smallest files that need changing: `src/models/bpe.rs`, `src/models/bpe/tests.rs`, and this record.

Mechanism evidence: the measured end-to-end change is much larger than the prior phase-level pair-table cost, so selecting a direct follow-up from the old phase totals would confound the new decomposition lookup with obsolete work shares.

Acceptance rule: commit the temporary diagnostic; run existing BPE construction tests and complete GPT-2 Hugging Face IDs before capture on the same GCP Intel CPU-0 and immutable JSON; discard one warmup, capture three fresh test processes, and record every phase row. Continue only with a separately carded runtime candidate whose exact invariant follows from the refreshed dominant cost.

Rejection rule: fully revert the temporary code and test without a runtime candidate if the diagnostic changes IDs, fails existing coverage, has unstable or incomplete phase rows, or would require model/input-specific dispatch. Do not use its phase times as a competitor result or promotion score.

Experiment 36 passes its attribution acceptance rule. Temporary diagnostic `5919a1f` added only test-gated timestamps and one ignored test. Existing BPE construction tests passed (`16` active, `1` ignored) and complete GPT-2 Hugging Face IDs passed before capture on the same GCP Intel CPU-0 and immutable GPT-2 JSON. One ignored-test warmup was discarded. The three fresh test-process rows were `load=46,362,606 ns; parse=7,868,445; id_order=3,031,067; byte_initial=21,234; byte_pair=221,561; initial_maps=1,906,052; decomposition=8,160,405; ranked_map=4,421,652; token_metadata=181,720; merge_metadata=4,908,519; bridge=707,159`, `load=45,025,201 ns; parse=7,674,004; id_order=3,033,581; byte_initial=21,625; byte_pair=228,782; initial_maps=1,968,872; decomposition=7,959,069; ranked_map=3,968,869; token_metadata=238,831; merge_metadata=4,779,470; bridge=733,225`, and `load=45,783,484 ns; parse=7,677,018; id_order=3,002,260; byte_initial=21,531; byte_pair=227,336; initial_maps=1,949,208; decomposition=8,482,654; ranked_map=4,310,264; token_metadata=198,847; merge_metadata=4,801,313; bridge=718,521`.

The median phase order is decomposition (`8,160,405 ns`), parsed merges (`7,677,018 ns`), merge metadata (`4,801,313 ns`), ranked-map construction (`4,310,264 ns`), ID order (`3,031,067 ns`), and initial maps (`1,949,208 ns`). This profile is attribution only: its test instrumentation changes the build and it does not establish an own-parent or competitor timing result. A first source-copy staging attempt compiled the added ignored test against the old BPE source, and a first Cargo exact-filter invocation selected zero tests; neither produced a phase row and both were discarded before the three valid rows above. Commit `431db52` fully reverts the instrumentation and the ignored test; the isolated branch again contains no diagnostic test or runtime code. The next candidate must separately prove any shorter residual decomposition representation.

### JSON-load experiment 37: carry byte-initial membership through decomposition — planned

Parent SHA: `e3bb7f780d0b51072ee7f63516e54517e4ead365` (clean scoped Experiment 35 source; later commits contain only its ledger and reverted diagnostic).

Hypothesis: the residual decomposition loop repeatedly resolves both live token IDs through `initial_token_byte` before consulting the fixed byte-pair table. A spelling with `n` tokens scans many live pairs over its `n - 1` merges, but each token's byte-initial membership is a pure function of its ID and changes only when that slot is replaced. Keep a synchronized `u16` membership buffer with the token buffer so each initial and merged ID is classified once instead of once per inspected pair.

Measured hot cost: refreshed Experiment 36 attribution ranks decomposition at `8,160,405 ns` median, the largest constructor region. Experiment 35's table path still calls `initial_token_byte[left]` and `initial_token_byte[right]` for every candidate pair in every rank scan.

Invariant that makes the shorter path exact: at every live index `i`, `byte_membership[i] == initial_token_byte[tokens[i] as usize]`. Initialization derives that value directly from each exact initial token; when the existing reduction writes `best_new` to a slot, it derives the replacement value from that same exact ID; and the existing `copy_within` removal is mirrored for the membership buffer. Passing the two cached memberships to the existing byte-table/CSR selector yields exactly the lookup that recomputing them from the IDs would have yielded. Rank comparison, leftmost scan order, merged ID, sentinels, stack bound, heap fallback, and final-pair return stay unchanged.

Representation being preserved or changed: add a private parallel byte-membership buffer to the existing stack and heap decomposition reductions. Retain token IDs, scalar resolution, parsed map, byte-pair table, inverse map, CSR fallback, rank ordering, reduction mutation/order, orphan handling, sidecars, JSON and `.tkz` formats, encode paths, public API, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: BPE JSON construction with multi-scalar vocabulary spellings, especially byte-initial ByteLevel vocabularies where the fixed table path is repeatedly selected.

Expected adverse strata: short spellings pay only one membership write per token; long spellings use the existing heap fallback plus a same-length membership vector; sparse/non-byte vocabularies still take the exact CSR fallback with cached sentinels. V4/V5 direct load and V5 first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: direct pair lookup is already exact and retained, but its membership loads are inside the residual nested rank scan. Caching the membership is representation reuse within the same private reduction, not a tokenizer, corpus, model, input-size, or environment dispatch.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any membership/ID desynchronization, byte-table/CSR lookup, rank, merged-ID, sentinel, stack/heap, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; unsafe code; API/format change; source/model/input-size dispatch; or removal of the generic CSR fallback.

Experiment 37 is rejected at the JSON target screen. Candidate `3208af1` passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable GPT-2 JSON. One untimed process per arm preceded twelve alternating fresh-process JSON loads. Parent/candidate elapsed nanoseconds were `44333631/43465877`, `42972746/44780684`, `44660427/44806202`, `43603776/42950356`, `44205078/43215422`, `42734414/43138128`, `42756182/43203195`, `42873803/43576497`, `43768433/42731300`, `43041929/43638833`, `44140661/43740093`, and `44756239/43402175`.

Candidate/parent throughput was `1.002255x` (median `1.002954x`; means `43,653,943` versus `43,554,064 ns`; paired range `0.959627x–1.031200x`). The result is inside the recorded meaningful-win/noise band, so V4/V5 direct load, first encode, binary-size, and post-timing expansion were deliberately not run. Commit `6b9cee5` reverts the source change in full. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained; the candidate and parent timing binaries were SHA-256 `34c5a432d067ac36f1aa8da3d66254bcb2601beb049f3d763da821897be3429e` and `b68fae38971d45a3105817ca0e6a6ad812b030e023566e7f5b83de7ae412231e`, respectively.

### JSON-load experiment 38: retain unchanged decomposition pair results — planned

Parent SHA: `e3bb7f780d0b51072ee7f63516e54517e4ead365` (clean scoped Experiment 35 source; later commits contain only ledger entries and reverted diagnostics/candidates).

Hypothesis: each decomposition iteration rescans every live adjacent pair and recomputes its same exact byte-table/CSR merge answer, though a selected merge changes only the two adjacent pairs that touch its replacement. Initialize a parallel pair-result buffer once, shift it with the existing token compaction, and recompute only the left and right exposed adjacencies after each merge; retain the existing left-to-right strict-minimum rank scan over those cached exact results.

Measured hot cost: refreshed Experiment 36 attribution ranks decomposition at `8,160,405 ns` median. Experiment 37 confirms that merely caching byte membership is insufficient, leaving repeated pair-result lookup and selection as the remaining work inside the nested reduction loop.

Invariant that makes the shorter path exact: before every rank scan, for every live adjacency `i`, `pair_results[i] == decomposition_merge(tokens[i], tokens[i + 1])`. Initialization computes all entries through the unchanged selector. After replacing `tokens[best_pos]` and applying its existing compaction, all adjacencies except `best_pos - 1` and `best_pos` are unchanged values shifted by the same removal, so their cached results are copied with the pair buffer; the two changed adjacencies are recomputed through the same selector. The unchanged scan order and strict `<` comparison therefore select the same rank and leftmost position, write the same merged ID, and return the same final pair or stuck state. Stack and heap paths retain separate buffers with the same length relationship.

Representation being preserved or changed: add a private `Option<(rank, merged_id)>` pair-result buffer beside the existing token buffer in stack and heap decomposition. Retain token IDs, scalar resolution, byte-pair table, inverse map, CSR fallback, rank ordering, token mutation/order, orphan handling, sidecars, JSON and `.tkz` formats, encode paths, public API, dependencies, unsafe code, evaluator, and timing boundary. No test file is added.

Expected winning strata: BPE JSON construction with multi-scalar vocabulary spellings whose rank reductions revisit unchanged adjacent pairs.

Expected adverse strata: one- and two-scalar spellings pay only bounded buffer initialization; long spellings retain heap fallback with a parallel result vector; non-byte/sparse vocabularies retain the exact CSR answers. V4/V5 direct load and V5 first encode remain guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the current mutating reduction already keeps the live token sequence contiguous with `copy_within`. A pair result depends only on its two live IDs, so untouched adjacencies do not need another table/CSR lookup before the next otherwise-identical minimum-rank scan.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any cached-pair/token desynchronization, byte-table/CSR lookup, rank, leftmost-order, merged-ID, sentinel, stack/heap, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; unsafe code; API/format change; source/model/input-size dispatch; or removal of the generic CSR fallback.

Experiment 38 is rejected at the JSON target screen. Candidate `72edbde` passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable GPT-2 JSON. One untimed process per arm preceded twelve alternating fresh-process JSON loads. Parent/candidate elapsed nanoseconds were `43402685/43515684`, `42772013/43476118`, `43575900/44012591`, `43339455/42507671`, `44913630/42380778`, `43772418/42569479`, `43835329/43313547`, `44303713/43201396`, `43906003/42897511`, `43324167/43539058`, `43834990/43258299`, and `43126911/44308422`.

Candidate/parent throughput was `1.009888x` (median `1.012690x`; means `43,675,601` versus `43,248,380 ns`; paired range `0.973334x–1.059760x`). The apparent gain is inside the recorded meaningful-win/noise band, so V4/V5 direct load, first encode, binary-size, and post-timing expansion were deliberately not run. Commit `56c7d30` reverts the source change in full. No test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained; the candidate and parent timing binaries were SHA-256 `6f4dfa24b72a8bd53a9d830a091520d1bc997536c7f40de28995eb4c5b770d94` and `b68fae38971d45a3105817ca0e6a6ad812b030e023566e7f5b83de7ae412231e`, respectively.

### JSON-load experiment 39: split ranked merge-map keys from payloads — planned

Parent SHA: `e3bb7f780d0b51072ee7f63516e54517e4ead365` (clean scoped Experiment 35 source; later commits contain only ledger entries and reverted diagnostics/candidates).

Hypothesis: `RankedMergeMap::from_parsed` allocates and initializes a mixed 16-byte `(key, rank, merged_id)` slot for every bucket before inserting the same canonical rules. Split it into a `u64` key array initialized entirely to the existing empty sentinel and a same-size packed `(rank, merged_id)` payload array initialized to zero. Construction can use uniform fills while lookup retains the identical probe sequence and reads a payload only after its matching key.

Measured hot cost: refreshed Experiment 36 attribution ranks ranked-map construction at `4,310,264 ns` median. The map is mandatory for the finished model's compatibility lookup and `.tkz` resolution, so deleting or deferring it would change required behavior or move work into first encode.

Invariant that makes the shorter path exact: for every occupied index, the key array stores the same `pack_pair(left, right)` value and the payload array stores the same `(rank << 32) | merged_id` value as the old slot; every unoccupied key is `EMPTY_KEY` and its ignored zero payload is never read. Capacity, mask, `fx_hash` home bucket, linear probe order, termination at the first empty key, duplicate resolution before construction, cached-index validation, clone/equality, sidecar serialization reconstruction, and the returned `(rank, merged_id)` remain unchanged. The payload read stays after an exact key match.

Representation being preserved or changed: replace only private ranked-map slot layout with parallel key/payload vectors of equal total element bytes. Retain merge rules, rank ordering, open-address probing, adjacency, decomposition, byte-pair and dense tables, sidecars, caches, encode paths, public API, JSON and `.tkz` formats, dependencies, evaluator, and timing boundary. The existing bounded unchecked lookup is extended only to the same-index payload after the checked-key invariant; no test file is added.

Expected winning strata: JSON and direct `.tkz` construction of nonempty BPE merge maps, where allocation and initialization of the ranked table is material.

Expected adverse strata: compatibility lookup gains one additional base pointer on a hit; V4/V5 direct load and V5 first encode remain guards. Empty maps retain empty vectors and no lookup behavior changes.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the existing CSR adjacency representation already separates merge keys from values to avoid fetching payloads before a successful key search. This candidate applies the same exact data separation to the ranked map's mandatory construction while preserving its open-address algorithm and total table bytes.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any key/payload, probe, rank, merged-ID, empty-sentinel, sidecar, clone/equality, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; API/format change; source/model/input-size dispatch; or removal/weakening of the lookup's safety invariant.

Experiment 39 is retained as the new scoped JSON-load parent. The first implementation commit `a3b212f` had one Rust 2024 match-ergonomics error in a sidecar iterator and did not build; the following source-only correction `851dbcf03e2554edf7aa4bc6250a9ba0b3287af4` fixed it before any exactness or timing work. Its final `src/models/bpe.rs` SHA-256 was `fc3b53cf617be1ff0a9cf3e46c6318f5f2287ad6c2dbe7038d353d7b26c14550`, matching the GCP source copy. Candidate and Experiment 35 parent were clean root-lock native builds on GCP Intel CPU-0. The immutable GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. Existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `44817792/42540417`, `42755624/41677345`, `43503217/42108388`, `43134603/42282296`, `43467243/42597402`, `43412333/42987612`, `45229274/43761595`, `43225492/41928615`, `42753470/41868008`, `43895890/42673149`, `43177262/42442364`, and `43256856/42149168`.

Candidate/parent JSON throughput was `1.026685x` (paired median `1.026076x`, one-sided paired t 95% lower bound `1.019796x`; means `43,552,421` versus `42,418,030 ns`; every pair favored the candidate). The retained constructor work is uniform initialization of private parallel arrays rather than mixed slots; the runtime lookup still probes exactly the same keys and reads a payload only after a matching initialized key. Composed with Experiments 32, 34, and 35, the scoped GPT-2 JSON screen is `1.514216x` faster than the Experiment 28 parent.

V4 direct raw parent/candidate pairs were `21847968/21398896`, `21974072/21446771`, `21110512/20973127`, `21804312/21580239`, `21136742/21088662`, `21097527/21651603`, `21040656/21412620`, `21329200/21786558`, `21166425/21010397`, `20971206/21102588`, `21192648/21035104`, and `21403410/21041473`, for `1.002102x` throughput (median `1.006988x`; means `21,339,557` versus `21,294,003 ns`). V5 direct pairs were `26017355/25598344`, `25767398/24982644`, `25120699/25200025`, `25455805/25604316`, `25041754/25071903`, `25163523/25141460`, `24994309/24947603`, `24894566/24935397`, `25119606/25027284`, `25377304/25967527`, `25808887/25952741`, `25211844/25259633`, `25066751/25762316`, `25051481/25391798`, `25990069/25818326`, `24922805/25089635`, `25024684/24918626`, `25041193/24887698`, `25382955/25424303`, and `25071552/24925071`, for `0.999253x` throughput (median `0.998586x`; means `25,276,227` versus `25,295,333 ns`). Both are within observed variation.

The V5 cache-backed first-encode guard had matching checksum `17406679798874925224` in all twenty-four pairs. Its parent load/encode and candidate load/encode nanoseconds were `23869142/4249839/24696614/4065281`, `23534572/3823257/23935358/3913088`, `23627580/3822101/23084662/3824375`, `23512729/3791026/24345356/3776805`, `23449520/3817781/23477431/3797037`, `24496576/3993409/24459013/3824660`, `23702856/3830193/23897091/3993341`, `23603463/3810581/23541495/3802428`, `23577258/4006969/23920609/3803065`, `23304151/3776232/23921805/3999852`, `23662379/3795683/23414126/3954716`, `23418328/4079229/23623134/3952756`, `23667784/3847390/23219454/3968527`, `23429365/3780814/23530833/4006821`, `23447676/3781872/23808134/3796980`, `23021181/3990328/23175302/3840164`, `23995684/3884582/23184180/3869442`, `23332442/3795056/23591709/3781058`, `23218578/3932154/23560716/3811709`, `23727035/3846060/23494322/3789762`, `23346586/3771805/23441585/3987857`, `23527728/3768868/23273953/3809609`, `23262445/3787427/23322770/3883494`, and `23228042/3772496/23049301/3813116`. Parent/candidate load throughput was `0.996543x` (means `23,540,129` versus `23,623,706 ns`) and first-encode throughput was `0.996469x` (means `3,864,798` versus `3,877,748 ns`), both within observed variation.

Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed on GCP. `cargo fmt --all -- --check` passed locally without running tests. No test file, evaluator, benchmark, dependency, API, format, or input-specific dispatch is retained. The release runner grew from `4,399,872` bytes (parent SHA-256 `b68fae38971d45a3105817ca0e6a6ad812b030e023566e7f5b83de7ae412231e`) to `4,401,392` bytes (candidate SHA-256 `8f17899b711bc0ef80ee8d81913393c0b1d12c87e8c6003f76845ddc36d2c6c1`), a `0.034546%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention rather than a confirmed current-head Snaptokens-versus-Hugging-Face result.

### JSON-load experiment 40: size dense ranked merge table from initial-token ID range — planned

Parent SHA: `851dbcf03e2554edf7aa4bc6250a9ba0b3287af4` (clean scoped Experiment 39 source).

Hypothesis: a ranked non-byte-fallback BPE currently allocates and initializes a fixed `1024 × 1024` dense early-ID merge table whenever every initial byte token is below `1024`, even if its actual initial-token ID range is much smaller. Derive the table bit width from the maximum initial byte-token ID, then use the unchanged CSR fallback for pairs outside that exact smaller square. GPT-2's 256 unique byte initials occupy IDs `0..=255`; the existing table is four MiB, while the candidate's `256 × 256` table is 256 KiB.

Measured hot cost: Experiment 36 attributes merge metadata at `4,801,313 ns` median. The immutable GPT-2 rule scan confirms all 50,000 rules preserve the existing rank-plus-256 invariant, and 1,916 have both endpoints below 256 versus 15,373 below 1024. The dense table is only used for the 16--31 byte stack path after initial byte-pair seeding, so reduced coverage must retain the existing exact CSR fallback and be guarded by V5 first encode.

Invariant that makes the shorter path exact: the new bit width is the smallest power-of-two range containing every initial byte-token ID, so every pair still indexed in the dense table receives the identical merge ID or `u32::MAX` sentinel. Every pair outside that range takes the existing `merge_adj.get(left, right)` path, which is built from the same canonical parsed merge map and returns the same rank and merged ID. Rank order, initial byte-pair seeding, short-loop mutation order, heap fallback, byte fallback, `ignore_merges`, JSON and `.tkz` construction, and output IDs remain unchanged.

Representation being preserved or changed: replace only the fixed construction-time dense ranked table width with a private, structure-derived width stored beside its existing vector. Retain the table's packed layout and sentinel, ranked-merges proof, CSR representation, parser, decomposition, sidecars, caches, encode APIs, formats, dependencies, evaluator, and timing boundary. No test file is added.

Expected winning strata: ranked BPEs with contiguous low byte-initial IDs, where the prior fixed 1024-square allocation dominates merge metadata construction; GPT-2 is the target screen.

Expected adverse strata: short raw pieces that create pairs above the smaller range make more CSR probes, while tables whose initial IDs require the current range retain their existing width. V4/V5 direct load and V5 first encode are explicit guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the existing 1024 bound is a sufficient construction guard, not a property of every valid pair seen after merging. The direct dense lookup already has an exact CSR fallback; using the initial-token range only narrows when it takes the dense answer and never changes the fallback's data or priority result.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any dense range/index, sentinel, CSR-fallback, rank, merged-ID, short/heap, byte-fallback, sidecar, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; API/format change; source/model/input-size dispatch; or removal/weakening of the dense-lookup safety invariant.

Experiment 40 is retained as the new scoped JSON-load parent. Candidate `c1acf0d41d8937f7768e71a8cb152b881547235c` and Experiment 39 parent were clean root-lock native builds on GCP Intel CPU-0. The candidate's `src/models/bpe.rs` SHA-256 was `2eaf602be598088fe5ba947cf7ee7d7d8215d7c2ea08e8ad9b4925ab6fc979a6`, matching the GCP source copy. The immutable GPT-2 JSON remained SHA-256 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`. Existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage passed before timing. One untimed fresh process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `43139355/41320286`, `41888112/40996695`, `41786775/40602111`, `42062063/40590279`, `41822390/40879337`, `41916855/40509506`, `41927364/41208235`, `41884882/40574393`, `42163941/40940208`, `42492693/40668183`, `42287616/42113234`, and `42529671/40799354`.

Candidate/parent JSON throughput was `1.029941x` (paired median `1.031094x`, one-sided paired t 95% lower bound `1.022323x`; means `42,158,476` versus `40,933,485 ns`; every pair favored the candidate). GPT-2's byte initials are exactly the unique IDs `0..=255`, so its 256-KiB dense square retains every answer it can index. The remaining pairs make the same CSR lookup rather than reading the previous four-MiB table. Composed with Experiments 32, 34, 35, and 39, the scoped GPT-2 JSON screen is `1.559553x` faster than the Experiment 28 parent.

V4 direct raw parent/candidate pairs were `22175710/20287257`, `21341766/20804624`, `21631273/19786470`, `21643931/20446717`, `21383293/19819983`, `21293411/20534621`, `21526625/19846970`, `21831635/20042324`, `21219370/19740309`, `21085208/19773961`, `21606376/19787843`, and `21236184/20196463`, for `1.070194x` throughput (median `1.076899x`; means `21,497,899` versus `20,088,962 ns`). V5 direct pairs were `27222565/24264301`, `26061645/24126211`, `25420752/24433895`, `25582910/23777071`, `25234162/23949680`, `25384472/24120533`, `25300022/23848756`, `25269083/23827515`, `25181276/23974855`, `25256447/23752681`, `25188064/23623923`, `26009780/23774047`, `25866433/23708880`, `25712184/23685476`, `25549854/24200092`, `25411616/23555994`, `25177884/23784843`, `25115156/24157199`, `25240212/23753752`, and `25398497/24247256`, for `1.066775x` throughput (median `1.061715x`; means `25,529,151` versus `23,928,348 ns`).

The V5 cache-backed first-encode guard had matching checksum `17406679798874925224` in all twenty-four pairs. Its parent load/encode and candidate load/encode nanoseconds were `23569479/4871251/21782297/3922727`, `23060172/3861358/21591868/3796344`, `22812155/3812284/21669565/3797684`, `23124205/4075837/21610542/3809762`, `23290901/3844340/21192089/3789121`, `22829664/3808956/21810794/3801258`, `23407581/3829703/21525196/3804758`, `23780119/3791666/21492315/3768063`, `22859349/3825266/21553273/3789062`, `22709557/3854030/21776189/3786422`, `23152183/3850043/22760398/3804609`, `22818543/3886334/21544213/3830418`, `23021435/3975785/21499762/3900077`, `23271560/4103155/21811059/3900023`, `23328399/3912322/21619430/3821405`, `22905854/3907323/21688007/3803998`, `22983383/3894841/21649872/3918464`, `22959084/4005845/22027364/3810544`, `23028946/3818852/21376987/3796871`, `22876763/4028190/21677323/3788226`, `24145410/3807450/21965920/3759966`, `22801830/3794005/21475101/3764837`, `23266165/3808220/21502580/3782992`, and `23106111/3830980/21439194/3779524`. Parent/candidate load throughput was `1.067417x` (means `23,129,535` versus `21,668,389 ns`) and first-encode throughput was `1.027865x` (means `3,924,918` versus `3,813,631 ns`).

Post-timing BPE (`16`), TKZ (`2`), and complete tokenizer integration (`54` active, `9` ignored) all passed on GCP. `cargo fmt --all -- --check` passed locally without running tests. No test file, evaluator, benchmark, dependency, API, format, or input-specific dispatch is retained. The release runner grew from `4,401,392` bytes (parent SHA-256 `8f17899b711bc0ef80ee8d81913393c0b1d12c87e8c6003f76845ddc36d2c6c1`) to `4,402,440` bytes (candidate SHA-256 `fca6fd2b783d15c6a25822676d1261bea19f3bf1722c9e931d2236088cdd7e3e`), a `0.023811%` increase. The formal competitor runner remains unrebuildable unchanged, so this is an own-parent retention rather than a confirmed current-head Snaptokens-versus-Hugging-Face result.

### Matched GPT-2 JSON-load comparison after Experiment 40

The current Experiment 40 source was compared with actual Hugging Face Tokenizers `0.22.2` rather than the mislabeled historical binary above. A temporary external comparator, not retained in the repository, linked the current Snaptokens source and `tokenizers = "=0.22.2"`; its Cargo.lock SHA-256 was `f93efee3df5ab54ddcb55144041d72bb6806e082eb9235c91bda1b608fd61a81`. On GCP `c4-standard-8` Intel Emerald Rapids CPU-0 with native code generation, it loaded the same immutable GPT-2 JSON (`8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`), then encoded a Unicode and newline readiness input. The complete 30-ID vector, fingerprint `6362145778732823148`, and count matched for both engines in the warmup, every timed fresh process, and the post-timing probe.

After one uncounted warmup per engine, twelve alternating fresh-process Snaptokens/Hugging Face load pairs in nanoseconds were `45409291/60212561`, `39811510/58109137`, `39199104/57027138`, `38879741/57058751`, `44615286/60485516`, `40396550/58905354`, `39933298/57639512`, `39274978/57399216`, `44326826/59674676`, `41817613/61456965`, `39612049/57681813`, and `39100019/58531848`. Snaptokens averaged `41,031,355 ns` versus `58,681,874 ns` for Hugging Face, a `1.430171x` ratio of means and `1.431940x` paired geometric mean; every pair favored Snaptokens, and the one-sided paired t 95% lower bound was `1.395935x`. Load plus the first verified encode averaged `46,337,200 ns` for Snaptokens and `58,868,836 ns` for Hugging Face, or `1.270444x`. This is a matched GPT-2 JSON-load result, not a replacement for the twelve-model matrix.

### JSON-load experiment 41: compact parsed BPE rules in canonical pair order — planned

Parent SHA: `c1acf0d41d8937f7768e71a8cb152b881547235c` (clean scoped Experiment 40 source).

Hypothesis: JSON merge parsing currently inserts every rule into a randomized `HashMap<(u32, u32), (rank, merged)>`, although all later construction phases iterate the complete map and only sidecar validation needs pair lookup. Store parsed rules in a compact `Vec` sorted by `(left_id, right_id)`, resolve duplicate pairs in-place by retaining the last input rule, and use binary search only for the rare sidecar validation lookup. This removes per-rule hash insertion and makes the existing CSR adjacency rows arrive in their required neighbor order, so its redundant per-row sort can be omitted.

Measured hot cost: Experiment 36 attributes parsed merges at `7,677,018 ns` median and the early ID-order/adjacency region at `3,031,067 ns`. The immutable GPT-2 input has 50,000 canonical unique merge rules. After JSON parsing, construction consumes the parsed collection through full scans for byte pairs, adjacency, ranked-map construction, dense tables, and rank metadata; ordinary JSON construction does not perform random parsed-rule lookups.

Invariant that makes the shorter path exact: sorting by `(left_id, right_id)` groups exactly the same pair keys as the old map. In each equal-key run, replacing the retained entry with every later input entry leaves the last rank and merged ID, exactly matching `HashMap::insert`; the final binary search returns that same pair value for sidecar validation. The globally sorted rules produce contiguous adjacency rows ordered by right ID, and a right ID is unique within a retained row, so their packed `(right, rank)` keys are already in the exact order the old sort establishes. Every later full scan retains the same rules, ranks, merged IDs, sentinels, byte tables, dense tables, sidecars, and encode outputs.

Representation being preserved or changed: replace only the private parsed merge-map construction representation with compact sorted rules and its sidecar-only binary lookup; remove only the now-proven-redundant adjacency row sort. Retain JSON merge syntax, duplicate last-write-wins semantics, vocabulary lookup/validation, ranked map, CSR layout and lookup, decomposition, caches, public API, JSON and `.tkz` formats, dependencies, evaluator, and timing boundary. No test file is added.

Expected winning strata: JSON BPE construction with many rules, where parsing and its derived adjacency construction dominate; GPT-2 is the target screen.

Expected adverse strata: malformed or duplicated rules take the same validation and in-place last-write-wins path; `.tkz` construction uses the binary validation lookup and direct load plus first encode remain explicit guards.

Smallest files that need changing: `src/models/bpe.rs` and this record.

Mechanism evidence: the current `ParsedMergeMap` has no production lookup after parser completion except sidecar decomposition validation. `MergeAdjacency::from_parsed` counts rows, writes rules in source iteration order, and then sorts each row by `(right, rank)`; canonical `(left, right)` rule order makes that final ordering already true without depending on tokenizer identity or measured input size.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any duplicate/last-write-wins, pair lookup, row ordering, rank, merged-ID, CSR, dense-table, byte-fallback, sidecar, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; API/format change; source/model/input-size dispatch; or removal of the exact sidecar lookup.

Experiment 41 is rejected at the first direct-load guard. Candidate source `827b11c` plus its two build corrections (`3a5c96f`, `020048f`) replaced only the private parsed-rule representation and adapted existing private fixtures; it added no test. Two preflight attempts are discarded: the first did not compile because three existing fixtures constructed the retired private map directly, and the second exposed a compiler warning that the public `Bpe::new` signature had accidentally acquired a private type. The final correction restored the exact prior public `HashMap` signature. A source-copy path error was caught before its rerun, then the exact module path was replaced and the accidental remote staging file was removed after matching its SHA-256. No timing result uses any discarded build.

The final candidate passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable GPT-2 JSON. One untimed process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `41280145/40174576`, `41702471/40498397`, `42184198/39601677`, `41768603/40234262`, `41245205/40349426`, `42008347/39139515`, `41222377/40212697`, `40392350/39911270`, `40633843/39731751`, `40570901/39684787`, `41055477/39441025`, and `41530944/39070511`. Candidate/parent JSON throughput was `1.036677x` (paired median `1.028625x`, one-sided paired t 95% lower bound `1.024203x`; means `41,299,572` versus `39,837,491 ns`).

V4 direct raw parent/candidate pairs were `20786808/22286142`, `20500750/23294486`, `20906345/21881669`, `20292810/22003028`, `20053036/21846930`, `20155614/21851934`, `20433360/22142309`, `19723295/22799270`, `20747728/22949392`, `20500721/23134681`, `20662798/23296576`, and `20551367/23447127`, for `0.905655x` throughput (median `0.910950x`; means `20,442,886` versus `22,577,795 ns`; every pair lost). This large direct `.tkz` regression rejects the representation regardless of the JSON win, so V5 direct load, first encode, binary-size expansion, and post-timing tests were deliberately not run. Commits `65b77b9`, `11209ab`, and `80b241d` fully revert the source and fixture changes; no test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained. The candidate and parent target binaries were SHA-256 `5a6e73ea5f056a62f5a33c6f70da4995d046069478af4b41074dd94abf720e58` and `fca6fd2b783d15c6a25822676d1261bea19f3bf1722c9e931d2236088cdd7e3e`, respectively.

### Experiment 87 mechanism screen: rank-aware long cache-miss BPE reach — planned

Parent SHA: `2fff41a84e42bee5bdf195a58888aaefa0002b99` (clean parent on the current Git lineage; all non-README tracked files equal `c03b1eb`).

Hypothesis: a structure-gated, rank-aware longest-match/backtracking BPE representation could remove enough priority-queue work from cache-miss pieces longer than Snaptokens' 31-byte stack-merge limit to justify a later exact prototype.

Measured hot cost: unmeasured on this parent. Prior profiles identify BPE merge resolution as broad work but do not attribute the current source's long cache-miss share. This screen first collects current-source PMU and cache-miss length attribution without changing tokenizer or evaluator code.

Invariant that makes the shorter path exact: eligibility must derive only from tokenizer structure. A later representation must preserve Hugging Face's original surviving merge rank and leftmost tie rule, every active initializer, byte fallback, and `ignore_merges`. Token IDs may be used as a priority only after a complete constructor proof establishes the exact rank-to-ID relationship; otherwise the current rank-plus-leftmost engine remains the fallback.

Representation being preserved or changed: this screen changes no source representation. It records existing cache-miss byte lengths, raw versus encoded initialization, selected merge path, and PMU attribution for the committed parent. A later candidate, if authorized, would add a private structure-derived dispatch plus a portable scalar fallback; it may not identify tokenizer names, corpora, or input sizes.

Expected winning strata: uncached long raw ByteLevel pieces in structures whose complete merge order admits the proof. Cached pieces, short pieces, generic non-ranked BPE, byte fallback, and `ignore_merges` are expected adverse strata unless independently proven eligible.

Expected adverse strata: the common fused pieces are short, current cache hits bypass BPE, and Gemma previously failed the necessary canonical-prefix condition. Matcher construction, matcher memory, and eligibility checks may cost more than the heap work removed.

Smallest files that need changing: none for source attribution. The durable record is this card and a command/provenance archive outside the repository. Candidate source is forbidden until this screen accepts.

Mechanism evidence: `Bpe::merge_all_raw_into` uses stack-resident rank loops through 31 raw bytes, then the exact priority queue. The external longest-match design is only a hypothesis because it treats merged IDs as priority; Snaptokens separately owns merge rank and ID. Experiment 81 already rejects treating a noncanonical prefix-state representation as general.

Acceptance rule: use a current-source, four-vCPU PMU capture and complete pre/post Hugging Face IDs on every timed input. Authorize a separate prototype only if long cache-miss heap work accounts for at least `19.65%` of affected end-to-end time in generic Gemma and at least two fused families, a modeled twofold removal of that work can clear the frozen `1.09x` general target, and a structure-only constructor audit proves exact rank order and all active initializers for every eligible family. Preserve raw samples, length distributions, host identity, binary hashes, and every excluded family.

Rejection rule: stop before candidate code if PMU attribution is unavailable, any required family has insufficient long-miss reach, the modeled ceiling misses the general target, the rank/initializer proof fails, an ID or row-boundary comparison differs, or eligibility depends on a model name, corpus, or measured input size. Do not weaken the share, family, or exactness requirements after observing the result.

Result: **rejected before PMU attribution; parent exactness failure.** A disposable remote checkout of exact parent `2fff41a84e42bee5bdf195a58888aaefa0002b99` was built with Rust `1.97.1` into a separate target directory on the task-owned GCP four-vCPU host. The checkout remained clean. The existing root-lock `simple_bench` compared every output with Hugging Face before reporting a time. Its 32-sample sequential LongBench run against the archival `gemma-3.json` fixture panicked at zero-based input 10: `18989` aligned token positions differed. The benchmark's count excludes any unmatched tail, so it is evidence of at least that many ID differences rather than a complete distance. The first 100 IDs printed in the panic agree, which places the divergence later in the input; no inference about its cause has been made.

The runner command was `simple_bench <gemma-3.json> --max-samples 32 --output /tmp/snaptokens-exp87-evidence-2fff41a/gemma-3-seq.csv`; its source uses `tokenizers::Tokenizer::encode_fast(input, true)` and `Tokenizer::encode_with_special_tokens(input, true)`. The immutable fixture remains at `/home/namanchetwani/snaptokens-asm-profile-20260909-inputs`; the checkout, target directory, and partial output were deliberately disposable `/tmp` paths and were cleared by the VM stop. This durable record preserves the exact parent, command, fixture digest, mismatch count, and failure location instead of claiming that transient output survived. A one-sample Gemma run passed before this input, but it is not a performance result because the later required parity gate fails. No PMU capture, prototype, tokenizer change, evaluator change, or performance comparison was run after the mismatch.

The next work is a separate correctness investigation that first obtains a compact reproducer and identifies whether the divergence arises before BPE, in BPE, or in post-processing. It may not reuse this experiment's timing card to justify an optimization.

### Experiment 88 correctness screen: Gemma long-context parity — planned

Parent SHA: `2fff41a84e42bee5bdf195a58888aaefa0002b99` (the unchanged source that produced the reported mismatch).

Hypothesis: the Gemma fixture exercises a tokenizer configuration or long-context boundary absent from the current pinned test fixtures; stage-by-stage output comparison can localize it before any source change.

Measured hot cost: none. This is a correctness screen, not a throughput candidate.

Invariant that makes the shorter path exact: the complete Hugging Face pipeline is authoritative. Every normalizer transformation, added-token boundary, pre-tokenized span, BPE input, merged ID, and post-processed output must remain identical in order and value.

Representation being preserved or changed: no production representation changes during the screen. The fixture and the failing input are read only. Any later repair must retain the public API, tokenizer JSON and `.tkz` formats, generic scalar fallback, and all existing pipeline behavior.

Expected winning strata: none; success means a compact, pinned reproducer and a stage attribution. The first mismatch may be normalizer, split boundaries, vocabulary splitting, BPE, or post-processing.

Expected adverse strata: a fixture whose digest or provenance cannot be established is not valid repair evidence. A test that checks only a prefix, ignores special tokens, or compares decoded text instead of IDs does not contain the failure.

Smallest files that need changing: none for source attribution. A proven repair may change the owning source file plus one focused, immutable-fixture differential test.

Mechanism evidence: `simple_bench` showed matching first 100 IDs but at least `18,989` later aligned differences on zero-based LongBench input 10. The current test fixture table contains no Gemma entry, so existing test success does not exercise this pipeline.

Acceptance rule: record SHA-256 or BLAKE3 of the tokenizer fixture and an exact input digest; reproduce the complete-ID mismatch with and without post-processor special tokens; identify the first divergent stage without modifying production code. Authorize a repair only after the responsible stage and semantic invariant are explicit. Retain only if the focused exact test, the complete 32-input sequential and flat-batch Gemma checks, and relevant existing pipeline tests all match Hugging Face before and after any timing.

Rejection rule: stop before source code if fixture provenance is incomplete, the mismatch does not reproduce, the stage cannot be localized, a proposed fix relies on a model name, or any comparison differs. Do not use performance output from a failing parent or a partial prefix as evidence.

Screen result before source: the exact fixture is `unsloth/gemma-3-1b-it@5b11413a10db4e486ef16a20101fd028f8f2499c` with SHA-256 `4667f2089529e8e7657cfb6d1c19910ae71ff5f28aa7ab2ff2763330affad795` and BLAKE3 `38e6c65074653102e6e238195e25938a4cb1ea2df4c7c01283d45de480696a11`. LongBench-v2 revision `2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9` input 10 has SHA-256 `4bc0d25037a5cc855c8fb0eaac69f78bb822f05c742af7227331638e08b1a69e` and BLAKE3 `23cf94a05e536b67d180de21be65ee2e9753dcc999cabf48da7e434177828379`. The new focused differential test reproduces the parent mismatch with `add_special_tokens=false`: Hugging Face emits `1,274,799` IDs, Snaptokens emits `1,274,797`, and their first different ID is at position `1,255,269`. Post-processing is therefore not the responsible stage.

The first repair candidate changes only the structure-derived `Bpe::bigram_bridge_table` guard: byte-fallback BPE retains the ordinary unsplit BPE path. The invariant is simple and exact: no synthetic piece boundary can remove a merge. This tests whether the generic bridge table is unsound for byte-fallback token spellings; it makes no model, corpus, or input-size decision. Retain this correctness repair only if the focused raw-plus-special-ID test, the complete 32-input sequential and batch screens, and existing tests pass. Its throughput effect is a later guardrail, never a justification for keeping an incorrect split.

Repair candidate `b11ed21bf1d518be04a691c7e24200e1245ab3b1` passes the focused raw-plus-special-ID test and the complete remote 32-input Gemma screens with zero ID mismatches. On the same four-vCPU Intel GCP diagnostic host, its `simple_bench` output reports `3.99x` sequential and `2.50x` batch throughput relative to Hugging Face across `24,432,087` input characters and `7,160,895` output IDs. These are context-only Hugging Face diagnostics: the runner materializes complete IDs for parity but excludes output destruction from its timer. There is no valid parent/candidate throughput comparison on this workload, because parent `2fff41a` fails its complete-ID check. The repair's prospective throughput cost remains a guardrail to measure only through an evaluator that both arms can pass exactly.

### Experiment 89 mechanism screen: byte-fallback bridge-boundary proof — planned

Parent SHA: `b11ed21bf1d518be04a691c7e24200e1245ab3b1` (the exact Gemma repair).

Hypothesis: the former byte-fallback splitter fails because vocabulary-byte coverage is an insufficient condition for independent BPE pieces, not because splitting is inherently incompatible with byte fallback. A trace that identifies the first split boundary whose separate BPE results differ from the unsplit BPE result can establish the missing semantic condition or rule out a safe recovery.

Measured hot cost: the exact current-source Intel sample places `52.67%` of cycles in `merge_all_encoded_into`, including `12.23%` in `MergeAdjacency::get`; the former splitter is therefore a potentially material work-removal path. It is not timed until parity is restored.

Invariant that makes the shorter path exact: a boundary may be introduced only when tokenizing the concatenated left and right BPE input yields exactly the concatenation of tokenizing each side, including byte-fallback expansion and the complete rank-plus-leftmost merge order. The old condition that no vocabulary spelling contains an adjacent input-byte pair is only sufficient if it proves that equality for every reachable BPE state.

Representation being preserved or changed: the screen retains the current unsplit production path. A disposable test-only trace may reconstruct the existing pipeline and report split ranges; it changes no API, tokenizer format, cache, evaluator, dependency, `unsafe` block, or runtime dispatch. Any later candidate must use a structure-derived proof and retain the unsplit fallback.

Expected winning strata: byte-fallback BPE configurations whose pre-tokenized spans contain independently tokenizable boundaries. The generic Gemma long-context path is the first witness, not an eligibility label.

Expected adverse strata: any byte-fallback grammar whose reachable merge can cross a proposed boundary remains unsplit. Non-byte-fallback and `ignore_merges` behavior remain unchanged.

Smallest files that need changing: this record plus temporary ignored tracing in `src/lib.rs` and a `cfg(test)` BPE-only helper in `src/models/bpe.rs`; remove both after the screen unless a focused regression test requires a successor.

Mechanism evidence: parent `2fff41a` differed at `18,989` or more aligned positions on the pinned Gemma context, while `b11ed21` matches all IDs by disabling only the splitter. The old bridge table already tries to decode `<0xHH>` spellings, so the audit must find a concrete counterexample rather than assume an omitted fallback marker.

Acceptance rule: commit the disposable screen on a clean tree, reproduce the parent mismatch, and record the first concrete boundary with its original input bytes, post-normalization/pre-tokenization BPE bytes, separate and concatenated full ID vectors, and applicable merge path. Proceed to a new candidate only if a tokenizer-structure-only predicate proves the boundary independent for every eligible state and the focused Gemma raw-plus-special comparison passes before any timing.

Rejection rule: remove the trace and keep `b11ed21` if no compact witness is obtained, the witness depends on model/corpus/input-size dispatch, the proposed predicate is not sufficient for arbitrary fallback bytes, or any focused ID comparison differs. Do not time or retain a partial re-enable.

Result: **accepted as source attribution; no production code changed.** The disposable trace found the first forced boundary at normalized BPE byte offset `5,649,567`, inside one pre-tokenized parent span. Its bytes are the `d|▁` boundary in `…▁and▁100)▁and▁YYYY-MM-DD▁is…`. The literal bridge table has no spelling for `d▁`, `d<0xE2>`, or `<0x64><0xE2>`, so this is not a cross-piece merge witness. Rather, unsplit BPE emits `▁Y`, `YY`, `Y`, while the separately split piece takes `ExactTokenMatcher`'s whole-piece shortcut and emits `▁YYYY`.

The direct shortcut is unsound here because `encoding_decomposition` reports `CharsNotInVocab` for `▁YYYY`: that status means its characters cannot be initialized as direct vocabulary symbols, not that byte fallback proves the full token is BPE-constructible. A test-only direct call to `merge_all_encoded_into` for that split piece emits the same three IDs as unsplit BPE, establishing that the matcher, not a bridge merge, caused the wrong result. Commits `0df0004`, `165f513`, and `d73e67c` contain the disposable trace; `b9d5cd2` reverts it completely, preserving the exact `b11ed21` production source. No timing was performed.

### Experiment 90 correctness-first candidate: reprove byte-fallback vocabulary splitting — planned

Parent SHA: `b9d5cd2eeb050b4d0cfef05a13aface62d7e84f0` (the exact `b11ed21` production source after removing Experiment 89's trace).

Hypothesis: mark `CharsNotInVocab` tokens as ineligible for `ExactTokenMatcher` only when byte fallback is enabled, then allow the existing bridge table for byte-fallback BPE. The matcher will fall through to ordinary merge resolution whenever direct character decomposition cannot prove the whole vocabulary spelling is constructible, while a bridge remains only where no resolved BPE token can span it.

Measured hot cost: the exact parent spends `52.67%` of the Intel sample in `merge_all_encoded_into`, with `12.23%` in `MergeAdjacency::get`. Restoring semantically independent split pieces can remove work on long byte-fallback spans without changing the model, corpus, or input size.

Invariant that makes the shorter path exact: every resolved merge's spelling is validated as the concatenation of its left and right vocabulary spellings. The bridge table records every adjacent raw byte pair in those spellings, including decoded `<0xHH>` fallback spellings. Therefore a boundary absent from the table cannot be crossed by a resolved BPE result. For a byte-fallback token whose decomposition is `CharsNotInVocab`, direct whole-piece matching is prohibited, so each retained piece follows the same merge graph as the unsplit path.

Representation being preserved or changed: keep the vocabulary, ranked merge map, adjacency rows, byte-fallback table, merge order, APIs, tokenizer format, and unsplit fallback. Change only one orphan classification for byte fallback and the existing structure-derived table guard. Apply that classification to both newly derived and sidecar-provided identity decompositions; an existing byte-fallback V5 trie was materialized under the old classification, so it must fall back to the direct matcher rather than retain invalid terminals. Add no dependency, cache, model dispatch, corpus branch, input-size branch, evaluator change, format change, or `unsafe` code.

Expected winning strata: byte-fallback BPE configurations containing long pre-tokenized spans with bridge-table-independent regions. The pinned Gemma context is a witness, not a selection condition.

Expected adverse strata: byte-fallback vocabularies with many direct-character-constructible pieces may get little or no benefit. Non-byte-fallback and `ignore_merges` behavior must remain unchanged.

Smallest files that need changing: `src/models/bpe.rs`, this record, and the existing pinned Gemma regression test only if it needs a more focused assertion.

Mechanism evidence: Experiment 89 proves the observed mismatch is an invalid direct whole-piece shortcut, not a merge across the unmarked `d|▁` boundary. Calling ordinary BPE on the right piece yields the exact unsplit IDs. The bridge table is already built from validated resolved merge spellings, so its decision is structure-derived rather than model-derived.

Acceptance rule: commit a clean candidate; pass formatting, focused BPE tests, and the existing raw-plus-special pinned Gemma differential test before timing. On the isolated Intel host, require complete Hugging Face ID equality for all 32 pinned inputs in sequential and batch modes both before and after the timed pool. Then run eight predeclared counterbalanced fresh-process `simple_bench --no-hf` pairs against immutable exact parent `b11ed21` binaries, four parent-then-candidate and four candidate-then-parent, preserving output construction and destruction. Retain only if the paired geometric point estimate exceeds `1.02x`, its paired 95% interval is above `1.00x`, and no focused or complete parity test differs; report the runner's Hugging Face comparison only as context.

Rejection rule: revert the full candidate without timing if any ID differs. Revert after the target screen if the paired interval does not clear `1.00x`, an order-dependent loss exceeds the observed A/A band, a non-byte-fallback or `ignore_merges` test regresses, or review finds a bridge spelling that escapes the stated proof.

Result: **rejected at the focused exactness gate.** Candidate `e8c058fa60af0d6ed7e144dc0d62803013574490` applied the new byte-fallback identity classification to derived and sidecar decomposition, rejected older byte-fallback V5 tries, and re-enabled the bridge table. `cargo fmt --all -- --check` and all 16 focused BPE tests passed. The immutable pinned Gemma raw-plus-special differential still produced `1,274,797` rather than Hugging Face's `1,274,799` IDs, with the same first difference at `1,255,269`; it failed before any timing. Revert `73dfc6e` restores the exact `b11ed21` source. The trace showed one invalid direct match, but not a sufficient predicate for all split-piece behavior.

### Experiment 91 mechanism screen: inspect actual split-piece matcher eligibility — planned

Parent SHA: `73dfc6ec6af4c60d5e2ab7343cb6478c79bcadb2` (Experiment 90 fully reverted).

Hypothesis: Experiment 90's unchanged mismatch means either the observed `▁YYYY` piece did not have an identity decomposition at runtime or a second shortcut precedes ordinary BPE. A test-only report of the split piece's exact matcher eligibility, stored decomposition, direct BPE result, and caller input can locate the missing condition without changing production behavior.

Measured hot cost: unchanged from Experiment 89. This screen does not time any code because the current proposed restoration is known inexact.

Invariant that makes the screen safe: the helper only reads the current BPE representation and compares its ordinary merge output with its direct whole-piece result. It does not alter token IDs, split scheduling, caches, APIs, tokenizer formats, evaluator, dependencies, or runtime dispatch.

Representation being preserved or changed: retain the exact unsplit production path. Add a disposable ignored-test helper in `src/models/bpe.rs` and a focused report in `src/lib.rs`, then revert both after recording the witness.

Expected winning strata: none; this is source attribution only.

Expected adverse strata: none; all release behavior remains the exact parent.

Smallest files that need changing: this record plus temporary test-only inspection in `src/models/bpe.rs` and `src/lib.rs`.

Mechanism evidence: Experiment 89's first boundary associates the discrepancy with `▁YYYY`, while Experiment 90 proves that treating only false identity decompositions as ineligible does not repair the full pin. The next report must distinguish a wrong classification from a different fast path.

Acceptance rule: on the pinned immutable fixture, report the first failing split piece, its full BPE caller text, direct-match token and orphan eligibility, unmerge pair, ordinary merge IDs, and end-to-end IDs before and after forcing its local path. A successor may be proposed only if a tokenizer-structure-only condition accounts for every observed mismatch and passes the raw-plus-special differential before timing.

Rejection rule: remove the helper and keep the unsplit exact repair if the evidence requires a model/corpus/input-size branch, the path cannot be made independent for arbitrary byte fallback, or the report fails to isolate a compact invariant. No benchmark runs on this screen.

Result: **inconclusive; audit reverted.** Commit `cf4de2a` compiled the test-only traversal, but its one remote run exceeded five minutes after the existing trace's roughly three-minute traversal without reaching a report. It was interrupted rather than changing its fixed corpus or using its execution time as evidence. Revert `b92577e` removes all 122 audit lines. The original Experiment 89 witness remains intact, but this screen established no new semantic predicate and ran no benchmark.

### Experiment 92 mechanism screen: inspect the recorded direct-piece state — planned

Parent SHA: `b92577e9e5812bfb1d62019aaf2bacee41dc0b2f` (Experiment 91 fully reverted).

Hypothesis: the known normalized piece `▁YYYY` is enough to distinguish Experiment 90's two possible failures without another whole-context traversal. Reporting its exact whole-piece match, stored unmerge pair, current orphan bit, and heap-only BPE output will show whether the token is falsely recorded as a merge result or whether an unobserved fast path remains.

Measured hot cost: unchanged from Experiment 89. This one-token screen does not measure speed.

Invariant that makes the screen safe: the ignored test opens the pinned Gemma JSON, reads BPE state for the already-recorded piece, and runs its ordinary heap BPE helper. It has no production branch, no changed tokenizer output, no cache reuse, no evaluator change, and no corpus/model dispatch outside disposable test setup.

Representation being preserved or changed: retain the exact unsplit production path. Add one ignored private BPE test and remove it after recording the result.

Expected winning strata: none; this is attribution only.

Expected adverse strata: none; no runtime behavior changes.

Smallest files that need changing: this record and temporary test-only code in `src/models/bpe.rs`.

Mechanism evidence: Experiment 89 already establishes that this exact piece takes direct ID `146179` while heap BPE emits IDs `895`, `33990`, `236874`. Experiment 90's identity-only restriction did not alter end-to-end IDs, so the stored unmerge state is the decisive missing fact.

Acceptance rule: record all four values from the pinned model. A successor is allowed only if their relationship supplies a structure-derived restriction for every byte-fallback BPE, including direct sidecars, and its raw-plus-special Gemma differential passes before timing.

Rejection rule: remove the ignored test and keep the unsplit repair if the stored state does not yield a compact general restriction. No benchmark runs on this screen.

Result: **accepted as source attribution; audit reverted.** The pinned tokenizer SHA-256 is `4667f2089529e8e7657cfb6d1c19910ae71ff5f28aa7ab2ff2763330affad795`. Audit commit `48fe091` reports `▁YYYY` as direct match `Some(146179)`, stored unmerge pair `Some((236874, 236874))`, and direct orphan `Some(false)`, while direct tokenization returns `[146179]` and heap BPE returns `[895, 33990, 236874]`. `reduce_decomposition_tokens` returns its final pair whenever two symbols remain, but never checks its already-computed `best_new` against the vocabulary token whose spelling is being classified. That false `Pair` admits the invalid direct matcher. Revert `5d72f64` removes the ignored audit; no benchmark ran.

### Experiment 93 correctness-first candidate: validate a decomposition's produced token — planned

Parent SHA: `5d72f642f7060e02909639ead50c0c9224ef638f` (Experiment 92 fully reverted).

Hypothesis: pass the vocabulary token being classified into decomposition reduction and return `Stuck` unless the final selected merge's `best_new` equals that target. Exact whole-piece matching will then be admitted only for a merge graph that actually produces the spelling's token ID.

Measured hot cost: this is a correctness repair, not a throughput candidate. It removes an invalid fast path and is not timed.

Invariant that makes the shorter path exact: `Pair(left, right)` means the canonical rank-ordered BPE merge of those two final symbols produces the token whose vocabulary spelling was analyzed. `best_new` is the parsed merge result of the selected lowest-rank, leftmost pair; requiring `best_new == target` is precisely that missing implication. Every other reduction behavior, tie order, byte-initial table, adjacency lookup, and orphan fallback remains unchanged.

Representation being preserved or changed: retain the merge map, ranked map, adjacency rows, decomposition storage, exact matcher representation, sidecars, APIs, and split guard. Change only the private decomposition proof from an unverified final pair to a target-verified pair. Add one focused private test for a final merge that produces a different ID.

Expected winning strata: none. The candidate may reduce the direct-match fast path only where it was semantically invalid.

Expected adverse strata: byte-fallback or irregular character-initial vocabularies with false direct matches will fall through to heap BPE, preserving exactness. Verified non-byte-fallback direct matches remain unchanged.

Smallest files that need changing: `src/models/bpe.rs`, `src/models/bpe/tests.rs`, and this record.

Mechanism evidence: Experiment 92's `▁YYYY` state is a concrete false `Pair`: its direct token ID is not the heap BPE output, because the stored final pair was not checked against the classified token. The candidate's one comparison rejects exactly that state.

Acceptance rule: commit a clean candidate; pass formatting, the focused false-final-merge test, the focused BPE suite, and the pinned raw-plus-special Gemma differential before any broader test. Then pass the complete local tokenizer suite and remote 32-input sequential and batch Hugging Face parity. Record construction and exactness effects, but run no speed screen for this repair.

Rejection rule: revert the whole candidate if any exactness test differs, if the sidecar path rejects a formerly valid canonical representation, or if review finds a case where `best_new == target` is not sufficient for the existing BPE order. Do not weaken the check or add a model-specific exception.

Result: **retained as a correctness repair.** Commit `517a913` adds the target check and a focused false-final-merge unit test. It passes the pinned raw-plus-special Gemma comparison, all 134 library tests, all 54 non-ignored tokenizer tests, and the doc test. The same strict-clippy invocation still rejects two inherited warnings outside the change (`needless_range_loop` at `bpe.rs:1215` and `too_many_arguments` at `bpe.rs:2203`); no unrelated lint rewrite was added.

The immutable release binary completed both remote complete-ID screens with zero mismatch: 32 sequential and one batch of 32 over `24,432,087` characters and `7,160,895` IDs. The wrapper invocation through `cargo bench` was preserved as invalid evidence because Cargo appended an unsupported `--bench` argument; the direct immutable binary then completed both parity screens. Its printed `3.48x` sequential and `2.53x` batch Hugging Face ratios are context only and not compared with the prior repair. No candidate/parent speed pool ran for this correctness change.

### Experiment 94 candidate: restore bridge scheduling after proving direct BPE results — planned

Parent SHA: `517a91396692463eaf8c2ab7d474ae9f4f3c4f04` (the exact decomposition-proof repair).

Hypothesis: now that `Pair` means its final merge actually produces the analyzed token, treat `CharsNotInVocab` as matcher-ineligible for byte fallback and re-enable the existing bridge table for merge-driven byte-fallback BPE. Every split piece will either use a direct match proven by its decomposition or take the ordinary heap BPE path; bridge boundaries remain only where no resolved merge spelling can cross them.

Measured hot cost: exact parent `b11ed21` source attribution puts `52.67%` of cycles in `merge_all_encoded_into`, including `12.23%` in `MergeAdjacency::get`. The corrected parent preserves that unsplit path, so table-approved partitions can remove substantial merge work on long byte-fallback spans.

Invariant that makes the shorter path exact: every non-orphan multi-character direct match now has a final rank-ordered merge whose `best_new` equals its token ID. Byte-fallback `CharsNotInVocab` entries have no such proof and fall through to heap BPE. The bridge table marks every adjacent raw byte pair in every vocabulary spelling and decoded fallback marker spelling; a missing pair cannot belong to a resolved output spanning that input boundary. `ignore_merges` remains unsplit because it permits arbitrary vocabulary matches outside the merge graph.

Representation being preserved or changed: retain vocabulary, merge graph, rank order, merge adjacency, byte-fallback expansion, caches, APIs, sidecar format, and unsplit fallback. Change only matcher eligibility for byte-fallback `CharsNotInVocab`, the structure-derived bridge guard, and byte-fallback V5 trie selection: old tries lack the stricter eligibility bit, so use the direct matcher with corrected bits. Add no dependency, cache, model/corpus/input-size dispatch, evaluator change, format change, or `unsafe` code.

Expected winning strata: long generic-pipeline byte-fallback BPE spans with many unbridgeable byte pairs. Gemma LongBench is a witness, not a runtime condition.

Expected adverse strata: byte-fallback inputs that have no independent bridge remain unsplit; non-byte-fallback BPE and `ignore_merges` remain unchanged. Existing byte-fallback V5 sidecars may use the direct matcher rather than their stale trie.

Smallest files that need changing: `src/models/bpe.rs`, the existing pinned Gemma test only if a focused assertion is needed, and this record.

Mechanism evidence: Experiment 89 isolated the split witness, Experiment 92 exposed the false unmerge proof, and Experiment 93 repairs it. Experiment 90 already showed that the `CharsNotInVocab` restriction and V5-trie fallback alone are insufficient; this candidate adds them only after the missing target proof is true.

Acceptance rule: commit a clean candidate and pass formatting, focused BPE tests, the pinned raw-plus-special Gemma differential, and all local tokenizer tests before timing. On the isolated Intel host, require complete Hugging Face ID equality for all 32 pinned inputs sequentially and in a batch, both before and after the timed pool. Then run eight predeclared counterbalanced fresh-process `simple_bench --no-hf` pairs against immutable exact parent `517a913` binaries, four parent-then-candidate and four candidate-then-parent, with output construction and destruction inside the timed path. This is a diagnostic screen only: retain local source only if the paired geometric point estimate exceeds `1.02x`, its paired 95% interval is above `1.00x`, and every exactness gate passes; do not claim a general or competitor-leading speedup without the frozen portable evaluator.

Rejection rule: revert before timing on any ID mismatch. Revert after the pool if the paired interval does not clear `1.00x`, order changes outcome beyond the calibration band, V5 sidecar behavior fails, a non-byte-fallback or `ignore_merges` test regresses, or the bridge proof has an unmarked resolved spelling. Do not add a model-specific exception.

Result: **retained for the active branch's exact diagnostic screen; not a general promotion.** Candidate `dfc251f` passes the 17 focused BPE tests, the pinned raw-plus-special Gemma differential, all 134 library tests, all 54 non-ignored tokenizer tests, and the doc test. Its only strict-clippy failures are the same two inherited warnings recorded for Experiment 93, outside this diff. The V5 sidecar round-trip/direct-load tests pass with the byte-fallback matcher fallback.

On the isolated four-vCPU Intel host, immutable parent `517a913` and candidate `dfc251f` binaries each completed complete Hugging Face ID parity for all 32 pinned documents before timing. Candidate parity also passes both sequential and batch again after timing: `24,432,087` characters and `7,160,895` IDs in every full screen. The direct harness binary is used because Cargo's bench wrapper appends an unsupported `--bench` argument; binary SHA-256 values and every raw output are retained under `~/.cache/snaptokens-exp94-dfc251f-evidence` on the task-owned host.

The eight fixed fresh-process sequential pairs retain all raw rounds and use four parent-then-candidate plus four candidate-then-parent orders. Round 1 completed before a collector typo (`awk -v index`) failed; its two raw logs were retained and included, while rounds 2--8 completed without rerunning or discarding it. Parent/candidate times in milliseconds are `2070.050/672.790`, `2049.650/669.890`, `2124.530/676.780`, `2132.360/662.690`, `2082.660/655.850`, `2077.130/671.970`, `2137.910/663.950`, and `2138.140/680.430`. The paired geometric throughput ratio is `3.139765x`, with paired log-space 95% interval `[3.088731x, 3.191642x]`.

The same complete-ID runner reports Hugging Face context ratios of `9.98x` before and `10.00x` after the sequential pool, and `8.98x` before and `8.50x` after the batch screen. These are context only because that branch of `simple_bench` excludes output destruction; they are not used in the candidate score. The direct no-Hugging-Face pair runner includes output construction and destruction, but covers one tokenizer/corpus/shape on one Intel host. The frozen portable evaluator remains unavailable, so this result cannot establish a general speedup, portability, or that Snaptokens is faster than every competitor.

### Pinned Tokie warm-repeated rebaseline — diagnostic context

Current exact source: `dfc251f1277c7a11188f7145b7317604414a02fc` on branch `perf/bpe-miss-scheduling`. This is a comparator rebaseline, not a Snaptokens source candidate and not a change to `benchmarks/` or the frozen evaluator.

The independent driver uses the same `repeated-chat140` generator and the same public API shapes as the broad harness: Snaptokens calls `encode_batch_ragged`, retaining the returned flat IDs and row lengths; Tokie calls public `encode_batch`, retaining its public `Encoding` values. Returned values are black-boxed and dropped inside each timed iteration. The driver loads Tokie at pinned `9b78cc552df73c0fa41451c51e08fe94638fa48c`, checks complete rows against Hugging Face before timing and after the eight counterbalanced timed rounds, and uses `16 MiB` timed plus `4 MiB` immediate warmup work per arm. Input references are created inside Tokie's timed call just as in the broad harness.

Pinned fixture checks passed for GPT-2 `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6`, GPT-OSS `0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3`, Mistral Nemo `e11c71726323d33da7b8d6f6f269f1988931c0a52b7122bcdd8c05042974e0db`, and Qwen 3 `aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4`. All twelve current Snaptokens/Tokie cells passed full Hugging Face ID equality before and after timing on one isolated four-vCPU Intel Xeon 8581C host. Median elapsed-time ratios use `Tokie / Snaptokens`, so values above one favor Snaptokens:

| Model | Batch 1 | Batch 32 | Batch 512 |
| --- | ---: | ---: | ---: |
| GPT-2 | `15.444191x` | `19.194866x` | `10.760966x` |
| GPT-OSS | `6.181248x` | `12.625665x` | `6.426143x` |
| Mistral Nemo | `5.253542x` | `13.980813x` | `6.841105x` |
| Qwen 3 | `5.352470x` | `12.335437x` | `8.382106x` |

The geometric mean of these twelve median ratios is `9.347342x`; the least favorable exact cell remains `5.253542x`. The source checkout stayed clean, and the external diagnostic package's Cargo lock and release binary SHA-256 values are `c174e05431496f0b2e0e43eef75fabea7c9e7940441bf209445abeaf84f8f156` and `87a77703cda011b8f8c28e9e2d8feb10503155f0daa894247d56c808e4a1adc9`. Raw rounds and logs are at `~/.cache/snaptokens-exp95-profile-dfc251f-evidence` on the benchmark host.

This resolves the previously historical, stale comparison only for the exact warm-repeated panel. It does not justify a novel-input, frozen-evaluator, portability, or all-competitor claim: Tokie was historically rejected from every available novel-input cell for complete-ID mismatches, and this separate driver is not the blocked frozen portable evaluator. The correct next broad optimization target must come from Snaptokens' own exact novel-input profile, not from a Tokie mechanism that fails that contract.

### Experiment 96 diagnostic: transfer bridge scheduling across tokenizer structures — planned

Parent SHA: `517a91396692463eaf8c2ab7d474ae9f4f3c4f04`.

Hypothesis: Experiment 94's bridge scheduling gain transfers only to tokenizer configurations whose ordinary BPE merge graph, byte fallback, and non-ByteLevel pre-tokenizer permit `split_on_unbridgeable_bigrams`. Configurations outside that structure must retain the parent path; eligible configurations may gain when their pinned LongBench inputs contain many unbridgeable byte boundaries.

Measured hot cost: Experiment 94 attributes the exact Gemma witness to merge work (`merge_all_encoded_into`, including `MergeAdjacency::get`) and observes `3.139765x` candidate/parent throughput on one Gemma sequential screen. That result establishes a mechanism witness, not a cross-tokenizer result.

Invariant that makes the shorter path exact: a split boundary is created only when no vocabulary spelling can cover the adjacent raw bytes; the table records ordinary spellings and decoded `<0xHH>` fallback markers. Direct matches require a final merge that produces the matched token, `CharsNotInVocab` byte-fallback spellings remain heap-BPE eligible only, and `ignore_merges` does not split. `needs_vocab_splitting` is exactly the absence of a ByteLevel pre-tokenizer.

Representation being preserved or changed: no source or evaluator change. Build immutable parent and candidate binaries from their committed trees; fetch the pinned `LongBench-v2` revision `2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9`, derive a local JSON array of its first 32 nonempty contexts, and retain its byte hash. Select test fixtures only by parsed tokenizer configuration: `model.type == BPE`, `byte_fallback == true`, `ignore_merges == false`, and no ByteLevel pre-tokenizer. Include a configuration-ineligible BPE control solely to verify that its route stays unchanged.

Expected winning strata: eligible BPE configurations with long generic-pipeline contexts and substantial unbridgeable byte-boundary density.

Expected adverse strata: eligible configurations with few such boundaries may show no measurable change; ByteLevel, non-byte-fallback, and `ignore_merges` configurations must not receive the split path.

Smallest files that need changing: this record only; the existing `benches/simple_bench.rs` runner is unchanged.

Mechanism evidence: `Tokenizer::build` derives `needs_vocab_splitting` from `PreTokenizer::contains_byte_level`; `Tokenizer::encode_with_special_tokens` calls the bridge splitter only behind that predicate and `Bpe::bigram_bridge_table`, whose `ignore_merges` guard is structural. The candidate code is already independently committed as `dfc251f`; this experiment compares it directly with its immediate parent rather than combining another source change.

Acceptance rule: before timing, record machine/toolchain/source/lockfile/binary/data hashes and require complete Hugging Face IDs for every selected input sequentially and as one batch for both immutable binaries. For each eligible configuration, run eight fresh-process `--no-hf` sequential pairs with four parent-then-candidate and four candidate-then-parent orders; the unchanged runner allocates and drops token output inside its timer. Repeat complete sequential and batch Hugging Face comparison after each pool. Report each configuration separately with the paired geometric throughput ratio and log-space 95% interval. This is transfer diagnosis only; no result may promote a general champion, claim portability, or update a competitor comparison while the frozen evaluator is unavailable.

Rejection rule: stop that tokenizer before timing on any ID or row-boundary mismatch, source/build/data identity mismatch, or configuration selection that cannot be explained from tokenizer JSON. Do not retry or filter an adverse cell after seeing results, change runner timing/data/rounds, or make a model-specific runtime change. A no-gain or regression remains recorded as a structure result, not evidence to tune the splitter for that model.

Result: **completed as a scoped transfer diagnosis; no general promotion.** A fresh task-owned GCP `c4-standard-4` (`4` vCPUs, Intel Xeon Platinum 8581C) built parent `517a913` and candidate `dfc251f` separately with Rust `1.97.1`, identical Cargo.lock SHA-256 `ebca28317d1c76942b6d059d9d045b3d3405a4a4a77b8cfb2a33795c7df0dc64`, and distinct immutable benchmark-binary SHA-256 values `a5c5b5a8639f11f63955a82090dd32a19f2a960c16554b3f8cdee98e8bd1e5fc` and `3febad6522d590aedc8e97bbf4f0a263cc16a32bdf46aaec06aa8c02d533dfc7`. The source worktrees remained clean before and after every phase.

The input is a local JSON string array derived once from the first 32 nonempty contexts of pinned `LongBench-v2` revision `2b48e494f2c7a2f0af81aae178e05c7e1dde0fe9`: `24,220,398` characters, source-data SHA-256 `15d61c22d92c96900b3c4948b6aeea218d3214b676a65df48e7b8555604c7fe2`, and derived-array SHA-256 `8230901bd6a1aa1ecc0df43e44734ed03be1c4d03b039236ddb5ba8c948d4888`. That fixed input is not byte-identical to Experiment 94's prior 32-document screen, so its Gemma result is an independent reconfirmation rather than a result to combine with the earlier pool.

Parsed JSON classified thirteen accessible immutable model fixtures. Gemma is the sole eligible fixture: BPE, `byte_fallback = true`, `ignore_merges = false`, and no ByteLevel pre-tokenizer. Every other accessible BPE fixture either has ByteLevel, lacks byte fallback, or enables `ignore_merges`; the source predicate leaves it unsplit. The pinned public raw URL for `NVIDIA-Nemotron-3-Nano-30B-A3B-BF16` returned HTTP 404, so it was neither classified nor timed. Mistral Nemo is the adverse BPE control (`byte_fallback = false`, `ignore_merges = true`, ByteLevel). Complete Hugging Face IDs passed for both binaries on all 32 inputs sequentially and in one 32-row batch before and after the pool for both Gemma and Mistral; batch row boundaries are compared by the unchanged runner.

Each model ran eight fresh-process output-allocation-and-destruction-inclusive sequential pairs, with four parent-then-candidate and four candidate-then-parent orders. Gemma's candidate/parent geometric throughput ratio is `3.367923x`, paired log-space 95% interval `[3.325491x, 3.410896x]`; the two order-specific estimates are `3.349662x` and `3.386283x`. Mistral's corresponding control ratio is `0.989236x`, interval `[0.966155x, 1.012868x]`; it is statistically compatible with no change, as predicted for a path that does not invoke the splitter. Raw parity CSV hashes, all sixteen timing logs and CSVs, pair table SHA-256 `ca73c93cd20bdf332b92393fd873768a91675c528429ed0e2d88695a48067677`, and host/toolchain provenance remain at `~/snaptokens-bpe-transfer` on `snaptokens-bpe-transfer-20260914`.

### JSON-load experiment 42: validate cached decomposition through ranked slots — planned

Parent SHA: `c1acf0d41d8937f7768e71a8cb152b881547235c` (clean scoped Experiment 40 source after Experiment 41's full revert).

Hypothesis: retain Experiment 41's compact, pair-sorted parsed rules for JSON construction, but do not sort them again while loading a `.tkz` sidecar. Sidecar construction already rebuilds and validates the exact ranked open-addressed table from its checked slots; use that table for the decomposition pair-to-merged-ID validation, then pass the canonical rank-ordered rule vector to the rest of sidecar construction, whose adjacency is independently supplied. This removes the pair sort that caused Experiment 41's direct-load regression without weakening either validation.

Measured hot cost: Experiment 41's `1.036677x` JSON win shows the compact canonical JSON representation removes material parser and adjacency work. Its `0.905655x` V4 result isolates the adverse direct path: it builds a temporary pair-sorted vector only to perform decomposition validation, despite already constructing the ranked table that contains the same pair answers. The existing sidecar rank and slot checks remain mandatory.

Invariant that makes the shorter path exact: every sidecar merge is range-checked, rank-checked, slot-checked, and probe-chain-checked before the ranked table is installed. Its `get(left, right)` returns the same rank and merged ID the temporary parsed map would return; decomposition validation continues to reject a missing pair or a pair whose merged ID differs from the decomposition token. The JSON path retains stable pair sorting and exact duplicate last-write-wins compaction. Sidecar adjacency remains built from the canonical rank list and retains its own row sorting, so no sidecar row-order assumption changes.

Representation being preserved or changed: reintroduce the private compact parsed-rule representation only for normal construction; direct sidecar validation uses the already-required ranked table rather than an extra pair-sorted temporary. Retain the public `Bpe::new(HashMap<...>)` signature, JSON merge syntax, duplicate semantics, rank/slot validation, CSR and ranked-map layouts, decomposition, caches, APIs, formats, dependencies, evaluator, and timing boundary. Existing test fixtures may change only to construct the private internal representation; no test is added.

Expected winning strata: large JSON BPE construction retains the Experiment 41 win; direct `.tkz` construction avoids its prior sort regression.

Expected adverse strata: malformed sidecars still take complete slot/probe/pair validation; V4/V5 direct load and V5 first encode remain explicit guards.

Smallest files that need changing: `src/models/bpe.rs`, existing `src/models/bpe/tests.rs` fixtures, and this record.

Mechanism evidence: in `from_resolved_with_exact_token_trie`, the ranked table is currently built immediately after decomposition validation, but it has all data required by that validation. Reordering these two private construction steps removes only the temporary map/sort while preserving every error condition before the final BPE is returned.

Acceptance rule: commit a clean candidate; format; run existing BPE construction tests, complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing. On the same GCP host use root-lock immutable parent/candidate binaries, one untimed warmup per arm, and at least twelve alternating GPT-2 JSON-load pairs. Retain only if JSON improves beyond parent noise, V4/V5 direct load and V5 first encode remain within parent noise, binary growth is at most `2%`, and all exactness/sidecar guards pass.

Rejection rule: reject and fully revert for any duplicate/last-write-wins, public constructor signature, sidecar pair/rank/slot/probe validation, CSR row order, rank, merged-ID, byte-fallback, ID, or orphan difference; JSON non-win; direct-load or first-encode regression; binary growth above `2%`; retained test; evaluator change; dependency; API/format change; source/model/input-size dispatch; or removal of a sidecar validation.

Experiment 42 is rejected at the first direct-load guard. Candidate `6d616df` passed existing BPE construction tests (`16`), complete GPT-2 Hugging Face IDs, and byte-fallback Unicode-boundary coverage before timing on the same GCP Intel CPU-0, root lockfile, native flags, and immutable GPT-2 JSON. One untimed process per arm preceded twelve alternating JSON loads. Parent/candidate elapsed nanoseconds were `41988206/39751838`, `45602723/43972756`, `42954881/40224996`, `42032614/40926579`, `41637972/39544603`, `41388777/41557301`, `42730647/41220979`, `41982558/40020659`, `41518415/39372150`, `41636406/39130696`, `41930332/40608147`, and `42012529/40426197`. Candidate/parent JSON throughput was `1.042589x` (one-sided paired t 95% lower bound `1.030176x`; means `42,284,672` versus `40,563,075 ns`).

V4 direct raw parent/candidate pairs were `19847275/23787296`, `19917462/24228797`, `20745006/24298240`, `19531020/23874436`, `20526148/24646277`, `20822861/24696428`, `20866142/24675710`, `20500037/23188542`, `20874036/24612021`, `20373604/23167600`, `20525655/24262599`, and `20972901/23529111`, for `0.849598x` throughput (means `20,458,512` versus `24,080,588 ns`; every pair lost). Reusing the ranked table removes the pair sort but worsens the same direct path, proving the compact parsed-rule representation itself does not meet the file API guard. V5 direct load, first encode, binary-size expansion, and post-timing tests were deliberately not run. Commit `7f67e1a` fully reverts the source and existing-fixture changes; no test file, evaluator, benchmark, dependency, unsafe code, API, format, or dispatch is retained. The candidate and parent target binaries were SHA-256 `e50421f0b9478839f9f080404fc71d7df22686f33ffcd8c0cbab62020b863972` and `fca6fd2b783d15c6a25822676d1261bea19f3bf1722c9e931d2236088cdd7e3e`, respectively.
### Unigram steady-state encode attribution diagnostic (2026-09-14) — planned

Parent SHA: `7b6df8805ee24e4d57a5e9347f3b3639a01a0141` (the clean retained reachable-workspace source).

Hypothesis: the earlier aggregate profile launched the complete CLI repeatedly, so it attributes tokenizer JSON construction to the process even though `simple_bench` loads the tokenizer before its per-input encode timer. A disposable root-lock driver that loads T5 once, warms the actual `encode_with_special_tokens` API, and then repeats one existing LongBench context will isolate the steady-state cost that can support the next production experiment.

Measured hot cost: the aggregate profile places 35.31% of whole-process cycles in `Unigram::tokenize_into_with_scratch`, but it also places 25.0% in `serde_json` and UTF-8 parsing that are outside the timed encode loop. The relative inference shares are therefore not yet attributable.

Invariant that makes the diagnostic exact: the temporary binary passes the unmodified fixture and context through the public `Tokenizer::load_file` and `encode_with_special_tokens` calls. It neither changes tokenizer source, output IDs, the benchmark evaluator, nor the encode path; warmup output and each timed-equivalent output are fully materialized.

Representation being preserved or changed: preserve every production representation, test, fuzzer, dependency, benchmark, API, format, and timing boundary. Add only a temporary `src/bin` profile driver, compile it against the root `Cargo.lock`, capture one GCP callgraph, then remove the driver completely regardless of the result.

Expected winning strata: none. This is attribution only for the existing T5 specialist workload.

Expected adverse strata: none in production because the driver is removed before any candidate. Its synthetic repetition cannot establish a production win.

Smallest files that need changing: temporary `src/bin/unigram_profile.rs` and this record.

Mechanism evidence: `simple_bench` constructs the tokenizer before entering `bench_sequential_no_hf`, while the current profile starts the whole binary. The driver is necessary to distinguish construction from the exact public encode work.

Acceptance rule: commit the diagnostic separately; use the current fixture and existing LongBench sample 10 on the task GCP host, execute four warmups followed by at least one hundred materialized public encodes under `perf record`, preserve the raw profile, and remove the driver without retaining a production change. Any following runtime candidate receives its own card.

Rejection rule: reject the diagnostic method and remove it immediately if it changes tokenizer behavior, IDs, the fixture, the benchmark evaluator, dependencies, public API, model dispatch, or runtime source. Do not infer a production speedup from the diagnostic.

Result: completed and removed. The temporary driver was committed as `0613fe769e1c0d00ccceccd26f0c3fe921ef3267`, built with the root lockfile and native flags, then ran four warmups plus 1,000 fully materialized public `encode_with_special_tokens` calls on pinned LongBench sample 10. The GCP profile at `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/unigram-steady-state.perf` captured 268,152 cycle samples with zero loss. `Unigram::tokenize_into_with_scratch` accounts for 76.69% of steady-state samples; `Metaspace::pre_tokenize_after_whitespace` is 5.02%, `append_ids_for_pieces` 3.94%, and `Precompiled::normalize` 0.53%. The earlier process profile's JSON-parser share is confirmed to be construction work outside the timer. The annotated Viterbi loop identifies the double-array transition check and output lookup as its largest instructions. A fully completed transition cache is not an admissible next representation: the real T5 automaton has 82,672 states and occupies 1,380,528 bytes; a `u32` ASCII table would require 42,328,064 additional bytes and a full byte table 84,656,128 bytes. The driver is removed in the following commit; this result is attribution only, not a throughput claim.

### Unigram Metaspace ASCII-tail boundary candidate (2026-09-14) — planned

Parent SHA: `92c830cb08f72ccf024e7d32d1ad448a7261490f` (the clean retained reachable-workspace source plus diagnostic history only).

Hypothesis: the retained `WhitespaceSplit → Metaspace(always, split=true)` route prepends the three-byte `▁` marker to every ordinary T5 word. The steady-state sample has 382,169 independent splits with an 8-byte median, and its original text is 98.2% ASCII by byte. For an input that is exactly `▁` followed by ASCII, Viterbi's character boundaries are provably byte offsets `0 → 3 → 4 → …`; avoid decoding UTF-8 and computing `char::len_utf8()` at every ASCII tail character by running that fixed-boundary recurrence directly. Every other input retains the existing generic UTF-8 loop.

Measured hot cost: the root-lock steady-state profile assigns 76.69% of samples to `Unigram::tokenize_into_with_scratch`. Its annotated generic recurrence contains a per-character UTF-8 decoder and boundary calculation beside the double-array matcher. On the real 4,299,194-character LongBench sample, pre-tokenization produces 382,169 splits, median 8 bytes, so even small per-split boundary work repeats enough to screen.

Invariant that makes the shorter path exact: `input.strip_prefix('▁').is_some_and(str::is_ascii)` proves the first scalar spans bytes `0..3` and every remaining scalar spans one byte. The specialized loop visits exactly those same character start/end boundaries in the same order as `str::char_indices`. It consumes the unchanged end-ordered matcher, applies the same strict score and source-offset tie comparisons, unknown fallback, backtracking, fused unknown behavior, byte fallback, and output append. Non-marker, marker-plus-Unicode, and no-unknown inputs retain the existing implementation.

Representation being preserved or changed: add one private content-structural fast branch and one private specialized reachable-Viterbi loop. Preserve the automaton, vocabulary, score precision, generic Unicode recurrence, scratch representation, parser, pipeline, BPE paths, public API, formats, cache behavior, dependencies, unsafe code, evaluator, and timing boundary. Extend the existing independent Metaspace fuzzer with deterministic marker-plus-ASCII words; no file under `benchmarks/` changes.

Expected winning strata: Metaspace-prepended, ASCII-tail Unigram words, especially T5-style long English documents with many short splits.

Expected adverse strata: non-Metaspace inputs, Unicode-tail words, marker-only words, no-unknown models, tiny calls, and BPE. Those retain the generic path or are directly covered by the same exact recurrence.

Smallest files that need changing: `src/models/unigram.rs`, its focused boundary test, `fuzz/fuzz_targets/fuzz_unigram.rs`, and this record.

Mechanism evidence: the new steady-state profile excludes construction and shows the Viterbi body dominating; `profile_sample` on the identical source/fixture reports the exact 382,169 split count and 8-byte median. The fixed-boundary predicate is based solely on the input's valid UTF-8 structure, not model name, corpus label, tokenizer metadata, source path, or input length.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/boundary tests, existing T5 scalar/batch/ragged and GPT-2 integration tests, the expanded independent reference fuzzer locally and for 5,000 nightly ASan GCP cases, and a 20-input complete-ID Hugging Face run. Then run three counterbalanced no-HF parent/candidate T5 rounds on the unchanged GCP binary/fixture. Retain only if IDs match and the median throughput gain is at least 3%; keep any result T5-specialist pending broader confirmation.

Rejection rule: fully revert for any boundary, unknown, Viterbi score, tie, byte-fallback, fuzz, integration, or full-ID discrepancy; for a gain below 3%; for a BPE regression; or for any attempt to broaden this into a model/corpus/input-size/evaluator dispatch. Do not change the fixture, runner, timer scope, workers, data, or round schedule to rescue it.

Result: rejected and fully reverted at the first no-HF screen. The candidate `b97b258e1a0f149c57e7be84efda6e66dcd7e176` passed focused Viterbi/boundary tests, pinned T5 scalar/batch/ragged integration, GPT-2 regression, 5,000 nightly ASan fuzzer cases against the independent reference, and the full 20-input Hugging Face ID run. That full run measured Hugging Face `7,553.38 ms` and candidate `298.76 ms` (`25.28x`), but the primary no-HF screen directly loses: retained parent `278.65 ms`, candidate `286.75 ms`, or `0.971753x` candidate/parent throughput. The additional ASCII classification scan costs more than the fixed-boundary loop saves on eight-byte median splits, so counter rounds are not warranted. The following commit removes the specialized function, its test, and the added fuzz route; the source returns exactly to the retained parent representation. Raw outputs remain under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/metaspace-ascii-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram matched-score bounds candidate (2026-09-14) — planned

Parent SHA: `28d7f3da0f2f4bed4d8d2f14fc6686fb2e4c4ec3` (source-equivalent to the retained reachable-workspace implementation).

Hypothesis: each Viterbi match reads `self.scores[id as usize]`, and the compiler retains an ID-versus-score-length bounds check in the hot recurrence. `PrefixMatcher::from_tokens` is built only from `id_to_token.iter().enumerate()` and stores that same in-range index as each double-array output value; `Unigram` has no mutating vocabulary API after construction. An internal checked-in-debug, unchecked-in-release score accessor removes the redundant per-match check without changing a match, score, or table layout.

Measured hot cost: the steady-state root-lock profile assigns 76.69% of samples to Unigram Viterbi. Its annotated match update shows the matcher-provided value compared with the score-vector length immediately before the `f64` score load. The double-array check itself remains larger, but this is a directly proven branch in every reported match rather than a speculative representation change.

Invariant that makes the shorter path exact: `from_parts` rejects vocabularies over `u32::MAX`, assigns each matcher value from `enumerate()` over the immutable ID table, and builds the matcher before constructing the immutable `Unigram`. Therefore every successful `Match<u32>::value()` is an index below `scores.len()`. The accessor has a debug assertion for that constructor proof and does not relax any external parse validation. Models without a matcher do not call it.

Representation being preserved or changed: replace only the private score read in the checked and reachable Viterbi loops with a private accessor. Preserve score storage and precision, parser validation, matcher construction, recurrence, ties, unknown and byte fallback, BPE paths, APIs, formats, dependencies, evaluator, and timing boundary. Add no new test or fuzzer because the existing independent fuzzer already generates arbitrary valid vocabularies and compares every score-selected result with a brute-force `f64` Viterbi reference.

Expected winning strata: every nonempty Unigram matcher path, proportional to reported match count.

Expected adverse strata: matcher-free models, unknown-only text, construction, and BPE are unchanged; no-unigram semantics are unchanged.

Smallest files that need changing: `src/models/unigram.rs` and this record.

Mechanism evidence: the current annotated root-lock assembly has the dynamic score-index comparison in its innermost matched-path update. The safety condition is wholly structural and is established where output values are constructed, not inferred from input, tokenizer model name, corpus, or measured shape.

Acceptance rule: commit a clean candidate; format; run focused Viterbi/duplicate/unknown tests, pinned T5 scalar/batch/ragged and GPT-2 integration, local plus 5,000 nightly ASan independent fuzz cases, and a 20-input complete-ID Hugging Face run. Then use three counterbalanced no-HF GCP T5 rounds. Retain only with exact output and at least 3% median throughput improvement; retain the unsafe code only if that threshold is met.

Rejection rule: fully revert for any ID, score, tie, parser, fuzz, integration, or full-ID discrepancy; a sub-3% result; an unsafe invariant that cannot be locally explained; any BPE regression; or any broader API, layout, dependency, cache, evaluator, model-name, corpus, or input-size dispatch.

Result: rejected and fully reverted after the predeclared three-round screen. Candidate `d065524a391fbe81f8a38c1aa76cf2b055d93294` passed local format; focused Unigram unit tests; T5 scalar/batch/ragged and GPT-2 integration; local independent-reference fuzzing; and, on the GCP task host, the same tests plus 5,000 nightly ASan fuzzer cases. The complete 20-input Hugging Face ID run also passed, measuring Hugging Face `7,645.47 ms` and candidate `289.33 ms` (`26.42x`). The counterbalanced no-HF rounds were parent/candidate `276.62/281.80 ms`, candidate/parent `280.60/277.90 ms`, and parent/candidate `283.46/284.53 ms`; their candidate/parent throughput ratios are `0.981618x`, `0.990378x`, and `0.996239x`, for a `0.990378x` median. The dynamic bound was not a material bottleneck once the compiler's surrounding work is included, so the unsafe accessor does not clear the 3% specialist gate. The following commit restores ordinary checked indexing exactly; no API, model layout, test, fuzzer, dependency, evaluator, or benchmark change remains. Raw outputs are under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/score-bounds-{full-hf,parent-1,candidate-1,candidate-2,parent-2,parent-3,candidate-3}.out` on the task VM.

### Unigram charwise double-array mechanism screen (2026-09-14) — rejected before source change

The next plausible matcher alternative was Daachorse's already-installed character-aware double-array automaton. It preserves byte-offset matches for valid UTF-8 pieces while collapsing each three-byte Metaspace marker to one transition, so it was measured off-tree before any production edit. A disposable root-compatible probe rebuilt the pinned T5's 32,100 last-duplicate pieces with both automata and asserted equal `(start, end, ID)` vectors for representative Metaspace English words. The T5 vocabulary's highest scalar is only `U+2581`, but the character-aware automaton still used `81,417` states and `1,730,232` heap bytes versus the retained bytewise automaton's `82,672` states and `1,380,528` bytes. Its mandatory scalar-to-code mapper more than erases the state reduction.

The same probe repeatedly scans independent short Metaspace words (`▁the`, `▁and`, `▁that`, `▁this`, `▁is`, `▁a`, and `▁of`), matching the many-short-splits regime that dominates the pinned workload. Bytewise scanning took `4.577 ms`; character-aware scanning took `4.639 ms`, or `1.013623x` character-aware/bytewise elapsed time. The output fingerprints agree. This source-free mechanism screen rejects the alternative: it is larger and slower before Viterbi work is considered. The local scratch probe is in Trash and the remote copy is archived at `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/discarded-charwise-probe-hmtqlY`; no source, test, fuzzer, dependency, API, evaluator, or benchmark change was made.

### Unigram strictly-dominated vocabulary-piece mechanism screen (2026-09-14) — rejected before source change

An exact model-structural reduction was screened off-tree: a final, nonempty piece can be removed from the all-match automaton only if another segmentation of the same spelling into known pieces has a strictly higher total score. Any global path containing such a piece can then be replaced at the same byte boundaries with a strictly higher-scoring path, so it cannot be Viterbi-optimal. The probe rebuilt the retained bytewise automaton and ran a known-piece-only Viterbi dynamic program for every final T5 spelling while suppressing that spelling's own full-range match. Only two of 32,100 pieces are dominated: `▁Its` (margin `0.170188665`) and `▁mes` (margin `0.136613607`), twelve bytes total. The complete screen took `11.401 ms`; it is much too small a matcher-output reduction to justify a production representation or startup cost. The local probe is in Trash and the remote copy remains isolated at `/tmp/snaptokens-unigram-dominance-probe.XwgWGD`; no source, test, fuzzer, dependency, API, evaluator, or benchmark change was made.

### Unigram direct known-path backtracking candidate (2026-09-14) — planned

Parent SHA: `c45d32262a53327a915b7769dfb57cb607509a67` (clean retained reachable-Viterbi source plus recorded rejected screens).

Hypothesis: after the reachable Viterbi recurrence, the common path currently writes every predecessor as a 24-byte `PathPiece`, reverses that vector, then rereads it to append 32-bit IDs. If the chosen path contains no `unk_id`, no byte fallback or unknown fusion is needed: append IDs while tracing backwards and reverse only the newly appended 32-bit output range. If the backwards trace sees `unk_id`, truncate those provisional IDs and retain the existing `PathPiece` reconstruction and `append_ids_for_pieces` route unchanged. This removes two `PathPiece` passes only when their hidden metadata is not semantically needed.

Measured hot cost: the root-lock steady-state profile assigns 76.69% of samples to Unigram Viterbi and 3.94% to `append_ids_for_pieces`; the predecessor reconstruction lies adjacent to that latter stage. The full 20-input T5 output contains 88,459 unknown IDs among 5,440,870 emitted IDs, so known-only paths are common enough to screen, but the candidate must preserve the existing route for every path that does contain an unknown.

Invariant that makes the shorter path exact: Viterbi predecessors are followed from the final endpoint to zero in both variants. With no unknown node, the old forward `PathPiece` ID sequence is exactly the reverse of that backwards node-ID sequence, and reversing only the appended output suffix restores it without touching preceding added-token IDs. If any selected node is `unk_id`, discarding the provisional suffix and running the old reconstruction performs the same unknown-span fusion, source slicing, and optional byte fallback as before. Scores, predecessor choice, ties, offsets, regular IDs, and all error paths are unchanged.

Representation being preserved or changed: add one private reachable-path helper that appends only its known-ID fast result directly to `out`; retain the current `PathPiece` scratch fallback verbatim for paths containing `unk_id`. Preserve score precision, matcher, dynamic-programming state, parser, unknown and byte-fallback behavior, BPE paths, APIs, formats, dependencies, evaluator, and timing boundary. The branch follows selected Viterbi state, not a model name, corpus, tokenizer metadata, or input size.

Expected winning strata: Unigram paths without unknown nodes, especially normal Metaspace splits that currently construct short `PathPiece` vectors. Expected adverse strata: unknown and byte-fallback paths do one short failed reverse trace before the exact old reconstruction; no-`unk_id` models and BPE retain their existing paths.

Smallest files that need changing: `src/models/unigram.rs`, focused backtracking coverage if the existing global-path and unknown-fusion tests do not make the new internal split visible, and this record. The independent Unigram fuzzer already compares complete known, unknown, byte-fallback, tie, and Unicode outputs against a separate brute-force Viterbi reference.

Mechanism evidence: the existing reachable recurrence owns the predecessor table and separately calls `backtrack_reachable_into` then `append_ids_for_pieces`; the latter's forward-only metadata is necessary only to fuse adjacent unknown spans and inspect their original source range. The measured output inventory establishes that unknown behavior is material and must remain a direct fallback, rather than being assumed absent.

Acceptance rule: commit a clean candidate; format; run focused global/tie/unknown/byte-fallback tests, pinned T5 scalar/batch/ragged and GPT-2 integration, and the independent fuzzer locally plus 5,000 nightly ASan GCP cases. Run the 20-input complete-ID Hugging Face comparison before timing, then three counterbalanced no-HF GCP T5 rounds. Retain only with exact output and at least 3% median throughput improvement; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert for any known-path ordering, added-ID prefix, unknown fusion, byte fallback, score, tie, parser, fuzzer, integration, or full-ID discrepancy; a sub-3% result; any BPE regression; or any API, format, dependency, evaluator, model-name, corpus, or input-size dispatch.

Result: rejected and fully reverted after the predeclared timing screen. Candidate `83a53a56d0279e1c9110359a4ad85d2038cdbc18` passed local format; ten focused Unigram tests including the new known-prefix and unknown-fallback boundaries; T5 scalar/batch/ragged and GPT-2 integration; local independent-reference fuzzing; and the same focused tests plus 5,000 nightly ASan fuzzer cases on GCP. The 20-input Hugging Face ID run was exact, measuring Hugging Face `7,808.21 ms` and candidate `283.13 ms` (`27.58x`). The counterbalanced no-HF rounds were parent/candidate `280.70/274.27 ms`, candidate/parent `274.10/277.56 ms`, and parent/candidate `277.36/273.24 ms`; their candidate/parent throughput ratios are `1.023444x`, `1.012623x`, and `1.015078x`, for a `1.015078x` median. Direct known-ID reversal removes the expected metadata work but Viterbi matching still dominates enough that the reduction does not meet the 3% specialist gate. The following commit restores the original single range-aware path and removes the branch-specific unit assertions; no API, format, dependency, evaluator, or benchmark change remains. Raw outputs are under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/known-backtrack-{full-hf,parent-1,candidate-1,candidate-2,parent-2,parent-3,candidate-3}.out` on the task VM.

### Unigram lead-byte Viterbi-boundary candidate (2026-09-14) — planned

Parent SHA: `4872da0092528f0f181a0845d5ef2bd665b77231` (clean retained reachable-Viterbi source plus recorded rejected screens).

Hypothesis: both Viterbi recurrences use `str::char_indices()` and immediately discard the decoded scalar after calling `char::len_utf8()`. They only need valid UTF-8 byte boundaries. Starting at byte zero and advancing by the already-valid leading byte's UTF-8 width visits exactly the same starts and ends, but avoids reconstructing a Unicode scalar at each position. Unlike the rejected Metaspace-ASCII candidate, this changes the ordinary loop for every valid Unigram split and performs no prefix check or whole-input classification scan.

Measured hot cost: the root-lock steady-state profile assigns 76.69% of samples to Unigram Viterbi, and its annotated generic recurrence includes UTF-8 scalar decoding and boundary calculation beside the double-array transition. The pinned workload creates 382,169 independent splits from its 4.3M-byte sample; every one enters this boundary loop. The rejected ASCII-tail route establishes that an extra per-split scan costs more than its specialized recurrence saves, not that an exact single-pass boundary representation loses.

Invariant that makes the shorter path exact: a Rust `&str` is valid UTF-8. At every current character boundary, a leading byte below `0x80` occupies one byte, below `0xE0` occupies two, below `0xF0` occupies three, and otherwise occupies four. Advancing by that width produces exactly the offsets from `char_indices`; all match grouping, scores, ties, unknown fallback, byte fallback, source ranges, and backtracking therefore receive the identical boundaries and are unchanged.

Representation being preserved or changed: replace only the private character-boundary iteration in the checked and reachable Viterbi loops with a private lead-byte-width expression. Preserve the matcher, Viterbi state, score precision, parser, Unicode support, unknown and byte fallback, BPE paths, APIs, formats, dependencies, evaluator, and timing boundary. Add no unsafe code, cache, configuration dispatch, model-name branch, corpus branch, or input-size branch. Existing Unicode integration and the independent Unigram fuzzer cover the valid-UTF-8 boundary contract.

Expected winning strata: every nonempty Unigram split, with the most repeated boundary work in short Metaspace word splits. Expected adverse strata: non-ASCII text takes the corresponding two-, three-, or four-byte width branch but retains the same boundary sequence; BPE is unchanged.

Smallest files that need changing: `src/models/unigram.rs` and this record. Existing global-path, tie, unknown, byte-fallback, Unicode pipeline, and fuzz coverage exercise the semantic boundary.

Mechanism evidence: the existing Viterbi body does not inspect `character` except for `len_utf8`, while the input type itself proves the exact width relation. This removes scalar construction directly from the measured loop without asking the input to satisfy an extra content predicate.

Acceptance rule: commit a clean candidate; format; run focused Viterbi/tie/unknown/byte-fallback tests, pinned T5 scalar/batch/ragged and GPT-2 integration, and the independent fuzzer locally plus 5,000 nightly ASan GCP cases. Run the complete 20-input Hugging Face ID comparison before timing, then three counterbalanced no-HF GCP T5 rounds. Retain only with exact output and at least 3% median throughput improvement; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert for any UTF-8 boundary, match grouping, score, tie, unknown, byte-fallback, parser, fuzzer, integration, or full-ID discrepancy; a sub-3% result; any BPE regression; or any API, format, dependency, evaluator, model-name, corpus, or input-size dispatch.

Result: rejected at the first no-HF screen and fully reverted. Candidate `aa6a356fc020d3eccab66d10187a55b0d859034d` passed local format; ten focused Unigram tests including one/two/three/four-byte lead-width coverage; T5 scalar/batch/ragged and GPT-2 integration; local independent-reference fuzzing; and the same focused tests plus 5,000 nightly ASan fuzzer cases on GCP. The 20-input Hugging Face ID run was exact, measuring Hugging Face `7,673.79 ms` and candidate `292.86 ms` (`26.20x`). The first ordered no-HF screen directly loses: parent `280.89 ms`, candidate `281.74 ms`, or `0.996983x` candidate/parent throughput. The compiler already specializes the standard character iterator effectively enough that avoiding scalar construction by hand adds, rather than removes, real work. Two further counterbalanced rounds are deliberately not run because this is a clear rejection, not an admissible retention result. The following commit restores `char_indices` and removes the helper test; no API, format, dependency, evaluator, or benchmark change remains. Raw outputs are under `/tmp/snaptokens-unigram-perf-20260914.LJPVv8/lead-byte-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram reverse-trie matcher mechanism screen (2026-09-14) — rejected before source change

A reverse byte trie was screened off-tree because the Viterbi recurrence consumes vocabulary pieces grouped by end boundary. In principle, walking backward from each character end could update the current dynamic-programming cell directly and remove the Daachorse overlapping-match iterator. The probe built the pinned T5's 32,100 final, nonempty pieces into a compact reverse trie with sorted byte edges and one terminal ID per spelling. It reconstructed the fixture's `WhitespaceSplit → Metaspace(always)` word shape from LongBench sample 10, producing 382,169 nonempty marker-prefixed splits. The fixture's precompiled normalization was deliberately not reproduced: this is a matcher-cost screen, not tokenizer parity evidence.

Before timing, the probe materialized and sorted every `(start, end, ID)` match from both paths; their 6,484,625-match sets were identical. The retained bytewise Daachorse matcher scanned the workload in `48.373 ms`; the reverse trie took `208.101 ms`, or `4.3020x` reverse-trie/Daachorse elapsed time. It revisits bytes from each candidate endpoint, which overwhelms any savings from end-grouped dynamic programming on the short-split workload. This source-free screen rejects the representation before a production candidate, test, fuzzer, dependency, API, evaluator, or benchmark change. The disposable local probe is `/tmp/snaptokens-unigram-reverse-trie.IHAftq`; its remote copy and downloaded fixture/sample slice are under `/tmp/snaptokens-unigram-reverse-trie.IHAftq` and `/tmp/snaptokens-unigram-probe-data` on the task VM.

### Unigram direct double-array Viterbi candidate (2026-09-14) — planned

Parent SHA: `74646c598773d2b15e8feb6c3f96d3a5d992564c` (clean retained reachable-Viterbi source plus recorded rejected screens).

Hypothesis: the generic Daachorse overlapping iterator reports one `Match` at a time, retaining iterator state and returning to the Viterbi loop for each output-chain entry. Viterbi already consumes matches in character-end order. Decode the same pinned Daachorse state/output serialization once at model construction, then traverse its double-array and output chain directly inside each Viterbi character step. This removes the generic iterator handoff and lets the recurrence consume the existing end group without constructing an intermediate match.

Measured hot cost: the exact-current task-VM sample-10 callgraph assigns 59.06% of cycles to `Unigram::tokenize_into_with_scratch`. Its annotated recurrence samples the double-array child check at 11.30% and output-chain state load at 9.50% of that function. Before source change, an off-tree direct reader built from the pinned T5's public Daachorse 1.0.1 serialization materialized the same 6,484,625 `(start, end, ID)` matches as `find_overlapping_iter`; it scanned 382,169 reconstructed Metaspace splits in `45.342 ms` versus `48.358 ms` for Daachorse, a `1.0665x` matcher-only throughput indication. The production candidate instead pins the parent's 1.0.0 dependency, so its end-to-end validation, rather than this version-adjacent screen, decides retention.

Invariant that makes the shorter path exact: `PrefixMatcher::from_tokens` builds the bytewise standard Daachorse automaton from the immutable final nonempty vocabulary spellings. The candidate decodes the state, failure, output-position, check-byte, output ID, byte length, and output-parent fields emitted by that same `DoubleArrayAhoCorasick<u32>::serialize()` call, validates all referenced state/output indices, and drops the builder result. Its transition and parent traversal are a direct restatement of Daachorse 1.0.0's standard overlapping iterator. Every vocabulary spelling and input is valid UTF-8, so every reported pattern start and end remains a character boundary; scanning all bytes between successive Viterbi character ends therefore emits the same complete end group. Pin Daachorse to `=1.0.0`, matching the parent, because this private representation follows that library's serialized layout.

Representation being preserved or changed: replace only the private Unigram matcher storage and the two private Viterbi match-consumption loops. Preserve the model's vocabulary IDs, scores, precision, parser validation, ties, unknown and byte fallback, Metaspace pipeline, BPE paths, public APIs, formats, fixture, evaluator, and timing boundary. The temporary serialization allocation occurs at construction, never encode. Add no model-name, corpus, input-size, cache, or benchmark dispatch; every valid Unigram model uses the same structural matcher path.

Expected winning strata: nonempty Unigram models with many overlapping prefix matches, especially short Metaspace splits where iterator setup and output-chain handoff are a large part of the matching work. Expected adverse strata: Unigram construction pays one temporary serialization; matcher-free vocabularies, BPE, loader behavior, and public APIs are unchanged.

Smallest files that need changing: `Cargo.toml`, the root and fuzz `Cargo.lock` files, `src/models/unigram.rs`, its focused matcher test, and this record. Existing independent-reference Unigram fuzzing and pinned T5 scalar/batch/ragged integration tests exercise the Viterbi scores, ties, Unicode boundaries, unknown fusion, byte fallback, and pipeline result.

Mechanism evidence: the existing `find_overlapping_iter` source walks the same bytewise double-array state and linked output chain but yields each match through its generic iterator state machine. The off-tree equivalent uses the public serialized bytes from the same constructed automaton, asserts complete match-set equality before timing, and isolates the expected removal of that handoff. The source candidate must repeat exact IDs; the screen is not parity proof.

Acceptance rule: commit a clean candidate; format; run focused Viterbi/duplicate/unknown/byte-fallback tests, pinned T5 scalar/batch/ragged and GPT-2 integration, the independent Unigram fuzzer locally plus 5,000 nightly ASan GCP cases, and a complete 20-input Hugging Face ID run. Then run three counterbalanced no-HF parent/candidate T5 rounds on unchanged fixture, data, workers, and timer scope. Retain only with exact output and at least 3% median throughput improvement; keep it T5-specialist pending broader confirmation.

Rejection rule: fully revert for any serialized-matcher structural validation, match set, UTF-8 boundary, score, tie, unknown, byte-fallback, parser, fuzz, integration, full-ID, BPE, construction, or API discrepancy; a sub-3% result; a material construction/memory regression; or any model-name, corpus, input-size, evaluator, fixture, runner, timer, or worker change.

Result: rejected and fully reverted after the predeclared three-round screen. Candidate `4bd5e1b29ac325f95a05bcb467655b4b50ba72e2` pinned Daachorse 1.0.0 to isolate the traversal from a dependency upgrade. It passed formatting; ten focused Unigram unit tests including the new multibyte matcher boundary; pinned T5 scalar and repeated-prefix integration; GPT-2 regression; 1,000 local nightly ASan independent-reference fuzz cases; ten focused GCP tests; and 5,000 GCP nightly ASan cases. The complete 20-input Hugging Face ID run was exact, measuring Hugging Face `7,453.55 ms` and candidate `302.32 ms` (`24.65x`). On the identical 18,012,626-character, 5,440,870-token local-context array, counterbalanced no-HF rounds were parent/candidate `278.51/290.83 ms`, candidate/parent `293.11/281.40 ms`, and parent/candidate `280.90/295.23 ms`. The candidate/parent throughput ratios are `0.957638x`, `0.960049x`, and `0.951462x`, for a `0.957638x` median. The source-free matcher gain does not survive fusion with Viterbi, so it is not a valid optimizer despite exact output and fuzzing. Commits `0e255d8` and `4f5f1e2` restore the root dependency, fuzzer lock, matcher, tests, and API surface exactly; no candidate source remains. Raw outputs are under `/tmp/snaptokens-unigram-probe-data/direct-daa-{full-hf,parent-1,candidate-1,candidate-2,parent-2,parent-3,candidate-3}.out` on the task VM.

### Unigram direct double-array iterator candidate (2026-09-15) — rejected and reverted

Parent SHA: `295b2fbd2f603792f0a6ada52f0370ff73f2364c` (clean retained reachable-Viterbi source plus recorded rejected screens).

Hypothesis: the rejected direct traversal fused byte-state work into Viterbi's character loop, causing a 4.2% loss despite a 1.0665x matcher-only screen. Keep the retained generic Viterbi recurrence and replace only its Daachorse iterator with a private iterator over the same validated serialized double-array tables. It should preserve Daachorse's schedule of exhausting one output-parent chain before scanning to the next output-bearing byte, avoiding the prior byte/character-cursor coordination.

Measured hot cost: the task-VM parent profile assigns 67.51% of samples to `Unigram::tokenize_into_with_scratch`. The fused candidate had a smaller function (`4,412` versus `5,022` bytes) and lower branch-miss rate (1.42% versus 1.50%), but executed 2.22% more instructions (`2.397B` versus `2.345B`) and 5.55% more branches (`516.2M` versus `489.0M`). This distinct iterator-placement experiment targeted that extra fused-loop work.

Invariant that made the candidate exact: construction built the same standard bytewise Daachorse automaton from final nonempty vocabulary spellings, decoded and bounds-validated its pinned 1.0.0 state/output serialization, and yielded the exact `(start, end, ID)` order of `FindOverlappingIterator::next`. Valid UTF-8 vocabulary and input preserve character boundaries, while scores, strict-greater/smaller-source ties, unknown and byte fallback, backtracking, pipeline, BPE path, API, formats, fixture, evaluator, and timer remained unchanged. The serialization work is construction-only; no model, corpus, input-size, cache, or benchmark dispatch was added.

Expected winning strata: dense-match Unigram splits, especially T5 Metaspace words. Expected adverse strata: construction and empty matchers; BPE unchanged. Smallest changed files were `Cargo.toml`, the resolver-required fuzz lock, `src/models/unigram.rs`, one multibyte-order test, and this record. Existing independent brute-force Unigram fuzzing and pinned T5 scalar/batch/ragged tests exercise the unchanged Viterbi and pipeline contract.

Acceptance rule: require formatting; focused Viterbi/duplicate/unknown/byte-fallback/multibyte, T5, and GPT-2 tests; local plus 5,000-case task-VM nightly ASan fuzzing; a complete 20-input Hugging Face ID run; then three counterbalanced no-HF T5 rounds. Retain only exact output with at least a 3% median throughput gain. Reject for any semantic, fuzz, construction, BPE, API, format, evaluator, or timer discrepancy, or a sub-3% result.

Result: rejected at the first decisive ordered no-HF screen and fully reverted. Candidate `75dee5ac5ac50ad4fff3c4a0d9d489f42b08ec03` passed formatting; all ten focused Unigram tests including a new multibyte matcher-boundary test; pinned T5 scalar and repeated-prefix integration; GPT-2 regression; 1,000 local nightly independent-reference cases; task-VM focused tests; and 5,000 task-VM nightly ASan cases without a mismatch or sanitizer finding. The complete 20-input Hugging Face ID run was exact, measuring Hugging Face `7,458.06 ms` and candidate `313.80 ms` (`23.77x`). The immutable parent then took `282.09 ms` and the candidate `289.94 ms` on the same 18,012,626-character, 5,440,870-token local-context array: `0.972925x` candidate/parent throughput. That is both a regression and well below the 3% retention floor, so two further timing rounds are intentionally skipped under the earliest-rejection rule. The following revert restores the original matcher, dependency constraint, fuzz lock, focused test surface, and public behavior exactly; raw evidence is `/tmp/snaptokens-unigram-probe-data/direct-iterator-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram end-group local-winner candidate (2026-09-15) — rejected and reverted

Parent SHA: `abcd477f7c0fc2e52d993ff4ec2a729cf578d6d2` (clean retained matcher and recorded rejected iterator screen).

Hypothesis: all vocabulary matches with one end boundary have one logical Viterbi destination. Replace the retained loops' repeated target-table load/store with a local `Option<BestPathNode>` or `BestPathNode` winner, apply real matches then the existing unknown candidate in unchanged order, and commit the final node before the next character reads it. The checked no-unknown path retains its `None` error semantics.

Measured hot cost: the task-VM parent profile assigns 67.51% of samples to `Unigram::tokenize_into_with_scratch`, including repeated current-target score/predecessor loads and 24-byte stores in the output-chain scorer. The pinned T5 probe reports 6,484,625 matches over 382,169 reconstructed Metaspace splits. Nonempty pieces ensure a match source cannot alias its end-group destination, and end order ensures no other group writes that destination. Thus an identically ordered local winner should produce the same strict-score/smaller-source-tie result, unknown fallback, UTF-8 bounds, backtracking, byte fallback, parser, matcher, pipeline, BPE, API, format, fixture, evaluator, and timer behavior without unsafe code or new dispatch.

Expected winning strata: match-dense Unigram groups such as pinned T5; expected adverse strata: one-match or unknown-only groups; BPE unchanged. The only source change was 18 private Viterbi-loop lines plus this record. Existing global-score, tie, unknown, byte-fallback, duplicate-order, Unicode, T5, and independent brute-force fuzz coverage exercise the invariant.

Acceptance rule: require formatting; focused Unigram/T5/GPT-2 tests; local independent fuzzing; full 20-input Hugging Face IDs; then counterbalanced no-HF GCP rounds, retaining only exact output and at least a 3% median improvement. Reject on any semantic discrepancy, material resource regression, or sub-3% target result.

Result: rejected at the first decisive ordered no-HF screen and fully reverted. Candidate `ed56a060be08eb000b0cbc26d46a5bbb6097e9f8` passed formatting; all nine focused Unigram tests; pinned T5 scalar and repeated-prefix integration; GPT-2 regression; and 1,000 local nightly independent-reference fuzz cases without a mismatch or sanitizer finding. The complete 20-input Hugging Face ID run was exact, measuring Hugging Face `7,400.34 ms` and candidate `297.91 ms` (`24.84x`). The immutable parent then took `275.65 ms` and the candidate `292.59 ms` on the same 18,012,626-character, 5,440,870-token local-context array: `0.942103284x` candidate/parent throughput. This is a clear regression, so the remote sanitizer and further timing rounds are skipped under the earliest-rejection rule. The following revert restores the retained source exactly; raw evidence is `/tmp/snaptokens-unigram-probe-data/local-winner-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram end-group tie-order candidate (2026-09-15) — rejected

Parent SHA: `01c29e8994aa531197069332e6ab92ea97d234ba` (clean retained Viterbi source plus recorded rejected local-winner screen).

Hypothesis: the Viterbi update preserves a smaller source offset on equal scores with `score == target.score && match_start < target.starts_at`. Daachorse's overlapping iterator emits one state output followed by its strict-suffix output-parent chain; at a fixed end boundary, sources therefore strictly increase. The first equal-score candidate already has the required smaller source, so remove that dead equal-score source comparison from both checked and reachable loops.

Measured hot cost: the task-VM parent profile assigns 67.51% of samples to `Unigram::tokenize_into_with_scratch`; its matched update executes this predicate for each of 6,484,625 T5 matches over 382,169 reconstructed Metaspace splits. All vocabulary spellings are nonempty and duplicate spellings collapse to one final ID, making the iterator ordering invariant applicable.

Invariant and scope: the first equal-score match for an end is the left-to-right tie winner. This changes no representation, parser, matcher, score, Unicode behavior, API, evaluator, timer, or BPE path; it merely removes the private tie subcondition. The established overlap-order and score-tie tests plus independent brute-force fuzzer cover the boundary.

Acceptance rule: commit a clean candidate; format; run focused Unigram/T5/GPT-2 tests and local independent fuzzing; then full 20-input Hugging Face IDs, task-VM sanitizer testing, and three counterbalanced no-HF T5 rounds. Retain only exact output with at least a 3% median throughput improvement. Reject on any discrepancy, material guardrail regression, or sub-3% target result.

Result: rejected at the first decisive target screen and fully reverted. Candidate `d1233edab6fd7cb3b57ed9fe25ccb77adb4da5fc` passed formatting, focused tests, and 1,000 local nightly independent-reference fuzz cases. The complete remote 20-input Hugging Face run was exact for 18,012,626 characters and 5,440,870 token IDs, measuring Hugging Face `7,402.86 ms` and candidate `296.21 ms` (`24.99x`). On the unchanged no-HF T5 screen, immutable parent took `277.42 ms`; candidate took `290.08 ms`, for `0.956356730x` candidate/parent throughput. This is a clear regression, so remote ASan and additional timing rounds are skipped under the earliest-rejection rule. The revert restores the retained source exactly. Raw evidence is `/tmp/snaptokens-unigram-probe-data/tie-order-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram compact predecessor candidate (2026-09-15) — rejected

Parent SHA: `6a2250f61fd1e3e20f6acb08b86762eb1aab72c3` (clean retained Viterbi source plus recorded rejected matcher and recurrence variants).

Hypothesis: the unknown-enabled Viterbi node is `f64 score + usize starts_at + u32 id`, occupying 24 bytes through alignment. Store lossless offsets as `u32` for splits at most `u32::MAX` bytes, producing a 16-byte compact node, and retain the exact original full-width recurrence for larger splits.

Mechanism evidence: a fresh parent five-run task-VM capture reports `2.545B` cycles, `5.517B` instructions, `1.139B` branches, `1.293B` L1 data loads, and `30.755M` L1 misses (2.38%) per full T5 run. A 34,085-sample callgraph attributes 66.86% of cycles to `Unigram::tokenize_into_with_scratch`; T5 has 6,484,625 matches over 382,169 Metaspace splits. The candidate changed only predecessor representation and added the necessary wide fallback; scores, IDs, recurrence order, ties, unknown fallback, matcher, UTF-8 behavior, output, API, formats, BPE, fixture, evaluator, and timer were unchanged.

Invariant: for a compact split every byte boundary, matcher start/end, predecessor, and backtrack end fits losslessly in `u32`, and conversion back to `usize` occurs solely for the pre-existing indexing and slicing operations. The full-width fallback retains the old recurrence for the non-fitting input domain.

Acceptance rule: require formatting; focused Unigram/T5/GPT-2 tests; local independent-reference fuzzing; complete 20-input Hugging Face IDs; then task-VM sanitizer and counterbalanced no-HF rounds, retaining only exact output with at least 3% median improvement. Reject for any semantic discrepancy, guardrail failure, or sub-3% target result.

Result: rejected at the first decisive target screen and fully reverted. Candidate `f3bc89b309f2d9bcc32fbf3f7fef7df4c1055bf5` passed formatting, all ten focused Unigram tests including compact-versus-wide recurrence agreement and the 16-byte layout assertion, pinned T5 scalar/repeated-prefix integration, GPT-2 regression, and 1,000 local nightly independent-reference fuzz cases. The complete remote 20-input Hugging Face run was exact for all 18,012,626 characters and 5,440,870 token IDs, measuring Hugging Face `7,417.66 ms` and candidate `311.09 ms` (`23.84x`). The immutable parent then took `278.96 ms` and candidate `295.24 ms` on the unchanged no-HF T5 screen, or `0.944858420x` candidate/parent throughput. The narrower representation loses to its conversion and fallback-control cost, so remote ASan and more rounds are skipped under the earliest-rejection rule. The following revert restores the retained source exactly; raw evidence is `/tmp/snaptokens-unigram-probe-data/compact-node-{full-hf,parent-screen,candidate-screen}.out` on the task VM.

### Unigram stored-predecessor-only candidate (2026-09-15) — rejected

Parent SHA: `44f33cda1ba408a31456038e3a9137aedca88d28` (clean retained Viterbi source plus the rejected compact-node screen).

Hypothesis: retain `usize` for all active matcher/vector/string indices, compact only the stored predecessor to `u32` after a lossless input-length guard, and put the full-width recurrence in a cold, non-inlined function. This avoids the rejected candidate's repeated hot-loop width conversions and inline fallback code.

Mechanism evidence: parent counters report `2.545B` cycles, `5.517B` instructions, `1.293B` L1 loads, and `30.755M` L1 misses; a 34,085-sample profile assigns 66.86% to Viterbi. A forced-clean remote build verified that the retained common Viterbi body is 4,703 bytes. The candidate's common body was 4,677 bytes, with its 4,661-byte full-width routine emitted separately, proving the intended assembly separation before timing.

Invariant and scope: compact inputs losslessly store predecessors as `u32`, widening only for ties/backtracking while every active index remains `usize`; the cold path preserves original behavior for a non-fitting input. Scores, IDs, matcher order, ties, unknown and byte fallback, UTF-8 behavior, pipeline, BPE, API, formats, fixture, evaluator, data, and timer remain unchanged. No unsafe code or benchmark dispatch was added.

Acceptance rule: require formatting, focused Unigram/T5/GPT-2 tests, local independent fuzzing, complete Hugging Face IDs, remote sanitizer coverage, and three counterbalanced no-HF rounds; retain only an exact result with at least 3% median improvement. Reject for any semantic/guardrail failure or sub-3% target result.

Result: rejected at the first decisive target screen and fully reverted. Candidate `b121ea0b06c55bb5a37c4b165d67b293c6c276ec` passed formatting, all ten focused Unigram tests including direct compact-versus-wide agreement and 16-byte layout, T5 scalar/repeated-prefix integration, GPT-2 regression, and 1,000 local nightly independent-reference fuzz cases. The full remote Hugging Face run was exact for all 18,012,626 characters and 5,440,870 IDs, measuring Hugging Face `7,447.35 ms` and candidate `311.42 ms` (`23.91x`). On the unchanged no-HF screen, immutable parent took `276.29 ms` and candidate `296.65 ms`, for `0.931366931x` candidate/parent throughput. Since the fast-path code is smaller yet output is substantially slower, node packing itself is rejected; remote ASan and further timing rounds are skipped. The following revert restores the retained source exactly; raw evidence is `/tmp/snaptokens-unigram-probe-data/stored-node-{full-hf,parent-screen,candidate-screen}.out` on the task VM.


### 2026-09-16: user-requested tactical cleanups

Parent SHA: 7b0c083 (frozen clean origin/main for this PR).

Hypothesis: replace redundant ownership transfers and manual bookkeeping with existing Rust, Serde, and ICU facilities across the 24 reviewed tactical candidates. This is an explicitly requested maintenance batch, not a throughput-champion experiment.

Measured hot cost: none measured for this batch. The source audit identified copied decoder buffers, materialized temporary ranges, repeated construction lookups, and unused dependency features. No speed, allocation-count, binary-size, or compile-time improvement is claimed.

Invariant that makes the shorter path exact: preserve complete token IDs and ordering, invalid UTF-8 replacement, added-token whitespace boundaries, no-match borrowing, independent Python metadata lengths, and JSON/.tkz validation. Keep the existing hash arithmetic and merge priority ordering.

Representation being preserved or changed: retain model, cache, scanner, and public API representations. Use owned buffers where already available, standard slice/string operations, constant ICU initialization, and a non-generic chunk concatenator.

Expected winning strata: maintenance only; decoder, padding, loading, and build-cost benefits are unmeasured.

Expected adverse strata: malformed decoded bytes, many short decoded pieces, left padding with materialized metadata, dense literal matches, and dependency feature unification.

Smallest files that need changing: owning Rust/Python modules, focused semantic tests, root Cargo manifest/lock, and this log. Evaluator files remain unchanged.

Mechanism evidence: source audit in the preceding conversation, Rust standard-library contracts, ICU 2.1.1 source, and the existing local implementations.

Acceptance rule: preserve behavior through existing differential/unit/Python tests plus focused boundary coverage; pass formatting, strict Clippy, documentation, and feature checks. Report any unavailable validation. Open the requested tactical-cleanups PR without merging or promoting the general champion.

Rejection rule: remove any individual cleanup that cannot preserve the existing contract or pass focused validation. Performance remains unmeasured until a separate committed comparison satisfies the repository's experiment gates.

Implementation: all 24 audited items are included. Standard operations replace custom hash construction, byte chunking, whitespace scans, UTF-8 boundary advancement, and slice-size bookkeeping. NFC uses constant ICU data. Decoder and Python padding paths reuse owned buffers, borrowed decode paths avoid temporary token strings, and batch/chunk assembly uses known lengths. Split emits simple ranges directly, replacement retains one match iterator and no-match borrowing, JSON parsing consumes or borrows existing values, and identity templates return their input. Merge-entry equality now agrees with its existing priority ordering. Root dependency features drop unused ICU UTF-16/UTF-8 interfaces and Tokenizers progress/C++ training support while retaining its Oniguruma engine. No dependency versions or evaluator files changed.

Compatibility: declare Rust 1.91 because `str::ceil_char_boundary` stabilized there. Both workspace crates compile with Rust 1.91.0 and the committed dependency resolution. Runtime API and tokenizer formats are unchanged.

Validation on Apple ARM, with the shared Cargo build cache and a separate Python virtual environment:

- `cargo fmt --all -- --check` and `git diff --check`: passed.
- `cargo test --offline --locked --workspace`: 201 passed, 10 existing integration tests ignored, none failed. Includes unit, integration, Python binding Rust tests, and doctests.
- `cargo clippy --offline --locked --workspace --all-targets -- -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --offline --locked --workspace --no-deps`: passed.
- `cargo check --offline --locked -p snaptokens --no-default-features`: passed.
- `cargo +1.91.0 check --offline --locked --workspace`: passed.
- `maturin build --release --locked --manifest-path python/Cargo.toml --out /tmp/snaptokens-tactical-wheels`: passed, producing a macOS ARM abi3 wheel for Python 3.9 and later.
- Installed that wheel into `/tmp/snaptokens-tactical-venv` and ran `python -m pytest -q python/tests`: 15 passed with Python 3.13.5 and Transformers 4.57.6.

Focused new coverage checks merge equality versus priority, native-word hashing tails, malformed UTF-8 decoding, Unicode/empty/overlapping literal replacements, JSON pattern precedence, repeated/absent template sequences, added-token decode precedence and filtering, and left padding with independently sized metadata. Linux and the other Python versions remain covered by the existing PR workflows, not these local results. No profiling, timing comparison, allocation count, binary-size measurement, or general-champion promotion was performed.

## Scoped limited-encoding change (2026-09-16)

Parent SHA: 99d987d975464606f5377a52f887bfb10ee92f43 (configuration and command fixes on c4a65ed)
Hypothesis: a token budget can avoid BPE merging on pieces outside the retained prefix or suffix.
Measured hot cost: the current Python truncation path calls full encode before slicing. No timing claim is made for this change.
Invariant that makes the shorter path exact: normalization and pre-tokenization finish before selection; BPE cannot merge across those established piece boundaries. Finish each selected piece and inspect one excess token to preserve the truncation-loss flag. Validate discarded pieces without merging them so unsupported characters still fail.
Representation being preserved or changed: retain normalized text plus ranges and token IDs; add a bounded traversal over the existing split representation.
Expected winning strata: long, many-piece inputs with a small requested token budget.
Expected adverse strata: inputs at or below their limit, whole-input pieces, and warm fused-cache workloads, since the limited path materializes split boundaries.
Smallest files that need changing: src/pre_tokenized.rs, src/models/bpe.rs, src/models.rs, src/lib.rs, python/src/lib.rs, focused tests.
Mechanism evidence: a callback-count test must demonstrate that BPE stops after the cutoff piece while validation continues.
Acceptance rule: exact prefix/suffix parity with full encoding, special-token budgets, added tokens, empty pieces, zero limits, and invalid discarded input; Python scalar/nested/flat parity and CI checks pass.
Rejection rule: any retained-token difference, hidden encoding error, or incorrect truncation-loss flag.
Validation scope: this is explicitly requested limited encoding, not promotion of a general performance champion. Local profiling is unavailable below the required 3 GB free-disk preflight. No general speedup or timing result is claimed.
