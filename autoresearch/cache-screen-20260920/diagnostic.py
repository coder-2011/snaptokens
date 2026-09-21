from pathlib import Path
import subprocess
subprocess.run(['git','switch','-c','perf/cache-counts-20260920','1183b2564971ed49e7d14a1a7a1f3350a4b63751'],check=True)
p=Path('log.md')
with p.open('a') as f:
 f.write('''\n### Shared-cache access diagnostic (not a timing candidate)\n\nParent SHA: 1183b2564971ed49e7d14a1a7a1f3350a4b63751.\nHypothesis: Current direct fused BPE inputs bypass SharedCache, while ordinary Gemma BPE reaches it after local misses.\nMeasured hot cost: Attribute shared-cache access frequency; source passes false in EncodeStream, unlike the older historical profiler.\nInvariant that makes the shorter path exact: No shortcut is introduced; counters observe gets, successful hits and insertions.\nRepresentation being preserved or changed: Add three Relaxed counters per cache and emit counts at destruction.\nExpected winning strata: None; diagnostic timings are discarded.\nExpected adverse strata: Counter overhead; never compare its runtime with candidates.\nSmallest files that need changing: src/models/bpe.rs.\nMechanism evidence: Source-audited call sites plus exactness and emitted access counts.\nAcceptance rule: Counts establish which measured configurations call the shared cache.\nRejection rule: Any output mismatch invalidates diagnostic evidence; never promote instrumented source.\n''')
subprocess.run(['git','add','log.md'],check=True)
subprocess.run(['git','commit','-m','record shared-cache access diagnostic'],check=True)
p=Path('src/models/bpe.rs');s=p.read_text()
s=s.replace('''struct SharedCache {
    shards:''','''struct SharedCache {
    diagnostic_gets: AtomicUsize,
    diagnostic_hits: AtomicUsize,
    diagnostic_inserts: AtomicUsize,
    shards:''')
s=s.replace('''impl SharedCache {
    fn new() -> Self {
        Self {
            shards:''','''impl SharedCache {
    fn new() -> Self {
        Self {
            diagnostic_gets: AtomicUsize::new(0),
            diagnostic_hits: AtomicUsize::new(0),
            diagnostic_inserts: AtomicUsize::new(0),
            shards:''')
s=s.replace('''        let shard = self.shards[Self::shard_index(key)].lock().unwrap();''','''        self.diagnostic_gets.fetch_add(1, Ordering::Relaxed);
        let shard = self.shards[Self::shard_index(key)].lock().unwrap();''')
s=s.replace('''        if let Some(ids) = shard.get(key) {
            out.extend_from_slice(ids);''','''        if let Some(ids) = shard.get(key) {
            self.diagnostic_hits.fetch_add(1, Ordering::Relaxed);
            out.extend_from_slice(ids);''')
s=s.replace('''    fn insert(&self, key: String, value: Vec<u32>) {
        let mut shard''','''    fn insert(&self, key: String, value: Vec<u32>) {
        self.diagnostic_inserts.fetch_add(1, Ordering::Relaxed);
        let mut shard''')
needle='''#[derive(Deserialize)]
struct RawBpe'''
s=s.replace(needle,'''impl Drop for SharedCache {
    fn drop(&mut self) {
        eprintln!("shared_cache gets={} hits={} inserts={}",
            self.diagnostic_gets.load(Ordering::Relaxed),
            self.diagnostic_hits.load(Ordering::Relaxed),
            self.diagnostic_inserts.load(Ordering::Relaxed));
    }
}

'''+needle)
p.write_text(s)
subprocess.run(['cargo','fmt','--all'],check=True)
subprocess.run(['git','add','src/models/bpe.rs'],check=True)
subprocess.run(['git','commit','-m','measure shared-cache access counts'],check=True)
root=Path('/tmp/snaptokens-cache-evidence-20260920')
(root/'raw/diagnostic-source.txt').write_text(subprocess.check_output(['git','rev-parse','HEAD'],text=True)+subprocess.check_output(['git','diff','1183b256','--','src'],text=True))
