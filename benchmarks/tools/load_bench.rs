use std::{hint::black_box, path::Path, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .ok_or("usage: load_bench <path> <json|st> [iterations]")?;
    let mode = args.next().ok_or("missing load mode")?;
    let iterations = args.next().map_or(Ok(10), |value| value.parse())?;

    for _ in 0..iterations {
        let start = Instant::now();
        let tokenizer = match mode.as_str() {
            "json" => snaptokens::Tokenizer::load_file(Path::new(&path))?,
            "st" => snaptokens::Tokenizer::load_file_with_st_cache(Path::new(&path))?,
            _ => return Err("load mode must be json or st".into()),
        };
        black_box(tokenizer);
        println!("{}", start.elapsed().as_nanos());
    }
    Ok(())
}
