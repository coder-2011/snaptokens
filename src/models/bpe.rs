use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt,
    hash::BuildHasherDefault,
    mem::MaybeUninit,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use bincode::{Decode, Encode};
use dary_heap::QuaternaryHeap;
use serde::Deserialize;
use serde_json::Value;

use super::Result;
use crate::pre_tokenizers::{BYTE_TO_CHAR, FusedPieceSink};

type TokenId = u32;
type ParsedMergeMap = HashMap<(u32, u32), (u32, u32)>;
type Vocab = HashMap<String, u32>;

const INVALID_TOKEN: u32 = u32::MAX;
const DENSE_MERGE_BITS: u32 = 9;
const DENSE_MERGE_LIMIT: u32 = 1 << DENSE_MERGE_BITS;
const DENSE_MERGE_SIZE: usize = 1 << (DENSE_MERGE_BITS * 2);
const MAX_DENSE_RANKED_LIMIT: u32 = 1 << 10;
const FUSED_CACHE_PROBED_SEED_LIMIT: usize = 160 * 1024;

const EMPTY_KEY: u64 = u64::MAX;

#[inline(always)]
fn advise_huge_pages<T>(_slice: &[T]) {
    #[cfg(all(target_os = "linux", feature = "huge-pages"))]
    {
        let ptr = _slice.as_ptr();
        let len = std::mem::size_of_val(_slice);
        const PAGE_SIZE: usize = 4096;
        let aligned_ptr = (ptr as usize & !(PAGE_SIZE - 1)) as *mut libc::c_void;
        let aligned_len = (len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        unsafe {
            libc::madvise(aligned_ptr, aligned_len, libc::MADV_HUGEPAGE);
        }
    }
}

/// Bigram bridgeability table for vocab-aware safe splitting.
///
/// For each of 256×256 possible byte pairs, records whether that pair
/// can be covered by any vocabulary token. Used to identify split points
/// that cannot be crossed by BPE merges.
#[derive(Clone, PartialEq)]
pub struct BigramBridgeTable {
    bridgeable: Box<[u64; 1024]>,
}

impl BigramBridgeTable {
    /// Check if a byte pair can be bridged by some vocab token.
    #[inline(always)]
    pub fn is_bridgeable(&self, prev: u8, cur: u8) -> bool {
        let pair = prev as usize * 256 + cur as usize;
        self.bridgeable[pair / 64] & (1u64 << (pair % 64)) != 0
    }

    fn insert(&mut self, prev: u8, cur: u8) {
        let pair = prev as usize * 256 + cur as usize;
        self.bridgeable[pair / 64] |= 1u64 << (pair % 64);
    }
}

fn build_bigram_bridge_table(id_to_token: &[String], byte_fallback: bool) -> BigramBridgeTable {
    let mut table = BigramBridgeTable {
        bridgeable: Box::new([0; 1024]),
    };

    for token_str in id_to_token {
        let bytes = token_str.as_bytes();
        for window in bytes.windows(2) {
            table.insert(window[0], window[1]);
        }

        // Fallback markers encode bytes, so record the byte adjacencies of merged fallback tokens.
        if byte_fallback && memchr::memchr(b'<', bytes).is_some() {
            let mut previous = None;
            let mut remaining = bytes;
            while !remaining.is_empty() {
                let (byte, width) = parse_byte_fallback_prefix(remaining)
                    .map_or((remaining[0], 1), |byte| (byte, 6));
                if let Some(previous) = previous {
                    table.insert(previous, byte);
                }
                previous = Some(byte);
                remaining = &remaining[width..];
            }
        }
    }

    table
}

fn parse_byte_fallback_prefix(bytes: &[u8]) -> Option<u8> {
    if bytes.len() < 6 || &bytes[..3] != b"<0x" || bytes[5] != b'>' {
        return None;
    }
    let digits = std::str::from_utf8(&bytes[3..5]).ok()?;
    u8::from_str_radix(digits, 16).ok()
}

#[inline(always)]
fn pack_pair(t1: u32, t2: u32) -> u64 {
    (t1 as u64) << 32 | t2 as u64
}

#[inline(always)]
fn fx_hash(key: u64) -> u64 {
    key.wrapping_mul(0x517cc1b727220a95)
}

#[inline(always)]
fn fx_hash_bytes(bytes: &[u8], mut state: u64) -> u64 {
    let (words, tail) = bytes.as_chunks::<8>();
    for &bytes in words {
        // Vocabulary hashes are persisted in .st files and must survive byte-order changes.
        let word = u64::from_le_bytes(bytes);
        state = state.wrapping_add(word).wrapping_mul(0x517cc1b727220a95);
    }
    let (pairs, remainder) = tail.as_chunks::<2>();
    const K: u64 = 0x517cc1b727220a95;
    for &[first, second] in pairs {
        // Two serial steps equal (state + first) * K² + second * K modulo 2⁶⁴.
        state = state
            .wrapping_add(first as u64)
            .wrapping_mul(K.wrapping_mul(K))
            .wrapping_add((second as u64).wrapping_mul(K));
    }
    for &byte in remainder {
        state = state
            .wrapping_add(byte as u64)
            .wrapping_mul(0x517cc1b727220a95);
    }
    state
}

/// Empty VocabLookup slot. Real token hashes of 0 are remapped to 1 so probing
/// can stop at the first empty bucket.
const EMPTY_VOCAB_HASH: u64 = 0;

/// Hash vocabulary bytes with the same Fx mix used by the token caches.
fn vocab_hash(bytes: &[u8]) -> u64 {
    let hash = fx_hash_bytes(bytes, 0);
    if hash == EMPTY_VOCAB_HASH { 1 } else { hash }
}

/// Packed UTF-8 vocabulary: one byte buffer plus prefix offsets.
#[derive(Clone)]
struct PackedVocabulary {
    bytes: Vec<u8>,
    offsets: Vec<u32>,
}

impl PackedVocabulary {
    /// Copy owned construction strings into the packed snapshot layout.
    fn from_strings(tokens: &[String]) -> Result<Self> {
        let mut bytes = Vec::new();
        let mut offsets = Vec::with_capacity(tokens.len() + 1);
        offsets.push(0);
        for token in tokens {
            bytes.extend_from_slice(token.as_bytes());
            let offset =
                u32::try_from(bytes.len()).map_err(|_| "vocabulary arena exceeds u32 offsets")?;
            offsets.push(offset);
        }
        Ok(Self { bytes, offsets })
    }

    /// Accept a `.st` arena after checking monotonic offsets and UTF-8 spans.
    fn from_parts(bytes: Vec<u8>, offsets: Vec<u32>) -> Result<Self> {
        if offsets.len() < 2 || offsets[0] != 0 {
            return Err("invalid .st vocabulary offsets".into());
        }
        let last = *offsets.last().unwrap() as usize;
        if last != bytes.len() {
            return Err("invalid .st vocabulary arena length".into());
        }
        for pair in offsets.windows(2) {
            let start = pair[0] as usize;
            let end = pair[1] as usize;
            if start > end || end > bytes.len() {
                return Err("invalid .st vocabulary span".into());
            }
            std::str::from_utf8(&bytes[start..end]).map_err(|_| "invalid .st vocabulary utf-8")?;
        }
        Ok(Self { bytes, offsets })
    }

    fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    fn get(&self, id: u32) -> Option<&str> {
        let id = id as usize;
        let start = *self.offsets.get(id)? as usize;
        let end = *self.offsets.get(id + 1)? as usize;
        std::str::from_utf8(self.bytes.get(start..end)?).ok()
    }

    fn bytes_at(&self, id: usize) -> &[u8] {
        let start = self.offsets[id] as usize;
        let end = self.offsets[id + 1] as usize;
        &self.bytes[start..end]
    }

    fn len_at(&self, id: usize) -> usize {
        (self.offsets[id + 1] - self.offsets[id]) as usize
    }
}

/// Open-addressed token-to-id table over arena slices. Slots store the cached
/// hash and id; probes compare arena bytes and never clone keys.
#[derive(Clone)]
struct VocabLookup {
    mask: usize,
    hashes: Vec<u64>,
    ids: Vec<u32>,
}

impl VocabLookup {
    /// Build a power-of-two linear-probe table from already-packed spans.
    fn from_arena(arena: &PackedVocabulary) -> Result<Self> {
        if arena.len() == 0 {
            return Err("cannot build VocabLookup with empty vocabulary".into());
        }
        let capacity = arena
            .len()
            .checked_mul(2)
            .and_then(usize::checked_next_power_of_two)
            .ok_or("vocabulary lookup capacity overflow")?;
        let mask = capacity - 1;
        let mut hashes = vec![EMPTY_VOCAB_HASH; capacity];
        let mut ids = vec![0; capacity];
        for id in 0..arena.len() {
            let bytes = arena.bytes_at(id);
            let hash = vocab_hash(bytes);
            let mut idx = hash as usize & mask;
            loop {
                if hashes[idx] == EMPTY_VOCAB_HASH {
                    hashes[idx] = hash;
                    ids[idx] = id as u32;
                    break;
                }
                if hashes[idx] == hash && arena.bytes_at(ids[idx] as usize) == bytes {
                    return Err("duplicate token text in vocabulary".into());
                }
                idx = (idx + 1) & mask;
            }
        }
        Ok(Self { mask, hashes, ids })
    }

    /// Scatter persisted occupied slots and prove every probe chain plus hash.
    fn from_cached_slots(
        arena: &PackedVocabulary,
        capacity: u32,
        slots: &[u32],
        stored_hashes: &[u64],
        stored_ids: &[u32],
    ) -> Result<Self> {
        if slots.len() != stored_hashes.len() || stored_hashes.len() != stored_ids.len() {
            return Err("invalid .st vocabulary lookup length".into());
        }
        if slots.len() != arena.len() {
            return Err("invalid .st vocabulary lookup occupancy".into());
        }
        let expected_capacity = arena
            .len()
            .checked_mul(2)
            .and_then(usize::checked_next_power_of_two)
            .ok_or("vocabulary lookup capacity overflow")?;
        if capacity as usize != expected_capacity {
            return Err("invalid .st vocabulary lookup capacity".into());
        }
        let capacity = capacity as usize;
        let mask = capacity - 1;
        let mut hashes = vec![EMPTY_VOCAB_HASH; capacity];
        let mut ids = vec![0u32; capacity];
        let mut seen = vec![false; arena.len()];
        for ((&slot, &hash), &id) in slots.iter().zip(stored_hashes).zip(stored_ids) {
            let index = slot as usize;
            if index >= capacity
                || hashes[index] != EMPTY_VOCAB_HASH
                || hash == EMPTY_VOCAB_HASH
                || id as usize >= arena.len()
                || seen[id as usize]
                || hash != vocab_hash(arena.bytes_at(id as usize))
            {
                return Err("invalid .st vocabulary lookup slot".into());
            }
            hashes[index] = hash;
            ids[index] = id;
            seen[id as usize] = true;
        }
        if seen.iter().any(|&present| !present) {
            return Err("invalid .st vocabulary lookup coverage".into());
        }

        let first_empty = hashes
            .iter()
            .position(|&hash| hash == EMPTY_VOCAB_HASH)
            .ok_or("vocabulary lookup has no empty slot")?;
        let mut cluster_start = 1;
        for step in 1..capacity {
            let hash = hashes[(first_empty + step) & mask];
            if hash == EMPTY_VOCAB_HASH {
                cluster_start = step + 1;
                continue;
            }
            let home = hash as usize & mask;
            let relative_home = home.wrapping_add(capacity).wrapping_sub(first_empty) & mask;
            if relative_home < cluster_start || relative_home > step {
                return Err("invalid .st vocabulary lookup probe chain".into());
            }
        }

        Ok(Self { mask, hashes, ids })
    }

    fn get(&self, arena: &PackedVocabulary, query: &[u8]) -> Option<u32> {
        if self.hashes.is_empty() {
            return None;
        }
        let hash = vocab_hash(query);
        let mut idx = hash as usize & self.mask;
        loop {
            let slot_hash = self.hashes[idx];
            if slot_hash == EMPTY_VOCAB_HASH {
                return None;
            }
            if slot_hash == hash {
                let id = self.ids[idx];
                if arena.bytes_at(id as usize) == query {
                    return Some(id);
                }
            }
            idx = (idx + 1) & self.mask;
        }
    }
}

#[derive(Default)]
struct FxStrHasher(u64);

impl std::hash::Hasher for FxStrHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0 = fx_hash_bytes(bytes, self.0);
    }
}

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxStrHasher>>;

const PROBED_CACHE_BITS: usize = 18;
const PROBED_CACHE_SIZE: usize = 1 << PROBED_CACHE_BITS;
const EMPTY_SHORT_KEY: u128 = 0;
const DIRECT_CACHE_BITS: u32 = 17;
const DIRECT_CACHE_SIZE: usize = 1 << DIRECT_CACHE_BITS;
const PARALLEL_DIRECT_CACHE_BITS: u32 = 19;
const PARALLEL_DIRECT_CACHE_SIZE: usize = 1 << PARALLEL_DIRECT_CACHE_BITS;
const FUSED_PIECE_BATCH: usize = 256;
const FUSED_PIECE_CAPACITY: usize = FUSED_PIECE_BATCH + 64;

#[inline(always)]
fn pack_short_key(key: &str) -> Option<u128> {
    let bytes = key.as_bytes();
    if bytes.is_empty() || bytes.len() > 15 {
        return None;
    }
    let mut packed = (bytes.len() as u128) << 120;
    for (shift, &byte) in bytes.iter().enumerate() {
        packed |= (byte as u128) << (shift * 8);
    }
    Some(packed)
}

#[inline(always)]
const fn short_key_masks(len: usize) -> (u64, u64) {
    let bits = (len * 8) as u32;
    let low = if len < 8 {
        u64::MAX >> (64u32.wrapping_sub(bits) & 63)
    } else {
        u64::MAX
    };
    let high = if len > 8 {
        u64::MAX >> (128u32.wrapping_sub(bits) & 63)
    } else {
        0
    };
    (low, high)
}

