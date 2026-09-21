import collections, json, math, pathlib, random, statistics, sys
ROOT=pathlib.Path(__file__).resolve().parent
def metrics(p):return [p[0][0]]+[statistics.median(row[i] for row in p[1:]) for i in range(3)]
def read(label):
 pairs=collections.defaultdict(dict)
 for line in (ROOT/'raw'/(label+'.jsonl')).read_text().splitlines():
  r=json.loads(line);key=(r['model'],r['corpus'],r['threads'],r['round']);pairs[key][r['side']]=metrics(r['passes_ns'])
 cells=collections.defaultdict(list)
 for (m,c,t,r),v in pairs.items():
  if len(v)!=2:continue
  for i,name in enumerate(['cold_scalar','warm_scalar','warm_batch','warm_ragged']):cells[(m,c,t,name)].append(math.log(v['a'][i]/v['b'][i]))
 return cells
rng=random.Random(20260920)
def interval(values, aggregate=False):
 def estimate(v):return statistics.mean(statistics.median(a) for a in v) if aggregate else statistics.median(v)
 point=estimate(values);boot=[]
 for _ in range(10000):
  if aggregate:
   ix=[rng.randrange(len(values[0])) for _ in values[0]];x=[[v[i] for i in ix] for v in values]
  else:x=rng.choices(values,k=len(values))
  boot.append(estimate(x))
 boot.sort();return [math.exp(x) for x in (point,boot[249],boot[9749])]
report={}
for label in sys.argv[1:]:
 cells=read(label);result={'cells':{'/'.join(map(str,k)):interval(v) for k,v in cells.items()},'aggregates':{}}
 for name in ['cold_scalar','warm_scalar','warm_batch','warm_ragged']:
  rows=[v for k,v in cells.items() if k[-1]==name]
  if rows:result['aggregates'][name]=interval(rows,True)
 for scope,accept in [('gemma',lambda k:k[0]=='gemma-3'),('fused_controls',lambda k:k[0]!='gemma-3')]:
  result[scope]={name:interval([v for k,v in cells.items() if k[-1]==name and accept(k)],True) for name in ['cold_scalar','warm_scalar','warm_batch','warm_ragged']}
 report[label]=result
 print(label,json.dumps(result['aggregates']),flush=True)
(ROOT/'summary.json').write_text(json.dumps(report,indent=2)+'\n')
