# GOAT Feature Backlog from References

Prioritized backlog extracted from the Phase 2.2 Deep Research Report.

## Must-Have (P0)

| Feature | Source Agent | Why GOAT needs it | Proposed Implementation | Dependencies | Suggested Phase | Risk/Complexity |
|---|---|---|---|---|---|---|
| **Repo Map** | Aider | Context management for large codebases. | Add a tool to `project.rs` that generates a token-optimized tree/map of the project. | ProjectScanner | Phase 2.3 | Low |
| **Auto Lint/Test Loop** | Aider | Prevents agent from committing broken code. | Post-edit hook to run `cargo check` or `npm test` before continuing. | Configurable project commands | Phase 2.3 | Medium |
| **Diff-before-write UX** | Jules, Aider | User trust and safety. | Intercept `write_file` calls, show a ratatui diff, ask approval. | ApprovalGate, TUI | Phase 2.3 | Medium |
| **Subagent Thread Isolation** | Cline, Kiro | Parallel processing without context pollution. | Spawn a new `Brain` session ID and `headless::run` sub-process. | Multi-agent framework | Phase 2.5 | High |
| **Multi-Surface Engine** | OpenCode, Cursor | Prevent lock-in to TUI only. | Expose `GoatRuntime` over an HTTP or WebSockets API. | Runtime stabilization | Phase 4.0 | High |

## Should-Have (P1)

| Feature | Source Agent | Why GOAT needs it | Proposed Implementation | Dependencies | Suggested Phase | Risk/Complexity |
|---|---|---|---|---|---|---|
| **Background/Cloud Tasks** | Codex, Devin | Long-running tasks. | Daemonize `headless` mode and tail logs to a file/DB. | Daemon manager | Phase 2.7 | High |
| **Browser/Computer Use** | Browser Use, OpenInterpreter | Testing web apps and doing research. | MCP server adapter or native playwright/puppeteer integration. | MCP stability | Phase 2.4 | Medium |
| **GitHub PR Flow** | Copilot, v0 | Integration with dev team workflows. | Native tool for `git checkout -b`, `git commit`, `gh pr create`. | Git tools | Phase 2.4 | Low |
| **Skill Curator** | Claude, Hermes | Self-improving skills based on session success. | After session ends, LLM reviews and extracts/updates SKILL.md. | Skills system | Phase 2.2 | High |
| **Specs + Steering** | Kiro | Structured goal-oriented coding. | Parse `SPEC.md` and track completion checkboxes. | Project scanner | Phase 2.4 | Low |

## Later (P2)

| Feature | Source Agent | Why GOAT needs it | Proposed Implementation | Dependencies | Suggested Phase | Risk/Complexity |
|---|---|---|---|---|---|---|
| **Voice/Messaging Transports** | Hermes, OpenInterpreter | Ubiquitous access. | Discord/Telegram bot wrappers calling GOAT API. | HTTP API | Phase 6.0 | Medium |
| **Team Package Registry** | Cursor | Sharing skills and rules across teams. | Remote registry server to pull `SKILL.md` packages. | Skills format | Phase 4.0 | Medium |
| **Enterprise Audit** | Lovable, OpenHands | Organizational compliance. | Export `brain.db` to SIEM or centralized logging. | Database | Phase 4.0 | Low |

## Experimental / Not Planned

| Feature | Source Agent | Why Not Planned |
|---|---|---|
| **Browser-native WebContainers runtime** | Bolt.new | GOAT is focused on local developer environments, not in-browser VMs. |
| **Low-code block/workflow UI** | AutoGPT | Too complex to build and maintain in v1; prefer Markdown and CLI configuration. |
| **Nightly/Stable Release Channels** | Gemini CLI | Not necessary until wide user adoption. |
