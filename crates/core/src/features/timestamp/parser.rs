//! Timestamp parser.

use chrono::{DateTime, Utc};

use super::model::TimestampAnalysis;

/// Parse a Unix timestamp into a structured model.
pub fn parse(input: &str) -> Option<TimestampAnalysis> {
    let text = input.trim();

    if !text.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    if text.len() == 10 {
        let seconds = text.parse::<i64>().ok()?;

        let utc = DateTime::<Utc>::from_timestamp(seconds, 0)?;

        return Some(TimestampAnalysis {
            unix_seconds: seconds,
            unix_milliseconds: seconds * 1000,
            utc,
        });
    }

    if text.len() == 13 {
        let millis = text.parse::<i64>().ok()?;

        let seconds = millis / 1000;

        let nanos = ((millis % 1000) * 1_000_000) as u32;

        let utc = DateTime::<Utc>::from_timestamp(seconds, nanos)?;

        return Some(TimestampAnalysis {
            unix_seconds: seconds,
            unix_milliseconds: millis,
            utc,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unix_seconds() {
        let result = parse("1719668412").unwrap();

        assert_eq!(result.unix_seconds, 1719668412);
    }

    #[test]
    fn parses_unix_milliseconds() {
        let result = parse("1719668412000").unwrap();

        assert_eq!(result.unix_milliseconds, 1719668412000);
    }

    #[test]
    fn rejects_invalid_timestamp() {
        assert!(parse("hello").is_none());
    }
}
