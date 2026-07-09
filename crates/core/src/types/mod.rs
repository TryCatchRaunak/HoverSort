//! Shared enums and value types.

/// The kind of data HoverSort detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataKind {
    /// HoverSort couldn't identify the value.
    Unknown,

    /// Unix timestamp or date/time.
    Timestamp,
}
