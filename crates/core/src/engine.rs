//! HoverSort analysis engine.

use crate::{detector::Detector, models::AnalysisResult};

/// The main HoverSort analysis engine.
pub struct HoverSort;

impl HoverSort {
    /// Analyze a piece of text.
    pub fn analyze(input: &str) -> AnalysisResult {

        let kind = Detector::detect(input);
        let is_match = kind != crate::DataKind::Unknown;

        AnalysisResult {
            original: input.to_string(),
            kind,
            is_match,
        }
    }
}