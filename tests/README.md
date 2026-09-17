# Running the tests

```sh
cargo test --workspace
cargo test --lib
cargo test --test tokenizer
python -m pytest -q -rx python/tests
```

Unit tests live in `#[cfg(test)] mod tests` at the end of the owning source
file under `src/`. They construct tokenizers from in-memory JSON and need no
model downloads.

Hugging Face exactness tests live in `tests/tokenizer/`. They load pinned
tokenizer revisions, verify BLAKE3 hashes, and compare complete token IDs
against Hugging Face Tokenizers. Shared loaders and the Comparison oracle live
in `tests/tokenizer/common.rs`. Opt-in large-corpus tests are ignored. Run a
named test with `-- --ignored`. The Gemma case also requires authorized model
access.

Throughput regression lives in `benchmarks/`, not `cargo test`.

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
