use serde_json::json;

use super::*;
use crate::{
    json_structs::{MetaspaceConfig, MetaspacePrependScheme},
    pre_tokenized::{PreTokenizedString, Split as PtSplit},
};

#[test]
fn metaspace_preserves_ranges_around_added_tokens() {
    let metaspace = Metaspace::from_config(MetaspaceConfig {
        replacement: "▁".into(),
        add_prefix_space: None,
        prepend_scheme: Some(MetaspacePrependScheme::Never),
        split: Some(true),
    })
    .unwrap();
    let mut pts = PreTokenizedString::new(
        "ab cd!x y".into(),
        vec![
            PtSplit {
                range: 0..5,
                token_id: None,
            },
            PtSplit {
                range: 5..6,
                token_id: Some(99),
            },
            PtSplit {
                range: 6..9,
                token_id: None,
            },
        ],
    );

    metaspace.pre_tokenize(&mut pts);

    assert_eq!(pts.buffer(), "ab▁cd!x▁y");
    assert_eq!(
        pts.splits(),
        &[
            PtSplit {
                range: 0..2,
                token_id: None,
            },
            PtSplit {
                range: 2..7,
                token_id: None,
            },
            PtSplit {
                range: 7..8,
                token_id: Some(99),
            },
            PtSplit {
                range: 8..9,
                token_id: None,
            },
            PtSplit {
                range: 9..13,
                token_id: None,
            },
        ]
    );
}

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
