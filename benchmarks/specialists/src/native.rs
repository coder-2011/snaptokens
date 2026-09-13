use std::{
    ffi::{CStr, c_char, c_void},
    mem,
    ptr::NonNull,
    slice,
};

use anyhow::{Context, Result, anyhow, ensure};
use rustc_hash::FxHashMap;

#[repr(C)]
struct NativeRankEntry {
    data: *const u8,
    len: usize,
    rank: u32,
}

#[repr(C)]
struct NativeSpecialEntry {
    data: *const u8,
    len: usize,
    id: u32,
}

#[repr(C)]
struct NativeBuffer {
    data: *const c_void,
    len: usize,
    owner: *mut c_void,
}

impl NativeBuffer {
    const fn empty() -> Self {
        Self {
            data: std::ptr::null(),
            len: 0,
            owner: std::ptr::null_mut(),
        }
    }
}

unsafe extern "C" {
    fn st_tokendagger_create(
        pattern: *const u8,
        pattern_len: usize,
        ranks: *const NativeRankEntry,
        rank_count: usize,
        specials: *const NativeSpecialEntry,
        special_count: usize,
        error: *mut NativeBuffer,
    ) -> *mut c_void;
    fn st_tokendagger_encode(
        handle: *mut c_void,
        text: *const u8,
        text_len: usize,
        output: *mut NativeBuffer,
        error: *mut NativeBuffer,
    ) -> i32;
    fn st_tokendagger_release_output(output: NativeBuffer);
    fn st_tokendagger_release_error(error: NativeBuffer);
    fn st_tokendagger_destroy(handle: *mut c_void);
    fn st_tokendagger_pcre2_info(
        version: *mut c_char,
        version_capacity: usize,
        jit_available: *mut u32,
        header_major: *mut u32,
        header_minor: *mut u32,
    ) -> i32;
}

pub struct NativeRuntime {
    pub pcre2_header_version: String,
    pub pcre2_version: String,
    pub pcre2_jit_available: bool,
}

pub struct NativeTokenizer {
    handle: NonNull<c_void>,
}

// TokenDagger keeps its vocabularies immutable after construction and allocates
// PCRE2 match scratch in thread-local storage.
unsafe impl Send for NativeTokenizer {}
unsafe impl Sync for NativeTokenizer {}

impl NativeTokenizer {
    pub fn tokendagger(
        ranks: &FxHashMap<Vec<u8>, u32>,
        specials: &FxHashMap<String, u32>,
        pattern: &str,
    ) -> Result<Self> {
        ensure!(
            ranks.values().all(|&rank| i32::try_from(rank).is_ok()),
            "TokenDagger cannot represent a rank above i32::MAX"
        );
        ensure!(
            specials.values().all(|&id| i32::try_from(id).is_ok()),
            "TokenDagger cannot represent a special ID above i32::MAX"
        );
        let native_ranks: Vec<_> = ranks
            .iter()
            .map(|(token, &rank)| NativeRankEntry {
                data: token.as_ptr(),
                len: token.len(),
                rank,
            })
            .collect();
        let native_specials: Vec<_> = specials
            .iter()
            .map(|(token, &id)| NativeSpecialEntry {
                data: token.as_ptr(),
                len: token.len(),
                id,
            })
            .collect();
        let mut error = NativeBuffer::empty();
        let handle = unsafe {
            st_tokendagger_create(
                pattern.as_ptr(),
                pattern.len(),
                native_ranks.as_ptr(),
                native_ranks.len(),
                native_specials.as_ptr(),
                native_specials.len(),
                &mut error,
            )
        };
        let handle = match NonNull::new(handle) {
            Some(handle) => handle,
            None => return Err(Self::take_error(error, "native constructor failed")),
        };
        if !error.owner.is_null() {
            unsafe { st_tokendagger_destroy(handle.as_ptr()) }
            return Err(Self::take_error(
                error,
                "native constructor returned a stale error",
            ));
        }
        Ok(Self { handle })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        let mut output = NativeBuffer::empty();
        let mut error = NativeBuffer::empty();
        let status = unsafe {
            st_tokendagger_encode(
                self.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
                &mut output,
                &mut error,
            )
        };
        if status != 0 {
            if !output.owner.is_null() {
                unsafe { st_tokendagger_release_output(output) }
            }
            return Err(Self::take_error(error, "native encode failed"));
        }
        if !error.owner.is_null() {
            if !output.owner.is_null() {
                unsafe { st_tokendagger_release_output(output) }
            }
            return Err(Self::take_error(
                error,
                "native encode returned a stale error",
            ));
        }
        if output.owner.is_null() {
            return Err(anyhow!("native encode returned no owner"));
        }
        if output.len > isize::MAX as usize / mem::size_of::<i32>() {
            unsafe { st_tokendagger_release_output(output) }
            return Err(anyhow!("native encode returned an oversized ID buffer"));
        }
        if output.len > 0 && output.data.is_null() {
            unsafe { st_tokendagger_release_output(output) }
            return Err(anyhow!("native encode returned null IDs"));
        }

        if output.len == 0 {
            unsafe { st_tokendagger_release_output(output) }
            return Ok(Vec::new());
        }

        // TokenDagger's public vector uses 32-bit signed int for nonnegative IDs.
        let native_ids = unsafe { slice::from_raw_parts(output.data.cast::<i32>(), output.len) };
        if let Some((index, id)) = native_ids
            .iter()
            .copied()
            .enumerate()
            .find(|(_, id)| *id < 0)
        {
            unsafe { st_tokendagger_release_output(output) }
            return Err(anyhow!(
                "native encode returned negative ID {id} at {index}"
            ));
        }
        let ids = native_ids.iter().map(|&id| id as u32).collect();
        unsafe { st_tokendagger_release_output(output) }
        Ok(ids)
    }

    fn take_error(error: NativeBuffer, fallback: &str) -> anyhow::Error {
        if error.owner.is_null() {
            return anyhow!(fallback.to_owned());
        }
        let message = if error.data.is_null() {
            fallback.to_owned()
        } else {
            unsafe {
                String::from_utf8_lossy(slice::from_raw_parts(error.data.cast::<u8>(), error.len))
                    .into_owned()
            }
        };
        unsafe { st_tokendagger_release_error(error) }
        anyhow!(message)
    }
}

pub fn tokendagger_runtime() -> Result<NativeRuntime> {
    let mut version = [0 as c_char; 64];
    let mut jit_available = 0;
    let mut header_major = 0;
    let mut header_minor = 0;
    let status = unsafe {
        st_tokendagger_pcre2_info(
            version.as_mut_ptr(),
            version.len(),
            &mut jit_available,
            &mut header_major,
            &mut header_minor,
        )
    };
    ensure!(
        status == 0,
        "PCRE2 runtime query failed with status {status}"
    );
    let version = unsafe { CStr::from_ptr(version.as_ptr()) }
        .to_str()
        .context("PCRE2 runtime version is not UTF-8")?
        .to_owned();
    Ok(NativeRuntime {
        pcre2_header_version: format!("{header_major}.{header_minor}"),
        pcre2_version: version,
        pcre2_jit_available: jit_available != 0,
    })
}

impl Drop for NativeTokenizer {
    fn drop(&mut self) {
        unsafe { st_tokendagger_destroy(self.handle.as_ptr()) }
    }
}
