# Fixed-encoding specialist benchmarks

This lane compares Snaptokens with fast, fixed tiktoken-family Rust and C++
implementations. It is deliberately separate from the generic
`tokenizer.json` matrix: none of the competitors accepts arbitrary Hugging Face
pipelines.

The pinned candidates are:

- [OpenAI `tiktoken` 0.13.0](https://github.com/openai/tiktoken/tree/08a5f3b2c987ada4fc5aa1f16c643c203fa8acaa),
  revision `08a5f3b2c987ada4fc5aa1f16c643c203fa8acaa`.
- [`rust-tiktoken` 3.5.1](https://github.com/goliajp/rust-tiktoken/tree/v3.5.1),
  revision `1e3efa1c665b646dfe46cc667b4d46dddf56cf71`.
- [`riptoken` 0.3.0](https://github.com/daechoi/riptoken/tree/v0.3.0),
  revision `1ac8add56e02c3d711b13f1059f6b522ee7b2010`.
- [`wordchipper` 0.9.2](https://github.com/zspacelabs/wordchipper/tree/e82a4bdb656afe6d212f4ffbaa4f14e7d7071fca),
  revision `e82a4bdb656afe6d212f4ffbaa4f14e7d7071fca`.
- [`blaze-bpe` 0.1.0](https://github.com/Lulzx/blaze-bpe/tree/b753de2a2688c4acf09e64605585ea8e514dddd4),
  revision `b753de2a2688c4acf09e64605585ea8e514dddd4`; GPT-OSS/o200k only.
- [TokenDagger](https://github.com/M4THYOU/TokenDagger/tree/1eed815d1ef3a23a73b0d6554f99fc0be6ff4164),
  revision `1eed815d1ef3a23a73b0d6554f99fc0be6ff4164`; GPT-2/r50k only.

## Eligibility

Every implementation uses a public text-to-token-ID API. Batch 1 calls each
direct encoder. Wider batches use Snaptokens, Riptoken, and Wordchipper's native
batch APIs; `tiktoken`, which has no batch API, gets an order-preserving outer
Rayon adapter. OpenAI `tiktoken` and `rust-tiktoken` are distinct engines and
are labeled separately. Blaze-BPE's public batch method skips configured
specials, so its GPT-OSS lane uses the same adapter around
`encode_with_specials`. TokenDagger also uses the ordered adapter around its
public scalar API; its measured bridge copies input bytes into `std::string`
and copies returned IDs into the Rust result.
Construction, vocabulary parsing, the Hugging Face oracle, and one disjoint
whole-batch initialization call stay outside the timer. Output allocation and
destruction stay inside it.

Each round consumes new, non-overlapping corpus slices. Complete Williams
cycles balance implementation position and within-round first-order carryover.
The default 14 requested rounds expand to 18 for both models' six timed
candidates.
Timings are buffered until the same persistent instances match Hugging Face
`add_special_tokens=false` on:

- multilingual, Unicode-boundary, whitespace, punctuation, and code probes;
- every `tokenizer.json` AddedToken in isolation and context;
- candidate-only special-token spellings; and
- every input from every timed batch.

The harness emits no measurement row for an inexact candidate. A reversible
decode or matching token count is not enough.

GPT-2 is eligible for all specialists except Blaze-BPE, whose public scanner has
no r50k pattern. TokenDagger is not executed on GPT-OSS: its pinned special-token
lookup erases the current map key while continuing that range iteration, as the
retained [source audit](audits/tokendagger.md) documents. GPT-OSS needs its exact
21-entry AddedToken map, which OpenAI `tiktoken`, Riptoken, and Blaze-BPE receive
at construction and Wordchipper receives as a public special-token filter.
`rust-tiktoken` 3.5.1's public `o200k_harmony` table is not configurable and is
broader than the pinned HF file: the HF file assigns `<|endofprompt|>` to
`200018`, while that release recognizes `<|reserved_200018|>` instead. The
harness probes both spellings and discards any candidate's timings if the
complete IDs differ.

## Immutable inputs

The runner and binary both verify the official rank artifacts:

| Encoding | URL | SHA-256 |
| --- | --- | --- |
| `r50k_base` | `https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken` | `306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930` |
| `o200k_base` | `https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken` | `446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d` |

The two tokenizer JSON hashes and both corpus hashes are also fixed in
`run-matrix.sh`. Output records the source commit, binary and lockfile hashes,
Rust and C++ compilers and flags, CPU fingerprint and affinity, thread count,
and the exact PCRE2 header/runtime versions, JIT capability, library path, and
library hash. PCRE2 runtime identity is part of the resume signature.

## Run

Install PCRE2 development headers and its shared library. Build with the exact
flags that will be recorded, then run from a committed, clean checkout:

```bash
SNAPTOKENS_BUILD_SOURCE_COMMIT=$(git rev-parse HEAD) \
CXX=c++ RUSTFLAGS='-C target-cpu=native' \
  cargo build --release --locked \
  --manifest-path benchmarks/specialists/Cargo.toml

TOKENIZER_DIR=/path/to/tokenizers \
CORPUS_DIR=/path/to/corpora \
RANK_DIR=/path/to/ranks \
OUTPUT_DIR=/path/to/output \
HOST_LABEL=my-cpu-host \
CPU_SET=0-31 \
THREADS=32 \
BUILD_COMMAND="SNAPTOKENS_BUILD_SOURCE_COMMIT=$(git rev-parse HEAD) CXX=c++ RUSTFLAGS='-C target-cpu=native' cargo build --release --locked --manifest-path benchmarks/specialists/Cargo.toml" \
BUILD_RUSTFLAGS='-C target-cpu=native' \
  benchmarks/specialists/run-matrix.sh
```

Tokenization is CPU-only. An attached GPU is neither used nor part of the
reported throughput.

[`tiktoken-rs` 0.12.0](https://github.com/zurawiki/tiktoken-rs/tree/a79050e)
is not a separate bar because that release backports the same OpenAI 0.13 core.
Counting the same implementation twice would exaggerate competitor coverage.

## Researched exclusion: Runtoken

[`runtoken` 0.1.2](https://github.com/Thibault00/runtoken/tree/v0.1.2) has a
public Rust encoder, but it does not support `r50k_base`/GPT-2 or
`o200k_harmony`/GPT-OSS and does not implement the required added-special-token
semantics. Its headline gains also rely heavily on a 2,048-entry full-text LRU
and an 8,192-entry chunk LRU. `encode_no_text_cache` leaves the chunk cache
enabled, so there is no public cache-free API that meets this lane's semantic
contract. It is researched but not measured here.
