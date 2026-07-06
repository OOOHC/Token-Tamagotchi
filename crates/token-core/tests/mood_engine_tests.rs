use token_core::models::{Confidence, Mood, QuotaSnapshot, QuotaSource};
use token_core::mood_engine::{
    companion_state_for_snapshot, mood_for_snapshot, mood_for_transition,
};

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
fn maps_unknown_quota_to_unknown() {
    assert_eq!(mood_for_snapshot(&snapshot(None, None)), Mood::Unknown);
}

#[test]
fn maps_healthy_quota_to_happy() {
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(81), Some(100))),
        Mood::Happy
    );
}

#[test]
fn maps_companion_state_thresholds() {
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(80), Some(100))),
        Mood::Relaxed
    );
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(50), Some(100))),
        Mood::Relaxed
    );
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(20), Some(100))),
        Mood::Concerned
    );
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(5), Some(100))),
        Mood::Panicking
    );
    assert_eq!(
        mood_for_snapshot(&snapshot(Some(0), Some(100))),
        Mood::Exhausted
    );
}

#[test]
fn returns_display_copy_for_companion() {
    let state = companion_state_for_snapshot(&snapshot(Some(4), Some(100)));

    assert_eq!(state.mood, Mood::Exhausted);
    assert_eq!(state.remaining_percent, Some(4.0));
    assert!(state.status_copy.contains("Quota exhausted"));
}

#[test]
fn returns_celebrating_for_recovered_transition() {
    let previous = snapshot(Some(4), Some(100));
    let current = snapshot(Some(60), Some(100));

    assert_eq!(mood_for_transition(&previous, &current), Mood::Celebrating);
}
