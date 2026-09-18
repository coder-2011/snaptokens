import json
import math
from pathlib import Path
import statistics
import sys

p = Path(sys.argv[1])
assert (p/'status').read_text().strip() == '0'
manifest = json.loads((p/'manifest.json').read_text())
models = [Path(m).stem for m in manifest['inputs'] if m != 'corpus.json']
cells = {}
for model in models:
    ratios = {key: [] for key in ['warm_load','cycles','instructions','branch-misses']}
    for pair in range(manifest['pairs']):
        rows = {}
        for label in ['parent','candidate']:
            stem = f'{pair}-{model}-{label}'
            ns = [int(n) for n in (p/(stem+'.ns')).read_text().splitlines()]
            row = {'warm_load':statistics.median(ns[1:])}
            for line in (p/(stem+'.perf')).read_text().splitlines():
                fields = line.split(';')
                if len(fields) > 2 and fields[2] in ratios:
                    row[fields[2]] = float(fields[0])
            rows[label] = row
        for key in ratios: ratios[key].append(rows['parent'][key]/rows['candidate'][key])
    cells[model] = {key:{'median':statistics.median(v), 'ratios':v} for key,v in ratios.items()}
aggregate = {key:math.exp(statistics.mean(math.log(c[key]['median']) for c in cells.values())) for key in ratios}
result = {'meaning':'parent/candidate; mechanism screen only, no portable retention', 'aggregate':aggregate, 'cells':cells}
(p/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(aggregate))
for name,c in cells.items(): print(name, {k:round(v['median'],4) for k,v in c.items()})
