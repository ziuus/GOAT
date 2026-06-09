# CHANGELOG

All notable changes to GOAT are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.11.0] — Phase 3.3: Interactive Repo/File Tree + Patch/Diff UX (2026-06-09)

### Added — Phase 3.3: Interactive Repo/File Tree + Patch/Diff UX

#### Interactive Repo View (`/repo`)
- **`/repo` command** replaces `/repo-map`. Renders a beautiful visual tree of the project structure in the `ActiveView::RepoMap` UI layout.
- Added `/repo refresh` (force rescan), `/repo summary` (log compact summary), and `/repo context`.
- Cached `repo_map` in `App` state to allow rendering the visual tree dynamically.

#### Safe File Preview (`/open` | `/preview`)
- Safely read text files from disk and display their contents in the chat log.
- **Security Check**: Auto-detects and blocks previews for `.env`, `id_rsa`, `key`, `credentials`, etc.
- **Redaction Check**: Pipes the file content through the `redact_secrets` module.
- Auto-truncates large files to prevent UI flooding.

#### Enhanced Patch UX & Diff Viewer (`/diff`)
- Improved the `ActiveView::Patches` rendering by including file paths, colorized patch status, and a small snippet of the diff inline in the UI.
- **`/diff [patch-id]`**: Displays the full unified diff of a pending patch directly in the chat with correct syntax coloring (+/- lines).
- Also functions as a local git diff shortcut when no arguments are provided.

#### Git Status View (`/changes` | `/git-status`)
- Fast command to view the current branch, dirty state, and list of changed files based on `git status -s`.

## [0.10.0] — Phase 3.2: Premium Focus Layout (2026-06-09)

### Added — Phase 3.2: Premium Focus Layout + OpenCode-style Clean TUI

#### Layout Modes (`/layout`)
- **Focus layout** (default): Clean centered interface, no always-visible panels. Chat and views fill the screen cleanly. Input composer centered on wide terminals. The default experience now resembles OpenCode / Claude Code.
- **Dashboard layout**: Restores the 3-pane sidebar + center + context layout. Sidebar and context panel can be individually toggled with Ctrl+B / Ctrl+R.
- **Compact layout**: Chat + input only, perfect for narrow terminals.
- `/layout` — show current layout and options.
- `/layout focus` / `dashboard` / `compact` — switch instantly.
- `Ctrl+F` → focus layout. `Ctrl+D` → dashboard layout.
- `Ctrl+B` → toggle sidebar. `Ctrl+R` → toggle context panel.
- `layout_mode`, `sidebar_visible`, `context_visible` added to `App` struct.

#### Premium Empty State
- When Focus layout is active with no conversation: shows a centered, branded landing screen.
- Displays: GOAT logo + version, tagline, status row (profile/model/mode/tools/subagents/brain), keyboard hints (/status, /help, /agents, Tab, Ctrl+P, Ctrl+D).
- No startup log noise in the main view — logs moved to `/logs` / `/view logs`.

#### Logs View (`/logs`)
- `/logs` / `/view logs` — dedicated view for all system/tool log output.
- `/logs clear` — clear system logs while preserving conversation.
- `ActiveView::Logs` added to the view system.
- Chat view stays clean; system/tool noise is separated.

#### Agent & Skill Selector Modal (`/agents`)
- `/agents` / `/agent-selector` / `/subagent-selector` open a polished centered modal.
- Shows all **Internal Subagents** with `/ask-agent <name> <task>` instructions.
- Shows **External Agents** status and delegate command (or notes if disabled).
- Shows **Active Skill** status and `/skill deactivate` instructions.
- `ActiveView::AgentSelector` added.

#### Theme Command (`/theme`)
- `/theme` — shows current theme: `goat-dark` (active) + planned future themes.
- Planned: `minimal`, `glass`, `high-contrast` (Phase 3.3).

#### Polished View Empty States
All non-Chat views now have intentional, instructional empty states instead of raw placeholder text:
- **Tasks**: "No active task. Run /code <task>." with full command hints.
- **Repo Map**: "Run /repo-map refresh." with explanation.
- **Patches**: "No pending patches. Proposed edits will appear here." with /patch commands.
- **Memory**: Brain status indicator, USER.md/MEMORY.md info, all memory commands.
- **Skills**: Active skill display, /skills, /skill, /save-skill commands.
- **Subagents**: List of registered subagents with action hints.
- **External Agents**: Execution status, agent list or "disabled" notice.
- **Help**: Quick reference with keyboard shortcuts and key commands.
- **Logs**: Full log view with /logs clear support.
- **Agent Selector**: Unified modal for subagents + external agents + skills.

#### Improved Input Composer
- Context-aware title: shows `mode | profile | Message`.
- Premium placeholder that changes by state (thinking, running tool, error, idle).
- Centered on wide terminals in Focus layout (max-width 120).
- Active/typing state highlights border more brightly.
- No changes to approval state rendering.

#### Improved Command Palette Modal
- Now renders as a **centered floating modal** over the workspace (not a view).
- Registry-powered grouped command listing with color-coded categories.
- Command names highlighted in blue, descriptions in dim, risk markers in amber.
- Planned commands shown in a subdued style.

#### Improved Slash Suggestion Popup
- Risk markers: `⛔` for Critical, `⚠` for High risk commands.
- Planned commands shown with 🔮 badge and dimmed styling.
- Selected row: bright green background with ▶ indicator.
- Aligns with centered input in Focus layout on wide terminals.
- Cleaner color split: command name vs description.

#### New Keyboard Shortcuts
- `Ctrl+F` — switch to Focus layout
- `Ctrl+D` — switch to Dashboard layout
- `Ctrl+B` — toggle sidebar visibility
- `Ctrl+R` — toggle context panel visibility

#### CommandRegistry Updates
- Added: `/layout`, `/logs`, `/agents` (alias: `/agent-selector`, `/subagent-selector`), `/theme`.
- Total: 53 implemented commands + 23 planned.

