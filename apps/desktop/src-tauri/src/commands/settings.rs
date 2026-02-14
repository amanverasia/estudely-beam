use std::sync::Arc;

use tauri::{Manager, State};

use crate::settings::AppSettings;
use crate::state::AppState;

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().await.clone())
}

#[tauri::command]
pub async fn set_settings(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    settings: AppSettings,
) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    settings.save(&app_data_dir)?;
    *state.settings.lock().await = settings;
    Ok(())
}
