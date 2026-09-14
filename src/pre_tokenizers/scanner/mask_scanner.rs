//! SIMD boundary walkers for fixed BPE tokenizer grammars.
//! Derived from Gigatoken's MIT-licensed o200k scanner; scalar scanning remains exact.

use crate::pre_tokenizers::FusedPieceSink;

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx512_scanner_available() -> bool {
    std::arch::is_x86_feature_detected!("avx512f")
        && std::arch::is_x86_feature_detected!("avx512bw")
        && std::arch::is_x86_feature_detected!("avx512vl")
        && std::arch::is_x86_feature_detected!("bmi1")
        && std::arch::is_x86_feature_detected!("bmi2")
        && std::arch::is_x86_feature_detected!("lzcnt")
        && std::arch::is_x86_feature_detected!("popcnt")
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_scanner_available() -> bool {
    std::arch::is_x86_feature_detected!("avx2")
        && std::arch::is_x86_feature_detected!("bmi1")
        && std::arch::is_x86_feature_detected!("bmi2")
        && std::arch::is_x86_feature_detected!("lzcnt")
        && std::arch::is_x86_feature_detected!("popcnt")
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn simd_scanner_available() -> bool {
    avx512_scanner_available() || avx2_scanner_available()
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
fn simd_scanner_available() -> bool {
    cfg!(target_arch = "aarch64")
}

// Target-feature sets must match the runtime guards above.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn movemask64(
    v0: std::arch::aarch64::uint8x16_t,
    v1: std::arch::aarch64::uint8x16_t,
    v2: std::arch::aarch64::uint8x16_t,
    v3: std::arch::aarch64::uint8x16_t,
) -> u64 {
    use std::arch::aarch64::*;
    unsafe {
        let magic = vld1q_u32([0xf0f1_f3f8, 0x0f1f_3f80, 0xf0f1_f3f8, 0x0f1f_3f80].as_ptr());
        let first = vpaddq_u32(
            vmulq_u32(vreinterpretq_u32_u8(v0), magic),
            vmulq_u32(vreinterpretq_u32_u8(v1), magic),
        );
        let second = vpaddq_u32(
            vmulq_u32(vreinterpretq_u32_u8(v2), magic),
            vmulq_u32(vreinterpretq_u32_u8(v3), magic),
        );
        let tables = uint8x16x2_t(vreinterpretq_u8_u32(first), vreinterpretq_u8_u32(second));
        let indices = vld1_u8([3, 7, 11, 15, 19, 23, 27, 31].as_ptr());
        let packed = vqtbl2_u8(tables, indices);
        vget_lane_u64::<0>(vreinterpret_u64_u8(packed))
    }
}

#[derive(Clone, Copy, Default)]
struct AsciiMasks {
    pub l: u64,
    pub d: u64,
    pub s: u64,
    pub wt: u64,
    pub n: u64,
    pub hi: u64,
    pub ap: u64,
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx512vl,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
fn ascii_masks_avx512(bytes: &[u8], scan: usize) -> AsciiMasks {
    use std::arch::x86_64::*;
    unsafe {
        let v = _mm512_loadu_si512(bytes.as_ptr().add(scan) as *const _);
        let lowered = _mm512_or_si512(v, _mm512_set1_epi8(0x20));
        let l = _mm512_cmple_epu8_mask(
            _mm512_sub_epi8(lowered, _mm512_set1_epi8(b'a' as i8)),
            _mm512_set1_epi8(25),
        );
        let d = _mm512_cmple_epu8_mask(
            _mm512_sub_epi8(v, _mm512_set1_epi8(b'0' as i8)),
            _mm512_set1_epi8(9),
        );
        let s = _mm512_cmpeq_epi8_mask(v, _mm512_set1_epi8(b' ' as i8));
        let n = _mm512_cmpeq_epi8_mask(v, _mm512_set1_epi8(b'\r' as i8))
            | _mm512_cmpeq_epi8_mask(v, _mm512_set1_epi8(b'\n' as i8));
        let wt =
            _mm512_cmple_epu8_mask(_mm512_sub_epi8(v, _mm512_set1_epi8(9)), _mm512_set1_epi8(4))
                & !n;
        let hi = _mm512_movepi8_mask(v) as u64;
        let ap = _mm512_cmpeq_epi8_mask(v, _mm512_set1_epi8(b'\'' as i8));
        AsciiMasks {
            l,
            d,
            s,
            wt,
            n,
            hi,
            ap,
        }
    }
}

/// AVX2 classifier for one guarded 64-byte batch.
/// `#[inline(never)]` keeps the mask algebra out of the vector domain.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
#[inline(never)]
fn ascii_masks_avx2(bytes: &[u8], scan: usize) -> AsciiMasks {
    use std::arch::x86_64::*;
    unsafe {
        let le =
            |v: __m256i, lim: __m256i| -> __m256i { _mm256_cmpeq_epi8(_mm256_min_epu8(v, lim), v) };
        let mm = |m0: __m256i, m1: __m256i| -> u64 {
            (_mm256_movemask_epi8(m0) as u32 as u64)
                | ((_mm256_movemask_epi8(m1) as u32 as u64) << 32)
        };

        let p = bytes.as_ptr().add(scan);
        let v0 = _mm256_loadu_si256(p as *const _);
        let v1 = _mm256_loadu_si256(p.add(32) as *const _);

        let x20 = _mm256_set1_epi8(0x20);
        let ca = _mm256_set1_epi8(b'a' as i8);
        let c25 = _mm256_set1_epi8(25);
        let l = mm(
            le(_mm256_sub_epi8(_mm256_or_si256(v0, x20), ca), c25),
            le(_mm256_sub_epi8(_mm256_or_si256(v1, x20), ca), c25),
        );
        let c0 = _mm256_set1_epi8(b'0' as i8);
        let c9 = _mm256_set1_epi8(9);
        let d = mm(
            le(_mm256_sub_epi8(v0, c0), c9),
            le(_mm256_sub_epi8(v1, c0), c9),
        );
        let sp = _mm256_set1_epi8(b' ' as i8);
        let s = mm(_mm256_cmpeq_epi8(v0, sp), _mm256_cmpeq_epi8(v1, sp));
        let cr = _mm256_set1_epi8(b'\r' as i8);
        let lf = _mm256_set1_epi8(b'\n' as i8);
        let n = mm(
            _mm256_or_si256(_mm256_cmpeq_epi8(v0, cr), _mm256_cmpeq_epi8(v0, lf)),
            _mm256_or_si256(_mm256_cmpeq_epi8(v1, cr), _mm256_cmpeq_epi8(v1, lf)),
        );
        let c4 = _mm256_set1_epi8(4);
        let wt = mm(
            le(_mm256_sub_epi8(v0, c9), c4),
            le(_mm256_sub_epi8(v1, c9), c4),
        ) & !n;
        let hi = mm(v0, v1); // vpmovmskb takes the sign bit directly
        let apc = _mm256_set1_epi8(b'\'' as i8);
        let ap = mm(_mm256_cmpeq_epi8(v0, apc), _mm256_cmpeq_epi8(v1, apc));
        AsciiMasks {
            l,
            d,
            s,
            wt,
            n,
            hi,
            ap,
        }
    }
}

#[inline(always)]
unsafe fn nn_at_full(bytes: &[u8], index: usize) -> bool {
    let byte = bytes[index];
    if byte < 0x80 {
        return !is_ascii_ws(byte);
    }
    let (codepoint, _) = unsafe { decode_cp_inbounds(bytes, index) };
    mask_class_of(codepoint) != MaskCharClass::Whitespace
}

#[inline(always)]
fn digit_run_splits3(d: u64) -> u64 {
    let mut b = d & !(d << 1); // run starts
    let mut c = d & (d >> 1) & (d >> 2) & (d >> 3);
    let mut sh = 3u32;
    while sh < 64 {
        b |= (b & c) << sh;
        c &= c >> sh;
        sh <<= 1;
    }
    b
}

trait MaskScheme {
    fn advance(bytes: &[u8], pos: usize) -> usize;

    /// Returns disjoint trusted-start and scalar-fallback masks for a guarded batch.
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64);
}

/// Scheme-agnostic state that interleaves trusted masks with scalar fallback.
struct MaskState {
    pub pos: usize,
    scan: usize,
    mask_base: usize,
    rem: u64,
    batch_usable: u64,
    batch_bad: u64,
    scalar_until: usize,
    pre_base: usize,
    pre_usable: u64,
    pre_bad: u64,
}

impl MaskState {
    #[inline]
    fn new(pos: usize) -> Self {
        let scalar_until = if simd_scanner_available() {
            pos
        } else {
            usize::MAX
        };
        Self {
            pos,
            scan: pos,
            mask_base: pos,
            rem: 0,
            batch_usable: 0,
            batch_bad: 0,
            scalar_until,
            pre_base: usize::MAX,
            pre_usable: 0,
            pre_bad: 0,
        }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn load_segment(&mut self, from_bit: u32) {
        let live = u64::MAX << from_bit;
        let seg_bad = self.batch_bad & live;
        if seg_bad == 0 {
            self.rem = self.batch_usable & live;
            self.batch_bad = 0;
        } else {
            let nb = seg_bad.trailing_zeros();
            self.rem = self.batch_usable & live & ((1u64 << nb) - 1);
            let rest = self.batch_usable & (u64::MAX << nb);
            self.scalar_until = if rest != 0 {
                self.mask_base + rest.trailing_zeros() as usize
            } else {
                self.mask_base + 64
            };
        }
        let at_start = self.pos == self.mask_base + from_bit as usize;
        self.rem &= !(u64::from(at_start) << from_bit);
    }

    #[inline(always)]
    fn for_each_span<S: MaskScheme>(&mut self, input: &str, sink: &mut impl FusedPieceSink) {
        let bytes = input.as_bytes();
        let len = bytes.len();
        loop {
            if self.rem != 0 {
                let rem = std::mem::take(&mut self.rem);
                // SAFETY: mask bits are exact UTF-8 token boundaries.
                unsafe { sink.push_mask(input, self.mask_base, &mut self.pos, rem) };
            }
            while self.pos < self.scalar_until {
                if self.pos >= len {
                    return;
                }
                let start = self.pos;
                let end = S::advance(bytes, start);
                self.pos = end;
                // SAFETY: `advance` returns the next exact UTF-8 boundary.
                unsafe { sink.push_piece(input, start, end) };
            }
            #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
            {
                if self.batch_bad != 0 && self.pos < self.mask_base + 64 {
                    self.load_segment((self.pos - self.mask_base) as u32);
                    continue;
                }
                self.batch_bad = 0;
                // Resume on the grid; `from_bit` masks stale run-internal bits.
                while self.scan + 64 <= self.pos {
                    self.scan += 64;
                }
                if self.scan + 70 > len {
                    self.scalar_until = usize::MAX;
                    continue;
                }
                let (usable, bad) = if self.pre_base == self.scan {
                    (self.pre_usable, self.pre_bad)
                } else {
                    S::batch_masks(bytes, self.scan)
                };
                self.mask_base = self.scan;
                self.scan += 64;
                self.batch_usable = usable;
                self.batch_bad = bad;
                if self.scan + 70 <= len {
                    let (u2, b2) = S::batch_masks(bytes, self.scan);
                    self.pre_base = self.scan;
                    self.pre_usable = u2;
                    self.pre_bad = b2;
                } else {
                    self.pre_base = usize::MAX;
                }
                if self.pos > self.mask_base {
                    self.load_segment((self.pos - self.mask_base) as u32);
                } else {
                    self.load_segment(0);
                }
            }
            #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
            {
                self.scalar_until = usize::MAX;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum MaskCharClass {
    Upper,
    Lower,
    Caseless,
    Mark,
    Number,
    Whitespace,
    Other,
}

const MASK_HAN: u8 = 1 << 3;

trait MaskFlavor {
    const CONTRACTIONS: bool;
    const THREE_DIGIT_NUMBERS: bool;
    const SLASH_PUNCT_TAIL: bool;
    const SIMD_UNICODE: bool;
}

fn build_mask_class_table() -> Box<[u8]> {
    use icu_properties::{
        CodePointMapData, CodePointSetData,
        props::{GeneralCategory, GeneralCategoryGroup, Script, WhiteSpace},
    };

    let mut classes = vec![MaskCharClass::Other as u8; 0x110000];
    let categories = CodePointMapData::<GeneralCategory>::new();
    for (category, class) in [
        (GeneralCategory::UppercaseLetter, MaskCharClass::Upper),
        (GeneralCategory::TitlecaseLetter, MaskCharClass::Upper),
        (GeneralCategory::LowercaseLetter, MaskCharClass::Lower),
        (GeneralCategory::ModifierLetter, MaskCharClass::Caseless),
        (GeneralCategory::OtherLetter, MaskCharClass::Caseless),
    ] {
        for range in categories.iter_ranges_for_value(category) {
            classes[*range.start() as usize..=*range.end() as usize].fill(class as u8);
        }
    }
    for (group, class) in [
        (GeneralCategoryGroup::Mark, MaskCharClass::Mark),
        (GeneralCategoryGroup::Number, MaskCharClass::Number),
    ] {
        for range in categories.iter_ranges_for_group(group) {
            classes[*range.start() as usize..=*range.end() as usize].fill(class as u8);
        }
    }
    for range in CodePointSetData::new::<WhiteSpace>().iter_ranges() {
        classes[*range.start() as usize..=*range.end() as usize]
            .fill(MaskCharClass::Whitespace as u8);
    }
    for range in CodePointMapData::<Script>::new().iter_ranges_for_value(Script::Han) {
        for class in &mut classes[*range.start() as usize..=*range.end() as usize] {
            *class |= MASK_HAN;
        }
    }
    // Unicode has an even number of scalar values, so every class byte forms one packed pair.
    classes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| pair[0] | pair[1] << 4)
        .collect()
}

static MASK_CLASS_TABLE: std::sync::LazyLock<Box<[u8]>> =
    std::sync::LazyLock::new(build_mask_class_table);

#[derive(Clone, Copy)]
struct MaskClassTable(&'static [u8]);

impl MaskClassTable {
    #[inline]
    fn get() -> Self {
        Self(&MASK_CLASS_TABLE)
    }

    #[inline(always)]
    fn tag_of(self, codepoint: u32) -> u8 {
        let packed = unsafe { *self.0.get_unchecked((codepoint >> 1) as usize) };
        packed >> ((codepoint & 1) << 2) & 0x0f
    }

    #[inline(always)]
    fn class_and_han(self, codepoint: u32) -> (MaskCharClass, bool) {
        let tag = self.tag_of(codepoint);
        let class = match tag & !MASK_HAN {
            0 => MaskCharClass::Upper,
            1 => MaskCharClass::Lower,
            2 => MaskCharClass::Caseless,
            3 => MaskCharClass::Mark,
            4 => MaskCharClass::Number,
            5 => MaskCharClass::Whitespace,
            _ => MaskCharClass::Other,
        };
        (class, tag & MASK_HAN != 0)
    }

    #[inline(always)]
    fn class_of(self, codepoint: u32) -> MaskCharClass {
        self.class_and_han(codepoint).0
    }

    #[inline(always)]
    fn is_han(self, codepoint: u32) -> bool {
        // Dense unified-ideograph blocks avoid a dependent table load on ordinary Han text.
        matches!(codepoint, 0x3400..=0x4dbf | 0x4e00..=0x9fff)
            || self.tag_of(codepoint) & MASK_HAN != 0
    }
}

#[inline(always)]
fn mask_class_of(codepoint: u32) -> MaskCharClass {
    MaskClassTable::get().class_of(codepoint)
}

#[inline(always)]
fn is_letter(byte: u8) -> bool {
    (byte | 0x20).wrapping_sub(b'a') < 26
}

#[inline(always)]
fn is_digit(byte: u8) -> bool {
    byte.wrapping_sub(b'0') < 10
}

#[inline(always)]
fn is_ascii_ws(byte: u8) -> bool {
    byte == b' ' || byte.wrapping_sub(9) < 5
}

#[inline(always)]
fn is_upper_ascii(byte: u8) -> bool {
    byte.wrapping_sub(b'A') < 26
}

#[inline(always)]
fn is_tail_byte<F: MaskFlavor>(byte: u8) -> bool {
    matches!(byte, b'\r' | b'\n') || (F::SLASH_PUNCT_TAIL && byte == b'/')
}

#[inline(always)]
unsafe fn decode_cp_inbounds(bytes: &[u8], pos: usize) -> (u32, usize) {
    unsafe {
        let first = *bytes.get_unchecked(pos) as u32;
        let second = (*bytes.get_unchecked(pos + 1) & 0x3f) as u32;
        if first < 0xe0 {
            return (((first & 0x1f) << 6) | second, 2);
        }
        let third = (*bytes.get_unchecked(pos + 2) & 0x3f) as u32;
        if first < 0xf0 {
            return (((first & 0x0f) << 12) | (second << 6) | third, 3);
        }
        let fourth = (*bytes.get_unchecked(pos + 3) & 0x3f) as u32;
        (
            ((first & 7) << 18) | (second << 12) | (third << 6) | fourth,
            4,
        )
    }
}

#[inline(always)]
unsafe fn decode_cp(bytes: &[u8], pos: usize) -> (u32, usize) {
    if pos + 4 <= bytes.len() {
        return unsafe { decode_cp_inbounds(bytes, pos) };
    }
    let text = unsafe { std::str::from_utf8_unchecked(&bytes[pos..]) };
    let ch = unsafe { text.chars().next().unwrap_unchecked() };
    (ch as u32, ch.len_utf8())
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn is_direct_kimi_han_start(bytes: &[u8], pos: usize) -> bool {
    match bytes[pos] {
        0xe3 => bytes[pos + 1] >= 0x90,
        0xe4 => bytes[pos + 1] != 0xb7,
        0xe5..=0xe9 => true,
        _ => false,
    }
}

/// Skips ten direct-Han scalars at a time after a scalar start was confirmed.
///
/// The UTF-8 layout places one second byte across the 128-bit lane boundary,
/// so the vector test covers the other nine starts and the scalar predicate
/// handles that one byte pair. A false result leaves the whole block for the
/// existing exact decoder.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn skip_direct_kimi_han_blocks_avx2(bytes: &[u8], mut end: usize) -> usize {
    use std::arch::x86_64::*;

    const SELECT: u32 = 0x000f_001f;
    // The first lane supplies starts 0, 3, 6, 9, 12; the second supplies
    // 18, 21, 24, 27. Start 15 crosses lanes, so it stays scalar below.
    let starts = _mm256_setr_epi8(
        0, 3, 6, 9, 12, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, 2, 5, 8,
        11, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    );
    let seconds = _mm256_setr_epi8(
        1, 4, 7, 10, 13, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, 3, 6, 9,
        12, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    );
    let sign = _mm256_set1_epi8(-128);
    let e3 = _mm256_set1_epi8(0xe3_u8 as i8);
    let e4 = _mm256_set1_epi8(0xe4_u8 as i8);
    let e4_unsigned = _mm256_set1_epi8((0xe4_u8 ^ 0x80) as i8);
    let ea_unsigned = _mm256_set1_epi8((0xea_u8 ^ 0x80) as i8);
    let b7 = _mm256_set1_epi8(0xb7_u8 as i8);
    let eight_f = _mm256_set1_epi8((0x8f_u8 ^ 0x80) as i8);

    while end + 32 <= bytes.len() {
        let block = unsafe { _mm256_loadu_si256(bytes.as_ptr().add(end) as *const _) };
        let first = _mm256_shuffle_epi8(block, starts);
        let second = _mm256_shuffle_epi8(block, seconds);
        let first_unsigned = _mm256_xor_si256(first, sign);
        let second_unsigned = _mm256_xor_si256(second, sign);
        let e5_through_e9 = _mm256_and_si256(
            _mm256_cmpgt_epi8(first_unsigned, e4_unsigned),
            _mm256_cmpgt_epi8(ea_unsigned, first_unsigned),
        );
        let e3_direct = _mm256_and_si256(
            _mm256_cmpeq_epi8(first, e3),
            _mm256_cmpgt_epi8(second_unsigned, eight_f),
        );
        let e4_direct =
            _mm256_andnot_si256(_mm256_cmpeq_epi8(second, b7), _mm256_cmpeq_epi8(first, e4));
        let direct = _mm256_or_si256(e5_through_e9, _mm256_or_si256(e3_direct, e4_direct));
        let all_vector_direct = _mm256_movemask_epi8(direct) as u32 & SELECT == SELECT;
        if !all_vector_direct || !is_direct_kimi_han_start(bytes, end + 15) {
            break;
        }
        end += 30;
    }
    end
}

#[inline(always)]
fn scan_kimi_han_run(bytes: &[u8], pos: usize) -> Option<usize> {
    if bytes[pos] < 0x80 {
        return None;
    }
    let classes = MaskClassTable::get();
    // SAFETY: mask schemes receive bytes from the caller's valid UTF-8 `str`.
    let (codepoint, length) = unsafe { decode_cp(bytes, pos) };
    if !classes.is_han(codepoint) {
        return None;
    }

    let mut end = pos + length;
    #[cfg(target_arch = "x86_64")]
    if end + 32 <= bytes.len() && std::arch::is_x86_feature_detected!("avx2") {
        // SAFETY: the runtime guard proves AVX2; the helper checks its 32-byte load bound.
        end = unsafe { skip_direct_kimi_han_blocks_avx2(bytes, end) };
    }
    while end < bytes.len() && bytes[end] >= 0x80 {
        // SAFETY: `end` advances only by decoded scalar lengths from valid UTF-8.
        let (codepoint, length) = unsafe { decode_cp(bytes, end) };
        if !classes.is_han(codepoint) {
            break;
        }
        end += length;
    }
    Some(end)
}

#[inline(always)]
fn scan_kimi_punctuation_run(bytes: &[u8], pos: usize) -> Option<usize> {
    let classes = MaskClassTable::get();
    let first = bytes[pos];
    let first_end = if first < 0x80 {
        if is_letter(first) || is_digit(first) || is_ascii_ws(first) {
            return None;
        }
        pos + 1
    } else {
        // SAFETY: mask schemes receive bytes from the caller's valid UTF-8 `str`.
        let (codepoint, length) = unsafe { decode_cp(bytes, pos) };
        let (class, han) = classes.class_and_han(codepoint);
        if class != MaskCharClass::Other || han {
            return None;
        }
        pos + length
    };

    if first_end < bytes.len() {
        let next = bytes[first_end];
        let starts_word = if next < 0x80 {
            is_letter(next)
        } else {
            // SAFETY: `first_end` is the boundary after one valid scalar.
            let (codepoint, _) = unsafe { decode_cp(bytes, first_end) };
            let (class, han) = classes.class_and_han(codepoint);
            !han && matches!(
                class,
                MaskCharClass::Upper
                    | MaskCharClass::Lower
                    | MaskCharClass::Caseless
                    | MaskCharClass::Mark
            )
        };
        // One punctuation scalar may be the optional prefix of a word alternative.
        if starts_word {
            return None;
        }
    }

    let mut end = first_end;
    while end < bytes.len() {
        let byte = bytes[end];
        if byte < 0x80 {
            if is_letter(byte) || is_digit(byte) || is_ascii_ws(byte) {
                break;
            }
            end += 1;
            continue;
        }
        // SAFETY: `end` advances only by decoded scalar lengths from valid UTF-8.
        let (codepoint, length) = unsafe { decode_cp(bytes, end) };
        let class = classes.class_of(codepoint);
        if !matches!(class, MaskCharClass::Other | MaskCharClass::Mark) {
            break;
        }
        end += length;
    }
    while matches!(bytes.get(end), Some(b'\r' | b'\n')) {
        end += 1;
    }
    Some(end)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
unsafe fn char_through_mask(bytes: &[u8], pos: usize) -> (MaskCharClass, usize, usize) {
    let byte = bytes[pos - 1];
    if byte < 0x80 {
        let class = if is_upper_ascii(byte) {
            MaskCharClass::Upper
        } else if is_letter(byte) {
            MaskCharClass::Lower
        } else if is_digit(byte) {
            MaskCharClass::Number
        } else if is_ascii_ws(byte) {
            MaskCharClass::Whitespace
        } else {
            MaskCharClass::Other
        };
        return (class, pos - 1, pos);
    }
    let mut lead = pos - 1;
    while lead > 0 && bytes[lead] & 0xc0 == 0x80 {
        lead -= 1;
    }
    let (codepoint, length) = unsafe { decode_cp_inbounds(bytes, lead) };
    (mask_class_of(codepoint), lead, lead + length)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn r50k_is_ws_at(bytes: &[u8], index: usize) -> bool {
    let byte = bytes[index];
    if byte < 0x80 {
        return is_ascii_ws(byte);
    }
    let (codepoint, _) = unsafe { decode_cp_inbounds(bytes, index) };
    mask_class_of(codepoint) == MaskCharClass::Whitespace
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn r50k_algebra(
    bytes: &[u8],
    scan: usize,
    masks: AsciiMasks,
    unicode: OUni,
    carries: (u64, u64, u64, u64, u64),
) -> (u64, u64) {
    let ascii_ws = masks.s | masks.wt | masks.n;
    let letter = masks.l | unicode.l;
    let digit = masks.d | unicode.n;
    let whitespace = ascii_ws | unicode.ws;
    let other = !(masks.l | masks.d | ascii_ws | masks.hi) | unicode.o;
    let (previous_letter, previous_digit, previous_space, previous_ws, previous_other) = carries;

    let same_run = (letter & ((letter << 1) | previous_letter))
        | (digit & ((digit << 1) | previous_digit))
        | (other & ((other << 1) | previous_other));
    let after_space = (masks.s << 1) | previous_space;
    let ordinary = !whitespace & !same_run & !after_space & !unicode.cont;

    // Lead-length masks test the scalar after each whitespace character.
    let non_ws = !whitespace;
    let mut split =
        (ascii_ws & (non_ws >> 1)) | (unicode.w2 & (non_ws >> 2)) | (unicode.w3 & (non_ws >> 3));
    let edge_multi = (unicode.w2 & (1 << 62)) | (unicode.w3 & (1 << 61));
    let next = bytes[scan + 64];
    if next < 0x80 && edge_multi == 0 {
        split = (split & !(1 << 63)) | ((u64::from(!is_ascii_ws(next)) << 63) & ascii_ws);
    } else {
        let edge = edge_multi | ((1 << 63) & ascii_ws);
        if edge != 0 {
            if !r50k_is_ws_at(bytes, scan + 64) {
                split |= edge;
            } else {
                split &= !edge;
            }
        }
    }
    let previous_ws = (whitespace << 1) | previous_ws;
    let whitespace_boundary = (ascii_ws | unicode.w2 | unicode.w3) & (!previous_ws | split);
    let mut boundary = ordinary | whitespace_boundary;
    let mut bad = unicode.resid | (unicode.resid << 1) | (unicode.resid >> 1);

    let mut contractions = masks.ap & boundary & !bad;
    while contractions != 0 {
        let index = contractions.trailing_zeros() as usize;
        contractions &= contractions - 1;
        if index >= 61 {
            bad |= u64::MAX << index;
            break;
        }
        let length = match bytes[scan + index + 1] {
            b's' | b'd' | b'm' | b't' => 2,
            b'l' if bytes[scan + index + 2] == b'l' => 3,
            b'v' if bytes[scan + index + 2] == b'e' => 3,
            b'r' if bytes[scan + index + 2] == b'e' => 3,
            _ => 0,
        };
        if length != 0 {
            boundary &= !(1u64 << (index + 1));
            boundary |= 1u64 << (index + length);
        }
    }
    (boundary & !bad, bad)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "bmi1,bmi2,lzcnt,popcnt")
)]
#[inline(never)]
fn r50k_extended_masks(bytes: &[u8], scan: usize, masks: AsciiMasks) -> (u64, u64) {
    let mut claimed = OUni::default();
    let carries = if scan == 0 {
        (0, 0, 0, 0, 0)
    } else if bytes[scan - 1] < 0x80 {
        let byte = bytes[scan - 1];
        let letter = u64::from(is_letter(byte));
        let digit = u64::from(is_digit(byte));
        let whitespace = u64::from(is_ascii_ws(byte));
        (
            letter,
            digit,
            u64::from(byte == b' '),
            whitespace,
            u64::from(letter == 0 && digit == 0 && whitespace == 0),
        )
    } else {
        let (class, _, end) = unsafe { char_through_mask(bytes, scan) };
        // Claim continuation bytes from the scalar that straddles this block.
        let bytes_in_batch = if end > scan {
            (1u64 << (end - scan)) - 1
        } else {
            0
        };
        claimed.cont = bytes_in_batch;
        match class {
            MaskCharClass::Upper | MaskCharClass::Lower | MaskCharClass::Caseless => {
                claimed.l = bytes_in_batch;
                (1, 0, 0, 0, 0)
            }
            MaskCharClass::Number => {
                claimed.n = bytes_in_batch;
                (0, 1, 0, 0, 0)
            }
            MaskCharClass::Whitespace => {
                claimed.ws = bytes_in_batch;
                claimed.resid = bytes_in_batch;
                (0, 0, 0, 1, 0)
            }
            MaskCharClass::Mark | MaskCharClass::Other => {
                claimed.o = bytes_in_batch;
                (0, 0, 0, 0, 1)
            }
        }
    };

    let mut unicode = unsafe { classify_uni_mask::<false>(bytes, scan, masks.hi & !claimed.cont) };
    unicode.l |= claimed.l;
    unicode.n |= claimed.n;
    unicode.o |= claimed.o;
    unicode.ws |= claimed.ws;
    unicode.cont |= claimed.cont;
    unicode.resid |= claimed.resid;
    r50k_algebra(bytes, scan, masks, unicode, carries)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "bmi1,bmi2,lzcnt,popcnt")
)]
#[cfg_attr(target_arch = "aarch64", inline(always))]
#[cfg_attr(target_arch = "x86_64", inline)]
fn r50k_masks(bytes: &[u8], scan: usize, masks: AsciiMasks) -> (u64, u64) {
    if masks.hi != 0 || (scan != 0 && bytes[scan - 1] >= 0x80) {
        return r50k_extended_masks(bytes, scan, masks);
    }
    let carries = if scan == 0 {
        (0, 0, 0, 0, 0)
    } else {
        let byte = bytes[scan - 1];
        let letter = u64::from(is_letter(byte));
        let digit = u64::from(is_digit(byte));
        let whitespace = u64::from(is_ascii_ws(byte));
        (
            letter,
            digit,
            u64::from(byte == b' '),
            whitespace,
            u64::from(letter == 0 && digit == 0 && whitespace == 0),
        )
    };
    r50k_algebra(bytes, scan, masks, OUni::default(), carries)
}

#[cfg(target_arch = "aarch64")]
#[inline]
fn r50k_batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
    use std::arch::aarch64::*;
    debug_assert!(scan + 70 <= bytes.len());
    unsafe {
        let pointer = bytes.as_ptr().add(scan);
        let zero = vdupq_n_u8(0);
        let mut letters = [zero; 4];
        let mut digits = [zero; 4];
        let mut spaces = [zero; 4];
        let mut whitespace = [zero; 4];
        let mut apostrophes = [zero; 4];
        let mut high = [zero; 4];
        for index in 0..4 {
            let value = vld1q_u8(pointer.add(index * 16));
            let lower = vorrq_u8(value, vdupq_n_u8(0x20));
            letters[index] = vcleq_u8(vsubq_u8(lower, vdupq_n_u8(b'a')), vdupq_n_u8(25));
            digits[index] = vcleq_u8(vsubq_u8(value, vdupq_n_u8(b'0')), vdupq_n_u8(9));
            spaces[index] = vceqq_u8(value, vdupq_n_u8(b' '));
            whitespace[index] = vorrq_u8(
                spaces[index],
                vcleq_u8(vsubq_u8(value, vdupq_n_u8(9)), vdupq_n_u8(4)),
            );
            apostrophes[index] = vceqq_u8(value, vdupq_n_u8(b'\''));
            high[index] = vcltzq_s8(vreinterpretq_s8_u8(value));
        }
        let letter = movemask64(letters[0], letters[1], letters[2], letters[3]);
        let digit = movemask64(digits[0], digits[1], digits[2], digits[3]);
        let space = movemask64(spaces[0], spaces[1], spaces[2], spaces[3]);
        let ws = movemask64(whitespace[0], whitespace[1], whitespace[2], whitespace[3]);
        let high = movemask64(high[0], high[1], high[2], high[3]);
        let apostrophe_any = vorrq_u8(
            vorrq_u8(apostrophes[0], apostrophes[1]),
            vorrq_u8(apostrophes[2], apostrophes[3]),
        );
        let apostrophe = if vmaxvq_u8(apostrophe_any) == 0 {
            0
        } else {
            movemask64(
                apostrophes[0],
                apostrophes[1],
                apostrophes[2],
                apostrophes[3],
            )
        };
        r50k_masks(
            bytes,
            scan,
            AsciiMasks {
                l: letter,
                d: digit,
                s: space,
                wt: ws & !space,
                n: 0,
                hi: high,
                ap: apostrophe,
            },
        )
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn r50k_batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
    debug_assert!(simd_scanner_available());
    if avx512_scanner_available() {
        return unsafe { r50k_batch_masks_avx512(bytes, scan) };
    }
    unsafe { r50k_batch_masks_avx2(bytes, scan) }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx512vl,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
unsafe fn r50k_batch_masks_avx512(bytes: &[u8], scan: usize) -> (u64, u64) {
    let masks = ascii_masks_avx512(bytes, scan);
    r50k_masks(bytes, scan, masks)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
unsafe fn r50k_batch_masks_avx2(bytes: &[u8], scan: usize) -> (u64, u64) {
    let masks = ascii_masks_avx2(bytes, scan);
    r50k_masks(bytes, scan, masks)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn qwen_ascii_carries(bytes: &[u8], scan: usize) -> OCarries {
    if scan == 0 {
        return OCarries::default();
    }
    let previous = bytes[scan - 1];
    let letter = is_letter(previous);
    let digit = is_digit(previous);
    let whitespace = is_ascii_ws(previous);
    let previous_two = scan >= 2
        && (bytes[scan - 2] == b' '
            || (!is_letter(bytes[scan - 2])
                && !is_digit(bytes[scan - 2])
                && !is_ascii_ws(bytes[scan - 2])));
    OCarries {
        pl: u64::from(letter),
        ps: u64::from(previous == b' '),
        pwt: u64::from(whitespace && !matches!(previous, b' ' | b'\r' | b'\n')),
        po: u64::from(!letter && !digit && !whitespace),
        pws: u64::from(whitespace),
        pd: u64::from(digit),
        c2_os: u64::from(previous_two),
        ..OCarries::default()
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn qwen_algebra<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
    masks: AsciiMasks,
    carries: OCarries,
    mut unicode: OUni,
) -> (u64, u64) {
    if WORD_MARKS {
        unicode.l |= unicode.mk;
        unicode.o &= !unicode.mk;
    }

    let letter = masks.l | unicode.l;
    let whitespace_tail = masks.wt | unicode.ws;
    let other = !(masks.l | masks.d | masks.s | masks.wt | masks.n | masks.hi) | unicode.o;
    let whitespace = masks.s | whitespace_tail | masks.n;

    // An optional punctuation prefix is absorbed only when it did not
    // itself follow punctuation or a literal space.
    let one_byte = !(unicode.cont | unicode.lead2 | unicode.lead3 | unicode.lead4);
    let previous_other_or_space = ((other | masks.s) << 1) | carries.po | carries.ps;
    let two_back = ((previous_other_or_space & one_byte) << 1)
        | ((previous_other_or_space & unicode.lead2) << 2)
        | ((previous_other_or_space & unicode.lead3) << 3)
        | ((previous_other_or_space & unicode.lead4) << 4)
        | carries.c2_os
        | carries.b2b_in;
    let previous_letter = (letter << 1) | carries.pl;
    let previous_space = (masks.s << 1) | carries.ps;
    let previous_tail_ws = (whitespace_tail << 1) | carries.pwt;
    let previous_other = (other << 1) | carries.po;
    let absorbed_prefix = previous_other & !two_back;
    let letter_boundaries = letter
        & !unicode.cont
        & !previous_letter
        & !previous_space
        & !previous_tail_ws
        & !absorbed_prefix;

    let digit_boundaries = if DIGITS3 && masks.d & (masks.d >> 1) != 0 {
        digit_run_splits3(masks.d)
    } else if DIGITS3 {
        masks.d
    } else {
        masks.d | (unicode.n & !unicode.cont)
    };
    let punctuation_boundaries = other & !unicode.cont & !previous_other & !previous_space;

    // CR/LF directly after punctuation remains part of that punctuation token.
    let absorbed_newline_seed = masks.n & ((other << 1) | carries.po);
    let absorbed_newlines = if absorbed_newline_seed == 0 {
        0
    } else {
        smear_up(absorbed_newline_seed, masks.n)
    };
    let effective_whitespace = whitespace & !absorbed_newlines;
    let mut bad = unicode.resid | (unicode.resid << 1) | (unicode.resid >> 1);

    // Resolve whitespace that reaches byte 63 only when the following
    // scalar proves the run ends at this block boundary.
    let next = bytes[scan + 64];
    let next_non_whitespace = if next < 0x80 {
        !is_ascii_ws(next)
    } else {
        bad >> 63 == 0 && !r50k_is_ws_at(bytes, scan + 64)
    };
    if absorbed_newlines >> 63 != 0 && !next_non_whitespace {
        bad |= 1 << 63;
    }
    let non_whitespace = !effective_whitespace;
    if effective_whitespace >> 63 != 0 && !next_non_whitespace {
        if non_whitespace == 0 {
            return (0, u64::MAX);
        }
        let highest_non_whitespace = 63 - non_whitespace.leading_zeros();
        bad |= u64::MAX << (highest_non_whitespace + 1);
    }

    // Three-character number groups crossing a scalar-only zone must
    // preserve their phase in the scalar fallback.
    if DIGITS3 {
        let seed = (masks.d & (bad << 1)) | (masks.d & carries.pd);
        if seed != 0 {
            bad |= smear_up(seed, masks.d);
        }
    }

    let whitespace_leads = (masks.s | masks.wt | masks.n | unicode.w2 | unicode.w3)
        & effective_whitespace
        & !absorbed_newlines;
    let previous_whitespace = (effective_whitespace << 1) | carries.pws;
    let edge_last = ((masks.s | masks.wt | masks.n) & (1 << 63))
        | (unicode.w2 & (1 << 62))
        | (unicode.w3 & (1 << 61));
    let next_non_whitespace_mask = u64::from(next_non_whitespace).wrapping_neg();
    let split_before_last = ((masks.s | masks.wt | masks.n) & (non_whitespace >> 1))
        | (unicode.w2 & (non_whitespace >> 2))
        | (unicode.w3 & (non_whitespace >> 3))
        | (edge_last & next_non_whitespace_mask);
    let mut whitespace_boundaries = whitespace_leads & (!previous_whitespace | split_before_last);

    // A whitespace run containing newlines ends at its last newline;
    // any remaining whitespace becomes a separate tail token.
    let mut newline_runs = masks.n & effective_whitespace & !bad;
    while newline_runs != 0 {
        let first_newline = newline_runs.trailing_zeros();
        let gap_below = non_whitespace & ((1u64 << first_newline) - 1);
        let run_start = if gap_below == 0 {
            0
        } else {
            64 - gap_below.leading_zeros()
        };
        let run_end = (non_whitespace & (u64::MAX << first_newline)).trailing_zeros();
        let run = (u64::MAX << run_start) & !u64::MAX.unbounded_shl(run_end);
        whitespace_boundaries &= !run;
        whitespace_boundaries |= 1 << run_start;
        let last_newline = 63 - (masks.n & run).leading_zeros();
        if last_newline + 1 < run_end {
            whitespace_boundaries |= 1 << (last_newline + 1);
            let tail = whitespace_leads & run & (u64::MAX << (last_newline + 1));
            whitespace_boundaries |= 1 << (63 - tail.leading_zeros());
        }
        newline_runs &= !run;
    }

    let mut boundaries =
        letter_boundaries | digit_boundaries | punctuation_boundaries | whitespace_boundaries;

    // Move boundaries across the fixed case-insensitive contraction suffixes.
    let mut contractions = masks.ap & boundaries & !bad;
    while contractions != 0 {
        let index = contractions.trailing_zeros() as usize;
        contractions &= contractions - 1;
        if index >= 61 {
            bad |= u64::MAX << index;
            break;
        }
        let first = bytes[scan + index + 1];
        if first >= 0x80 {
            bad |= 0b111 << index;
            continue;
        }
        let length = match first | 0x20 {
            b's' | b'd' | b'm' | b't' => 2,
            b'l' if bytes[scan + index + 2] | 0x20 == b'l' => 3,
            b'v' if bytes[scan + index + 2] | 0x20 == b'e' => 3,
            b'r' if bytes[scan + index + 2] | 0x20 == b'e' => 3,
            _ => 0,
        };
        if length != 0 {
            boundaries &= !(1 << (index + 1));
            boundaries |= 1 << (index + length);
        }
    }

    (boundaries & !bad, bad)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "bmi1,bmi2,lzcnt,popcnt")
)]
#[inline(never)]
fn qwen_extended_masks<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
    masks: AsciiMasks,
) -> (u64, u64) {
    let mut claimed = OUni::default();
    let carries = if scan == 0 {
        OCarries::default()
    } else if bytes[scan - 1] < 0x80 && (scan < 2 || bytes[scan - 2] < 0x80) {
        qwen_ascii_carries(bytes, scan)
    } else {
        // Claim any scalar that starts before this block and ends inside it.
        let (class, lead, end) = unsafe { char_through_mask(bytes, scan) };
        let claimed_bytes = if end > scan {
            (1 << (end - scan)) - 1
        } else {
            0
        };
        claimed.cont = claimed_bytes;
        let previous_two_other_or_space = if lead == 0 {
            false
        } else if bytes[lead - 1] == b' ' {
            true
        } else {
            let previous_two = unsafe { char_through_mask(bytes, lead) }.0;
            matches!(previous_two, MaskCharClass::Other)
                || (!WORD_MARKS && previous_two == MaskCharClass::Mark)
        };
        let mut carries = OCarries {
            c2_os: u64::from(end <= scan && previous_two_other_or_space),
            b2b_in: if end > scan {
                u64::from(previous_two_other_or_space) << (end - scan)
            } else {
                0
            },
            pd: u64::from(class == MaskCharClass::Number),
            ..OCarries::default()
        };
        match class {
            MaskCharClass::Upper | MaskCharClass::Lower | MaskCharClass::Caseless => {
                claimed.l = claimed_bytes;
                carries.pl = 1;
            }
            MaskCharClass::Mark if WORD_MARKS => {
                claimed.l = claimed_bytes;
                carries.pl = 1;
            }
            MaskCharClass::Mark | MaskCharClass::Other => {
                claimed.o = claimed_bytes;
                carries.po = 1;
            }
            MaskCharClass::Number => {
                claimed.n = claimed_bytes;
                claimed.resid = claimed_bytes;
            }
            MaskCharClass::Whitespace => {
                claimed.ws = claimed_bytes;
                claimed.resid = claimed_bytes;
                let previous_byte = bytes[scan - 1];
                carries.ps = u64::from(previous_byte == b' ');
                carries.pwt = u64::from(
                    previous_byte >= 0x80 || !matches!(previous_byte, b' ' | b'\r' | b'\n'),
                );
                carries.pws = 1;
            }
        }
        carries
    };

    let mut unicode =
        unsafe { classify_uni_mask::<DIGITS3>(bytes, scan, masks.hi & !claimed.cont) };
    unicode.l |= claimed.l;
    unicode.n |= claimed.n;
    unicode.o |= claimed.o;
    unicode.ws |= claimed.ws;
    unicode.cont |= claimed.cont;
    unicode.resid |= claimed.resid;
    qwen_algebra::<WORD_MARKS, DIGITS3>(bytes, scan, masks, carries, unicode)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "bmi1,bmi2,lzcnt,popcnt")
)]
#[cfg_attr(target_arch = "aarch64", inline(always))]
#[cfg_attr(target_arch = "x86_64", inline)]
fn qwen_masks<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
    masks: AsciiMasks,
) -> (u64, u64) {
    if masks.hi != 0
        || (scan >= 1 && bytes[scan - 1] >= 0x80)
        || (scan >= 2 && bytes[scan - 2] >= 0x80)
    {
        return qwen_extended_masks::<WORD_MARKS, DIGITS3>(bytes, scan, masks);
    }
    qwen_algebra::<WORD_MARKS, DIGITS3>(
        bytes,
        scan,
        masks,
        qwen_ascii_carries(bytes, scan),
        OUni::default(),
    )
}

#[cfg(target_arch = "aarch64")]
#[inline]
fn qwen_batch_masks<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
) -> (u64, u64) {
    use std::arch::aarch64::*;
    debug_assert!(scan + 70 <= bytes.len());
    unsafe {
        let pointer = bytes.as_ptr().add(scan);
        let zero = vdupq_n_u8(0);
        let mut letters = [zero; 4];
        let mut digits = [zero; 4];
        let mut spaces = [zero; 4];
        let mut whitespace = [zero; 4];
        let mut newlines = [zero; 4];
        let mut apostrophes = [zero; 4];
        let mut high = [zero; 4];
        for index in 0..4 {
            let value = vld1q_u8(pointer.add(index * 16));
            let lower = vorrq_u8(value, vdupq_n_u8(0x20));
            letters[index] = vcleq_u8(vsubq_u8(lower, vdupq_n_u8(b'a')), vdupq_n_u8(25));
            digits[index] = vcleq_u8(vsubq_u8(value, vdupq_n_u8(b'0')), vdupq_n_u8(9));
            spaces[index] = vceqq_u8(value, vdupq_n_u8(b' '));
            whitespace[index] = vorrq_u8(
                spaces[index],
                vcleq_u8(vsubq_u8(value, vdupq_n_u8(9)), vdupq_n_u8(4)),
            );
            newlines[index] = vorrq_u8(
                vceqq_u8(value, vdupq_n_u8(b'\r')),
                vceqq_u8(value, vdupq_n_u8(b'\n')),
            );
            apostrophes[index] = vceqq_u8(value, vdupq_n_u8(b'\''));
            high[index] = vcltzq_s8(vreinterpretq_s8_u8(value));
        }
        let letter = movemask64(letters[0], letters[1], letters[2], letters[3]);
        let digit = movemask64(digits[0], digits[1], digits[2], digits[3]);
        let space = movemask64(spaces[0], spaces[1], spaces[2], spaces[3]);
        let ws = movemask64(whitespace[0], whitespace[1], whitespace[2], whitespace[3]);
        let newline = movemask64(newlines[0], newlines[1], newlines[2], newlines[3]);
        let high = movemask64(high[0], high[1], high[2], high[3]);
        let apostrophe_any = vorrq_u8(
            vorrq_u8(apostrophes[0], apostrophes[1]),
            vorrq_u8(apostrophes[2], apostrophes[3]),
        );
        let apostrophe = if vmaxvq_u8(apostrophe_any) == 0 {
            0
        } else {
            movemask64(
                apostrophes[0],
                apostrophes[1],
                apostrophes[2],
                apostrophes[3],
            )
        };
        qwen_masks::<WORD_MARKS, DIGITS3>(
            bytes,
            scan,
            AsciiMasks {
                l: letter,
                d: digit,
                s: space,
                wt: ws & !space & !newline,
                n: newline,
                hi: high,
                ap: apostrophe,
            },
        )
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn qwen_batch_masks<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
) -> (u64, u64) {
    debug_assert!(simd_scanner_available());
    if avx512_scanner_available() {
        return unsafe { qwen_batch_masks_avx512::<WORD_MARKS, DIGITS3>(bytes, scan) };
    }
    unsafe { qwen_batch_masks_avx2::<WORD_MARKS, DIGITS3>(bytes, scan) }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx512vl,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
unsafe fn qwen_batch_masks_avx512<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
) -> (u64, u64) {
    qwen_masks::<WORD_MARKS, DIGITS3>(bytes, scan, ascii_masks_avx512(bytes, scan))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
unsafe fn qwen_batch_masks_avx2<const WORD_MARKS: bool, const DIGITS3: bool>(
    bytes: &[u8],
    scan: usize,
) -> (u64, u64) {
    qwen_masks::<WORD_MARKS, DIGITS3>(bytes, scan, ascii_masks_avx2(bytes, scan))
}

// Mask-scanner boundary algebra

/// Smear `seed` upward (toward higher bits) through contiguous set bits of
/// `within`, in log steps.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn smear_up(seed: u64, within: u64) -> u64 {
    let mut a = seed;
    let mut m = within;
    let mut sh = 1u32;
    while sh < 64 {
        a |= (a << sh) & m;
        m &= m << sh;
        sh <<= 1;
    }
    a
}

/// Per-byte class masks for a batch's Unicode chars under the shared mask
/// classifier — the mask-scanner analogue of [`UniClasses`], with the
/// case-split letter masks the scheme needs. Every byte of a classified
/// char carries the char's class, so byte-adjacency == char-adjacency.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
struct OUni {
    /// All letter-run bytes (upper + lower + caseless; marks excluded —
    /// they are contextual and deferred via `mk`).
    l: u64,
    u: u64,
    cl: u64,
    n: u64,
    o: u64,
    ws: u64,
    w2: u64,
    w3: u64,
    lead2: u64,
    lead3: u64,
    lead4: u64,
    cont: u64,
    /// Bytes only the scalar path can decide (±1 bad smear): number chars
    /// when their scheme groups by count, whitespace straddling the batch
    /// end, and stray continuation bytes.
    resid: u64,
    /// Mark bytes (±4 bad smear). A mark's run-contextual class can
    /// affect boundaries up to two CHARS after it, which multi-byte
    /// followers can push past the 4-byte smear; those stragglers are
    /// wrongly-cleared bits (extending the scalar walk) or wrongly-set
    /// bits interior to a token starting inside the zone, both killed by
    /// MaskState's resume masking after the scalar overrun — the same
    /// invariant the resid zones rely on.
    mk: u64,
}

/// Boundary carries from the chars before the batch (mask-scanner variant of the
/// cl100k family's `Carries`, plus the case and absorbed-tail bits).
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
struct OCarries {
    /// P1 is a letter / strict-upper / caseless / space (0x20) /
    /// non-newline non-space ws / punct / any ws / digit.
    pl: u64,
    pu: u64,
    pcl: u64,
    ps: u64,
    pwt: u64,
    po: u64,
    pws: u64,
    pd: u64,
    c2_os: u64,
    b2b_in: u64,
    /// P1 is an absorbed `[\r\n/]*` tail byte whose token may continue
    /// into this batch (seeds the tail smear at bit 0).
    p_abs: bool,
    /// The tail walkback could not resolve (pathological run): the batch's
    /// leading tail-class run must be a bad zone.
    force_bad_lead: bool,
}

/// Was the tail-class byte at `scan - 1` absorbed by a punct run's
/// `[\r\n/]*` tail (as opposed to being a fresh punct-run `/` or a
/// ws-run newline)? Walks the tail-class run back (bounded) and
/// classifies the byte before it. `None`: unresolved (over-long run, or a
/// preceding mark whose own class is run-contextual).
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn prev_tail_absorbed<F: MaskFlavor>(bytes: &[u8], scan: usize) -> Option<bool> {
    debug_assert!(scan >= 1 && is_tail_byte::<F>(bytes[scan - 1]));
    let mut r = scan - 1;
    let mut steps = 0;
    while r > 0 && is_tail_byte::<F>(bytes[r - 1]) {
        r -= 1;
        steps += 1;
        if steps > 8 {
            return None;
        }
    }
    // T-run = bytes[r..scan]. The `[\r\n/]*` tail is greedy, so once
    // absorption triggers — at the first newline that directly follows a
    // punct-run char (an in-run slash, or the pre-run char for a
    // run-leading newline) — everything to the run's end is absorbed.
    // Before the trigger, newlines are ws-run members and slashes are
    // ordinary punct-run bytes.
    let run = &bytes[r..scan];
    let mut trigger = usize::MAX;
    let mut seen_slash = false;
    for (j, &b) in run.iter().enumerate() {
        if b == b'/' {
            seen_slash = true;
            continue;
        }
        if seen_slash {
            trigger = j;
            break;
        }
        if j == 0 {
            // Run-leading newline: the pre-run char decides.
            if r == 0 {
                continue;
            }
            let pb = bytes[r - 1];
            let pred_punct = if pb < 0x80 {
                if !is_letter(pb) && !is_digit(pb) && !is_ascii_ws(pb) {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                let mut k = r - 1;
                while k > 0 && bytes[k] & 0xC0 == 0x80 {
                    k -= 1;
                }
                let (cp, _) = unsafe { decode_cp(bytes, k) };
                match mask_class_of(cp) {
                    MaskCharClass::Other => Some(true),
                    // A mark continues whatever run precedes it.
                    MaskCharClass::Mark => None,
                    _ => Some(false),
                }
            };
            match pred_punct {
                Some(true) => {
                    trigger = 0;
                    break;
                }
                Some(false) => {}
                None => return None,
            }
        }
    }
    Some(scan - 1 - r >= trigger)
}

/// Two-back "punct or space" test (`c2_os`) for the ASCII byte at `idx`.
/// A slash may be an absorbed tail byte — a token end, neither punct-run
/// member nor space — so it resolves through the walkback. `None`:
/// unresolved (callers set `force_bad_lead`).
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn c2_os_ascii<F: MaskFlavor>(bytes: &[u8], idx: usize) -> Option<u64> {
    let b2 = bytes[idx];
    if F::SLASH_PUNCT_TAIL && b2 == b'/' {
        return prev_tail_absorbed::<F>(bytes, idx + 1).map(|abs| u64::from(!abs));
    }
    Some(u64::from(
        b2 == b' ' || (!is_letter(b2) && !is_digit(b2) && !is_ascii_ws(b2)),
    ))
}

/// Pure-ASCII carries. Requires `scan > 0`, `bytes[scan-1] < 0x80` (and
/// `bytes[scan-2] < 0x80` when present), and `bytes[scan-1]` NOT a
/// tail-class byte (those route through [`prev_tail_absorbed`] first).
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn ascii_carries<F: MaskFlavor>(bytes: &[u8], scan: usize) -> OCarries {
    let b = bytes[scan - 1];
    debug_assert!(!is_tail_byte::<F>(b));
    let bit = |c: bool| u64::from(c);
    let (l, d, w) = (is_letter(b), is_digit(b), is_ascii_ws(b));
    let (c2_os, c2_unresolved) = if scan >= 2 {
        match c2_os_ascii::<F>(bytes, scan - 2) {
            Some(v) => (v, false),
            None => (0, true),
        }
    } else {
        (0, false)
    };
    OCarries {
        force_bad_lead: c2_unresolved,
        pl: bit(l),
        pu: bit(is_upper_ascii(b)),
        pcl: 0,
        ps: bit(b == b' '),
        pwt: bit(w && b != b' '), // \r\n excluded by the debug_assert
        po: bit(!l && !d && !w),
        pws: bit(w),
        pd: bit(d),
        c2_os,
        b2b_in: 0,
        p_abs: false,
    }
}

/// Carries when the byte before the batch is tail-class (`[\r\n/]`):
/// resolves absorbed-tail vs fresh-run via [`prev_tail_absorbed`]. An
/// absorbed tail ended the previous token, so every "P1 is X" carry is
/// zero and only the tail-continuation seed survives.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(never)]
fn tail_carries<F: MaskFlavor>(bytes: &[u8], scan: usize) -> OCarries {
    match prev_tail_absorbed::<F>(bytes, scan) {
        Some(true) => OCarries {
            p_abs: true,
            ..OCarries::default()
        },
        Some(false) => {
            let b = bytes[scan - 1];
            let bit = |c: bool| u64::from(c);
            // A fresh '/' is an ordinary punct byte; \r\n are newlines
            // (ws class, po = 0). c2_os as in ascii_carries when P2 is
            // ASCII; a non-ASCII P2 next to a tail byte is rare — defer.
            if scan >= 2 && bytes[scan - 2] >= 0x80 {
                return OCarries {
                    force_bad_lead: true,
                    ..OCarries::default()
                };
            }
            let c2_os = if scan >= 2 {
                let b2 = bytes[scan - 2];
                bit(b2 == b' ' || (!is_letter(b2) && !is_digit(b2) && !is_ascii_ws(b2)))
            } else {
                0
            };
            OCarries {
                po: bit(b == b'/'),
                pws: bit(b != b'/'),
                c2_os,
                ..OCarries::default()
            }
        }
        None => OCarries {
            force_bad_lead: true,
            ..OCarries::default()
        },
    }
}

/// Classify every unicode char whose lead bit is in `m` for
/// `bytes[scan..scan+64]` with the scanner classifier — the mask-scanner analogue
/// of [`classify_uni_chars`] (NUMBERS = false, LEADS = true), with
/// case-split letter masks and marks deferred via `mk`.
///
/// # Safety
///
/// `scan + 70 <= bytes.len()` (the batch classifiers' lookahead guard).
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
unsafe fn classify_uni_mask<const DEFER_NUMBERS: bool>(
    bytes: &[u8],
    scan: usize,
    mut m: u64,
) -> OUni {
    let classes = MaskClassTable::get();
    let mut u = OUni::default();
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        m &= m - 1;
        let b = bytes[scan + i];
        if b < 0xE0 {
            // Two-byte UTF-8 dominates western Unicode text, so keep its
            // decode and constant-width masks off the general length ladder.
            if b < 0xC2 {
                u.resid |= 1 << i; // stray continuation byte (invalid UTF-8)
                continue;
            }
            let lead = 1u64 << i;
            let chm = 3u64 << i; // in-batch bytes (excess drops at bit 63)
            // SAFETY: the function's lookahead contract covers i + 1.
            let b1 = unsafe { *bytes.get_unchecked(scan + i + 1) };
            let cp = ((b as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F);
            match classes.class_of(cp) {
                MaskCharClass::Upper => {
                    u.l |= chm;
                    u.u |= chm;
                }
                MaskCharClass::Lower => u.l |= chm,
                MaskCharClass::Caseless => {
                    u.l |= chm;
                    u.cl |= chm;
                }
                MaskCharClass::Mark => {
                    u.o |= chm;
                    u.mk |= chm;
                }
                MaskCharClass::Number => {
                    u.n |= chm;
                    if DEFER_NUMBERS {
                        u.resid |= chm;
                    }
                }
                MaskCharClass::Other => u.o |= chm,
                MaskCharClass::Whitespace => {
                    u.ws |= chm;
                    if i + 2 > 64 {
                        u.resid |= chm;
                    } else {
                        u.w2 |= lead;
                    }
                }
            }
            u.lead2 |= lead;
            u.cont |= chm & !lead;
            m &= !chm;
            continue;
        }
        let l = if b < 0xF0 { 3 } else { 4 };
        let chm = ((1u64 << l) - 1) << i; // in-batch bytes (excess drops)
        let lead = 1u64 << i;
        // SAFETY: scan + 70 <= len (this fn's contract), i <= 63, so
        // scan + i + 4 <= len even for a 4-byte lead at bit 63.
        let (cp, _) = unsafe { decode_cp_inbounds(bytes, scan + i) };
        match classes.class_of(cp) {
            MaskCharClass::Upper => {
                u.l |= chm;
                u.u |= chm;
            }
            MaskCharClass::Lower => u.l |= chm,
            MaskCharClass::Caseless => {
                u.l |= chm;
                u.cl |= chm;
            }
            MaskCharClass::Mark => {
                // Contextual (letter-run joiner AND punct-run member):
                // punct-class for the neighbors' mask algebra, deferred
                // (±4) for everything the context could change.
                u.o |= chm;
                u.mk |= chm;
            }
            MaskCharClass::Number => {
                u.n |= chm;
                if DEFER_NUMBERS {
                    u.resid |= chm;
                }
            }
            MaskCharClass::Other => u.o |= chm,
            MaskCharClass::Whitespace => {
                u.ws |= chm;
                if i + l > 64 || l == 4 {
                    u.resid |= chm; // straddling-out ws stays a bad zone
                } else {
                    u.w3 |= lead;
                }
            }
        }
        if l == 3 {
            u.lead3 |= lead;
        } else {
            u.lead4 |= lead;
        }
        u.cont |= chm & !lead;
        m &= !chm;
    }
    u
}

/// ASCII class masks the shared mask algebra needs on top of
/// [`AsciiMasks`]: strict-uppercase letters and slashes.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
struct OAsciiExtra {
    up: u64,
    sl: u64,
}

/// Slow(er) path for batches with non-ASCII in or just before them — the
/// mask-scanner analogue of `cl100k_family::family_extended_masks`: carries walk
/// back through multi-byte chars with the scanner classifier, unicode chars
/// join the effective class masks, then the shared algebra applies.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "bmi1,bmi2,lzcnt,popcnt")
)]
#[inline(never)]
fn extended_masks<F: MaskFlavor>(
    bytes: &[u8],
    scan: usize,
    am: AsciiMasks,
    ax: OAsciiExtra,
) -> (u64, u64) {
    let mut cl = OUni::default();
    let cr = if scan == 0 {
        OCarries::default()
    } else if bytes[scan - 1] < 0x80 && is_tail_byte::<F>(bytes[scan - 1]) {
        tail_carries::<F>(bytes, scan)
    } else if bytes[scan - 1] < 0x80 && (scan < 2 || bytes[scan - 2] < 0x80) {
        ascii_carries::<F>(bytes, scan)
    } else {
        // A multi-byte char within two bytes of the batch start.
        // SAFETY: scan > 0, and the batch guard covers pos + 3 <= len.
        let (c1, j1, e1) = unsafe { char_through_mask(bytes, scan) };
        let chm = if e1 > scan {
            (1u64 << (e1 - scan)) - 1
        } else {
            0
        };
        cl.cont = chm;
        let (c2v, c2_defer) = if j1 == 0 {
            (0, false)
        } else if F::SLASH_PUNCT_TAIL && bytes[j1 - 1] == b'/' {
            // An absorbed tail slash is a token end, not a punct-run char.
            match prev_tail_absorbed::<F>(bytes, j1) {
                Some(abs) => (u64::from(!abs), false),
                None => (0, true),
            }
        } else {
            // SAFETY: j1 > 0, j1 < scan keeps the decode in the guard.
            let c2c = unsafe { char_through_mask(bytes, j1) }.0;
            (
                u64::from(
                    bytes[j1 - 1] == b' '
                        || matches!(c2c, MaskCharClass::Other | MaskCharClass::Mark),
                ),
                // A mark P2 makes the two-back test run-contextual.
                c2c == MaskCharClass::Mark,
            )
        };
        let mut c = OCarries::default();
        if e1 > scan {
            c.b2b_in = c2v << (e1 - scan);
        } else {
            c.c2_os = c2v;
        }
        if c2_defer {
            c.force_bad_lead = true;
        }
        c.pd = u64::from(c1 == MaskCharClass::Number);
        match c1 {
            MaskCharClass::Upper => {
                cl.l |= chm;
                cl.u |= chm;
                c.pl = 1;
                c.pu = 1;
            }
            MaskCharClass::Lower => {
                cl.l |= chm;
                c.pl = 1;
            }
            MaskCharClass::Caseless => {
                cl.l |= chm;
                cl.cl |= chm;
                c.pl = 1;
                c.pcl = 1;
            }
            MaskCharClass::Mark => {
                // Contextual: defer the batch front to the scalar path.
                cl.o |= chm;
                cl.mk |= chm | 1; // bit 0 seeds the ±4 smear even when
                // the mark sits entirely before the batch
                c.po = 1;
            }
            MaskCharClass::Number => {
                cl.n |= chm;
                // A digit char straddling INTO the batch: the leading
                // ASCII digit run's `\p{N}{1,3}` phase started before the
                // batch, and the `pd` seed below can't see it (bit 0 is a
                // continuation byte, not an ASCII digit). Defer via resid
                // so the bad<<1 seed catches the run.
                cl.resid |= chm;
            }
            MaskCharClass::Other => {
                cl.o |= chm;
                c.po = 1;
            }
            MaskCharClass::Whitespace => {
                cl.ws |= chm;
                if e1 > scan {
                    cl.resid |= chm;
                }
                let pb = bytes[scan - 1];
                c.ps = u64::from(pb == b' ');
                let nl = pb == b'\r' || pb == b'\n';
                c.pwt = u64::from(pb < 0x80 && pb != b' ' && !nl || pb >= 0x80);
                c.pws = 1;
            }
        }
        c
    };

    let mut uni = if am.hi != 0 {
        // SAFETY: the batch guard is exactly `classify_uni_mask`'s
        // contract.
        unsafe { classify_uni_mask::<true>(bytes, scan, am.hi & !cl.cont) }
    } else {
        OUni::default()
    };
    uni.l |= cl.l;
    uni.u |= cl.u;
    uni.cl |= cl.cl;
    uni.n |= cl.n;
    uni.o |= cl.o;
    uni.ws |= cl.ws;
    uni.cont |= cl.cont;
    uni.resid |= cl.resid;
    uni.mk |= cl.mk;

    mask_algebra::<F>(bytes, scan, am, ax, cr, uni)
}

/// The shared u64 boundary algebra over per-byte class
/// masks — `cl100k_family::family_algebra` with the fixed-grammar rules: casing
/// boundaries inside letter runs, suffix contractions, `[\r\n/]*` punct
/// tails. `uni` is all-zero on the pure-ASCII path.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn mask_algebra<F: MaskFlavor>(
    bytes: &[u8],
    scan: usize,
    am: AsciiMasks,
    ax: OAsciiExtra,
    cr: OCarries,
    uni: OUni,
) -> (u64, u64) {
    let OCarries {
        pl,
        pu,
        pcl,
        ps,
        pwt,
        po,
        pws,
        pd,
        c2_os,
        b2b_in,
        p_abs,
        force_bad_lead,
    } = cr;
    let contm = uni.cont;

    // Effective per-byte classes.
    let lb = am.l | uni.l;
    let ub = ax.up | uni.u;
    let clb = uni.cl;
    let sb = am.s;
    let wtb = am.wt | uni.ws;
    let ob = !(am.l | am.d | am.s | am.wt | am.n | am.hi) | uni.o;
    let ws_all = sb | wtb | am.n;

    // Seed: a newline right after a punct byte (a slash after a punct run
    // is already a run member, so tails always begin with a newline), or
    // a tail continuing from before the batch. Smear through [\r\n/].
    let tcls = if F::SLASH_PUNCT_TAIL {
        am.n | ax.sl
    } else {
        am.n
    };
    let abs_seed = (am.n & ((ob << 1) | po)) | (u64::from(p_abs) & tcls);
    let abs_t = if abs_seed == 0 {
        0
    } else {
        smear_up(abs_seed, tcls)
    };
    // Absorbed bytes are no longer punct-run members for any boundary rule.
    let ob_eff = ob & !abs_t;

    let len1 = !(contm | uni.lead2 | uni.lead3 | uni.lead4);
    let c_test = ((ob_eff | sb) << 1) | po | ps; // bit 0: byte scan-1 in O|S
    let b2back = ((c_test & len1) << 1)
        | ((c_test & uni.lead2) << 2)
        | ((c_test & uni.lead3) << 3)
        | ((c_test & uni.lead4) << 4)
        | c2_os
        | b2b_in;
    let p_l = (lb << 1) | pl;
    let p_u = (ub << 1) | pu;
    let p_cl = (clb << 1) | pcl;
    let p_s = (sb << 1) | ps;
    let p_wt = (wtb << 1) | pwt;
    let p_o = (ob_eff << 1) | po;
    let absorb = p_o & !b2back;
    // Casing boundary: a strict-upper char after a strict-lower one. (For
    // ASCII text this is the whole rule; see the module docs.)
    let p_sl = p_l & !p_u & !p_cl;
    let b_su = ub & !contm & p_sl;
    let b_letters = (lb & !contm & !p_l & !p_s & !p_wt & !absorb) | b_su;

    let b_digits = if F::THREE_DIGIT_NUMBERS && am.d & (am.d >> 1) != 0 {
        digit_run_splits3(am.d)
    } else {
        am.d
    };

    let b_punct = ob_eff & !contm & !p_o & !p_s;

    let resid = uni.resid;
    let mut bad = resid | resid << 1 | resid >> 1;
    // Marks are run-contextual: they, and anything whose boundary rules
    // can see them (up to two chars back — 4 bytes of lookahead for the
    // following leads), go to the scalar path.
    let mk = uni.mk;
    if mk != 0 {
        bad |= mk | mk << 1 | mk << 2 | mk << 3 | mk << 4 | mk >> 1;
    }
    // A strict-upper char after a caseless letter: phase- and
    // lookahead-dependent (see the module docs) — scalar.
    bad |= ub & !contm & ((clb << 1) | pcl);
    if force_bad_lead {
        // Unresolved carries: the leading tail-class run (plus the byte
        // after it) can't be trusted.
        bad |= smear_up(tcls & 1, tcls) << 1 | 0b11;
    }

    let ws_eff = ws_all & !abs_t;

    // Byte-64 lookahead: is the char at the next batch's first byte
    // non-ws? (See cl100k_family for the full reasoning.)
    let nb64 = bytes[scan + 64]; // in bounds: scan + 70 <= len
    let nn64 = if nb64 < 0x80 {
        !is_ascii_ws(nb64)
    } else {
        // SAFETY: the batch guard puts the decode at scan + 64 in bounds.
        bad >> 63 == 0 && unsafe { nn_at_full(bytes, scan + 64) }
    };
    let nn64m = u64::from(nn64).wrapping_neg();

    // An absorbed tail touching the batch end continues iff byte 64 is
    // tail-class; the next batch's `tail_carries` walkback re-derives the
    // context either way, so nothing defers here. A ws run touching the
    // batch end still defers when byte 64 is ws (its last newline may lie
    // beyond this batch).
    let nonws = !ws_eff;
    if ws_eff >> 63 != 0 && !nn64 {
        if nonws == 0 {
            return (0, u64::MAX); // whole batch one ws run
        }
        let h = 63 - nonws.leading_zeros(); // highest non-ws bit (< 63)
        bad |= u64::MAX << (h + 1);
    }

    // A digit run whose `\p{N}{1,3}` phase did not start inside this batch
    // (continuation from before it, or after a bad zone) defers.
    if F::THREE_DIGIT_NUMBERS {
        let seed = (am.d & (bad << 1)) | (am.d & pd);
        if seed != 0 {
            bad |= smear_up(seed, am.d);
        }
    }

    // Base rule (NL-free runs; NL runs are overridden below).
    let ws_leads1 = (am.s | am.wt | am.n) & ws_eff;
    let ws_leads = (ws_leads1 | uni.w2 | uni.w3) & !abs_t;
    let p_ws = (ws_eff << 1) | pws;
    let edge_last = (ws_leads1 & (1 << 63)) | (uni.w2 & (1 << 62)) | (uni.w3 & (1 << 61));
    let split_ok = (ws_leads1 & (nonws >> 1))
        | (uni.w2 & (nonws >> 2))
        | (uni.w3 & (nonws >> 3))
        | (edge_last & nn64m);
    let mut b_ws = ws_leads & (!p_ws | split_ok);

    // Override every run containing a (non-absorbed) newline: one token
    // through the run's last newline, then tail rules.
    let mut runs_n = am.n & ws_eff & !bad;
    while runs_n != 0 {
        let f = runs_n.trailing_zeros();
        let below_gap = nonws & ((1u64 << f) - 1);
        let a = if below_gap == 0 {
            0
        } else {
            64 - below_gap.leading_zeros()
        };
        let e = (nonws & (u64::MAX << f)).trailing_zeros();
        let run_mask = (u64::MAX << a) & !u64::MAX.unbounded_shl(e);
        b_ws &= !run_mask;
        b_ws |= 1u64 << a;
        let q = 63 - (am.n & run_mask).leading_zeros(); // last NL in run
        if (q + 1) < e {
            b_ws |= 1u64 << (q + 1);
            let tail = run_mask & (u64::MAX << (q + 1));
            let tail_leads = ws_leads & tail;
            b_ws |= 1u64 << (63 - tail_leads.leading_zeros());
        }
        runs_n &= !run_mask;
    }

    let mut boundary = b_letters | b_digits | b_punct | b_ws;

    // An apostrophe right after a letter-run char merges the suffix into
    // that token and forces a boundary right after it. ('ſ is non-ASCII:
    // an apostrophe before any non-ASCII char defers.)
    if F::CONTRACTIONS {
        let mut cand = am.ap & boundary & p_l & !bad;
        let mut last_forced = usize::MAX;
        while cand != 0 {
            let i = cand.trailing_zeros() as usize;
            cand &= cand - 1;
            if i <= 2 {
                // The preceding letter could itself end an earlier
                // contraction that started before the batch — scalar.
                bad |= 0b111u64 << i;
                continue;
            }
            if i >= 61 {
                bad |= u64::MAX << i;
                break;
            }
            if i == last_forced {
                // "x'll'd": the letter before this apostrophe is a
                // consumed suffix's last char; a new (prefix) match
                // starts here instead.
                continue;
            }
            // The letter before this apostrophe may itself be a consumed
            // suffix's last char resolved where last_forced can't see it
            // (a scalar-walked zone like 'ſ, or a fixup before the
            // batch): locally ambiguous, defer.
            let p = scan + i;
            let prev_suffix_possible = (bytes[p - 2] == b'\''
                && matches!(bytes[p - 1] | 0x20, b's' | b'd' | b'm' | b't'))
                || (p >= 3
                    && bytes[p - 3] == b'\''
                    && (matches!(
                        (bytes[p - 2] | 0x20, bytes[p - 1] | 0x20),
                        (b'l', b'l') | (b'v', b'e') | (b'r', b'e')
                    ) || (bytes[p - 2] == 0xC5 && bytes[p - 1] == 0xBF)));
            if prev_suffix_possible {
                bad |= 0b111u64 << i;
                continue;
            }
            let b1 = bytes[scan + i + 1];
            if b1 >= 0x80 {
                bad |= 0b111u64 << i;
                continue;
            }
            let k = match b1 | 0x20 {
                b's' | b'd' | b'm' | b't' => 2,
                b'l' if bytes[scan + i + 2] | 0x20 == b'l' => 3,
                b'v' if bytes[scan + i + 2] | 0x20 == b'e' => 3,
                b'r' if bytes[scan + i + 2] | 0x20 == b'e' => 3,
                _ => 0,
            };
            if k != 0 {
                boundary &= !(1u64 << i);
                boundary &= !(((1u64 << (k - 1)) - 1) << (i + 1));
                boundary |= 1u64 << (i + k);
                last_forced = i + k;
            }
        }
    }

    (boundary & !bad, bad)
}

// Batch classifiers (per-arch front-ends)

/// Carries for a batch known to have only ASCII in and just before it:
/// tail-class prev bytes route through the walkback, everything else
/// through the branchless ASCII carries.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline(always)]
fn ascii_batch_carries<F: MaskFlavor>(bytes: &[u8], scan: usize) -> OCarries {
    if scan == 0 {
        OCarries::default()
    } else if is_tail_byte::<F>(bytes[scan - 1]) {
        tail_carries::<F>(bytes, scan)
    } else {
        ascii_carries::<F>(bytes, scan)
    }
}

/// `(usable, bad)` for `bytes[scan..scan+64]` under the fixed-grammar
/// rules — same contract as the cl100k family's `batch_masks`.
///
/// NEON front-end: classifies the ASCII classes (letter, upper, digit,
/// space, whitespace, newline) with movemasks; apostrophe and slash sit
/// behind horizontal any-tests. Batches with non-ASCII in or just before
/// them take [`extended_masks`].
#[cfg(target_arch = "aarch64")]
#[inline]
fn batch_masks<F: MaskFlavor>(bytes: &[u8], scan: usize) -> (u64, u64) {
    use std::arch::aarch64::*;
    debug_assert!(scan + 70 <= bytes.len());
    unsafe {
        let p = bytes.as_ptr().add(scan);
        let zero = vdupq_n_u8(0);
        let mut vs = [zero; 4];
        let mut hi_any = zero;
        for (i, v) in vs.iter_mut().enumerate() {
            *v = vld1q_u8(p.add(16 * i));
            hi_any = vorrq_u8(hi_any, *v);
        }

        // A flavor without SIMD Unicode support hands every Unicode-adjacent
        // batch to its exact scalar scanner, so test the bail condition before
        // the seven class chains it would otherwise discard. Flavors with
        // SIMD Unicode keep the test after classification, where its latency
        // overlaps the class work.
        if !F::SIMD_UNICODE
            && (vmaxvq_u8(hi_any) >= 0x80
                || (scan >= 1 && bytes[scan - 1] >= 0x80)
                || (scan >= 2 && bytes[scan - 2] >= 0x80))
        {
            return (0, u64::MAX);
        }

        let mut lv = [zero; 4];
        let mut casev = [zero; 4];
        let mut dv = [zero; 4];
        let mut sv = [zero; 4];
        let mut wsv = [zero; 4];
        let mut nv = [zero; 4];
        let mut apv = [zero; 4];
        let newline_table =
            vld1q_u8([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0, 0, 0xff, 0, 0].as_ptr());
        for i in 0..4 {
            let v = vs[i];
            let lowered = vorrq_u8(v, vdupq_n_u8(0x20));
            lv[i] = vcleq_u8(vsubq_u8(lowered, vdupq_n_u8(b'a')), vdupq_n_u8(25));
            casev[i] = vtstq_u8(v, vdupq_n_u8(0x20));
            dv[i] = vcleq_u8(vsubq_u8(v, vdupq_n_u8(b'0')), vdupq_n_u8(9));
            sv[i] = vceqq_u8(v, vdupq_n_u8(b' '));
            wsv[i] = vorrq_u8(sv[i], vcleq_u8(vsubq_u8(v, vdupq_n_u8(9)), vdupq_n_u8(4)));
            // `tbl` zeroes indices above 15, so only CR and LF select set lanes.
            nv[i] = vqtbl1q_u8(newline_table, v);
            apv[i] = vceqq_u8(v, vdupq_n_u8(b'\''));
        }
        let l64 = movemask64(lv[0], lv[1], lv[2], lv[3]);
        let bit5 = movemask64(casev[0], casev[1], casev[2], casev[3]);
        // ASCII lowercase letters set bit 5; `l64` excludes every other byte.
        let u64_ = l64 & !bit5;
        let d64 = movemask64(dv[0], dv[1], dv[2], dv[3]);
        let wsa = movemask64(wsv[0], wsv[1], wsv[2], wsv[3]);
        // Space is the only byte in the ASCII whitespace set with bit 5 set.
        let s64 = wsa & bit5;
        let n64 = movemask64(nv[0], nv[1], nv[2], nv[3]);

        // Apostrophes only matter for the contraction fixup.
        let ap64 = if F::CONTRACTIONS {
            let ap_any = vorrq_u8(vorrq_u8(apv[0], apv[1]), vorrq_u8(apv[2], apv[3]));
            if vmaxvq_u8(ap_any) != 0 {
                movemask64(apv[0], apv[1], apv[2], apv[3])
            } else {
                0
            }
        } else {
            0
        };
        // Slashes affect only absorbed newline tails. Without a newline or
        // a tail-class carry, this block cannot start or continue one.
        let sl64 = if F::SLASH_PUNCT_TAIL
            && (n64 != 0 || (scan != 0 && is_tail_byte::<F>(bytes[scan - 1])))
        {
            let s0 = vceqq_u8(vld1q_u8(p), vdupq_n_u8(b'/'));
            let s1 = vceqq_u8(vld1q_u8(p.add(16)), vdupq_n_u8(b'/'));
            let s2 = vceqq_u8(vld1q_u8(p.add(32)), vdupq_n_u8(b'/'));
            let s3 = vceqq_u8(vld1q_u8(p.add(48)), vdupq_n_u8(b'/'));
            movemask64(s0, s1, s2, s3)
        } else {
            0
        };

        let am = AsciiMasks {
            l: l64,
            d: d64,
            s: s64,
            wt: wsa & !s64 & !n64,
            n: n64,
            hi: 0,
            ap: ap64,
        };
        let ax = OAsciiExtra { up: u64_, sl: sl64 };

        if F::SIMD_UNICODE
            && (vmaxvq_u8(hi_any) >= 0x80
                || (scan >= 1 && bytes[scan - 1] >= 0x80)
                || (scan >= 2 && bytes[scan - 2] >= 0x80))
        {
            let h0 = vcltzq_s8(vreinterpretq_s8_u8(vs[0]));
            let h1 = vcltzq_s8(vreinterpretq_s8_u8(vs[1]));
            let h2 = vcltzq_s8(vreinterpretq_s8_u8(vs[2]));
            let h3 = vcltzq_s8(vreinterpretq_s8_u8(vs[3]));
            let mut am = am;
            am.hi = movemask64(h0, h1, h2, h3);
            return extended_masks::<F>(bytes, scan, am, ax);
        }

        let cr = ascii_batch_carries::<F>(bytes, scan);
        mask_algebra::<F>(bytes, scan, am, ax, cr, OUni::default())
    }
}

/// x86-64 front-end: same contract as the NEON `batch_masks` above,
/// dispatching on the runtime-detected SIMD tier (AVX-512 or AVX2).
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn batch_masks<F: MaskFlavor>(bytes: &[u8], scan: usize) -> (u64, u64) {
    debug_assert!(simd_scanner_available());
    if avx512_scanner_available() {
        // SAFETY: runtime AVX-512 detection right above.
        unsafe { batch_masks_avx512::<F>(bytes, scan) }
    } else {
        // SAFETY: `MaskState` reaches this branch only on AVX2-capable CPUs.
        unsafe { batch_masks_avx2::<F>(bytes, scan) }
    }
}

/// The strict-uppercase and slash masks for `bytes[scan..scan+64]`,
/// AVX-512 tier.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx512vl,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
fn ascii_extra_avx512(bytes: &[u8], scan: usize) -> OAsciiExtra {
    use std::arch::x86_64::*;
    unsafe {
        let v = _mm512_loadu_si512(bytes.as_ptr().add(scan) as *const _);
        let up = _mm512_cmple_epu8_mask(
            _mm512_sub_epi8(v, _mm512_set1_epi8(b'A' as i8)),
            _mm512_set1_epi8(25),
        );
        let sl = _mm512_cmpeq_epi8_mask(v, _mm512_set1_epi8(b'/' as i8));
        OAsciiExtra { up, sl }
    }
}

/// The strict-uppercase and slash masks for `bytes[scan..scan+64]`,
/// AVX2 tier. `#[inline(never)]` for the same vector-domain reason as
/// [`ascii_masks_avx2`].
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
#[inline(never)]
fn ascii_extra_avx2(bytes: &[u8], scan: usize) -> OAsciiExtra {
    use std::arch::x86_64::*;
    unsafe {
        let le =
            |v: __m256i, lim: __m256i| -> __m256i { _mm256_cmpeq_epi8(_mm256_min_epu8(v, lim), v) };
        let mm = |m0: __m256i, m1: __m256i| -> u64 {
            (_mm256_movemask_epi8(m0) as u32 as u64)
                | ((_mm256_movemask_epi8(m1) as u32 as u64) << 32)
        };
        let p = bytes.as_ptr().add(scan);
        let v0 = _mm256_loadu_si256(p as *const _);
        let v1 = _mm256_loadu_si256(p.add(32) as *const _);
        let ca = _mm256_set1_epi8(b'A' as i8);
        let c25 = _mm256_set1_epi8(25);
        let up = mm(
            le(_mm256_sub_epi8(v0, ca), c25),
            le(_mm256_sub_epi8(v1, ca), c25),
        );
        let slc = _mm256_set1_epi8(b'/' as i8);
        let sl = mm(_mm256_cmpeq_epi8(v0, slc), _mm256_cmpeq_epi8(v1, slc));
        OAsciiExtra { up, sl }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx512vl,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
fn batch_masks_avx512<F: MaskFlavor>(bytes: &[u8], scan: usize) -> (u64, u64) {
    debug_assert!(scan + 70 <= bytes.len());
    // A flavor without SIMD Unicode support hands every Unicode-adjacent
    // batch to its exact scalar scanner; one sign-bit mask decides that
    // before the discarded classification work.
    if !F::SIMD_UNICODE {
        use std::arch::x86_64::*;
        let hi = unsafe {
            _mm512_movepi8_mask(_mm512_loadu_si512(bytes.as_ptr().add(scan) as *const _))
        };
        if hi != 0
            || (scan >= 1 && bytes[scan - 1] >= 0x80)
            || (scan >= 2 && bytes[scan - 2] >= 0x80)
        {
            return (0, u64::MAX);
        }
    }
    let am = ascii_masks_avx512(bytes, scan);
    let ax = ascii_extra_avx512(bytes, scan);
    if F::SIMD_UNICODE
        && (am.hi != 0
            || (scan >= 1 && bytes[scan - 1] >= 0x80)
            || (scan >= 2 && bytes[scan - 2] >= 0x80))
    {
        return extended_masks::<F>(bytes, scan, am, ax);
    }
    let cr = ascii_batch_carries::<F>(bytes, scan);
    mask_algebra::<F>(bytes, scan, am, ax, cr, OUni::default())
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
#[inline]
fn batch_masks_avx2<F: MaskFlavor>(bytes: &[u8], scan: usize) -> (u64, u64) {
    debug_assert!(scan + 70 <= bytes.len());
    // Same early scalar-handoff test as the AVX-512 tier.
    if !F::SIMD_UNICODE {
        use std::arch::x86_64::*;
        let hi = unsafe {
            let p = bytes.as_ptr().add(scan);
            let m0 = _mm256_movemask_epi8(_mm256_loadu_si256(p as *const _)) as u32;
            let m1 = _mm256_movemask_epi8(_mm256_loadu_si256(p.add(32) as *const _)) as u32;
            m0 as u64 | (m1 as u64) << 32
        };
        if hi != 0
            || (scan >= 1 && bytes[scan - 1] >= 0x80)
            || (scan >= 2 && bytes[scan - 2] >= 0x80)
        {
            return (0, u64::MAX);
        }
    }
    let am = ascii_masks_avx2(bytes, scan);
    let ax = ascii_extra_avx2(bytes, scan);
    if F::SIMD_UNICODE
        && (am.hi != 0
            || (scan >= 1 && bytes[scan - 1] >= 0x80)
            || (scan >= 2 && bytes[scan - 2] >= 0x80))
    {
        return extended_masks::<F>(bytes, scan, am, ax);
    }
    let cr = ascii_batch_carries::<F>(bytes, scan);
    mask_algebra::<F>(bytes, scan, am, ax, cr, OUni::default())
}

struct R50kScheme;

impl MaskScheme for R50kScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::advance_gpt2(input, pos, false)
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        r50k_batch_masks(bytes, scan)
    }
}

struct QwenScheme;

impl MaskScheme for QwenScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::advance_qwen(input, pos)
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        qwen_batch_masks::<false, false>(bytes, scan)
    }
}

struct QwenMarkScheme;

impl MaskScheme for QwenMarkScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::scan_qwen::<true, 1>(input, pos).expect("Qwen3.5 token boundary")
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        qwen_batch_masks::<true, false>(bytes, scan)
    }
}

