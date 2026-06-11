# Developing GOAT

Welcome to GOAT development! This guide provides an overview of the architecture and how to get started contributing.

## Architecture Overview

GOAT is built with a decoupled architecture:
1. **Core Backend (Rust):** Handles the LLM interactions, memory indexing, security (ApprovalGate), and tool execution.
2. **Daemon API (Rust):** Exposes the Core Backend functionalities over a local REST API.
3. **Frontend Dashboard (Next.js):** A comprehensive interface for interacting with the daemon.
4. **Desktop App (Tauri):** Wraps the Next.js frontend and Rust backend into a single executable.

## Setting Up Your Development Environment

Ensure you have Rust, Node.js, and your preferred IDE set up.

1. Clone the repository and build the workspace.
2. Run tests to ensure everything is working: `cargo test`

## Useful Commands

* `cargo fmt` - Formats the Rust codebase.
* `cargo check` - Fast compilation check without generating binaries.
* `npm run dev` (in `apps/dashboard`) - Run the web interface in watch mode.

## Security Rule: The ApprovalGate

When contributing to GOAT, especially when adding new tools or external integrations, you **MUST NOT** bypass the `ApprovalGate`.

Any action that can mutate the filesystem, run commands on the host, or transmit sensitive data over the network must prompt the user for explicit approval unless it falls under extremely narrow exceptions.

## Committing Code

We follow conventional commits for our commit messages. Ensure your code passes `cargo check`, `cargo fmt`, and `cargo test` before opening a pull request.
