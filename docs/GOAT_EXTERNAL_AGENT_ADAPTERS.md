# GOAT External Agent Adapters (Phase 2.8)

## Overview
Phase 2.8 introduces the External Agent Adapter Framework to the GOAT project. This framework is responsible for detecting, managing, and orchestrating external autonomous coding agents (like OpenCode, Aider, Claude Code, Cline, etc.) directly from within the GOAT environment. 

GOAT remains a "Rust-first, terminal-first" agent. The primary loop, security, context injection, memory retrieval, and approval flows occur entirely within the Rust runtime. External agents are treated as "heavy tools" that can be delegated tasks but are constrained by GOAT's orchestration layer.

## Architecture

The `ExternalAgentManager` is the core component injected into the `GoatRuntime`. It manages an `ExternalAgentRegistry` that holds instances of `Adapter`.

### Key Components
1. **Registry & Adapters**: Found in `src/external_agents.rs`. Maintains a hardcoded list of supported adapters and their metadata (`Adapter`).
2. **Detection**: Upon initialization or manual trigger (`/external-agents detect`), GOAT utilizes `which::which` to detect if the binary for an external agent is available in the user's `PATH`.
3. **Execution**: The `delegate` function spawns the external agent as a child process via `std::process::Command`. 
4. **Tool Registry Integration**: An external agent is triggered via the `delegate_external_agent` tool in the `ToolRegistry`. This ensures that delegation goes through the central security policy.
5. **Approval Gate Integration**: Before execution, GOAT verifies against `ApprovalGate` (Risk Level: High).
6. **Audit Log**: Every execution run is documented in an audit log (stored at `~/.local/state/goat/external_agents.log`).

## Configuration
External agents are controlled via `~/.config/goat/config.toml`:
```toml
[external_agents]
enabled = false             # By default, external agents are explicitly disabled for safety
default_timeout_secs = 300  # Max execution time (in seconds)
allow_workspace_modification = false # Currently disabled. Future phases will toggle this.
```

## Supported Agents
The registry currently checks for a variety of known agents:
- `aider` (Aider)
- `opencode-cli` (OpenCode)
- `cline-cli` (Cline)
- `claude` (Claude Code)
- `vibe` (Mistral Vibe)
- `kiro-cli` (Kiro)
- `codex` / `codexcli` (Codex)
- `repomap` (Aider Repomap Tool)
- `sweep` (Sweep)
- `cursor` / `windsurf`

## CLI Commands

- `goat external-agents list`: List all known external agents and their detection status.
- `goat external-agents detect`: Trigger a new scan for external binaries in the system.
- `goat external-agents show <name>`: Show detailed metadata for a specific agent adapter.
- `goat external-agents audit`: Dump the contents of the external agent audit log.
- `goat delegate-external <agent_name> <task_prompt>`: Delegate a specific task to an external agent. (Requires `ApprovalGate` and `ToolRegistry` approval).

## TUI slash commands
Inside GOAT interactive sessions (headless or TUI):
- `/external-agents [list|detect|audit|show <name>]`
- `/external-agent <name>`
- `/delegate-external <name> <task_prompt>`

## Security and Safety
- **Detection without execution**: Finding agents uses `which`, not subprocess execution with `--version`, to prevent arbitrary side-effects during startup.
- **Disabled by default**: To prevent rogue AI recursion, `enabled = false` is standard.
- **Approval Gate**: All triggers funnel through the `ApprovalGate` as `High` risk operations.

## Next Steps (Phase 2.9)
- Orchestration metrics (token consumption integration).
- Proper interactive overlay mapping (letting external agents stream output into Ratatui overlays).
- Subagent to External Agent delegation workflows.

## Phase 2.9 Updates
Phase 2.9 brings safe orchestration and workspace isolation to the external agent adapters.
- **Isolated Workspace Execution**: Supports `isolated-copy` mode, where a temporary workspace is cloned with `.git`, `node_modules`, `target`, and secrets intentionally excluded. The external agent runs inside this directory without modifying the host project.
- **Run Tracking**: Implements `ExternalAgentRun` capturing structured outputs (stdout/stderr), metadata, exit codes, and durations.
- **Historical Runs**: History is appended to `external-agent-runs.jsonl`. Accessible via `goat external-agents runs` (CLI) or `/external-runs` (TUI/headless).
- **Run Inspection**: Explore detailed run outputs with `goat external-agents run <id>` (CLI) or `/external-run <id>` (TUI/headless).
- **Comparison Tool**: Added `/compare-agents <task>` to synchronously diff an internal agent execution against a designated external agent (Aider by default).
