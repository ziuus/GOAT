# CHANGELOG

All notable changes to GOAT are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — Phase 1: Minimal Working Core

### Added — Phase 1.1: ApprovalGate for Dangerous Tools (2026-06-08)

**New file: `src/approval.rs`**
- `RiskLevel` enum: `Low`, `Medium`, `High`, `Critical`
- `ApprovalRequest`: describes a proposed dangerous action (tool name, action summary, risk, explanation, working directory)
- `ApprovalDecision`: `Approved` or `Denied(reason)`
- `SessionPolicy`: `AlwaysApprove` or `AlwaysDeny` — per-tool session overrides
- `ApprovalGate`: holds session policies, checks policy, resolves interactive input
- `assess_bash_risk()`: classifies bash commands as Critical/High/Medium using pattern matching
  - Separate root-targeted check for `rm -rf /` (avoids false positives like `rm -rf /tmp/foo`)
  - Pure-substring critical patterns: `mkfs`, `dd if=`, `shred`, etc.
  - High-risk patterns: `sudo`, `rm -rf`, `| sh`, `| bash`, `.ssh`, `.env`, `kill -9`, etc.
- `assess_write_risk()`: classifies write paths as Critical/High/Medium
- `assess_subagent_risk()`: classifies external agent spawns
- `bash_approval_request()`: builds `ApprovalRequest` for bash tool calls
- `write_file_approval_request()`: builds `ApprovalRequest` for write_file calls  
- `call_subagent_approval_request()`: builds `ApprovalRequest` for subagent spawns
- `redact_secrets()`: basic heuristic redaction of API keys/tokens before display
- 16 unit tests — all passing

**Modified: `src/app.rs`**
- `App` struct gains `approval_gate: ApprovalGate` and `pending_approval: Option<DeferredToolCall>`
- `DeferredToolCall` struct: stores paused tool call awaiting approval
- `App::has_pending_approval()`: queried by TUI event loop
- `App::pending_approval_lines()`: used by `ui.rs` to render the overlay
- `App::resolve_approval(char)`: called by event loop with user's keypress; executes or blocks the deferred tool
- `handle_user_input()`: now calls `build_approval_request()` before every dangerous tool call
  - Checks session policy first (immediate decision)
  - Pauses execution and sets `pending_approval` for interactive cases
  - Returns early from agent loop; resumes after `resolve_approval()` is called
- `build_approval_request()`: routes by tool name to the correct approval builder
- `execute_native_tool()`: dedicated post-approval execution function
- Startup log now shows security status and key bindings

**Modified: `src/ui.rs`**
- Colour-coded log lines by prefix: `[ERROR]`=red, `[APPROVAL]`=yellow+bold, `[LLM]`=green, `[TOOL]`=cyan, etc.
- Approval-mode input box: shows approval key hint when `has_pending_approval()` is true
- Centred ratatui `Clear`+`Paragraph` overlay rendered when approval is pending
- Active Mission panel turns red+bold during approval wait

**Modified: `src/main.rs`**
- `run_app()` event loop: when `app.has_pending_approval()` is true, all keys are routed to `app.resolve_approval(c)`
- Normal shortcuts suspended during approval wait (safe: no accidental executions)
- Added `mod approval;`

**Modified: `src/tools.rs`**
- Added `call_subagent` to `all_tools()` (was missing)
- Added safety-invariant doc comment: `execute()` is post-approval only
- Improved descriptions noting approval requirement

### Added — Phase 1.2A: Agent Feature Research (2026-06-08)

**New file: `docs/GOAT_AGENT_FEATURE_RESEARCH.md`**
- Researched 19 reference agents/tools for features, UX patterns, and architecture
- Tools covered: OpenCode, Claude Code, Gemini CLI / Antigravity, OpenAI Codex CLI/Cloud, GitHub Copilot, Aider, Cline, Continue, Devin, Jules, Cursor, Windsurf, Hermes, JCode, Little Bird AI, Pi, OpenClaw, GitHub Copilot CLI
- License compatibility table for all researched tools
- Master feature blueprint with priority order for GOAT roadmap
- Planned slash command catalog (/help /status /mcp /learn /route /clear /tools /sessions /plan /review etc.)
- Architecture principles informed by research (harness design, GOAT.md, Plan/Act, git-native, repo map, etc.)

