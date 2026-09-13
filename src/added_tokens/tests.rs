use super::*;

#[test]
fn packed_flags_preserve_all_combinations() {
    assert_eq!(std::mem::size_of::<AddedTokenFlags>(), 1);
    for bits in 0u8..16 {
        let mut config = make_config(0, "x");
        config.single_word = bits & 1 != 0;
        config.lstrip = bits & 2 != 0;
        config.rstrip = bits & 4 != 0;
        config.special = bits & 8 != 0;
        let flags = AddedTokenFlags::from_config(&config);
        assert_eq!(
            flags.contains(AddedTokenFlags::SINGLE_WORD),
            config.single_word
        );
        assert_eq!(flags.contains(AddedTokenFlags::LSTRIP), config.lstrip);
        assert_eq!(flags.contains(AddedTokenFlags::RSTRIP), config.rstrip);
        assert_eq!(flags.contains(AddedTokenFlags::SPECIAL), config.special);
    }
}

fn make_config(id: u32, content: &str) -> AddedTokenConfig {
    AddedTokenConfig {
        id,
        content: content.to_string(),
        single_word: false,
        lstrip: false,
        rstrip: false,
        normalized: false,
        special: false,
    }
}

#[test]
fn empty_configs() {
    let result = AddedTokens::from_configs(&[]).unwrap();
    assert!(result.is_none());
}

