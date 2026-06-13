# GOAT Runtime Wiring — Phase 9.3

## Overview

Phase 9.3 introduces the **Capability Runtime Adapter** (`src/capability_runtime.rs`), which bridges the metadata-only `CapabilityRegistry` (Phase 9.2) with GOAT's runtime/tool environment.

**Core safety guarantee**: Extension capabilities are metadata first. The `CapabilityRegistry` stores discovery data; the `CapabilityRuntimeAdapter` enforces safety before any execution is attempted.

---

## Architecture

```
ExtensionManifest (untrusted)
        ↓
  ExtensionManager
        ↓
  CapabilityRegistry (metadata, Phase 9.2)
        ↓
  CapabilityRuntimeAdapter (Phase 9.3)
        ↓ prepare()
   PrepareResult (pre-flight checks, never executes)
        ↓ invoke_sync()
   ApprovalGate → user prompt → Approved/Denied
        ↓ (if Approved + Command type only)
   std::process::Command (no shell expansion)
        ↓
   ToolRegistry audit log + capability-runtime.log
```

---

## Capability Lifecycle

| Status | Meaning |
|--------|---------|
| `Discoverable` | Known but not yet validated |
| `Available` | Pre-conditions met, can be invoked |
| `RequiresApproval` | Risk level ≥ Medium; must pass ApprovalGate |
| `ApprovedForThisRun` | User approved (single run, not persistent) |
| `Executed` | Successfully executed |
| `Failed(msg)` | Execution failed |
| `Blocked(reason)` | Disabled, bad path, denied, or unsupported type |

---

## Capability Types and Invocation Rules

| Type | Prepare | Invoke | Notes |
|------|---------|--------|-------|
| `Command` | ✅ Full checks | ✅ With ApprovalGate | Only type that can be directly invoked |
| `McpServer` | ✅ Metadata display | ❌ Never auto-started | Use external MCP client |
| `Skill` | ✅ Path check | ❌ Guided only | Use `goat skills run --from-extension <id>` |
| `ValidationRecipe` | ✅ Recipe path shown | ❌ Explicit run only | Use `goat validate --recipe <id>` |
| `NativeTool` | ✅ Info only | ❌ ToolRegistry handles | Core tools managed separately |

---

## CLI Commands

### `goat tools prepare <capability-id>`

Runs pre-flight checks without executing anything. Shows:
- Capability enabled status
- Source extension enabled status
- Command safety (no shell meta-characters)
- Path safety (for skills/validators)
- Risk level and approval requirements

### `goat tools invoke <capability-id>`

Invokes a `Command`-type capability after ApprovalGate. Interactive prompt shown.
Non-shell execution: `std::process::Command` (no bash expansion, no piping).

### `goat tools runtime`

Lists all capabilities with their current `CapabilityStatus`.

### `goat tools refresh`

Re-scans enabled extensions and updates the `CapabilityRegistry`.

---

## Security Decisions

1. **No shell expansion**: Commands are split by whitespace and passed to `std::process::Command` directly. Shell meta-characters (`;`, `|`, `&`, `` ` ``, `$()`) block preparation.
2. **No auto-start**: MCP servers are never started automatically. Blocked at `invoke_sync`.
3. **No persistent approval**: Approval is session-only via `ApprovalGate`. No permanent trust stored.
4. **Fail-closed**: Unknown input to ApprovalGate defaults to Deny.
5. **Extension must be enabled**: Both the capability and the source extension must be enabled. Disabled extension → `Blocked`.
6. **Path safety**: Skill/ValidationRecipe paths must be relative OR within `skills_dir`/`data_dir`.
7. **Secret redaction**: All log lines pass through `approval::redact_secrets` before writing.

---

## Logging

Two log files are updated on every prepare/invoke:

- `~/.local/share/goat/capability-runtime.log` — capability-specific invocation log
- `~/.local/share/goat/tool-audit.log` — shared tool audit log (via `ToolRegistry::log_execution`)

Log format:
```
[timestamp] id=... name=... source=... ext=... action=... risk=... approval_req=... approval_result=... exec_result=... error=...
```

Secrets are redacted. Log is append-only.

---

## Current Limitations

- MCP server startup is not implemented. Metadata displayed only.
- Skill guided execution via `goat skills run --from-extension` is documented but not yet wired in the Skill CLI path.
- `goat validate --recipe <id>` is documented but extension recipe integration into `ValidationManager` is future work (Phase 9.4).
- No TUI integration in this phase (planned Phase 9.4).
- No dashboard "run" buttons — CLI only.

---

## Future Work (Phase 9.4)

- TUI `/tools` panel with capability status display
- `goat validate --recipe <id>` integration with `ValidationManager`
- `goat skills run --from-extension <id>` integration with `SkillManager`
- Sample extension packs for testing
- Dashboard capability status widget
