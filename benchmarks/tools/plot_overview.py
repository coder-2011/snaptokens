#!/usr/bin/env python3
"""Render the July 2026 README chart with python3 benchmarks/tools/plot_overview.py.

Requires matplotlib. Reads recorded results without running benchmarks.
"""

import csv
from pathlib import Path
from statistics import geometric_mean

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def main():
    """Compare Snaptokens with each engine using its matching output contract."""
    root = Path(__file__).resolve().parents[2]
    source = root / "benchmarks/data/2026-07-31/portable-current/summaries/paired-comparisons.csv"
    with source.open(newline="") as handle:
        rows = list(csv.DictReader(handle))

    shapes = ["140-byte batch 1", "140-byte batch 32", "140-byte batch 512", "4 KiB single", "64 KiB single"]
    panels = [
        ("gigatoken-json", "flat-ragged", "vs Gigatoken", "#b7a9e8", 4.4, [0, 1, 2, 3, 4]),
        ("huggingface-json", "nested", "vs Hugging Face", "#e9b896", 132, [0, 40, 80, 120]),
    ]
    background, ink, muted = "#18191e", "#f3efe8", "#a5a5ad"
    plt.rcParams.update({"font.family": "DejaVu Sans", "svg.fonttype": "none", "svg.hashsalt": "snaptokens-july-2026"})
    fig, axes = plt.subplots(1, 2, figsize=(14.4, 6.4), facecolor=background, sharey=True)
    fig.subplots_adjust(left=.18, right=.955, top=.73, bottom=.12, wspace=.20)
    fig.text(.045, .90, "Snaptokens speedup", fontsize=28, weight="bold", color=ink)

    for ax, (engine, contract, title, color, limit, ticks) in zip(axes, panels):
        selected = [r for r in rows if r["competitor"] == engine]
        # Keep all 12 models, 13 hosts, and five workloads for each output contract.
        hosts, models = {r["host"] for r in selected}, {r["model"] for r in selected}
        expected = {(host, model, shape) for host in hosts for model in models for shape in shapes}
        actual = {(r["host"], r["model"], r["shape"]) for r in selected}
        if (len(hosts) != 13 or len(models) != 12 or len(selected) != 780
                or actual != expected or len(actual) != len(selected)
                or any(r["contract"] != contract for r in selected)):
            raise ValueError(f"Expected the complete historical {contract} comparison for {engine}")
        values = [geometric_mean(float(r["speedup"]) for r in selected if r["shape"] == shape)
                  for shape in shapes]

        ax.set_facecolor(background)
        ax.barh(range(5), values, height=.46, color=color, zorder=3)
        for y, value in enumerate(values):
            ax.text(value + limit * .025, y, f"{value:.2f}×", va="center",
                    fontsize=15, weight="bold", color=ink)
        ax.set_title(title, loc="left", pad=22, fontsize=17, weight="bold", color=color)
        ax.set_xlim(0, limit)
        ax.set_ylim(4.6, -.6)
        ax.set_xticks(ticks, [f"{tick}×" for tick in ticks])
        ax.set_yticks(range(5), ["140 B × 1", "140 B × 32", "140 B × 512", "4 KiB × 1", "64 KiB × 1"])
        ax.tick_params(axis="x", length=0, pad=12, labelsize=11, colors=muted)
        ax.tick_params(axis="y", length=0, pad=20, labelsize=14, colors=ink)
        ax.set_axisbelow(True)
        ax.xaxis.grid(True, color="#303138", linewidth=.8)
        for spine in ax.spines.values():
            spine.set_visible(False)

    output = root / "assets/benchmark-portable-overview.svg"
    fig.savefig(output, facecolor=background,
                metadata={"Date": None, "Title": "Snaptokens speedup by workload",
                          "Description": "July 2026 geometric means of paired median throughput ratios. Each panel compares matching output contracts: flat-ragged for Gigatoken, nested for Hugging Face. Each workload includes all 156 host-model pairs."})
    plt.close(fig)
    # Matplotlib emits trailing spaces in SVG paths; keep the generated file diff-clean.
    output.write_text("\n".join(line.rstrip() for line in output.read_text().splitlines()) + "\n")


if __name__ == "__main__":
    main()
