use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SettingsResponse {
    pub refresh_interval_seconds: u64,
    pub provider: String,
}

#[tauri::command]
pub fn get_settings() -> SettingsResponse {
    SettingsResponse {
        refresh_interval_seconds: 30,
        provider: "mock".to_string(),
    }
}
