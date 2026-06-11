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

#### 1.1 Restore TUI (Unblocks Compilation) ✅ COMPLETE
- [x] Create `src/ui.rs` with `pub fn render(f: &mut Frame, app: &App)`
- [x] Log panel: renders `app.logs` with wrapping and RGB colour-coding
- [x] Input panel: renders `app.input` with cursor indicator and placeholder text
- [x] Status bar (header): shows provider/model, session ID, MCP count, AppStatus
- [x] Approval overlay panel rendered when approval is pending
- [x] `cargo check` passes
- [x] `cargo run` launches and is interactive

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

#### 1.2A Agent Feature Research ✅ COMPLETE
- [x] Research 19 reference agent platforms
- [x] Document features, UX patterns, architecture ideas, license status for each
- [x] Create `docs/GOAT_AGENT_FEATURE_RESEARCH.md` (master blueprint)
- [x] License compatibility table
- [x] Master feature priority blueprint for GOAT roadmap
- [x] Slash command catalog planned
- [x] Architecture principles documented (GOAT.md, Plan/Act, repo map, git-native, etc.)

#### 1.2B TUI/UX Overhaul ✅ COMPLETE
- [x] Remove modal `InputMode` (no more "press i to type")
- [x] Always-active input composer at bottom
- [x] Placeholder text: "Ask GOAT anything..."
- [x] Ctrl+C for clean exit
- [x] Arrow key / PageUp / PageDown log scrolling
- [x] Home/End to jump to top/bottom of log
- [x] Esc clears input (or scrolls to bottom if input empty)
- [x] Slash command dispatcher (`/help`, `/status`, `/mcp`, `/learn`, `/route`, `/clear`, `/tools`, `/sessions`)
- [x] Unknown slash command shows helpful error
- [x] Header bar: provider/model, session, MCP count, status with color
- [x] AppStatus enum: Ready, Thinking, ToolRunning, WaitingApproval, Error
- [x] Rich RGB colour palette for all message types
- [x] Scroll indicator shows "N lines above" when scrolled up
- [x] Input composer approval override during pending approval
- [x] Approval overlay improved with key hints and colour
- [x] Auto-scroll to bottom after LLM response
- [x] User messages shown as `[YOU]` (more chat-like)
- [x] Agent responses shown as `[GOAT]`
- [x] `cargo check` 0 errors
- [x] `cargo test` 16/16 pass

#### 1.3 Foundation Cleanup ✅ COMPLETE

- [x] Move brain DB to XDG path (`~/.local/share/goat/goat.db`)
- [x] `src/paths.rs` — `GoatPaths` struct, XDG path resolution
- [x] `src/error.rs` — typed `GoatError` enum (thiserror)
- [x] `src/cli.rs` — clap CLI: `--version`, `--config`, `--db`, subcommands
- [x] `goat config-path`, `goat data-path`, `goat db-path` — path inspection commands
- [x] `goat doctor` — system readiness health report
- [x] `goat migrate-db` — copy legacy DB to XDG path
- [x] `src/config.rs` — anyhow throughout, `ConfigLoadResult` with warnings
- [x] Config `chmod 600` auto-applied on Unix at creation
- [x] Config permission check (mode bits) in doctor + TUI startup warning
- [x] UUID v4 session IDs for new sessions (old IDs preserved)
- [x] Legacy DB detection: warns in doctor and stderr before TUI
- [x] Logs moved to `~/.local/share/goat/logs/` (XDG, not `./logs/`)
- [x] 10 new unit tests for paths, XDG, doctor, permissions
- [x] `cargo test` 26/26 pass
- [x] Version bumped to 0.2.0

#### 1.4 Headless Mode + Runtime Separation ✅ COMPLETE

- [x] `src/runtime.rs` — `GoatRuntime` struct: shared, surface-agnostic bootstrap
  - [x] `GoatRuntime::bootstrap()` — single unified init: brain, session, LLM, gate
  - [x] Shared by both TUI (`App::from_runtime()`) and headless (`headless::run()`)
  - [x] Returns `(GoatRuntime, boot_log)` — no code duplication
- [x] `src/headless.rs` — complete non-TUI agent loop
  - [x] Banner: provider, session, brain path, mode (new/resumed)
  - [x] Stdin line reader, EOF/Ctrl+D exit
  - [x] Slash commands: `/help` `/status` `/clear` `/sessions` `/tools` `/exit`
  - [x] Full ReAct LLM loop (same tools, same providers, same DB)
  - [x] `prompt_approval_stdin()` — blocking y/n/a/d prompt, re-prompts on invalid, denies on EOF
  - [x] Same `ApprovalGate` as TUI — deny-by-default
  - [x] MCP server shutdown on exit
