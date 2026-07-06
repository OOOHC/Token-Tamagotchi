use codex_adapter::parser::{calculate_confidence, parse_status_output};
use token_core::models::Confidence;

#[test]
fn parses_basic_status_fixture() {
    let input = include_str!("../../../fixtures/codex/status/status-basic.txt");
    let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

    assert_eq!(snapshot.five_hour_remaining, Some(120_000));
    assert_eq!(snapshot.five_hour_limit, Some(150_000));
    assert_eq!(snapshot.total_remaining, Some(1_200_000));
    assert_eq!(snapshot.total_limit, Some(1_500_000));
    assert!(snapshot.raw_input_sha256.is_some());
    assert_eq!(snapshot.parser_warnings, vec!["Reset time was not found."]);
    assert_eq!(snapshot.confidence, Confidence::Medium);
}

#[test]
fn parses_complete_status_with_high_confidence() {
    let input = include_str!("../../../fixtures/codex/status/status-window-reset.txt");
    let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

    assert_eq!(snapshot.five_hour_remaining, Some(150_000));
    assert_eq!(snapshot.five_hour_limit, Some(150_000));
    assert_eq!(snapshot.total_remaining, Some(1_110_000));
    assert_eq!(snapshot.total_limit, Some(1_500_000));
    assert_eq!(snapshot.reset_at.as_deref(), Some("2026-07-06T01:00:00Z"));
    assert!(snapshot.parser_warnings.is_empty());
    assert_eq!(snapshot.confidence, Confidence::High);
}

#[test]
fn parses_low_quota_status_with_rate_limit_text() {
    let input = include_str!("../../../fixtures/codex/status/status-with-rate-limit.txt");
    let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

    assert_eq!(snapshot.five_hour_remaining, Some(12_000));
    assert_eq!(snapshot.five_hour_limit, Some(150_000));
    assert_eq!(snapshot.total_remaining, Some(820_000));
    assert_eq!(snapshot.total_limit, Some(1_500_000));
    assert_eq!(snapshot.reset_at.as_deref(), Some("2026-07-05T23:00:00Z"));
    assert!(snapshot.parser_warnings.is_empty());
    assert_eq!(snapshot.confidence, Confidence::High);
}

#[test]
fn parses_zero_remaining_status() {
    let input = include_str!("../../../fixtures/codex/status/status-zero-remaining.txt");
    let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

    assert_eq!(snapshot.five_hour_remaining, Some(0));
    assert_eq!(snapshot.five_hour_limit, Some(150_000));
    assert_eq!(snapshot.total_remaining, Some(500_000));
    assert_eq!(snapshot.total_limit, Some(1_500_000));
    assert_eq!(snapshot.confidence, Confidence::High);
}

#[test]
fn produces_stable_hash_for_duplicate_raw_input() {
    let input = include_str!("../../../fixtures/codex/status/status-basic.txt");
    let first = parse_status_output(input, "2026-07-05T00:00:00Z");
    let second = parse_status_output(input, "2026-07-05T00:05:00Z");

    assert_eq!(first.raw_input_sha256, second.raw_input_sha256);
    assert_ne!(first.parsed_at, second.parsed_at);
}

#[test]
fn leaves_unknown_format_empty() {
    let input = include_str!("../../../fixtures/codex/status/status-unknown-format.txt");
    let snapshot = parse_status_output(input, "2026-07-05T00:00:00Z");

    assert_eq!(snapshot.five_hour_remaining, None);
    assert_eq!(snapshot.five_hour_limit, None);
    assert_eq!(snapshot.total_remaining, None);
    assert_eq!(snapshot.total_limit, None);
    assert!(snapshot.raw_input_sha256.is_some());
    assert_eq!(snapshot.parser_warnings.len(), 5);
    assert_eq!(snapshot.confidence, Confidence::Low);
}

#[test]
fn confidence_is_high_only_without_warnings() {
    assert_eq!(
        calculate_confidence(Some(100), Some(1_000), &[]),
        Confidence::High
    );
}

#[test]
fn confidence_is_medium_for_one_main_field_and_limited_warnings() {
    let warnings = vec![
        "Total remaining quota was not found.".to_string(),
        "Reset time was not found.".to_string(),
    ];

    assert_eq!(
        calculate_confidence(Some(100), None, &warnings),
        Confidence::Medium
    );
}

#[test]
fn confidence_is_low_when_no_main_fields_parse() {
    assert_eq!(calculate_confidence(None, None, &[]), Confidence::Low);
}

#[test]
fn confidence_is_low_when_warning_count_exceeds_medium_limit() {
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
