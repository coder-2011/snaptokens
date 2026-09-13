#!/usr/bin/env bash

set -euo pipefail

: "${TOKENIZER_DIR:?set TOKENIZER_DIR}"
: "${CORPUS_DIR:?set CORPUS_DIR}"
: "${OUTPUT_DIR:?set OUTPUT_DIR}"
: "${HOST_LABEL:?set HOST_LABEL}"
: "${CPU_SET:?set CPU_SET}"
: "${THREADS:?set THREADS}"
: "${BUILD_COMMAND:?set BUILD_COMMAND to the exact release build command, including SNAPTOKENS_BUILD_SOURCE_COMMIT}"
: "${BUILD_RUSTFLAGS:?set BUILD_RUSTFLAGS used for the release build}"

HARNESS_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$HARNESS_DIR/../.." && pwd)
BIN=${BIN:-$HARNESS_DIR/target/release/snaptokens-long-single}
RUN_VERSION=2
REQUESTED_ROUNDS=4
HF_REVISION=b62132e4e0ec7518caba201408a680819dfdcd22
CXUU_REVISION=1c8b302cfbde3b5f4b78476b0dec7bf37d04cbb6
FASTOKENS_REVISION=326cb5afc5a033d2f7885832d12fd43b9ea50cdd
CORPUS_SHA256=2b49720ec4d78c3c9fabaee6e4179a5e997302b3a70029f30f2d582218c024a8

# cxuu treats presence, including DEBUG_PARALLEL=0, as a request to print.
unset DEBUG_PARALLEL

SOURCE_COMMIT=$(git -C "$REPO_ROOT" rev-parse HEAD)
BINARY_SHA256=$(sha256sum "$BIN" | cut -d' ' -f1)
CARGO_LOCK_SHA256=$(sha256sum "$HARNESS_DIR/Cargo.lock" | cut -d' ' -f1)
RUSTC_VERSION=$(rustc --version)
# The fingerprint prevents resuming cells after silent CPU, topology, or kernel drift.
HOST_FINGERPRINT=$(
  {
    LC_ALL=C uname -srvm
    LC_ALL=C lscpu -p=CPU,CORE,SOCKET,NODE,ONLINE
    LC_ALL=C awk -F: '/^(model name|cpu family|model|stepping|microcode)[[:space:]]*:/ { gsub(/^[ \t]+|[ \t]+$/, "", $1); gsub(/^[ \t]+|[ \t]+$/, "", $2); print $1 ":" $2 }' /proc/cpuinfo | sort -u
  } | sha256sum | cut -d' ' -f1
)

# The path dependency makes every tracked repository edit part of the measured source.
if [[ -n $(git -C "$REPO_ROOT" status --porcelain --untracked-files=all) ]]; then
  printf 'benchmark source is not committed and clean; commit it before measuring\n' >&2
  exit 1
fi

# Rayon must receive exactly the CPUs declared by each benchmark process.
AFFINITY_CPUS=$(taskset -c "$CPU_SET" nproc)
if [[ $THREADS != "$AFFINITY_CPUS" ]]; then
  printf 'THREADS=%s differs from %s CPUs in CPU_SET=%s\n' "$THREADS" "$AFFINITY_CPUS" "$CPU_SET" >&2
  exit 1
fi

mkdir -p "$OUTPUT_DIR"

# Rejects any tokenizer or corpus byte drift before the first timed process.
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

# Returns the immutable tokenizer digest for one admitted model.
tokenizer_sha() {
  case $1 in
    gpt-2) printf '%s\n' 8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6 ;;
    gpt-oss) printf '%s\n' 0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3 ;;
    *) return 1 ;;
  esac
}

# Publishes a cell only after the binary's preflight and fresh-instance parity gates pass.
run_cell() {
  local model=$1
  local input_bytes=$2
  local tokenizer=$TOKENIZER_DIR/$model.json
  local corpus=$CORPUS_DIR/enwik8.txt
  local tokenizer_digest
  local stem=$model--enwik8--$input_bytes
  local output=$OUTPUT_DIR/$stem.jsonl
  local done_file=$output.done
  local signature

  tokenizer_digest=$(tokenizer_sha "$model")
  signature="$RUN_VERSION|$SOURCE_COMMIT|$BINARY_SHA256|$CARGO_LOCK_SHA256|$tokenizer_digest|$CORPUS_SHA256|$input_bytes|$REQUESTED_ROUNDS|$HOST_LABEL|$HOST_FINGERPRINT|$CPU_SET|$THREADS|$HF_REVISION|$CXUU_REVISION|$FASTOKENS_REVISION"
  if [[ -s $output && -s $done_file && $(<"$done_file") == "$signature" ]]; then
    return
  fi

  rm -f "$output.tmp" "$done_file.tmp"
  if SOURCE_COMMIT="$SOURCE_COMMIT" \
    BINARY_SHA256="$BINARY_SHA256" \
    CARGO_LOCK_SHA256="$CARGO_LOCK_SHA256" \
    RUSTC_VERSION="$RUSTC_VERSION" \
    BUILD_COMMAND="$BUILD_COMMAND" \
    BUILD_RUSTFLAGS="$BUILD_RUSTFLAGS" \
    HOST_LABEL="$HOST_LABEL" \
    HOST_FINGERPRINT="$HOST_FINGERPRINT" \
    CPU_SET="$CPU_SET" \
    THREADS="$THREADS" \
    RAYON_NUM_THREADS="$THREADS" \
    TOKENIZERS_PARALLELISM=true \
    SNAPTOKENS_TOKENIZER_SHA256="$tokenizer_digest" \
    SNAPTOKENS_CORPUS_SHA256="$CORPUS_SHA256" \
    taskset -c "$CPU_SET" "$BIN" \
      "$model" \
      "$tokenizer" \
      "$corpus" \
      "$input_bytes" \
      >"$output.tmp" \
      2>"$output.error"; then
    mv "$output.tmp" "$output"
    printf '%s\n' "$signature" >"$done_file.tmp"
    mv "$done_file.tmp" "$done_file"
    rm -f "$output.error"
  else
    rm -f "$output.tmp" "$done_file.tmp"
    printf 'failed %s; no rows published, see %s.error\n' "$stem" "$output" >&2
    return 1
  fi
}

verify_sha256 "$CORPUS_DIR/enwik8.txt" "$CORPUS_SHA256" enwik8
for model in gpt-2 gpt-oss; do
  verify_sha256 "$TOKENIZER_DIR/$model.json" "$(tokenizer_sha "$model")" "$model tokenizer"
done

# Each process is an isolated model/size cell with one complete Williams cycle.
for model in gpt-2 gpt-oss; do
  for input_bytes in 262144 1048576 4194304; do
    run_cell "$model" "$input_bytes"
  done
done
