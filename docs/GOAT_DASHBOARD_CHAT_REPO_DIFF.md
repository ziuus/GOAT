# GOAT Dashboard: Chat, Repo Explorer, and Diffs

Phase 4.3 expands the Web Dashboard into a true visual coding companion. The dashboard is now equipped with the foundations for Chat, Repo navigation, Context Management, and Diff viewing. 

## Features

### 1. Chat Foundation (`/chat`)
- Provides a conversational UI to interact with GOAT via the daemon.
- Currently processes messages and connects to the active session.
- Real-time streaming and dangerous tool executions via chat will be routed through the `ApprovalQueue` in Phase 4.4.

### 2. Repo Explorer (`/repo`)
- Leverages GOAT's native `RepoMap` to render an interactive file tree.
- Clicking files fetches content safely through the `/v1/repo/file` endpoint.
- Protects secret files (`.env`, `*.pem`, etc.) and automatically redacts inline secrets.
- Allows files to be dynamically added to the Agent's Context via the `/v1/context` API.

### 3. Diff Viewer (`/diffs`)
- Renders the current workspace git diff with syntax highlighting in the browser.
- Uses local Git executable and returns truncated diffs for extremely large changes.
- Read-only: patch application logic is strictly guarded by the backend ApprovalGate.

## Security Model
- **No Path Transversal**: Repo Explorer refuses requests outside the project directory.
- **Redaction**: All files requested from the Dashboard undergo secret redaction via `ApprovalGate::redact_secrets`.
- **Token Auth**: All API endpoints remain locked behind Bearer token authentication.

## Limitations & Next Steps
- Real-time streaming LLM chat isn't fully bound to the dashboard yet.
- Modifying files directly from the Repo Explorer is disabled by design.
- The next step (Phase 4.4) will complete the real-time chat pipeline and asynchronous job-based agent actions.
