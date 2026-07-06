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
            commands::codex_app_server::get_codex_rate_limits,
            commands::local_codex::get_local_codex_usage,
            commands::settings::get_settings,
            commands::pet::get_pet_state,
            commands::diagnostics::get_diagnostics,
            commands::window::start_window_drag,
            commands::window::set_window_layout,
            commands::window::constrain_window_to_screen,
            commands::window::minimize_window,
            commands::window::close_window
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Token Tamagotchi");
}
