#!/usr/bin/env bash
# Validate a local tokenizer JSON by running simple_bench against both datasets.
#
# Usage:
#   ./benchmarks/tools/validate_model.sh /path/to/tokenizer.json
#   ./benchmarks/tools/validate_model.sh /path/to/tokenizer.json -n 50

set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 <tokenizer.json> [extra args...]"
  echo "Example: $0 /path/to/tokenizer.json"
  echo "         $0 /path/to/tokenizer.json -n 50"
  exit 1
fi

TOKENIZER_JSON="$1"
shift
EXTRA_ARGS=("$@")

DATASETS=("RyokoAI/ShareGPT52K" "zai-org/LongBench-v2")

echo "=== Validating tokenizer: $TOKENIZER_JSON ==="
echo ""

FAILED=0
for DATASET in "${DATASETS[@]}"; do
  echo "--- $DATASET ---"
  if cargo bench --manifest-path benchmarks/tools/Cargo.toml --bench simple_bench -- "$TOKENIZER_JSON" --dataset "$DATASET" ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"}; then
    echo "PASS: $TOKENIZER_JSON on $DATASET"
  else
    echo "FAIL: $TOKENIZER_JSON on $DATASET"
    FAILED=1
  fi
  echo ""
done

if [ "$FAILED" -eq 0 ]; then
  echo "=== All datasets passed for $TOKENIZER_JSON ==="
else
  echo "=== FAILURES detected for $TOKENIZER_JSON ==="
  exit 1
fi
