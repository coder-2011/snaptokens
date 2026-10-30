# D7 added-token membership: retained locally with a hit-heavy tradeoff

Candidate `ec7e22154ad49ad3f05d9028e6f3bc84fc36b6ab` versus retained D6 `90423a9079c1d147b1ac2ec3b493ee4ecce493ae`. Added-heavy diagnostic parent is evidence-only outcome9dc98aad with identical runtime. One unused bit in the existing flags byte rejects absent IDs before hashing; hits retain the original content HashMap. No table allocation, object-layout change, unsafe code, dependency, format change or heuristic dispatch is introduced. Construction adds a constant OR per configured entry.

The complete twelve-pair primary run measures **1.330192 [1.296248, 1.348919]**. Families: GPT-2 **1.294102 [1.268990, 1.308068]**, Gemma **1.138501 [1.116852, 1.151877]**, T5 **1.597499 [1.493775, 1.664578]**. GPT-2 scalar short/long are1.455987/1.413754; T5 scalar1.736423/1.822796; Qwen scalar1.952902/2.123444; GPT-OSS scalar1.689157/1.640767. All28 cell points exceed their historical mirrored A/A context and every cell interval exceeds1. Primary AB/BA agree at1.329781/1.322916. There are still three losing individual pairs: GPT-2 short/stream1 rounds0/5 are0.996864/0.968569 and long/stream1 round1 is0.992711. They remain in the full672measurement/280fixture/32synthetic records.

**This is not a universal decode improvement.** The separately frozen nine-cell diagnostic finds an explicit added-heavy regression. Ordinary direct misses improve5.802063 [5.471012,5.912364], saving6.147ns/lookup, while sparse direct hits add0.473ns/lookup. Compact direct hits remain inconclusive. Full added-heavy rows lose roughly3–8% throughput and add58–151ns per128-ID row, with69/72 full-decode pairs slower. All six full-decode cell intervals are below1; five ordinary-hit full cells lie below their own mirrored A/A lower edges. Compact batch and both sparse full cells lose12/12 pairs. Compact and sparse losses are similar, without a separate catastrophic sparse-only cliff. These are supported costs, not noise and not hidden in the primary aggregate.

The design is retained locally because the directly removed hashing work produces large, consistent ordinary/mixed-ID gains across all measured families, while its extra indexed read has a bounded, measured hit-heavy cost and adds no retained allocation. This accepts that workload tradeoff explicitly; it does not promise non-regression for hit-dominated clients. No post-result density/ID threshold was added. Gemma's fixed scalar pools include225/3144 plain added-token occurrences, so the primary result is not described as an all-miss pool. Main timings use skip_special=true; the added-heavy screen separately includes skip=false and compact skip=true. Every cell is reported separately without inventing a combined utility score.

The added diagnostic has its own pre-candidate parent-only pilot and identical/independent A/A, evaluator83063ae3 and frozen protocol74f2bae2…. Its independent miss control falsely improves1.055103 [1.013219,1.101292]; candidate confidence intervals alone are not treated as causation. Both controls and all per-cell absolute costs/order splits remain in added-baseline. Candidate stress run has216measurements/468complete parity gates and exactly the frozen twelve balanced pairs. Compact full outputs use same-ID HF parity; sparse IDs use an explicit persisted spelling remap. Both project crates explicitly recompile after release cleaning, and accepted parent/candidate actual locks match. No primary evaluator change or further D7 rerun.

All189 release workspace tests pass (nine existing ignored), including19 focused added-token tests; strict Clippy, formatting and warning-denied docs pass. New coverage verifies all-false entries, ID0, sparse neighbors, arbitrary unknown IDs and duplicate-ID last-writer behavior clearing prior word/strip/special flags while preserving old matcher patterns. Existing added overrides, JSON/direct-ST and full scalar/batch/stream parity remain exact. All28 allocation cells and84 resource cases match D6 exactly. A separate profile removes added-ID hash frames from ordinary GPT-2 decode; ByteLevel conversion and checked lookup/reference collection now dominate. Sample proportions are not used as speed measurements.

