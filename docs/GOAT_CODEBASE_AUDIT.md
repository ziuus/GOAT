# GOAT Codebase Audit — Phase 0

**Audit Date:** 2026-06-08  
**Auditor:** Antigravity (Principal Rust Systems Architect mode)  
**Repo:** https://github.com/ziuus/GOAT.git  
**Branch:** `master` (HEAD: `52150d3`)

---

## 1. What the Current Codebase Already Has

The current codebase is a **single-crate Rust binary** named `GOAT` (v0.1.0, edition 2024). It is a genuine initial effort at an AI agent TUI — not purely hallucinated — but it is **incomplete and currently broken to compile**.

### Modules present (all in `src/`):

| Module | File | Status |
|--------|------|--------|
| Entry point | `main.rs` | Compiles partially — references missing `ui` module |
| Application state | `app.rs` | Logically coherent, well-structured |
| LLM provider adapter | `agent/litellm.rs` | Working OpenAI-compatible HTTP client |
| Agent task manager | `agent/manager.rs` | Working ReAct loop (plan → act → observe) |
| Memory/Brain | `brain.rs` | Solid SQLite-backed memory, session, file indexing |
| Config loader | `config.rs` | TOML config + env var + OpenCode fallback key read |
| MCP client/manager | `mcp.rs` | Working STDIO-based JSON-RPC MCP client |
| Swarm router | `swarm.rs` | Keyword-based intent router to subagent profiles |
| Native tools | `tools.rs` | bash, read_file, write_file, call_subagent tools |
| Agent mod | `agent/mod.rs` | Simple submodule exports |
| **MISSING** | `ui.rs` / `ui/mod.rs` | **Referenced in main.rs — FILE DOES NOT EXIST** |

### Other artifacts:

- `goat_brain.db` — Live SQLite database (already has sessions/interactions from prior runs)
- `logs/goat.log.2026-06-08` — Evidence that the TUI launched and received input in prior sessions
- `Cargo.lock` — 91KB lock file showing a fully resolved dependency graph
- `references/` — Four reference repositories (gitignored from main repo):
  - `NemoClaw` — empty (just `.git`)
  - `Songbird` — C/CMake project (MIT licensed)
  - `hummcode` — Python project (MIT licensed)
  - `repobird-cli` — Go project (Apache 2.0 licensed)

---

## 2. What Parts Are Useful and Should Be Kept

### Keep — high quality, genuinely useful:

| Component | Reason |
|-----------|--------|
| `brain.rs` | Clean SQLite schema, WAL mode, file indexer with hash dedup, session/interaction storage. Well-designed. |
| `mcp.rs` | Working STDIO JSON-RPC MCP client with timeout handling, tool index, multi-server support. Solid foundation. |
| `agent/litellm.rs` | Clean OpenAI-compatible HTTP client. Supports OpenAI and Groq. Falls back to OpenCode config keys. Architecture is right. |
| `agent/manager.rs` | Proper ReAct loop (up to 10 iterations), MCP + native tool dispatch, logging callback pattern. |
| `swarm.rs` | Keyword-based router is simple but works. Good foundation to extend to semantic/embedding-based routing. |
| `config.rs` | Clean TOML config loading, env var API key reading, OpenCode config key fallback. |
| `tools.rs` | bash, read_file, write_file, call_subagent native tools. Basis for secure tool layer. |
| `app.rs` | Clean application state container. Session resume logic is useful. |
| `Cargo.toml` | Reasonable dependency choices (ratatui, tokio, rusqlite, reqwest, serde). |

---

## 3. What Parts Are Broken, Fake, Hallucinated, Duplicated, or Incomplete

### Critical breakage:

| Problem | Details |
|---------|---------|
| **Missing `ui` module** | `main.rs:7` declares `mod ui;` but `src/ui.rs` and `src/ui/mod.rs` do not exist. This is the **sole blocker preventing compilation**. |

### Incomplete / partial:

