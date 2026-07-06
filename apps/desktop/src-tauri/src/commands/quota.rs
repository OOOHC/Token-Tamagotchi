use chrono::Utc;
use serde::Serialize;
use tauri::State;
use token_core::models::QuotaSnapshot;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSnapshotResponse {
    pub snapshot: QuotaSnapshot,
    pub save_outcome: local_store::SaveOutcome,
}

#[tauri::command]
pub fn get_quota_snapshot(state: State<'_, AppState>) -> Result<QuotaSnapshot, String> {
    let store = state
        .store
        .lock()
        .map_err(|_| "Local store lock was poisoned.".to_string())?;

    store
        .latest_quota_snapshot()
        .map_err(|error| error.to_string())
        .map(|snapshot| snapshot.unwrap_or_else(codex_adapter::mock_provider::mock_snapshot))
}

#[tauri::command]
pub fn parse_status_text(
    state: State<'_, AppState>,
    raw_text: String,
) -> Result<ImportSnapshotResponse, String> {
    let snapshot = codex_adapter::parser::parse_status_output(&raw_text, Utc::now().to_rfc3339());
    let store = state
        .store
        .lock()
        .map_err(|_| "Local store lock was poisoned.".to_string())?;

    let save_outcome = store
        .save_quota_snapshot(&snapshot)
        .map_err(|error| error.to_string())?;

    Ok(ImportSnapshotResponse {
        snapshot,
        save_outcome,
    })
}
