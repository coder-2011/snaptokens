# Speed

## Combined generic result

The published generic headline is recalculated from raw paired medians. It
keeps the 780 historical 2026-07-31 cells and adds the exact primary-shape
cells measured on the 2026-09-09 AMD C3D and Intel C4 GCP hosts. The current
hosts do not identify any of the historical 13 hosts, so no unmatched host was
silently substituted or overwritten.

| Competitor | Historical cells | New GCP cells | Combined cells | Snaptokens wins | Geometric mean speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| Gigatoken | 780 | 100 | 880 | 840 | 2.04× |
| upstream fastokens | 585 | 90 | 675 | 675 | 13.06× |
| Hugging Face Tokenizers | 780 | 110 | 890 | 890 | 46.41× |

For each host, model, shape, and implementation, the calculation first takes
the median timed MiB/s across that cell's rounds. It divides the Snaptokens
median by the competitor median, then takes the geometric mean of those paired
ratios. The historical raw recomputation returns the previously recorded
Gigatoken result of 2.1891185881× over 777/780 cells before the new GCP rows
are included. The complete machine-readable calculation is
[`data/2026-09-09/gcp-regeneration/merged-generic-report.json`](data/2026-09-09/gcp-regeneration/merged-generic-report.json).

The five shapes shared by both cohorts are 140-byte batches of 1, 32, and 512,
one 4 KiB input, and one 64 KiB input. These are a combined historical and
current measurement set, not a claim that all 15 hosts ran the same source
revision.

## 2026-09-09 GCP generic matrix

The standalone current GCP cohort is intentionally shown separately from the
combined headline. It used source
`305c6cb04fcd2059dcea19ef81f09b03680d977b`, portable evaluator v2, four
pinned physical CPUs (`0-3`), four Rayon workers, novel rotating inputs, and
pre- and post-timing exactness checks.

| Competitor | Exact cells | Snaptokens wins | Geometric mean speedup |
| --- | ---: | ---: | ---: |
| Gigatoken | 100 | 63 | 1.16× |
| upstream fastokens | 90 | 90 | 5.42× |
| Hugging Face Tokenizers | 110 | 110 | 29.85× |

Each GCP host retained 617 `exact_on_probes_and_all_timed_inputs` generic
coverage records. The raw files also retain, without timing rows, 142
unsupported, 119 probe-mismatch, and 112 timed-input-mismatch records per
host. None is imputed into a score.

## Specialist and long-input results

These are GCP-only specialist measurements. They are not folded into the
generic headline.

| Specialist competitor | Exact cells | Snaptokens wins | Geometric mean speedup |
| --- | ---: | ---: | ---: |
| OpenAI tiktoken | 20 | 20 | 8.26× |
| riptoken | 18 | 17 | 2.96× |
| rust-tiktoken | 10 | 10 | 4.84× |
| tokendagger | 10 | 10 | 6.38× |
| WordChipper | 20 | 19 | 3.07× |

| Long single-input competitor | Exact cells | Snaptokens wins | Geometric mean speedup |
| --- | ---: | ---: | ---: |
| cxuu parallel | 12 | 12 | 21.99× |
| upstream fastokens | 6 | 3 | 1.05× |
| Hugging Face serial | 12 | 12 | 32.93× |

The long-input rows time construction, output allocation, observation, and
destruction through the stated public APIs. The raw coverage records retain
unsupported candidate types separately.

## Loading and `.tkz`

The README's loading figures recombine 156 historical warm-filesystem cells
with ten current warm-page cells: direct `.tkz` loading is 1.58× faster than
Snaptokens JSON, while direct `.tkz` takes 1.31× the Hugging Face JSON load
time. The twelve-model `.tkz` size geometric mean remains 24.87% smaller than
JSON; the five rerun artifacts have the same bytes as their historical
counterparts.

The current GCP load rows also retain the separate page-cache states. For the
five models on both hosts, direct `.tkz` is 2.28× faster than Snaptokens JSON
with warm pages and 2.10× faster with guest-page-cache-cold pages. In those
same current rows, Hugging Face JSON is 1.13× and 1.14× faster than direct
`.tkz`, respectively. These state-specific results are not substituted for
the historical loading rows.

The retained Intel C4 CPU-0 GPT-2 direct-load cell loads V5 `.tkz` in
29,659,905 ns versus 60,006,180 ns for the preceding V4 direct loader,
2.023141× throughput. JSON is 124,611,724 ns versus 123,725,976 ns,
0.992892× throughput.

## Exactness and evaluator controls

Every included timing row has its recorded exactness coverage. The generic
suite compares complete IDs and row lengths on probes and every timed input
before and after timing. The long-input suite records complete-ID parity for
all runnable candidates. Load results use the persisted probe manifest and
retain failures rather than filling them with a benchmark value.

Both GCP workers also passed `cargo test --workspace --locked`: 133 unit
tests, 54 integration tests with 9 existing ignores, 3 Python-binding tests,
and the crate doctest. The two captured logs sit beside each host's raw data.

Portable paired-v2 A/A controls are evidence about timing noise, not a
performance comparison:

| Host | Build relationship | Equal-cell geometric mean | 95% interval |
| --- | --- | ---: | --- |
| AMD C3D | independently built same source | 0.9876× | [0.9844×, 0.9876×] |
| Intel C4 | independently built same source | 1.0004× | [0.9927×, 1.0080×] |
| AMD C3D | identical binary | 1.0024× | [0.9972×, 1.0055×] |
| Intel C4 | identical binary | 1.0091× | [1.0057×, 1.0113×] |

The raw rounds, hardware and run metadata, generic, specialist, long-input,
load, and A/A records are in
[`data/2026-09-09/gcp-regeneration/`](data/2026-09-09/gcp-regeneration/).
The earlier blocked GLM evidence remains separately retained and is not used
in any value above.
