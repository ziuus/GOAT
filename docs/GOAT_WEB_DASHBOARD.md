# GOAT Web Dashboard

**Version:** 0.13.0 (Phase 4.2)
**Status:** Alpha

The GOAT Web Dashboard is a local-first interface to monitor and control your GOAT Daemon. It connects to the `axum` API over HTTP and uses Server-Sent Events (SSE) for real-time updates.

## Architecture

* **Frontend**: Next.js 15, React 19, TypeScript, TailwindCSS
* **Location**: `apps/dashboard/`
* **Connection**: Communicates with the local GOAT daemon API (`127.0.0.1:47647`)
* **Real-time**: Uses `EventSource` to receive live system events (e.g. `daemon_started`, `job_started`) via SSE at `/v1/events/stream`.
* **Security**: Token-based authentication using `~/.local/share/goat/daemon.token`
* **State**: Allows read-only observability and interactive approval for risky commands via the Approval Queue.

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
2. **Approvals**: Real-time interactive queue to securely review, approve, or deny sensitive operations intercepted by the ApprovalGate.
3. **Jobs**: Lists active and recent scheduled background tasks.
4. **Schedule**: Shows configured background routines from `goat.toml`.
5. **Hooks**: Displays configured lifecycle hooks.
6. **MCP & Tools**: Lists active Model Context Protocol servers and their discovered tools.
7. **Logs**: Real-time view of recent daemon logs.
8. **Settings**: Token configuration and API endpoint configuration.

## Command Integrations

Inside the TUI or Headless mode, you can type `/dashboard` to see the dashboard integration status.

* `/dashboard path` - Prints the physical location of the dashboard code.
* `/dashboard doctor` - Validates if the `package.json` exists.
* `/dashboard dev` - Prints instructions to run the development server.

## Security Model

* **Local Only**: The daemon API and the dashboard are designed for local operation.
* **Token Auth**: The daemon enforces Bearer token authentication. The token is stored in your browser's `localStorage` and never transmitted remotely.
* **Approval Gates**: The dashboard does not bypass GOAT's ApprovalGate. Dangerous operations are intercepted and surfaced in the Approval Queue, maintaining a centralized point of user authorization.

## Next Phases

Phase 4.3+ will introduce interactive chat panels, voice interfaces, and deeper workflow automation components.

## Phase 4.3 Expansions
The Dashboard was upgraded to function as a visual coding companion with:
- **Chat**: Accessible at `/chat`, routes conversational messages to the local GOAT session.
- **Repo Explorer**: Accessible at `/repo`, utilizes `RepoMap` to render the project structure and safe file content previews.
- **Diffs**: Accessible at `/diffs`, provides real-time Git diff inspection of the workspace.
- **Context Management**: Files can be added to the Agent's context directly from the UI.
