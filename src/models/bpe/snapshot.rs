use bincode::{Decode, Encode};

use super::{
    BigramBridgeTable, Bpe, CrossThreadCache, EMPTY_KEY, EMPTY_VOCAB_HASH, ExactTokenMatcher,
    ExactTokenTrie, INVALID_TOKEN, MergeAdjacency, PackedVocabulary, RankedMergeMap, Result,
    TokenId, VocabLookup, byte_pair_initial_from_ranked, dense_tables_from_ranked, fx_hash,
    initial_token_byte_map, next_bpe_id,
};

// Persist encode tables in wire order; only this module can interpret unchecked fields.
#[derive(Encode, Decode)]
pub(crate) struct NativeBpeTables {
    token_bytes: Vec<u8>,
    token_offsets: Vec<u32>,
    unmerge_map: Vec<(u32, u32)>,
    is_orphan: Vec<u8>,
    byte_to_initial_token: Vec<u32>,
    byte_fallback_token_ids: Vec<u32>,
    ranked_capacity: u32,
    ranked_slots: Vec<u32>,
    ranked_keys: Vec<u64>,
    ranked_values: Vec<u64>,
    ranked_merges: bool,
    fused_cache_seeds: Vec<(u128, u32)>,
    merge_adj_offsets: Vec<u32>,
    merge_adj_keys: Vec<u64>,
    merge_adj_new_ids: Vec<u32>,
    ignore_merges: bool,
    byte_fallback: bool,
    bridgeable: Vec<u64>,
    exact_token_trie: Option<ExactTokenTrie>,
    lookup_capacity: u32,
    lookup_slots: Vec<u32>,
    lookup_hashes: Vec<u64>,
    lookup_ids: Vec<u32>,
}

impl NativeBpeTables {
    pub(crate) fn from_model(model: &Bpe) -> Result<Self> {
        let is_orphan = match &model.matcher {
            ExactTokenMatcher::Direct(flags) => flags.iter().map(|&flag| u8::from(flag)).collect(),
            ExactTokenMatcher::Trie(_) => (0..model.packed_vocabulary.len())
                .map(|token| {
                    let id = token as TokenId;
                    let text = model.packed_vocabulary.get(id).unwrap();
                    u8::from(
                        model.unmerge_map[token] == (id, id) && model.next_match(text) != Some(id),
                    )
                })
                .collect(),
        };
        let exact_token_trie = match &model.matcher {
            ExactTokenMatcher::Trie(trie) => Some(trie.clone()),
            ExactTokenMatcher::Direct(_) => None,
        };

        let mut ranked_slots = Vec::new();
        let mut ranked_keys = Vec::new();
        let mut ranked_values = Vec::new();
        for (slot, (&key, &value)) in model
            .ranked_merge_map
            .keys
            .iter()
            .zip(&model.ranked_merge_map.values)
            .enumerate()
        {
            if key != EMPTY_KEY {
                ranked_slots.push(slot as u32);
                ranked_keys.push(key);
                ranked_values.push(value);
            }
        }

        let mut lookup_slots = Vec::new();
        let mut lookup_hashes = Vec::new();
        let mut lookup_ids = Vec::new();
        for (slot, (&hash, &id)) in model
            .vocab_lookup
            .hashes
            .iter()
            .zip(&model.vocab_lookup.ids)
            .enumerate()
        {
            if hash != EMPTY_VOCAB_HASH {
                lookup_slots.push(slot as u32);
                lookup_hashes.push(hash);
                lookup_ids.push(id);
            }
        }

        Ok(NativeBpeTables {
            token_bytes: model.packed_vocabulary.bytes.clone(),
            token_offsets: model.packed_vocabulary.offsets.clone(),
            unmerge_map: model.unmerge_map.clone(),
            is_orphan,
            byte_to_initial_token: model.byte_to_initial_token.to_vec(),
            byte_fallback_token_ids: model.byte_fallback_token_ids.to_vec(),
            ranked_capacity: u32::try_from(model.ranked_merge_map.keys.len())
                .map_err(|_| "`.st` ranked table exceeds u32 capacity")?,
            ranked_slots,
            ranked_keys,
            ranked_values,
            ranked_merges: model.ranked_merges,
            fused_cache_seeds: model.fused_cache_seeds.clone(),
            merge_adj_offsets: model.merge_adj.offsets.clone(),
            merge_adj_keys: model.merge_adj.keys.clone(),
            merge_adj_new_ids: model.merge_adj.new_ids.clone(),
            ignore_merges: model.ignore_merges,
            byte_fallback: model.byte_fallback,
            bridgeable: model.bigram_bridge_table.bridgeable.to_vec(),
            exact_token_trie,
            lookup_capacity: u32::try_from(model.vocab_lookup.hashes.len())
                .map_err(|_| "`.st` vocabulary lookup exceeds u32 capacity")?,
            lookup_slots,
            lookup_hashes,
            lookup_ids,
        })
    }

