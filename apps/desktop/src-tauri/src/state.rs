use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use estudely_core::receive::PendingReceive;
use estudely_core::transfer::{TransferId, TransferManager};

use crate::settings::AppSettings;

pub struct AppState {
    pub transfer_manager: TransferManager,
    pub pending_receives: Arc<Mutex<HashMap<TransferId, PendingReceive>>>,
    pub settings: Arc<Mutex<AppSettings>>,
}

impl AppState {
    pub fn new(settings: AppSettings) -> Self {
        Self {
            transfer_manager: TransferManager::new(),
            pending_receives: Arc::new(Mutex::new(HashMap::new())),
            settings: Arc::new(Mutex::new(settings)),
        }
    }
}
