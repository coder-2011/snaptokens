#!/usr/bin/env python3
"""Builds the decode-eval-v1 corpora from locally cached datasets.

Outputs (under --output):
- enwik8.txt        copied from the frozen dev-v1 corpus
- chat.jsonl        ShareGPT messages, 100-2000 bytes, deduplicated
- longbench.jsonl   LongBench-v2 contexts, deduplicated, full documents
- giant-single.txt  LongBench contexts concatenated to ~128 MiB

Uses only the Python standard library. Prints a SHA-256 manifest.
"""

import argparse
import hashlib
import json
import shutil
import sys
from pathlib import Path

CHAT_MIN, CHAT_MAX = 100, 2000
CHAT_ROW_CAP = 20000
GIANT_SINGLE_BYTES = 128 * 1024 * 1024


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--enwik8", required=True, type=Path)
    parser.add_argument("--sharegpt", required=True, type=Path, nargs="+")
    parser.add_argument("--longbench", required=True, type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)

    enwik8 = args.output / "enwik8.txt"
    if not enwik8.exists():
        shutil.copyfile(args.enwik8, enwik8)

    chat_path = args.output / "chat.jsonl"
    if not chat_path.exists():
        seen = set()
        rows = 0
        with chat_path.open("w", encoding="utf-8") as out:
            for part in args.sharegpt:
                if rows >= CHAT_ROW_CAP:
                    break
                data = json.loads(part.read_text(encoding="utf-8"))
                for record in data:
                    for turn in record.get("conversations", []):
                        text = turn.get("value", "")
                        size = len(text.encode("utf-8"))
                        if not CHAT_MIN <= size <= CHAT_MAX:
                            continue
                        key = hashlib.sha256(text.encode("utf-8")).digest()
                        if key in seen:
                            continue
                        seen.add(key)
                        out.write(json.dumps({"text": text}, ensure_ascii=False))
                        out.write("\n")
                        rows += 1
                        if rows >= CHAT_ROW_CAP:
                            break
                    if rows >= CHAT_ROW_CAP:
                        break
                del data

    longbench_path = args.output / "longbench.jsonl"
    giant_single = args.output / "giant-single.txt"
    if not longbench_path.exists() or not giant_single.exists():
        data = json.loads(args.longbench.read_text(encoding="utf-8"))
        seen = set()
        contexts = []
        for record in data:
            text = record.get("context", "")
            if not text:
                continue
            key = hashlib.sha256(text.encode("utf-8")).digest()
            if key in seen:
                continue
            seen.add(key)
            contexts.append(text)
        del data
        if not longbench_path.exists():
            with longbench_path.open("w", encoding="utf-8") as out:
                for text in contexts:
                    out.write(json.dumps({"text": text}, ensure_ascii=False))
                    out.write("\n")
        if not giant_single.exists():
            written = 0
            with giant_single.open("w", encoding="utf-8") as out:
                for text in contexts:
                    if written >= GIANT_SINGLE_BYTES:
                        break
                    out.write(text)
                    out.write("\n")
                    written += len(text.encode("utf-8")) + 1

    manifest = {}
    for path in sorted(args.output.iterdir()):
        if path.suffix in {".txt", ".jsonl"}:
            manifest[path.name] = {
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
            }
    json.dump(manifest, sys.stdout, indent=2)
    print()


if __name__ == "__main__":
    main()
