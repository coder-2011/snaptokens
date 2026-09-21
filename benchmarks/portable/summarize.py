#!/usr/bin/env python3
"""Reduce complete portable JSONL runs into auditable paired aggregates."""

from __future__ import annotations

import argparse
import csv
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable


EncodeKey = tuple[str, str, str, str, str]
LoadKey = tuple[str, str, str]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", action="append", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    return parser.parse_args()


def read_rows(paths: Iterable[Path]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    seen: set[tuple[str, int]] = set()
    for path in paths:
        resolved = path.resolve()
        if not resolved.is_file():
            raise ValueError(f"portable input is not a file: {resolved}")
        with resolved.open(encoding="utf-8") as source:
            for line_number, line in enumerate(source, start=1):
                try:
                    row = json.loads(line)
                except json.JSONDecodeError as error:
                    raise ValueError(
                        f"invalid JSON in {resolved}:{line_number}"
                    ) from error
                if not isinstance(row, dict):
                    raise ValueError(f"non-object JSON row in {resolved}:{line_number}")
                identity = (str(resolved), line_number)
                if identity in seen:
                    raise ValueError(f"duplicate raw row identity: {identity}")
                seen.add(identity)
                rows.append(row)
    if not rows:
        raise ValueError("no portable JSONL rows supplied")
    return rows


def atomic_json(path: Path, value: object) -> None:
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    temporary.replace(path)


def atomic_csv(
    path: Path, fields: list[str], rows: Iterable[dict[str, object]]
) -> None:
    temporary = path.with_suffix(path.suffix + ".tmp")
    with temporary.open("w", newline="", encoding="utf-8") as destination:
        writer = csv.DictWriter(destination, fieldnames=fields, extrasaction="raise")
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(path)


def geomean(values: Iterable[float]) -> float:
    materialized = list(values)
    if not materialized or any(
        value <= 0 or not math.isfinite(value) for value in materialized
    ):
        raise ValueError("geometric mean requires finite positive values")
    return math.exp(statistics.fmean(math.log(value) for value in materialized))


def exact_status(status: object) -> bool:
    return status in {"exact", "oracle"}


def encode_medians(
    rows: list[dict[str, Any]],
) -> tuple[list[dict[str, object]], dict[EncodeKey, float]]:
    samples: dict[EncodeKey, dict[int, float]] = defaultdict(dict)
    for row in rows:
        if row.get("kind") != "encode_measurement":
            continue
        key = tuple(
            str(row[field])
            for field in ("host", "model", "contract", "shape", "implementation")
        )
        round_index = int(row["round"])
        if round_index in samples[key]:
            raise ValueError(f"duplicate encode round for {key}: {round_index}")
        elapsed_ns = float(row["elapsed_ns"])
        if elapsed_ns <= 0:
            raise ValueError(f"non-positive encode duration for {key}: {elapsed_ns}")
        samples[key][round_index] = elapsed_ns

    table: list[dict[str, object]] = []
    medians: dict[EncodeKey, float] = {}
    for key, rounds in sorted(samples.items()):
        host, model, contract, shape, implementation = key
        values = list(rounds.values())
        median_ns = statistics.median(values)
        medians[key] = median_ns
        table.append(
            {
                "host": host,
                "model": model,
                "contract": contract,
                "shape": shape,
                "implementation": implementation,
                "rounds": len(values),
                "median_elapsed_ns": median_ns,
                "minimum_elapsed_ns": min(values),
                "maximum_elapsed_ns": max(values),
            }
        )
    if not medians:
        raise ValueError("portable input contains no encode measurements")
    return table, medians


def paired_encode_rows(medians: dict[EncodeKey, float]) -> list[dict[str, object]]:
    pairs: list[dict[str, object]] = []
    comparisons = {
        "gigatoken-json": "flat-ragged",
        "fastokens-json": "nested",
        "huggingface-json": "nested",
    }
    for competitor, contract in comparisons.items():
        for key, competitor_ns in sorted(medians.items()):
            host, model, observed_contract, shape, implementation = key
            if implementation != competitor or observed_contract != contract:
                continue
            snaptokens_key = (host, model, contract, shape, "snaptokens-json")
            snaptokens_ns = medians.get(snaptokens_key)
            if snaptokens_ns is None:
                raise ValueError(f"missing Snaptokens timing for {key}")
            pairs.append(
                {
                    "host": host,
                    "model": model,
                    "contract": contract,
                    "shape": shape,
                    "competitor": competitor,
                    "snaptokens_median_ns": snaptokens_ns,
                    "competitor_median_ns": competitor_ns,
                    "snaptokens_speedup": competitor_ns / snaptokens_ns,
                }
            )
    if not pairs:
        raise ValueError("no complete Snaptokens/competitor encode pairs")
    return pairs


def aggregate_pairs(pairs: list[dict[str, object]]) -> list[dict[str, object]]:
    grouped: dict[str, list[dict[str, object]]] = defaultdict(list)
    for pair in pairs:
        grouped[str(pair["competitor"])].append(pair)
    aggregates = []
    for competitor, values in sorted(grouped.items()):
        ratios = [float(value["snaptokens_speedup"]) for value in values]
        aggregates.append(
            {
                "competitor": competitor,
                "paired_cells": len(values),
                "snaptokens_wins": sum(ratio > 1.0 for ratio in ratios),
                "speedup_geomean": geomean(ratios),
                "minimum_cell_speedup": min(ratios),
                "maximum_cell_speedup": max(ratios),
            }
        )
    return aggregates


def load_medians(
    rows: list[dict[str, Any]],
) -> tuple[list[dict[str, object]], dict[LoadKey, dict[str, float]]]:
    samples: dict[LoadKey, dict[int, dict[str, float]]] = defaultdict(dict)
    for row in rows:
        if row.get("kind") != "load_measurement":
            continue
        key = tuple(str(row[field]) for field in ("host", "model", "implementation"))
        round_index = int(row["round"])
        if round_index in samples[key]:
            raise ValueError(f"duplicate load round for {key}: {round_index}")
        samples[key][round_index] = {
            "load_ns": float(row["load_ns"]),
            "first_encode_ns": float(row["first_encode_ns"]),
            "load_plus_first_encode_ns": float(row["load_plus_first_encode_ns"]),
        }

    table: list[dict[str, object]] = []
    medians: dict[LoadKey, dict[str, float]] = {}
    for key, rounds in sorted(samples.items()):
        host, model, implementation = key
        values = list(rounds.values())
        medians[key] = {
            metric: statistics.median(value[metric] for value in values)
            for metric in ("load_ns", "first_encode_ns", "load_plus_first_encode_ns")
        }
        table.append(
            {
                "host": host,
                "model": model,
                "implementation": implementation,
                "rounds": len(values),
                **medians[key],
            }
        )
    return table, medians


def st_load_rows(medians: dict[LoadKey, dict[str, float]]) -> list[dict[str, object]]:
    pairs: list[dict[str, object]] = []
    for host, model, implementation in sorted(medians):
        if implementation != "snaptokens-json":
            continue
        json_values = medians[(host, model, implementation)]
        st_values = medians.get((host, model, "snaptokens-st"))
        if st_values is None:
            raise ValueError(f"missing direct ST load for {host}/{model}")
        pairs.append(
            {
                "host": host,
                "model": model,
                "load_speedup": json_values["load_ns"] / st_values["load_ns"],
                "load_plus_first_encode_speedup": (
                    json_values["load_plus_first_encode_ns"]
                    / st_values["load_plus_first_encode_ns"]
                ),
            }
        )
    if not pairs:
        raise ValueError("no Snaptokens JSON/ST load pairs")
    return pairs


def artifact_rows(rows: list[dict[str, Any]]) -> list[dict[str, object]]:
    artifacts = []
    for row in rows:
        if row.get("kind") != "artifact":
            continue
        json_bytes = int(row["json_bytes"])
        st_bytes = int(row["st_bytes"])
        if json_bytes <= 0 or st_bytes <= 0:
            raise ValueError(f"invalid artifact sizes: {row}")
        artifacts.append(
            {
                "host": str(row["host"]),
                "model": str(row["model"]),
                "json_bytes": json_bytes,
                "st_bytes": st_bytes,
                "disk_saving_percent": (1.0 - st_bytes / json_bytes) * 100.0,
            }
        )
    if not artifacts:
        raise ValueError("portable input contains no artifacts")
    return sorted(artifacts, key=lambda row: (str(row["host"]), str(row["model"])))


def main() -> None:
    args = parse_args()
    output = args.output.resolve()
    if output.exists() and any(output.iterdir()):
        raise SystemExit(f"output directory must be empty: {output}")
    output.mkdir(parents=True, exist_ok=True)
    try:
        rows = read_rows(args.input)
        coverage = sorted(
            (
                {
                    "host": str(row["host"]),
                    "model": str(row["model"]),
                    "implementation": str(row["implementation"]),
                    "status": str(row["status"]),
                    "probe_count": int(row["probe_count"]),
                }
                for row in rows
                if row.get("kind") == "coverage"
            ),
            key=lambda row: (row["host"], row["model"], row["implementation"]),
        )
        if not coverage or not any(exact_status(row["status"]) for row in coverage):
            raise ValueError("portable input lacks exact coverage")
        encode_table, encode_values = encode_medians(rows)
        encode_pairs = paired_encode_rows(encode_values)
        aggregates = aggregate_pairs(encode_pairs)
        load_table, load_values = load_medians(rows)
        st_pairs = st_load_rows(load_values)
        artifacts = artifact_rows(rows)
        input_paths = [str(path.resolve()) for path in args.input]
        report = {
            "inputs": input_paths,
            "hosts": sorted({str(row["host"]) for row in coverage}),
            "aggregate_method": "geometric mean of matched per-host/model/shape median elapsed-time ratios",
            "encode_aggregates": aggregates,
            "st_load_speedup_geomean": geomean(
                float(row["load_speedup"]) for row in st_pairs
            ),
            "st_load_plus_first_encode_speedup_geomean": geomean(
                float(row["load_plus_first_encode_speedup"]) for row in st_pairs
            ),
            "st_disk_saving_geomean_percent": (
                1.0
                - geomean(
                    float(row["st_bytes"]) / float(row["json_bytes"])
                    for row in artifacts
                )
            )
            * 100.0,
        }
        atomic_csv(
            output / "coverage.csv",
            ["host", "model", "implementation", "status", "probe_count"],
            coverage,
        )
        atomic_csv(
            output / "encode-medians.csv",
            [
                "host",
                "model",
                "contract",
                "shape",
                "implementation",
                "rounds",
                "median_elapsed_ns",
                "minimum_elapsed_ns",
                "maximum_elapsed_ns",
            ],
            encode_table,
        )
        atomic_csv(
            output / "paired-comparisons.csv",
            [
                "host",
                "model",
                "contract",
                "shape",
                "competitor",
                "snaptokens_median_ns",
                "competitor_median_ns",
                "snaptokens_speedup",
            ],
            encode_pairs,
        )
        atomic_csv(
            output / "load-medians.csv",
            [
                "host",
                "model",
                "implementation",
                "rounds",
                "load_ns",
                "first_encode_ns",
                "load_plus_first_encode_ns",
            ],
            load_table,
        )
        atomic_csv(
            output / "st-load-comparisons.csv",
            ["host", "model", "load_speedup", "load_plus_first_encode_speedup"],
            st_pairs,
        )
        atomic_csv(
            output / "artifacts.csv",
            ["host", "model", "json_bytes", "st_bytes", "disk_saving_percent"],
            artifacts,
        )
        atomic_json(output / "report.json", report)
    except ValueError as error:
        raise SystemExit(str(error)) from error


if __name__ == "__main__":
    main()
