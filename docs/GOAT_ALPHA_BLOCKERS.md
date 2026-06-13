# GOAT Alpha Blockers Matrix

This document tracks blocking issues that must be resolved prior to the Alpha 1 and Public Alpha releases, categorized by severity based on our Migration Parity Audit.

## Severity Classifications
* **P0**: Must fix before Alpha 1 (internal testing).
* **P1**: Should fix before Public Alpha.
* **P2**: Can be documented as a known limitation.
* **Later**: Post-alpha roadmap.

---

## Blockers

### P0 (Must fix before Alpha 1)

| Feature / Issue | Description | Mitigation / Next Step |
| :--- | :--- | :--- |
| **Command Discoverability** | With 40+ subcommands, `goat help` is overwhelming for new users migrating from simpler tools. | Improve `goat help` or `goat quickstart` to highlight the "golden path" (learn -> mission -> patch -> validate). |
| **Onboarding Friction** | Users dropping into the TUI need to know what to do immediately. | Ensure `docs/GOAT_ALPHA_QUICKSTART.md` is clear and TUI greeting is helpful. |
| **Tool Visibility** | Capabilities must be explicitly prepared before use. If users don't know this, GOAT will feel "broken". | Ensured by the `/tools` dashboard and `goat doctor alpha` (completed in Phase 9.4). |

### P1 (Should fix before Public Alpha)

| Feature / Issue | Description | Mitigation / Next Step |
| :--- | :--- | :--- |
| **Approval Fatigue** | Users migrating from Aider or Claude Code may find the constant `ApprovalGate` prompts annoying for low-risk actions. | Polish the `goat validate --auto-approve` flow and streamline TUI approval UX. |
| **Broken Flows / Missing Examples** | Documentation for creating custom capabilities/skills might be sparse. | Add more detailed examples beyond the basics in `examples/capabilities/`. |

### P2 (Documented Limitations)

| Feature / Issue | Description | Mitigation / Next Step |
| :--- | :--- | :--- |
| **Lack of IDE Context** | GOAT operates in the terminal and doesn't read open files from VS Code/Cursor. | Document that GOAT relies on `repo-map` and project intelligence rather than active IDE editor state. |
| **Subagent Debugging** | Diagnosing failures in nested subagents can be opaque. | Document the usage of `goat memory search` or logs to debug subagents. |

### Later (Post-Alpha)

| Feature / Issue | Description | Mitigation / Next Step |
| :--- | :--- | :--- |
| **Native IDE Extensions** | Full VS Code / IntelliJ integration. | Phase 10 / Roadmap item. |
| **Advanced Capability Scheduling** | Cron-like autonomous capability execution. | Post-alpha roadmap. |
