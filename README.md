# estudely-send

Cross-platform file transfer app built on [magic-wormhole](https://github.com/magic-wormhole/magic-wormhole) for secure, peer-to-peer transfers using human-readable codes and end-to-end encryption.

## Features

- Send files, directories, and text via wormhole codes
- End-to-end encrypted transfers (SPAKE2)
- Interoperable with the official `wormhole` CLI and other compatible clients
- CLI and desktop GUI (Tauri 2.0)

## Installation

### From source

```bash
cargo install --path crates/cli
```

### CLI Usage

```bash
# Send a file
estudely-send send ./document.pdf

# Send text
estudely-send text "Hello, world!"

# Receive
estudely-send receive 7-crossover-clockwork
```

## Architecture

- **crates/core** - UI-agnostic core library wrapping magic-wormhole.rs
- **crates/cli** - Command-line interface with progress bars
- **apps/desktop** - Tauri 2.0 desktop app (React + TypeScript)

## License

GPL-3.0 - See [LICENSE](LICENSE) for details.

The `magic-wormhole.rs` crate is EUPL-1.2, which is compatible with GPL-3.0.
