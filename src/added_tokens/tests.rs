use super::*;
use Segment::{Text, Token};

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
fn splits_preserve_text_and_added_token_boundaries() {
    let configs = [make_config(1, "<a>"), make_config(2, "<b>")];
    let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
    for (input, expected) in [
        ("", vec![]),
        ("hello world", vec![Text("hello world")]),
        ("<a>", vec![Token(1)]),
        ("<a>hello", vec![Token(1), Text("hello")]),
        ("hello<a>", vec![Text("hello"), Token(1)]),
        (
            "hello<a>world",
            vec![Text("hello"), Token(1), Text("world")],
        ),
        (
            "prefix <a> suffix",
            vec![Text("prefix "), Token(1), Text(" suffix")],
        ),
        (
            "x<a>y<b>z",
            vec![Text("x"), Token(1), Text("y"), Token(2), Text("z")],
        ),
        ("<a><b>", vec![Token(1), Token(2)]),
        ("<a><a><a>", vec![Token(1), Token(1), Token(1)]),
    ] {
        assert_eq!(added.split(input), expected, "{input:?}");
    }
    assert!(AddedTokens::from_configs(&[]).unwrap().is_none());
}

#[test]
fn shared_prefixes_choose_the_longest_token() {
    let configs = [
        make_config(1, "<"),
        make_config(2, "<file>"),
        make_config(3, "<filename>"),
    ];
    let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(
        added.split("x<filename>y<file>z<"),
        [
            Text("x"),
            Token(3),
            Text("y"),
            Token(2),
            Text("z"),
            Token(1)
        ]
    );
}

#[test]
fn unicode_and_distinct_start_bytes_match() {
    for spellings in [
        ["▁", "Ġ", "日本語", "🌍"],
        ["<bos>", "[SEP]", "{pad}", "|mask|"],
    ] {
        let configs: Vec<_> = spellings
            .iter()
            .enumerate()
            .map(|(id, text)| make_config(id as u32, text))
            .collect();
        for count in 1..=configs.len() {
            let added = AddedTokens::from_configs(&configs[..count])
                .unwrap()
                .unwrap();
            for (id, text) in spellings[..count].iter().enumerate() {
                let input = format!("hello {text} world");
                assert_eq!(
                    added.split(&input),
                    [Text("hello "), Token(id as u32), Text(" world")]
                );
                assert_eq!(added.token_to_id(text), Some(id as u32));
            }
            let expected: Vec<_> = (0..count).map(|id| Token(id as u32)).collect();
            assert_eq!(added.split(&spellings[..count].concat()), expected);
        }
    }
}

#[test]
fn vocabulary_access_preserves_ids_and_special_flags() {
    let mut special = make_config(10, "<bos>");
    special.special = true;
    let configs = [special, make_config(11, "<eos>"), make_config(12, "<pad>")];
    let added = AddedTokens::from_configs(&configs).unwrap().unwrap();
    assert_eq!(added.len(), configs.len());
    assert!(!added.is_empty());
    for config in &configs {
        assert_eq!(added.token_to_id(&config.content), Some(config.id));
        assert_eq!(added.id_to_token(config.id), Some(config.content.as_str()));
        assert_eq!(added.is_special(config.id), config.special);
    }
    assert_eq!(added.token_to_id("<unknown>"), None);
    assert_eq!(added.id_to_token(99), None);
    assert!(!added.is_special(99));
    let mut entries: Vec<_> = added.iter().collect();
    entries.sort_by_key(|entry| entry.id);
    let expected: Vec<_> = configs
        .iter()
        .map(|config| AddedTokenInfo {
            id: config.id,
            content: &config.content,
            special: config.special,
        })
        .collect();
    assert_eq!(entries, expected);
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
fn stripping_absorbs_only_the_requested_whitespace() {
    for (left, right, input, expected) in [
        (
            true,
            false,
            "ab   <s>cd",
            vec![Text("ab"), Token(1), Text("cd")],
        ),
        (
            false,
            true,
            "ab<s>   cd",
            vec![Text("ab"), Token(1), Text("cd")],
        ),
        (
            true,
            true,
            "ab \t <s> \n cd",
            vec![Text("ab"), Token(1), Text("cd")],
        ),
        (
            false,
            false,
            "ab <s> cd",
            vec![Text("ab "), Token(1), Text(" cd")],
        ),
    ] {
        let added = AddedTokens::from_configs(&[make_strip_config(1, "<s>", left, right)])
            .unwrap()
            .unwrap();
        assert_eq!(added.split(input), expected, "left={left}, right={right}");
    }
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
