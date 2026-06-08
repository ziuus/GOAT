# GOAT — Architecture Document

**Version:** 0.1 (Phase 0 Draft)  
**Status:** Pre-Alpha — reflects CURRENT state + target architecture  
**Last Updated:** 2026-06-08

> This document describes both the current (actual) architecture and the target architecture. Sections are clearly labeled.

---

## 1. Current Architecture (As of Phase 0 Audit)

### 1.1 Crate Structure

The project is currently a **single Rust binary crate** (`GOAT v0.1.0`). There is no workspace yet.

```
src/
├── main.rs           # Entry point: logging init, terminal init, event loop
├── app.rs            # App state struct: owns all runtime state
├── brain.rs          # SQLite-backed memory, session, and file indexer
├── config.rs         # TOML config loader + env var + OpenCode fallback key
├── mcp.rs            # STDIO JSON-RPC MCP client + server manager
├── swarm.rs          # Keyword-based intent router to subagent profiles
├── tools.rs          # Native tool executors (bash, read_file, write_file, call_subagent)
└── agent/
    ├── mod.rs        # Submodule re-exports
    ├── litellm.rs    # OpenAI-compatible HTTP client (OpenAI + Groq)
    └── manager.rs    # ReAct agent loop (plan → act → observe)
```

**Missing:** `src/ui.rs` — prevents compilation.

### 1.2 Current Data Flow

```
User Input (keyboard)
       │
       ▼
main.rs event loop
       │
       ├─► ui::render(f, app)           [MISSING — blocks compilation]
       │
       └─► app.handle_user_input(msg)
                 │
                 ├─► brain.log_interaction()     [SQLite]
                 ├─► swarm_router.route(msg)     [keyword matching]
                 └─► agent_manager.execute_task()
                           │
                           ├─► llm_router.completion()   [HTTP → OpenAI/Groq]
                           ├─► NativeTools::execute()    [bash/fs tools]
                           └─► mcp_manager.call_tool()   [STDIO → MCP servers]
```

### 1.3 Current Storage

- `goat_brain.db` (SQLite) — located in project root (should be in XDG data dir)
  - Tables: `memory_blocks`, `sessions`, `interactions`, `indexed_files`
- `~/.config/goat/goat.toml` — TOML config file
- `logs/goat.log.YYYY-MM-DD` — rolling daily log file

---

## 2. Target Architecture (Post-Phase-8)

### 2.1 Workspace Structure

```
GOAT/                              # Cargo workspace root
├── Cargo.toml                     # workspace = true, members = [...]
├── crates/
│   ├── goat-cli/                  # Binary: CLI entrypoint, clap arg parsing
│   ├── goat-tui/                  # Library: ratatui TUI rendering
│   ├── goat-core/                 # Library: agent runtime, ReAct loop, task FSM
│   ├── goat-models/               # Library: provider trait + adapters
│   ├── goat-router/               # Library: model selection, fallback, retry
│   ├── goat-memory/               # Library: memory + indexer
│   ├── goat-tools/                # Library: tool implementations
│   ├── goat-harness/              # Library: async command executor
│   ├── goat-subagents/            # Library: subagent framework
│   ├── goat-skills/               # Library: skill/plugin system
│   ├── goat-voice/                # Library: STT voice prompting
│   ├── goat-config/               # Library: config, secrets, profiles
│   ├── goat-security/             # Library: permissions, approval, audit
│   ├── goat-dashboard/            # Binary: optional web dashboard
│   ├── goat-indexer/              # Library: project/system scanner
│   └── goat-integrations/         # Library: external agent adapters
└── tests/
    └── integration/               # Cross-crate integration tests
```

### 2.2 Crate Responsibilities

#### `goat-cli`
- Binary entry point
- `clap`-based CLI argument parsing
- Subcommands: `run`, `chat`, `learn`, `memory`, `sessions`, `skills`, `config`
- Initializes logging, config, and launches TUI or headless mode
- Depends on: `goat-core`, `goat-tui`, `goat-config`

#### `goat-tui`
- ratatui + crossterm rendering
- Panel system: chat, logs, sessions, diff, approval, memory, status
- Input handling and mode switching (Normal/Editing/Approval)
- Depends on: `goat-core`, `goat-config`

#### `goat-core`
- Central agent runtime
- `TaskLoop`: the ReAct (plan → act → observe) state machine
- `Task` struct with FSM: `Pending → Running → WaitingApproval → Completed | Failed`
- Session management
- Context compaction and summarization
- Checkpointing
- Depends on: `goat-models`, `goat-tools`, `goat-memory`, `goat-security`

#### `goat-models`
- `Provider` trait (async):
  ```rust
  pub trait Provider: Send + Sync {
      async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse>;
      fn name(&self) -> &str;
      fn supports_tools(&self) -> bool;
  }
  ```
- Adapters: OpenAI, Anthropic, Gemini, Groq, Ollama, OpenRouter
- Shared types: `Message`, `ToolCall`, `CompletionRequest`, `CompletionResponse`

