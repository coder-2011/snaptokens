#!/bin/bash
set -eu
root="$HOME/st-campaign-20260917"
test "$(cat "$root/evidence/e7-runtime-recheck.status")" = 0
trap 'echo $? > "$root/evidence/combined-profile.status"' EXIT
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$root/target"
export CARGO_PROFILE_RELEASE_DEBUG=1
out="$root/evidence/combined-profile"
mkdir "$out"
git -C "$root/base" worktree add --detach "$root/combined-profile-source" 204aae80ae16190c0dccd03f57a54ea24bdccb89
cd "$root/combined-profile-source"
test -z "$(git status --porcelain)"
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 build --release --locked --manifest-path autoresearch/st-eval/Cargo.toml > "$out/build.log" 2>&1
cp "$CARGO_TARGET_DIR/release/st-eval" "$root/combined-profile-st-eval"
readelf -p .comment "$root/combined-profile-st-eval" > "$out/compiler.txt"
grep -q 'rustc version 1.98.1 ' "$out/compiler.txt"
sha256sum "$root/combined-profile-st-eval" Cargo.lock autoresearch/st-eval/Cargo.lock > "$out/hashes.txt"
git rev-parse HEAD > "$out/source.commit"
python3 - <<'PY'
import hashlib,json,os,subprocess
from pathlib import Path
root=Path.home()/'st-campaign-20260917';out=root/'evidence/combined-profile';data=root/'st-transfer-panel-20260917';binary=root/'combined-profile-st-eval'
models=['Qwen--Qwen3-0.6B','mistralai--Mistral-Large-3-675B-Instruct-2512','mistralai--Mistral-Nemo-Instruct-2407','openai--gpt-oss-120b','unsloth--gemma-3-1b-it']
env=dict(os.environ,RAYON_NUM_THREADS='4')
def run(args,name):
 with (out/name).open('w') as f:subprocess.run(list(map(str,args)),env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
manifest={'purpose':'fresh profile of combined BPE/Unigram runtime, no candidate timing','models':models,'loads':100,'event':'cycles','frequency':997,'stack':'dwarf','binaries':{binary.name:hashlib.sha256(binary.read_bytes()).hexdigest()},'inputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [data/'corpus.json',*[data/(m+s) for m in models for s in ['.json','.st']]]}}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
for model in models:
 run([binary,'check',data/(model+'.json'),data/'corpus.json'],model+'-pre-exact.txt')
 run(['sudo','--preserve-env=RAYON_NUM_THREADS','perf','record','-F','997','-e','cycles','--call-graph','dwarf','-o',out/(model+'.data'),'--',binary,'st',data/(model+'.st'),'100'],model+'-record.log')
 run(['sudo','perf','report','--stdio','--no-children','-i',out/(model+'.data'),'--percent-limit','0.8'],model+'-report.txt')
 run(['sudo','--preserve-env=RAYON_NUM_THREADS','perf','stat','-x',';','-o',out/(model+'-counters.csv'),'-e','cycles,instructions,minor-faults,branch-misses','--',binary,'st',data/(model+'.st'),'100'],model+'-counter-loads.txt')
 run([binary,'check',data/(model+'.json'),data/'corpus.json'],model+'-post-exact.txt')
 print('profile completed',model,flush=True)
PY
