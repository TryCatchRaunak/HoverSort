//! Timestamp detection.

/// Returns true if the input looks like a timestamp.
pub fn is_timestamp(input: &str) -> bool {
    let text = input.trim();

    // Unix timestamp in seconds
    if text.len() == 10 && text.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    // Unix timestamp in milliseconds
    if text.len() == 13 && text.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    // ISO-8601 (very basic detection)
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