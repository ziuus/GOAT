# GOAT — Implementation Roadmap

**Last Updated:** 2026-06-08  
**Current Phase:** 0 (Audit Complete)

---

## Phase 0: Audit and Documentation ✅ COMPLETE

**Goal:** Understand exactly what exists, what works, what is broken, and produce honest documentation before writing any new code.

### Tasks Completed:
- [x] Full repository inspection
- [x] All source files read and understood
- [x] `cargo check` run — identified compile blocker (`ui.rs` missing)
- [x] Git history reviewed (13 commits)
- [x] Reference repositories reviewed and licensed
- [x] Security risks catalogued
- [x] Dependency analysis complete

### Documentation Produced:
- [x] `docs/GOAT_CODEBASE_AUDIT.md`
- [x] `docs/GOAT_PRODUCT_SPEC.md`
- [x] `docs/GOAT_ARCHITECTURE.md`
- [x] `docs/GOAT_FEATURE_MATRIX.md`
- [x] `docs/GOAT_IMPLEMENTATION_ROADMAP.md` (this file)
- [x] `docs/GOAT_SECURITY_MODEL.md`
- [x] `README.md` (rewritten to be accurate)
- [x] `CHANGELOG.md` (created)

### Exit Criteria:
- All 8 documentation files exist ✅
- Compile status is documented ✅
- Feature matrix is honest (no fake "working" claims) ✅

---

## Phase 1: Minimal Working Core

**Goal:** GOAT compiles, runs, and provides a functional basic TUI with one working provider and basic security.

### Priority: CRITICAL — restores compilation

### Tasks:

#### 1.1 Restore TUI (Unblocks Compilation)
- [ ] Create `src/ui.rs` with `pub fn render(f: &mut Frame, app: &App)`
- [ ] Log panel: renders `app.logs` with wrapping
- [ ] Input panel: renders `app.input` with cursor indicator
- [ ] Status bar: shows current agent, model, session ID
- [ ] Normal/Editing mode visual indicator
- [ ] MCP server count in status bar
- [ ] Confirm `cargo check` passes
- [ ] Confirm `cargo run` launches and is interactive

#### 1.2 Security — Approval Gate for Dangerous Tools ✅ COMPLETE
- [x] Create `src/approval.rs` with `ApprovalGate`, `ApprovalRequest`, `ApprovalDecision`, `RiskLevel`, `SessionPolicy`
- [x] Risk assessment for bash commands (`assess_bash_risk`): Critical/High/Medium levels
- [x] Risk assessment for write paths (`assess_write_risk`): Critical/High/Medium levels
- [x] `bash` tool: approval gate checked before executing any command
- [x] `write_file` tool: approval gate checked before writing
- [x] `call_subagent` tool: approval gate checked before spawning
- [x] Session policy: `a` = always allow, `d` = always deny, for lifetime of session
- [x] Deny-by-default: any unrecognised input character → Denied
- [x] Denial message forwarded to LLM in tool-result role so it can adapt
- [x] TUI approval overlay rendered by `src/ui.rs` when approval is pending
- [x] Input box overridden with approval hint during pending state
- [x] Log approval decisions to tracing log
- [x] Secret redaction in command display before showing to user
- [x] 16 unit tests for all approval scenarios — all passing
- [x] `cargo check` passes
- [x] `cargo test` 16/16 pass

#### 1.3 Config and Data Path Fix
- [ ] Move `goat_brain.db` to `~/.local/share/goat/goat_brain.db` (XDG data dir)
- [ ] Update `brain.rs` path to use XDG via `dirs::data_dir()`
- [ ] Add migration: if old `goat_brain.db` exists in project root, warn and optionally migrate

#### 1.4 CLI Improvements
- [ ] Add `clap` dependency
- [ ] Implement `--version` flag
- [ ] Implement `--config` flag for custom config path
- [ ] Implement `--headless` flag for non-TUI mode (print responses to stdout)
- [ ] Implement `--no-brain` flag to run without SQLite

#### 1.5 Session ID Improvement
- [ ] Add `uuid` dependency
- [ ] Replace timestamp-based session IDs with UUID v4
- [ ] Handle existing sessions gracefully

#### 1.6 Error Handling
- [ ] Add `anyhow` for structured error propagation
- [ ] Replace `Box<dyn Error>` with `anyhow::Error` throughout
- [ ] Add context strings to all error returns

