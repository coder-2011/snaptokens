use crate::common::{CORPUS, load_reference_tokenizer, load_tokenizer};

// Above the single-document parallel threshold, so one ragged input is split across workers.
const DOCUMENT_BYTES: usize = 64 * 1024 + 1;

const MODELS: &[&str] = &[
    "openai-community/gpt2",
    "Qwen/Qwen3-0.6B",
    "Qwen/Qwen3.5-27B",
    "MiniMaxAI/MiniMax-M2.1",
    "deepseek-ai/DeepSeek-V3.2",
    "zai-org/GLM-4.7",
    "hoangquan456/Kimi-K2.5",
    "mistralai/Mistral-Nemo-Instruct-2407",
    "openai/gpt-oss-120b",
    "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
];

// Each unit tiles a document, so partition targets land at every phase of the unit.
const UNITS: &[&str] = &[
    "a  1",
    "x\t\t2 ",
    "x\t\t\t\t2",
    "a\n\n\n\n1",
    "word   \r\n\r\n  next",
    "it's we'LL THEY'RE you've I'd ",
    "'s's 's  't",
    "12345 678 9 ",
    "a \u{3000}中文字符 日本語テキスト ",
    "emoji 🦀🦀 👨\u{200d}👩\u{200d}👧 ok ",
    "cafe\u{301} e\u{301}\u{302} n\u{303}o \u{301}x ",
    "\r\r\n\n \t \n",
    "nonewlinesjustwords ",
    " ",
    "\t\r\n ",
    "!!!  ???\n/ ",
    "a\u{a0}b\u{2009}c\u{85}d ",
    "<p>[[a|b]] <s </s [INS <|im_star <thin <dsml </tool_cal ]~ ]! x</p>\n",
];

const ATOMS: &[&str] = &[
    "a", "Z", "é", "e\u{301}", "\u{301}", "1", "٣", " ", "  ", "\t", "\n", "\r\n", "\r", "'s",
    "'LL", ".", "/", "中", "あ", "🦀", "\u{3000}", "\u{a0}", "\0",
];

fn tiled(unit: &str, tail: usize) -> String {
    let mut text = unit.repeat(DOCUMENT_BYTES.div_ceil(unit.len()));
    text.push_str(&"q".repeat(tail));
    text
}

fn random_document(seed: u64, atoms: &[&str]) -> String {
    let mut state = seed;
    let mut text = String::new();
    while text.len() < DOCUMENT_BYTES {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        text.push_str(atoms[(state >> 33) as usize % atoms.len()]);
    }
    text
}

fn documents(added_tokens: &[String]) -> Vec<String> {
    let mut documents = Vec::new();
    for unit in UNITS {
        for tail in 0..(2 * unit.len()).min(24) {
            documents.push(tiled(unit, tail));
        }
    }
    for seed in 0..8 {
        documents.push(random_document(seed, ATOMS));
    }
    documents.push(random_document(99, &[" ", "\t", "\n", "\r\n", "\u{3000}"]));
    let mut prose = String::new();
    while prose.len() < DOCUMENT_BYTES {
        for (index, text) in CORPUS.iter().enumerate() {
            prose.push_str(text);
            prose.push_str([" ", "\n", "  ", "\r\n"][index % 4]);
        }
    }
    documents.push(prose.clone());
    let mut with_added = prose;
    for (index, token) in added_tokens.iter().enumerate() {
        let at = (index * 7919 + 1) * with_added.len() / (added_tokens.len() * 7919 + 2);
        let at = (at..).find(|&at| with_added.is_char_boundary(at)).unwrap();
        with_added.insert_str(at, token);
    }
    documents.push(with_added);
    documents
}

#[test]
fn single_document_partitions_match_hugging_face() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    for model in MODELS {
        let ours = load_tokenizer(model).unwrap();
        let reference = load_reference_tokenizer(model).unwrap();
        let mut added: Vec<_> = reference
            .get_added_tokens_decoder()
            .into_values()
            .map(|token| token.content)
            .collect();
        added.sort();
        added.truncate(6);
        for document in documents(&added) {
            let expected = reference.encode(document.as_str(), false).unwrap();
            let expected = expected.get_ids();
            let serial = ours.encode(&document, false).unwrap();
            let (ids, lengths) = pool
                .install(|| ours.encode_batch_ragged(&[document.as_str()], false))
                .unwrap();
            let prefix: String = document.chars().take(40).collect();
            assert_eq!(serial, expected, "{model} serial {prefix:?}");
            assert_eq!(ids, expected, "{model} partitioned {prefix:?}");
            assert_eq!(lengths, [expected.len()], "{model} row length");
        }
    }
}
