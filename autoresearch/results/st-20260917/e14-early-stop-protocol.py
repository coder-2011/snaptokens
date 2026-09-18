import datetime, json, os, signal, subprocess, sys, time
from pathlib import Path

root = Path(sys.argv[1])
out = root / 'evidence/e14-portable'
status = root / 'evidence/e14-portable.status'
assert not status.exists(), 'Do not interrupt a completed run'
assert (out / 'bpe/calibration.json').exists(), 'Expected verified baseline reuse before candidate timing'

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
         and command.split()[-1].endswith('/st-e14-portable-20260917.py')]
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

record = {'reason': 'E14 rejected by completed Apple twelve-pair BPE load gate; skip later gates and stop other CPU pools without scoring partial data',
          'apple_point': 1.0878624132367933,
          'apple_ci95': [1.081973487229623, 1.0962103754199948],
          'apple_failures': ['Gemma JSON 0.9628975773 below 0.9749916877', 'Mistral Nemo JSON 0.9793839849 below 0.9832396316'],
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
(out / 'rejected.marker').write_text('Rejected by complete Apple load gate; this host stopped early; see early-stop.json\n')
status.write_text('0\n')
print(json.dumps(record, indent=2))
