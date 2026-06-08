# GOAT — Agent Feature Research & Master Blueprint

**Version:** 1.0  
**Last Updated:** 2026-06-08  
**Status:** Research complete — features classified by status for GOAT roadmap

> This document is the master research reference for GOAT development. Every major AI agent platform is analyzed for features GOAT should learn from, implement, or avoid. Feature parity is the goal; code copying without license verification is prohibited.

---

## Research Methodology

- Official GitHub repos, docs, product pages, and technical writeups consulted
- License status checked for all open-source tools
- Features verified against actual capabilities (not marketing claims)
- GOAT implementation status uses: `working` / `partial` / `planned` / `research` / `blocked` / `not planned`
- Code reuse marked: `yes (license)` / `maybe (check license)` / `no (proprietary)` / `unknown`

---

## 1. OpenCode

**Source:** https://github.com/anomalyco/opencode | https://opencode.ai  
**License:** MIT (open-source)  
**Interface:** TUI (primary), desktop app (beta), IDE extensions  
**Code Reuse:** Yes (MIT) — architecture patterns and concepts can be adapted  

### Core Purpose
Terminal-first AI coding agent with a polished Bubble Tea TUI. Designed for developers who live in the terminal. Provider-agnostic, supports 75+ models.

### Best Features
1. **Smooth animated TUI** built with Bubble Tea (Go) — keyboard-driven, vim-like feel but approachable
2. **Provider agnostic** — OpenAI, Anthropic, Gemini, Groq, Bedrock, Ollama, 75+ models without lock-in
3. **LSP integration** — IDE-like diagnostics loaded automatically into agent context
4. **Multi-session parallel agents** — run multiple agents on the same project simultaneously
5. **Shareable sessions** — export conversation link for team debugging/review
6. **Custom slash commands** — reusable named commands with arguments
7. **Deep repo context** — indexes files, follows imports, uses grep/search tools
8. **GitHub integration** — usable from within GitHub issues/PRs
9. **Desktop app (beta)** — GUI option alongside TUI

### UX Patterns GOAT Should Learn
- Input box always at bottom, always active — no modal "press i to type" pattern
- Clean three-panel layout: status bar → main content → input composer
- Provider/model displayed in status bar at all times
- Slash commands (`/help`, `/clear`, `/model`) as primary feature discovery mechanism
- Smooth transition animations between states
- Syntax-highlighted code in responses
- File paths are clickable/navigable

### Tool Execution Model
- Shell commands, file read/write, search/grep, LSP diagnostics
- Commands show in a dedicated tool panel with expand/collapse
- File edits shown as diffs before applying

### Approval/Security Model
- Shows tool calls with context before executing
- User can review diffs before file writes
- Minimal friction — designed for power users who trust the agent

### Memory/Context Model
- Session-based conversation history
- Repository indexing for code context
- No persistent long-term memory by default

### Provider/Model Routing
- Pluggable provider system — add any OpenAI-compatible endpoint
- Model selection at startup or via slash command

### Code/Architecture Ideas for GOAT
- Always-active bottom input (remove modal mode) ✓
- Status bar with provider/model/session ✓
- Slash command system ✓
- Tool execution log panel separate from chat ✓

---

## 2. Claude Code

**Source:** https://www.anthropic.com/claude-code | https://github.com/anthropics/claude-code  
**License:** Proprietary (Anthropic product)  
**Interface:** CLI (runs in terminal, non-TUI)  
**Code Reuse:** No (proprietary)  

### Core Purpose
Anthropic's official agentic coding CLI. Deep codebase understanding, iterative edits, multi-step task execution. Uses Claude models exclusively.

### Best Features
1. **CLAUDE.md** — project-level persistent instructions file that survives across sessions, acts as "system prompt for the project"
2. **Subagents** — isolated, parallel specialized agents for context isolation
3. **Hooks** — deterministic event-driven scripts (run eslint after every file edit, guaranteed)
4. **MCP integration** — connects to external data sources via Model Context Protocol
5. **Skills** — modular reusable workflow definitions
6. **Context-aware refactoring** — understands entire codebase, not just open file
7. **Plan-first workflow** — proposes a plan before executing destructive changes
8. **Deep safety harness** — ~98% of codebase is infrastructure, not AI logic