struct PhiGlmScheme;

impl MaskScheme for PhiGlmScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::scan_qwen::<false, 3>(input, pos).expect("Phi/GLM token boundary")
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        qwen_batch_masks::<false, true>(bytes, scan)
    }
}

struct ContractionScheme;

impl MaskFlavor for ContractionScheme {
    const CONTRACTIONS: bool = true;
    const THREE_DIGIT_NUMBERS: bool = true;
    const SLASH_PUNCT_TAIL: bool = true;
    const SIMD_UNICODE: bool = true;
}

impl MaskScheme for ContractionScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::scan_llama::<true, 3>(input, pos).expect("mask-scanner token boundary")
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        batch_masks::<Self>(bytes, scan)
    }
}

struct KimiScheme;

impl MaskFlavor for KimiScheme {
    const CONTRACTIONS: bool = true;
    const THREE_DIGIT_NUMBERS: bool = true;
    const SLASH_PUNCT_TAIL: bool = false;
    const SIMD_UNICODE: bool = false;
}

impl MaskScheme for KimiScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        if let Some(end) = scan_kimi_han_run(bytes, pos) {
            return end;
        }
        if let Some(end) = scan_kimi_punctuation_run(bytes, pos) {
            return end;
        }
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::scan_kimi(input, pos).expect("Kimi token boundary")
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        batch_masks::<Self>(bytes, scan)
    }
}

