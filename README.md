<h1 align="center">snaptokens</h1>

<p align="center">
Fast BPE and Unigram tokenization in Rust.
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/coder-2011/snaptokens/main/assets/benchmark-throughput-a8b4520196a2.svg" alt="Throughput relative to Hugging Face for batches of 1, 32, and 512, then single 4 KiB and 64 KiB inputs. Snaptokens: 58.2×, 24.2×, 20.5×, 94.9×, 107.2×. Gigatoken: 16.7×, 12.8×, 14.8×, 52.7×, 77.9×.">
</p>

snaptokens is a high-performance tokenizer, built to be compatible with hf tokenziers.

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
- Opt-in `.st` snapshots store native BPE tables or validated Unigram inputs to reduce tokenizer loading work.

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

Python's `encode()`, `encode_batch()`, and `encode_batch_flat()` apply the
tokenizer's configured special tokens by default, matching Hugging Face.
Pass `add_special_tokens=False` to omit them.

`enable_truncation(max_length=100, direction="right")` limits the returned
sequence, including requested post-processor special tokens. 

Pass `st_cache=True` to create and reuse an adjacent `.st` snapshot. A direct `.st`
path loads without that flag.

To replace a Transformers v4 or v5 fast-tokenizer backend:

```python
import snaptokens

snaptokens.patch_transformers()
```

Call `snaptokens.unpatch_transformers()` to restore the original backend.

Rust:

```rust
use std::error::Error;

use snaptokens::Tokenizer;

/// Loads one local tokenizer and encodes a prompt.
fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::load_file("/path/to/tokenizer.json")?;
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

### Optional `.st` snapshots

Create and reuse a snapshot from a compatible Hugging Face `tokenizer.json`:

```python
from snaptokens import Tokenizer

tokenizer = Tokenizer.from_file("/path/to/tokenizer.json", st_cache=True)
# The snapshot also loads without the original JSON or a cache flag.
restored = Tokenizer.from_file("/path/to/tokenizer.st")
```

```rust
use snaptokens::Tokenizer;

let tokenizer = Tokenizer::load_file_with_st_cache("/path/to/tokenizer.json")?;
let restored = Tokenizer::load_file_with_st_cache("/path/to/tokenizer.st")?;
```

The first cached JSON load writes a sibling `tokenizer.st` atomically after the
complete tokenizer has been constructed. Later cached loads validate and reuse
the snapshot without reading the JSON. The payload checksum detects corruption;
changes to the source JSON are not checked. Delete the sibling `.st` to rebuild
it after editing the JSON. An invalid or unsupported snapshot is rebuilt when
its JSON source is available. Direct `.st` loads report validation errors without
rebuilding. JSON-only loading remains read-only.

BPE version 3 stores native lookup and merge tables with a packed vocabulary.
Unigram version 4 stores vocabulary strings, exact scores, the unknown-token ID,
and byte-fallback settings, then validates them and rebuilds the matcher on load.
Both preserve the tokenizer pipeline, including added tokens, normalization,
pre-tokenization, post-processing and decoding. Python also restores stored
padding and supported truncation settings. `.st` does not expand support to
otherwise unsupported tokenizer pipelines or native SentencePiece `.model` files.

The recorded twelve-model BPE comparison measured the following warm-load ratios:

| CPU | JSON / ST |
| --- | ---: |
| Apple ARM | 9.33× |
| AMD x86-64 | 10.86× |
| Intel x86-64 | 9.73× |

These are descriptive geometric-mean ratios from twelve rounds in a fixed format
order, including load and destruction. They measure loading, not encoding, and
are not a randomized cross-format comparison. The [BPE results and method](autoresearch/results/st-20260917/format-comparison.json)
identify the measured commits, which precede the structural refactor and format
cleanup. The current PR head has not been remeasured.

A separate pinned T5/UMT5 comparison on one Intel host measured direct `.st`
loading at 1.157× JSON throughput across twelve paired rounds. See the
[Unigram results](autoresearch/results/st-20260917/unigram-format-baseline-v1/summary.json).
This Unigram result has not been confirmed across CPU classes.

## Scope

Snaptokens provides inference for BPE tokenizers and compatible Hugging Face
`tokenizer.json` Unigram pipelines, with optional `.st` snapshots for both. Native
SentencePiece `.model` files are not supported.

## Credits

Licensed under [Apache-2.0](https://github.com/coder-2011/snaptokens/blob/main/LICENSE).

## Status

Unchecked items are not currently supported.

- [x] **Hugging Face JSON Unigram:** Tagged and legacy untagged Unigram JSON, including T5-style Precompiled normalization, `WhitespaceSplit → Metaspace`, and Metaspace decoding.
- [ ] **Other model algorithms:** WordPiece, WordLevel, and model types other than BPE or Unigram. Native SentencePiece `.model` files are explicitly rejected.
- [ ] **BPE options beyond its core:** `dropout`, `unk_token`, `fuse_unk`, `continuing_subword_prefix`, and `end_of_word_suffix` are not represented or guaranteed exact.
- [ ] **Normalizers:** NFD/NFKC/NFKD, Lowercase, Strip, and BertNormalizer.
- [ ] **Pre-tokenizers:** Whitespace (distinct from supported WhitespaceSplit), Bert, Digits, Punctuation, and UnicodeScripts.
- [ ] **Post-processors:** Bert/Roberta processors and pair-sequence processing; parsed pair templates are unused.
- [ ] **Decoders:** WordPiece, BPE, CTC, and Strip decoders.
- [ ] **Training:** Tokenizer training and vocabulary/model-construction APIs.
- [ ] **Pair encoding:** Pair encoding and pair post-processing; Python raises `NotImplementedError`.
- [ ] **Offset/word metadata:** Python token strings, character offsets, sequence IDs, word IDs, and overflow rows after truncation; the associated mapping methods raise `NotImplementedError`.
- [x] **Serialized padding/truncation settings:** Python JSON and `.st` loading restore stored padding and supported single-sequence truncation settings. Nonzero stride and `only_second` are rejected because overflow rows and pair encoding are unsupported.
- [ ] **Splitting added special tokens:** `encode_special_tokens=True` raises `NotImplementedError`.