### Changed
- `render_workspace` replaced by layout-mode dispatch (`render_focus_layout`, `render_dashboard_layout`, `render_compact_layout`).
- Header now shows layout mode badge (`📐 focus` etc.).
- Dashboard sidebar uses icon-prefixed view items for better scannability.
- Dashboard context panel shows active skill and provider info.
- `ActiveView::Chat` in focus mode with empty conversation → shows empty state, not the log.
- `ActiveView::CommandPalette` and `ActiveView::AgentSelector` render as modals, not full views.

## [0.9.0] — Phase 3.1: Unified Command System (2026-06-09)

### Added — Phase 3.1: Unified Command System + Slash Recommendation UX

- **`CommandRegistry` module** (`src/command_registry.rs`): Central registry for all GOAT slash commands.
  - `CommandMetadata` struct: name, aliases, category, description, usage, examples, shortcut, surface, risk, status.
  - `CommandCategory` with emoji labels: General, Sessions, Models, Project, Repo, Coding, Patches, Memory, Skills, Tools, MCP, Subagents, ExternalAgents, UI, Logs, System, Future.
  - `CommandStatus` enum: Working ✅, Partial ⚡, Planned 🔮, Disabled ❌.
  - `CommandRisk` enum: None, Low, Medium, High, Critical — enforces `requires_approval` on high-risk commands.
  - `CommandSurface`: tracks which commands work in TUI vs headless vs CLI.
  - **49 implemented commands** + **23 planned future commands** registered with full metadata.
  - Prefix matching (case-insensitive) for autocomplete.
  - Full-text search by name, description, category, or alias.
  - Grouped output formatter for `/help` and `/commands`.
  - Palette formatter for `/palette [query]`.

- **Slash Command Suggestion Popup** (TUI):
  - Appears automatically above the input composer when input starts with `/`.
  - Shows up to 12 filtered suggestions matching the current prefix.
  - `↑`/`↓` to navigate; `Tab` to complete; `Esc` to dismiss.
  - Selected entry highlighted in bright green; others show command + description.
  - Popup renders cleanly above input line using Ratatui `Clear` widget.

- **Tab Completion**: Pressing `Tab` completes to the selected (or first) suggestion's name + trailing space.

- **Registry-Powered `/help`**: Now uses `CommandRegistry::format_help()` — grouped by category, status-labeled, complete.

- **Registry-Powered `/palette`**: Now uses `CommandRegistry::format_palette()` — includes risk warnings, grouped by category, optional search filter.

- **New `/commands` command** (alias `/cmd`):
  - `/commands` — list all working/partial commands grouped by category.
  - `/commands all` — list all commands including planned.
  - `/commands planned` — list only planned commands.
  - `/commands search <q>` — fuzzy search across name/description/category.

- **Headless parity**: `/help` and `/commands` in `headless.rs` use the same `CommandRegistry`.

- **21 tests** in `command_registry::tests` covering prefix matching, search, grouping, completion, help/palette formatting, security invariants.

### Changed

- `/help` output replaced with registry-driven grouped format (no content loss — all commands still listed).
- `/palette` output now uses registry groups with risk labels.
- `App::input` changes now call `update_suggestions()` automatically.
- `↑`/`↓` keys route to suggestion navigation when popup is open, otherwise fall through to history/scroll.
- `Esc` dismisses suggestions without clearing input (second Esc clears input as before).

## [0.8.0] — Phase 3.0: Advanced Ratatui TUI (2026-06-09)

### Added — Phase 3.0: Advanced Ratatui TUI

- **Multi-pane Layout:** Refactored TUI to support left sidebar (views/shortcuts), center pane (chat/workspace), and right context panel (workflow/tasks).
- **View System:** Added `/view <tab>` commands (`chat`, `tasks`, `repo`, `patches`, `tools`, `memory`, `skills`, `subagents`, `external`, `help`).
- **Command Palette:** Added `/command` and `/palette` commands that display a categorized shortcut list.
- **Improved Context Displays:** Refactored displays for patches, tasks, and agents in their respective views instead of spamming chat history.
- **Keyboard Shortcuts:** Added `Ctrl+1` through `Ctrl+9` for view switching and `Ctrl+P` for Command Palette.
- **Graceful Degradation:** Automatic single-pane fallback for narrow terminals.
- **Approval Overlay UX:** High/Critical risk commands are now clearly highlighted with red borders inside the approval overlay.

## [Unreleased] — Phase 2: GOAT Brain Foundation

### Added — Phase 2.7: Internal Subagent Framework

- **Subagent Registry:** Defined 10 internal subagents (`planner`, `coder`, `reviewer`, `tester`, `debugger`, `documenter`, `researcher`, `security-auditor`, `ui-designer`, `refactorer`) in `src/subagents.rs` with explicit profiles, risk levels, and allowed tools.
- **Context Isolation:** Subagents receive a limited context budget (task, active skill, memory snapshot, and truncated repo map) to conserve tokens and prevent hallucination.
- **Subagent Audit Log:** Added `~/.local/share/goat/subagent-audit.log` to track subagent executions and securely log redacted output.
- **Commands:**
  - `goat subagents [list|show|audit]` / `/subagents`
  - `goat ask-agent <name> "<task>"` / `/ask-agent <name> <task>`
  - Helper shortcuts: `/review`, `/debug`, `/test-plan`
- **Doctor/Status Integration:** `goat doctor` now checks the subagent registry and audit log. `/status` reports available subagents.
- **Documentation:** Added `docs/GOAT_SUBAGENTS.md` detailing the internal subagent architecture.

### Added — Phase 2.4: UI/UX Architecture Review + TUI Polish (2026-06-09)

**Version bump: 0.6.0 → 0.7.0**

