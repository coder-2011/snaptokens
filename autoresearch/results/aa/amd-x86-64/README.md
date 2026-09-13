# AMD x86-64 A/A calibration

These artifacts freeze the `dev-v1` basic A/A calibration for champion
`b3e8a34415f51a4ba4cb9b05d65bccfbd9ed8cfa` and evaluator
`d9086dffdb6b197e47156c32de5beb16ac89c0d9c90c92297d94d7a9b661763b`.

The accepted GCP US-East sandbox exposed `AuthenticAMD family 175 model 17`.
The provider did not report a model name, so no processor brand is inferred.
Each mode retains its evaluator manifest, all 1,440 paired ratios, summary,
empty failure inventory, and the complete checksummed remote bundle. Host and
build provenance are under `provenance/`; interpretation is in `log.md` and
active gates are in `autoresearch/state.md`.
