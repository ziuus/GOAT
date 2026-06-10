# GOAT Feature Parity Review

**Date:** June 2026
**Version:** Phase 4.7

This document provides an honest comparative assessment of GOAT against industry-standard coding agents (OpenCode, Claude Code, Cline, Cursor, etc.).

| Feature Category | GOAT Status | Comparison | Notes |
| :--- | :--- | :--- | :--- |
| **TUI / CLI** | `Working` | On par with Claude Code / Aider. | Highly robust, multi-pane ratatui interface. |
| **Dashboard** | `Working` | Superior to CLI-only tools; similar to OpenHands. | Next.js 15 local web dashboard with full API sync. |
| **Command Palette** | `Working` | On par with Cursor/Linear. | `Ctrl+K` implemented in Dashboard. |
| **Slash Commands** | `Working` | On par with OpenCode/Continue. | Available in both TUI and Dashboard Chat. |
| **Repo Awareness** | `Working` | On par with Aider/Cursor. | Context system loads targeted files into the context window securely. |
| **Editing/Diffing** | `Partial` | Behind Cursor/Windsurf. | Diffs are generated and viewable, but we lack direct "click-to-apply" inline editor integrations (e.g. VS Code extension). |
| **Approval Safety** | `Working` | Best in class. | `ApprovalGate` risk-based model is highly restrictive and transparent, outperforming blind-execution agents. |
| **MCP/Tools** | `Working` | On par with Claude Code/OpenCode. | Full Model Context Protocol foundation is established. |
| **Agents/Subagents** | `Partial` | Behind Antigravity. | Basic routing exists, but autonomous multi-agent swarm delegation is still planned. |
| **External Agents** | `Partial` | Behind specialized frameworks. | Adapters exist, but deep integration is ongoing. |
| **Memory/Skills** | `Working` | On par with OpenCode. | Semantic local DB tracks sessions and learned configurations. |
| **Scheduler/Jobs/Hooks** | `Working` | Superior to most desktop assistants. | Background async tasks are deeply integrated into the daemon. |
| **Browser/Computer Use** | `Missing` | Behind Antigravity/Browser-use. | Not currently implemented. |
| **Git/PR Workflow** | `Partial` | Behind GitHub Copilot Workspace. | Can read git diffs safely, but automated PR generation logic is planned. |
| **Voice Mode** | `Planned` | Behind advanced Jarvis concepts. | Fully scoped for Phase 7. |
| **Desktop/Mobile** | `Planned` | Behind Cursor. | Tauri wrapper is planned for Phase 5. |
| **Cloud/Remote Exec** | `Not planned` | Intentional restriction. | GOAT is strictly local-first by design. |
| **Collaboration** | `Not planned` | Intentional restriction. | Single-user local daemon. |
| **Analytics/Token Tracking** | `Missing` | Behind OpenCode. | Token counting is not yet surfaced to the UI. |

## Verdict
GOAT has achieved a highly competitive feature set for local, secure agentic coding. It excels in safety (`ApprovalGate`) and UI flexibility (TUI + Dashboard). It is currently behind in raw IDE-integration (VSCode/JetBrains extensions) and fully autonomous swarm delegation.
