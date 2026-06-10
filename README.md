# GOAT: General Omniscient Agentic Tool

GOAT is a powerful, local-first agentic coding assistant designed to interface with your codebase securely. It provides a robust backend daemon and a beautifully polished Next.js Web Dashboard.

## Core Features
- **Local-First Security:** The Daemon API runs strictly on `127.0.0.1`.
- **ApprovalGate:** A strict risk-evaluation security model that intercepts destructive actions.
- **Multiple Interfaces:** Operate GOAT via Terminal UI (TUI), Headless CLI, or the modern Web Dashboard.
- **Async Execution:** Heavy operations run as background `tokio::spawn` tasks with SSE live updates.
- **Premium Dashboard:** Features custom theming, command palettes, and safe code/diff viewers.

## Current Status (Phase 4.7)
GOAT is currently completing its Phase 4.x milestone. 
- The **TUI** and **Headless** modes are fully functional but considered *Beta* pending further layout polish.
- The **Web Dashboard** is highly polished and considered production-ready for encapsulation into a desktop shell.
- **Tauri Desktop Wrap**, **Voice Mode**, and **Multi-Agent Swarms** are *Planned* for future phases.

## Quickstart

1. Configure API keys in `.env`:
   ```env
   OPENAI_API_KEY=sk-...
   ```
2. Run the background Daemon:
   ```bash
   cargo run -- daemon
   ```
3. Boot the Dashboard:
   ```bash
   cd apps/dashboard && npm install && npm run dev
   ```

For detailed configuration, see [docs/GOAT_SETUP_AND_TROUBLESHOOTING.md](docs/GOAT_SETUP_AND_TROUBLESHOOTING.md).
