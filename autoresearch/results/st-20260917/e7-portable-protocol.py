import hashlib,json,math,os,shutil,subprocess,sys,time
from pathlib import Path
root=Path(os.environ.get('ST_TASK_ROOT',Path.home()/'st-campaign-20260917'))
wait=Path(os.environ['ST_WAIT_STATUS'])
while not wait.exists():time.sleep(5)
assert wait.read_text().strip()=='0',wait
out=root/'evidence/e7-portable'
status=root/'evidence/e7-portable.status'
def run(args,log,cwd=None):
    with log.open('w') as f:subprocess.run(list(map(str,args)),cwd=cwd,stdout=f,stderr=subprocess.STDOUT,check=True)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def freeze(calibrator,folder,previous):
    run(['python3',calibrator,folder,folder/'fresh-calibration.json'],folder/'calibration.log')
    c=json.loads((folder/'fresh-calibration.json').read_text());old=json.loads(previous.read_text())
    for category in ['load','encode']:
        for mode,cells in c[category].items():
            for model,band in cells.items():
                prior=old[category][mode][model]
                cells[model]=[max(band[0],prior[0]),min(band[1],prior[1])]
    c['policy']+='; intersect with previously frozen host bands, before candidate data'
    c['previous_calibration_sha256']=digest(previous)
    (folder/'calibration.json').write_text(json.dumps(c,indent=2)+'\n')
def gates(folder,label,prefix,primary):
    c=json.loads((folder/'calibration.json').read_text())
    s=json.loads((folder/(prefix+label)/'summary.json').read_text());fail=[]
    if primary and (s['st']['point']<1.02 or s['st']['ci95'][0]<=1):fail.append(['primary',s['st']['point'],s['st']['ci95']])
    for mode,cells in c['load'].items():
        for model,band in cells.items():
            value=s[mode]['cells'][model]['median']
            if value<band[0]:fail.append([mode,model,value,band[0]])
    (folder/'load-gate.json').write_text(json.dumps({'pass':not fail,'failures':fail,'primary_bpe':primary},indent=2)+'\n')
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
    (folder/'guard-gate.json').write_text(json.dumps({'pass':not fail,'failures':fail,'binary_ratio':ratio},indent=2)+'\n')
    return not fail
try:
    out.mkdir();assert shutil.disk_usage(root).free>=3*1024**3
    os.environ['PATH']=str(Path.home()/'.cargo/bin')+':'+os.environ['PATH']
    os.environ['CARGO_TARGET_DIR']=os.environ.get('ST_TARGET',str(root/'target'))
    os.environ['CARGO_PROFILE_RELEASE_DEBUG']='1'
    repo=Path(os.environ.get('ST_GIT_ROOT',root/'base'))
    bundle=Path(os.environ.get('ST_BUNDLE',Path.home()/'st-e7-runtime.bundle'))
    source=Path(os.environ.get('ST_SOURCE_PATH',root/'e7-portable-source'))
    parent='61a038b6f5d244e3e2fb2c9560d36f8650067ea3';candidate='3ee40d61690eebe0a9338ea61e62a9705a4d590e'
    run(['rustup','component','add','rustfmt','--toolchain','1.98.1'],out/'rustfmt.log')
    run(['git','-C',repo,'fetch',bundle,'refs/heads/perf/st-parallel-checksum'],out/'fetch.log')
    run(['git','-C',repo,'worktree','add','--detach',source,parent],out/'worktree.log')
    independent=Path(str(source)+'-independent')
    run(['git','-C',repo,'worktree','add','--detach',independent,parent],out/'independent-worktree.log')
    bins=out/'binaries';bins.mkdir();locks=[]
    for label in ['parent','independent','candidate']:
        cwd=independent if label=='independent' else source;sha=candidate if label=='candidate' else parent
        run(['git','switch','--detach',sha],out/(label+'-checkout.log'),cwd)
        assert not subprocess.check_output(['git','status','--porcelain'],cwd=cwd)
        run(['cargo','+1.98.1','fmt','--all','--','--check'],out/(label+'-fmt.log'),cwd)
        for kind,name in [('bpe','st-eval'),('unigram','unigram-st-eval')]:
            run(['cargo','+1.98.1','build','--locked','--release','--manifest-path',f'autoresearch/{name}/Cargo.toml'],out/(label+'-'+kind+'-build.log'),cwd)
            binary=bins/(label+'-'+kind);shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release'/name,binary)
            if sys.platform=='linux':
                metadata=subprocess.check_output(['readelf','-p','.comment',binary],text=True)
                assert 'rustc version 1.98.1 ' in metadata
                (out/(label+'-'+kind+'-compiler.txt')).write_text(metadata)
        hashes={str(p):digest(cwd/p) for p in map(Path,['Cargo.lock','autoresearch/st-eval/Cargo.lock','autoresearch/unigram-st-eval/Cargo.lock'])};locks.append(hashes)
        (out/(label+'-identity.json')).write_text(json.dumps({'sha':sha,'locks':hashes,'binaries':{p.name:digest(p) for p in bins.glob(label+'-*')}},indent=2)+'\n')
        assert not subprocess.check_output(['git','status','--porcelain'],cwd=cwd)
    assert locks[0]==locks[1]==locks[2]
    run(['rustc','+1.98.1','--version','--verbose'],out/'rustc.txt')
    bpe_eval=root/'evaluator-v2';uni_eval=root/'unigram-protocol-v1'
    for kind,ev,data,prefix,guardprefix,runner,calib in [
      ('bpe',bpe_eval,Path(os.environ.get('ST_BPE_DATA',root/'data')),'v2-','v2-guard-','run.py',Path(os.environ.get('ST_BPE_CALIBRATOR',Path.home()/'st-freeze-v2-calibration-20260917.py'))),
      ('unigram',uni_eval,root/'unigram-v1-data','','guard-','paired.py',uni_eval/'calibrate.py')]:
        folder=out/kind;folder.mkdir()
        for label in ['aa-identical','aa-independent','e7']:
            compared='parent' if label=='aa-identical' else 'independent' if label=='aa-independent' else 'candidate'
            args=[bins/('parent-'+kind),bins/(compared+'-'+kind),data]
            run(['python3',ev/runner,*args,folder/(prefix+label)],folder/(prefix+label+'.log'))
            if label=='e7' and not gates(folder,label,prefix,kind=='bpe'):
                (out/'rejected.marker').write_text(kind+' load gate failed\n');break
            run(['python3',ev/'guards.py',*args,folder/(guardprefix+label)],folder/(guardprefix+label+'.log'))
            if label=='aa-independent':
                previous=root/'evidence/v2-calibration.json' if kind=='bpe' else root/'evidence/unigram-v1/calibration.json'
                freeze(calib,folder,previous)
            if label=='e7' and not guards(folder,label,guardprefix):
                (out/'rejected.marker').write_text(kind+' encode/resource gate failed\n')
        if (out/'rejected.marker').exists():break
    status.write_text('0\n')
except BaseException:
    status.write_text('1\n');raise