    pub(crate) fn into_model(self) -> Result<Bpe> {
        let NativeBpeTables {
            token_bytes,
            token_offsets,
            unmerge_map,
            is_orphan,
            byte_to_initial_token,
            byte_fallback_token_ids,
            ranked_capacity,
            ranked_slots,
            ranked_keys,
            ranked_values,
            ranked_merges,
            fused_cache_seeds,
            merge_adj_offsets,
            merge_adj_keys,
            merge_adj_new_ids,
            ignore_merges,
            byte_fallback,
            bridgeable,
            exact_token_trie,
            lookup_capacity,
            lookup_slots,
            lookup_hashes,
            lookup_ids,
        } = self;

        let packed_vocabulary = PackedVocabulary::from_parts(token_bytes, token_offsets)?;
        let vocab_size = packed_vocabulary.len();
        if vocab_size == 0 || vocab_size > u32::MAX as usize {
            return Err("invalid .st vocabulary size".into());
        }
        if unmerge_map.len() != vocab_size || is_orphan.len() != vocab_size {
            return Err("invalid .st per-token table length".into());
        }
        if byte_to_initial_token.len() != 256 || byte_fallback_token_ids.len() != 256 {
            return Err("invalid .st byte-table length".into());
        }
        if bridgeable.len() != 1024 {
            return Err("invalid .st bridge table length".into());
        }
        let expected_ranked_capacity = if ranked_keys.is_empty() {
            0
        } else {
            ranked_keys
                .len()
                .checked_mul(2)
                .and_then(usize::checked_next_power_of_two)
                .ok_or("ranked .st table capacity overflow")?
        };
        if ranked_slots.len() != ranked_keys.len()
            || ranked_keys.len() != ranked_values.len()
            || ranked_capacity as usize != expected_ranked_capacity
        {
            return Err("invalid .st ranked merge table".into());
        }
        let mut ranked_full_keys = vec![EMPTY_KEY; ranked_capacity as usize];
        let mut ranked_full_values = vec![0u64; ranked_capacity as usize];
        for ((&slot, &key), &value) in ranked_slots.iter().zip(&ranked_keys).zip(&ranked_values) {
            let index = slot as usize;
            if index >= ranked_full_keys.len()
                || ranked_full_keys[index] != EMPTY_KEY
                || key == EMPTY_KEY
            {
                return Err("invalid .st ranked merge slot".into());
            }
            ranked_full_keys[index] = key;
            ranked_full_values[index] = value;
        }
        if ranked_capacity != 0 && !ranked_full_keys.contains(&EMPTY_KEY) {
            return Err("invalid .st ranked merge table".into());
        }
        // An in-range slot is reachable only if its home lies in the same probe cluster.
        if let Some(first_empty) = ranked_full_keys.iter().position(|&key| key == EMPTY_KEY) {
            let capacity = ranked_full_keys.len();
            let mask = capacity - 1;
            let mut cluster_start = 1;
            for step in 1..capacity {
                let key = ranked_full_keys[(first_empty + step) & mask];
                if key == EMPTY_KEY {
                    cluster_start = step + 1;
                    continue;
                }
                let home = fx_hash(key) as usize & mask;
                let relative_home = home.wrapping_add(capacity).wrapping_sub(first_empty) & mask;
                if relative_home < cluster_start || relative_home > step {
                    return Err("invalid .st ranked merge probe chain".into());
                }
            }
        }
        if merge_adj_offsets.len() != vocab_size + 1
            || merge_adj_offsets.first().copied() != Some(0)
            || merge_adj_offsets.last().copied() != Some(merge_adj_keys.len() as u32)
            || merge_adj_keys.len() != merge_adj_new_ids.len()
            || merge_adj_offsets.windows(2).any(|pair| pair[0] > pair[1])
        {
            return Err("invalid .st merge-adjacency table".into());
        }
        for bounds in merge_adj_offsets.windows(2) {
            let row = &merge_adj_keys[bounds[0] as usize..bounds[1] as usize];
            if row
                .iter()
                .any(|&key| (key >> 32) as usize >= vocab_size || key as u32 == u32::MAX)
                || row.windows(2).any(|pair| pair[0] >> 32 >= pair[1] >> 32)
            {
                return Err("invalid .st merge-adjacency row".into());
            }
        }

        let vocab_lookup = VocabLookup::from_cached_slots(
            &packed_vocabulary,
            lookup_capacity,
            &lookup_slots,
            &lookup_hashes,
            &lookup_ids,
        )?;

        let is_orphan: Vec<bool> = is_orphan.into_iter().map(|flag| flag != 0).collect();
        for &(left, right) in &unmerge_map {
            if left as usize >= vocab_size || right as usize >= vocab_size {
                return Err("out-of-range .st decomposition token".into());
            }
        }
        for &id in byte_to_initial_token
            .iter()
            .chain(byte_fallback_token_ids.iter())
        {
            if id != INVALID_TOKEN && id as usize >= vocab_size {
                return Err("out-of-range .st character token".into());
            }
        }
        for &id in &merge_adj_new_ids {
            if id as usize >= vocab_size {
                return Err("out-of-range .st adjacency token".into());
            }
        }
        for (&key, &payload) in ranked_keys.iter().zip(&ranked_values) {
            if key == EMPTY_KEY {
                continue;
            }
            let left = (key >> 32) as u32;
            let right = key as u32;
            let merged = payload as u32;
            if left as usize >= vocab_size
                || right as usize >= vocab_size
                || merged as usize >= vocab_size
            {
                return Err("out-of-range .st ranked merge token".into());
            }
        }
        for &(_, id) in &fused_cache_seeds {
            if id as usize >= vocab_size {
                return Err("out-of-range .st fused-cache seed".into());
            }
        }

        let matcher = if let Some(trie) = exact_token_trie {
            ExactTokenMatcher::Trie(trie.validate_with(vocab_size, &is_orphan, |token| {
                packed_vocabulary.bytes_at(token)
            })?)
        } else {
            ExactTokenMatcher::Direct(is_orphan)
        };

        let mut byte_to_initial = [INVALID_TOKEN; 256];
        byte_to_initial.copy_from_slice(&byte_to_initial_token);
        let mut byte_fallback_ids = [INVALID_TOKEN; 256];
        byte_fallback_ids.copy_from_slice(&byte_fallback_token_ids);
        let mut bmp_char_token = vec![INVALID_TOKEN; 0x10000].into_boxed_slice();
        for id in 0..vocab_size {
            let token = packed_vocabulary.get(id as u32).unwrap();
            let mut chars = token.chars();
            if let (Some(ch), None) = (chars.next(), chars.next())
                && (ch as u32) < 0x10000
            {
                bmp_char_token[ch as usize] = id as TokenId;
            }
        }
        let mut single_char = [INVALID_TOKEN; 128];
        single_char.copy_from_slice(&bmp_char_token[..128]);
        let token_lens = (0..vocab_size)
            .map(|token| {
                let len = packed_vocabulary.len_at(token);
                u16::try_from(len)
                    .map(|len| len.min(u8::MAX as u16) as u8)
                    .map_err(|_| format!("token {token} length {len} exceeds u16::MAX"))
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let bridgeable: [u64; 1024] = bridgeable
            .try_into()
            .map_err(|_| "invalid .st bridge table")?;
        let initial_token_byte = initial_token_byte_map(&byte_to_initial, vocab_size);
        let byte_pair_initial =
            byte_pair_initial_from_ranked(&ranked_keys, &ranked_values, &initial_token_byte);
        let (dense_ranked_bits, dense_ranked_merge, dense_merge) = dense_tables_from_ranked(
            &ranked_keys,
            &ranked_values,
            &byte_to_initial,
            byte_fallback,
            ranked_merges,
        );

        Ok(Bpe {
            id: next_bpe_id(),
            matcher,
            unmerge_map,
            token_lens,
            cross_thread_cache: CrossThreadCache::new(),
            fused_cross_thread_cache: CrossThreadCache::new(),
            packed_vocabulary,
            vocab_lookup,
            bmp_char_token,
            byte_to_initial_token: byte_to_initial,
            byte_fallback_token_ids: byte_fallback_ids,
            single_char_token: single_char,
            ranked_merge_map: RankedMergeMap {
                mask: ranked_full_keys.len().saturating_sub(1),
                keys: ranked_full_keys,
                values: ranked_full_values,
            },
            byte_pair_initial,
            dense_merge,
            dense_ranked_merge,
            dense_ranked_bits,
            ranked_merges,
            fused_cache_seeds,
            merge_adj: MergeAdjacency {
                offsets: merge_adj_offsets,
                keys: merge_adj_keys,
                new_ids: merge_adj_new_ids,
            },
            ignore_merges,
            byte_fallback,
            bigram_bridge_table: BigramBridgeTable {
                bridgeable: Box::new(bridgeable),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{pack_pair, tests::test_bpe};

    #[test]
    fn native_snapshot_bounds_lookup_allocations_by_occupancy() {
        let mut tables = NativeBpeTables::from_model(&test_bpe()).unwrap();
        tables.ranked_capacity = 1 << 31;
        assert!(tables.into_model().is_err());
        let mut tables = NativeBpeTables::from_model(&test_bpe()).unwrap();
        tables.lookup_capacity = 1 << 31;
        assert!(tables.into_model().is_err());
    }

    #[test]
    fn native_snapshot_rejects_unreachable_ranked_slot() {
        let mut tables = NativeBpeTables::from_model(&test_bpe()).unwrap();
        let old = tables.ranked_slots[0];
        let empty = (1..tables.ranked_capacity)
            .map(|step| (old + step) & (tables.ranked_capacity - 1))
            .find(|slot| !tables.ranked_slots.contains(slot))
            .unwrap();
        tables.ranked_slots[0] = empty;
        assert!(tables.into_model().is_err());
    }

    #[test]
    fn native_snapshot_rejects_invalid_adjacency_rows() {
        let original: Bpe = serde_json::from_value(serde_json::json!({
            "vocab": {"a": 0, "b": 1, "c": 2, "ab": 3, "ac": 4},
            "merges": [["a", "b"], ["a", "c"]]
        }))
        .unwrap();
        let restored = NativeBpeTables::from_model(&original)
            .unwrap()
            .into_model()
            .unwrap();
        assert_eq!(restored.merge_adj.get(0, 1), Some((0, 3)));
        assert_eq!(restored.merge_adj.get(0, 2), Some((1, 4)));

        for (case, keys) in [
            ("unsorted", [pack_pair(2, 1), pack_pair(1, 0)]),
            ("duplicate neighbor", [pack_pair(1, 0), pack_pair(1, 1)]),
            ("out-of-range neighbor", [pack_pair(1, 0), pack_pair(5, 1)]),
            ("invalid rank", [pack_pair(1, 0), pack_pair(2, u32::MAX)]),
        ] {
            let mut tables = NativeBpeTables::from_model(&original).unwrap();
            tables.merge_adj_keys = keys.to_vec();
            assert_eq!(
                tables.into_model().unwrap_err(),
                "invalid .st merge-adjacency row",
                "{case}"
            );
        }
    }

    #[test]
    fn snapshot_restores_persisted_exact_token_trie() {
        use super::super::ExactTokenTrieNode;
        let original: Bpe = serde_json::from_value(serde_json::json!({
            "vocab": {"a": 0, "b": 1, "ab": 2}, "merges": [["a", "b"]]
        }))
        .unwrap();
        let mut snapshot = NativeBpeTables::from_model(&original).unwrap();
        snapshot.exact_token_trie = Some(ExactTokenTrie {
            nodes: vec![
                ExactTokenTrieNode {
                    first_child: 1,
                    edge_count: 2,
                    token: INVALID_TOKEN,
                },
                ExactTokenTrieNode {
                    first_child: 3,
                    edge_count: 1,
                    token: 0,
                },
                ExactTokenTrieNode {
                    first_child: 0,
                    edge_count: 0,
                    token: 1,
                },
                ExactTokenTrieNode {
                    first_child: 0,
                    edge_count: 0,
                    token: 2,
                },
            ],
            incoming_bytes: b"abb".to_vec(),
        });
        let bytes = bincode::encode_to_vec(&snapshot, bincode::config::standard()).unwrap();
        let (snapshot, _): (NativeBpeTables, _) =
            bincode::decode_from_slice(&bytes, bincode::config::standard()).unwrap();
        let restored = snapshot.into_model().unwrap();
        for input in ["", "a", "b", "ab", "aba", "abab"] {
            let mut expected = Vec::new();
            let mut actual = Vec::new();
            original.append_bpe_ids(input, &mut expected).unwrap();
            restored.append_bpe_ids(input, &mut actual).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
