"""Descriptive cross-format ratios from completed pools, not promotion scoring."""
import hashlib
import json
import math
import statistics
from pathlib import Path

root = Path(__file__).resolve().parent
results = {
    "purpose": "Describe snapshot-format loading benefit separately from incremental optimization",
    "source_runtime": "f78689974948e282e581fda603a6dfbbcf3e2b1c",
    "timing_source": "204aae80ae16190c0dccd03f57a54ea24bdccb89",
    "method": "Within each model and round, divide candidate JSON or TKZ load time by candidate ST load time. Warm time is the median of loads 2-6. Take exp(median(log(ratios))) over 12 rounds, then equal-model geometric mean over 12 models.",
    "limitations": "Post-hoc descriptive analysis of fixed ST/TKZ/JSON order. No cross-format randomized-order confidence or promotion claim. First load means first in each fresh process, not cold disk. Load/drop only, not encoding.",
    "hosts": {},
}
for host in ("apple", "amd", "intel"):
    folder = root / f"integrated-portable-{host}"
    if host != "apple":
        folder /= "integrated-portable"
    path = folder / "load/rounds.jsonl"
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    assert len(rows) == 864
    cells = {}
    for row in rows:
        if row["label"] == "candidate":
            cells.setdefault(row["model"], {}).setdefault(row["pair"], {})[row["mode"]] = row["ns"]
    assert len(cells) == 12
    summaries = {}
    for reference in ("json", "tkz"):
        for phase in ("warm", "first"):
            values = {}
            for model, pairs in cells.items():
                assert set(pairs) == set(range(12))
                ratios = []
                for pair in pairs.values():
                    assert set(pair) == {"st", "tkz", "json"}
                    if phase == "warm":
                        ratio = statistics.median(pair[reference][1:]) / statistics.median(pair["st"][1:])
                    else:
                        ratio = pair[reference][0] / pair["st"][0]
                    ratios.append(ratio)
                values[model] = {"ratios": ratios, "point": math.exp(statistics.median(math.log(ratio) for ratio in ratios))}
            summaries[f"{reference}_over_st_{phase}"] = {
                "point": math.exp(statistics.mean(math.log(value["point"]) for value in values.values())),
                "models": values,
            }
    results["hosts"][host] = {"source": str(path.relative_to(root)), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "comparisons": summaries}
output = root / "format-comparison.json"
output.write_text(json.dumps(results, indent=2) + "\n")
print(json.dumps({host: {name: values["point"] for name, values in data["comparisons"].items()} for host, data in results["hosts"].items()}, indent=2))
