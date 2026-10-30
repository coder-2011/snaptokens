# D4 checked byte consumer: retained locally

Candidate `c373e8b2bb79a2563ea95e528278944b8db7ca14`; immediate parent `e8df10f1e09571fea7397309097df53acc3dc61d`. This complete composition includes the safe byte mapper and checked stored-byte lookup. It is measured against retained D2; standalone D3 remains rejected with both pools preserved.

Twelve paired rounds complete with672 measurements,280 fixture and32 synthetic parity passes. Primary ratio1.133358[1.120269,1.146426]; GPT-2 family1.425020[1.374763,1.449910]. GPT-2 scalar short/long are1.556127/1.630291. Qwen guards1.344593/1.463516 and GPT-OSS1.542571/1.673186. Every one of the144 affected cell-pairs improves; both execution orders agree. These ratios compare D4 with D2 only. Do not multiply earlier campaign ratios and call the product a measured original-baseline result.

Untouched Gemma1.013322[0.992410,1.036231] and T5 1.008168[0.977357,1.042179] do not establish attributable gains. The earlier Gemma-wide negative pattern does not repeat. Small point losses remain in the table and raw records, including Gemma long/stream1 and several T5 streams. T5 short/batch32 round11 has ratio0.659768 and is retained without filtering. Host load1 spans2.83–5.45, within the completed calibration context. Earlier A/A used the slower pre-D2 source, so its noise distribution is informative but imperfect for current shorter cell durations. The large intended-path gains and complete adverse controls support local retention despite that limitation.

All84 resource records and28 allocation-cell counts are identical to D2, including returned capacity and peak tracked heap. Eleven focused tests and186 release workspace tests pass (nine existing ignored), strict Clippy/fmt/diff checks and warning-denied docs pass. Independent source and raw retention reviews approve. Sampling verifies per-token vocabulary UTF-8 checks disappear while final aggregate validation remains; byte conversion and added-ID hashing are now the largest observed costs. Executable grows3376 bytes to8,028,480; Mach-O text/data segment sizes remain4,915,200/32,768. No retained tokenizer state or construction pass is added.

Public string lookup and every snapshot span validation remain unchanged. Focused constructor tests cover malformed UTF-8 and split/invalid spans; existing JSON/direct-ST parity and wire tests pass. No new end-to-end valid-checksum corrupt-ST test was added. The current throughput matrix does not time added-heavy or Unigram ByteLevel separately; those have focused semantic/resource coverage, not a speed claim.

This is local Apple M2 repeated-input decode retention. AMD/Intel, sealed holdout, full release/package matrix, standalone RSS and construction-timing confirmation are not completed. No general champion promotion or publication.

| Cell | Candidate / D2 [95% CI] |
|---|---:|
| gpt-2/short/scalar | 1.556127 [1.518168, 1.577545] |
| gpt-2/short/batch32 | 1.569380 [1.487559, 1.589564] |
| gpt-2/short/stream1 | 1.178435 [1.134407, 1.186004] |
| gpt-2/short/stream8 | 1.349194 [1.259307, 1.385338] |
| gpt-2/long/scalar | 1.630291 [1.564804, 1.696017] |
| gpt-2/long/batch32 | 1.668789 [1.607725, 1.720557] |
| gpt-2/long/stream1 | 1.182167 [1.102846, 1.224327] |
| gpt-2/long/stream8 | 1.361664 [1.338512, 1.406201] |
| gemma-3/short/scalar | 1.016564 [0.985847, 1.035692] |
| gemma-3/short/batch32 | 1.004426 [0.989291, 1.027536] |
| gemma-3/short/stream1 | 1.016482 [0.982820, 1.063635] |
| gemma-3/short/stream8 | 1.048024 [0.998257, 1.085243] |
| gemma-3/long/scalar | 1.025102 [0.995062, 1.052940] |
| gemma-3/long/batch32 | 0.999951 [0.985546, 1.034403] |
| gemma-3/long/stream1 | 0.995275 [0.985807, 1.046394] |
| gemma-3/long/stream8 | 1.001767 [0.984992, 1.017242] |
| t5-small/short/scalar | 1.012358 [0.976576, 1.044897] |
| t5-small/short/batch32 | 1.010193 [0.990055, 1.071585] |
| t5-small/short/stream1 | 1.033539 [0.972909, 1.082247] |
| t5-small/short/stream8 | 0.988515 [0.958436, 1.044230] |
| t5-small/long/scalar | 1.030203 [0.961364, 1.055953] |
| t5-small/long/batch32 | 1.007312 [0.977805, 1.032739] |
| t5-small/long/stream1 | 0.991436 [0.977142, 1.029400] |
| t5-small/long/stream8 | 0.992803 [0.946577, 1.024825] |
| qwen-3/short/scalar | 1.344593 [1.213898, 1.485736] |
| qwen-3/long/scalar | 1.463516 [1.176574, 1.581684] |
| gpt-oss/short/scalar | 1.542571 [1.510647, 1.620468] |
| gpt-oss/long/scalar | 1.673186 [1.613295, 1.718290] |
