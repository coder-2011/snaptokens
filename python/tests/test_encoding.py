"""Exercise metadata mutation through the public Python encoding contract."""

import pytest

from snaptokens._native import Encoding


def fields(encoding):
    """Materialize all supported fields as a caller such as Transformers does."""
    return (encoding.ids, encoding.attention_mask, encoding.type_ids,
            encoding.special_tokens_mask)


@pytest.mark.parametrize("side", ["left", "right"])
def test_metadata_survives_mutation_padding_truncation_and_merge(side):
    """Preserve explicit values when implicit defaults become mixed or are sliced."""
    encoding = Encoding([10, 20, 30])
    encoding.attention_mask = [1, 0, 1]
    encoding.type_ids = [2, 3, 4]
    encoding.special_tokens_mask = [0, 1, 0]
    encoding.sequence_ids = [0, None, 1]
    encoding.word_ids = [5, None, 6]
    encoding.pad(5, direction=side, pad_id=9, pad_type_id=7)
    expected = ([10, 20, 30], [1, 0, 1], [2, 3, 4], [0, 1, 0])
    padding = ([9, 9], [0, 0], [7, 7], [0, 0])
    padded = tuple(p + v if side == "left" else v + p
                   for v, p in zip(expected, padding))
    assert fields(encoding) == padded
    encoding.truncate(3, direction="right" if side == "left" else "left")
    sliced = tuple(v[:3] if side == "left" else v[-3:] for v in padded)
    assert fields(encoding) == sliced
    tail = Encoding([40, 50])
    merged = Encoding.merge([encoding, tail])
    assert fields(merged) == tuple(a + b for a, b in zip(sliced, fields(tail)))
    assert merged.n_sequences == 2
    for name in ("sequence_ids", "word_ids", "words"):
        with pytest.raises(NotImplementedError):
            getattr(merged, name)


def test_metadata_lengths_are_independent_and_getters_return_copies():
    """Setting IDs must not resize metadata, and returned lists must not alias storage."""
    encoding = Encoding([1, 2, 3])
    mask = encoding.attention_mask
    mask[0] = 9
    assert encoding.attention_mask == [1, 1, 1]
    encoding.ids = [4, 5]
    assert encoding.attention_mask == [1, 1, 1]
    encoding.attention_mask = [7, 8]
    encoding.truncate(1)
    assert fields(encoding) == ([4], [7], [0], [0])
    encoding.pad(3, pad_type_id=0)
    assert fields(encoding) == ([4, 0, 0], [7, 0, 0], [0, 0, 0], [0, 0, 0])


def test_empty_and_repeated_metadata_merge():
    """Keep defaults exact across empty rows, identical rows, and custom assignments."""
    merged = Encoding.merge([Encoding([]), Encoding([1]), Encoding([2, 3])])
    assert fields(merged) == ([1, 2, 3], [1, 1, 1], [0, 0, 0], [0, 0, 0])
    merged.type_ids = [7, 8, 9]
    assert merged.type_ids == [7, 8, 9]
    assert fields(Encoding.merge([])) == ([], [], [], [])


def test_overflowing_is_empty_until_truncation_discards_tokens():
    """Reject overflow reads only once truncation actually dropped tokens."""
    encoding = Encoding([1, 2, 3])
    assert encoding.overflowing == []
    encoding.truncate(3)
    assert encoding.overflowing == []
    encoding.truncate(2)
    with pytest.raises(NotImplementedError):
        encoding.overflowing
    with pytest.raises(NotImplementedError):
        encoding.overflowing = []


def test_truncation_loss_survives_padding_and_merge():
    """Keep the discarded-token signal attached through later encoding edits."""
    truncated = Encoding([1, 2, 3])
    truncated.truncate(1)
    truncated.pad(4)
    with pytest.raises(NotImplementedError):
        truncated.overflowing
    with pytest.raises(NotImplementedError):
        Encoding.merge([Encoding([9]), truncated]).overflowing
    assert Encoding.merge([Encoding([9]), Encoding([8])]).overflowing == []