#[test]
fn no_match() {
    let configs = vec![make_config(100, "<special>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("hello world");
    assert_eq!(segs, vec![Segment::Text("hello world")]);
}

#[test]
fn single_match_at_start() {
    let configs = vec![make_config(100, "<s>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("<s>hello");
    assert_eq!(segs, vec![Segment::Token(100), Segment::Text("hello")]);
}

#[test]
fn single_match_at_end() {
    let configs = vec![make_config(100, "</s>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("hello</s>");
    assert_eq!(segs, vec![Segment::Text("hello"), Segment::Token(100)]);
}

#[test]
fn match_in_middle() {
    let configs = vec![make_config(42, "<sep>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("hello<sep>world");
    assert_eq!(
        segs,
        vec![
            Segment::Text("hello"),
            Segment::Token(42),
            Segment::Text("world"),
        ]
    );
}

#[test]
fn multiple_matches() {
    let configs = vec![make_config(1, "<a>"), make_config(2, "<b>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("x<a>y<b>z");
    assert_eq!(
        segs,
        vec![
            Segment::Text("x"),
            Segment::Token(1),
            Segment::Text("y"),
            Segment::Token(2),
            Segment::Text("z"),
        ]
    );
}

#[test]
fn adjacent_matches() {
    let configs = vec![make_config(1, "<a>"), make_config(2, "<b>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("<a><b>");
    assert_eq!(segs, vec![Segment::Token(1), Segment::Token(2)]);
}

#[test]
fn longest_match_wins() {
    let configs = vec![make_config(1, "<file>"), make_config(2, "<filename>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("a<filename>b");
    assert_eq!(
        segs,
        vec![Segment::Text("a"), Segment::Token(2), Segment::Text("b"),]
    );
}

#[test]
fn entire_input_is_added_token() {
    let configs = vec![make_config(99, "hello")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("hello");
    assert_eq!(segs, vec![Segment::Token(99)]);
}

#[test]
fn empty_input() {
    let configs = vec![make_config(1, "<s>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("");
    assert!(segs.is_empty());
}

#[test]
fn token_to_id_finds_added_token() {
    let configs = vec![make_config(42, "<special>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(at.token_to_id("<special>"), Some(42));
}

#[test]
fn token_to_id_returns_none_for_unknown() {
    let configs = vec![make_config(1, "<known>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(at.token_to_id("<unknown>"), None);
}

#[test]
fn token_to_id_and_id_to_token_are_inverses() {
    let configs = vec![
        make_config(10, "<bos>"),
        make_config(11, "<eos>"),
        make_config(12, "<pad>"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    for cfg in &configs {
        let id = at.token_to_id(&cfg.content).unwrap();
        assert_eq!(id, cfg.id);
        assert_eq!(at.id_to_token(id), Some(cfg.content.as_str()));
    }
}

#[test]
fn unicode_token_content() {
    let configs = vec![
        make_config(1, "▁"),
        make_config(2, "Ġ"),
        make_config(3, "日本語"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("▁hello"),
        vec![Segment::Token(1), Segment::Text("hello")]
    );
    assert_eq!(
        at.split("Ġworld"),
        vec![Segment::Token(2), Segment::Text("world")]
    );
    assert_eq!(
        at.split("日本語text"),
        vec![Segment::Token(3), Segment::Text("text")]
    );
    assert_eq!(at.token_to_id("▁"), Some(1));
    assert_eq!(at.token_to_id("Ġ"), Some(2));
    assert_eq!(at.token_to_id("日本語"), Some(3));
}

#[test]
fn emoji_token_content() {
    let configs = vec![make_config(7, "🌍")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("hello 🌍 world"),
        vec![
            Segment::Text("hello "),
            Segment::Token(7),
            Segment::Text(" world"),
        ]
    );
}

#[test]
fn is_special_only_for_marked_tokens() {
    let mut special = make_config(1, "<bos>");
    special.special = true;
    let non_special = make_config(2, "<extra>");
    let at = AddedTokens::from_configs(&[special, non_special])
        .unwrap()
        .unwrap();
    assert!(at.is_special(1));
    assert!(!at.is_special(2));
    assert!(!at.is_special(99));
}

#[test]
fn iter_exposes_id_content_and_special_flag() {
    let mut special = make_config(1, "<bos>");
    special.special = true;
    let plain = make_config(2, "<extra>");
    let at = AddedTokens::from_configs(&[special, plain])
        .unwrap()
        .unwrap();

    let mut entries: Vec<_> = at.iter().collect();
    entries.sort_by_key(|entry| entry.id);

    assert_eq!(
        entries,
        vec![
            AddedTokenInfo {
                id: 1,
                content: "<bos>",
                special: true,
            },
            AddedTokenInfo {
                id: 2,
                content: "<extra>",
                special: false,
            },
        ]
    );
}

#[test]
fn len_returns_token_count() {
    let configs = vec![
        make_config(1, "<a>"),
        make_config(2, "<b>"),
        make_config(3, "<c>"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(at.len(), 3);
    assert!(!at.is_empty());
}

#[test]
fn three_tokens_with_shared_start_byte() {
    let configs = vec![
        make_config(1, "<"),
        make_config(2, "<s>"),
        make_config(3, "<sep>"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("x<sep>y<s>z<");
    assert_eq!(
        segs,
        vec![
            Segment::Text("x"),
            Segment::Token(3),
            Segment::Text("y"),
            Segment::Token(2),
            Segment::Text("z"),
            Segment::Token(1),
        ]
    );
}

#[test]
fn four_distinct_start_bytes_uses_full_scan() {
    let configs = vec![
        make_config(1, "<bos>"),
        make_config(2, "[SEP]"),
        make_config(3, "{pad}"),
        make_config(4, "|mask|"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("<bos>[SEP]{pad}|mask|");
    assert_eq!(
        segs,
        vec![
            Segment::Token(1),
            Segment::Token(2),
            Segment::Token(3),
            Segment::Token(4),
        ]
    );
}

#[test]
fn token_surrounded_by_text() {
    let configs = vec![make_config(5, "<mid>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("prefix <mid> suffix");
    assert_eq!(
        segs,
        vec![
            Segment::Text("prefix "),
            Segment::Token(5),
            Segment::Text(" suffix"),
        ]
    );
}

#[test]
fn repeated_same_token() {
    let configs = vec![make_config(9, "<r>")];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    let segs = at.split("<r><r><r>");
    assert_eq!(
        segs,
        vec![Segment::Token(9), Segment::Token(9), Segment::Token(9)]
    );
}

#[test]
fn single_word_respects_ascii_and_unicode_boundaries() {
    let mut mask = make_config(1, "<mask>");
    mask.single_word = true;
    let input = "<mask>, <mask>- ◌̰<mask> A<mask> <mask>";
    let expected = vec![
        Segment::Token(1),
        Segment::Text(", "),
        Segment::Token(1),
        Segment::Text("- ◌̰<mask> A<mask> "),
        Segment::Token(1),
    ];

    let prefiltered = AddedTokens::from_configs(&[mask.clone()]).unwrap().unwrap();
    assert_eq!(prefiltered.split(input), expected);

    let full_scan = AddedTokens::from_configs(&[
        mask,
        make_config(2, "[unused]"),
        make_config(3, "{unused}"),
        make_config(4, "|unused|"),
    ])
    .unwrap()
    .unwrap();
    assert_eq!(full_scan.split(input), expected);
}

fn make_strip_config(id: u32, content: &str, lstrip: bool, rstrip: bool) -> AddedTokenConfig {
    AddedTokenConfig {
        id,
        content: content.to_string(),
        single_word: false,
        lstrip,
        rstrip,
        normalized: false,
        special: true,
    }
}

#[test]
fn lstrip_absorbs_leading_whitespace() {
    let configs = vec![make_strip_config(1, "<s>", true, false)];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("ab   <s>cd"),
        vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
    );
}

#[test]
fn rstrip_absorbs_trailing_whitespace() {
    let configs = vec![make_strip_config(1, "<s>", false, true)];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("ab<s>   cd"),
        vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
    );
}

#[test]
fn strip_both_sides() {
    let configs = vec![make_strip_config(1, "<s>", true, true)];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("ab \t <s> \n cd"),
        vec![Segment::Text("ab"), Segment::Token(1), Segment::Text("cd")]
    );
}

#[test]
fn no_strip_keeps_whitespace() {
    let configs = vec![make_strip_config(1, "<s>", false, false)];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("ab <s> cd"),
        vec![
            Segment::Text("ab "),
            Segment::Token(1),
            Segment::Text(" cd"),
        ]
    );
}

#[test]
fn adjacent_strip_tokens_share_whitespace() {
    let configs = vec![
        make_strip_config(1, "<|im_end|>", true, true),
        make_strip_config(2, "<|im_start|>", true, true),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("hi<|im_end|>\n<|im_start|>user"),
        vec![
            Segment::Text("hi"),
            Segment::Token(1),
            Segment::Token(2),
            Segment::Text("user"),
        ]
    );
}

#[test]
fn lstrip_bounded_by_previous_token() {
    let configs = vec![
        make_strip_config(1, "<a>", false, false),
        make_strip_config(2, "<b>", true, false),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("<a><b>"),
        vec![Segment::Token(1), Segment::Token(2)]
    );
}

#[test]
fn strip_via_full_scan_path() {
    let configs = vec![
        make_strip_config(1, "<bos>", false, true),
        make_config(2, "[SEP]"),
        make_config(3, "{pad}"),
        make_config(4, "|mask|"),
    ];
    let at = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        at.split("<bos>   x"),
        vec![Segment::Token(1), Segment::Text("x")]
    );
}

#[test]
fn strip_overlap_matches_huggingface_in_both_paths() {
    let prefilter_configs = vec![
        make_strip_config(1, "<a>", false, true),
        make_strip_config(2, " X", true, false),
        make_config(3, "[d]"),
    ];
    let prefiltered = AddedTokens::from_configs(&prefilter_configs)
        .unwrap()
        .unwrap();

    let mut full_scan_configs = prefilter_configs;
    full_scan_configs.push(make_config(4, "{e}"));
    let full_scan = AddedTokens::from_configs(&full_scan_configs)
        .unwrap()
        .unwrap();

    let expected = vec![Segment::Token(1), Segment::Token(2)];
    assert_eq!(prefiltered.split("<a> X"), expected);
    assert_eq!(full_scan.split("<a> X"), expected);
}
