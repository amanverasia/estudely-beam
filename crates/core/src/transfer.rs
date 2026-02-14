use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};

/// Unique identifier for a transfer.
pub type TransferId = u64;

/// Tracks active transfers and provides cancellation.
pub struct TransferManager {
    inner: Arc<Mutex<TransferManagerInner>>,
}

struct TransferManagerInner {
    next_id: TransferId,
    active: HashMap<TransferId, oneshot::Sender<()>>,
}

impl TransferManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(TransferManagerInner {
                next_id: 1,
                active: HashMap::new(),
            })),
        }
    }

    /// Register a new transfer. Returns the ID and a cancel receiver
    /// that the transfer task should select on.
    pub async fn register(&self) -> (TransferId, oneshot::Receiver<()>) {
        let mut inner = self.inner.lock().await;
        let id = inner.next_id;
        inner.next_id += 1;
        let (tx, rx) = oneshot::channel();
        inner.active.insert(id, tx);
        (id, rx)
    }

    /// Cancel a transfer by ID. Returns true if the transfer was found.
    pub async fn cancel(&self, id: TransferId) -> bool {
        let mut inner = self.inner.lock().await;
        if let Some(tx) = inner.active.remove(&id) {
            let _ = tx.send(());
            true
        } else {
            false
        }
    }

    /// Remove a completed transfer from tracking.
    pub async fn complete(&self, id: TransferId) {
        let mut inner = self.inner.lock().await;
        inner.active.remove(&id);
    }

    /// Get the number of active transfers.
    pub async fn active_count(&self) -> usize {
        let inner = self.inner.lock().await;
        inner.active.len()
    }
}

impl Default for TransferManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn register_returns_incrementing_ids() {
        let mgr = TransferManager::new();
        let (id1, _) = mgr.register().await;
        let (id2, _) = mgr.register().await;
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[tokio::test]
    async fn cancel_active_transfer() {
        let mgr = TransferManager::new();
        let (id, rx) = mgr.register().await;
        assert_eq!(mgr.active_count().await, 1);

        assert!(mgr.cancel(id).await);
        assert!(rx.await.is_ok());
        assert_eq!(mgr.active_count().await, 0);
    }

    #[tokio::test]
    async fn cancel_nonexistent_returns_false() {
        let mgr = TransferManager::new();
        assert!(!mgr.cancel(999).await);
    }

    #[tokio::test]
    async fn complete_removes_transfer() {
        let mgr = TransferManager::new();
        let (id, _rx) = mgr.register().await;
        assert_eq!(mgr.active_count().await, 1);

        mgr.complete(id).await;
        assert_eq!(mgr.active_count().await, 0);
    }
}
