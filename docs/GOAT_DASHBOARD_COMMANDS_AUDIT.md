# GOAT Dashboard: Safe Commands and Audit Explorer

This document details Phase 4.4 features designed to turn the web dashboard into a safe control center without compromising the GOAT security model.

## Command Center (`/commands`)

The Command Center provides a web UI to execute GOAT slash commands against the daemon API.

### Safe Command Classification
To prevent security bypasses, commands are classified into safe and risky:
* **Safe Read-Only Commands**: Executed immediately. Examples: `/status`, `/jobs`, `/schedule`, `/hooks`, `/mcp status`, `/repo`, `/changes`, `/context show`.
* **Risky Operations**: Cannot be executed directly. When a destructive command (e.g. `/bash`, `/write_file`, `/commit`) is sent via `/v1/command`, the API intersects it and automatically creates a new pending `ApprovalRequest`.
* **Unknown Operations**: Blocked with an error.

The frontend receives a special `approval_required: true` response for risky commands, displaying a risk badge and linking to the newly generated Approval Request ID.

## Approval History

The `ApprovalQueue` has been extended to retain a full history of all processed approval requests in the current session.

* **Tracking**: Stores the original `ApprovalRequest`, creation timestamp, resolution timestamp, source (`dashboard`, `tui`, etc.), and final decision (`y` or `n`).
* **UI Integration**: The dashboard `/approvals` page now contains two tabs: **Pending Queue** and **History Log**.
* **Events**: `approval_requested` and `approval_resolved` events are now emitted over SSE to keep all dashboard clients perfectly in sync.

## Audit Explorer (`/audit`)

The Audit Explorer provides a graphical interface to browse redacted system audit logs.

* **Sources**: Aggregates logs from `tool-audit.log` and `scheduler-audit.log`.
* **Safety**: Employs `redact_secrets` logic to mask sensitive payload variables before delivering the text to the dashboard. The dashboard token is also stripped dynamically.
* **UI**: Includes category filtering and real-time frontend search filtering.

## Security Constraints Enforced

* **No ApprovalGate Bypass**: The dashboard is strictly bound by `ApprovalGate`. Safe command routes are explicitly whitelisted; everything else defers to the approval queue.
* **Local-Only Enforcements**: Connection settings proactively warn if configuring a remote URL, reinforcing the "local daemon only" topology.
* **Token Redaction**: Tokens are never exposed in audit logs, stdout, or UI rendering.

## Next Steps
The final UI feature phase (Phase 4.5) will build upon the Command Center to implement async agent tasks and long-running job execution.
