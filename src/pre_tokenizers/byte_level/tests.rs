use super::*;

#[test]
fn byte_alphabet_and_encoding_match_gpt2() {
    let alphabet: std::collections::HashSet<_> = BYTE_TO_CHAR.into_iter().collect();
    assert_eq!(alphabet.len(), 256);
    for byte in [b'!', b'A', b'z', b'~', 0xA1, 0xAC, 0xAE, 0xFF] {
        assert_eq!(BYTE_TO_CHAR[byte as usize], char::from(byte));
    }
    for (input, expected) in [("Hello", "Hello"), (" ", "Ġ"), ("\0\n", "ĀĊ"), ("€", "âĤ¬")]
    {
        assert_eq!(encode_bytes(input), expected, "{input:?}");
    }
}

#[test]
fn prefix_and_regex_options_preserve_pieces() {
    for (prefix, regex, input, expected) in [
        (false, true, "Hello world", vec!["Hello", "Ġworld"]),
        (false, true, "I'm", vec!["I", "'m"]),
        (false, true, "'The", vec!["'", "The"]),
        (false, true, "price: $100", vec!["price", ":", "Ġ$", "100"]),
        (true, true, "Hello", vec!["ĠHello"]),
        (true, true, " Hello", vec!["ĠHello"]),
        (false, false, "Hello world", vec!["HelloĠworld"]),
        (false, true, "", vec![]),
        (true, true, "", vec![]),
        (false, true, "   ", vec!["ĠĠĠ"]),
        (false, true, "猫", vec!["çĮ«"]),
    ] {
        let byte_level = ByteLevel::from_config(prefix, true, regex).unwrap();
        let mut text = PreTokenizedString::from_text(input);
        byte_level.pre_tokenize(&mut text).unwrap();
        let actual: Vec<_> = text
            .splits()
            .iter()
            .map(|split| text.split_text(split))
            .collect();
        assert_eq!(
            actual, expected,
            "prefix={prefix} regex={regex} input={input:?}"
        );
    }
}

#[test]
fn bulk_encoding_supports_overlapping_splits() {
    let bl = ByteLevel::from_config(false, true, false).unwrap();
    let split = PtSplit {
        range: 0..3,
        token_id: None,
    };
    let mut pts = PreTokenizedString::new("abc".into(), vec![split; 4]);

    bl.pre_tokenize(&mut pts).unwrap();

    assert_eq!(pts.buffer(), "abcabcabcabc");
    assert_eq!(
        pts.splits()
            .iter()
            .map(|split| split.range.clone())
            .collect::<Vec<_>>(),
        vec![0..3, 3..6, 6..9, 9..12]
    );
}

#[test]
fn added_token_splits_preserved() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let buffer = "hello<sep>world".to_string();
    let splits = vec![
        PtSplit {
            range: 0..5,
            token_id: None,
        },
        PtSplit {
            range: 5..10,
            token_id: Some(42),
        },
        PtSplit {
            range: 10..15,
            token_id: None,
        },
    ];
    let mut pts = PreTokenizedString::new(buffer, splits);
    bl.pre_tokenize(&mut pts).unwrap();

    let added = pts
        .splits()
        .iter()
        .find(|s| s.token_id == Some(42))
        .expect("added token split missing");
    assert_eq!(pts.split_text(added), encode_bytes("<sep>"));
}

#[test]
fn deserialization_preserves_defaults_and_overrides() {
    for (json, regex) in [("{}", true), (r#"{"use_regex":false}"#, false)] {
        let byte_level: ByteLevel = serde_json::from_str(json).unwrap();
        assert_eq!(byte_level.use_regex, regex);
        assert!(byte_level.add_prefix_space);
    }
}