### Architecture Insight
- "Harness" design: deterministic infrastructure wraps AI model
- Only ~1.6% of codebase is "AI decision logic"
- 4-layer model: Persistent Memory (CLAUDE.md) → Execution Layer (Skills) → Integration (MCP) → Orchestration (Subagents + Hooks)

### UX Patterns GOAT Should Learn
- CLAUDE.md equivalent: `GOAT.md` project instruction file
- Hooks: post-tool-execution scripts (lint, test, format)
- Skills as modular named workflows
- Plan before act for destructive operations
- Subagent context isolation pattern

### Approval/Security Model
- Approval required before destructive tool execution
- Permission system with tool-level control
- `--allowedTools` flag for auto-approve specific tools

### Memory/Context Model
- CLAUDE.md for persistent project context
- Session-scoped conversation history
- No global long-term memory

### GOAT Implementation
- GOAT.md support: `planned` (Phase 4)
- Hooks: `planned` (Phase 6)
- Skills: `planned` (Phase 6)
- Subagent context isolation: `planned` (Phase 5)

---

## 3. Google Antigravity / Gemini CLI

**Source:** https://github.com/google-gemini/gemini-cli | Antigravity successor  
**License:** Apache 2.0 (original Gemini CLI) — compatible with GOAT  
**Interface:** CLI/TUI, transitioning to Antigravity  
**Code Reuse:** Maybe (Apache 2.0 compatible, but check specific components)  

### Core Purpose
Terminal-based AI agent with Google Gemini models. Huge context window (1M+ tokens), multimodal support, MCP integration, GEMINI.md project context.

### Best Features
1. **Massive context window** — 1M+ token context makes whole-repo analysis practical
2. **GEMINI.md** — per-project persistent context file (same pattern as CLAUDE.md)
3. **MCP integration** — Model Context Protocol as first-class feature
4. **Multimodal** — can accept images, screenshots as context
5. **Google Search grounding** — search results injected into agent context
6. **GitHub Actions integration** — automated code review, issue triage
7. **Apache 2.0 license** — open for reuse with attribution

### Antigravity Successor Features
- Multi-agent orchestration
- Parallel subagents
- Artifact previews
- Task cards and plans
- Browser/screenshot access
- Go-based for fast execution

### UX Patterns GOAT Should Learn
- GEMINI.md / GOAT.md project context pattern
- Google Search as a tool (web search integration)
- Multi-agent parallel task execution visualization
- Artifact preview panels

### GOAT Implementation
- GOAT.md context file: `planned` (Phase 4)
- Web search tool: `planned` (Phase 4)
- Multimodal input: `planned` (Phase 7+)
- Apache 2.0 code reuse: eligible where relevant

---

## 4. OpenAI Codex CLI / Codex Cloud

**Source:** https://github.com/openai/codex (CLI) | cloud.openai.com/codex  
**License:** Apache 2.0 (Codex CLI)  
**Interface:** CLI (local) + Cloud/Web (background agent)  
**Code Reuse:** Maybe (Apache 2.0, check components)  

### Core Purpose
Two distinct products: (1) local CLI coding agent, (2) cloud-based async agentic platform for delegating coding tasks.

### Best Features — Local CLI
1. **Sandboxed execution** — commands run in network-isolated Docker containers
2. **Read/edit/run cycle** — reads files, proposes edits, runs tests
3. **Multi-model support** — OpenAI GPT-4o, o1, o3, etc.
4. **Git-native** — all changes staged as git diffs, easy revert

### Best Features — Cloud Platform
1. **Async task delegation** — assign task, agent works in background
2. **Parallel cloud agents** — multiple agents on different branches simultaneously
3. **PR creation** — agent creates PRs for review
4. **Sandboxed VMs** — isolated cloud execution environment
5. **Task dashboard** — monitor agent progress via web UI
6. **Issue-to-PR flow** — connect GitHub issue to automatic implementation

### UX Patterns GOAT Should Learn
- Sandbox mode as a first-class feature flag (`--sandbox`)
- Async task delegation with status updates
- PR/branch workflow integration
- Agent team management UI concept

### GOAT Implementation
- Sandbox mode: `planned` (Phase 6)
- Background task mode: `planned` (Phase 8)
- Git-native diff workflow: `planned` (Phase 2)
- PR creation: `planned` (Phase 8)

