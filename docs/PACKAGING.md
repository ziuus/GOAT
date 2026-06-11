# Packaging Plan

This document outlines the planned packaging and distribution strategies for GOAT.

## Core CLI & Daemon (Rust Binary)

The `goat` executable encompasses the TUI, headless CLI, and local API daemon.
- **GitHub Releases:** Static binaries compiled for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc`.
- **Crates.io:** Publishing via `cargo install goat-agent` is planned for the future, pending CLI stabilization and API stabilization.
- **Homebrew:** A tap (`brew install ziuus/goat/goat`) is planned.
- **AUR:** Arch User Repository packaging is planned once stable.

## Desktop App (Tauri)

The desktop application bundles the frontend and Rust core.
- **Linux:** AppImage and `.deb` distribution.
- **macOS:** Universal `.dmg` with signed binaries.
- **Windows:** `.msi` installers.

## Scripts & Automation

We intend to add non-destructive automation scripts (e.g., `scripts/build-all.sh`) in upcoming phases to streamline this process across CI/CD pipelines.

*(Currently, all packaging is handled manually via `cargo build --release` during the Alpha phase).*
