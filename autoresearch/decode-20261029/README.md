# Decode PR evidence, 2026-10-30

The `decode-review/*` PR stack extracts the four correctness repairs and seven
locally retained decode experiments from the October 29–30 campaign. Every
runtime boundary is checked against its original candidate, including Rust and
Python source, tests, Cargo manifests and lockfile. No rejected candidate or
unfinished D14 stream change is included.

| PR layer | Original runtime boundary | Result records |
| --- | --- | --- |
| correctness | `5fc4efac0a4252a617a7760e7d3c62061c025c05` | four HF decoder repairs |
| bytelevel | `c373e8b2bb79a2563ea95e528278944b8db7ca14` | D2, then D4 |
| metaspace | `c23b8a2586f905a56279b91d145cb4857e5c1095` | D5 |
| literal-search | `90423a9079c1d147b1ac2ec3b493ee4ecce493ae` | D6 and its confirmation |
| added-tokens | `ec7e22154ad49ad3f05d9028e6f3bc84fc36b6ab` | D7 and added-heavy diagnostic |
| ownership | `04d8a45ba07e16357806d2690d7b0a30b114d3c2` | D9, then D11 |

The `d*-results.md` files are unchanged historical outcome records, added with
their respective PR layer. Statements there about publication authorization
describe the time of the experiment. The user subsequently authorized these
PRs, without authorizing a merge or release.

These reports are summaries, not the complete reproducibility archive. Their
references to adjacent raw rounds, guard sources, mechanism reports and the
decode evaluator refer to the original local campaign checkout. Those files
are not included in this small review stack. Full original commits, inputs,
immutable binaries and raw data remain in the local campaign and
`~/.cache/snaptokens-decode-20261029`. Original SHA names below therefore denote
local provenance, not necessarily publicly reachable GitHub commits.

The primary evaluator was frozen at
`123209fba6439a043529425f86e40b087f8ede62`, with protocol SHA-256
`05c15a7841c0a484f1854cb7e65ad3a775f1a38420e9943dc6c933058180bc2e`.
It used Apple M2, Rust 1.97 release builds, one Rayon worker, 12 counterbalanced
paired rounds, 24 equally weighted GPT-2/Gemma/T5 cells and four Qwen/GPT-OSS
scalar controls. The four APIs were scalar, batch-32 and stream chunks of 1/8
IDs, with 64 short rows up to 256 bytes and 32 long rows up to 4096 bytes.
Inputs rotate within repeated pools. Output allocation and destruction are
inside timing, with complete before/after HF 0.22.2 parity, JSON/direct-ST and
both special-token settings checked. Timing uses skip-special=true.

The score is the exponential of the mean per-cell median log throughput ratio.
Each completed pool has 672 timing records, 280 fixture parity passes and 32
synthetic parity passes. Intervals use 3000 paired bootstrap draws, seed 0.
The historical identical-binary A/A was 0.989409 [0.980727, 0.996422], and the
independently built same-source A/A was 0.992530 [0.976876, 1.005049]. Thus a
bootstrap interval excluding one does not by itself establish causality.
Fresh D11 controls were 0.995877 [0.985578, 1.004413] and 0.999455
[0.994191, 1.009332], with false per-cell movements preserved in the archive.

Every speed ratio compares the named candidate with its immediate measured
parent. Do not multiply these ratios into an unmeasured whole-stack speedup.
The supported tradeoffs include D7's 3–8% added-heavy decode loss and D11's
GPT-2 long-stream losses. D5 marker-free long singleton throughput and D6
shared encoding throughput remain unmeasured. Full per-cell losses and
resource caveats remain in the corresponding reports.

These PRs are review candidates supported by local Apple results. They are
not a general champion promotion. AMD/Intel confirmation, sealed holdout,
full package/release checks and physical RSS confirmation remain outstanding.
The older independent decode PR #43 overlaps with this stack and needs an
explicit implementation choice before either is integrated.
