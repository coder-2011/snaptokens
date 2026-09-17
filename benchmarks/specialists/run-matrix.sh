#!/usr/bin/env bash

set -euo pipefail

: "${TOKENIZER_DIR:?set TOKENIZER_DIR}"
: "${CORPUS_DIR:?set CORPUS_DIR}"
: "${RANK_DIR:?set RANK_DIR}"
: "${OUTPUT_DIR:?set OUTPUT_DIR}"
: "${HOST_LABEL:?set HOST_LABEL}"
: "${CPU_SET:?set CPU_SET}"
: "${THREADS:?set THREADS}"
: "${BUILD_COMMAND:?set BUILD_COMMAND to the exact release build command, including SNAPTOKENS_BUILD_SOURCE_COMMIT and CXX}"
: "${BUILD_RUSTFLAGS:?set BUILD_RUSTFLAGS used for the release build}"

HARNESS_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$HARNESS_DIR/../.." && pwd)
BIN=${BIN:-$HARNESS_DIR/target/release/snaptokens-specialists}
ROUNDS=${ROUNDS:-14}
SAMPLE_MIB=${SAMPLE_MIB:-4}
RUN_VERSION=7

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
PCRE2_LIBRARY_PATH=$(LC_ALL=C ldd "$BIN" | awk '/libpcre2-8/ { print $3; exit }')
if [[ -z $PCRE2_LIBRARY_PATH || ! -e $PCRE2_LIBRARY_PATH ]]; then
  printf 'could not resolve the PCRE2-8 runtime loaded by %s\n' "$BIN" >&2
  exit 1
fi
PCRE2_LIBRARY_PATH=$(readlink -f "$PCRE2_LIBRARY_PATH")
PCRE2_LIBRARY_SHA256=$(sha256sum "$PCRE2_LIBRARY_PATH" | cut -d' ' -f1)

if [[ -n $(git -C "$REPO_ROOT" status --porcelain --untracked-files=all) ]]; then
  printf 'benchmark source is not committed and clean; commit it before measuring\n' >&2
  exit 1
fi

AFFINITY_CPUS=$(taskset -c "$CPU_SET" nproc)
if [[ $THREADS != "$AFFINITY_CPUS" ]]; then
  printf 'THREADS=%s differs from %s CPUs in CPU_SET=%s\n' "$THREADS" "$AFFINITY_CPUS" "$CPU_SET" >&2
  exit 1
fi

mkdir -p "$RANK_DIR" "$OUTPUT_DIR"

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

fetch_rank() {
  local path=$1
  local url=$2
  local sha=$3
  if [[ ! -e $path ]]; then
    curl --fail --location --retry 3 --proto '=https' "$url" -o "$path.tmp"
    verify_sha256 "$path.tmp" "$sha" "$path"
    mv "$path.tmp" "$path"
  fi
  verify_sha256 "$path" "$sha" "$path"
}

tokenizer_sha() {
  case $1 in
    gpt-2)
      printf '%s\n' 8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6
      ;;
    gpt-oss)
      printf '%s\n' 0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3
      ;;
    *) return 1 ;;
  esac
}

rank_path() {
  case $1 in
    gpt-2) printf '%s\n' "$RANK_DIR/r50k_base.tiktoken" ;;
    gpt-oss) printf '%s\n' "$RANK_DIR/o200k_base.tiktoken" ;;
    *) return 1 ;;
  esac
}

