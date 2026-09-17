import hashlib,json,os,shutil,subprocess,sys,time
from pathlib import Path
root=Path(os.environ.get('ST_TASK_ROOT',Path.home()/'st-campaign-20260917'))
wait=root/'evidence/e13-portable.status'
while not wait.exists():time.sleep(5)
assert wait.read_text().strip()=='0',wait
out=root/'evidence/e14-portable';status=root/'evidence/e14-portable.status'
def run(args,log,cwd=None):
    with log.open('w') as f:subprocess.run(list(map(str,args)),cwd=cwd,stdout=f,stderr=subprocess.STDOUT,check=True)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def gates(folder,label,prefix,primary):
    c=json.loads((folder/'calibration.json').read_text())
    s=json.loads((folder/(prefix+label)/'summary.json').read_text());fail=[]
    if primary and (s['st']['point']<1.02 or s['st']['ci95'][0]<=1):fail.append(['primary',s['st']['point'],s['st']['ci95']])
    for mode,cells in c['load'].items():
        for model,band in cells.items():
            value=s[mode]['cells'][model]['median']
            if value<band[0]:fail.append([mode,model,value,band[0]])
    (folder/(label+'-load-gate.json')).write_text(json.dumps({'pass':not fail,'failures':fail,'primary_bpe':primary},indent=2)+'\n')
    return not fail
def guards(folder,label,prefix):
    c=json.loads((folder/'calibration.json').read_text());p=folder/(prefix+label);s=json.loads((p/'summary.json').read_text());fail=[]
    for mode,cells in c['encode'].items():
        for model,band in cells.items():
            value=s[mode]['cells'][model]['median']
            if value<band[0]:fail.append([mode,model,value,band[0]])
    for model,cell in s['rss']['cells'].items():
        if cell['median']>1.05:fail.append(['rss',model,cell['median'],1.05])
    m=json.loads((p/'manifest.json').read_text());bins=list(m['binaries'].values());ratio=bins[1]['size']/bins[0]['size']
    if ratio>1.02:fail.append(['binary',ratio,1.02])
    (folder/(label+'-guard-gate.json')).write_text(json.dumps({'pass':not fail,'failures':fail,'binary_ratio':ratio},indent=2)+'\n')
    return not fail
try:
    out.mkdir();assert shutil.disk_usage(root).free>=3*1024**3
    assert not any(value for name,value in os.environ.items() if name.endswith('RUSTFLAGS')), 'unrecorded Rust flags'
    os.environ['PATH']=str(Path.home()/'.cargo/bin')+':'+os.environ['PATH']
    os.environ['CARGO_TARGET_DIR']=os.environ.get('ST_TARGET',str(root/'target'))
    os.environ['CARGO_PROFILE_RELEASE_DEBUG']='1'
    repo=Path(os.environ.get('ST_GIT_ROOT',root/'base'))
    bundle=Path(os.environ.get('ST_BUNDLE',Path.home()/'st-e14.bundle'))
    source=Path(os.environ.get('ST_SOURCE_PATH',root/'e14-portable-source'))
    sha='320b9615050f2ede54c4d6a8bf68bb39b505795b'
    run(['git','-C',repo,'fetch',bundle,'refs/heads/perf/st-trusted-arena-get'],out/'fetch.log')
    run(['git','-C',repo,'worktree','add','--detach',source,sha],out/'worktree.log')
    assert not subprocess.check_output(['git','status','--porcelain'],cwd=source)
    run(['cargo','+1.98.1','fmt','--all','--','--check'],out/'fmt.log',source)
    bins=out/'binaries';bins.mkdir()
    for kind,name in [('bpe','st-eval'),('unigram','unigram-st-eval')]:
        run(['cargo','+1.98.1','build','--locked','--release','--manifest-path',f'autoresearch/{name}/Cargo.toml'],out/(kind+'-build.log'),source)
        binary=bins/('candidate-'+kind);shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release'/name,binary)
        if sys.platform=='linux':
            metadata=subprocess.check_output(['readelf','-p','.comment',binary],text=True)
            assert 'rustc version 1.98.1 ' in metadata
            (out/(kind+'-compiler.txt')).write_text(metadata)
    locks={str(p):digest(source/p) for p in map(Path,['Cargo.lock','autoresearch/st-eval/Cargo.lock','autoresearch/unigram-st-eval/Cargo.lock'])}
    parent_info=json.loads((root/'evidence/e13-portable/parent-identity.json').read_text())
    assert parent_info['sha']=='204aae80ae16190c0dccd03f57a54ea24bdccb89'
    assert locks==parent_info['locks']
    (out/'candidate-identity.json').write_text(json.dumps({'sha':sha,'locks':locks,'binaries':{p.name:digest(p) for p in bins.iterdir()}},indent=2)+'\n')
    assert not subprocess.check_output(['git','status','--porcelain'],cwd=source)
    run(['rustc','+1.98.1','--version','--verbose'],out/'rustc.txt')
    for kind,ev,data,runner,parent,prior,aa in [
        ('bpe',root/'evaluator-v2',Path(os.environ.get('ST_BPE_DATA',root/'data')),'run.py',root/'evidence/e13-portable/binaries/parent-bpe',root/'evidence/e13-portable/bpe/calibration.json',root/'evidence/e13-portable/bpe/v2-aa-identical/manifest.json'),
        ('unigram',root/'unigram-protocol-v1',root/'unigram-v1-data','paired.py',root/'unigram-v1-binaries/parent',root/'evidence/unigram-v1/calibration.json',root/'evidence/unigram-v1/aa-identical/manifest.json')]:
        folder=out/kind;folder.mkdir();frozen=json.loads(aa.read_text())
        identity=frozen['binaries'][str(parent)]
        assert digest(parent)==(identity['sha256'] if isinstance(identity,dict) else identity)
        assert digest(ev/runner)==frozen['runner_sha256']
        assert digest(ev/'scoring.py')==frozen['scoring_sha256']
        for name,expected in frozen['inputs'].items():assert digest(data/name)==expected,name
        assert {p.name for p in data.glob('*.json') if p.name!='panel.json'}==set(frozen['inputs'])
        shutil.copy2(prior,folder/'calibration.json')
        (folder/'calibration-reuse.json').write_text(json.dumps({'path':str(prior),'sha256':digest(prior),'aa_manifest':str(aa),'aa_manifest_sha256':digest(aa)},indent=2)+'\n')
        args=[parent,bins/('candidate-'+kind),data]
        run(['python3',ev/runner,*args,folder/'e14'],folder/'e14.log')
        if not gates(folder,'e14','',kind=='bpe'):
            (out/'rejected.marker').write_text(kind+' load gate failed\n');break
        run(['python3',ev/'guards.py',*args,folder/'guard-e14'],folder/'guard-e14.log')
        if not guards(folder,'e14','guard-'):
            (out/'rejected.marker').write_text(kind+' encode/resource gate failed\n');break
    status.write_text('0\n')
except BaseException:
    status.write_text('1\n');raise
