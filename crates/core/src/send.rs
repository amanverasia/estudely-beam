use std::path::Path;

use magic_wormhole::transfer;
use magic_wormhole::{MailboxConnection, Wormhole};
use tracing::info;

use crate::config::EstudelyConfig;
use crate::error::SendError;
use crate::progress::{ProgressCallback, TransferProgress};

/// Send a file or directory. Returns the code, then drives the transfer
/// to completion.
///
/// The `on_code` callback is invoked with the code as soon as it's generated,
/// before the transfer starts. This lets callers display the code while
/// waiting for the receiver to connect.
pub async fn send_file(
    path: &Path,
    config: &EstudelyConfig,
    on_code: impl FnOnce(&str),
    mut progress: ProgressCallback,
    cancel: tokio::sync::oneshot::Receiver<()>,
) -> Result<(), SendError> {
    if !path.exists() {
        return Err(SendError::FileNotFound(path.display().to_string()));
    }

    let app_config = transfer::APP_CONFIG;
    let mailbox = MailboxConnection::create(app_config, config.code_length).await?;
    let code = mailbox.code().to_string();

    info!(code = %code, "wormhole code generated");
    on_code(&code);

    let abilities = config.transit_abilities;
    let relay_hints = crate::receive::default_relay_hints(&abilities);
    let wormhole = Wormhole::connect(mailbox).await?;

    let cancel_fut = async move {
        let _ = cancel.await;
    };

    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    transfer::send_file_or_folder(
        wormhole,
        relay_hints,
        path,
        file_name,
        abilities,
        |_transit_info| {
            info!("transit connection established");
        },
        move |sent, total| {
            progress(TransferProgress {
                bytes_transferred: sent,
                bytes_total: total,
            });
        },
        cancel_fut,
    )
    .await?;

    Ok(())
}

/// Send a text message. Creates a temporary file and sends it.
/// Note: This is not interoperable with the Python wormhole CLI's `--text`
/// mode, which uses a different protocol layer. The receiver will get a
/// file named "message.txt".
pub async fn send_text(
    text: &str,
    config: &EstudelyConfig,
    on_code: impl FnOnce(&str),
    cancel: tokio::sync::oneshot::Receiver<()>,
) -> Result<(), SendError> {
    let tmp_dir = tempfile::tempdir()?;
    let tmp_path = tmp_dir.path().join("message.txt");
    tokio::fs::write(&tmp_path, text).await?;

    send_file(
        &tmp_path,
        config,
        on_code,
        Box::new(|_| {}),
        cancel,
    )
    .await
}