#[inline(always)]
fn pack_short_range(input: &str, start: usize, end: usize) -> u128 {
    let len = end - start;
    debug_assert_ne!(len, 0);
    if len > 15 {
        return EMPTY_SHORT_KEY;
    }
    if start + 16 <= input.len() {
        return unsafe { pack_short_range_inbounds(input, start, len) };
    }
    pack_short_range_tail(input, start, end)
}

// Pack a short range after its caller proves sixteen readable bytes at `start`.
#[inline(always)]
unsafe fn pack_short_range_inbounds(input: &str, start: usize, len: usize) -> u128 {
    debug_assert!((1..=15).contains(&len));
    debug_assert!(start + 16 <= input.len());
    #[cfg(target_arch = "aarch64")]
    unsafe {
        use core::arch::aarch64::*;

        const LANES: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let raw = vld1q_u8(input.as_ptr().add(start));
        let live = vcltq_u8(vld1q_u8(LANES.as_ptr()), vdupq_n_u8(len as u8));
        let packed = vsetq_lane_u8::<15>(len as u8, vandq_u8(raw, live));
        let words = vreinterpretq_u64_u8(packed);
        let low = vgetq_lane_u64::<0>(words);
        let high = vgetq_lane_u64::<1>(words);
        (low as u128) | (high as u128) << 64
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::*;

        const LANES: [i8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let raw = _mm_loadu_si128(input.as_ptr().add(start).cast());
        let indices = _mm_loadu_si128(LANES.as_ptr().cast());
        let live = _mm_cmpgt_epi8(_mm_set1_epi8(len as i8), indices);
        let tag = _mm_slli_si128::<15>(_mm_cvtsi64_si128(len as i64));
        let packed = _mm_or_si128(_mm_and_si128(raw, live), tag);
        let low = _mm_cvtsi128_si64(packed) as u64;
        let high = _mm_cvtsi128_si64(_mm_srli_si128::<8>(packed)) as u64;
        (low as u128) | (high as u128) << 64
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        let word = unsafe { (input.as_ptr().add(start) as *const u128).read_unaligned() };
        let (low_mask, high_mask) = short_key_masks(len);
        let low = (word as u64) & low_mask;
        let high = ((word >> 64) as u64) & high_mask;
        (low as u128) | (high as u128) << 64 | (len as u128) << 120
    }
}

#[cold]
#[inline(never)]
fn pack_short_range_tail(input: &str, start: usize, end: usize) -> u128 {
    let len = end - start;
    if input.len() >= 16 {
        let window_start = input.len() - 16;
        let word = unsafe { (input.as_ptr().add(window_start) as *const u128).read_unaligned() };
        let shift = (start - window_start) * 8;
        let word = word >> shift;
        let (low_mask, high_mask) = short_key_masks(len);
        let low = (word as u64) & low_mask;
        let high = ((word >> 64) as u64) & high_mask;
        return (low as u128) | (high as u128) << 64 | (len as u128) << 120;
    }
    pack_short_key(&input[start..end]).unwrap_or(EMPTY_SHORT_KEY)
}

#[inline(always)]
fn packed_key_hash(key: u128) -> u64 {
    #[cfg(all(target_arch = "aarch64", target_feature = "crc"))]
    {
        use core::arch::aarch64::__crc32d;

        let crc = unsafe { __crc32d(__crc32d(0, key as u64), (key >> 64) as u64) };
        crc as u64
    }
    #[cfg(not(all(target_arch = "aarch64", target_feature = "crc")))]
    {
        let low = key as u64;
        let high = (key >> 64) as u64;
        let mut hash = (low ^ high.rotate_right(25)).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        hash ^= hash >> 32;
        hash
    }
}

#[inline(always)]
fn direct_cache_index(key: u128, mask: usize) -> usize {
    packed_key_hash(key) as usize & mask
}

#[inline(always)]
fn probed_cache_index(key: u128) -> usize {
    packed_key_hash(key) as usize & (PROBED_CACHE_SIZE - 1)
}

// Level 1 (fastest): direct-mapped — the key hashes to exactly one slot with no
// probing and no eviction policy. a colliding insert overwrites the value.
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct DirectCacheSlot {
    key: [u64; 2],
    value: u64,
    extension: u64,
}

const _: () = assert!(std::mem::size_of::<DirectCacheSlot>() == 32);

#[inline(always)]
fn append_direct_value(out: &mut Vec<u32>, value: u64, extension: u64) {
    let len = (value as u8) as usize;
    debug_assert!(out.capacity() - out.len() >= 4);
    debug_assert!(len <= 4);
    let ids = ((value >> 8) & 0x00ff_ffff) | (value & 0xffff_ffff_0000_0000);
    let start = out.len();
    unsafe {
        (out.as_mut_ptr().add(start) as *mut u64).write_unaligned(ids);
        (out.as_mut_ptr().add(start + 2) as *mut u64).write_unaligned(extension);
        out.set_len(start + len);
    }
}

#[inline(always)]
fn append_inline_value(out: &mut Vec<u32>, ids: &[u32; 2], len: usize) {
    debug_assert!(len <= 2 && out.capacity() - out.len() >= 2);
    let start = out.len();
    unsafe {
        std::ptr::copy_nonoverlapping(ids.as_ptr(), out.as_mut_ptr().add(start), 2);
        out.set_len(start + len);
    }
}

// Level 2: open-addressed with linear probing, so a lookup may walk several slots.
// Catches results displaced from or too wide for the direct cache. Up to two IDs
#[derive(Clone, Copy)]
#[repr(C)]
struct ProbedCacheSlot {
    key: [u64; 2],
    value: u64,
}

const _: () = assert!(std::mem::size_of::<ProbedCacheSlot>() == 24);

const PROBED_CACHE_MAX_LOAD: usize = PROBED_CACHE_SIZE * 3 / 4;
const PROBED_CACHE_MAX_POOL: usize = 64 * 1024 * 1024;

struct FlatCache {
    bpe_id: usize,
    // Level 1: direct-mapped, no probing; answers nearly every lookup in one slot read.
    direct_cache: Vec<DirectCacheSlot>,
    direct_mask: usize,
    // Level 2: linear probing; cleared wholesale at 3/4 load rather than tracking recency.
    probed_cache: Vec<ProbedCacheSlot>,
    // Spill space for probed results longer than two IDs, referenced as (offset, len).
    pool: Vec<u32>,
    // Level 3 (slowest local tier): ordinary hash map for pieces over 15 bytes.
    long_map: FxHashMap<Box<str>, (u32, u16)>,
    count: usize,
}

impl FlatCache {
    fn new() -> Self {
        Self::with_direct_size(DIRECT_CACHE_SIZE)
    }

    fn new_parallel() -> Self {
        Self::with_direct_size(PARALLEL_DIRECT_CACHE_SIZE)
    }

    fn with_direct_size(direct_size: usize) -> Self {
        debug_assert!(direct_size.is_power_of_two());
        let direct_cache = vec![DirectCacheSlot::default(); direct_size];
        let probed_cache = vec![
            ProbedCacheSlot {
                key: [0; 2],
                value: 0,
            };
            PROBED_CACHE_SIZE
        ];
        advise_huge_pages(&direct_cache);
        advise_huge_pages(&probed_cache);
        Self {
            bpe_id: 0,
            direct_cache,
            direct_mask: direct_size - 1,
            probed_cache,
            pool: Vec::with_capacity(256 * 1024),
            long_map: FxHashMap::default(),
            count: 0,
        }
    }

    #[inline(always)]
    fn direct_index(&self, key: u128) -> usize {
        direct_cache_index(key, self.direct_mask)
    }

    fn clear(&mut self) {
        self.direct_cache.fill(DirectCacheSlot::default());
        self.clear_probed();
    }

    fn clear_probed(&mut self) {
        for slot in &mut self.probed_cache {
            slot.key = [0; 2];
        }
        self.pool.clear();
        self.long_map.clear();
        self.count = 0;
    }

    #[inline(always)]
    fn prefetch_direct(&self, slot: *const DirectCacheSlot) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!(
                "prfm pldl1keep, [{ptr}]",
                ptr = in(reg) slot,
                options(readonly, nostack, preserves_flags)
            );
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};

            _mm_prefetch(slot.cast(), _MM_HINT_T0);
        }
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        let _ = slot;
    }

    #[inline(always)]
    fn get(&mut self, key: &str, out: &mut Vec<u32>) -> bool {
        let packed = pack_short_key(key).unwrap_or(EMPTY_SHORT_KEY);
        out.reserve(4);
        self.get_piece(key, packed, out)
    }

    #[inline(always)]
    fn get_piece(&mut self, key: &str, packed: u128, out: &mut Vec<u32>) -> bool {
        if packed != EMPTY_SHORT_KEY {
            return self.get_packed(packed, out);
        }
        let Some(&(offset, len)) = self.long_map.get(key) else {
            return false;
        };
        let start = offset as usize;
        out.extend_from_slice(unsafe { self.pool.get_unchecked(start..start + len as usize) });
        true
    }

    #[inline(always)]
    fn get_packed(&mut self, packed: u128, out: &mut Vec<u32>) -> bool {
        self.get_direct_packed(packed, out) || self.get_packed_probed(packed, out)
    }

    #[inline(always)]
    fn get_direct_packed(&self, packed: u128, out: &mut Vec<u32>) -> bool {
        let Some((value, extension)) = self.direct_packed_value(packed) else {
            return false;
        };
        append_direct_value(out, value, extension);
        true
    }

    #[inline(always)]
    fn direct_packed_value(&self, packed: u128) -> Option<(u64, u64)> {
        if packed == EMPTY_SHORT_KEY {
            return None;
        }
        let index = self.direct_index(packed);
        let slot = unsafe { self.direct_cache.as_ptr().add(index) };
        let key = [packed as u64, (packed >> 64) as u64];
        let (value, extension, found) = self.direct_packed_value_at(key, slot);
        found.then_some((value, extension))
    }

    #[inline(always)]
    fn direct_packed_value_at(
        &self,
        key: [u64; 2],
        slot: *const DirectCacheSlot,
    ) -> (u64, u64, bool) {
        debug_assert_ne!(key, [0; 2]);
        let slot = unsafe { &*slot };
        (slot.value, slot.extension, slot.key == key)
    }

    #[inline(always)]
    fn get_packed_probed(&mut self, packed: u128, out: &mut Vec<u32>) -> bool {
        let mut idx = probed_cache_index(packed);
        let key = [packed as u64, (packed >> 64) as u64];
        loop {
            let slot = unsafe { *self.probed_cache.get_unchecked(idx) };
            if slot.key == key {
                let output_start = out.len();
                let tag = (slot.value & 3) as usize;
                let len = if tag <= 2 {
                    // Tags 0..=2 store the length and two non-overlapping 31-bit IDs.
                    let ids = [
                        ((slot.value >> 2) & 0x7fff_ffff) as u32,
                        (slot.value >> 33) as u32,
                    ];
                    append_inline_value(out, &ids, tag);
                    tag
                } else {
                    let len = (slot.value >> 2) as u16 as usize;
                    let start = (slot.value >> 18) as usize;
                    let end = start + len;
                    // The pooled tag is published only after this exact range is appended.
                    out.extend_from_slice(unsafe { self.pool.get_unchecked(start..end) });
                    len
                };
                if len <= 4 {
                    self.insert_direct(packed, &out[output_start..]);
                }
                return true;
            }
            if slot.key == [0; 2] {
                return false;
            }
            idx = (idx + 1) & (PROBED_CACHE_SIZE - 1);
        }
    }

    #[inline(always)]
    fn insert(&mut self, key: &str, ids: &[u32]) {
        let packed = pack_short_key(key).unwrap_or(EMPTY_SHORT_KEY);
        self.insert_piece(key, packed, ids);
    }

    #[inline(always)]
    fn insert_piece(&mut self, key: &str, packed: u128, ids: &[u32]) {
        if packed != EMPTY_SHORT_KEY {
            self.insert_packed(packed, ids);
            return;
        }
        if self.pool.len() >= PROBED_CACHE_MAX_POOL {
            return;
        }
        let Ok(offset) = u32::try_from(self.pool.len()) else {
            return;
        };
        let Ok(len) = u16::try_from(ids.len()) else {
            return;
        };
        self.pool.extend_from_slice(ids);
        self.long_map.insert(key.into(), (offset, len));
    }

    #[inline(always)]
    fn insert_packed(&mut self, packed: u128, ids: &[u32]) {
        if self.insert_packed_probed(packed, ids) {
            self.insert_direct(packed, ids);
        }
    }

    fn insert_packed_probed(&mut self, packed: u128, ids: &[u32]) -> bool {
        if packed == EMPTY_SHORT_KEY {
            return false;
        }
        let Ok(len) = u16::try_from(ids.len()) else {
            return false;
        };
        if self.count >= PROBED_CACHE_MAX_LOAD || self.pool.len() >= PROBED_CACHE_MAX_POOL {
            self.clear_probed();
        }
        let mut idx = probed_cache_index(packed);
        let key = [packed as u64, (packed >> 64) as u64];
        loop {
            let slot = unsafe { *self.probed_cache.get_unchecked(idx) };
            if slot.key == [0; 2] {
                let Some(value) = self.store_value(ids, len) else {
                    return false;
                };
                self.count += 1;
                let slot = unsafe { self.probed_cache.get_unchecked_mut(idx) };
                slot.key = key;
                slot.value = value;
                return true;
            }
            if slot.key == key {
                let Some(value) = self.store_value(ids, len) else {
                    return false;
                };
                let slot = unsafe { self.probed_cache.get_unchecked_mut(idx) };
                slot.value = value;
                return true;
            }
            idx = (idx + 1) & (PROBED_CACHE_SIZE - 1);
        }
    }

    #[inline(always)]
    fn store_value(&mut self, ids: &[u32], len: u16) -> Option<u64> {
        let first = ids.first().copied().unwrap_or(0);
        let second = ids.get(1).copied().unwrap_or(0);
        if len <= 2 && (first | second) < 1 << 31 {
            return Some(len as u64 | (first as u64) << 2 | (second as u64) << 33);
        }
        // Tag 3 stores a u16 length and u32 pool offset; unrepresentable offsets must skip caching.
        let offset = u32::try_from(self.pool.len()).ok()?;
        self.pool.extend_from_slice(ids);
        Some(3 | (len as u64) << 2 | (offset as u64) << 18)
    }

    #[inline(always)]
    fn insert_direct(&mut self, key: u128, ids: &[u32]) {
        if !(1..=4).contains(&ids.len()) || ids[0] >= 1 << 24 {
            return;
        }
        let second = ids.get(1).copied().unwrap_or(0);
        let third = ids.get(2).copied().unwrap_or(0);
        let fourth = ids.get(3).copied().unwrap_or(0);
        let value = ids.len() as u64 | (ids[0] as u64) << 8 | (second as u64) << 32;
        let extension = third as u64 | (fourth as u64) << 32;
        let slot = DirectCacheSlot {
            key: [key as u64, (key >> 64) as u64],
            value,
            extension,
        };
        let index = self.direct_index(key);
        self.direct_cache[index] = slot;
    }
}

