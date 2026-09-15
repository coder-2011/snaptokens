use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt,
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
const FUSED_CACHE_BACKING_SEED_LIMIT: usize = 160 * 1024;

const EMPTY_KEY: u64 = u64::MAX;

/// Hint the kernel to back this memory with transparent huge pages (Linux only).
/// This reduces TLB misses for large cache tables. No-op on non-Linux platforms
/// or when the `huge-pages` feature is not enabled.
#[inline(always)]
fn advise_huge_pages<T>(_vec: &Vec<T>) {
    #[cfg(all(target_os = "linux", feature = "huge-pages"))]
    {
        let ptr = _vec.as_ptr();
        let len = _vec.len() * std::mem::size_of::<T>();
        // Round down to page boundary and round up length.
        const PAGE_SIZE: usize = 4096;
        let aligned_ptr = (ptr as usize & !(PAGE_SIZE - 1)) as *mut libc::c_void;
        let aligned_len = (len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // SAFETY: We're advising on memory we own, and madvise is safe to call
        // even if it fails (the kernel will just ignore the hint).
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
    /// One bit per byte pair, set when a vocabulary token can cover that pair.
    bridgeable: Box<[u64; 1024]>,
}

impl BigramBridgeTable {
    /// Check if a byte pair can be bridged by some vocab token.
    #[inline(always)]
    pub fn is_bridgeable(&self, prev: u8, cur: u8) -> bool {
        let pair = prev as usize * 256 + cur as usize;
        self.bridgeable[pair / 64] & (1u64 << (pair % 64)) != 0
    }

    /// Record one exact byte-pair adjacency while constructing the bitset.
    fn insert(&mut self, prev: u8, cur: u8) {
        let pair = prev as usize * 256 + cur as usize;
        self.bridgeable[pair / 64] |= 1u64 << (pair % 64);
    }
}

/// Build a bigram bridge table by scanning all vocab tokens.
fn build_bigram_bridge_table(id_to_token: &[String], byte_fallback: bool) -> BigramBridgeTable {
    let mut table = BigramBridgeTable {
        bridgeable: Box::new([0; 1024]),
    };

    for token_str in id_to_token {
        let bytes = token_str.as_bytes();
        // Mark all adjacent byte pairs in this token as bridgeable
        for window in bytes.windows(2) {
            table.insert(window[0], window[1]);
        }

        // Fallback markers represent one input byte, so also record the
        // semantic adjacencies of merged fallback tokens.
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

/// Parse the byte represented by a leading `<0xHH>` fallback marker.
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

/// Mix bytes into an Fx-style hash state shared by both token caches.
#[inline(always)]
fn fx_hash_bytes(bytes: &[u8], mut state: u64) -> u64 {
    let mut i = 0;
    while i + 8 <= bytes.len() {
        let word = u64::from_ne_bytes(bytes[i..i + 8].try_into().unwrap());
        state = state.wrapping_add(word).wrapping_mul(0x517cc1b727220a95);
        i += 8;
    }
    while i < bytes.len() {
        state = state
            .wrapping_add(bytes[i] as u64)
            .wrapping_mul(0x517cc1b727220a95);
        i += 1;
    }
    state
}

/// FxHash-based [`BuildHasher`] for the token cache.
struct FxBuildHasher;

impl std::hash::BuildHasher for FxBuildHasher {
    type Hasher = FxStrHasher;
    fn build_hasher(&self) -> FxStrHasher {
        FxStrHasher(0)
    }
}

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

type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

const FLAT_CACHE_BITS: usize = 18;
const FLAT_CACHE_SIZE: usize = 1 << FLAT_CACHE_BITS;
const EMPTY_SHORT_KEY: u128 = 0;
// Generic BPE can initialize one cache per worker, so keep its direct table bounded.
const FRONT_CACHE_BITS: u32 = 17;
const FRONT_CACHE_SIZE: usize = 1 << FRONT_CACHE_BITS;
const PARALLEL_FRONT_CACHE_BITS: u32 = 19;
const PARALLEL_FRONT_CACHE_SIZE: usize = 1 << PARALLEL_FRONT_CACHE_BITS;
const FUSED_PIECE_BATCH: usize = 256;
const FUSED_PIECE_CAPACITY: usize = FUSED_PIECE_BATCH + 64;

/// Pack a short string into bytes plus a length tag for exact comparison.
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

/// Build the two native-word masks for a one-to-fifteen-byte packed key.
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

/// Pack a non-empty short range with one in-bounds wide load when lookahead permits it.
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

/// Pack a short range after its caller proves sixteen readable bytes at `start`.
#[inline(always)]
unsafe fn pack_short_range_inbounds(input: &str, start: usize, len: usize) -> u128 {
    debug_assert!((1..=15).contains(&len));
    debug_assert!(start + 16 <= input.len());
    #[cfg(target_arch = "aarch64")]
    unsafe {
        use core::arch::aarch64::*;

        // The lookahead guard makes the unaligned vector load in-bounds.
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

        // SSE2 is baseline on x86-64 and the lookahead guard covers the load.
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

/// Pack a short range whose start lacks sixteen bytes of forward lookahead.
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

/// Hash all bytes and the length tag of one packed short key.
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

/// Map a packed key to a direct hot-cache slot under its table mask.
#[inline(always)]
fn front_cache_index(key: u128, mask: usize) -> usize {
    packed_key_hash(key) as usize & mask
}

/// Map a packed key to its backing-cache home slot.
#[inline(always)]
fn flat_cache_index(key: u128) -> usize {
    packed_key_hash(key) as usize & (FLAT_CACHE_SIZE - 1)
}

/// One hot-cache entry with up to four token IDs stored inline.
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct FrontCacheSlot {
    key: [u64; 2],
    value: u64,
    extension: u64,
}

const _: () = assert!(std::mem::size_of::<FrontCacheSlot>() == 32);

/// Append a packed front-cache value after its caller reserves four lanes.
#[inline(always)]
fn append_front_value(out: &mut Vec<u32>, value: u64, extension: u64) {
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

/// Append an inline cache value after its caller reserves two output lanes.
#[inline(always)]
fn append_inline_value(out: &mut Vec<u32>, ids: &[u32; 2], len: usize) {
    debug_assert!(len <= 2 && out.capacity() - out.len() >= 2);
    let start = out.len();
    unsafe {
        std::ptr::copy_nonoverlapping(ids.as_ptr(), out.as_mut_ptr().add(start), 2);
        out.set_len(start + len);
    }
}

/// Exact short key plus either two compact IDs or a pooled output range.
#[derive(Clone, Copy)]
#[repr(C)]
struct CacheSlot {
    key: [u64; 2],
    value: u64,
}

const _: () = assert!(std::mem::size_of::<CacheSlot>() == 24);

/// Maximum load factor before the cache is cleared.
const FLAT_CACHE_MAX_LOAD: usize = FLAT_CACHE_SIZE * 3 / 4;
/// Maximum pool size in u32 entries before cache is cleared (64M entries = 256MB).
const FLAT_CACHE_MAX_POOL: usize = 64 * 1024 * 1024;

struct FlatCache {
    bpe_id: usize,
    front: Vec<FrontCacheSlot>,
    front_mask: usize,
    slots: Vec<CacheSlot>,
    pool: Vec<u32>,
    long: FxHashMap<Box<str>, (u32, u16)>,
    count: usize,
}

impl FlatCache {
    /// Allocate the larger direct cache used by sequential encoding.
    fn new() -> Self {
        Self::with_front_size(FRONT_CACHE_SIZE)
    }

    /// Allocate the compact direct cache used by parallel workers.
    fn new_parallel() -> Self {
        Self::with_front_size(PARALLEL_FRONT_CACHE_SIZE)
    }

    /// Allocate one cache with a selected power-of-two direct table.
    fn with_front_size(front_size: usize) -> Self {
        debug_assert!(front_size.is_power_of_two());
        let front = vec![FrontCacheSlot::default(); front_size];
        let slots = vec![
            CacheSlot {
                key: [0; 2],
                value: 0,
            };
            FLAT_CACHE_SIZE
        ];
        // Hint kernel to use transparent huge pages for these large allocations.
        advise_huge_pages(&front);
        advise_huge_pages(&slots);
        Self {
            bpe_id: 0,
            front,
            front_mask: front_size - 1,
            slots,
            pool: Vec::with_capacity(256 * 1024),
            long: HashMap::with_hasher(FxBuildHasher),
            count: 0,
        }
    }

    /// Map a packed key into this cache's direct table.
    #[inline(always)]
    fn front_index(&self, key: u128) -> usize {
        front_cache_index(key, self.front_mask)
    }

    fn clear(&mut self) {
        self.front.fill(FrontCacheSlot::default());
        self.clear_backing();
    }

    /// Reset the replaceable backing table while retaining inline hot entries.
    fn clear_backing(&mut self) {
        for slot in &mut self.slots {
            slot.key = [0; 2];
        }
        self.pool.clear();
        self.long.clear();
        self.count = 0;
    }

    /// Fetch a direct-cache line before probing it.
    #[inline(always)]
    fn prefetch_front(&self, slot: *const FrontCacheSlot) {
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

    /// Probe the packed table or its exact long-piece fallback.
    #[inline(always)]
    fn get_piece(&mut self, key: &str, packed: u128, out: &mut Vec<u32>) -> bool {
        if packed != EMPTY_SHORT_KEY {
            return self.get_packed(packed, out);
        }
        let Some(&(offset, len)) = self.long.get(key) else {
            return false;
        };
        let start = offset as usize;
        out.extend_from_slice(unsafe { self.pool.get_unchecked(start..start + len as usize) });
        true
    }

    /// Probe with a packed short key already derived by the scanner.
    #[inline(always)]
    fn get_packed(&mut self, packed: u128, out: &mut Vec<u32>) -> bool {
        self.get_front_packed(packed, out) || self.get_packed_backing(packed, out)
    }

    /// Probe only the direct cache used by the fused scanner's hot loop.
    #[inline(always)]
    fn get_front_packed(&self, packed: u128, out: &mut Vec<u32>) -> bool {
        let Some((value, extension)) = self.front_packed_value(packed) else {
            return false;
        };
        append_front_value(out, value, extension);
        true
    }

    /// Return one direct-cache value without updating an output vector.
    #[inline(always)]
    fn front_packed_value(&self, packed: u128) -> Option<(u64, u64)> {
        if packed == EMPTY_SHORT_KEY {
            return None;
        }
        let index = self.front_index(packed);
        let slot = unsafe { self.front.as_ptr().add(index) };
        let key = [packed as u64, (packed >> 64) as u64];
        let (value, extension, found) = self.front_packed_value_at(key, slot);
        found.then_some((value, extension))
    }

    /// Load one direct-cache slot and report whether its exact key matches.
    #[inline(always)]
    fn front_packed_value_at(
        &self,
        key: [u64; 2],
        slot: *const FrontCacheSlot,
    ) -> (u64, u64, bool) {
        debug_assert_ne!(key, [0; 2]);
        // SAFETY: callers derive the slot from this cache's fixed-size front table.
        let slot = unsafe { &*slot };
        (slot.value, slot.extension, slot.key == key)
    }

    /// Probe the replaceable table after a direct-cache miss.
    #[inline(always)]
    fn get_packed_backing(&mut self, packed: u128, out: &mut Vec<u32>) -> bool {
        let mut idx = flat_cache_index(packed);
        let key = [packed as u64, (packed >> 64) as u64];
        loop {
            let slot = unsafe { *self.slots.get_unchecked(idx) };
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
                    self.insert_front(packed, &out[output_start..]);
                }
                return true;
            }
            if slot.key == [0; 2] {
                return false;
            }
            idx = (idx + 1) & (FLAT_CACHE_SIZE - 1);
        }
    }

    #[inline(always)]
    fn insert(&mut self, key: &str, ids: &[u32]) {
        let packed = pack_short_key(key).unwrap_or(EMPTY_SHORT_KEY);
        self.insert_piece(key, packed, ids);
    }

    /// Insert into the packed table or its exact long-piece fallback.
    #[inline(always)]
    fn insert_piece(&mut self, key: &str, packed: u128, ids: &[u32]) {
        if packed != EMPTY_SHORT_KEY {
            self.insert_packed(packed, ids);
            return;
        }
        if self.pool.len() >= FLAT_CACHE_MAX_POOL {
            return;
        }
        let Ok(offset) = u32::try_from(self.pool.len()) else {
            return;
        };
        let Ok(len) = u16::try_from(ids.len()) else {
            return;
        };
        self.pool.extend_from_slice(ids);
        self.long.insert(key.into(), (offset, len));
    }

    /// Insert while reusing a packed short key from the scanner.
    #[inline(always)]
    fn insert_packed(&mut self, packed: u128, ids: &[u32]) {
        if self.insert_packed_backing(packed, ids) {
            self.insert_front(packed, ids);
        }
    }

    /// Insert into the collision-resolving table without changing the direct slot.
    fn insert_packed_backing(&mut self, packed: u128, ids: &[u32]) -> bool {
        if packed == EMPTY_SHORT_KEY {
            return false;
        }
        let Ok(len) = u16::try_from(ids.len()) else {
            return false;
        };
        if self.count >= FLAT_CACHE_MAX_LOAD || self.pool.len() >= FLAT_CACHE_MAX_POOL {
            self.clear_backing();
        }
        let mut idx = flat_cache_index(packed);
        let key = [packed as u64, (packed >> 64) as u64];
        loop {
            let slot = unsafe { *self.slots.get_unchecked(idx) };
            if slot.key == [0; 2] {
                let Some(value) = self.store_value(ids, len) else {
                    return false;
                };
                self.count += 1;
                let slot = unsafe { self.slots.get_unchecked_mut(idx) };
                slot.key = key;
                slot.value = value;
                return true;
            }
            if slot.key == key {
                let Some(value) = self.store_value(ids, len) else {
                    return false;
                };
                let slot = unsafe { self.slots.get_unchecked_mut(idx) };
                slot.value = value;
                return true;
            }
            idx = (idx + 1) & (FLAT_CACHE_SIZE - 1);
        }
    }

    /// Pack small IDs inline, or retain full-width IDs in the existing pool.
    #[inline(always)]
    fn store_value(&mut self, ids: &[u32], len: u16) -> Option<u64> {
        let first = ids.first().copied().unwrap_or(0);
        let second = ids.get(1).copied().unwrap_or(0);
        if len <= 2 && (first | second) < 1 << 31 {
            return Some(len as u64 | (first as u64) << 2 | (second as u64) << 33);
        }
        // Tag 3 separates a full u16 length and u32 offset from inline IDs.
        // If the pool exceeds that offset range, skip memoization without truncation.
        let offset = u32::try_from(self.pool.len()).ok()?;
        self.pool.extend_from_slice(ids);
        Some(3 | (len as u64) << 2 | (offset as u64) << 18)
    }

    /// Insert only compact values that the hot-cache entry can hold inline.
    #[inline(always)]
    fn insert_front(&mut self, key: u128, ids: &[u32]) {
        if !(1..=4).contains(&ids.len()) || ids[0] >= 1 << 24 {
            return;
        }
        let second = ids.get(1).copied().unwrap_or(0);
        let third = ids.get(2).copied().unwrap_or(0);
        let fourth = ids.get(3).copied().unwrap_or(0);
        let value = ids.len() as u64 | (ids[0] as u64) << 8 | (second as u64) << 32;
        let extension = third as u64 | (fourth as u64) << 32;
        let slot = FrontCacheSlot {
            key: [key as u64, (key >> 64) as u64],
            value,
            extension,
        };
        let index = self.front_index(key);
        self.front[index] = slot;
    }
}

thread_local! {
    static TL_BPE_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new());
    static TL_FUSED_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new());
    static TL_FUSED_PARALLEL_CACHE: RefCell<FlatCache> = RefCell::new(FlatCache::new_parallel());
}

/// One scanner-produced range waiting for the cache-probe phase.
#[derive(Clone, Copy)]
struct FusedPiece {
    slot: *const FrontCacheSlot,
    key: [u64; 2],
}

const _: () = assert!(std::mem::size_of::<FusedPiece>() == 24);

/// Concrete scanner sink that keeps fused cache probes monomorphized.
pub(crate) struct FusedStream<'a> {
    model: &'a Bpe,
    cache: &'a mut FlatCache,
    out: &'a mut Vec<u32>,
    error: Option<String>,
    // Only the prefix before `pending_len` is initialized and read.
    pending: &'a mut [MaybeUninit<FusedPiece>; FUSED_PIECE_CAPACITY],
    pending_len: usize,
}

impl FusedStream<'_> {
    /// Emit one added-token ID after all preceding text ranges.
    #[inline(always)]
    pub(crate) fn push_id(&mut self, id: u32) {
        self.flush();
        if self.error.is_none() {
            self.out.push(id);
        }
    }

    /// Resolve pending ranges before their borrowed source can expire.
    #[inline(always)]
    pub(crate) fn flush_pending(&mut self) {
        self.flush();
    }

    /// Return the number of token IDs emitted so far.
    #[inline(always)]
    pub(crate) fn output_len(&mut self) -> usize {
        self.flush_pending();
        self.out.len()
    }

    /// Queue one trusted scanner range and prefetch its direct-cache line.
    ///
    /// # Safety
    ///
    /// `start..end` must be a non-empty in-bounds UTF-8 range of `input`.
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
        let front = self.cache.front.as_ptr();
        let pending = unsafe {
            self.pending
                .as_mut_ptr()
                .add(self.pending_len)
                .cast::<FusedPiece>()
        };
        unsafe { self.push_short::<false>(input, start, end, front, pending) };
        self.pending_len += 1;
    }

    /// Queue one short piece at a caller-provided pending slot.
    #[inline(always)]
    unsafe fn push_short<const INBOUNDS: bool>(
        &mut self,
        input: &str,
        start: usize,
        end: usize,
        front: *const FrontCacheSlot,
        pending: *mut FusedPiece,
    ) {
        let len = end - start;
        debug_assert!(len <= 15);
        let packed = if INBOUNDS {
            unsafe { pack_short_range_inbounds(input, start, len) }
        } else {
            pack_short_range(input, start, end)
        };
        let index = self.cache.front_index(packed);
        // The fixed-size front allocation stays live for this stream.
        let slot = unsafe { front.add(index) };
        // SAFETY: the caller checked capacity before deriving this slot.
        unsafe {
            pending.write(FusedPiece {
                slot,
                key: [packed as u64, (packed >> 64) as u64],
            })
        };
        self.cache.prefetch_front(slot);
    }

    /// Resolve one long piece outside the short-key batch.
    #[cold]
    #[inline(never)]
    fn push_long(&mut self, input: &str, start: usize, end: usize) {
        self.flush();
        if self.error.is_none()
            && let Err(current) = self.model.tokenize_fused_cache_miss(
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

    /// Resolve queued ranges after their cache lines have had time to arrive.
    #[inline(never)]
    fn flush(&mut self) {
        let count = std::mem::take(&mut self.pending_len);
        if count == 0 || self.error.is_some() {
            return;
        }
        let out = &mut *self.out;
        out.reserve(4 * count);
        // The reserve leaves four writable lanes per pending piece.
        let mut destination = unsafe { out.as_mut_ptr().add(out.len()) };
        // SAFETY: every queue path writes a slot before increasing `pending_len`.
        let pending = unsafe {
            std::slice::from_raw_parts(self.pending.as_ptr().cast::<FusedPiece>(), count)
        };
        for (index, &piece) in pending.iter().enumerate() {
            let (value, extension, found) = self.cache.front_packed_value_at(piece.key, piece.slot);
            let ids = ((value >> 8) & 0x00ff_ffff) | (value & 0xffff_ffff_0000_0000);
            // SAFETY: the initial reserve and every miss-path reserve
            // leave four writable lanes for each remaining piece. Miss
            // stores stay past the cursor and are overwritten below.
            unsafe {
                (destination as *mut u64).write_unaligned(ids);
                (destination.add(2) as *mut u64).write_unaligned(extension);
                destination = destination.add(if found { value as u8 as usize } else { 0 });
            }
            if found {
                continue;
            }

            // SAFETY: the cursor belongs to `out`'s current allocation, and
            // every lane below it was initialized by a hit.
            unsafe { out.set_len(destination.offset_from(out.as_ptr()) as usize) };
            let packed = piece.key[0] as u128 | (piece.key[1] as u128) << 64;
            let result = if self.cache.get_packed_backing(packed, out) {
                Ok(())
            } else {
                let bytes = packed.to_le_bytes();
                let len = (piece.key[1] >> 56) as usize;
                // SAFETY: the packed key came from one exact UTF-8 scanner range.
                let input = unsafe { std::str::from_utf8_unchecked(&bytes[..len]) };
                self.model
                    .tokenize_fused_uncached(self.cache, input, 0..len, packed, out, false)
            };
            if let Err(current) = result {
                self.error = Some(current);
                return;
            }
            out.reserve(4 * (count - index - 1));
            destination = unsafe { out.as_mut_ptr().add(out.len()) };
        }
        // SAFETY: hits initialized every lane through the final cursor.
        unsafe { out.set_len(destination.offset_from(out.as_ptr()) as usize) };
    }
}

impl FusedPieceSink for FusedStream<'_> {
    /// Queue one scanner-produced piece directly into the fused cache batch.
    #[inline(always)]
    unsafe fn push_piece(&mut self, input: &str, start: usize, end: usize) {
        unsafe { self.push(input, start, end) };
    }

    /// Queue one trusted mask after checking the pending capacity once.
    #[inline(always)]
    unsafe fn push_mask(&mut self, input: &str, mask_base: usize, start: &mut usize, mask: u64) {
        let inbounds = input.len().saturating_sub(mask_base) >= 78;
        let first_end = mask_base + mask.trailing_zeros() as usize;
        // Smear each boundary 1..=15 bits upward to prove every internal gap is short.
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

impl FusedStream<'_> {
    /// Queue one boundary mask with its wide-load proof resolved outside the loop.
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
        // Misses never resize the fixed-size front allocation.
        let front = self.cache.front.as_ptr();
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
                unsafe { self.push_short::<INBOUNDS>(input, *start, end, front, pending) };
                pending = unsafe { pending.add(1) };
            }
            *start = end;
        }
        self.pending_len = unsafe { pending.offset_from(pending_base) as usize };
    }
}

