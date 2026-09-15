#![no_main]

use std::collections::HashMap;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use snaptokens::Tokenizer;

#[derive(Arbitrary, Debug)]
struct UnigramInput {
    pieces: Vec<String>,
    input: String,
    byte_fallback: bool,
}

fn bounded_text(text: &str, max_characters: usize) -> String {
    text.chars().take(max_characters).collect()
}

#[derive(Clone, Copy)]
struct BestPathNode {
    score: f64,
    starts_at: usize,
    id: u32,
}

#[derive(Clone, Copy)]
struct PathPiece {
    starts_at: usize,
    ends_at: usize,
    id: u32,
}

/// A deliberately simple scan of every vocabulary spelling at every boundary.
/// It is independent from the production trie and keeps the fuzzer sensitive to
/// lookup-representation mistakes as well as parser panics.
fn reference_tokenize(vocab: &[(String, f64)], input: &str, byte_fallback: bool) -> Vec<u32> {
    if input.is_empty() {
        return Vec::new();
    }

    let min_score = vocab
        .iter()
        .map(|(_, score)| *score)
        .reduce(f64::min)
        .unwrap();
    // The production matcher retains only the last ID for duplicate spellings.
    let token_to_id: HashMap<&str, u32> = vocab
        .iter()
        .enumerate()
        .map(|(id, (piece, _))| (piece.as_str(), id as u32))
        .collect();
    let mut best = vec![None; input.len() + 1];
    best[0] = Some(BestPathNode {
        score: 0.0,
        starts_at: 0,
        id: 0,
    });

    for (starts_at, character) in input.char_indices() {
        let current = best[starts_at].unwrap();
        let character_end = starts_at + character.len_utf8();
        let mut has_single_character_piece = false;
        for (id, (piece, score)) in vocab.iter().enumerate() {
            if piece.is_empty()
                || token_to_id.get(piece.as_str()) != Some(&(id as u32))
                || !input[starts_at..].starts_with(piece)
            {
                continue;
            }
            let ends_at = starts_at + piece.len();
            let next_score = current.score + score;
            let target = &mut best[ends_at];
            if target.is_none_or(|node: BestPathNode| next_score > node.score) {
                *target = Some(BestPathNode {
                    score: next_score,
                    starts_at,
                    id: id as u32,
                });
            }
            has_single_character_piece |= ends_at == character_end;
        }
        if !has_single_character_piece {
            let target = &mut best[character_end];
            let next_score = current.score + min_score - 10.0;
            if target.is_none_or(|node: BestPathNode| next_score > node.score) {
                *target = Some(BestPathNode {
                    score: next_score,
                    starts_at,
                    id: 0,
                });
            }
        }
    }

    let mut pieces = Vec::new();
    let mut ends_at = input.len();
    while ends_at != 0 {
        let node = best[ends_at].unwrap();
        pieces.push(PathPiece {
            starts_at: node.starts_at,
            ends_at,
            id: node.id,
        });
        ends_at = node.starts_at;
    }
    pieces.reverse();

    let mut ids = Vec::new();
    let mut index = 0;
    while index < pieces.len() {
        let piece = pieces[index];
        if piece.id != 0 {
            ids.push(piece.id);
            index += 1;
            continue;
        }

        let start = piece.starts_at;
        let mut end = piece.ends_at;
        index += 1;
        while index < pieces.len() && pieces[index].id == 0 {
            end = pieces[index].ends_at;
            index += 1;
        }
        let unknown = &input[start..end];
        if byte_fallback {
            let byte_ids: Option<Vec<u32>> = unknown
                .bytes()
                .map(|byte| {
                    let spelling = format!("<0x{byte:02X}>");
                    token_to_id.get(spelling.as_str()).copied()
                })
                .collect();
            if let Some(byte_ids) = byte_ids {
                ids.extend(byte_ids);
                continue;
            }
        }
        ids.push(0);
    }
    ids
}

/// Reproduces the pinned Metaspace `prepend_scheme=never, split=true` shape
/// without sharing the production pre-tokenizer implementation.
fn reference_metaspace_tokenize(
    vocab: &[(String, f64)],
    input: &str,
    byte_fallback: bool,
) -> Vec<u32> {
    let rewritten = input.replace(' ', "▁");
    let mut ids = Vec::new();
    let mut start = 0;
    for (offset, character) in rewritten.char_indices() {
        if character == '▁' && offset > start {
            ids.extend(reference_tokenize(
                vocab,
                &rewritten[start..offset],
                byte_fallback,
            ));
            start = offset;
        }
    }
    if start < rewritten.len() {
        ids.extend(reference_tokenize(
            vocab,
            &rewritten[start..],
            byte_fallback,
        ));
    }
    ids
}

fuzz_target!(|config: UnigramInput| {
    // Keep a five-way root in every input so even a tiny generated payload
    // crosses the optimized dense-child branch before exercising its random tail.
    let mut vocab = vec![
        ("<unk>".to_string(), 0.0),
        ("ax".to_string(), -1.0),
        ("bx".to_string(), -2.0),
        ("cx".to_string(), -3.0),
        ("dx".to_string(), -4.0),
        ("ex".to_string(), -5.0),
    ];
    for (index, raw_piece) in config.pieces.iter().take(64).enumerate() {
        let piece = bounded_text(raw_piece, 16);
        if !piece.is_empty() {
            vocab.push((piece, -(index as f64 + 1.0)));
        }
    }
    let json = serde_json::json!({
        "added_tokens": [{
            "id": 256,
            "content": "!",
            "normalized": false
        }],
        "pre_tokenizer": {
            "type": "Metaspace",
            "replacement": "▁",
            "prepend_scheme": "never",
            "split": true
        },
        "model": {
            "type": "Unigram",
            "unk_id": 0,
            "vocab": vocab,
            "byte_fallback": config.byte_fallback
        }
    });
    // The added marker separates two ordinary pieces, exercising the
    // production chunk-local scratch reuse as well as direct Viterbi output.
    let tail = bounded_text(&config.input, 512).replace('!', "?");
    let input = format!("ax{tail}");
    if let Ok(tokenizer) = Tokenizer::from_json(json) {
        assert_eq!(
            tokenizer.encode(&input).unwrap(),
            reference_metaspace_tokenize(&vocab, &input, config.byte_fallback)
        );

        let split_input = format!("ax!{input}");
        let mut expected = reference_metaspace_tokenize(&vocab, "ax", config.byte_fallback);
        expected.push(256);
        expected.extend(reference_metaspace_tokenize(
            &vocab,
            &input,
            config.byte_fallback,
        ));
        assert_eq!(tokenizer.encode(&split_input).unwrap(), expected);
    }
});
