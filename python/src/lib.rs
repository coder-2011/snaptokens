use std::{path::Path, sync::RwLock};

use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use snaptokens::TruncationDirection as Direction;
use snaptokens::json_structs::{
    AddedTokenConfig, DecoderConfig, LoadError, NormalizerConfig, PaddingParams, PaddingStrategy,
    PostProcessorConfig, PreTokenizerConfig, TokenizerJson, TruncationParams, TruncationStrategy,
};

// PyEncoding

#[derive(Clone)]
enum Metadata<T> {
    Repeated { value: T, len: usize },
    Values(Vec<T>),
}

impl<T: Copy + PartialEq> Metadata<T> {
    fn to_vec(&self) -> Vec<T> {
        match self {
            Self::Repeated { value, len } => vec![*value; *len],
            Self::Values(values) => values.clone(),
        }
    }

    fn slice(&mut self, start: usize, end: usize) {
        match self {
            Self::Repeated { len, .. } => {
                assert!(start <= end && end <= *len, "metadata slice out of bounds");
                *len = end - start;
            }
            Self::Values(values) => {
                // `T: Copy`, so the retained range can stay in this allocation.
                values.copy_within(start..end, 0);
                values.truncate(end - start);
            }
        }
    }

    fn append(&mut self, other: &Self) {
        if let (
            Self::Repeated { value, len },
            Self::Repeated {
                value: next,
                len: count,
            },
        ) = (&mut *self, other)
            && (*value == *next || *len == 0)
        {
            *value = *next;
            *len += count;
            return;
        }
        if let Self::Repeated { .. } = self {
            *self = Self::Values(self.to_vec());
        }
        if let Self::Values(values) = self {
            match other {
                Self::Repeated { value, len } => values.resize(values.len() + len, *value),
                Self::Values(other) => values.extend_from_slice(other),
            }
        }
    }

    /// Retain compact runs and shift materialized left padding within its allocation.
    fn pad(&mut self, value: T, count: usize, left: bool) {
        if left && let Self::Values(values) = self {
            let len = values.len();
            values.resize(len + count, value);
            values.copy_within(..len, count);
            values[..count].fill(value);
            return;
        }
        let mut padding = Self::Repeated { value, len: count };
        if left {
            padding.append(self);
            *self = padding;
        } else {
            self.append(&padding);
        }
    }
}

/// Minimal stand-in for `tokenizers.Encoding`.
///
/// Returned directly by `Tokenizer.encode` and `Tokenizer.encode_batch` so
/// no Python-side wrapping is needed.  Fields that `snaptokens` does not
/// track (`tokens`, `offsets`, `sequence_ids`, `word_ids`) have getters that
/// raise `NotImplementedError` to match the HuggingFace API surface.
#[pyclass(name = "Encoding", module = "snaptokens._native")]
pub struct PyEncoding {
    /// Token IDs in this encoded sequence.
    #[pyo3(get, set)]
    pub ids: Vec<u32>,
    attention_mask: Metadata<u32>,
    type_ids: Metadata<u32>,
    special_tokens_mask: Metadata<u32>,
    /// Number of source sequences represented by this encoding.
    #[pyo3(get, set)]
    pub n_sequences: usize,
    // Backing storage for set-only properties.
    _sequence_ids: Metadata<Option<i64>>,
    _word_ids: Metadata<Option<i64>>,
    // Set when truncation discarded tokens that overflow rows would have kept.
    truncated: bool,
}

impl PyEncoding {
    /// Creates an encoding from IDs and an optional attention mask.
    pub fn make(ids: Vec<u32>, attention_mask: Option<Vec<u32>>) -> Self {
        let n = ids.len();
        Self {
            type_ids: Metadata::Repeated { value: 0, len: n },
            special_tokens_mask: Metadata::Repeated { value: 0, len: n },
            n_sequences: 1,
            _sequence_ids: Metadata::Repeated {
                value: Some(0),
                len: n,
            },
            _word_ids: Metadata::Repeated {
                value: None,
                len: n,
            },
            ids,
            attention_mask: attention_mask
                .map_or(Metadata::Repeated { value: 1, len: n }, Metadata::Values),
            truncated: false,
        }
    }

    fn empty_merge() -> Self {
        Self {
            ids: Vec::new(),
            attention_mask: Metadata::Repeated { value: 1, len: 0 },
            type_ids: Metadata::Repeated { value: 0, len: 0 },
            special_tokens_mask: Metadata::Repeated { value: 0, len: 0 },
            n_sequences: 0,
            _sequence_ids: Metadata::Repeated {
                value: None,
                len: 0,
            },
            _word_ids: Metadata::Repeated {
                value: None,
                len: 0,
            },
            truncated: false,
        }
    }

    fn append(&mut self, other: &Self) {
        self.ids.extend_from_slice(&other.ids);
        self.attention_mask.append(&other.attention_mask);
        self.type_ids.append(&other.type_ids);
        self.special_tokens_mask.append(&other.special_tokens_mask);
        self.n_sequences += other.n_sequences;
        self._sequence_ids.append(&other._sequence_ids);
        self._word_ids.append(&other._word_ids);
        self.truncated |= other.truncated;
    }

