# tryAGI/Tiktoken exclusion

Revision [`7318178538cb3f31fa560cf6c839d400723e8a99`](https://github.com/tryAGI/Tiktoken/tree/7318178538cb3f31fa560cf6c839d400723e8a99) is excluded from the GPT-2/r50k and GPT-OSS/o200k specialist benchmark. Its public `EncodeWithAllAllowedSpecial` API returns full ordered IDs, but every probe containing a JSON-configured special token followed by ordinary text disagreed with Hugging Face Tokenizers. The generic `TokenizerJsonLoader` is also ineligible for arbitrary Hugging Face BPE `tokenizer.json` files. No throughput was timed after these correctness gates failed.

The focused probe built the unmodified source in `Release` for `net10.0` with 0 warnings and 0 errors. The exact toolchain and build-relevant package pins were:

| Component | Pin |
| --- | --- |
| tryAGI/Tiktoken | `7318178538cb3f31fa560cf6c839d400723e8a99` |
| .NET SDK | `10.0.100`, SDK commit `b0f34d51fc` |
| .NET runtime | `10.0.0`, Darwin arm64 |
| Hugging Face `tokenizers` oracle | `0.22.2` |
| `DotNet.ReproducibleBuilds` | `2.0.5` |
| `MinVer` | `7.0.0` |
| `PolySharp` | `1.16.0` |
| `System.Text.Json` | `10.0.9` |

## Full-ID admission gate

Each constructed candidate was checked with cache disabled, cache enabled on its first call, and cache enabled on a repeated call. The result was then compared as a complete ordered ID vector with `Tokenizer.encode(text, add_special_tokens=false).ids` from the pinned JSON. The nine ordinary cases covered empty input, ASCII, isolated and trailing whitespace, newline/tab whitespace, CJK, emoji and ZWJ sequences, combining marks and multiple scripts, and deterministic text of exactly 65,536 UTF-8 bytes. Special-token coverage included each JSON-configured special embedded between ordinary prefix and suffix text plus all configured specials in one input.

| Model | Candidate | Configured specials tested | Rows | Exact | Mismatched | Cache-consistency failures |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| GPT-2 / r50k | Built-in `R50KBase` | 1 | 11 | 9 | 2 | 0 |
| GPT-2 / r50k | `TokenizerJsonLoader` | 1 | 11 | 9 | 2 | 0 |
| GPT-OSS / o200k | Custom public `Encoding` with the exact JSON map | 21 | 31 | 9 | 22 | 0 |
| GPT-OSS / o200k | Built-in `O200KBase` map | 2 of the JSON's 21 | 31 | 9 | 22 | 0 |
| GPT-OSS / o200k | `TokenizerJsonLoader` | n/a | 0 | 0 | 0 | n/a; construction failed |

All nine ordinary, whitespace, Unicode, and 64 KiB rows were exact for every candidate that constructed. All 48 emitted special-containing rows failed: 2 from each GPT-2 candidate and 22 from each constructed GPT-OSS candidate. Supplying the exact 21-token GPT-OSS special map through the public `Encoding` constructor did not fix the failures, so incomplete built-in special-token inventory is not the root cause.

Two minimal failures show the behavior:

| Encoding and input | tryAGI IDs | Hugging Face IDs |
| --- | --- | --- |
| GPT-2, `prefix <\|endoftext\|> suffix` | `[40290, 220, 50256, 40290, 220]` | `[40290, 220, 50256, 35488]` |
| GPT-OSS, `prefix <\|startoftext\|> suffix` | `[26155, 220, 199998, 26155, 220]` | `[26155, 220, 199998, 43412]` |

The suffix is tokenized as the prefix. In the audited source, the regex enumerates matches relative to `textSpan[start..specialStart]`, but [`fastKey` is sliced from the original span with the relative `match.Index`](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Core/CoreBPE.cs#L424-L430), omitting the required `start + match.Index`. After a special token, [`start` advances to the end of that token](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Core/CoreBPE.cs#L531-L541), which makes subsequent relative match offsets select text from the beginning of the original input. The same offset error exists in the .NET 8 branch. Patching it would create a different competitor revision, so this audit records the upstream behavior rather than benchmarking a local fork.

## Generic `tokenizer.json` loader gate

The pinned GPT-OSS file fails during `TokenizerJsonLoader.FromFile` before an encoder can be constructed:

```text
System.Text.Json.JsonException: The JSON value could not be converted to System.String. Path: $.model.merges[0]
```

That JSON represents merges as token-pair arrays, while the audited schema declares [`model.merges` as `IReadOnlyList<string>`](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Encodings.Tokenizer/TokenizerJson.cs#L94-L114). The loader is not a general Hugging Face BPE implementation even for files it can deserialize:

- Its [`TokenizerJson` schema](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Encodings.Tokenizer/TokenizerJson.cs#L8-L114) omits normalizer, post-processor, and decoder configuration. `added_tokens` retains only `id`, `special`, and `content`, dropping `single_word`, `lstrip`, `rstrip`, and `normalized` semantics.
- [`ToEncoding`](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Encodings.Tokenizer/TokenizerJsonLoader.cs#L171-L210) does not consume `model.merges`; it treats vocabulary token IDs as merge ranks, includes only tokens marked special, and reduces pre-tokenizer configuration to a detected regex family.

Accordingly, `TokenizerJsonLoader` is excluded as an arbitrary-tokenizer competitor independently of the fixed r50k/o200k special-token defect.

## Cache and batch observations

Cache-disabled, first cached, and repeated cached calls produced identical full IDs for all 84 emitted rows, including the incorrect special-containing rows. This establishes internal cache consistency only; it does not rescue reference parity. The implementation defaults to caching and stores pieces in [unbounded concurrent dictionaries](https://github.com/tryAGI/Tiktoken/blob/7318178538cb3f31fa560cf6c839d400723e8a99/src/libs/Tiktoken.Core/CoreBPE.cs#L36-L38), with no capacity, eviction, or clear API. Any future admissible revision therefore needs separate novel/uncached and repeated/warm-cache tracks.

Reflection found no public `Encoder` method containing `Batch` or `Parallel`. Upstream had already [removed its per-match `EncodeParallel` API](https://github.com/tryAGI/Tiktoken/commit/de4fe89c17f87de6d0393ac1c2afa11fe0f236c5) after its own benchmark reported 14% lower throughput on the long input, 5–12× lower throughput on small inputs, and roughly 7× more allocation. A separately labeled outer `Parallel.For` adapter over 128 ordered items, capped at `MaxDegreeOfParallelism = 4`, observed four worker threads and reproduced the candidate's scalar IDs for each constructed encoder. That adapter is not an upstream batch API and was not timed. Since scalar full-ID correctness failed, neither local macOS timings nor Linux/GPU-host timings were warranted.

## Pinned tokenizer inputs

| Input | Immutable source | SHA-256 |
| --- | --- | --- |
| GPT-2 `tokenizer.json` | [`gpt2@607a30d`](https://huggingface.co/gpt2/resolve/607a30d783dfa663caf39e06633721c8d4cfcd7e/tokenizer.json) | `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6` |
| GPT-OSS `tokenizer.json` | [`gpt-oss-20b@6cee5e8`](https://huggingface.co/openai/gpt-oss-20b/resolve/6cee5e81ee83917806bbde320786a8fb61efebee/tokenizer.json) | `0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3` |
| `r50k_base.tiktoken` | [`openaipublic` rank file](https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken) | `306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930` |
| `o200k_base.tiktoken` | [`openaipublic` rank file](https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken) | `446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d` |

## Reproduction record

The audit used a disposable project outside the repository so the rejected candidate could not affect the benchmark harness:

```bash
git clone https://github.com/tryAGI/Tiktoken /tmp/tryagi-tiktoken-audit
git -C /tmp/tryagi-tiktoken-audit checkout 7318178538cb3f31fa560cf6c839d400723e8a99

curl -fsSL https://dot.net/v1/dotnet-install.sh -o /tmp/dotnet-install.sh
bash /tmp/dotnet-install.sh --version 10.0.100 --architecture arm64 --os osx \
  --install-dir /tmp/dotnet-10.0.100 --no-path

DOTNET_CLI_HOME=/tmp/dotnet-home NUGET_PACKAGES=/tmp/nuget-tryagi DOTNET_NOLOGO=1 \
  /tmp/dotnet-10.0.100/dotnet build /tmp/tryagi-probe/TryAgiProbe.csproj \
  -c Release -f net10.0 --nologo

DOTNET_CLI_HOME=/tmp/dotnet-home NUGET_PACKAGES=/tmp/nuget-tryagi DOTNET_NOLOGO=1 \
  /tmp/dotnet-10.0.100/dotnet run --project /tmp/tryagi-probe/TryAgiProbe.csproj \
  -c Release -f net10.0 --no-build -- \
  /tmp/tryagi-tiktoken-audit/src/tests/Tiktoken.UnitTests/Resources/gpt2.tokenizer.json \
  /tmp/gpt-oss-tokenizer.json /tmp/tryagi-probe-output.jsonl

UV_CACHE_DIR=/tmp/uv-cache-tryagi uv run --isolated --no-project \
  --with tokenizers==0.22.2 python /tmp/tryagi-probe/compare_hf.py \
  /tmp/tryagi-probe-output.jsonl \
  /tmp/tryagi-tiktoken-audit/src/tests/Tiktoken.UnitTests/Resources/gpt2.tokenizer.json \
  /tmp/gpt-oss-tokenizer.json > /tmp/tryagi-hf-comparison.json
```

The comparison command exits nonzero by design when it records mismatches. The disposable local artifacts were:

| Artifact | SHA-256 |
| --- | --- |
| `/tmp/dotnet-install.sh` | `082f7685e156738a1b2e2ed8381a621870d4ce8e8c59278034556f05c186eb2e` |
| `/tmp/tryagi-probe/TryAgiProbe.csproj` | `b61a0af6e36da93c9b3cb9d3feaaf45580278c88d08ca3e3201aa67d7ff99f65` |
| `/tmp/tryagi-probe/Program.cs` | `7de829902f55d0a23a960c7ed1099ea06c76112928977a3ec0826456b443ac62` |
| `/tmp/tryagi-probe/compare_hf.py` | `132de6282ce217268978f74786f377a104843ee9a6b148a292b2237f7a95d991` |
| `/tmp/tryagi-probe-output.jsonl` | `84f5da26f19bbc404d4b7885621c24256084763b291128b8544bbde255c93828` |
| `/tmp/tryagi-hf-comparison.json` | `763d296d4c155f9ee03c032ff68b6e80b122cb62c6688ed2a1e639847bbc0fd4` |

The JSONL artifact is 897,348 bytes and the comparison report is 19,505 bytes. A different parity result means the source revision, SDK, package restore, tokenizer files, reference package, or probe source has drifted.
