use super::*;
use crate::json_structs::ModelConfig;

#[test]
fn byte_hash_keeps_native_words_and_byte_tail() {
    let mix = |state: u64, word: u64| state.wrapping_add(word).wrapping_mul(0x517cc1b727220a95);
    let first = mix(17, u64::from_ne_bytes(*b"abcdefgh"));
    let second = mix(first, u64::from_ne_bytes(*b"ijklmnop"));
    assert_eq!(fx_hash_bytes(b"", 17), 17);
    assert_eq!(fx_hash_bytes(b"abc", 17), mix(mix(mix(17, 97), 98), 99));
    assert_eq!(fx_hash_bytes(b"abcdefgh", 17), first);
    assert_eq!(fx_hash_bytes(b"abcdefghi", 17), mix(first, b'i' as u64));
    assert_eq!(fx_hash_bytes(b"abcdefghijklmnop", 17), second);
    assert_eq!(
        fx_hash_bytes(b"abcdefghijklmnopq", 17),
        mix(second, b'q' as u64)
    );
}

#[test]
fn merge_entry_equality_matches_heap_priority() {
    let first = MergeEntry::new(7, 3, 1, 2);
    let stale = MergeEntry::new(7, 3, 8, 9);
    let later = MergeEntry::new(7, 4, 1, 2);
    assert!(first == stale);
    assert_eq!(first.cmp(&stale), std::cmp::Ordering::Equal);
    assert!(first < later);
    assert!(first != later);
}

#[test]
fn packed_bridge_table_preserves_every_pair() {
    let vocabulary = vec!["aé中".into(), "<0xFF><0x00>z".into()];
    for fallback in [false, true] {
        let table = build_bigram_bridge_table(&vocabulary, fallback);
        assert_eq!(std::mem::size_of_val(&*table.bridgeable), 8192);
        let mut expected = [false; 65536];
        for token in &vocabulary {
            for pair in token.as_bytes().windows(2) {
                expected[pair[0] as usize * 256 + pair[1] as usize] = true;
            }
        }
        if fallback {
            expected[255 * 256] = true;
            expected[b'z' as usize] = true;
        }
        for prev in 0u16..256 {
            for cur in 0u16..256 {
                assert_eq!(
                    table.is_bridgeable(prev as u8, cur as u8),
                    expected[prev as usize * 256 + cur as usize],
                );
            }
        }
    }
    let mut table = BigramBridgeTable {
        bridgeable: Box::new([0; 1024]),
    };
    for pair in (0..65536).step_by(2) {
        table.insert((pair / 256) as u8, pair as u8);
    }
    for pair in 0..65536 {
        assert_eq!(
            table.is_bridgeable((pair / 256) as u8, pair as u8),
            pair % 2 == 0
        );
    }
}

#[test]
fn compact_token_lengths_preserve_long_matches() {
    let lengths = [1usize, 254, 255, 256, u16::MAX as usize];
    let vocab: Vocab = lengths
        .iter()
        .enumerate()
        .map(|(id, &len)| ("a".repeat(len), id as u32))
        .collect();
    let bpe = Bpe::build(vocab, HashMap::new(), false, true, None).unwrap();
    assert_eq!(bpe.token_lens, [1, 254, 255, 255, 255]);
    for (id, &len) in lengths.iter().enumerate() {
        assert!(bpe.token_length_matches(id as u32, len));
        assert!(!bpe.token_length_matches(id as u32, len - 1));
        assert!(!bpe.token_length_matches(id as u32, len + 1));
        assert_eq!(bpe.tokenize(&"a".repeat(len)).unwrap(), [id as u32]);
    }
    let oversized = HashMap::from([("a".repeat(65536), 0)]);
    assert!(
        Bpe::build(oversized, HashMap::new(), false, true, None)
            .unwrap_err()
            .contains("exceeds u16::MAX")
    );
}

fn test_bpe() -> Bpe {
    let vocab: Vocab = [
        ("a", 0),
        ("b", 1),
        ("c", 2),
        ("d", 3),
        ("ab", 4),
        ("cd", 5),
        ("abcd", 6),
    ]
    .into_iter()
    .map(|(s, id)| (s.to_string(), id))
    .collect();

    let merges: Vec<Value> = vec![
        Value::String("a b".into()),
        Value::String("c d".into()),
        Value::String("ab cd".into()),
    ];

    let merge_map = parse_merges(&vocab, &merges).unwrap();
    Bpe::new(&vocab, merge_map).unwrap()
}

#[test]
fn empty_input() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("").unwrap(), Vec::<u32>::new());
}

#[test]
fn single_char() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("a").unwrap(), vec![0]);
    assert_eq!(bpe.tokenize("d").unwrap(), vec![3]);
}

#[test]
fn simple_merge() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("ab").unwrap(), vec![4]);
    assert_eq!(bpe.tokenize("cd").unwrap(), vec![5]);
}

/// A final merge for another token cannot make this vocabulary spelling an exact match.
#[test]
fn mismatched_final_merge_is_not_an_exact_token() {
    let vocab: Vocab = [("a", 0), ("b", 1), ("ab", 2)]
        .into_iter()
        .map(|(text, token)| (text.to_string(), token))
        .collect();
    let merge_map = HashMap::from([((0, 1), (0, 0))]);
    let bpe = Bpe::build(vocab, merge_map, false, false, None).unwrap();

    assert_eq!(bpe.next_match("ab"), None);
}

#[test]
fn chained_merge() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("abcd").unwrap(), vec![6]);
}

#[test]
fn partial_merge() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("abc").unwrap(), vec![4, 2]);
}

