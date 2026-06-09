# GOAT UI Design System

**Version:** 0.7.0
**Phase:** 2.4 — UI/UX Architecture Review
**Date:** 2026-06-09

> This document defines the visual language for all GOAT frontends.
> It covers the current Ratatui TUI and the future web/desktop direction.

---

## Design Principles

1. **Futuristic but minimal** — clean lines, purposeful use of color, no clutter
2. **Elegant dark** — primary dark backgrounds; light elements for emphasis
3. **Readable first** — readability takes priority over aesthetics
4. **Developer-native** — monospace for code, syntax highlighting for diffs
5. **Status is sacred** — the user must always know what GOAT is doing
6. **Trust through transparency** — approvals, risks, and diffs shown clearly
7. **No Tron** — avoid neon-on-black grid aesthetics; aim for glass/aurora instead

---

## 1. Color Palette

### Terminal (Ratatui RGB Colors)

```
Background Levels:
  Base       Rgb(18, 20, 30)   — nearly black, deep navy
  Surface    Rgb(25, 28, 45)   — panels, cards
  Elevated   Rgb(35, 40, 65)   — header, overlays
  Border     Rgb(55, 70, 110)  — panel borders (dim)
  Border Hi  Rgb(80, 110, 200) — focused/active borders

Brand:
  GOAT Teal  Rgb(0, 210, 190)  — primary accent, logo
  GOAT Blue  Rgb(100, 160, 255)— secondary accent

Text Levels:
  Primary    Rgb(220, 230, 255)— main content
  Secondary  Rgb(160, 175, 210)— metadata, labels
  Dim        Rgb(100, 110, 140)— inactive, system messages
  Muted      Rgb(70, 80, 100)  — placeholders, hints
```

### Status Colors

```
  Ready/OK   Rgb(80, 200, 80)  — soft green (not pure #00FF00)
  Warning    Rgb(255, 190, 60) — amber
  Error      Rgb(240, 70, 70)  — bright red
  Thinking   Rgb(255, 200, 50) — golden yellow
  Running    Rgb(100, 180, 255)— sky blue
  Approval   Rgb(255, 160, 40) — deep amber
```

### Message Type Colors (Ratatui)

| Tag | Color | Use |
|---|---|---|
| [YOU] | Rgb(130, 200, 255) | User messages — soft sky blue, BOLD |
| [GOAT] | Rgb(100, 220, 160) | Agent responses — mint green |
| [TOOL] | Rgb(190, 130, 255) | Tool execution — purple |
| [AGENT] | Rgb(80, 150, 255) | Subagent — darker blue |
| [APPROVAL] | Rgb(255, 190, 60) | Approval pending — amber, BOLD |
| [APPROVAL]✓ | Rgb(80, 220, 120) | Approved — green, BOLD |
| [APPROVAL]✗ | Rgb(240, 80, 80) | Denied — red, BOLD |
| [SECURITY] | Rgb(255, 130, 175) | Security notice — pink |
| [ERROR] | Rgb(240, 70, 70) | Error — bright red, BOLD |
| [WARN] | Rgb(255, 190, 60) | Warning — amber |
| [HELP] | Rgb(130, 205, 205) | Help text — teal |
| [STATUS] | Rgb(200, 200, 100) | Status info — yellow |
| [SESSION] | Rgb(200, 200, 100) | Session info — yellow |
| [SYSTEM] | Rgb(130, 130, 150) | System notices — dim grey |
| [TOOLS] | Rgb(190, 130, 255) | Tool list — purple |
| [BRAIN] | Rgb(80, 150, 255) | Memory/brain — blue |
| [MCP] | Rgb(80, 150, 255) | MCP events — blue |
| [MEMORY] | Rgb(130, 200, 220) | Memory operations — cyan |
| [SKILL] | Rgb(160, 230, 170) | Skills — light green |
| [PROJECT] | Rgb(160, 200, 255) | Project context — light blue |
| [REPO-MAP] | Rgb(140, 190, 240) | Repo map — pale blue |
| [DEV] | Rgb(220, 160, 255) | Dev commands — lavender |
| [PATCH] | Rgb(255, 210, 100) | Patch/diff — golden |
| [RESEARCH] | Rgb(100, 220, 210) | Research — teal green |
| [SWARM] | Rgb(80, 150, 255) | Swarm routing — blue |
| Diff + line | Rgb(80, 220, 100) | Added lines — green |
| Diff - line | Rgb(240, 80, 80) | Removed lines — red |
| Diff @@ | Rgb(100, 160, 255) | Hunk headers — blue |

