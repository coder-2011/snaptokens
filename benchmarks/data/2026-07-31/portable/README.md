# Portable benchmark evidence

This directory contains the accepted 2026-07-31 matrix: one Apple M2 run and
twelve four-physical-core Modal placements across AWS, GCP, OCI, and four
requested regions. Modal did not expose commercial CPU model names, so
`summaries/hosts.csv` records the exact observed vendor/family/model CPUID.

Every run used Snaptokens `a675159`, corpus
`c8a179774f32edaa53f56abe94ded6f0302e53845f762a310e2abd4acb51818b`, nightly-2026-07-15, fat LTO, one codegen unit,
`target-cpu=native`, four Rayon workers, and the same twelve pinned
`tokenizer.json` artifacts. `failures.json` is empty.

- `raw/`: all 2,211 JSONL records per host, gzip-compressed.
- `hardware/`: Modal `lscpu --json` captures.
- `metadata/`: requested provider, region, resources, and sandbox ID.
- `summaries/`: paired medians, coverage, loads, conversion, tokenizer hashes,
  dependency revisions, hosts, and the report aggregates.
- `SHA256SUMS`: hashes for every file above.

The throughput report uses paired medians and then geometric-means the paired
ratios. It never divides unrelated global means. Gigatoken uses the shared flat
IDs-plus-row-lengths contract; Fastokens and Hugging Face use the nested
contract. Fresh loads run in new processes with warm filesystem pages and
exclude process startup.

Accepted coverage is 12/12 for
Snaptokens JSON and TKZ, 12/12 for
Gigatoken, and 9/12 for
Fastokens. Hugging Face is the token-ID oracle.

TKZ files had identical sizes and exact IDs on every host, but not identical
bytes. The current payload preserves process-built open-addressed merge-table
slots, whose insertion order begins in a randomized source `HashMap`;
`summaries/tokenizers.csv` therefore records the 13 observed TKZ checksum
variants per model instead of pretending they are reproducible artifacts.
