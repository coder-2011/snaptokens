#!/usr/bin/env bash

set -euo pipefail

: "${TOKENIZER_DIR:?set TOKENIZER_DIR}"
: "${CORPUS_DIR:?set CORPUS_DIR}"
: "${OUTPUT_DIR:?set OUTPUT_DIR}"
: "${HOST_LABEL:?set HOST_LABEL}"
: "${CPU_SET:?set CPU_SET}"
: "${THREADS:?set THREADS}"
: "${BUILD_COMMAND:?set BUILD_COMMAND to the exact release build command}"
: "${BUILD_RUSTFLAGS:?set BUILD_RUSTFLAGS used for the release build}"

HARNESS_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$HARNESS_DIR/../.." && pwd)
BIN=${BIN:-$HARNESS_DIR/target/release/snaptokens-benchmark}
ROUNDS=${ROUNDS:-14}
SAMPLE_MIB=${SAMPLE_MIB:-4}
RUN_VERSION=6

SOURCE_COMMIT=$(git -C "$REPO_ROOT" rev-parse HEAD)
BINARY_SHA256=$(sha256sum "$BIN" | cut -d' ' -f1)
CARGO_LOCK_SHA256=$(sha256sum "$HARNESS_DIR/Cargo.lock" | cut -d' ' -f1)
RUSTC_VERSION=$(rustc --version)
HOST_FINGERPRINT=$(
  {
    LC_ALL=C uname -srvm
    LC_ALL=C lscpu -p=CPU,CORE,SOCKET,NODE,ONLINE
    LC_ALL=C awk -F: '/^(model name|cpu family|model|stepping|microcode)[[:space:]]*:/ { gsub(/^[ \t]+|[ \t]+$/, "", $1); gsub(/^[ \t]+|[ \t]+$/, "", $2); print $1 ":" $2 }' /proc/cpuinfo | sort -u
  } | sha256sum | cut -d' ' -f1
)

if [[ -n $(git -C "$REPO_ROOT" status --porcelain --untracked-files=all -- benchmarks/harness) ]] ||
  ! git -C "$REPO_ROOT" diff --quiet ||
  ! git -C "$REPO_ROOT" diff --cached --quiet; then
  printf 'benchmark source is not committed and clean; commit it before measuring\n' >&2
  exit 1
fi

if [[ -n ${IREE_SOURCE_DIR+x} ]]; then
  printf 'IREE_SOURCE_DIR must be unset so the pinned dependency source is built\n' >&2
  exit 1
fi

AFFINITY_CPUS=$(taskset -c "$CPU_SET" nproc)
if [[ $THREADS != "$AFFINITY_CPUS" ]]; then
  printf 'THREADS=%s differs from %s CPUs in CPU_SET=%s\n' "$THREADS" "$AFFINITY_CPUS" "$CPU_SET" >&2
  exit 1
fi

TOKENIZERS=(
  'deepseek-r1|ecb6f9fc369894346f0511f4074ca75cee5cd5f3b06d02f1ba35fcd39f8e121d'
  'glm-4-7|9340665016419c825c4bdabbcc9acc43b7ca2c68ce142724afa829abb1be5efd'
  'gpt-2|8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6'
  'gpt-oss|0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3'
  'llama-3|e134af98b985517b4f068e3755ae90d4e9cd2d45d328325dc503f1c6b2d06cc7'
  'minimax-m2-1|757622126525aeeb131756849d93298070ff3f0319c455ec8c5bb0f6b1cebbe8'
  'mistral-nemo|e11c71726323d33da7b8d6f6f269f1988931c0a52b7122bcdd8c05042974e0db'
  'nemotron-3|c6021eb6847e682f89aa52d5eb6e8c7d902a23acfc8137e25211cf84828f1592'
  'phi-4-mini|382cc235b56c725945e149cc25f191da667c836655efd0857b004320e90e91ea'
  'qwen-2-5|c0382117ea329cdf097041132f6d735924b697924d6f6fc3945713e96ce87539'
  'qwen-3|aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4'
)

SWEEP_MODELS=(gpt-2 qwen-3 deepseek-r1 gpt-oss mistral-nemo)

mkdir -p "$OUTPUT_DIR"