---

## 2. Typography

### Terminal
- **Font:** User's terminal font (not controllable by GOAT)
- **Convention:** Use BOLD sparingly — only for user messages, errors, approvals
- **Width:** Assume 80 cols minimum; handle gracefully at 40 cols
- **Truncation:** Truncate long labels with `…` rather than wrapping header content

### Web Dashboard (Future)
```
Headings:       Inter, 600-700 weight
Body/UI:        Inter, 400 weight
Code/Mono:      JetBrains Mono, 400 weight
Brand logo:     Outfit, 700 weight (if used)
```

---

## 3. Spacing Rules

### Terminal (Ratatui)
- Panels: 1-cell border on all sides
- Header: 1 line, no border
- Input: 3 lines with border
- Minimum content padding: 1 space inside border
- Overlay: centered, minimum 4 cells padding from edges

### Web Dashboard (Future)
- Base unit: 4px
- Spacing scale: 4, 8, 12, 16, 24, 32, 48, 64px
- Panel padding: 16px
- Card radius: 8px
- Border: 1px solid rgba(255,255,255,0.08)

---

## 4. Header/Status Bar Design

### Current (Phase 2.4)
```
 🐐 GOAT v0.7.0 │ profile:coding │ provider:openai/gpt-4o │ skill:my-skill │ ● READY
```

### Phase 3.0 (2-line header, planned)
```
Line 1:  🐐 GOAT v0.7.0    profile: coding    provider: openai/gpt-4o    MCP: 2
Line 2:  session: "Fix auth bug in main.rs"  │  skill: rust-expert  │  ● READY
```

### Status Indicators
```
● READY          — green dot, GOAT waiting for input
◌ THINKING…      — yellow spinning dot, LLM call in progress
⚙ RUNNING        — blue gear, tool executing
⚠ APPROVAL       — amber warning, waiting for user
✕ ERROR          — red cross, last request failed
```

---

## 5. Input Composer Design

### Normal State
```
┌──── Message ────────────────────────────────────────────────┐
│  Ask GOAT anything… (/help for commands · Ctrl+C to quit)   │
└─────────────────────────────────────────────────────────────┘
```

### Active Typing
```
┌──── Message ────────────────────────────────────────────────┐
│  Refactor the auth module to use JWT...█                    │
└─────────────────────────────────────────────────────────────┘
```

