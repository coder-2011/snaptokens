import hashlib,json,pathlib,subprocess,sys
name=sys.argv[1]; root=pathlib.Path('/tmp/snaptokens-cache-evidence-20260920')
if name!='atomic':
 subprocess.run(['git','switch','-c','perf/cache-'+name+'-20260920','1183b2564971ed49e7d14a1a7a1f3350a4b63751'],check=True)
 p=pathlib.Path('src/models/bpe.rs');s=p.read_text()
 if name=='boxed':
  old='shards: Vec<Mutex<FxHashMap<String, Vec<u32>>>>';new='shards: Vec<Mutex<FxHashMap<Box<str>, Vec<u32>>>>';assert old in s;s=s.replace(old,new)
  old='        shard.insert(key, value);';assert old in s;s=s.replace(old,'        shard.insert(key.into_boxed_str(), value);')
 elif name=='long-standard':
  old='long: FxHashMap<Box<str>, (u32, u16)>';assert old in s;s=s.replace(old,'long: HashMap<Box<str>, (u32, u16)>');s=s.replace('long: FxHashMap::default()','long: HashMap::new()')
 elif name=='shared-standard':
  old='shards: Vec<Mutex<FxHashMap<String, Vec<u32>>>>';assert old in s;s=s.replace(old,'shards: Vec<Mutex<HashMap<String, Vec<u32>>>>');s=s.replace('Mutex::new(FxHashMap::default())','Mutex::new(HashMap::new())')
 else:raise ValueError(name)
 p.write_text(s)
 subprocess.run(['cargo','fmt','--all'],check=True)
 subprocess.run(['git','add','src/models/bpe.rs'],check=True)
 subprocess.run(['git','commit','-m','experiment with '+name+' cache representation'],check=True)
else:
 subprocess.run(['git','switch','perf/cache-atomic-20260920'],check=True)
assert not subprocess.check_output(['git','status','--porcelain'],text=True)
(root/'raw'/(name+'-source.txt')).write_text(subprocess.check_output(['git','rev-parse','HEAD'],text=True)+subprocess.check_output(['git','diff','1183b256','--','src'],text=True))
