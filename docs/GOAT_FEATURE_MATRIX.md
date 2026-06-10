# GOAT — Feature Matrix

**Last Updated:** 2026-06-09  
**Current Phase:** 3.0 (Advanced TUI)

Status legend:
- `planned` — Designed, not started
- `in-progress` — Actively being built
- `partial` — Exists but incomplete or broken
- `working` — Implemented and tested
- `broken` — Was working, now broken
- `removed` — Removed from scope

---

## Core Infrastructure

| Feature | Status | Notes |
|---------|--------|-------|
| Single-crate Rust binary | `working` | Compiles cleanly |
| Cargo workspace (multi-crate) | `planned` | Phase 2+ |
| `cargo check` passes | `working` |  |
| `cargo test` passes | `working` | 63/63 tests passing |
| XDG-compliant config path (`~/.config/goat/goat.toml`) | `working` | Auto-creates default |
| XDG-compliant data path for brain DB | `partial` | DB currently in project root (wrong) |
| Rolling daily log file | `working` | `logs/goat.log.YYYY-MM-DD` |
| Structured tracing logs | `working` | tracing + tracing-appender |
| Background Daemon | `working` | Phase 4.0 |
| Local API Server (Axum) | `working` | Phase 4.0 |

---

## Terminal UI (TUI)

| Feature | Status | Notes |
|---------|--------|-------|
| TUI launches (ratatui + crossterm) | `working` | Complete with multi-pane support |
| Logs panel | `working` | Chat view implemented |
| Input panel | `working` | Input composer implemented |
| Status bar / agent indicator | `working` | Header implemented |
| Session list panel | `partial` | Sidebar has sessions placeholder |
| File diff view | `partial` | Patch view diff placeholder |
| Task timeline | `planned` | Phase 2 |
| Approval prompt UI | `planned` | Phase 1 (security priority) |
| Subagent status panel | `planned` | Phase 5 |
| Memory panel | `working` | Memory commands & context injection |
| Provider/fallback status | `working` | Handled via /status and profiles |
| Project context panel | `working` | Handled via /status |
| Token/cost indicator | `planned` | Phase 3 |
| Keyboard shortcuts | `working` | Ctrl+1..9 to switch views |
| MCP start shortcut (c) | `partial` | Logic in main.rs |
| Command palette shortcut (Ctrl+P) | `working` | Opens palette view |

---

## Model Provider System

| Feature | Status | Notes |
|---------|--------|-------|
| OpenAI-compatible HTTP client | `working` | `agent/litellm.rs` |
| OpenAI provider | `working` | Via OPENAI_API_KEY or config |
| Groq provider | `working` | Via GROQ_API_KEY or config |
| Anthropic provider | `planned` | Not implemented despite README claim |
| Gemini provider | `planned` | Not implemented despite README claim |
| Ollama provider | `planned` | Phase 3 |
| OpenRouter provider | `planned` | Phase 3 |
| Custom provider via config | `planned` | Phase 3 |
| OpenCode config key fallback | `working` | Reads `~/.config/opencode/opencode.json` |
| OPENAI_API_KEY env var | `working` | Checked in litellm.rs |
| GROQ_API_KEY env var | `working` | Checked in litellm.rs |
| Fallback model chain | `planned` | Phase 3 |
| Retry with exponential backoff | `planned` | Phase 3 |
| Rate-limit detection (429 handling) | `planned` | Phase 3 |
| Provider health checks | `planned` | Phase 3 |
| Model profiles (cheap/balanced/powerful/etc.) | `planned` | Phase 3 |
| Per-request timeout (120s) | `working` | Hard-coded in litellm.rs |
| Tool calling (function calling) | `working` | Implemented in manager.rs |
| Streaming responses | `planned` | Phase 3 |

---

## Agent Runtime

| Feature | Status | Notes |
|---------|--------|-------|
| ReAct loop (plan → act → observe) | `working` | Up to 10 iterations |
| Configurable max iterations | `planned` | Currently hardcoded to 10 |
| Tool calling dispatch | `working` | Native + MCP tools |
| Session persistence | `working` | SQLite via brain.rs |
| Session resume on startup | `working` | Loads latest session from DB |
| Multi-session support | `partial` | DB supports it; UI does not |
| Task cancellation | `planned` | Phase 2 |
| Context compaction | `planned` | Phase 4 |
| History bounding (80 messages) | `working` | app.rs trim_history() |
| Checkpointing | `planned` | Phase 4 |
| Error recovery / retry | `planned` | Phase 3 |
| Human approval gates | `planned` | Phase 1 priority |
| Dry-run mode | `planned` | Phase 2 |