    fn apply_slice(&mut self, start: usize, end: usize) {
        self.ids.copy_within(start..end, 0);
        self.ids.truncate(end - start);
        self.attention_mask.slice(start, end);
        self.type_ids.slice(start, end);
        self.special_tokens_mask.slice(start, end);
        self._sequence_ids.slice(start, end);
        self._word_ids.slice(start, end);
    }

    fn extend_right(&mut self, pad_id: u32, pad_type_id: u32, count: usize) {
        self.ids.resize(self.ids.len() + count, pad_id);
        self.attention_mask.pad(0, count, false);
        self.type_ids.pad(pad_type_id, count, false);
        self.special_tokens_mask.pad(0, count, false);
        self._sequence_ids.pad(None, count, false);
        self._word_ids.pad(None, count, false);
    }

    /// Shift IDs in place, padding each metadata field using its own length.
    fn extend_left(&mut self, pad_id: u32, pad_type_id: u32, count: usize) {
        let len = self.ids.len();
        self.ids.resize(len + count, pad_id);
        self.ids.copy_within(..len, count);
        self.ids[..count].fill(pad_id);
        self.attention_mask.pad(0, count, true);
        self.type_ids.pad(pad_type_id, count, true);
        self.special_tokens_mask.pad(0, count, true);
        self._sequence_ids.pad(None, count, true);
        self._word_ids.pad(None, count, true);
    }
}

#[pymethods]
impl PyEncoding {
    /// Creates an encoding with optional explicit attention-mask values.
    #[new]
    #[pyo3(signature = (ids, attention_mask = None))]
    fn new(ids: Vec<u32>, attention_mask: Option<Vec<u32>>) -> Self {
        Self::make(ids, attention_mask)
    }

    /// Returns one attention-mask value per token.
    #[getter]
    fn attention_mask(&self) -> Vec<u32> {
        self.attention_mask.to_vec()
    }

    #[setter]
    fn set_attention_mask(&mut self, values: Vec<u32>) {
        self.attention_mask = Metadata::Values(values);
    }

    /// Returns one type ID per token.
    #[getter]
    fn type_ids(&self) -> Vec<u32> {
        self.type_ids.to_vec()
    }

    #[setter]
    fn set_type_ids(&mut self, values: Vec<u32>) {
        self.type_ids = Metadata::Values(values);
    }

    /// Returns one special-token marker per token.
    #[getter]
    fn special_tokens_mask(&self) -> Vec<u32> {
        self.special_tokens_mask.to_vec()
    }

    #[setter]
    fn set_special_tokens_mask(&mut self, values: Vec<u32>) {
        self.special_tokens_mask = Metadata::Values(values);
    }

    fn __len__(&self) -> usize {
        self.ids.len()
    }

    /// Returns the number of token IDs.
    #[getter]
    fn num_tokens(&self) -> usize {
        self.ids.len()
    }

    fn __repr__(&self) -> String {
        format!("Encoding(num_tokens={})", self.ids.len())
    }

