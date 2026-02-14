use std::path::{Path, PathBuf};

use magic_wormhole::transit::{self, Abilities};
use magic_wormhole::transfer::{self, ReceiveRequest};
use magic_wormhole::{Code, MailboxConnection, Wormhole};
use tracing::info;

use crate::config::EstudelyConfig;
use crate::error::ReceiveError;
use crate::progress::{ProgressCallback, TransferProgress};

/// Information about a pending incoming transfer.
pub struct PendingReceive {
    request: ReceiveRequest,
}

impl PendingReceive {
    /// The offered filename (untrusted input from sender).
    pub fn file_name(&self) -> String {
        self.request.file_name()
    }

    /// The offered file size in bytes.
    pub fn file_size(&self) -> u64 {
        self.request.file_size()
    }

    /// Accept the transfer and save to the given directory.
    /// Returns the full path of the saved file.
    pub async fn accept(
        self,
        save_dir: &Path,
        mut progress: ProgressCallback,
        cancel: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<PathBuf, ReceiveError> {
        let file_name = sanitize_filename(&self.file_name());
        let save_path = save_dir.join(&file_name);

        info!(path = %save_path.display(), "accepting transfer");

        // magic-wormhole uses futures_io::AsyncWrite, so we use async_std's File
        let mut file = async_std::fs::File::create(&save_path).await?;

        let cancel_fut = async move {
            let _ = cancel.await;
        };

        self.request
            .accept(
                |_transit_info| {
                    info!("transit connection established");
                },
                move |received, total| {
                    progress(TransferProgress {
                        bytes_transferred: received,
                        bytes_total: total,
                    });
                },
                &mut file,
                cancel_fut,
            )
            .await?;

        Ok(save_path)
    }

    /// Reject the incoming transfer.
    pub async fn reject(self) -> Result<(), ReceiveError> {
        self.request.reject().await?;
        Ok(())
    }
}

/// Connect to a sender using a wormhole code and get the transfer offer.
pub async fn receive_request(
    code: &str,
    config: &EstudelyConfig,
    cancel: tokio::sync::oneshot::Receiver<()>,
) -> Result<PendingReceive, ReceiveError> {
    let app_config = transfer::APP_CONFIG;
    let code = Code(code.to_string());

    let mailbox = MailboxConnection::connect(app_config, code, false).await?;
    let wormhole = Wormhole::connect(mailbox).await?;

    let relay_hints = default_relay_hints(&config.transit_abilities);
    let abilities = config.transit_abilities;

    let cancel_fut = async move {
        let _ = cancel.await;
    };

    let request = transfer::request_file(wormhole, relay_hints, abilities, cancel_fut)
        .await?
        .ok_or(ReceiveError::NoOffer)?;

    Ok(PendingReceive { request })
}

/// Build the default relay hints.
pub(crate) fn default_relay_hints(_abilities: &Abilities) -> Vec<transit::RelayHint> {
    vec![
        transit::RelayHint::from_urls(
            None,
            [transit::DEFAULT_RELAY_SERVER.parse().unwrap()],
        )
        .unwrap(),
    ]
}

/// Sanitize a filename from untrusted input.
fn sanitize_filename(name: &str) -> String {
    let name = Path::new(name)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "received_file".to_string());

    if name.is_empty() {
        "received_file".to_string()
    } else {
        name
    }
}
