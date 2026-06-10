# GOAT Dashboard Agent Chat (Async)

## Overview
Phase 4.5 of GOAT introduces **Async Dashboard Agent Chat**. This connects the frontend web dashboard to the local Rust Daemon via background jobs and Server-Sent Events (SSE). It allows users to have long-running interactions with GOAT without blocking the API or the UI.

## Architecture

The system uses three main components:
1. **Frontend (React/Next.js)**
2. **REST API (`POST /v1/chat`)**
3. **Background Job Tracker (`JobTracker`)**
4. **Event Bus (`EventBus` via SSE)**

### 1. The Frontend UI
The UI is built with Next.js and Tailwind CSS. It connects to the daemon using an API client (`goat-api.ts`) and an SSE client (`goat-events.ts`). The SSE client listens for live events (e.g., `job_started`, `chat_message`, `job_completed`, `approval_required`).
The chat interface includes a Mode Selector allowing users to switch between "Chat", "Plan", and "Act".

### 2. Async `chat_handler`
When the frontend sends a chat message (`POST /v1/chat`), the API server creates a `BackgroundJob` of type `chat`. It returns a 200 OK immediately with the `job_id` and the `status` as `queued`.
The daemon spawns a `tokio::spawn` task to process the LLM request asynchronously, so the API router remains unblocked.

### 3. LLM Processing & Context
Inside the background job, the daemon uses `LlmRouter` to process the message. It loads the `session_id`'s history from the `Brain` (SQLite DB) and injects the proper system prompts based on the selected mode (e.g. adding tool-usage permission for ACT mode).

### 4. Approval Integration (Safety First)
If the AI decides to execute a tool (e.g. creating a file, running a shell command) during an ACT mode chat session, the job marks itself as `approval_required`.
It pauses execution and emits an `ApprovalRequest` into the daemon's `ApprovalQueue`.
The SSE channel notifies the frontend, displaying an "Approval Required" banner. The user can review the request on the `/approvals` dashboard page.

### 5. Live Events via SSE
The entire process is transparent to the frontend via Server-Sent Events:
- `job_created`: Job is registered.
- `job_started`: The async task began execution.
- `chat_message`: The LLM returned a partial/final response.
- `job_completed` / `job_failed`: Terminal job states.
- `approval_required`: The job is paused pending user authorization.

## Security Constraints
- **Local Only:** Like the rest of the daemon, the chat relies on `127.0.0.1` binding and token authentication.
- **Approval Gate:** The dashboard cannot automatically apply patches or run commands without explicit approval queue ingestion. All high-risk actions fallback to `approval_required`.
- **No Direct Shell Over API:** Users cannot type `/bash` commands directly into the chat prompt; they must be routed as safe tool calls subject to the ApprovalGate.
