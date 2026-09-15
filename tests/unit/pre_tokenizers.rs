use serde_json::json;

use crate::pre_tokenizers::*;

#[test]
fn deepseek_fused_scanner_matches_split_chain() {
    let patterns = [
        scanner::PatternId::DeepSeekNumber.pattern(),
        scanner::PatternId::DeepSeekCjk.pattern(),
        scanner::PatternId::DeepSeekMain.pattern(),
    ];
    let steps: Vec<_> = patterns
        .into_iter()
        .map(|pattern| {
            PreTokenizer::Split(
                Split::from_config(&json!({"Regex": pattern}), "Isolated", false).unwrap(),
            )
        })
        .collect();
    let fused = FusedSplits { steps: &steps };

    let compare = |input: &str| {
        let mut expected = Vec::new();
        visit_fused_splits(&steps, input, 0, input.len(), &mut |_, start, end| {
            expected.push((start, end));
        });
        let mut actual = Vec::new();
        fused.for_each_piece(input, |_, start, end| actual.push((start, end)));
        assert_eq!(actual, expected, "DeepSeek pieces diverged on {input:?}");
    };

    for input in [
        "a  1 日本語・テスト\n",
        "cafe\u{301} \u{301}leading ١٢٣٤",
        "x\0\0b \u{200b}word 3.14159",
        "\u{3040}あ ゛カナ 一二三１２３",
        "@user #tag <|endoftext|>",
    ] {
        compare(input);
    }

    let atoms = [
        "a", "Z", "é", "١", " ", "\t", "\n", "\u{301}", ".", "☃", "\0", "\u{200b}", "一", "あ",
        "・",
    ];
    let mut state = 0xDEE9_5EEC_u64;
    for _ in 0..1_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let length = 1 + (state as usize % 39);
        let mut input = String::new();
        for _ in 0..length {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push_str(atoms[state as usize % atoms.len()]);
        }
        compare(&input);
    }
}

