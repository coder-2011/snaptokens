use super::*;

#[test]
fn llama_mask_matches_scalar() {
    fn compare<const CONTRACTIONS: bool, const DIGITS: usize>(input: &str) {
        let mut expected = Vec::new();
        scan_matches(input, scan_llama::<CONTRACTIONS, DIGITS>, |start, end| {
            expected.push((start, end));
        });
        let mut actual = Vec::new();
        let pattern = if CONTRACTIONS {
            PatternId::LlamaContraction
        } else {
            PatternId::Llama
        };
        pattern.for_each_match(input, |start, end| {
            actual.push((start, end));
        });
        assert_eq!(actual, expected, "Llama pieces diverged on {input:?}");
    }

    for input in [
        format!("m]\n/m/ is a bilabial nasal. {}", "tail ".repeat(16)),
        format!("{}HTTPResponse can't 1234567\r\n// tail", "a".repeat(61)),
        format!("{} camelCase  123456789", " ".repeat(63)),
        format!("{}éclair ١٢３", "x".repeat(64)),
        format!("{}\t\u{000b}\u{000c}\r\nword", "z".repeat(127)),
    ] {
        compare::<true, 3>(&input);
        compare::<false, 1>(&input);
    }

    let atoms = [
        "a", "Z", "0", " ", "\t", "\n", "\r", "'", ".", "/", "_", "\0", "é", "一",
    ];
    let mut state = 0x0200_0B1E_u64;
    for _ in 0..1_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let length = 70 + (state as usize % 192);
        let mut input = String::new();
        while input.len() < length {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push_str(atoms[state as usize % atoms.len()]);
        }
        compare::<true, 3>(&input);
        compare::<false, 1>(&input);
    }
}

#[test]
fn kimi_mask_matches_scalar() {
    fn compare(input: &str) {
        let mut expected = Vec::new();
        scan_matches(input, scan_kimi, |start, end| expected.push((start, end)));
        let mut actual = Vec::new();
        PatternId::Kimi.for_each_match(input, |start, end| {
            actual.push((start, end));
        });
        assert_eq!(actual, expected, "Kimi pieces diverged on {input:?}");
    }

    for input in [
        format!("{}HTTPResponse can't 1234567\r\n// tail", "a".repeat(61)),
        format!("{}中文模型/路径\r\nword", " ".repeat(63)),
        format!("{}\u{4dbf}\u{4dc0}\u{4dff}\u{4e00}東", "x".repeat(61)),
        format!("{}éclair ١٢３ 〇⼀々", "x".repeat(64)),
        format!("{}\t\u{000b}\u{000c}\r\nword", "z".repeat(127)),
    ] {
        compare(&input);
    }

    let atoms = [
        "a", "Z", "0", " ", "\t", "\n", "\r", "'", ".", "/", "_", "\0", "é", "中", "〇", "⼀",
        "々", "١", "\u{301}", "\u{2003}",
    ];
    let mut state = 0x4B49_4D49_u64;
    for _ in 0..1_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let length = 70 + (state as usize % 192);
        let mut input = String::new();
        while input.len() < length {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push_str(atoms[state as usize % atoms.len()]);
        }
        compare(&input);
    }
}

#[test]
fn gpt2_mask_matches_scalar() {
    fn compare(input: &str) {
        let mut expected = Vec::new();
        let mut start = 0;
        while start < input.len() {
            let end = advance_gpt2(input, start, false);
            expected.push((start, end));
            start = end;
        }
        let mut actual = Vec::new();
        PatternId::Gpt2.for_each_match(input, |start, end| {
            actual.push((start, end));
        });
        assert_eq!(actual, expected, "GPT-2 pieces diverged on {input:?}");
    }

    for input in [
        format!("{}café, 東京 {}", "a".repeat(60), "tail ".repeat(16)),
        format!("{}éclair ١٢３ can't", " ".repeat(63)),
        format!("{}東京's\r\nword", "x".repeat(64)),
        format!("{}\u{2003}word  \tend", "z".repeat(127)),
    ] {
        compare(&input);
    }

    let atoms = [
        "a", "Z", "0", " ", "\t", "\n", "\r", "'", ".", "_", "\0", "é", "一", "\u{2003}",
    ];
    let mut state = 0x5235_304B_u64;
    for _ in 0..1_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let length = 70 + (state as usize % 192);
        let mut input = String::new();
        while input.len() < length {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push_str(atoms[state as usize % atoms.len()]);
        }
        compare(&input);
    }
}

#[test]
fn qwen_mask_matches_scalar() {
    fn compare<const WORD_MARKS: bool, const DIGITS: usize>(input: &str) {
        let mut expected = Vec::new();
        scan_matches(input, scan_qwen::<WORD_MARKS, DIGITS>, |start, end| {
            expected.push((start, end));
        });
        let mut actual = Vec::new();
        let pattern = if WORD_MARKS {
            PatternId::QwenMark
        } else if DIGITS == 3 {
            PatternId::PhiGlm
        } else {
            PatternId::Qwen
        };
        pattern.for_each_match(input, |start, end| {
            actual.push((start, end));
        });
        assert_eq!(actual, expected, "Qwen pieces diverged on {input:?}");
    }

    for input in [
        format!("{}cafe\u{301} ١٢٣٤\r\n tail", "a".repeat(60)),
        format!("{}'ſ HTTPResponse 1234567", " ".repeat(63)),
        format!("{}東京's\r\nword", "x".repeat(64)),
        format!("{}\u{2003}\u{301}word  \tend", "z".repeat(127)),
    ] {
        compare::<false, 1>(&input);
        compare::<true, 1>(&input);
        compare::<false, 3>(&input);
    }

    let atoms = [
        "a", "Z", "0", " ", "\t", "\n", "\r", "'", ".", "_", "\0", "é", "一", "١", "\u{301}",
        "\u{2003}",
    ];
    let mut state = 0x5157_454E_u64;
    for _ in 0..1_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let length = 70 + (state as usize % 192);
        let mut input = String::new();
        while input.len() < length {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push_str(atoms[state as usize % atoms.len()]);
        }
        compare::<false, 1>(&input);
        compare::<true, 1>(&input);
        compare::<false, 3>(&input);
    }
}
