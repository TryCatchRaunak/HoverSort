use hoversort_core::{AnalysisData, DataKind, HoverSort};

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

#[test]
fn parses_timestamp() {
    let result = HoverSort::analyze("1719668412000");

    assert_eq!(result.kind, DataKind::Timestamp);

    match result.data {
        AnalysisData::Timestamp(ts) => {
            assert_eq!(ts.unix_milliseconds, 1719668412000);
        }

        _ => panic!("Expected timestamp analysis"),
    }
}