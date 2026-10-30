"""Paired A/B runner: fresh process per measurement, same inputs per pair, alternating order."""
import json
import os
import platform
import subprocess
import sys
from pathlib import Path

binaries = {"a": sys.argv[1], "b": sys.argv[2]}
cells = json.loads(Path(sys.argv[3]).read_text())
output = Path(sys.argv[4])
pairs = int(sys.argv[5])
threads = os.environ.get("THREADS", "4")
env = dict(os.environ, RAYON_NUM_THREADS=threads)
prefix = ["taskset", "-c", os.environ["CPU_SET"]] if platform.system() == "Linux" and "CPU_SET" in os.environ else []

with output.open("a") as log:
    for pair in range(pairs):
        for cell in cells:
            order = ["a", "b"] if (pair + cells.index(cell)) % 2 == 0 else ["b", "a"]
            for side in order:
                args = [binaries[side], "bench", cell["tokenizer"], cell["corpus"],
                        str(cell["rows"]), str(cell["row_bytes"]), str(cell["calls"]), str(pair + 1)]
                result = json.loads(subprocess.check_output(prefix + args, env=env, text=True))
                record = dict(pair=pair, cell=cell["name"], side=side, **result)
                log.write(json.dumps(record) + "\n")
                log.flush()
        print(f"pair {pair} done", flush=True)
