use std::sync::Arc;

use tauri::State;

use estudely_core::transfer::TransferId;

use crate::state::AppState;

#[tauri::command]
pub async fn cancel_transfer(
    state: State<'_, Arc<AppState>>,
    transfer_id: TransferId,
) -> Result<bool, String> {
    // Also remove any pending receive for this transfer
    state.pending_receives.lock().await.remove(&transfer_id);
    Ok(state.transfer_manager.cancel(transfer_id).await)
}
