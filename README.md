<h1 align="center">snaptokens</h1>

<p align="center">
Fast BPE and Unigram tokenization in Rust. <a href="https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md"><strong>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/coder-2011/snaptokens/main/assets/benchmark-portable-overview.svg" alt="Snaptokens throughput advantage across 15 hosts">
</p>

snaptokens is a high-performance tokenizer, built to be compatible with hf tokenziers.

We also support `.tkz` tokenization, similar to [Tokie](https://github.com/feyninc/tokie).

The current twelve-tokenizer, fifteen-host comparison is 2.04× faster than Gigatoken, 13.06× faster than upstream fastokens, and 46.41× faster than Hugging Face by geometric mean of paired medians. [Snaptokens wins 840/880 times against gigatoken](https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md). That generic matrix is BPE only. Hugging Face JSON Unigram (T5-style) is supported separately and is not one of those twelve models.

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

The 2026-07-31 suite uses DeepSeek R1, Gemma 3, GLM 4.7, GPT-2, GPT-OSS, Llama 3, MiniMax M2.1, Mistral Nemo, Nemotron 3, Phi-4 mini, Qwen 2.5, and Qwen 3. It runs five batch and input shapes on Apple M2 plus twelve four-core Modal placements across AWS, GCP, and OCI.

Every timed engine first passes Hugging Face token-ID parity. Gigatoken and Snaptokens cover all twelve models; upstream fastokens covers nine. Flat-ragged engines are compared only with the same flat-ragged output contract, nested engines only with nested output.

The [full benchmark report](https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md) contains the per-host, per-model, per-shape, load, footprint, variability, correctness, and limitation analysis. The complete [accepted evidence](https://github.com/coder-2011/snaptokens/tree/main/benchmarks/data) includes every raw round, hardware capture, hash, summary, and an empty failure ledger.

T5 Unigram is a specialist JSON path, not a thirteenth generic cell. On idle GCP Intel `c4-standard-8`, `simple_bench` over the same 20 LongBench contexts (`18,012,626` characters) measured sequential Snaptokens at `160 MB/s` / `66.1×` Hugging Face for `google-t5/t5-small`, versus `197 MB/s` / `75.7×` Hugging Face for GPT-2 BPE on the same binary and corpus.

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

## Use

Python:

```python
from snaptokens import Tokenizer

tokenizer = Tokenizer.from_file("/path/to/tokenizer.json")
ids = tokenizer.encode("Tokenization should not be the bottleneck.").ids
```

Pass `tkz_cache=True` to create and reuse the adjacent `.tkz` file. A `.tkz`
path loads directly without that flag.

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

Download the model yourself and pass the local JSON path. snaptokens never tries to infer or download a Hugging Face model.

## Benchmark tools

The runnable benchmark tools live in `benches/`. They are repository-only development tools and are not included in published crate archives. Run one with Cargo and pass its arguments after `--`:

```bash
cargo bench --manifest-path benches/Cargo.toml --bench simple_bench -- /path/to/tokenizer.json --max-samples 50
cargo bench --manifest-path benches/Cargo.toml --bench load_bench -- /path/to/tokenizer.json json 10
cargo bench --manifest-path benches/Cargo.toml --bench print_pipeline -- /path/to/tokenizer.json
cargo bench --manifest-path benches/Cargo.toml --bench profile_sample -- /path/to/tokenizer.json 10
```

### Optional `.tkz` cache

```rust
let tokenizer = Tokenizer::load_file(
    Path::new("/path/to/tokenizer.json"),
    LoadMode::TkzCache,
)?;
```

The first cached load atomically writes `tokenizer.tkz`; later loads validate and reuse it. Passing that `.tkz` path loads it directly without creating another copy. `Tokenizer::load_file(path, LoadMode::JsonOnly)` remains JSON-only and never writes a cache.

## Scope

Snaptokens provides inference for BPE tokenizers and compatible Hugging Face
`tokenizer.json` Unigram pipelines. Native SentencePiece `.model` files and
Unigram `.tkz` caching are not supported.
## Credits

Inspired by SIMD stuff from hyperscan and gigatoken. Also uses the .tkz extension from Tokie.

Licensed under [Apache-2.0](https://github.com/coder-2011/snaptokens/blob/main/LICENSE).

## Status

Unchecked items are not currently supported.

- [x] **Hugging Face JSON Unigram:** Tagged and legacy untagged Unigram JSON, including T5-style Precompiled normalization, `WhitespaceSplit → Metaspace`, and Metaspace decoding.
- [ ] **Other model algorithms:** WordPiece, WordLevel, and every other non-BPE model type. Native SentencePiece `.model` files are explicitly rejected, and Unigram cannot use `.tkz` caching yet.
- [ ] **BPE options beyond its core:** `dropout`, `unk_token`, `fuse_unk`, `continuing_subword_prefix`, and `end_of_word_suffix` are not represented or guaranteed exact.
- [ ] **Normalizers:** NFD/NFKC/NFKD, Lowercase, Strip, and BertNormalizer.
- [ ] **Pre-tokenizers:** Whitespace (distinct from supported WhitespaceSplit), Bert, Digits, Punctuation, and UnicodeScripts.
- [ ] **Post-processors:** Bert/Roberta processors and pair-sequence processing; parsed pair templates are unused.
- [ ] **Decoders:** WordPiece, BPE, CTC, and Strip decoders.
- [ ] **Training:** Tokenizer training and vocabulary/model-construction APIs.
- [ ] **Pair encoding:** Pair encoding and pair post-processing; Python raises `NotImplementedError`.
- [ ] **Offset/word metadata:** Python token strings, character offsets, sequence IDs, word IDs, and overflow rows after truncation; the associated mapping methods raise `NotImplementedError`.
- [ ] **Serialized padding/truncation settings:** Top-level `tokenizer.json` settings are not restored by Python `from_file` or `from_json_str`; they must be enabled at runtime.
