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
        if let Some(s) = value.as_str() {
            return Ok(Self::Literal(s.to_string()));
        }

        let obj = value.as_object().ok_or_else(|| {
            serde_json::Error::custom("Replace.pattern must be a string or an object")
        })?;

        if let Some(literal) = obj.get("String").and_then(Value::as_str) {
            return Ok(Self::Literal(literal.to_string()));
        }

        if let Some(regex) = obj.get("Regex").and_then(Value::as_str) {
            return Ok(Self::Regex(Regex::new(regex)?));
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
        let mut output = String::new();
        output.push_str(replacement);
        for ch in input.chars() {
            output.push(ch);
            output.push_str(replacement);
        }
        return Cow::Owned(output);
    }

    let Some(first_match) = input.find(needle) else {
        return Cow::Borrowed(input);
    };

    let mut output = String::with_capacity(input.len());
    output.push_str(&input[..first_match]);
    output.push_str(replacement);

    let mut tail = &input[first_match + needle.len()..];
    while let Some(next_match) = tail.find(needle) {
        output.push_str(&tail[..next_match]);
        output.push_str(replacement);
        tail = &tail[next_match + needle.len()..];
    }

    output.push_str(tail);
    Cow::Owned(output)
}

#[cfg(test)]
mod tests;
