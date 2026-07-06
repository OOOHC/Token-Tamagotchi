use chrono::{DateTime, Utc};
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalCodexUsageSnapshot {
    pub available: bool,
    pub five_hour_used: u64,
    pub weekly_used: u64,
    pub total_used: Option<u64>,
    pub latest_usage_total_tokens: Option<u64>,
    pub primary_reset_at: Option<String>,
    pub secondary_reset_at: Option<String>,
    pub limit_reached: Option<bool>,
    pub observed_at: String,
    pub warnings: Vec<String>,
}

#[tauri::command]
pub fn get_local_codex_usage() -> LocalCodexUsageSnapshot {
    let observed_at = Utc::now().to_rfc3339();
    let mut warnings = Vec::new();
    let Some(codex_home) = codex_home() else {
        return unavailable(
            observed_at,
            "CODEX_HOME or user home could not be resolved.",
        );
    };

    let logs_path = codex_home.join("logs_2.sqlite");
    let state_path = codex_home.join("state_5.sqlite");

    let usage = match read_usage_from_logs(&logs_path) {
        Ok(usage) => usage,
        Err(error) => {
            return unavailable(
                observed_at,
                &format!("Codex local logs could not be read: {error}"),
            );
        }
    };

    let total_used = match read_total_tokens_from_state(&state_path) {
        Ok(value) => Some(value),
        Err(error) => {
            warnings.push(format!("Codex state total could not be read: {error}"));
            None
        }
    };

    LocalCodexUsageSnapshot {
        available: true,
        five_hour_used: usage.five_hour_used,
        weekly_used: usage.weekly_used,
        total_used,
        latest_usage_total_tokens: usage.latest_usage_total_tokens,
        primary_reset_at: usage.primary_reset_at,
        secondary_reset_at: usage.secondary_reset_at,
        limit_reached: usage.limit_reached,
        observed_at,
        warnings,
    }
}

#[derive(Debug, Clone)]
struct LogUsage {
    five_hour_used: u64,
    weekly_used: u64,
    latest_usage_total_tokens: Option<u64>,
    primary_reset_at: Option<String>,
    secondary_reset_at: Option<String>,
    limit_reached: Option<bool>,
}

fn read_usage_from_logs(path: &PathBuf) -> Result<LogUsage, String> {
    let connection = open_read_only(path)?;
    let now = Utc::now().timestamp();
    let five_hours_ago = now - (5 * 60 * 60);
    let seven_days_ago = now - (7 * 24 * 60 * 60);
    let mut five_hour_used = 0_u64;
    let mut weekly_used = 0_u64;
    let mut latest_usage_total_tokens = None;
    let mut primary_reset_at = None;
    let mut secondary_reset_at = None;
    let mut limit_reached = None;

    let mut statement = connection
        .prepare(
            "select ts, feedback_log_body
             from logs
             where ts >= ?1
               and feedback_log_body like 'Received message {%'
             order by ts desc
             limit 10000",
        )
        .map_err(|error| error.to_string())?;

    let rows = statement
        .query_map([seven_days_ago], |row| {
            let ts: i64 = row.get(0)?;
            let body: String = row.get(1)?;
            Ok((ts, body))
        })
        .map_err(|error| error.to_string())?;

    for row in rows {
        let (ts, body) = row.map_err(|error| error.to_string())?;
        let Some(payload) = body.strip_prefix("Received message ") else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(payload) else {
            continue;
        };

        if let Some(total_tokens) = value
            .pointer("/response/usage/total_tokens")
            .and_then(Value::as_u64)
        {
            weekly_used = weekly_used.saturating_add(total_tokens);
            if ts >= five_hours_ago {
                five_hour_used = five_hour_used.saturating_add(total_tokens);
            }
            if latest_usage_total_tokens.is_none() {
                latest_usage_total_tokens = Some(total_tokens);
            }
        }

        if primary_reset_at.is_none() {
            primary_reset_at = unix_seconds_to_rfc3339(
                value
                    .pointer("/rate_limits/primary/reset_at")
                    .and_then(Value::as_i64),
            );
        }

        if secondary_reset_at.is_none() {
            secondary_reset_at = unix_seconds_to_rfc3339(
                value
                    .pointer("/rate_limits/secondary/reset_at")
                    .and_then(Value::as_i64),
            );
        }

        if limit_reached.is_none() {
            limit_reached = value
                .pointer("/rate_limits/limit_reached")
                .and_then(Value::as_bool);
        }
    }

    Ok(LogUsage {
        five_hour_used,
        weekly_used,
        latest_usage_total_tokens,
        primary_reset_at,
        secondary_reset_at,
        limit_reached,
    })
}

fn read_total_tokens_from_state(path: &PathBuf) -> Result<u64, String> {
    let connection = open_read_only(path)?;
    let value: i64 = connection
        .query_row(
            "select coalesce(sum(tokens_used), 0) from threads",
            [],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;

    Ok(value.max(0) as u64)
}

fn open_read_only(path: &PathBuf) -> Result<Connection, String> {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| format!("{} ({})", path.display(), error))
}

fn codex_home() -> Option<PathBuf> {
    if let Some(value) = std::env::var_os("CODEX_HOME") {
        return Some(PathBuf::from(value));
    }

    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(|home| PathBuf::from(home).join(".codex"))
}

fn unix_seconds_to_rfc3339(value: Option<i64>) -> Option<String> {
    let value = value?;
    let datetime: DateTime<Utc> = DateTime::from_timestamp(value, 0)?;
    Some(datetime.to_rfc3339())
}

fn unavailable(observed_at: String, warning: &str) -> LocalCodexUsageSnapshot {
    LocalCodexUsageSnapshot {
        available: false,
        five_hour_used: 0,
        weekly_used: 0,
        total_used: None,
        latest_usage_total_tokens: None,
        primary_reset_at: None,
        secondary_reset_at: None,
        limit_reached: None,
        observed_at,
        warnings: vec![warning.to_string()],
    }
}