Independent reviews recompute both raw summaries and verify provenance, work/order, all gates and resources. Earlier primary A/A used slower binaries and is imperfect for current short timings; the intended gains greatly exceed that observed context. These are Apple M2 repeated-pool results. AMD/Intel, sealed holdout, full release/package matrix, physical RSS, construction timing and general promotion remain unmeasured. No upstreaming is authorized.

Full-process load1 spans 2.29–4.17. Executable size changes from 8,057,008 to 8,056,368 bytes (-640).

| Primary/guard cell | Candidate / D6 [95% CI] |
|---|---:|
| gpt-2/short/scalar | 1.455987 [1.429181, 1.516904] |
| gpt-2/short/batch32 | 1.445366 [1.433279, 1.465257] |
| gpt-2/short/stream1 | 1.098719 [1.039020, 1.099927] |
| gpt-2/short/stream8 | 1.253675 [1.224223, 1.299525] |
| gpt-2/long/scalar | 1.413754 [1.380034, 1.440418] |
| gpt-2/long/batch32 | 1.403224 [1.388208, 1.409478] |
| gpt-2/long/stream1 | 1.084643 [1.070437, 1.094816] |
| gpt-2/long/stream8 | 1.261113 [1.219061, 1.269849] |
| gemma-3/short/scalar | 1.166536 [1.132733, 1.177783] |
| gemma-3/short/batch32 | 1.169134 [1.156268, 1.176205] |
| gemma-3/short/stream1 | 1.094823 [1.074812, 1.108249] |
| gemma-3/short/stream8 | 1.128550 [1.099012, 1.137537] |
| gemma-3/long/scalar | 1.167323 [1.132102, 1.176998] |
| gemma-3/long/batch32 | 1.170710 [1.121192, 1.210226] |
| gemma-3/long/stream1 | 1.080248 [1.053569, 1.126996] |
| gemma-3/long/stream8 | 1.134686 [1.092831, 1.172578] |
| t5-small/short/scalar | 1.736423 [1.592295, 1.826860] |
| t5-small/short/batch32 | 1.753556 [1.601304, 1.850663] |
| t5-small/short/stream1 | 1.271189 [1.159327, 1.375464] |
| t5-small/short/stream8 | 1.571817 [1.450647, 1.638272] |
| t5-small/long/scalar | 1.822796 [1.663735, 1.922048] |
| t5-small/long/batch32 | 1.809501 [1.679086, 1.961256] |
| t5-small/long/stream1 | 1.311902 [1.234113, 1.345949] |
| t5-small/long/stream8 | 1.611164 [1.561371, 1.674813] |
| qwen-3/short/scalar | 1.952902 [1.817172, 2.181985] |
| qwen-3/long/scalar | 2.123444 [1.971984, 2.376242] |
| gpt-oss/short/scalar | 1.689157 [1.543603, 1.774240] |
| gpt-oss/long/scalar | 1.640767 [1.572052, 1.896903] |

| Separate added diagnostic cell | Ratio [95% CI] | Candidate minus parent time [95% CI] |
|---|---:|---:|
| lookup/compact-hits | 1.010815 [0.991722, 1.024946] | -0.087 [-0.203, +0.066] ns/lookup |
| lookup/sparse-hits | 0.949172 [0.930630, 0.982597] | +0.473 [+0.157, +0.634] ns/lookup |
| lookup/in-range-misses | 5.802063 [5.471012, 5.912364] | -6.147 [-6.566, -5.705] ns/lookup |
| decode/compact/scalar128 | 0.942180 [0.911399, 0.963474] | +106.731 [+65.142, +164.047] ns/row |
| decode/compact/batch32 | 0.925942 [0.918660, 0.949543] | +138.585 [+93.554, +150.814] ns/row |
| decode/sparse/scalar128 | 0.941168 [0.930151, 0.946607] | +118.345 [+105.775, +139.879] ns/row |
| decode/sparse/batch32 | 0.926081 [0.916903, 0.963268] | +150.939 [+73.550, +170.140] ns/row |
| decode/compact/scalar128-skip | 0.938594 [0.905425, 0.947780] | +113.736 [+96.127, +179.444] ns/row |
| decode/repeated-special/scalar128 | 0.971509 [0.928051, 0.993822] | +57.924 [+12.352, +152.269] ns/row |
