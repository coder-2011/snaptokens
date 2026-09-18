import hashlib,json,os,subprocess
from pathlib import Path
root=Path.home()/'st-campaign-20260917';out=root/'evidence/e11-loss-diagnostic';out.mkdir()
assert (root/'evidence/e14-sequence.status').read_text().strip()=='0'
binaries={'parent':root/'95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71','candidate':root/'6f297b1e0f1c30e7bce1423ae657aa19fbec0468'}
data=root/'st-transfer-panel-20260917';model=data/'mistralai--Mistral-Large-3-675B-Instruct-2512.json';env=dict(os.environ,RAYON_NUM_THREADS='4')
manifest={'purpose':'post-selected attribution of already rejected E11 loss, never promotion','model':model.name,'pairs':3,'loads':100,'binaries':{k:hashlib.sha256(v.read_bytes()).hexdigest() for k,v in binaries.items()},'inputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [model,model.with_suffix('.st'),data/'corpus.json']},'events':['cycles','instructions:u','instructions:k','minor-faults']}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
def run(args,name):
 with (out/name).open('w') as f:subprocess.run(list(map(str,args)),env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
for phase in ['pre','post']:
 for label,binary in binaries.items():run([binary,'check',model,data/'corpus.json'],phase+'-'+label+'-exact.txt')
 if phase=='post':break
 for pair in range(3):
  order=list(binaries.items())
  if pair%2:order.reverse()
  for label,binary in order:
   run(['sudo','--preserve-env=RAYON_NUM_THREADS','perf','stat','-x',';','-o',out/f'{pair}-{label}.perf','-e',','.join(manifest['events']),binary,'st',model.with_suffix('.st'),'100'],f'{pair}-{label}.ns')
 for label,binary in binaries.items():
  run(['strace','-f','-c','-e','brk,mmap,munmap,mremap','-o',out/(label+'-strace.txt'),binary,'st',model.with_suffix('.st'),'100'],label+'-strace-loads.txt')
  run(['sudo','--preserve-env=RAYON_NUM_THREADS','perf','record','-F','997','-e','cycles','--call-graph','dwarf','-o',out/(label+'.data'),'--',binary,'st',model.with_suffix('.st'),'100'],label+'-record.log')
  run(['sudo','perf','report','--stdio','--no-children','-i',out/(label+'.data'),'--percent-limit','0.8'],label+'-report.txt')
(out/'status').write_text('0\n')
