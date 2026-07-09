//! Timestamp domain models.

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct TimestampAnalysis {
    /// Original Unix timestamp in seconds.
    pub unix_seconds: i64,

    /// Original Unix timestamp in milliseconds.
    pub unix_milliseconds: i64,

    /// Parsed UTC date and time.
    pub utc: DateTime<Utc>,
}
