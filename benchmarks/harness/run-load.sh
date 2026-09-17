#!/usr/bin/env bash

set -euo pipefail

: "${TOKENIZER_DIR:?set TOKENIZER_DIR}"
: "${TOKIE_DIR:?set TOKIE_DIR}"
: "${KITOKEN_DIR:?set KITOKEN_DIR}"
: "${OUTPUT_FILE:?set OUTPUT_FILE}"
: "${HOST_LABEL:?set HOST_LABEL}"
: "${CACHE_STATE:?set CACHE_STATE}"
: "${CPU_SET:?set CPU_SET}"
: "${BUILD_COMMAND:?set BUILD_COMMAND to the exact release build command}"
: "${BUILD_RUSTFLAGS:?set BUILD_RUSTFLAGS used for the release build}"

HARNESS_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$HARNESS_DIR/../.." && pwd)
BIN=${BIN:-$HARNESS_DIR/target/release/load}
ROUNDS=${ROUNDS:-20}
REUSE_ARTIFACTS=${REUSE_ARTIFACTS:-false}
PROMPT=${PROMPT:-'Tokenization should not be the bottleneck. café 東京 👩🏽‍💻'}
RUN_VERSION=4
SCHEDULE_VERSION=williams-v1

case $REUSE_ARTIFACTS in
  true) ARTIFACT_POLICY=reuse-frozen ;;
  false) ARTIFACT_POLICY=prepare-per-run ;;
  *)
    printf 'REUSE_ARTIFACTS must be true or false\n' >&2
    exit 1
    ;;
esac

SOURCE_COMMIT=$(git -C "$REPO_ROOT" rev-parse HEAD)
BINARY_SHA256=$(sha256sum "$BIN" | cut -d' ' -f1)
RUSTC_VERSION=$(rustc --version)
CARGO_LOCK_SHA256=$(sha256sum "$HARNESS_DIR/Cargo.lock" | cut -d' ' -f1)
PROMPT_SHA256=$(printf '%s' "$PROMPT" | sha256sum | cut -d' ' -f1)
PROMPT_BYTES=$(printf '%s' "$PROMPT" | wc -c | tr -d ' ')
CREATE_DIR="$TOKIE_DIR/snaptokens-create"
COVERAGE_FILE="$OUTPUT_FILE.coverage.tmp"
MEASUREMENTS_FILE="$OUTPUT_FILE.measurements.tmp"

if [[ -n $(git -C "$REPO_ROOT" status --porcelain --untracked-files=all -- benchmarks/harness) ]] ||
  ! git -C "$REPO_ROOT" diff --quiet ||
  ! git -C "$REPO_ROOT" diff --cached --quiet; then
  printf 'benchmark source is not committed and clean; commit it before measuring\n' >&2
  exit 1
fi

MODELS=(gpt-2 qwen-3 deepseek-r1 gpt-oss mistral-nemo)
MODES=(
  snaptokens-json
  snaptokens-json-create
  snaptokens-json-sidecar
  snaptokens-tkz-direct
  fastokens-json
  huggingface-json
  kitoken-json
  kitoken-kit
  tokie-json
  tokie-tkz
  splintr-json
)

mkdir -p "$TOKIE_DIR" "$KITOKEN_DIR" "$CREATE_DIR" "$(dirname "$OUTPUT_FILE")"

file_sha256() {
  sha256sum "$1" | cut -d' ' -f1
}

file_identity() {
  stat -c '%d:%i:%s:%y' "$1"
}

verify_sha256() {
  local path=$1
  local expected=$2
  local label=$3
  local actual
  actual=$(file_sha256 "$path")
  if [[ $actual != "$expected" ]]; then
    printf '%s SHA-256 mismatch: expected %s, got %s\n' "$label" "$expected" "$actual" >&2
    return 1
  fi
}

tokenizer_sha() {
  case $1 in
    deepseek-r1) printf '%s\n' ecb6f9fc369894346f0511f4074ca75cee5cd5f3b06d02f1ba35fcd39f8e121d ;;
    gpt-2) printf '%s\n' 8414cab924d8b9b33013f0d221c5862f365ee9be39c5c2bfae8a5a9e970478a6 ;;
    gpt-oss) printf '%s\n' 0614fe83cadab421296e664e1f48f4261fa8fef6e03e63bb75c20f38e37d07d3 ;;
    mistral-nemo) printf '%s\n' e11c71726323d33da7b8d6f6f269f1988931c0a52b7122bcdd8c05042974e0db ;;
    qwen-3) printf '%s\n' aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4 ;;
    *) return 1 ;;
  esac
}

