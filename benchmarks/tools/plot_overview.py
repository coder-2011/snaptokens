#!/usr/bin/env python3
"""Render the historical README chart: python3 benchmarks/tools/plot_overview.py.

Requires matplotlib. Reads recorded July 2026 results without running benchmarks.
"""

import csv
from pathlib import Path
from statistics import geometric_mean

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch


def main():
    """Plot geometric mean throughput ratios for each of the five input shapes."""
    root = Path(__file__).resolve().parents[2]
    source = root / "benchmarks/data/2026-07-31/portable-current/summaries/paired-comparisons.csv"
    with source.open(newline="") as handle:
        rows = [row for row in csv.DictReader(handle) if row["competitor"] == "gigatoken-json"]

    shapes = ["140-byte batch 1", "140-byte batch 32", "140-byte batch 512", "4 KiB single", "64 KiB single"]
    # Each workload has the same 12 models on 13 hosts, with matching output contracts.
    expected = {(host, model, shape) for host in {r["host"] for r in rows}
                for model in {r["model"] for r in rows} for shape in shapes}
    actual = {(r["host"], r["model"], r["shape"]) for r in rows}
    if len(rows) != 780 or actual != expected or len(actual) != len(rows):
        raise ValueError("Expected the complete historical 780-cell comparison")
    if any(r["contract"] != "flat-ragged" for r in rows):
        raise ValueError("Expected matching flat-ragged output contracts")

    overall = geometric_mean(float(r["speedup"]) for r in rows)
    speedups = [geometric_mean(float(r["speedup"]) for r in rows if r["shape"] == shape)
                for shape in shapes]
    wins = sum(float(r["speedup"]) > 1 for r in rows)

    background, ink, muted = "#faf9f6", "#243b35", "#69746e"
    green, green_edge, gray = "#8bb5a5", "#527c6d", "#dedfd8"
    plt.rcParams.update({"font.family": "DejaVu Sans", "svg.fonttype": "none", "svg.hashsalt": "snaptokens-july-2026"})
    fig = plt.figure(figsize=(14.4, 8.4), facecolor=background)
    fig.text(.055, .925, "SNAPTOKENS  /  JULY 2026 BENCHMARK", fontsize=12, weight="bold", color=green_edge)
    fig.text(.055, .832, f"{overall:.2f}× faster tokenization", fontsize=36, weight="bold", color=ink)
    fig.text(.055, .778, "vs Gigatoken · 12 tokenizers · 13 hosts · geometric mean throughput", fontsize=14, color=muted)

    ax = fig.add_axes([.07, .215, .885, .475], facecolor=background)
    ax.set_ylim(0, 4.1)
    ax.set_xlim(-.55, 4.55)
    ax.set_axisbelow(True)
    ax.set_yticks([0, 1, 2, 3, 4], ["0", "1×", "2×", "3×", "4×"])
    ax.yaxis.grid(True, color="#e4e6df", linewidth=.8)
    ax.axhline(1, color=muted, linestyle=(0, (4, 4)), linewidth=1.1, zorder=2)
    ax.bar([x - .17 for x in range(5)], [1] * 5, width=.29, color=gray, edgecolor="#b5bbb1", linewidth=1, zorder=3)
    ax.bar([x + .17 for x in range(5)], speedups, width=.29, color=green, edgecolor=green_edge, linewidth=1, zorder=3)
    for x, speedup in enumerate(speedups):
        ax.text(x + .17, speedup + .12, f"{speedup:.2f}×", ha="center", fontsize=20, weight="bold", color=ink)
    ax.set_xticks(range(5), ["Batch of 1", "Batch of 32", "Batch of 512", "4 KiB", "64 KiB"])
    ax.tick_params(axis="x", length=0, pad=13, labelsize=14, colors=ink)
    ax.tick_params(axis="y", length=0, pad=10, labelsize=11, colors=muted)
    for label in ax.get_xticklabels():
        label.set_weight("bold")
    for x, label in enumerate(["140 bytes / input"] * 3 + ["single input"] * 2):
        ax.text(x, -.15, label, transform=ax.get_xaxis_transform(), ha="center", fontsize=12, color=muted)
    for side in ("top", "left", "right"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color("#b5bbb1")
    fig.legend(handles=[Patch(facecolor=gray, edgecolor="#b5bbb1", label="Gigatoken · 1×"),
                        Patch(facecolor=green, edgecolor=green_edge, label="Snaptokens")],
               loc="upper right", bbox_to_anchor=(.955, .751), frameon=False, ncol=2,
               fontsize=12, labelcolor=ink, handlelength=1.1, handleheight=1.1)
    fig.text(.055, .083, "Geometric mean of 156 paired median ratios per workload · higher is better", fontsize=11, color=muted)
    fig.text(.055, .048, f"Historical results · 2026-07-31 · matching flat-ragged outputs · {wins}/{len(rows)} comparisons won", fontsize=11, color=muted)
    output = root / "assets/benchmark-portable-overview.svg"
    fig.savefig(output, facecolor=background,
                metadata={"Date": None, "Title": "Snaptokens vs Gigatoken: July 2026 workload comparison",
                          "Description": "Historical geometric mean throughput speedups. Overall 2.19x; batch 1: 3.57x, batch 32: 2.35x, batch 512: 2.18x, 4 KiB: 1.89x, 64 KiB: 1.46x."})
    plt.close(fig)
    # Matplotlib emits trailing spaces in SVG paths; keep the generated file diff-clean.
    output.write_text("\n".join(line.rstrip() for line in output.read_text().splitlines()) + "\n")


if __name__ == "__main__":
    main()
