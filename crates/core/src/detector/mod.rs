//! Detector engine.

pub mod timestamp;

/// The detector engine.
///
/// Responsible for identifying what kind of data
/// the provided text represents.
pub struct Detector;

impl Detector {
    /// Detect the type of the provided input.
    pub fn detect(input: &str) -> crate::DataKind {
        if timestamp::is_timestamp(input) {
            crate::DataKind::Timestamp
        } else {
            crate::DataKind::Unknown
        }
    }
}