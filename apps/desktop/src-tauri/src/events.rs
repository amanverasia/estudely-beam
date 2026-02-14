use serde::{Deserialize, Serialize};

use estudely_core::transfer::TransferId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEvent {
    pub transfer_id: TransferId,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub transfer_id: TransferId,
    pub bytes_transferred: u64,
    pub bytes_total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteEvent {
    pub transfer_id: TransferId,
    pub saved_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub transfer_id: TransferId,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveOfferEvent {
    pub transfer_id: TransferId,
    pub file_name: String,
    pub file_size: u64,
}
