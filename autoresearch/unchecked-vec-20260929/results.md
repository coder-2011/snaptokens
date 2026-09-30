# UncheckedVec local speed screen, 2026-09-29

The local Apple M2 screen establishes no reliable speedup. Warm scalar is essentially flat; the positive nested-batch estimate is comparable to a false gain from independently rebuilt identical source. Ragged and first-pass scalar intervals include no change. This is inconclusive, not proof that both implementations perform identically.

Parent runtime: `42c62992421108d0e1659ed7957188e327f469d3`. Candidate: `0fafc3e9126aec53a2aa1b0b1cc10ffac46f0ff9`. No runtime changes were made during measurement or after seeing results.

## Aggregate throughput ratios

Ratios are candidate/parent throughput, greater than 1 favors UncheckedVec. Each API weights the eleven model/corpus/thread cells equally. Paired round bootstrap uses 10,000 draws.

| Metric | Candidate | 95% CI | Identical A/A | Rebuilt A/A |
|---|---:|---|---:|---:|
| First-pass scalar | 0.996767x | [0.973920, 1.009515] | 1.007470x | 1.013822x |
| Warm scalar | 0.998391x | [0.987820, 1.005283] | 0.995407x | 1.001269x |
| Warm nested batch | 1.085111x | [1.005450, 1.207554] | 0.932465x | 1.081522x |
| Warm flat-ragged | 1.011228x | [0.966208, 1.065472] | 1.040070x | 1.017365x |

The nested-batch candidate interval is above 1, but that alone is insufficient: same-source A/A produced point estimates of 0.932465x and 1.081522x, with rebuilt A/A interval [0.977958, 1.226928]. Candidate 1.085111x therefore does not establish a calibrated gain. No additional runs were selected after seeing this result.

## Per-model warm ratios, four-worker cells

These points aggregate the two corpora, separately for each API. They are diagnostic estimates, not individual claims of significance. The one-worker GPT-OSS cell remains in the primary eleven-cell aggregate and appears separately below.

| Model | Scalar | Nested batch | Flat-ragged |
|---|---:|---:|---:|
| gpt-2 | 0.996629x | 1.147333x | 1.003783x |
| qwen-3 | 1.006341x | 1.295232x | 1.068776x |
| gpt-oss | 0.998568x | 1.037736x | 0.970505x |
| gemma-3 | 0.989945x | 0.992503x | 0.990438x |
| t5-small | 1.004889x | 1.019941x | 1.019636x |

## Complete cell results

Every measured cell and observed loss is included. Intervals are descriptive paired bootstrap intervals without multiplicity adjustment.

