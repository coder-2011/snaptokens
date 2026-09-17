import hashlib,json,os,shutil,subprocess,sys
from pathlib import Path
root=Path(os.environ.get('ST_TASK_ROOT',Path.home()/'st-campaign-20260917'))
ev=root/'evaluator-v2'; evidence=root/'evidence';out=evidence/'integrated-portable';status=evidence/'integrated-portable.status'
data=Path(os.environ.get('ST_BPE_DATA',root/'data'))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(args,name):
    with (out/name).open('w') as f:subprocess.run(list(map(str,args)),stdout=f,stderr=subprocess.STDOUT,check=True)
try:
    out.mkdir(); assert shutil.disk_usage(root).free>=3*1024**3
    assert (evidence/'e14-portable.status').read_text().strip()=='0'
    frozen=json.loads((evidence/'v2-aa-identical/manifest.json').read_text())
    assert len(frozen['binaries'])==1
    parent=Path(next(iter(frozen['binaries'])))
    assert digest(parent)==frozen['binaries'][str(parent)]
    candidate=evidence/'e13-portable/binaries/parent-bpe'
    identity=json.loads((evidence/'e13-portable/parent-identity.json').read_text())
    assert identity['sha']=='204aae80ae16190c0dccd03f57a54ea24bdccb89'
    assert digest(candidate)==identity['binaries']['parent-bpe']
    guard=json.loads((evidence/'v2-guard-aa-identical/manifest.json').read_text())
    assert digest(ev/'run.py')==frozen['runner_sha256']
    assert digest(ev/'guards.py')==guard['runner_sha256']
    assert digest(ev/'scoring.py')==frozen['scoring_sha256']==guard['scoring_sha256']
    assert frozen['inputs']==guard['inputs']
    for name,h in frozen['inputs'].items(): assert digest(data/name)==h,name
    assert {p.name for p in data.glob('*.json') if p.name!='panel.json'}==set(frozen['inputs'])
    if sys.platform=='linux':
        for label,binary in [('parent',parent),('combined',candidate)]:
            metadata=subprocess.check_output(['readelf','-p','.comment',binary],text=True)
            assert 'rustc version 1.98.1 ' in metadata
            (out/(label+'-compiler.txt')).write_text(metadata)
    calibration=evidence/'v2-calibration.json'; c=json.loads(calibration.read_text()); assert c['minimum_point']==1.02
    shutil.copy2(calibration,out/'calibration.json')
    (out/'identity.json').write_text(json.dumps({'purpose':'Complete characterization of user-retained E8 plus requested Unigram support; no optimization promotion or rescue of rejected candidates','parent_sha':'95bc1acf46d7dee9dc7ea0d8e23715ad2d59ff71','combined_sha':identity['sha'],'root_runtime_sha':'f78689974948e282e581fda603a6dfbbcf3e2b1c','binaries':{str(p):digest(p) for p in [parent,candidate]},'calibration_sha256':digest(calibration),'source_identity':identity},indent=2)+'\n')
    args=[parent,candidate,data]
    run(['python3',ev/'run.py',*args,out/'load'],'load.log')
    s=json.loads((out/'load/summary.json').read_text());fail=[]
    if s['st']['point']<1.02 or s['st']['ci95'][0]<=1:fail.append(['primary',s['st']['point'],s['st']['ci95']])
    for mode,cells in c['load'].items():
        for model,band in cells.items():
            value=s[mode]['cells'][model]['median']
            if value<band[0]: fail.append([mode,model,value,band[0]])
    (out/'load-gate.json').write_text(json.dumps({'pass':not fail,'failures':fail},indent=2)+'\n')
    # The tree is user-authorized feature work; characterize all guards even if it is not a speed win.
    run(['python3',ev/'guards.py',*args,out/'guards'],'guards.log')
    s=json.loads((out/'guards/summary.json').read_text());fail=[]
    for mode,cells in c['encode'].items():
        for model,band in cells.items():
            value=s[mode]['cells'][model]['median']
            if value<band[0]:fail.append([mode,model,value,band[0]])
    for model,cell in s['rss']['cells'].items():
        if cell['median']>1.05:fail.append(['rss',model,cell['median'],1.05])
    ratio=candidate.stat().st_size/parent.stat().st_size
    if ratio>1.02:fail.append(['binary',ratio,1.02])
    (out/'guard-gate.json').write_text(json.dumps({'pass':not fail,'failures':fail,'binary_ratio':ratio},indent=2)+'\n')
    status.write_text('0\n')
except BaseException:
    status.write_text('1\n');raise
