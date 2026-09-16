use super::*;
use crate::pre_tokenizers::scanner::PatternId;
use serde_json::json;
use tokenizers::{
    OffsetReferential, OffsetType, PreTokenizedString as HfPreTokenizedString, PreTokenizer,
    pre_tokenizers::split::Split as HfSplit,
};

const BEHAVIORS: &[&str] = &[
    "Removed",
    "Isolated",
    "MergedWithPrevious",
    "MergedWithNext",
    "Contiguous",
];

#[test]
fn unmatched_input_needs_no_segments() {
    // None lets pre_tokenize skip the intermediate segment and behavior buffers.
    for literal in ["-", ""] {
        let split =
            Split::from_config(&json!({"String": literal}), "MergedWithPrevious", false).unwrap();
        for input in ["", "a", "café中文"] {
            assert!(split.find_segments(input).is_none());
        }
    }
    let split = Split::from_config(&json!({"String": "-"}), "MergedWithPrevious", false).unwrap();
    assert!(split.find_segments("-a").is_some());
}

fn assert_matches_hf(pattern: &Value, behavior: &str, invert: bool, input: &str) {
    let split = Split::from_config(pattern, behavior, invert).unwrap();
    let reference: HfSplit = serde_json::from_value(json!({
        "type": "Split", "pattern": pattern, "behavior": behavior, "invert": invert,
    }))
    .unwrap();
    let mut actual = PreTokenizedString::from_text(input);
    split.pre_tokenize(&mut actual).unwrap();
    let actual: Vec<_> = actual
        .splits()
        .iter()
        .map(|piece| {
            (
                actual.split_text(piece),
                (piece.range.start, piece.range.end),
            )
        })
        .collect();
    let mut expected = HfPreTokenizedString::from(input);
    reference.pre_tokenize(&mut expected).unwrap();
    let expected: Vec<_> = expected
        .get_splits(OffsetReferential::Original, OffsetType::Byte)
        .into_iter()
        .map(|(text, range, _)| (text, range))
        .collect();
    assert_eq!(
        actual, expected,
        "{pattern} {behavior} invert={invert} input={input:?}"
    );
}

#[test]
fn literal_and_regex_ranges_match_hf() {
    for pattern in [
        json!({"String":"-"}),
        json!({"String":"▁"}),
        json!({"String":"::"}),
        json!({"String":"[a]"}),
        json!({"Regex":r"\d+"}),
        json!({"Regex":r"(?=a)"}),
        json!({"Regex":r"(?<=a)"}),
        json!({"Regex":r"z+"}),
    ] {
        for behavior in BEHAVIORS {
            for invert in [false, true] {
                for input in [
                    "",
                    "x",
                    "café中文",
                    "-leading",
                    "trailing-",
                    "a--b",
                    "é▁中▁",
                    "::a::::b::",
                    "a[a]b",
                    "abc123def456",
                    "a",
                    "baa",
                    "the-final--countdown",
                ] {
                    assert_matches_hf(&pattern, behavior, invert, input);
                }
            }
        }
    }
}

#[test]
fn fixed_scanners_match_hugging_face_on_semantic_edges() {
    let patterns = [
        (PatternId::Llama.pattern(), "Isolated", false),
        (PatternId::LlamaContraction.pattern(), "Isolated", false),
        (PatternId::LlamaContraction.pattern(), "Removed", true),
        (PatternId::Kimi.pattern(), "Isolated", false),
        (PatternId::Kimi.pattern(), "MergedWithPrevious", false),
        (PatternId::Qwen.pattern(), "Isolated", false),
        (PatternId::QwenMark.pattern(), "Isolated", false),
        (PatternId::PhiGlm.pattern(), "Isolated", false),
        (PatternId::PhiGlm.pattern(), "Removed", true),
        (PatternId::DeepSeekNumber.pattern(), "Isolated", false),
        (PatternId::DeepSeekCjk.pattern(), "Isolated", false),
        (PatternId::DeepSeekMain.pattern(), "Isolated", false),
        (PatternId::Gpt2.pattern(), "Isolated", false),
        (PatternId::Gpt2CaseInsensitive.pattern(), "Isolated", false),
    ];
    let inputs = [
        "I'm DON'T we're HTTPServer camelCase 12345",
        " café\u{301} \u{01c5}uro 你好世界! \t\n  trailing  ",
        "a.\n\nx",
        "$True foo\tbar\r\n/baz 🚀",
        "\u{216b}\u{b2} numbers\u{85}\u{a0}space\u{2028}line",
        "\u{300}\u{301}\u{302}",
        "汉字𠀀かなカナ mixedHan字word",
        "\u{3}\u{b}BASKETBALL",
    ];

    for (source, behavior, invert) in patterns {
        for input in inputs {
            assert_matches_hf(&json!({"Regex": source}), behavior, invert, input);
        }
    }
}

#[test]
fn invalid_regex_is_rejected() {
    let error = Split::from_config(&json!({"Regex": "(unclosed"}), "Isolated", false).unwrap_err();
    assert!(matches!(error, Error::Unsupported(_)));
}

#[test]
fn no_match_preserves_protected_ranges_for_every_behavior() {
    let original = vec![
        PtSplit {
            range: 0..2,
            token_id: None,
        },
        PtSplit {
            range: 2..3,
            token_id: Some(u32::MAX),
        },
        PtSplit {
            range: 3..6,
            token_id: None,
        },
        PtSplit {
            range: 6..6,
            token_id: Some(0),
        },
    ];
    for &behavior in BEHAVIORS {
        for invert in [false, true] {
            let split = Split::from_config(&json!({"String": "-"}), behavior, invert).unwrap();
            let mut ranges = original.clone();
            ranges.push(PtSplit {
                range: 6..6,
                token_id: None,
            });
            let mut pts = PreTokenizedString::new("é|中".to_string(), ranges);
            split.pre_tokenize(&mut pts).unwrap();
            let expected: Vec<_> = original
                .iter()
                .filter(|piece| behavior != "Removed" || !invert || piece.token_id.is_some())
                .cloned()
                .collect();
            assert_eq!(pts.splits(), expected, "{behavior} invert={invert}");
            assert_eq!(pts.buffer(), "é|中");
        }
    }
}

#[test]
fn pre_tokenize_preserves_added_tokens() {
    let split = Split::from_config(&json!({"String": " "}), "Removed", false).unwrap();
    let mut pts = PreTokenizedString::new(
        "hello world".to_string(),
        vec![
            PtSplit {
                range: 0..5,
                token_id: None,
            },
            PtSplit {
                range: 5..5,
                token_id: Some(42),
            },
            PtSplit {
                range: 5..11,
                token_id: None,
            },
        ],
    );
    split.pre_tokenize(&mut pts).unwrap();
    assert!(pts.splits().iter().any(|piece| piece.token_id == Some(42)));
}
