# GOAT — Product Specification

**Version:** 0.1 (Phase 0 Draft)  
**Status:** Pre-Alpha  
**Last Updated:** 2026-06-08

---

## 1. Vision

GOAT (General Omniscient Agentic Tool) is a universal, extremely powerful AI CLI/TUI agent platform built in Rust. It combines the best features from leading AI agent systems into a single, coherent, terminal-first experience.

GOAT is designed to be:
- **Universal:** Works with any LLM provider (OpenAI, Anthropic, Gemini, Groq, Ollama, OpenRouter, and more)
- **Powerful:** Full ReAct agent loop with tool execution, subagent orchestration, and long-running task support
- **Terminal-first:** Runs entirely in the terminal with a rich TUI, no GUI required
- **Modular:** Every feature lives behind a clean interface; nothing is monolithic
- **Safe:** Permission gates, approval prompts, sandboxing, and audit logs for all dangerous operations
- **Aware:** Learns about your projects, system, and work history to provide context-aware assistance

---

## 2. Inspiration Sources

GOAT draws architectural and feature inspiration (not code) from:

| Tool | What GOAT learns from it |
|------|-------------------------|
| OpenCode | Robust TUI, developer workflow, MCP integration |
| Claude Code | Software building ability, iterative code edits |
| Antigravity | Subagent orchestration, skill system |
| Gemini CLI | Provider flexibility, coding tasks |
| JCode | Fast Rust execution harness |
| Hermes | Memory/brain system, fallback model chain |
| Little Bird AI | Project and system awareness |
| OpenClaw | Skills, custom agents, web dashboard, integrations |
| Codex/Copilot/Cline | Coding assistant workflows |

> License note: No code is copied from any external project without explicit license verification. All inspiration is implemented as clean GOAT-native code.

---

## 3. Core Principles

1. Rust-first architecture
2. Modular, clean, maintainable design
3. No hallucinated features — every claimed feature must work
4. Every feature behind a clear interface/trait
5. No monolith — small working vertical slices
6. Security first at every layer
7. Documentation must always reflect reality

---

## 4. Target User

- Developers who live in the terminal
- Power users who want AI assistance without browser-based tools
- Teams who need auditable, repeatable AI agent runs
- Users who want to run multiple AI providers and agents from one interface

---

## 5. Feature Areas

### 5.1 Terminal UI (TUI)

A rich ratatui-based TUI with panels for:
- Active session / chat history
- Session list
- Command execution log
- File diff view
- Model/provider indicator with token/cost tracker
- Task timeline
- Approval prompts for dangerous actions
- Subagent status panel
- Memory/brain panel
- Provider/fallback status
- Project context panel

### 5.2 Model Provider System

- OpenAI-compatible APIs (OpenAI, Azure, OpenRouter, local)
- Anthropic (Claude family)
- Gemini / Google AI
- Groq
- Ollama (local models)
- OpenRouter (meta-router)
- Custom providers via config
- Fallback model chain (try model A, fall back to B, then C)
- Retry policy with exponential backoff
- Rate-limit detection and backoff
- Provider health checks
- Model profiles: cheap, balanced, powerful, local, coding, reasoning, vision, long-context

### 5.3 Agent Runtime

- Plan → Act → Observe (ReAct) loop
- Tool calling with structured arguments
- Task state machine (pending, running, waiting-approval, completed, failed)
- Resumable sessions (persist and reload from SQLite)
- Multi-session support
- Task cancellation (Ctrl+C / TUI action)
- Safe shell execution with approval gate
- Structured logs (tracing)
- Checkpointing (save state mid-task)
- Context compaction (summarize old history when context window fills)
- Error recovery (retry, fallback, user intervention)
- Human approval gates for destructive operations

### 5.4 Memory / Brain

- Session memory (current conversation)
- Project memory (per-project knowledge)
- User profile memory (long-term user preferences)
- Long-term knowledge store (SQLite, optionally vector-extended)
- File indexer with SHA-256 dedup
- Automatic summarization of old sessions
- Memory ranking and retrieval
- Explicit "learn my system/projects" mode
- Privacy controls (user can disable/limit scanning)
- Memory edit/delete commands
- Per-project memory isolation
- Memory import/export

### 5.5 Project / System Learning Mode

- Scan projects, READMEs, package files, docs, git history
- Detect tech stack automatically
- Detect project commands (dev, build, test, lint, format, deploy)
- Detect important files and architecture
- Build searchable project index
- Ignore: secrets, node_modules, build outputs, .git internals, binaries, env files, cache
- Ask approval before reading sensitive folders
- Produce project summaries for future tasks
- Support re-indexing when project changes

### 5.6 Subagent System

Internal subagents (built-in GOAT agents):
- Coder
- Reviewer
- Researcher
- Planner
- Tester
- Debugger
- Documentation Writer
- Refactorer
- Security Auditor
- UI Designer
- System Fixer

External subagent adapters (spawn as subprocess):
- OpenCode
- Claude Code
- Antigravity
- Gemini CLI
- Codex
- Cline
- JCode
- Any future agent

External subagent requirements:
- Run as isolated subprocesses
- Capture stdout/stderr
- Use timeouts and budgets
- Log all activity
- No destructive changes without approval
- Route tasks to best available agent

### 5.7 Skills / Plugins

- Skill manifest format (TOML)
- Custom commands and tools
- MCP (Model Context Protocol) support
- Browser automation support
- API integrations
- User-created skills
- Skill permissions model
- Skill enable/disable commands

### 5.8 Voice Prompting

- Push-to-talk command in TUI
- Local STT preferred (whisper.cpp or cpal-based)
- Remote STT optional
- Transcript inserted into prompt
- Voice command history
- Voice confirmation for dangerous actions (later)

### 5.9 Security

- Permission system for shell/file/network operations
- Approval prompt before destructive commands
- Secret detection in prompts and tool outputs
- Sandbox mode (read-only filesystem, no network)
- Persistent audit log
- Dry-run mode
- Command allowlist/blocklist
- Path allowlist/blocklist
- Workspace isolation per session
- Safe API key handling (keyring support planned)
- Clear warnings before running external agents
- User control over memory scanning
- Redaction of secrets from logs and prompts

### 5.10 Web Dashboard (Optional, Later)

- View sessions and tasks
- View memory
- View subagent runs
- Manage settings, providers, skills
- Not required in Phase 1–6

---

## 6. Non-Goals (Explicit Exclusions)

- GOAT is NOT a web application or browser extension
- GOAT does NOT copy proprietary code from any AI tool
- GOAT does NOT claim integrations that are not working
- GOAT does NOT store user data externally (all data is local by default)

---

## 7. Success Criteria by Phase

| Phase | Success Criteria |
|-------|----------------|
| 0 (Audit) | Full docs, compile status known, honest feature matrix |
| 1 (Core) | Compiles, runs, basic chat with one provider, approval on bash |
| 2 (TUI) | Full TUI with chat, logs, approval prompts |
| 3 (Router) | 3+ providers, fallback chain working |
| 4 (Memory) | Session/project memory, indexer working |
| 5 (Subagents) | Internal subagent framework, one external adapter |
| 6 (Skills) | Skill manifest, loader, MCP support |
| 7 (Voice) | Push-to-talk STT working |
| 8 (Dashboard) | Web dashboard for session/memory/settings view |
