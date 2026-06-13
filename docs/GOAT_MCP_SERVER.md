# GOAT MCP Server

GOAT supports running as a headless Model Context Protocol (MCP) server. This allows other AI agents, editors, or interfaces (like VS Code, Cursor, or Claude Desktop) to invoke GOAT's core capabilities seamlessly.

## How to Run

Start the GOAT MCP server over standard input/output (stdio) by running:

```bash
goat --mcp
```
or 
```bash
goat --mcp-server
```

When running in this mode, GOAT will accept JSON-RPC payloads according to the [Model Context Protocol Specification](https://modelcontextprotocol.io/). It outputs JSON-RPC responses on standard output. Diagnostic messages (e.g., debug, info, warnings) are emitted over standard error or via configured tracing subscribers to avoid polluting the JSON stream.

## Available Tools

The MCP server currently exposes safe, read-only or strictly bounded tools designed to assist context gathering.

### 1. `goat_memory_add`

Adds a new memory to the GOAT structured memory store. 

- **Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "content": { "type": "string", "description": "The memory content to store." }
    },
    "required": ["content"]
  }
  ```
- **Safety Rules:** 
  - All input is passed through `crate::memory::redact_secrets()`.
  - If the content is entirely sensitive (e.g. looks like an API key or password) and redacts to an empty string or `[REDACTED]`, the tool will reject the payload to prevent secret leakage into the memory database.

### 2. `goat_repo_status`

Generates a summarized repository map of the current workspace, providing awareness of the project structure.

- **Schema:**
  ```json
  {
    "type": "object",
    "properties": {}
  }
  ```
- **Safety Rules:**
  - Operates only in the current approved workspace or `data_dir`.
  - Automatically ignores deep directories (e.g. `node_modules`, `target`), hidden `.git` folders, and secret files.
  - Limits output size to prevent context overflow.

## VS Code Extension Usage

A prototype VS Code extension is available in `vscode-goat/`. It serves as a reference implementation of an MCP client connecting to the GOAT MCP server.

### Building the Extension
```bash
cd vscode-goat
npm install
npm run compile
```

### Running the Extension
- Open `vscode-goat/` in VS Code.
- Press `F5` to open an Extension Development Host.
- Run the command **GOAT: Start MCP Server**.
- Run the command **GOAT: Show Repo Status** to view the parsed repository map.

## Current Limitations

- The MCP server only supports `stdio` transport. HTTP/SSE is not currently implemented.
- Tools are limited to context and memory. Destructive actions (like executing patches or shell commands) are disabled in MCP mode due to the requirement for explicit interactive user approval (`ApprovalGate`).

## Future Roadmap

As part of the upcoming **Phase 9.1**, we plan to implement:
- **Ecosystem Extension Manifest:** `GOAT_EXTENSION.toml` for defining local MCP packs.
- **Local Plugin Registry:** `~/.local/share/goat/extensions/` to store and manage multiple tool sets.
- **Dynamic Tool Loading:** The ability for GOAT to proxy or load custom MCP skill-packs and validation-packs safely.