### Documentation Update for Phase 1:
- [ ] Update `README.md` with accurate install, run, and config instructions
- [ ] Update `CHANGELOG.md` with Phase 1 entries
- [ ] Update `GOAT_FEATURE_MATRIX.md` to reflect new working status
- [ ] Update `GOAT_SECURITY_MODEL.md` with approval gate details

### Exit Criteria:
- `cargo check` passes ✅
- `cargo run` launches the TUI ✅
- Basic chat works with one provider (OpenAI or Groq) ✅
- `bash` tool prompts for approval before executing ✅
- `write_file` tool prompts for approval before writing ✅
- Brain DB is in XDG data dir ✅

---

## Phase 2: TUI Foundation

**Goal:** A genuinely useful multi-panel TUI with sessions, diff view, and proper task flow.

### Tasks:

#### 2.1 TUI Layout System
- [ ] Implement proper multi-pane layout with resizable panels
- [ ] Panel switching (Tab key or mouse)
- [ ] Scrollable log panel
- [ ] Scrollable chat history panel
- [ ] Sessions list panel (sidebar)

#### 2.2 Session Management UI
- [ ] Sessions list: shows all sessions from brain DB
- [ ] Session switching: select a session and load its history
- [ ] New session command
- [ ] Session rename command
- [ ] Session delete command

#### 2.3 Command Execution Log Panel
- [ ] Separate panel for tool execution logs
- [ ] Distinguish user input, LLM response, tool call, tool result, errors
- [ ] Color coding by message type

#### 2.4 Diff View Panel
- [ ] Show file diffs when `write_file` is about to run
- [ ] Syntax highlight if possible
- [ ] Accept/reject in diff view

#### 2.5 Task State Display
- [ ] Show current task state (idle, thinking, executing, waiting-approval)
- [ ] Show iteration count for ReAct loop
- [ ] Show elapsed time

#### 2.6 Dry-Run Mode
- [ ] `--dry-run` CLI flag
- [ ] In dry-run mode, show what tools would execute without executing them
- [ ] Log dry-run decisions

#### 2.7 Audit Log
- [ ] Write all tool executions to `~/.local/share/goat/audit.log`
- [ ] Format: timestamp, session_id, tool_name, args_hash, result_status, approved_by

#### 2.8 Command Security Controls
- [ ] Implement command blocklist (e.g., `rm -rf /`, `dd`, `mkfs`)
- [ ] Implement path blocklist (e.g., `~/.ssh/`, `/etc/`)
- [ ] Read blocklist from config

### Exit Criteria:
- Full multi-panel TUI working ✅
- Session list and switching working ✅
- Diff view shown before file writes ✅
- Audit log file written ✅

---

## Phase 3: Model Router and Fallback

**Goal:** Multiple providers, fallback chain, retry policy, rate-limit handling, model profiles.

### Tasks:

#### 3.1 Provider Trait
- [ ] Define `Provider` trait in `agent/provider.rs`
- [ ] Wrap existing OpenAI + Groq into trait-implementing structs
- [ ] Unit-test with mock provider

#### 3.2 Anthropic Provider
- [ ] Implement Anthropic Messages API adapter
- [ ] Support Claude Haiku, Sonnet, Opus
- [ ] Tool calling via Anthropic format
- [ ] Add `anthropic_api_key` to config

#### 3.3 Gemini Provider
- [ ] Implement Gemini API adapter
- [ ] Support Gemini Flash, Pro
- [ ] Add `gemini_api_key` to config

#### 3.4 Ollama Provider
- [ ] Implement Ollama local model adapter
- [ ] Auto-detect running Ollama instance
- [ ] List available local models

#### 3.5 OpenRouter Provider
- [ ] Implement OpenRouter adapter (OpenAI-compatible)
- [ ] Add `openrouter_api_key` to config

#### 3.6 Model Router
- [ ] Implement `ModelRouter` struct
- [ ] Fallback chain: ordered list of providers to try
- [ ] Retry with exponential backoff (configurable max retries, initial delay)
- [ ] 429 rate-limit detection: wait and retry
- [ ] Provider health tracking: mark providers as degraded

#### 3.7 Model Profiles
- [ ] Define profiles: cheap, balanced, powerful, local, coding, reasoning, vision, long-context
- [ ] Map profiles to provider/model combinations in config
- [ ] Allow swarm router to use profiles