### Approval State
```
┌──── ⚠ Action Required ─────────────────────────────────────┐
│  Approval required — [y] approve  [n] deny  [a] always  [d] always deny │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Approval Overlay Design

### Layout
```
┌──── ⚠ APPROVAL REQUIRED ────────────────────────────────────┐
│                                                              │
│  Tool:    bash                                               │
│  Risk:    🔴 CRITICAL — Modifies filesystem                  │
│  Command: rm -rf /tmp/build                                  │
│                                                              │
│  ─── Diff Preview ─────────────────────────────────────────  │
│  + Added:    0 lines                                         │
│  - Removed:  1 file                                          │
│                                                              │
│  [y] approve  [n] deny  [a] always allow  [d] always deny   │
└──────────────────────────────────────────────────────────────┘
```

### Risk Styling
```
CRITICAL risk:  Red border + "🔴 CRITICAL" in bright red BOLD
HIGH risk:      Amber border + "🟡 HIGH" in amber BOLD
MEDIUM risk:    Blue border + "🔵 MEDIUM" in blue
LOW risk:       Green border + "🟢 LOW" in green
```

---

## 7. Diff Preview Design

### Terminal (Log Panel)
```
[PATCH] Modified src/auth.rs — 12 added, 3 removed
[PATCH] + pub fn verify_jwt(token: &str) -> Result<Claims> {
[PATCH] +     let decoded = decode::<Claims>(
[PATCH] +         token, &KEYS.decoding, &Validation::default()
[PATCH] +     )?;
[PATCH] - pub fn verify_token(token: &str) -> bool {
[PATCH] -     // TODO: implement this
[PATCH] -     true
```

- `+` lines → bright green
- `-` lines → bright red
- `@@` hunk headers → blue
- Truncated after 40 lines with "... N more lines" notice

### Web Dashboard (Future, Monaco Editor)
- Full unified diff view with line numbers
- Syntax highlighting for the target language
- Accept / Reject buttons per hunk
- Expand/collapse unchanged sections
- Side-by-side vs. inline toggle

---

## 8. Command Palette Design (Phase 3.0 — Completed)

```
┌──── Command Palette  ───────────────────────────────────────┐
│  > /                                                        │
├─────────────────────────────────────────────────────────────┤
│  /help          Show all commands                           │
│  /status        System status                               │
│  /repo-map      Show repository map                    [RM] │
│  /check         Run project check                      [CK] │
│  /test          Run tests                              [TS] │
│  /memory        View/manage memory                     [MM] │
│  /skills        List skills                            [SK] │
│  /profile       Switch profile                         [PR] │
│  /new           Start new session                      [NW] │
│  /clear         Clear log                              [CL] │
└─────────────────────────────────────────────────────────────┘
```

Activated via `Ctrl+K` or typing `/` at start of input.
Fuzzy search across all commands.

---

## 9. Session Panel Design (Phase 3.0 — Planned)

```
┌─ Sessions ──────────┐
│  ▶ Fix auth bug      │  ← active session, highlighted
│    (8 msgs)          │
│                      │
│  ✓ Refactor API      │
│    (24 msgs)         │
│                      │
│  ✓ Write tests       │
│    (12 msgs)         │
│                      │
│  [+] New Session     │
└──────────────────────┘
```

---

## 10. Future Web/Desktop Visual Language

### Glassmorphism Style
```css
/* Panel style */
background: rgba(20, 22, 35, 0.85);
border: 1px solid rgba(100, 140, 255, 0.12);
border-radius: 12px;
backdrop-filter: blur(12px);
box-shadow: 0 4px 32px rgba(0, 0, 0, 0.4);

/* Active/focused panel */
border: 1px solid rgba(100, 140, 255, 0.35);
box-shadow: 0 0 0 2px rgba(100, 140, 255, 0.15), 0 4px 32px rgba(0,0,0,0.5);
```

### Aurora Color Palette (Web/Desktop)
```
--color-bg:         #0d0f1a   /* deep dark navy */
--color-surface:    #141622   /* card backgrounds */
--color-elevated:   #1c1e30   /* modals, overlays */
--color-border:     rgba(100, 140, 255, 0.12)
--color-border-hi:  rgba(100, 200, 255, 0.30)
--color-text:       #d4deff   /* primary text */
--color-muted:      #6c7a9f   /* secondary text */

/* Brand accents */
--color-teal:       #00d4be   /* GOAT primary accent */
--color-blue:       #6090ff   /* GOAT secondary */
--color-purple:     #a066ff   /* skills/tools */
--color-green:      #50dc7a   /* success/approved */
--color-amber:      #ffb83d   /* warning/approval */
--color-red:        #f04040   /* error/denied */
```

---

## 11. ASCII/Wireframe Sketches

### Main TUI Layout (Current Phase 2.4)
```
╔═════════════════════════════════════════════════════════════╗
║ 🐐 GOAT v0.7.0 │ balanced │ openai:gpt-4o │ ● READY        ║
╠═════════════════════════════════════════════════════════════╣
║ Chat & Logs  [↑↓ scroll | End=bottom | 5 lines above]       ║
║                                                             ║
║ [SYSTEM] Session resumed: "Fix auth bug"                    ║
║ [YOU] Can you fix the JWT verification?                     ║
║ [GOAT] I'll analyze the auth module first...                ║
║ [TOOL] read_file: src/auth.rs (234 lines)                   ║
║ [GOAT] Found the issue — the token expiry check is...       ║
║ [PATCH] Modified src/auth.rs: +12 -3                        ║
║ [APPROVAL] ⚠ write_file: src/auth.rs — MEDIUM risk          ║
║                                                             ║
╠═════════════════════════════════════════════════════════════╣
║ ⚠ Approval — [y] approve  [n] deny  [a] always  [d] deny   ║
╚═════════════════════════════════════════════════════════════╝
```

### Approval Modal
```
            ┌───── ⚠ APPROVAL REQUIRED ──────┐
            │                                 │
            │  Tool:   write_file             │
            │  Risk:   🔵 MEDIUM              │
            │  Target: src/auth.rs            │
            │                                 │
            │  Modified: +12 lines -3 lines   │
            │                                 │
            │  + pub fn verify_jwt(token:...) │
            │  + let decoded = decode::<..    │
            │  - pub fn verify_token(token:.. │
            │  - true // TODO                 │
            │                                 │
            │  [y] approve  [n] deny          │
            │  [a] always   [d] always deny   │
            └─────────────────────────────────┘
```

### Phase 3.0 Multi-Pane TUI (Completed)
```
╔══════════════════════════════════════════════════════════════╗
║ 🐐 GOAT v0.7.0   profile:coding   openai:gpt-4o   ● READY  ║
╠═══════════════╦══════════════════════════════════════════════╣
║ Sessions      ║  Chat                                        ║
║ ─────────────  ║  [YOU] Fix the JWT verification             ║
║ ▶ Fix auth    ║  [GOAT] I'll analyze src/auth.rs first...   ║
║   bug ←       ║  [TOOL] read_file → 234 lines               ║
║ ✓ Refactor   ║  [GOAT] Found the issue — expiry check      ║
║   API         ║──────────────────────────────────────────────║
║               ║  Tool Log                                    ║
║ [+] New       ║  [TOOL] write_file: src/auth.rs             ║
╠═══════════════╬══════════════════════════════════════════════╣
║ skill: rust   ║  Context: jwt-auth  │ branch: fix/auth       ║
╠═══════════════╩══════════════════════════════════════════════╣
║ Message (↑ history · /help for commands)                     ║
╚══════════════════════════════════════════════════════════════╝
```

### Future Web Dashboard Layout
```
┌────────────────────────────────────────────────────────────────┐
│ 🐐 GOAT   Sessions ▼   Skills ▼   Memory ▼        ● READY    │
├───────────┬────────────────────────────────────────────────────┤
│ Sessions  │ Chat                            │ Context          │
│ ──────── │ ─────────────────────────────── │ ──────────────── │
│ ▶ Fix auth│ YOU: Fix the JWT verification  │ Project: GOAT    │
│   bug     │                                │ Branch: fix/auth │
│           │ GOAT: I'll analyze auth.rs...  │ Skill: Rust      │
│ Refactor │ ── diff ─────────────────────── │ Memory: 2 notes  │
│ API       │ + pub fn verify_jwt(...)        │                  │
│           │ - pub fn verify_token(...)      │ Provider: GPT-4o │
│ [+ New]   │                [Accept] [Reject]│ Profile: coding  │
├───────────┴────────────────────────────────┴──────────────────┤
│ Ask GOAT anything…                                   [Send ↵] │
└────────────────────────────────────────────────────────────────┘
```

### Future Voice Companion (Phase 6.0)
```
┌───────────────────────────────────────────────┐
│                                               │
│      🐐 GOAT Voice Companion                 │
│                                               │
│   ┌──────────────────────────────────┐       │
│   │  ▶▶ ═══════════════════════ 0:42 │       │
│   └──────────────────────────────────┘       │
│                                               │
│   🎙 LISTENING...  [soft waveform visual]    │
│                                               │
│   You:  "Fix the auth bug in the API"        │
│   GOAT: "I found the issue — the expiry..."  │
│                                               │
│   [■ Stop]  [📋 Tasks]  [🧠 Memory]  [⚙]    │
│                                               │
└───────────────────────────────────────────────┘
```

---

## Implementation Status

| Element | Current State | Phase 2.4 | Phase 3.0 |
|---|---|---|---|
| Color palette | 10 colors | Extended to 20+ | Refined |
| Header | 1 line, v0.1 | Updated, shows skill | 2-line |
| Log colors | Basic | Full tag coverage | Enhanced |
| Diff colors | ❌ No +/- colors | ✅ Green/red | Full syntax hl |
| Approval overlay | ✅ Functional | Improved styling | Full modal |
| Input | 1 line | + history nav | Multi-line |
| Session sidebar | ❌ None | ❌ Deferred | ✅ Sidebar implemented |
| Command palette | ❌ None | ❌ Deferred | ✅ /palette view |
| Web dashboard | ❌ None | ❌ Deferred | ❌ Phase 4.1 |
| Voice UI | ❌ None | ❌ Deferred | ❌ Phase 6.0 |
