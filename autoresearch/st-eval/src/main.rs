use snaptokens::{LoadMode, Tokenizer};
use std::{error::Error, fs, hint::black_box, path::Path, time::Instant};

// Validate every load representation and output contract before timing a candidate.
fn check(path: &Path, corpus: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
    let inputs: Vec<String> = serde_json::from_slice(&fs::read(corpus)?)?;
    let reference = tokenizers::Tokenizer::from_file(path)?;
    let json = Tokenizer::load_file(path)?;
    let cached = Tokenizer::load_file_with_st_cache(path)?;
    let direct = Tokenizer::load_file_with_st_cache(path.with_extension("st"))?;
    let tokenizers = [&json, &cached, &direct];
    for special in [false, true] {
        let expected: Vec<_> = inputs
            .iter()
            .map(|input| {
                reference
                    .encode(input.as_str(), special)
                    .map(|e| e.get_ids().to_vec())
            })
            .collect::<Result<_, _>>()?;
        for tokenizer in tokenizers {
            for (input, expected) in inputs.iter().zip(&expected) {
                assert_eq!(
                    &tokenizer.encode(input, special)?,
                    expected,
                    "scalar {}",
                    path.display()
                );
            }
            assert_eq!(
                tokenizer.encode_batch(&inputs, special)?,
                expected,
                "batch {}",
                path.display()
            );
            let (ids, lengths) = tokenizer.encode_batch_ragged(&inputs, special)?;
            assert_eq!(lengths.len(), expected.len());
            let mut offset = 0;
            for (length, expected) in lengths.into_iter().zip(&expected) {
                assert_eq!(
                    &ids[offset..offset + length],
                    expected,
                    "ragged {}",
                    path.display()
                );
                offset += length;
            }
            assert_eq!(offset, ids.len());
        }
    }
    for (text, id) in reference.get_vocab(false) {
        for tokenizer in tokenizers {
            assert_eq!(tokenizer.token_to_id(&text), Some(id));
            assert_eq!(tokenizer.id_to_token(id), Some(text.as_str()));
        }
    }
    println!(
        "exact {} {} {}",
        path.display(),
        inputs.len(),
        blake3::hash(&fs::read(path)?)
    );
    Ok(())
}

// Each timed load includes materialization and destruction; no cached Tokenizer survives.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<_> = std::env::args().collect();
    let path = Path::new(&args[2]);
    if args[1] == "check" {
        return check(path, Path::new(&args[3]));
    }
    if args[1] == "encode" {
        let inputs: Vec<String> = serde_json::from_slice(&fs::read(&args[3])?)?;
        let tokenizer = Tokenizer::load_file_with_st_cache(path)?;
        for _ in 0..6 {
            let start = Instant::now();
            for input in &inputs {
                drop(black_box(tokenizer.encode(input, false)?));
            }
            let scalar = start.elapsed().as_nanos();
            let start = Instant::now();
            drop(black_box(tokenizer.encode_batch(&inputs, false)?));
            let batch = start.elapsed().as_nanos();
            let start = Instant::now();
            drop(black_box(tokenizer.encode_batch_ragged(&inputs, false)?));
            println!("{scalar} {batch} {}", start.elapsed().as_nanos());
        }
        return Ok(());
    }
    let mode = match args[1].as_str() {
        "st" => LoadMode::StCache,
        "json" => LoadMode::JsonOnly,
        _ => return Err("expected check, st, or json".into()),
    };
    let rounds: usize = args[3].parse()?;
    for _ in 0..rounds {
        let start = Instant::now();
        drop(black_box(match mode {
            LoadMode::JsonOnly => Tokenizer::load_file(path)?,
            LoadMode::StCache => Tokenizer::load_file_with_st_cache(path)?,
        }));
        println!("{}", start.elapsed().as_nanos());
    }
    Ok(())
}
