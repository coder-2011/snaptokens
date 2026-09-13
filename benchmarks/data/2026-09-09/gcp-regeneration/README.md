# 2026-09-09 GCP regeneration

These are the raw results from source
`305c6cb04fcd2059dcea19ef81f09b03680d977b`, collected on four pinned CPUs on
GCP AMD C3D and Intel C4 hosts. `generic/`, `specialists/`, `long-single/`,
and `load/` contain the timing rows and their coverage records. The two A/A
directories contain the paired evaluator's independent-build and
identical-build controls.

`merged-generic-report.json` recomputes the historical 2026-07-31 generic
matrix from raw rows and combines it with the five shared primary shapes from
these two new GCP hosts. A cell is identified by host, model, primary shape,
and competitor. Neither current host matches a historical host identity, so no
historical row is overwritten.

Coverage records with an unsupported or mismatch status are retained in the
raw JSONL and contribute no timing row. The report uses only measured exact
pairs; it does not impute missing engines or filter completed rounds.
