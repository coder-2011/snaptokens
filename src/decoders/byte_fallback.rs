/// Decoder for Hugging Face byte-fallback tokens such as `<0x0A>`.
#[derive(Debug)]
pub struct ByteFallbackDecoder;

impl ByteFallbackDecoder {
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
