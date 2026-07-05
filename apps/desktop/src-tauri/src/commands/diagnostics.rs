use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticsResponse {
    pub app: String,
    pub data_source: String,
}

#[tauri::command]
pub fn get_diagnostics() -> DiagnosticsResponse {
    DiagnosticsResponse {
        app: "Token Tamagotchi".to_string(),
        data_source: "mock".to_string(),
    }
}

