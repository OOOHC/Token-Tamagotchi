use crate::models::{CompanionState, Mood, QuotaSnapshot};
use crate::quota_engine::five_hour_percent;

pub fn mood_for_snapshot(snapshot: &QuotaSnapshot) -> Mood {
    let Some(percent) = five_hour_percent(snapshot) else {
        return Mood::Unknown;
    };

    mood_for_percent(percent)
}

pub fn companion_state_for_snapshot(snapshot: &QuotaSnapshot) -> CompanionState {
    let remaining_percent = five_hour_percent(snapshot);
    let mood = remaining_percent.map_or(Mood::Unknown, mood_for_percent);
    let (expression, status_copy) = copy_for_mood(&mood);

    CompanionState {
        mood,
        remaining_percent,
        expression: expression.to_string(),
        status_copy: status_copy.to_string(),
    }
}

pub fn mood_for_transition(previous: &QuotaSnapshot, current: &QuotaSnapshot) -> Mood {
    let previous_mood = mood_for_snapshot(previous);
    let current_mood = mood_for_snapshot(current);

    if matches!(previous_mood, Mood::Panicking | Mood::Exhausted)
        && matches!(current_mood, Mood::Happy | Mood::Relaxed)
    {
        Mood::Celebrating
    } else {
        current_mood
    }
}

pub fn mood_for_percent(percent: f64) -> Mood {
    if percent > 80.0 {
        Mood::Happy
    } else if percent >= 50.0 {
        Mood::Relaxed
    } else if percent >= 20.0 {
        Mood::Concerned
    } else if percent >= 5.0 {
        Mood::Panicking
    } else {
        Mood::Exhausted
    }
}

pub fn copy_for_mood(mood: &Mood) -> (&'static str, &'static str) {
    match mood {
        Mood::Happy => ("(•ᴗ•)", "[Status]: Quota healthy. nom nom..."),
        Mood::Relaxed => ("(•‿•)", "[Status]: Quota stable."),
        Mood::Concerned => ("(•﹏•)", "[Status]: Quota dropping. I'm getting hungry..."),
        Mood::Panicking => (
            "(╥﹏╥)",
            "[Status]: Low Quota. Suggestion: Refactor Prompt.",
        ),
        Mood::Exhausted => (
            "(×﹏×)",
            "[Status]: Quota exhausted. Please don't send another huge prompt...",
        ),
        Mood::Celebrating => ("＼(＾▽＾)／", "[Status]: Quota restored. Breakfast!!"),
        Mood::Unknown => ("[?]", "[Status]: Waiting for quota data."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Confidence, QuotaSource};

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
    fn maps_companion_thresholds() {
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(81), Some(100))),
            Mood::Happy
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(80), Some(100))),
            Mood::Relaxed
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(50), Some(100))),
            Mood::Relaxed
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(49), Some(100))),
            Mood::Concerned
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(20), Some(100))),
            Mood::Concerned
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(19), Some(100))),
            Mood::Panicking
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(5), Some(100))),
            Mood::Panicking
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(4), Some(100))),
            Mood::Exhausted
        );
        assert_eq!(
            mood_for_snapshot(&snapshot(Some(0), Some(100))),
            Mood::Exhausted
        );
    }

    #[test]
    fn creates_display_ready_companion_state() {
        let state = companion_state_for_snapshot(&snapshot(Some(12), Some(100)));

        assert_eq!(state.mood, Mood::Panicking);
        assert_eq!(state.remaining_percent, Some(12.0));
        assert!(state.status_copy.contains("Low Quota"));
    }

    #[test]
    fn celebrates_after_recovering_from_pressure() {
        let previous = snapshot(Some(0), Some(100));
        let current = snapshot(Some(100), Some(100));

        assert_eq!(mood_for_transition(&previous, &current), Mood::Celebrating);
    }
}
