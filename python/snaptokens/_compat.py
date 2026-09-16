"""
Compatibility shim for monkey-patching the ``transformers`` library.

Provides :class:`_TokenizerShim` (a complete replacement for
``tokenizers.Tokenizer``).  All encoding, decoding, vocabulary operations,
truncation, and padding are handled by the Rust backend.  The returned
:class:`Encoding` objects are also Rust-backed ``#[pyclass]`` instances --
no Python-side wrapping is needed.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Optional

from snaptokens._native import Encoding, Tokenizer


class _TokenizerShim:
    """
    Complete replacement for ``tokenizers.Tokenizer``.

    All encoding, decoding, vocabulary, truncation, and padding operations
    are delegated to the Rust :class:`Tokenizer`.  No reference to the
    original ``tokenizers.Tokenizer`` is kept.
    """

    def __init__(self, src) -> None:
        """Build a native tokenizer from serialized or Hugging Face state."""
        if isinstance(src, str):
            self._json = src
        elif hasattr(src, "to_str"):
            self._json = src.to_str()
        else:
            raise TypeError(
                f"expected JSON string or an object with to_str(); got {type(src).__name__}"
            )
        self._fast = Tokenizer.from_json_str(self._json)

    def __getstate__(self):
        """Use the same configuration for pickle, deepcopy, and JSON files."""
        return self.to_str()

    def __setstate__(self, state) -> None:
        """Read current JSON state and the older four-field pickle format."""
        if isinstance(state, str):
            self.__init__(state)
        else:
            json_str, trunc, pad, enc_special = state
            # Old pickles duplicated settings using the Python getter schema.
            # Those separate fields were authoritative, including None.
            cfg = json.loads(json_str)
            cfg["truncation"] = None
            cfg["padding"] = None
            self.__init__(json.dumps(cfg))
            self.truncation = trunc
            self.padding = pad
            self.encode_special_tokens = enc_special

    @classmethod
    def from_str(cls, json_str: str) -> _TokenizerShim:
        """Build a shim from serialized tokenizer JSON."""
        return cls(json_str)

    @classmethod
    def from_file(cls, path: str) -> _TokenizerShim:
        """Build a shim from a local tokenizer JSON file."""
        return cls(Path(path).read_text(encoding="utf-8"))

    @classmethod
    def from_pretrained(
        cls,
        identifier: str,
        revision: str = "main",
        token: Optional[str] = None,
    ) -> _TokenizerShim:
        """Download tokenizer JSON through Hugging Face, then load it locally."""
        from huggingface_hub import hf_hub_download

        path = hf_hub_download(
            identifier,
            "tokenizer.json",
            revision=revision,
            token=token,
        )
        return cls.from_file(path)

    @classmethod
    def from_buffer(cls, buf: bytes) -> _TokenizerShim:
        """Build a shim from UTF-8 tokenizer JSON bytes."""
        return cls(buf.decode("utf-8"))

    def to_str(self, pretty: bool = False) -> str:
        """Serialize source JSON with the current mutable encode settings."""
        cfg = json.loads(self._json)
        cfg.update(json.loads(self._fast._settings_json()))
        if pretty:
            return json.dumps(cfg, indent=2, ensure_ascii=False)
        return json.dumps(cfg, ensure_ascii=False)

    def save(self, path: str, pretty: bool = True) -> None:
        """Write the current tokenizer configuration to one JSON file."""
        Path(path).write_text(self.to_str(pretty=pretty), encoding="utf-8")

    @property
    def encode_special_tokens(self) -> bool:
        """Return whether added special-token strings should be encoded."""
        return False

    @encode_special_tokens.setter
    def encode_special_tokens(self, value: bool) -> None:
        """Set whether added special-token strings should be encoded."""
        if value:
            raise NotImplementedError("encoding added special tokens as ordinary text is not supported")

    @property
    def truncation(self) -> Optional[dict]:
        """Return the active native truncation settings."""
        return self._fast.truncation

    @truncation.setter
    def truncation(self, value: Optional[dict]) -> None:
        """Replace or disable the native truncation settings."""
        if value is None:
            self._fast.no_truncation()
        else:
            self._fast.enable_truncation(**value)

    @property
    def padding(self) -> Optional[dict]:
        """Return the active native padding settings."""
        return self._fast.padding

    @padding.setter
    def padding(self, value: Optional[dict]) -> None:
        """Replace or disable the native padding settings."""
        if value is None:
            self._fast.no_padding()
        else:
            self._fast.enable_padding(
                **{k: v for k, v in value.items() if v is not None}
            )

    def enable_truncation(
        self,
        max_length: int,
        stride: int = 0,
        strategy: str = "longest_first",
        direction: str = "right",
    ) -> None:
        """Configure native truncation for later encode calls."""
        self._fast.enable_truncation(
            max_length, stride=stride, strategy=strategy, direction=direction
        )

    def no_truncation(self) -> None:
        """Disable native truncation."""
        self._fast.no_truncation()

    def enable_padding(
        self,
        direction: str = "right",
        pad_id: int = 0,
        pad_type_id: int = 0,
        pad_token: str = "[PAD]",
        length: Optional[int] = None,
        pad_to_multiple_of: Optional[int] = None,
    ) -> None:
        """Configure native padding for later encode calls."""
        self._fast.enable_padding(
            direction=direction,
            pad_id=pad_id,
            pad_type_id=pad_type_id,
            pad_token=pad_token,
            **({"length": length} if length is not None else {}),
            **(
                {"pad_to_multiple_of": pad_to_multiple_of}
                if pad_to_multiple_of is not None
                else {}
            ),
        )

    def no_padding(self) -> None:
        """Disable native padding."""
        self._fast.no_padding()

    def encode(
        self,
        sequence: str,
        pair: Optional[str] = None,
        is_pretokenized: bool = False,
        add_special_tokens: bool = True,
    ) -> Encoding:
        """Encode one raw string through the native tokenizer."""
        if pair is not None:
            raise NotImplementedError("pair encoding is not supported by snaptokens")
        if is_pretokenized:
            raise NotImplementedError(
                "pre-tokenized input is not supported by snaptokens"
            )
        return self._fast.encode(sequence, add_special_tokens=add_special_tokens)

    def encode_batch(
        self,
        inputs: list,
        is_pretokenized: bool = False,
        add_special_tokens: bool = True,
    ) -> list[Encoding]:
        """Encode raw strings through the native batch scheduler."""
        if is_pretokenized or any(isinstance(inp, (list, tuple)) for inp in inputs):
            raise NotImplementedError(
                "pair/pre-tokenized batch encoding is not supported by snaptokens"
            )
        return self._fast.encode_batch(inputs, add_special_tokens=add_special_tokens)

    def encode_batch_fast(
        self,
        inputs: list[str],
        is_pretokenized: bool = False,
        add_special_tokens: bool = True,
    ) -> list[Encoding]:
        """Provide the tokenizers fast-batch alias over native batch encoding."""
        return self.encode_batch(inputs, is_pretokenized, add_special_tokens)

    def post_process(
        self,
        encoding: Encoding,
        pair: Optional[Encoding] = None,
        add_special_tokens: bool = True,
    ) -> Encoding:
        """Apply native single-sequence post-processing to an encoding."""
        return self._fast.post_process(encoding, pair, add_special_tokens)

    def num_special_tokens_to_add(self, is_pair: bool) -> int:
        """Return how many special tokens post-processing would add."""
        return self._fast.num_special_tokens_to_add(is_pair)

    def decode(self, ids: list[int], skip_special_tokens: bool = True) -> str:
        """Decode one sequence of token IDs through the native pipeline."""
        return self._fast.decode(ids, skip_special_tokens=skip_special_tokens)

    def decode_batch(
        self,
        sequences: list[list[int]],
        skip_special_tokens: bool = True,
    ) -> list[str]:
        """Decode several token-ID sequences through the native pipeline."""
        return self._fast.decode_batch(
            sequences, skip_special_tokens=skip_special_tokens
        )

    def id_to_token(self, id: int) -> Optional[str]:
        """Look up one token string by ID."""
        return self._fast.id_to_token(id)

    def token_to_id(self, token: str) -> Optional[int]:
        """Look up one token ID by string."""
        return self._fast.token_to_id(token)

    def get_vocab(self, with_added_tokens: bool = True) -> dict[str, int]:
        """Materialize the native vocabulary as a Python dictionary."""
        cfg = json.loads(self._json)
        vocab = dict(cfg["model"]["vocab"])
        if with_added_tokens:
            vocab.update((entry["content"], entry["id"]) for entry in cfg.get("added_tokens", []))
        return vocab

    def get_vocab_size(self, with_added_tokens: bool = True) -> int:
        """Count unique vocabulary entries, optionally including added tokens."""
        return len(self.get_vocab(with_added_tokens))

    def get_added_tokens_decoder(self) -> dict[int, object]:
        """Rebuild added-token metadata objects from the source JSON."""
        try:
            cfg = json.loads(self._json)
        except (json.JSONDecodeError, TypeError):
            return {}
        result: dict[int, object] = {}
        for entry in cfg.get("added_tokens", []):
            tid = entry.get("id")
            if tid is not None:
                result[tid] = _AddedTokenInfo(
                    content=entry.get("content", ""),
                    single_word=entry.get("single_word", False),
                    lstrip=entry.get("lstrip", False),
                    rstrip=entry.get("rstrip", False),
                    normalized=entry.get("normalized", True),
                    special=entry.get("special", False),
                )
        return result

    def add_tokens(self, tokens) -> int:
        """Reject runtime vocabulary mutation that the native tokenizer cannot apply."""
        raise NotImplementedError("runtime token addition is not supported by snaptokens")

    def add_special_tokens(self, special_tokens) -> int:
        """Reject runtime special-token mutation that the native tokenizer cannot apply."""
        raise NotImplementedError(
            "runtime special-token addition is not supported by snaptokens"
        )

    @property
    def model(self):
        """Return the minimal model adapter needed by tokenizer saving."""
        return _ModelStub(self)

    @model.setter
    def model(self, value) -> None:
        """Reject model replacement because the native tokenizer is immutable."""
        raise NotImplementedError("model replacement is not supported by snaptokens")

    @property
    def normalizer(self):
        """Return no standalone normalizer because it stays fused in Rust."""
        return None

    @normalizer.setter
    def normalizer(self, value) -> None:
        """Reject normalizer replacement because the native pipeline is immutable."""
        raise NotImplementedError("normalizer replacement is not supported by snaptokens")

    @property
    def decoder(self):
        """Return a decoder adapter backed by the native tokenizer."""
        return _DecoderShim(self._fast)

    @decoder.setter
    def decoder(self, value) -> None:
        """Reject decoder replacement because the native pipeline is immutable."""
        raise NotImplementedError("decoder replacement is not supported by snaptokens")

    @property
    def pre_tokenizer(self):
        """Return no standalone pre-tokenizer because it stays fused in Rust."""
        return None

    @pre_tokenizer.setter
    def pre_tokenizer(self, value) -> None:
        """Reject pre-tokenizer replacement because the native pipeline is immutable."""
        raise NotImplementedError(
            "pre-tokenizer replacement is not supported by snaptokens"
        )

    @property
    def post_processor(self):
        """Return the native tokenizer's current post-processor adapter."""
        return self._fast.post_processor

    @post_processor.setter
    def post_processor(self, value) -> None:
        """Replace the native tokenizer's post-processor."""
        self._fast.post_processor = value


