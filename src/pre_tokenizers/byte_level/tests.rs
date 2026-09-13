use std::collections::HashSet;

use super::*;

#[test]
fn table_has_256_unique_chars() {
    let mut seen = HashSet::new();
    for &c in &BYTE_TO_CHAR {
        assert!(seen.insert(c), "duplicate char: {c:?}");
    }
}

#[test]
fn nice_bytes_map_to_themselves() {
    assert_eq!(BYTE_TO_CHAR[b'!' as usize], '!');
    assert_eq!(BYTE_TO_CHAR[b'A' as usize], 'A');
    assert_eq!(BYTE_TO_CHAR[b'z' as usize], 'z');
    assert_eq!(BYTE_TO_CHAR[b'~' as usize], '~');
    assert_eq!(BYTE_TO_CHAR[0xA1], '\u{A1}');
    assert_eq!(BYTE_TO_CHAR[0xAC], '\u{AC}');
    assert_eq!(BYTE_TO_CHAR[0xAE], '\u{AE}');
    assert_eq!(BYTE_TO_CHAR[0xFF], '\u{FF}');
}

#[test]
fn remapped_bytes_start_at_256() {
    assert_eq!(BYTE_TO_CHAR[0], '\u{100}');
    assert_eq!(BYTE_TO_CHAR[b' ' as usize], 'Ġ');
    assert_eq!(BYTE_TO_CHAR[b'\n' as usize], 'Ċ');
}

#[test]
fn non_nice_count_is_68() {
    let count = BYTE_TO_CHAR.iter().filter(|&&c| c as u32 >= 256).count();
    assert_eq!(count, 68);
}

#[test]
fn encode_ascii() {
    assert_eq!(encode_bytes("Hello"), "Hello");
}

#[test]
fn encode_space() {
    assert_eq!(encode_bytes(" "), "\u{120}");
}

#[test]
fn encode_multibyte_utf8() {
    let encoded = encode_bytes("\u{20AC}");
    assert_eq!(encoded.chars().count(), 3);
    assert_eq!(
        encoded,
        format!(
            "{}{}{}",
            BYTE_TO_CHAR[0xE2], BYTE_TO_CHAR[0x82], BYTE_TO_CHAR[0xAC],
        )
    );
}

fn run(bl: &ByteLevel, input: &str) -> Vec<String> {
    let mut pts = PreTokenizedString::from_text(input);
    bl.pre_tokenize(&mut pts).unwrap();
    pts.splits()
        .iter()
        .map(|s| pts.split_text(s).to_string())
        .collect()
}

#[test]
fn simple_words() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "Hello world");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "Hello");
    assert_eq!(result[1], format!("{}world", BYTE_TO_CHAR[b' ' as usize]));
}

#[test]
fn contractions() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "I'm");
    assert_eq!(result, vec!["I", "'m"]);
}

#[test]
fn uppercase_contraction_suffix() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "'The");
    assert_eq!(result, vec!["'", "The"]);
}

#[test]
fn numbers_and_punctuation() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "price: $100");
    assert!(result.len() >= 3);
}

#[test]
fn prefix_space_added() {
    let bl = ByteLevel::from_config(true, true, true).unwrap();
    let result = run(&bl, "Hello");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], format!("{}Hello", BYTE_TO_CHAR[b' ' as usize]));
}

#[test]
fn prefix_space_not_doubled() {
    let bl = ByteLevel::from_config(true, true, true).unwrap();
    let result = run(&bl, " Hello");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], format!("{}Hello", BYTE_TO_CHAR[b' ' as usize]));
}

#[test]
fn no_regex_single_segment() {
    let bl = ByteLevel::from_config(false, true, false).unwrap();
    let result = run(&bl, "Hello world");
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        format!("Hello{}world", BYTE_TO_CHAR[b' ' as usize]),
    );
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
fn empty_input() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "");
    assert!(result.is_empty());
}

#[test]
fn empty_input_with_prefix_space() {
    let bl = ByteLevel::from_config(true, true, true).unwrap();
    let result = run(&bl, "");
    assert!(result.is_empty());
}

#[test]
fn all_whitespace() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "   ");
    assert!(!result.is_empty());
}

#[test]
fn non_ascii_input() {
    let bl = ByteLevel::from_config(false, true, true).unwrap();
    let result = run(&bl, "猫");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chars().count(), 3);
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
fn deserialize_default_config() {
    let bl: ByteLevel = serde_json::from_str("{}").unwrap();
    assert!(bl.use_regex);
    assert!(bl.add_prefix_space);
}

#[test]
fn deserialize_no_regex() {
    let bl: ByteLevel = serde_json::from_str(r#"{"use_regex":false}"#).unwrap();
    assert!(!bl.use_regex);
}
