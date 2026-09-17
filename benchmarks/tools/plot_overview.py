#!/usr/bin/env python3
"""Render the July 2026 README chart with python3 benchmarks/tools/plot_overview.py.

Requires matplotlib and Arial. Reads recorded results without running benchmarks.
"""

import csv
import hashlib
import re
from pathlib import Path
from statistics import geometric_mean

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import font_manager


def main():
    """Normalize each engine to Snaptokens with the same output contract."""
    root = Path(__file__).resolve().parents[2]
    source = root / "benchmarks/data/2026-07-31/portable-current/summaries/paired-comparisons.csv"
    with source.open(newline="") as handle:
        rows = list(csv.DictReader(handle))

    shapes = ["140-byte batch 1", "140-byte batch 32", "140-byte batch 512", "4 KiB single", "64 KiB single"]
    series = [("Snaptokens", [100] * 5, "#bbb6cf")]
    for engine, contract, label, color in [
        ("gigatoken-json", "flat-ragged", "Gigatoken", "#a8b6ad"),
        ("huggingface-json", "nested", "Hugging Face", "#d8c4a8"),
    ]:
        selected = [r for r in rows if r["competitor"] == engine]
        # Keep all 12 models, 13 hosts, and five workloads for each output contract.
        hosts, models = {r["host"] for r in selected}, {r["model"] for r in selected}
        expected = {(host, model, shape) for host in hosts for model in models for shape in shapes}
        actual = {(r["host"], r["model"], r["shape"]) for r in selected}
        if (len(hosts) != 13 or len(models) != 12 or len(selected) != 780
                or actual != expected or len(actual) != len(selected)
                or any(r["contract"] != contract for r in selected)):
            raise ValueError(f"Expected the complete historical {contract} comparison for {engine}")
        # Normalize within each matched pair, not across flat-ragged and nested timings.
        values = [100 / geometric_mean(float(r["speedup"]) for r in selected if r["shape"] == shape)
                  for shape in shapes]
        series.append((label, values, color))

    # Outline text in the SVG so GitHub uses the chosen font without substitutions.
    font_manager.findfont("Arial", fallback_to_default=False)
    plt.rcParams.update({"font.family": "Arial", "svg.fonttype": "path", "svg.hashsalt": "snaptokens-july-2026"})
    background, ink, muted = "#faf9f6", "#242424", "#65645f"
    fig, ax = plt.subplots(figsize=(14.4, 7.0), facecolor=background)
    fig.subplots_adjust(left=.07, right=.97, top=.78, bottom=.19)
    fig.text(.045, .91, "Tokenization throughput", fontsize=22, weight="bold", color=ink)
    ax.set_facecolor(background)
    for index, (label, values, color) in enumerate(series):
        positions = [x + (index - 1) * .23 for x in range(5)]
        ax.bar(positions, values, width=.20, color=color, edgecolor="#777670", linewidth=.8, label=label, zorder=3)
        for x, value in zip(positions, values):
            ax.text(x, value + 2, f"{value:.0f}%" if value >= 10 else f"{value:.1f}%",
                    ha="center", va="bottom", fontsize=12, color=ink)

    ax.set_xlim(-.55, 4.55)
    ax.set_ylim(0, 112)
    ax.set_yticks([0, 25, 50, 75, 100], ["0", "25", "50", "75", "100"])
    ax.set_axisbelow(True)
    ax.yaxis.grid(True, color="#dfded8", linewidth=.8)
    ax.axhline(100, color="#8b8982", linestyle=(0, (4, 4)), linewidth=1)
    ax.set_xticks(range(5), ["Batch of 1", "Batch of 32", "Batch of 512", "4 KiB", "64 KiB"])
    ax.tick_params(axis="x", length=0, pad=14, labelsize=14, colors=ink)
    ax.tick_params(axis="y", length=0, pad=10, labelsize=12, colors=muted)
    for label in ax.get_xticklabels():
        label.set_weight("bold")
    for x, label in enumerate(["140 bytes / input"] * 3 + ["single input"] * 2):
        ax.text(x, -.14, label, transform=ax.get_xaxis_transform(), ha="center", fontsize=12, color=muted)
    ax.set_ylabel("Relative throughput (%)", fontsize=13, labelpad=12, color=ink)
    for side in ("top", "left", "right"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color("#777670")
    fig.legend(loc="upper right", bbox_to_anchor=(.97, .925), frameon=False, ncol=3,
               fontsize=13, labelcolor=ink, handlelength=1.2, handleheight=.85, columnspacing=1.6)
    output = root / "assets/benchmark-portable-overview.svg"
    fig.savefig(output, facecolor=background,
                metadata={"Date": None, "Title": "Tokenization throughput by workload",
                          "Description": "July 2026 geometric mean relative throughput. Snaptokens is 100% in each group. Gigatoken is normalized to flat-ragged Snaptokens; Hugging Face to nested Snaptokens. Each workload includes all 156 host-model pairs."})
    plt.close(fig)
    # Matplotlib emits trailing spaces in SVG paths; keep the generated file diff-clean.
    output.write_text("\n".join(line.rstrip() for line in output.read_text().splitlines()) + "\n")
    # Give each rendered image a fresh URL so README readers do not see a cached design.
    version = hashlib.sha256(output.read_bytes()).hexdigest()[:12]
    readme = root / "README.md"
    readme.write_text(re.sub(
        r'(benchmark-portable-overview\.svg)(?:\?v=[a-f0-9]+)?(?=")',
        rf'\1?v={version}', readme.read_text(),
    ))


if __name__ == "__main__":
    main()