#### `goat-router`
- `ModelRouter` trait: selects provider/model for a given task
- Fallback chain: try models in order until one succeeds
- Retry policy: exponential backoff with configurable max retries
- Rate-limit detection: 429 handling with wait
- Model profiles: `cheap`, `balanced`, `powerful`, `local`, `coding`, `reasoning`
- Provider health tracking

#### `goat-memory`
- `MemoryStore` trait backed by SQLite (current `brain.rs`)
- Session memory: conversation history per session
- Project memory: per-project context store
- User memory: long-term user profile and preferences
- Indexed files: content-addressed file store with SHA-256 dedup
- Search: simple keyword search initially; vector embedding later
- Automatic summarization via model call

#### `goat-tools`
- `Tool` trait + registry
- Implementations: bash, read_file, write_file, git, browser, web search, editor diff
- MCP client integration (current `mcp.rs`)
- All tools require approval metadata (can prompt, can bypass, risk level)

#### `goat-harness`
- Fast async command executor
- Streaming stdout/stderr
- Timeout enforcement
- Exit code capture
- Resource limits (CPU/memory where possible on Linux)
- Depends on: `tokio`

#### `goat-subagents`
- `SubagentAdapter` trait for internal and external agents
- Internal agents: Coder, Reviewer, Researcher, Planner, Tester, Debugger
- External adapters: OpenCode, Claude Code, Gemini CLI, Codex, Cline, etc.
- External agents run via `goat-harness` with isolation, timeout, and budget

#### `goat-skills`
- Skill manifest format (TOML)
- Skill loader from `~/.config/goat/skills/`
- Custom commands, tools, and prompts via skills
- MCP server launch support
- Skill permissions and enable/disable

#### `goat-voice`
- Push-to-talk recording (cpal)
- STT via local whisper.cpp (preferred) or remote API
- Transcript injected into prompt
- Voice command history

#### `goat-config`
- Config file: `~/.config/goat/goat.toml`
- Secrets: API keys (with keyring support planned)
- Profiles: model profile definitions
- Validation and migration

#### `goat-security`
- `PermissionSet` per operation type (shell, file, network, subprocess)
- `ApprovalGate`: prompts user in TUI before executing dangerous tools
- Allowlist/blocklist for commands and paths
- Audit log: persistent append-only log of all tool executions
- Secret redactor: strips API keys/tokens from log output
- Dry-run mode: describe actions without executing

#### `goat-dashboard`
- Optional axum-based web server
- REST API for sessions, memory, settings, subagent runs
- Not required until Phase 8

#### `goat-indexer`
- Project scanner: detects tech stack, commands, important files
- Ignores: secrets, node_modules, build artifacts, .git internals, binaries
- Approval before scanning sensitive directories
- Produces structured project summaries
- Feeds into `goat-memory`

#### `goat-integrations`
- Adapters for specific external tool configs (e.g., read OpenCode config for keys)
- Protocol adapters (MCP is already in `goat-tools`)
- LSP integration (later)

---

## 3. Key Design Decisions

### 3.1 Trait-based Provider Abstraction

All LLM providers implement the `Provider` trait. This allows:
- Swapping providers without changing call sites
- Testing with mock providers
- Adding new providers without touching core

### 3.2 Approval Gates Are Mandatory for Dangerous Tools

Every tool that can modify the system (bash, write_file, subagent spawn) must:
1. Declare its risk level in its metadata
2. Pass through `ApprovalGate` before execution in non-bypass mode
3. Log the approval decision to the audit trail

### 3.3 Memory is Local-First

All memory (sessions, project data, indexed files) lives in SQLite on the user's machine by default. No cloud sync. User controls what is indexed.

### 3.4 Workspace Migration Strategy

The migration from single-crate to workspace will happen incrementally:
- Phase 1: Add `ui.rs`, remain single-crate
- Phase 2: Extract `goat-tui` as first workspace crate
- Phase 3: Extract `goat-models` as second crate
- Continue per phase

---

## 4. External Dependencies (Current)

| Crate | Version | Used By |
|-------|---------|---------|
| ratatui | 0.30.1 | TUI |
| crossterm | 0.29.0 | TUI backend |
| tokio | 1.52.3 | Async runtime |
| reqwest | 0.13.4 | HTTP client |
| rusqlite | 0.40.1 | Brain/memory |
| serde | 1.0.228 | Serialization |
| serde_json | 1.0.150 | JSON |
| toml | 1.1.2 | Config |
| tracing | 0.1.44 | Logging |
| tracing-appender | 0.2.5 | Log files |
| tracing-subscriber | 0.3.23 | Log formatting |
| walkdir | 2.5.0 | File indexer |
| sha2 | 0.11.0 | File hashing |
| dirs | 6.0.0 | XDG paths |
| rand | 0.10.1 | Random IDs |

---

## 5. Architecture Invariants

These rules MUST NOT be violated:

1. No crate may directly access the filesystem without going through `goat-security` approval in non-test code
2. No crate may spawn a subprocess without using `goat-harness`
3. No crate may store secrets in plain log output
4. Provider adapters must not assume a specific model name in business logic
5. All async functions must have timeouts
6. SQLite connections must use WAL mode and busy timeout