class _DecoderShim:
    """
    Stand-in for ``tokenizers.decoders.Decoder``.

    ``PreTrainedTokenizerFast.convert_tokens_to_string`` calls
    ``self.backend_tokenizer.decoder.decode(tokens)`` when the decoder is not
    None.  Without this shim it falls back to ``" ".join(tokens)``, which
    leaves ByteLevel-encoded tokens (e.g. "Ġhello") undecoded in the output.
    """

    __slots__ = ("_fast",)

    def __init__(self, fast) -> None:
        """Keep the native tokenizer used for token-string decoding."""
        self._fast = fast

    def decode(self, tokens: list[str]) -> str:
        """Decode token strings through the native decoder pipeline."""
        return self._fast.decode_tokens(tokens)


class _AddedTokenInfo:
    """Minimal stand-in for ``tokenizers.AddedToken``."""

    __slots__ = ("content", "single_word", "lstrip", "rstrip", "normalized", "special")

    def __init__(
        self,
        content: str = "",
        single_word: bool = False,
        lstrip: bool = False,
        rstrip: bool = False,
        normalized: bool = True,
        special: bool = False,
    ) -> None:
        """Store the added-token fields consumed by Transformers."""
        self.content = content
        self.single_word = single_word
        self.lstrip = lstrip
        self.rstrip = rstrip
        self.normalized = normalized
        self.special = special

    def __repr__(self) -> str:
        """Render the token using the Hugging Face AddedToken style."""
        return (
            f"AddedToken({self.content!r}, "
            f"rstrip={self.rstrip}, lstrip={self.lstrip}, "
            f"single_word={self.single_word}, "
            f"normalized={self.normalized}, "
            f"special={self.special})"
        )


class _ModelStub:
    """Minimal stub for ``tokenizers.models.Model`` to support saving."""

    def __init__(self, shim: _TokenizerShim) -> None:
        """Keep the tokenizer used to materialize a vocabulary file."""
        self._shim = shim

    def save(self, folder: str, prefix: Optional[str] = None) -> list[str]:
        """Write the vocabulary shape expected by tokenizer save routines."""
        name = f"{prefix}-vocab.json" if prefix else "vocab.json"
        path = Path(folder) / name
        vocab = self._shim.get_vocab()
        path.write_text(json.dumps(vocab, ensure_ascii=False), encoding="utf-8")
        return [str(path)]
