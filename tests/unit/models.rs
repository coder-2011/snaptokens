mod unigram {
    use crate::models::unigram::Unigram;

    fn model(vocab: &[(&str, f64)], byte_fallback: bool) -> Unigram {
        Unigram::from_parts(
            vocab
                .iter()
                .map(|(token, score)| ((*token).to_string(), *score))
                .collect(),
            Some(0),
            byte_fallback,
        )
        .unwrap()
    }

    #[test]
    fn chooses_the_global_viterbi_path() {
        let unigram = model(
            &[
                ("<unk>", 0.0),
                ("a", 0.0),
                ("ab", 2.0),
                ("bc", 5.0),
                ("c", 0.0),
            ],
            false,
        );
        assert_eq!(unigram.tokenize("abc").unwrap(), vec![1, 3]);
    }

    #[test]
    fn preserves_the_first_prefix_on_equal_scores() {
        let unigram = model(
            &[("<unk>", 0.0), ("a", 0.0), ("b", 1.0), ("ab", 1.0)],
            false,
        );
        assert_eq!(unigram.tokenize("ab").unwrap(), vec![3]);
    }

    #[test]
    fn fuses_adjacent_unknown_characters() {
        let unigram = model(&[("<unk>", 0.0), ("a", 0.0)], false);
        assert_eq!(unigram.tokenize("a☃b").unwrap(), vec![1, 0]);
    }

    #[test]
    fn uses_byte_fallback_only_when_every_byte_piece_exists() {
        let unigram = model(&[("<unk>", 0.0), ("<0xC3>", 0.0), ("<0xA9>", 0.0)], true);
        assert_eq!(unigram.tokenize("é").unwrap(), vec![1, 2]);
    }

    #[test]
    fn accepts_legacy_untagged_hugging_face_json() {
        let unigram: Unigram =
            serde_json::from_str(r#"{"unk_id":0,"vocab":[["<unk>",0.0],["a",1.0]]}"#).unwrap();
        assert_eq!(unigram.tokenize("a").unwrap(), vec![1]);
    }
}