    #[getter]
    fn tokens(&self) -> PyResult<Vec<String>> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track token strings; \
             use Tokenizer.id_to_token() to convert individual IDs",
        ))
    }
    #[setter]
    fn set_tokens(&mut self, _v: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(PyNotImplementedError::new_err(
            "snaptokens cannot assign token strings without storing them",
        ))
    }

    #[getter]
    fn offsets(&self) -> PyResult<Vec<(usize, usize)>> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track character offsets",
        ))
    }
    #[setter]
    fn set_offsets(&mut self, _v: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(PyNotImplementedError::new_err(
            "snaptokens cannot assign character offsets without storing them",
        ))
    }

    #[getter]
    fn sequence_ids(&self) -> PyResult<Vec<Option<i64>>> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track sequence IDs",
        ))
    }
    #[setter]
    fn set_sequence_ids(&mut self, value: Vec<Option<i64>>) {
        self._sequence_ids = Metadata::Values(value);
    }

    #[getter]
    fn word_ids(&self) -> PyResult<Vec<Option<i64>>> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track word IDs",
        ))
    }
    #[setter]
    fn set_word_ids(&mut self, value: Vec<Option<i64>>) {
        self._word_ids = Metadata::Values(value);
    }

    #[getter]
    fn words(&self) -> PyResult<Vec<Option<i64>>> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track word IDs",
        ))
    }
    #[setter]
    fn set_words(&mut self, value: Vec<Option<i64>>) {
        self._word_ids = Metadata::Values(value);
    }

    /// Returns an empty list until truncation has discarded tokens.
    ///
    /// An untruncated encoding genuinely has no overflow rows, so the empty
    /// list is exact. Truncation discards precisely the tokens overflow rows
    /// would have carried, and answering with an empty list there would hide
    /// that loss from a caller who asked for them.
    #[getter]
    fn overflowing<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        if self.truncated {
            return Err(PyNotImplementedError::new_err(
                "snaptokens does not produce overflowing encodings; \
                 truncation discarded the tokens past max_length",
            ));
        }
        Ok(PyList::empty(py))
    }
    #[setter]
    fn set_overflowing(&mut self, _v: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(PyNotImplementedError::new_err(
            "snaptokens does not support overflowing encodings",
        ))
    }

    /// Assigns one sequence ID to every token.
    fn set_sequence_id(&mut self, sequence_id: i64) {
        let n = self.ids.len();
        self._sequence_ids = Metadata::Repeated {
            value: Some(sequence_id),
            len: n,
        };
    }

    #[pyo3(signature = (char_pos, sequence_index = 0))]
    fn char_to_token(&self, char_pos: usize, sequence_index: usize) -> PyResult<Option<usize>> {
        let _ = (char_pos, sequence_index);
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track character offsets",
        ))
    }

    #[pyo3(signature = (char_pos, sequence_index = 0))]
    fn char_to_word(&self, char_pos: usize, sequence_index: usize) -> PyResult<Option<usize>> {
        let _ = (char_pos, sequence_index);
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track word IDs",
        ))
    }

    fn token_to_chars(&self, token_index: usize) -> PyResult<Option<(usize, usize)>> {
        let _ = token_index;
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track character offsets",
        ))
    }

    fn token_to_sequence(&self, token_index: usize) -> PyResult<Option<usize>> {
        let _ = token_index;
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track sequence IDs",
        ))
    }

    fn token_to_word(&self, token_index: usize) -> PyResult<Option<usize>> {
        let _ = token_index;
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track word IDs",
        ))
    }

    #[pyo3(signature = (word_index, sequence_index = 0))]
    fn word_to_chars(
        &self,
        word_index: usize,
        sequence_index: usize,
    ) -> PyResult<Option<(usize, usize)>> {
        let _ = (word_index, sequence_index);
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track character offsets",
        ))
    }

    #[pyo3(signature = (word_index, sequence_index = 0))]
    fn word_to_tokens(
        &self,
        word_index: usize,
        sequence_index: usize,
    ) -> PyResult<Option<(usize, usize)>> {
        let _ = (word_index, sequence_index);
        Err(PyNotImplementedError::new_err(
            "snaptokens does not track word IDs",
        ))
    }

    /// Truncates in place from the left or right.
    #[pyo3(signature = (max_length, stride = 0, direction = "right"))]
    fn truncate(&mut self, max_length: usize, stride: usize, direction: &str) -> PyResult<()> {
        let direction = parse_direction(direction)?;
        if stride != 0 {
            return Err(PyNotImplementedError::new_err(
                "nonzero truncation stride requires overflow rows",
            ));
        }
        let n = self.ids.len();
        if n <= max_length {
            return Ok(());
        }
        if direction == Direction::Left {
            self.apply_slice(n - max_length, n);
        } else {
            self.apply_slice(0, max_length);
        }
        self.truncated = true;
        Ok(())
    }

    /// Pads in place to `length` with the configured ID and direction.
    #[pyo3(signature = (length, direction = "right", pad_id = 0, pad_type_id = 0, pad_token = "[PAD]"))]
    fn pad(
        &mut self,
        length: usize,
        direction: &str,
        pad_id: u32,
        pad_type_id: u32,
        pad_token: &str,
    ) -> PyResult<()> {
        let _ = pad_token;
        let direction = parse_direction(direction)?;
        let n = self.ids.len();
        if length <= n {
            return Ok(());
        }
        let deficit = length - n;
        if direction == Direction::Left {
            self.extend_left(pad_id, pad_type_id, deficit);
        } else {
            self.extend_right(pad_id, pad_type_id, deficit);
        }
        Ok(())
    }

    /// Concatenates encodings into one encoding.
    #[staticmethod]
    #[pyo3(signature = (encodings, growing_offsets = true))]
    fn merge(py: Python<'_>, encodings: Vec<Py<PyEncoding>>, growing_offsets: bool) -> PyEncoding {
        let _ = growing_offsets;
        let mut merged = Self::empty_merge();
        for enc_py in &encodings {
            merged.append(&enc_py.borrow(py));
        }
        merged
    }
}

// TruncationParams / PaddingParams

fn parse_direction(value: &str) -> PyResult<Direction> {
    match value {
        "left" => Ok(Direction::Left),
        "right" => Ok(Direction::Right),
        _ => Err(PyValueError::new_err("direction must be 'left' or 'right'")),
    }
}

fn parse_truncation_strategy(value: &str) -> PyResult<TruncationStrategy> {
    match value {
        "longest_first" => Ok(TruncationStrategy::LongestFirst),
        "only_first" => Ok(TruncationStrategy::OnlyFirst),
        "only_second" => Err(PyNotImplementedError::new_err(
            "only_second requires pair encoding",
        )),
        _ => Err(PyValueError::new_err("unknown truncation strategy")),
    }
}

fn validate_truncation(truncation: &TruncationParams) -> PyResult<()> {
    parse_truncation_strategy(truncation.strategy.as_str())?;
    if truncation.stride != 0 {
        return Err(PyNotImplementedError::new_err(
            "nonzero truncation stride requires overflow rows",
        ));
    }
    Ok(())
}

fn build_encoding(
    ids: Vec<u32>,
    pad: Option<&PaddingParams>,
    target: usize,
    truncated: bool,
) -> PyEncoding {
    let mut enc = PyEncoding::make(ids, None);
    enc.truncated = truncated;
    if let Some(p) = pad {
        let deficit = target.saturating_sub(enc.ids.len());
        match p.direction {
            Direction::Left => enc.extend_left(p.pad_id, p.pad_type_id, deficit),
            Direction::Right => enc.extend_right(p.pad_id, p.pad_type_id, deficit),
        }
    }
    enc
}