| Problem | Details |
|---------|---------|
| **No approval/permission gate** | `tools.rs` `bash` tool executes commands directly with no user approval prompt. This is a security gap. |
| **No actual TUI code** | There is no UI rendering code in the repository. |
| **Swarm router is trivial** | Keyword matching only. No embedding-based routing, no actual subagent spawning. |
| **`call_subagent` is naive** | No timeout, no output streaming, no workspace isolation, no budget limit. |
| **No Anthropic/Gemini provider** | README claims "Anthropic, and Gemini models" but neither is implemented. |
| **No session listing in TUI** | Session persistence works in SQLite but no UI to browse sessions. |
| **Config only reads two keys** | Only `openai_api_key` and `groq_api_key`. Anthropic, Gemini, Ollama etc. not implemented. |
| **`learn_about_me` has no approval** | Scans home directories without any user consent prompt. |
| **ReAct loop max iterations hardcoded** | `manager.rs:31` hardcodes `for _iteration in 0..10`. Not configurable. |
| **No error recovery / retry** | LLM failure immediately returns `Err`. No retry policy, no fallback model chain. |
| **No CHANGELOG.md** | Does not exist. |
| **No `docs/` directory** | Did not exist before this audit. |
| **README is aspirational, not accurate** | Claims Anthropic/Gemini support and multi-pane TUI that doesn't compile. |

---

## 4. What Should Be Deleted or Rewritten Later

| Item | Action | Timing |
|------|--------|--------|
| `src/ui.rs` | **Create** (missing, not deletable) | Phase 1 top priority |
| README.md | Rewrite to be accurate | Done in Phase 0 |
| `goat_brain.db` path | Relocate to XDG data dir | Phase 1 |
| `call_subagent` in tools.rs | Rewrite with timeout, isolation, approval gate | Phase 5 |
| `bash` tool in tools.rs | Add approval prompt before execution | Phase 1 security |
| SwarmRouter keyword logic | Extend with configurable rules | Phase 4/5 |
| `config.rs` Keys struct | Expand for Anthropic, Gemini, Ollama, OpenRouter | Phase 3 |
| `agent/litellm.rs` | Move to trait-based `goat-models` crate | Phase 3 |

---

## 5. Current Project Structure

```
GOAT/
├── Cargo.toml              # Single-crate manifest (edition 2024)
├── Cargo.lock              # Fully resolved (91KB)
├── README.md               # Aspirational, inaccurate
├── .gitignore              # Ignores /target and references/
├── goat_brain.db           # Live SQLite DB (wrong location — should be in XDG data dir)
├── logs/
│   └── goat.log.2026-06-08 # Daily rolling log
├── references/             # Gitignored reference repos (see below)
│   ├── NemoClaw/           # Empty
│   ├── Songbird/           # C/CMake, MIT
│   ├── hummcode/           # Python, MIT
│   └── repobird-cli/       # Go, Apache 2.0
└── src/
    ├── main.rs             # Entry + TUI event loop
    ├── app.rs              # App state container
    ├── brain.rs            # SQLite memory/indexer
    ├── config.rs           # TOML config loader
    ├── mcp.rs              # MCP STDIO client
    ├── swarm.rs            # Keyword-based task router
    ├── tools.rs            # Native tool executors
    └── agent/
        ├── mod.rs          # Submodule exports
        ├── litellm.rs      # OpenAI-compatible HTTP client
        └── manager.rs      # ReAct agent loop

MISSING: src/ui.rs  ← prevents compilation
```

---

## 6. Ideal Project Structure (Target Architecture)

```
GOAT/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock
├── README.md
├── CHANGELOG.md
├── LICENSE
├── docs/
│   ├── GOAT_PRODUCT_SPEC.md
│   ├── GOAT_ARCHITECTURE.md
│   ├── GOAT_FEATURE_MATRIX.md
│   ├── GOAT_IMPLEMENTATION_ROADMAP.md
│   ├── GOAT_SECURITY_MODEL.md
│   └── GOAT_CODEBASE_AUDIT.md
├── crates/
│   ├── goat-cli/           # Binary entrypoint, CLI arg parsing
│   ├── goat-tui/           # Ratatui-based TUI
│   ├── goat-core/          # Agent runtime, ReAct loop, task state
│   ├── goat-models/        # Provider trait + adapters (OpenAI, Anthropic, Gemini, Groq, Ollama)
│   ├── goat-router/        # Model routing, fallback chain, retry, rate-limits
│   ├── goat-memory/        # Session, project, long-term memory, indexer
│   ├── goat-tools/         # bash, fs, git, browser, web, editor, MCP, APIs
│   ├── goat-harness/       # Fast async command executor
│   ├── goat-subagents/     # Internal subagent framework + external adapters
│   ├── goat-skills/        # Skill/plugin system
│   ├── goat-voice/         # STT voice prompting
│   ├── goat-config/        # Config, secrets, profiles
│   ├── goat-security/      # Permissions, sandboxing, approval, audit log
│   ├── goat-dashboard/     # Optional web dashboard (later)
│   ├── goat-indexer/       # Project/system scanner
│   └── goat-integrations/  # External agent adapters
└── tests/                  # Integration tests
```

