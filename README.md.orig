# GOAT — README

**GOAT** (General Omniscient Agentic Tool) is a Rust-first, terminal-first AI agent platform that combines the best features from leading AI agent systems into a single, modular, secure CLI/TUI experience.

**Version:** 0.7.0 — Phase 2.4 (UI/UX Architecture Review + TUI Polish)  
**Status:** Pre-Alpha — compiles, runs, memory/skills/project context active, ApprovalGate + polished TUI, headless mode, multi-frontend architecture planned  

> Do not use in production. This is pre-alpha software. Features listed as `planned` (e.g. Voice Companion/Jarvis Mode, Web Dashboard) are NOT yet implemented and are slated for future development phases.

---

## What GOAT Will Be

- **Universal LLM provider support** — OpenAI, Anthropic (planned), Gemini (planned), Groq, Ollama, OpenRouter
- **Powerful ReAct agent loop** — plan → act → observe with tool execution, session persistence, and approval gates
- **Rich terminal TUI & Headless modes** — built with ratatui; headless for piping
- **Memory / Brain** — SQLite-backed session memory, project indexer, and long-term knowledge store
- **Project awareness** — learns your tech stack, commands, and architecture
- **Subagent orchestration** — internal specialized agents + external agent adapters (OpenCode, Claude Code, etc.)
- **Skills / Plugins** — extensible via TOML skill manifests
- **Voice Companion/Jarvis Mode** — **PLANNED FOR LATER PHASE**. Fully optional, disableable, no microphone listening without permission.
- **Security-first** — approval gates, audit log, secret redaction, sandboxing

See [`docs/GOAT_PRODUCT_SPEC.md`](docs/GOAT_PRODUCT_SPEC.md) for the full product specification.

---

## Current Status

| Component | Status |
|-----------|--------|
| `cargo check/test` | ✅ Passing |
| `cargo run` | ✅ Working (TUI & Headless) |
| Brain (SQLite memory) | ✅ Working |
| MCP STDIO client | ✅ Working |
| Provider: OpenAI | ✅ Working |
| Provider: Groq | ✅ Working |
| Provider: OpenRouter | ✅ Working |
| Provider: Ollama | ✅ Working |
| Provider: Anthropic | ❌ Planned |
| Provider: Gemini | ❌ Planned |
| ReAct agent loop | ✅ Working |
| TUI & Headless modes | ✅ Working |
| Approval gates | ✅ Working |
| Project indexer | ✅ Working |
| Curated Memory System | ✅ Working (USER.md / MEMORY.md) |
| Voice/Jarvis Mode | ❌ Planned (Future Phase) |

See [`docs/GOAT_FEATURE_MATRIX.md`](docs/GOAT_FEATURE_MATRIX.md) for the complete feature status table.

---

## Prerequisites

- **Rust** (stable, edition 2024): https://rustup.rs
- **At least one API key**: OpenAI, Groq, or a compatible OpenAI-API endpoint
- **Linux or macOS** (Windows not tested)

---

## Installation

```bash
git clone https://github.com/ziuus/GOAT.git
cd GOAT
cargo build
```

---

## Configuration

GOAT loads config from `~/.config/goat/goat.toml`. This file is auto-created with defaults on first run.

### Minimal working config:

```toml
[keys]
openai_api_key = "sk-your-key-here"
# groq_api_key = "gsk_your-key-here"
```

### MCP server config (optional):

```toml
[mcp_servers.my_server]
command = "node"
args = ["path/to/mcp-server.js"]

[mcp_servers.my_server.env]
MY_ENV_VAR = "value"
```

### API key fallback:

If `openai_api_key` is not set in `goat.toml`, GOAT will attempt to read API key from:
- `OPENAI_API_KEY` environment variable
- `~/.config/opencode/opencode.json` (OpenCode config fallback)

---

## Running (Once Phase 1 is Complete)

```bash
cargo run
```

### TUI Controls (planned for Phase 1):

| Key | Mode | Action |
|-----|------|--------|
| `i` | Normal | Enter editing mode |
| `Esc` | Editing | Return to normal mode |
| `Enter` | Editing | Submit input to agent |
| `q` | Normal | Quit |
| `l` | Normal | Learn about me (index projects) |
| `c` | Normal | Start configured MCP servers |
| `r` | Normal | Route current input via swarm router |
| `m` | Normal | Show MCP server status |

---

## Brain Database & Memory

