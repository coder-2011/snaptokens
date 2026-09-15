mod byte_fallback {
    use crate::decoders::byte_fallback::*;

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
}

mod byte_level {
    use crate::decoders::byte_level::*;
    use crate::pre_tokenizers::byte_level::BYTE_TO_CHAR;

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
}

mod replace {
    use serde_json::json;

    use crate::decoders::replace::*;

    #[test]
    fn literal_replace_decoder() {
        let dec = ReplaceDecoder::from_config(json!("▁"), " ".to_string()).unwrap();
        let out = dec.decode_chain(vec!["▁Hello".to_string(), "▁world".to_string()]);
        assert_eq!(out, vec![" Hello", " world"]);
    }

    #[test]
    fn regex_replace_decoder() {
        let dec = ReplaceDecoder::from_config(json!({"Regex": "[0-9]+"}), "#".to_string()).unwrap();
        let out = dec.decode_chain(vec!["a12b".to_string(), "34".to_string()]);
        assert_eq!(out, vec!["a#b", "#"]);
    }

    #[test]
    fn literal_empty_pattern_is_not_noop() {
        let dec = ReplaceDecoder::from_config(json!(""), "-".to_string()).unwrap();
        let out = dec.decode_chain(vec!["ab".to_string()]);
        assert_eq!(out, vec!["-a-b-"]);
    }

    #[test]
    fn regex_replacement_content_is_literal() {
        let dec =
            ReplaceDecoder::from_config(json!({"Regex": "([a-z]+)"}), "$1".to_string()).unwrap();
        let out = dec.decode_chain(vec!["abc".to_string()]);
        assert_eq!(out, vec!["$1"]);
    }
}
