use super::*;

#[test]
fn decode() {
    let decoder = ByteFallbackDecoder;

    let res = decoder.decode_chain(vec!["Hey".into(), "friend!".into()]);
    assert_eq!(res, vec!["Hey", "friend!"]);

    let res = decoder.decode_chain(vec!["<0x61>".into()]);
    assert_eq!(res, vec!["a"]);

    let res = decoder.decode_chain(vec!["<0xE5>".into()]);
    assert_eq!(res, vec!["�"]);

    let res = decoder.decode_chain(vec!["<0xE5>".into(), "<0x8f>".into()]);
    assert_eq!(res, vec!["�", "�"]);

    let res = decoder.decode_chain(vec!["<0xE5>".into(), "<0x8f>".into(), "<0xab>".into()]);
    assert_eq!(res, vec!["叫"]);

    let res = decoder.decode_chain(vec![
        "<0xE5>".into(),
        "<0x8f>".into(),
        "<0xab>".into(),
        "a".into(),
    ]);
    assert_eq!(res, vec!["叫", "a"]);

    let res = decoder.decode_chain(vec!["<0xE5>".into(), "<0x8f>".into(), "a".into()]);
    assert_eq!(res, vec!["�", "�", "a"]);
}
