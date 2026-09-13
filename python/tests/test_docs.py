import snaptokens
from snaptokens._native import DecodeStream, Encoding, Tokenizer


def test_public_runtime_docstrings_are_exposed() -> None:
    documented = (
        snaptokens,
        Encoding,
        Tokenizer,
        DecodeStream,
        Tokenizer.from_file,
        Tokenizer.encode,
        Tokenizer.encode_batch,
        Tokenizer.decode,
        DecodeStream.step,
    )
    for value in documented:
        assert value.__doc__ and value.__doc__.strip()