---

## Memory / Brain

| Feature | Status | Notes |
|---------|--------|-------|
| SQLite brain (WAL mode) | `working` | brain.rs |
| Session storage | `working` | `sessions` table |
| Interaction log | `working` | `interactions` table |
| Memory blocks | `working` | `memory_blocks` table |
| File indexer | `working` | `indexed_files` table with SHA-256 dedup |
| File indexer ignore list | `working` | .git, node_modules, target, etc. |
| File indexer extension allowlist | `working` | rs, py, ts, md, etc. |
| File indexer size limit (512KB) | `working` | MAX_INDEXED_FILE_BYTES |
| Binary file detection | `working` | Checks for null bytes |
| Memory search | `planned` | Phase 4 |
| Vector embeddings | `planned` | Phase 4 |
| Automatic summarization | `planned` | Phase 4 |
| Memory ranking/retrieval | `planned` | Phase 4 |
| Memory edit/delete commands | `planned` | Phase 4 |
| Per-project memory isolation | `planned` | Phase 4 |
| Memory import/export | `planned` | Phase 4 |
| Privacy controls | `planned` | Phase 4 |

---

## Project / System Learning Mode

| Feature | Status | Notes |
|---------|--------|-------|
| `learn_about_me` command (l key) | `partial` | Logic works; no UI, no approval |
| Scans ~/Projects, ~/PAI, ~/Documents | `partial` | Hard-coded paths |
| User approval before scanning | `planned` | Phase 1 security priority |
| Tech stack detection | `planned` | Phase 4 |
| Project Context Injection | `working` | Phase 1.7 |
| Memory Injection | `working` | Phase 2.0 |
| Recall | `working` | Phase 2.0 |
| Skills System | `working` | Phase 2.1 |
| Automatic Context Pruning | `planned` | Phase 4 |
| Configurable scan roots | `planned` | Phase 4 |
| Re-indexing on change | `planned` | Phase 4 |
| Project summaries | `planned` | Phase 4 |

---

## Skills System

| Feature | Status | Notes |
|---------|--------|-------|
| Skills Directory (`~/.config/goat/skills`) | `working` | Phase 2.1 |
| SKILL.md format & parsing | `working` | Phase 2.1 |
| CLI commands (`goat skills ...`) | `working` | Phase 2.1 |
| Slash commands (`/skills`, `/skill`) | `working` | Phase 2.1 |
| Search & Validation | `working` | Phase 2.1 |
| Context Injection (Progressive) | `working` | Phase 2.1 |
| Security / Suspicious Pattern Detection | `working` | Phase 2.1 |
| `save-skill` | `partial` | Creates placeholder template |
| Auto-summarization of sessions into skills | `planned` | Phase 2.2 |

---

## Subagent System

| Feature | Status | Notes |
|---------|--------|-------|
| SwarmRouter (keyword-based) | `working` | Routes to Coder/Browser/Researcher/General profiles |
| Subagent profiles (4 types) | `working` | SubagentKind enum in swarm.rs |
| `call_subagent` native tool | `partial` | Works but unsafe: no timeout, no isolation |
| Internal subagent framework | `planned` | Phase 5 |
| External subagent adapter trait | `planned` | Phase 5 |
| OpenCode adapter | `planned` | Phase 5 |
| Claude Code adapter | `planned` | Phase 5 |
| Gemini CLI adapter | `planned` | Phase 5 |
| Subagent workspace isolation | `planned` | Phase 5 |
| Subagent timeouts and budgets | `planned` | Phase 5 |
| Subagent output streaming | `planned` | Phase 5 |
| Multi-agent comparison | `planned` | Phase 8 |

---

## Tools

| Feature | Status | Notes |
|---------|--------|-------|
| `bash` tool | `partial` | Works but no approval gate |
| `read_file` tool | `partial` | Works but no path allowlist |
| `write_file` tool | `partial` | Works but no approval gate |
| `call_subagent` tool | `partial` | Works but unsafe |
| MCP tool dispatch | `working` | Via McpManager |
| Git tools | `planned` | Phase 4 |
| Browser tools | `planned` | Phase 5 |
| Web search tools | `planned` | Phase 4 |
| Editor diff tools | `planned` | Phase 2 |