const CACHE_SHARDS: usize = 64;
const SHARED_CACHE_MAX_PER_SHARD: usize = 16 * 1024;

struct SharedCache {
    shards: Vec<Mutex<FxHashMap<String, Vec<u32>>>>,
}

impl SharedCache {
    fn new() -> Self {
        Self {
            shards: (0..CACHE_SHARDS)
                .map(|_| Mutex::new(HashMap::with_hasher(FxBuildHasher)))
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
        if shard.len() >= SHARED_CACHE_MAX_PER_SHARD {
            shard.clear();
        }
        shard.insert(key, value);
    }
}

/// Raw deserialization helper.
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

/// Distinguish an omitted optional JSON field from a present invalid `null` value.
fn deserialize_present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

/// One resolved merge rule stored in a portable tokenizer sidecar.
#[derive(Encode, Decode)]
struct ResolvedMerge {
    left: u32,
    right: u32,
    rank: u32,
    merged: u32,
}

/// Expensive merge-graph results persisted independently of runtime tables.
#[derive(Encode, Decode)]
struct ResolvedDecomposition {
    unmerge_map: Vec<(TokenId, TokenId)>,
    is_orphan: Vec<bool>,
}

impl ResolvedDecomposition {
    /// Validate cached decomposition before it reaches unchecked token lookups.
    fn validate(
        self,
        vocab_size: usize,
        merge_map: &ParsedMergeMap,
        ignore_merges: bool,
    ) -> Result<Self> {
        if self.unmerge_map.len() != vocab_size || self.is_orphan.len() != vocab_size {
            return Err("invalid .tkz decomposition length".into());
        }
        if ignore_merges && self.is_orphan.iter().any(|&is_orphan| is_orphan) {
            return Err("ignore_merges .tkz model contains an orphan token".into());
        }

        for (token, &(left, right)) in self.unmerge_map.iter().enumerate() {
            if left as usize >= vocab_size || right as usize >= vocab_size {
                return Err("out-of-range .tkz decomposition token".into());
            }

            let token = token as TokenId;
            if (left, right) == (token, token) {
                continue;
            }
            if self.is_orphan[token as usize]
                || merge_map
                    .get(&(left, right))
                    .is_none_or(|&(_, merged)| merged != token)
            {
                return Err("invalid .tkz decomposition pair".into());
            }
        }
        Ok(self)
    }
}

/// Canonical BPE inputs persisted by the native tokenizer sidecar.
#[derive(Encode, Decode)]
pub(crate) struct ResolvedBpe {
    id_to_token: Vec<String>,
    merges: Vec<ResolvedMerge>,
    decomposition: ResolvedDecomposition,
    ranked_slot_indices: Vec<u32>,
    byte_fallback: bool,
    ignore_merges: bool,
}

/// A compact, fully checkable trie for exact token-prefix lookup in a `.tkz` sidecar.
#[derive(Clone, Encode, Decode, PartialEq)]
pub(crate) struct ExactTokenTrie {
    nodes: Vec<ExactTokenTrieNode>,
    incoming_bytes: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq)]