mode_path() {
  local mode=$1
  local model=$2
  local repetition=${3:-shared}
  case $mode in
    snaptokens-json-create) printf '%s\n' "$CREATE_DIR/$model-$repetition.json" ;;
    snaptokens-tkz-direct) printf '%s\n' "$TOKENIZER_DIR/$model.tkz" ;;
    kitoken-kit) printf '%s\n' "$KITOKEN_DIR/$model.kit" ;;
    tokie-tkz) printf '%s\n' "$TOKIE_DIR/$model.tkz" ;;
    *) printf '%s\n' "$TOKENIZER_DIR/$model.json" ;;
  esac
}

mode_sidecar_path() {
  local mode=$1
  local path=$2
  case $mode in
    snaptokens-json-create | snaptokens-json-sidecar) printf '%s\n' "${path%.json}.tkz" ;;
    snaptokens-tkz-direct | kitoken-kit | tokie-tkz) printf '%s\n' "$path" ;;
    *) return 1 ;;
  esac
}

prepare_artifacts() {
  local model=$1
  if [[ $REUSE_ARTIFACTS == true ]]; then
    if [[ ! -f $TOKENIZER_DIR/$model.tkz ||
      ! -f $KITOKEN_DIR/$model.kit ||
      ! -f $TOKIE_DIR/$model.tkz ]]; then
      printf 'missing frozen artifact for %s\n' "$model" >&2
      return 1
    fi
    return
  fi
  rm -f "$TOKENIZER_DIR/$model.tkz" "$KITOKEN_DIR/$model.kit" "$TOKIE_DIR/$model.tkz"
  "$BIN" snaptokens-create "$TOKENIZER_DIR/$model.json" ignored
  "$BIN" kitoken-create "$TOKENIZER_DIR/$model.json" "$KITOKEN_DIR/$model.kit"
  "$BIN" tokie-create "$TOKENIZER_DIR/$model.json" "$TOKIE_DIR/$model.tkz"
  [[ -f $TOKENIZER_DIR/$model.tkz && -f $KITOKEN_DIR/$model.kit && -f $TOKIE_DIR/$model.tkz ]]
}

build_probe_manifest() {
  local model=$1
  local output=$2
  jq -cn \
    --arg prompt "$PROMPT" \
    --slurpfile tokenizer "$TOKENIZER_DIR/$model.json" \
    '
      def variants($token): [
        $token,
        (" " + $token),
        ($token + " "),
        ("  " + $token + "  "),
        ("x" + $token + "y"),
        ("prefix " + $token + " suffix"),
        ("\t" + $token + "\n"),
        ($token + $token)
      ];
      ([
        "",
        "hello",
        " hello",
        "  hello  ",
        "I am cant cannot I\u0027m can\u0027t we\u0027ve they\u0027ll",
        "foo_bar::baz... 12345\n",
        "\t\r\n  \u00a0",
        "\u0000\u0001\u001f\u007f",
        "café",
        "cafe\u0301",
        "東京 日本語",
        "العربية لغة",
        "हिन्दी भाषा",
        "ไทย ภาษา",
        "👩🏽‍💻 🧪",
        "ÿ ࠀ ퟿ ",
        $prompt
      ] + [
        ($tokenizer[0].added_tokens // [])[]
        | .content
        | select(type == "string" and length > 0)
        | variants(.)[]
      ]) | unique
    ' >"$output"
}

warm_pages() {
  local mode=$1
  local model=$2
  local path=$3
  dd if="$path" of=/dev/null bs=1M status=none
  if [[ $mode == snaptokens-json-sidecar ]]; then
    dd if="$TOKENIZER_DIR/$model.tkz" of=/dev/null bs=1M status=none
  fi
}

prepare_cache_state() {
  local mode=$1
  local model=$2
  local path=$3
  case $CACHE_STATE in
    warm-page) warm_pages "$mode" "$model" "$path" ;;
    guest-page-cache-cold)
      sync
      printf '3\n' >/proc/sys/vm/drop_caches
      ;;
    *)
      printf 'unknown cache state: %s\n' "$CACHE_STATE" >&2
      return 1
      ;;
  esac
}

assert_unchanged() {
  local path=$1
  local expected_identity=$2
  local expected_sha=$3
  local label=$4
  if [[ $(file_identity "$path") != "$expected_identity" ]] ||
    [[ $(file_sha256 "$path") != "$expected_sha" ]]; then
    printf '%s changed during a supposed cache hit: %s\n' "$label" "$path" >&2
    return 1
  fi
}

