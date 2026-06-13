# GOAT Migration Parity Matrix (Pre-Alpha)

This document audits GOAT against the expectations of users migrating from other popular AI agent tools. It maps user expectations from these tools to GOAT's current status, identifying missing features and blockers for Alpha 1.

## Status Legend
* **Ready**: fully implemented and functioning.
* **Partial**: partially implemented, missing some workflows or UX polish.
* **Missing**: not yet implemented.
* **Intentionally Blocked**: purposefully excluded or restricted for safety reasons.
* **Planned**: on the roadmap for a future phase.

---

## Parity Matrix

### 1. Claude Code / Codex CLI / Gemini CLI (Terminal Assistants)
Expectation: Drop into a terminal, ask a question, and get a fast shell command or code edit back.
* **Inline Shell Execution**: **Intentionally Blocked**. GOAT routes shell commands through `ApprovalGate` and uses specific capabilities to prevent rogue bash loops.
* **Context Awareness (cwd)**: **Ready**. GOAT automatically learns the project via `goat learn` and maintains a `repo-map`.
* **Quick Edits**: **Ready**. Via the `goat` TUI and patches.
* **CLI Chat Loop**: **Ready**. The TUI provides an interactive chat loop.
* **Direct File Edits**: **Partial**. Managed via patches and `ApprovalGate`. Users must approve edits.

### 2. Aider (Repo-Editing Assistants)
Expectation: Connects to git, edits multiple files, commits changes automatically.
* **Git Integration**: **Ready**. GOAT supports patch generation and applies them cleanly.
* **Repo Map (AST-based)**: **Ready**. Implemented in the `repo-map` feature.
* **Multi-file Edits**: **Ready**. Patch system supports complex multi-file diffs.
* **Auto-Commit**: **Intentionally Blocked**. Patches must be reviewed and approved before committing, emphasizing safety.
* **In-terminal Diff Review**: **Ready**. `goat diff` and patch approval TUI.

### 3. Cline / Continue / Cursor / Windsurf (IDE-Integrated Agents)
Expectation: Lives in the IDE, reads open files, highlights code, applies diffs in the editor.
* **VS Code Extension**: **Partial/Planned**. GOAT has a desktop/dashboard roadmap, but native IDE extensions are in early phases.
* **Read Open Files**: **Missing**. GOAT operates primarily as a CLI and relies on the project context rather than IDE state.
  * *Severity*: P2.
  * *Why*: IDE users expect the agent to know what they are looking at.
  * *Phase*: Phase 10 / Post-Alpha.
* **Inline Editor Diffs**: **Partial**. Handled via CLI TUI instead of IDE GUI.
* **MCP Integration**: **Ready**. GOAT supports MCP metadata, auto-start, and capability routing.

### 4. OpenCode / OpenClaw / OpenCalw (Open Source Autonomous Agents)
Expectation: Long-running autonomous loops, shell access, browser access, deep research.
* **Autonomous Loops**: **Partial**. GOAT supports missions, but `ApprovalGate` intentionally interrupts unbounded loops for safety.
* **Shell Access**: **Ready (Gated)**. Executed via `code_execution` and validation recipes.
* **Browser Automation**: **Ready**. `goat browser` handles safe, headless automation workflows.
* **Subagents**: **Ready**. `goat subagents` framework is implemented.

### 5. Hermes / Little Bird (Persistent / Workflow Agents)
Expectation: Long-term memory, customizable workflows, specialized skills.
* **Long-Term Memory**: **Ready**. `goat memory` and the brain database maintain persistent state across sessions.
* **Reusable Workflows (Skills)**: **Ready**. `goat skills` allows recording, curating, and executing repeatable workflows.
* **Scheduled Tasks**: **Ready**. `goat schedule` handles cron-like agent tasks.

---

## Where GOAT is Already Stronger

1. **Local-First Safety & Fail-Closed Design**: Unlike tools that execute arbitrary code with minimal oversight, GOAT runs everything through a strict `ApprovalGate`. If a capability isn't explicitly defined and approved, it fails closed.
2. **Patch & Checkpoint Flow**: Changes are never blindly applied to the filesystem. They are proposed as patches, checked against a diff analyzer, and can be easily rolled back using checkpoints.
3. **Mission Memory & Brain Database**: A robust SQLite-backed brain ensures GOAT remembers context, decisions, and failure patterns across sessions, preventing the "amnesia" common in simple CLI wrappers.
4. **Validation Runner & Deterministic Skills**: Code is validated natively (`goat validate`) before being considered complete. Skills are treated as first-class, guided workflows rather than fragile shell scripts.
5. **Extension Metadata Safety**: MCP servers and third-party tools are declarative capabilities. They don't autostart uninvited and must be prepared (`goat tools prepare <id>`).
6. **Multi-Agent Compatibility**: The framework inherently supports spawning specialized subagents and external adapters, coordinating complex tasks beyond a single LLM context window.

---

## Feature Gaps & Blockers Summary

* **Missing IDE Integration Context** (P2): The lack of IDE state awareness is a gap compared to Cursor/Continue, but acceptable for a CLI-first Alpha.
* **Approval UX Friction** (P1): Frequent `ApprovalGate` prompts might fatigue users coming from Aider/Claude Code. Needs a smooth UX in the TUI (e.g., `goat validate --auto-approve`).
* **Command Discoverability** (P0): With 40+ subcommands, `goat help` can be overwhelming. Needs a focused quickstart for first-time users.
