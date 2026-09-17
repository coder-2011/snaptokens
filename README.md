<h1 align="center">snaptokens</h1>

<p align="center">
Fast BPE and Unigram tokenization in Rust.
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/coder-2011/snaptokens/main/assets/benchmark-throughput-a8b4520196a2.svg" alt="Throughput relative to Hugging Face for batches of 1, 32, and 512, then single 4 KiB and 64 KiB inputs. Snaptokens: 58.2×, 24.2×, 20.5×, 94.9×, 107.2×. Gigatoken: 16.7×, 12.8×, 14.8×, 52.7×, 77.9×.">
</p>

snaptokens is a high-performance tokenizer, built to be compatible with hf tokenziers.

We also support `.tkz` tokenization, similar to [Tokie](https://github.com/feyninc/tokie).
Optional [`.st` snapshots](#optional-st-snapshots) support both BPE and Unigram loading.

The current twelve-tokenizer, fifteen-host comparison is 2.19× faster than Gigatoken, 13.06× faster than fastokens, and 46.41× faster than Hugging Face by geometric mean of paired medians. It is (to the best of my knowledge) [the fastest OSS BPE and Unigram tokenizer, in the world]((https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md)

## Optimizations

- Hand-written scanners replace general regex machinery for recognized production tokenizer patterns, with a table-driven ASCII path and exact Unicode fallback.
- Split, ByteLevel transformation, and BPE are fused so raw bytes map directly to initial token IDs and cache hits skip ByteLevel work entirely.
- BPE uses precomputed flat lookups: byte-to-token IDs, all 65,536 initial byte pairs, open-addressed merge ranks, and CSR merge adjacency.
- Pre-tokenized text stays in one contiguous buffer with byte ranges instead of allocating a string for every piece.
- Thread-local and shared caches are amortized across whole chunks, while a fixed Rayon pool keeps worker caches warm and balances uneven BPE work.
- Non-ByteLevel tokenizers split only at vocabulary-proven unbridgeable byte pairs; byte-fallback models stay on the normal exact merge path.
- Eligible Unigram documents cut at whitespace-aligned anchors so charsmap, Metaspace walking, and Viterbi run per partition on the shared pool instead of serializing a rewritten buffer.
- Opt-in `.tkz` sidecars cache validated native construction data. Across twelve models and fifteen hosts, direct `.tkz` loads are 1.58× faster than Snaptokens JSON and artifacts are 24.9% smaller by geometric mean. The retained GCP direct-load cell is 2.02× faster than its preceding direct loader, while JSON is 0.99×. Unigram JSON cannot use `.tkz` yet.

## Benchmarks

The 2026-07-31 suite uses DeepSeek R1, Gemma 3, GLM 4.7, GPT-2, GPT-OSS, Llama 3, MiniMax M2.1, Mistral Nemo, Nemotron 3, Phi-4 mini, Qwen 2.5, and Qwen 3. It runs five batch and input shapes across a variety of machines

The [full benchmark report](https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md) contains a much more thorough analysis. The complete [evidence](https://github.com/coder-2011/snaptokens/tree/main/benchmarks/data) includes every piece of data

On Unigram, we measure Snaptokens at `160 MB/s` / `66.1×` Hugging Face. Not, supported by Gigatoken, and 2.21x faster than Tokie.

## Install

Install the Python package:

```bash
pip install snaptokens
```

Or add the Rust crate:

```toml
[dependencies]
snaptokens = "0.2"
```

## Usage

Python:

```python
from snaptokens import Tokenizer

tokenizer = Tokenizer.from_file("/path/to/tokenizer.json")
ids = tokenizer.encode("Tokenization should not be the bottleneck.").ids
```

`enable_truncation(max_length=100, direction="right")` limits the returned
sequence, including requested post-processor special tokens. 

Pass `tkz_cache=True` to create and reuse the adjacent `.tkz` file. A `.tkz`
path loads directly without that flag, and usage of `.tkz` is hidden

To replace a Transformers v4 or v5 fast-tokenizer backend:

```python
import snaptokens

snaptokens.patch_transformers()
```

Call `snaptokens.unpatch_transformers()` to restore the original backend.

Rust:

```rust
use std::{error::Error, path::Path};

use snaptokens::{LoadMode, Tokenizer};

/// Loads one local tokenizer and encodes a prompt.
fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::load_file(
        Path::new("/path/to/tokenizer.json"),
        LoadMode::TkzCache,
    )?;
    let ids = tokenizer.encode("Tokenization should not be the bottleneck.", false)?;

    println!("{ids:?}");
    Ok(())
}
```

Pass `true` as the second argument to apply the tokenizer's configured special-token
post-processing.

## Benchmark tools

The runnable benchmark tools live in `benchmarks/tools/`. They are repository-only development tools and are not included in published crate archives. Run one with Cargo and pass its arguments after `--`:

```bash
cargo bench --manifest-path benchmarks/tools/Cargo.toml --bench simple_bench -- /path/to/tokenizer.json --max-samples 50
cargo bench --manifest-path benchmarks/tools/Cargo.toml --bench load_bench -- /path/to/tokenizer.json json 10
cargo bench --manifest-path benchmarks/tools/Cargo.toml --bench print_pipeline -- /path/to/tokenizer.json
cargo bench --manifest-path benchmarks/tools/Cargo.toml --bench profile_sample -- /path/to/tokenizer.json 10
```

### Optional `.tkz` cache

```rust
let tokenizer = Tokenizer::load_file(
    Path::new("/path/to/tokenizer.json"),
    LoadMode::TkzCache,
)?;
```

The first cached load atomically writes `tokenizer.tkz`; later loads validate and reuse it. Passing that `.tkz` path loads it directly without creating another copy. `Tokenizer::load_file(path, LoadMode::JsonOnly)` remains JSON-only and never writes a cache.

### Optional `.st` snapshots

Create and reuse a snapshot from a compatible Hugging Face `tokenizer.json`:

```python
from snaptokens import Tokenizer

tokenizer = Tokenizer.from_file("/path/to/tokenizer.json", st_cache=True)
# The snapshot also loads without the original JSON or a cache flag.
restored = Tokenizer.from_file("/path/to/tokenizer.st")
```

```rust
use std::path::Path;
use snaptokens::{LoadMode, Tokenizer};

let tokenizer = Tokenizer::load_file(
    Path::new("/path/to/tokenizer.json"),
    LoadMode::StCache,
)?;
let restored = Tokenizer::load_file(
    Path::new("/path/to/tokenizer.st"),
    LoadMode::StCache,
)?;
```

The first cached JSON load writes a sibling `tokenizer.st` atomically after the
complete tokenizer has been constructed. Later loads check the source hash and
payload checksum. A stale or invalid snapshot is rebuilt when its JSON source is
available. Direct `.st` loads validate the snapshot and report errors without
rebuilding it. With `StCache` / `st_cache=True`, a missing JSON path also falls back
to its existing sibling `.st`. JSON-only loading remains read-only.

BPE version 1 stores native lookup and merge tables with a packed vocabulary.
Unigram version 2 stores vocabulary strings, exact scores, the unknown-token ID,
and byte-fallback settings, then validates them and rebuilds the matcher on load.
Both preserve the tokenizer pipeline, including added tokens, normalization,
pre-tokenization, post-processing and decoding. Python also restores stored
padding and supported truncation settings. `.st` does not expand support to
otherwise unsupported tokenizer pipelines or native SentencePiece `.model` files.

The recorded twelve-model BPE comparison measured the following warm-load ratios:

| CPU | JSON / ST | TKZ / ST |
| --- | ---: | ---: |
| Apple ARM | 9.33× | 2.69× |
| AMD x86-64 | 10.86× | 3.44× |
| Intel x86-64 | 9.73× | 2.95× |

These are descriptive geometric-mean ratios from twelve rounds in a fixed format
order, including load and destruction. They measure loading, not encoding, and
are not a randomized cross-format comparison. The [BPE results and method](autoresearch/results/st-20260917/format-comparison.json)
identify the measured commits, which precede the structural refactor.

A separate pinned T5/UMT5 comparison on one Intel host measured direct `.st`
loading at 1.157× JSON throughput across twelve paired rounds. See the
[Unigram results](autoresearch/results/st-20260917/unigram-format-baseline-v1/summary.json).
This Unigram result has not been confirmed across CPU classes.

## Scope

Snaptokens provides inference for BPE tokenizers and compatible Hugging Face
`tokenizer.json` Unigram pipelines. Native SentencePiece `.model` files and
Unigram `.tkz` caching are not supported.

## Credits

Licensed under [Apache-2.0](https://github.com/coder-2011/snaptokens/blob/main/LICENSE).

## Status

Unchecked items are not currently supported.

- [x] **Hugging Face JSON Unigram:** Tagged and legacy untagged Unigram JSON, including T5-style Precompiled normalization, `WhitespaceSplit → Metaspace`, and Metaspace decoding.
- [ ] **Other model algorithms:** WordPiece, WordLevel, and model types other than BPE or Unigram. Native SentencePiece `.model` files are explicitly rejected, and Unigram cannot use `.tkz` caching yet.
- [ ] **BPE options beyond its core:** `dropout`, `unk_token`, `fuse_unk`, `continuing_subword_prefix`, and `end_of_word_suffix` are not represented or guaranteed exact.
- [ ] **Normalizers:** NFD/NFKC/NFKD, Lowercase, Strip, and BertNormalizer.
- [ ] **Pre-tokenizers:** Whitespace (distinct from supported WhitespaceSplit), Bert, Digits, Punctuation, and UnicodeScripts.
- [ ] **Post-processors:** Bert/Roberta processors and pair-sequence processing; parsed pair templates are unused.
- [ ] **Decoders:** WordPiece, BPE, CTC, and Strip decoders.
- [ ] **Training:** Tokenizer training and vocabulary/model-construction APIs.
- [ ] **Pair encoding:** Pair encoding and pair post-processing; Python raises `NotImplementedError`.
- [ ] **Offset/word metadata:** Python token strings, character offsets, sequence IDs, word IDs, and overflow rows after truncation; the associated mapping methods raise `NotImplementedError`.
- [x] **Serialized padding/truncation settings:** Python JSON, `.tkz`, and `.st` loading restore stored padding and supported single-sequence truncation settings. Nonzero stride and `only_second` are rejected because overflow rows and pair encoding are unsupported.
- [ ] **Splitting added special tokens:** `encode_special_tokens=True` raises `NotImplementedError`.
