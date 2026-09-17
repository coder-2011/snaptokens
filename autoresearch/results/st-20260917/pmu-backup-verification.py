import hashlib,json,tarfile
from pathlib import Path
root=Path('/Users/namanchetwani/Projects/snaptokens-st-evidence-20260917')
archive=root/'st-pmu-evidence-20260917.tar.gz'
expected='8ef30de4a9bc7a91dec09e6663c728e6d3e205edd4cef7f92903d58c55e1e214'
def digest_stream(f):
 h=hashlib.sha256()
 for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
with archive.open('rb') as f: assert digest_stream(f)==expected
manifest=json.loads((root/'st-pmu-evidence-20260917-manifest.json').read_text())
seen=set()
with tarfile.open(archive,'r|gz') as t:
 for member in t:
  assert member.isfile(),member.name
  if member.name=='backup-manifest.json':
   assert json.load(t.extractfile(member))==manifest
   continue
  record=manifest[member.name];assert member.name not in seen
  assert member.size==record['bytes']
  assert digest_stream(t.extractfile(member))==record['sha256'],member.name
  seen.add(member.name)
assert seen==set(manifest)
receipt={'archive':str(archive),'sha256':expected,'bytes':archive.stat().st_size,'verified_files':len(seen),'uncompressed_bytes':sum(v['bytes'] for v in manifest.values()),'method':'Archive SHA256 and every member size/SHA256 checked against remote manifest; no extraction'}
(root/'verified.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt),flush=True)
