use std::borrow::Cow;

/// Decoder for Hugging Face byte-fallback tokens such as `<0x0A>`.
#[derive(Debug)]
pub struct ByteFallbackDecoder;

impl ByteFallbackDecoder {
    pub(crate) fn decode_terminal(&self, tokens: Vec<Cow<'_, str>>) -> String {
        let mut pieces = Vec::with_capacity(tokens.len());
        let mut byte_run = Vec::new();
        for token in tokens {
            if let Some(byte) = parse_byte_token(&token) {
                byte_run.push(byte);
                continue;
            }
            flush_borrowed_byte_run(&mut pieces, &mut byte_run);
            pieces.push(token);
        }
        flush_borrowed_byte_run(&mut pieces, &mut byte_run);
        if pieces.len() == 1 {
            return pieces.pop().unwrap().into_owned();
        }
        let capacity = pieces.iter().map(|piece| piece.len()).sum();
        let mut output = String::with_capacity(capacity);
        for piece in pieces {
            output.push_str(&piece);
        }
        output
    }

    /// Replaces consecutive byte-fallback tokens with decoded UTF-8 text.
    pub fn decode_chain(&self, tokens: Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::with_capacity(tokens.len());
        let mut byte_run: Vec<u8> = Vec::new();

        for token in tokens {
            if let Some(b) = parse_byte_token(&token) {
                byte_run.push(b);
                continue;
            }

            flush_byte_run(&mut out, &mut byte_run);
            out.push(token);
        }

        flush_byte_run(&mut out, &mut byte_run);
        out
    }
}

fn flush_borrowed_byte_run<'a>(out: &mut Vec<Cow<'a, str>>, byte_run: &mut Vec<u8>) {
    if byte_run.is_empty() {
        return;
    }
    match String::from_utf8(std::mem::take(byte_run)) {
        Ok(text) => out.push(Cow::Owned(text)),
        Err(error) => {
            // Any invalid byte makes the whole run produce one replacement per byte.
            for _ in 0..error.into_bytes().len() {
                out.push(Cow::Borrowed("\u{FFFD}"));
            }
        }
    }
}

fn parse_byte_token(token: &str) -> Option<u8> {
    let bytes = token.as_bytes();
    if bytes.len() != 6
        || bytes[0] != b'<'
        || bytes[1] != b'0'
        || bytes[2] != b'x'
        || bytes[5] != b'>'
    {
        return None;
    }

    u8::from_str_radix(&token[3..5], 16).ok()
}

