# GOAT Daemon and Local API

GOAT provides a persistent background service (Daemon) that enables background operations, scheduled tasks, and local integrations via a secure API.

## Starting the Daemon

```bash
# Start the daemon in the foreground
goat daemon start

# View the status of the daemon
goat daemon status

# Run a diagnostics check on daemon configuration and reachability
goat daemon doctor
```

## Security & Authentication

The GOAT Daemon is designed to be **safe by default**:
- **Bind Address**: Defaults to `127.0.0.1:47647` (no public network access).
- **Authentication**: A secure Bearer token is automatically generated upon first start and saved to `~/.local/share/goat/daemon.token` with strict UNIX permissions (`chmod 600`).
- **Authorization**: The API requires the `Authorization: Bearer <token>` header for all requests unless explicitly disabled in the configuration (not recommended).

## API Endpoints

The API is structured around local observability and non-destructive operations for now.

- `GET /health`: Returns daemon version and health status.
- `GET /v1/status`: Detailed runtime metrics, model profile, job counts, etc.
- `GET /v1/jobs`: List of background jobs (active and historical).
- `GET /v1/jobs/:id`: Fetch specific job details.
- `GET /v1/hooks`: List all registered hooks.
- `GET /v1/schedule`: List all scheduled background tasks.
- `GET /v1/mcp/status`: Details on connected MCP server states.
- `GET /v1/logs/recent`: Fetch recent daemon logs (tokens automatically redacted).
- `POST /v1/command`: Submit a slash command for execution. 
  - *Note: Only safe, read-only commands (e.g., `/status`, `/jobs`, `/hooks`, `/schedule`) are currently permitted. Risky or state-altering commands will return a `403 Forbidden` requesting manual TUI approval.*
- `POST /v1/chat`: Send a chat message (currently asynchronous foundation).
- `GET /v1/repo/tree`: Fetch `RepoMap` workspace tree structure.
- `GET /v1/repo/file?path=...`: Safely read a file, automatically redacting secrets.
- `GET /v1/diffs`: Read-only view of the current uncommitted Git diff.
- `GET /v1/context`, `POST /v1/context/add`, `POST /v1/context/remove`, `POST /v1/context/clear`: Workspace context management.
- `GET /v1/sessions`: Fetch session history and details.

## Interaction with TUI and Headless Mode

- **Scheduler Persistence**: When the Daemon is running, it assumes responsibility for ticking the `SchedulerManager`. If you run the TUI or Headless mode while the Daemon is active, GOAT will warn you that duplicated jobs may occur if both processes attempt to execute scheduled events simultaneously.
- You can observe Daemon state directly from the TUI/Headless modes using the `/daemon status` and `/api status` slash commands.
