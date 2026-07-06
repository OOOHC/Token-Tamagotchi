use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(12);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexRateLimitSnapshot {
    pub available: bool,
    pub limit_id: Option<String>,
    pub plan_type: Option<String>,
    pub five_hour_used_percent: Option<u64>,
    pub five_hour_remaining_percent: Option<u64>,
    pub five_hour_reset_at: Option<String>,
    pub weekly_used_percent: Option<u64>,
    pub weekly_remaining_percent: Option<u64>,
    pub weekly_reset_at: Option<String>,
    pub rate_limit_reached_type: Option<String>,
    pub observed_at: String,
    pub warnings: Vec<String>,
}

#[tauri::command]
pub fn get_codex_rate_limits() -> CodexRateLimitSnapshot {
    let observed_at = Utc::now().to_rfc3339();

    match read_codex_rate_limits(observed_at.clone()) {
        Ok(snapshot) => snapshot,
        Err(error) => unavailable(observed_at, &error),
    }
}

fn read_codex_rate_limits(observed_at: String) -> Result<CodexRateLimitSnapshot, String> {
    let mut child = Command::new("codex")
        .args(["app-server", "--stdio"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Codex app-server could not be started: {error}"))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Codex app-server stdout was not available.".to_string())?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Codex app-server stdin was not available.".to_string())?;

    let (sender, receiver) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if sender.send(line).is_err() {
                break;
            }
        }
    });

    write_json_line(
        &mut stdin,
        json!({
            "id": 1,
            "method": "initialize",
            "params": {
                "clientInfo": {
                    "name": "token-tamagotchi",
                    "title": "Token Tamagotchi",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "capabilities": {
                    "experimentalApi": true,
                    "requestAttestation": false
                }
            }
        }),
    )?;

    read_response_for_id(&receiver, 1)?;

    write_json_line(
        &mut stdin,
        json!({
            "id": 2,
            "method": "account/rateLimits/read",
            "params": null
        }),
    )?;

    let response = read_response_for_id(&receiver, 2)?;
    drop(stdin);
    stop_child(&mut child);

    let result = response
        .get("result")
        .ok_or_else(|| "Codex rate limit response did not include result.".to_string())?;

    Ok(snapshot_from_rate_limit_result(result, observed_at))
}

fn write_json_line(stdin: &mut std::process::ChildStdin, value: Value) -> Result<(), String> {
    writeln!(stdin, "{value}")
        .map_err(|error| format!("Codex app-server write failed: {error}"))?;
    stdin
        .flush()
        .map_err(|error| format!("Codex app-server flush failed: {error}"))
}

fn read_response_for_id(receiver: &mpsc::Receiver<String>, id: i64) -> Result<Value, String> {
    loop {
        let line = receiver
            .recv_timeout(REQUEST_TIMEOUT)
            .map_err(|_| format!("Timed out waiting for Codex app-server response id {id}."))?;

        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };

        if value.get("id").and_then(Value::as_i64) == Some(id) {
            if let Some(error) = value.get("error") {
                return Err(format!("Codex app-server returned error: {error}"));
            }

            return Ok(value);
        }
    }
}

fn snapshot_from_rate_limit_result(result: &Value, observed_at: String) -> CodexRateLimitSnapshot {
    let snapshot = result
        .pointer("/rateLimitsByLimitId/codex")
        .or_else(|| result.get("rateLimits"));

    let mut warnings = Vec::new();

    let Some(snapshot) = snapshot else {
        return unavailable(
            observed_at,
            "Codex rate limit response did not include the codex bucket.",
        );
    };

    let five_hour_used_percent = snapshot
        .pointer("/primary/usedPercent")
        .and_then(Value::as_u64);
    let weekly_used_percent = snapshot
        .pointer("/secondary/usedPercent")
        .and_then(Value::as_u64);

    if five_hour_used_percent.is_none() {
        warnings.push("Codex 5-hour usedPercent was not available.".to_string());
    }

    if weekly_used_percent.is_none() {
        warnings.push("Codex weekly usedPercent was not available.".to_string());
    }

    CodexRateLimitSnapshot {
        available: five_hour_used_percent.is_some() || weekly_used_percent.is_some(),
        limit_id: snapshot
            .get("limitId")
            .and_then(Value::as_str)
            .map(str::to_string),
        plan_type: snapshot
            .get("planType")
            .and_then(Value::as_str)
            .map(str::to_string),
        five_hour_used_percent,
        five_hour_remaining_percent: remaining_percent(five_hour_used_percent),
        five_hour_reset_at: unix_seconds_to_rfc3339(
            snapshot
                .pointer("/primary/resetsAt")
                .and_then(Value::as_i64),
        ),
        weekly_used_percent,
        weekly_remaining_percent: remaining_percent(weekly_used_percent),
        weekly_reset_at: unix_seconds_to_rfc3339(
            snapshot
                .pointer("/secondary/resetsAt")
                .and_then(Value::as_i64),
        ),
        rate_limit_reached_type: snapshot
            .get("rateLimitReachedType")
            .and_then(Value::as_str)
            .map(str::to_string),
        observed_at,
        warnings,
    }
}

fn remaining_percent(used_percent: Option<u64>) -> Option<u64> {
    used_percent.map(|used| 100_u64.saturating_sub(used.min(100)))
}

fn unix_seconds_to_rfc3339(value: Option<i64>) -> Option<String> {
    let datetime: DateTime<Utc> = DateTime::from_timestamp(value?, 0)?;
    Some(datetime.to_rfc3339())
}

fn stop_child(child: &mut Child) {
    if matches!(child.try_wait(), Ok(None)) {
        let _ = child.kill();
    }

    let _ = child.wait();
}

fn unavailable(observed_at: String, warning: &str) -> CodexRateLimitSnapshot {
    CodexRateLimitSnapshot {
        available: false,
        limit_id: None,
        plan_type: None,
        five_hour_used_percent: None,
        five_hour_remaining_percent: None,
        five_hour_reset_at: None,
        weekly_used_percent: None,
        weekly_remaining_percent: None,
        weekly_reset_at: None,
        rate_limit_reached_type: None,
        observed_at,
        warnings: vec![warning.to_string()],
    }
}
