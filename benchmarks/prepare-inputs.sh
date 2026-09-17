#!/usr/bin/env bash

set -euo pipefail

: "${OUTPUT_DIR:?set OUTPUT_DIR to an empty writable input directory}"

TOKENIZER_DIR="$OUTPUT_DIR/tokenizers"
CORPUS_DIR="$OUTPUT_DIR/corpora"
ARCHIVE_DIR="$OUTPUT_DIR/archives"
WORK_DIR="$OUTPUT_DIR/work"

mkdir -p "$TOKENIZER_DIR" "$CORPUS_DIR" "$ARCHIVE_DIR" "$WORK_DIR"

fetch_verified() {
  local url=$1
  local destination=$2
  local expected_sha256=$3
  local temporary="$destination.tmp"
  local actual_sha256

  if [[ -e $destination ]]; then
    actual_sha256=$(sha256sum "$destination" | cut -d' ' -f1)
    if [[ $actual_sha256 == "$expected_sha256" ]]; then
      return
    fi
    printf 'refusing to replace mismatched existing input: %s\n' "$destination" >&2
    return 1
  fi

  rm -f "$temporary"
  curl --fail --location --retry 3 --proto '=https' "$url" -o "$temporary"
  actual_sha256=$(sha256sum "$temporary" | cut -d' ' -f1)
  if [[ $actual_sha256 != "$expected_sha256" ]]; then
    rm -f "$temporary"
    printf '%s SHA-256 mismatch: expected %s, got %s\n' "$destination" "$expected_sha256" "$actual_sha256" >&2
    return 1
  fi
  mv "$temporary" "$destination"
}

fetch_tokenizer() {
  local name=$1
  local repository=$2
  local revision=$3
  local expected_sha256=$4
  fetch_verified \
    "https://huggingface.co/$repository/resolve/$revision/tokenizer.json" \
    "$TOKENIZER_DIR/$name.json" \
    "$expected_sha256"
}

fetch_tokenizer gpt-2 openai-community/gpt2 607a30d783dfa663caf39e06633721c8d4cfcd7e 8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6
fetch_tokenizer llama-3 NousResearch/Meta-Llama-3-8B 315b20096dc791d381d514deb5f8bd9c8d6d3061 e134af98b985517b4f068e3755ae90d4e9cd2d45d328325dc503f1c6b2d06cc7
fetch_tokenizer qwen-2-5 Qwen/Qwen2.5-7B-Instruct a09a35458c702b33eeacc393d103063234e8bc28 c0382117ea329cdf097041132f6d735924b697924d6f6fc3945713e96ce87539
fetch_tokenizer qwen-3 Qwen/Qwen3-0.6B c1899de289a04d12100db370d81485cdf75e47ca aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4
fetch_tokenizer deepseek-r1 deepseek-ai/DeepSeek-R1 56d4cbbb4d29f4355bab4b9a39ccb717a14ad5ad ecb6f9fc369894346f0511f4074ca75cee5cd5f3b06d02f1ba35fcd39f8e121d
fetch_tokenizer gpt-oss openai/gpt-oss-20b 6cee5e81ee83917806bbde320786a8fb61efebee 0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3
fetch_tokenizer nemotron-3 nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16 cbd3fa9f933d55ef16a84236559f4ee2a0526848 c6021eb6847e682f89aa52d5eb6e8c7d902a23acfc8137e25211cf84828f1592
fetch_tokenizer minimax-m2-1 MiniMaxAI/MiniMax-M2.1 cd97f59135f37b2a6bf09356e485d5e4aeb7dc9c 757622126525aeeb131756849d93298070ff3f0319c455ec8c5bb0f6b1cebbe8
fetch_tokenizer mistral-nemo mistralai/Mistral-Nemo-Instruct-2407 04d8a90549d23fc6bd7f642064003592df51e9b3 e11c71726323d33da7b8d6f6f269f1988931c0a52b7122bcdd8c05042974e0db
fetch_tokenizer glm-4-7 zai-org/GLM-4.7 602d01efcdd332c5238ca4bcede555defbe83eb7 9340665016419c825c4bdabbcc9acc43b7ca2c68ce142724afa829abb1be5efd
fetch_tokenizer phi-4-mini microsoft/Phi-4-mini-instruct cfbefacb99257ffa30c83adab238a50856ac3083 382cc235b56c725945e149cc25f191da667c836655efd0857b004320e90e91ea
fetch_tokenizer gemma-3 unsloth/gemma-3-1b-it 5b11413a10db4e486ef16a20101fd028f8f2499c 4667f2089529e8e7657cfb6d1c19910ae71ff5f28aa7ab2ff2763330affad795

fetch_verified \
  https://mattmahoney.net/dc/enwik8.zip \
  "$ARCHIVE_DIR/enwik8.zip" \
  547994d9980ebed1288380d652999f38a14fe291a6247c157c3d33d4932534bc

# The first 100,000,000 decompressed archive bytes are the published enwik8 input.
if [[ ! -e $CORPUS_DIR/enwik8.txt ]]; then
  unzip -p "$ARCHIVE_DIR/enwik8.zip" enwik8 >"$CORPUS_DIR/enwik8.txt.tmp"
  mv "$CORPUS_DIR/enwik8.txt.tmp" "$CORPUS_DIR/enwik8.txt"
fi
[[ $(sha256sum "$CORPUS_DIR/enwik8.txt" | cut -d' ' -f1) == 2b49720ec4d78c3c9fabaee6e4179a5e997302b3a70029f30f2d582218c024a8 ]]

fetch_verified \
  https://github.com/rust-lang/rust/archive/refs/tags/1.88.0.tar.gz \
  "$ARCHIVE_DIR/rust-1.88.0.tar.gz" \
  0c6095e659991ed9ae4fb029003e7d90d0ffc775d6112d500cdf7eab3bd3e5ef

# Preserve the historical corpus recipe: raw regular .rs bytes sorted by full archive path, no separators.
if [[ ! -e $CORPUS_DIR/rust-code.txt ]]; then
  rust_extract_dir=$(mktemp -d "$WORK_DIR/rust-1.88.0.XXXXXX")
  trap 'rm -rf "$rust_extract_dir"' EXIT
  tar -xzf "$ARCHIVE_DIR/rust-1.88.0.tar.gz" -C "$rust_extract_dir"
  find "$rust_extract_dir/rust-1.88.0" -type f -name '*.rs' -print \
    | LC_ALL=C sort \
    | while IFS= read -r path; do cat "$path"; done >"$CORPUS_DIR/rust-code.txt.tmp"
  mv "$CORPUS_DIR/rust-code.txt.tmp" "$CORPUS_DIR/rust-code.txt"
  rm -rf "$rust_extract_dir"
  trap - EXIT
fi
[[ $(sha256sum "$CORPUS_DIR/rust-code.txt" | cut -d' ' -f1) == ed855017307cfc188f89f5f8185ac00d93b4257dd2dbf348459dc93191b0fd15 ]]

sha256sum "$TOKENIZER_DIR"/*.json "$CORPUS_DIR"/*.txt >"$OUTPUT_DIR/SHA256SUMS"
