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
from score import paired_center

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
    "pairs": 12, "loads_per_process": 6,
    "discarded_initial_loads": {"st": 1, "json": 1, "value": 1, "st-create": 0},
    "timer": "load plus drop; first load recorded separately", "threads": 4,
    "modes": ["st", "json", "value", "st-create"],
    "value_setup": "parse and clone input before timer; construct and drop inside timer",
    "st_create": "fresh JSON copy and absent sidecar per arm; score first load only",
    "center": "median log ratio per model, equal model weights",
    "bootstrap": {"seed": 0, "draws": 3000, "paired_round_indices": True},
    "primary_load_samples": "median of loads 2-6 per process for st/json/value",
    "first_st_samples": "first load per process, including creation, without discarded warmup",
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
                fresh = output / "parity-fresh.json"
                fresh.write_bytes(model.read_bytes())
                fresh.with_suffix(".st").unlink(missing_ok=True)
                result = invoke(binary, "check", fresh, data / "corpus.json")
                log.write(f"{binary}: fresh creation {result}\n")
                log.flush()
    if phase == "post":
        break
    manifest["direct_st_inputs"] = {
        model.with_suffix(".st").name: digest(model.with_suffix(".st"))
        for model in models
    }
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2))
    records = []
    with (output / "rounds.jsonl").open("w") as log:
        for pair in range(12):
            for model in models:
                for mode in ["st", "json", "value", "st-create"]:
                    path = model if mode == "json" else model.with_suffix("." + mode)
                    order = [("parent", parent), ("candidate", candidate)]
                    if pair % 2:
                        order.reverse()
                    for label, binary in order:
                        invocation_mode = mode
                        if mode == "value":
                            path = model
                        elif mode == "st-create":
                            path = output / "fresh.json"
                            path.write_bytes(model.read_bytes())
                            path.with_suffix(".st").unlink(missing_ok=True)
                            invocation_mode = "st"
                        values = [int(v) for v in invoke(binary, invocation_mode, path, 6).splitlines()]
                        assert len(values) == 6
                        record = dict(pair=pair, model=model.stem, mode=mode, label=label, ns=values)
                        records.append(record)
                        log.write(json.dumps(record) + "\n")
                        log.flush()
            print(f"finished pair {pair + 1}/12", flush=True)

summary = {}
for mode in ["st", "json", "value", "st-create"]:
    cells = {}
    for model in models:
        ratios = []
        first_ratios = []
        for pair in range(12):
            rows = {r["label"]: r["ns"] for r in records
                    if r["pair"] == pair and r["model"] == model.stem and r["mode"] == mode}
            ratios.append(statistics.median(rows["parent"][1:]) / statistics.median(rows["candidate"][1:]))
            first_ratios.append(rows["parent"][0] / rows["candidate"][0])
        if mode == "st-create":
            ratios = first_ratios
        cells[model.stem] = {"ratios": ratios, "median": paired_center(ratios),
                             "first_load_median": paired_center(first_ratios)}
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
