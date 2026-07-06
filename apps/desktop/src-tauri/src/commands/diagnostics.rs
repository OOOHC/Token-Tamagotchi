use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsResponse {
    pub app: String,
    pub data_source: CodexSourceStatus,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexSourceStatus {
    pub cli_detected: bool,
    pub version: Option<String>,
    pub status_command_available: bool,
    pub usage_command_available: bool,
    pub mode: DataSourceMode,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataSourceMode {
    ManualImport,
    AutoCapable,
    CliMissing,
}

#[tauri::command]
pub fn get_diagnostics() -> DiagnosticsResponse {
    DiagnosticsResponse {
        app: "Token Tamagotchi".to_string(),
        data_source: detect_codex_source(),
    }
}

fn detect_codex_source() -> CodexSourceStatus {
    let version = Command::new("codex")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .filter(|value| !value.is_empty());

    let help_output = match Command::new("codex").arg("--help").output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        _ => {
            return CodexSourceStatus {
                cli_detected: false,
                version,
                status_command_available: false,
                usage_command_available: false,
                mode: DataSourceMode::CliMissing,
                message: "Codex CLI was not detected. Manual import remains available.".to_string(),
            };
        }
    };

    let status_command_available = command_is_listed(&help_output, "status");
    let usage_command_available = command_is_listed(&help_output, "usage");
    let mode = if status_command_available || usage_command_available {
        DataSourceMode::AutoCapable
    } else {
        DataSourceMode::ManualImport
    };
    let message = match mode {
        DataSourceMode::AutoCapable => {
            "Codex CLI exposes quota commands. Auto refresh can be enabled.".to_string()
        }
        DataSourceMode::ManualImport => {
            "Codex CLI is installed, but no public quota command was detected. Use manual import for now.".to_string()
        }
        DataSourceMode::CliMissing => unreachable!("handled before help parsing"),
    };

    CodexSourceStatus {
        cli_detected: true,
        version,
        status_command_available,
        usage_command_available,
        mode,
        message,
    }
}

fn command_is_listed(help_output: &str, command: &str) -> bool {
    help_output
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("{command} ")))
}
