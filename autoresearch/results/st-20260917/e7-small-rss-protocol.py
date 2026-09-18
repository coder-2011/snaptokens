import hashlib,json,os,statistics,subprocess,time
from pathlib import Path
root=Path.home()/'st-campaign-20260917'
while not (root/'evidence/st-argv0-diagnostic/status').exists():time.sleep(5)
assert (root/'evidence/st-argv0-diagnostic/status').read_text().strip()=='0'
out=root/'evidence/e7-small-rss';out.mkdir();data=root/'e7-small-data';model=data/'gpt-2.json';corpus=data/'corpus.json';env=dict(os.environ,RAYON_NUM_THREADS='4')
binaries={'parent':root/'e7-runtime-parent-st-eval','candidate':root/'e7-runtime-candidate-st-eval'}
manifest={'purpose':'predeclared small-model resource rejection screen; no promotion','pairs':3,'loads':6,'rss_limit':1.05,'early_reject_rule':'all three candidate/parent RSS ratios exceed 1.05','binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in binaries.items()},'inputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [model,corpus]},'threads':4}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');rows=[]
for phase in ['pre','post']:
 for label,binary in binaries.items():
  with (out/f'{phase}-{label}.txt').open('w') as f:subprocess.run([str(binary),'check',str(model),str(corpus)],env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
 if phase=='post':break
 for pair in range(3):
  order=list(binaries.items())
  if pair%2:order.reverse()
  for label,binary in order:
   path=out/f'{pair}-{label}.ns'
   with path.open('w') as f:
    proc=subprocess.Popen([str(binary),'st',str(model.with_suffix('.st')),'6'],env=env,stdout=f,stderr=subprocess.STDOUT)
    _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status);assert proc.returncode==0
   values=[int(x) for x in path.read_text().splitlines()];assert len(values)==6
   rows.append(dict(pair=pair,label=label,rss_bytes=usage.ru_maxrss*1024,ns=values))
ratios=[]
for pair in range(3):
 r={v['label']:v for v in rows if v['pair']==pair};ratios.append(r['candidate']['rss_bytes']/r['parent']['rss_bytes'])
(out/'rounds.json').write_text(json.dumps(rows,indent=2)+'\n');(out/'verdict.json').write_text(json.dumps({'reject':all(r>1.05 for r in ratios),'rss_ratios':ratios,'median':statistics.median(ratios)},indent=2)+'\n');(out/'status').write_text('0\n')