struct ExactTokenTrieNode {
    first_child: u32,
    edge_count: u16,
    token: TokenId,
}

/// Exact whole-piece lookup built from JSON/V4 inputs or restored by a V5 sidecar.
#[derive(Clone, PartialEq)]
enum ExactTokenMatcher {
    /// JSON and V4 only need to recognize a complete legal vocabulary spelling.
    Direct(Vec<bool>),
    Trie(ExactTokenTrie),
}

/// Monotonic counter for unique Bpe instance IDs.
static BPE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

impl ResolvedBpe {
    /// Materialize the exact non-orphan vocabulary lookup used by a version-5 sidecar.
    pub(crate) fn exact_token_trie(&self) -> Result<ExactTokenTrie> {
        ExactTokenTrie::from_tokens(&self.id_to_token, &self.decomposition.is_orphan)
    }
}

impl ExactTokenTrie {
    /// Build a compact prefix tree from the canonical sidecar vocabulary.
    fn from_tokens(id_to_token: &[String], is_orphan: &[bool]) -> Result<Self> {
        if id_to_token.is_empty() || id_to_token.len() != is_orphan.len() {
            return Err("invalid .tkz exact-token trie vocabulary".into());
        }

        #[derive(Default)]
        struct BuildNode {
            token: TokenId,
            children: Vec<(u8, u32)>,
        }

        let mut nodes = vec![BuildNode {
            token: INVALID_TOKEN,
            children: Vec::new(),
        }];
        for (token, text) in id_to_token.iter().enumerate() {
            if is_orphan[token] {
                continue;
            }
            if text.is_empty() {
                return Err("empty token in .tkz exact-token trie".into());
            }

            let mut node = 0usize;
            for &byte in text.as_bytes() {
                if let Some((_, child)) = nodes[node]
                    .children
                    .iter()
                    .find(|(existing, _)| *existing == byte)
                {
                    node = *child as usize;
                    continue;
                }

                let child = u32::try_from(nodes.len())
                    .map_err(|_| "exact-token trie exceeds u32 node indexes")?;
                nodes.push(BuildNode {
                    token: INVALID_TOKEN,
                    children: Vec::new(),
                });
                nodes[node].children.push((byte, child));
                node = child as usize;
            }

            let token = u32::try_from(token).map_err(|_| "vocabulary exceeds u32 token IDs")?;
            if nodes[node].token != INVALID_TOKEN {
                return Err("duplicate token path in .tkz exact-token trie".into());
            }
            nodes[node].token = token;
        }

        // Breadth-first numbering makes a node's children one contiguous range. Each
        // child position then identifies its transition, so only its incoming byte is stored.
        let mut order = vec![0u32];
        let mut next = 0usize;
        let mut resolved_nodes = Vec::with_capacity(nodes.len());
        let mut incoming_bytes = Vec::with_capacity(nodes.len().saturating_sub(1));
        while next < order.len() {
            let node_index = order[next] as usize;
            next += 1;
            let node = &mut nodes[node_index];
            node.children.sort_unstable_by_key(|(byte, _)| *byte);
            let edge_count = u16::try_from(node.children.len())
                .map_err(|_| "exact-token trie node has too many edges")?;
            let first_child = if node.children.is_empty() {
                0
            } else {
                u32::try_from(order.len())
                    .map_err(|_| "exact-token trie exceeds u32 node indexes")?
            };
            for &(byte, child) in &node.children {
                order.push(child);
                incoming_bytes.push(byte);
            }
            resolved_nodes.push(ExactTokenTrieNode {
                first_child,
                edge_count,
                token: node.token,
            });
        }

        Self {
            nodes: resolved_nodes,
            incoming_bytes,
        }
        .validate(id_to_token, is_orphan)
    }

