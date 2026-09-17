import hashlib,json,math,os,random,statistics,subprocess
from pathlib import Path
root=Path.home()/'st-campaign-20260917';out=root/'evidence/st-argv0-diagnostic';out.mkdir();binary=root/'combined-profile-st-eval';data=root/'st-transfer-panel-20260917';env=dict(os.environ,RAYON_NUM_THREADS='4')
models=['Qwen--Qwen3-0.6B','mistralai--Mistral-Nemo-Instruct-2407','mistralai--Mistral-Large-3-675B-Instruct-2512']
manifest={'purpose':'same-binary argv0 and fresh-process allocator control, not candidate retention','pairs':12,'loads':100,'modes':['identical-argv0','different-argv0'],'models':models,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'argv0_parent':str(binary),'argv0_candidate':str(root/'e13-candidate-st-eval'),'inputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [data/'corpus.json',*[data/(m+s) for m in models for s in ['.json','.st']]]}}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');rows=[]
for phase in ['pre','post']:
 for model in models:
  with (out/f'{phase}-{model}.txt').open('w') as f:subprocess.run([str(binary),'check',str(data/(model+'.json')),str(data/'corpus.json')],env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
 if phase=='post':break
 for mode in manifest['modes']:
  for pair in range(12):
   for model in models:
    order=['parent','candidate']
    if pair%2:order.reverse()
    for label in order:
     argv0=manifest['argv0_candidate'] if mode=='different-argv0' and label=='candidate' else str(binary)
     p=out/f'{mode}-{pair}-{model}-{label}.ns'
     with p.open('w') as f:
      proc=subprocess.Popen([argv0,'st',str(data/(model+'.st')),'100'],executable=str(binary),env=env,stdout=f,stderr=subprocess.STDOUT)
      _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status);assert proc.returncode==0
     ns=[int(v) for v in p.read_text().splitlines()];assert len(ns)==100
     rows.append(dict(mode=mode,pair=pair,model=model,label=label,ns=ns,minor_faults=usage.ru_minflt,rss_bytes=usage.ru_maxrss*1024,user_seconds=usage.ru_utime,system_seconds=usage.ru_stime))
   print(mode,'pair',pair+1,flush=True)
summary={}
for mode in manifest['modes']:
 cells={}
 for model in models:
  ratios=[];faults=[]
  for pair in range(12):
   r={x['label']:x for x in rows if x['mode']==mode and x['model']==model and x['pair']==pair}
   ratios.append(statistics.median(r['parent']['ns'][1:])/statistics.median(r['candidate']['ns'][1:]));faults.append([r['parent']['minor_faults'],r['candidate']['minor_faults']])
  rng=random.Random(0);boots=sorted(math.exp(statistics.median(math.log(ratios[i]) for i in rng.choices(range(12),k=12))) for _ in range(3000))
  cells[model]={'ratios':ratios,'point':math.exp(statistics.median(map(math.log,ratios))),'ci95':[boots[75],boots[2924]],'minor_faults':faults}
 summary[mode]=cells
(out/'rounds.json').write_text(json.dumps(rows,indent=2)+'\n');(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');(out/'status').write_text('0\n')
