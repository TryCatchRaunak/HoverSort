//! Domain models used throughout HoverSort.

use crate::types::DataKind;

/// Represents the result of analyzing a piece of text.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// The original text that was analyzed.
    pub original: String,

    /// What HoverSort detected.
    pub kind: DataKind,

    /// Whether HoverSort successfully understood the value.
    pub is_match: bool,
}
