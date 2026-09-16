import copy
import json
import pickle
import subprocess
import sys
from pathlib import Path

import pytest

from snaptokens import Tokenizer


def test_native_package_json_tkz_and_flat_batch(tokenizer_file) -> None:
    """Keep local loading, native sidecars, and packed batches equivalent."""
    json_path = tokenizer_file

    tokenizer = Tokenizer.from_file(str(json_path), tkz_cache=True)
    assert Tokenizer.__module__ == "snaptokens._native"
    assert tokenizer.encode("ab").ids == [2]
    assert tokenizer.decode([2]) == "ab"

    rows = tokenizer.encode_batch(["ab", "a", ""])
    assert [row.ids for row in rows] == [[2], [0], []]

    packed_ids, packed_offsets = tokenizer.encode_batch_flat(["ab", "a", ""])
    assert list(memoryview(packed_ids).cast("I")) == [2, 0]
    assert list(memoryview(packed_offsets).cast("Q")) == [0, 1, 2, 2]

    tkz_path = json_path.with_suffix(".tkz")
    assert tkz_path.is_file()
    cached = Tokenizer.from_file(str(tkz_path))
    assert cached.encode("ab").ids == [2]


def test_encode_paths_report_truncation_instead_of_empty_overflow(tokenizer) -> None:
    """Keep configured truncation from silently dropping tokens as no overflow."""
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


def test_transformers_patch_round_trips_local_tokenizer(tokenizer_file) -> None:
    """Verify patch and unpatch replace a local Transformers backend exactly once."""
    transformers = pytest.importorskip("transformers")

    tmp_path = tokenizer_file.parent
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
def test_encoding_releases_gil_and_allows_concurrent_settings(method, tokenizer_json):
    """Other Python threads must progress without a GIL/state-lock deadlock."""
    subprocess.run(
        [sys.executable, str(Path(__file__).with_name("encoding_worker.py")), tokenizer_json, method],
        check=True, timeout=20,
    )


def test_post_processing_matches_reference(template_json):
    Reference = pytest.importorskip("tokenizers").Tokenizer

    encoded = template_json
    tokenizer = Tokenizer.from_json_str(encoded)
    reference = Reference.from_str(encoded)
    rows = ["ab", "a", ""]
    assert [e.ids for e in tokenizer.encode_batch(rows, add_special_tokens=True)] == [
        e.ids for e in reference.encode_batch(rows, add_special_tokens=True)]
    raw = tokenizer.encode("ab")
    assert tokenizer.post_process(raw, add_special_tokens=True).ids == [3, 2, 4]


@pytest.mark.parametrize("method", ["encode", "encode_batch", "encode_batch_flat"])
def test_truncation_preserves_template_special_tokens(template_json, method):
    from tokenizers import Tokenizer as Reference

    tokenizer = Tokenizer.from_json_str(template_json)
    reference = Reference.from_str(template_json)
    texts = ["ab", "aba", "baba", "", "a"]
    for max_length, direction, special in [
        (2, "right", True), (3, "right", True), (3, "left", True),
        (4, "right", True), (2, "right", False), (2, "left", False),
    ]:
        tokenizer.enable_truncation(max_length, direction=direction)
        reference.enable_truncation(max_length, direction=direction)
        expected = reference.encode_batch(texts, add_special_tokens=special)
        if method == "encode_batch_flat":
            packed, offsets = tokenizer.encode_batch_flat(texts, add_special_tokens=special)
            ids = list(memoryview(packed).cast("I"))
            ends = list(memoryview(offsets).cast("Q"))
            assert len(ends) == len(texts) + 1
            assert ends[0] == 0 and ends[-1] == len(ids)
            actual = [ids[start:end] for start, end in zip(ends, ends[1:])]
        else:
            rows = (tokenizer.encode_batch(texts, add_special_tokens=special)
                    if method == "encode_batch" else
                    [tokenizer.encode(text, add_special_tokens=special) for text in texts])
            actual = [row.ids for row in rows]
            for row, reference_row in zip(rows, expected):
                if reference_row.overflowing:
                    with pytest.raises(NotImplementedError):
                        _ = row.overflowing
                else:
                    assert row.overflowing == []
        assert actual == [row.ids for row in expected], (max_length, direction, special)

    tokenizer.enable_truncation(1)
    with pytest.raises(ValueError, match="cannot fit 2 special tokens"):
        getattr(tokenizer, method)("ab" if method == "encode" else ["ab"],
                                   add_special_tokens=True)


def test_padding_metadata_and_flat_offsets(tokenizer):
    tokenizer.enable_truncation(1)
    tokenizer.enable_padding(direction="left", pad_id=0, pad_type_id=7, length=4)
    for row in [tokenizer.encode("aba"), *tokenizer.encode_batch(["aba"])]:
        assert row.ids == [0, 0, 0, 2]
        assert row.attention_mask == [0, 0, 0, 1]
        assert row.type_ids == [7, 7, 7, 0]
        assert row.special_tokens_mask == [0, 0, 0, 0]
    packed, offsets = tokenizer.encode_batch_flat(["aba", "a", ""])
    assert list(memoryview(packed).cast("I")) == [2, 0]
    assert list(memoryview(offsets).cast("Q")) == [0, 1, 2, 2]


@pytest.mark.parametrize("initial,replacement", [
    (None, "[SEP] $A [CLS]"),
    ("[CLS] $A [SEP]", "[SEP] $A [CLS]"),
    ("[CLS] $A [SEP]", None),
])
def test_shim_round_trips_current_post_processor(tmp_path, tokenizer_config, initial, replacement):
    tokenizers = pytest.importorskip("tokenizers")
    from snaptokens._compat import _TokenizerShim

    config = tokenizer_config
    config["model"]["vocab"].update({"[CLS]": 3, "[SEP]": 4})
    reference = tokenizers.Tokenizer.from_str(json.dumps(config))
    if initial is not None:
        reference.post_processor = tokenizers.processors.TemplateProcessing(
            single=initial, special_tokens=[("[CLS]", 3), ("[SEP]", 4)]
        )
    tokenizer = _TokenizerShim(reference)
    processor = None
    if replacement is not None:
        processor = tokenizers.processors.TemplateProcessing(
            single=replacement, special_tokens=[("[CLS]", 3), ("[SEP]", 4)]
        )
    reference.post_processor = processor
    tokenizer.post_processor = processor
    expected_config = json.loads(reference.to_str())["post_processor"]
    expected_ids = reference.encode("ab").ids
    assert tokenizer.encode("ab").ids == expected_ids

    path = tmp_path / "saved.json"
    tokenizer.save(str(path))
    restored = [
        _TokenizerShim.from_str(tokenizer.to_str()),
        _TokenizerShim.from_file(str(path)),
        _TokenizerShim(tokenizer),
        copy.deepcopy(tokenizer),
        pickle.loads(pickle.dumps(tokenizer)),
    ]
    assert json.loads(tokenizer.to_str())["post_processor"] == expected_config
    tokenizer.post_processor = None
    for clone in restored:
        assert json.loads(clone.to_str())["post_processor"] == expected_config
        assert clone.encode("ab").ids == expected_ids
