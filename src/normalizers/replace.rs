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
    /// Consume literal spellings while preserving String-before-Regex precedence.
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

/// Replace non-overlapping matches while borrowing unchanged input.
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
mod tests;
