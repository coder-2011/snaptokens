import copy
import json
import pickle
import subprocess
import sys
from pathlib import Path

import pytest

from snaptokens import Tokenizer


def _unigram_json() -> dict:
    return {
        "added_tokens": [],
        "normalizer": None,
        "pre_tokenizer": {
            "type": "Sequence",
            "pretokenizers": [
                {"type": "WhitespaceSplit"},
                {"type": "Metaspace", "replacement": "▁", "add_prefix_space": True},
            ],
        },
        "post_processor": None,
        "decoder": {"type": "Metaspace", "replacement": "▁", "add_prefix_space": True},
        "model": {
            "type": "Unigram",
            "unk_id": 0,
            "vocab": [["<unk>", 0.0], ["▁hello", 3.0], ["▁world", 3.0]],
        },
    }


def test_native_package_json_tkz_and_flat_batch(tokenizer_file) -> None:
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
    assert cached.truncation is None
    assert cached.padding is None

    st_tokenizer = Tokenizer.from_file(str(json_path), st_cache=True)
    st_path = json_path.with_suffix(".st")
    assert st_path.is_file()
    assert Tokenizer.from_file(str(st_path)).encode("ab").ids == [2]
    assert st_tokenizer.encode("ab").ids == [2]


def test_python_json_unigram_matches_tokenizers_and_rejects_native_model(
    tmp_path,
) -> None:
    Reference = pytest.importorskip("tokenizers").Tokenizer
    encoded = json.dumps(_unigram_json())
    tokenizer = Tokenizer.from_json_str(encoded)
    reference = Reference.from_str(encoded)
    inputs = ["hello world", " hello  world", "unknown"]

    assert [row.ids for row in tokenizer.encode_batch(inputs)] == [
        reference.encode(text).ids for text in inputs
    ]
    assert tokenizer.decode(tokenizer.encode("hello world").ids) == reference.decode(
        reference.encode("hello world").ids
    )

    json_path = tmp_path / "tokenizer.json"
    json_path.write_text(encoded, encoding="utf-8")
    from_file = Tokenizer.from_file(str(json_path))
    assert from_file.encode("hello world").ids == [1, 2]
    with pytest.raises(
        ValueError, match=r"Unigram tokenizers cannot use \.tkz caching yet"
    ):
        Tokenizer.from_file(str(json_path), tkz_cache=True)
    assert not json_path.with_suffix(".tkz").exists()
    cached = Tokenizer.from_file(str(json_path), st_cache=True)
    st_path = json_path.with_suffix(".st")
    assert st_path.is_file()
    direct = Tokenizer.from_file(str(st_path))
    json_path.unlink()
    sidecar_only = Tokenizer.from_file(str(json_path), st_cache=True)
    expected = [reference.encode(text).ids for text in inputs]
    for loaded in (cached, direct, sidecar_only):
        assert [row.ids for row in loaded.encode_batch(inputs)] == expected
        packed, offsets = loaded.encode_batch_flat(inputs)
        assert list(memoryview(packed).cast("I")) == [v for row in expected for v in row]
        expected_offsets = [0]
        for row in expected:
            expected_offsets.append(expected_offsets[-1] + len(row))
        assert list(memoryview(offsets).cast("Q")) == expected_offsets
        assert loaded.decode(loaded.encode("hello world").ids) == reference.decode(
            reference.encode("hello world").ids
        )

    model_path = tmp_path / "tokenizer.model"
    with pytest.raises(
        ValueError, match=r"native SentencePiece \.model files are not supported"
    ):
        Tokenizer.from_file(str(model_path))


def test_encode_paths_report_truncation_instead_of_empty_overflow(tokenizer) -> None:
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
    subprocess.run(
        [
            sys.executable,
            str(Path(__file__).with_name("encoding_worker.py")),
            tokenizer_json,
            method,
        ],
        check=True,
        timeout=20,
    )


def test_post_processing_matches_reference(template_json):
    Reference = pytest.importorskip("tokenizers").Tokenizer

    encoded = template_json
    tokenizer = Tokenizer.from_json_str(encoded)
    reference = Reference.from_str(encoded)
    rows = ["ab", "a", ""]
    assert [e.ids for e in tokenizer.encode_batch(rows, add_special_tokens=True)] == [
        e.ids for e in reference.encode_batch(rows, add_special_tokens=True)
    ]
    raw = tokenizer.encode("ab")
    assert tokenizer.post_process(raw, add_special_tokens=True).ids == [3, 2, 4]


