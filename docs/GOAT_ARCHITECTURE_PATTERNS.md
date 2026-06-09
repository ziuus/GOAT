# GOAT Architecture Patterns

Reusable architecture patterns extracted from the Phase 2.2 Deep Research Report.

## Multi-Surface Engine (OpenCode, Cursor)
- Build a core `GoatRuntime` that encapsulates LLM, memory, skills, and tools.
- Expose it via a headless server, TUI, and CLI interfaces.

## First-Class Primitives (Claude, Windsurf, CrewAI)
- Separate "Rules" (project instructions), "Memory" (global context), "Skills" (reusable workflow packages), and "Tools" (raw executable functions).
- Do not mix them into a single monolithic prompt blob.

## Approval Gate & Sandbox (Codex, Cline, OpenInterpreter)
- Default to "deny". Explicitly check a session policy for `[always_allow, always_deny, ask]`.
- Enforce risk thresholds (e.g. `rm -rf` is High Risk).

## Subagent Isolation (Kiro, Antigravity)
- Subagents should receive a branched session ID and an isolated context (e.g., using worktrees).
- Subagents should have their own isolated LLM contexts and explicit tool permissions, separate from the master agent.

## Background / Cloud Tasks (Devin, Jules, OpenHands)
- Long-running jobs should write state to a database/audit log continuously.
- Tasks must be resumable.
- Activity feeds must be exportable.

## Repo Map & Context Pruning (Aider)
- Use an AST or token-optimized tree parser to inject only relevant parts of the codebase into the LLM context.

## State Machines & Time Travel (LangGraph, SWE-agent)
- The agent loop (ReAct) should be an observable state machine.
- Enable replays of trajectories for debugging and evaluation.

## MCP Backbone (Claude, Copilot)
- Use Model Context Protocol as the standard for all external tool and data integrations.
