"""Copy the existing twelve-model panel and freeze varied text before any experiment."""
import hashlib
import json
from pathlib import Path
import shutil
import sys

source, longbench, destination = map(Path, sys.argv[1:])
destination.mkdir(parents=True, exist_ok=False)
manifest = {}
for path in sorted(source.glob("*.json")):
    shutil.copyfile(path, destination / path.name)
    model = json.loads(path.read_text())["model"]
    manifest[path.name] = {
        "source": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "type": model.get("type"), "vocab": len(model["vocab"]),
        "byte_fallback": model.get("byte_fallback"), "ignore_merges": model.get("ignore_merges"),
    }
documents = json.loads(longbench.read_text())
inputs = ["", "hello world", "  \r\n\t", "café e\u0301 中文 Ελληνικά 👩🏽‍💻", "<|endoftext|>",
          'fn main() { println!("hello"); }\n' * 80,
          json.dumps({"rows": list(range(300)), "value": "雪"}, ensure_ascii=False)]
for document in documents[:4]:
    text = document["context"]
    for length in [256, 4096, 65536]:
        inputs.append(text[:length])
(destination / "corpus.json").write_text(json.dumps(inputs, ensure_ascii=False))
(destination / "inputs-manifest.txt").write_text(json.dumps(manifest, indent=2))
print(json.dumps(manifest, indent=2))
