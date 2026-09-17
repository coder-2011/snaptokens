import json

import pytest

from snaptokens import Tokenizer


def _pair_template_config() -> dict:
    return {
        "version": "1.0",
        "added_tokens": [],
        "normalizer": None,
        "pre_tokenizer": {"type": "WhitespaceSplit"},
        "model": {
            "type": "BPE",
            "vocab": {"a": 0, "b": 1, "<s>": 2, "</s>": 3},
            "merges": [],
        },
        "post_processor": {
            "type": "TemplateProcessing",
            "single": [
                {"SpecialToken": {"id": "<s>", "type_id": 0}},
                {"Sequence": {"id": "A", "type_id": 0}},
            ],
            "pair": [
                {"SpecialToken": {"id": "<s>", "type_id": 0}},
                {"Sequence": {"id": "A", "type_id": 0}},
                {"SpecialToken": {"id": "</s>", "type_id": 1}},
                {"Sequence": {"id": "B", "type_id": 1}},
                {"SpecialToken": {"id": "</s>", "type_id": 1}},
            ],
            "special_tokens": {
                "<s>": {"id": "<s>", "ids": [2], "tokens": ["<s>"]},
                "</s>": {"id": "</s>", "ids": [3], "tokens": ["</s>"]},
            },
        },
        "decoder": None,
    }


def _load(tmp_path, config) -> Tokenizer:
    path = tmp_path / "tokenizer.json"
    path.write_text(json.dumps(config), encoding="utf-8")
    return Tokenizer.from_file(str(path))


def _load_reference(config):
    Reference = pytest.importorskip("tokenizers").Tokenizer
    return Reference.from_str(json.dumps(config))


PAIR_INPUTS = [("a b", "b a"), ("a", "b"), ("", "a b"), ("a b", ""), ("", "")]


@pytest.mark.parametrize("add_special_tokens", [True, False])
def test_pair_template_matches_tokenizers(tmp_path, add_special_tokens) -> None:
    config = _pair_template_config()
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)

    for first, second in PAIR_INPUTS:
        ours = tokenizer.encode(
            first, pair=second, add_special_tokens=add_special_tokens
        )
        theirs = reference.encode(
            first, second, add_special_tokens=add_special_tokens
        )
        assert ours.ids == theirs.ids
        assert ours.type_ids == theirs.type_ids
        assert ours.special_tokens_mask == theirs.special_tokens_mask
        assert ours.attention_mask == theirs.attention_mask
        assert ours.n_sequences == theirs.n_sequences


def test_pair_without_post_processor_matches_tokenizers(tmp_path) -> None:
    config = _pair_template_config()
    config["post_processor"] = None
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)

    ours = tokenizer.encode("a b", pair="b a", add_special_tokens=True)
    theirs = reference.encode("a b", "b a", add_special_tokens=True)
    assert ours.ids == theirs.ids
    assert ours.type_ids == theirs.type_ids
    assert ours.special_tokens_mask == theirs.special_tokens_mask


def test_pair_with_padding_matches_tokenizers(tmp_path) -> None:
    config = _pair_template_config()
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)
    tokenizer.enable_padding(length=12)
    reference.enable_padding(length=12)

    ours = tokenizer.encode("a b", pair="b", add_special_tokens=True)
    theirs = reference.encode("a b", "b", add_special_tokens=True)
    assert ours.ids == theirs.ids
    assert ours.type_ids == theirs.type_ids
    assert ours.attention_mask == theirs.attention_mask
    assert ours.special_tokens_mask == theirs.special_tokens_mask


def test_single_padding_marks_special_tokens_mask_like_tokenizers(tmp_path) -> None:
    config = _pair_template_config()
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)
    tokenizer.enable_padding(length=6)
    reference.enable_padding(length=6)

    ours = tokenizer.encode("a b", add_special_tokens=True)
    theirs = reference.encode("a b", add_special_tokens=True)
    assert ours.ids == theirs.ids
    assert ours.special_tokens_mask == theirs.special_tokens_mask
    assert ours.type_ids == theirs.type_ids


