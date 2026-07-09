//! HoverSort analysis engine.

use crate::{
    DataKind,
    features::timestamp::detector,
    models::{AnalysisData, AnalysisResult},
};

pub struct HoverSort;

impl HoverSort {
    pub fn analyze(input: &str) -> AnalysisResult {
        let kind = if detector::is_timestamp(input) {
            DataKind::Timestamp
        } else {
            DataKind::Unknown
        };

        let is_match = kind != DataKind::Unknown;

        AnalysisResult {
            original: input.to_string(),
            kind,
            is_match,
            data: AnalysisData::Unknown,
        }
    }
}
