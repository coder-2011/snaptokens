# D6 literal-search length guard: retained locally

Candidate `90423a9079c1d147b1ac2ec3b493ee4ecce493ae` versus retained D5 `c23b8a2586f905a56279b91d145cb4857e5c1095`. Final independent retention audit supports bounded local retention for the repeated Gemma benefit. Three production lines return the existing borrowed result when a nonempty literal needle is longer than its input in bytes. The writer, allocation policy, regex branch, empty-pattern behavior, decoder stages and public APIs remain unchanged. Two equal-length inputs extend the existing differential test.

Gemma benefit repeats in two complete, separately scored twelve-pair pools: **1.039861 [1.019410, 1.054213]** first and **1.032400 [1.014094, 1.052179]** confirmation. All eight Gemma cell points improve in both pools. Short/long scalar are 1.041931/1.055512 first and 1.031824/1.042395 confirmation. First Gemma AB/BA agree at 1.039267/1.038516; confirmation has a material order split, 1.046706/1.004900, preserved below and in raw records.

The whole-matrix result is uncertain: first primary 0.997672 [0.986596, 1.024171], confirmation 1.014616 [1.000438, 1.033514]. Do not describe the latter alone as an established general aggregate win. The first T5 family result is adverse at 0.948521 [0.916608, 1.019618], with every cell point down and long/stream8 0.956686 [0.914054, 0.987542]. That pattern does not repeat: confirmation T5 is 1.013990 [0.992234, 1.068156] with all eight points up. This does not prove that the first loss was noise or that T5 is equivalent. GPT-2 family is 1.006796 then 0.997753, with all its family intervals spanning one. Qwen/GPT-OSS guards remain variable, with point losses including GPT-OSS long 0.968915/0.967206 in both pools, with both AB and BA below one in each pool and broad intervals. Every loss and outlier remains in the records.

The confirmation was declared in commit2a1b0214 after the first pool, before any additional timing. It uses exactly the same binary hashes, compiler/profile/flags, fixtures, passes, twelve AB/BA pairs and frozen scoring; there is no third run or pooled preferred score. Each pool has672 measurements,280 fixture parity and32 synthetic parity passes. Original A/A is informative but imperfect for shorter post-D5 timings and different load conditions; its false-gain behavior prevents treating naive candidate confidence intervals alone as proof. Repeated intended-path benefit, compiled removal of named work and a minimal change support bounded local retention; they do not establish an encode or portable general win.

The fixed fixture spelling analysis shows29.42%/37.12% of Gemma short/long token occurrences cannot contain its three-byte marker. This is descriptive, not dispatch: the code tests actual byte lengths for any literal. Gemma's opposite encoding normalizer uses a one-byte space needle and generally pays the added comparison without this benefit. Shared encoding semantics pass the relevant workspace tests; encoding throughput remains unmeasured.

All18 focused replacement tests and188 release workspace tests pass (nine existing ignored), plus strict Clippy, formatting and warning-denied documentation. All28 allocation cells match D5 exactly, as do84 ByteLevel resource controls, compared against valid `resource-d5-02`; INVALID first D5 diagnostics remain excluded. Assembly shows the new unsigned length branch reaching borrowed return before StrSearcher::new; the equal/longer route retains the existing search. Separate unscored profiling completes full parity and is consistent with fewer search-setup samples, but is not used as a throughput measurement. No retained state, construction work, dependency or unsafe code is added.

Full-process load1 ranges are 2.51–3.68 and 2.12–7.96. The confirmation maximum occurs during POST checks; recorded time-process starts range 2.505–3.223 first and 2.267–3.612 in confirmation. Do not attribute timed outcomes to that later peak or treat load as constant. Binary sizes are D5 8,057,152 bytes and D6 8,057,008 bytes (delta -144).

This is local Apple M2 repeated-input decode evidence. AMD/Intel, sealed holdout, release/package matrix, standalone RSS and encode timing remain unmeasured. No general promotion or publication.

