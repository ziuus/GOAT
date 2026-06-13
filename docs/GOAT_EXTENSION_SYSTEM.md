# GOAT Extension System

The GOAT Extension System provides a safe, locally sandboxed mechanism to extend GOAT's capabilities without modifying the core Rust binary. It focuses on safely aggregating metadata, skills, commands, MCP servers, and agent configurations while delegating execution to GOAT's core safety boundaries.

## Architecture & Security Model

Extensions in GOAT are **untrusted by default**. They are treated purely as metadata and content packages.
- **No Arbitrary Code Execution:** GOAT will never blindly `eval()` or dynamically execute a binary bundled inside an extension.
- **Safety First:** Actions proposed by extension skills/workflows must still pass through GOAT's `ApprovalGate`.
- **Secret Protection:** Extensions cannot silently read `goat_brain.db` or workspace secrets.
- **Validation:** Extensions must provide a valid `GOAT_EXTENSION.toml` manifest to be installed.

## Extension Manifest (`GOAT_EXTENSION.toml`)

Every extension must include a `GOAT_EXTENSION.toml` at its root. 

```toml
[extension]
id = "goat-rust-workflow-pack"
name = "Rust Workflow Pack"
version = "0.1.0"
description = "Common Rust workflows."
author = "Your Name"
license = "MIT"
type = "workflow_pack"
risk_level = "medium"

[capabilities]
skills = true
validation_recipes = false
commands = false
mcp_servers = false
external_agents = false
dashboard_widgets = false

[permissions]
read_project = true
write_project = true
run_commands = false
network = false
access_memory = false

[entrypoints]
skills = ["skills/rust.md"]
validation_recipes = []
```

### Extension Types
- `skill_pack`: Provides natural language skills and standard operating procedures.
- `mcp_pack`: Exposes Model Context Protocol definitions.
- `agent_adapter`: Connects GOAT to external agent platforms.
- `workflow_pack`: Bundles skills, validations, and project structures.
- `command_pack`: Declares predefined, bounded terminal commands.
- `validation_pack`: Provides custom `validation.toml` templates.
- `memory_pack`: Injects static architectural context or domain knowledge.
- `dashboard_widget`: Custom UI components for the GOAT dashboard (Planned).
- `project_template`: Scaffolds specific directory layouts.

## CLI Management

The `goat extensions` (or `goat extension`) CLI allows local management.

- `goat extension validate <path>`: Check a local directory's manifest for schema errors and path traversals.
- `goat extension install <path>`: Copies the extension to `~/.local/share/goat/extensions/installed/`. It defaults to `disabled`.
- `goat extension list`: Lists installed extensions.
- `goat extension show <id>`: Displays detailed manifest and status information.
- `goat extension enable <id>`: Activates the extension. If the risk level is High or Critical, GOAT will prompt for explicit user confirmation.
- `goat extension disable <id>`: Deactivates the extension.
- `goat extension remove <id>`: Archives/deletes the extension from the system.
- `goat extension doctor`: Scans the registry for broken entrypoints, missing directories, or unsafe state.

## Registry Storage

GOAT stores installed extensions locally inside the data directory (typically `~/.local/share/goat/extensions/` on Linux).
- `installed/`: Contains deep-copied extension content.
- `registry.jsonl`: The local database mapping extension IDs to their status, install paths, and metadata.

---

## Phase 9.3 Update: Runtime Wiring

Extension capabilities discovered by `CapabilityRegistry` can now be prepared and invoked via the `CapabilityRuntimeAdapter`.

Key additions:
- `goat tools prepare <id>` — pre-flight checks, no execution
- `goat tools invoke <id>` — Command-type only, requires ApprovalGate
- `goat tools runtime` — runtime status of all capabilities
- Invocation lifecycle: `Available → RequiresApproval → ApprovedForThisRun → Executed | Failed | Blocked`

**Extension capabilities are metadata first. No extension code runs without explicit user approval.**

For full details, see [GOAT_RUNTIME_WIRING.md](./GOAT_RUNTIME_WIRING.md).
