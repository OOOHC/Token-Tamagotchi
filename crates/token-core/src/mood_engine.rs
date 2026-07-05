use crate::models::{Mood, QuotaSnapshot};

pub fn mood_for_snapshot(snapshot: &QuotaSnapshot) -> Mood {
    let Some(remaining) = snapshot.five_hour_remaining else {
        return Mood::Sleeping;
    };
    let Some(limit) = snapshot.five_hour_limit else {
        return Mood::Sleeping;
    };

    if limit == 0 {
        return Mood::Sleeping;
    }

    let ratio = remaining as f64 / limit as f64;

    if ratio <= 0.15 {
        Mood::Tired
    } else if ratio <= 0.5 {
        Mood::Focused
    } else {
        Mood::Happy
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
    fn maps_unknown_quota_to_sleeping() {
        assert_eq!(mood_for_snapshot(&snapshot(None, None)), Mood::Sleeping);
    }

    #[test]
    fn maps_low_quota_to_tired() {
        assert_eq!(mood_for_snapshot(&snapshot(Some(10), Some(100))), Mood::Tired);
    }
}
