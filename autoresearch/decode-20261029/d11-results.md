# D11 terminal borrowed pieces: retained locally

Candidate `04d8a45ba07e16357806d2690d7b0a30b114d3c2` versus retained D9 `975ff641c8dbe4ac77e7078e249f0d8429c10809`. The exact top-level [Replace, ByteFallback, Fuse] route retains unchanged text as Cow through byte fallback and joins the final pieces directly. All Replace calls still finish first; the existing byte parser, empty separators, whole-run invalid repair, valid-buffer transfer and singleton output capacity remain exact. Other pipeline shapes and the public owned decoder remain unchanged. Two existing metadata vectors remain; Fuse's one-element Vec is removed.

The complete primary comparison is **1.054985 [1.042813, 1.068932]**, AB/BA 1.048251/1.061256. Gemma is **1.173037 [1.153334, 1.193871]**: all 96 target pairs improve and every one of its eight cell intervals clears its historical own-cell A/A context. Scalar short/long are1.204488/1.148740; batch short/long1.220187/1.150055. This is useful supported local evidence, with the following workload costs retained.

GPT-2 long stream1 is **0.954503 [0.922564, 0.968527]**, losing 9/12 pairs, AB/BA 0.940636/0.987608, about+10.074 microseconds per row. Long stream8 is **0.975455 [0.948404, 0.998113]**, losing 8/12 pairs, AB/BA 0.964996/1.004178, about+1.978 microseconds per row. Their algorithm and allocation records are unchanged; source/layout/host effects remain possible, not isolated causes. Older slower-binary A/A limits causal certainty but does not erase these observed losses. This is not a blanket non-regression claim. Bounded local retention accepts the explicit costs against consistent 14.9–22.0% target-cell gains and a small decoder-only ownership change; no invented utility score or fixed threshold is used.

GPT-2 family 1.003063 and T5 family 0.997927 have intervals spanning one. Favorable unrelated GPT-2 short scalar/batch1.044464/1.065243, Qwen short1.092107 and GPT-OSS short/long1.055685/1.020559 are preserved without attributing them to Cow ownership. The combined reward therefore is not a pure causal measure of the terminal change. There are 103 losing pairs overall, 93 primary: 44 GPT-2, 49 T5, 6 Qwen, 4 GPT-OSS. No rounds, outliers or controls are filtered. Process load1 ranges 2.225–3.583. Binary grows 1,760 bytes to 8,059,264 bytes.

The allocation mechanism is exact. Gemma scalar short4, 498→2,117 and long 35,843→18,934 match the frozen predictions. All eight Gemma allocation cells improve and 20 controls match exactly; all 84 ByteLevel resource records match. Under unchanged Replace guard 802cb4bb, 386 standalone measurements are identical and 386 terminal records match the predeclared allocation-call/requested-byte formula. All returned capacities/live heap match; 108 records have lower peaks and 664 equal peaks, with no peak/requested-heap growth. Both resource phases return to balanced ownership after destruction. These are warmed output-resource measurements, not constructor, first-use regex or RSS evidence.

All 192 workspace tests pass (147 library, 41 integration, 3 Python binding, 1 doc; nine preexisting ignored), four focused ByteFallback tests, strict Clippy, warning-denied docs and fmt. Focused coverage includes 256 singleton bytes, valid and invalid UTF-8 runs, a 4097-byte invalid run with a valid prefix, empty/filter separators, mixed borrowed/owned inputs with deliberately retained spare capacity, and an extra trailing decoder that must not be skipped. Complete 672 timings, 280 fixture and 32 synthetic parity gates pass with immutable matching builds/locks/inputs and the unchanged evaluator 123209fb/protocol 05c15a78. Independent audits recompute the scorer/bootstrap, every resource formula and provenance.