// PyPostProcessor

/// Python-facing post-processor object — mirrors `tokenizers.processors.*`.
///
/// Keeps typed configuration until an external caller requests JSON.
#[pyclass(name = "PostProcessor", module = "snaptokens._native")]
#[derive(Clone)]
struct PyPostProcessor {
    config: PostProcessorConfig,
}

#[pymethods]
impl PyPostProcessor {
    fn __str__(&self) -> PyResult<String> {
        serde_json::to_string(&self.config).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

// PyTokenizer

/// Mutable state guarded by `PyTokenizer::state`.
///
/// All read paths (encode/decode/getters) hold a read lock; mutators
/// (`enable_truncation`, `set_post_processor`, …) hold a write lock so they
/// cannot race with concurrent reads when the GIL is released.
struct TokenizerState {
    inner: snaptokens::Tokenizer,
    truncation: Option<TruncationParams>,
    padding: Option<PaddingParams>,
    post_processor: Option<PostProcessorConfig>,
    added_tokens: Vec<AddedTokenConfig>,
    normalizer: Option<NormalizerConfig>,
    pre_tokenizer: Option<PreTokenizerConfig>,
    decoder: Option<DecoderConfig>,
    extra: serde_json::Map<String, serde_json::Value>,
}

impl TokenizerState {
    /// Borrow the model and snapshot the current serializable configuration.
    fn config(&self) -> TokenizerJson<&snaptokens::Model> {
        TokenizerJson {
            truncation: self.truncation.clone(),
            padding: self.padding.clone(),
            post_processor: self.post_processor.clone(),
            added_tokens: self.added_tokens.clone(),
            normalizer: self.normalizer.clone(),
            pre_tokenizer: self.pre_tokenizer.clone(),
            decoder: self.decoder.clone(),
            extra: self.extra.clone(),
            model: self.inner.model(),
        }
    }

    /// Reserve special-token space before asking BPE for a limited content sequence.
    fn content_limit(
        &self,
        add_special_tokens: bool,
    ) -> Result<Option<(usize, Direction)>, String> {
        self.truncation
            .as_ref()
            .map(|t| {
                let added = self
                    .inner
                    .post_process(Vec::new(), add_special_tokens)
                    .len();
                t.max_length
                    .checked_sub(added)
                    .map(|limit| (limit, t.direction))
                    .ok_or_else(|| {
                        format!(
                            "truncation max_length {} cannot fit {added} special tokens",
                            t.max_length
                        )
                    })
            })
            .transpose()
    }

    fn encode(&self, input: &str, add_special_tokens: bool) -> Result<(Vec<u32>, bool), String> {
        let (ids, truncated) =
            if let Some((max_tokens, direction)) = self.content_limit(add_special_tokens)? {
                self.inner
                    .encode_with_limit(input, max_tokens, direction)
                    .map_err(|e| e.to_string())?
            } else {
                (
                    self.inner.encode(input, false).map_err(|e| e.to_string())?,
                    false,
                )
            };
        Ok((self.inner.post_process(ids, add_special_tokens), truncated))
    }

    fn pad_target(&self, n: usize) -> usize {
        let Some(ref p) = self.padding else { return n };
        let base = p.length().unwrap_or(n).max(n);
        match p.pad_to_multiple_of {
            Some(m) if m > 0 => base.div_ceil(m) * m,
            _ => base,
        }
    }

    fn encode_batch(
        &self,
        inputs: &[String],
        add_special_tokens: bool,
    ) -> Result<Vec<(Vec<u32>, bool)>, String> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        if let Some((max_tokens, direction)) = self.content_limit(add_special_tokens)? {
            return self
                .inner
                .encode_batch_with_limit(inputs, max_tokens, direction)
                .map(|rows| {
                    rows.into_iter()
                        .map(|(ids, truncated)| {
                            (self.inner.post_process(ids, add_special_tokens), truncated)
                        })
                        .collect()
                })
                .map_err(|e| e.to_string());
        }
        self.inner
            .encode_batch(inputs, add_special_tokens)
            .map(|rows| rows.into_iter().map(|ids| (ids, false)).collect())
            .map_err(|e| e.to_string())
    }

    fn update_post_processor(&mut self, config: Option<PostProcessorConfig>) -> PyResult<()> {
        let processor = config
            .clone()
            .map(snaptokens::PostProcessor::from_config)
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.set_post_processor(processor);
        self.post_processor = config;
        Ok(())
    }
}

/// A thread-safe tokenizer backed by local `tokenizer.json` or `.tkz` data.
#[pyclass(name = "Tokenizer", module = "snaptokens._native")]
struct PyTokenizer {
    state: RwLock<TokenizerState>,
}

impl PyTokenizer {
    fn read(&self) -> std::sync::RwLockReadGuard<'_, TokenizerState> {
        self.state.read().expect("PyTokenizer state lock poisoned")
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, TokenizerState> {
        self.state.write().expect("PyTokenizer state lock poisoned")
    }