- **UI/UX Audit (`docs/GOAT_UI_UX_AUDIT.md`):** 20-category audit of current TUI vs OpenCode, Claude Code, Hermes, Antigravity, Cursor, Cline, Codex CLI, Gemini CLI, Aider, JCode. Each issue tagged with severity and fix path (Ratatui now / Phase 3.0 / Phase 4.x).
- **Multi-Frontend Architecture (`docs/GOAT_MULTI_FRONTEND_ARCHITECTURE.md`):** Full plan for GOAT's UI surface strategy: Ratatui TUI, Headless, Daemon+API, Next.js Web Dashboard, Tauri Desktop, Voice Companion. Tech stack, security guarantees, and rationale per surface.
- **Design System (`docs/GOAT_UI_DESIGN_SYSTEM.md`):** Complete visual language for all GOAT frontends: 20+ RGB color tokens, typography (Inter/JetBrains Mono for web), spacing, ASCII/wireframe sketches of TUI/web/voice layouts, CSS glassmorphism tokens.
- **TUI Polish (`src/ui.rs`):**
  - `GOAT_VERSION` constant using `env!("CARGO_PKG_VERSION")` — header now shows actual version.
  - Extended color palette to 20+ tag-specific colors: `[MEMORY]`, `[SKILL]`/`[SKILLS]`, `[PROJECT]`, `[REPO-MAP]`, `[DEV]`, `[PATCH]`, `[RESEARCH]`, `[UI]`, `[RECALL]`, plus `[TOOLS]` distinction.
  - Diff line colorization: `+ lines` green (COLOR_DIFF_ADD), `- lines` red (COLOR_DIFF_REMOVE), `@@ hunk headers` blue.
  - Active skill shown in header (truncated to 12 chars): `│ 🎯 skill-name`.
  - `GOAT_VERSION` replaces hardcoded `v0.1` in header.
  - Contextual input placeholder changes based on `AppStatus` (Thinking, Running, Error).
  - Input border brightens when user is typing.
  - Approval overlay width expanded to 86 cols.
  - Approval overlay shows CRITICAL→red border, and applies diff colors to `+`/`-`/`@@` lines in preview.
  - Log panel scroll hint updated to include PgUp/PgDn info.
- **Input History (`src/app.rs`):**
  - `App.input_history: Vec<String>` and `App.history_idx: Option<usize>` fields.
  - `history_up()`, `history_down()`, `commit_to_history()` methods.
  - Cap at 200 entries; no duplicate consecutive entries.
- **Help & Commands (`src/app.rs`):**
  - `/help` completely rewritten: grouped into General, Sessions, Project & Dev, Memory & Skills, Infrastructure, Keyboard, Approval — all Phase 2.x slash commands included.
  - `/ui` command: shows current TUI info and lists all planned future UI surfaces (3.0/4.0/4.1/5.0/6.0).
  - `/clear` enhanced: shows current version after clearing.
  - `/repo-map` and `/repo-map refresh` work in TUI slash commands (via `RepoMapScanner`).
  - `/check`, `/test`, `/lint`, `/format` in TUI: detect project command and inform user to use CLI for execution.
  - `/patch`, `/patch apply`, `/patch discard` in TUI.
  - Unknown slash commands now show friendly "type /help for a full list" message.
  - Startup splash updated: correct version string, grouped hint lines for quick start.
- **Key Bindings (`src/main.rs`):**
  - `↑` key: if input non-empty or browsing history → `history_up()`; if input empty → scroll log up.
  - `↓` key: if browsing history → `history_down()`; else → scroll log down.
  - `Enter`: calls `commit_to_history()` before clearing input.
  - `Ctrl+L`: clears log (same as `/clear`).


**Version bump: 0.5.0 → 0.6.0**

- **Repo Map (`src/repo_map.rs`):** New module implementing safe, lightweight repository awareness:
  - Scans project root for stack, source dirs, key files, and source file metadata.
  - Symbol extraction via regex for Rust (fn/struct/enum/trait/mod), JS/TS (function/class/export), Python (def/class), Go (func/type).
  - Git status awareness: current branch, dirty tree, changed file count (safe subprocess call, no libgit2).
  - Ignored dirs: `node_modules`, `target`, `dist`, `.git`, `venv`, `__pycache__`, and more.
  - Secret file detection: never reads `.env`, `id_rsa`, `*.key`, `*.pem`, etc.
  - Budget-capped compact string output (default 4000 chars) for LLM context injection.
- **Diff-before-write:** `write_file` tool now generates a unified diff preview before ApprovalGate:
  - Shows `+N lines added / -N removed` plus a truncated unified diff for existing files.
  - Shows line count for new files.
  - Detects and redacts secret-like values in content previews before showing to user.
  - Applies in both TUI and headless mode.
- **RepoMapConfig:** Added `[repo_map]` config section to `goat.toml`:
  - `enabled`, `inject`, `max_chars` (default 4000), `include_symbols` (default true).
- **CLI Commands:** Added `goat repo-map [show|refresh]`, `goat check`, `goat test [args...]`, `goat lint`, `goat format`, `goat patch [show|apply|discard]`.
- **Slash Commands (TUI + headless):** Added `/repo-map`, `/repo-map refresh`, `/check`, `/test`, `/lint`, `/format`, `/patch`, `/patch apply`, `/patch discard`.
- **Command Detection (`ProjectCommands::detect`):** Auto-detects check/test/lint/format commands for:
  - Rust: `cargo check/test/clippy/fmt`
  - Node: detects runner (npm/pnpm/yarn) + package.json scripts
  - Python: `pytest`, `ruff check/format`
  - Go: `go build/test`, `gofmt`
  - Makefile: `make`, `make test`
- **Dev Command Runner:** `/check`, `/test`, `/lint`, `/format` slash commands route through ApprovalGate before execution.
- **Git Awareness:** `/status` now shows git branch, dirty tree status, and changed file count when project is scanned. Works without libgit2 via safe subprocess.
- **Agent Coding Loop Design:** Architecture documented in code and docs for future Plan/Act mode (Phase 2.4+).
- **Security:** All file scans skip secret-named files; diff previews redact secret-like values; no files injected automatically; all shell commands require ApprovalGate.
- **Tests:** Added 13 new tests covering repo map scanning, symbol extraction (Rust/Python/JS), ignore rules, secret detection, diff preview, command detection.

### Added — Phase 2.1: GOAT Skills System (2026-06-09)

**Version bump: 0.7.0 → 0.8.0**

