# GOAT Agent Execution Runtime

**Version:** v1 (Phase 6.3)

The GOAT Agent Execution Runtime is the core engine for executing Prime Agent workflows and background tasks in a resilient, trackable, and approval-gated manner.

## Core Concepts

1. **AgentJob**: A discrete, trackable workflow assigned to a specific Prime Agent. It tracks input, status, artifacts, and a detailed timeline of steps.
2. **Persistence**: Jobs are stored in JSONL format at `~/.local/share/goat/runtime/jobs.jsonl` allowing them to survive daemon restarts.
3. **ApprovalGate Integration**: Jobs that attempt destructive or high-risk actions will automatically enter a `WaitingForApproval` state, pausing execution until the user approves or denies the action via the TUI or Dashboard.
4. **Lifecycle**: `Pending` -> `Running` <-> `Paused` / `WaitingForApproval` -> `Completed` / `Failed` / `Cancelled`.

## Architecture

* **Backend (`src/agent_runtime.rs`)**: Manages state transitions, file persistence, and safe execution. Integrated into `GoatRuntime`.
* **API Server (`src/api_server.rs`)**: Exposes REST endpoints (`/v1/runtime/jobs/*`) for the dashboard and external clients.
* **TUI Integration (`src/app.rs`)**: Users can manage jobs via the `/jobs` slash command in the interactive chat.
* **Dashboard (`apps/dashboard/src/app/runtime`)**: A React UI for monitoring active jobs, pausing/resuming, and viewing artifacts.

## Usage (TUI)

```bash
# List all active and recent jobs
/jobs list

# Start a new agent job manually
/jobs run <agent_id> <task_description>

# Pause a running job
/jobs pause <job_id>

# Resume a paused job
/jobs resume <job_id>

# Cancel an active job
/jobs cancel <job_id>

# Retry a failed or cancelled job
/jobs retry <job_id>
```

## Security & Constraints

- The runtime **never** bypasses ApprovalGate.
- The runtime **does not** create invisible background agents. All activity is logged and visible in the UI.
- The runtime enforces risk thresholds configured by the user, pausing for any action exceeding `Medium` or `High` risk based on settings.

## Future Phases

- Integration of complex branching workflows.
- Agent Collaboration (Handoffs) executing natively as multi-agent jobs within the runtime.
- Streaming real-time job events to the Dashboard via Server-Sent Events (SSE).