    #[allow(non_snake_case)]
    fn constructPythonTokenizer(config: TokenizerJson) -> PyResult<Self> {
        if let Some(truncation) = &config.truncation {
            validate_truncation(truncation)?;
        }
        let inner = snaptokens::Tokenizer::from_config(TokenizerJson {
            model: config.model,
            truncation: None,
            padding: None,
            post_processor: config.post_processor.clone(),
            added_tokens: config.added_tokens.clone(),
            normalizer: config.normalizer.clone(),
            pre_tokenizer: config.pre_tokenizer.clone(),
            decoder: config.decoder.clone(),
            extra: Default::default(),
        })
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            state: RwLock::new(TokenizerState {
                inner,
                truncation: config.truncation,
                padding: config.padding,
                post_processor: config.post_processor,
                added_tokens: config.added_tokens,
                normalizer: config.normalizer,
                pre_tokenizer: config.pre_tokenizer,
                decoder: config.decoder,
                extra: config.extra,
            }),
        })
    }
}

#[pymethods]
impl PyTokenizer {
    /// Create a tokenizer from local JSON or `.tkz` data.
    ///
    /// JSON remains read-only unless `tkz_cache` is true. Direct `.tkz` paths
    /// always use the native loader because they cannot be parsed as JSON.
    #[staticmethod]
    #[pyo3(signature = (path, tkz_cache = false))]
    fn from_file(path: &str, tkz_cache: bool, py: Python<'_>) -> PyResult<Self> {
        let path = Path::new(path);
        let is_tkz = path.extension().is_some_and(|extension| extension == "tkz");
        let mode = if tkz_cache || is_tkz {
            snaptokens::LoadMode::TkzCache
        } else {
            snaptokens::LoadMode::JsonOnly
        };
        py.allow_threads(|| {
            TokenizerJson::load_file_with(path, mode, Self::constructPythonTokenizer).map_err(
                |error| match error {
                    LoadError::Load(error) => PyValueError::new_err(error.to_string()),
                    LoadError::Construct(error) => error,
                },
            )
        })
    }