- **Skills Directory:** Created local-first skills directory at `~/.config/goat/skills/`.
- **SKILL.md Format:** Implemented Markdown parser for GOAT reusable skills, supporting Name, Description, Triggers, Tools Needed, and Content.
- **Skill Discovery & Index:** Implemented an indexer with progressive disclosure — only skill summaries (names, descriptions, triggers) are injected to save context budget, unless specifically activated.
- **Context Injection:** `SkillManager::build_context` handles injecting the skill index and the active skill's content directly into the system prompt.
- **CLI Commands:** Added `goat skills [list|show|path|create|validate|search]`.
- **Slash Commands:** Added `/skills`, `/skill <name>`, `/skill search <query>`, `/skill create <name>`, `/skill path`, and `/save-skill <name>`.
- **Security & Validation:** Implemented naive pattern detection during parsing (checking for `rm -rf`, `sk-`, `password=`, `sudo`, etc.). Skills cannot bypass ApprovalGate.
- **Doctor Check:** Added Skills System status to `goat doctor`.

### Added — Phase 2.0: Curated Memory & Safe Context Injection (2026-06-09)

**Version bump: 0.6.0 → 0.7.0**

- **Curated Memory System:** Introduced `src/memory.rs` and `MemoryManager` to handle local `USER.md` (preferences) and `MEMORY.md` (notes).
- **Safe Context Injection:** Automatically injects memory notes and user preferences into the LLM system prompt. Configurable via `[memory]` section in `goat.toml` (limits, enabled status).
- **Secret Protection Heuristics:** Memory files have built-in blocklists for detecting and rejecting common secrets (API keys, passwords, AWS keys) before saving.
- **Budgeting System:** Soft character limits (configurable, defaults: user 1500, memory 4000 chars) ensure context length stays manageable.
- **Memory CLI Commands:** Added `goat memory [status|show|path|add-user|add-note]` and `goat recall "<query>"`.
- **Slash Commands:** Added `/memory` and `/recall` to both TUI and headless modes.
- **Doctor Check:** `goat doctor` now displays the status and budget warnings for the memory subsystem.

---

## [Unreleased] — Phase 1: Minimal Working Core

### Added — Phase 1.7: Project Awareness & Deterministic Titles (2026-06-09)

**Version bump: 0.5.0 → 0.6.0**

- **Project Awareness:** Added `src/project.rs` with `ProjectScanner` to safely index repo metadata (git status, stack, package files, commands). Skips ignored dirs like `node_modules` or `target`.
- **Project Database Schema:** Added `projects` table to the SQLite memory (via `Brain`).
- **Project CLI & Slash Commands:** Added `goat project scan` / `/project scan` and `goat project status` / `/status` additions to display project context.
- **Deterministic Session Titles:** Sessions now automatically generate a human-readable title based on the user's first message, updating in the `sessions` database table.
- **Documentation Updates:** Marked Voice Companion / Jarvis Mode clearly as planned/future-only to prevent false feature claims.

### Added — Phase 1.6: Profile Selection, OpenRouter/Ollama, Retry Config, Session Control (2026-06-09)

**Version bump: 0.4.0 → 0.5.0**

**`src/config.rs` — extended**
- New `LlmConfig` struct: `max_retries`, `timeout_secs`, `fallback_on_rate_limit`, `fallback_on_network`, `fallback_on_server_error`
  - Configurable via `[llm]` section in `goat.toml`
  - Sensible defaults: max_retries=2, timeout_secs=60, all fallback flags=true
  - `.validate()` — returns user-readable warnings for bad values
  - `.effective_max_retries()` / `.effective_timeout_secs()` — clamped safe values
- New `ProviderCustomConfig`: `enabled`, `base_url`, `api_key_env`
  - Configurable via `[providers.openrouter]` / `[providers.ollama]` etc.
- `Config.providers: HashMap<String, ProviderCustomConfig>` field
- `Config.llm: LlmConfig` field (serde default — backward compatible)
- `Config.keys.openrouter_api_key` field
- `Config::provider_api_key(provider)` — unified key resolution (config → env var)
- `Config::provider_base_url(provider)` — unified base URL resolution
- `Config::provider_enabled(provider)` — per-provider enable flag
- 8 unit tests

**`src/llm.rs` — rewrite**
- `LlmRouter::from_config(config)` — primary constructor from full `Config`
  - Uses `LlmConfig` for timeout (reqwest client), retries, and fallback policy
  - Reads OpenRouter and Ollama keys/URLs from config
- `LlmRouter::new()` kept for backward compat
- New provider: **OpenRouter** (OpenAI-compatible API)
  - Adds required `HTTP-Referer` and `X-Title` headers
  - Key: `OPENROUTER_API_KEY` env var or `openrouter_api_key` in config
  - Base URL: `https://openrouter.ai/api/v1` (customizable)
- New provider: **Ollama** (local OpenAI-compatible endpoint)
  - No API key required — uses local server
  - Base URL: `http://localhost:11434` (customizable via `[providers.ollama] base_url`)
  - Graceful error if Ollama is not running
- `is_error_fallback_allowed()` — checks per-error-class `fallback_on_*` config flags
- `is_provider_implemented()` now includes `openrouter` and `ollama`
- Anthropic and Gemini: planned, still not implemented (clearly documented)

**`src/brain.rs` — rewrite (anyhow cleanup)**
- All errors now use `anyhow::Result` with `.context()` strings
- New `SessionRecord` struct: `id`, `title`, `created_at`, `updated_at`, `is_uuid()`
  - `is_uuid()` — detects v4 UUID format (8-4-4-4-12) vs legacy numeric IDs
- `Brain::get_session_records()` — returns `Vec<SessionRecord>` with full metadata
- `Brain::log_interaction()` — now updates `sessions.updated_at` on every message
- Migration 001: `sessions.updated_at` column
  - Added automatically if column is absent (safe for old DBs)
  - Uses `NULL` default (SQLite limitation), back-fills from `created_at`
  - Old sessions still list — no data loss
- 6 unit tests covering brain open, session CRUD, timestamps, uuid detection, migration

