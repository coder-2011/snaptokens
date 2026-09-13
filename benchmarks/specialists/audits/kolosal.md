# KolosalAI `tiktoken.cpp` exclusion

Revision [`42f88725052992c902c0aafaa87fb23365f7f102`](https://github.com/KolosalAI/tiktoken.cpp/tree/42f88725052992c902c0aafaa87fb23365f7f102) exposes the required native surface: `EncodingDefinition` accepts the pattern, merge ranks, and arbitrary special-token map, and `Encoding::encode` returns ordered IDs. The admission build used its upstream release flags (`-O3 -march=native -mtune=native -ffast-math -funroll-loops`) and the source-supported `TIKTOKEN_NO_JIT=1` switch. The switch is required on Apple arm64 with PCRE2 10.47 because copied regex objects otherwise report `bad JIT option`; it does not change tokenization semantics.

The candidate is excluded because the unmodified public scalar API truncates the existing `"a".repeat(1024)` model probe on both supported encodings:

| Model | Input index | Zero-based token position | Raw oracle/candidate fragment |
| --- | ---: | ---: | --- |
| GPT-2 / r50k | 11 | 255 | `expected=Some(24794) actual=None` |
| GPT-OSS / o200k | 11 | 127 | `expected=Some(117525) actual=None` |

The candidate therefore received `probe_mismatch_timings_discarded`; none of its timed rows were admissible. Patching its merge implementation would turn this into a different competitor, so the throughput harness does not build or schedule it.

## Pinned inputs

| Input | URL | SHA-256 |
| --- | --- | --- |
| GPT-2 `tokenizer.json` | `https://huggingface.co/openai-community/gpt2/resolve/main/tokenizer.json` | `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6` |
| GPT-OSS `tokenizer.json` | `https://huggingface.co/openai/gpt-oss-20b/resolve/main/tokenizer.json` | `0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3` |
| `r50k_base.tiktoken` | `https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken` | `306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930` |
| `o200k_base.tiktoken` | `https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken` | `446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d` |

## Reproduction record

The focused admission driver was disposable and was intentionally removed with the rejected bridge. The committed `snaptokens-specialists` binary does not build KolosalAI and must not be used to reproduce this audit.

Recreate the pinned native dependencies with:

```bash
git clone https://github.com/KolosalAI/tiktoken.cpp /tmp/tiktoken.cpp
git -C /tmp/tiktoken.cpp checkout 42f88725052992c902c0aafaa87fb23365f7f102
git clone https://github.com/nemtrif/utfcpp /tmp/utfcpp
git -C /tmp/utfcpp checkout ca9e9bcc6ca05540f474b0d0e98f320b86233568
```

Then write a standalone driver against the public header only:

1. Load the appropriate pinned rank file with `tiktoken::load_tiktoken_bpe_from_file`.
2. Copy `GPT2_PATTERN` or `O200K_PATTERN` from `benchmarks/specialists/src/main.rs` and copy every `(content, id)` pair from that JSON's `added_tokens` into `EncodingDefinition::special_tokens`.
3. Construct `tiktoken::Encoding`, put the same special strings in the allowed set, and call `encode(std::string(1024, 'a'), allowed_special)`.
4. Compare the ordered vector with Hugging Face `encode(..., add_special_tokens=false)` from the pinned JSON.

Compile that driver with the audited flags and run it with the source-supported fallback:

```bash
c++ -std=c++17 -O3 -march=native -mtune=native -ffast-math -funroll-loops \
  -I/tmp/tiktoken.cpp/src -I/tmp/utfcpp/source \
  -I"$(brew --prefix pcre2)/include" -L"$(brew --prefix pcre2)/lib" \
  kolosal_probe.cpp -lpcre2-8 -o kolosal-probe
TIKTOKEN_NO_JIT=1 ./kolosal-probe
```

The first absent candidate IDs must match the raw fragments recorded above; a different result means the source, input hash, regex, special map, PCRE2 behavior, or build configuration drifted.
