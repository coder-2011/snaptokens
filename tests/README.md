# Running the tests

```sh
cargo test --workspace
cargo test --lib
cargo test --test tokenizer tokenizer::local
python -m pytest -q -rx python/tests
```

The library tests and `tokenizer::local` need no model downloads. Integration
tests use pinned Hugging Face tokenizer revisions and verify their BLAKE3
hashes. The default integration suite still runs those comparisons.
`tokenizer::extended` contains the opt-in large-corpus tests. Run a named test
with `-- --ignored`; the Gemma case also requires authorized model access.
Python tests require an installed wheel, pytest, and Transformers. CI builds
the wheel and tests it in a separate environment.

Keep cases close to the behavior they protect:

- Inline `#[cfg(test)] mod tests` at the end of each owning Rust source file:
  focused module behavior, packed-data boundaries, cache collisions, scalar/SIMD
  agreement, and binary-format recovery.
- `tokenizer/local.rs`: small public API regressions using in-memory JSON.
- `tokenizer/hugging_face.rs`: real-model compatibility, including added tokens
  and long inputs. `support.rs` shares fixture loading and checks complete
  scalar, nested, and ragged output against the same reference.
- `tokenizer/streaming.rs`: output across chunk sizes, seeded streams, special
  tokens, bounded state, and invalid-prefix rejection.
- `python/tests`: installed-package behavior, metadata, flat buffers, Transformers
  patching, and process-isolated GIL/deadlock checks.

Use case tables when setup and assertions are the same. Keep distinct regression
mechanisms separate. Compare complete outputs and row lengths, require expected
errors, and never turn unexpected errors into skipped cases. Test production
entry points instead of copying their control flow into test-only methods.

Python truncation parity tests check that content is shortened before template
special tokens are inserted. With `[CLS] $A [SEP]` and maximum length 2,
encoding `ab` must produce `[CLS, SEP]` across scalar, batch, and flat APIs.
These are ordinary passing tests, including left/right truncation, empty rows,
and encoding without special tokens.

This organization follows the ordinary unit/integration split described in
[the Rust book](https://doc.rust-lang.org/book/ch11-03-test-organization.html).
The design rationale also draws on *Rust for Rustaceans* (early-access PDF,
pages 89–94) and *A Philosophy of Software Design*, second edition (local PDF,
pages 136–137): focused tests protect refactoring, and private tests remain
useful when they check an invariant that public output alone cannot establish.