@pytest.mark.parametrize("method", ["encode", "encode_batch", "encode_batch_flat"])
def test_truncation_preserves_template_special_tokens(template_json, method):
    from tokenizers import Tokenizer as Reference

    tokenizer = Tokenizer.from_json_str(template_json)
    reference = Reference.from_str(template_json)
    texts = ["ab", "aba", "baba", "", "a"]
    for max_length, direction, special in [
        (2, "right", True),
        (3, "right", True),
        (3, "left", True),
        (4, "right", True),
        (2, "right", False),
        (2, "left", False),
    ]:
        tokenizer.enable_truncation(max_length, direction=direction)
        reference.enable_truncation(max_length, direction=direction)
        expected = reference.encode_batch(texts, add_special_tokens=special)
        if method == "encode_batch_flat":
            packed, offsets = tokenizer.encode_batch_flat(
                texts, add_special_tokens=special
            )
            ids = list(memoryview(packed).cast("I"))
            ends = list(memoryview(offsets).cast("Q"))
            assert len(ends) == len(texts) + 1
            assert ends[0] == 0 and ends[-1] == len(ids)
            actual = [ids[start:end] for start, end in zip(ends, ends[1:])]
        else:
            rows = (
                tokenizer.encode_batch(texts, add_special_tokens=special)
                if method == "encode_batch"
                else [
                    tokenizer.encode(text, add_special_tokens=special) for text in texts
                ]
            )
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
        getattr(tokenizer, method)(
            "ab" if method == "encode" else ["ab"], add_special_tokens=True
        )


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


@pytest.mark.parametrize(
    "initial,replacement",
    [
        (None, "[SEP] $A [CLS]"),
        ("[CLS] $A [SEP]", "[SEP] $A [CLS]"),
        ("[CLS] $A [SEP]", None),
    ],
)
def test_shim_round_trips_current_post_processor(
    tmp_path, tokenizer_config, initial, replacement
):
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
    cached = Tokenizer.from_file(str(path), tkz_cache=True)
    for native in [cached, Tokenizer.from_file(str(path.with_suffix(".tkz")))]:
        processor = native.post_processor
        assert (
            None if processor is None else json.loads(str(processor))
        ) == expected_config
        native.post_processor = None
        native.post_processor = processor
        with pytest.raises(ValueError, match="single template"):
            native.post_processor = '{"type":"TemplateProcessing","single":42}'
        assert native.encode("ab", add_special_tokens=True).ids == expected_ids
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


@pytest.mark.parametrize("side", ["left", "right"])
def test_settings_survive_json_file_copy_and_pickle(tmp_path, tokenizer_json, side):
    from snaptokens._compat import _TokenizerShim

    original = _TokenizerShim(tokenizer_json)
    original.enable_truncation(2, direction=side, strategy="only_first")
    original.enable_padding(direction=side, length=4, pad_id=0, pad_type_id=7)
    saved = original.to_str()
    reference = pytest.importorskip("tokenizers").Tokenizer.from_str(saved)
    path = tmp_path / "tokenizer.json"
    original.save(str(path))
    old_state = (tokenizer_json, original.truncation, original.padding, True)
    legacy = object.__new__(_TokenizerShim)
    legacy.__setstate__(old_state)
    assert legacy.encode_special_tokens is False
    disabled_legacy = object.__new__(_TokenizerShim)
    disabled_legacy.__setstate__((saved, None, None, False))
    assert disabled_legacy.truncation is None
    assert disabled_legacy.padding is None
    restored = [
        _TokenizerShim.from_str(saved),
        _TokenizerShim.from_file(str(path)),
        _TokenizerShim(original),
        copy.deepcopy(original),
        pickle.loads(pickle.dumps(original)),
        legacy,
        Tokenizer.from_json_str(saved),
        Tokenizer.from_file(str(path)),
        Tokenizer.from_file(str(path), tkz_cache=True),
        Tokenizer.from_file(str(path), tkz_cache=True),  # Reuse the sidecar.
        Tokenizer.from_file(str(path.with_suffix(".tkz"))),
    ]
    for clone in restored:
        assert clone.truncation == original.truncation
        assert clone.padding == original.padding
        for text in ["", "a", "ababab"]:
            assert clone.encode(text).ids == reference.encode(text).ids
    original.no_padding()
    original.no_truncation()
    assert restored[0].truncation is not None
    assert restored[0].padding is not None
    disabled = _TokenizerShim.from_str(original.to_str())
    assert disabled.truncation is None
    assert disabled.padding is None


