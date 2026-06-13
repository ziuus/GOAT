# GOAT Approval Profiles

GOAT uses an `ApprovalGate` to ensure potentially destructive actions—such as shell execution, file modification, subagent creation, and MCP calls—are never run silently without user consent. 

However, some workflows (like TDD) involve repeatedly running the exact same safe validation command (e.g., `cargo test` or `npm test`). To help mitigate **approval fatigue**, GOAT supports configurable **Approval Profiles**.

## Available Profiles

### 1. `strict` (Default)
The safest and recommended mode. 
- **Behavior**: Every gated operation prompts for approval.
- **Can skip**: Nothing automatically. (You can still use the `'a'` / `'d'` interactive overrides to allow or deny a specific tool for the duration of the current session).

### 2. `validation-fast`
Designed for active development loops.
- **Behavior**: Automatically skips prompts for **safe validation commands** that you have *already approved* in the current session.
- **Can skip**: Repeated invocations of safe code execution tools (e.g., `validate`, `code_execution`) running the exact same command.
- **Never skips**: Arbitrary shell execution (`bash`), file writes (`write_file`), file deletion, MCP tool invocation, external agents, or extension capabilities.

### 3. `balanced`
*(Planned - currently maps to strict behavior)*
Will provide heuristics to safely reduce prompts for read-only actions that might otherwise be caught by the gate. 

### 4. `audit-only`
A dry-run mode for testing and debugging GOAT itself. 
- **Behavior**: Does not block execution. It simply logs what *would* have required approval.
- **Warning**: Do not use this in daily workflows unless you explicitly trust the LLM.

## How to Change Your Profile

Currently, profiles are managed via the config file.

**View current profile:**
```bash
goat approval profile
```

**Explain profiles:**
```bash
goat approval profile explain
```

**Set profile (Instructions):**
```bash
goat approval profile set validation-fast
```
*(Note: To actually change the configuration, edit `~/.config/goat/config.toml` and set `profile = "validation-fast"` under the `[approval]` section).*

## Safety Guarantees

No matter which profile is active:
1. File writes are **never** auto-approved by a profile.
2. Extension capabilities and MCP tools are **never** auto-approved by a profile.
3. If an operation's risk level is evaluated as `High` or `Critical` (e.g. `rm -rf /`), it bypasses any fast-path heuristics and forces a prompt.
