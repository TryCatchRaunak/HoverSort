//! Timestamp domain models.

#[derive(Debug, Clone)]
pub struct TimestampAnalysis {
    pub unix_seconds: Option<i64>,
    pub unix_milliseconds: Option<i64>,
}
