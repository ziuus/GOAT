# GOAT UI/UX Audit

**Version:** 0.7.0
**Phase:** 2.4 — UI/UX Architecture Review
**Date:** 2026-06-09

---

## Methodology

GOAT's current TUI (`src/ui.rs`, `src/app.rs`) was compared against:
OpenCode, Claude Code, Hermes Agent, Antigravity, Cursor/Windsurf, Cline, Codex CLI, Gemini CLI, Aider, JCode.

---

## Current TUI Architecture

```
┌──────────────────────────────────────────────────────┐
│  Header (1 line): GOAT v0.1 │ provider │ session │ status │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Chat / Log panel (scrollable, fill height)          │
│  Single pane: all output mixed together              │
│                                                      │
├──────────────────────────────────────────────────────┤
│  Input composer (3 lines)                            │
└──────────────────────────────────────────────────────┘
```

Approval overlay renders centered on top of the log panel.

---

## Issue Catalogue

### 1. Visual Design — Score: 4/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 1.1 | Header cramped at narrow widths | HIGH | Fix in Ratatui now |
| 1.2 | Shows "v0.1" hardcoded — should be v0.7.0 | CRITICAL | Fix in Ratatui now |
| 1.3 | Only one visual differentiation (navy background) | HIGH | Fix in Ratatui now |
| 1.4 | No separator lines between message groups | MEDIUM | Fix in Ratatui now |
| 1.5 | Flat palette, no visual hierarchy | MEDIUM | Fix in Phase 3.0 |
| 1.6 | Borders too dim | MEDIUM | Fix in Ratatui now |
| 1.7 | No premium visual identity | CRITICAL | Fix in Phase 3.0 + 4.1 |
| 1.8 | No logo/brand in header | LOW | Fix in Ratatui now |

### 2. Layout — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 2.1 | Single pane mixes chat, tool, errors, help | HIGH | Fix in Phase 3.0 (multi-pane) |
| 2.2 | No sidebar for sessions/skills/project | HIGH | Fix in Phase 3.0 |
| 2.3 | No split diff view | HIGH | Fix in Phase 3.0 / Phase 4.1 |
| 2.4 | Tool output lost in chat stream | MEDIUM | Fix in Phase 3.0 |
| 2.5 | No dedicated status/context sidebar | HIGH | Fix in Phase 3.0 |
| 2.6 | Scroll indicator functional but unpolished | MEDIUM | Fix in Ratatui now |
| 2.7 | Input is single-line only | MEDIUM | Fix in Phase 3.0 |

### 3. Input Experience — Score: 6/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 3.1 | No cursor blinking (Ratatui limitation) | LOW | Future advanced TUI |
| 3.2 | No input history navigation (↑ key) | HIGH | Fix in Ratatui now |
| 3.3 | No slash command autocomplete | HIGH | Fix in Phase 3.0 |
| 3.4 | No paste handling for long text | MEDIUM | Fix in Phase 3.0 |
| 3.5 | Placeholder truncated on narrow terminals | LOW | Fix in Ratatui now |
| 3.6 | No visual feedback when streaming starts | MEDIUM | Fix in Ratatui now |

### 4. Command Palette — Score: 2/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 4.1 | No command palette (Ctrl+K) | CRITICAL | Fix in Phase 3.0 |
| 4.2 | Must memorize all slash commands | HIGH | Fix in Phase 3.0 |
| 4.3 | No fuzzy search for commands | HIGH | Fix in Phase 3.0 |

### 5. Slash Commands — Score: 6/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 5.1 | /help output is unsorted flat list | HIGH | Fix in Ratatui now |
| 5.2 | TUI /help is missing /repo-map, /check, /test, /lint, /format, /patch, /ui | CRITICAL | Fix in Ratatui now |
| 5.3 | Placeholder hint too cryptic for new users | MEDIUM | Fix in Ratatui now |
| 5.4 | No groups in /help | MEDIUM | Fix in Ratatui now |
| 5.5 | Raw [TAG] prefixes look like debugging output | MEDIUM | Fix in Phase 3.0 |

