mod commands;
mod events;
mod settings;
mod state;

use std::sync::Arc;

use settings::AppSettings;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let settings = AppSettings::load(&app_data_dir);
            let state = Arc::new(AppState::new(settings));
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::send::send_file,
            commands::send::send_text,
            commands::receive::receive_connect,
            commands::receive::receive_accept,
            commands::receive::receive_reject,
            commands::transfer::cancel_transfer,
            commands::settings::get_settings,
            commands::settings::set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
