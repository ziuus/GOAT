# GOAT Tauri Desktop Foundation

## Overview
Phase 5.0 introduces the foundation for GOAT as a native desktop application using [Tauri v2](https://v2.tauri.app/).

The desktop app serves as a secure, native wrapper around the existing Next.js dashboard, communicating directly with the local GOAT Daemon (`http://127.0.0.1:47647`).

## Architecture

* **Frontend:** Reuses the existing `apps/dashboard` (Next.js statically exported to `apps/dashboard/out`).
* **Backend:** Tauri (Rust) handles window management and system integration (`apps/desktop/src-tauri`).
* **Daemon Link:** The dashboard frontend connects to the daemon using the same token-based authentication mechanism.

## Security Model

The desktop application inherits and strictly enforces the GOAT security model:

1. **Local Only:** Tauri Content Security Policy (CSP) restricts connections to `127.0.0.1`.
2. **Token Auth Required:** The desktop app does not bypass daemon authentication. It must read the token from `~/.local/share/goat/daemon.token` (via user input or future secure IPC) just like the web dashboard.
3. **ApprovalGate Enforced:** The desktop app cannot auto-execute destructive commands. Any risky action requested via the desktop app still enters the ApprovalQueue for explicit user consent.
4. **No Telemetry/Cloud Sync:** The application is entirely localized.

## Directory Structure

```
apps/
├── dashboard/        # The existing Next.js React frontend
└── desktop/          # The new Tauri desktop shell
    ├── package.json  # Desktop specific dev scripts
    └── src-tauri/    # Tauri Rust configuration
        ├── tauri.conf.json
        ├── Cargo.toml
        └── src/
            ├── main.rs
            └── lib.rs (Tauri logic & commands)
```

## Tauri Commands (IPC)
The following safe native commands are exposed to the dashboard frontend:
- `get_app_version()` -> String
- `get_default_api_url()` -> String
- `get_daemon_status()` -> bool
- `get_daemon_token_path()` -> String

## Running the Desktop App Locally

### Prerequisites
You need the standard Tauri dependencies installed (Rust, Node, and platform build tools).

### Development Mode

1. Start the GOAT Daemon in one terminal:
   ```bash
   cargo run -- daemon start
   ```

2. Start the Tauri app in development mode:
   ```bash
   cd apps/desktop
   npm install
   npm run dev
   ```
   *Note: `tauri dev` automatically triggers `npm run dev` in the `apps/dashboard` directory via the `beforeDevCommand` hook.*

### Building for Release

1. Build the desktop binaries:
   ```bash
   cd apps/desktop
   npm install
   npm run build
   ```
   *Note: `tauri build` automatically runs `npm run build` inside `apps/dashboard` via the `beforeBuildCommand` hook, exporting the static assets to `apps/dashboard/out`.*

## CLI Integration
You can inspect the desktop environment using the GOAT CLI:
- `goat desktop` -> Prints launch instructions
- `goat desktop path` -> Prints the absolute path to the desktop code
- `goat desktop doctor` -> Checks desktop scaffolding readiness

## Current Status (Phase 5.0)
- **Implemented:** Tauri scaffold, config, CLI integration, secure CSP, basic IPC commands, and dashboard awareness ("Desktop Mode" badge).
- **Pending (Phase 5.1):** Auto-launching the daemon from Tauri if stopped, native OS notifications for Approval requests, deep OS file system integration via Tauri rather than proxying through the daemon.
