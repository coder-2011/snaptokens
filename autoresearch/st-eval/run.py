"""Frozen scoped load screen: complete-ID gates, fresh processes, balanced pairs."""
import hashlib
import json
import math
import os
from pathlib import Path
import random
import statistics
import subprocess
import sys

parent, candidate, data, output = map(Path, sys.argv[1:5])
output.mkdir(parents=True, exist_ok=False)
models = sorted(data.glob("*.json"))
models = [p for p in models if p.name != "corpus.json"]
env = dict(os.environ, RAYON_NUM_THREADS="4")


def invoke(binary, mode, path, last):
    return subprocess.check_output(
        [str(binary), mode, str(path), str(last)], env=env, text=True, stderr=subprocess.STDOUT
    ).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


manifest = {
    "binaries": {str(p): digest(p) for p in [parent, candidate]},
    "inputs": {p.name: digest(p) for p in [*models, data / "corpus.json"]},
    "pairs": 12, "loads_per_process": 6, "warmup_loads": 1,
    "timer": "load plus drop; first load recorded separately", "threads": 4,
    "host": os.uname().nodename,
}
(output / "manifest.json").write_text(json.dumps(manifest, indent=2))
for phase in ["pre", "post"]:
    with (output / f"{phase}-exactness.txt").open("w") as log:
        for binary in [parent, candidate]:
            for model in models:
                result = invoke(binary, "check", model, data / "corpus.json")
                log.write(f"{binary}: {result}\n")
                log.flush()
    if phase == "post":
        break
    records = []
    with (output / "rounds.jsonl").open("w") as log:
        for pair in range(12):
            for model in models:
                for mode in ["st", "tkz", "json"]:
                    path = model if mode == "json" else model.with_suffix("." + mode)
                    order = [("parent", parent), ("candidate", candidate)]
                    if pair % 2:
                        order.reverse()
                    for label, binary in order:
                        values = [int(v) for v in invoke(binary, mode, path, 6).splitlines()]
                        assert len(values) == 6
                        record = dict(pair=pair, model=model.stem, mode=mode, label=label, ns=values)
                        records.append(record)
                        log.write(json.dumps(record) + "\n")
                        log.flush()
            print(f"finished pair {pair + 1}/12", flush=True)

summary = {}
for mode in ["st", "tkz", "json"]:
    cells = {}
    for model in models:
        ratios = []
        first_ratios = []
        for pair in range(12):
            rows = {r["label"]: r["ns"] for r in records
                    if r["pair"] == pair and r["model"] == model.stem and r["mode"] == mode}
            ratios.append(statistics.median(rows["parent"][1:]) / statistics.median(rows["candidate"][1:]))
            first_ratios.append(rows["parent"][0] / rows["candidate"][0])
        cells[model.stem] = {"ratios": ratios, "median": statistics.median(ratios),
                             "first_load_median": statistics.median(first_ratios)}
    point = math.exp(statistics.mean(math.log(c["median"]) for c in cells.values()))
    rng = random.Random(0)
    bootstrap = []
    for _ in range(3000):
        indices = rng.choices(range(12), k=12)
        bootstrap.append(math.exp(statistics.mean(
            statistics.median(math.log(c["ratios"][i]) for i in indices) for c in cells.values())))
    bootstrap.sort()
    summary[mode] = dict(point=point, ci95=[bootstrap[75], bootstrap[2924]], cells=cells)
(output / "summary.json").write_text(json.dumps(summary, indent=2))
print(json.dumps(summary, indent=2))