sample() {
  local mode=$1
  local model=$2
  local repetition=$3
  local position=$4
  local sha=$5
  local manifest_sha=$6
  local probe_count=$7
  local actual_rounds=$8
  local cycle_length=$9
  local manifest=${10}
  local reference_hash=${11}
  local path
  local artifact_identity
  local artifact_sha
  local sidecar_path=
  local sidecar_identity=
  local sidecar_sha=
  local output
  local direct_output
  local direct_hash
  local status

  path=$(mode_path "$mode" "$model" "$repetition")
  if [[ $mode == snaptokens-json-create ]]; then
    cp "$TOKENIZER_DIR/$model.json" "$path"
    rm -f "${path%.json}.tkz"
  fi
  artifact_identity=$(file_identity "$path")
  artifact_sha=$(file_sha256 "$path")
  if sidecar_path=$(mode_sidecar_path "$mode" "$path") && [[ -f $sidecar_path ]]; then
    sidecar_identity=$(file_identity "$sidecar_path")
    sidecar_sha=$(file_sha256 "$sidecar_path")
  fi
  prepare_cache_state "$mode" "$model" "$path" || return 1
  if output=$(SNAPTOKENS_BENCH_RUN_ID="20260716-v${RUN_VERSION}-${HOST_LABEL}-load-${CACHE_STATE}" \
    SNAPTOKENS_BENCH_HOST="$HOST_LABEL" \
    SNAPTOKENS_MODEL="$model" \
    SNAPTOKENS_TOKENIZER_SHA256="$sha" \
    SNAPTOKENS_CACHE_STATE="$CACHE_STATE" \
    SNAPTOKENS_REPETITION="$repetition" \
    SNAPTOKENS_ORDER="$position" \
    SNAPTOKENS_CPU_SET="$CPU_SET" \
    SNAPTOKENS_SOURCE_COMMIT="$SOURCE_COMMIT" \
    SNAPTOKENS_BINARY_SHA256="$BINARY_SHA256" \
    SNAPTOKENS_BUILD_COMMAND="$BUILD_COMMAND" \
    SNAPTOKENS_RUSTC_VERSION="$RUSTC_VERSION" \
    SNAPTOKENS_BUILD_RUSTFLAGS="$BUILD_RUSTFLAGS" \
    SNAPTOKENS_CARGO_LOCK_SHA256="$CARGO_LOCK_SHA256" \
    SNAPTOKENS_ARTIFACT_SHA256="$artifact_sha" \
    SNAPTOKENS_ARTIFACT_POLICY="$ARTIFACT_POLICY" \
    SNAPTOKENS_SIDECAR_SHA256="$sidecar_sha" \
    SNAPTOKENS_PROMPT_SHA256="$PROMPT_SHA256" \
    SNAPTOKENS_PROMPT_BYTES="$PROMPT_BYTES" \
    SNAPTOKENS_PROBE_MANIFEST_SHA256="$manifest_sha" \
    SNAPTOKENS_PROBE_COUNT="$probe_count" \
    SNAPTOKENS_REQUESTED_ROUNDS="$ROUNDS" \
    SNAPTOKENS_ACTUAL_ROUNDS="$actual_rounds" \
    SNAPTOKENS_CYCLE_LENGTH="$cycle_length" \
    SNAPTOKENS_SCHEDULE_VERSION="$SCHEDULE_VERSION" \
    taskset -c "$CPU_SET" "$BIN" "$mode" "$path" "$PROMPT"); then
    status=0
  else
    status=$?
  fi
  if ((status != 0)); then
    [[ $mode != snaptokens-json-create ]] || rm -f "$path" "${path%.json}.tkz"
    return "$status"
  fi
  if ! assert_unchanged "$path" "$artifact_identity" "$artifact_sha" "$mode input"; then
    [[ $mode != snaptokens-json-create ]] || rm -f "$path" "${path%.json}.tkz"
    return 1
  fi

  if [[ $mode == snaptokens-json-create ]]; then
    sidecar_path="${path%.json}.tkz"
    if ! direct_output=$(taskset -c "$CPU_SET" "$BIN" \
      parity snaptokens-tkz-direct "$sidecar_path" "$manifest"); then
      rm -f "$path" "$sidecar_path"
      return 1
    fi
    direct_hash=$(jq -cj .ids <<<"$direct_output" | file_sha256 /dev/stdin)
    if [[ $direct_hash != "$reference_hash" ]]; then
      printf 'created TKZ differs from Hugging Face on the parity manifest: %s\n' \
        "$sidecar_path" >&2
      rm -f "$path" "$sidecar_path"
      return 1
    fi
    sidecar_sha=$(file_sha256 "$sidecar_path")
  elif [[ -n $sidecar_path && $sidecar_path != "$path" ]]; then
    assert_unchanged \
      "$sidecar_path" "$sidecar_identity" "$sidecar_sha" "$mode sidecar" || return 1
  fi

  [[ $mode != snaptokens-json-create ]] || rm -f "$path" "$sidecar_path"
  jq -c \
    --arg sidecar_sha256 "$sidecar_sha" \
    '.sidecar_sha256 = (if $sidecar_sha256 == "" then null else $sidecar_sha256 end)' \
    <<<"$output"
}

