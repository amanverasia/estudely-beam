use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use tauri::{AppHandle, Emitter, State};

use estudely_core::progress::TransferProgress;
use estudely_core::transfer::TransferId;

use crate::events::{CodeEvent, CompleteEvent, ErrorEvent, ProgressEvent};
use crate::state::AppState;

#[tauri::command]
pub async fn send_file(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    path: String,
) -> Result<TransferId, String> {
    let file_path = PathBuf::from(&path);
    if !file_path.exists() {
        return Err(format!("File not found: {path}"));
    }

    let config = state.settings.lock().await.to_core_config();
    let (transfer_id, cancel_rx) = state.transfer_manager.register().await;

    let app_handle = app.clone();
    let id = transfer_id;

    let state_clone = Arc::clone(&state);

    tokio::spawn(async move {
        let on_code = {
            let app = app_handle.clone();
            move |code: &str| {
                let _ = app.emit("transfer:code", CodeEvent {
                    transfer_id: id,
                    code: code.to_string(),
                });
            }
        };

        let app_progress = app_handle.clone();
        let mut last_emit = Instant::now();
        let progress: Box<dyn FnMut(TransferProgress) + Send> =
            Box::new(move |p: TransferProgress| {
                let now = Instant::now();
                let is_complete = p.bytes_transferred >= p.bytes_total;
                if is_complete || now.duration_since(last_emit).as_millis() >= 100 {
                    last_emit = now;
                    let _ = app_progress.emit(
                        "transfer:progress",
                        ProgressEvent {
                            transfer_id: id,
                            bytes_transferred: p.bytes_transferred,
                            bytes_total: p.bytes_total,
                        },
                    );
                }
            });

        match estudely_core::send_file(&file_path, &config, on_code, progress, cancel_rx).await {
            Ok(()) => {
                let _ = app_handle.emit(
                    "transfer:complete",
                    CompleteEvent {
                        transfer_id: id,
                        saved_path: None,
                    },
                );
            }
            Err(e) => {
                let _ = app_handle.emit(
                    "transfer:error",
                    ErrorEvent {
                        transfer_id: id,
                        error: e.to_string(),
                    },
                );
            }
        }

        state_clone.transfer_manager.complete(id).await;
    });

    Ok(transfer_id)
}

#[tauri::command]
pub async fn send_text(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    text: String,
) -> Result<TransferId, String> {
    let config = state.settings.lock().await.to_core_config();
    let (transfer_id, cancel_rx) = state.transfer_manager.register().await;

    let app_handle = app.clone();
    let id = transfer_id;

    let state_clone = Arc::clone(&state);

    tokio::spawn(async move {
        let on_code = {
            let app = app_handle.clone();
            move |code: &str| {
                let _ = app.emit("transfer:code", CodeEvent {
                    transfer_id: id,
                    code: code.to_string(),
                });
            }
        };

        match estudely_core::send_text(&text, &config, on_code, cancel_rx).await {
            Ok(()) => {
                let _ = app_handle.emit(
                    "transfer:complete",
                    CompleteEvent {
                        transfer_id: id,
                        saved_path: None,
                    },
                );
            }
            Err(e) => {
                let _ = app_handle.emit(
                    "transfer:error",
                    ErrorEvent {
                        transfer_id: id,
                        error: e.to_string(),
                    },
                );
            }
        }

        state_clone.transfer_manager.complete(id).await;
    });

    Ok(transfer_id)
}
