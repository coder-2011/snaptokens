"""Fast exact BPE tokenization from local Hugging Face tokenizer files.

Use :class:`Tokenizer` directly for local `tokenizer.json`, `.tkz`, or `.st` files,
or call :func:`patch_transformers` to replace a Transformers fast-tokenizer
backend for supported single-sequence operations.
"""

from snaptokens._native import Tokenizer

__all__ = ["Tokenizer", "patch_transformers", "unpatch_transformers"]


_patched = False
_originals: dict = {}


def _swap_backend(tokenizer, shim_cls):
    """Replace the backend ``_tokenizer`` with a snaptokens shim if needed."""
    backend = getattr(tokenizer, "_tokenizer", None)
    if backend is not None and not isinstance(backend, shim_cls):
        tokenizer._tokenizer = shim_cls(backend)
    return tokenizer


def patch_transformers() -> None:
    """
    Monkey-patch ``tokenizers.Tokenizer`` so that the
    ``transformers`` library uses snaptokens for encoding.

    Call this before any ``AutoTokenizer.from_pretrained``
    invocation::

        import snaptokens
        snaptokens.patch_transformers()

        from transformers import AutoTokenizer
        tok = AutoTokenizer.from_pretrained(
            "meta-llama/Llama-3.1-8B"
        )

    Supports both transformers v4 (``tokenization_utils_fast``)
    and v5+ (``tokenization_utils_tokenizers``).
    """
    global _patched
    if _patched:
        print("[snaptokens] patch_transformers: already patched.")
        return

    from snaptokens._compat import _TokenizerShim
    from snaptokens._native import DecodeStream

    import tokenizers.decoders as _td
    _v5_patched = False
    try:
        from transformers.tokenization_utils_tokenizers import TokenizersBackend

        _orig_fp = TokenizersBackend.from_pretrained

        @classmethod
        def _patched_from_pretrained(cls, *args, **kwargs):
            """Wrap a fully initialized Transformers tokenizer with Snaptokens."""
            tokenizer = _orig_fp.__func__(cls, *args, **kwargs)
            return _swap_backend(tokenizer, _TokenizerShim)

        _originals["TokenizersBackend.from_pretrained"] = _orig_fp
        TokenizersBackend.from_pretrained = _patched_from_pretrained
        _v5_patched = True
    except ImportError:
        pass

    # In transformers v4, PreTrainedTokenizerFast.__init__ loads the
    # backend via TokenizerFast.from_file() from this module.
    if not _v5_patched:
        try:
            import transformers.tokenization_utils_fast as _tuf

            _originals["tokenization_utils_fast"] = (_tuf, _tuf.TokenizerFast)
            _tuf.TokenizerFast = _TokenizerShim
        except ImportError:
            pass

    if not _v5_patched and "tokenization_utils_fast" not in _originals:
        raise ImportError(
            "Could not import transformers.tokenization_utils_tokenizers "
            "(v5+) or transformers.tokenization_utils_fast (v4). "
            "Is transformers installed?"
        )

    # vLLM requires a stream that accepts `_TokenizerShim` rather than a
    # `tokenizers.Tokenizer` instance.
    _originals["DecodeStream"] = _td.DecodeStream
    _td.DecodeStream = DecodeStream

    _patched = True

    from importlib.metadata import version

    transformers_version = version("transformers")
    print(
        "[snaptokens] patch_transformers: successfully patched "
        f"transformers v{transformers_version}"
    )


def unpatch_transformers() -> None:
    """
    Reverse the monkey-patching applied by :func:`patch_transformers`,
    restoring the ``transformers`` library to its original state.
    """
    global _patched
    if not _patched:
        return

    import tokenizers.decoders as _td

    if "TokenizersBackend.from_pretrained" in _originals:
        from transformers.tokenization_utils_tokenizers import TokenizersBackend

        # `from_pretrained` is inherited from `PreTrainedTokenizerBase`, not
        # defined on `TokenizersBackend`. The value captured during patch
        # via attribute access is a bound `method`, not a classmethod
        # descriptor — assigning it back installs a stray attribute in
        # `TokenizersBackend.__dict__` that shadows the inherited
        # classmethod and breaks `cls` polymorphism for subclasses.
        # Removing our patch attribute restores plain inheritance.
        if "from_pretrained" in TokenizersBackend.__dict__:
            del TokenizersBackend.from_pretrained

    # v4 path
    if "tokenization_utils_fast" in _originals:
        mod, original_cls = _originals["tokenization_utils_fast"]
        mod.TokenizerFast = original_cls

    _td.DecodeStream = _originals["DecodeStream"]

    _originals.clear()
    _patched = False