def test_shim_vocabulary_flags_and_unsupported_special_encoding(tokenizer_config):
    from snaptokens._compat import _TokenizerShim

    tokenizer_config["added_tokens"] = [
        {
            "id": 0,
            "content": "a",
            "special": True,
            "normalized": False,
            "single_word": False,
            "lstrip": False,
            "rstrip": False,
        },
        {
            "id": 7,
            "content": "[NEW]",
            "special": True,
            "normalized": False,
            "single_word": False,
            "lstrip": False,
            "rstrip": False,
        },
    ]
    shim = _TokenizerShim(json.dumps(tokenizer_config))
    assert shim.get_vocab(False) == tokenizer_config["model"]["vocab"]
    assert shim.get_vocab(True) == {**tokenizer_config["model"]["vocab"], "[NEW]": 7}
    assert shim.get_vocab_size(False) == 3
    assert shim.get_vocab_size(True) == 4
    shim.encode_special_tokens = False
    with pytest.raises(NotImplementedError):
        shim.encode_special_tokens = True
    assert shim.encode_special_tokens is False


@pytest.mark.parametrize("length", [None, 4])
def test_legacy_padding_json_restores_and_saves_canonical_settings(
    tmp_path, tokenizer_json, length
):
    from snaptokens._compat import _TokenizerShim

    original = _TokenizerShim(tokenizer_json)
    original.enable_padding(length=length, direction="left", pad_id=0)
    legacy_config = json.loads(original.to_str())
    legacy_config["padding"] = original.padding
    legacy_json = json.dumps(legacy_config)
    path = tmp_path / "legacy.json"
    path.write_text(legacy_json)
    for restored in [
        _TokenizerShim.from_str(legacy_json),
        _TokenizerShim.from_file(str(path)),
    ]:
        assert restored.padding == original.padding
        assert restored.encode("ab").ids == original.encode("ab").ids
        assert json.loads(restored.to_str())["padding"]["strategy"] == (
            "BatchLongest" if length is None else {"Fixed": length}
        )


def test_invalid_settings_are_rejected_without_mutating_state(tokenizer):
    tokenizer.enable_truncation(2)
    tokenizer.enable_padding(length=4)
    trunc, pad = tokenizer.truncation, tokenizer.padding
    for configure in [tokenizer.enable_truncation, tokenizer.enable_padding]:
        args = (2,) if configure == tokenizer.enable_truncation else ()
        with pytest.raises(ValueError):
            configure(*args, direction="banana")
    with pytest.raises(ValueError):
        tokenizer.enable_truncation(2, strategy="banana")
    with pytest.raises(NotImplementedError):
        tokenizer.enable_truncation(2, strategy="only_second")
    with pytest.raises(NotImplementedError):
        tokenizer.enable_truncation(2, stride=1)
    assert tokenizer.truncation == trunc
    assert tokenizer.padding == pad
    encoding = tokenizer.encode("ab")
    for mutate in [encoding.truncate, encoding.pad]:
        with pytest.raises(ValueError):
            mutate(2, direction="banana")


@pytest.mark.parametrize(
    "extra,error",
    [
        ({"direction": "banana"}, ValueError),
        ({"strategy": "only_second"}, NotImplementedError),
        ({"stride": 1}, NotImplementedError),
    ],
)
def test_loaded_truncation_settings_keep_validation(
    tmp_path, tokenizer_config, extra, error
):
    tokenizer_config["truncation"] = {"max_length": 2, **extra}
    saved = json.dumps(tokenizer_config)
    path = tmp_path / "tokenizer.json"
    path.write_text(saved)
    with pytest.raises(error):
        Tokenizer.from_json_str(saved)
    for cached in [False, True]:
        with pytest.raises(error):
            Tokenizer.from_file(str(path), tkz_cache=cached)


@pytest.mark.parametrize("method", ["encode", "encode_batch", "encode_batch_flat"])
def test_truncation_rejects_invalid_discarded_input(tokenizer_config, method):
    tokenizer_config["pre_tokenizer"] = {
        "type": "Split",
        "pattern": {"String": " "},
        "behavior": "Removed",
        "invert": False,
    }
    tokenizer = Tokenizer.from_json_str(json.dumps(tokenizer_config))
    for direction in ["left", "right"]:
        tokenizer.enable_truncation(1, direction=direction)
        for text in ["ab ab z", "z ab ab"]:
            args = text if method == "encode" else [text]
            with pytest.raises(ValueError, match="not in vocabulary"):
                getattr(tokenizer, method)(args)
    assert tokenizer.encode_batch([]) == []
    ids, offsets = tokenizer.encode_batch_flat([])
    assert len(ids) == 0
    assert list(memoryview(offsets).cast("Q")) == [0]


def test_regex_error_becomes_value_error(tokenizer_config) -> None:
    tokenizer_config["pre_tokenizer"] = {
        "type": "Split",
        "pattern": {"Regex": r"(?i)(a|b|ab)*(?>c)|a"},
        "behavior": "Isolated",
        "invert": False,
    }
    tokenizer = Tokenizer.from_json_str(json.dumps(tokenizer_config))
    with pytest.raises(ValueError, match="regex matching failed:.*backtrack"):
        tokenizer.encode("ab" * 20)