thread_local! {
    static TL_BPE_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new());
    static TL_FUSED_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new());
    static TL_FUSED_PARALLEL_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new_parallel());
}

#[derive(Clone, Copy)]
struct FusedPiece {
    slot: *const DirectCacheSlot,
    key: [u64; 2],
}

const _: () = assert!(std::mem::size_of::<FusedPiece>() == 24);

// Batches scanner pieces to prefetch direct-cache slots before probing them; queued keys own the input bytes.
pub(crate) struct EncodeStream<'a> {
    model: &'a Bpe,
    cache: &'a mut FlatCache,
    out: &'a mut Vec<u32>,
    error: Option<String>,
    // Only the prefix before `pending_len` is initialized and read.
    pending: &'a mut [MaybeUninit<FusedPiece>; FUSED_PIECE_CAPACITY],
    pending_len: usize,
}

impl EncodeStream<'_> {
    #[inline(always)]
    pub(crate) fn push_id(&mut self, id: u32) {
        self.flush();
        if self.error.is_none() {
            self.out.push(id);
        }
    }

    #[inline(always)]
    pub(crate) fn flush_pending(&mut self) {
        self.flush();
    }

    #[inline(always)]
    pub(crate) fn output_len(&mut self) -> std::result::Result<usize, crate::Error> {
        self.flush_pending();
        self.error
            .take()
            .map_or(Ok(self.out.len()), |error| Err(crate::Error::Model(error)))
    }

    #[inline(always)]
    pub(crate) unsafe fn push(&mut self, input: &str, start: usize, end: usize) {
        if self.pending_len >= FUSED_PIECE_BATCH {
            self.flush();
            if self.error.is_some() {
                return;
            }
        }
        let len = end - start;
        if len > 15 {
            self.push_long(input, start, end);
            return;
        }
        let direct_cache = self.cache.direct_cache.as_ptr();
        let pending = unsafe {
            self.pending
                .as_mut_ptr()
                .add(self.pending_len)
                .cast::<FusedPiece>()
        };
        unsafe { self.push_short::<false>(input, start, end, direct_cache, pending) };
        self.pending_len += 1;
    }

    #[inline(always)]
    unsafe fn push_short<const INBOUNDS: bool>(
        &mut self,
        input: &str,
        start: usize,
        end: usize,
        direct_cache: *const DirectCacheSlot,
        pending: *mut FusedPiece,
    ) {
        let len = end - start;
        debug_assert!(len <= 15);
        let packed = if INBOUNDS {
            unsafe { pack_short_range_inbounds(input, start, len) }
        } else {
            pack_short_range(input, start, end)
        };
        let index = self.cache.direct_index(packed);
        // The fixed-size direct-cache allocation stays live for this stream.
        let slot = unsafe { direct_cache.add(index) };
        unsafe {
            pending.write(FusedPiece {
                slot,
                key: [packed as u64, (packed >> 64) as u64],
            })
        };
        self.cache.prefetch_direct(slot);
    }

    #[cold]
    #[inline(never)]
    fn push_long(&mut self, input: &str, start: usize, end: usize) {
        self.flush();
        if self.error.is_none()
            && let Err(current) = self.model.append_piece_bpe_ids_cache_miss(
                self.cache,
                input,
                start..end,
                EMPTY_SHORT_KEY,
                self.out,
                false,
            )
        {
            self.error = Some(current);
        }
    }

    #[inline(never)]
    fn flush(&mut self) {
        let count = std::mem::take(&mut self.pending_len);
        if count == 0 || self.error.is_some() {
            return;
        }
        let out = &mut *self.out;
        out.reserve(4 * count);
        let mut destination = unsafe { out.as_mut_ptr().add(out.len()) };
        let pending = unsafe {
            std::slice::from_raw_parts(self.pending.as_ptr().cast::<FusedPiece>(), count)
        };
        for (index, &piece) in pending.iter().enumerate() {
            let (value, extension, found) =
                self.cache.direct_packed_value_at(piece.key, piece.slot);
            let ids = ((value >> 8) & 0x00ff_ffff) | (value & 0xffff_ffff_0000_0000);
            // SAFETY: initial/miss reserves leave four writable lanes per piece; miss stores remain past the cursor.
            unsafe {
                (destination as *mut u64).write_unaligned(ids);
                (destination.add(2) as *mut u64).write_unaligned(extension);
                destination = destination.add(if found { value as u8 as usize } else { 0 });
            }
            if found {
                continue;
            }

            unsafe { out.set_len(destination.offset_from(out.as_ptr()) as usize) };
            let packed = piece.key[0] as u128 | (piece.key[1] as u128) << 64;
            let result = if self.cache.get_packed_probed(packed, out) {
                Ok(())
            } else {
                let bytes = packed.to_le_bytes();
                let len = (piece.key[1] >> 56) as usize;
                // SAFETY: the packed key came from one exact UTF-8 scanner range.
                let input = unsafe { std::str::from_utf8_unchecked(&bytes[..len]) };
                self.model.append_uncached_piece_bpe_ids(
                    self.cache,
                    input,
                    0..len,
                    packed,
                    out,
                    false,
                )
            };
            if let Err(current) = result {
                self.error = Some(current);
                return;
            }
            out.reserve(4 * (count - index - 1));
            destination = unsafe { out.as_mut_ptr().add(out.len()) };
        }
        unsafe { out.set_len(destination.offset_from(out.as_ptr()) as usize) };
    }
}

impl FusedPieceSink for EncodeStream<'_> {
    #[inline(always)]
    unsafe fn push_piece(&mut self, input: &str, start: usize, end: usize) {
        unsafe { self.push(input, start, end) };
    }

    #[inline(always)]
    unsafe fn push_mask(&mut self, input: &str, mask_base: usize, start: &mut usize, mask: u64) {
        let inbounds = input.len().saturating_sub(mask_base) >= 78;
        let first_end = mask_base + mask.trailing_zeros() as usize;
        // Smearing boundaries by 1..=15 bits proves every internal gap fits a short key.
        let mut nearby = mask << 1;
        nearby |= nearby << 1;
        nearby |= nearby << 2;
        nearby |= nearby << 4;
        nearby |= nearby << 7;
        let all_short = first_end - *start <= 15 && mask & (mask - 1) & !nearby == 0;
        if all_short && inbounds {
            unsafe { self.push_mask_ranges::<true, true>(input, mask_base, start, mask) };
        } else if all_short {
            unsafe { self.push_mask_ranges::<false, true>(input, mask_base, start, mask) };
        } else if inbounds {
            unsafe { self.push_mask_ranges::<true, false>(input, mask_base, start, mask) };
        } else {
            unsafe { self.push_mask_ranges::<false, false>(input, mask_base, start, mask) };
        }
    }
}

impl EncodeStream<'_> {
    #[inline(always)]
    unsafe fn push_mask_ranges<const INBOUNDS: bool, const ALL_SHORT: bool>(
        &mut self,
        input: &str,
        mask_base: usize,
        start: &mut usize,
        mut mask: u64,
    ) {
        // One u64 mask adds at most 64 ranges, which fit in the batch's slack.
        if self.pending_len >= FUSED_PIECE_BATCH {
            self.flush();
        }
        // Misses never resize the fixed-size direct-cache allocation.
        let direct_cache = self.cache.direct_cache.as_ptr();
        let pending_base = self.pending.as_mut_ptr().cast::<FusedPiece>();
        let mut pending = unsafe { pending_base.add(self.pending_len) };
        while mask != 0 {
            let end = mask_base + mask.trailing_zeros() as usize;
            mask &= mask - 1;
            if !ALL_SHORT && end - *start > 15 {
                self.pending_len = unsafe { pending.offset_from(pending_base) as usize };
                self.push_long(input, *start, end);
                pending = pending_base;
            } else {
                unsafe { self.push_short::<INBOUNDS>(input, *start, end, direct_cache, pending) };
                pending = unsafe { pending.add(1) };
            }
            *start = end;
        }
        self.pending_len = unsafe { pending.offset_from(pending_base) as usize };
    }
}

const CACHE_SHARDS: usize = 64;
const CROSS_THREAD_MAX_PER_SHARD: usize = 16 * 1024;

// Level 4 (slowest, but process-wide): the only tier shared between threads. A piece
// computed anywhere is published here so other threads skip the merge loop; probed
// only after every thread-local tier misses. Sharded 64 ways to spread lock traffic.
struct CrossThreadCache {
    shards: Vec<Mutex<FxHashMap<String, Vec<u32>>>>,
}

impl CrossThreadCache {
    fn new() -> Self {
        Self {
            shards: (0..CACHE_SHARDS)
                .map(|_| Mutex::new(FxHashMap::default()))
                .collect(),
        }
    }

    #[inline]
    fn shard_index(key: &str) -> usize {
        let bytes = key.as_bytes();
        let mut h: u64 = bytes.len() as u64;
        for &b in &bytes[..bytes.len().min(8)] {
            h = h.wrapping_add(b as u64).wrapping_mul(0x9E3779B97F4A7C15);
        }
        h as usize & (CACHE_SHARDS - 1)
    }

    #[inline]
    fn get_into(&self, key: &str, out: &mut Vec<u32>) -> bool {
        let shard = self.shards[Self::shard_index(key)].lock().unwrap();
        if let Some(ids) = shard.get(key) {
            out.extend_from_slice(ids);
            true
        } else {
            false
        }
    }

    fn insert(&self, key: String, value: Vec<u32>) {
        let mut shard = self.shards[Self::shard_index(&key)].lock().unwrap();
        if shard.len() >= CROSS_THREAD_MAX_PER_SHARD {
            shard.clear();
        }
        shard.insert(key, value);
    }
}

#[derive(Deserialize)]
struct RawBpe {
    #[serde(rename = "type", default, deserialize_with = "deserialize_present")]
    model_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_present")]
    vocab: Option<Vocab>,
    #[serde(default, deserialize_with = "deserialize_present")]
    merges: Option<Vec<Value>>,
    #[serde(default)]
    byte_fallback: bool,
    #[serde(default)]
    ignore_merges: bool,
}

// Distinguish an omitted optional JSON field from a present invalid `null` value.
fn deserialize_present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Encode, Decode, PartialEq)]
struct ExactTokenTrie {
    nodes: Vec<ExactTokenTrieNode>,
    incoming_bytes: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq)]
struct ExactTokenTrieNode {
    first_child: u32,
    edge_count: u16,
    token: TokenId,
}

#[derive(Clone, PartialEq)]
enum ExactTokenMatcher {
    Direct(Vec<bool>),
    Trie(ExactTokenTrie),
}

static BPE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

impl ExactTokenTrie {
    fn validate_with<'a>(
        self,
        vocab_size: usize,
        is_orphan: &[bool],
        token_bytes: impl Fn(usize) -> &'a [u8],
    ) -> Result<Self> {
        if self.nodes.is_empty()
            || vocab_size == 0
            || vocab_size != is_orphan.len()
            || self.nodes[0].token != INVALID_TOKEN
        {
            return Err("invalid .st exact-token trie root".into());
        }

        if self.incoming_bytes.len() != self.nodes.len() - 1 {
            return Err("exact-token trie incoming-byte count is invalid".into());
        }

        let mut parent_counts = vec![0u8; self.nodes.len()];
        for (node_index, node) in self.nodes.iter().enumerate() {
            if node.edge_count == 0 {
                if node.first_child != 0 {
                    return Err("leaf exact-token trie node has a child range".into());
                }
                continue;
            }
            let first = node.first_child as usize;
            let end = first
                .checked_add(node.edge_count as usize)
                .ok_or("exact-token trie child range overflow")?;
            if first == 0 || first <= node_index || end > self.nodes.len() {
                return Err("exact-token trie has an invalid child range".into());
            }
            let bytes = self
                .incoming_bytes
                .get(first - 1..end - 1)
                .ok_or("exact-token trie child bytes are out of bounds")?;
            if bytes.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err("exact-token trie child bytes are not strictly ordered".into());
            }
            for parent_count in &mut parent_counts[first..end] {
                *parent_count = parent_count
                    .checked_add(1)
                    .ok_or("exact-token trie node has multiple parents")?;
            }
        }
        if parent_counts[0] != 0 || parent_counts[1..].iter().any(|&count| count != 1) {
            return Err("exact-token trie is not one rooted tree".into());
        }

        enum Visit {
            Enter(u32),
            Exit(bool),
        }

        let mut seen_tokens = vec![false; vocab_size];
        let mut path = Vec::new();
        let mut visits = vec![Visit::Enter(0)];
        while let Some(visit) = visits.pop() {
            match visit {
                Visit::Enter(node) => {
                    let node_index = node as usize;
                    let pushed = node_index != 0;
                    if pushed {
                        path.push(self.incoming_bytes[node_index - 1]);
                    }
                    let node = &self.nodes[node_index];
                    if node.token != INVALID_TOKEN {
                        let token = node.token as usize;
                        if token >= vocab_size
                            || is_orphan[token]
                            || seen_tokens[token]
                            || token_bytes(token) != path
                        {
                            return Err(
                                "exact-token trie terminal does not match vocabulary".into()
                            );
                        }
                        seen_tokens[token] = true;
                    }

                    visits.push(Visit::Exit(pushed));
                    let first = node.first_child as usize;
                    let end = first + node.edge_count as usize;
                    for child in (first..end).rev() {
                        visits.push(Visit::Enter(child as u32));
                    }
                }
                Visit::Exit(pushed) => {
                    if pushed {
                        path.pop();
                    }
                }
            }
        }
        if seen_tokens
            .iter()
            .zip(is_orphan)
            .any(|(&seen, &orphan)| seen == orphan)
        {
            return Err("exact-token trie does not cover canonical vocabulary".into());
        }
        Ok(self)
    }

    fn next_match(&self, input: &str) -> Option<TokenId> {
        let mut node = 0usize;
        let mut matched = None;
        for &byte in input.as_bytes() {
            let current = &self.nodes[node];
            let first = current.first_child as usize;
            let end = first + current.edge_count as usize;
            if current.edge_count == 0 {
                break;
            }
            let child_offset = if current.edge_count <= 4 {
                self.incoming_bytes[first - 1..end - 1]
                    .iter()
                    .position(|&candidate| candidate == byte)
            } else {
                self.incoming_bytes[first - 1..end - 1]
                    .binary_search(&byte)
                    .ok()
            };
            let Some(child_offset) = child_offset else {
                break;
            };
            node = first + child_offset;
            let token = self.nodes[node].token;
            if token != INVALID_TOKEN {
                matched = Some(token);
            }
        }
        matched
    }
}

