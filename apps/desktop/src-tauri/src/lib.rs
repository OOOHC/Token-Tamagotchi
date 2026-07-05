mod commands;
mod state;

pub fn run() {
    tauri::Builder::default()
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::quota::get_quota_snapshot,
            commands::settings::get_settings,
            commands::pet::get_pet_state,
            commands::diagnostics::get_diagnostics
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Token Tamagotchi");
}