mod byte_level_tests {
    use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};
    use std::collections::HashSet;

    use crate::pre_tokenizers::byte_level::*;

    #[test]
    fn table_has_256_unique_chars() {
        let mut seen = HashSet::new();
        for &c in &BYTE_TO_CHAR {
            assert!(seen.insert(c), "duplicate char: {c:?}");
        }
    }

    #[test]
    fn nice_bytes_map_to_themselves() {
        assert_eq!(BYTE_TO_CHAR[b'!' as usize], '!');
        assert_eq!(BYTE_TO_CHAR[b'A' as usize], 'A');
        assert_eq!(BYTE_TO_CHAR[b'z' as usize], 'z');
        assert_eq!(BYTE_TO_CHAR[b'~' as usize], '~');
        assert_eq!(BYTE_TO_CHAR[0xA1], '\u{A1}');
        assert_eq!(BYTE_TO_CHAR[0xAC], '\u{AC}');
        assert_eq!(BYTE_TO_CHAR[0xAE], '\u{AE}');
        assert_eq!(BYTE_TO_CHAR[0xFF], '\u{FF}');
    }

    #[test]
    fn remapped_bytes_start_at_256() {
        assert_eq!(BYTE_TO_CHAR[0], '\u{100}');
        assert_eq!(BYTE_TO_CHAR[b' ' as usize], 'Ġ');
        assert_eq!(BYTE_TO_CHAR[b'\n' as usize], 'Ċ');
    }

    #[test]
    fn non_nice_count_is_68() {
        let count = BYTE_TO_CHAR.iter().filter(|&&c| c as u32 >= 256).count();
        assert_eq!(count, 68);
    }

    #[test]
    fn encode_ascii() {
        assert_eq!(encode_bytes("Hello"), "Hello");
    }

    #[test]
    fn encode_space() {
        assert_eq!(encode_bytes(" "), "\u{120}");
    }

    #[test]
    fn encode_multibyte_utf8() {
        let encoded = encode_bytes("\u{20AC}");
        assert_eq!(encoded.chars().count(), 3);
        assert_eq!(
            encoded,
            format!(
                "{}{}{}",
                BYTE_TO_CHAR[0xE2], BYTE_TO_CHAR[0x82], BYTE_TO_CHAR[0xAC],
            )
        );
    }

    fn run(bl: &ByteLevel, input: &str) -> Vec<String> {
        let mut pts = PreTokenizedString::from_text(input);
        bl.pre_tokenize(&mut pts).unwrap();
        pts.splits()
            .iter()
            .map(|s| pts.split_text(s).to_string())
            .collect()
    }

    #[test]
    fn simple_words() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "Hello world");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "Hello");
        assert_eq!(result[1], format!("{}world", BYTE_TO_CHAR[b' ' as usize]));
    }

    #[test]
    fn contractions() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "I'm");
        assert_eq!(result, vec!["I", "'m"]);
    }

    #[test]
    fn uppercase_contraction_suffix() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "'The");
        assert_eq!(result, vec!["'", "The"]);
    }

    #[test]
    fn numbers_and_punctuation() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "price: $100");
        assert!(result.len() >= 3);
    }

    #[test]
    fn prefix_space_added() {
        let bl = ByteLevel::from_config(true, true, true).unwrap();
        let result = run(&bl, "Hello");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], format!("{}Hello", BYTE_TO_CHAR[b' ' as usize]));
    }

    #[test]
    fn prefix_space_not_doubled() {
        let bl = ByteLevel::from_config(true, true, true).unwrap();
        let result = run(&bl, " Hello");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], format!("{}Hello", BYTE_TO_CHAR[b' ' as usize]));
    }

    #[test]
    fn no_regex_single_segment() {
        let bl = ByteLevel::from_config(false, true, false).unwrap();
        let result = run(&bl, "Hello world");
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            format!("Hello{}world", BYTE_TO_CHAR[b' ' as usize]),
        );
    }

    #[test]
    fn bulk_encoding_supports_overlapping_splits() {
        let bl = ByteLevel::from_config(false, true, false).unwrap();
        let split = PtSplit {
            range: 0..3,
            token_id: None,
        };
        let mut pts = PreTokenizedString::new("abc".into(), vec![split; 4]);

        bl.pre_tokenize(&mut pts).unwrap();

        assert_eq!(pts.buffer(), "abcabcabcabc");
        assert_eq!(
            pts.splits()
                .iter()
                .map(|split| split.range.clone())
                .collect::<Vec<_>>(),
            vec![0..3, 3..6, 6..9, 9..12]
        );
    }

    #[test]
    fn empty_input() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "");
        assert!(result.is_empty());
    }

    #[test]
    fn empty_input_with_prefix_space() {
        let bl = ByteLevel::from_config(true, true, true).unwrap();
        let result = run(&bl, "");
        assert!(result.is_empty());
    }

    #[test]
    fn all_whitespace() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "   ");
        assert!(!result.is_empty());
    }

    #[test]
    fn non_ascii_input() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let result = run(&bl, "猫");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].chars().count(), 3);
    }

    #[test]
    fn added_token_splits_preserved() {
        let bl = ByteLevel::from_config(false, true, true).unwrap();
        let buffer = "hello<sep>world".to_string();
        let splits = vec![
            PtSplit {
                range: 0..5,
                token_id: None,
            },
            PtSplit {
                range: 5..10,
                token_id: Some(42),
            },
            PtSplit {
                range: 10..15,
                token_id: None,
            },
        ];
        let mut pts = PreTokenizedString::new(buffer, splits);
        bl.pre_tokenize(&mut pts).unwrap();

        let added = pts
            .splits()
            .iter()
            .find(|s| s.token_id == Some(42))
            .expect("added token split missing");
        assert_eq!(pts.split_text(added), encode_bytes("<sep>"));
    }

    #[test]
    fn deserialize_default_config() {
        let bl: ByteLevel = serde_json::from_str("{}").unwrap();
        assert!(bl.use_regex);
        assert!(bl.add_prefix_space);
    }

    #[test]
    fn deserialize_no_regex() {
        let bl: ByteLevel = serde_json::from_str(r#"{"use_regex":false}"#).unwrap();
        assert!(!bl.use_regex);
    }
}

mod split_tests {
    use crate::pre_tokenized::{PreTokenizedString, Split as PtSplit};
    use crate::pre_tokenizers::Error;
    use crate::pre_tokenizers::split::Matcher;
    use serde_json::json;
    use tokenizers::{
        OffsetReferential, OffsetType, PreTokenizedString as HfPreTokenizedString, PreTokenizer,
        SplitDelimiterBehavior,
        pre_tokenizers::split::{Split as HfSplit, SplitPattern},
    };

    use crate::pre_tokenizers::scanner::PatternId;
    use crate::pre_tokenizers::split::*;

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
        let error =
            Split::from_config(&json!({"Regex": "(unclosed"}), "Isolated", false).unwrap_err();
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
                return if input.is_empty()
                    || (self.behavior == SplitBehavior::Removed && self.invert)
                {
                    Vec::new()
                } else {
                    vec![input]
                };
            };
            self.apply_behavior(&segments)
                .into_iter()
                .filter(|&(start, end)| start < end)
                .map(|(start, end)| &input[start..end])
                .collect()
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
                Split::from_config(&json!({"String": literal}), "MergedWithPrevious", false)
                    .unwrap();
            for input in ["", "a", "café中文"] {
                assert!(split.find_segments(input).is_none());
            }
        }
        let split =
            Split::from_config(&json!({"String": "-"}), "MergedWithPrevious", false).unwrap();
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
}