fn flush_byte_run(out: &mut Vec<String>, byte_run: &mut Vec<u8>) {
    if byte_run.is_empty() {
        return;
    }

    let bytes = std::mem::take(byte_run);

    match String::from_utf8(bytes) {
        Ok(s) => out.push(s),
        Err(err) => {
            for _ in 0..err.into_bytes().len() {
                out.push("\u{FFFD}".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokenizers::Decoder as _;

    use super::*;

    fn check_terminal(tokens: &[String]) {
        let reference = tokenizers::decoders::byte_fallback::ByteFallback::new();
        let expected = reference.decode_chain(tokens.to_vec()).unwrap().concat();
        for mode in 0..3 {
            let mut borrowed = Vec::new();
            let mut owned = Vec::new();
            for (index, token) in tokens.iter().enumerate() {
                if mode == 0 || (mode == 2 && index % 2 == 0) {
                    borrowed.push(Cow::Borrowed(token.as_str()));
                    owned.push(token.clone());
                } else {
                    let make_owned = || {
                        let mut text = String::with_capacity(token.len() + 17);
                        text.push_str(token);
                        text
                    };
                    borrowed.push(Cow::Owned(make_owned()));
                    owned.push(make_owned());
                }
            }
            let parent = crate::decoders::join_tokens(ByteFallbackDecoder.decode_chain(owned));
            let actual = ByteFallbackDecoder.decode_terminal(borrowed);
            assert_eq!(actual, expected);
            assert_eq!(actual, parent);
            assert_eq!(actual.capacity(), parent.capacity());
        }
    }

    #[test]
    fn terminal_cow_preserves_runs_and_singleton_capacities() {
        for byte in 0..=255u8 {
            check_terminal(&[format!("<0x{byte:02x}>")]);
        }
        for bytes in [
            b"\xc3\xa9".as_slice(),
            b"\xe2\x82\xac",
            b"\xf0\x9f\x99\x82",
            b"\xf0\x90\x80x",
            b"\xc0\xaf",
            b"\xed\xa0\x80",
            b"a\xff",
        ] {
            let tokens: Vec<_> = bytes.iter().map(|byte| format!("<0x{byte:02x}>")).collect();
            check_terminal(&tokens);
        }
        for tokens in [
            vec![],
            vec![""],
            vec!["", ""],
            vec!["ordinary"],
            vec!["<0x+a>", "<0x+F>", "<0xé>"],
            vec!["<0x61>", "", "<0xff>"],
            vec!["<0xe2>", "", "<0x82>", "<0xac>"],
        ] {
            check_terminal(&tokens.into_iter().map(str::to_owned).collect::<Vec<_>>());
        }
        let mut long = vec!["<0x61>".to_owned(); 4096];
        long.push("<0xff>".to_owned());
        check_terminal(&long);
        long.insert(4096, String::new());
        check_terminal(&long);
    }

    #[test]
    fn terminal_prefix_keeps_extra_stages_and_filtered_run_boundaries() {
        use serde_json::json;

        let mut config = json!({
            "model": {"type": "BPE", "vocab": {"a": 0, "b": 1, "": 2,
                "<0x61>": 3, "<0xff>": 4}, "merges": []},
            "added_tokens": [{"id": 5, "content": "<special>", "special": true,
                "single_word": false, "lstrip": false, "rstrip": false, "normalized": false}],
            "decoder": {"type": "Sequence", "decoders": [
                {"type": "Replace", "pattern": {"String": "unused"}, "content": ""},
                {"type": "ByteFallback"}, {"type": "Fuse"}
            ]}
        });
        for extra in [false, true] {
            if extra {
                config["decoder"]["decoders"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({
                        "type": "Replace", "pattern": {"String": "ab"}, "content": "x"
                    }));
            }
            let tokenizer = crate::Tokenizer::from_json(config.clone()).unwrap();
            let reference =
                tokenizers::Tokenizer::from_bytes(serde_json::to_vec(&config).unwrap()).unwrap();
            for ids in [
                vec![0, 1],
                vec![3, 2, 4],
                vec![3, u32::MAX, 4],
                vec![3, 5, 4],
            ] {
                for skip in [false, true] {
                    assert_eq!(
                        tokenizer.decode(&ids, skip).unwrap(),
                        reference.decode(&ids, skip).unwrap()
                    );
                }
            }
            assert_eq!(
                tokenizer.decode(&[0, 1], false).unwrap(),
                if extra { "x" } else { "ab" }
            );
            assert_eq!(tokenizer.decode(&[3, 2, 4], true).unwrap(), "a�");
            assert_eq!(tokenizer.decode(&[3, 5, 4], true).unwrap(), "��");
        }
    }

    #[test]
    fn optional_plus_payload_matches_hugging_face() {
        let reference = tokenizers::decoders::byte_fallback::ByteFallback::new();
        for token in [
            "<0x+a>", "<0x+F>", "<0x+0>", "<0x0a>", "<0xAf>", "<0x+g>", "<0x-1>", "<0x++>",
            "<0x+aa>", "<0xé>",
        ] {
            let tokens = vec![token.to_owned()];
            assert_eq!(
                ByteFallbackDecoder.decode_chain(tokens.clone()),
                reference.decode_chain(tokens).unwrap(),
                "{token:?}"
            );
        }
    }

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