@pytest.mark.parametrize("add_special_tokens", [True, False])
def test_single_template_metadata_matches_tokenizers(
    tmp_path, add_special_tokens
) -> None:
    # Templates stamp type IDs on singles too, even without special tokens.
    config = _pair_template_config()
    config["post_processor"]["single"] = [
        {"Sequence": {"id": "A", "type_id": 0}},
        {"SpecialToken": {"id": "</s>", "type_id": 1}},
    ]
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)

    ours = tokenizer.encode("a b", add_special_tokens=add_special_tokens)
    theirs = reference.encode("a b", add_special_tokens=add_special_tokens)
    assert ours.ids == theirs.ids
    assert ours.type_ids == theirs.type_ids
    assert ours.special_tokens_mask == theirs.special_tokens_mask


def test_pair_batch_matches_tokenizers(tmp_path) -> None:
    config = _pair_template_config()
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)

    pairs = [("a b", "b a"), ("a", "b"), ("b b", "a"), ("", "a")]
    ours = tokenizer.encode_pair_batch(pairs, add_special_tokens=True)
    theirs = reference.encode_batch(pairs, add_special_tokens=True)
    assert len(ours) == len(theirs)
    for our_row, their_row in zip(ours, theirs):
        assert our_row.ids == their_row.ids
        assert our_row.type_ids == their_row.type_ids
        assert our_row.special_tokens_mask == their_row.special_tokens_mask


def test_pair_template_survives_tkz_cache(tmp_path) -> None:
    path = tmp_path / "tokenizer.json"
    path.write_text(json.dumps(_pair_template_config()), encoding="utf-8")
    Tokenizer.from_file(str(path), tkz_cache=True)
    cached = Tokenizer.from_file(str(path.with_suffix(".tkz")))
    encoding = cached.encode("a b", pair="b a", add_special_tokens=True)
    assert encoding.ids == [2, 0, 1, 3, 1, 0, 3]
    assert encoding.type_ids == [0, 0, 0, 1, 1, 1, 1]


def test_pair_with_truncation_raises(tmp_path) -> None:
    tokenizer = _load(tmp_path, _pair_template_config())
    tokenizer.enable_truncation(4)
    with pytest.raises(NotImplementedError):
        tokenizer.encode("a b", pair="b a")


def test_num_special_tokens_to_add_pair(tmp_path) -> None:
    config = _pair_template_config()
    tokenizer = _load(tmp_path, config)
    reference = _load_reference(config)
    assert tokenizer.num_special_tokens_to_add(True) == 3
    assert tokenizer.num_special_tokens_to_add(
        True
    ) == reference.num_special_tokens_to_add(True)
    assert tokenizer.num_special_tokens_to_add(
        False
    ) == reference.num_special_tokens_to_add(False)


def test_post_process_pair(tmp_path) -> None:
    tokenizer = _load(tmp_path, _pair_template_config())
    first = tokenizer.encode("a b")
    second = tokenizer.encode("b a")
    combined = tokenizer.post_process(first, second, add_special_tokens=True)
    assert combined.ids == [2, 0, 1, 3, 1, 0, 3]
    assert combined.type_ids == [0, 0, 0, 1, 1, 1, 1]
    assert combined.n_sequences == 2


def test_transformers_patch_encodes_pairs(tmp_path) -> None:
    transformers = pytest.importorskip("transformers")

    path = tmp_path / "tokenizer.json"
    path.write_text(json.dumps(_pair_template_config()), encoding="utf-8")
    (tmp_path / "tokenizer_config.json").write_text(
        json.dumps(
            {
                "tokenizer_class": "PreTrainedTokenizerFast",
                "model_input_names": [
                    "input_ids",
                    "token_type_ids",
                    "attention_mask",
                ],
            }
        ),
        encoding="utf-8",
    )

    import snaptokens

    reference = transformers.AutoTokenizer.from_pretrained(tmp_path)
    expected = reference("a b", "b a")

    snaptokens.patch_transformers()
    try:
        patched = transformers.AutoTokenizer.from_pretrained(tmp_path)
        result = patched("a b", "b a")
        assert result["input_ids"] == expected["input_ids"]
        assert result["token_type_ids"] == expected["token_type_ids"]
        assert result["attention_mask"] == expected["attention_mask"]
    finally:
        snaptokens.unpatch_transformers()