- [x] `--headless` flag added to `Cli` (global, composable with `--config`, `--db`)
- [x] `goat sessions` CLI subcommand — lists sessions from brain DB
- [x] `goat doctor` improved:
  - [x] Provider count: `N of 2 providers configured`
  - [x] DB migration status: OK/WARN for legacy DB
  - [x] Headless mode: shows readiness
  - [x] ApprovalGate + Log directory preserved
- [x] `App::from_runtime()` constructor added — TUI App now built from GoatRuntime
- [x] `App::new()` preserved as thin wrapper (backward compat, tests)
- [x] `main.rs` routes `--headless` → `headless::run()`, default → `run_tui()`
- [x] All Phase 1.2 TUI UX preserved
- [x] All 26 tests still pass
- [x] Version bumped to 0.3.0

**NOT implemented in Phase 1.4 (deferred):**
- `--no-brain` flag (deferred to Phase 1.5 — needs runtime field)
- Workspace/crate split (deferred — full multi-crate split is Phase 3+)
- anyhow adoption in `brain.rs`, `llm.rs`, `tools.rs` (partial — deferred to Phase 1.5)

### Documentation Update for Phase 1: ✅ COMPLETE
- [x] `README.md` with accurate install, run, and config instructions
- [x] `CHANGELOG.md` with Phase 1.1 through 1.4 entries
- [x] `GOAT_FEATURE_MATRIX.md` reflects new working status
- [x] `GOAT_SECURITY_MODEL.md` with approval gate details
- [x] `GOAT_ARCHITECTURE.md` documents runtime separation

### Exit Criteria: ✅ ALL PASSED
- `cargo check` passes ✅
- `cargo run` launches the TUI ✅
- `cargo run -- --headless` starts headless loop ✅
- Basic chat works with one provider (OpenAI or Groq) ✅
- `bash` tool prompts for approval before executing ✅
- `write_file` tool prompts for approval before writing ✅
- Brain DB is in XDG data dir ✅
- `goat sessions` lists sessions ✅
- `goat doctor` shows full readiness report ✅

---


#### 1.5 Provider Abstraction + Model Profiles + Fallback Chain ✅ COMPLETE

- [x] `src/provider.rs` — `ProviderError` enum with recoverability/retryability classification
  - [x] `ProviderError::from_http()` — HTTP status → typed error
  - [x] `.is_recoverable()` — drives fallback chain advancement
  - [x] `.is_retryable()` — drives retry-same-provider logic
  - [x] `ProviderStatus` — `Ready`, `NotConfigured`, `Planned`
  - [x] 10 unit tests
- [x] `src/models.rs` — `ModelEntry`, `ModelChain`, `ProfilesConfig`, `ProfileRegistry`
  - [x] `ProfileRegistry::from_config()` — merges user config + built-in defaults (user wins)
  - [x] `ProfileRegistry::with_defaults()` — built-in defaults only
  - [x] `ProfileRegistry::resolve(name)` — falls back to default → balanced
  - [x] 6 built-in profiles: `balanced`, `cheap`, `powerful`, `coding`, `reasoning`, `local`
  - [x] `ProfilesConfig` — TOML-deserializable with `#[serde(default)]` support
  - [x] 12 unit tests
- [x] `src/llm.rs` — `completion_with_fallback()`, typed `ProviderError`, retry policy
  - [x] MAX_RETRIES=2 with 500ms×attempt delay for retryable errors
  - [x] Chain advances on recoverable errors; stops on non-recoverable
  - [x] Returns `(MessageContent, used_label)` — surface knows which model was used
  - [x] Unimplemented providers (ollama/anthropic/gemini) skipped with warn log
- [x] `src/config.rs` — `Config.profiles: ProfilesConfig` field (serde default)
- [x] `src/runtime.rs` — `profile_registry`, `active_profile`, `model_chain`, `brain_disabled`
  - [x] `bootstrap()` accepts `no_brain: bool` — skips SQLite if true
  - [x] `provider_label` from first available chain entry (not hardcoded)
  - [x] Boot log includes profile + chain summary
- [x] `src/app.rs` — uses `completion_with_fallback`, new profile/chain/brain_disabled fields
  - [x] `/status` shows Profile + Fallback
  - [x] `provider_label` updated with actual model used per response
- [x] `src/headless.rs` — banner + /status shows Profile + Fallback
  - [x] Uses `completion_with_fallback`, updates `rt.provider_label` per response