---

## 5. GitHub Copilot Coding Agent

**Source:** https://github.com/features/copilot  
**License:** Proprietary (GitHub/Microsoft)  
**Interface:** GitHub.com + CLI + IDE  
**Code Reuse:** No (proprietary)  

### Core Purpose
Cloud-based agentic coding assistant tightly integrated with GitHub. Assigns issues, creates branches, implements code, opens PRs.

### Best Features
1. **Issue-to-PR workflow** — assign a GitHub issue to Copilot, get a PR back
2. **GitHub Actions ephemeral environments** — runs in isolated, secure cloud VMs
3. **Implementation plan** — proposes a plan before coding
4. **Draft PR with CI integration** — creates draft PR, shows CI/CD results
5. **Agents tab in repos** — centralized dashboard for all agent sessions
6. **Custom agents** — define specialized agents for specific task types
7. **GitHub status/check integration** — agent respects required status checks

### UX Patterns GOAT Should Learn
- "Assign issue to agent" delegation model
- Agents tab / dashboard concept
- Draft PR workflow (agent produces reviewable output)
- Implementation plan surface before executing

### GOAT Implementation
- GitHub issue integration: `planned` (Phase 8)
- Draft PR workflow: `planned` (Phase 8)
- Implementation plan before execution: `planned` (Phase 3)

---

## 6. Aider

**Source:** https://github.com/Aider-AI/aider  
**License:** Apache 2.0  
**Interface:** CLI (terminal, minimal UI)  
**Code Reuse:** Yes (Apache 2.0) — patterns and approaches can be adapted  

### Core Purpose
Terminal pair-programming tool with 40K+ GitHub stars. Git-native edits, repo map, multi-file editing, atomic commits.

### Best Features
1. **Repo map via Tree-sitter** — structured index of all classes, functions, symbols across the codebase
2. **Atomic git commits** — every AI change is automatically committed; easy `git revert` for any edit
3. **Multi-file editing** — understands cross-file dependencies, edits multiple files per task
4. **Architect/Editor dual model** — one LLM plans, another LLM implements (better for complex tasks)
5. **LiteLLM integration** — supports 100+ providers via LiteLLM
6. **Voice-to-code** — hands-free coding input
7. **Auto lint/test** — runs linter/tests after changes, self-corrects on failure
8. **Image/URL context** — accept screenshots or web pages as context
9. **Prompt caching** — reduces API costs on large codebases

### UX Patterns GOAT Should Learn
- Repo map concept for codebase awareness
- Git-native workflow (every edit is a commit)
- Architect/Editor split for complex tasks
- Auto-run linter/tests after tool execution
- Minimal friction CLI — just start and type

### Tool Execution Model
- Edit files via diff/patch format applied to filesystem
- Run shell commands (linter, tests)
- No approval gates by default — trusts the user

### Memory/Context Model
- Repo map for structural awareness
- No persistent memory between sessions
- `/add` command to include specific files in context

### GOAT Implementation
- Repo map: `planned` (Phase 4)
- Git-aware edits / auto-commit: `planned` (Phase 4)
- Architect/Editor dual-model: `planned` (Phase 5)
- Auto lint/test: `planned` (Phase 4)

---

## 7. Cline

**Source:** https://github.com/cline/cline  
**License:** Apache 2.0  
**Interface:** IDE Extension (VS Code, JetBrains)  
**Code Reuse:** Yes (Apache 2.0) — architecture patterns eligible  

### Core Purpose
Human-in-the-loop IDE agent. Plan/Act mode separation. Every tool use requires explicit approval. MCP support.

### Best Features
1. **Plan mode / Act mode separation** — planning phase restricted from making changes; act phase can execute
2. **Per-tool approval** — every file edit, shell command, browser action requires explicit user approval
3. **Diff-before-apply** — shows file diff before writing; accept/reject
4. **Checkpoints** — one-click undo for any change
5. **Auto-approve categories** — whitelist specific tool types for non-interactive use
6. **.clinerules** — repo-level rules file for agent behavior
7. **MCP integration** — extensible via Model Context Protocol
8. **Native tool calling** — parallel file reads, reliable execution
9. **Browser use** — Playwright-based browser control

