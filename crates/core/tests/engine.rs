use core::{DataKind, HoverSort};

#[test]
fn detects_timestamp() {
    let result = HoverSort::analyze("1719668412000");

    assert_eq!(result.kind, DataKind::Timestamp);
    assert!(result.is_match);
}

#[test]
fn detects_unknown_text() {
    let result = HoverSort::analyze("hello world");

    assert_eq!(result.kind, DataKind::Unknown);
    assert!(!result.is_match);
}
