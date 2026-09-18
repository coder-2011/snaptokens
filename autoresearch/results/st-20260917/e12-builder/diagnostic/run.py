import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import subprocess
import sys

binary, data, out = map(Path, sys.argv[1:4])
out.mkdir(exist_ok=False)
models = [data/r['file'] for r in json.loads((data/'panel.json').read_text())['models']]
manifest = {'pairs': 3, 'builds_per_process': 10, 'warmup_builds': 1,
            'reference_blocks': 16, 'candidate_blocks': 1,
            'heap_limit': 1.05, 'minimum_builder_speedup': 1.10,
            'purpose': 'matcher-only mechanism, no tokenizer retention',
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
            'inputs': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in [*models,data/'corpus.json']},
            'runner_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            'host': os.uname().nodename}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
checks={}
for model in models:
    result=subprocess.check_output([str(binary),'check',str(model),str(data/'corpus.json')],text=True,stderr=subprocess.STDOUT)
    (out/(model.stem+'-check.json')).write_text(result)
    checks[model.stem]=json.loads(result)
if any(v['candidate_bytes']/v['reference_bytes']>1.05 for v in checks.values()):
    (out/'verdict.json').write_text(json.dumps({'pass':False,'reason':'matcher heap exceeds 1.05','checks':checks},indent=2)+'\n')
    print('rejected footprint before timing',flush=True)
    sys.exit(0)
records=[]
for pair in range(3):
    for model in models:
        order=[('parent',16),('candidate',1)]
        if pair%2: order.reverse()
        for label,blocks in order:
            result=subprocess.check_output([str(binary),'build',str(model),str(blocks),'10'],text=True,stderr=subprocess.STDOUT)
            (out/f'{pair}-{model.stem}-{label}.ns').write_text(result)
            values=[int(v) for v in result.splitlines()]
            assert len(values)==10 and min(values)>0
            records.append(dict(pair=pair,model=model.stem,label=label,ns=values))
    print('finished builder pair',pair+1,flush=True)
cells={}
for model in models:
    ratios=[]; first=[]
    for pair in range(3):
        rows={r['label']:r['ns'] for r in records if r['model']==model.stem and r['pair']==pair}
        ratios.append(statistics.median(rows['parent'][1:])/statistics.median(rows['candidate'][1:]))
        first.append(rows['parent'][0]/rows['candidate'][0])
    cells[model.stem]={'ratios':ratios,'median':statistics.median(ratios),'first_ratios':first}
point=math.exp(statistics.mean(math.log(c['median']) for c in cells.values()))
for model in models:
    result=subprocess.check_output([str(binary),'check',str(model),str(data/'corpus.json')],text=True,stderr=subprocess.STDOUT)
    (out/(model.stem+'-post-check.json')).write_text(result)
(out/'verdict.json').write_text(json.dumps({'pass':point>=1.10,'point':point,'checks':checks,'cells':cells},indent=2)+'\n')
print(point,flush=True)