**`src/runtime.rs` — updates**
- `GoatRuntime::bootstrap()` accepts `profile_override: Option<String>`
  - Validates profile name early; warns and falls back to default if invalid
  - Clear boot log message: `[WARN] Profile '...' not found — falling back to default`
- Uses `LlmRouter::from_config(&config)` instead of `LlmRouter::new()`
- New method: `GoatRuntime::switch_profile(name)` — `Result<(), String>`
  - Validates profile name, updates `active_profile`, `model_chain`, `provider_label`
  - Used by headless `/profile <name>` command
- New method: `GoatRuntime::create_new_session()` — creates UUID session, clears history

**`src/cli.rs` — updates**
- New global flag: `--profile <name>` — select model profile at startup
  - Works with TUI: `goat --profile coding`
  - Works with headless: `goat --headless --profile cheap`
  - Invalid names: warns and falls back (does not hard-exit)
- New subcommand: `goat new-session`
  - Creates a new UUID session in the brain DB (if it exists)
  - Prints UUID to stdout (scriptable)
  - Does not destroy old sessions
- `goat sessions` — improved output
  - Shows: ID (short), Type (uuid/legacy), Created, Updated, Title
  - Tabular columnar format
- `goat models` — improved output
  - Shows OpenRouter, Ollama status (✓/✗) alongside OpenAI/Groq
  - Shows LLM retry/timeout config
  - Shows fallback status per profile entry
  - Legend, usage hint
- `goat doctor` — improved
  - Uses `Config::provider_api_key()` (consistent with runtime)
  - Shows OpenRouter key status
  - Shows Ollama config status
  - Shows Anthropic/Gemini as Planned
  - Shows LLM retry config (`max_retries`, `timeout_secs`, fallback flags)

**`src/app.rs` (TUI) — updates**
- New field: `profile_registry: ProfileRegistry` (from GoatRuntime)
- New slash commands:
  - `/profile` — show current profile, primary, fallback
  - `/profile <name>` — switch profile at runtime (only when READY, not during agent run)
  - `/profiles` — list all profiles with primary/fallback/active markers
  - `/new` — start a new session (clears history, creates UUID in brain)
- `/status` — now shows `Retries: N max / Xs timeout`
- `/sessions` — now uses `get_session_records()` with uuid/legacy/timestamp display
- `/help` — updated with all new commands
- Splash header updated: v0.4 → v0.4 with new commands listed

**`src/headless.rs` — updates**
- New slash commands: `/profile`, `/profiles`, `/new`
  - `/profile <name>` uses `GoatRuntime::switch_profile()`
  - `/new` uses `GoatRuntime::create_new_session()`
- `/status` — shows `Retries: N max / Xs timeout`
- `/sessions` — uses `get_session_records()`
- `/help` — updated with all new commands
- Banner slash command list updated

**`src/paths.rs` doctor — updates**
- `run_doctor()` now accepts `has_openrouter_key`, `ollama_enabled`, `llm_config` params
- New checks:
  - OpenRouter key: OK/INFO
  - Ollama (local): INFO (no key needed, configured or not)
  - Anthropic: INFO (planned, not implemented)
  - Gemini: INFO (planned, not implemented)
  - LLM retry config: OK/WARN (shows max_retries, timeout_secs, fallback flags)
- Total: 23 doctor checks (was 17)
- 5 unit tests updated/added for new params

### Not implemented in Phase 1.6 (deferred)
- Anthropic provider — API format too different for this phase (planned, no stub)
- Gemini provider — planned, no stub
- `/new` + `/profile` while agent is running: correctly denied with clear message
- Per-profile custom `max_retries`/`timeout_secs` overrides (global config only for now)
- Session title from first message (always "New Session" for now)

### Exit Criteria: ALL PASSED
- `cargo fmt` ✅
- `cargo check` — 0 errors, 19 dead_code warnings (public API surface) ✅
- `cargo test` — **63/63** ✅ (was 47, +16 new tests)
- `goat 0.5.0` ✅
- `goat --profile coding` — selects coding profile ✅
- `goat --headless --profile cheap /status` — shows cheap profile + fallback ✅
- Invalid profile falls back gracefully with warning ✅
- `goat new-session` — prints UUID, creates in DB ✅
- `goat sessions` — shows uuid/legacy, timestamps ✅
- `goat doctor` — 23 checks, OpenRouter/Ollama/Anthropic/Gemini/LLM config ✅
- `goat models` — providers (including OpenRouter/Ollama), retry config, profiles ✅
- `/profile`, `/profiles`, `/new` work in headless mode ✅
- OpenRouter implemented (real requests, no fake) ✅
- Ollama implemented (real requests, graceful if not running) ✅
- `brain.rs` fully ported to anyhow ✅
- Session timestamps (created_at, updated_at) ✅
- DB migration safe for old databases ✅

---

### Added — Phase 1.5: Provider Abstraction + Model Profiles + Fallback Chain (2026-06-08)

**Version bump: 0.3.0 → 0.4.0**

**New file: `src/provider.rs`**
- `ProviderError` — typed, classified error enum for all provider failures
  - Variants: `RateLimit`, `AuthFailed`, `BadRequest`, `ModelNotFound`, `ServerError`, `NetworkError`, `NotConfigured`, `UnknownProvider`, `ChainExhausted`, `Other`
  - `.is_recoverable()` — whether the fallback chain should try the next model
  - `.is_retryable()` — whether the same provider/model should be retried
  - `ProviderError::from_http(provider, model, status, body)` — classify HTTP status codes
- `ProviderStatus` — `Ready`, `NotConfigured`, `Planned` (for doctor/models display)
- `ProviderInfo` — summary struct for listing providers with their available models
- 10 unit tests covering all error classifications

