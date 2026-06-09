# GOAT UI/UX Reference

Patterns extracted from the Phase 2.2 Deep Research Report to guide GOAT UI/UX design.

## TUI Patterns
- **JCode**: High-performance Rust TUI with multi-session layouts.
- **OpenCode**: Terminal-first TUI with clean session separation and built-in help commands.
- **Aider**: Interactive pair-programmer interface with real-time response streams and diff indicators.

## CLI Patterns
- **Cline**: Rich slash command interface combining commands and subcommands (e.g., `/skills create`).
- **Gemini CLI**: Command registry, reload commands, clear error handling for missing flags.

## IDE Patterns
- **Cursor / Windsurf**: Embedded agent panels, multi-workspace awareness, inline diff/accept/reject.

## Dashboard Patterns
- **Replit / Lovable / v0**: Centralized project view, deployments, team collaboration boards, and activity feeds.

## Voice Companion Patterns
- **Hermes / OpenInterpreter**: Server-client voice split, voice-to-text transcriptions treated as standard prompt inputs.

## Approval UX
- **Jules / Codex / Cline**: Explicit Plan vs Act stages. The plan is presented as a diff or task list, and requires a single Y/N approval before massive code execution.

## Diff UX
- **Aider / Jules**: Show exact line changes inline. Allow rejection of the diff.

## Project Context UX
- **Aider**: Repo map generated and fed as context automatically.
- **Kiro**: Specs and steering files are displayed or linked directly in the agent's status view.

## Session UX
- **JCode / Kiro**: Auto-saving sessions, easy session switching or resumption via `goat sessions`.

## Memory & Skills UX
- **Claude / Windsurf**: `CLAUDE.md`, `AGENTS.md`, and `SKILL.md` are surfaced clearly to the user.
- **Claude**: Inbox model where memories are staged for review before being committed to persistent storage.

## Subagent UX
- **Claude / Cursor**: Visible spawning of a subagent with its own isolated progress bar or logging prefix (e.g. `[SUBAGENT-1]`).

## Command Palette / Slash Command UX
- **Cline / OpenCode**: `/help`, `/undo`, `/redo`, `/share` baked directly into the main prompt handler.

## Onboarding / Setup UX
- **OpenHands / Copilot CLI**: Auth flows integrated directly into the CLI `init` command (e.g., prompting for keys).

## Logs / History / Share / Export UX
- **Jules**: Export a branch or PR at any time during an async task.
- **OpenCode**: Public share links for local sessions.
