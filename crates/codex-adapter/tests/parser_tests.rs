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
