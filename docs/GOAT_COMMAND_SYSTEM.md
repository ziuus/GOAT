# GOAT Command System

GOAT implements a unified slash command registry across TUI, Headless, and Daemon modes.

## Available Commands

### General
- `/help`: Show all available commands.
- `/clear`: Clear the console output history.
- `/status`: Show system, provider, and context status.

### Memory & Sessions
- `/sessions`: List all available past sessions.
- `/memory add-user <text>`: Add preferences to `USER.md`.
- `/memory add-note <text>`: Add knowledge to `MEMORY.md`.
- `/memory status`: View memory budget.

### Context (Phase 3.6 / Phase 4.3)
- `/context add <path>`: Add file to context.
- `/context remove <path>`: Remove file from context.
- `/context clear`: Clear context.
- `/context budget`: Check token footprint.

### Skills & Tools
- `/tools list`: List available tools.
- `/mcp list`: List MCP servers and tools.

### Background Jobs & Hooks
- `/jobs list`: View active and historical background tasks.
- `/schedule list`: View automated execution schedules.
- `/hooks list`: View registered pre/post action hooks.

### Daemon (Phase 4.0+)
- `/daemon status`: Inspect local daemon.
- `/daemon doctor`: Check daemon reachability.
- `/api status`: View REST API stats.

### Phase 4.3 Dashboard CLI (Planned)
- `/dashboard chat`: Open dashboard chat page in browser.
- `/dashboard repo`: Open dashboard repo explorer.
- `/dashboard diffs`: Open dashboard diff viewer.
