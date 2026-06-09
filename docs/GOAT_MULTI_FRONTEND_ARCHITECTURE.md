# GOAT Multi-Frontend Architecture

**Version:** 0.7.0
**Phase:** 2.4 — UI/UX Architecture Review
**Date:** 2026-06-09

> This document describes the planned multi-surface architecture for GOAT.
> **Only the Rust core, Ratatui TUI, and headless frontend are currently implemented.**
> All other frontends are documented here for planning purposes — NOT implemented.

---

## Architecture Philosophy

GOAT separates **core intelligence** from **UI surface** cleanly:

```
┌──────────────────────────────────────────────────────────────────┐
│                         UI SURFACES                              │
│  TUI (Ratatui)  │  Web Dashboard  │  Desktop (Tauri)  │  Voice  │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                        GOAT CORE (Rust)                          │
│  runtime  │  agent loop  │  tools  │  ApprovalGate  │  memory   │
│  skills  │  repo map  │  project scanner  │  provider router    │
└──────────────────────────────────────────────────────────────────┘
```

**Rust is the brain. Any surface can be the face.**

This allows:
- Fast terminal workflow (Ratatui TUI)
- Automation and CI (headless)
- Beautiful web UI (Next.js dashboard) without rewriting the agent
- Native desktop app (Tauri) without reimplementing the brain
- Voice companion (STT/TTS + UI) without changing the core logic
- Future mobile/IoT surfaces without a full rewrite

---

## 1. Rust Core — Current (IMPLEMENTED)

### Responsibilities
- Agent runtime (GoatRuntime)
- LLM provider router (LlmRouter) + fallback chain
- Tool execution (NativeTools + MCP)
- ApprovalGate (security)
- Memory system (Brain/SQLite, USER.md, MEMORY.md)
- Skills system (SkillManager)
- Repo map scanner (RepoMapScanner)
- Project scanner (ProjectScanner)
- Session management
- Configuration (TOML + XDG paths)

### Key Files
```
src/
├── main.rs          — CLI entry + TUI/headless dispatch
├── runtime.rs       — GoatRuntime bootstrap
├── app.rs           — TUI App state
├── headless.rs      — Headless mode
├── agent.rs         — ReAct agent loop
├── llm.rs           — LLM client + types
├── provider.rs      — Provider abstraction
├── models.rs        — Profile registry + fallback chain
├── tools.rs         — Native tools
├── approval.rs      — ApprovalGate
├── brain.rs         — SQLite memory
├── memory.rs        — Curated memory (USER.md / MEMORY.md)
├── skills.rs        — Skills system
├── repo_map.rs      — Repo map scanner + diff preview
├── project.rs       — Project scanner
├── config.rs        — Config loader
└── paths.rs         — XDG paths + doctor
```

### Design Rules
- Rust core must never contain frontend-specific code
- All dangerous operations gate through ApprovalGate
- No secrets exposed via any public API
- All core types implement Serialize/Deserialize for future API use

---

## 2. TUI Frontend — Ratatui (IMPLEMENTED, BEING POLISHED)

### Technology
- **Ratatui** 0.30+ (Rust)
- **crossterm** for terminal input/output
- Single-binary, no external runtime needed

### Current Layout
```
┌──────────────────────────────────────────────┐
│  Header (1 line): provider│session│skill│status │
├──────────────────────────────────────────────┤
│  Chat/Log Panel (scrollable, mixed output)    │
├──────────────────────────────────────────────┤
│  Input Composer (1 line)                      │
└──────────────────────────────────────────────┘
Approval overlay rendered centered when active.
```

### Phase 3.0 Target Layout (NOT YET IMPLEMENTED)
```
┌─────────────────────────────────────────────────────────────┐
│  Header: GOAT v0.7.0 │ profile │ provider │ status          │
├──────────────────┬──────────────────────────────────────────┤
│  Session Sidebar │  Chat Panel (main conversation)          │
│  (20 cols)       ├──────────────────────────────────────────┤
│  ─────────────   │  Tool Log Panel (collapsible, 8-12 rows) │
│  Session 1       ├──────────────────────────────────────────┤
│  Session 2 ←     │  Context Bar: skill│project│git│memory   │
│  Session 3       ├──────────────────────────────────────────┤
│                  │  Input Composer (1-3 lines)              │
└──────────────────┴──────────────────────────────────────────┘
Approval/diff overlays rendered as modal popups.
```

