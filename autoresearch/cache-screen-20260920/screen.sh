#!/bin/zsh
set -euo pipefail
cd /tmp/snaptokens-cache-screen-20260920
export CARGO_TARGET_DIR=/tmp/snaptokens-cache-target-20260920
export CARGO_PROFILE_RELEASE_DEBUG=1
export CARGO_BUILD_JOBS=2
unset RUSTFLAGS
for variant in "$@"; do
  python3 /tmp/snaptokens-cache-evidence-20260920/variant.py "$variant"
  cargo fmt --all -- --check
  if [[ "$variant" != atomic ]]; then
    cargo test --release --locked --lib models::bpe:: -- --test-threads=2 > /tmp/snaptokens-cache-evidence-20260920/raw/$variant-tests.log 2>&1
    cargo build --release --locked --manifest-path autoresearch/st-eval/Cargo.toml > /tmp/snaptokens-cache-evidence-20260920/raw/$variant-build.log 2>&1
    cp /tmp/snaptokens-cache-target-20260920/release/st-eval /tmp/snaptokens-cache-evidence-20260920/bin/$variant
  fi
  shasum -a 256 /tmp/snaptokens-cache-evidence-20260920/bin/$variant > /tmp/snaptokens-cache-evidence-20260920/raw/$variant-binary.sha256
  python3 autoresearch/cache-screen-20260920/run.py check "$variant" pre
  python3 autoresearch/cache-screen-20260920/run.py pairs parent "$variant" "$variant" 12
  python3 autoresearch/cache-screen-20260920/run.py check "$variant" post
  python3 autoresearch/cache-screen-20260920/run.py check parent "post-$variant"
done