impl ExactTokenMatcher {
    // Return a legal match; the caller still proves that it covers the whole input.
    fn next_match(
        &self,
        input: &str,
        lookup: &VocabLookup,
        arena: &PackedVocabulary,
    ) -> Option<TokenId> {
        match self {
            Self::Direct(is_orphan) => {
                let token = lookup.get(arena, input.as_bytes())?;
                (!is_orphan[token as usize]).then_some(token)
            }
            Self::Trie(trie) => trie.next_match(input),
        }
    }
}

#[inline]
fn next_bpe_id() -> usize {
    let id = BPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    if id == 0 {
        BPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    } else {
        id
    }
}

// `key = (rank << 32) | pos`, `val = (left_c << 32) | right_c`.
#[derive(Clone, Copy, Eq)]
#[repr(C)]
struct MergeEntry {
    key: u64,
    val: u64,
}

impl MergeEntry {
    #[inline(always)]
    fn new(rank: u32, pos: u32, left_c: u32, right_c: u32) -> Self {
        Self {
            key: (rank as u64) << 32 | pos as u64,
            val: (left_c as u64) << 32 | right_c as u64,
        }
    }

    #[inline(always)]
    fn pos(&self) -> u32 {
        self.key as u32
    }

    #[inline(always)]
    fn rank(&self) -> u32 {
        (self.key >> 32) as u32
    }

    #[inline(always)]
    fn left_c(&self) -> u32 {
        (self.val >> 32) as u32
    }

    #[inline(always)]
    fn right_c(&self) -> u32 {
        self.val as u32
    }
}

