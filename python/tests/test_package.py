import json
import subprocess
import sys
import textwrap
from array import array

import pytest

from snaptokens import Tokenizer


def _tokenizer_json() -> dict:
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


def test_native_package_json_tkz_and_flat_batch(tmp_path) -> None:
    """Keep local loading, native sidecars, and packed batches equivalent."""
    json_path = tmp_path / "tokenizer.json"
    json_path.write_text(json.dumps(_tokenizer_json()), encoding="utf-8")

    tokenizer = Tokenizer.from_file(str(json_path), tkz_cache=True)
    assert Tokenizer.__module__ == "snaptokens._native"
    assert tokenizer.encode("ab").ids == [2]
    assert tokenizer.decode([2]) == "ab"

    rows = tokenizer.encode_batch(["ab", "a", ""])
    assert [row.ids for row in rows] == [[2], [0], []]

    packed_ids, packed_offsets = tokenizer.encode_batch_flat(["ab", "a", ""])
    ids = array("I")
    ids.frombytes(packed_ids)
    offsets = array("Q")
    offsets.frombytes(packed_offsets)
    assert ids.tolist() == [2, 0]
    assert offsets.tolist() == [0, 1, 2, 2]

    tkz_path = json_path.with_suffix(".tkz")
    assert tkz_path.is_file()
    cached = Tokenizer.from_file(str(tkz_path))
    assert cached.encode("ab").ids == [2]


def test_encode_paths_report_truncation_instead_of_empty_overflow(tmp_path) -> None:
    """Keep configured truncation from silently dropping tokens as no overflow."""
    json_path = tmp_path / "tokenizer.json"
    json_path.write_text(json.dumps(_tokenizer_json()), encoding="utf-8")
    tokenizer = Tokenizer.from_file(str(json_path))
    assert tokenizer.encode("aba").ids == [2, 0]
    assert tokenizer.encode("aba").overflowing == []

    tokenizer.enable_truncation(1)
    assert tokenizer.encode("aba").ids == [2]
    with pytest.raises(NotImplementedError):
        tokenizer.encode("aba").overflowing
    assert tokenizer.encode("a").overflowing == []

    rows = tokenizer.encode_batch(["aba", "a"])
    with pytest.raises(NotImplementedError):
        rows[0].overflowing
    assert rows[1].overflowing == []


def test_transformers_patch_round_trips_local_tokenizer(tmp_path) -> None:
    """Verify patch and unpatch replace a local Transformers backend exactly once."""
    transformers = pytest.importorskip("transformers")

    json_path = tmp_path / "tokenizer.json"
    json_path.write_text(json.dumps(_tokenizer_json()), encoding="utf-8")
    config = {"tokenizer_class": "PreTrainedTokenizerFast"}
    (tmp_path / "tokenizer_config.json").write_text(
        json.dumps(config), encoding="utf-8"
    )

    import snaptokens
    from snaptokens._compat import _TokenizerShim

    snaptokens.patch_transformers()
    try:
        tokenizer = transformers.AutoTokenizer.from_pretrained(tmp_path)
        assert isinstance(tokenizer._tokenizer, _TokenizerShim)
        assert tokenizer("ab")["input_ids"] == [2]
        assert tokenizer.decode([2]) == "ab"
    finally:
        snaptokens.unpatch_transformers()

    tokenizer = transformers.AutoTokenizer.from_pretrained(tmp_path)
    assert not isinstance(tokenizer._tokenizer, _TokenizerShim)


@pytest.mark.parametrize("method", ["encode", "encode_batch", "encode_batch_flat"])
def test_encoding_releases_gil_and_allows_concurrent_settings(method):
    """Other Python threads must progress without a GIL/state-lock deadlock."""
    script = textwrap.dedent("""
        import json, sys, threading
        from snaptokens import Tokenizer

        tokenizer = Tokenizer.from_json_str(sys.argv[1])
        ready, begin, changed = (threading.Event() for _ in range(3))
        def configure():
            ready.set()
            begin.wait()
            tokenizer.enable_truncation(17)
            changed.set()
        thread = threading.Thread(target=configure, daemon=True)
        thread.start()
        ready.wait()
        # Prevent a Python bytecode timeslice from masquerading as a native GIL release.
        sys.setswitchinterval(10)
        text = 'ab' * 40000
        begin.set()
        method = sys.argv[2]
        if method == 'encode':
            ids = tokenizer.encode(text).ids
        elif method == 'encode_batch':
            ids = tokenizer.encode_batch([text])[0].ids
        else:
            data, offsets = tokenizer.encode_batch_flat([text])
            ids = list(memoryview(data).cast('I'))
            assert list(memoryview(offsets).cast('Q')) == [0, len(ids)]
        assert changed.is_set(), 'Python settings thread did not progress during encode'
        assert len(ids) in (17, 40000) and ids == [2] * len(ids)
        thread.join()
        assert tokenizer.encode(text).ids == [2] * 17
    """)
    subprocess.run([sys.executable, "-c", script, json.dumps(_tokenizer_json()), method],
                   check=True, timeout=20)


def test_special_tokens_and_configured_metadata():
    """Keep post-processing, truncation, and nondefault padding exact in native calls."""
    Reference = pytest.importorskip("tokenizers").Tokenizer

    config = _tokenizer_json()
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
    encoded = json.dumps(config)
    tokenizer = Tokenizer.from_json_str(encoded)
    reference = Reference.from_str(encoded)
    rows = ["ab", "a", ""]
    assert [e.ids for e in tokenizer.encode_batch(rows, add_special_tokens=True)] == [
        e.ids for e in reference.encode_batch(rows, add_special_tokens=True)]
    raw = tokenizer.encode("ab")
    assert tokenizer.post_process(raw, add_special_tokens=True).ids == [3, 2, 4]

    tokenizer.enable_truncation(2)
    tokenizer.enable_padding(direction="left", pad_id=0, pad_type_id=7, length=4)
    scalar = tokenizer.encode("ab", add_special_tokens=True)
    batch = tokenizer.encode_batch(["ab"], add_special_tokens=True)[0]
    for result in (scalar, batch):
        assert result.ids == [0, 0, 3, 2]
        assert result.attention_mask == [0, 0, 1, 1]
        assert result.type_ids == [7, 7, 0, 0]
        assert result.special_tokens_mask == [0, 0, 0, 0]
    packed, offsets = tokenizer.encode_batch_flat(["ab"], add_special_tokens=True)
    assert list(memoryview(packed).cast("I")) == [3, 2]
    assert list(memoryview(offsets).cast("Q")) == [0, 2]