### Added — Phase 1.2B: TUI/UX Overhaul (2026-06-08)

**Complete interaction model change:**
- **Removed modal `InputMode`** — no more "press i to type" Vim-style mode switching
- GOAT now works like a normal chat app: type immediately, Enter sends, Ctrl+C quits

**Modified: `src/app.rs`**
- Removed `InputMode::Normal/Editing` enum entirely
- Added `AppStatus` enum: `Ready`, `Thinking`, `ToolRunning(tool)`, `WaitingApproval(tool)`, `Error(msg)`
- Added `log_scroll: usize` field for user-controllable log scrolling
- Added `provider_label: String` for status bar display
- Added `mcp_server_count: usize` for status bar
- Added `scroll_up()`, `scroll_down()`, `scroll_to_bottom()` methods
- Added `handle_slash_command()` — slash command dispatcher
  - `/help` — full help text with all commands and key bindings
  - `/status` — show provider/session/brain/history status
  - `/mcp` — start MCP servers
  - `/learn` — trigger project indexing
  - `/route` — show swarm route
  - `/clear` — clear log display
  - `/tools` — list native + MCP tools
  - `/sessions` — show session history from brain
- `handle_user_input()` dispatches slash commands before sending to LLM
- User messages now shown as `[YOU]` instead of `[USER]` (more chat-like)
- Agent responses shown as `[GOAT]` instead of `[LLM]`
- Friendly startup splash instead of debug-like system messages
- Auto-scrolls to bottom after responses and after approval resolution

**Modified: `src/main.rs`**
- Removed all `InputMode` references — no more modal switching
- `run_app()` now: Enter sends, Char pushes, Backspace pops (always active)
- Added `Ctrl+C` for clean exit (safe, works anywhere)
- Added `Up/Down` arrow keys for log scrolling (1 line)
- Added `PageUp/PageDown` for fast scroll (10 lines)
- Added `Home/End` for jump to top/bottom of log
- `Esc` clears input if non-empty; scrolls to bottom if input already empty
- Approval mode: only `y/n/a/d` and `Esc` (= deny) intercepted; all other keys ignored
- Removed `q` quit (was only in Normal mode; replaced by universal Ctrl+C)

**Modified: `src/ui.rs`**
- **New 3-panel layout:**
  - Row 0: Header bar (1 line, always visible)
  - Row 1: Chat + log panel (fills available height, scrollable)
  - Row 2: Input composer (3 lines, always visible at bottom)
- **Header bar:** `GOAT v0.1 │ provider:model │ Session:ID │ [MCP:N] │ STATUS`
  - Status shows: `● READY` / `◌ THINKING…` / `⚙ RUNNING` / `⚠ APPROVAL` / `✕ ERROR`
  - Status color changes by state (green/amber/blue/orange/red)
- **Input composer:**
  - Placeholder text: `Ask GOAT anything… (Enter to send · Ctrl+C to quit · /help for commands)`
  - During approval: replaced with `⚠ Approval required — [y] approve [n] deny [a] always allow [d] always deny [Esc] deny`
  - Cursor always visible in correct position
- **Log panel:**
  - Rich RGB colour-coding by message prefix
  - `[YOU]` = soft blue + bold (user messages stand out)
  - `[GOAT]` = soft green (assistant responses)
  - `[TOOL]` = purple
  - `[AGENT]` = lighter blue
  - `[APPROVAL] ✓` = green, `[APPROVAL] ✗` = red, `[APPROVAL]` = amber
  - `[ERROR]` = bright red + bold
  - `[SYSTEM]` = dim grey
  - `[HELP]` / `[STATUS]` = teal / yellow
  - Scroll indicator: `[↑↓ scroll | End = bottom | N lines above]` when scrolled up
- **Approval overlay:** centred modal with proper key hints, amber/red colour scheme

