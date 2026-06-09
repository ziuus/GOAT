# GOAT Internal Subagent Framework (Phase 2.7)

This document outlines the architecture, permissions, and usage of the GOAT Internal Subagent Framework, introduced in Phase 2.7.

## Architecture

GOAT implements a registry-based subagent system. Internal subagents are pre-defined AI personas with specific roles, system prompts, default model profiles, and explicitly allowed tool access. 

They are managed by the `SubagentManager` and `SubagentRegistry`.

### Subagent Kinds
- **Planner:** Breaks down tasks into sequences.
- **Coder:** Writes and proposes code patches.
- **Reviewer:** Reviews plans and patches for correctness.
- **Tester:** Suggests testing strategies and unit tests.
- **Debugger:** Analyzes error logs and tracebacks.
- **Documenter:** Writes documentation and comments.
- **Researcher:** Searches web and project context.
- **Security Auditor:** Looks for vulnerabilities and leaks.
- **UI Designer:** Suggests layouts and component design.
- **Refactorer:** Proposes structural improvements.

### Subagent Profile Context Isolation
Subagents do not receive the full session history by default to conserve tokens and prevent confusion. When a subagent is invoked, it receives:
1. The specific task request.
2. A constrained project summary (`repo-map`).
3. Active memory context (if enabled).
4. The currently active skill (if any).
5. No extraneous tool listings (except those allowed for that subagent).

## Security and Tool Permissions

**Subagents are not fully trusted.**
- Subagents are constrained by the **Tool Registry**.
- Each subagent has a hardcoded list of `allowed_tools`.
- Even if a subagent is allowed to use a tool, if that tool is classified as `Medium` or `High` risk, it **must** pass through the **ApprovalGate** before execution.
- Subagents cannot bypass user configurations regarding memory privacy or XDG isolation.

## Audit Logs

All subagent executions are logged to `~/.local/share/goat/subagent-audit.log`. The log captures:
- Timestamp and session ID
- Subagent name and model used
- Truncated task summary
- Success/failure status
- Redacted output preview (secrets automatically scrubbed)

## Commands

### CLI Commands
- `goat subagents list` - List all internal subagents.
- `goat subagents show <name>` - Show capabilities and profile of a specific subagent.
- `goat subagents audit` - Display the subagent audit log.
- `goat ask-agent <name> "<task>"` - Run a headless subagent turn.

### Slash Commands
- `/subagents` - List all internal subagents.
- `/subagents audit` - Display the subagent audit log.
- `/subagent <name>` - Show subagent details.
- `/ask-agent <name> <task>` - Ask a specific subagent to execute a task.
- `/review` - Shortcut to ask the `reviewer` to review the current patch or plan.
- `/debug` - Shortcut to ask the `debugger` to analyze an error.
- `/test-plan` - Shortcut to ask the `tester` to propose a verification strategy.

## Future Roadmap (Phase 2.8+)
- **External Agent Adapters:** Running full instances of OpenCode, Claude Code, or Hermes as sub-processes.
- **Parallel Orchestration:** Having the `planner` spin up a `coder` and `tester` concurrently.
- **Debate & Consensus:** Allowing subagents to debate solutions before presenting them to the user.
- **Subagent Scratchpads:** Dedicated memory files for subagents to track state across multiple turns.