impl PartialEq for MergeEntry {
    // Equality follows heap priority; the payload only validates stale candidates.
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Ord for MergeEntry {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for MergeEntry {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct MergeSymbol {
    c: u32,
    prev: i32,
    next: i32,
}

#[derive(Default)]
struct MergeScratch {
    symbols: Vec<MergeSymbol>,
    heap: BinaryHeap<Reverse<MergeEntry>>,
    heap_buf: Vec<Reverse<MergeEntry>>,
}

#[derive(Default)]
struct EncodedMergeScratch {
    symbols: Vec<MergeSymbol>,
    heap: QuaternaryHeap<Reverse<MergeEntry>>,
}

impl EncodedMergeScratch {
    #[inline(always)]
    fn push_encoded_symbol(&mut self, token: TokenId) {
        let index = self.symbols.len() as i32;
        if let Some(previous) = self.symbols.last_mut() {
            previous.next = index;
        }
        self.symbols.push(MergeSymbol {
            c: token,
            prev: index - 1,
            next: -1,
        });
    }
}

macro_rules! run_merge_loop_body {
    ($bpe:ident, $scratch:ident, $out:ident) => {{
        let symbols = &mut $scratch.symbols;
        let heap = &mut $scratch.heap;

        while let Some(Reverse(entry)) = heap.pop() {
            let pos = entry.pos() as usize;
            let sym = symbols[pos];

            let left_c = entry.left_c();
            let right_c = entry.right_c();
            if sym.c != left_c {
                continue;
            }
            let next_idx = sym.next;
            if next_idx < 0 {
                continue;
            }
            let next_idx = next_idx as usize;
            let next_sym = symbols[next_idx];
            if next_sym.c != right_c {
                continue;
            }

            let new_id = match $bpe
                .merge_result_ids
                .get(entry.rank() as usize)
                .copied()
                .filter(|&id| id != INVALID_TOKEN)
                .or_else(|| $bpe.merge_adj.get(left_c, right_c).map(|(_, id)| id))
            {
                Some(id) => id,
                None => continue,
            };

            symbols[pos].c = new_id;
            symbols[pos].next = next_sym.next;
            if next_sym.next >= 0 {
                symbols[next_sym.next as usize].prev = pos as i32;
            }
            symbols[next_idx].c = INVALID_TOKEN;

            if sym.prev >= 0 {
                let prev_c = symbols[sym.prev as usize].c;
                if let Some((rank, _)) = $bpe.merge_adj.get(prev_c, new_id) {
                    heap.push(Reverse(MergeEntry::new(
                        rank,
                        sym.prev as u32,
                        prev_c,
                        new_id,
                    )));
                }
            }
            let new_next = symbols[pos].next;
            if new_next >= 0 {
                let next_c = symbols[new_next as usize].c;
                if let Some((rank, _)) = $bpe.merge_adj.get(new_id, next_c) {
                    heap.push(Reverse(MergeEntry::new(rank, pos as u32, new_id, next_c)));
                }
            }
        }

        let mut index = 0_i32;
        while index >= 0 {
            let symbol = symbols[index as usize];
            $out.push(symbol.c);
            index = symbol.next;
        }
    }};
}

thread_local! {
    static TL_MERGE_SCRATCH: RefCell<MergeScratch> = RefCell::new(MergeScratch::default());
    static TL_ENCODED_MERGE_SCRATCH: RefCell<EncodedMergeScratch> =
        RefCell::new(EncodedMergeScratch::default());
}

// Equal power-of-two arrays share a mask and retain an empty key to terminate unchecked probing.
#[derive(Clone, PartialEq)]
struct RankedMergeMap {
    mask: usize,
    keys: Vec<u64>,
    // Packed `rank << 32 | merged_id`, indexed by the matching pair's key slot.
    values: Vec<u64>,
}

impl RankedMergeMap {
    fn from_parsed(parsed: &ParsedMergeMap) -> Self {
        if parsed.is_empty() {
            return Self {
                mask: 0,
                keys: Vec::new(),
                values: Vec::new(),
            };
        }
        let capacity = (parsed.len() * 2).next_power_of_two();
        let mask = capacity - 1;
        let mut keys = vec![EMPTY_KEY; capacity];
        let mut values = vec![0; capacity];

        for (&(t1, t2), &(rank, merged_id)) in parsed {
            let key = pack_pair(t1, t2);
            let mut idx = fx_hash(key) as usize & mask;
            loop {
                if keys[idx] == EMPTY_KEY {
                    keys[idx] = key;
                    values[idx] = (rank as u64) << 32 | merged_id as u64;
                    break;
                }
                idx = (idx + 1) & mask;
            }
        }

        Self { mask, keys, values }
    }

    #[inline(always)]
    fn get(&self, t1: u32, t2: u32) -> Option<(u32, u32)> {
        if self.keys.is_empty() {
            return None;
        }
        let key = pack_pair(t1, t2);
        let mut idx = fx_hash(key) as usize & self.mask;
        loop {
            let slot_key = unsafe { *self.keys.get_unchecked(idx) };
            if slot_key == key {
                let payload = unsafe { *self.values.get_unchecked(idx) };
                return Some(((payload >> 32) as u32, payload as u32));
            }
            if slot_key == EMPTY_KEY {
                return None;
            }
            idx = (idx + 1) & self.mask;
        }
    }

    fn len(&self) -> usize {
        self.keys.iter().filter(|&&key| key != EMPTY_KEY).count()
    }
}

#[derive(Clone)]
struct MergeAdjacency {
    offsets: Vec<u32>,
    // Sorted `neighbor << 32 | rank` keys keep payload reads out of unsuccessful probes.
    keys: Vec<u64>,
    new_ids: Vec<u32>,
}

impl MergeAdjacency {
    fn result_ids_by_rank(&self) -> Vec<u32> {
        let Some(len) = self
            .keys
            .iter()
            .map(|&key| key as u32)
            .max()
            .and_then(|rank| (rank as usize).checked_add(1))
            .filter(|&len| len <= self.keys.len().saturating_mul(2))
        else {
            return Vec::new();
        };
        let mut results = vec![INVALID_TOKEN; len];
        for (&key, &id) in self.keys.iter().zip(&self.new_ids) {
            let slot = &mut results[key as u32 as usize];
            if *slot != INVALID_TOKEN && *slot != id {
                return Vec::new();
            }
            *slot = id;
        }
        results
    }

    fn from_parsed(parsed: &ParsedMergeMap, vocab_size: usize) -> Self {
        let mut counts = vec![0u32; vocab_size];
        for &(left, _right) in parsed.keys() {
            counts[left as usize] += 1;
        }

        let mut offsets = Vec::with_capacity(vocab_size + 1);
        offsets.push(0u32);
        let mut running = 0u32;
        for &c in &counts {
            running += c;
            offsets.push(running);
        }

        let mut rows = vec![(0u64, 0u32); running as usize];
        let mut write_pos = offsets[..vocab_size].to_vec();
        for (&(left, right), &(rank, merged_id)) in parsed {
            let idx = write_pos[left as usize] as usize;
            rows[idx] = ((right as u64) << 32 | rank as u64, merged_id);
            write_pos[left as usize] += 1;
        }

        for i in 0..vocab_size {
            let start = offsets[i] as usize;
            let end = offsets[i + 1] as usize;
            rows[start..end].sort_unstable_by_key(|&(key, _)| key);
        }

        let (keys, new_ids) = rows.into_iter().unzip();
        Self {
            offsets,
            keys,
            new_ids,
        }
    }

    #[inline(always)]
    fn get(&self, left: u32, right: u32) -> Option<(u32, u32)> {
        let start = unsafe { *self.offsets.get_unchecked(left as usize) } as usize;
        let end = unsafe { *self.offsets.get_unchecked(left as usize + 1) } as usize;
        let keys = unsafe { self.keys.get_unchecked(start..end) };
        let target = (right as u64) << 32;
        let idx = keys.partition_point(|&key| key < target);
        match keys.get(idx) {
            Some(&key) if (key >> 32) as u32 == right => {
                let new_id = unsafe { *self.new_ids.get_unchecked(start + idx) };
                Some((key as u32, new_id))
            }
            _ => None,
        }
    }
}

/// A fully parsed BPE vocabulary, merge graph, and encode-time caches.
#[derive(Deserialize)]
#[serde(try_from = "RawBpe")]
pub struct Bpe {
    #[serde(skip)]
    id: usize,
    matcher: ExactTokenMatcher,
    unmerge_map: Vec<(TokenId, TokenId)>,
    // Lengths below 255 are inline; 255 requests the exact vocabulary string length.
    token_lens: Vec<u8>,
    cross_thread_cache: CrossThreadCache,
    fused_cross_thread_cache: CrossThreadCache,
    packed_vocabulary: PackedVocabulary,
    vocab_lookup: VocabLookup,
    bmp_char_token: Box<[u32]>,
    byte_to_initial_token: [u32; 256],
    byte_fallback_token_ids: [u32; 256],
    single_char_token: [u32; 128],
    ranked_merge_map: RankedMergeMap,
    byte_pair_initial: Vec<(u32, u32)>,
    dense_merge: Vec<u64>,
    dense_ranked_merge: Vec<u32>,
    // Bit width of each dense ranked-table endpoint; zero means no table.
    dense_ranked_bits: u32,
    ranked_merges: bool,
    fused_cache_seeds: Vec<(u128, u32)>,
    merge_adj: MergeAdjacency,
    // Only accepted heap candidates read this table; empty keeps the adjacency fallback.
    merge_result_ids: Vec<u32>,
    ignore_merges: bool,
    byte_fallback: bool,
    /// Byte-pair coverage used to find BPE-safe input split boundaries.
    pub bigram_bridge_table: BigramBridgeTable,
}

impl TryFrom<RawBpe> for Bpe {
    type Error = String;

    fn try_from(raw: RawBpe) -> Result<Self> {
        let RawBpe {
            model_type,
            vocab,
            merges,
            byte_fallback,
            ignore_merges,
        } = raw;
        let (vocab, merges) = match model_type.as_deref() {
            Some("BPE") => (vocab.unwrap_or_default(), merges.unwrap_or_default()),
            Some(model_type) => return Err(format!("unsupported model type: {model_type}")),
            None => match (vocab, merges) {
                (Some(vocab), Some(merges)) => (vocab, merges),
                _ => return Err("legacy BPE model requires vocab and merges".into()),
            },
        };
        let merge_map = parse_merges(&vocab, &merges)?;
        Self::build(vocab, merge_map, byte_fallback, ignore_merges)
    }
}

enum Decomposition {
    Pair(TokenId, TokenId, TokenId),
    CharsNotInVocab,
    Stuck,
}

const DECOMPOSITION_STACK_CAPACITY: usize = 16;

fn decomposition_initial_token(ch: char, vocab: &Vocab, bmp_char_token: &[u32]) -> Option<TokenId> {
    if (ch as u32) < 0x10000 {
        let token = bmp_char_token[ch as usize];
        (token != INVALID_TOKEN).then_some(token)
    } else {
        let mut buf = [0u8; 4];
        vocab.get(ch.encode_utf8(&mut buf)).copied()
    }
}

fn decomposition_merge(
    left: TokenId,
    right: TokenId,
    initial_token_byte: &[u16],
    byte_pair_initial: &[(u32, u32)],
    merge_adjacency: &MergeAdjacency,
) -> Option<(u32, u32)> {
    let left_byte = initial_token_byte[left as usize];
    let right_byte = initial_token_byte[right as usize];
    if left_byte != u16::MAX && right_byte != u16::MAX {
        let pair = byte_pair_initial[left_byte as usize * 256 + right_byte as usize];
        return (pair.0 != u32::MAX).then_some(pair);
    }
    merge_adjacency.get(left, right)
}

fn reduce_decomposition_tokens(
    tokens: &mut [TokenId],
    initial_token_byte: &[u16],
    byte_pair_initial: &[(u32, u32)],
    merge_adjacency: &MergeAdjacency,
) -> Decomposition {
    let mut len = tokens.len();
    if len < 2 {
        return Decomposition::CharsNotInVocab;
    }

    loop {
        let mut best_rank = u32::MAX;
        let mut best_pos = usize::MAX;
        let mut best_new = 0;
        for i in 0..len - 1 {
            if let Some((rank, new_id)) = decomposition_merge(
                tokens[i],
                tokens[i + 1],
                initial_token_byte,
                byte_pair_initial,
                merge_adjacency,
            ) && rank < best_rank
            {
                best_rank = rank;
                best_pos = i;
                best_new = new_id;
            }
        }
        if best_pos == usize::MAX {
            return Decomposition::Stuck;
        }
        if len == 2 {
            return Decomposition::Pair(tokens[0], tokens[1], best_new);
        }
        tokens[best_pos] = best_new;
        tokens.copy_within(best_pos + 1..len, best_pos);
        len -= 1;
    }
}

fn encoding_decomposition_heap(
    text: &str,
    vocab: &Vocab,
    initial_token_byte: &[u16],
    byte_pair_initial: &[(u32, u32)],
    merge_adjacency: &MergeAdjacency,
    bmp_char_token: &[u32],
) -> Decomposition {
    let mut tokens = Vec::new();
    for ch in text.chars() {
        let Some(token) = decomposition_initial_token(ch, vocab, bmp_char_token) else {
            return Decomposition::CharsNotInVocab;
        };
        tokens.push(token);
    }
    reduce_decomposition_tokens(
        &mut tokens,
        initial_token_byte,
        byte_pair_initial,
        merge_adjacency,
    )
}

fn encoding_decomposition(
    text: &str,
    vocab: &Vocab,
    initial_token_byte: &[u16],
    byte_pair_initial: &[(u32, u32)],
    merge_adjacency: &MergeAdjacency,
    bmp_char_token: &[u32],
) -> Decomposition {
    let mut tokens = [INVALID_TOKEN; DECOMPOSITION_STACK_CAPACITY];
    let mut len = 0;
    for ch in text.chars() {
        if len == tokens.len() {
            return encoding_decomposition_heap(
                text,
                vocab,
                initial_token_byte,
                byte_pair_initial,
                merge_adjacency,
                bmp_char_token,
            );
        }
        let Some(token) = decomposition_initial_token(ch, vocab, bmp_char_token) else {
            return Decomposition::CharsNotInVocab;
        };
        tokens[len] = token;
        len += 1;
    }
    reduce_decomposition_tokens(
        &mut tokens[..len],
        initial_token_byte,
        byte_pair_initial,
        merge_adjacency,
    )
}

fn parse_merges(vocab: &Vocab, merges: &[Value]) -> Result<ParsedMergeMap> {
    let mut merge_map = ParsedMergeMap::with_capacity(merges.len());
    let mut merged = String::new();
    for (rank, entry) in merges.iter().enumerate() {
        let (left, right) = parse_merge_entry(entry)?;
        let &left_id = vocab
            .get(left)
            .ok_or_else(|| format!("merge token not in vocab: {left:?}"))?;
        let &right_id = vocab
            .get(right)
            .ok_or_else(|| format!("merge token not in vocab: {right:?}"))?;
        merged.clear();
        merged.push_str(left);
        merged.push_str(right);
        let &merged_id = vocab
            .get(merged.as_str())
            .ok_or_else(|| format!("merged token not in vocab: {merged:?}"))?;
        merge_map.insert((left_id, right_id), (rank as u32, merged_id));
    }
    Ok(merge_map)
}

fn parse_merge_entry(entry: &Value) -> Result<(&str, &str)> {
    match entry {
        Value::String(s) => {
            let (left, right) = s
                .split_once(' ')
                .ok_or_else(|| format!("invalid merge entry (no space): {s:?}"))?;
            Ok((left, right))
        }
        Value::Array(arr) if arr.len() == 2 => {
            let left = arr[0]
                .as_str()
                .ok_or_else(|| format!("merge element not a string: {:?}", arr[0]))?;
            let right = arr[1]
                .as_str()
                .ok_or_else(|| format!("merge element not a string: {:?}", arr[1]))?;
            Ok((left, right))
        }
        _ => Err(format!("unrecognized merge entry format: {entry:?}")),
    }
}

fn initial_token_byte_map(byte_to_initial_token: &[TokenId; 256], vocab_size: usize) -> Vec<u16> {
    let mut initial_token_byte = vec![u16::MAX; vocab_size];
    for (byte, &token) in byte_to_initial_token.iter().enumerate() {
        if token != INVALID_TOKEN {
            initial_token_byte[token as usize] = byte as u16;
        }
    }
    initial_token_byte
}

/// Rebuild the 65,536-entry byte-pair table from a persisted ranked merge map.
fn byte_pair_initial_from_ranked(
    ranked_keys: &[u64],
    ranked_values: &[u64],
    initial_token_byte: &[u16],
) -> Vec<(u32, u32)> {
    let mut table = vec![(u32::MAX, 0); 65536];
    for (&key, &payload) in ranked_keys.iter().zip(ranked_values) {
        if key == EMPTY_KEY {
            continue;
        }
        let left = (key >> 32) as u32;
        let right = key as u32;
        let left_byte = initial_token_byte[left as usize];
        let right_byte = initial_token_byte[right as usize];
        if left_byte != u16::MAX && right_byte != u16::MAX {
            table[left_byte as usize * 256 + right_byte as usize] =
                ((payload >> 32) as u32, payload as u32);
        }
    }
    table
}

/// Rebuild dense merge shortcuts from ranked slots instead of storing megabyte tables.
fn dense_tables_from_ranked(
    ranked_keys: &[u64],
    ranked_values: &[u64],
    byte_to_initial_token: &[TokenId; 256],
    byte_fallback: bool,
    ranked_merges: bool,
) -> (u32, Vec<u32>, Vec<u64>) {
    let (dense_ranked_bits, dense_ranked_merge) = if ranked_merges
        && !byte_fallback
        && byte_to_initial_token
            .iter()
            .all(|&id| id < MAX_DENSE_RANKED_LIMIT)
    {
        let max_initial = byte_to_initial_token.iter().copied().max().unwrap();
        let bits = (u32::BITS - max_initial.leading_zeros()).max(1);
        let limit = 1 << bits;
        let mut table = vec![u32::MAX; (limit * limit) as usize];
        for (&key, &payload) in ranked_keys.iter().zip(ranked_values) {
            if key == EMPTY_KEY {
                continue;
            }
            let left = (key >> 32) as u32;
            let right = key as u32;
            if left < limit && right < limit {
                table[(left << bits | right) as usize] = payload as u32;
            }
        }
        (bits, table)
    } else {
        (0, Vec::new())
    };
    let dense_merge = if dense_ranked_merge.is_empty()
        && !byte_fallback
        && byte_to_initial_token
            .iter()
            .all(|&id| id < DENSE_MERGE_LIMIT)
    {
        let mut table = vec![u64::MAX; DENSE_MERGE_SIZE];
        for (&key, &payload) in ranked_keys.iter().zip(ranked_values) {
            if key == EMPTY_KEY {
                continue;
            }
            let left = (key >> 32) as u32;
            let right = key as u32;
            if left < DENSE_MERGE_LIMIT && right < DENSE_MERGE_LIMIT {
                table[(left << DENSE_MERGE_BITS | right) as usize] = payload;
            }
        }
        table
    } else {
        Vec::new()
    };
    (dense_ranked_bits, dense_ranked_merge, dense_merge)
}

fn build_byte_pair_initial(
    merge_map: &ParsedMergeMap,
    initial_token_byte: &[u16],
) -> Vec<(u32, u32)> {
    let mut table = vec![(u32::MAX, 0); 65536];
    for (&(left, right), &(rank, merged)) in merge_map {
        let left_byte = initial_token_byte[left as usize];
        let right_byte = initial_token_byte[right as usize];
        if left_byte != u16::MAX && right_byte != u16::MAX {
            table[left_byte as usize * 256 + right_byte as usize] = (rank, merged);
        }
    }
    table
}

impl Bpe {
    /// Return bridge pairs when BPE merge resolution determines every output.
    pub fn bigram_bridge_table(&self) -> Option<&BigramBridgeTable> {
        // `ignore_merges` permits direct vocabulary matches unconstrained by merge reachability.
        (!self.ignore_merges).then_some(&self.bigram_bridge_table)
    }

    /// Build a BPE model whose output is determined by its merge graph.
    pub fn new(vocab: &Vocab, merge_map: ParsedMergeMap) -> Result<Self> {
        if vocab.is_empty() {
            return Err("cannot build Bpe with empty vocabulary".into());
        }
        let vocab_size = vocab.len();
        if merge_map.iter().any(|(&(left, right), &(_, merged))| {
            left as usize >= vocab_size
                || right as usize >= vocab_size
                || merged as usize >= vocab_size
        }) {
            return Err("merge token id exceeds vocabulary size".into());
        }
        Self::build(vocab.clone(), merge_map, false, false)
    }

    fn build(
        vocab: Vocab,
        merge_map: ParsedMergeMap,
        byte_fallback: bool,
        ignore_merges: bool,
    ) -> Result<Self> {
        if vocab.is_empty() {
            return Err("cannot build Bpe with empty vocabulary".into());
        }

        let id_to_token = {
            // IDs must be a permutation of `0..vocab.len()` before unchecked encode-time indexing.
            let mut ordered_tokens = vec![None; vocab.len()];
            for (text, &token) in &vocab {
                let slot = ordered_tokens.get_mut(token as usize).ok_or_else(|| {
                    format!("non-contiguous tokens - token id {token} exceeds vocabulary size")
                })?;
                if slot.replace(text.as_str()).is_some() {
                    return Err(format!("duplicate token id {token}"));
                }
            }
            ordered_tokens
                .into_iter()
                .enumerate()
                .map(|(token, text)| {
                    text.ok_or_else(|| format!("non-contiguous tokens - token {token} is missing"))
                        .map(str::to_owned)
                })
                .collect::<std::result::Result<Vec<_>, _>>()?
        };
        let max_token =
            u32::try_from(id_to_token.len() - 1).map_err(|_| "vocabulary exceeds u32 token IDs")?;
        let vocab_size = id_to_token.len();
        let mut byte_to_initial_token = [INVALID_TOKEN; 256];
        for byte_val in 0u16..256 {
            let ch = BYTE_TO_CHAR[byte_val as usize];
            let mut buf = [0u8; 4];
            let s = ch.encode_utf8(&mut buf);
            if let Some(&id) = vocab.get(s) {
                byte_to_initial_token[byte_val as usize] = id;
            }
        }
        let initial_token_byte = initial_token_byte_map(&byte_to_initial_token, vocab_size);
        let byte_pair_initial = build_byte_pair_initial(&merge_map, &initial_token_byte);

        let merge_adj = MergeAdjacency::from_parsed(&merge_map, vocab_size);
        let merge_result_ids = merge_adj.result_ids_by_rank();

        let mut bmp_char_token = vec![INVALID_TOKEN; 0x10000].into_boxed_slice();
        for (id, token) in id_to_token.iter().enumerate() {
            let mut chars = token.chars();
            if let (Some(ch), None) = (chars.next(), chars.next())
                && (ch as u32) < 0x10000
            {
                bmp_char_token[ch as usize] = id as TokenId;
            }
        }

        let (unmerge_map, mut is_orphan) = {
            let mut unmerge_map = (0..=max_token).map(|t| (t, t)).collect::<Vec<_>>();
            let mut is_orphan = vec![false; (max_token + 1) as usize];
            for (tid, text) in id_to_token.iter().enumerate() {
                if text.chars().nth(1).is_none() {
                    continue;
                }
                let token = tid as TokenId;
                match encoding_decomposition(
                    text,
                    &vocab,
                    &initial_token_byte,
                    &byte_pair_initial,
                    &merge_adj,
                    &bmp_char_token,
                ) {
                    // A whole-spelling shortcut is valid only if the final merge produces this exact token ID.
                    Decomposition::Pair(left, right, merged) if merged == token => {
                        unmerge_map[tid] = (left, right);
                    }
                    Decomposition::Pair(..) | Decomposition::Stuck => {
                        is_orphan[tid] = true;
                    }
                    Decomposition::CharsNotInVocab => {}
                }
            }
            (unmerge_map, is_orphan)
        };
        if byte_fallback && !ignore_merges {
            for (id, text) in id_to_token.iter().enumerate() {
                let token = id as TokenId;
                if text.chars().nth(1).is_some() && unmerge_map[id] == (token, token) {
                    // Fallback initials can be absent from the character vocabulary; identity alone is not a merge proof.
                    is_orphan[id] = true;
                }
            }
        }
        if ignore_merges {
            // Exact whole-piece lookup must include tokens the merge graph cannot construct.
            is_orphan.fill(false);
        }

        let ranked_merge_map = RankedMergeMap::from_parsed(&merge_map);

        let mut byte_fallback_token_ids = [INVALID_TOKEN; 256];
        if byte_fallback {
            for byte_val in 0u16..256 {
                let token = format!("<0x{byte_val:02X}>");
                if let Some(&id) = vocab.get(token.as_str()) {
                    byte_fallback_token_ids[byte_val as usize] = id;
                }
            }
        }

        let token_lens = id_to_token
            .iter()
            .enumerate()
            .map(|(token, text)| {
                u16::try_from(text.len())
                    .map(|len| len.min(u8::MAX as u16) as u8)
                    .map_err(|_| format!("token {token} length {} exceeds u16::MAX", text.len()))
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let mut single_char_token = [INVALID_TOKEN; 128];
        single_char_token.copy_from_slice(&bmp_char_token[..128]);

        let rank_offset = merge_map
            .values()
            .find_map(|&(rank, id)| (rank == 0).then_some(id));
        let ranked_merges = rank_offset.is_some_and(|offset| {
            merge_map
                .values()
                .all(|&(rank, id)| rank.checked_add(offset) == Some(id))
        });
        let (dense_ranked_bits, dense_ranked_merge) = if ranked_merges
            && !byte_fallback
            && byte_to_initial_token
                .iter()
                .all(|&id| id < MAX_DENSE_RANKED_LIMIT)
        {
            let max_initial = byte_to_initial_token.iter().copied().max().unwrap();
            let bits = (u32::BITS - max_initial.leading_zeros()).max(1);
            let limit = 1 << bits;
            let mut table = vec![u32::MAX; (limit * limit) as usize];
            for (&(left, right), &(_, id)) in &merge_map {
                if left < limit && right < limit {
                    let index = (left << bits | right) as usize;
                    table[index] = id;
                }
            }
            (bits, table)
        } else {
            (0, Vec::new())
        };
        let dense_merge = if dense_ranked_merge.is_empty()
            && !byte_fallback
            && byte_to_initial_token
                .iter()
                .all(|&id| id < DENSE_MERGE_LIMIT)
        {
            let mut table = vec![u64::MAX; DENSE_MERGE_SIZE];
            for (&(left, right), &(rank, id)) in &merge_map {
                if left < DENSE_MERGE_LIMIT && right < DENSE_MERGE_LIMIT {
                    let index = (left << DENSE_MERGE_BITS | right) as usize;
                    table[index] = (rank as u64) << 32 | id as u64;
                }
            }
            table
        } else {
            Vec::new()
        };
        let fused_cache_seeds = if byte_fallback {
            Vec::new()
        } else {
            let mut inverse = [u16::MAX; 324];
            for (byte, &ch) in BYTE_TO_CHAR.iter().enumerate() {
                inverse[ch as usize] = byte as u16;
            }
            id_to_token
                .iter()
                .enumerate()
                .filter_map(|(id, token)| {
                    if !ignore_merges && is_orphan[id] {
                        return None;
                    }
                    let mut bytes = [0u8; 15];
                    let mut len = 0;
                    for ch in token.chars() {
                        let byte = inverse.get(ch as usize).copied().unwrap_or(u16::MAX);
                        if byte == u16::MAX || len == bytes.len() {
                            return None;
                        }
                        bytes[len] = byte as u8;
                        len += 1;
                    }
                    let text = std::str::from_utf8(&bytes[..len]).ok()?;
                    pack_short_key(text).map(|key| (key, id as u32))
                })
                .collect()
        };

        let matcher = ExactTokenMatcher::Direct(is_orphan);

        let bigram_bridge_table = build_bigram_bridge_table(&id_to_token, byte_fallback);
        let packed_vocabulary = PackedVocabulary::from_strings(&id_to_token)?;
        let vocab_lookup = VocabLookup::from_arena(&packed_vocabulary)?;
        Ok(Self {
            id: next_bpe_id(),
            matcher,
            unmerge_map,
            token_lens,
            cross_thread_cache: CrossThreadCache::new(),
            fused_cross_thread_cache: CrossThreadCache::new(),
            packed_vocabulary,
            vocab_lookup,
            bmp_char_token,
            byte_to_initial_token,
            byte_fallback_token_ids,
            single_char_token,
            ranked_merge_map,
            byte_pair_initial,
            dense_merge,
            dense_ranked_merge,
            dense_ranked_bits,
            ranked_merges,
            fused_cache_seeds,
            merge_adj,
            merge_result_ids,
            ignore_merges,
            byte_fallback,
            bigram_bridge_table,
        })
    }

    /// Return whether a token boundary is compatible with the canonical merge graph.
    pub fn is_compatible_token_pair(&self, mut t1: TokenId, mut t2: TokenId) -> bool {
        if t1 == INVALID_TOKEN {
            return false;
        }

        let mut limit = u32::MAX;
        loop {
            if let Some((_rank, t)) = self.ranked_merge_map.get(t1, t2)
                && t < limit
            {
                return false;
            }

            if t1 > t2 {
                limit = t1;
                t1 = self.unmerge_map[t1 as usize].1;
                if t1 == limit {
                    limit = t2 + 1;
                    t2 = self.unmerge_map[t2 as usize].0;
                    if t2 + 1 == limit {
                        return true;
                    }
                }
            } else {
                limit = t2 + 1;
                t2 = self.unmerge_map[t2 as usize].0;
                if t2 + 1 == limit {
                    limit = t1;
                    t1 = self.unmerge_map[t1 as usize].1;
                    if t1 == limit {
                        return true;
                    }
                }
            }
        }
    }

    fn next_match(&self, input: &str) -> Option<TokenId> {
        self.matcher
            .next_match(input, &self.vocab_lookup, &self.packed_vocabulary)
    }

    fn token_length_matches(&self, token: TokenId, len: usize) -> bool {
        let compact = self.token_lens[token as usize];
        if compact == u8::MAX {
            self.packed_vocabulary.len_at(token as usize) == len
        } else {
            compact as usize == len
        }
    }

    #[inline(always)]
    pub(crate) fn append_bpe_ids(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        if input.is_empty() {
            return Ok(());
        }

        if let Some(token) = self.next_match(input)
            && self.token_length_matches(token, input.len())
        {
            out.push(token);
            return Ok(());
        }

        let bpe_id = self.id;
        let hit = TL_BPE_CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if c.bpe_id != bpe_id {
                return false;
            }
            c.get(input, out)
        });
        if hit {
            return Ok(());
        }

        let start = out.len();
        if self.cross_thread_cache.get_into(input, out) {
            TL_BPE_CACHE.with(|c| {
                let mut c = c.borrow_mut();
                if c.bpe_id != bpe_id {
                    c.bpe_id = bpe_id;
                    c.clear();
                }
                c.insert(input, &out[start..]);
            });
            return Ok(());
        }

        self.merge_all_encoded_into(input, out)?;

        let ids = &out[start..];
        TL_BPE_CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if c.bpe_id != bpe_id {
                c.bpe_id = bpe_id;
                c.clear();
            }
            c.insert(input, ids);
        });
        self.cross_thread_cache
            .insert(input.to_string(), ids.to_vec());

        Ok(())
    }

    pub(crate) fn validate_input(&self, input: &str) -> Result<()> {
        if input.is_empty()
            || self
                .next_match(input)
                .is_some_and(|token| self.token_length_matches(token, input.len()))
        {
            return Ok(());
        }
        self.for_each_initial_token(input, |_| {})
    }

    fn for_each_initial_token(&self, input: &str, mut emit: impl FnMut(TokenId)) -> Result<()> {
        for ch in input.chars() {
            let mut buf = [0u8; 4];
            let s = ch.encode_utf8(&mut buf);
            let found = if ch.is_ascii() {
                let id = self.single_char_token[ch as usize];
                (id != INVALID_TOKEN).then_some(id)
            } else if (ch as u32) < 0x10000 {
                let id = self.bmp_char_token[ch as usize];
                (id != INVALID_TOKEN).then_some(id)
            } else {
                self.vocab_lookup.get(&self.packed_vocabulary, s.as_bytes())
            };
            if let Some(id) = found {
                emit(id);
                continue;
            }

            if !self.byte_fallback {
                return Err(format!("character {ch:?} not in vocabulary"));
            }

            for &byte in s.as_bytes() {
                let id = self.byte_fallback_token_ids[byte as usize];
                if id == INVALID_TOKEN {
                    return Err(format!(
                        "byte fallback token <0x{byte:02X}> not in vocabulary"
                    ));
                }
                emit(id);
            }
        }
        Ok(())
    }

    fn merge_all_encoded_into(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        if input.is_empty() {
            return Ok(());
        }

        TL_ENCODED_MERGE_SCRATCH.with(|s| {
            let mut scratch = s.borrow_mut();
            scratch.symbols.clear();
            scratch.heap.clear();

            self.for_each_initial_token(input, |id| scratch.push_encoded_symbol(id))?;

            let n = scratch.symbols.len();
            if n == 1 {
                out.push(scratch.symbols[0].c);
                return Ok(());
            }

            self.init_merge_heap(&mut scratch, n);
            self.run_encoded_merge_loop(&mut scratch, out);
            Ok(())
        })
    }

    fn merge_all_raw_into(&self, raw_input: &str, out: &mut Vec<u32>) -> Result<()> {
        if raw_input.is_empty() {
            return Ok(());
        }
        if !self.byte_fallback && (2..=15).contains(&raw_input.len()) {
            return if self.ranked_merges {
                self.merge_short_raw_into::<16, true>(raw_input.as_bytes(), out)
            } else {
                self.merge_short_raw_into::<16, false>(raw_input.as_bytes(), out)
            };
        }
        if (!self.dense_merge.is_empty() || !self.dense_ranked_merge.is_empty())
            && (16..=31).contains(&raw_input.len())
        {
            return if self.ranked_merges {
                self.merge_short_raw_into::<32, true>(raw_input.as_bytes(), out)
            } else {
                self.merge_short_raw_into::<32, false>(raw_input.as_bytes(), out)
            };
        }
        TL_MERGE_SCRATCH.with(|s| {
            let mut scratch = s.borrow_mut();
            scratch.symbols.clear();
            scratch.heap.clear();
            scratch.heap_buf.clear();

            let bytes = raw_input.as_bytes();
            let n = bytes.len();
            let mut prev_byte = 0u8;
            for (i, &byte) in bytes.iter().enumerate() {
                let id = self.byte_to_initial_token[byte as usize];
                if id == INVALID_TOKEN {
                    return Err(format!("byte 0x{byte:02x} has no token in vocabulary"));
                }
                scratch.symbols.push(MergeSymbol {
                    c: id,
                    prev: if i == 0 { -1 } else { (i - 1) as i32 },
                    next: if i == n - 1 { -1 } else { (i + 1) as i32 },
                });
                if i > 0 {
                    let (rank, _new_id) =
                        self.byte_pair_initial[prev_byte as usize * 256 + byte as usize];
                    if rank != u32::MAX {
                        scratch.heap_buf.push(Reverse(MergeEntry::new(
                            rank,
                            (i - 1) as u32,
                            self.byte_to_initial_token[prev_byte as usize],
                            id,
                        )));
                    }
                }
                prev_byte = byte;
            }

            if n == 1 {
                out.push(scratch.symbols[0].c);
                return Ok(());
            }

            let mut tmp = std::mem::take(&mut scratch.heap_buf);
            scratch.heap.extend(tmp.drain(..));
            scratch.heap_buf = tmp;

            self.run_merge_loop(&mut scratch, out);

            Ok(())
        })
    }

    #[inline]
    fn merge_short_raw_into<const CAPACITY: usize, const RANKED: bool>(
        &self,
        bytes: &[u8],
        out: &mut Vec<u32>,
    ) -> Result<()> {
        let n = bytes.len();
        debug_assert!((2..CAPACITY).contains(&n));

        let mut symbols = [0u32; CAPACITY];
        let mut next = [0u8; CAPACITY];
        let mut prev = [0u8; CAPACITY];
        let mut ranks = [u32::MAX; CAPACITY];
        let mut merged = [0u32; CAPACITY];

        for (i, &byte) in bytes.iter().enumerate() {
            let id = self.byte_to_initial_token[byte as usize];
            if id == INVALID_TOKEN {
                return Err(format!("byte 0x{byte:02x} has no token in vocabulary"));
            }
            symbols[i] = id;
            next[i] = (i + 1) as u8;
            prev[i] = (i as u8).wrapping_sub(1);
            if i > 0 {
                let pair = self.byte_pair_initial[bytes[i - 1] as usize * 256 + byte as usize];
                ranks[i - 1] = if RANKED && pair.0 != u32::MAX {
                    pair.1
                } else {
                    pair.0
                };
                if !RANKED {
                    merged[i - 1] = pair.1;
                }
            }
        }

        loop {
            let mut best_rank = u32::MAX;
            let mut best = 0;
            for (i, &rank) in ranks[..n - 1].iter().enumerate() {
                if rank < best_rank {
                    best_rank = rank;
                    best = i;
                }
            }
            if best_rank == u32::MAX {
                break;
            }

            symbols[best] = if RANKED { best_rank } else { merged[best] };
            let dead = next[best] as usize;
            let right = next[dead] as usize;
            next[best] = right as u8;
            ranks[dead] = u32::MAX;

            if right < n {
                prev[right] = best as u8;
                if let Some((rank, id)) =
                    self.short_merge_lookup::<RANKED>(symbols[best], symbols[right])
                {
                    ranks[best] = if RANKED { id } else { rank };
                    if !RANKED {
                        merged[best] = id;
                    }
                } else {
                    ranks[best] = u32::MAX;
                }
            } else {
                ranks[best] = u32::MAX;
            }

            let left = prev[best] as usize;
            if left < n {
                if let Some((rank, id)) =
                    self.short_merge_lookup::<RANKED>(symbols[left], symbols[best])
                {
                    ranks[left] = if RANKED { id } else { rank };
                    if !RANKED {
                        merged[left] = id;
                    }
                } else {
                    ranks[left] = u32::MAX;
                }
            }
        }

        let mut i = 0;
        while i < n {
            out.push(symbols[i]);
            i = next[i] as usize;
        }
        Ok(())
    }

    #[inline(always)]
    fn short_merge_lookup<const RANKED: bool>(&self, left: u32, right: u32) -> Option<(u32, u32)> {
        if RANKED && !self.dense_ranked_merge.is_empty() {
            let limit = 1 << self.dense_ranked_bits;
            if left < limit && right < limit {
                let index = (left << self.dense_ranked_bits | right) as usize;
                // SAFETY: both endpoints are below limit, and construction allocates limit * limit entries.
                let id = unsafe { *self.dense_ranked_merge.get_unchecked(index) };
                return (id != u32::MAX).then_some((id, id));
            }
        }
        if !self.dense_merge.is_empty() && left < DENSE_MERGE_LIMIT && right < DENSE_MERGE_LIMIT {
            let index = (left << DENSE_MERGE_BITS | right) as usize;
            let packed = unsafe { *self.dense_merge.get_unchecked(index) };
            return (packed != u64::MAX).then_some(((packed >> 32) as u32, packed as u32));
        }
        self.merge_adj.get(left, right)
    }

    #[inline(always)]
    fn init_merge_heap(&self, scratch: &mut EncodedMergeScratch, n: usize) {
        let symbols = &scratch.symbols;
        scratch.heap.extend((0..n - 1).filter_map(|i| {
            let left = symbols[i].c;
            let right = symbols[i + 1].c;
            self.merge_adj
                .get(left, right)
                .map(|(rank, _new_id)| Reverse(MergeEntry::new(rank, i as u32, left, right)))
        }));
    }

    #[inline(always)]
    fn run_merge_loop(&self, scratch: &mut MergeScratch, out: &mut Vec<u32>) {
        run_merge_loop_body!(self, scratch, out);
    }

    #[inline(always)]
    fn run_encoded_merge_loop(&self, scratch: &mut EncodedMergeScratch, out: &mut Vec<u32>) {
        run_merge_loop_body!(self, scratch, out);
    }

    #[cfg(test)]
    #[inline(always)]
    fn append_raw_bpe_ids(&self, raw_input: &str, out: &mut Vec<u32>) -> Result<()> {
        if raw_input.is_empty() {
            return Ok(());
        }

        let bpe_id = self.id;
        let hit = TL_FUSED_CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if c.bpe_id != bpe_id {
                self.prepare_fused_cache(&mut c);
            }
            c.get(raw_input, out)
        });
        if hit {
            return Ok(());
        }

