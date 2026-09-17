# Running the tests

```sh
cargo test --workspace
cargo test --lib
python -m pytest -q -rx python/tests
```

Rust tests live in `#[cfg(test)] mod tests` at the end of the owning source
file under `src/`. Shared Hugging Face loaders and the Comparison oracle live
in `src/test_support.rs`. Library tests that construct tokenizers from
in-memory JSON need no model downloads. Hugging Face comparisons live next to
the code they cover and load pinned tokenizer revisions, then verify BLAKE3
hashes. Opt-in large-corpus tests are ignored. Run a named test with
`-- --ignored`. The Gemma case also requires authorized model access.

Python tests require an installed wheel, pytest, and Transformers. CI builds
the wheel and tests it in a separate environment.

Use case tables when setup and assertions are the same. Keep distinct regression
mechanisms separate. Compare complete outputs and row lengths, require expected
errors, and never turn unexpected errors into skipped cases. Test production
entry points instead of copying their control flow into test-only methods.

Python truncation parity tests check that content is shortened before template
special tokens are inserted. With `[CLS] $A [SEP]` and maximum length 2,
encoding `ab` must produce `[CLS, SEP]` across scalar, batch, and flat APIs.
These are ordinary passing tests, including left/right truncation, empty rows,
and encoding without special tokens.