GOAT stores sessions, interactions, and indexed file content in a SQLite database.
Additionally, GOAT uses a curated memory system to safely inject context into the LLM system prompt.

**Target Database location:** `~/.local/share/goat/goat_brain.db`  
**User Preferences:** `~/.local/share/goat/USER.md`  
**Memory Notes:** `~/.local/share/goat/MEMORY.md`  

### Memory Usage Guide
- Use `/memory add-user <text>` to add your preferences (e.g. "I prefer Rust code to be heavily documented").
- Use `/memory add-note <text>` to add project-specific notes or long-term facts.
- Use `/memory status` to check your current memory budget.
- Memory files are automatically injected into the agent's context. GOAT respects soft character limits (1500 for USER.md, 4000 for MEMORY.md) to keep contexts manageable.
- **Privacy & Security:** GOAT uses basic secret protection heuristics to prevent API keys or passwords from being accidentally saved to `USER.md` or `MEMORY.md`. 

---

## Logs

Rolling daily log files are written to `./logs/goat.log.YYYY-MM-DD`.

---

## Documentation

| Document | Description |
|----------|-------------|
| [`docs/GOAT_PRODUCT_SPEC.md`](docs/GOAT_PRODUCT_SPEC.md) | Full product vision and feature areas |
| [`docs/GOAT_ARCHITECTURE.md`](docs/GOAT_ARCHITECTURE.md) | Current and target architecture |
| [`docs/GOAT_FEATURE_MATRIX.md`](docs/GOAT_FEATURE_MATRIX.md) | Status of every feature |
| [`docs/GOAT_IMPLEMENTATION_ROADMAP.md`](docs/GOAT_IMPLEMENTATION_ROADMAP.md) | Phased implementation plan |
| [`docs/GOAT_SECURITY_MODEL.md`](docs/GOAT_SECURITY_MODEL.md) | Security model, threats, controls |
| [`docs/GOAT_CODEBASE_AUDIT.md`](docs/GOAT_CODEBASE_AUDIT.md) | Full Phase 0 codebase audit |
| [`CHANGELOG.md`](CHANGELOG.md) | Version history |

---

## Development Roadmap

| Phase | Goal | Status |
|-------|------|--------|
| 0 | Audit + documentation | ✅ Complete |
| 1 | Minimal working core (TUI + approval gates) | ✅ Complete |
| 2 | GOAT Brain Foundation (Memory) | ✅ Complete |
| 3 | Model router + fallback | ✅ Complete |
| 4 | Project awareness | ✅ Complete |
| 5 | Subagent system | 📋 Planned |
| 6 | Skills/plugins + MCP | 📋 Planned |
| 7 | Voice prompting | 📋 Planned |
| 8 | Dashboard + orchestration | 📋 Planned |

---

## Security

GOAT is designed with security-first principles:
- **Approval gates implemented** (Phase 1.1) — bash, write_file, and call_subagent all require interactive confirmation before executing
- Risk levels: `CRITICAL` / `HIGH` / `MEDIUM` assessed per-command with pattern matching
- Session policies: `a` (always allow) / `d` (always deny) per tool for the session lifetime
- Denial forwarded to LLM so it can adapt its plan without crashing the loop
- Secret redaction before displaying command arguments
- Persistent audit log (Phase 2)
- Secret redaction from trace logs (Phase 2)
- Sandbox mode (Phase 6)

**Key bindings when approval is pending:**

| Key | Action |
|-----|--------|
| `y` | Approve once |
| `n` | Deny once |
| `a` | Approve + always allow this tool for the session |
| `d` | Deny + always deny this tool for the session |
| *any other key* | **Denied** (safe default) |

See [`docs/GOAT_SECURITY_MODEL.md`](docs/GOAT_SECURITY_MODEL.md) for the full security model.

---

## Contributing

GOAT is under active development. Contributions will be welcomed after Phase 1 is complete and the project compiles.

**Core rules:**
- No hallucinated features
- Every claimed feature must work
- Documentation must be updated with every change
- No code copied from external projects without license verification

---

## License

To be determined. No license file exists yet. All rights reserved until a license is chosen.

---

## Acknowledgements

GOAT draws design inspiration (not code) from:
OpenCode, Claude Code, Antigravity, Gemini CLI, JCode, Hermes, Little Bird AI, OpenClaw, Codex, Copilot, Cline, Pi.

All inspiration is implemented as clean GOAT-native Rust code.