### Use Cases
- Primary developer workflow
- Local machine terminal use
- SSH remote sessions
- Low-resource environments (no browser)
- Scripting-adjacent interactive use

---

## 3. Headless Frontend — stdin/stdout (IMPLEMENTED)

### Technology
- Pure Rust, no terminal dependencies
- `println!` / `stdin().lock()` based I/O
- Reads from stdin (pipe or interactive terminal without raw mode)
- Writes to stdout in `[TAG] message` format

### Use Cases
- Piping: `echo "question" | goat --headless`
- CI/CD: agent tasks in GitHub Actions / GitLab CI
- Automation: shell scripts calling GOAT for specific tasks
- Background daemon use (future)

### Architecture Notes
- All slash commands work in headless mode (same ApprovalGate)
- Headless can be scripted without modification
- Future: JSON-line output mode (`--output-format=jsonl`)

---

## 4. Future: GOAT Daemon + HTTP/WebSocket API (Phase 4.0)

**Status:** PLANNED — not implemented.

### Technology
- **Rust** (axum or actix-web)
- HTTP REST + WebSocket
- JSON event stream

### Responsibilities
- Expose GOAT runtime events over WebSocket
- Sessions API (list, create, resume)
- Tool call API (trigger, approve, result)
- Approval request API (pending approvals → UI)
- Memory API (read, write, search)
- Skills API (list, activate, deactivate)
- Repo map API (trigger scan, read results)
- Provider status API

### Security Rules
- Daemon API must NEVER expose secrets in response bodies
- Approval endpoint requires session token
- Diff previews use redacted content
- API runs locally only (no public port by default)
- mTLS optional for remote access

### Event Stream Format (Draft)
```json
{"type":"message","role":"assistant","content":"...","session":"uuid"}
{"type":"tool_call","tool":"bash","args":{"command":"ls"},"status":"pending_approval"}
{"type":"approval_request","tool":"bash","risk":"MEDIUM","summary":"..."}
{"type":"approval_decision","decision":"approved","tool":"bash"}
{"type":"status_change","status":"thinking"}
{"type":"error","message":"...","recoverable":true}
```

---

## 5. Future: Web Dashboard (Phase 4.1)

**Status:** PLANNED — not implemented.

### Recommended Technology Stack
```
Next.js 15 (App Router)
├── React 19
├── TypeScript
├── Tailwind CSS + shadcn/ui (or custom glass/minimal components)
├── Framer Motion (animations)
├── Monaco Editor / CodeMirror (diff/code viewing)
├── xterm.js (terminal output panel, optional)
└── WebSocket client (connects to GOAT Daemon)
```

### Why Next.js + React?
- Best-in-class developer tooling ecosystem
- shadcn/ui provides premium, customizable components
- Monaco is the standard for code/diff viewing (same as VS Code)
- Tailwind enables rapid, consistent design iteration
- Server-side rendering for fast initial load
- API routes for BFF (Backend for Frontend) patterns

### UI Features Planned
- Session list + timeline view
- Real-time agent chat panel
- Diff viewer with accept/reject (Monaco)
- Tool log panel (collapsible, filterable)
- Approval request modal
- Memory/skills browser
- Project view (repo map visualization)
- Provider/profile selector
- Settings panel
- Session share/export
- Dark glassmorphism aesthetic

### Design System (Web)
- Color: GOAT Aurora dark palette (see GOAT_UI_DESIGN_SYSTEM.md)
- Typography: Inter + JetBrains Mono
- Glassmorphism panels with subtle borders
- Framer Motion for smooth transitions
- Responsive: desktop-first (1280px+), tablet support

---

## 6. Future: Desktop App (Phase 5.0)

**Status:** PLANNED — not implemented.

### Recommended Technology Stack
```
Tauri 2.x
├── Rust backend (GOAT core reused directly)
├── React/Svelte frontend (same web components as Phase 4.1)
├── System tray integration
├── Native notifications
└── Local file system access via Tauri APIs
```

### Why Tauri?
- Reuses the existing Rust core without rewriting
- Much smaller bundle than Electron
- Native feel on all platforms
- Secure: Rust sandboxes WebView access
- System tray for background agent monitoring
- No need to run a separate daemon process