### 6. Status Bar — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 6.1 | 1 line status — all info crammed | HIGH | Fix in Ratatui now |
| 6.2 | Shows "v0.1" hardcoded | CRITICAL | Fix in Ratatui now |
| 6.3 | Active skill not shown | HIGH | Fix in Ratatui now |
| 6.4 | Active project not shown | MEDIUM | Fix in Ratatui now |
| 6.5 | MCP count only shown when > 0 | LOW | Fix in Ratatui now |
| 6.6 | Git branch not shown in TUI | MEDIUM | Fix in Phase 3.0 |
| 6.7 | No live streaming indicator | MEDIUM | Future advanced TUI |

### 7. Session Display — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 7.1 | Session shows raw UUID prefix — should show title | MEDIUM | Fix in Ratatui now |
| 7.2 | No session list accessible from sidebar | HIGH | Fix in Phase 3.0 |
| 7.3 | No visual distinction for resumed vs new session | LOW | Fix in Ratatui now |

### 8. Tool Logs — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 8.1 | Tool output mixed with chat | HIGH | Fix in Phase 3.0 |
| 8.2 | No collapsible tool sections | MEDIUM | Fix in Phase 3.0 |
| 8.3 | Large tool output floods log | MEDIUM | Fix in Ratatui now (truncation) |
| 8.4 | [TOOL] and [AGENT] colors too similar | LOW | Fix in Ratatui now |

### 9. Approval Prompts — Score: 7/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 9.1 | Diff preview not rendered in overlay | HIGH | Fix in Ratatui now |
| 9.2 | Overlay width fixed at 78 chars | MEDIUM | Fix in Phase 3.0 |
| 9.3 | CRITICAL risk badge needs more prominence | MEDIUM | Fix in Ratatui now |
| 9.4 | Approval hotkeys not highlighted | MEDIUM | Fix in Ratatui now |
| 9.5 | No auto-deny timeout | LOW | Fix in Phase 3.0 |

### 10. Diff Preview — Score: 3/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 10.1 | No syntax highlighting for diff | CRITICAL | Fix in Phase 3.0 |
| 10.2 | + lines not green, - lines not red | HIGH | Fix in Ratatui now |
| 10.3 | No accept/reject buttons | HIGH | Fix in Phase 3.0 |
| 10.4 | Large diffs overflow log | MEDIUM | Fix in Ratatui now |

### 11. Repo Map Display — Score: 4/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 11.1 | Output is plain text, no highlighting | HIGH | Fix in Ratatui now |
| 11.2 | No dedicated pane | HIGH | Fix in Phase 3.0 |
| 11.3 | Not auto-displayed at startup | MEDIUM | After Phase 2.4 |
| 11.4 | No tree visualization | MEDIUM | Fix in Phase 3.0 |

### 12. Memory/Skills Display — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 12.1 | Memory status not passively visible | MEDIUM | Fix in Phase 3.0 |
| 12.2 | Active skill easy to forget | HIGH | Fix in Ratatui now (show in header) |
| 12.3 | /skills output unformatted | LOW | Fix in Ratatui now |
| 12.4 | Suspicious skill warning easy to miss | HIGH | Fix in Ratatui now |

### 13. Error States — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 13.1 | Auth errors not actionable | CRITICAL | Fix in Ratatui now |
| 13.2 | Fallback failure not distinguished from timeout | HIGH | Fix in Ratatui now |
| 13.3 | ERROR status not descriptive | MEDIUM | Fix in Ratatui now |
| 13.4 | No retry last message | MEDIUM | Fix in Phase 3.0 |
| 13.5 | Rate limit errors generic | MEDIUM | Fix in Ratatui now |

### 14. Empty State — Score: 5/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 14.1 | Startup has no clear call-to-action | HIGH | Fix in Ratatui now |
| 14.2 | No first-run onboarding flow | HIGH | Fix in Phase 3.0 |
| 14.3 | No example prompts | MEDIUM | Fix in Ratatui now |
| 14.4 | Boot log looks same as agent responses | MEDIUM | Fix in Ratatui now |

