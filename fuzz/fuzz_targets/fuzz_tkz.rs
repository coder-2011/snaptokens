#![no_main]

use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;

const TKZ_HEADER_LEN: usize = 84;
const MAX_TKZ_PAYLOAD_BYTES: usize = 512 * 1024 * 1024 - TKZ_HEADER_LEN;

// Wraps fuzz-controlled serialized payload bytes in the valid outer TKZ envelope.
// Direct `.tkz` loads do not compare the source hash, so zeroes leave only payload
// decoding and tokenizer reconstruction under mutation.
fn tkz_file(payload: &[u8]) -> Vec<u8> {
    let mut file = Vec::with_capacity(TKZ_HEADER_LEN + payload.len());
    file.extend_from_slice(b"SNAPTKZ\0");
    file.extend_from_slice(&5_u32.to_le_bytes());
    file.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    file.extend_from_slice(&[0; 32]);
    file.extend_from_slice(blake3::hash(payload).as_bytes());
    file.extend_from_slice(payload);
    file
}

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_TKZ_PAYLOAD_BYTES {
        return;
    }

    let dir = std::env::temp_dir().join(format!("fuzz_tkz_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let tkz_path = dir.join("test.tkz");

    if std::fs::write(&tkz_path, tkz_file(data)).is_ok() {
        let _ = Tokenizer::load_file_with_tkz_cache(&tkz_path);
    }
    let _ = std::fs::remove_file(&tkz_path);
});