verify_sha256() {
  local path=$1
  local expected=$2
  local label=$3
  local actual
  actual=$(sha256sum "$path" | cut -d' ' -f1)
  if [[ $actual != "$expected" ]]; then
    printf '%s SHA-256 mismatch: expected %s, got %s\n' "$label" "$expected" "$actual" >&2
    return 1
  fi
}

tokenizer_sha() {
  local wanted=$1
  local entry
  for entry in "${TOKENIZERS[@]}"; do
    if [[ ${entry%%|*} == "$wanted" ]]; then
      printf '%s\n' "${entry#*|}"
      return 0
    fi
  done
  return 1
}

run_cell() {
  local model=$1
  local corpus_name=$2
  local corpus_spec=$3
  local batch=$4
  local track=$5
  local sha
  local stem
  local output
  local signature
  local done_file

  sha=$(tokenizer_sha "$model") || return 1
  stem="${model}--${corpus_name}--b${batch}--${track}"
  output="$OUTPUT_DIR/$stem.jsonl"
  done_file="$output.done"
  signature="$RUN_VERSION|$SOURCE_COMMIT|$BINARY_SHA256|$sha|$corpus_spec|$batch|$track|$ROUNDS|$SAMPLE_MIB|$HOST_LABEL|$HOST_FINGERPRINT|$CPU_SET|$THREADS|${QUICKTOK_LIBRARY_SHA256:-none}"
  if [[ -s $output && -s $done_file && $(<"$done_file") == "$signature" ]]; then
    return 0
  fi
  rm -f "$output" "$done_file" "$output.tmp" "$output.error" "$done_file.tmp"

  printf 'running %s on %s\n' "$stem" "$HOST_LABEL"
  if SNAPTOKENS_BENCH_RUN_ID="20260909-v${RUN_VERSION}-${HOST_LABEL}-${stem}" \
    SNAPTOKENS_BENCH_HOST="$HOST_LABEL" \
    SNAPTOKENS_HOST_FINGERPRINT="$HOST_FINGERPRINT" \
    SNAPTOKENS_TOKENIZER_SHA256="$sha" \
    SNAPTOKENS_CPU_SET="$CPU_SET" \
    SNAPTOKENS_SOURCE_COMMIT="$SOURCE_COMMIT" \
    SNAPTOKENS_BINARY_SHA256="$BINARY_SHA256" \
    SNAPTOKENS_BUILD_COMMAND="$BUILD_COMMAND" \
    SNAPTOKENS_RUSTC_VERSION="$RUSTC_VERSION" \
    SNAPTOKENS_BUILD_RUSTFLAGS="$BUILD_RUSTFLAGS" \
    SNAPTOKENS_CARGO_LOCK_SHA256="$CARGO_LOCK_SHA256" \
    RAYON_NUM_THREADS="$THREADS" \
    TOKENIZERS_PARALLELISM=true \
    taskset -c "$CPU_SET" "$BIN" \
      "$TOKENIZER_DIR/$model.json" \
      "$model" \
      "$corpus_spec" \
      "$batch" \
      "$ROUNDS" \
      "$SAMPLE_MIB" \
      false \
      "$track" \
      >"$output.tmp" 2>"$output.error"; then
    jq -s -e 'any(.[]; .kind == "coverage" and .implementation == "snaptokens" and .status == "exact_on_probes_and_all_timed_inputs")' \
      "$output.tmp" >/dev/null
    jq -s -e 'any(.[]; .kind == "measurement" and .implementation == "snaptokens")' \
      "$output.tmp" >/dev/null
    mv "$output.tmp" "$output"
    printf '%s\n' "$signature" >"$done_file.tmp"
    mv "$done_file.tmp" "$done_file"
    rm -f "$output.error"
  else
    rm -f "$output.tmp" "$done_file.tmp"
    printf 'failed %s; see %s.error\n' "$stem" "$output" >&2
    return 1
  fi
}

is_sweep_model() {
  local wanted=$1
  local model
  for model in "${SWEEP_MODELS[@]}"; do
    [[ $model == "$wanted" ]] && return 0
  done
  return 1
}

for entry in "${TOKENIZERS[@]}"; do
  model=${entry%%|*}
  verify_sha256 "$TOKENIZER_DIR/$model.json" "${entry#*|}" "$model tokenizer"
