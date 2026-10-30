# D9 borrowed first Replace: retained locally

Candidate `975ff641c8dbe4ac77e7078e249f0d8429c10809` versus retained D7 `ec7e22154ad49ad3f05d9028e6f3bc84fc36b6ab`. Only a top-level Sequence whose first stage is Replace borrows original vocabulary text into that stage. Owned replacement results move directly into the same vector; unchanged tokens are cloned once. All remaining owned stages, public owned decoding, empty-token semantics, output capacities and constructor/format behavior remain unchanged.

Gemma improves **1.197188 [1.189301, 1.204589]** across its eight equally weighted decode cells. All 96 paired Gemma observations improve, and every cell interval clears its historical A/A upper context. Short/long scalar are **1.196387 / 1.262399**, batch **1.186021 / 1.244304**. Both execution orders support the gain. The full primary matrix is **1.071689 [1.064460, 1.081914]**, with AB/BA 1.072317/1.072752; this aggregate includes favorable unrelated controls and is not attributed entirely to the ownership mechanism.

GPT-2 family is 1.005182 with an interval spanning 1; Qwen/GPT-OSS scalar intervals also span 1. T5 family is 1.022819, with short scalar/batch 1.072290/1.081218. This repeats the unexplained favorable T5 pattern seen in rejected D8 against D7, despite an unchanged Metaspace algorithm. Preserve it without a T5 ownership-speedup claim. T5 short/long stream1 points are 0.996225/0.993808 with intervals spanning 1. All 91 losing individual pairs (70 primary), including GPT-OSS short 0.773053 and T5 long stream8 0.844486, and every favorable outlier remain in the raw pool. Uncertain controls are not declared equivalent.

The allocation mechanism is exact: Gemma short/long scalar calls fall from **6,423/54,652 to 4,498/35,843**, precisely the predeclared **1,925/18,809** removed original source clones. Requested bytes fall by 13,795/147,421, with unchanged reallocations. All eight Gemma allocation cells improve; the other twenty remain identical. All 84 ByteLevel resource cases are identical.

The separately frozen 772-record warmed Replace guard passes full HF scalar/batch parity and balanced output destruction. It has 508 identical and 264 improved records. Independent recomputation confirms exactly one removed source allocation/deallocation and its byte length per nonempty matched token, multiplied by batch size, with unchanged reallocations/output capacities and no requested or peak heap increase. All 28 baseline records with nonzero empty-string capacity remain valid and unchanged in output capacity. Standalone Replace and no-match controls remain identical. This is warmed allocation evidence, not first-use regex-scratch or pure no-match throughput evidence.

All 190 workspace tests (nine existing ignored), six focused Replace tests, strict Clippy, warning-denied docs and fmt pass. New focused coverage checks every suffix decoder variant, nested/reordered/empty controls, special/unknown/present-empty IDs, public owned strings and complete owned piece vectors. Complete fixed twelve-pair timing has 672 measurements, 280 fixture and 32 synthetic parity gates. Builds, immutable artifacts, protocol/fixtures/locks, full scores, work/order and resource deltas pass independent review. No additional timing is needed.

The fresh unscored Gemma profile has 3,611 main-thread samples: initial lookup/replacement collection contains 2,634, including 1,366 at the replacement call, 859 at lookup (564 UTF-8 validation), 205 at unchanged-token allocation and 55 at copying. Remaining stages contain 975, including 822 Fuse/join and 153 ByteFallback. These enclosing/descendant counts overlap and are not additive speed predictions. Repeated search preparation, lookup validation and remaining ownership are still open costs.

Full-process load1 spans 2.832–4.046; timed-process starts span 2.832–4.046. Executable size changes from 8,056,368 to 8,057,504 bytes (+1,136). No new state, retained allocation, unsafe code, dependency or load pass is introduced. Historical primary A/A used slower binaries and remains imperfect for current durations. This is bounded local Apple M2 repeated-pool retention; AMD/Intel, holdout, full release/package matrix, first-use and pure no-match throughput remain unmeasured. No upstreaming or blanket non-regression claim.

| Cell | Candidate / D7 [95% CI] | AB / BA |
|---|---:|---:|
| gpt-2/short/scalar | 0.993841 [0.980125, 1.037854] | 0.985990 / 1.005034 |
| gpt-2/short/batch32 | 1.020818 [0.971478, 1.048307] | 1.020818 / 1.028110 |
| gpt-2/short/stream1 | 0.996087 [0.987534, 1.006787] | 0.989475 / 1.003184 |
| gpt-2/short/stream8 | 1.008485 [0.990923, 1.048174] | 1.009975 / 0.997862 |
| gpt-2/long/scalar | 1.001746 [0.989065, 1.018390] | 1.001746 / 1.002594 |
| gpt-2/long/batch32 | 1.001445 [0.992702, 1.010301] | 0.998228 / 1.004981 |
| gpt-2/long/stream1 | 1.007100 [0.978832, 1.018810] | 1.004898 / 1.011301 |
| gpt-2/long/stream8 | 1.012205 [0.998693, 1.027468] | 1.008032 / 1.015241 |
| gemma-3/short/scalar | 1.196387 [1.183635, 1.205093] | 1.188271 / 1.196387 |
| gemma-3/short/batch32 | 1.186021 [1.157501, 1.196080] | 1.165409 / 1.196112 |
| gemma-3/short/stream1 | 1.135893 [1.095746, 1.154438] | 1.120158 / 1.140464 |
| gemma-3/short/stream8 | 1.194255 [1.182767, 1.241634] | 1.189703 / 1.202217 |
| gemma-3/long/scalar | 1.262399 [1.242874, 1.279328] | 1.238975 / 1.275167 |
| gemma-3/long/batch32 | 1.244304 [1.240359, 1.275091] | 1.244304 / 1.256311 |
| gemma-3/long/stream1 | 1.150721 [1.134124, 1.181316] | 1.150707 / 1.153166 |
| gemma-3/long/stream8 | 1.212848 [1.190630, 1.232881] | 1.219324 / 1.209846 |
| t5-small/short/scalar | 1.072290 [1.047510, 1.116743] | 1.093749 / 1.063856 |
| t5-small/short/batch32 | 1.081218 [1.047986, 1.125525] | 1.114217 / 1.062726 |
| t5-small/short/stream1 | 0.996225 [0.993899, 1.024140] | 0.996382 / 0.995542 |
| t5-small/short/stream8 | 1.012938 [0.985197, 1.016302] | 1.025260 / 1.006342 |
| t5-small/long/scalar | 1.010787 [0.965953, 1.018248] | 0.981629 / 1.012553 |
| t5-small/long/batch32 | 1.016128 [1.001362, 1.050095] | 1.028893 / 1.013978 |
| t5-small/long/stream1 | 0.993808 [0.971361, 1.011470] | 1.021421 / 0.988251 |
| t5-small/long/stream8 | 1.003030 [0.998626, 1.073662] | 1.022782 / 1.000375 |
| qwen-3/short/scalar | 1.007764 [0.975612, 1.037162] | 0.995795 / 1.007764 |
| qwen-3/long/scalar | 0.999922 [0.984869, 1.016757] | 0.991910 / 1.004878 |
| gpt-oss/short/scalar | 1.008287 [0.988608, 1.029663] | 1.017866 / 1.008121 |
| gpt-oss/long/scalar | 1.000583 [0.985284, 1.008228] | 1.000583 / 1.000097 |
