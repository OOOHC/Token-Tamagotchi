use crate::models::QuotaSnapshot;

pub fn five_hour_ratio(snapshot: &QuotaSnapshot) -> Option<f64> {
    let remaining = snapshot.five_hour_remaining?;
    let limit = snapshot.five_hour_limit?;

    if limit == 0 {
        return None;
    }

    Some(remaining as f64 / limit as f64)
}

