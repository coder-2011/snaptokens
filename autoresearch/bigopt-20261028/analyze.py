"""Per-cell median paired log ratio (b/a throughput) with bootstrap 95% CI, plus geomean."""
import collections
import json
import math
import random
import sys

records = [json.loads(line) for line in open(sys.argv[1])]
by = collections.defaultdict(dict)
for r in records:
    by[(r["cell"], r["pair"])][r["side"]] = r
cells = collections.defaultdict(list)
problems = 0
for (cell, pair), sides in by.items():
    if len(sides) != 2:
        continue
    a, b = sides["a"], sides["b"]
    if a["hash"] != b["hash"] or a["errors"] or b["errors"]:
        problems += 1
    cells[cell].append((math.log(b["mib_s"] / a["mib_s"]), a["mib_s"], b["mib_s"]))
rng = random.Random(1)


def median(values):
    values = sorted(values)
    n = len(values)
    return values[n // 2] if n % 2 else (values[n // 2 - 1] + values[n // 2]) / 2


logs = []
for cell, rows in sorted(cells.items()):
    ratios = [r[0] for r in rows]
    boot = sorted(median(rng.choices(ratios, k=len(ratios))) for _ in range(4000))
    m = median(ratios)
    logs.append(m)
    print(f"{cell:40s} n={len(rows):2d} a={median([r[1] for r in rows]):8.2f} b={median([r[2] for r in rows]):8.2f} "
          f"ratio={math.exp(m):.4f} [{math.exp(boot[100]):.4f}, {math.exp(boot[3899]):.4f}]")
print(f"geomean {math.exp(sum(logs) / len(logs)):.4f} over {len(logs)} cells; hash/error problems: {problems}")