| Cell/API | Candidate | 95% CI | Identical A/A | Rebuilt A/A |
|---|---:|---|---:|---:|
| gemma-3/longbench/4/cold_scalar | 0.971352x | [0.947887, 1.015608] | 0.984426x | 0.980610x |
| gemma-3/longbench/4/warm_batch | 0.987690x | [0.971276, 0.993317] | 1.000577x | 0.979193x |
| gemma-3/longbench/4/warm_ragged | 0.993568x | [0.978088, 1.005673] | 1.002618x | 0.999182x |
| gemma-3/longbench/4/warm_scalar | 0.991301x | [0.964637, 1.041032] | 0.998951x | 0.977569x |
| gemma-3/sharegpt/4/cold_scalar | 0.998855x | [0.985948, 1.022991] | 1.010613x | 1.011977x |
| gemma-3/sharegpt/4/warm_batch | 0.997340x | [0.945955, 1.016191] | 1.016049x | 1.013157x |
| gemma-3/sharegpt/4/warm_ragged | 0.987318x | [0.945224, 1.023461] | 1.019721x | 0.999209x |
| gemma-3/sharegpt/4/warm_scalar | 0.988591x | [0.967779, 0.995699] | 1.017399x | 1.009571x |
| gpt-2/longbench/4/cold_scalar | 0.965893x | [0.914284, 1.003750] | 0.949600x | 0.984377x |
| gpt-2/longbench/4/warm_batch | 1.397856x | [0.999336, 2.636642] | 0.679544x | 0.894681x |
| gpt-2/longbench/4/warm_ragged | 0.955090x | [0.810868, 1.416687] | 1.010581x | 1.011321x |
| gpt-2/longbench/4/warm_scalar | 1.010182x | [0.989204, 1.016487] | 0.982779x | 0.957346x |
| gpt-2/sharegpt/4/cold_scalar | 0.989784x | [0.935984, 1.008286] | 1.016020x | 0.997654x |
| gpt-2/sharegpt/4/warm_batch | 0.941708x | [0.867231, 1.086138] | 0.949915x | 1.019036x |
| gpt-2/sharegpt/4/warm_ragged | 1.054959x | [0.870271, 1.109390] | 0.862551x | 1.080588x |
| gpt-2/sharegpt/4/warm_scalar | 0.983256x | [0.965504, 1.012401] | 0.986308x | 1.018327x |
| gpt-oss/longbench/1/cold_scalar | 0.991168x | [0.974912, 1.060046] | 1.033972x | 1.012256x |
| gpt-oss/longbench/1/warm_batch | 1.007756x | [0.991259, 1.010589] | 0.983857x | 1.001060x |
| gpt-oss/longbench/1/warm_ragged | 1.022689x | [0.980468, 1.044512] | 0.996323x | 1.000029x |
| gpt-oss/longbench/1/warm_scalar | 0.989780x | [0.972475, 1.010530] | 0.988837x | 1.003127x |
| gpt-oss/longbench/4/cold_scalar | 1.005735x | [0.928713, 1.024320] | 1.062574x | 1.127728x |
| gpt-oss/longbench/4/warm_batch | 1.140804x | [0.756114, 1.658018] | 1.012546x | 1.692153x |
| gpt-oss/longbench/4/warm_ragged | 0.935254x | [0.864568, 1.126111] | 1.375699x | 1.017754x |
| gpt-oss/longbench/4/warm_scalar | 0.991422x | [0.970280, 1.033950] | 1.003128x | 1.042118x |
| gpt-oss/sharegpt/4/cold_scalar | 1.047117x | [0.978432, 1.101469] | 0.971101x | 1.018413x |
| gpt-oss/sharegpt/4/warm_batch | 0.943980x | [0.704685, 1.176624] | 1.016629x | 1.099948x |
| gpt-oss/sharegpt/4/warm_ragged | 1.007085x | [0.912982, 1.250465] | 1.226965x | 0.930890x |
| gpt-oss/sharegpt/4/warm_scalar | 1.005766x | [0.979966, 1.016590] | 0.999717x | 1.027506x |
| qwen-3/longbench/4/cold_scalar | 0.997583x | [0.989884, 1.032762] | 0.997983x | 1.057438x |
| qwen-3/longbench/4/warm_batch | 1.708206x | [0.885960, 2.632055] | 1.010387x | 1.550956x |
| qwen-3/longbench/4/warm_ragged | 0.994383x | [0.842184, 1.068968] | 0.869212x | 0.994007x |
| qwen-3/longbench/4/warm_scalar | 1.012191x | [1.001202, 1.039994] | 1.000181x | 1.005606x |
| qwen-3/sharegpt/4/cold_scalar | 1.024320x | [0.995359, 1.077774] | 0.988735x | 0.953171x |
| qwen-3/sharegpt/4/warm_batch | 0.982099x | [0.867929, 1.223024] | 0.720684x | 0.923149x |
| qwen-3/sharegpt/4/warm_ragged | 1.148736x | [0.922699, 1.294999] | 1.187729x | 1.194342x |
| qwen-3/sharegpt/4/warm_scalar | 1.000524x | [0.979047, 1.012533] | 0.966478x | 0.954258x |
| t5-small/longbench/4/cold_scalar | 0.973775x | [0.832434, 1.009125] | 1.079783x | 1.027442x |
| t5-small/longbench/4/warm_batch | 1.023725x | [0.984405, 1.048554] | 0.996602x | 0.985164x |
| t5-small/longbench/4/warm_ragged | 1.034122x | [1.005173, 1.063098] | 0.982239x | 0.986954x |
| t5-small/longbench/4/warm_scalar | 1.007103x | [0.923892, 1.015818] | 1.014350x | 1.025924x |
| t5-small/sharegpt/4/cold_scalar | 1.001647x | [0.973317, 1.033977] | 0.994654x | 0.991215x |
| t5-small/sharegpt/4/warm_batch | 1.016171x | [1.004122, 1.030786] | 0.960788x | 0.996175x |
| t5-small/sharegpt/4/warm_ragged | 1.005354x | [0.977921, 1.014463] | 1.013682x | 0.997764x |
| t5-small/sharegpt/4/warm_scalar | 1.002681x | [0.981934, 1.009594] | 0.992401x | 0.996672x |

## Scope and verification

- Unchanged `autoresearch/st-eval` and existing cache-screen orchestration. No evaluator code, dependency lock, rounds, timing boundary, or tokenizer changes. Protocol/configuration and file hashes were frozen before timing.
- Five tokenizers: GPT-2, Qwen 3, GPT-OSS, Gemma 3 and T5 Unigram. LongBench has eight 65,536-byte documents; ShareGPT has 64 conversations capped at 4,096 UTF-8 bytes. Four Rayon workers, plus GPT-OSS/LongBench with one worker.
- Six identical-binary A/A pairs, six independently rebuilt same-source A/A pairs, and twelve candidate pairs per cell; 528 timed processes total. All six scalar/nested/ragged passes are retained. First-pass scalar is separate; warmed metrics use passes 2–6.
- Fresh processes, alternating AB/BA order, same fixtures and baseline-generated ST snapshots. Timing includes public output allocation and destruction, excludes tokenizer loading. This is repeated-input behavior, not a novel-input or broad portable result.
- Full Hugging Face IDs, special tokens on/off, nested/ragged row boundaries, JSON/cached/direct ST loading and vocabulary checks passed before and after every pool. Corpus/model/snapshot/binary hashes remained unchanged.
- Rust 1.97.0, release, default features, debug symbols level 1, two build jobs, no RUSTFLAGS or CPU affinity. All builds finished before timing; user applications remained running. AC power, no reported thermal/performance warning before or after.
- Independent rebuild was verified from a compiler log after `cargo clean --release -p snaptokens`. An earlier clean without --release removed zero files and its cached attempt was excluded; that log is retained. Rebuilt parent binary is byte-identical to the original.
- Evaluator executables including debug symbols: parent 7,847,416 bytes, candidate 7,849,640 bytes (+2,224 bytes). No construction-time, RSS, Python-wheel or cross-CPU measurements.

Raw rounds, parity logs, binaries, corpus/tokenizer sources, manifests, original scorer output and SHA256 inventory: `/Users/namanchetwani/.cache/snaptokens-unchecked-vec-20260929/speed`.

The source remains an explicitly user-requested isolated implementation, not a performance-promoted champion. The result supports no speedup claim; no source was reverted or merged as part of this measurement request.
