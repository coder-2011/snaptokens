use daachorse::DoubleArrayAhoCorasickBuilder;
use std::{collections::HashMap, error::Error, fs, hint::black_box, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let json: serde_json::Value = serde_json::from_slice(&fs::read(&args[2])?)?;
    let vocab: Vec<(String, f64)> = serde_json::from_value(json["model"]["vocab"].clone())?;
    let last_ids: HashMap<_, _> = vocab
        .iter()
        .enumerate()
        .map(|(id, (token, _))| (token.as_str(), id as u32))
        .collect();
    let patterns: Vec<_> = vocab
        .iter()
        .enumerate()
        .filter_map(|(id, (token, _))| {
            (!token.is_empty() && last_ids.get(token.as_str()) == Some(&(id as u32)))
                .then_some((token.as_str(), id as u32))
        })
        .collect();
    if args[1] == "check" {
        let reference = DoubleArrayAhoCorasickBuilder::new()
            .num_free_blocks(16)
            .build_with_values(patterns.iter().copied())
            .map_err(|error| error.to_string())?;
        let candidate = DoubleArrayAhoCorasickBuilder::new()
            .num_free_blocks(1)
            .build_with_values(patterns.iter().copied())
            .map_err(|error| error.to_string())?;
        let corpus: Vec<String> = serde_json::from_slice(&fs::read(&args[3])?)?;
        for (index, text) in corpus
            .iter()
            .map(String::as_str)
            .chain(patterns.iter().map(|&(text, _)| text))
            .enumerate()
        {
            let expected = reference
                .find_overlapping_iter(text)
                .map(|m| (m.start(), m.end(), m.value()));
            let actual = candidate
                .find_overlapping_iter(text)
                .map(|m| (m.start(), m.end(), m.value()));
            assert!(expected.eq(actual), "match difference at input {index}");
        }
        println!(
            "{}",
            serde_json::json!({"reference_bytes": reference.heap_bytes(),
                "candidate_bytes": candidate.heap_bytes(),
                "reference_states": reference.num_states(),
                "candidate_states": candidate.num_states(),
                "checked_inputs": corpus.len() + patterns.len()})
        );
        return Ok(());
    }
    assert_eq!(args[1], "build");
    let blocks: u32 = args[3].parse()?;
    let rounds: usize = args[4].parse()?;
    for _ in 0..rounds {
        let start = Instant::now();
        let matcher = DoubleArrayAhoCorasickBuilder::new()
            .num_free_blocks(blocks)
            .build_with_values(patterns.iter().copied())
            .map_err(|error| error.to_string())?;
        drop(black_box(matcher));
        println!("{}", start.elapsed().as_nanos());
    }
    Ok(())
}
