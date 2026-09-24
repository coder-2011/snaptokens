#!/usr/bin/env python3
"""Paired fresh-process runner for decode-eval-v1 cells.

Given a cells file and two immutable decode_bench binaries, runs
counterbalanced AB/BA fresh-process pairs per cell, parses summary lines,
and reports per-cell paired ratios plus the equal-weight log-mean aggregate.

With --write-manifests it instead generates each cell's manifest through one
binary (optionally with --hf-verify). Standard library only.
"""

import argparse
import json
import math
import statistics
import subprocess
import sys
from pathlib import Path


def cell_args(cell: dict, manifest_dir: Path) -> list[str]:
    manifest_key = cell.get("manifest_key", cell["name"])
    args = [
        "--tokenizer", cell["tokenizer"],
        "--corpus", cell["corpus"],
        "--manifest", str(manifest_dir / f"{manifest_key}.json"),
        "--row-bytes", str(cell.get("row_bytes", 4096)),
        "--batch", str(cell.get("batch", 1)),
        "--repeat", str(cell.get("repeat", 1)),
        "--rounds", str(cell.get("rounds", 5)),
        "--warmup", str(cell.get("warmup", 1)),
    ]
    if cell.get("max_rows") is not None:
        args += ["--max-rows", str(cell["max_rows"])]
    if cell.get("max_bytes") is not None:
        args += ["--max-bytes", str(cell["max_bytes"])]
    if cell.get("insert_specials"):
        args += ["--insert-specials"]
    args += ["--skip-special", "true" if cell.get("skip_special", True) else "false"]
    return args


def run_bench(
    binary: str, extra: list[str], base: list[str], expect_summary: bool = True
) -> dict:
    process = subprocess.run(
        [binary, *base, *extra], capture_output=True, text=True, check=False
    )
    if process.returncode != 0:
        raise RuntimeError(
            f"bench failed ({binary}): {process.stderr.strip()[-2000:]}"
        )
    summary = None
    for line in process.stdout.splitlines():
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not expect_summary or record.get("summary"):
            summary = record
    if summary is None:
        raise RuntimeError(f"no summary line from {binary}")
    return summary


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cells", required=True, type=Path)
    parser.add_argument("--manifest-dir", required=True, type=Path)
    parser.add_argument("--parent-bin")
    parser.add_argument("--candidate-bin")
    parser.add_argument("--pairs", type=int, default=4)
    parser.add_argument("--only", nargs="*", help="cell name substrings to include")
    parser.add_argument("--skip", nargs="*", help="cell name substrings to exclude")
    parser.add_argument("--out", type=Path)
    parser.add_argument("--write-manifests", action="store_true")
    parser.add_argument("--hf-verify", action="store_true")
    args = parser.parse_args()

    cells = json.loads(args.cells.read_text())
    if args.only:
        cells = [c for c in cells if any(s in c["name"] for s in args.only)]
    if args.skip:
        cells = [c for c in cells if not any(s in c["name"] for s in args.skip)]
    if not cells:
        raise SystemExit("no cells selected")
    args.manifest_dir.mkdir(parents=True, exist_ok=True)

    if args.write_manifests:
        binary = args.parent_bin or args.candidate_bin
        for cell in cells:
            extra = ["--write-manifest"] + (["--hf-verify"] if args.hf_verify else [])
            summary = run_bench(
                binary, extra, cell_args(cell, args.manifest_dir), expect_summary=False
            )
            print(f"manifest {cell['name']}: rows={summary['rows']} "
                  f"tokens={summary['total_tokens']}", flush=True)
        return

    results = {"cells": {}, "pairs": args.pairs}
    all_log_ratios = []
    for cell in cells:
        base = cell_args(cell, args.manifest_dir)
        pair_ratios = []
        rounds = {"parent": [], "candidate": []}
        for pair in range(args.pairs):
            order = (
                [("parent", args.parent_bin), ("candidate", args.candidate_bin)]
                if pair % 2 == 0
                else [("candidate", args.candidate_bin), ("parent", args.parent_bin)]
            )
            medians = {}
            for role, binary in order:
                summary = run_bench(binary, ["--time"], base)
                if summary["parity"] != "pass":
                    raise RuntimeError(f"parity failure in {cell['name']} ({role})")
                medians[role] = summary["median_seconds"]
                rounds[role].append(summary["round_seconds"])
            ratio = medians["parent"] / medians["candidate"]
            pair_ratios.append(ratio)
            print(f"{cell['name']} pair {pair}: {ratio:.6f}x", flush=True)
        cell_median = statistics.median(pair_ratios)
        results["cells"][cell["name"]] = {
            "pair_ratios": pair_ratios,
            "median_ratio": cell_median,
            "rounds": rounds,
        }
        all_log_ratios.extend(math.log(r) for r in pair_ratios)
        print(f"{cell['name']}: median {cell_median:.6f}x", flush=True)

    cell_medians = [c["median_ratio"] for c in results["cells"].values()]
    aggregate = math.exp(sum(math.log(m) for m in cell_medians) / len(cell_medians))
    mean = statistics.mean(all_log_ratios)
    if len(all_log_ratios) > 1:
        half = 1.96 * statistics.stdev(all_log_ratios) / math.sqrt(len(all_log_ratios))
    else:
        half = float("nan")
    results["aggregate_geomean"] = aggregate
    results["pooled_log_ci"] = [math.exp(mean - half), math.exp(mean + half)]
    print(f"aggregate: {aggregate:.6f}x  pooled CI "
          f"[{results['pooled_log_ci'][0]:.6f}, {results['pooled_log_ci'][1]:.6f}]")
    if args.out:
        args.out.write_text(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
