import datetime, json, os, signal, subprocess, sys, time
from pathlib import Path

root = Path(sys.argv[1])
out = root / 'evidence/e7-portable'
status = root / 'evidence/e7-portable.status'
assert not status.exists(), 'Do not interrupt a completed run'

def processes():
    result = {}
    for line in subprocess.check_output(['ps', '-axo', 'pid,ppid,stat,args'], text=True).splitlines()[1:]:
        fields = line.strip().split(None, 3)
        if len(fields) == 4:
            result[int(fields[0])] = (int(fields[1]), fields[2], fields[3])
    return result

table = processes()
roots = [pid for pid, (_, _, command) in table.items()
         if 'python' in command.split()[0].lower()
         and command.split()[-1].endswith('/st-e7-portable-20260917.py')]
assert len(roots) == 1, roots
stopped = set()
pending = roots[:]
while pending:
    pid = pending.pop()
    try:
        os.kill(pid, signal.SIGSTOP)
    except ProcessLookupError:
        continue
    stopped.add(pid)
    table = processes()
    pending.extend(child for child, (parent, _, _) in table.items()
                   if parent == pid and child not in stopped)

record = {'reason': 'E7 rejected by completed Intel twelve-pair BPE load gate; skip later gates and stop other CPU pools without scoring partial data',
          'intel_point': 0.993212061560974,
          'intel_ci95': [0.9715954839223405, 1.0331366361175331],
          'intel_failures': ['GPT-OSS ST 0.7398057637 below 0.9246390221', 'Nemotron ST 0.7015543912 below 0.9381451338'],
          'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'stopped_task_processes': {str(pid): table.get(pid) for pid in sorted(stopped)},
          'partial_results': 'Preserved as incomplete; no acceptance score or post-pool exactness claim'}
(out / 'early-stop.json').write_text(json.dumps(record, indent=2) + '\n')
for pid in stopped:
    try:
        os.kill(pid, signal.SIGTERM)
        os.kill(pid, signal.SIGCONT)
    except ProcessLookupError:
        pass
for _ in range(100):
    table = processes()
    live = [pid for pid in stopped if pid in table and not table[pid][1].startswith('Z')]
    if not live:
        break
    time.sleep(0.1)
assert not live, live
(out / 'rejected.marker').write_text('Rejected by complete Intel load gate; this host stopped early; see early-stop.json\n')
status.write_text('0\n')
print(json.dumps(record, indent=2))
