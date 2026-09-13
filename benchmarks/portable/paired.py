#!/usr/bin/env python3
"""Run and summarize counterbalanced pairs of immutable portable binaries."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import os
import random
import shutil
import statistics
import subprocess
import tempfile
from collections import defaultdict
from pathlib import Path


Cell = tuple[str, str, str, str, int, int]
Measurement = tuple[int, int, int, int]
Block = dict[Cell, list[float]]


def parse_args() -> argparse.Namespace:
    """Parse one explicit paired portable-benchmark schedule."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--parent", required=True, type=Path)
    parser.add_argument("--candidate", required=True, type=Path)
    parser.add_argument("--tokenizers", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument(
        "--mode",
        required=True,
        choices=("identical-aa", "independent-aa", "candidate"),
    )
    parser.add_argument("--cycles", type=int, default=1)
    return parser.parse_args()


def sha256(path: Path) -> str:
    """Hash one immutable benchmark input or executable."""
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def checked_path(path: Path, label: str, executable: bool = False) -> Path:
    """Resolve and validate one required benchmark path."""
    resolved = path.resolve()
    if not resolved.exists():
        raise SystemExit(f"{label} does not exist: {resolved}")
    if executable and not os.access(resolved, os.X_OK):
        raise SystemExit(f"{label} is not executable: {resolved}")
    return resolved


def write_json(path: Path, value: object) -> None:
    """Atomically publish one evaluator artifact."""
    temporary = path.with_suffix(path.suffix + ".tmp")
    contents = json.dumps(value, indent=2, sort_keys=True) + "\n"
    temporary.write_text(contents, encoding="utf-8")
    temporary.replace(path)


def evaluator_identity() -> dict[str, object]:
    """Hash every frozen file that owns portable timing or paired scoring."""
    portable = Path(__file__).resolve().parent
    repository = portable.parent.parent
    files = [
        portable / "Cargo.toml",
        portable / "Cargo.lock",
        portable / "src" / "main.rs",
        portable / "paired.py",
        portable.parent / "shared.rs",
    ]
    entries = [
        {"path": str(path.relative_to(repository)), "sha256": sha256(path)}
        for path in files
    ]
    digest = hashlib.sha256()
    for entry in entries:
        digest.update(str(entry["path"]).encode())
        digest.update(str(entry["sha256"]).encode())
    return {"version": "portable-paired-v2", "sha256": digest.hexdigest(), "files": entries}


def tokenizer_manifest(directory: Path) -> list[dict[str, object]]:
    """Freeze the sorted JSON tokenizer inventory without accepting sidecars."""
    files = sorted(directory.glob("*.json"))
    if not files:
        raise SystemExit(f"no tokenizer JSON files found: {directory}")
    return [
        {"name": path.name, "bytes": path.stat().st_size, "sha256": sha256(path)}
        for path in files
    ]


def run_schedule(args: argparse.Namespace) -> list[dict[str, object]]:
    """Execute complete portable matrices in repeated AB and BA process order."""
    if args.cycles <= 0:
        raise SystemExit("--cycles must be positive")

    parent = checked_path(args.parent, "parent binary", executable=True)
    candidate = checked_path(args.candidate, "candidate binary", executable=True)
    tokenizers = checked_path(args.tokenizers, "tokenizer directory")
    corpus = checked_path(args.corpus, "corpus")
    if not tokenizers.is_dir():
        raise SystemExit(f"tokenizer path is not a directory: {tokenizers}")

    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    if any(output.iterdir()):
        raise SystemExit(f"output directory must be empty: {output}")

    binaries = {"parent": parent, "candidate": candidate}
    binary_hashes = {role: sha256(path) for role, path in binaries.items()}
    frozen_tokenizers = tokenizer_manifest(tokenizers)
    corpus_sha256 = sha256(corpus)
    evaluator = evaluator_identity()
    cargo_lock_sha256 = next(
        entry["sha256"]
        for entry in evaluator["files"]
        if entry["path"] == "benchmarks/portable/Cargo.lock"
    )
    runs: list[dict[str, object]] = []
    for cycle in range(args.cycles):
        for order, roles in (("ab", ("parent", "candidate")), ("ba", ("candidate", "parent"))):
            for position, role in enumerate(roles):
                result = output / f"cycle-{cycle:03d}-{order}-{position}-{role}.jsonl"
                runs.append(
                    {
                        "cycle": cycle,
                        "order": order,
                        "position": position,
                        "role": role,
                        "binary": str(binaries[role]),
                        "binary_sha256": binary_hashes[role],
                        "output": str(result),
                    }
                )

    # One prior process per binary proves exactness without warming any scored process.
    preflights = []
    seen_hashes = set()
    for role in ("parent", "candidate"):
        binary_sha256 = binary_hashes[role]
        if binary_sha256 in seen_hashes:
            continue
        seen_hashes.add(binary_sha256)
        preflights.append(
            {
                "role": role,
                "binary": str(binaries[role]),
                "binary_sha256": binary_sha256,
                "output": str(output / f"preflight-{role}.jsonl"),
            }
        )

    controls = {
        name: os.environ.get(name)
        for name in (
            "SNAP_THREADS",
            "SNAP_LOAD_ROUNDS",
            "SNAP_ENCODE_ROUNDS",
            "SNAP_SAMPLE_MIB",
            "SNAP_HOST",
            "SNAP_CPU",
        )
    }
    manifest = {
        "evaluator": evaluator,
        "mode": args.mode,
        "cycles": args.cycles,
        "parent_binary": str(parent),
        "parent_sha256": binary_hashes["parent"],
        "candidate_binary": str(candidate),
        "candidate_sha256": binary_hashes["candidate"],
        "tokenizer_dir": str(tokenizers),
        "tokenizers": frozen_tokenizers,
        "corpus": str(corpus),
        "corpus_sha256": corpus_sha256,
        "controls": controls,
        "exactness": (
            "one unscored fresh-process matrix verifies every deterministic pool before scored "
            "runs; each scored process verifies its pools after timing"
        ),
        "preflights": preflights,
        "schedule": runs,
    }
    write_json(output / "manifest.json", manifest)
    failures: list[dict[str, object]] = []
    # Optional competitor eligibility must not change Snaptokens' inner position.
    frozen_schedule = None

    for run_index, run in enumerate([*preflights, *runs]):
        if sha256(corpus) != corpus_sha256 or tokenizer_manifest(tokenizers) != frozen_tokenizers:
            raise SystemExit("benchmark inputs changed after the manifest was written")
        environment = os.environ.copy()
        environment["SNAP_RUN_ID"] = f"paired-{run_index}-{run['role']}"
        # Each treatment gets JSON-only inputs because the runner creates and
        # removes native sidecars while measuring conversion and load behavior.
        with tempfile.TemporaryDirectory(prefix="paired-inputs-", dir=output) as scratch:
            scratch_path = Path(scratch)
            for tokenizer in sorted(tokenizers.glob("*.json")):
                shutil.copy2(tokenizer, scratch_path / tokenizer.name)
            command = [
                str(run["binary"]),
                "run",
                str(scratch_path),
                str(corpus),
                str(run["output"]),
            ]
            try:
                subprocess.run(command, check=True, env=environment)
            except subprocess.CalledProcessError as error:
                failures.append({**run, "returncode": error.returncode})
                write_json(output / "failures.json", failures)
                raise
        host, measurements = read_run(Path(str(run["output"])))
        verify_run_inputs(host, measurements, corpus_sha256, frozen_tokenizers, run)
        schedule = schedule_inventory(measurements)
        if frozen_schedule is None:
            frozen_schedule = schedule
        elif schedule != frozen_schedule:
            raise SystemExit(f"{run['role']} internal schedule differs from the preflight")
        if host.get("evaluator_sha256") != evaluator["sha256"]:
            raise SystemExit(f"{run['role']} binary was built for a different evaluator")
        if host.get("cargo_lock_sha256") != cargo_lock_sha256:
            raise SystemExit(f"{run['role']} binary was built with a different Cargo.lock")
    return runs


def read_run(path: Path) -> tuple[dict[str, object], dict[Cell, dict[int, Measurement]]]:
    """Load one raw run and retain exact Snaptokens measurements by cell and round."""
    host: dict[str, object] | None = None
    measurements: dict[Cell, dict[int, Measurement]] = defaultdict(dict)
    with path.open() as source:
        for line in source:
            row = json.loads(line)
            if row.get("kind") == "host":
                host = row
            if row.get("kind") != "encode_measurement":
                continue
            if row.get("implementation") != "snaptokens-json":
                continue
            if row.get("timed_inputs_exact") is not True:
                raise SystemExit(f"measurement lacks exactness evidence: {path}")
            cell = (
                row["model"],
                row["tokenizer_sha256"],
                row["contract"],
                row["shape"],
                int(row["input_bytes"]),
                int(row["batch"]),
            )
            round_index = int(row["round"])
            if round_index in measurements[cell]:
                raise SystemExit(f"duplicate measurement for {cell} round={round_index}: {path}")
            measurements[cell][round_index] = (
                int(row["elapsed_ns"]),
                int(row["total_bytes"]),
                int(row["position"]),
                int(row["actual_rounds"]),
            )

    if host is None or not measurements:
        raise SystemExit(f"run is missing host metadata or measurements: {path}")
    return host, measurements


def verify_run_inputs(
    host: dict[str, object],
    measurements: dict[Cell, dict[int, Measurement]],
    corpus_sha256: str,
    tokenizers: list[dict[str, object]],
    run: dict[str, object],
) -> None:
    """Bind one completed run to its executable and initial input manifest."""
    if host.get("binary_sha256") != run["binary_sha256"]:
        raise SystemExit(f"{run['role']} binary hash differs from the schedule")
    if host.get("corpus_sha256") != corpus_sha256:
        raise SystemExit(f"{run['role']} corpus hash differs from the manifest")
    expected_tokenizers = {
        (Path(entry["name"]).stem, entry["sha256"]) for entry in tokenizers
    }
    observed_tokenizers = {(cell[0], cell[1]) for cell in measurements}
    if observed_tokenizers != expected_tokenizers:
        raise SystemExit(f"{run['role']} tokenizer inventory differs from the manifest")


def schedule_inventory(
    measurements: dict[Cell, dict[int, Measurement]],
) -> dict[Cell, dict[int, tuple[int, int]]]:
    """Retain the inner position and round count that make timed rows comparable."""
    return {
        cell: {
            round_index: (measurement[2], measurement[3])
            for round_index, measurement in rounds.items()
        }
        for cell, rounds in measurements.items()
    }


def cell_value(cell: Cell) -> dict[str, object]:
    """Convert one stable cell key into JSON fields."""
    model, tokenizer_sha256, contract, shape, input_bytes, batch = cell
    return {
        "model": model,
        "tokenizer_sha256": tokenizer_sha256,
        "contract": contract,
        "shape": shape,
        "input_bytes": input_bytes,
        "batch": batch,
    }


def percentile(values: list[float], probability: float) -> float:
    """Interpolate one percentile from a non-empty sorted bootstrap sample."""
    ordered = sorted(values)
    index = (len(ordered) - 1) * probability
    lower = math.floor(index)
    upper = math.ceil(index)
    if lower == upper:
        return ordered[lower]
    fraction = index - lower
    return ordered[lower] * (1.0 - fraction) + ordered[upper] * fraction


def block_score(blocks: list[Block]) -> float:
    """Score selected whole-process blocks with the declared equal-cell estimator."""
    ratios: dict[Cell, list[float]] = defaultdict(list)
    for block in blocks:
        for cell, logs in block.items():
            ratios[cell].extend(logs)
    cell_logs = [statistics.median(logs) for logs in ratios.values()]
    return math.exp(statistics.fmean(cell_logs))


def bootstrap_interval(
    blocks: list[Block], samples: int = 10_000
) -> tuple[float | None, float | None]:
    """Resample complete AB-plus-BA cycles so order remains counterbalanced."""
    if len(blocks) < 2:
        return None, None
    generator = random.Random(0)
    scores = []
    for _ in range(samples):
        selected = [generator.choice(blocks) for _ in blocks]
        scores.append(block_score(selected))
    return percentile(scores, 0.025), percentile(scores, 0.975)


def validate_mode(mode: str, builds: dict[str, dict[str, object]]) -> None:
    """Reject build relationships that do not match the declared comparison mode."""
    parent = builds["parent"]
    candidate = builds["candidate"]
    for role, build in builds.items():
        source_commit = build["source_commit"]
        if not isinstance(source_commit, str) or len(source_commit) != 40:
            raise SystemExit(f"{role} source commit is not a full SHA")
        if any(character not in "0123456789abcdef" for character in source_commit):
            raise SystemExit(f"{role} source commit is not a full SHA")
    if mode.endswith("-aa") and parent["source_commit"] != candidate["source_commit"]:
        raise SystemExit(f"{mode} requires one source commit")
    if mode == "identical-aa" and parent["binary_sha256"] != candidate["binary_sha256"]:
        raise SystemExit("identical-aa requires one binary")
    if mode == "independent-aa" and parent["build_id"] == candidate["build_id"]:
        raise SystemExit("independent-aa requires distinct build identities")


def write_pairs(path: Path, rows: list[dict[str, object]]) -> None:
    """Atomically publish every matched duration used by the scorer."""
    fields = [
        "cycle",
        "order",
        "round",
        "model",
        "tokenizer_sha256",
        "contract",
        "shape",
        "input_bytes",
        "batch",
        "parent_elapsed_ns",
        "candidate_elapsed_ns",
        "total_bytes",
        "ratio",
        "log_ratio",
    ]
    temporary = path.with_suffix(path.suffix + ".tmp")
    with temporary.open("w", newline="", encoding="utf-8") as destination:
        writer = csv.DictWriter(destination, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(path)


def summarize(runs: list[dict[str, object]], output: Path) -> dict[str, object]:
    """Pair matching rounds and summarize observed candidate-to-parent throughput ratios."""
    manifest = json.loads((output / "manifest.json").read_text(encoding="utf-8"))
    if manifest["evaluator"] != evaluator_identity():
        raise SystemExit("the current evaluator differs from the run manifest")
    expected_corpus = manifest["corpus_sha256"]
    expected_tokenizers = manifest["tokenizers"]
    expected_evaluator = manifest["evaluator"]["sha256"]
    expected_cargo_lock = next(
        entry["sha256"]
        for entry in manifest["evaluator"]["files"]
        if entry["path"] == "benchmarks/portable/Cargo.lock"
    )
    grouped: dict[tuple[int, str], dict[str, dict[str, object]]] = defaultdict(dict)
    for run in runs:
        key = (int(run["cycle"]), str(run["order"]))
        grouped[key][str(run["role"])] = run
    expected_groups = {
        (cycle, order)
        for cycle in range(int(manifest["cycles"]))
        for order in ("ab", "ba")
    }
    if set(grouped) != expected_groups:
        raise SystemExit("scored schedule does not contain every declared AB and BA pair")

    logs_by_cell: dict[Cell, list[float]] = defaultdict(list)
    cycle_blocks: dict[int, Block] = {}
    pair_rows: list[dict[str, object]] = []
    pair_aggregates: list[dict[str, object]] = []
    builds: dict[str, dict[str, object]] = {}
    cell_inventory: set[Cell] | None = None
    host_environment: dict[str, object] | None = None
    common_host_fields = (
        "host",
        "cpu",
        "os",
        "arch",
        "logical_cpus",
        "rayon_threads",
        "rustc_version",
        "rustflags",
        "corpus_sha256",
        "tokenizers",
        "load_rounds_requested",
        "encode_rounds_requested",
        "sample_mib_per_engine_round",
        "evaluator_sha256",
        "cargo_lock_sha256",
    )
    for (cycle, order), pair in sorted(grouped.items()):
        if set(pair) != {"parent", "candidate"}:
            raise SystemExit(f"incomplete pair: cycle={cycle} order={order}")
        parent_host, parent = read_run(Path(str(pair["parent"]["output"])))
        candidate_host, candidate = read_run(Path(str(pair["candidate"]["output"])))
        for role, host in (("parent", parent_host), ("candidate", candidate_host)):
            environment = {field: host.get(field) for field in common_host_fields}
            if host_environment is None:
                host_environment = environment
            elif environment != host_environment:
                raise SystemExit("host or benchmark controls changed between scored processes")
            if host.get("evaluator_sha256") != expected_evaluator:
                raise SystemExit(f"{role} binary was built for a different evaluator")
            if host.get("cargo_lock_sha256") != expected_cargo_lock:
                raise SystemExit(f"{role} binary was built with a different Cargo.lock")
            expected_hash = str(pair[role]["binary_sha256"])
            build = {
                "binary_sha256": expected_hash,
                "source_commit": host.get("source_commit"),
                "rustc_version": host.get("rustc_version"),
                "rustflags": host.get("rustflags"),
                "build_command": host.get("build_command"),
                "build_id": host.get("build_id"),
            }
            if not build["build_command"] or not build["build_id"]:
                raise SystemExit(f"{role} build metadata is incomplete")
            if role in builds and builds[role] != build:
                raise SystemExit(f"{role} build metadata changed between process pairs")
            builds[role] = build
        if parent.keys() != candidate.keys():
            raise SystemExit(f"pair cell inventory differs: cycle={cycle} order={order}")
        verify_run_inputs(parent_host, parent, expected_corpus, expected_tokenizers, pair["parent"])
        verify_run_inputs(
            candidate_host,
            candidate,
            expected_corpus,
            expected_tokenizers,
            pair["candidate"],
        )
        if cell_inventory is None:
            cell_inventory = set(parent)
        elif set(parent) != cell_inventory:
            raise SystemExit(
                f"cell inventory changed between process pairs: cycle={cycle} order={order}"
            )

        pair_block: Block = {}
        cycle_block = cycle_blocks.setdefault(cycle, {})
        for cell in sorted(parent):
            if parent[cell].keys() != candidate[cell].keys():
                raise SystemExit(f"pair round inventory differs for {cell}")
            round_logs: list[float] = []
            for round_index in sorted(parent[cell]):
                parent_ns, parent_bytes, parent_position, parent_rounds = parent[cell][round_index]
                candidate_ns, candidate_bytes, candidate_position, candidate_rounds = candidate[
                    cell
                ][round_index]
                if parent_bytes != candidate_bytes:
                    raise SystemExit(f"pair timed bytes differ for {cell} round={round_index}")
                if (parent_position, parent_rounds) != (candidate_position, candidate_rounds):
                    raise SystemExit(f"pair inner schedule differs for {cell} round={round_index}")
                ratio = parent_ns / candidate_ns
                log_ratio = math.log(ratio)
                round_logs.append(log_ratio)
                logs_by_cell[cell].append(log_ratio)
                pair_rows.append(
                    {
                        "cycle": cycle,
                        "order": order,
                        "round": round_index,
                        **cell_value(cell),
                        "parent_elapsed_ns": parent_ns,
                        "candidate_elapsed_ns": candidate_ns,
                        "total_bytes": parent_bytes,
                        "ratio": ratio,
                        "log_ratio": log_ratio,
                    }
                )
            pair_block[cell] = round_logs
            cycle_block.setdefault(cell, []).extend(round_logs)
        pair_aggregates.append(
            {
                "cycle": cycle,
                "order": order,
                "equal_cell_geomean": block_score([pair_block]),
            }
        )

    validate_mode(manifest["mode"], builds)
    write_pairs(output / "pairs.csv", pair_rows)
    cells = []
    for cell, logs in sorted(logs_by_cell.items()):
        cells.append(
            {
                **cell_value(cell),
                "paired_rounds": len(logs),
                "median_ratio": math.exp(statistics.median(logs)),
                "minimum_ratio": math.exp(min(logs)),
                "maximum_ratio": math.exp(max(logs)),
            }
        )
    blocks = [cycle_blocks[cycle] for cycle in sorted(cycle_blocks)]
    point_estimate = block_score(blocks)
    confidence_low, confidence_high = bootstrap_interval(blocks)
    cell_medians = [float(cell["median_ratio"]) for cell in cells]
    mode = manifest["mode"]
    scope = (
        "A/A noise screen only; promotion requires the frozen stratum manifest"
        if mode.endswith("-aa")
        else "portable candidate screen only; promotion requires the frozen stratum manifest"
    )
    summary = {
        "evaluator": "portable-paired-v2",
        "mode": mode,
        "aggregate_scope": scope,
        "aggregate_method": (
            "equal-cell geomean of exp(median paired log(candidate/parent throughput ratios))"
        ),
        "equal_cell_geomean": point_estimate,
        "block_bootstrap_samples": 10_000 if len(blocks) > 1 else 0,
        "block_bootstrap_seed": 0,
        "bootstrap_unit": "one complete AB-plus-BA outer cycle",
        "confidence_95_low": confidence_low,
        "confidence_95_high": confidence_high,
        "minimum_cell_median": min(cell_medians),
        "maximum_cell_median": max(cell_medians),
        "builds": builds,
        "pair_aggregates": pair_aggregates,
        "cells": cells,
    }
    write_json(output / "summary.json", summary)
    return summary


def main() -> None:
    """Run the declared schedule and publish its paired noise summary."""
    args = parse_args()
    runs = run_schedule(args)
    summary = summarize(runs, args.output.resolve())
    write_json(args.output.resolve() / "failures.json", [])
    print(json.dumps({key: summary[key] for key in ("evaluator", "equal_cell_geomean")}))


if __name__ == "__main__":
    main()
