mod commands;
mod state;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data directory");
            app.manage(state::AppState::open(
                app_data_dir.join("token-tamagotchi.sqlite3"),
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::quota::get_quota_snapshot,
            commands::quota::parse_status_text,
            commands::settings::get_settings,
            commands::pet::get_pet_state,
            commands::diagnostics::get_diagnostics
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Token Tamagotchi");
}
