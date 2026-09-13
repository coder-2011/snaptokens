<h1 align="center">snaptokens</h1>

<p align="center">
Fast BPE tokenization in Rust. <a href="https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md"><strong>The fastest in the current generic BPE matrix</strong></a>, in fact.
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/coder-2011/snaptokens/main/assets/benchmark-portable-overview.svg" alt="Snaptokens throughput advantage across 15 hosts">
</p>

snaptokens is a high-performance tokenizer simplified to work in [sinter](https://github.com/coder-2011/sinter). This repo will be archived once ported to sinter.

Give snaptokens a local Hugging Face `tokenizer.json` and get token IDs back. Opt in to a `.tkz` copy when faster future loads matter.

The current twelve-tokenizer, fifteen-host comparison is 2.04× faster than Gigatoken, 13.06× faster than upstream fastokens, and 46.41× faster than Hugging Face by geometric mean of paired medians. Snaptokens wins 840/880 Gigatoken cells; the [full report](https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md) keeps every reported cell visible.

## Optimizations

- Hand-written scanners replace general regex machinery for recognized production tokenizer patterns, with a table-driven ASCII path and exact Unicode fallback.
- Split, ByteLevel transformation, and BPE are fused so raw bytes map directly to initial token IDs and cache hits skip ByteLevel work entirely.
- BPE uses precomputed flat lookups: byte-to-token IDs, all 65,536 initial byte pairs, open-addressed merge ranks, and CSR merge adjacency.
- Pre-tokenized text stays in one contiguous buffer with byte ranges instead of allocating a string for every piece.
- Thread-local and shared caches are amortized across whole chunks, while a fixed Rayon pool keeps worker caches warm and balances uneven BPE work.
- Non-ByteLevel tokenizers split only at vocabulary-proven unbridgeable byte pairs; byte-fallback models stay on the normal exact merge path.
- Opt-in `.tkz` sidecars cache validated native construction data. Across twelve models and fifteen hosts, direct `.tkz` loads are 1.58× faster than Snaptokens JSON and artifacts are 24.9% smaller by geometric mean. The retained GCP direct-load cell is 2.02× faster than its preceding direct loader, while JSON is 0.99×.

## Benchmarks

The 2026-07-31 suite uses DeepSeek R1, Gemma 3, GLM 4.7, GPT-2, GPT-OSS, Llama 3, MiniMax M2.1, Mistral Nemo, Nemotron 3, Phi-4 mini, Qwen 2.5, and Qwen 3. It runs five batch and input shapes on Apple M2 plus twelve four-core Modal placements across AWS, GCP, and OCI.

Every timed engine first passes Hugging Face token-ID parity. Gigatoken and Snaptokens cover all twelve models; upstream fastokens covers nine. Flat-ragged engines are compared only with the same flat-ragged output contract, nested engines only with nested output.

`tokenizer.json` and `.tkz` produce the same steady-state Snaptokens encoder, so the format comparison measures fresh load, first encode, disk size, and memory rather than drawing two redundant throughput bars. TKZ loads 1.58× faster than Snaptokens JSON, but is still 1.31× slower than Hugging Face JSON in this suite. Current GPT-2 JSON loads are 1.43× faster than Hugging Face Tokenizers 0.22.2 on GCP C4.

The [full benchmark report](https://github.com/coder-2011/snaptokens/blob/main/benchmarks/speed.md) contains the per-host, per-model, per-shape, load, footprint, variability, correctness, and limitation analysis. The complete [accepted evidence](https://github.com/coder-2011/snaptokens/tree/main/benchmarks/data) includes every raw round, hardware capture, hash, summary, and an empty failure ledger.

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

use snaptokens::Tokenizer;

/// Loads one local tokenizer and encodes a prompt.
fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::load_file_with_tkz_cache(Path::new("/path/to/tokenizer.json"))?;
    let ids = tokenizer.encode("Tokenization should not be the bottleneck.")?;

    println!("{ids:?}");
    Ok(())
}
```

Download the model yourself and pass the local JSON path. snaptokens never tries to infer or download a Hugging Face model.

## Benchmark tools

The runnable benchmark tools live in `benches/`. They are repository-only development tools and are not included in published crate archives. Run one with Cargo and pass its arguments after `--`:

```bash
cargo bench --bench simple_bench -- /path/to/tokenizer.json --max-samples 50
cargo bench --bench load_bench -- /path/to/tokenizer.json json 10
cargo bench --bench print_pipeline -- /path/to/tokenizer.json
cargo bench --bench profile_sample -- /path/to/tokenizer.json 10
```

### Optional `.tkz` cache

```rust
let tokenizer = Tokenizer::load_file_with_tkz_cache(Path::new("/path/to/tokenizer.json"))?;
```

The first cached load atomically writes `tokenizer.tkz`; later loads validate and reuse it. Passing that `.tkz` path loads it directly without creating another copy. `Tokenizer::load_file` remains JSON-only and never writes a cache.

## Scope

Snaptokens is only built for inference on BPE tokenizers, and to do just that, well.
## Credits

Inspired by SIMD stuff from hyperscan and gigatoken. Also uses the .tkz extension from Tokie.

Licensed under [Apache-2.0](https://github.com/coder-2011/snaptokens/blob/main/LICENSE).
