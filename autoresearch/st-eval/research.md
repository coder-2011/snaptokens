# `.st` research notes, 2026-09-17

These are hypotheses, not retained optimizations.

- Bincode 2.0.1 `src/features/impl_alloc.rs`, locally installed pinned source:
  `Vec<u8>` takes one bulk read; all other Vec elements call Decode individually
  and update the decoder limit accounting per element. Inspect generated code
  and profile before testing a fixed-width bulk primitive decoder. Preserve
  little-endian bytes, element counts, malformed-input errors, and allocation
  limits. [Wincode](https://github.com/anza-xyz/wincode) provides an independent
  implementation of the direct-write approach; its schema contracts require
  all bit patterns, alignment, layout, and byte order to be valid. A new library
  is not yet justified over a small format-local change.
- [rkyv format](https://rkyv.org/format.html) and
  [validation](https://rkyv.org/validation.html): persistent native-like layouts
  need explicit endian/alignment/offset rules and bounded ownership validation.
  Zero-copy does not make the validation pass disappear. Consider indexed
  sections only if bincode and scattering dominate the actual profile.
- [memmap2 safety](https://docs.rs/memmap2/latest/memmap2/struct.Mmap.html): a
  read-only mapping is not immutable against external writes or truncation.
  The current safe public load API cannot simply assume external immutability.
  First measure read/copy cost and prefer owned bytes; direct mapping remains
  deferred unless a portable ownership argument is available.
- [simdutf8](https://docs.rs/simdutf8/latest/simdutf8/): whole-buffer validation
  can exploit SIMD, unlike hundreds of thousands of tiny string calls. First
  test representing the arena as one validated String plus character-boundary
  offsets, using std only. This could remove repeat validation in `get` too.
- [Rust Performance Book](https://nnethercote.github.io/perf-book/heap-allocations.html)
  and [heaptrack](https://github.com/KDE/heaptrack): count allocations on the
  executable, not Cargo. Profile construction and destruction separately from
  throughput timing; debug symbols improve attribution.
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) already dispatches SIMD. Hash
  parallelism is a candidate only if checksumming dominates sufficiently large
  files; thread startup and extra cores are separate costs, not free speed.

Open lanes to rank after profiling: bulk numeric decoding; validated String
arena; fused per-token validation/character tables; sparse-versus-dense stored
maps; lookup layout and probe validation; trie validation allocations; pipeline
JSON construction; cache initialization; payload read/copy; compact sections;
compression versus I/O; checksum parallelism. Do not reopen the rejected
BMP-persistence/hash-skipping experiment without immutable paired evidence.
