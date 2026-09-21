#!/usr/bin/env python3
"""Frozen local orchestration of the unchanged st-eval executable."""
import hashlib, json, os, pathlib, statistics, subprocess, sys, time
ROOT=pathlib.Path('/tmp/snaptokens-cache-evidence-20260920')
MODELS=['gpt-2','qwen-3','gpt-oss','gemma-3']
CELLS=[(m,c,4) for m in MODELS for c in ['longbench','sharegpt']]+[('gpt-oss','longbench',1)]
def invoke(binary,mode,model,corpus,threads):
 env=dict(os.environ,RAYON_NUM_THREADS=str(threads),TOKENIZERS_PARALLELISM='false')
 return subprocess.run([str(ROOT/'bin'/binary),mode,str(ROOT/'data'/(model+'.json')),str(ROOT/'data'/(corpus+'.json'))],env=env,text=True,capture_output=True,check=True)
def check(binary,stage):
 with (ROOT/'raw'/(binary+'-'+stage+'-parity.txt')).open('w') as log:
  for model in MODELS:
   for corpus in ['semantic','longbench','sharegpt']:
    r=invoke(binary,'check',model,corpus,4); log.write(r.stdout+r.stderr);log.flush()
    print(binary,stage,model,corpus,'exact',flush=True)
  r=invoke(binary,'check','gpt-oss','longbench',1);log.write(r.stdout+r.stderr)
def pairs(a,b,label,rounds):
 path=ROOT/'raw'/(label+'.jsonl')
 with path.open('x') as f:
  for round in range(rounds):
   for index,(model,corpus,threads) in enumerate(CELLS):
    order=[('a',a),('b',b)] if (round+index)%2==0 else [('b',b),('a',a)]
    for side,binary in order:
     before=time.time();r=invoke(binary,'encode',model,corpus,threads)
     passes=[list(map(int,line.split())) for line in r.stdout.splitlines()]
     assert len(passes)==6 and all(len(p)==3 for p in passes)
     row=dict(round=round,model=model,corpus=corpus,threads=threads,side=side,binary=binary,started=before,wall_s=time.time()-before,passes_ns=passes,stderr=r.stderr)
     f.write(json.dumps(row)+'\n');f.flush()
   print(label,'round',round+1,'/',rounds,flush=True)
if __name__=='__main__':
 if sys.argv[1]=='check':check(*sys.argv[2:])
 elif sys.argv[1]=='pairs':pairs(*sys.argv[2:5],int(sys.argv[5]))