### 15. Help/Onboarding — Score: 4/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 15.1 | /help unsorted | HIGH | Fix in Ratatui now |
| 15.2 | No "run goat doctor first" prompt for new users | HIGH | Fix in Ratatui now |
| 15.3 | No in-TUI onboarding wizard | MEDIUM | Fix in Phase 3.0 |
| 15.4 | Auth setup not explained in TUI | CRITICAL | Fix in Ratatui now |

### 16. Keyboard Shortcuts — Score: 6/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 16.1 | Shortcuts only in /help | MEDIUM | Fix in Ratatui now |
| 16.2 | No ↑ for input history | HIGH | Fix in Ratatui now |
| 16.3 | No Ctrl+L to clear | LOW | Fix in Ratatui now |
| 16.4 | Home/End not in /help | LOW | Fix in Ratatui now |

### 17. Color System — Score: 7/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 17.1 | + diff lines not green, - not red | HIGH | Fix in Ratatui now |
| 17.2 | [REPO-MAP], [PROJECT] use DIM fallback | MEDIUM | Fix in Ratatui now |
| 17.3 | [MEMORY], [SKILL], [DEV], [PATCH] use DIM | MEDIUM | Fix in Ratatui now |
| 17.4 | No 256-color fallback | LOW | Future advanced TUI |

### 18. Typography/Spacing — Score: 4/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 18.1 | No blank separators between message groups | HIGH | Fix in Ratatui now |
| 18.2 | All lines same visual weight | MEDIUM | Fix in Phase 3.0 |
| 18.3 | Long lines wrap awkwardly | MEDIUM | Fix in Ratatui now |

### 19. Discoverability — Score: 3/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 19.1 | No command suggestions while typing | HIGH | Fix in Phase 3.0 |
| 19.2 | Slash commands hidden in placeholder | HIGH | Fix in Ratatui now |
| 19.3 | No "what can GOAT do?" at startup | HIGH | Fix in Ratatui now |
| 19.4 | /repo-map, /check, /patch not in TUI /help | CRITICAL | Fix in Ratatui now |

### 20. Overall "Premium Agent" Feeling — Score: 3/10

| # | Issue | Severity | Fix Path |
|---|---|---|---|
| 20.1 | Feels like a debug tool | CRITICAL | Fix in Phase 3.0 + 4.1 |
| 20.2 | No animations/streaming indicators | HIGH | Future advanced TUI |
| 20.3 | No GOAT brand identity | HIGH | Fix in Ratatui now |
| 20.4 | Nothing wow-worthy on first open | CRITICAL | Fix in Phase 3.0 + 4.1 |
| 20.5 | Clearly inferior to OpenCode/Claude Code | CRITICAL | Fix progressively in 3.0, 4.0, 4.1 |

---

## Fix Priority Summary

### Fix NOW — Phase 2.4 (Ratatui Polish)
- Hardcoded "v0.1" → "v0.7.0" (actual version constant)
- Active skill shown in header
- /help updated and grouped: agent, dev, memory, session, system
- + diff lines green, - diff lines red in log
- [REPO-MAP], [PROJECT], [MEMORY], [SKILL], [DEV], [PATCH] log colors
- Input history navigation (↑ key)
- Auth error actionable explanation
- Better startup splash / call-to-action
- Separator lines between user message groups
- /ui command (UI mode info + future plans)
- More prominent CRITICAL risk in approval overlay
- Session title in header (instead of raw UUID)
- /help includes all Phase 2.3 commands

### Fix in Phase 3.0 — Advanced Ratatui TUI (COMPLETED)
- Multi-pane layout (chat | right context | sidebar)
- Command palette (/palette, /command)
- View system with `/view <name>` commands (Tasks, Patches, Repo, Skills, Subagents)
- Diff viewer pane inside Patches view
- Memory/skills panel views
- Shortcut keys for navigation (`Ctrl+1-9`, `Ctrl+P`)

### Fix in Phase 4.0 — Daemon + API
- WebSocket event streaming
- Session API
- Approval request API

### Fix in Phase 4.1 — Web Dashboard
- Monaco/CodeMirror diff viewer
- Session timeline
- Premium glassmorphism UI

### Fix in Phase 5.0 — Tauri Desktop
- System tray, native notifications

### Not Worth Fixing in Ratatui
- Cursor blinking, gradient backgrounds, paste for huge blobs
