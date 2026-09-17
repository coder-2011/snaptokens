#!/bin/bash
set -eu
root="$HOME/st-campaign-20260917"
test "$(cat "$root/evidence/combined-profile.status")" = 0
trap 'echo $? > "$root/evidence/e16-sequence.status"' EXIT
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$root/target"
export CARGO_PROFILE_RELEASE_DEBUG=1
git -C "$root/base" fetch "$HOME/st-e16.bundle" refs/heads/perf/st-id-max-validation
git -C "$root/base" worktree add --detach "$root/e16-source" 29b43f2e191cf601231104f1482cc8281006869e
cd "$root/e16-source"
test -z "$(git status --porcelain)"
test -z "${RUSTFLAGS:-}"
for lock in Cargo.lock autoresearch/st-eval/Cargo.lock autoresearch/unigram-st-eval/Cargo.lock; do
 git diff --exit-code 204aae80ae16190c0dccd03f57a54ea24bdccb89 -- "$lock"
done
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 test --locked --lib > "$root/evidence/e16-unit.log" 2>&1
cargo +1.98.1 clippy --locked --workspace --all-targets -- -D warnings > "$root/evidence/e16-clippy.log" 2>&1
for kind in st-eval unigram-st-eval; do
 cargo +1.98.1 build --release --locked --manifest-path "autoresearch/$kind/Cargo.toml" > "$root/evidence/e16-$kind-build.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/$kind" "$root/e16-candidate-$kind"
 readelf -p .comment "$root/e16-candidate-$kind" > "$root/evidence/e16-$kind-compiler.txt"
 grep -q 'rustc version 1.98.1 ' "$root/evidence/e16-$kind-compiler.txt"
done
sha256sum "$root/e16-candidate-st-eval" "$root/e16-candidate-unigram-st-eval" Cargo.lock autoresearch/st-eval/Cargo.lock autoresearch/unigram-st-eval/Cargo.lock > "$root/evidence/e16-hashes.txt"
test -z "$(git status --porcelain)"
python3 "$HOME/st-e16-assembly-20260917.py" > "$root/evidence/e16-assembly.log"
python3 "$HOME/st-e16-screen-20260917.py" > "$root/evidence/e16-pmu.log" 2>&1
python3 "$HOME/st-summarize-pmu-20260917.py" "$root/evidence/e16-pmu" > "$root/evidence/e16-pmu-summary.txt"
python3 - <<'PY'
import json
from pathlib import Path
root=Path.home()/'st-campaign-20260917/evidence'
point=json.loads((root/'e16-pmu-summary.txt').read_text().splitlines()[0])['warm_load']
(root/'e16-pmu/admission.txt').write_text(f'{point:.12f}\n')
if point<1.02:
 (root/'e16-pmu/rejected.marker').write_text('BPE mechanism below 1.02; Unigram/later gates skipped\n')
PY
if test -f "$root/evidence/e16-pmu/rejected.marker"; then exit 0; fi
python3 - <<'PY' 
from pathlib import Path
s=(Path.home()/'st-unigram-screen-fixed-20260917.py').read_text().replace("cycles,instructions,branch-misses", "cycles,instructions,branch-misses,minor-faults")
(Path.home()/'st-e16-unigram-screen-20260917.py').write_text(s)
PY
python3 "$HOME/st-e16-unigram-screen-20260917.py" "$root/e10-parent-204aae80ae16190c0dccd03f57a54ea24bdccb89" "$root/e16-candidate-unigram-st-eval" "$root/unigram-data-v1" "$root/evidence/e16-unigram-pmu" > "$root/evidence/e16-unigram-pmu.log" 2>&1
python3 "$HOME/st-summarize-unigram-pmu-20260917.py" "$root/evidence/e16-unigram-pmu" > "$root/evidence/e16-unigram-summary.txt"
