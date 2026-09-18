import hashlib,json,os,statistics,subprocess
from pathlib import Path
root=Path.home()/'st-campaign-20260917';out=root/'evidence/e13-rss';out.mkdir();env=dict(os.environ,RAYON_NUM_THREADS='4');rows=[]
for family,data,parent,candidate in [('bpe',root/'st-transfer-panel-20260917',root/'combined-profile-st-eval',root/'e13-candidate-st-eval'),('unigram',root/'unigram-data-v1',root/'e10-parent-204aae80ae16190c0dccd03f57a54ea24bdccb89',root/'e13-candidate-unigram-st-eval')]:
 models=sorted(p for p in data.glob('*.json') if p.name not in ['corpus.json','panel.json'])
 for pair in range(3):
  for model in models:
   order=[('parent',parent),('candidate',candidate)]
   if pair%2:order.reverse()
   for label,binary in order:
    with (out/f'{family}-{pair}-{model.stem}-{label}.txt').open('w') as f:
     proc=subprocess.Popen([str(binary),'st',str(model.with_suffix('.st')),'6'],env=env,stdout=f,stderr=subprocess.STDOUT)
     _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status);assert proc.returncode==0
    rows.append(dict(family=family,pair=pair,model=model.stem,label=label,rss_bytes=usage.ru_maxrss*1024))
 (out/(family+'-binaries.json')).write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [parent,candidate]},indent=2)+'\n')
(out/'rounds.json').write_text(json.dumps(rows,indent=2)+'\n');summary={}
for family,model in sorted({(r['family'],r['model']) for r in rows}):
 ratios=[]
 for pair in range(3):
  values={r['label']:r['rss_bytes'] for r in rows if r['family']==family and r['model']==model and r['pair']==pair};ratios.append(values['candidate']/values['parent'])
 summary[family+'/'+model]={'ratios':ratios,'median':statistics.median(ratios)}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
