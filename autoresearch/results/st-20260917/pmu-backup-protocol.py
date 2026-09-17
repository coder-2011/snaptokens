import hashlib,json,os,tarfile
from pathlib import Path
root=Path('/home/namanchetwani/st-campaign-20260917')
archive=Path('/tmp/st-pmu-evidence-20260917.tar.gz')
assert not archive.exists()
files=sorted(p for p in (root/'evidence').rglob('*') if p.is_file())
files+=sorted(p for p in root.iterdir() if p.is_file())
def digest(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
manifest={str(p.relative_to(root)):{'bytes':p.stat().st_size,'sha256':digest(p)} for p in files}
m=Path('/tmp/st-pmu-evidence-20260917-manifest.json');m.write_text(json.dumps(manifest,indent=2)+'\n')
with tarfile.open(archive,'w:gz',compresslevel=3) as t:
 for p in files:t.add(p,arcname=str(p.relative_to(root)),recursive=False)
 t.add(m,arcname='backup-manifest.json')
for p in [archive,m]:os.chown(p,root.stat().st_uid,root.stat().st_gid)
print(json.dumps({'archive':str(archive),'bytes':archive.stat().st_size,'sha256':digest(archive),'files':len(files),'uncompressed_bytes':sum(v['bytes'] for v in manifest.values())}),flush=True)