parity_sample() {
  local mode=$1
  local model=$2
  local manifest=$3
  local path
  local artifact_identity
  local artifact_sha
  local sidecar_path=
  local sidecar_identity=
  local sidecar_sha=
  local output
  local output_ids
  local direct_output
  local direct_ids
  local created_sidecar_sha=

  path=$(mode_path "$mode" "$model" parity)
  if [[ $mode == snaptokens-json-create ]]; then
    cp "$TOKENIZER_DIR/$model.json" "$path"
    rm -f "${path%.json}.tkz"
  fi
  artifact_identity=$(file_identity "$path")
  artifact_sha=$(file_sha256 "$path")
  if sidecar_path=$(mode_sidecar_path "$mode" "$path") && [[ -f $sidecar_path ]]; then
    sidecar_identity=$(file_identity "$sidecar_path")
    sidecar_sha=$(file_sha256 "$sidecar_path")
  fi

  if ! output=$(taskset -c "$CPU_SET" "$BIN" parity "$mode" "$path" "$manifest"); then
    [[ $mode != snaptokens-json-create ]] || rm -f "$path" "${path%.json}.tkz"
    return 1
  fi
  if ! assert_unchanged "$path" "$artifact_identity" "$artifact_sha" "$mode parity input"; then
    [[ $mode != snaptokens-json-create ]] || rm -f "$path" "${path%.json}.tkz"
    return 1
  fi

  if [[ $mode == snaptokens-json-create ]]; then
    sidecar_path="${path%.json}.tkz"
    if ! direct_output=$(taskset -c "$CPU_SET" "$BIN" \
      parity snaptokens-tkz-direct "$sidecar_path" "$manifest"); then
      rm -f "$path" "$sidecar_path"
      return 1
    fi
    direct_ids=$(jq -c .ids <<<"$direct_output")
    output_ids=$(jq -c .ids <<<"$output")
    if [[ $direct_ids != "$output_ids" ]]; then
      printf 'created TKZ parity differs from its JSON-created tokenizer: %s\n' "$sidecar_path" >&2
      rm -f "$path" "$sidecar_path"
      return 1
    fi
    created_sidecar_sha=$(file_sha256 "$sidecar_path")
    output=$(jq -c --arg sha "$created_sidecar_sha" \
      '. + {sidecar_sha256: $sha}' <<<"$output")
    rm -f "$path" "$sidecar_path"
  elif [[ -n $sidecar_path && $sidecar_path != "$path" ]]; then
    assert_unchanged \
      "$sidecar_path" "$sidecar_identity" "$sidecar_sha" "$mode parity sidecar" || return 1
  fi
  printf '%s\n' "$output"
}

