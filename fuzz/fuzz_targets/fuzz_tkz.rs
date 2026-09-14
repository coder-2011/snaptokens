#![no_main]

use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;
use std::io::Write;

fuzz_target!(|data: &[u8]| {
    let dir = std::env::temp_dir().join(format!("fuzz_tkz_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let tkz_path = dir.join("test.tkz");

    if let Ok(mut f) = std::fs::File::create(&tkz_path) {
        let _ = f.write_all(data);
        drop(f);
        let _ = Tokenizer::load_file_with_tkz_cache(&tkz_path);
    }
    let _ = std::fs::remove_file(&tkz_path);
});
