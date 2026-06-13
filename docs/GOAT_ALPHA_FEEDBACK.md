# GOAT Alpha 1 Feedback Guide

Welcome to the GOAT Alpha 1! This document outlines what we need from early testers to help harden GOAT into a reliable, migration-grade universal agent.

## What Alpha Testers Should Try

1. **Local Workspaces**: Run `goat` inside an existing local project. Let it explore, inspect your `Cargo.toml`, `package.json`, or `requirements.txt`.
2. **Migrations & Refactors**: Ask GOAT to perform a small refactor or write tests for an existing file.
3. **Approvals**: Watch the `ApprovalGate` catch destructive actions. Try the `validation-fast` profile to see how it reduces prompt fatigue during test-driven loops.
4. **Slash Commands**: Use `/help`, `/tools`, and `/registry` in the TUI to see how GOAT structures its context.

## Reporting Bugs

Please keep all feedback local/manual. We intentionally do **not** use telemetry, remote analytics, or hidden tracking in GOAT. Your privacy and local-first execution are paramount.

When reporting a bug, please collect the following:
1. The exact command or prompt you provided.
2. The output or error shown in the TUI.
3. Relevant lines from the local SQLite brain (if applicable) or the session log output.
4. The GOAT version (`goat --version`).
5. Your OS and terminal emulator.

## Migration Questions for Existing Agent Users

If you are coming from Claude Code, OpenCode, OpenClaw, Hermes, Little Bird, Aider, Cline, or Continue, we want to know:
- **Approval Fatigue**: Does the strict approval gate feel too noisy? Did `validation-fast` help?
- **Speed**: How fast does GOAT boot and execute compared to your previous tool?
- **Visibility**: Does the TUI make it clear *what* the LLM is doing behind the scenes?
- **Correctness**: Were the file modifications applied cleanly without truncation or hallucinated diffs?

## Workspace Compatibility Checklist

Please report if GOAT fails to correctly detect or operate in:
- [ ] Rust workspaces (cargo)
- [ ] Node/JS/TS projects (npm, yarn, pnpm)
- [ ] Python projects (pip, poetry, uv)
- [ ] Unknown or generic directories
- [ ] Very large repositories (should gracefully ignore heavy dirs like `node_modules` or `target`)

## Known Limitations

- **File Editing**: Patch application might occasionally fail on very complex, multi-line refactors in dynamically typed languages.
- **Extensions**: Only basic script-based capabilities are supported in Alpha 1. Full MCP server lifecycle management is still experimental.
- **Auto-Approval**: We do not support a global `--auto-approve` flag. Safety is currently strict by default.

Thank you for testing GOAT!