        let start = out.len();
        if self.fused_cross_thread_cache.get_into(raw_input, out) {
            TL_FUSED_CACHE.with(|c| {
                let mut c = c.borrow_mut();
                if c.bpe_id != bpe_id {
                    self.prepare_fused_cache(&mut c);
                }
                c.insert(raw_input, &out[start..]);
            });
            return Ok(());
        }

        if self.ignore_merges {
            let mut encoded = String::with_capacity(raw_input.len());
            for &byte in raw_input.as_bytes() {
                encoded.push(BYTE_TO_CHAR[byte as usize]);
            }
            if let Some(id) = self
                .vocab_lookup
                .get(&self.packed_vocabulary, encoded.as_bytes())
            {
                out.push(id);
                return Ok(());
            }
        }

        self.merge_all_raw_into(raw_input, out)?;

        let ids = &out[start..];
        TL_FUSED_CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if c.bpe_id != bpe_id {
                self.prepare_fused_cache(&mut c);
            }
            c.insert(raw_input, ids);
        });
        self.fused_cross_thread_cache
            .insert(raw_input.to_string(), ids.to_vec());

        Ok(())
    }

    pub(crate) fn append_scanned_bpe_ids(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        use_parallel_cache: bool,
        scan: impl FnOnce(&mut EncodeStream<'_>) -> std::result::Result<(), crate::Error>,
    ) -> std::result::Result<(), crate::Error> {
        out.reserve(input.len().saturating_add(3));

        if use_parallel_cache {
            return TL_FUSED_PARALLEL_CACHE
                .with(|cache| self.append_scanned_bpe_ids_with_cache(out, cache, scan));
        }

        TL_FUSED_CACHE.with(|cache| self.append_scanned_bpe_ids_with_cache(out, cache, scan))
    }

    fn append_scanned_bpe_ids_with_cache(
        &self,
        out: &mut Vec<u32>,
        cache: &RefCell<FlatCache>,
        scan: impl FnOnce(&mut EncodeStream<'_>) -> std::result::Result<(), crate::Error>,
    ) -> std::result::Result<(), crate::Error> {
        let bpe_id = self.id;
        let mut cache = cache.borrow_mut();
        if cache.bpe_id != bpe_id {
            self.prepare_fused_cache(&mut cache);
        }

        let mut pending = MaybeUninit::<[MaybeUninit<FusedPiece>; FUSED_PIECE_CAPACITY]>::uninit();
        let pending = unsafe { &mut *pending.as_mut_ptr() };
        let mut stream = EncodeStream {
            model: self,
            cache: &mut cache,
            out,
            error: None,
            pending,
            pending_len: 0,
        };
        let result = scan(&mut stream);
        // Queued pieces own their bytes, even if a failed scan dropped its input buffer.
        if stream.pending_len != 0 {
            stream.flush();
        }
        // Pre-tokenization finishes before modeling in the generic path, so scanner errors take precedence.
        result?;
        stream
            .error
            .map_or(Ok(()), |error| Err(crate::Error::Model(error)))
    }

    #[inline(always)]
    fn append_cached_piece_bpe_ids(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_cross_thread_cache: bool,
    ) -> Result<()> {
        if range.is_empty() {
            return Ok(());
        }
        if cache.get_direct_packed(packed, out) {
            return Ok(());
        }

        self.append_piece_bpe_ids_cache_miss(
            cache,
            input,
            range,
            packed,
            out,
            use_cross_thread_cache,
        )
    }

    #[cold]
    #[inline(never)]
    fn append_piece_bpe_ids_cache_miss(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_cross_thread_cache: bool,
    ) -> Result<()> {
        if packed != EMPTY_SHORT_KEY && cache.get_packed_probed(packed, out) {
            return Ok(());
        }
        self.append_uncached_piece_bpe_ids(cache, input, range, packed, out, use_cross_thread_cache)
    }

    #[cold]
    #[inline(never)]
    fn append_uncached_piece_bpe_ids(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_cross_thread_cache: bool,
    ) -> Result<()> {
        let text = &input[range];
        if packed == EMPTY_SHORT_KEY && cache.get_piece(text, packed, out) {
            return Ok(());
        }

        let start = out.len();
        if use_cross_thread_cache && self.fused_cross_thread_cache.get_into(text, out) {
            cache.insert_piece(text, packed, &out[start..]);
            return Ok(());
        }

        if self.ignore_merges {
            let mut encoded = String::with_capacity(text.len());
            for &byte in text.as_bytes() {
                encoded.push(BYTE_TO_CHAR[byte as usize]);
            }
            if let Some(id) = self
                .vocab_lookup
                .get(&self.packed_vocabulary, encoded.as_bytes())
            {
                out.push(id);
                cache.insert_piece(text, packed, &out[start..]);
                return Ok(());
            }
        }

        self.merge_all_raw_into(text, out)?;
        cache.insert_piece(text, packed, &out[start..]);
        if use_cross_thread_cache {
            self.fused_cross_thread_cache
                .insert(text.to_string(), out[start..].to_vec());
        }
        Ok(())
    }

    fn prepare_fused_cache(&self, cache: &mut FlatCache) {
        cache.bpe_id = self.id;
        cache.clear();
        for &(key, id) in self.fused_cache_seeds.iter().rev() {
            cache.insert_direct(key, &[id]);
        }
        if cache.direct_cache.len() == PARALLEL_DIRECT_CACHE_SIZE {
            for &(key, id) in self.fused_cache_seeds.iter().rev() {
                let slot = &cache.direct_cache[cache.direct_index(key)];
                if slot.key != [key as u64, (key >> 64) as u64] {
                    cache.insert_packed_probed(key, &[id]);
                }
            }
        } else {
            for &(key, id) in self.fused_cache_seeds.iter().rev() {
                if id as usize >= FUSED_CACHE_PROBED_SEED_LIMIT {
                    continue;
                }
                cache.insert_packed_probed(key, &[id]);
            }
        }
    }

    pub(crate) fn append_split_bpe_ids(
        &self,
        buffer: &str,
        splits: &[crate::pre_tokenized::Split],
        out: &mut Vec<u32>,
    ) -> Result<()> {
        out.reserve(buffer.len().saturating_add(splits.len()).saturating_add(3));
        let bpe_id = self.id;
        TL_FUSED_CACHE.with(|c| {
            let mut cache = c.borrow_mut();
            if cache.bpe_id != bpe_id {
                self.prepare_fused_cache(&mut cache);
            }

            for split in splits {
                if let Some(id) = split.token_id {
                    out.push(id);
                } else if !split.range.is_empty() {
                    let packed = pack_short_range(buffer, split.range.start, split.range.end);
                    self.append_cached_piece_bpe_ids(
                        &mut cache,
                        buffer,
                        split.range.clone(),
                        packed,
                        out,
                        true,
                    )?;
                }
            }
            Ok(())
        })
    }

    /// Returns the vocabulary text for an ID.
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.packed_vocabulary.get(id)
    }

    /// Returns the vocabulary ID for exact token text.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.vocab_lookup
            .get(&self.packed_vocabulary, token.as_bytes())
    }

    /// Returns the number of entries in the model vocabulary.
    pub fn vocab_size(&self) -> usize {
        self.packed_vocabulary.len()
    }
}

