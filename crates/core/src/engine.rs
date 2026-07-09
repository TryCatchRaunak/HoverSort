//! HoverSort analysis engine.

use crate::{
    DataKind,
    features::timestamp::detector,
    models::{AnalysisData, AnalysisResult},
};

pub struct HoverSort;

impl HoverSort {
    pub fn analyze(input: &str) -> AnalysisResult {
        let parsed = detector::is_timestamp(input)
            .then(|| crate::features::timestamp::parser::parse(input))
            .flatten();

        match parsed {
            Some(timestamp) => AnalysisResult {
                original: input.to_string(),
                kind: DataKind::Timestamp,
                is_match: true,
                data: AnalysisData::Timestamp(timestamp),
            },

            None => AnalysisResult {
                original: input.to_string(),
                kind: DataKind::Unknown,
                is_match: false,
                data: AnalysisData::Unknown,
            },
        }
    }
}