run_cell() {
  local model=$1
  local corpus_label=$2
  local corpus_path=$3
  local input_bytes=$4
  local batch=$5
  local tokenizer
  local ranks
  local stem
  local output
  local done_file
  local signature

  tokenizer="$TOKENIZER_DIR/$model.json"
  ranks=$(rank_path "$model")
  stem="${model}--${corpus_label}--b${batch}"
  output="$OUTPUT_DIR/$stem.jsonl"
  done_file="$output.done"
  signature="$RUN_VERSION|$SOURCE_COMMIT|$BINARY_SHA256|$CARGO_LOCK_SHA256|$(tokenizer_sha "$model")|$corpus_label|$input_bytes|$batch|$ROUNDS|$SAMPLE_MIB|$HOST_LABEL|$HOST_FINGERPRINT|$CPU_SET|$THREADS|$PCRE2_LIBRARY_PATH|$PCRE2_LIBRARY_SHA256"
  if [[ -s $output && -s $done_file && $(<"$done_file") == "$signature" ]]; then
    return 0
  fi
  rm -f "$output" "$done_file" "$output.tmp" "$output.error" "$done_file.tmp"

  printf 'running %s on %s\n' "$stem" "$HOST_LABEL"
  if SNAPTOKENS_BENCH_RUN_ID="20260909-specialists-v${RUN_VERSION}-${HOST_LABEL}-${stem}" \
    SNAPTOKENS_BENCH_HOST="$HOST_LABEL" \
    SNAPTOKENS_HOST_FINGERPRINT="$HOST_FINGERPRINT" \
    SNAPTOKENS_CPU_SET="$CPU_SET" \
    SNAPTOKENS_SOURCE_COMMIT="$SOURCE_COMMIT" \
    SNAPTOKENS_BINARY_SHA256="$BINARY_SHA256" \
    SNAPTOKENS_BUILD_COMMAND="$BUILD_COMMAND" \
    SNAPTOKENS_RUSTC_VERSION="$RUSTC_VERSION" \
    SNAPTOKENS_BUILD_RUSTFLAGS="$BUILD_RUSTFLAGS" \
    SNAPTOKENS_CARGO_LOCK_SHA256="$CARGO_LOCK_SHA256" \
    SNAPTOKENS_PCRE2_LIBRARY_PATH="$PCRE2_LIBRARY_PATH" \
    SNAPTOKENS_PCRE2_LIBRARY_SHA256="$PCRE2_LIBRARY_SHA256" \
    RAYON_NUM_THREADS="$THREADS" \
    taskset -c "$CPU_SET" "$BIN" \
    "$model" \
    "$tokenizer" \
    "$ranks" \
    "$corpus_label" \
    "$corpus_path" \
    "$input_bytes" \
    "$batch" \
    "$ROUNDS" \
    "$SAMPLE_MIB" \
    >"$output.tmp" 2>"$output.error"; then
    jq -s -e 'any(.[]; .kind == "coverage" and .implementation == "snaptokens" and .status == "exact_on_model_probes_and_all_timed_inputs")' \
      "$output.tmp" >/dev/null
    jq -s -e 'any(.[]; .kind == "coverage" and .implementation != "snaptokens" and .status == "exact_on_model_probes_and_all_timed_inputs")' \
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

fetch_rank \
  "$RANK_DIR/r50k_base.tiktoken" \
  https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken \
  306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930
fetch_rank \
  "$RANK_DIR/o200k_base.tiktoken" \
  https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken \
  446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d

for model in gpt-2 gpt-oss; do
  verify_sha256 "$TOKENIZER_DIR/$model.json" "$(tokenizer_sha "$model")" "$model tokenizer"
done
verify_sha256 \
  "$CORPUS_DIR/enwik8.txt" \
  2b49720ec4d78c3c9fabaee6e4179a5e997302b3a70029f30f2d582218c024a8 \
  enwik8
verify_sha256 \
  "$CORPUS_DIR/rust-code.txt" \
  ed855017307cfc188f89f5f8185ac00d93b4257dd2dbf348459dc93191b0fd15 \
  rust-code

for model in gpt-2 gpt-oss; do
  for batch in 1 32 512; do
    run_cell "$model" enwik8-140 "$CORPUS_DIR/enwik8.txt" 140 "$batch"
  done
  run_cell "$model" rust-code-4096 "$CORPUS_DIR/rust-code.txt" 4096 1
  run_cell "$model" enwik8-65536 "$CORPUS_DIR/enwik8.txt" 65536 1
done
