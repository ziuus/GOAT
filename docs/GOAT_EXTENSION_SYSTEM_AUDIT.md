# GOAT Extension System Audit (Phase 6.8)

## 1. Current State of Extension Points

### Tools (`src/tool_registry.rs`)
- **Existing:** Built-in native tools and dynamic tools supplied via MCP servers.
- **Installable:** Tools cannot be installed standalone. They must be either hardcoded native tools or exposed by configured MCP servers.
- **Safety:** MCP tools execute through the MCP runtime; native tools have specific Rust implementations. Dangerous native tools (e.g., shell commands) use `ApprovalGate` for user confirmation.

### MCP Support (`src/mcp.rs`, `src/mcp_runtime.rs`)
- **Existing:** Configurable external MCP servers defined in `goat.toml` under `[[mcp_servers]]`.
- **Installable:** Adding a new MCP server requires manual editing of the `goat.toml` configuration file.
- **Safety:** MCP servers run as child processes (local) or communicate via HTTP (remote). Local execution inherits GOAT's user permissions. Command execution is subject to `ApprovalGate` policies if configured correctly.

### Skills (`src/skill_marketplace.rs`)
- **Existing:** A local directory containing Markdown files with instructions (`~/.gemini/antigravity/skills/`).
- **Installable:** Yes, `skill_marketplace` supports listing and downloading skills from a remote registry or local cache.
- **Safety:** Skills are primarily text prompts/instructions used by agents. They don't execute arbitrary code directly but can instruct agents to run dangerous tools.

### Recipes (`src/recipe_marketplace.rs`)
- **Existing:** A catalog of predefined multi-step workflows.
- **Installable:** Yes, similar to skills, recipes can be downloaded and stored locally.
- **Safety:** Recipes define a sequence of actions. High-risk actions triggered within recipes trigger `ApprovalGate`.

### Providers (`src/models.rs`)
- **Existing:** Hardcoded list of default provider profiles, merged with user configurations in `goat.toml`.
- **Installable:** Users can add providers by manually editing `goat.toml`.
- **Safety:** Configured via `goat.toml`. API keys are loaded via environment variables or explicitly.

### ApprovalGate (`src/approval.rs`)
- **Existing:** Core component that intercepts high-risk operations (shell commands, file overwrites) and requests user confirmation via CLI or UI.

## 2. Identified Issues & Duplications

- **Fragmentation:** Skills, recipes, and MCP servers all have their own discovery, download, and registration mechanisms. There is no unified "Marketplace" or "Extension Registry".
- **Lack of Dependency Management:** A skill might rely on an MCP server, but this cannot be natively expressed or installed as a single bundle.
- **Manual Configuration:** Adding new providers or MCP servers requires manual `goat.toml` edits.
- **Permissions Context:** When a tool requests approval via `ApprovalGate`, the user sees the command, but not necessarily which extension originated it or what its declared permissions are.

## 3. Scope of Phase 6.8 Implementation

**What will be implemented:**
1. **Unified `ExtensionRegistry`:** A central system (`src/extensions.rs`) to manage the lifecycle (install, audit, enable, disable) of all extension kinds.
2. **Standardized `ExtensionManifest`:** A unified `manifest.json/toml` format for packaging tools, MCP servers, skills, recipes, and providers.
3. **Explicit Permission Model:** Extensions must declare permissions (`shell_command`, `network_access`, etc.).
4. **Audit Engine:** Automated inspection of extension manifests and artifacts to generate findings (e.g., undeclared permissions, remote sources).
5. **Install Flow:** Discover -> Audit -> Install (Disabled) -> User Review -> Enable.
6. **Dashboard Integration:** A new `/extensions` page on the dashboard to view, audit, and manage extensions.

**What will remain partial/stubbed:**
- Remote index fetching (remote marketplaces will be stubbed safely; local-folder and local-builtin will be fully implemented).
- Direct remote execution (strictly forbidden).
- Deep automatic integration of every old feature (we will integrate at a high level; detailed rewiring of legacy skills/recipes will be progressive).

## 4. Unification Strategy

The `ExtensionManifest` will serve as the single source of truth. An extension can declare it provides:
- `skills`: Pointers to `.md` files.
- `recipes`: Pointers to JSON/YAML recipes.
- `mcp_servers`: Configuration blocks for MCP servers.
- `provider_profiles`: LLM provider templates.

When an extension is **Enabled**, the `ExtensionRegistry` will inject these definitions into the respective underlying registries (`ToolRegistry`, `McpRuntime`, etc.). When **Disabled**, they are withdrawn.
