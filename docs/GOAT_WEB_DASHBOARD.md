# GOAT Web Dashboard

**Version:** 0.13.0 (Phase 4.1)
**Status:** Alpha

The GOAT Web Dashboard is a local-first, read-only interface to monitor your GOAT Daemon. It connects to the `axum` API over HTTP.

## Architecture

* **Frontend**: Next.js 15, React 19, TypeScript, TailwindCSS
* **Location**: `apps/dashboard/`
* **Connection**: Communicates with the local GOAT daemon API (`127.0.0.1:47647`)
* **Security**: Token-based authentication using `~/.local/share/goat/daemon.token`
* **State**: Read-only observability for Phase 4.1. No remote telemetry or cloud sync.

## Setup & Running

To use the dashboard, you must first start the GOAT daemon:

```bash
cargo run -- daemon start
```

Retrieve your daemon token (auto-generated on first daemon start):
```bash
cat ~/.local/share/goat/daemon.token
```

Then, run the dashboard development server:

```bash
cd apps/dashboard
npm install
npm run dev
```

Visit `http://localhost:3000` in your browser. Upon first load, you will be prompted to enter the API URL and the daemon token.

## Views

1. **Overview**: Displays daemon health, GOAT version, active profile, and system specs.
2. **Jobs**: Lists active and recent scheduled background tasks.
3. **Schedule**: Shows configured background routines from `goat.toml`.
4. **Hooks**: Displays configured lifecycle hooks.
5. **MCP & Tools**: Lists active Model Context Protocol servers and their discovered tools.
6. **Logs**: Real-time view of recent daemon logs.
7. **Settings**: Token configuration and API endpoint configuration.

## Command Integrations

Inside the TUI or Headless mode, you can type `/dashboard` to see the dashboard integration status.

* `/dashboard path` - Prints the physical location of the dashboard code.
* `/dashboard doctor` - Validates if the `package.json` exists.
* `/dashboard dev` - Prints instructions to run the development server.

## Security Model

* **Local Only**: The daemon API and the dashboard are designed for local operation.
* **Token Auth**: The daemon enforces Bearer token authentication. The token is stored in your browser's `localStorage` and never transmitted remotely.
* **Approval Gates**: The dashboard does not bypass GOAT's ApprovalGate. Dangerous operations are not executed directly from the dashboard UI at this time.

## Next Phases

Phase 4.2+ will introduce real-time WebSocket events, interactive chat panels, and secure approval queues for remote execution.
