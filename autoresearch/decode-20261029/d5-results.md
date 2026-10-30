# D5 borrowed terminal Metaspace: retained locally

Candidate `c23b8a2586f905a56279b91d145cb4857e5c1095` versus immediate retained D4 parent `c373e8b2bb79a2563ea95e528278944b8db7ca14`. Independent retention review supports bounded local retention. The two-file runtime change borrows surviving spellings, computes the exact transformed output length, then writes one final String. Owned `decode_chain`, Sequence boundaries, encoding, formats and public APIs remain unchanged.

The complete twelve-pair run contains 672 measurements, 280 fixture parity passes and 32 synthetic parity passes. Primary ratio is 1.241769 [1.230751, 1.253648]; T5 family is 1.949152 [1.920313, 1.983333]. T5 scalar short/long are 2.251980/2.173298; batch32 2.257898/2.118301; stream1 1.521754/1.533148; stream8 1.980175/1.926451. T5 family AB/BA estimates are 1.953954/1.950644. All eight T5 cell intervals exceed one; 95 of 96 individual T5 pairs improve. The remaining short/stream1 pair, zero-based round 6, is 0.952347 and remains in the data.

Losses are not hidden by the aggregate. GPT-2 family is 0.995839 [0.985510, 1.010423], with short/stream1 0.975086 [0.960279, 0.992434]. Gemma family is 0.986478 [0.969049, 1.001332], including short/batch32 0.966569 [0.951894, 0.998316] and long/scalar 0.968826 [0.916815, 0.999875]. Gemma long/batch32 is 0.967192 [0.956815, 1.009865], below its historical A/A point-envelope lower edge of 0.976915. Qwen short/scalar is likewise below its 0.930786 A/A point edge. These are adverse observations, not demonstrated equivalence, automatically noise or proven causal regressions. Some order effects are material: GPT-2 family AB/BA is 1.020915/0.977137. Qwen guard points are 0.922424/0.960311 with wide intervals spanning one; GPT-OSS guards are 1.063480/1.048032 with similarly wide intervals. The minimum single-pair ratio is Qwen long/scalar round 10 at 0.710080. Every observation remains; no rerun, round removal or score adjustment.

Host load1 ranges 2.68–3.60, below the previous completed A/A ranges (identical 4.45–8.35, independent 3.65–5.34) and their maxima. This is a different host-load condition, not an exact matched calibration. Identical/independent A/A primary ratios were 0.989409/0.992530; their systematic variation means bootstrap intervals alone do not establish causation. Those controls also used the slower pre-D2 runtime, so they do not perfectly describe today's shorter timed cells. The intended T5 gains are much larger than observed calibration variation, with a directly measured ownership-removal mechanism. Smaller untouched-path losses remain a local tradeoff to assess independently, not a reason to claim a portable general win.

All 188 release workspace tests pass, with nine existing ignored tests, plus focused differential tests, strict Clippy, formatting and warning-denied docs. The four focused Metaspace tests cover arbitrary marker widths, Always/Never, empty and first-empty tokens, marker-heavy text, Unicode, unknown/special filtering, BPE/Unigram and configured added overlaps. Full fixture parity covers JSON/direct-ST and both skip settings before and after timing.

The instrumented build reduces short/long T5 scalar allocations from 9,800/66,796 to 128/64; reallocations fall from 171/2,046 to zero. All 512 valid Metaspace resource records pass HF output and balanced allocation accounting, with no returned-capacity, peak tracked-heap or requested-traffic increases. The 84 ByteLevel controls are identical to D4. These are requested allocation measurements, not physical RSS. Separate profiling confirms per-token temporary String construction, transformed vectors and final joining disappear; lookup and character conversion now dominate. See d5-mechanism.md for sample scope.

The first D5 resource builds incorrectly reused a cached D4 dependency through same-named source symlinks. Their outputs and binaries are marked INVALID and preserved; they were caught by the mechanism gate before throughput timing. Only `metaspace-resource-d5-02` and `resource-d5-02` are valid candidate resource data. Explicit project-package cleaning forced recompilation, and symbol/build/hash checks independently confirm D5. The normal and instrumented evaluator builds explicitly compiled D5 and were unaffected. The complete audit and rebuild trail are included.

Executable size increases 28,672 bytes to 8,057,152; __TEXT increases 16,384 bytes to 4,931,584 and __DATA remains 32,768. No retained tokenizer state, construction pass, dependency or unsafe code is added. The exact output reserve avoids D1's sparse-output over-allocation failure.

This is Apple M2 repeated-input decode evidence. AMD/Intel, sealed holdout, full release/package matrix, standalone physical RSS and construction timings remain unmeasured. Long singleton ASCII and Unicode tokens without markers have a plausible throughput loss from the added full character-count pass; the parent already moves its single final piece without joining. Resource parity does not measure that cost. This shape needs a separately frozen adverse timing screen before broader promotion or any blanket Metaspace non-regression claim. Other adverse resource cases likewise have no throughput claim. No general champion promotion, push, PR or release is authorized.

| Cell | Candidate / D4 [95% CI] |
|---|---:|
| gpt-2/short/scalar | 1.017021 [0.989614, 1.040151] |
| gpt-2/short/batch32 | 0.994645 [0.975046, 1.009292] |
| gpt-2/short/stream1 | 0.975086 [0.960279, 0.992434] |
| gpt-2/short/stream8 | 0.990604 [0.968786, 1.044474] |
| gpt-2/long/scalar | 0.997527 [0.977465, 1.040271] |
| gpt-2/long/batch32 | 0.993939 [0.980953, 1.025094] |
| gpt-2/long/stream1 | 1.005488 [0.980298, 1.019073] |
| gpt-2/long/stream8 | 0.992911 [0.981054, 1.009258] |
| gemma-3/short/scalar | 0.993676 [0.972264, 1.010759] |
| gemma-3/short/batch32 | 0.966569 [0.951894, 0.998316] |
| gemma-3/short/stream1 | 0.993825 [0.983121, 1.022738] |
| gemma-3/short/stream8 | 1.001056 [0.951311, 1.021786] |
| gemma-3/long/scalar | 0.968826 [0.916815, 0.999875] |
| gemma-3/long/batch32 | 0.967192 [0.956815, 1.009865] |
| gemma-3/long/stream1 | 1.004956 [0.959166, 1.025065] |
| gemma-3/long/stream8 | 0.996665 [0.983080, 1.023154] |
| t5-small/short/scalar | 2.251980 [2.224163, 2.370269] |
| t5-small/short/batch32 | 2.257898 [2.241334, 2.323608] |
| t5-small/short/stream1 | 1.521754 [1.498601, 1.581157] |
| t5-small/short/stream8 | 1.980175 [1.897079, 2.045592] |
| t5-small/long/scalar | 2.173298 [2.127012, 2.243052] |
| t5-small/long/batch32 | 2.118301 [2.073545, 2.166262] |
| t5-small/long/stream1 | 1.533148 [1.479846, 1.560354] |
| t5-small/long/stream8 | 1.926451 [1.889654, 1.943131] |
| qwen-3/short/scalar | 0.922424 [0.883290, 1.067099] |
| qwen-3/long/scalar | 0.960311 [0.831274, 1.117979] |
| gpt-oss/short/scalar | 1.063480 [0.893736, 1.137909] |
| gpt-oss/long/scalar | 1.048032 [0.838837, 1.153050] |