    #[staticmethod]
    /// Builds a tokenizer from serialized Hugging Face tokenizer JSON.
    fn from_json_str(json: &str, py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            let config =
                serde_json::from_str(json).map_err(|e| PyValueError::new_err(e.to_string()))?;
            Self::constructPythonTokenizer(config)
        })
    }

    /// The current post-processor, or ``None`` if none is configured.
    ///
    /// The returned object's ``__str__`` yields its JSON representation,
    /// so ``str(tokenizer.post_processor)`` round-trips through the setter.
    #[getter]
    fn post_processor(&self, py: Python<'_>) -> PyResult<PyObject> {
        match &self.read().post_processor {
            None => Ok(py.None()),
            Some(config) => Py::new(
                py,
                PyPostProcessor {
                    config: config.clone(),
                },
            )
            .map(|p| p.into_any()),
        }
    }

    /// Serialize the current tokenizer configuration as JSON.
    #[pyo3(signature = (pretty = false))]
    fn to_str(&self, pretty: bool, py: Python<'_>) -> PyResult<String> {
        py.allow_threads(|| {
            let state = self.read();
            let config = state.config();
            if pretty {
                serde_json::to_string_pretty(&config)
            } else {
                serde_json::to_string(&config)
            }
            .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }

    /// Save current configuration to the explicit JSON or `.tkz` destination.
    #[pyo3(signature = (path, pretty = true))]
    fn save(&self, path: &str, pretty: bool, py: Python<'_>) -> PyResult<()> {
        py.allow_threads(|| self.read().config().save_file(Path::new(path), pretty))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Materialize model vocabulary, optionally including added tokens.
    #[pyo3(signature = (with_added_tokens = true))]
    fn get_vocab(&self, with_added_tokens: bool, py: Python<'_>) -> PyResult<PyObject> {
        let state = self.read();
        let vocab = PyDict::new(py);
        let model = state.inner.model();
        for id in 0..model.vocab_size() {
            if let Some(token) = model.id_to_token(id as u32) {
                vocab.set_item(token, id)?;
            }
        }
        if with_added_tokens {
            for token in &state.added_tokens {
                vocab.set_item(&token.content, token.id)?;
            }
        }
        Ok(vocab.into())
    }

    /// Expose retained added-token metadata to the compatibility wrapper.
    fn _added_tokens(&self, py: Python<'_>) -> PyResult<Vec<PyObject>> {
        self.read()
            .added_tokens
            .iter()
            .map(|token| {
                let entry = PyDict::new(py);
                entry.set_item("id", token.id)?;
                entry.set_item("content", &token.content)?;
                entry.set_item("single_word", token.single_word)?;
                entry.set_item("lstrip", token.lstrip)?;
                entry.set_item("rstrip", token.rstrip)?;
                entry.set_item("normalized", token.normalized)?;
                entry.set_item("special", token.special)?;
                Ok(entry.into())
            })
            .collect()
    }

    /// Set the post-processor.
    ///
    /// Accepts anything whose ``str()`` yields a valid post-processor JSON —
    /// including our own ``PostProcessor`` objects and ``tokenizers.processors.*``
    /// objects from the HuggingFace tokenizers library.
    #[setter]
    fn set_post_processor(&self, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let config =
            if value.is_none() {
                None
            } else if let Ok(processor) = value.extract::<PyRef<'_, PyPostProcessor>>() {
                Some(processor.config.clone())
            } else {
                // Foreign processors expose their configuration as JSON bytes or text.
                let json = match value
                    .call_method0("__getstate__")
                    .and_then(|state| state.extract::<Vec<u8>>())
                {
                    Ok(bytes) => String::from_utf8(bytes).map_err(|e| {
                        PyValueError::new_err(format!("non-UTF-8 processor state: {e}"))
                    })?,
                    Err(_) => value.str()?.to_cow()?.into_owned(),
                };
                Some(serde_json::from_str(&json).map_err(|e| {
                    PyValueError::new_err(format!("invalid post-processor JSON: {e}"))
                })?)
            };
        self.write().update_post_processor(config)
    }

    /// Enables per-encoding truncation for later encode calls.
    #[pyo3(signature = (max_length, stride = 0, strategy = "longest_first", direction = "right"))]
    fn enable_truncation(
        &self,
        max_length: usize,
        stride: usize,
        strategy: &str,
        direction: &str,
    ) -> PyResult<()> {
        let trunc = TruncationParams {
            max_length,
            stride,
            strategy: parse_truncation_strategy(strategy)?,
            direction: parse_direction(direction)?,
        };
        validate_truncation(&trunc)?;
        self.write().truncation = Some(trunc);
        Ok(())
    }

    /// Disables configured truncation.
    fn no_truncation(&self) {
        self.write().truncation = None;
    }

    /// Returns the active truncation settings, or `None`.
    #[getter]
    fn truncation(&self, py: Python<'_>) -> PyResult<PyObject> {
        match &self.read().truncation {
            None => Ok(py.None()),
            Some(t) => {
                let d = PyDict::new(py);
                d.set_item("max_length", t.max_length)?;
                d.set_item("stride", t.stride)?;
                d.set_item("strategy", t.strategy.as_str())?;
                d.set_item("direction", t.direction.as_str())?;
                Ok(d.into())
            }
        }
    }

    /// Enables padding for later batch encode calls.
    #[pyo3(signature = (direction = "right", pad_id = 0, pad_type_id = 0, pad_token = "[PAD]", length = None, pad_to_multiple_of = None))]
    fn enable_padding(
        &self,
        direction: &str,
        pad_id: u32,
        pad_type_id: u32,
        pad_token: &str,
        length: Option<usize>,
        pad_to_multiple_of: Option<usize>,
    ) -> PyResult<()> {
        self.write().padding = Some(PaddingParams {
            direction: parse_direction(direction)?,
            pad_id,
            pad_type_id,
            pad_token: pad_token.to_string(),
            strategy: length.map_or(PaddingStrategy::BatchLongest, PaddingStrategy::Fixed),
            pad_to_multiple_of,
        });
        Ok(())
    }

    /// Disables configured padding.
    fn no_padding(&self) {
        self.write().padding = None;
    }

    /// Returns the active padding settings, or `None`.
    #[getter]
    fn padding(&self, py: Python<'_>) -> PyResult<PyObject> {
        match &self.read().padding {
            None => Ok(py.None()),
            Some(p) => {
                let d = PyDict::new(py);
                d.set_item("direction", p.direction.as_str())?;
                d.set_item("pad_id", p.pad_id)?;
                d.set_item("pad_type_id", p.pad_type_id)?;
                d.set_item("pad_token", &p.pad_token)?;
                d.set_item("length", p.length())?;
                d.set_item("pad_to_multiple_of", p.pad_to_multiple_of)?;
                Ok(d.into())
            }
        }
    }

    /// Run the full encoding pipeline.
    ///
    /// Truncation and padding configured via `enable_truncation` /
    /// `enable_padding` are applied before returning.
    #[pyo3(signature = (input, add_special_tokens = false))]
    fn encode(
        &self,
        input: &str,
        add_special_tokens: bool,
        py: Python<'_>,
    ) -> PyResult<Py<PyEncoding>> {
        // Drop the state lock before reacquiring the GIL: a Python setter may
        // hold the GIL while waiting for this read lock to be released.
        let encoding = py
            .allow_threads(|| {
                let state = self.read();
                let (ids, truncated) = state.encode(input, add_special_tokens)?;
                let target = state.pad_target(ids.len());
                Ok::<_, String>(build_encoding(
                    ids,
                    state.padding.as_ref(),
                    target,
                    truncated,
                ))
            })
            .map_err(PyValueError::new_err)?;
        Py::new(py, encoding)
    }

    /// Encode a batch of inputs in parallel.
    ///
    /// Truncation is applied per-sequence; padding (if enabled) pads the
    /// batch to a uniform length.
    #[pyo3(signature = (inputs, add_special_tokens = false))]
    fn encode_batch(
        &self,
        inputs: Vec<String>,
        add_special_tokens: bool,
        py: Python<'_>,
    ) -> PyResult<Vec<Py<PyEncoding>>> {
        // Build Rust-only results while detached, releasing state before Py::new.
        let encodings = py
            .allow_threads(|| {
                let state = self.read();
                let batch = state.encode_batch(&inputs, add_special_tokens)?;
                let pad_target = state.padding.as_ref().map(|_| {
                    let max_len = batch.iter().map(|(ids, _)| ids.len()).max().unwrap_or(0);
                    state.pad_target(max_len)
                });
                Ok::<Vec<_>, String>(
                    batch
                        .into_iter()
                        .map(|(ids, truncated)| {
                            let target = pad_target.unwrap_or(ids.len());
                            build_encoding(ids, state.padding.as_ref(), target, truncated)
                        })
                        .collect(),
                )
            })
            .map_err(PyValueError::new_err)?;
        encodings
            .into_iter()
            .map(|encoding| Py::new(py, encoding))
            .collect()
    }

    /// Encode a batch into packed token IDs and cumulative row offsets.
    ///
    /// This keeps per-token Python objects out of high-throughput corpus work.
    /// Padding is intentionally omitted because offsets already delimit rows.
    #[pyo3(signature = (inputs, add_special_tokens = false))]
    fn encode_batch_flat<'py>(
        &self,
        inputs: Vec<String>,
        add_special_tokens: bool,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyBytes>, Bound<'py, PyBytes>)> {
        // Python buffers are created only after native work and its state lock finish.
        let (ids, offsets) = py
            .allow_threads(|| {
                let state = self.read();
                let (ids, lengths) = if state.truncation.is_some() {
                    let rows = state.encode_batch(&inputs, add_special_tokens)?;
                    let lengths: Vec<usize> = rows.iter().map(|(ids, _)| ids.len()).collect();
                    let mut ids = Vec::with_capacity(lengths.iter().sum());
                    for (row, _) in rows {
                        ids.extend(row);
                    }
                    (ids, lengths)
                } else {
                    state
                        .inner
                        .encode_batch_ragged(&inputs, add_special_tokens)
                        .map_err(|error| error.to_string())?
                };
                let mut offsets = Vec::with_capacity(lengths.len() + 1);
                let mut total = 0u64;
                offsets.push(0u64);
                for length in lengths {
                    total += length as u64;
                    offsets.push(total);
                }
                Ok::<_, String>((ids, offsets))
            })
            .map_err(PyValueError::new_err)?;

        // Native-endian buffers let PyBytes perform the only copy instead of
        // materializing a second Rust buffer.
        let id_bytes = unsafe {
            std::slice::from_raw_parts(ids.as_ptr().cast::<u8>(), ids.len() * size_of::<u32>())
        };
        let offset_bytes = unsafe {
            std::slice::from_raw_parts(
                offsets.as_ptr().cast::<u8>(),
                offsets.len() * size_of::<u64>(),
            )
        };
        Ok((PyBytes::new(py, id_bytes), PyBytes::new(py, offset_bytes)))
    }

    /// Apply the post-processor to an existing encoding.
    ///
    /// When `add_special_tokens` is true the post-processor inserts special
    /// tokens (BOS/EOS/etc.).  Pair encodings are not supported.
    #[pyo3(signature = (encoding, pair = None, add_special_tokens = true))]
    fn post_process(
        &self,
        encoding: Py<PyEncoding>,
        pair: Option<Py<PyEncoding>>,
        add_special_tokens: bool,
        py: Python<'_>,
    ) -> PyResult<Py<PyEncoding>> {
        if pair.is_some() {
            return Err(PyNotImplementedError::new_err(
                "pair post-processing is not supported by snaptokens",
            ));
        }
        if !add_special_tokens {
            return Ok(encoding);
        }
        let (ids, truncated) = {
            let enc = encoding.borrow(py);
            (enc.ids.clone(), enc.truncated)
        };
        let new_ids = self.read().inner.post_process(ids, true);
        let mut processed = PyEncoding::make(new_ids, None);
        processed.truncated = truncated;
        Py::new(py, processed)
    }

    /// Returns how many tokens single-sequence post-processing adds.
    fn num_special_tokens_to_add(&self, is_pair: bool) -> PyResult<usize> {
        if is_pair {
            return Err(PyNotImplementedError::new_err(
                "pair encodings are not supported by snaptokens",
            ));
        }
        // Probe: encode empty IDs with and without special tokens.
        let with_special = self.read().inner.post_process(vec![], true);
        Ok(with_special.len())
    }

    /// Decode a list of token strings back into text using the decoder pipeline.
    ///
    /// This is what `convert_tokens_to_string` needs: token strings (e.g.
    /// "Ġhello") → decoded text (" hello").  The decoder (e.g. ByteLevel)
    /// is applied exactly as during normal `decode`.
    fn decode_tokens(&self, tokens: Vec<String>) -> PyResult<String> {
        self.read()
            .inner
            .decode_tokens(tokens)
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    /// Decodes token IDs into text.
    #[pyo3(signature = (ids, skip_special_tokens = false))]
    fn decode(&self, ids: Vec<u32>, skip_special_tokens: bool) -> PyResult<String> {
        self.read()
            .inner
            .decode(&ids, skip_special_tokens)
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    /// Decodes every token-ID row in order.
    #[pyo3(signature = (sentences, skip_special_tokens = false))]
    fn decode_batch(
        &self,
        sentences: Vec<Vec<u32>>,
        skip_special_tokens: bool,
    ) -> PyResult<Vec<String>> {
        let state = self.read();
        sentences
            .iter()
            .map(|ids| state.inner.decode(ids, skip_special_tokens))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    /// Returns the ID for exact token text, if present.
    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.read().inner.token_to_id(token)
    }

    /// Returns the token text for an ID, if present.
    fn id_to_token(&self, id: u32) -> Option<String> {
        self.read().inner.id_to_token(id).map(String::from)
    }

    /// Returns the BPE vocabulary size including added tokens.
    #[getter]
    fn vocab_size(&self) -> usize {
        self.read().inner.vocab_size()
    }
}

// DecodeStream

/// Python binding for [`snaptokens::DecodeStream`].
///
/// Drop-in replacement for `tokenizers.decoders.DecodeStream`. Accepts both a
/// bare `snaptokens.Tokenizer` and any shim that stores one in `._fast`
/// (e.g. `_TokenizerShim`).
#[pyclass(name = "DecodeStream", module = "snaptokens._native")]
struct PyDecodeStream {
    inner: snaptokens::DecodeStream,
}

#[pymethods]
impl PyDecodeStream {
    /// Starts a stream, optionally seeded with prompt token IDs.
    #[new]
    #[pyo3(signature = (ids = None, skip_special_tokens = false))]
    fn new(ids: Option<Vec<u32>>, skip_special_tokens: bool) -> Self {
        Self {
            inner: snaptokens::DecodeStream::new(ids.unwrap_or_default(), skip_special_tokens),
        }
    }

    /// Appends one ID or a list of IDs and returns newly completed text, if any.
    #[pyo3(signature = (tokenizer, id))]
    fn step(
        &mut self,
        tokenizer: &Bound<'_, PyAny>,
        id: &Bound<'_, PyAny>,
        py: Python<'_>,
    ) -> PyResult<Option<String>> {
        let new_ids: Vec<u32> = if let Ok(single) = id.extract::<u32>() {
            vec![single]
        } else {
            id.extract::<Vec<u32>>()?
        };

        // Accept a PyTokenizer directly or any shim that stores one in ._fast.
        let py_tok: Py<PyTokenizer> = tokenizer
            .extract::<Py<PyTokenizer>>()
            .or_else(|_| tokenizer.getattr("_fast")?.extract::<Py<PyTokenizer>>())?;

        let tok = py_tok.borrow(py);
        let state = tok.read();
        self.inner
            .step(&state.inner, new_ids)
            .map_err(PyValueError::new_err)
    }
}

// Module

/// Native classes backing the public `snaptokens` Python package.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEncoding>()?;
    m.add_class::<PyPostProcessor>()?;
    m.add_class::<PyTokenizer>()?;
    m.add_class::<PyDecodeStream>()?;
    Ok(())
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    /// `PyEncoding::pad` correctly fills `type_ids` with `pad_type_id` for
    /// padded positions.  This is the expected behaviour.
    #[test]
    fn encoding_pad_applies_pad_type_id() {
        let mut enc = PyEncoding::new(vec![10u32, 20, 30], None);
        // 3 real tokens → pad to length 5 with pad_type_id = 1
        enc.pad(5, "right", 0u32, 1u32, "[PAD]").unwrap();

        assert_eq!(enc.ids, vec![10u32, 20, 30, 0, 0]);
        assert_eq!(enc.attention_mask(), vec![1u32, 1, 1, 0, 0]);
        assert_eq!(
            enc.type_ids(),
            vec![0u32, 0, 0, 1, 1],
            "padded positions should carry pad_type_id=1 in type_ids"
        );
    }

    /// The tokenizer encode paths build returned encodings through the same
    /// padding owner as `PyEncoding::pad`, preserving `pad_type_id` metadata.
    #[test]
    fn encode_batch_pad_type_id_applied_to_type_ids() {
        let pad = PaddingParams {
            direction: Direction::Right,
            pad_id: 0,
            pad_type_id: 1,
            pad_token: "[PAD]".to_string(),
            strategy: PaddingStrategy::BatchLongest,
            pad_to_multiple_of: None,
        };
        let enc = build_encoding(vec![10u32, 20, 30], Some(&pad), 5, false);

        assert_eq!(enc.ids, vec![10u32, 20, 30, 0, 0]);
        assert_eq!(enc.attention_mask(), vec![1u32, 1, 1, 0, 0]);
        assert_eq!(enc.type_ids(), vec![0u32, 0, 0, 1, 1]);
    }

    #[test]
    fn build_encoding_left_padding_applies_pad_type_id() {
        let pad = PaddingParams {
            direction: Direction::Left,
            pad_id: 0,
            pad_type_id: 7,
            pad_token: "[PAD]".to_string(),
            strategy: PaddingStrategy::BatchLongest,
            pad_to_multiple_of: None,
        };
        let enc = build_encoding(vec![10u32, 20, 30], Some(&pad), 5, false);

        assert_eq!(enc.ids, vec![0u32, 0, 10, 20, 30]);
        assert_eq!(enc.attention_mask(), vec![0u32, 0, 1, 1, 1]);
        assert_eq!(enc.type_ids(), vec![7u32, 7, 0, 0, 0]);
    }
}
