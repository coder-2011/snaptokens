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
