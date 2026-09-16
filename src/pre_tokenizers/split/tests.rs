use serde_json::json;
use tokenizers::{
    OffsetReferential, OffsetType, PreTokenizedString as HfPreTokenizedString, PreTokenizer,
    SplitDelimiterBehavior,
    pre_tokenizers::split::{Split as HfSplit, SplitPattern},
};

use super::*;
use crate::pre_tokenizers::scanner::PatternId;

fn hf_behavior(behavior: &str) -> SplitDelimiterBehavior {
    match behavior {
        "Removed" => SplitDelimiterBehavior::Removed,
        "Isolated" => SplitDelimiterBehavior::Isolated,
        "MergedWithPrevious" => SplitDelimiterBehavior::MergedWithPrevious,
        "MergedWithNext" => SplitDelimiterBehavior::MergedWithNext,
        "Contiguous" => SplitDelimiterBehavior::Contiguous,
        _ => panic!("unknown behavior"),
    }
}

fn hf_pieces(source: &str, behavior: &str, invert: bool, input: &str) -> Vec<String> {
    let split = HfSplit::new(
        SplitPattern::Regex(source.to_string()),
        hf_behavior(behavior),
        invert,
    )
    .unwrap();
    let mut pts = HfPreTokenizedString::from(input);
    split.pre_tokenize(&mut pts).unwrap();
    pts.get_splits(OffsetReferential::Original, OffsetType::Byte)
        .into_iter()
        .map(|(piece, _, _)| piece.to_string())
        .collect()
}

#[test]
fn literal_behaviors_match_expected_boundaries() {
    let cases = [
        ("Removed", vec!["the", "final", "countdown"]),
        ("Isolated", vec!["the", "-", "final", "-", "-", "countdown"]),
        (
            "MergedWithPrevious",
            vec!["the-", "final-", "-", "countdown"],
        ),
        ("MergedWithNext", vec!["the", "-final", "-", "-countdown"]),
        ("Contiguous", vec!["the", "-", "final", "--", "countdown"]),
    ];
    for (behavior, expected) in cases {
        let split = Split::from_config(&json!({"String": "-"}), behavior, false).unwrap();
        assert_eq!(split.split("the-final--countdown"), expected);
    }

    let literal = Split::from_config(&json!({"String": "[a]"}), "Isolated", false).unwrap();
    assert_eq!(literal.split("a[a]b"), vec!["a", "[a]", "b"]);
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
        let scanner = Split::from_config(&json!({"Regex": source}), behavior, invert).unwrap();
        for input in inputs {
            let actual: Vec<String> = if behavior == "Removed" && invert {
                let mut pts = PreTokenizedString::from_text(input);
                scanner.pre_tokenize(&mut pts).unwrap();
                pts.splits()
                    .iter()
                    .map(|split| pts.split_text(split).to_string())
                    .collect()
            } else {
                scanner
                    .split(input)
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            };
            let expected = hf_pieces(source, behavior, invert, input);
            assert_eq!(actual, expected, "pattern={source:?}, input={input:?}");
        }
    }
}

#[test]
fn invalid_regex_is_rejected() {
    let error = Split::from_config(&json!({"Regex": "(unclosed"}), "Isolated", false).unwrap_err();
    assert!(matches!(error, Error::Unsupported(_)));
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

impl Split {
    fn split<'a>(&self, input: &'a str) -> Vec<&'a str> {
        let Some(segments) = self.find_segments(input) else {
            return if input.is_empty() || (self.behavior == SplitBehavior::Removed && self.invert) {
                Vec::new()
            } else {
                vec![input]
            };
        };
        let mut pieces = Vec::new();
        self.apply_behavior(&segments, |start, end| {
            if start < end {
                pieces.push(&input[start..end]);
            }
        });
        pieces
    }
}

#[test]
fn literal_ranges_match_hf_for_every_behavior_and_inversion() {
    for behavior in [
        "Removed",
        "Isolated",
        "MergedWithPrevious",
        "MergedWithNext",
        "Contiguous",
    ] {
        for invert in [false, true] {
            for literal in ["-", "▁", "::"] {
                let split =
                    Split::from_config(&json!({"String": literal}), behavior, invert).unwrap();
                let reference = HfSplit::new(
                    SplitPattern::String(literal.to_string()),
                    hf_behavior(behavior),
                    invert,
                )
                .unwrap();
                for input in [
                    "",
                    "x",
                    "café中文",
                    "-leading",
                    "trailing-",
                    "a--b",
                    "é▁中▁",
                    "::a::::b::",
                ] {
                    let mut actual = PreTokenizedString::from_text(input);
                    split.pre_tokenize(&mut actual).unwrap();
                    let actual: Vec<_> = actual
                        .splits()
                        .iter()
                        .map(|piece| {
                            (
                                actual.split_text(piece).to_string(),
                                (piece.range.start, piece.range.end),
                            )
                        })
                        .collect();
                    let mut expected = HfPreTokenizedString::from(input);
                    reference.pre_tokenize(&mut expected).unwrap();
                    let expected: Vec<_> = expected
                        .get_splits(OffsetReferential::Original, OffsetType::Byte)
                        .into_iter()
                        .map(|(text, range, _)| (text.to_string(), range))
                        .collect();
                    assert_eq!(
                        actual, expected,
                        "{behavior} invert={invert} literal={literal:?} input={input:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn absent_matches_have_no_segment_allocation() {
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
    for behavior in [
        "Removed",
        "Isolated",
        "MergedWithPrevious",
        "MergedWithNext",
        "Contiguous",
    ] {
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
fn unrecognized_regex_falls_back_to_fancy_regex() {
    let split = Split::from_config(&json!({"Regex": "\\d+"}), "Isolated", false).unwrap();
    assert!(matches!(split.matcher, Matcher::Regex(_)));
    assert!(split.pattern_id().is_none());
    let mut matches = Vec::new();
    split
        .matcher
        .for_each_match("abc123def456", |s, e| matches.push((s, e)));
    assert_eq!(matches, [(3, 6), (9, 12)]);
}

#[test]
fn regex_fallback_matches_hf_on_semantic_edges() {
    for source in [r"\d+", r"(?=a)", r"(?<=a)", r"z+"] {
        for behavior in [
            "Removed",
            "Isolated",
            "MergedWithPrevious",
            "MergedWithNext",
            "Contiguous",
        ] {
            for invert in [false, true] {
                let split =
                    Split::from_config(&json!({"Regex": source}), behavior, invert).unwrap();
                for input in ["", "abc123def456", "café中", "a", "baa"] {
                    let mut actual = PreTokenizedString::from_text(input);
                    split.pre_tokenize(&mut actual).unwrap();
                    let actual: Vec<_> = actual
                        .splits()
                        .iter()
                        .map(|piece| actual.split_text(piece).to_string())
                        .collect();
                    assert_eq!(
                        actual,
                        hf_pieces(source, behavior, invert, input),
                        "source={source:?} behavior={behavior} invert={invert} input={input:?}"
                    );
                }
            }
        }
    }
}
