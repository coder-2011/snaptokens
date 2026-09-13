# 2026-07-16 benchmark evidence

This directory contains the complete retained evidence for the AMD EPYC 9655
results in [`benchmarks/speed.md`](../../speed.md). The measured lanes share one
host, affinity, and compiler policy, but they are not pooled across source
revisions or API contracts.

| Lane | Snaptokens source | Purpose |
| --- | --- | --- |
| Generic `tokenizer.json` | `e23b4ff90410bc70a095af8c2c0b0dd63b848493` | Arbitrary compatible JSON engines, native batch APIs, and exact ordered IDs |
| Fixed GPT specialists | `f9ed5ca1fc990aa1089008dd0b1c4b75858f17c5` | GPT-2 and GPT-OSS encoders with fixed vocabularies or public specialist APIs |
| Long single document | `f9ed5ca1fc990aa1089008dd0b1c4b75858f17c5` | Scalar 256 KiB, 1 MiB, and 4 MiB inputs |
| Fresh-process load | `18f7e66512a94aac296eb36b7fa5119f5c5351c8` | JSON, validated `.tkz` sidecar, direct `.tkz`, and Kitoken `.kit` construction |

The raw rows carry the source revision, binary and lockfile hashes, input
hashes, CPU allocation, and compiler settings available to that lane. The
environment capture retains the generic host-fingerprint input and recipe. A
result from one lane is never combined with another lane to produce a speedup.

## Layout

- `verda-amd-epyc-9655-32c/generic` contains one JSONL file per generic matrix
  cell and its atomic `.done` marker.
- `specialists` and `long-single` contain the corresponding exact-ID timing
  cells for their narrower APIs.
- `load` contains fresh-process observations, file probes, and the byte-exact
  frozen-artifact manifests captured before and after timing. The shorter
  `frozen-artifacts.sha256` rewrites their remote prefix to `frozen/` for
  readability; its content hashes are unchanged, but its own file hash differs.
- `summaries` contains the exact coverage ledgers, paired ratios, equal-cell
  aggregates, complete observed-loss tables, and chart-source CSV files.
- `logs` retains build and run output. `environment` captures the provider VM,
  CPU topology, frequency policy, kernel, libc, toolchains, process context,
  and artifact hashes after timing stopped.
- `SHA256SUMS` covers every retained file below this dated directory except the
  manifest itself.

The tree is auditable and byte-integrity-checked, not a claim that every large
input is stored in Git. Tokenizer, corpus, native-format, benchmark-binary, and
system-library hashes bind the run, while the input table below identifies how
public inputs were obtained. Temporary analysis tooling is not shipped as
Python in this Rust-only repository.

## Inputs

Every tokenizer came from a commit-pinned Hugging Face URL. The corpora came
from immutable downloads; the output hashes bind the exact bytes timed.

