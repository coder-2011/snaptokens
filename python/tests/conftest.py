import json

import pytest
from snaptokens import Tokenizer


@pytest.fixture
def tokenizer_config() -> dict:
    """Return a minimal BPE that exercises merging and decoding."""
    return {
        "version": "1.0",
        "truncation": None,
        "padding": None,
        "added_tokens": [],
        "normalizer": None,
        "pre_tokenizer": None,
        "post_processor": None,
        "decoder": {"type": "Fuse"},
        "model": {
            "type": "BPE",
            "vocab": {"a": 0, "b": 1, "ab": 2},
            "merges": [["a", "b"]],
            "byte_fallback": False,
            "ignore_merges": False,
        },
    }


@pytest.fixture
def tokenizer_json(tokenizer_config):
    return json.dumps(tokenizer_config)


@pytest.fixture
def tokenizer_file(tmp_path, tokenizer_json):
    path = tmp_path / "tokenizer.json"
    path.write_text(tokenizer_json, encoding="utf-8")
    return path


@pytest.fixture
def tokenizer(tokenizer_json):
    return Tokenizer.from_json_str(tokenizer_json)


@pytest.fixture
def template_json(tokenizer_config):
    config = tokenizer_config
    config["model"]["vocab"].update({"[CLS]": 3, "[SEP]": 4})
    config["post_processor"] = {
        "type": "TemplateProcessing",
        "single": [{"SpecialToken": {"id": "[CLS]", "type_id": 0}},
                   {"Sequence": {"id": "A", "type_id": 0}},
                   {"SpecialToken": {"id": "[SEP]", "type_id": 0}}],
        "pair": [],
        "special_tokens": {
            "[CLS]": {"id": "[CLS]", "ids": [3], "tokens": ["[CLS]"]},
            "[SEP]": {"id": "[SEP]", "ids": [4], "tokens": ["[SEP]"]},
        },
    }
    return json.dumps(config)