#[test]
fn repeated_merge() {
    let bpe = test_bpe();
    assert_eq!(bpe.tokenize("abab").unwrap(), vec![4, 4]);
}

#[test]
fn deserialize_from_json() {
    let json = serde_json::json!({
        "type": "BPE",
        "vocab": {"a": 0, "b": 1, "ab": 2},
        "merges": ["a b"]
    });
    let config: ModelConfig = serde_json::from_value(json).unwrap();
    assert!(matches!(config, ModelConfig::Bpe(_)));
}

#[test]
fn deserialize_array_merges() {
    let json = serde_json::json!({
        "type": "BPE",
        "vocab": {"a": 0, "b": 1, "ab": 2},
        "merges": [["a", "b"]]
    });
    let config: ModelConfig = serde_json::from_value(json).unwrap();
    let ModelConfig::Bpe(bpe) = config;
    assert_eq!(bpe.tokenize("ab").unwrap(), vec![2]);
}

#[test]
fn cache_returns_same_result() {
    let vocab: Vocab = [("a", 0), ("b", 1), ("ab", 2)]
        .into_iter()
        .map(|(s, id)| (s.to_string(), id))
        .collect();
    let merges = vec![Value::String("a b".into())];
    let merge_map = parse_merges(&vocab, &merges).unwrap();
    let bpe = Bpe::new(&vocab, merge_map).unwrap();

    let first = bpe.tokenize("ab").unwrap();
    let second = bpe.tokenize("ab").unwrap();
    assert_eq!(first, second);
    assert_eq!(first, vec![2]);
}

#[test]
fn flat_cache_preserves_front_and_long_values() {
    let mut cache = FlatCache::new();
    let short = "four-token-hit";
    let short_ids = [1, 2, 3, 4];
    let packed = pack_short_key(short).unwrap();
    cache.insert(short, &short_ids);
    let front_index = cache.front_index(packed);
    cache.front[front_index] = FrontCacheSlot::default();

    let mut out = Vec::new();
    assert!(cache.get(short, &mut out));
    assert_eq!(out, short_ids);
    out.clear();
    assert!(cache.get(short, &mut out));
    assert_eq!(out, short_ids);

    let long = "a fused cache key longer than fifteen bytes";
    let long_ids = [5, 6, 7];
    cache.insert(long, &long_ids);
    out.clear();
    assert!(cache.get(long, &mut out));
    assert_eq!(out, long_ids);

    cache.clear();
    out.clear();
    assert!(!cache.get(short, &mut out));
    assert!(!cache.get(long, &mut out));
}

#[test]
fn backing_cache_packed_values_preserve_boundaries() {
    let mut cache = FlatCache::new();
    let key = pack_short_key("dtype-boundary").unwrap();
    let values = [
        vec![],
        vec![0],
        vec![(1 << 31) - 1, (1 << 31) - 1],
        vec![1 << 31],
        vec![0, u32::MAX],
        vec![u32::MAX, 0],
        vec![0, 1, u32::MAX],
        vec![0, 1, 2, 3],
        vec![0, 1, 2, 3, 4],
        vec![u32::MAX; u16::MAX as usize],
        vec![u32::MAX],
    ];
    for ids in values {
        assert!(cache.insert_packed_backing(key, &ids));
        let mut out = Vec::with_capacity(4);
        out.push(123);
        assert!(cache.get_packed_backing(key, &mut out));
        assert_eq!(&out[1..], ids);
        assert_eq!(cache.count, 1);
    }
    assert!(!cache.insert_packed_backing(key, &vec![0; u16::MAX as usize + 1]));
    cache.clear_backing();
    let mut out = Vec::with_capacity(4);
    assert!(!cache.get_packed_backing(key, &mut out));
    assert_eq!(cache.count, 0);
    assert!(cache.pool.is_empty());
}

#[test]
fn backing_cache_packed_keys_resolve_collisions() {
    for high in [false, true] {
        let mut homes = HashMap::new();
        let (first, second) = (1..=FLAT_CACHE_SIZE + 1)
            .find_map(|value| {
                let key = if high {
                    123 | (value as u128) << 64
                } else {
                    value as u128 | 123u128 << 64
                };
                homes
                    .insert(flat_cache_index(key), key)
                    .map(|old| (old, key))
            })
            .unwrap();
        let mut cache = FlatCache::new();
        assert!(cache.insert_packed_backing(first, &[7, 8]));
        assert!(cache.insert_packed_backing(second, &[u32::MAX]));
        for (key, expected) in [(first, vec![7, 8]), (second, vec![u32::MAX])] {
            let mut out = Vec::with_capacity(4);
            assert!(cache.get_packed_backing(key, &mut out));
            assert_eq!(out, expected);
        }
        assert_eq!(cache.count, 2);
    }
}

#[test]
fn rejects_out_of_range_merge_token_ids() {
    let vocab: Vocab = [("a".to_string(), 0), ("b".to_string(), 1)]
        .into_iter()
        .collect();

    for (left, right, merged) in [(2, 1, 0), (0, 2, 0), (0, 1, 2)] {
        let merge_map = HashMap::from([((left, right), (0, merged))]);
        assert_eq!(
            Bpe::new(&vocab, merge_map).unwrap_err(),
            "merge token id exceeds vocabulary size"
        );
    }
}

#[test]
fn rejects_corrupt_cached_ranked_merge_table() {
    let mut resolved = test_bpe().resolved_config();
    resolved.ranked_slot_indices[0] = u32::MAX;
    assert!(Bpe::from_resolved(resolved).is_err());
}
