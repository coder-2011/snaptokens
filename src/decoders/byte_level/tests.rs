use super::*;

#[test]
fn roundtrip_ascii() {
    let dec = ByteLevelDecoder;
    let result = dec.decode_chain(vec!["Hello".to_string()]);
    assert_eq!(result, vec!["Hello"]);
}

#[test]
fn roundtrip_space() {
    let dec = ByteLevelDecoder;
    let result = dec.decode_chain(vec!["\u{120}Hello".to_string()]);
    assert_eq!(result, vec![" Hello"]);
}

#[test]
fn roundtrip_multibyte() {
    let dec = ByteLevelDecoder;
    let encoded: String = [0xE2u8, 0x82, 0xAC]
        .iter()
        .map(|&b| BYTE_TO_CHAR[b as usize])
        .collect();
    let result = dec.decode_chain(vec![encoded]);
    assert_eq!(result, vec!["€"]);
}

#[test]
fn non_gpt2_chars_preserved() {
    let dec = ByteLevelDecoder;
    let result = dec.decode_chain(vec![
        "\u{00AD}".to_string(),
        "<\u{FF5C}begin\u{2581}of\u{2581}sentence\u{FF5C}>".to_string(),
    ]);
    assert_eq!(result, vec!["\u{00AD}<｜begin▁of▁sentence｜>"]);
}
