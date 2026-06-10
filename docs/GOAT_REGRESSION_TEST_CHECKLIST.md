# GOAT Regression Test Checklist

Before any major release or architectural shift (e.g., Phase 5 Tauri Wrapper), the following manual regression tests must be executed.

## 1. Core Daemon & Execution
- [ ] `cargo run -- daemon` starts cleanly on `127.0.0.1:47647`.
- [ ] Daemon creates/updates `~/.local/share/goat/daemon.token` securely.
- [ ] `cargo run -- doctor` successfully identifies configuration health.
- [ ] `cargo run -- headless "What is 2+2?"` returns a valid LLM response.

## 2. Terminal UI (TUI)
- [ ] `cargo run -- tui` opens the multi-pane interface without panicking.
- [ ] Submitting a prompt in the TUI input box successfully triggers an LLM generation.
- [ ] `/help` slash command populates the log view correctly.
- [ ] System logs appear in the event pane seamlessly.

## 3. Web Dashboard (Next.js)
- [ ] Dashboard builds cleanly (`npm run build`).
- [ ] Dashboard Settings page correctly persists authentication token and URL.
- [ ] Theme toggling (Dark/Minimal/High-Contrast) works and persists via `localStorage`.
- [ ] Global Command Palette (`Ctrl+K`) opens and navigates correctly.

## 4. Chat & Async Jobs
- [ ] Sending a message in the Dashboard Chat generates a background `tokio::spawn` job.
- [ ] The SSE Event Bus updates the UI live with `job_started` and `job_completed` events.
- [ ] Message bubbles differentiate visually between User and Assistant.

## 5. Security & Approvals (`ApprovalGate`)
- [ ] Running a destructive command in `Act` mode triggers an `approval_required` state.
- [ ] The action is paused and does not execute blindly.
- [ ] Approving the action via Dashboard `/approvals` or TUI allows the job to resume.
- [ ] Denying the action cleanly aborts the operation.

## 6. Repo & Context
- [ ] Dashboard Repo Explorer `/repo` correctly maps local workspace files.
- [ ] Custom graceful file viewer displays line numbers and syntax safely.
- [ ] Dashboard Diff Viewer `/diffs` correctly applies green/red colors to git diff hunks.
- [ ] Adding a file to Context successfully injects it into the active session.