---

## MCP (Model Context Protocol)

| Feature | Status | Notes |
|---------|--------|-------|
| STDIO MCP client | `working` | mcp.rs |
| Multi-server management | `working` | McpManager |
| Server spawn with env vars | `working` | McpClient::spawn_with_env |
| Initialize handshake | `working` | With 30s timeout |
| Tool listing | `working` | tools/list |
| Tool calling | `working` | tools/call with 60s timeout |
| Server shutdown | `working` | With 10s timeout |
| Config-driven server list | `working` | From goat.toml mcp_servers section |
| Tool index (tool → server map) | `working` | tool_index HashMap |

---

## Configuration

| Feature | Status | Notes |
|---------|--------|-------|
| TOML config file | `working` | Auto-creates default |
| OpenAI API key config | `working` | |
| Groq API key config | `working` | |
| MCP server config | `working` | |
| Anthropic API key config | `planned` | Phase 3 |
| Gemini API key config | `planned` | Phase 3 |
| Ollama config | `planned` | Phase 3 |
| Model profiles in config | `planned` | Phase 3 |
| Keyring support | `planned` | Phase 3 |
| Multiple config profiles | `planned` | Phase 3 |

---

## Security

| Feature | Status | Notes |
|---------|--------|-------|
| Approval gate for bash | `working` | Phase 1.1 — `src/approval.rs` |
| Approval gate for write_file | `working` | Phase 1.1 — `src/approval.rs` |
| Approval gate for call_subagent | `working` | Phase 1.1 — `src/approval.rs` |
| Command allowlist/blocklist | `planned` | Phase 2 |
| Path allowlist/blocklist | `planned` | Phase 2 |
| Persistent audit log | `planned` | Phase 2 |
| Secret redaction from logs | `partial` | Basic secret redaction in approval.rs display |
| Sandbox mode | `planned` | Phase 6 |
| Dry-run mode | `planned` | Phase 2 |
| API key encryption | `planned` | Phase 3 |
| Risk level assessment (Critical/High/Medium) | `working` | Phase 1.1 — assess_bash_risk, assess_write_risk |
| Session policy (always allow/deny per tool) | `working` | Phase 1.1 — SessionPolicy in ApprovalGate |

---

## Voice Companion / Jarvis Mode (PLANNED)

> **Note:** Voice features are **strictly optional and fully disableable**. They will not be active without explicit user opt-in and permission. These features are slated for a future development phase (Phase 7) and are not currently implemented.

| Feature | Status | Notes |
|---------|--------|-------|
| Push-to-talk / Voice trigger | `planned` | Phase 7 |
| Local STT (Whisper, etc) | `planned` | Phase 7 |
| Remote STT (OpenAI, etc) | `planned` | Phase 7 |
| Voice history / TTS responses | `planned` | Phase 7 |

---

## Web Dashboard

| Feature | Status | Notes |
|---------|--------|-------|
| Web server / Next.js app | `working` | Phase 4.1 |
| Local API endpoints | `working` | Phase 4.0 |
| Token Auth | `working` | Phase 4.1 |
| Approvals & SSE Streaming | `working` | Phase 4.2 |
| Chat & Command UI | `working` | Phase 4.3 |
| Repo Explorer | `working` | Phase 4.3 |
| Diff Viewer | `working` | Phase 4.3 |
| Context Management API | `working` | Phase 4.3 |
| Safe Command Execution UI | `working` | Phase 4.4 |
| Async Agent Chat & Jobs | `working` | Phase 4.5 |

---

## Documentation

| Document | Status | Notes |
|----------|--------|-------|
| README.md | `partial` | Exists but inaccurate — being updated |
| CHANGELOG.md | `planned` | Being created in Phase 0 |
| docs/GOAT_PRODUCT_SPEC.md | `working` | Created in Phase 0 |
| docs/GOAT_ARCHITECTURE.md | `working` | Created in Phase 0 |
| docs/GOAT_FEATURE_MATRIX.md | `working` | This document |
| docs/GOAT_IMPLEMENTATION_ROADMAP.md | `working` | Created in Phase 0 |
| docs/GOAT_SECURITY_MODEL.md | `working` | Created in Phase 0 |
| docs/GOAT_CODEBASE_AUDIT.md | `working` | Created in Phase 0 |
| Inline code documentation | `partial` | Some comments, no rustdoc |