- [x] `src/cli.rs` — `--no-brain` global flag; `goat models` subcommand
  - [x] `goat models` prints providers (✓/✗), profiles, chains; no secrets
- [x] `src/main.rs` — `mod models`, `mod provider` registered; `no_brain` passed to bootstrap
- [x] `src/paths.rs` — doctor: `Default profile` + `Model profiles` checks (2 new)
- [x] 47 total tests pass (was 26)
- [x] Version bumped to 0.4.0

**NOT implemented in Phase 1.5 (deferred):**
- `--profile <name>` flag to select profile at launch
- `/profile <name>` slash command for per-session switching
- Anthropic, Gemini, Ollama, OpenRouter — planned, no fake implementations
- `anyhow` in `brain.rs`
- Configurable max-retry / timeout in `goat.toml`

### Exit Criteria: ✅ ALL PASSED
- `cargo check` passes ✅
- `cargo test` 47/47 ✅
- `goat models` shows providers, profiles, status ✅
- `goat doctor` shows Default profile + Model profiles ✅
- `goat --headless /status` shows Provider + Profile + Fallback ✅
- `goat --headless --no-brain` runs ephemeral (brain disabled) ✅
- Fallback chain implemented and used for all LLM calls ✅
- No fake Anthropic/Gemini/Ollama provider ✅
- Version 0.4.0 ✅

---


## Phase 2: GOAT Brain Foundation (Memory) ✅ COMPLETE

**Goal:** Curated memory, recall, and safe context injection.

### Tasks:

#### 2.0 Curated Memory & Safe Context Injection ✅ COMPLETE
- [x] Memory manager implemented (`src/memory.rs`) for `USER.md` and `MEMORY.md`
- [x] Context injection into LLM system prompt respecting character budgets
- [x] Secret protection heuristics to reject API keys and passwords
- [x] CLI commands (`goat memory ...`, `goat recall`)
- [x] Slash commands (`/memory`, `/recall`)
- [x] Doctor checks for memory subsystem status and budgets
- [ ] New session command
- [ ] Session rename command
- [ ] Session delete command

#### 2.1 GOAT Skills System ✅ COMPLETE
- [x] Local-first skills directory (`~/.config/goat/skills/`)
- [x] SKILL.md parsing (Name, Description, Triggers, Tools Needed, Content)
- [x] Skill Discovery & Indexer (ignores malformed skills safely)
- [x] CLI commands (`goat skills [list|show|path|create|validate|search]`)
- [x] Slash commands (`/skills`, `/skill <name>`, `/skill search <query>`, `/skill create <name>`, `/skill path`)
- [x] Context Injection with progressive disclosure
- [x] Security/Validation (blocklists for `rm -rf`, `sk-`, `password=`, `sudo`)
- [x] `save-skill` command (creates placeholder template)
- [x] Skills status added to `goat doctor` and `/status`

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

## Phase 4.5: Dashboard Agent Chat + Async Coding Jobs (✅ COMPLETED)
**Goal:** Turn the dashboard chat from a foundation into a real visual coding companion.

*   **[x] Async Chat Endpoints:** Switch `POST /v1/chat` to a background job model, returning a `job_id` instead of blocking.
*   **[x] Async Job Processing:** Daemon spawns `tokio::spawn` tasks to run `LlmRouter` operations in the background.
*   **[x] Safe Tool Integration:** When the agent operates in `Act` mode via dashboard chat, tools trigger `approval_required` status rather than blindly applying.
*   **[x] Event Streaming for UI:** Use SSE to broadcast `job_started`, `chat_message`, and `job_completed` to the UI for live updates.
*   **[x] UI Completion:** Build out `apps/dashboard/src/app/chat/page.tsx` with a Mode Selector (Chat/Plan/Act) and live job tracking.

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

---

## Phase 2.4: UI/UX Architecture Review + TUI Polish ✅ COMPLETED

**Goal:** Audit current TUI weaknesses, design multi-frontend architecture, polish Ratatui TUI.