### UI Features Planned
- Identical to web dashboard UI
- System tray: agent status, quick actions
- Native notifications for approvals
- OS file picker for project selection
- Keyboard shortcut registration

### Security Notes
- Tauri permission model restricts file system access
- GOAT's ApprovalGate remains active
- No network requests bypass GOAT's permission layer

---

## 7. Future: Voice Companion / Jarvis Mode (Phase 6.0)

**Status:** PLANNED — not implemented. Fully optional.

### Design Principles
- **Opt-in only** — disabled by default, must be enabled in config
- **Explicit permission** — always asks for microphone permission
- **Visual indicator** — always shows when mic is active (no silent recording)
- **Privacy-first** — STT can run locally (whisper.cpp) or via API
- **Disableable** — `voice.enabled = false` in goat.toml turns it completely off
- **Clean UI** — NOT "ugly Tron" aesthetic — calm, minimal, premium

### Recommended Technology Stack
```
Voice Frontend (Web/Tauri):
├── WebRTC / getUserMedia (mic capture)
├── Whisper.cpp / OpenAI Whisper API (STT)
├── ElevenLabs / Coqui TTS / edge-tts (TTS, optional)
└── React voice UI component (waveform, transcript, state)

Voice Backend (Rust):
├── GOAT core (unchanged — voice is just another input)
├── Audio chunk processor
└── Session continuity (voice transcript → message history)
```

### UI Vision
```
┌───────────────────────────────────────────────┐
│  🎙 GOAT Voice Companion                      │
│                                               │
│  ●  Listening...  [waveform animation]        │
│                                               │
│  You: "Hey GOAT, what's the status of..."    │
│  GOAT: "The project has 3 pending..."        │
│                                               │
│  [Tasks]   [Memory]   [Project]   [Settings] │
│                                               │
│  🔴  [Stop listening]    ⚙  [Settings]       │
└───────────────────────────────────────────────┘
```

---

## Implementation Timeline

| Phase | Frontend | Status | ETA |
|---|---|---|---|
| 2.4 | Ratatui TUI polish | IN PROGRESS | Now |
| 3.0 | Advanced Ratatui TUI (multi-pane, palette) | PLANNED | Near term |
| 4.0 | Rust Daemon + WebSocket API | PLANNED | Medium term |
| 4.1 | Next.js Web Dashboard | PLANNED | Medium term |
| 5.0 | Tauri Desktop App | PLANNED | Long term |
| 6.0 | Voice Companion (opt-in) | PLANNED | Long term |

---

## Security Guarantees Across All Frontends

1. **ApprovalGate is core** — no frontend can bypass it
2. **Secrets never leave core** — API keys not exposed via any API or UI
3. **Diff previews are redacted** — all frontends see redacted content
4. **Voice is opt-in** — never active without config + permission
5. **Daemon binds localhost by default** — no remote access without explicit config
6. **Desktop app uses Tauri permission model** — restricted file system access
7. **All frontends use same session/approval system** — no parallel auth paths

---

## Why NOT force all UI into Rust/Ratatui?

| Requirement | Ratatui | Next.js | Tauri |
|---|---|---|---|
| Terminal first | ✅ Best | ❌ N/A | ⚠ Possible |
| Rich code/diff display | ⚠ Limited | ✅ Monaco | ✅ Monaco |
| Real-time WebSocket | ⚠ Custom | ✅ Built-in | ✅ Built-in |
| Premium visual design | ⚠ Limited (no gradients, etc.) | ✅ Full CSS | ✅ Full CSS |
| Animations | ⚠ Very limited | ✅ Framer Motion | ✅ Framer Motion |
| System tray | ❌ No | ❌ No | ✅ Yes |
| Voice UI | ❌ No | ✅ WebRTC | ✅ WebRTC |
| Bundle size | ✅ Tiny (single binary) | ⚠ Node.js deps | ✅ Small |
| Cross-platform | ✅ Most terminals | ✅ Browser | ✅ Native |

**Conclusion:** Ratatui excels for terminal-first, keyboard-driven developer workflow.
For premium visual experiences, diff viewers, voice, and dashboards, Next.js/Tauri are
the right tools. Use each where it excels — with the same Rust core powering all of them.