emit_load_coverage() {
  local model=$1
  local sha=$2
  local mode=$3
  local status=$4
  local expected_hash=$5
  local actual_hash=$6
  local error=$7
  local manifest_sha=$8
  local probe_count=$9
  local artifact_sha=${10}
  local sidecar_sha=${11}
  jq -cn \
    --arg host "$HOST_LABEL" \
    --arg model "$model" \
    --arg tokenizer_sha256 "$sha" \
    --arg implementation "$mode" \
    --arg cache_state "$CACHE_STATE" \
    --arg cpu_set "$CPU_SET" \
    --arg source_commit "$SOURCE_COMMIT" \
    --arg binary_sha256 "$BINARY_SHA256" \
    --arg cargo_lock_sha256 "$CARGO_LOCK_SHA256" \
    --arg artifact_sha256 "$artifact_sha" \
    --arg artifact_policy "$ARTIFACT_POLICY" \
    --arg sidecar_sha256 "$sidecar_sha" \
    --arg prompt_sha256 "$PROMPT_SHA256" \
    --argjson prompt_bytes "$PROMPT_BYTES" \
    --arg probe_manifest_sha256 "$manifest_sha" \
    --argjson probe_count "$probe_count" \
    --argjson requested_rounds "$ROUNDS" \
    --arg schedule_version "$SCHEDULE_VERSION" \
    --arg status "$status" \
    --arg expected_ids_sha256 "$expected_hash" \
    --arg actual_ids_sha256 "$actual_hash" \
    --arg error "$error" \
    '{kind:"load_coverage",host:$host,model:$model,tokenizer_sha256:$tokenizer_sha256,implementation:$implementation,cache_state:$cache_state,cpu_set:$cpu_set,source_commit:$source_commit,binary_sha256:$binary_sha256,cargo_lock_sha256:$cargo_lock_sha256,artifact_sha256:$artifact_sha256,artifact_policy:$artifact_policy,sidecar_sha256:(if $sidecar_sha256 == "" then null else $sidecar_sha256 end),prompt_sha256:$prompt_sha256,prompt_bytes:$prompt_bytes,probe_manifest_sha256:$probe_manifest_sha256,probe_count:$probe_count,requested_rounds:$requested_rounds,schedule_version:$schedule_version,status:$status,expected_ids_sha256:$expected_ids_sha256,actual_ids_sha256:$actual_ids_sha256,error:$error}' \
    >>"$COVERAGE_FILE"
}

supported_modes() {
  local model=$1
  local sha=$2
  local manifest=$3
  local manifest_sha=$4
  local probe_count=$5
  local reference_output
  local reference
  local reference_hash
  local mode
  local output
  local ids
  local ids_hash
  local artifact_path
  local artifact_sha
  local sidecar_sha
  local error_file
  local error

  reference_output=$(parity_sample huggingface-json "$model" "$manifest") || return 1
  jq -e --argjson count "$probe_count" '.probe_count == $count' \
    <<<"$reference_output" >/dev/null
  reference=$(jq -c .ids <<<"$reference_output")
  reference_hash=$(printf '%s' "$reference" | file_sha256 /dev/stdin)
  PARITY_REFERENCE_HASH=$reference_hash
  for mode in "${MODES[@]}"; do
    error_file="$OUTPUT_FILE.$model.$mode.probe.error"
    if [[ $mode == huggingface-json ]]; then
      output=$reference_output
    elif ! output=$(parity_sample "$mode" "$model" "$manifest" 2>"$error_file"); then
      error=$(<"$error_file")
      artifact_path=$(mode_path "$mode" "$model" shared)
      [[ $mode != snaptokens-json-create ]] || artifact_path="$TOKENIZER_DIR/$model.json"
      artifact_sha=$(file_sha256 "$artifact_path")
      sidecar_sha=
      case $mode in
        snaptokens-json-sidecar | snaptokens-tkz-direct)
          sidecar_sha=$(file_sha256 "$TOKENIZER_DIR/$model.tkz")
          ;;
        kitoken-kit) sidecar_sha=$(file_sha256 "$KITOKEN_DIR/$model.kit") ;;
        tokie-tkz) sidecar_sha=$(file_sha256 "$TOKIE_DIR/$model.tkz") ;;
      esac
      emit_load_coverage "$model" "$sha" "$mode" error "$reference_hash" "" "$error" \
        "$manifest_sha" "$probe_count" "$artifact_sha" "$sidecar_sha"
      rm -f "$error_file"
      continue
    fi
    rm -f "$error_file"
    jq -e --argjson count "$probe_count" '.probe_count == $count' <<<"$output" >/dev/null
    ids=$(jq -c .ids <<<"$output")
    ids_hash=$(printf '%s' "$ids" | file_sha256 /dev/stdin)
    artifact_path=$(mode_path "$mode" "$model" shared)
    [[ $mode != snaptokens-json-create ]] || artifact_path="$TOKENIZER_DIR/$model.json"
    artifact_sha=$(file_sha256 "$artifact_path")
    sidecar_sha=
    case $mode in
      snaptokens-json-create)
        sidecar_sha=$(jq -r '.sidecar_sha256 // empty' <<<"$output")
        ;;
      snaptokens-json-sidecar | snaptokens-tkz-direct)
        sidecar_sha=$(file_sha256 "$TOKENIZER_DIR/$model.tkz")
        ;;
      kitoken-kit) sidecar_sha=$(file_sha256 "$KITOKEN_DIR/$model.kit") ;;
      tokie-tkz) sidecar_sha=$(file_sha256 "$TOKIE_DIR/$model.tkz") ;;
    esac
    if [[ $ids == "$reference" ]]; then
      emit_load_coverage "$model" "$sha" "$mode" exact_on_manifest \
        "$reference_hash" "$ids_hash" "" "$manifest_sha" "$probe_count" \
        "$artifact_sha" "$sidecar_sha"
      printf '%s\n' "$mode"
    else
      emit_load_coverage "$model" "$sha" "$mode" manifest_mismatch \
        "$reference_hash" "$ids_hash" "IDs differ" "$manifest_sha" "$probe_count" \
        "$artifact_sha" "$sidecar_sha"
    fi
  done
}

