Decode optimization on Apple M2, 2026-10-29 UTC. D2 `e8df10f1e09571fea7397309097df53acc3dc61d` is retained locally against `123209fba6439a043529425f86e40b087f8ede62`. This is a fixed rotating-input decode result, not a general tokenizer champion or a cross-CPU claim. Nothing was pushed, published or released.

The candidate borrows vocabulary spellings into a temporary reference vector, reserves the same spelling-byte capacity as the parent, and applies the existing ByteLevel conversion. It removes per-token String ownership while keeping public APIs, output text, added-token precedence, special filtering, sequence handling and owned-token destruction unchanged.

All 12 paired rounds completed: 672 measurements, 280 fixture parity gates and 32 synthetic parity gates. The primary equal-weight aggregate is **1.19878x [1.19031, 1.20948]**. GPT-2 across scalar/batch/stream cells is **1.64499x [1.61325, 1.68750]**. The four ByteLevel scalar guards also improve clearly. The apparent T5 gain is inside its calibrated family variation and is not attributed to this mechanism.

Identical-binary control: 0.989409x [0.980727, 0.996422]. Independent same-source build: 0.992530x [0.976876, 1.005049]. The identical control excludes one despite identical code, so the naive bootstrap alone understates environmental/systematic uncertainty. All control results, intervals, label-reversed envelopes and per-cell observations remain in calibration.json. Candidate gains in the changed ByteLevel cells clearly exceed those controls. There is no fixed percentage floor or automatic decision from a positive estimate.

Every observed cell is reported below. Intervals are paired 95% bootstrap intervals from the frozen scorer; unchanged-path movement is not causal evidence. Gemma long stream8 has a 0.98442x point with an interval spanning one, and several other unchanged cells permit losses. Individual slow rounds remain in the raw files.

| Cell | Candidate / parent | 95% interval |
| --- | ---: | --- |
| gpt-2/short/scalar | 1.79471x | [1.71033, 1.81402] |
| gpt-2/short/batch32 | 1.80577x | [1.78905, 1.84845] |
| gpt-2/short/stream1 | 1.46834x | [1.45731, 1.56373] |
| gpt-2/short/stream8 | 1.61123x | [1.57918, 1.68930] |
| gpt-2/long/scalar | 1.75199x | [1.72033, 1.81505] |
| gpt-2/long/batch32 | 1.70806x | [1.67543, 1.78430] |
| gpt-2/long/stream1 | 1.45808x | [1.39054, 1.49246] |
| gpt-2/long/stream8 | 1.60272x | [1.57949, 1.62866] |
| gemma-3/short/scalar | 1.03141x | [1.00842, 1.04023] |
| gemma-3/short/batch32 | 1.01792x | [1.00006, 1.02905] |
| gemma-3/short/stream1 | 1.00794x | [0.99174, 1.03373] |
| gemma-3/short/stream8 | 1.00393x | [0.97789, 1.03201] |
| gemma-3/long/scalar | 1.00371x | [0.96798, 1.02263] |
| gemma-3/long/batch32 | 1.01906x | [0.99137, 1.03499] |
| gemma-3/long/stream1 | 1.00408x | [0.97220, 1.01242] |
| gemma-3/long/stream8 | 0.98442x | [0.97738, 1.02827] |
| t5-small/short/scalar | 1.06458x | [1.02890, 1.08456] |
| t5-small/short/batch32 | 1.06111x | [1.02912, 1.07687] |
| t5-small/short/stream1 | 1.02423x | [0.99927, 1.05522] |
| t5-small/short/stream8 | 1.01750x | [1.00595, 1.03875] |
| t5-small/long/scalar | 1.02597x | [1.00822, 1.07014] |
| t5-small/long/batch32 | 1.04584x | [1.01198, 1.08535] |
| t5-small/long/stream1 | 1.04448x | [1.02432, 1.06795] |
| t5-small/long/stream8 | 1.02103x | [1.01084, 1.05806] |
| qwen-3/short/scalar | 1.63751x | [1.52273, 1.79193] |
| qwen-3/long/scalar | 1.59197x | [1.46667, 1.74816] |
| gpt-oss/short/scalar | 1.64859x | [1.55602, 1.70681] |
| gpt-oss/long/scalar | 1.56039x | [1.48135, 1.62847] |

GPT-2 short scalar allocations fall from 5,917 to 128 over 64 rows; long scalar allocations fall from 40,644 to 64 over 32 rows. Both have zero reallocations. All 84 unchanged resource cases match HF completely, retain exactly the parent’s requested output heap, and have no increase in peak tracked heap. This measures Rust allocation requests/capacities, not allocator usable size or RSS.

The first design, D1 `9cbdb2f2`, removed more temporary allocations but reserved from the original ID count. In 4,096 sparse rows it retained 2,195,456 bytes versus the parent’s 102,400. It was rejected before throughput timing and fully reverted before D2. Guard setup errors for a nested Cargo workspace and a token beyond the existing 65,535-byte limit were corrected before any resource records; original errors remain in the external evidence directory.

Validation: eight focused ByteLevel tests; 183 release workspace tests passing with nine existing ignored; strict release workspace/all-target Clippy; formatting; warning-denied release workspace documentation; complete before/after timing parity. Independent source and retention reviews approve local retention and document uncertain control losses.

The evaluator executables are 8,023,856 bytes (parent) and 8,025,104 bytes (candidate). Both Mach-O __TEXT segments are 4,915,200 bytes and __DATA segments are 32,768 bytes. There is no new retained tokenizer state. No standalone construction/RSS, package/release, sealed holdout, AMD or Intel campaign was run; these remain requirements before general promotion.

A separate five-second CPU sample of extended GPT-2 long scalar decode attributes 1,302 of 3,613 main-thread samples to per-token UTF-8 validation during lookup, 1,134 to ByteLevel conversion (including 50 final-output validation samples), and 572 to added-token hashing. This diagnostic is unscored; its extended single-cell protocol is separate from the unchanged evaluation protocol. It motivates a byte mapper and then checked raw-byte lookup, with added-token membership and other decoder families still open.

Correctness prerequisites, separately committed before the evaluator: explicit Fuse behavior, whole-token ByteLevel fallback, optional-plus ByteFallback payloads, and empty-token Replace. The new public Decoder::Fuse variant can break downstream exhaustive matches and requires compatibility attention before future publication. The documented no-decoder concatenation behavior remains unchanged.

Raw controls, candidate rounds, manifests, allocation/resource records and source reviews are beside this report. The frozen evaluator is in ../decode-eval. Immutable binaries, complete build/test logs, fixtures, 25 inspected Ruff/uv/Bun PR patches and additional design reports remain in ~/.cache/snaptokens-decode-20261029; fixture and protocol hashes identify the exact inputs. The interrupted first control remains unscored and was not spliced into either completed control.
