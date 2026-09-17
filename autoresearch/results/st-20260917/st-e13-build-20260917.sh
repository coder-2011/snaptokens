#!/bin/bash
set -eu
root="$HOME/st-campaign-20260917"
test "$(cat "$root/evidence/combined-profile.status")" = 0
trap 'echo $? > "$root/evidence/e13-sequence.status"' EXIT
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$root/target"
export CARGO_PROFILE_RELEASE_DEBUG=1
git -C "$root/base" fetch "$HOME/st-e13.bundle" refs/heads/perf/st-release-file-buffer
git -C "$root/base" worktree add --detach "$root/e13-source" 0d12df599ce6edf8b20fc20a50b5cd78a86ba669
cd "$root/e13-source"
test -z "$(git status --porcelain)"
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 test --locked --lib > "$root/evidence/e13-unit.log" 2>&1
cargo +1.98.1 clippy --locked --workspace --all-targets -- -D warnings > "$root/evidence/e13-clippy.log" 2>&1
for kind in st-eval unigram-st-eval; do
 cargo +1.98.1 build --release --locked --manifest-path "autoresearch/$kind/Cargo.toml" > "$root/evidence/e13-$kind-build.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/$kind" "$root/e13-candidate-$kind"
 readelf -p .comment "$root/e13-candidate-$kind" > "$root/evidence/e13-$kind-compiler.txt"
 grep -q 'rustc version 1.98.1 ' "$root/evidence/e13-$kind-compiler.txt"
done
sha256sum "$root/e13-candidate-st-eval" "$root/e13-candidate-unigram-st-eval" Cargo.lock autoresearch/st-eval/Cargo.lock autoresearch/unigram-st-eval/Cargo.lock > "$root/evidence/e13-hashes.txt"
test -z "$(git status --porcelain)"
python3 "$HOME/st-e13-screen-20260917.py" > "$root/evidence/e13-pmu.log" 2>&1
python3 "$HOME/st-summarize-pmu-20260917.py" "$root/evidence/e13-pmu" > "$root/evidence/e13-pmu-summary.txt"
python3 - <<'PY'
from pathlib import Path
s=(Path.home()/'st-unigram-screen-fixed-20260917.py').read_text().replace("cycles,instructions,branch-misses", "cycles,instructions,branch-misses,minor-faults")
(Path.home()/'st-e13-unigram-screen-20260917.py').write_text(s)
PY
python3 "$HOME/st-e13-unigram-screen-20260917.py" "$root/e10-parent-204aae80ae16190c0dccd03f57a54ea24bdccb89" "$root/e13-candidate-unigram-st-eval" "$root/unigram-data-v1" "$root/evidence/e13-unigram-pmu" > "$root/evidence/e13-unigram-pmu.log" 2>&1
python3 "$HOME/st-summarize-unigram-pmu-20260917.py" "$root/evidence/e13-unigram-pmu" > "$root/evidence/e13-unigram-summary.txt"
python3 "$HOME/st-e13-rss-20260917.py" > "$root/evidence/e13-rss.log" 2>&1
