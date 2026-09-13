# Rejected 2026-09-09 GCP regeneration evidence

This directory preserves an incomplete benchmark attempt. It does not contain
an accepted matrix or a publishable performance result.

`generic-gcp-us-central1-intel-c4-8.command` and
`generic-gcp-us-central1-intel-c4-8.log` record the Intel C4 build and runner.
The Intel JSONL files at this directory level are the 20 completed cells. The
`glm-4-7--enwik8-65536--b1--novel.jsonl.error` file records the pre-timing
parity rejection. `diagnostic-glm-64k.jsonl` records the follow-up mismatch,
including the differing token IDs.

`amd-c3d-8/` contains the corresponding AMD C3D partial rows, command, log,
empty error marker, and interrupted `.tmp` file. The AMD runner was stopped
before it timed the known failing GLM-4.7 64 KiB cell.

Neither host's completed rows may be aggregated. The failing cell and all
later cells are absent, and the full load, long-single, specialist, A/A, and
post-timing exactness gates did not run.
