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

/// Streams tokens through a hoisted literal replacement and the byte-run
/// logic, emitting exactly the pieces the Replace→ByteFallback→Fuse chain
/// concatenates.
pub(crate) fn decode_literal_replace_byte_fallback<'a>(
    needle: &str,
    replacement: &str,
    tokens: impl Iterator<Item = &'a str>,
    capacity_hint: usize,
) -> String {
    let finder = memchr::memmem::Finder::new(needle.as_bytes());
    let mut out = String::with_capacity(capacity_hint);
    let mut byte_run: Vec<u8> = Vec::new();
    let mut scratch = String::new();
    for token in tokens {
        // A valid UTF-8 needle match in valid UTF-8 always lies on char
        // boundaries, so byte offsets slice exactly like match_indices.
        let replaced: &str = if finder.find(token.as_bytes()).is_none() {
            token
        } else {
            scratch.clear();
            let mut previous_end = 0;
            for start in finder.find_iter(token.as_bytes()) {
                scratch.push_str(&token[previous_end..start]);
                scratch.push_str(replacement);
                previous_end = start + needle.len();
            }
            scratch.push_str(&token[previous_end..]);
            &scratch
        };
        if let Some(byte) = parse_byte_token(replaced) {
            byte_run.push(byte);
            continue;
        }
        flush_byte_run_into(&mut out, &mut byte_run);
        out.push_str(replaced);
    }
    flush_byte_run_into(&mut out, &mut byte_run);
    out
}

fn flush_byte_run_into(out: &mut String, byte_run: &mut Vec<u8>) {
    if byte_run.is_empty() {
        return;
    }
    match std::str::from_utf8(byte_run) {
        Ok(run) => out.push_str(run),
        // Matches flush_byte_run: an invalid run yields one replacement
        // character per byte, including any valid prefix bytes.
        Err(_) => {
            for _ in 0..byte_run.len() {
                out.push('\u{FFFD}');
            }
        }
    }
    byte_run.clear();
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

    fn hex_value(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    Some(hex_value(bytes[3])? << 4 | hex_value(bytes[4])?)
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
    use super::*;
    use crate::decoders::ReplaceDecoder;

    #[test]
    fn fused_literal_replace_matches_chain() {
        let cases: &[(&str, &str, &[&str])] = &[
            ("▁", " ", &["▁Hey", "<0xE5>", "<0x8f>", "<0xab>", "▁a▁"]),
            ("▁", " ", &["<0xE5>", "<0x8f>", "x"]),
            ("▁", " ", &["<0x61>", "<0xE5>"]),
            ("▁", " ", &["no-marker", "<0x0A>", "▁▁", "▁"]),
            // Replace runs before byte parsing, so it can demote a byte token.
            ("<0", "(", &["<0x61>", "<0xE5>", "plain"]),
            ("x", "xx", &["<0x61>", "axa"]),
        ];
        for &(needle, content, tokens) in cases {
            let replace =
                ReplaceDecoder::from_config(serde_json::json!(needle), content.to_string())
                    .unwrap();
            let replaced =
                replace.decode_chain(tokens.iter().map(|token| token.to_string()).collect());
            let chain = ByteFallbackDecoder.decode_chain(replaced).concat();
            let fused =
                decode_literal_replace_byte_fallback(needle, content, tokens.iter().copied(), 0);
            assert_eq!(fused, chain, "needle {needle:?} tokens {tokens:?}");
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