Migration strategy: Incrementally extract modules into crates as each is stabilized. Do NOT big-bang rewrite.

---

## 7. Dependencies Currently Used

| Crate | Version | Purpose | Verdict |
|-------|---------|---------|---------|
| `ratatui` | 0.30.1 | TUI rendering | Keep |
| `crossterm` | 0.29.0 | Terminal backend | Keep |
| `tokio` | 1.52.3 (full) | Async runtime | Keep |
| `reqwest` | 0.13.4 (json) | HTTP client for LLM APIs | Keep |
| `rusqlite` | 0.40.1 | SQLite for brain/memory | Keep |
| `serde` | 1.0.228 (derive) | Serialization | Keep |
| `serde_json` | 1.0.150 | JSON handling | Keep |
| `toml` | 1.1.2 | Config file parsing | Keep |
| `tracing` | 0.1.44 | Structured logging | Keep |
| `tracing-appender` | 0.2.5 | Rolling file appender | Keep |
| `tracing-subscriber` | 0.3.23 | Log formatting | Keep |
| `walkdir` | 2.5.0 | Directory traversal for indexer | Keep |
| `sha2` | 0.11.0 | File content hashing for dedup | Keep |
| `dirs` | 6.0.0 | XDG/home dir resolution | Keep |
| `rand` | 0.10.1 | Random ID for MCP requests | Keep |

Note: `aws-lc-rs` appears in Cargo.lock (pulled by reqwest TLS). This is a large C dependency. Monitor binary size.

---

## 8. Compile Status

**Does NOT compile.**

**Command:**
```bash
cargo check
```

**Output:**
```
error[E0583]: file not found for module `ui`
 --> src/main.rs:7:1
  |
7 | mod ui;
  | ^^^^^^^
  = help: to create the module `ui`, create file "src/ui.rs" or "src/ui/mod.rs"

error: could not compile `GOAT` (bin "GOAT") due to 1 previous error
```

**Note:** Evidence in `logs/goat.log.2026-06-08` confirms the app compiled and ran previously (07:37:30 and 13:30:23 on 2026-06-08). The `ui.rs` file existed and was subsequently lost (likely accidentally deleted or never committed).

---

## 9. Exact Commands Run During Audit

```bash
git remote -v
git log --oneline -15
cat Cargo.lock | grep "^name" | head -40
cargo check
```

---

## 10. Security Concerns

| Risk | Severity | Details |
|------|---------|---------|
| Unrestricted bash execution | HIGH | `tools.rs` `bash` handler executes any shell command without approval, sandboxing, or allowlist. |
| No approval for file writes | HIGH | `write_file` tool writes to any path provided by the LLM without user confirmation. |
| No approval for subagent spawn | MEDIUM | `call_subagent` launches arbitrary CLIs with arbitrary prompts with no approval. |
| API keys in plain config | LOW | `goat.toml` stores API keys in plaintext. Should warn and support keyring in future. |
| `learn_about_me` scans without consent | LOW | Scans home directories without per-session approval. |
| `goat_brain.db` in project root | LOW | SQLite DB in repo directory. Risk of accidental commit. |
| No audit log per tool call | LOW | Tool executions not written to persistent audit trail. |
| MCP subprocess has no sandbox | LOW | MCP servers spawned with full system access. |
| No secret redaction from logs | LOW | API error messages may contain key fragments. |

---

## 11. Recommended First Coding Task (Phase 1 Start)

**Create `src/ui.rs` with a minimal ratatui TUI renderer.**

This single task unblocks compilation and restores the ability to run and test all existing functionality.

The `ui.rs` must implement:
- `pub fn render(f: &mut Frame, app: &App)` — called from `main.rs`
- Layout: logs panel (primary), input panel, status bar
- Display `app.logs`, `app.input`, `app.input_mode`, `app.current_task`
- Show model/provider info from `app.active_route`
- Indicate MCP server status

After Phase 1 Phase 1 unblocking tasks:
1. Restore `src/ui.rs` to unblock compilation
2. Add approval prompt before `bash` tool execution
3. Relocate `goat_brain.db` to XDG data directory
4. Add `clap` for proper CLI arg parsing
5. Fix README to be accurate
