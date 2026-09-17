use std::borrow::Cow;

use fancy_regex::{NoExpand, Regex};
use serde::de::Error as _;
use serde_json::Value;

use super::Error;

#[derive(Clone, Debug)]
enum Pattern {
    Literal(String),
    Regex(Regex),
}

impl Pattern {
    fn from_json(value: Value) -> Result<Self, Error> {
        if let Value::String(s) = value {
            return Ok(Self::Literal(s));
        }

        let Value::Object(mut obj) = value else {
            return Err(
                serde_json::Error::custom("Replace.pattern must be a string or an object").into(),
            );
        };

        if let Some(Value::String(literal)) = obj.remove("String") {
            return Ok(Self::Literal(literal));
        }

        if let Some(Value::String(regex)) = obj.remove("Regex") {
            return Ok(Self::Regex(Regex::new(&regex)?));
        }

        Err(serde_json::Error::custom("Replace.pattern object must contain String or Regex").into())
    }
}

/// A literal or regular-expression text replacement.
#[derive(Clone, Debug)]
pub struct Replace {
    pattern: Pattern,
    content: String,
}

impl Replace {
    /// Builds a replacement normalizer from a JSON pattern and replacement text.
    pub fn from_config(pattern: Value, content: String) -> Result<Self, Error> {
        Ok(Self {
            pattern: Pattern::from_json(pattern)?,
            content,
        })
    }

    /// Applies the replacement, borrowing input when no match occurs.
    pub fn normalize<'a>(&self, input: &'a str) -> Cow<'a, str> {
        match &self.pattern {
            Pattern::Literal(needle) => replace_literal(input, needle, &self.content),
            Pattern::Regex(re) => re.replace_all(input, NoExpand(&self.content)),
        }
    }
}

fn replace_literal<'a>(input: &'a str, needle: &str, replacement: &str) -> Cow<'a, str> {
    if needle.is_empty() {
        return Cow::Owned(input.replace(needle, replacement));
    }

    let mut matches = input.match_indices(needle);
    let Some((first_match, _)) = matches.next() else {
        return Cow::Borrowed(input);
    };

    let mut output = String::with_capacity(input.len());
    output.push_str(&input[..first_match]);
    output.push_str(replacement);

    let mut previous_end = first_match + needle.len();
    for (next_match, _) in matches {
        output.push_str(&input[previous_end..next_match]);
        output.push_str(replacement);
        previous_end = next_match + needle.len();
    }

    output.push_str(&input[previous_end..]);
    Cow::Owned(output)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json::json;

    use super::*;

    #[test]
    fn literal_replace() {
        let repl = Replace::from_config(json!({"String": " "}), "▁".to_string()).unwrap();
        assert_eq!(repl.normalize("a b c"), "a▁b▁c");
    }

    #[test]
    fn literal_no_change_borrowed() {
        let repl = Replace::from_config(json!({"String": "x"}), "y".to_string()).unwrap();
        let out = repl.normalize("abc");
        assert_eq!(out, "abc");
        assert!(matches!(out, Cow::Borrowed(_)));
    }

    #[test]
    fn regex_replace() {
        let repl = Replace::from_config(json!({"Regex": "\\s+"}), " ".to_string()).unwrap();
        assert_eq!(repl.normalize("hello   world"), "hello world");
    }

    #[test]
    fn accepts_plain_string_pattern() {
        let repl = Replace::from_config(json!("."), " ".to_string()).unwrap();
        assert_eq!(repl.normalize("hello.world"), "hello world");
    }

    #[test]
    fn literal_empty_pattern_is_not_noop() {
        let repl = Replace::from_config(json!(""), "-".to_string()).unwrap();
        assert_eq!(repl.normalize("ab"), "-a-b-");
    }

    #[test]
    fn regex_replacement_content_is_literal() {
        let repl = Replace::from_config(json!({"Regex": "([a-z]+)"}), "$1".to_string()).unwrap();
        assert_eq!(repl.normalize("abc"), "$1");
    }

    #[test]
    fn literal_matches_preserve_unicode_empty_and_overlap_semantics() {
        for input in ["", "ababa", "aaaa", "aé中é", "🚀🚀", "\0a\0"] {
            for needle in ["", "a", "aba", "aa", "é", "🚀", "\0", "missing"] {
                for replacement in ["", "$1", "中中"] {
                    let result = replace_literal(input, needle, replacement);
                    assert_eq!(result, input.replace(needle, replacement));
                    assert_eq!(matches!(result, Cow::Borrowed(_)), !input.contains(needle));
                }
            }
        }
    }

    #[test]
    fn pattern_object_preserves_literal_precedence_and_regex_fallback() {
        let literal =
            Replace::from_config(json!({"String": "a", "Regex": "["}), "b".into()).unwrap();
        assert_eq!(literal.normalize("a"), "b");
        let regex =
            Replace::from_config(json!({"String": null, "Regex": "a+"}), "b".into()).unwrap();
        assert_eq!(regex.normalize("aaa"), "b");
        for pattern in [
            json!(null),
            json!([]),
            json!({"String": 3}),
            json!({"Regex": false}),
        ] {
            assert!(Replace::from_config(pattern, "b".into()).is_err());
        }
    }
}
