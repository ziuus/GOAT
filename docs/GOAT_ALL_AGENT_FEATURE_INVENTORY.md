# GOAT All-Agent Feature Inventory

Master inventory of every public feature discovered from the deep research report.

| Agent | Feature | Category | Description | GOAT Priority | GOAT Status | Reuse Notes |
|---|---|---|---|---|---|---|
| OpenCode | TUI/Web/Desktop Multi-Surface | Architecture | Core engine running behind TUI, Desktop, and Web. | Must-have | Partial | Local harness + TUI implemented |
| OpenCode | AGENTS.md rules | Instructions | Directory-scoped rules file. | Must-have | Working | GOAT uses USER.md/MEMORY.md |
| OpenCode | SKILL.md | Skills | Reusable skills loaded on demand. | Must-have | Working | Implemented in Phase 2.1 |
| OpenCode | Subagents via @ mention | Subagents | Subagents invoked explicitly. | Should-have | Planned | Phase 5 |
| OpenCode | GitHub Actions `/opencode` | Git/PR | Agent interaction via PR comments. | Later | Planned | |
| OpenCode | Public share links | Team/Export | Cloud sharing of local sessions. | Later | Planned | |
| Claude Code | Interactive REPL | UI/UX | Terminal chat interface. | Must-have | Working | Implemented |
| Claude Code | Auto-memory with inbox | Memory | Extract memories and review before saving. | Should-have | Planned | GOAT Memory is manual currently |
| Claude Code | Custom Subagents | Subagents | Subagents with independent tool access. | Should-have | Planned | |
| Claude Code | Explicit Allow/Ask/Deny | Security | Permission rules for tools. | Must-have | Working | ApprovalGate implemented |
| Claude Code | HTTP/MCP Tool Hooks | Hooks | Callbacks on specific actions. | Later | Planned | |
| Antigravity | Migration Tooling | Setup | Import extensions and configs from Gemini CLI. | Later | Not planned | |
| Gemini CLI | Policy Engine | Security | Fine-grained tool allow/deny policies. | Must-have | Working | SessionPolicy implemented |
| Gemini CLI | Nightly/Stable Channels | Setup | Release channels for users. | Later | Not planned | |
| Codex | Local/Cloud Handoff | Architecture | Transition tasks between local and cloud. | Later | Planned | |
| Codex | Cloud Automations/Inbox | Cloud | Background cloud tasks and inbox. | Later | Planned | |
| Codex | Computer Use | Browser/OS | First-party web search and computer use. | Should-have | Planned | Phase 2.x |
| Copilot CLI | Tight GitHub native flows | Git/PR | `copilot init`, `login`, `pr`. | Should-have | Planned | |
| Cline | Plan/Act Modes | UI/UX | Explicit split between planning and acting. | Must-have | Working | GOAT uses ApprovalGate before acting |
| Cline | Human-in-the-loop by default | Security | Defaults to safe approval flows. | Must-have | Working | GOAT default behavior |
| Continue | `.continue/checks/` on PR | Git/PR | Repo-controlled checks on PR. | Later | Planned | |
| Aider | Repo Map | Project Context | Whole-repo context mapping. | Must-have | Planned | Phase 2.3 |
| Aider | Auto-lint/test | Workflow | Test after edits and auto-fix. | Must-have | Planned | Phase 2.3 |
| Aider | Watch-files mode | IDE | Reacts to comments in IDE. | Later | Planned | |
| JCode | High-Performance TUI | UI/UX | Fast Rust TUI harness. | Must-have | Partial | Pending `ratatui` UI rewrite |
| Kiro | Specs + Steering | Workflow | Structured specs to guide development. | Should-have | Planned | |
| Kiro | Auto session export | Memory/Export | Automatic session save. | Must-have | Working | SQLite Brain |
| OpenInterpreter | OS Mode | Browser/OS | Visual mouse/keyboard control. | Later | Planned | Phase 2.x |
| OpenHands | Docker Sandbox | Security | Execute code in isolated containers. | Should-have | Planned | Phase 6 |
| OpenHands | Open Source Cloud UI | Cloud | Cloud UI for agent runs. | Later | Planned | |
| SWE-agent | Replayable Trajectories | Debug/Eval | Replay agent action logs. | Later | Planned | |
| Cursor | Local + Cloud Agents | Architecture | IDE with local and cloud components. | Should-have | Planned | |
| Cursor | Team Marketplace | Team | Share rules, skills, plugins internally. | Later | Planned | |
| Windsurf | Cascade / Worktrees | Workflow | Real-time awareness, worktrees. | Should-have | Planned | |
| Devin | Async background runs | Cloud | Autonomous coding in background VMs. | Later | Planned | |
| Jules | Plan Approval + Live Feed | Cloud | Approve plan, watch async feed. | Should-have | Planned | |
| Replit | App Builder / Previews | Workflow | Collaborative previews and app hosting. | Later | Planned | |
| Bolt.new | Browser-native runtime | Architecture | WebContainers backend. | Later | Not planned | |
| Lovable | Ownership/Export | Team/Export | User owns generated code and data. | Must-have | Working | Core GOAT philosophy |
| v0 | Project over chat | Architecture | Project holds deployments and chats. | Should-have | Planned | |
| AutoGPT | Block/Workflow UI | Cloud | UI for creating continuous agent workflows. | Later | Not planned | |
| LangGraph | Interrupts/Time-travel | Architecture | Resumable state machines. | Should-have | Planned | |
| CrewAI | Tools/Skills/Knowledge split | Architecture | Clear separation of abstractions. | Must-have | Working | GOAT mimics this |
| Browser Use | Stealth browser API | Browser/OS | Remote stealth chromium access. | Should-have | Planned | |
| Hermes | Messaging/Voice Gateway | Interface | Agent lives on Discord/Telegram/Voice. | Later | Planned | Phase 6/7 |

