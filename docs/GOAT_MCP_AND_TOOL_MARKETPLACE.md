# GOAT MCP & Tool Marketplace Foundation (Phase 3.7)

This document describes the foundation for the Model Context Protocol (MCP) and Tool Marketplace integration within GOAT, introduced in Phase 3.7.

## Overview
GOAT supports the Model Context Protocol (MCP) to seamlessly integrate external tools and capabilities into the agent's context. Phase 3.7 lays the foundation by introducing configuration parsing, safe default behaviors, and CLI/TUI interfaces to inspect the available tools.

Full functional integration, including automatic installation, sandboxed execution, and live dynamic tool addition, is planned for Phase 3.8.

## Configuration Paths
GOAT detects MCP configurations from the following files, in priority order:
1. `<project_root>/mcp.json`
2. `<project_root>/mcp.toml`
3. `<config_dir>/mcp.json` (e.g., `~/.config/goat/mcp.json`)
4. `<config_dir>/mcp.toml` (e.g., `~/.config/goat/mcp.toml`)

## MCP Server Configuration Structure
A basic `mcp.json` structure looks like this:
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "uvx",
      "args": ["mcp-server-sqlite", "--db-path", "./test.db"],
      "enabled": true,
      "transport": "stdio",
      "risk": "ask"
    }
  }
}
```

### Risk Policies
All external MCP tools are considered untrusted by default. The `risk` field determines how the `ApprovalGate` handles execution requests:
- **`ask`** (default): Every execution request requires explicit user approval.
- **`deny`**: The tool is blocked and cannot be executed.
- **`allow`**: The tool is allowed to execute without prompting (use with extreme caution, restricted to safe/read-only tools).

## Tool Catalog Foundation
The Tool Catalog is a planned feature where users can browse, install, and enable verified tools directly from within GOAT.
- **Path**: `<config_dir>/tool_catalog.json`
- **Commands**:
  - `goat tools catalog` (CLI) or `/tools catalog` (TUI)
  - `goat tools catalog search <query>`
  - `goat tools catalog show <name>`

## Security Principles
- **ApprovalGate Integration**: Any `start`, `stop`, or execution command for an MCP server goes through GOAT's standard `ApprovalGate`.
- **Environment Variables**: MCP servers can receive custom environment variables, but sensitive values must be redacted in logs.
- **No Auto-Install**: Tools and dependencies (like `uvx`, `npm`) are not automatically installed. The user is in full control of their environment.

## Next Steps (Phase 3.8)
- Full lifecycle management (`/mcp start`, `/mcp stop`, `/mcp restart`).
- Live tool registration and deregistration with the `ToolRegistry`.
- Automated sandboxed tool executions and streaming output.