**New file: `src/models.rs`**
- `ModelEntry` — a single `provider:model` entry in a chain (parsed from `"openai:gpt-4o-mini"`)
- `ModelChain` — ordered list of `ModelEntry` values; `primary_display()`, `fallback_display()`
- `ProfilesConfig` — TOML-deserializable config struct for `[profiles]` section
- `ProfileEntry` — per-profile TOML config (list of `chain` strings)
- `ProfileRegistry` — runtime model profile registry
  - `ProfileRegistry::from_config(config)` — merges user config with built-in defaults (user wins)
  - `ProfileRegistry::with_defaults()` — built-in defaults only (no user config)
  - `ProfileRegistry::resolve(name)` — resolves by name, falls back to default then "balanced"
  - `ProfileRegistry::default_chain()` — returns the default profile's chain
- **6 built-in profiles**: `balanced`, `cheap`, `powerful`, `coding`, `reasoning`, `local`
  - `balanced`: openai:gpt-4o-mini → groq:llama-3.3-70b-versatile
  - `cheap`: groq:llama-3.1-8b-instant → openai:gpt-4o-mini
  - `powerful`: openai:gpt-4o → groq:llama-3.3-70b-versatile
  - `coding`: openai:gpt-4o → groq:qwen-qwq-32b
  - `reasoning`: openai:o1-mini → groq:llama-3.3-70b-versatile
  - `local`: ollama:llama3 (planned — not implemented)
- 12 unit tests covering parsing, chain ops, registry resolution, user config override

**`src/llm.rs` — rewrite**
- `LlmRouter::completion()` now returns `Result<MessageContent, ProviderError>` (was `Box<dyn Error>`)
- `LlmRouter::completion_with_fallback(chain, messages, tools)` — new fallback chain runner
  - Iterates `ModelChain` entries in order
  - Skips unimplemented providers (`ollama`, `anthropic`, `gemini`, `openrouter`) with a log warning
  - Retries retryable errors (network, 5xx) up to `MAX_RETRIES=2` times with 500ms*attempt delay
  - Advances chain on recoverable errors (rate limit, server error, network after retries)
  - Stops immediately on non-recoverable errors (401 auth, 400 bad request, 404 model not found)
  - Returns `(MessageContent, String)` — response + actual provider:model label used
- `LlmRouter::is_provider_available(provider)` — checks if a provider has a key and is implemented
- `LlmRouter::is_provider_implemented(provider)` — static check for coded support
- `LlmRouter::provider_status_label(provider)` — human-readable status without secrets
- HTTP error classification via `ProviderError::from_http()` — replaces raw string errors
- Network errors: timeout vs connection refused now classified correctly

**`src/config.rs`**
- `Config` struct: new `profiles: ProfilesConfig` field with `#[serde(default)]`
  - Absent `[profiles]` in TOML → built-in defaults used automatically

**`src/runtime.rs`**
- `GoatRuntime` struct: new fields:
  - `profile_registry: ProfileRegistry` — registry of all profiles
  - `active_profile: String` — name of the active profile (e.g. `"balanced"`)
  - `model_chain: ModelChain` — active fallback chain
  - `brain_disabled: bool` — whether `--no-brain` was passed
- `GoatRuntime::bootstrap()`: new `no_brain: bool` parameter
  - Builds `ProfileRegistry::from_config()` at startup
  - Skips SQLite entirely if `no_brain=true` (ephemeral session)
  - `provider_label` now resolved from first *available* chain entry (not hardcoded if-else)
  - Boot log now includes profile/chain summary

**`src/app.rs`**
- `App` struct: new `active_profile: String`, `model_chain: ModelChain`, `brain_disabled: bool` fields
- `App::from_runtime()`: copies new fields from `GoatRuntime`
- `App::new()`: passes `no_brain=false` to `GoatRuntime::bootstrap()`
- `/status` command output updated:
  - `Provider :`, `Profile  :`, `Fallback :`  (provider + profile + fallback chain)
  - Brain shows `"disabled (--no-brain)"` when brain disabled
- LLM call changed from `completion()` → `completion_with_fallback(&self.model_chain, …)`
  - `provider_label` updated with actual model used after each response

**`src/headless.rs`**
- Banner now shows `Profile  :` and `Fallback :` lines
- `Brain :` shows `"disabled (--no-brain)"` when brain disabled
- `/status` output updated to match TUI output (Profile + Fallback)
- LLM call changed from `completion()` → `completion_with_fallback(&rt.model_chain, …)`
  - `rt.provider_label` updated with actual model used

**`src/cli.rs`**
- New global flag `--no-brain` — disables SQLite brain for ephemeral sessions
- New subcommand `goat models` — lists all providers and profiles:
  - Provider status for openai, groq, anthropic (planned), gemini (planned), ollama (planned), openrouter (planned)
  - Each profile: primary (✓/✗), fallback chain, ready status
  - Legend and config instructions at bottom

**`src/main.rs`**
- `mod models` and `mod provider` registered
- `GoatRuntime::bootstrap()` call updated: passes `cli.no_brain`

**`src/paths.rs`**
- Doctor: 2 new checks after Provider count:
  - `Default profile` — shows profile name, primary, and fallback; WARN if primary provider unavailable
  - `Model profiles` — shows count and list of built-in profiles; includes `Run: goat models` hint

### Not Implemented (Deferred to Phase 1.6)
- Anthropic, Gemini, Ollama, OpenRouter providers (planned — no fake implementations)
- `--profile <name>` flag to select a non-default profile at launch
- Per-session profile switching via `/profile <name>` slash command
- `anyhow` adoption in `brain.rs` (safe to defer — no user-facing changes)
- Custom max-retry and timeout configuration in `goat.toml`

---

### Added — Phase 1.4: Headless Mode + Runtime Separation (2026-06-08)

**Version bump: 0.2.0 → 0.3.0**

**New file: `src/runtime.rs`**
- `GoatRuntime` struct — shared, surface-agnostic agent state
  - Holds: `paths`, `config`, `startup_warnings`, `session_id`, `session_resumed`
  - Holds: `brain`, `llm_router`, `swarm_router`, `approval_gate`, `mcp_manager`
  - Holds: `history`, `provider_label`, `mcp_server_count`