### UX Patterns GOAT Should Learn
- **Plan mode vs Act mode** is the best pattern for safety without constant interruption
- Diff-before-apply UX before any write_file
- Checkpoints allow fearless agent use
- .clinerules equivalent for GOAT: GOAT.md

### Approval/Security Model
- Granular per-tool approval settings
- Auto-approve toggle per tool category
- Diff review before write
- All actions logged

### GOAT Implementation
- Plan mode: `planned` (Phase 3)
- Diff-before-write: `planned` (Phase 2)
- Checkpoints/undo: `planned` (Phase 4)
- Auto-approve categories: `partial` (session policy in ApprovalGate)
- .clinerules equivalent: `planned` (Phase 4)

---

## 8. Continue

**Source:** https://github.com/continuedev/continue  
**License:** Apache 2.0  
**Interface:** IDE Extension (VS Code, JetBrains)  
**Code Reuse:** Yes (Apache 2.0)  

### Core Purpose
IDE-integrated AI assistant with deep customization. Repo-defined AI checks as CI/CD-like automated rules.

### Best Features
1. **Custom AI checks** — define checks in markdown, run as CI/CD pipeline validation
2. **Source-controlled AI rules** — store AI behavior config in the repo
3. **PR status integration** — AI checks as GitHub PR status checks
4. **Model customization** — swap any model for any task (chat vs. autocomplete vs. edit)
5. **Context providers** — explicit context sources (file, docs, web, clipboard, terminal)
6. **Slash commands** — slash-based command palette
7. **IDE chat/edit/autocomplete** — full IDE assistant flow

### UX Patterns GOAT Should Learn
- Context providers as explicit named sources user can add
- Source-controlled config for repeatable AI behavior
- Slash commands as feature discovery mechanism

### GOAT Implementation
- Context providers: `planned` (Phase 4)
- Source-controlled AI config: `planned` (Phase 4)
- Slash commands: `planned` (Phase 1.2 — current task)

---

## 9. GitHub Copilot CLI

**Source:** https://github.com/cli/cli (gh) + Copilot extension  
**License:** MIT (gh) + Proprietary (Copilot)  
**Interface:** CLI  
**Code Reuse:** No for Copilot portion (proprietary)  

### Best Features
1. **Refined terminal UI** — full-screen alt-screen TUI with mouse and scrolling
2. **Multi-repo context** — `/add-dir` bridges multiple project directories
3. **`/plan` command** — explicit task planning before execution
4. **`/review` command** — multi-model code review in terminal
5. **Context awareness** — inherits VS Code project context when available

### GOAT Implementation
- `/plan` slash command: `planned` (Phase 3)
- `/review` slash command: `planned` (Phase 5)
- Multi-dir context: `planned` (Phase 4)

---

## 10. Devin (Cognition AI)

**Source:** https://www.cognition.ai/blog/introducing-devin  
**License:** Proprietary (SaaS)  
**Interface:** Web + Slack integration  
**Code Reuse:** No (proprietary)  

### Core Purpose
First "autonomous software engineer" — long-running cloud agent that independently plans, codes, debugs, and deploys.

### Best Features
1. **Long-running autonomous tasks** — can work for hours without interruption
2. **Full dev environment** — shell, browser, editor in a sandboxed cloud VM
3. **Task planning** — explicit plan surface before execution
4. **Self-debugging** — runs tests, reads errors, fixes bugs autonomously
5. **Progress updates** — real-time progress visible to human reviewer
6. **PR creation** — produces ready-to-review PRs
7. **Resumable tasks** — pause and resume long tasks

### GOAT Implementation
- Long-running tasks: `planned` (Phase 8)
- Self-debugging loop: `partial` (ReAct loop runs up to 10 iterations)
- Progress display: `partial` (current_task field)
- Resumable tasks: `planned` (Phase 4)

---

## 11. Google Jules

**Source:** https://jules.google  
**License:** Proprietary (Google)  
**Interface:** Web + GitHub integration  
**Code Reuse:** No (proprietary)  

### Core Purpose
Async coding agent. Assign task → Google Cloud VM clones repo → agent implements → creates PR.

