import hashlib
import json
import os
from pathlib import Path
import subprocess

root = Path.home() / 'st-campaign-20260917'
data = root / 'st-transfer-panel-20260917'
out = root / 'evidence/e2-pmu'
out.mkdir(exist_ok=False)
binaries = {
    'parent': root / '95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71',
    'candidate': root / 'f59e94fbaea55760f946c775611888a5fff943d1',
}
models = sorted(p for p in data.glob('*.json') if p.name != 'corpus.json')
env = dict(os.environ, RAYON_NUM_THREADS='4')
manifest = {'purpose': 'mechanism screen only; transfer panel frozen before candidate results',
    'pairs': 3, 'loads': 100, 'events': ['cycles', 'instructions', 'branch-misses'],
    'binaries': {k: hashlib.sha256(p.read_bytes()).hexdigest() for k,p in binaries.items()},
    'inputs': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in [*models, data/'corpus.json']}}
(out/'manifest.json').write_text(json.dumps(manifest, indent=2))
for phase in ['pre','post']:
    with (out/(phase+'-exactness.txt')).open('w') as log:
        for label,binary in binaries.items():
            for model in models:
                result = subprocess.check_output([str(binary), 'check', str(model), str(data/'corpus.json')], env=env, text=True, stderr=subprocess.STDOUT)
                log.write(label+': '+result); log.flush()
                print(phase, label, model.stem, flush=True)
    if phase == 'post': break
    for pair in range(3):
        for model in models:
            order = list(binaries.items())
            if pair % 2: order.reverse()
            for label,binary in order:
                stem = f'{pair}-{model.stem}-{label}'
                with (out/(stem+'.ns')).open('w') as times:
                    subprocess.run(['sudo','perf','stat','-x',';', '-o',str(out/(stem+'.perf')),
                        '-e','cycles,instructions,branch-misses',str(binary),'st',str(model.with_suffix('.st')),'100'],
                        env=env,stdout=times,check=True)
        print('finished mechanism pair', pair+1, flush=True)
(out/'status').write_text('0\n')
