use std::{env, hint::black_box, time::Instant};

use snaptokens::Tokenizer;

struct Cell {
    rows: usize,
    row_bytes: usize,
    calls: usize,
    seed: usize,
}

fn floor_boundary(text: &str, mut index: usize) -> usize {
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

// Each call reads the next unseen corpus window, so piece caches see novel text.
fn windows<'a>(corpus: &'a str, cell: &Cell, warmup: bool) -> Vec<Vec<&'a str>> {
    let span = cell.rows * cell.row_bytes;
    let usable = corpus.len() - span - 8;
    let region = if warmup { usable / 2 } else { 0 };
    let mut offset = (cell.seed.wrapping_mul(7_919_993) + region) % usable;
    (0..cell.calls)
        .map(|_| {
            let rows = (0..cell.rows)
                .map(|_| {
                    let start = floor_boundary(corpus, offset);
                    let end = floor_boundary(corpus, (offset + cell.row_bytes).min(corpus.len()));
                    offset = (offset + cell.row_bytes) % usable;
                    &corpus[start..end]
                })
                .collect();
            rows
        })
        .collect()
}

fn fnv(hash: &mut u64, value: u64) {
    *hash = (*hash ^ value).wrapping_mul(0x100_0000_01b3);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].as_str();
    let tokenizer_path = &args[2];
    let corpus = String::from_utf8_lossy(&std::fs::read(&args[3]).expect("corpus")).into_owned();
    let number = |i: usize| args[i].parse::<usize>().expect("number");
    let cell = Cell { rows: number(4), row_bytes: number(5), calls: number(6), seed: number(7) };
    let tokenizer = Tokenizer::load_file(tokenizer_path).expect("load");
    let calls = windows(&corpus, &cell, false);

    match mode {
        "bench" => {
            for rows in windows(&corpus, &cell, true).iter().take(cell.calls.min(64)) {
                black_box(tokenizer.encode_batch_ragged(rows, false).expect("warmup"));
            }
            let mut hash = 0xcbf2_9ce4_8422_2325u64;
            let mut tokens = 0usize;
            let mut bytes = 0usize;
            let mut errors = 0usize;
            let mut elapsed = 0u128;
            for rows in &calls {
                bytes += rows.iter().map(|row| row.len()).sum::<usize>();
                let start = Instant::now();
                let result = tokenizer.encode_batch_ragged(rows, false);
                let (ids, lengths) = match result {
                    Ok(output) => output,
                    Err(_) => {
                        errors += 1;
                        continue;
                    }
                };
                // Consume the output inside the timer, then drop it there too.
                tokens += ids.len();
                fnv(&mut hash, ids.iter().fold(ids.len() as u64, |h, &id| h.rotate_left(5) ^ id as u64));
                fnv(&mut hash, lengths.iter().fold(0u64, |h, &len| h.rotate_left(7) ^ len as u64));
                drop(black_box((ids, lengths)));
                elapsed += start.elapsed().as_nanos();
            }
            let mib_s = bytes as f64 / (1 << 20) as f64 / (elapsed as f64 / 1e9);
            println!(
                "{{\"mib_s\":{mib_s:.4},\"ns\":{elapsed},\"bytes\":{bytes},\"tokens\":{tokens},\"errors\":{errors},\"hash\":\"{hash:016x}\"}}"
            );
        }
        "check" => {
            let reference = tokenizers::Tokenizer::from_file(tokenizer_path).expect("hf load");
            let mut mismatches = 0usize;
            let mut tokens = 0usize;
            for rows in &calls {
                let (ids, lengths) = tokenizer.encode_batch_ragged(rows, false).expect("encode");
                let mut start = 0;
                for (row, length) in rows.iter().zip(&lengths) {
                    let expected = reference.encode_fast(*row, false).expect("hf encode");
                    if expected.get_ids() != &ids[start..start + length] {
                        mismatches += 1;
                    }
                    start += length;
                    tokens += length;
                }
                if start != ids.len() || lengths.len() != rows.len() {
                    mismatches += 1;
                }
            }
            println!("{{\"mismatches\":{mismatches},\"tokens\":{tokens},\"calls\":{}}}", calls.len());
            assert_eq!(mismatches, 0, "Hugging Face parity failed");
        }
        _ => panic!("mode must be bench or check"),
    }
}