The fresh separate Gemma profile has 3,569 main-thread samples. Replace encloses 1,516, with 459 inclusive searcher-setup samples (406 self) and 435 active search samples. The retained lookup and final copying remain work. These overlapping diagnostic counts do not predict a speedup. They support the next independently guarded literal-Finder experiment, which changes shared encoding behavior and constructor state and therefore needs its own encode/construction controls first.

Retained only on local Apple M2 repeated pools. No extra D11 timing, AMD/Intel confirmation, holdout, full release matrix, general promotion or upstreaming. Preserve all raw data and practical limits.

| Cell | Candidate / D9 [95% CI] | AB / BA |
|---|---:|---:|
| gpt-2/short/scalar | 1.044464 [1.021689, 1.070907] | 1.043760 / 1.055024 |
| gpt-2/short/batch32 | 1.065243 [1.041465, 1.108760] | 1.059357 / 1.079470 |
| gpt-2/short/stream1 | 1.001318 [0.949073, 1.010872] | 0.984707 / 1.012918 |
| gpt-2/short/stream8 | 0.999079 [0.985628, 1.023875] | 0.998168 / 1.012628 |
| gpt-2/long/scalar | 1.002679 [0.936947, 1.011103] | 0.972921 / 1.009295 |
| gpt-2/long/batch32 | 0.986196 [0.961661, 1.033164] | 0.974775 / 1.010157 |
| gpt-2/long/stream1 | 0.954503 [0.922564, 0.968527] | 0.940636 / 0.987608 |
| gpt-2/long/stream8 | 0.975455 [0.948404, 0.998113] | 0.964996 / 1.004178 |
| gemma-3/short/scalar | 1.204488 [1.189331, 1.258801] | 1.203162 / 1.204488 |
| gemma-3/short/batch32 | 1.220187 [1.182439, 1.264788] | 1.254085 / 1.158677 |
| gemma-3/short/stream1 | 1.177168 [1.124964, 1.202712] | 1.184892 / 1.149504 |
| gemma-3/short/stream8 | 1.162694 [1.137829, 1.175897] | 1.157776 / 1.164945 |
| gemma-3/long/scalar | 1.148740 [1.113849, 1.173734] | 1.158262 / 1.139066 |
| gemma-3/long/batch32 | 1.150055 [1.123178, 1.180588] | 1.150055 / 1.143849 |
| gemma-3/long/stream1 | 1.174159 [1.140585, 1.205167] | 1.172157 / 1.181221 |
| gemma-3/long/stream8 | 1.148930 [1.121480, 1.188305] | 1.161572 / 1.139707 |
| t5-small/short/scalar | 1.005441 [0.979667, 1.015934] | 1.001671 / 1.009348 |
| t5-small/short/batch32 | 1.000739 [0.968085, 1.038958] | 1.010941 / 0.984913 |
| t5-small/short/stream1 | 0.999705 [0.986305, 1.033098] | 0.999705 / 1.011749 |
| t5-small/short/stream8 | 0.991693 [0.937043, 1.028125] | 0.971246 / 1.006783 |
| t5-small/long/scalar | 1.001193 [0.961387, 1.037085] | 0.982901 / 1.010871 |
| t5-small/long/batch32 | 1.001986 [0.968098, 1.040631] | 0.982133 / 1.019404 |
| t5-small/long/stream1 | 0.989897 [0.955062, 1.036900] | 0.970774 / 1.008877 |
| t5-small/long/stream8 | 0.992877 [0.973602, 1.053222] | 0.957466 / 1.021962 |
| qwen-3/short/scalar | 1.092107 [1.048746, 1.112955] | 1.086771 / 1.096300 |
| qwen-3/long/scalar | 1.003579 [0.973473, 1.048188] | 1.003579 / 1.008483 |
| gpt-oss/short/scalar | 1.055685 [1.024051, 1.081008] | 1.030124 / 1.097071 |
| gpt-oss/long/scalar | 1.020559 [1.004721, 1.050289] | 1.016399 / 1.020559 |