### Completed:
- [x] `docs/GOAT_UI_UX_AUDIT.md` — 20-category audit vs OpenCode, Claude Code, Hermes, Antigravity, Cursor/Windsurf, Cline, Codex CLI, Gemini CLI, Aider, JCode
- [x] `docs/GOAT_MULTI_FRONTEND_ARCHITECTURE.md` — TUI + Headless + Daemon + Web + Desktop + Voice architecture plan with tech stack per surface
- [x] `docs/GOAT_UI_DESIGN_SYSTEM.md` — Comprehensive design system: palette (20+ RGB colors), typography, spacing, ASCII wireframes
- [x] `src/ui.rs` — Polished: GOAT_VERSION constant (env!), 20+ tag colors (MEMORY, SKILL, PROJECT, REPO-MAP, DEV, PATCH, RESEARCH, etc.), diff +/- green/red, active skill in header, contextual input placeholder, wider approval overlay (86 cols), CRITICAL→red border in overlay
- [x] `src/app.rs` — Input history (history_up/history_down/commit_to_history), ↑=history nav, updated startup splash with correct version, grouped /help, /ui command, /repo-map TUI slash command, /check /test /lint /format /patch in TUI, friendly "unknown command" error
- [x] `src/main.rs` — ↑ key = history nav (when input non-empty), ↓ = history forward, Ctrl+L = /clear

### Exit Criteria:
- `cargo check` passes ✅
- `cargo test` passes with 76/76 ✅
- `cargo fmt` passes ✅
- Header shows correct version (v0.7.0) ✅
- Active skill shown in header ✅
- /help grouped and complete ✅
- Input history navigation (↑/↓) ✅
- /ui command works ✅
- /repo-map command works in TUI ✅
- Diff +/- lines colored green/red ✅
- All major log tags have distinct colors ✅
- 3 major docs created ✅

---

## Phase 3.0: Advanced Ratatui TUI ✅ COMPLETED

**Goal:** Multi-pane layout, command palette, sidebar, diff pane.

### Completed:
- [x] Multi-pane layout: chat + context panel + views sidebar
- [x] Command palette (/palette)
- [x] View system with commands (`/view <name>`)
- [x] Dedicated diff viewer placeholder in Patches view
- [x] Single-pane fallback for narrow terminals
- [x] Tasks, Patches, Repo, Skills, Subagents views
- [x] Global keyboard shortcuts (`Ctrl+1-9`, `Ctrl+P`)

## Phase 3.1: Unified Command System ✅ COMPLETED
**Goal:** Unified slash commands and command registry
- [x] Refactored `handle_slash_command` logic
- [x] CommandRegistry holding all `CommandMetadata`
- [x] Tab autocompletion logic
- [x] Slash command suggestion popup overlay

## Phase 3.2: Premium Focus Layout ✅ COMPLETED
**Goal:** OpenCode-style clean and distraction-free UI
- [x] Implement LayoutMode (Focus, Dashboard, Compact)
- [x] `/layout` toggle command
- [x] Centralized, wide input composer and chat area
- [x] Premium empty state design

## Phase 3.3: Interactive Repo/File Tree + Patch/Diff UX ✅ COMPLETED
**Goal:** In-terminal repo browsing and git/diff inspection
- [x] `RepoMap` tree formatting (`to_tree_lines`)
- [x] `/repo` command to scan and browse the workspace
- [x] `/open` / `/preview` for safe, redacted file viewing
- [x] Patch view diff rendering
- [x] `/changes` and `/git-status` implementation
- [x] `/diff` shortcut for both patch diffs and local git diffs

---

## Phase 4.0: Daemon + Local API Foundation ✅ COMPLETE

**Goal:** Local HTTP daemon for multi-frontend support and background jobs.

### Completed in 0.13.0:
- [x] axum-based daemon (`goat daemon`)
- [x] Secure Bearer token auto-generation
- [x] REST API endpoints (`/v1/status`, `/v1/jobs`, `/v1/hooks`, `/v1/schedule`)
- [x] CLI commands: `goat daemon start/status/doctor`
- [x] TUI/Headless awareness (overlapping scheduler warnings)
- [x] Local-only bind (127.0.0.1)

---

## Phase 4.1: Web Dashboard Foundation ✅ COMPLETE

**Goal:** Next.js + React local dashboard.

### Completed:
- [x] Next.js 15 + React 19 + TypeScript + Tailwind
- [x] Read-only REST API client for GOAT daemon
- [x] Overview page with system health and status
- [x] Background Jobs, Schedule, and Hooks pages
- [x] MCP server and tool registry view
- [x] Recent logs view
- [x] API token security implementation in frontend

---

## Phase 4.2: Dashboard Approval Queue + Events ✅ COMPLETE

**Goal:** Securely handle dangerous commands via dashboard and stream events.

### Completed:
- [x] Server-Sent Events (SSE) Bus in Daemon
- [x] `/v1/events/stream` for real-time daemon logs and actions
- [x] `ApprovalQueue` integration (bridges HTTP UI with synchronous ApprovalGate)
- [x] `/approvals` dashboard page

---

## Phase 4.3: Dashboard Chat + Repo Explorer + Diffs ✅ COMPLETE