### Best Features
1. **Async-first design** — designed to run in background, not interactively
2. **Secure Google Cloud VM** — isolated execution environment
3. **Test execution** — runs tests as part of implementation
4. **PR creation** — produces PR for human review

### GOAT Implementation
- Background task mode: `planned` (Phase 8)
- Cloud VM sandbox: `not planned` (local tool, no cloud VMs)

---

## 12. Cursor

**Source:** https://www.cursor.com  
**License:** Proprietary (Anysphere)  
**Interface:** Desktop IDE  
**Code Reuse:** No (proprietary)  

### Core Purpose
AI-first IDE fork of VS Code. Inline edits, composer (multi-file agent), codebase indexing.

### Best Features
1. **Composer** — multi-file agentic editing mode; propose and apply changes across files
2. **Inline edits** — cursor shows inline diff suggestions in the editor
3. **Codebase indexing** — semantic search across entire repository
4. **Tab-complete** — next-token and next-line completions
5. **Agent mode** — autonomous task execution with tool use
6. **Apply/Accept/Reject** — clear diff UX for every proposed change

### UX Patterns GOAT Should Learn
- Apply/Accept/Reject diff UX for file writes
- Codebase semantic search as a tool
- Multi-file change proposal before applying

### GOAT Implementation
- Accept/Reject diff for write_file: `planned` (Phase 2)
- Semantic codebase search: `planned` (Phase 4)
- Multi-file change proposals: `planned` (Phase 5)

---

## 13. Windsurf / Cascade

**Source:** https://windsurf.io  
**License:** Proprietary (Codeium)  
**Interface:** Desktop IDE  
**Code Reuse:** No (proprietary)  

### Core Purpose
AI-first IDE with "Cascade" agentic flow. Deep multi-step reasoning and execution for complex coding tasks.

### Best Features
1. **Cascade flow** — multi-step agent that reasons before acting, shows thought process
2. **Deep context** — reads full file tree, understands project structure
3. **Flow actions** — explicit list of planned actions before executing
4. **Code with context** — every change grounded in full project understanding

### GOAT Implementation
- Cascade/flow actions (show planned steps): `planned` (Phase 3)

---

## 14. Hermes Agent

