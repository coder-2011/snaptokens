#!/usr/bin/env python3
"""Render the July 2026 README chart with python3 benchmarks/tools/plot_overview.py.

Requires matplotlib and Helvetica Neue. Reads recorded results without running benchmarks.
"""

import csv
import hashlib
import io
import re
from pathlib import Path
from statistics import geometric_mean

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import font_manager
from matplotlib.patches import FancyBboxPatch


def main():
    """Plot recorded median throughput as multiples of the Hugging Face baseline."""
    root = Path(__file__).resolve().parents[2]
    source = root / "benchmarks/data/2026-07-31/portable-current/summaries/encode-medians.csv"
    with source.open(newline="") as handle:
        rows = list(csv.DictReader(handle))

    shapes = ["140-byte batch 1", "140-byte batch 32", "140-byte batch 512", "4 KiB single", "64 KiB single"]
    baseline = {(r["host"], r["model"], r["shape"]): float(r["mib_per_s"])
                for r in rows if r["implementation"] == "huggingface-json" and r["contract"] == "nested"}
    series = []
    for engine, contract, label, color in [
        ("snaptokens-json", "nested", "Snaptokens", "#bbb6cf"),
        ("gigatoken-json", "flat-ragged", "Gigatoken", "#a8b6ad"),
        ("huggingface-json", "nested", "Hugging Face", "#d8c4a8"),
    ]:
        selected = [r for r in rows if r["implementation"] == engine and r["contract"] == contract]
        # Keep all 12 models, 13 hosts, and five workloads for each output contract.
        hosts, models = {r["host"] for r in selected}, {r["model"] for r in selected}
        expected = {(host, model, shape) for host in hosts for model in models for shape in shapes}
        actual = {(r["host"], r["model"], r["shape"]) for r in selected}
        if (len(hosts) != 13 or len(models) != 12 or len(selected) != 780
                or actual != expected or len(actual) != len(selected) or actual != baseline.keys()):
            raise ValueError(f"Expected the complete historical {contract} comparison for {engine}")
        # Divide actual recorded medians, not ratios with different Snaptokens denominators.
        # Gigatoken only has flat-ragged measurements; the README discloses this difference.
        values = [geometric_mean(float(r["mib_per_s"]) / baseline[(r["host"], r["model"], shape)]
                                 for r in selected if r["shape"] == shape)
                  for shape in shapes]
        series.append((label, values, color))

    # Outline text in the SVG so GitHub uses the chosen font without substitutions.
    font_manager.findfont("Helvetica Neue", fallback_to_default=False)
    plt.rcParams.update({"font.family": "Helvetica Neue", "svg.fonttype": "path", "svg.hashsalt": "snaptokens-july-2026"})
    background, ink, secondary = "#faf9f6", "#242424", "#65645f"
    fig, ax = plt.subplots(figsize=(14.4, 7.0), facecolor=background)
    fig.subplots_adjust(left=.07, right=.97, top=.78, bottom=.19)
    fig.text(.045, .91, "BPE Tokenization throughput", fontsize=22, weight="medium", color=ink)
    ax.set_facecolor(background)
    for index, (label, values, color) in enumerate(series):
        positions = [x + (index - 1) * .23 for x in range(5)]
        ax.bar(positions, values, width=.20, color=color, edgecolor="black", linewidth=.7, label=label)
        if label != "Hugging Face":
            for x, value in zip(positions, values):
                ax.text(x, value + 2, f"{value:.1f}×",
                        ha="center", va="bottom", fontsize=12, color=ink)

    ax.set_xlim(-.55, 4.55)
    ax.set_ylim(0, 120)
    # Draw rounded fills and outlines within the original bar geometry.
    x_scale = ax.bbox.width / 5.1
    y_scale = ax.bbox.height / 120
    for bar in list(ax.patches):
        radius = min(2 * fig.dpi / 72, bar.get_width() * x_scale / 2,
                     bar.get_height() * y_scale / 2)
        ax.add_patch(FancyBboxPatch(
            bar.get_xy(), bar.get_width(), bar.get_height(),
            boxstyle=f"round,pad=0,rounding_size={radius / x_scale}",
            mutation_aspect=x_scale / y_scale, transform=ax.transData,
            facecolor=bar.get_facecolor(), edgecolor="black", linewidth=.7,
        ))
        bar.remove()
    ax.set_yticks([0, 25, 50, 75, 100], ["0", "25×", "50×", "75×", "100×"])
    ax.set_axisbelow(True)
    ax.yaxis.grid(True, color="#dfded8", linewidth=.8)
    ax.set_xticks(range(5), ["B=1", "B=32", "B=512", "4 KiB", "64 KiB"])
    ax.tick_params(axis="x", length=0, pad=14, labelsize=14, colors=ink)
    ax.tick_params(axis="y", length=0, pad=10, labelsize=12, colors=secondary)
    for label in ax.get_xticklabels():
        label.set_weight("medium")
    for side in ("top", "left", "right"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color("#777670")
    fig.legend(loc="upper right", bbox_to_anchor=(.97, .925), frameon=False, ncol=3,
               fontsize=13, labelcolor=ink, handlelength=1.2, handleheight=.85, columnspacing=1.6)
    buffer = io.StringIO()
    fig.savefig(buffer, format="svg", facecolor=background,
                metadata={"Date": None, "Title": "BPE Tokenization throughput",
                          "Description": "July 2026 geometric mean throughput relative to Hugging Face. Snaptokens and Hugging Face use nested output; Gigatoken uses flat-ragged output. Each workload includes all 156 host-model pairs."})
    plt.close(fig)
    # Matplotlib emits trailing spaces in SVG paths; keep the generated file diff-clean.
    svg = "\n".join(line.rstrip() for line in buffer.getvalue().splitlines()) + "\n"
    # Put the hash in the filename: the raw-file cache can ignore query parameters.
    version = hashlib.sha256(svg.encode()).hexdigest()[:12]
    filename = f"benchmark-throughput-{version}.svg"
    output = root / "assets" / filename
    output.write_text(svg)
    readme = root / "README.md"
    pattern = r'benchmark-throughput(?:-dark)?(?:-[a-f0-9]{12})?\.svg(?:\?v=[a-f0-9]+)?(?=")'
    previous = re.findall(pattern, readme.read_text())
    readme.write_text(re.sub(pattern, filename, readme.read_text()))
    # Remove only the superseded generated chart referenced by this README.
    for reference in previous:
        old = root / "assets" / reference.split("?")[0]
        if old != output:
            old.unlink(missing_ok=True)


if __name__ == "__main__":
    main()
