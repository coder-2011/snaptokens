"""Frozen encode/RSS guard orchestration; invokes the existing evaluator unchanged."""
import hashlib
import json
import math
import os
from pathlib import Path
import random
import statistics
import subprocess
import sys

parent, candidate, data, out = map(Path, sys.argv[1:5])
out.mkdir(exist_ok=False, parents=True)
models = sorted(p for p in data.glob('*.json') if p.name != 'corpus.json')
env = dict(os.environ, RAYON_NUM_THREADS='4')
manifest = {'pairs':12, 'warmup_passes':1, 'retained_passes':5,
    'load_rss_iterations':6, 'rss_unit':'bytes',
    'encode_boundary':'existing st-eval encode includes all outputs and destruction',
    'binaries':{str(p):{'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'size':p.stat().st_size} for p in [parent,candidate]},
    'inputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [*models,data/'corpus.json']}}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2))

def invoke(args, name):
    path = out/name
    with path.open('w') as capture:
        proc = subprocess.Popen([str(a) for a in args], stdout=capture, stderr=subprocess.STDOUT, env=env)
        _, status, usage = os.wait4(proc.pid, 0)
        proc.returncode = os.waitstatus_to_exitcode(status)
        if proc.returncode: raise RuntimeError(f'{name}: exit {proc.returncode}')
    return path.read_text(), int(usage.ru_maxrss * (1 if sys.platform == 'darwin' else 1024))

records = []
for phase in ['pre','post']:
    for label,binary in [('parent',parent),('candidate',candidate)]:
        for model in models:
            invoke([binary,'check',model,data/'corpus.json'],f'{phase}-{label}-{model.stem}.txt')
    if phase == 'post': break
    with (out/'rounds.jsonl').open('w') as log:
        for pair in range(12):
            for model in models:
                order = [('parent',parent),('candidate',candidate)]
                if pair % 2: order.reverse()
                for label,binary in order:
                    stem = f'{pair}-{model.stem}-{label}'
                    text,_ = invoke([binary,'encode',model.with_suffix('.st'),data/'corpus.json'],stem+'-encode.txt')
                    values = [[int(v) for v in line.split()] for line in text.splitlines()]
                    assert len(values)==6 and all(len(v)==3 for v in values)
                    _,rss = invoke([binary,'st',model.with_suffix('.st'),6],stem+'-load.txt')
                    record = {'pair':pair,'model':model.stem,'label':label,'encode_ns':values,'rss':rss}
                    records.append(record); log.write(json.dumps(record)+'\n'); log.flush()
            print('finished guard pair',pair+1,flush=True)
summary = {}
for column,mode in enumerate(['scalar','batch','ragged','rss']):
    cells = {}
    for model in models:
        ratios = []
        for pair in range(12):
            rows = {r['label']:r for r in records if r['pair']==pair and r['model']==model.stem}
            if mode == 'rss': ratios.append(rows['candidate']['rss']/rows['parent']['rss'])
            else:
                ratios.append(statistics.median(v[column] for v in rows['parent']['encode_ns'][1:])/statistics.median(v[column] for v in rows['candidate']['encode_ns'][1:]))
        rng = random.Random(0)
        samples = sorted(statistics.median(math.log(ratios[i]) for i in rng.choices(range(12),k=12)) for _ in range(3000))
        cells[model.stem] = {'ratios':ratios,'median':statistics.median(ratios),'ci95':[math.exp(samples[75]),math.exp(samples[2924])]}
    summary[mode] = {'cells':cells,'point':math.exp(statistics.mean(math.log(c['median']) for c in cells.values()))}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(out/'status').write_text('0\n')
print(json.dumps({m:r['point'] for m,r in summary.items()}),flush=True)
