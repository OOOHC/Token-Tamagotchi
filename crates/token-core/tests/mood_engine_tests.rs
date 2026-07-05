use token_core::models::{Confidence, Mood, QuotaSnapshot, QuotaSource};
use token_core::mood_engine::mood_for_snapshot;

fn snapshot(remaining: Option<u64>, limit: Option<u64>) -> QuotaSnapshot {
    QuotaSnapshot {
        five_hour_remaining: remaining,
        five_hour_limit: limit,
        total_remaining: None,
        total_limit: None,
        reset_at: None,
        source: QuotaSource::Mock,
        confidence: Confidence::Medium,
        parsed_at: "2026-07-05T00:00:00Z".to_string(),
        raw_input_sha256: None,
        parser_warnings: Vec::new(),
    }
}

#[test]
fn maps_unknown_quota_to_sleeping() {
    assert_eq!(mood_for_snapshot(&snapshot(None, None)), Mood::Sleeping);
}

#[test]
fn maps_healthy_quota_to_happy() {
    assert_eq!(mood_for_snapshot(&snapshot(Some(80), Some(100))), Mood::Happy);
}