struct PlainScheme;

impl MaskFlavor for PlainScheme {
    const CONTRACTIONS: bool = false;
    const THREE_DIGIT_NUMBERS: bool = false;
    const SLASH_PUNCT_TAIL: bool = true;
    const SIMD_UNICODE: bool = true;
}

impl MaskScheme for PlainScheme {
    #[inline(always)]
    fn advance(bytes: &[u8], pos: usize) -> usize {
        let input = unsafe { std::str::from_utf8_unchecked(bytes) };
        super::scan_llama::<false, 1>(input, pos).expect("mask-scanner token boundary")
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[inline(always)]
    fn batch_masks(bytes: &[u8], scan: usize) -> (u64, u64) {
        batch_masks::<Self>(bytes, scan)
    }
}

#[inline(never)]
pub(super) fn for_each_r50k_match_into<S: FusedPieceSink>(input: &str, sink: &mut S) {
    walk::<R50kScheme, _>(input, sink);
}

#[inline(never)]
pub(super) fn for_each_kimi_match_into<S: FusedPieceSink>(input: &str, sink: &mut S) {
    walk::<KimiScheme, _>(input, sink);
}

#[inline(never)]
pub(super) fn for_each_qwen_match_into<
    const WORD_MARKS: bool,
    const DIGITS: usize,
    S: FusedPieceSink,
>(
    input: &str,
    sink: &mut S,
) {
    if WORD_MARKS {
        debug_assert_eq!(DIGITS, 1);
        walk::<QwenMarkScheme, _>(input, sink);
    } else if DIGITS == 3 {
        walk::<PhiGlmScheme, _>(input, sink);
    } else {
        debug_assert_eq!(DIGITS, 1);
        walk::<QwenScheme, _>(input, sink);
    }
}

#[inline(never)]
pub(super) fn for_each_match_into<
    const CONTRACTIONS: bool,
    const DIGITS: usize,
    S: FusedPieceSink,
>(
    input: &str,
    sink: &mut S,
) {
    if CONTRACTIONS {
        debug_assert_eq!(DIGITS, 3);
        walk::<ContractionScheme, _>(input, sink);
    } else {
        debug_assert_eq!(DIGITS, 1);
        walk::<PlainScheme, _>(input, sink);
    }
}

#[inline(always)]
fn walk<S: MaskScheme, E: FusedPieceSink>(input: &str, sink: &mut E) {
    let mut state = MaskState::new(0);
    state.for_each_span::<S>(input, sink);
}