williams_base() {
  local count=$1
  local position
  for ((position = 0; position < count; position++)); do
    if ((position == 0)); then
      printf '0\n'
    elif ((position % 2 == 1)); then
      printf '%s\n' "$(((position + 1) / 2))"
    else
      printf '%s\n' "$((count - position / 2))"
    fi
  done
}

run_balanced() {
  local model=$1
  local sha=$2
  local manifest_sha=$3
  local probe_count=$4
  local manifest=$5
  local reference_hash=$6
  shift 6
  local modes=("$@")
  local count=${#modes[@]}
  local base=()
  local cycle
  local actual_rounds
  local round
  local within
  local shift_index
  local reverse
  local position
  local base_position
  local index

  mapfile -t base < <(williams_base "$count")
  cycle=$count
  ((count > 1 && count % 2 == 1)) && cycle=$((2 * count))
  actual_rounds=$((((ROUNDS + cycle - 1) / cycle) * cycle))
  for ((round = 0; round < actual_rounds; round++)); do
    within=$((round % cycle))
    shift_index=$((within % count))
    reverse=0
    ((count % 2 == 1 && within >= count)) && reverse=1
    for ((position = 0; position < count; position++)); do
      base_position=$position
      ((reverse == 1)) && base_position=$((count - 1 - position))
      index=$(((base[base_position] + shift_index) % count))
      sample "${modes[index]}" "$model" "$round" "$position" "$sha" \
        "$manifest_sha" "$probe_count" "$actual_rounds" "$cycle" \
        "$manifest" "$reference_hash" \
        >>"$MEASUREMENTS_FILE" || return 1
    done
  done
}

rm -f "$OUTPUT_FILE.tmp" "$COVERAGE_FILE" "$MEASUREMENTS_FILE"
touch "$COVERAGE_FILE" "$MEASUREMENTS_FILE"
for model in "${MODELS[@]}"; do
  sha=$(tokenizer_sha "$model") || exit 1
  verify_sha256 "$TOKENIZER_DIR/$model.json" "$sha" "$model tokenizer"
done
for model in "${MODELS[@]}"; do
  sha=$(tokenizer_sha "$model") || exit 1
  manifest="$OUTPUT_FILE.$model.probes.json"
  build_probe_manifest "$model" "$manifest"
  manifest_sha=$(file_sha256 "$manifest")
  probe_count=$(jq -r length "$manifest")
  ((probe_count > 0)) || exit 1
  prepare_artifacts "$model" || exit 1
  modes_file="$OUTPUT_FILE.$model.modes.tmp"
  PARITY_REFERENCE_HASH=
  supported_modes "$model" "$sha" "$manifest" "$manifest_sha" "$probe_count" >"$modes_file"
  mapfile -t exact_modes <"$modes_file"
  rm -f "$modes_file"
  if ((${#exact_modes[@]} == 0)); then
    printf 'no exact load modes for %s\n' "$model" >&2
    continue
  fi
  printf 'loading %s with %s exact modes on %s\n' "$model" "${#exact_modes[@]}" "$HOST_LABEL"
  [[ -n $PARITY_REFERENCE_HASH ]] || exit 1
  run_balanced "$model" "$sha" "$manifest_sha" "$probe_count" \
    "$manifest" "$PARITY_REFERENCE_HASH" \
    "${exact_modes[@]}" || exit 1
done
cat "$COVERAGE_FILE" "$MEASUREMENTS_FILE" >"$OUTPUT_FILE.tmp"
mv "$OUTPUT_FILE.tmp" "$OUTPUT_FILE"
rm -f "$COVERAGE_FILE" "$MEASUREMENTS_FILE"
