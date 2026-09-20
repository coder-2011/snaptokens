#![no_main]

use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;

const ST_HEADER_LEN: usize = 52;
const MAX_ST_PAYLOAD_BYTES: usize = 512 * 1024 * 1024 - ST_HEADER_LEN;

fn st_file(version: u32, payload: &[u8]) -> Vec<u8> {
    let mut file = Vec::with_capacity(ST_HEADER_LEN + payload.len());
    file.extend_from_slice(b"SNAPST\0\0");
    file.extend_from_slice(&version.to_le_bytes());
    file.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    file.extend_from_slice(blake3::hash(payload).as_bytes());
    file.extend_from_slice(payload);
    file
}

fuzz_target!(|data: &[u8]| {
    let Some((&selector, payload)) = data.split_first() else {
        return;
    };
    let version = if selector & 1 == 0 { 3 } else { 4 };
    if payload.len() > MAX_ST_PAYLOAD_BYTES {
        return;
    }

    let dir = std::env::temp_dir().join(format!("fuzz_st_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let st_path = dir.join("test.st");

    if std::fs::write(&st_path, st_file(version, payload)).is_ok() {
        let _ = Tokenizer::load_file_with_st_cache(&st_path);
    }
    let _ = std::fs::remove_file(&st_path);
});
