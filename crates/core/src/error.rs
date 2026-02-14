use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("failed to read file metadata: {0}")]
    Metadata(#[from] std::io::Error),
    #[error("wormhole error: {0}")]
    Wormhole(#[from] magic_wormhole::WormholeError),
    #[error("transfer error: {0}")]
    Transfer(#[from] magic_wormhole::transfer::TransferError),
    #[error("transfer cancelled")]
    Cancelled,
}

#[derive(Debug, Error)]
pub enum ReceiveError {
    #[error("wormhole error: {0}")]
    Wormhole(#[from] magic_wormhole::WormholeError),
    #[error("transfer error: {0}")]
    Transfer(#[from] magic_wormhole::transfer::TransferError),
    #[error("transfer cancelled")]
    Cancelled,
    #[error("transfer rejected by receiver")]
    Rejected,
    #[error("no file offer received")]
    NoOffer,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
