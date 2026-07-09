//! Timestamp detection.

/// Returns `true` if the input looks like a timestamp.
///
/// Currently supported:
/// - Unix timestamp (10 digits)
/// - Unix timestamp in milliseconds (13 digits)
/// - Basic ISO-8601 timestamps ending with `Z`
pub fn is_timestamp(input: &str) -> bool {
    let text = input.trim();

    // Unix timestamp (seconds)
    if text.len() == 10 && text.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    // Unix timestamp (milliseconds)
    if text.len() == 13 && text.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    // Very basic ISO-8601 detection
    if text.contains('T') && text.ends_with('Z') {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unix_seconds() {
        assert!(is_timestamp("1719668412"));
    }

    #[test]
    fn detects_unix_milliseconds() {
        assert!(is_timestamp("1719668412000"));
    }

    #[test]
    fn detects_iso8601() {
        assert!(is_timestamp("2026-07-09T12:30:00Z"));
    }

    #[test]
    fn rejects_random_text() {
        assert!(!is_timestamp("hello world"));
    }
}