#### 3.8 Streaming Responses
- [ ] Stream LLM responses chunk by chunk
- [ ] Display partial responses in TUI as they arrive

### Exit Criteria:
- 3+ providers working ✅
- Fallback chain tested (kill provider A, verifies fallback to B) ✅
- Model profiles configurable ✅

---

## Phase 4: Memory and Project Awareness

**Goal:** Session/project memory, project scanner, searchable index, summaries.

### Tasks:
- [ ] Memory search (keyword-based SQLite FTS5)
- [ ] Project-specific memory scope
- [ ] User memory profile
- [ ] Automatic session summarization when context fills
- [ ] Tech stack detector (reads Cargo.toml, package.json, pyproject.toml, etc.)
- [ ] Project command detector (dev/build/test/lint/format/deploy scripts)
- [ ] Configurable scan roots
- [ ] Approval before scanning sensitive paths
- [ ] Project summary generation (send to LLM, get structured summary)
- [ ] Memory edit/delete commands (`/memory list`, `/memory delete <id>`)
- [ ] Memory export (JSON)
- [ ] Memory import (JSON)

### Exit Criteria:
- `learn_about_me` produces usable project summaries ✅
- Memory search returns relevant results ✅
- Tech stack correctly detected for common project types ✅

---

## Phase 5: Subagent System

**Goal:** Internal subagent framework, external adapter interface, safe subprocess execution.

### Tasks:
- [ ] `SubagentAdapter` trait
- [ ] Internal subagent: Coder (runs LLM with coder prompt)
- [ ] Internal subagent: Researcher
- [ ] Internal subagent: Reviewer
- [ ] External adapter: OpenCode (subprocess)
- [ ] Safe subprocess execution with timeout, budget, workspace isolation
- [ ] Subagent output streaming to TUI
- [ ] Subagent logs panel
- [ ] Approval before external agent spawn

### Exit Criteria:
- Internal Coder subagent works ✅
- OpenCode external adapter works (if installed) ✅
- All external spawns require approval ✅

---

## Phase 6: Skills / Plugins and MCP

**Goal:** Skill manifest, loader, MCP support, custom tools.

### Tasks:
- [ ] Skill manifest format (TOML schema)
- [ ] Skill loader from `~/.config/goat/skills/`
- [ ] Skill enable/disable commands
- [ ] Custom commands via skills
- [ ] MCP server launch via skill manifest
- [ ] Browser automation skill (via playwright-mcp or similar)
- [ ] Skill permissions model

### Exit Criteria:
- Load a user-created skill from disk ✅
- Skill's custom tool callable from agent ✅

---

## Phase 7: Voice Prompting

**Goal:** Push-to-talk STT voice input.

### Tasks:
- [ ] `cpal` for audio capture
- [ ] Local whisper.cpp STT integration
- [ ] Push-to-talk key binding in TUI
- [ ] Transcript injection into prompt
- [ ] Voice command history
- [ ] Remote STT fallback (Whisper API)

### Exit Criteria:
- Push-to-talk records and transcribes correctly ✅
- Transcript appears in input field ✅

---

## Phase 8: Dashboard and Advanced Orchestration

**Goal:** Optional web dashboard, multi-agent orchestration, task routing.

### Tasks:
- [ ] axum-based web server (`goat-dashboard`)
- [ ] REST API for sessions, memory, settings
- [ ] Session viewer
- [ ] Memory viewer
- [ ] Settings manager
- [ ] Multi-agent task routing
- [ ] Agent comparison (run same task on multiple agents, compare outputs)

### Exit Criteria:
- Dashboard accessible at localhost:PORT ✅
- Sessions and memory viewable ✅

---

## Dependency Additions by Phase

| Phase | New Dependency | Purpose |
|-------|---------------|---------|
| 1 | `clap` | CLI argument parsing |
| 1 | `uuid` | Session IDs |
| 1 | `anyhow` | Structured error handling |
| 3 | `async-trait` | Async provider trait |
| 3 | `futures` | Async combinators |
| 4 | `rusqlite` FTS5 feature | Memory search |
| 5 | `tokio-stream` | Streaming subprocess output |
| 6 | N/A | Skills use existing deps |
| 7 | `cpal` | Audio capture |
| 7 | `whisper-rs` | Local STT |
| 8 | `axum` | Web dashboard |
| 8 | `tower` | HTTP middleware |
