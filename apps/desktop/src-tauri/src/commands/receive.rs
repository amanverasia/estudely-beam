use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use tauri::{AppHandle, Emitter, State};

use estudely_core::progress::TransferProgress;
use estudely_core::transfer::TransferId;

use crate::events::{CompleteEvent, ErrorEvent, ProgressEvent, ReceiveOfferEvent};
use crate::state::AppState;

#[tauri::command]
pub async fn receive_connect(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    code: String,
) -> Result<ReceiveOfferEvent, String> {
    let config = state.settings.lock().await.to_core_config();
    let (transfer_id, cancel_rx) = state.transfer_manager.register().await;

    let pending = estudely_core::receive_request(&code, &config, cancel_rx)
        .await
        .map_err(|e| e.to_string())?;

    let offer = ReceiveOfferEvent {
        transfer_id,
        file_name: pending.file_name(),
        file_size: pending.file_size(),
    };

    state
        .pending_receives
        .lock()
        .await
        .insert(transfer_id, pending);

    let _ = app.emit("receive:offer", offer.clone());

    Ok(offer)
}

#[tauri::command]
pub async fn receive_accept(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    transfer_id: TransferId,
    save_dir: Option<String>,
) -> Result<(), String> {
    let pending = state
        .pending_receives
        .lock()
        .await
        .remove(&transfer_id)
        .ok_or_else(|| format!("No pending receive for transfer {transfer_id}"))?;

    let download_dir = match save_dir {
        Some(dir) => dir,
        None => state.settings.lock().await.download_dir.clone(),
    };
    let save_path = PathBuf::from(&download_dir);

    // We need a new cancel channel for the accept phase
    // Remove the old one from manager and register new
    state.transfer_manager.complete(transfer_id).await;
    let (new_id, cancel_rx) = state.transfer_manager.register().await;
    // Note: new_id may differ from transfer_id. We use transfer_id for frontend continuity.
    let _ = new_id; // We track with transfer_id on frontend

    let app_handle = app.clone();
    let id = transfer_id;
    let state_clone = Arc::clone(&state);

    tokio::spawn(async move {
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

        match pending.accept(&save_path, progress, cancel_rx).await {
            Ok(saved) => {
                let _ = app_handle.emit(
                    "transfer:complete",
                    CompleteEvent {
                        transfer_id: id,
                        saved_path: Some(saved.to_string_lossy().to_string()),
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

        state_clone.transfer_manager.complete(new_id).await;
    });

    Ok(())
}

#[tauri::command]
pub async fn receive_reject(
    state: State<'_, Arc<AppState>>,
    transfer_id: TransferId,
) -> Result<(), String> {
    let pending = state
        .pending_receives
        .lock()
        .await
        .remove(&transfer_id)
        .ok_or_else(|| format!("No pending receive for transfer {transfer_id}"))?;

    pending.reject().await.map_err(|e| e.to_string())?;
    state.transfer_manager.complete(transfer_id).await;

    Ok(())
}