    /// Validate every stored path before a sidecar trie reaches the encode-time fast path.
    fn validate(self, id_to_token: &[String], is_orphan: &[bool]) -> Result<Self> {
        if self.nodes.is_empty()
            || id_to_token.is_empty()
            || id_to_token.len() != is_orphan.len()
            || self.nodes[0].token != INVALID_TOKEN
        {
            return Err("invalid .tkz exact-token trie root".into());
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
            for child in first..end {
                parent_counts[child] = parent_counts[child]
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

        let mut seen_tokens = vec![false; id_to_token.len()];
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
                        if token >= id_to_token.len()
                            || is_orphan[token]
                            || seen_tokens[token]
                            || id_to_token[token].as_bytes() != path
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

    /// Return the longest stored token beginning at byte zero of `input`.
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
    /// Return a legal match; the caller still proves that it covers the whole input.
    fn next_match(&self, input: &str, token_to_id: &Vocab) -> Option<TokenId> {
        match self {
            Self::Direct(is_orphan) => {
                let token = token_to_id.get(input).copied()?;
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

/// Entry in the BPE merge priority queue.
/// `key = (rank << 32) | pos`, `val = (left_c << 32) | right_c`.
#[derive(Clone, Copy, Eq, PartialEq)]
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
    fn left_c(&self) -> u32 {
        (self.val >> 32) as u32
    }

    #[inline(always)]
    fn right_c(&self) -> u32 {
        self.val as u32
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

/// Symbol in the merge linked list.
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

/// Scratch storage for the encoded-text path's quaternary candidate heap.
#[derive(Default)]
struct EncodedMergeScratch {
    symbols: Vec<MergeSymbol>,
    heap: QuaternaryHeap<Reverse<MergeEntry>>,
}

impl EncodedMergeScratch {
    /// Append one symbol produced by the variable-width encoded-text path.
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

/// Concrete heap operations preserve direct AArch64 code generation.
macro_rules! run_merge_loop_body {
    ($bpe:ident, $scratch:ident, $out:ident) => {{
        let symbols = &mut $scratch.symbols;
        let heap = &mut $scratch.heap;

        while let Some(Reverse(entry)) = heap.pop() {
            let pos = entry.pos() as usize;
            let sym = symbols[pos];

            // Skip candidates invalidated by an earlier overlapping merge.
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

            // Resolve the canonical token ID for this still-valid pair.
            let new_id = match $bpe.merge_adj.get(left_c, right_c) {
                Some((_, nid)) => nid,
                None => continue,
            };

            // Let the left symbol absorb the right and repair the linked list.
            symbols[pos].c = new_id;
            symbols[pos].next = next_sym.next;
            if next_sym.next >= 0 {
                symbols[next_sym.next as usize].prev = pos as i32;
            }
            symbols[next_idx].c = INVALID_TOKEN;

            // Queue only the two pairs newly exposed by this merge.
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

        // Emit the surviving linked-list nodes in their original order.
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
    // Encoded BPE uses four children; fused/raw BPE keeps the standard binary heap.
    static TL_ENCODED_MERGE_SCRATCH: RefCell<EncodedMergeScratch> =
        RefCell::new(EncodedMergeScratch::default());
}

/// Open-addressing hash table storing `(left_id, right_id) → (rank, merged_id)`.
#[derive(Clone, PartialEq)]
struct RankedMergeMap {
    mask: usize,
    /// Pair keys determine probe termination before their payload is read.
    keys: Vec<u64>,
    /// Packed `rank << 32 | merged_id` values aligned with occupied keys.
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

    /// Rebuild validated merges at cached slots and verify every linear-probe chain.
    fn from_cached_indices(merges: &[(ResolvedMerge, u32)]) -> Result<Self> {
        if merges.is_empty() {
            return Ok(Self {
                mask: 0,
                keys: Vec::new(),
                values: Vec::new(),
            });
        }

        let capacity = merges
            .len()
            .checked_mul(2)
            .and_then(usize::checked_next_power_of_two)
            .ok_or("ranked .tkz table capacity overflow")?;
        let mask = capacity - 1;
        let mut keys = vec![EMPTY_KEY; capacity];
        let mut values = vec![0; capacity];
        for (merge, slot_index) in merges {
            let key = keys
                .get_mut(*slot_index as usize)
                .ok_or("out-of-range ranked .tkz slot")?;
            if *key != EMPTY_KEY {
                return Err("duplicate ranked .tkz slot".into());
            }
            *key = pack_pair(merge.left, merge.right);
            values[*slot_index as usize] = (merge.rank as u64) << 32 | merge.merged as u64;
        }

        // Linear probing is valid exactly when each key's home bucket lies
        // between its cluster start and its stored position.
        let first_empty = keys
            .iter()
            .position(|&key| key == EMPTY_KEY)
            .ok_or("ranked .tkz table has no empty slot")?;
        let mut cluster_start = 1;
        for step in 1..capacity {
            let key = keys[(first_empty + step) & mask];
            if key == EMPTY_KEY {
                cluster_start = step + 1;
                continue;
            }
            let home = fx_hash(key) as usize & mask;
            let relative_home = home.wrapping_add(capacity).wrapping_sub(first_empty) & mask;
            if relative_home < cluster_start || relative_home > step {
                return Err("invalid ranked .tkz probe chain".into());
            }
        }

        Ok(Self { mask, keys, values })
    }

    /// Look up the rank and merged token ID for a pair.
    #[inline(always)]
    fn get(&self, t1: u32, t2: u32) -> Option<(u32, u32)> {
        if self.keys.is_empty() {
            return None;
        }
        let key = pack_pair(t1, t2);
        let mut idx = fx_hash(key) as usize & self.mask;
        loop {
            // SAFETY: both constructors keep same-length power-of-two arrays,
            // `mask == len - 1`, and at least one empty terminating key.
            let slot_key = unsafe { *self.keys.get_unchecked(idx) };
            if slot_key == key {
                // The matching key proves this same-index payload was initialized.
                let payload = unsafe { *self.values.get_unchecked(idx) };
                return Some(((payload >> 32) as u32, payload as u32));
            }
            if slot_key == EMPTY_KEY {
                return None;
            }
            idx = (idx + 1) & self.mask;
        }
    }

    /// Count the populated merge slots.
    fn len(&self) -> usize {
        self.keys.iter().filter(|&&key| key != EMPTY_KEY).count()
    }
}

/// CSR adjacency structure for merge pair discovery.
#[derive(Clone)]
struct MergeAdjacency {
    offsets: Vec<u32>,
    // Each row's rules as `neighbor << 32 | rank`, sorted: the binary search
    // touches only these aligned 8-byte keys (the old 12-byte tuples straddled
    // cache lines and dragged the payload through every probe), and a hit's
    // rank arrives in the same load. Merged IDs are read only on a hit.
    keys: Vec<u64>,
    new_ids: Vec<u32>,
}

impl MergeAdjacency {
    /// Build the same CSR rows from a validated, ordered `.tkz` merge list.
    fn from_resolved(merges: &[(ResolvedMerge, u32)], vocab_size: usize) -> Self {
        let mut counts = vec![0u32; vocab_size];
        for (merge, _) in merges {
            counts[merge.left as usize] += 1;
        }

        let mut offsets = Vec::with_capacity(vocab_size + 1);
        offsets.push(0u32);
        let mut running = 0u32;
        for &count in &counts {
            running += count;
            offsets.push(running);
        }

        let mut rows = vec![(0u64, 0u32); running as usize];
        let mut write_pos = offsets[..vocab_size].to_vec();
        for (merge, _) in merges {
            let index = write_pos[merge.left as usize] as usize;
            rows[index] = ((merge.right as u64) << 32 | merge.rank as u64, merge.merged);
            write_pos[merge.left as usize] += 1;
        }

        for left in 0..vocab_size {
            let start = offsets[left] as usize;
            let end = offsets[left + 1] as usize;
            rows[start..end].sort_unstable_by_key(|&(key, _)| key);
        }

        let (keys, new_ids) = rows.into_iter().unzip();
        Self {
            offsets,
            keys,
            new_ids,
        }
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

        // Neighbors are unique within a row, so sorting the packed keys is
        // exactly the old per-row neighbor sort.
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
        // The unique key for `right`, if present, is the first key at or above
        // `right << 32` (its low half is the rank, which is nonnegative).
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
    shared_cache: SharedCache,
    fused_shared_cache: SharedCache,
    id_to_token: Vec<String>,
    token_to_id: HashMap<String, u32>,
    // Direct char-to-token table for the Basic Multilingual Plane so the
    // encoded merge path resolves non-ASCII characters with one indexed load
    // instead of a string hash probe. Astral tokens use the map fallback.
    bmp_char_token: Box<[u32]>,
    byte_to_initial_token: [u32; 256],
    byte_fallback_token_ids: [u32; 256],
    /// Token id for each single ASCII-character string (`INVALID_TOKEN` when
    /// absent). Fast path for the char-based merge engine, avoiding a HashMap
    /// probe per character.
    single_char_token: [u32; 128],
    ranked_merge_map: RankedMergeMap,
    byte_pair_initial: Vec<(u32, u32)>,
    dense_merge: Vec<u64>,
    dense_ranked_merge: Vec<u32>,
    /// Bit width of each dense ranked-table endpoint; zero means no table.
    dense_ranked_bits: u32,
    /// Whether merge-result IDs preserve rank order.
    ranked_merges: bool,
    fused_cache_seeds: Vec<(u128, u32)>,
    merge_adj: MergeAdjacency,
    ignore_merges: bool,
    byte_fallback: bool,
    /// Byte-pair coverage used to find BPE-safe input split boundaries.
    pub bigram_bridge_table: BigramBridgeTable,
}

impl TryFrom<RawBpe> for Bpe {
    type Error = String;

    /// Build a BPE from its Hugging Face JSON representation.
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
            // Legacy models are untagged, but must still name the two BPE fields.
            None => match (vocab, merges) {
                (Some(vocab), Some(merges)) => (vocab, merges),
                _ => return Err("legacy BPE model requires vocab and merges".into()),
            },
        };
        let merge_map = parse_merges(&vocab, &merges)?;
        // Parsing owns this vocabulary, so the finished model can keep it directly.
        Self::build(vocab, merge_map, byte_fallback, ignore_merges, None)
    }
}

enum Decomposition {
    Pair(TokenId, TokenId),
    CharsNotInVocab,
    Stuck,
}

const DECOMPOSITION_STACK_CAPACITY: usize = 16;

/// Resolve one scalar to the same initial token ID used by BPE decomposition.
fn decomposition_initial_token(ch: char, vocab: &Vocab, bmp_char_token: &[u32]) -> Option<TokenId> {
    if (ch as u32) < 0x10000 {
        // This table has the same one-scalar mapping as the UTF-8 map probe below.
        let token = bmp_char_token[ch as usize];
        (token != INVALID_TOKEN).then_some(token)
    } else {
        // Astral scalars retain the existing exact UTF-8 vocabulary lookup.
        let mut buf = [0u8; 4];
        vocab.get(ch.encode_utf8(&mut buf)).copied()
    }
}

/// Return the parsed merge answer through the exact byte table when both IDs are byte initials.
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
        // This table is filled from the same parsed merge map as the CSR fallback.
        let pair = byte_pair_initial[left_byte as usize * 256 + right_byte as usize];
        return (pair.0 != u32::MAX).then_some(pair);
    }
    merge_adjacency.get(left, right)
}

/// Reduce one exact initial-token sequence and return the pair that produces its final token.
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
        // The surviving merge is the final pair that produces this token.
        if len == 2 {
            return Decomposition::Pair(tokens[0], tokens[1]);
        }
        tokens[best_pos] = best_new;
        tokens.copy_within(best_pos + 1..len, best_pos);
        len -= 1;
    }
}

/// Keep the existing heap-backed decomposition for vocabulary spellings beyond the stack bound.
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

/// Finds the final producing pair through exact adjacency and character-ID lookups.
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
            // Restart in the retained heap path before discarding any long spelling data.
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

/// Invert initial-byte IDs so byte-pair construction can recognize their token IDs directly.
fn initial_token_byte_map(byte_to_initial_token: &[TokenId; 256], vocab_size: usize) -> Vec<u16> {
    let mut initial_token_byte = vec![u16::MAX; vocab_size];
    for (byte, &token) in byte_to_initial_token.iter().enumerate() {
        if token != INVALID_TOKEN {
            // Dense vocabulary validation guarantees that every byte token indexes this map.
            initial_token_byte[token as usize] = byte as u16;
        }
    }
    initial_token_byte
}

/// Fill byte-pair merge entries directly from the canonical parsed merge rules.
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
    /// Copy the canonical model inputs needed to rebuild this BPE exactly.
    pub(crate) fn resolved_config(&self) -> ResolvedBpe {
        let mut merges = self
            .ranked_merge_map
            .keys
            .iter()
            .enumerate()
            .filter(|(_, key)| **key != EMPTY_KEY)
            .map(|(slot_index, &key)| {
                let payload = self.ranked_merge_map.values[slot_index];
                (
                    ResolvedMerge {
                        left: (key >> 32) as u32,
                        right: key as u32,
                        rank: (payload >> 32) as u32,
                        merged: payload as u32,
                    },
                    slot_index as u32,
                )
            })
            .collect::<Vec<_>>();

        // Rank order keeps canonical merges aligned with their cached slot indices.
        merges.sort_unstable_by_key(|(merge, _)| merge.rank);
        let (merges, ranked_slot_indices) = merges.into_iter().unzip();

        // Identity decomposition is ambiguous: an orphan and a token whose
        // characters are absent from the vocabulary both keep `(id, id)`.
        // The existing exact-token automaton distinguishes those cases.
        let is_orphan = self
            .unmerge_map
            .iter()
            .enumerate()
            .map(|(token, &pair)| {
                let token = token as TokenId;
                pair == (token, token)
                    && self.next_match(&self.id_to_token[token as usize]) != Some(token)
            })
            .collect();

        ResolvedBpe {
            id_to_token: self.id_to_token.clone(),
            merges,
            decomposition: ResolvedDecomposition {
                unmerge_map: self.unmerge_map.clone(),
                is_orphan,
            },
            ranked_slot_indices,
            byte_fallback: self.byte_fallback,
            ignore_merges: self.ignore_merges,
        }
    }

    /// Rebuild a BPE from validated canonical sidecar inputs with fresh caches.
    pub(crate) fn from_resolved(resolved: ResolvedBpe) -> Result<Self> {
        Self::from_resolved_with_exact_token_trie(resolved, None)
    }

    /// Rebuild a BPE from canonical sidecar inputs and an optional checked version-5 matcher.
    pub(crate) fn from_resolved_with_exact_token_trie(
        resolved: ResolvedBpe,
        exact_token_trie: Option<ExactTokenTrie>,
    ) -> Result<Self> {
        let ResolvedBpe {
            id_to_token,
            merges,
            decomposition,
            ranked_slot_indices,
            byte_fallback,
            ignore_merges,
        } = resolved;
        let vocab_size = id_to_token.len();
        if vocab_size == 0 || vocab_size > u32::MAX as usize {
            return Err("invalid .tkz vocabulary size".into());
        }

        // Generated sidecars have unique ranks and pairs. Recheck both before
        // the derived tables enter unchecked encode-time lookup paths.
        if merges.len() != ranked_slot_indices.len() {
            return Err("ranked .tkz index count mismatch".into());
        }
        let mut merges = merges
            .into_iter()
            .zip(ranked_slot_indices)
            .collect::<Vec<_>>();
        merges.sort_unstable_by_key(|(merge, _)| merge.rank);
        if merges
            .windows(2)
            .any(|pair| pair[0].0.rank == pair[1].0.rank)
        {
            return Err("duplicate merge rank in .tkz model".into());
        }

        let mut merge_map = ParsedMergeMap::with_capacity(merges.len());
        for (expected_rank, (merge, _)) in merges.iter().enumerate() {
            if merge.left as usize >= vocab_size
                || merge.right as usize >= vocab_size
                || merge.merged as usize >= vocab_size
            {
                return Err("out-of-range merge token in .tkz model".into());
            }
            if merge.rank as usize != expected_rank {
                return Err("out-of-range ranked .tkz token".into());
            }
            if merge_map
                .insert((merge.left, merge.right), (merge.rank, merge.merged))
                .is_some()
            {
                return Err("duplicate merge pair in .tkz model".into());
            }
        }

        let decomposition = decomposition.validate(vocab_size, &merge_map, ignore_merges)?;
        let ranked_merge_map = RankedMergeMap::from_cached_indices(&merges)?;
        let merge_adj = MergeAdjacency::from_resolved(&merges, vocab_size);
        let exact_token_trie = exact_token_trie
            .map(|trie| trie.validate(&id_to_token, &decomposition.is_orphan))
            .transpose()?;
        let mut vocab = Vocab::with_capacity(vocab_size);
        for (id, token) in id_to_token.iter().enumerate() {
            let id = u32::try_from(id).map_err(|_| "invalid .tkz vocabulary size")?;
            if vocab.insert(token.clone(), id).is_some() {
                return Err("duplicate token text in .tkz vocabulary".into());
            }
        }
        Self::build_with_exact_token_trie(
            vocab,
            merge_map,
            byte_fallback,
            ignore_merges,
            Some((decomposition, ranked_merge_map)),
            exact_token_trie,
            Some(id_to_token),
            Some(merge_adj),
        )
    }

    /// Return the safe-splitting table when piece boundaries do not affect model semantics.
    pub fn bigram_bridge_table(&self) -> Option<&BigramBridgeTable> {
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
        Self::build(vocab.clone(), merge_map, false, false, None)
    }

    /// Build runtime state from canonical inputs and optional validated sidecar tables.
    fn build(
        vocab: Vocab,
        merge_map: ParsedMergeMap,
        byte_fallback: bool,
        ignore_merges: bool,
        cached_tables: Option<(ResolvedDecomposition, RankedMergeMap)>,
    ) -> Result<Self> {
        Self::build_with_exact_token_trie(
            vocab,
            merge_map,
            byte_fallback,
            ignore_merges,
            cached_tables,
            None,
            None,
            None,
        )
    }

    /// Build runtime state with optional prevalidated sidecar representations.
    fn build_with_exact_token_trie(
        vocab: Vocab,
        merge_map: ParsedMergeMap,
        byte_fallback: bool,
        ignore_merges: bool,
        cached_tables: Option<(ResolvedDecomposition, RankedMergeMap)>,
        exact_token_trie: Option<ExactTokenTrie>,
        ordered_sidecar_tokens: Option<Vec<String>>,
        sidecar_merge_adjacency: Option<MergeAdjacency>,
    ) -> Result<Self> {
        if vocab.is_empty() {
            return Err("cannot build Bpe with empty vocabulary".into());
        }

        // A sidecar supplies both derived tables or neither, avoiding mixed construction modes.
        let (decomposition, ranked_merge_map) = cached_tables.unzip();

        // Sidecars already own canonical token order; JSON construction still derives it from
        // the map so its non-contiguous-ID validation remains unchanged.
        let id_to_token = if let Some(tokens) = ordered_sidecar_tokens {
            if tokens.len() != vocab.len() {
                return Err("invalid .tkz vocabulary size".into());
            }
            tokens
        } else {
            // Token IDs must be a permutation of `0..vocab.len()` before they
            // become unchecked vector indexes in the encode path.
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

        // The final BPE retains this exact CSR lookup; build it before
        // decomposition so pairs outside the byte table use its contiguous rows.
        let merge_adj = sidecar_merge_adjacency
            .unwrap_or_else(|| MergeAdjacency::from_parsed(&merge_map, vocab_size));

        // Build this retained direct character mapping before decomposition so
        // its per-character initialization avoids repeated vocabulary probes.
        let mut bmp_char_token = vec![INVALID_TOKEN; 0x10000].into_boxed_slice();
        for (id, token) in id_to_token.iter().enumerate() {
            let mut chars = token.chars();
            if let (Some(ch), None) = (chars.next(), chars.next())
                && (ch as u32) < 0x10000
            {
                bmp_char_token[ch as usize] = id as TokenId;
            }
        }

        let (unmerge_map, mut is_orphan) = if let Some(decomposition) = decomposition {
            (decomposition.unmerge_map, decomposition.is_orphan)
        } else {
            let mut unmerge_map = (0..=max_token).map(|t| (t, t)).collect::<Vec<_>>();
            let mut is_orphan = vec![false; (max_token + 1) as usize];
            for (tid, text) in id_to_token.iter().enumerate() {
                if text.chars().count() < 2 {
                    continue;
                }
                match encoding_decomposition(
                    text,
                    &vocab,
                    &initial_token_byte,
                    &byte_pair_initial,
                    &merge_adj,
                    &bmp_char_token,
                ) {
                    Decomposition::Pair(left, right) => {
                        unmerge_map[tid] = (left, right);
                    }
                    Decomposition::Stuck => {
                        is_orphan[tid] = true;
                    }
                    Decomposition::CharsNotInVocab => {}
                }
            }
            (unmerge_map, is_orphan)
        };
        if ignore_merges {
            // Exact whole-piece lookup must include tokens the merge graph cannot construct.
            is_orphan.fill(false);
        }

        let ranked_merge_map =
            ranked_merge_map.unwrap_or_else(|| RankedMergeMap::from_parsed(&merge_map));

        let mut byte_fallback_token_ids = [INVALID_TOKEN; 256];
        // Disabled fallback never reads this table in the encoded merge path.
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
        for (byte, slot) in single_char_token.iter_mut().enumerate() {
            let ch = byte as u8 as char;
            let mut buf = [0u8; 1];
            if let Some(&id) = vocab.get(ch.encode_utf8(&mut buf) as &str) {
                *slot = id;
            }
        }

        // A constant rank-to-ID offset lets the short loop store one priority value.
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
            // The guard excludes the sentinel, so this range contains every
            // byte initial while staying within the old ten-bit maximum.
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

        // V5 retains its checked trie. JSON and V4 need only a complete
        // spelling match, so retain the derived orphan bits instead of DAAC.
        let matcher = if let Some(trie) = exact_token_trie {
            ExactTokenMatcher::Trie(trie)
        } else {
            ExactTokenMatcher::Direct(is_orphan)
        };

        let bigram_bridge_table = build_bigram_bridge_table(&id_to_token, byte_fallback);
        Ok(Self {
            id: next_bpe_id(),
            matcher,
            unmerge_map,
            token_lens,
            shared_cache: SharedCache::new(),
            fused_shared_cache: SharedCache::new(),
            id_to_token,
            token_to_id: vocab,
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
        self.matcher.next_match(input, &self.token_to_id)
    }

    /// Tokenizes one BPE input into token IDs.
    pub fn tokenize(&self, input: &str) -> Result<Vec<TokenId>> {
        let mut out = Vec::new();
        self.tokenize_into(input, &mut out)?;
        Ok(out)
    }

    /// Test a whole-token match while keeping long vocabulary lengths exact.
    fn token_length_matches(&self, token: TokenId, len: usize) -> bool {
        let compact = self.token_lens[token as usize];
        if compact == u8::MAX {
            // The marker is not a truncated length; long tokens use their owned string.
            self.id_to_token[token as usize].len() == len
        } else {
            compact as usize == len
        }
    }

    /// Append exact IDs, using compact lengths only as a whole-token match check.
    #[inline(always)]
    pub fn tokenize_into(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
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
        if self.shared_cache.get_into(input, out) {
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
        self.shared_cache.insert(input.to_string(), ids.to_vec());

        Ok(())
    }

    /// Priority-queue BPE merge on already-encoded (ByteLevel) text.
    fn merge_all_encoded_into(&self, input: &str, out: &mut Vec<u32>) -> Result<()> {
        if input.is_empty() {
            return Ok(());
        }

        TL_ENCODED_MERGE_SCRATCH.with(|s| {
            let mut scratch = s.borrow_mut();
            scratch.symbols.clear();
            scratch.heap.clear();

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
                    self.token_to_id.get(s).copied()
                };
                if let Some(id) = found {
                    scratch.push_encoded_symbol(id);
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
                    scratch.push_encoded_symbol(id);
                }
            }

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

    /// Priority-queue BPE merge on raw (pre-ByteLevel) bytes.
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
                // Check pair with previous byte via pre-computed table.
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

            // Bulk heapify.
            let mut tmp = std::mem::take(&mut scratch.heap_buf);
            scratch.heap.extend(tmp.drain(..));
            scratch.heap_buf = tmp;

            self.run_merge_loop(&mut scratch, out);

            Ok(())
        })
    }

    /// Merge a short raw-byte piece with stack-resident neighbor state.
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

    /// Look up an early-ID merge in the dense table, falling back to CSR.
    #[inline(always)]
    fn short_merge_lookup<const RANKED: bool>(&self, left: u32, right: u32) -> Option<(u32, u32)> {
        if RANKED && !self.dense_ranked_merge.is_empty() {
            let limit = 1 << self.dense_ranked_bits;
            if left < limit && right < limit {
                let index = (left << self.dense_ranked_bits | right) as usize;
                // SAFETY: construction allocates exactly `limit * limit` entries,
                // and both checked endpoints make this row-major index in bounds.
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

    /// Seed the priority queue with all initial adjacent pairs.
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

    /// Apply valid candidates in priority order and emit surviving symbols.
    #[inline(always)]
    fn run_merge_loop(&self, scratch: &mut MergeScratch, out: &mut Vec<u32>) {
        run_merge_loop_body!(self, scratch, out);
    }

    /// Apply the exact merge loop with the encoded-path quaternary heap.
    #[inline(always)]
    fn run_encoded_merge_loop(&self, scratch: &mut EncodedMergeScratch, out: &mut Vec<u32>) {
        run_merge_loop_body!(self, scratch, out);
    }

    /// Tokenizes raw text through fused byte-level encoding and BPE merging.
    #[inline(always)]
    pub fn tokenize_into_fused(&self, raw_input: &str, out: &mut Vec<u32>) -> Result<()> {
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
        if self.fused_shared_cache.get_into(raw_input, out) {
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
            if let Some(&id) = self.token_to_id.get(encoded.as_str()) {
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
        self.fused_shared_cache
            .insert(raw_input.to_string(), ids.to_vec());

        Ok(())
    }

    /// Tokenize scanner-produced raw pieces while holding the local cache once.
    pub(crate) fn tokenize_fused_stream(
        &self,
        input: &str,
        out: &mut Vec<u32>,
        use_parallel_cache: bool,
        scan: impl FnOnce(&mut FusedStream<'_>),
    ) -> Result<()> {
        out.reserve(input.len().saturating_add(3));

        if use_parallel_cache {
            return TL_FUSED_PARALLEL_CACHE
                .with(|cache| self.tokenize_fused_stream_with_cache(out, cache, scan));
        }

        TL_FUSED_CACHE.with(|cache| self.tokenize_fused_stream_with_cache(out, cache, scan))
    }

    /// Run one fused scanner against the selected thread-local cache.
    fn tokenize_fused_stream_with_cache(
        &self,
        out: &mut Vec<u32>,
        cache: &RefCell<FlatCache>,
        scan: impl FnOnce(&mut FusedStream<'_>),
    ) -> Result<()> {
        let bpe_id = self.id;
        let mut cache = cache.borrow_mut();
        if cache.bpe_id != bpe_id {
            self.prepare_fused_cache(&mut cache);
        }

        let mut pending = MaybeUninit::<[MaybeUninit<FusedPiece>; FUSED_PIECE_CAPACITY]>::uninit();
        // SAFETY: the array contains `MaybeUninit` slots and stays live through the scan.
        let pending = unsafe { &mut *pending.as_mut_ptr() };
        let mut stream = FusedStream {
            model: self,
            cache: &mut cache,
            out,
            error: None,
            pending,
            pending_len: 0,
        };
        scan(&mut stream);
        if stream.pending_len != 0 {
            stream.flush();
        }
        stream.error.map_or(Ok(()), Err)
    }

    /// Resolve one raw piece through local cache, shared cache, or exact BPE.
    #[inline(always)]
    fn tokenize_fused_cached(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_shared_cache: bool,
    ) -> Result<()> {
        if range.is_empty() {
            return Ok(());
        }
        if cache.get_front_packed(packed, out) {
            return Ok(());
        }

        self.tokenize_fused_cache_miss(cache, input, range, packed, out, use_shared_cache)
    }

    /// Resolve the uncommon backing-cache miss outside the fused hit loop.
    #[cold]
    #[inline(never)]
    fn tokenize_fused_cache_miss(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_shared_cache: bool,
    ) -> Result<()> {
        if packed != EMPTY_SHORT_KEY && cache.get_packed_backing(packed, out) {
            return Ok(());
        }
        self.tokenize_fused_uncached(cache, input, range, packed, out, use_shared_cache)
    }

    /// Resolve a piece already known to miss both local packed-cache tiers.
    #[cold]
    #[inline(never)]
    fn tokenize_fused_uncached(
        &self,
        cache: &mut FlatCache,
        input: &str,
        range: std::ops::Range<usize>,
        packed: u128,
        out: &mut Vec<u32>,
        use_shared_cache: bool,
    ) -> Result<()> {
        let text = &input[range];
        if packed == EMPTY_SHORT_KEY && cache.get_piece(text, packed, out) {
            return Ok(());
        }

        let start = out.len();
        if use_shared_cache && self.fused_shared_cache.get_into(text, out) {
            cache.insert_piece(text, packed, &out[start..]);
            return Ok(());
        }

        if self.ignore_merges {
            let mut encoded = String::with_capacity(text.len());
            for &byte in text.as_bytes() {
                encoded.push(BYTE_TO_CHAR[byte as usize]);
            }
            if let Some(&id) = self.token_to_id.get(encoded.as_str()) {
                out.push(id);
                cache.insert_piece(text, packed, &out[start..]);
                return Ok(());
            }
        }

        self.merge_all_raw_into(text, out)?;
        cache.insert_piece(text, packed, &out[start..]);
        if use_shared_cache {
            self.fused_shared_cache
                .insert(text.to_string(), out[start..].to_vec());
        }
        Ok(())
    }

    /// Reset and seed one thread's fused cache from exact short vocabulary entries.
    fn prepare_fused_cache(&self, cache: &mut FlatCache) {
        cache.bpe_id = self.id;
        cache.clear();
        for &(key, id) in self.fused_cache_seeds.iter().rev() {
            cache.insert_front(key, &[id]);
        }
        // Parallel workers keep backing space for pretokens they actually observe.
        if cache.front.len() == PARALLEL_FRONT_CACHE_SIZE {
            // Preserve only seeds displaced by direct-slot collisions.
            for &(key, id) in self.fused_cache_seeds.iter().rev() {
                let slot = &cache.front[cache.front_index(key)];
                if slot.key != [key as u64, (key >> 64) as u64] {
                    cache.insert_packed_backing(key, &[id]);
                }
            }
        } else {
            for &(key, id) in self.fused_cache_seeds.iter().rev() {
                if id as usize >= FUSED_CACHE_BACKING_SEED_LIMIT {
                    continue;
                }
                cache.insert_packed_backing(key, &[id]);
            }
        }
    }

    /// Appends IDs for an already byte-level-pre-tokenized buffer.
    pub fn tokenize_batch_fused(
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
                    self.tokenize_fused_cached(
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
        self.id_to_token.get(id as usize).map(String::as_str)
    }

    /// Returns the vocabulary ID for exact token text.
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Returns the number of entries in the BPE vocabulary.
    pub fn vocab_size(&self) -> usize {
        self.id_to_token.len()
    }
}

impl Clone for Bpe {
    fn clone(&self) -> Self {
        Self {
            id: next_bpe_id(),
            matcher: self.matcher.clone(),
            unmerge_map: self.unmerge_map.clone(),
            token_lens: self.token_lens.clone(),
            shared_cache: SharedCache::new(),
            fused_shared_cache: SharedCache::new(),
            id_to_token: self.id_to_token.clone(),
            token_to_id: self.token_to_id.clone(),
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
            // Long-token markers must not collapse distinct original length sequences.
            && self.id_to_token.iter().map(String::len)
                .eq(other.id_to_token.iter().map(String::len))
            && self.ignore_merges == other.ignore_merges
            && self.byte_fallback == other.byte_fallback
    }
}

#[cfg(test)]
mod tests;
