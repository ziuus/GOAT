# GOAT Alpha Readiness Migration Guide

Welcome to the **GOAT Alpha** release. GOAT is a Rust-first, local-first universal AI agent operating system designed as a migration-grade alternative to existing CLI coding assistants (e.g., Aider, Claude Code, OpenCode).

Unlike simple wrappers or unconstrained bots, GOAT provides deterministic execution paths, an ApprovalGate for dangerous actions, and an explicit capability registry for native and third-party extensions.

## Why Migrate to GOAT?

1. **Security & Control (Fail-Closed by Default)**
   - No auto-running bash loops.
   - All shell commands, filesystem changes, and external tool calls run through our `ApprovalGate`. 
   - Strict audit logs track what was executed, by whom, and what secrets were redacted.
   
2. **First-Class Extension Capabilities (Phase 9)**
   - Most CLI bots hardcode "tools". GOAT treats capabilities (Skills, Validators, MCP Metadata) as declarative assets via `CapabilityRegistry`.
   - Capabilities must be *prepared* (`goat tools prepare <id>`) and *invoked* explicitly.

3. **Validation Recipes & Deterministic Skills**
   - Run tests natively before making changes: `goat validate --recipe <id>`
   - Run specific skill playbooks: `goat skills run --from-extension <id>`

## Breaking Changes from Pre-Alpha (Phase 8 & Below)

- **ApprovalGate Enforcement:** Unapproved tools will immediately fail. Use `/tool prepare <id>` inside the TUI or `goat tools prepare <id>` in the CLI to satisfy preconditions.
- **MCP Server Autostart Disabled:** To prevent untrusted processes from lingering, MCP servers no longer automatically spawn without explicit preparation.
- **Skill Execution:** Skills are no longer simple bash scripts; they are "guided workflows" attached to the capability engine.

## TUI (Terminal User Interface) Usage

- Launch: `goat`
- **Dashboard Widget:** Press `t` or use the `/tool` slash command to see the **Tools & Capabilities Dashboard**. This view summarizes built-in tools, MCP servers, and registered Extension Capabilities along with their risk levels.
- **Slash Commands:**
  - `/tool show <id>` - Inspect a capability's details.
  - `/tool prepare <id>` - Run safety checks (ApprovalGate) to make a capability ready.

## Alpha Health Check

Before filing bug reports, run:
```bash
goat doctor alpha
```
This performs standard filesystem/DB checks along with Alpha-specific checks (Tools enabled state, Extension Registry presence). 

## Building Custom Extensions

Create JSON files in `~/.local/share/goat/capabilities/`. Example validation recipe:
```json
{
  "id": "my-val",
  "name": "My Custom Validation",
  "capability_type": "ValidationRecipe",
  "risk_level": "Low",
  "description": "Runs local tests safely",
  "source": "Local",
  "version": "1.0",
  "metadata": {
    "command": "cargo test --bin my-val"
  }
}
```

Welcome to the safe, deterministic future of AI agent execution.
