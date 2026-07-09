//! Domain models used throughout HoverSort.

use crate::{features::timestamp::TimestampAnalysis, types::DataKind};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub original: String,
    pub kind: DataKind,
    pub is_match: bool,
    pub data: AnalysisData,
}

#[derive(Debug, Clone)]
pub enum AnalysisData {
    Unknown,
    Timestamp(TimestampAnalysis),
}