**Goal:** Provide chat context, repo preview, and workspace diff UI on the web dashboard.

### Completed:
- [x] `/chat` UI for queuing chat events
- [x] `RepoMap` API exposed at `/v1/repo/tree` and `/v1/repo/file`
- [x] Interactive Dashboard Repo Explorer with Secret Protection
- [x] Context API endpoints exposed to UI
- [x] Real-time Git diff viewer (`/diffs`) on Dashboard

---

## Phase 5.0: Tauri Desktop App (Planned)

**Goal:** Native desktop app wrapping GOAT core + web UI.

### Planned:
- [ ] Tauri 2.x shell (Rust core reused)
- [ ] System tray integration
- [ ] Native notifications for approvals
- [ ] OS file picker

---

## Phase 6.0: Voice Companion (Planned, opt-in only)

**Goal:** Optional voice interface — fully opt-in, never listens without permission.

### Planned:
- [ ] voice.enabled = false default
- [ ] WebRTC mic capture with explicit permission
- [ ] Whisper.cpp local STT (or Whisper API)
- [ ] Optional TTS output
- [ ] Visual listening indicator always visible

---

## Phase 4.6: Dashboard UX Polish + Theme System (✅ COMPLETED)
**Goal:** Make the dashboard feel like a premium developer tool.

*   **[x] Dashboard Design System:** Implementation of high-end Tailwind primitive components (cards, buttons, input, badges).
*   **[x] Theme System:** `goat-dark`, `minimal-dark`, and `high-contrast` CSS variable themes with `ThemeProvider` and Settings toggle.
*   **[x] Code/Diff Viewer Graceful Fallback:** Custom line-numbered syntax viewer and git diff viewer (since Monaco was restricted by offline environment).
*   **[x] Global Command Palette:** `cmdk` based `Ctrl+K` modal for navigating across dashboard workspaces securely.

---

## Phase 4.7: Product Hardening & Feature Parity Review (✅ COMPLETED)
**Goal:** Ensure GOAT is robust, user-friendly, and honestly documented before wrapping it inside a desktop app.

*   **[x] Product Health Audit:** Comprehensive breakdown of working vs partial features.
*   **[x] TUI & Dashboard UX Audits:** Thorough identification of UI pain points.
*   **[x] Feature Parity Review:** Benchmark against OpenCode, Cursor, Claude Code, and others.
*   **[x] Setup/Onboarding Polish:** Established robust documentation for installation and troubleshooting.
*   **[x] UX Small Fixes:** Minor dashboard component improvements and TUI padding/logic refinements applied.
## Phase 5.16: Prime Agent Architecture + Specialist Agent Layer (✅ COMPLETED)
* Unified agent architecture (`src/agents/mod.rs`).
* Prime Agents and Specialist Agents definitions.
* Report system (`src/reports.rs`).
* Dashboard `/agents` and `/reports` integration.
## Phase 5.17: Cofounder Agent Phase 1 (✅ COMPLETED)
* Implemented `CofounderAgent` in `src/agents/cofounder.rs`.
* Idea intake and transparent scorecard (1-5 scoring on 10 dimensions).
* Validation Plan generation.
* MVP Scoping (aggressive reduction).
* Competitor scan and outreach draft capabilities.
* Report generation for founder insights.
* Added `GET /v1/cofounder/*` and `POST /v1/cofounder/*` endpoints.
* Added Dashboard `/cofounder` route and `/agents` integration.

## Phase 6.8: Safe Extension Runtime & Plugin Marketplace (✅ COMPLETED)
* Implemented `ExtensionRegistry` and manifesting systems in `src/extensions.rs`.
* Created local storage configurations (`enabled.json`, `trust.json`) to enforce explicit permissions.
* Added `goat extensions` CLI management command block.
* Implemented `/v1/extensions` API suite.
* Designed and built `/extensions` Next.js dashboard workspace page.

## Phase 6.9: Real Browser + Desktop Automation Workflows (✅ COMPLETED)
* Implemented `BrowserWorkflow` and step execution in `src/browser_workflows.rs`.
* Added predefined workflows (`ui-qa`, `landing-review`, `dashboard-qa`, `web-health-check`).
* Supported CLI commands under `goat browser` command group.
* Added `/v1/browser/workflows` endpoints.
* Overhauled `/browser` Next.js dashboard page with active steps visual trace.
* Defined safety policies and desktop boundaries (`docs/GOAT_BROWSER_SAFETY.md`, `docs/GOAT_DESKTOP_AUTOMATION_BOUNDARY.md`).