| Input | Pinned source | SHA-256 |
| --- | --- | --- |
| GPT-2 tokenizer | [`gpt2@607a30d`](https://huggingface.co/gpt2/resolve/607a30d783dfa663caf39e06633721c8d4cfcd7e/tokenizer.json) | `8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6` |
| Llama 3 tokenizer | [`NousResearch/Meta-Llama-3-8B@315b200`](https://huggingface.co/NousResearch/Meta-Llama-3-8B/resolve/315b20096dc791d381d514deb5f8bd9c8d6d3061/tokenizer.json) | `e134af98b985517b4f068e3755ae90d4e9cd2d45d328325dc503f1c6b2d06cc7` |
| Qwen 2.5 tokenizer | [`Qwen/Qwen2.5-7B-Instruct@a09a354`](https://huggingface.co/Qwen/Qwen2.5-7B-Instruct/resolve/a09a35458c702b33eeacc393d103063234e8bc28/tokenizer.json) | `c0382117ea329cdf097041132f6d735924b697924d6f6fc3945713e96ce87539` |
| Qwen 3 tokenizer | [`Qwen/Qwen3-0.6B@c1899de`](https://huggingface.co/Qwen/Qwen3-0.6B/resolve/c1899de289a04d12100db370d81485cdf75e47ca/tokenizer.json) | `aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4` |
| DeepSeek R1 tokenizer | [`deepseek-ai/DeepSeek-R1@56d4cbb`](https://huggingface.co/deepseek-ai/DeepSeek-R1/resolve/56d4cbbb4d29f4355bab4b9a39ccb717a14ad5ad/tokenizer.json) | `ecb6f9fc369894346f0511f4074ca75cee5cd5f3b06d02f1ba35fcd39f8e121d` |
| GPT-OSS tokenizer | [`openai/gpt-oss-20b@6cee5e8`](https://huggingface.co/openai/gpt-oss-20b/resolve/6cee5e81ee83917806bbde320786a8fb61efebee/tokenizer.json) | `0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3` |
| Nemotron 3 tokenizer | [`nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16@cbd3fa9`](https://huggingface.co/nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16/resolve/cbd3fa9f933d55ef16a84236559f4ee2a0526848/tokenizer.json) | `c6021eb6847e682f89aa52d5eb6e8c7d902a23acfc8137e25211cf84828f1592` |
| MiniMax M2.1 tokenizer | [`MiniMaxAI/MiniMax-M2.1@cd97f59`](https://huggingface.co/MiniMaxAI/MiniMax-M2.1/resolve/cd97f59135f37b2a6bf09356e485d5e4aeb7dc9c/tokenizer.json) | `757622126525aeeb131756849d93298070ff3f0319c455ec8c5bb0f6b1cebbe8` |
| Mistral Nemo tokenizer | [`mistralai/Mistral-Nemo-Instruct-2407@04d8a90`](https://huggingface.co/mistralai/Mistral-Nemo-Instruct-2407/resolve/04d8a90549d23fc6bd7f642064003592df51e9b3/tokenizer.json) | `e11c71726323d33da7b8d6f6f269f1988931c0a52b7122bcdd8c05042974e0db` |
| GLM 4.7 tokenizer | [`zai-org/GLM-4.7@602d01e`](https://huggingface.co/zai-org/GLM-4.7/resolve/602d01efcdd332c5238ca4bcede555defbe83eb7/tokenizer.json) | `9340665016419c825c4bdabbcc9acc43b7ca2c68ce142724afa829abb1be5efd` |
| Phi-4 Mini tokenizer | [`microsoft/Phi-4-mini-instruct@cfbefac`](https://huggingface.co/microsoft/Phi-4-mini-instruct/resolve/cfbefacb99257ffa30c83adab238a50856ac3083/tokenizer.json) | `382cc235b56c725945e149cc25f191da667c836655efd0857b004320e90e91ea` |
| `enwik8` archive | [`enwik8.zip`](https://mattmahoney.net/dc/enwik8.zip) | `547994d9980ebed1288380d652999f38a14fe291a6247c157c3d33d4932534bc` |
| `enwik8` extracted bytes | First 100,000,000 bytes from the archive | `2b49720ec4d78c3c9fabaee6e4179a5e997302b3a70029f30f2d582218c024a8` |
| Rust 1.88.0 source archive | [`rust-1.88.0.tar.gz`](https://github.com/rust-lang/rust/archive/refs/tags/1.88.0.tar.gz) | `0c6095e659991ed9ae4fb029003e7d90d0ffc775d6112d500cdf7eab3bd3e5ef` |
| Concatenated Rust corpus | Sorted `*.rs` files from the archive | `ed855017307cfc188f89f5f8185ac00d93b4257dd2dbf348459dc93191b0fd15` |

## Reading the numbers

Every timed implementation must first match Hugging Face's complete ordered ID
sequence on all probes and all timed inputs. A token-count match, reversible
decode, or approximate segmentation does not pass. Inexact and unsupported
implementations remain in the coverage CSVs and contribute no timing row.

For one exact cell, `speedup = competitor elapsed time / Snaptokens elapsed
time` over equal bytes and equal rounds. Values above `1` favor Snaptokens.
Pairwise aggregate speedups are geometric means of those cell ratios. Common
throughput uses only the intersection of cells exact for every named engine;
each cell receives equal weight. Fresh-process load bars use the arithmetic
mean of the five per-model means because each process performs one timed load;
process startup itself is excluded.

The runner expands the requested round count to a balanced Williams design, so
the recorded `actual_rounds` can exceed the command-line minimum. Position and
first-order carryover are balanced before a cell is accepted. Warmup,
tokenizer construction, corpus preparation, and parity-oracle work are outside
encode timers; each implementation's public output allocation remains inside.

The generic, fixed-specialist, long-document, and load lanes answer different
questions. Their results are shown separately in the report and must not be
merged into one universal average.
