# GOAT Desktop Daemon Lifecycle

## Overview
The Tauri desktop app acts as a native shell around the Next.js dashboard, but GOAT's core operations still depend entirely on the local Rust daemon (`goat daemon start`). This document outlines how the desktop app manages and tracks the daemon lifecycle.

## Lifecycle Behavior
1. **Status Polling**: The Next.js dashboard, when running inside Tauri (`__TAURI_INTERNALS__`), calls the native Tauri IPC command `get_daemon_status`.
2. **Daemon Ping**: Tauri executes a 500ms timeout `reqwest` ping to `http://127.0.0.1:47647/v1/status`.
3. **Start Command**: If the daemon is stopped, the user can click "Start Daemon" in the dashboard settings. This triggers the native `start_daemon` Tauri command.
4. **Safe Process Spawn**: `start_daemon` invokes `std::process::Command` to run `goat daemon start` in the background. It does *not* use a shell, preventing injection. It captures the PID and reports success.
5. **Token Discovery**: Tauri resolves the daemon's secure token path natively via `get_daemon_token_path` and feeds this back to the dashboard, ensuring the user can easily locate their local authentication token.

## Native Notifications
Event streams (`/v1/events/stream`) from the daemon are intercepted by the dashboard. In Tauri mode, the dashboard utilizes `@tauri-apps/plugin-notification` to trigger native OS notifications for:
- `approval_requested` (Redacted to: "GOAT needs approval")
- `job_completed` / `job_failed`
- `daemon_started` / `daemon_stopped`

Native notifications strictly omit sensitive output and command execution specifics to ensure security.

## System Tray & Menu (Scaffolded)
Tauri v2 tray logic is scaffolded in `apps/desktop/src-tauri/src/lib.rs`. It provides basic menu items:
- Show GOAT
- Daemon Status
- Quit

*(Note: Full background lifecycle management and deep OS integration is planned for future phases. Currently, closing the Tauri app window closes the frontend, but the daemon may continue running in the background).*