| Cell | First ratio [95% CI] | Confirmation ratio [95% CI] |
|---|---:|---:|
| gpt-2/short/scalar | 0.997417 [0.990653, 1.023416] | 0.986692 [0.963259, 1.018853] |
| gpt-2/short/batch32 | 0.994648 [0.985020, 1.025158] | 1.004258 [0.981397, 1.021184] |
| gpt-2/short/stream1 | 1.009170 [0.998597, 1.014783] | 1.003225 [0.996856, 1.009864] |
| gpt-2/short/stream8 | 1.009335 [0.985457, 1.056959] | 1.006999 [0.998564, 1.027958] |
| gpt-2/long/scalar | 1.008917 [0.993323, 1.059459] | 1.004941 [0.979946, 1.014483] |
| gpt-2/long/batch32 | 1.009619 [0.990762, 1.071110] | 0.995859 [0.944024, 1.005685] |
| gpt-2/long/stream1 | 1.014848 [1.001809, 1.049377] | 0.994472 [0.931059, 1.012038] |
| gpt-2/long/stream8 | 1.010585 [1.006170, 1.042221] | 0.985825 [0.973489, 1.003119] |
| gemma-3/short/scalar | 1.041931 [1.009723, 1.063796] | 1.031824 [1.004046, 1.058098] |
| gemma-3/short/batch32 | 1.034913 [0.990174, 1.063314] | 1.037797 [1.016397, 1.051717] |
| gemma-3/short/stream1 | 1.028031 [0.994527, 1.037708] | 1.016076 [0.999484, 1.036137] |
| gemma-3/short/stream8 | 1.031522 [1.007756, 1.060126] | 1.031178 [1.017142, 1.045331] |
| gemma-3/long/scalar | 1.055512 [1.027224, 1.087530] | 1.042395 [1.017719, 1.075275] |
| gemma-3/long/batch32 | 1.051481 [1.032493, 1.067529] | 1.036119 [1.016189, 1.069816] |
| gemma-3/long/stream1 | 1.015861 [1.001926, 1.038135] | 1.028544 [0.981478, 1.060903] |
| gemma-3/long/stream8 | 1.060411 [1.037968, 1.077574] | 1.035479 [1.013482, 1.069206] |
| t5-small/short/scalar | 0.941425 [0.919701, 1.015758] | 1.009956 [0.972558, 1.077268] |
| t5-small/short/batch32 | 0.961754 [0.924005, 1.047684] | 1.009800 [0.990716, 1.082240] |
| t5-small/short/stream1 | 0.955730 [0.932972, 1.024141] | 1.005882 [1.001291, 1.082938] |
| t5-small/short/stream8 | 0.940981 [0.913266, 1.020345] | 1.008823 [0.993086, 1.047970] |
| t5-small/long/scalar | 0.921997 [0.862418, 1.042515] | 1.016826 [0.967550, 1.081332] |
| t5-small/long/batch32 | 0.936386 [0.866501, 1.044523] | 1.024879 [0.979205, 1.085562] |
| t5-small/long/stream1 | 0.974220 [0.948784, 1.033078] | 1.009972 [0.996524, 1.063128] |
| t5-small/long/stream8 | 0.956686 [0.914054, 0.987542] | 1.025980 [0.996675, 1.063700] |
| qwen-3/short/scalar | 1.024336 [0.949123, 1.282724] | 0.980668 [0.860172, 1.111959] |
| qwen-3/long/scalar | 1.036517 [0.959038, 1.325612] | 0.955035 [0.844603, 1.151166] |
| gpt-oss/short/scalar | 0.963125 [0.938994, 1.071320] | 0.991627 [0.892804, 1.077249] |
| gpt-oss/long/scalar | 0.968915 [0.924206, 1.063590] | 0.967206 [0.896664, 1.046209] |