**Source:** Internal reference (Zius's personal Hermes agent system)  
**License:** Personal project  
**Interface:** Messaging gateways (Telegram, WhatsApp, Slack, Discord, SMS, email)  
**Code Reuse:** Yes (personal project)  

### Core Purpose
Always-running personal AI agent. Accessible via any messaging platform. Persistent memory, natural-language scheduling, USER.md/MEMORY.md system.

### Best Features
1. **Messaging gateways** — same agent session across Telegram, WhatsApp, Slack, Discord, Signal, SMS, email, Teams
2. **Agent can message the user first** — proactive notifications, reminders, results
3. **Natural-language scheduler** — "remind me in 3 hours" works natively
4. **USER.md** — persistent file of user preferences, name, timezone, communication style
5. **MEMORY.md** — persistent file of environment/project lessons learned
6. **SQLite FTS recall archive** — full-text searchable interaction history
7. **Memory budget and consolidation** — controlled memory size, automatic summarization
8. **Skill markdown files** — extensible via markdown-defined skills
9. **Progressive skill disclosure** — agent learns new skills as needed
10. **Self-written skills** — agent can write its own new skills
11. **Local-first memory** — all data on user's machine
12. **24/7 deployment mode** — always running, not just on-demand

### GOAT Implementation
- USER.md / MEMORY.md: `planned` (Phase 4)
- Messaging gateways: `planned` (Phase 8+)
- Natural-language scheduler: `planned` (Phase 8)
- Skill markdown files: `planned` (Phase 6)
- FTS recall: `planned` (Phase 4)
- Memory consolidation: `planned` (Phase 4)
- Self-written skills: `research`

---

## 15. JCode

**Source:** Internal reference  
**License:** Internal/personal  
**Interface:** CLI/TUI (Rust-native)  
**Code Reuse:** Yes (personal project)  

### Core Purpose
Fast Rust-native AI agent harness. High-performance local execution, OpenAI-compatible provider support, vision-capable.

### Best Features
1. **Rust-native speed** — compiled binary, no interpreter overhead
2. **Vision support** — image context for local/multimodal models
3. **Local/OpenAI-compatible providers** — flexible provider config
4. **Efficient terminal workflow** — minimal overhead

### GOAT Implementation
- Rust-native speed: `working` (GOAT is Rust-native)
- Vision support: `planned` (Phase 7)
- Local provider: `planned` (Phase 3)

---

## 16. Little Bird AI

**Source:** Research needed (limited public information)  
**License:** Unknown  
**Interface:** Unknown  
**Code Reuse:** Unknown  

### Core Purpose
User-awareness agent. Tracks user work, projects, past activity. Provides contextual, personalized assistance based on understanding what the user has been doing.

### Key Concept
- Personal work graph
- Project/activity understanding
- Contextual recall
- Personalized assistance based on history

### GOAT Implementation
- Project/system learning: `partial` (learn_about_me + file indexer)
- User activity awareness: `planned` (Phase 4)
- Work graph: `planned` (Phase 8)

---

## 17. Pi (Inflection AI)

**Source:** https://pi.ai  
**License:** Proprietary  
**Interface:** Web, mobile, voice  
**Code Reuse:** No (proprietary)  

### Core Purpose
Personal AI assistant with exceptional conversational quality, emotional intelligence, and human-like continuity.

### Best Features
1. **Conversational quality** — extremely natural, warm, friendly responses
2. **Emotional tone awareness** — adapts tone to user's state
3. **Voice interaction** — natural speech input/output
4. **Human-like continuity** — remembers context, refers back to past discussions

### GOAT Implementation
- Conversational quality: dependent on chosen model
- Voice: `planned` (Phase 7)
- Emotional tone: `not planned`

---

## 18. OpenClaw

**Source:** Research needed  
**License:** Unknown  
**Interface:** Web dashboard + integrations  
**Code Reuse:** Unknown  

### Core Purpose
Agent platform with web dashboard, broad integrations, skills, custom agents, subagent orchestration, provider flexibility, automation workflows.

### GOAT Implementation
- Web dashboard: `planned` (Phase 8)
- Integrations: `planned` (Phase 6+)
- Skills: `planned` (Phase 6)

---

## 19. GitHub Copilot (IDE)

**Source:** https://github.com/features/copilot  
**License:** Proprietary  
**Interface:** IDE extension  
**Code Reuse:** No  

### Best UX Ideas
- Inline autocomplete as you type
- Tab to accept suggestion
- Ghost text for completions
- Chat panel alongside editor
- @workspace context reference
- `/explain`, `/fix`, `/tests` slash commands

### GOAT Implementation
- Slash commands: `planned` (Phase 1.2)
- Inline autocomplete: `not planned` (TUI, not IDE)

---

## Master Feature Blueprint for GOAT

### Priority Order

| Priority | Feature Family | Phase | Status |
|---------|----------------|-------|--------|
| 🔴 P0 | Always-active input (no modal) | 1.2 | `partial` |
| 🔴 P0 | Status bar (provider/model/session) | 1.2 | `partial` |
| 🔴 P0 | Ctrl+C exit | 1.2 | `partial` |
| 🔴 P0 | Slash command dispatcher | 1.2 | `planned` |
| 🔴 P0 | Scrollable message log | 1.2 | `planned` |
| 🟠 P1 | Config file path improvements | 1.3 | `planned` |
| 🟠 P1 | UUID session IDs | 1.3 | `planned` |
| 🟠 P1 | XDG data dir for brain DB | 1.3 | `planned` |
| 🟠 P1 | Multiple providers (Anthropic, Gemini) | 3 | `planned` |
| 🟠 P1 | Model fallback chain | 3 | `planned` |
| 🟠 P1 | GOAT.md project context file | 4 | `planned` |
| 🟠 P1 | Repo map / codebase indexing | 4 | `planned` |
| 🟡 P2 | Diff viewer before write_file | 2 | `planned` |
| 🟡 P2 | Plan mode / Act mode | 3 | `planned` |
| 🟡 P2 | Git-native workflow (auto-commit) | 4 | `planned` |
| 🟡 P2 | USER.md / MEMORY.md | 4 | `planned` |
| 🟡 P2 | Web search tool | 4 | `planned` |
| 🟢 P3 | Internal subagents (Coder, Reviewer) | 5 | `planned` |
| 🟢 P3 | External agent adapters | 5 | `planned` |
| 🟢 P3 | Skills/plugins (TOML manifest) | 6 | `planned` |
| 🟢 P3 | MCP server browser automation | 6 | `planned` |
| 🟢 P3 | Voice prompting (push-to-talk) | 7 | `planned` |
| 🔵 P4 | Web dashboard | 8 | `planned` |
| 🔵 P4 | Messaging gateways | 8+ | `planned` |
| 🔵 P4 | PR/branch automation | 8 | `planned` |
| 🔵 P4 | Cloud/background task delegation | 8 | `planned` |

### Slash Command Catalog (planned for Phase 1.2-3)

| Command | Description | Priority |
|---------|-------------|---------|
| `/help` | Show all commands | P0 |
| `/clear` | Clear chat history | P0 |
| `/model [name]` | Switch model | P1 |
| `/status` | Show system status | P0 |
| `/mcp` | Start MCP servers | P0 |
| `/learn` | Index project files | P0 |
| `/route` | Show current swarm route | P0 |
| `/plan` | Show task plan before executing | P2 |
| `/sessions` | List/switch sessions | P2 |
| `/memory` | Show/search memory | P3 |
| `/tools` | List available tools | P1 |
| `/export` | Export session | P3 |
| `/sandbox` | Toggle sandbox mode | P2 |
| `/review` | Review recent changes | P3 |
| `/undo` | Undo last change | P2 |

### GOAT Architecture Principles (informed by research)

1. **Always-active input** — like OpenCode, Gemini CLI, Codex CLI: no modal "press i to type"
2. **Harness over AI logic** — like Claude Code: 98% infrastructure, 2% AI decisions
3. **GOAT.md project context** — like CLAUDE.md, GEMINI.md: project-level persistent instructions
4. **Plan before act for destructive ops** — like Cline Plan mode, Windsurf Cascade
5. **Diff before write** — like Cursor, Cline: show diff before applying file edits
6. **Git-native** — like Aider: every edit as a git commit, easy revert
7. **Repo map** — like Aider: tree-sitter based structural index for codebase awareness
8. **Dual-model routing** — like Aider: Architect model for planning, Editor model for execution
9. **Self-correcting loops** — like Aider: run linter/tests after edits, auto-fix errors
10. **Slash commands** — like OpenCode, Copilot CLI, Continue: feature discovery via `/help`
11. **Local-first memory** — like Hermes: all data on user's machine, no cloud sync
12. **MCP-first extensibility** — like Claude Code, Cline, Gemini CLI: MCP as plugin protocol
13. **Skills as markdown** — like Claude Code, Hermes: modular named workflow files
14. **Human-in-the-loop** — like Cline: approval gates for all destructive operations
15. **24/7 deployment option** — like Hermes: background service mode
16. **Messaging gateways** — like Hermes: access GOAT via Telegram, Slack, etc.

---

## License Compatibility Summary

| Tool | License | GOAT Can Reuse Code? |
|------|---------|----------------------|
| OpenCode | MIT | ✅ Yes |
| Aider | Apache 2.0 | ✅ Yes |
| Cline | Apache 2.0 | ✅ Yes |
| Continue | Apache 2.0 | ✅ Yes |
| Gemini CLI | Apache 2.0 | ✅ Yes (check components) |
| Codex CLI | Apache 2.0 | ✅ Yes (check components) |
| Claude Code | Proprietary | ❌ No |
| GitHub Copilot | Proprietary | ❌ No |
| Cursor | Proprietary | ❌ No |
| Windsurf | Proprietary | ❌ No |
| Devin | Proprietary | ❌ No |
| Jules | Proprietary | ❌ No |
| Pi | Proprietary | ❌ No |
| OpenClaw | Unknown | ⚠️ Research needed |
| Little Bird AI | Unknown | ⚠️ Research needed |
| Hermes | Personal (Zius) | ✅ Yes |
| JCode | Personal (Zius) | ✅ Yes |

> **Note:** Apache 2.0 and MIT code can be incorporated into GOAT. Even with compatible licenses, GOAT prefers clean native Rust implementations over direct code transplantation. Licenses allow reuse of architecture patterns, algorithms, and limited code snippets with proper attribution.