- `GoatRuntime::bootstrap(config, paths, warnings)` — single unified bootstrap
  - Opens brain DB (XDG path), emits `[SYSTEM] Brain connected: ...` or WARN
  - Resumes most-recent session or creates fresh UUID session
  - Loads history for resumed session with message count
  - Initializes `LlmRouter`, `SwarmRouter`, `ApprovalGate`, `McpManager`
  - Returns `(GoatRuntime, boot_log)` where `boot_log` is a `Vec<String>` of startup messages
  - Used by BOTH TUI and headless mode — no duplication
- `format_approval_prompt()` — shared approval display helper

**New file: `src/headless.rs`**
- `headless::run(runtime)` — async headless agent loop
- Prints banner with provider, session ID (truncated for readability), brain path, mode
- Reads lines from stdin via `io::stdin().lock().lines()`
- Exits cleanly on EOF (Ctrl+D) with `[GOAT] EOF received — goodbye!`
- Handles slash commands: `/help`, `/status`, `/clear`, `/sessions`, `/tools`, `/exit`
- `run_agent_turn()` — full ReAct LLM loop identical to TUI (same tools, same providers)
  - Prints `[GOAT] Thinking…` on same line, clears with response
  - Saves to brain/session like TUI
  - Handles MCP tools
- `prompt_approval_stdin()` — blocking stdin approval prompt
  - Prints the full `ApprovalRequest::display_lines()` box
  - Reads `y`/`n`/`a`/`d` from stdin in a loop; re-prompts on invalid input
  - Denies on EOF (safe default) — never silently approves
  - Calls same `ApprovalGate::resolve()` as TUI
- MCP server shutdown on exit
- `#[allow]` on `SwarmRouter` import (not yet used in headless but planned)

**Modified: `src/cli.rs`**
- Added `--headless` boolean flag (global, works with all subcommands)
- Added `sessions` subcommand → `handle_sessions_command()`
  - Opens brain DB read-only
  - Lists all sessions: `id[:8]…  title`
  - Prints "No sessions found" if empty
  - Prints "No brain database found" if DB doesn't exist yet
- Updated `handle_subcommand()` → passes `cli.headless` to `run_doctor()`
- Improved `--help` long description: mode table with TUI / headless / subcommands
- Updated `Command::Doctor` docs with new checks listed

**Modified: `src/paths.rs`**
- `run_doctor()` signature: added `headless_ready: bool` 4th parameter
- New doctor checks:
  - `Provider count`: `N of 2 providers configured (OpenAI, Groq)`
  - `DB migration`: OK if no legacy DB, WARN if `./goat_brain.db` still exists
  - `Headless mode`: OK, shows "Running in headless mode" or "Available — run: goat --headless"
- ApprovalGate and Log directory checks preserved (restored after earlier edit)
- Updated test calls: `run_doctor(&paths, false, false, false)` (all 3 tests updated)

**Modified: `src/app.rs`**
- Added `use crate::runtime::GoatRuntime`
- New `App::from_runtime(rt: GoatRuntime, boot_log: Vec<String>) -> Self`
  - Takes pre-bootstrapped GoatRuntime — brain, session, history, LLM, gate already initialized
  - Renders TUI splash header, startup_warnings, then boot_log in TUI log panel
  - Consumes all runtime fields into App struct directly
- `App::new()` refactored to thin wrapper: calls `GoatRuntime::bootstrap()` then `from_runtime()`
  - Preserved for backward compatibility and tests
  - No bootstrap logic duplication — single source of truth in `runtime.rs`

**Modified: `src/main.rs`**
- Registered `mod headless;` and `mod runtime;`
- Calls `GoatRuntime::bootstrap()` to create shared runtime
- Routes to `headless::run(runtime)` if `--headless`, else `run_tui(runtime, boot_log)`
- `run_tui()` extracted as separate function: creates `App::from_runtime()`, then enters TUI loop
- TUI event loop unchanged: all Phase 1.2 UX preserved

**CLI commands (all working):**
```
goat --version        → goat 0.3.0
goat --headless       → starts headless stdin/stdout loop
goat --help           → full usage with mode table
goat --config <PATH>  → custom config (works with --headless)
goat --db <PATH>      → custom database (works with --headless)
goat config-path      → ~/.config/goat/goat.toml
goat data-path        → ~/.local/share/goat
goat db-path          → ~/.local/share/goat/goat.db
goat sessions         → list sessions from brain DB
goat doctor           → improved readiness report (6 new checks)
goat migrate-db       → copy legacy DB to XDG path
```

**Headless mode behavior:**
```
$ goat --headless
╔══════════════════════════════════════════════════╗
║  GOAT Headless v0.3.0                          ║
╚══════════════════════════════════════════════════╝
Provider : openai:gpt-4o-mini
Session  : 550e8400-e29b-...
Brain    : /home/user/.local/share/goat/goat.db
Mode     : new session / resumed session

> list files in /tmp

[AGENT] Using tool: bash
╔══════════════ APPROVAL REQUIRED ══════════════╗
  Tool   : bash
  Action : ls /tmp
  Risk   : MEDIUM
  ...
╚═══════════════════════════════════════════════╝

Approve? [y] yes  [n] no  [a] always allow  [d] always deny: y
[APPROVAL] ✓ Approved: bash
[TOOL] ...output...
[GOAT] The /tmp directory contains: ...

> ^D
[GOAT] EOF received — goodbye!
```

**Architecture after Phase 1.4:**
```
main()
 ├─ CLI parse (clap)
 ├─ GoatPaths::resolve()  → XDG paths
 ├─ Config::load_from()   → config + warnings
 ├─ handle_subcommand()   → doctor/sessions/paths/migrate (exits)
 ├─ GoatRuntime::bootstrap() → shared brain/session/LLM/gate
 │
 ├─ --headless → headless::run(runtime)  [stdin/stdout]
 └─ (default) → run_tui(runtime)         [ratatui TUI]
```

Future surfaces (web API, Tauri, daemon) follow same pattern:
  `GoatRuntime::bootstrap()` once → pass to surface-specific loop.

