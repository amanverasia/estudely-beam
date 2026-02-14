use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    /// Bytes transferred so far.
    pub bytes_transferred: u64,
    /// Total bytes to transfer.
    pub bytes_total: u64,
}

impl TransferProgress {
    pub fn fraction(&self) -> f64 {
        if self.bytes_total == 0 {
            return 0.0;
        }
        self.bytes_transferred as f64 / self.bytes_total as f64
    }
}

/// Callback type for reporting transfer progress.
pub type ProgressCallback = Box<dyn FnMut(TransferProgress) + Send>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraction_normal() {
        let p = TransferProgress {
            bytes_transferred: 50,
            bytes_total: 100,
        };
        assert!((p.fraction() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn fraction_zero_total() {
        let p = TransferProgress {
            bytes_transferred: 0,
            bytes_total: 0,
        };
        assert!((p.fraction() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn fraction_complete() {
        let p = TransferProgress {
            bytes_transferred: 100,
            bytes_total: 100,
        };
        assert!((p.fraction() - 1.0).abs() < f64::EPSILON);
    }
}
