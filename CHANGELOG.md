# CHANGELOG

All notable changes to GOAT are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — Phase 1: Minimal Working Core

### Planned
- Restore `src/ui.rs` to fix compilation blocker
- Add approval gate for `bash` tool
- Add approval gate for `write_file` tool
- Move brain DB to XDG data directory
- Add `clap` for CLI argument parsing
- Add `uuid` for session IDs
- Add `anyhow` for structured error handling

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