done
verify_sha256 \
  "$CORPUS_DIR/enwik8.txt" \
  2b49720ec4d78c3c9fabaee6e4179a5e997302b3a70029f30f2d582218c024a8 \
  enwik8
verify_sha256 \
  "$CORPUS_DIR/rust-code.txt" \
  ed855017307cfc188f89f5f8185ac00d93b4257dd2dbf348459dc93191b0fd15 \
  rust-code

# Optional QuickTok libraries and data must match the pinned external revision.
if [[ -n ${QUICKTOK_LIBRARY:-} || -n ${QUICKTOK_DATA_DIR:-} || -n ${QUICKTOK_SOURCE_DIR:-} ]]; then
  : "${QUICKTOK_LIBRARY:?set QUICKTOK_LIBRARY with QUICKTOK_DATA_DIR}"
  : "${QUICKTOK_DATA_DIR:?set QUICKTOK_DATA_DIR with QUICKTOK_LIBRARY}"
  : "${QUICKTOK_SOURCE_DIR:?set QUICKTOK_SOURCE_DIR with QUICKTOK_LIBRARY}"
  : "${QUICKTOK_BUILD_COMMAND:?record the exact QuickTok build command}"
  : "${QUICKTOK_CXX:?set QUICKTOK_CXX to the compiler used for QuickTok}"
  QUICKTOK_SOURCE_COMMIT=$(git -C "$QUICKTOK_SOURCE_DIR" rev-parse HEAD)
  if [[ $QUICKTOK_SOURCE_COMMIT != c69b29886ac508841c698487799d1904d0f488e8 ]] ||
    [[ -n $(git -C "$QUICKTOK_SOURCE_DIR" status --porcelain --untracked-files=no) ]]; then
    printf 'QuickTok source must be a clean checkout of c69b29886ac508841c698487799d1904d0f488e8\n' >&2
    exit 1
  fi
  QUICKTOK_LIBRARY_SHA256=$(sha256sum "$QUICKTOK_LIBRARY" | cut -d' ' -f1)
  QUICKTOK_CXX_VERSION=$("$QUICKTOK_CXX" --version | sed -n '1p')
  export QUICKTOK_LIBRARY_SHA256 QUICKTOK_CXX_VERSION
  verify_sha256 \
    "$QUICKTOK_DATA_DIR/qwen3.vocab" \
    6114087889f61af31f72abe80529bbd09826df4d396753a281a14bcc21abc2fa \
    'QuickTok qwen3.vocab'
  verify_sha256 \
    "$QUICKTOK_DATA_DIR/qwen3.special" \
    d3fccd273ee759d3d91c2a73458db4f5738d56e104f8cccfd21cb36f401b020a \
    'QuickTok qwen3.special'
  verify_sha256 \
    "$QUICKTOK_DATA_DIR/uniclass.bin" \
    6039f6f7bd056c0d211d02a907a8ed8f3317bb719fc495b9093f9cccca948809 \
    'QuickTok uniclass.bin'
  verify_sha256 \
    "$QUICKTOK_DATA_DIR/nfc.bin" \
    c5b8a5beee77e7dcea1a85aee9980d863a961d3ebd8aab0cca49fe4644eb4b72 \
    'QuickTok nfc.bin'
fi

for entry in "${TOKENIZERS[@]}"; do
  model=${entry%%|*}
  for batch in 1 32 512; do
    run_cell \
      "$model" \
      enwik8-140 \
      "file:enwik8-140:140:$CORPUS_DIR/enwik8.txt" \
      "$batch" \
      novel
  done
  run_cell \
    "$model" \
    rust-code-4096 \
    "file:rust-code-4096:4096:$CORPUS_DIR/rust-code.txt" \
    1 \
    novel
  run_cell \
    "$model" \
    enwik8-65536 \
    "file:enwik8-65536:65536:$CORPUS_DIR/enwik8.txt" \
    1 \
    novel

  if is_sweep_model "$model"; then
    for batch in 2 4 8 16 64 128 256 1024; do
      run_cell \
        "$model" \
        enwik8-140 \
        "file:enwik8-140:140:$CORPUS_DIR/enwik8.txt" \
        "$batch" \
        novel
    done
    for batch in 1 32 512; do
      run_cell \
        "$model" \
        repeated-chat140 \
        repeated-chat140 \
        "$batch" \
        warm-repeated
    done
  fi
done
