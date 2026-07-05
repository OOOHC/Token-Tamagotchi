use token_core::models::QuotaSnapshot;

#[tauri::command]
pub fn get_quota_snapshot() -> QuotaSnapshot {
    codex_adapter::mock_provider::mock_snapshot()
}