impl Clone for Bpe {
    fn clone(&self) -> Self {
        Self {
            id: next_bpe_id(),
            matcher: self.matcher.clone(),
            unmerge_map: self.unmerge_map.clone(),
            token_lens: self.token_lens.clone(),
            cross_thread_cache: CrossThreadCache::new(),
            fused_cross_thread_cache: CrossThreadCache::new(),
            packed_vocabulary: self.packed_vocabulary.clone(),
            vocab_lookup: self.vocab_lookup.clone(),
            bmp_char_token: self.bmp_char_token.clone(),
            byte_to_initial_token: self.byte_to_initial_token,
            byte_fallback_token_ids: self.byte_fallback_token_ids,
            single_char_token: self.single_char_token,
            ranked_merge_map: self.ranked_merge_map.clone(),
            byte_pair_initial: self.byte_pair_initial.clone(),
            dense_merge: self.dense_merge.clone(),
            dense_ranked_merge: self.dense_ranked_merge.clone(),
            dense_ranked_bits: self.dense_ranked_bits,
            ranked_merges: self.ranked_merges,
            fused_cache_seeds: self.fused_cache_seeds.clone(),
            merge_adj: self.merge_adj.clone(),
            merge_result_ids: self.merge_result_ids.clone(),
            ignore_merges: self.ignore_merges,
            byte_fallback: self.byte_fallback,
            bigram_bridge_table: self.bigram_bridge_table.clone(),
        }
    }
}

impl fmt::Debug for Bpe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bpe")
            .field("vocab_size", &self.token_lens.len())
            .field("merges", &self.ranked_merge_map.len())
            .finish()
    }
}

impl PartialEq for Bpe {
    fn eq(&self, other: &Self) -> bool {
        self.matcher == other.matcher
            && self.ranked_merge_map == other.ranked_merge_map
            && self.unmerge_map == other.unmerge_map
            // Equal compact-length sentinels can hide different original token lengths.
            && (0..self.packed_vocabulary.len())
                .map(|id| self.packed_vocabulary.len_at(id))
                .eq((0..other.packed_vocabulary.len()).map(|id| other.packed_vocabulary.len_at(id)))
            && self.ignore_merges == other.ignore_merges
            && self.byte_fallback == other.byte_fallback
    }
}

mod encode;
mod snapshot;

