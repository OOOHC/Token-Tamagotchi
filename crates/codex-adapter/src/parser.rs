use sha2::{Digest, Sha256};
use token_core::models::{Confidence, QuotaSnapshot, QuotaSource};

pub fn parse_status_output(input: &str, parsed_at: impl Into<String>) -> QuotaSnapshot {
    let five_hour_remaining = parse_labeled_number(input, "5h remaining");
    let five_hour_limit = parse_labeled_number(input, "5h limit");
    let total_remaining = parse_labeled_number(input, "total remaining");
    let total_limit = parse_labeled_number(input, "total limit");
    let reset_at = parse_labeled_text(input, "reset at");
    let parser_warnings = parser_warnings(
        five_hour_remaining,
        five_hour_limit,
        total_remaining,
        total_limit,
        reset_at.as_ref(),
    );
    let confidence = calculate_confidence(five_hour_remaining, total_remaining, &parser_warnings);

    QuotaSnapshot {
        five_hour_remaining,
        five_hour_limit,
        total_remaining,
        total_limit,
        reset_at,
        source: QuotaSource::CodexCli,
        confidence,
        parsed_at: parsed_at.into(),
        raw_input_sha256: Some(raw_input_sha256(input)),
        parser_warnings,
    }
}

fn raw_input_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn parser_warnings(
    five_hour_remaining: Option<u64>,
    five_hour_limit: Option<u64>,
    total_remaining: Option<u64>,
    total_limit: Option<u64>,
    reset_at: Option<&String>,
) -> Vec<String> {
    let mut warnings = Vec::new();

    if five_hour_remaining.is_none() {
        warnings.push("5-hour remaining quota was not found.".to_string());
    }

    if five_hour_limit.is_none() {
        warnings.push("5-hour quota limit was not found.".to_string());
    }

    if total_remaining.is_none() {
        warnings.push("Total remaining quota was not found.".to_string());
    }

    if total_limit.is_none() {
        warnings.push("Total quota limit was not found.".to_string());
    }

    if reset_at.is_none() {
        warnings.push("Reset time was not found.".to_string());
    }

    warnings
}

pub fn calculate_confidence(
    five_hour_remaining: Option<u64>,
    total_remaining: Option<u64>,
    parser_warnings: &[String],
) -> Confidence {
    let has_main_data = five_hour_remaining.is_some() && total_remaining.is_some();
    let has_any_data = five_hour_remaining.is_some() || total_remaining.is_some();
    let warning_count = parser_warnings.len();

    if has_main_data && warning_count == 0 {
        Confidence::High
    } else if has_any_data && warning_count <= 2 {
        Confidence::Medium
    } else {
        Confidence::Low
    }
}

fn parse_labeled_number(input: &str, label: &str) -> Option<u64> {
    parse_labeled_text(input, label).and_then(|value| {
        let digits: String = value.chars().filter(|char| char.is_ascii_digit()).collect();
        digits.parse().ok()
    })
}

fn parse_labeled_text(input: &str, label: &str) -> Option<String> {
    input.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;

        if key.trim().eq_ignore_ascii_case(label) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_status_fixture() {
        let input = include_str!("../../../fixtures/codex/status/status-basic.txt");
        let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

        assert_eq!(snapshot.five_hour_remaining, Some(120_000));
        assert_eq!(snapshot.five_hour_limit, Some(150_000));
        assert_eq!(snapshot.total_remaining, Some(1_200_000));
        assert!(snapshot.raw_input_sha256.is_some());
    }

    #[test]
    fn calculates_high_confidence_for_main_fields_without_warnings() {
        assert_eq!(
            calculate_confidence(Some(100), Some(1_000), &[]),
            Confidence::High
        );
    }

    #[test]
    fn calculates_medium_confidence_for_partial_data_with_limited_warnings() {
        let warnings = vec!["Total remaining quota was not found.".to_string()];

        assert_eq!(
            calculate_confidence(Some(100), None, &warnings),
            Confidence::Medium
        );
    }

    #[test]
    fn calculates_low_confidence_when_no_main_fields_are_present() {
        assert_eq!(calculate_confidence(None, None, &[]), Confidence::Low);
    }

    #[test]
    fn calculates_low_confidence_when_warning_count_is_high() {
        let warnings = vec![
            "5-hour quota limit was not found.".to_string(),
            "Total quota limit was not found.".to_string(),
            "Reset time was not found.".to_string(),
        ];

        assert_eq!(
            calculate_confidence(Some(100), Some(1_000), &warnings),
            Confidence::Low
        );
    }
}