**cargo check:** 0 errors, 8 dead_code warnings (public API)  
**cargo test:** 16/16 pass  
**cargo fmt:** clean

### Planned (remaining Phase 1 tasks)
- Restore `src/ui.rs` to fix compilation blocker (already done in Phase 0 recovery)
- Move brain DB to XDG data directory
- Add `clap` for CLI argument parsing
- Add `uuid` for session IDs
- Add `anyhow` for structured error handling
- Warn if config file is world-readable
- Confirm scanning before `learn_about_me`

---

## [0.1.0] — Phase 0 (2026-06-08)

### Added — Phase 0: Audit and Documentation
- Full codebase audit (`docs/GOAT_CODEBASE_AUDIT.md`)
- Product specification (`docs/GOAT_PRODUCT_SPEC.md`)
- Architecture document (`docs/GOAT_ARCHITECTURE.md`)
- Feature matrix (`docs/GOAT_FEATURE_MATRIX.md`)
- Implementation roadmap (`docs/GOAT_IMPLEMENTATION_ROADMAP.md`)
- Security model (`docs/GOAT_SECURITY_MODEL.md`)
- `docs/` directory created
- This `CHANGELOG.md`
- README rewritten to be accurate (removed false feature claims)

### Found — Compile Blocker
- `src/ui.rs` is missing; `main.rs` references `mod ui;` which does not exist
- `cargo check` fails with `error[E0583]: file not found for module ui`

### Assessed — Existing Code (from prior commits)
- `src/brain.rs` — SQLite memory, session storage, file indexer with SHA-256 dedup
- `src/mcp.rs` — STDIO JSON-RPC MCP client with multi-server management
- `src/agent/litellm.rs` — OpenAI-compatible HTTP client (OpenAI + Groq)
- `src/agent/manager.rs` — ReAct agent loop (plan → act → observe, up to 10 iterations)
- `src/swarm.rs` — Keyword-based intent router (Coder/Browser/Researcher/General profiles)
- `src/config.rs` — TOML config loader with env var and OpenCode config fallback
- `src/tools.rs` — Native tools: bash, read_file, write_file, call_subagent (all WITHOUT approval gates)
- `src/app.rs` — Application state container, session resume, log management

### Security Gaps Identified
- `bash` tool executes commands without user approval — HIGH risk
- `write_file` tool writes without user approval — HIGH risk
- `call_subagent` spawns arbitrary CLIs without approval — MEDIUM risk
- `learn_about_me` scans home directories without consent — LOW risk
- Brain DB stored in project root (should be in XDG data dir) — LOW risk

---

## Prior History (from git log, pre-audit)

### [pre-0.1.0] — 2026-06-08 (commit 52150d3)
- feat: read API keys and base url from opencode config fallback

### [pre-0.1.0] — (commit 6ce8a04)
- feat: complete session persistence and fix syntax errors

### [pre-0.1.0] — (commit fcbb84d)
- feat: add native bash and file tools

### [pre-0.1.0] — (commit 58c14c1)
- fix: harden history bounding and sqlite durability

### [pre-0.1.0] — (commit 80bd6d7)
- fix: bound logs and harden config

### [pre-0.1.0] — (commit 69ff86d)
- fix: add LLM and MCP timeouts

### [pre-0.1.0] — (commit 2048df7)
- fix: harden MCP server lifecycle

### [pre-0.1.0] — (commit babc027)
- feat: add persistent MCP manager

### [pre-0.1.0] — (commit 778ed56)
- feat: add subagent routing

### [pre-0.1.0] — (commit 0606bb2)
- feat: add learn about me indexing

### [pre-0.1.0] — (commit 58e2a3f)
- feat: add production agent loop and logging

### [pre-0.1.0] — (commit 155810c)
- chore: ignore references directory

### [pre-0.1.0] — (commit 008d486)
- Initial commit: Core MVP for GOAT (TUI, MCP, Memory, LLM Routing)

> Note: `src/ui.rs` that was present in earlier commits was lost (likely accidentally deleted or not committed). The app ran successfully as evidenced by log entries from 07:37:30 and 13:30:23 on 2026-06-08.