pub(crate) use snapshot::NativeBpeTables;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_structs::ModelConfig;
    use crate::{Tokenizer, TruncationDirection};
    use serde_json::json;

    fn ids(bpe: &Bpe, input: &str) -> Result<Vec<u32>> {
        let mut out = Vec::new();
        bpe.append_bpe_ids(input, &mut out)?;
        Ok(out)
    }

    #[test]
    fn vocab_lookup_probes_arena_slices_without_string_keys() {
        let tokens = ["a", "ab", "é", ""].map(String::from);
        let arena = PackedVocabulary::from_strings(&tokens).unwrap();
        let lookup = VocabLookup::from_arena(&arena).unwrap();
        assert_eq!(lookup.get(&arena, b"ab"), Some(1));
        assert_eq!(lookup.get(&arena, "é".as_bytes()), Some(2));
        assert_eq!(lookup.get(&arena, b""), Some(3));
        assert_eq!(lookup.get(&arena, b"missing"), None);

        let mut slots = Vec::new();
        let mut hashes = Vec::new();
        let mut ids = Vec::new();
        for (slot, (&hash, &id)) in lookup.hashes.iter().zip(&lookup.ids).enumerate() {
            if hash != EMPTY_VOCAB_HASH {
                slots.push(slot as u32);
                hashes.push(hash);
                ids.push(id);
            }
        }
        let restored = VocabLookup::from_cached_slots(
            &arena,
            lookup.hashes.len() as u32,
            &slots,
            &hashes,
            &ids,
        )
        .unwrap();
        assert_eq!(restored.get(&arena, b"a"), Some(0));
        assert!(
            VocabLookup::from_arena(
                &PackedVocabulary::from_strings(&["dup".into(), "dup".into()]).unwrap()
            )
            .is_err()
        );
        let mut empty_hashes = hashes.clone();
        empty_hashes[0] = EMPTY_VOCAB_HASH;
        assert!(
            VocabLookup::from_cached_slots(
                &arena,
                lookup.hashes.len() as u32,
                &slots,
                &empty_hashes,
                &ids,
            )
            .is_err()
        );
    }

    #[test]
    fn heap_merges_preserve_stale_neighbors_rank_holes_and_shared_results() {
        let model = json!({
            "type": "BPE",
            "vocab": {"a":0,"b":1,"c":2,"d":3,"ab":4,"bc":5,"abc":6,"cd":7,"abcd":8,"bd":9,"aa":10},
            "merges": [["a","b"],["b","c"],["a","b"],["ab","c"],["a","bc"],["c","d"],["ab","cd"],["abc","d"],["b","d"],["a","a"]]
        });
        let original: Bpe = serde_json::from_value(model.clone()).unwrap();
        let restored = NativeBpeTables::from_model(&original)
            .unwrap()
            .into_model()
            .unwrap();
        let reference = tokenizers::Tokenizer::from_bytes(
            serde_json::to_vec(&json!({"model": model})).unwrap(),
        )
        .unwrap();
        for bpe in [&original, &original.clone(), &restored] {
            for len in [0, 1, 2, 3, 15, 16, 31, 32, 33, 127, 1024] {
                for seed in 0..16u64 {
                    let mut state = seed + 1;
                    let input: String = (0..len)
                        .map(|_| {
                            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                            b"abcd"[(state >> 32) as usize % 4] as char
                        })
                        .collect();
                    let expected = reference.encode(input.as_str(), false).unwrap();
                    let mut encoded = Vec::new();
                    bpe.merge_all_encoded_into(&input, &mut encoded).unwrap();
                    assert_eq!(encoded, expected.get_ids(), "encoded len={len} seed={seed}");
                    let mut raw = Vec::new();
                    bpe.merge_all_raw_into(&input, &mut raw).unwrap();
                    assert_eq!(raw, expected.get_ids(), "raw len={len} seed={seed}");
                }
            }
        }
    }

    #[test]
    fn rank_results_allow_shared_ids_and_fall_back_for_sparse_or_ambiguous_ranks() {
        let make = |pairs: &[(u32, u32)]| MergeAdjacency {
            offsets: vec![],
            keys: pairs.iter().map(|&(rank, _)| rank as u64).collect(),
            new_ids: pairs.iter().map(|&(_, id)| id).collect(),
        };
        assert_eq!(
            make(&[(0, 8), (2, 8), (3, 9)]).result_ids_by_rank(),
            [8, INVALID_TOKEN, 8, 9]
        );
        assert!(make(&[(0, 8), (0, 9)]).result_ids_by_rank().is_empty());
        assert!(make(&[(100, 8)]).result_ids_by_rank().is_empty());
        assert!(make(&[(u32::MAX, 8)]).result_ids_by_rank().is_empty());
    }

    #[test]
    fn empty_input() {
        let bpe = test_bpe();
        assert_eq!(ids(&bpe, "").unwrap(), Vec::<u32>::new());
        let mut raw = Vec::new();
        bpe.append_raw_bpe_ids("", &mut raw).unwrap();
        assert!(raw.is_empty());
    }

    #[test]
    fn packed_bridge_table_preserves_every_pair() {
        let vocabulary = vec!["aé中".into(), "<0xFF><0x00>z".into()];
        for fallback in [false, true] {
            let table = build_bigram_bridge_table(&vocabulary, fallback);
            assert_eq!(std::mem::size_of_val(&*table.bridgeable), 8192);
            let mut expected = [false; 65536];
            for token in &vocabulary {
                for pair in token.as_bytes().windows(2) {
                    expected[pair[0] as usize * 256 + pair[1] as usize] = true;
                }
            }
            if fallback {
                expected[255 * 256] = true;
                expected[b'z' as usize] = true;
            }
            for prev in 0u16..256 {
                for cur in 0u16..256 {
                    assert_eq!(
                        table.is_bridgeable(prev as u8, cur as u8),
                        expected[prev as usize * 256 + cur as usize],
                    );
                }
            }
        }
        let mut table = BigramBridgeTable {
            bridgeable: Box::new([0; 1024]),
        };
        for pair in (0..65536).step_by(2) {
            table.insert((pair / 256) as u8, pair as u8);
        }
        for pair in 0..65536 {
            assert_eq!(
                table.is_bridgeable((pair / 256) as u8, pair as u8),
                pair % 2 == 0
            );
        }
    }

    #[test]
    fn compact_token_lengths_preserve_long_matches() {
        let lengths = [1usize, 254, 255, 256, u16::MAX as usize];
        let vocab: Vocab = lengths
            .iter()
            .enumerate()
            .map(|(id, &len)| ("a".repeat(len), id as u32))
            .collect();
        let bpe = Bpe::build(vocab, HashMap::new(), false, true).unwrap();
        assert_eq!(bpe.token_lens, [1, 254, 255, 255, 255]);
        for (id, &len) in lengths.iter().enumerate() {
            assert!(bpe.token_length_matches(id as u32, len));
            assert!(!bpe.token_length_matches(id as u32, len - 1));
            assert!(!bpe.token_length_matches(id as u32, len + 1));
            assert_eq!(ids(&bpe, &"a".repeat(len)).unwrap(), [id as u32]);
        }
        let oversized = HashMap::from([("a".repeat(65536), 0)]);
        assert!(
            Bpe::build(oversized, HashMap::new(), false, true)
                .unwrap_err()
                .contains("exceeds u16::MAX")
        );
    }

    pub(super) fn test_bpe() -> Bpe {
        let vocab: Vocab = [
            ("a", 0),
            ("b", 1),
            ("c", 2),
            ("d", 3),
            ("ab", 4),
            ("cd", 5),
            ("abcd", 6),
        ]
        .into_iter()
        .map(|(s, id)| (s.to_string(), id))
        .collect();

        let merges: Vec<Value> = vec![
            Value::String("a b".into()),
            Value::String("c d".into()),
            Value::String("ab cd".into()),
        ];

        let merge_map = parse_merges(&vocab, &merges).unwrap();
        Bpe::new(&vocab, merge_map).unwrap()
    }

    #[test]
    fn mismatched_final_merge_is_not_an_exact_token() {
        let vocab: Vocab = [("a", 0), ("b", 1), ("ab", 2)]
            .into_iter()
            .map(|(text, token)| (text.to_string(), token))
            .collect();
        let merge_map = HashMap::from([((0, 1), (0, 0))]);
        let bpe = Bpe::build(vocab, merge_map, false, false).unwrap();

        assert_eq!(bpe.next_match("ab"), None);
    }

    #[test]
    fn cache_preserves_direct_and_long_map_values() {
        let mut cache = FlatCache::new();
        let short = "four-token-hit";
        let short_ids = [1, 2, 3, 4];
        let packed = pack_short_key(short).unwrap();
        cache.insert(short, &short_ids);
        let direct_index = cache.direct_index(packed);
        cache.direct_cache[direct_index] = DirectCacheSlot::default();

        let mut out = Vec::new();
        assert!(cache.get(short, &mut out));
        assert_eq!(out, short_ids);
        out.clear();
        assert!(cache.get(short, &mut out));
        assert_eq!(out, short_ids);

        let long_piece = "a fused cache key longer than fifteen bytes";
        let long_ids = [5, 6, 7];
        cache.insert(long_piece, &long_ids);
        out.clear();
        assert!(cache.get(long_piece, &mut out));
        assert_eq!(out, long_ids);

        cache.clear();
        out.clear();
        assert!(!cache.get(short, &mut out));
        assert!(!cache.get(long_piece, &mut out));
    }

    #[test]
    fn probed_cache_packed_values_preserve_boundaries() {
        let mut cache = FlatCache::new();
        let key = pack_short_key("dtype-boundary").unwrap();
        let values = [
            vec![],
            vec![0],
            vec![(1 << 31) - 1, (1 << 31) - 1],
            vec![1 << 31],
            vec![0, u32::MAX],
            vec![u32::MAX, 0],
            vec![0, 1, u32::MAX],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3, 4],
            vec![u32::MAX; u16::MAX as usize],
            vec![u32::MAX],
        ];
        for ids in values {
            assert!(cache.insert_packed_probed(key, &ids));
            let mut out = Vec::with_capacity(4);
            out.push(123);
            assert!(cache.get_packed_probed(key, &mut out));
            assert_eq!(&out[1..], ids);
            assert_eq!(cache.count, 1);
        }
        assert!(!cache.insert_packed_probed(key, &vec![0; u16::MAX as usize + 1]));
        cache.clear_probed();
        let mut out = Vec::with_capacity(4);
        assert!(!cache.get_packed_probed(key, &mut out));
        assert_eq!(cache.count, 0);
        assert!(cache.pool.is_empty());
    }

    #[test]
    fn probed_cache_packed_keys_resolve_collisions() {
        for high in [false, true] {
            let mut homes = HashMap::new();
            let (first, second) = (1..=PROBED_CACHE_SIZE + 1)
                .find_map(|value| {
                    let key = if high {
                        123 | (value as u128) << 64
                    } else {
                        value as u128 | 123u128 << 64
                    };
                    homes
                        .insert(probed_cache_index(key), key)
                        .map(|old| (old, key))
                })
                .unwrap();
            let mut cache = FlatCache::new();
            assert!(cache.insert_packed_probed(first, &[7, 8]));
            assert!(cache.insert_packed_probed(second, &[u32::MAX]));
            for (key, expected) in [(first, vec![7, 8]), (second, vec![u32::MAX])] {
                let mut out = Vec::with_capacity(4);
                assert!(cache.get_packed_probed(key, &mut out));
                assert_eq!(out, expected);
            }
            assert_eq!(cache.count, 2);
        }
    }

    #[test]
    fn rejects_out_of_range_merge_token_ids() {
        let vocab: Vocab = [("a".to_string(), 0), ("b".to_string(), 1)]
            .into_iter()
            .collect();

        for (left, right, merged) in [(2, 1, 0), (0, 2, 0), (0, 1, 2)] {
            let merge_map = HashMap::from([((left, right), (0, merged))]);
            assert_eq!(
                Bpe::new(&vocab, merge_map).unwrap_err(),
                "merge token id exceeds vocabulary size"
            );
        }
    }

    #[test]
    fn merges_preserve_rank_order_and_cached_results() {
        let bpe = test_bpe();
        for (text, expected) in [
            ("", vec![]),
            ("a", vec![0]),
            ("d", vec![3]),
            ("ab", vec![4]),
            ("cd", vec![5]),
            ("abcd", vec![6]),
            ("abc", vec![4, 2]),
            ("abab", vec![4, 4]),
        ] {
            for _ in 0..2 {
                assert_eq!(ids(&bpe, text).unwrap(), expected, "{text:?}");
            }
        }
    }

    #[test]
    fn string_and_array_merge_formats_encode_identically() {
        for merges in [serde_json::json!(["a b"]), serde_json::json!([["a", "b"]])] {
            let config: ModelConfig = serde_json::from_value(serde_json::json!({
                "type":"BPE", "vocab":{"a":0,"b":1,"ab":2}, "merges":merges,
            }))
            .unwrap();
            let ModelConfig::Bpe(bpe) = config else {
                panic!("expected a BPE model");
            };
            assert_eq!(ids(&bpe, "ab").unwrap(), [2]);
        }
    }

    #[test]
    fn byte_hash_keeps_little_endian_words_and_byte_tail() {
        let mix = |state: u64, word: u64| state.wrapping_add(word).wrapping_mul(0x517cc1b727220a95);
        let first = mix(17, 0x6867_6665_6463_6261);
        let second = mix(first, 0x706f_6e6d_6c6b_6a69);
        assert_eq!(fx_hash_bytes(b"", 17), 17);
        assert_eq!(fx_hash_bytes(b"abc", 17), mix(mix(mix(17, 97), 98), 99));
        assert_eq!(fx_hash_bytes(b"abcdefgh", 17), first);
        assert_eq!(fx_hash_bytes(b"abcdefghi", 17), mix(first, b'i' as u64));
        assert_eq!(fx_hash_bytes(b"abcdefghijklmnop", 17), second);
        assert_eq!(
            fx_hash_bytes(b"abcdefghijklmnopq", 17),
            mix(second, b'q' as u64)
        );
        let bytes: Vec<_> = (0..136).map(|i| (i * 131 + 19) as u8).collect();
        for start in 0..8 {
            for len in 0..=128 {
                let input = &bytes[start..start + len];
                for initial in [0, 17, u64::MAX, 0x0123_4567_89ab_cdef] {
                    let mut expected = initial;
                    let (words, tail) = input.as_chunks::<8>();
                    for &word in words {
                        expected = mix(expected, u64::from_le_bytes(word));
                    }
                    for &byte in tail {
                        expected = mix(expected, byte as u64);
                    }
                    assert_eq!(fx_hash_bytes(input, initial), expected);
                }
            }
        }
    }

    #[test]
    fn merge_entry_equality_matches_heap_priority() {
        let first = MergeEntry::new(7, 3, 1, 2);
        let stale = MergeEntry::new(7, 3, 8, 9);
        let later = MergeEntry::new(7, 4, 1, 2);
        assert!(first == stale);
        assert_eq!(first.cmp(&stale), std::cmp::Ordering::Equal);
        assert!(first < later);
        assert!(first != later);
    }

    #[test]
    fn byte_fallback_merge_crosses_unicode_boundary() {
        let tokenizer = Tokenizer::from_json(json!({
            "normalizer": null,
            "pre_tokenizer": null,
            "model": {
                "type": "BPE",
                "vocab": {
                    "<unk>": 0,
                    "<0xC3>": 1,
                    "<0xA9>": 2,
                    "<0xAA>": 3,
                    "<0xA9><0xC3>": 4
                },
                "merges": [["<0xA9>", "<0xC3>"]],
                "unk_token": "<unk>",
                "byte_fallback": true
            },
            "post_processor": null,
            "decoder": null
        }))
        .unwrap();

        assert_eq!(tokenizer.encode("éê", false).unwrap(), vec![1, 4, 3]);
        for limit in 0..=4 {
            let full = [1, 4, 3];
            assert_eq!(
                tokenizer
                    .encode_with_limit("éê", limit, TruncationDirection::Right)
                    .unwrap(),
                (full[..limit.min(3)].to_vec(), limit < 3)
            );
            assert_eq!(
                tokenizer
                    .encode_with_limit("éê", limit, TruncationDirection::Left)
                    .unwrap(),
                (full[3usize.saturating_sub(limit)..].to_vec(), limit < 3)
            );
        }
    }

    #[test]
    fn ignore_merges_preserves_piece_semantics() {
        let tokenizer = |ignore_merges| {
            Tokenizer::from_json(json!({
                "normalizer": null,
                "pre_tokenizer": null,
                "model": {
                    "type": "BPE",
                    "vocab": {"a": 0, "b": 1, "c": 2, "d": 3, "ab": 4, "abc": 5},
                    "merges": [],
                    "ignore_merges": ignore_merges
                },
                "post_processor": null,
                "decoder": null
            }))
            .unwrap()
        };

        let merged = tokenizer(false);
        assert_eq!(merged.encode("ab", false).unwrap(), vec![0, 1]);

        let ignored = tokenizer(true);
        assert_eq!(ignored.encode("abc", false).unwrap(), vec![5]);
        assert_eq!(ignored.encode("abd", false).unwrap(), vec![0, 1, 3]);
    }
}
