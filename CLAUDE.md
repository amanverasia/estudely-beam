# estudely-send

## Build & Test Commands
- `cargo build --workspace` - Build all crates
- `cargo clippy --workspace` - Lint all crates
- `cargo test --workspace` - Run all tests
- `cargo run -p estudely-cli -- send <path>` - Send a file
- `cargo run -p estudely-cli -- text <message>` - Send text
- `cargo run -p estudely-cli -- receive <code>` - Receive a transfer

## Architecture
- `crates/core` (estudely-core) - UI-agnostic wormhole wrapper. Progress via `Box<dyn FnMut(TransferProgress) + Send>`.
- `crates/cli` (estudely-cli) - clap CLI with indicatif progress bars.
- `apps/desktop` - Tauri 2.0 desktop app (Phase 2).

## Key Patterns
- Two-step receive: `receive_request(code)` returns `PendingReceive`, then `.accept(path)` or `.reject()`.
- Cancellation via `tokio::sync::oneshot`. `TransferManager` tracks active transfers.
- AppID: `lothar.com/wormhole/text-or-file-xfer` for interoperability with official wormhole clients.
- Core errors use `thiserror`. CLI uses `anyhow` for ergonomic error handling.

## Style
- Keep core library free of UI dependencies.
- Tauri commands are thin wrappers around core functions.
- Prefer `tracing` over `println!` for logging.