**Test results:**
- `cargo fmt`: clean
- `cargo check`: 0 errors, 14 dead_code warnings (public API — expected)
- `cargo test`: 26/26 pass
- `goat --version`: goat 0.3.0
- `goat --headless /status /help /sessions /tools EOF`: all working
- `goat sessions`: shows session list from DB
- `goat doctor`: shows 15 checks including Provider count, DB migration, Headless mode

### Added — Phase 1.3: Foundation Cleanup (2026-06-08)

**Version bump: 0.1.0 → 0.2.0** | Binary renamed: `GOAT` → `goat`

**New crates added:** `uuid` (v4 session IDs), `anyhow` (error propagation), `thiserror` (typed errors), `clap` (CLI)

**New file: `src/paths.rs`**
- `GoatPaths` struct — single source of truth for all resolved filesystem paths
  - `config_file`: `~/.config/goat/goat.toml` (or `--config` override)
  - `data_dir`: `~/.local/share/goat/` (XDG data dir, platform-correct)
  - `db_file`: `~/.local/share/goat/goat.db` (inside data dir)
  - `log_dir`: `~/.local/share/goat/logs/` (no longer `./logs/`)
- `GoatPaths::resolve()` — platform-aware path resolution
- `GoatPaths::with_config()`, `with_db()`, `with_data_dir()` — CLI override helpers
- `GoatPaths::ensure_data_dir()`, `ensure_log_dir()` — create dirs if missing
- `GoatPaths::detect_legacy_db()` — finds old `./goat_brain.db` in CWD
- `check_config_permissions()` — `#[cfg(unix)]` permission bit check (warns if mode & 0o044 != 0)
- Doctor system: `DoctorCheck`, `DoctorStatus` (Ok/Warn/Fail/Info), `run_doctor()`, `print_doctor_results()`
  - Doctor checks: OS/platform info, GOAT version, config exists/permissions, data dir writable, DB open test, legacy DB detection, OpenAI/Groq key presence, provider configured, ApprovalGate status, log dir
- 10 new unit tests for path resolution, overrides, permission checks, doctor logic

**New file: `src/cli.rs`**
- `Cli` struct with `clap::Parser` — `--config <PATH>` and `--db <PATH>` global flags
- `Command` enum — subcommands: `config-path`, `data-path`, `db-path`, `doctor`, `migrate-db`
- `handle_subcommand()` — dispatches subcommands; returns `true` to skip TUI
- `handle_migrate_db()` — copies `./goat_brain.db` to XDG path, original preserved

**New file: `src/error.rs`**
- `GoatError` typed enum (thiserror): `NoHomeDir`, `NoDataDir`, `CreateDataDir`, `ReadFile`, `WriteFile`, `OpenDatabase`, `Database`, `ParseConfig`, `InsecureConfigPermissions`, `NoProviderConfigured`, `LlmRequest`, `SessionNotFound`, `ToolExecution`, `Internal`
- Designed for future use in typed error propagation

**Modified: `src/config.rs`**
- Now uses `anyhow::Result` throughout (removed `Box<dyn Error>`)
- `Config::load()` replaced by `Config::load_from(path: &Path) -> Result<ConfigLoadResult>`
- `ConfigLoadResult` struct: carries `config` + `warnings: Vec<String>`
- Newly created config files automatically `chmod 600` on Unix
- Insecure config permissions (mode & 0o044 != 0) added to `warnings`, shown in TUI at startup
- Config header comment written with security reminder
- `Config::load_from_path(PathBuf)` convenience wrapper added

**Modified: `src/main.rs`**
- Now uses `anyhow::Result` as return type
- `Cli::parse()` at startup — parses all CLI flags
- `GoatPaths::resolve()` + CLI overrides applied before anything else
- `ensure_data_dir()` + `ensure_log_dir()` called before logging starts
- Log files now written to `~/.local/share/goat/logs/` (XDG) not `./logs/`
- `Config::load_from()` used instead of `Config::load()`; config warnings passed to App
- `handle_subcommand()` called before raw mode — non-TUI commands print clean output and exit
- Legacy DB warning printed to stderr before TUI enters (visible on terminal restart)
- `App::new()` receives `(config, paths, warnings)` — all three passed explicitly
- `context()` strings on every `?` for human-readable error messages

**Modified: `src/app.rs`**
- Added `use crate::paths::GoatPaths` and `use uuid::Uuid`
- `App::new(config, paths, startup_warnings)` — new 3-argument signature
- Brain DB opened from `paths.db_file` (XDG) instead of hardcoded `"goat_brain.db"`
- Session IDs: new sessions use `Uuid::new_v4().to_string()` (e.g. `550e8400-e29b-41d4-a716-446655440000`)
- Old sessions (legacy timestamp IDs) still loaded and resumed without breakage
- Startup splash shows `v0.2` and brain DB path
- Config warnings (insecure permissions, created default, etc.) shown in log at startup

**CLI commands (all working):**
```
cargo run -- --version        → goat 0.2.0
cargo run -- --help           → full usage
cargo run -- config-path      → ~/.config/goat/goat.toml
cargo run -- data-path        → ~/.local/share/goat
cargo run -- db-path          → ~/.local/share/goat/goat.db
cargo run -- doctor           → system readiness report
cargo run -- migrate-db       → copy ./goat_brain.db → XDG path
cargo run -- --config <PATH>  → custom config file
cargo run -- --db <PATH>      → custom database file
```

**Migration/backward compatibility:**
- Old `./goat_brain.db` detected automatically
- `cargo run -- doctor` warns about legacy DB with copy command
- `cargo run -- migrate-db` performs the copy (original NOT deleted)
- Existing session IDs in old DB are kept as-is (no schema change)
- New sessions created after upgrade use UUIDs; old sessions use their original IDs

**Security improvements:**
- Config files created with `chmod 600` automatically on Unix
- Config permission check added (doctor + TUI startup warning)
- Legacy DB detection prevents silent data split
- All log directories now in XDG data path (not project root)

**Test results:**
- `cargo fmt`: clean
- `cargo check`: 0 errors, 11 dead_code warnings (public API)
- `cargo test`: 26/26 pass (16 approval + 10 paths tests)

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
