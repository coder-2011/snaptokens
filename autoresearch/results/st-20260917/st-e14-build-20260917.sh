#!/bin/bash
set -eu
root="$HOME/st-campaign-20260917"
test "$(cat "$root/evidence/combined-profile.status")" = 0
trap 'echo $? > "$root/evidence/e14-sequence.status"' EXIT
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$root/target"
export CARGO_PROFILE_RELEASE_DEBUG=1
git -C "$root/base" fetch "$HOME/st-e14.bundle" refs/heads/perf/st-trusted-arena-get
git -C "$root/base" worktree add --detach "$root/e14-source" 320b9615050f2ede54c4d6a8bf68bb39b505795b
cd "$root/e14-source"
test -z "$(git status --porcelain)"
cargo +1.98.1 fmt --all -- --check
rustup toolchain install nightly-2026-09-15 --profile minimal --component miri --component rust-src > "$root/evidence/e14-miri-install.log" 2>&1
cargo +nightly-2026-09-15 miri test --locked --lib arena_span_getters_preserve_utf8 -- --test-threads=1 > "$root/evidence/e14-miri.log" 2>&1
cargo +1.98.1 test --locked --lib > "$root/evidence/e14-unit.log" 2>&1
cargo +1.98.1 clippy --locked --workspace --all-targets -- -D warnings > "$root/evidence/e14-clippy.log" 2>&1
for kind in st-eval unigram-st-eval; do
 cargo +1.98.1 build --release --locked --manifest-path "autoresearch/$kind/Cargo.toml" > "$root/evidence/e14-$kind-build.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/$kind" "$root/e14-candidate-$kind"
 readelf -p .comment "$root/e14-candidate-$kind" > "$root/evidence/e14-$kind-compiler.txt"
 grep -q 'rustc version 1.98.1 ' "$root/evidence/e14-$kind-compiler.txt"
done
sha256sum "$root/e14-candidate-st-eval" "$root/e14-candidate-unigram-st-eval" Cargo.lock autoresearch/st-eval/Cargo.lock autoresearch/unigram-st-eval/Cargo.lock > "$root/evidence/e14-hashes.txt"
test -z "$(git status --porcelain)"
python3 "$HOME/st-e14-screen-20260917.py" > "$root/evidence/e14-pmu.log" 2>&1
python3 "$HOME/st-summarize-pmu-20260917.py" "$root/evidence/e14-pmu" > "$root/evidence/e14-pmu-summary.txt"
python3 - <<'PY'
from pathlib import Path
s=(Path.home()/'st-unigram-screen-fixed-20260917.py').read_text().replace("cycles,instructions,branch-misses", "cycles,instructions,branch-misses,minor-faults")
(Path.home()/'st-e14-unigram-screen-20260917.py').write_text(s)
PY
python3 "$HOME/st-e14-unigram-screen-20260917.py" "$root/e10-parent-204aae80ae16190c0dccd03f57a54ea24bdccb89" "$root/e14-candidate-unigram-st-eval" "$root/unigram-data-v1" "$root/evidence/e14-unigram-pmu" > "$root/evidence/e14-unigram-pmu.log" 2>&1
python3 "$HOME/st-summarize-unigram-pmu-20260917.py" "$root/evidence/e14-unigram-pmu" > "$root/evidence/e14-unigram-summary.txt"