## Cross-Agent Feature Map

* **TUI/chat UX**: OpenCode, Claude Code, Cline, Aider, JCode.
* **Provider routing**: OpenCode, Cline, OpenHands.
* **Project context**: Aider Repo Map, OpenCode AGENTS.md, Kiro Specs.
* **Repo editing**: Aider Auto-lint/test, Cline, OpenHands.
* **Memory**: Claude auto-memory, Gemini inbox, Windsurf Memories.
* **Skills**: Claude, OpenCode, Kiro, Windsurf, CrewAI.
* **Subagents**: Claude, Codex, Kiro, Cursor.
* **MCP/plugins**: Claude, Cursor, Windsurf, Copilot, Cline.
* **Browser/computer use**: Browser Use, Codex, Cline, OpenInterpreter.
* **Cloud/background tasks**: Codex, Cursor, Devin, Jules, OpenHands.
* **Git/PR workflows**: OpenCode, Claude, Copilot, OpenHands, v0.
* **Voice/companion mode**: Hermes, OpenInterpreter, Copilot CLI.
* **Security/approval**: Codex, Cursor, Cline, OpenInterpreter, OpenHands.
* **Dashboards**: v0, Cursor, Replit, Lovable.
* **Integrations**: Cursor, Windsurf, Replit.

## GOAT Feature Parity Roadmap

List what GOAT needs to reach:
1. **CLI/TUI coding agent parity**: Phase 2.3 (repo map, diffs, lint loops) and TUI integration.
2. **Memory/skills parity**: Nearly achieved (Phase 2.0/2.1), missing auto-extraction (Phase 2.2).
3. **Subagent/orchestration parity**: Phase 5.
4. **Dashboard/cloud parity**: Phase 8.
5. **Jarvis/voice companion parity**: Phase 7.
6. **Full universal agent platform parity**: Beyond Phase 8.
