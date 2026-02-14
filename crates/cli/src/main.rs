use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use tracing_subscriber::EnvFilter;

use estudely_core::{
    EstudelyConfig, TransferManager, TransferProgress,
};

#[derive(Parser)]
#[command(name = "estudely-beam", about = "Secure file transfer via wormhole codes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file or directory
    Send {
        /// Path to the file or directory to send
        path: PathBuf,
    },
    /// Send a text message
    Text {
        /// The message to send
        message: String,
    },
    /// Receive a file using a wormhole code
    Receive {
        /// The wormhole code (e.g., "7-crossover-clockwork")
        code: String,
        /// Directory to save received files (defaults to current directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let config = EstudelyConfig::default();
    let manager = TransferManager::new();

    match cli.command {
        Commands::Send { path } => {
            let path = path
                .canonicalize()
                .with_context(|| format!("path not found: {}", path.display()))?;

            let (id, cancel_rx) = manager.register().await;

            let pb = ProgressBar::new(0);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );

            let progress: Box<dyn FnMut(TransferProgress) + Send> =
                Box::new(move |p: TransferProgress| {
                    pb.set_length(p.bytes_total);
                    pb.set_position(p.bytes_transferred);
                    if p.bytes_transferred >= p.bytes_total {
                        pb.finish_with_message("done");
                    }
                });

            println!("Sending: {}", path.display());

            estudely_core::send_file(
                &path,
                &config,
                |code| {
                    println!("\nWormhole code: {code}");
                    println!("On the other computer, run:\n");
                    println!("  estudely-beam receive {code}\n");
                },
                progress,
                cancel_rx,
            )
            .await
            .with_context(|| "send failed")?;

            manager.complete(id).await;
            println!("Transfer complete!");
        }
        Commands::Text { message } => {
            let (_id, cancel_rx) = manager.register().await;

            println!("Sending text message...");

            estudely_core::send_text(
                &message,
                &config,
                |code| {
                    println!("\nWormhole code: {code}");
                    println!("On the other computer, run:\n");
                    println!("  estudely-beam receive {code}\n");
                },
                cancel_rx,
            )
            .await
            .with_context(|| "send failed")?;

            println!("Text sent!");
        }
        Commands::Receive { code, output } => {
            let output = output.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

            let (_connect_id, connect_cancel) = manager.register().await;

            println!("Connecting with code: {code}");

            let pending = estudely_core::receive_request(&code, &config, connect_cancel)
                .await
                .with_context(|| "failed to connect")?;

            println!(
                "\nIncoming file: {} ({})",
                pending.file_name(),
                HumanBytes(pending.file_size())
            );

            let accept = Confirm::new()
                .with_prompt("Accept this transfer?")
                .default(true)
                .interact()?;

            if !accept {
                pending.reject().await?;
                println!("Transfer rejected.");
                return Ok(());
            }

            let (id, cancel_rx) = manager.register().await;

            let pb = ProgressBar::new(pending.file_size());
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );

            let progress: Box<dyn FnMut(TransferProgress) + Send> =
                Box::new(move |p: TransferProgress| {
                    pb.set_length(p.bytes_total);
                    pb.set_position(p.bytes_transferred);
                    if p.bytes_transferred >= p.bytes_total {
                        pb.finish_with_message("done");
                    }
                });

            let saved = pending.accept(&output, progress, cancel_rx).await?;
            manager.complete(id).await;

            println!("Saved to: {}", saved.display());
        }
    }

    Ok(())
}
