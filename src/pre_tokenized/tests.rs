use super::*;

#[test]
fn inner_parallelism_scope_restores_state() {
    let enabled = || INNER_PARALLELISM.with(Cell::get);
    assert!(enabled());

    without_inner_parallelism(|| {
        assert!(!enabled());
        without_inner_parallelism(|| assert!(!enabled()));
        assert!(!enabled());
    });
    assert!(enabled());

    let panic = std::panic::catch_unwind(|| {
        without_inner_parallelism(|| panic!("test panic"));
    });
    assert!(panic.is_err());
    assert!(enabled());
}

#[test]
fn from_text_empty() {
    let pts = PreTokenizedString::from_text("");
    assert!(pts.splits().is_empty());
    assert!(pts.buffer().is_empty());
}

#[test]
fn from_text_single_span() {
    let pts = PreTokenizedString::from_text("hello world");
    assert_eq!(pts.splits().len(), 1);
    assert_eq!(pts.split_text(&pts.splits()[0]), "hello world");
    assert_eq!(pts.splits()[0].token_id, None);
}

#[test]
fn new_with_mixed_splits() {
    let buffer = "hello<sep>world".to_string();
    let splits = vec![
        Split {
            range: 0..5,
            token_id: None,
        },
        Split {
            range: 5..10,
            token_id: Some(42),
        },
        Split {
            range: 10..15,
            token_id: None,
        },
    ];
    let pts = PreTokenizedString::new(buffer, splits);
    assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
    assert_eq!(pts.split_text(&pts.splits()[1]), "<sep>");
    assert_eq!(pts.splits()[1].token_id, Some(42));
    assert_eq!(pts.split_text(&pts.splits()[2]), "world");
}

#[test]
fn set_buffer_replaces() {
    let mut pts = PreTokenizedString::from_text("old");
    pts.set_buffer(
        "new text".to_string(),
        vec![Split {
            range: 0..3,
            token_id: None,
        }],
    );
    assert_eq!(pts.buffer(), "new text");
    assert_eq!(pts.split_text(&pts.splits()[0]), "new");
}

#[test]
fn refine_splits_keeps_buffer() {
    let mut pts = PreTokenizedString::from_text("hello world");
    pts.refine_splits(vec![
        Split {
            range: 0..5,
            token_id: None,
        },
        Split {
            range: 5..11,
            token_id: None,
        },
    ]);
    assert_eq!(pts.buffer(), "hello world");
    assert_eq!(pts.split_text(&pts.splits()[0]), "hello");
    assert_eq!(pts.split_text(&pts.splits()[1]), " world");
}

#[test]
fn tokenize_text_splits() {
    let pts = PreTokenizedString::from_text("ab");
    let ids = pts
        .tokenize(|text, out| {
            out.extend(text.bytes().map(u32::from));
            Ok(())
        })
        .unwrap();
    assert_eq!(ids, vec![97, 98]);
}

#[test]
fn tokenize_mixed_splits() {
    let buffer = "helloXworld".to_string();
    let splits = vec![
        Split {
            range: 0..5,
            token_id: None,
        },
        Split {
            range: 5..6,
            token_id: Some(99),
        },
        Split {
            range: 6..11,
            token_id: None,
        },
    ];
    let pts = PreTokenizedString::new(buffer, splits);
    let ids = pts
        .tokenize(|text, out| {
            out.push(text.len() as u32);
            Ok(())
        })
        .unwrap();
    assert_eq!(ids, vec![5, 99, 5]);
}

#[test]
fn tokenize_empty() {
    let pts = PreTokenizedString::from_text("");
    let ids = pts
        .tokenize(|_, out| {
            out.push(1);
            Ok(())
        })
        .unwrap();
    assert!(ids.is_empty());
}

#[test]
fn tokenize_propagates_error() {
    let pts = PreTokenizedString::from_text("x");
    let err = pts.tokenize(|_, _out| Err("boom".to_string())).unwrap_err();
    assert_eq!(err, "boom");
}
