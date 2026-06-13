/// GOAT Unified Command Registry — Phase 3.1
///
/// This module defines a central registry of all slash commands available in
/// GOAT. It powers:
///   - `/help` grouped output
///   - `/palette` command palette
///   - `/commands` listing and search
///   - TUI slash-command recommendation popup (prefix filtering)
///   - Headless text output
///
/// SECURITY: The registry does not execute commands. It only provides metadata.
/// Command routing is still handled by `app.rs` and `headless.rs`.
/// Planned commands are listed but CANNOT execute — the routers check status.

// ── Types ─────────────────────────────────────────────────────────────────────

/// Unique identifier for a command.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandId(pub &'static str);

/// Category groupings for commands.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandCategory {
    General,
    Sessions,
    Models,
    Project,
    Repo,
    Coding,
    Patches,
    Memory,
    Skills,
    Tools,
    Mcp,
    Subagents,
    ExternalAgents,
    Ui,
    Logs,
    System,
    Hooks,
    Scheduler,
    Jobs,
    Browser,
    Transports,
    Voice,
    /// Commands planned for future phases — not yet implemented.
    Future,
}

impl CommandCategory {
    pub fn label(&self) -> &'static str {
        match self {
            CommandCategory::General => "General",
            CommandCategory::Sessions => "Sessions",
            CommandCategory::Models => "Models",
            CommandCategory::Project => "Project",
            CommandCategory::Repo => "Repo Map",
            CommandCategory::Coding => "Coding",
            CommandCategory::Patches => "Patches",
            CommandCategory::Memory => "Memory",
            CommandCategory::Skills => "Skills",
            CommandCategory::Tools => "Tools",
            CommandCategory::Mcp => "MCP",
            CommandCategory::Subagents => "Subagents",
            CommandCategory::ExternalAgents => "External Agents",
            CommandCategory::Ui => "UI / Views",
            CommandCategory::Logs => "Logs",
            CommandCategory::System => "System",
            CommandCategory::Hooks => "Hooks",
            CommandCategory::Scheduler => "Scheduler",
            CommandCategory::Jobs => "Jobs",
            CommandCategory::Browser => "Browser / QA",
            CommandCategory::Transports => "Transports",
            CommandCategory::Voice => "Voice",
            CommandCategory::Future => "Future (Planned)",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            CommandCategory::General => "💬",
            CommandCategory::Sessions => "📁",
            CommandCategory::Models => "🤖",
            CommandCategory::Project => "📂",
            CommandCategory::Repo => "🗺",
            CommandCategory::Coding => "⌨",
            CommandCategory::Patches => "🩹",
            CommandCategory::Memory => "🧠",
            CommandCategory::Skills => "🎯",
            CommandCategory::Tools => "🔧",
            CommandCategory::Mcp => "🔌",
            CommandCategory::Subagents => "👥",
            CommandCategory::ExternalAgents => "🌐",
            CommandCategory::Ui => "🖥",
            CommandCategory::Logs => "📋",
            CommandCategory::System => "⚙",
            CommandCategory::Hooks => "🪝",
            CommandCategory::Scheduler => "⏱",
            CommandCategory::Jobs => "🏗",
            CommandCategory::Browser => "🌐",
            CommandCategory::Transports => "📡",
            CommandCategory::Voice => "🎙",
            CommandCategory::Future => "🔮",
        }
    }
}

/// Implementation status of a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandStatus {
    /// Fully implemented and tested.
    Working,
    /// Implemented but incomplete or known limitations.
    Partial,
    /// Planned but not yet implemented. MUST NOT execute.
    Planned,
    /// Explicitly disabled or deprecated.
    Disabled,
}

impl CommandStatus {
    pub fn label(&self) -> &'static str {
        match self {
            CommandStatus::Working => "✅",
            CommandStatus::Partial => "⚡",
            CommandStatus::Planned => "🔮",
            CommandStatus::Disabled => "❌",
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            CommandStatus::Working => "working",
            CommandStatus::Partial => "partial",
            CommandStatus::Planned => "planned",
            CommandStatus::Disabled => "disabled",
        }
    }
}

/// Risk level for commands that trigger dangerous operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandRisk {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl CommandRisk {
    pub fn label(&self) -> &'static str {
        match self {
            CommandRisk::None => "",
            CommandRisk::Low => "🟢 low",
            CommandRisk::Medium => "🔵 medium",
            CommandRisk::High => "🟡 high",
            CommandRisk::Critical => "🔴 critical",
        }
    }
}

/// Which surfaces support this command.
#[derive(Debug, Clone)]
pub struct CommandSurface {
    pub tui: bool,
    pub headless: bool,
    pub cli: bool,
}

impl CommandSurface {
    pub fn both() -> Self {
        Self {
            tui: true,
            headless: true,
            cli: false,
        }
    }
    pub fn tui_only() -> Self {
        Self {
            tui: true,
            headless: false,
            cli: false,
        }
    }
    pub fn headless_only() -> Self {
        Self {
            tui: false,
            headless: true,
            cli: false,
        }
    }
    pub fn all() -> Self {
        Self {
            tui: true,
            headless: true,
            cli: true,
        }
    }
}

/// Full metadata for a single slash command.
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    /// Primary slash command name, e.g. "/help"
    pub name: &'static str,
    /// Aliases that resolve to this command.
    pub aliases: &'static [&'static str],
    pub category: CommandCategory,
    /// One-line description shown in listings.
    pub description: &'static str,
    /// Usage syntax, e.g. "/skill <name>"
    pub usage: &'static str,
    /// Short examples.
    pub examples: &'static [&'static str],
    /// Keyboard shortcut if any (e.g. "Ctrl+P")
    pub shortcut: Option<&'static str>,
    pub surface: CommandSurface,
    /// Whether this command requires ApprovalGate.
    pub requires_approval: bool,
    pub risk: CommandRisk,
    pub status: CommandStatus,
    /// Related feature name for cross-referencing.
    pub related: Option<&'static str>,
}

impl CommandMetadata {
    /// Returns true if this command should be shown in normal listings.
    /// Planned commands are hidden by default unless explicitly requested.
    pub fn is_visible(&self) -> bool {
        !matches!(
            self.status,
            CommandStatus::Planned | CommandStatus::Disabled
        )
    }

    /// Returns true if the command name or any alias starts with the prefix.
    pub fn matches_prefix(&self, prefix: &str) -> bool {
        let p = prefix.to_lowercase();
        if self.name.starts_with(p.as_str()) {
            return true;
        }
        self.aliases.iter().any(|a| a.starts_with(p.as_str()))
    }

    /// Returns true if the command name, description, or category label
    /// contains the query string (case-insensitive).
    pub fn matches_search(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.name.to_lowercase().contains(q.as_str())
            || self.description.to_lowercase().contains(q.as_str())
            || self.category.label().to_lowercase().contains(q.as_str())
            || self
                .aliases
                .iter()
                .any(|a| a.to_lowercase().contains(q.as_str()))
    }

    /// Short one-liner for popup suggestions: "/name — description"
    pub fn suggestion_line(&self) -> String {
        format!("{} — {}", self.name, self.description)
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────

/// The unified command registry. Holds all `CommandMetadata` entries.
pub struct CommandRegistry {
    commands: Vec<CommandMetadata>,
}

impl CommandRegistry {
    /// Build and return the global registry with all registered commands.
    pub fn build() -> Self {
        Self {
            commands: all_commands(),
        }
    }

    /// Return all commands, optionally including planned ones.
    pub fn all(&self, include_planned: bool) -> Vec<&CommandMetadata> {
        self.commands
            .iter()
            .filter(|c| include_planned || c.is_visible())
            .collect()
    }

    /// Return commands matching a prefix (for TUI autocomplete popup).
    /// Only returns working/partial commands by default.
    pub fn suggest(&self, prefix: &str) -> Vec<&CommandMetadata> {
        if prefix.is_empty() || prefix == "/" {
            return self.commands.iter().filter(|c| c.is_visible()).collect();
        }
        self.commands
            .iter()
            .filter(|c| c.is_visible() && c.matches_prefix(prefix))
            .collect()
    }

    /// Return all commands matching a search query (for /commands search).
    pub fn search(&self, query: &str, include_planned: bool) -> Vec<&CommandMetadata> {
        self.commands
            .iter()
            .filter(|c| (include_planned || c.is_visible()) && c.matches_search(query))
            .collect()
    }

    /// Return commands grouped by category. Sorted by category then name.
    pub fn grouped(&self, include_planned: bool) -> Vec<(CommandCategory, Vec<&CommandMetadata>)> {
        let mut groups: std::collections::BTreeMap<
            String,
            (CommandCategory, Vec<&CommandMetadata>),
        > = std::collections::BTreeMap::new();

        for cmd in &self.commands {
            if !include_planned && !cmd.is_visible() {
                continue;
            }
            let key = format!("{:?}", cmd.category);
            let entry = groups
                .entry(key)
                .or_insert_with(|| (cmd.category.clone(), Vec::new()));
            entry.1.push(cmd);
        }

        groups.into_values().collect()
    }

    /// Look up a command by exact name or alias.
    pub fn find(&self, name: &str) -> Option<&CommandMetadata> {
        self.commands
            .iter()
            .find(|c| c.name == name || c.aliases.iter().any(|a| *a == name))
    }

    /// Return the first suggestion for tab completion of a prefix.
    pub fn complete(&self, prefix: &str) -> Option<&str> {
        self.suggest(prefix).into_iter().next().map(|c| c.name)
    }

    /// Format grouped help output as a Vec<String> for display.
    pub fn format_help(&self, include_planned: bool) -> Vec<String> {
        let mut out = Vec::new();
        for (cat, cmds) in self.grouped(include_planned) {
            out.push(format!("\n{} {}", cat.emoji(), cat.label().to_uppercase()));
            out.push("─".repeat(50));
            for cmd in cmds {
                let status = cmd.status.label();
                let risk = if !matches!(cmd.risk, CommandRisk::None) {
                    format!(" {}", cmd.risk.label())
                } else {
                    String::new()
                };
                let shortcut = cmd.shortcut.map(|s| format!("  [{s}]")).unwrap_or_default();
                out.push(format!(
                    "  {status} {:<28} {}{risk}{shortcut}",
                    cmd.usage, cmd.description
                ));
            }
        }
        out
    }

    /// Format palette output: grouped list with short descriptions.
    pub fn format_palette(&self, filter: Option<&str>) -> Vec<String> {
        let mut out = Vec::new();
        let include_planned = false;

        if let Some(query) = filter {
            let results = self.search(query, false);
            if results.is_empty() {
                out.push(format!("  No commands found matching '{query}'"));
                out.push(String::new());
                out.push("  Try: /commands search <query>".to_string());
            } else {
                out.push(format!("  {} result(s) for '{query}':", results.len()));
                out.push(String::new());
                for cmd in results {
                    out.push(format!(
                        "  {} {:<28} {}",
                        cmd.status.label(),
                        cmd.usage,
                        cmd.description
                    ));
                }
            }
            return out;
        }

        for (cat, cmds) in self.grouped(include_planned) {
            out.push(format!("{} {}:", cat.emoji(), cat.label()));
            for cmd in cmds {
                let risk = if !matches!(cmd.risk, CommandRisk::None) {
                    format!(" ⚠{}", cmd.risk.label())
                } else {
                    String::new()
                };
                out.push(format!("  {}{risk}  {}", cmd.usage, cmd.description));
            }
            out.push(String::new());
        }
        out.push("  (Type /commands all to see planned future commands)".to_string());
        out
    }
}

// ── Command Definitions ───────────────────────────────────────────────────────

/// Build the master list of all commands.
/// Every slash command handled in app.rs and headless.rs must be here.
fn all_commands() -> Vec<CommandMetadata> {
    vec![
        // ── Phase 5.16: Agent Architecture ──────────────────────────────────────
        CommandMetadata {
            name: "/agents",
            aliases: &["\\agents", "\\agent", "\\prime", "\\specialist"],
            category: CommandCategory::System,
            description: "Manage GOAT Prime and Specialist agents",
            usage: "/agents [list | show <name> | enable <name> | disable <name> | reports <name> | specialists <prime>]",
            examples: &[
                "/agents list",
                "/agents show cofounder",
                "/agents enable finance_analyst",
                "/agents reports cofounder",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/cofounder",
            aliases: &["\\cofounder"],
            category: CommandCategory::System,
            description: "Cofounder Prime Agent commands",
            usage: "/cofounder [list | new-idea | validate <id> | score <id> | mvp <id> | competitors <id> | landing <id> | outreach <id> | report <id> | show <id>]",
            examples: &[
                "/cofounder list",
                "/cofounder new-idea",
                "/cofounder validate <id>",
                "/cofounder report <id>",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/socializer",
            aliases: &["\\socializer", "@socializer"],
            category: CommandCategory::System,
            description: "Socializer Prime Agent commands",
            usage: "/socializer [list | new-campaign | audience <id> | channels <id> | angles <id> | reddit <id> | linkedin <id> | x <id> | launch <id> | calendar <id> | outreach <id> | feedback <id> | report <id> | show <id> | from-idea <idea_id>]",
            examples: &[
                "/socializer list",
                "/socializer new-campaign",
                "/socializer audience <id>",
                "@socializer launch <id>",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/collab",
            aliases: &["@collab", "@team", "@handoff", "@workflow", "@agentflow"],
            category: CommandCategory::System,
            description: "Prime Agent Collaboration Layer",
            usage: "/collab [list | new | plan <goal> | show <id> | start <id> | step <id> | pause <id> | resume <id> | cancel <id> | handoffs <id> | report <id>]",
            examples: &[
                "/collab list",
                "/collab plan \"launch startup\"",
                "@team \"make GOAT launch-ready\"",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/promptforge",
            aliases: &["\\pf", "\\refine"],
            category: CommandCategory::System,
            description: "PromptForge optional refinement layer",
            usage: "/promptforge [status | enable | disable | doctor | refine <prompt> | score <prompt> | history | config | mode <mode> | test]",
            examples: &[
                "/promptforge status",
                "/promptforge refine \"make dashboard better\"",
                "\\pf status",
                "\\refine \"validate AI security audit agency\"",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        // ── Phase 5.14: Agent Modes, Projects, Onboarding ─────────────────────
        CommandMetadata {
            name: "/mode",
            aliases: &[
                "\\mode",
                "\\safe",
                "\\code",
                "\\plan",
                "\\review",
                "\\security",
                "\\docs",
                "\\qa",
                "\\github-pr",
            ],
            category: CommandCategory::System,
            description: "Switch or view Agent Mode Profiles",
            usage: "/mode [list | current | use <name> | recommend]",
            examples: &[
                "/mode list",
                "/mode use safe-chat",
                "/mode current",
                "/mode recommend",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/project",
            aliases: &["\\project", "\\checklist"],
            category: CommandCategory::System,
            description: "Manage and detect Project Profiles",
            usage: "/project [detect | show | recommend | save | checklist]",
            examples: &[
                "/project detect",
                "/project show",
                "/project save",
                "/project checklist",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        CommandMetadata {
            name: "/onboard",
            aliases: &["/setup", "/welcome", "\\setup"],
            category: CommandCategory::System,
            description: "Start the Setup Wizard / Onboarding",
            usage: "/onboard",
            examples: &["/onboard"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            status: CommandStatus::Working,
            risk: CommandRisk::Low,
            related: None,
        },
        // ── Browser / QA ───────────────────────────────────────────────────────
        CommandMetadata {
            name: "/browser",
            aliases: &[
                "\\browser",
                "\\browser doctor",
                "\\browser status",
                "\\browser workflows",
            ],
            category: CommandCategory::Browser,
            description: "Interact with the browser adapter layer and workflows",
            usage: "/browser <action> [url/id]",
            examples: &[
                "/browser status",
                "/browser doctor",
                "/browser open http://localhost:3000",
                "/browser screenshot",
                "/browser read",
                "/browser qa http://localhost:3000",
                "/browser landing-review http://localhost:3000",
                "/browser dashboard-qa",
                "/browser health http://localhost:3000",
                "/browser workflows",
                "/browser show <workflow-id>",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Builder / Coding ───────────────────────────────────────────────────
        CommandMetadata {
            name: "/builder",
            aliases: &[
                "\\builder",
                "@builder",
                "@code",
                "@plan",
                "@review",
                "@diff",
                "@tests",
                "@patch",
            ],
            category: CommandCategory::System,
            description: "Interact with the builder agent, inspect repos, plan patches, and review diffs",
            usage: "/builder <action> [goal/id]",
            examples: &[
                "/builder inspect",
                "/builder plan Create a helper module",
                "/builder diff-review",
                "/builder test-plan Create a helper module",
                "/builder validate",
                "/builder rollback-plan",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Transports ─────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/transports",
            aliases: &["\\transport", "\\msg sessions"],
            category: CommandCategory::Transports,
            description: "Manage messaging and voice transports",
            usage: "/transports <status|doctor|sessions|messages|send> [args]",
            examples: &[
                "/transports status",
                "/transports doctor",
                "/transports sessions",
                "/transports send <session-id> <msg>",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/telegram",
            aliases: &["\\telegram"],
            category: CommandCategory::Transports,
            description: "Telegram bot status",
            usage: "/telegram status",
            examples: &["/telegram status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: None,
        },
        CommandMetadata {
            name: "/discord",
            aliases: &["\\discord"],
            category: CommandCategory::Transports,
            description: "Discord bot status",
            usage: "/discord status",
            examples: &["/discord status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: None,
        },
        // ── Voice ──────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/voice",
            aliases: &["\\voice", "\\talk", "\\speak"],
            category: CommandCategory::Voice,
            description: "Interact with the voice manager",
            usage: "/voice <status|doctor|transcript|speak> [args]",
            examples: &[
                "/voice status",
                "/voice doctor",
                "/voice transcript hello",
                "/voice speak hello",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── General ────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/help",
            aliases: &[],
            category: CommandCategory::General,
            description: "Show all commands grouped by category",
            usage: "/help",
            examples: &["/help"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/status",
            aliases: &[],
            category: CommandCategory::General,
            description: "Show provider, profile, session, brain, and tool status",
            usage: "/status",
            examples: &["/status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/clear",
            aliases: &[],
            category: CommandCategory::General,
            description: "Clear the chat log display",
            usage: "/clear",
            examples: &["/clear"],
            shortcut: Some("Ctrl+L"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/quickstart",
            aliases: &["/qs"],
            category: CommandCategory::General,
            description: "Show the interactive quickstart guide",
            usage: "/quickstart",
            examples: &["/quickstart"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/doctor",
            aliases: &[],
            category: CommandCategory::General,
            description: "Check system health and workspace readiness",
            usage: "/doctor",
            examples: &["/doctor"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/migrate-from",
            aliases: &[],
            category: CommandCategory::General,
            description: "Help for migrating from other AI tools",
            usage: "/migrate-from <tool>",
            examples: &["/migrate-from aider", "/migrate-from claude-code"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/ui",
            aliases: &[],
            category: CommandCategory::General,
            description: "Show UI mode info and future UI surface plans",
            usage: "/ui",
            examples: &["/ui"],
            shortcut: None,
            surface: CommandSurface::tui_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/commands",
            aliases: &["/cmd"],
            category: CommandCategory::General,
            description: "List commands. Subcommands: all, planned, search <q>",
            usage: "/commands [all|planned|search <q>]",
            examples: &["/commands", "/commands all", "/commands search repo"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Sessions ───────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/sessions",
            aliases: &[],
            category: CommandCategory::Sessions,
            description: "List all sessions from brain database",
            usage: "/sessions",
            examples: &["/sessions"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("brain"),
        },
        CommandMetadata {
            name: "/new",
            aliases: &[],
            category: CommandCategory::Sessions,
            description: "Start a new session (clears history, creates new UUID)",
            usage: "/new",
            examples: &["/new"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("brain"),
        },
        // ── Models ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/profile",
            aliases: &[],
            category: CommandCategory::Models,
            description: "Show or switch model profile (balanced/coding/cheap/powerful…)",
            usage: "/profile [<name>]",
            examples: &["/profile", "/profile coding", "/profile balanced"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("models"),
        },
        CommandMetadata {
            name: "/profiles",
            aliases: &[],
            category: CommandCategory::Models,
            description: "List all available model profiles with chains",
            usage: "/profiles",
            examples: &["/profiles"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("models"),
        },
        // ── Project ────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/project",
            aliases: &[],
            category: CommandCategory::Project,
            description: "Show project context or run scan. Sub: scan, status",
            usage: "/project [scan|status]",
            examples: &["/project", "/project scan", "/project status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("project"),
        },
        CommandMetadata {
            name: "/learn",
            aliases: &[],
            category: CommandCategory::Project,
            description: "Trigger project indexing (learn about project structure)",
            usage: "/learn",
            examples: &["/learn"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: Some("project"),
        },
        // ── Repo ───────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/repo",
            aliases: &["/repo-map", "/repo_map"],
            category: CommandCategory::Repo,
            description: "Show interactive repository tree and map",
            usage: "/repo [refresh|summary|context]",
            examples: &["/repo", "/repo refresh", "/repo summary"],
            shortcut: Some("Ctrl+3"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("repo_map"),
        },
        CommandMetadata {
            name: "/context",
            aliases: &["/ctx"],
            category: CommandCategory::Repo,
            description: "Manage file context for AI prompts",
            usage: "/context [add|remove|clear|show|budget|suggest] <path>",
            examples: &["/context add src/main.rs", "/context show"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("repo"),
        },
        CommandMetadata {
            name: "/files",
            aliases: &[],
            category: CommandCategory::Repo,
            description: "Find relevant files for current task",
            usage: "/files relevant <query>",
            examples: &["/files relevant config"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("context"),
        },
        CommandMetadata {
            name: "/open",
            aliases: &["/preview"],
            category: CommandCategory::Repo,
            description: "Safely read and preview a text file",
            usage: "/open <path>",
            examples: &["/open src/main.rs"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/changes",
            aliases: &["/git-status"],
            category: CommandCategory::Repo,
            description: "Show changed git files",
            usage: "/changes",
            examples: &["/changes", "/git-status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/diff",
            aliases: &[],
            category: CommandCategory::Patches,
            description: "Show unified diff for a pending patch or local git",
            usage: "/diff [patch-id]",
            examples: &["/diff", "/diff a1b2c3d4"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Checkpoint & Git ───────────────────────────────────────────────────
        CommandMetadata {
            name: "/checkpoint",
            aliases: &[],
            category: CommandCategory::Repo,
            description: "Create or list safety checkpoints",
            usage: "/checkpoint [create|list|show|diff|restore]",
            examples: &["/checkpoint list", "/checkpoint create before-refactor"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("rollback"),
        },
        CommandMetadata {
            name: "/rollback",
            aliases: &[],
            category: CommandCategory::Repo,
            description: "Rollback to a previous checkpoint (defaults to safe plan)",
            usage: "/rollback [plan|restore|destructive] <id>",
            examples: &["/rollback plan a1b2c3d4", "/rollback destructive a1b2c3d4"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("checkpoint"),
        },
        CommandMetadata {
            name: "/branch",
            aliases: &[],
            category: CommandCategory::Repo,
            description: "Manage git branches safely",
            usage: "/branch [current|create]",
            examples: &["/branch current", "/branch create feature/auth"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false, // create requires approval internally
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/commit",
            aliases: &[],
            category: CommandCategory::Repo,
            description: "Prepare and create git commits",
            usage: "/commit [message|create]",
            examples: &["/commit message", "/commit create"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false, // create requires approval internally
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Coding ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/code",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Switch to ACT mode and run a coding task",
            usage: "/code [<description>]",
            examples: &["/code refactor auth module"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/plan",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Switch to PLAN mode for planning without execution",
            usage: "/plan [<description>]",
            examples: &["/plan design the new API"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/act",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Switch to ACT mode — allow code execution",
            usage: "/act",
            examples: &["/act"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/mode",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Show current workflow mode (PLAN / ACT)",
            usage: "/mode",
            examples: &["/mode"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/task",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Show current active coding task and its state",
            usage: "/task",
            examples: &["/task"],
            shortcut: Some("Ctrl+2"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/verify",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Run check+test+lint to verify current code state",
            usage: "/verify",
            examples: &["/verify"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/check",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Run project check command (cargo check, go build, etc.)",
            usage: "/check",
            examples: &["/check"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("project"),
        },
        CommandMetadata {
            name: "/test",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Run project test command (cargo test, pytest, etc.)",
            usage: "/test",
            examples: &["/test"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("project"),
        },
        CommandMetadata {
            name: "/lint",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Run project lint command (clippy, ruff, eslint, etc.)",
            usage: "/lint",
            examples: &["/lint"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("project"),
        },
        CommandMetadata {
            name: "/format",
            aliases: &[],
            category: CommandCategory::Coding,
            description: "Run project formatter (cargo fmt, black, prettier, etc.)",
            usage: "/format",
            examples: &["/format"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("project"),
        },
        // ── Patches ────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/patch",
            aliases: &[],
            category: CommandCategory::Patches,
            description: "Manage patches: list, show, apply, discard",
            usage: "/patch [list|show|apply|discard]",
            examples: &["/patch", "/patch list", "/patch apply", "/patch discard"],
            shortcut: Some("Ctrl+4"),
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: Some("workflow"),
        },
        // ── Memory ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/memory",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Manage memory: status, show, add-user <text>, add-note <text>",
            usage: "/memory [status|show|add-user <t>|add-note <t>]",
            examples: &[
                "/memory",
                "/memory status",
                "/memory add-note I prefer Rust",
            ],
            shortcut: Some("Ctrl+6"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/recall",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Search memory for a keyword or phrase",
            usage: "/recall <query>",
            examples: &["/recall auth", "/recall JWT verification"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/search-brain",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Alias for /brain search",
            usage: "/search-brain <query>",
            examples: &["/search-brain recipe"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/brain",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Manage unified brain index: status, index, search, semantic, hybrid, recall, embed",
            usage: "/brain [status|index|search|semantic|hybrid|recall|embed] [args]",
            examples: &[
                "/brain status",
                "/brain index",
                "/brain search rust",
                "/brain semantic setup",
                "/brain hybrid config",
                "/brain embeddings status",
                "/brain embeddings rebuild",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/learn",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Manage Brain Learning: status, extract, candidates, accept <id>, reject <id>",
            usage: "/learn [status|extract|candidates|accept <id>|reject <id>]",
            examples: &["/learn status", "/learn extract", "/learn accept 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/summary",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "Show Learning Summary of accepted, pending, rejected candidates",
            usage: "/summary",
            examples: &["/summary"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/memory-galaxy",
            aliases: &[],
            category: CommandCategory::Memory,
            description: "View the Memory Galaxy visualization and stored project memories",
            usage: "/memory-galaxy",
            examples: &["/memory-galaxy"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("memory"),
        },
        // ── Skills ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/skills",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "List available skills from ~/.config/goat/skills/",
            usage: "/skills",
            examples: &["/skills"],
            shortcut: Some("Ctrl+7"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills sources",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "List all skill sources (local, learned, studio, remote)",
            usage: "/skills sources",
            examples: &["/skills sources"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills remote",
            aliases: &["/marketplace"],
            category: CommandCategory::Skills,
            description: "Access the remote skill marketplace",
            usage: "/skills remote",
            examples: &["/skills remote"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills remote search",
            aliases: &["/marketplace search"],
            category: CommandCategory::Skills,
            description: "Search for skills in the remote marketplace",
            usage: "/skills remote search <query>",
            examples: &["/skills remote search rust"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills remote show",
            aliases: &["/marketplace show"],
            category: CommandCategory::Skills,
            description: "Show details of a remote skill",
            usage: "/skills remote show <id>",
            examples: &["/skills remote show 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills remote audit",
            aliases: &["/marketplace audit"],
            category: CommandCategory::Skills,
            description: "Audit a remote skill for security issues",
            usage: "/skills remote audit <id>",
            examples: &["/skills remote audit 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills remote install",
            aliases: &["/marketplace install"],
            category: CommandCategory::Skills,
            description: "Install a skill from the remote marketplace (Requires Approval)",
            usage: "/skills remote install <id>",
            examples: &["/skills remote install 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills installed",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "List all installed remote skills",
            usage: "/skills installed",
            examples: &["/skills installed"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills provenance",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Show metadata and provenance of an installed skill",
            usage: "/skills provenance <name>",
            examples: &["/skills provenance git-workflow"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills update",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Update an installed remote skill",
            usage: "/skills update <name>",
            examples: &["/skills update git-workflow"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Partial,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/skills uninstall",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Uninstall a local or remote skill",
            usage: "/skills uninstall <name>",
            examples: &["/skills uninstall git-workflow"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        // ── Recipes ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/recipes",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Manage workflow and automation recipes",
            usage: "/recipes",
            examples: &["/recipes"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/recipes list",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "List all installed recipes",
            usage: "/recipes list",
            examples: &["/recipes list"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes built-in",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "List built-in recipes",
            usage: "/recipes built-in",
            examples: &["/recipes built-in"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes search",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Search for recipes in the remote marketplace",
            usage: "/recipes search <query>",
            examples: &["/recipes search cargo"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes show",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Show details of a recipe",
            usage: "/recipes show <id>",
            examples: &["/recipes show builtin_1"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes audit",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Audit a recipe for security issues",
            usage: "/recipes audit <id>",
            examples: &["/recipes audit builtin_1"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes install",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Install a recipe from the marketplace (Requires Approval)",
            usage: "/recipes install <id>",
            examples: &["/recipes install builtin_1"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes enable",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Enable an installed recipe",
            usage: "/recipes enable <name>",
            examples: &["/recipes enable cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes disable",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Disable an installed recipe",
            usage: "/recipes disable <name>",
            examples: &["/recipes disable cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes uninstall",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Uninstall a recipe",
            usage: "/recipes uninstall <name>",
            examples: &["/recipes uninstall cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes provenance",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Show metadata and provenance of an installed recipe",
            usage: "/recipes provenance <name>",
            examples: &["/recipes provenance cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes plan",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "View execution plan for a recipe",
            usage: "/recipes plan <name>",
            examples: &["/recipes plan cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes activate",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Activate an enabled recipe to hook/schedule templates",
            usage: "/recipes activate <name>",
            examples: &["/recipes activate cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes deactivate",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Deactivate an active recipe",
            usage: "/recipes deactivate <name>",
            examples: &["/recipes deactivate cargo_check_on_save"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes run",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Manually run an enabled recipe",
            usage: "/recipes run <name>",
            examples: &["/recipes run summarize-jobs-daily"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes runs",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "View all recipe run logs",
            usage: "/recipes runs",
            examples: &["/recipes runs"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/recipes run-log",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "View run log for a specific recipe",
            usage: "/recipes run-log <name>",
            examples: &["/recipes run-log summarize-jobs-daily"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/agent-templates",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Manage agent templates",
            usage: "/agent-templates",
            examples: &["/agent-templates"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/agent-templates draft",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Create an agent template draft in AI Studio",
            usage: "/agent-templates draft <id>",
            examples: &["/agent-templates draft 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("recipes"),
        },
        CommandMetadata {
            name: "/skill",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Activate or manage a skill: <name>, search <q>, create <n>, path",
            usage: "/skill <name|search <q>|create <n>|path>",
            examples: &[
                "/skill rust-expert",
                "/skill search coding",
                "/skill create my-skill",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/save-skill",
            aliases: &[],
            category: CommandCategory::Skills,
            description: "Save current session as a reusable skill",
            usage: "/save-skill <name>",
            examples: &["/save-skill my-rust-workflow"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: Some("skills"),
        },
        // ── Tools ──────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/tools",
            aliases: &[],
            category: CommandCategory::Tools,
            description: "List tools: categories, doctor, audit, or by name",
            usage: "/tools [categories|doctor|audit]",
            examples: &["/tools", "/tools categories", "/tools doctor"],
            shortcut: Some("Ctrl+5"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("tool_registry"),
        },
        CommandMetadata {
            name: "/tool",
            aliases: &[],
            category: CommandCategory::Tools,
            description: "Show detailed info about a specific tool",
            usage: "/tool <name>",
            examples: &["/tool bash", "/tool write_file"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("tool_registry"),
        },
        // ── MCP ────────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/mcp",
            aliases: &[
                "/mcp status",
                "/mcp list",
                "/mcp show",
                "/mcp doctor",
                "/mcp start",
                "/mcp stop",
                "/mcp restart",
            ],
            category: CommandCategory::Mcp,
            description: "Manage MCP servers (status, list, show, start, stop, restart)",
            usage: "/mcp [action] [arg]",
            examples: &[
                "/mcp status",
                "/mcp list",
                "/mcp doctor",
                "/mcp show <name>",
                "/mcp start <name>",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("mcp"),
        },
        CommandMetadata {
            name: "/tools catalog",
            aliases: &["/tools catalog search", "/tools catalog show"],
            category: CommandCategory::Tools,
            description: "View and search the planned tool catalog",
            usage: "/tools catalog [search|show] [arg]",
            examples: &["/tools catalog", "/tools catalog search git"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("tools"),
        },
        CommandMetadata {
            name: "/tools install",
            aliases: &["/tools remove", "/tools enable", "/tools disable"],
            category: CommandCategory::Future,
            description: "Install, remove, enable, or disable a tool",
            usage: "/tools install <name>",
            examples: &["/tools install bash"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Planned,
            related: Some("tools"),
        },
        // ── Subagents ──────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/subagents",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "List registered internal subagents",
            usage: "/subagents",
            examples: &["/subagents"],
            shortcut: Some("Ctrl+8"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/subagent",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Show info about a specific subagent",
            usage: "/subagent <name>",
            examples: &["/subagent coder", "/subagent researcher"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/ask-agent",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Delegate a task to an internal subagent",
            usage: "/ask-agent <name> <task>",
            examples: &[
                "/ask-agent coder refactor auth",
                "/ask-agent researcher find JWT libs",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/review",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Ask the reviewer subagent to review current code",
            usage: "/review",
            examples: &["/review"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/debug",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Ask the debugger subagent to analyze current issues",
            usage: "/debug",
            examples: &["/debug"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/test-plan",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Ask the tester subagent to create a test plan",
            usage: "/test-plan",
            examples: &["/test-plan"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("subagents"),
        },
        // ── External Agents ────────────────────────────────────────────────────
        CommandMetadata {
            name: "/external-agents",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "List configured external agent adapters",
            usage: "/external-agents [list|status]",
            examples: &["/external-agents", "/external-agents status"],
            shortcut: Some("Ctrl+9"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/external-agent",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "Show details of a specific external agent",
            usage: "/external-agent <name>",
            examples: &["/external-agent opencode", "/external-agent claude-code"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/delegate-external",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "Delegate a task to an external agent (requires approval)",
            usage: "/delegate-external <agent> <task>",
            examples: &["/delegate-external opencode fix the build"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/external-runs",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "List recent external agent run records",
            usage: "/external-runs",
            examples: &["/external-runs"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/external-run",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "Show output of a specific external agent run",
            usage: "/external-run <id>",
            examples: &["/external-run abc123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/compare-agents",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "Run the same task on multiple external agents and compare",
            usage: "/compare-agents <agent1> <agent2> <task>",
            examples: &["/compare-agents opencode claude-code refactor auth"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Working,
            related: Some("external_agents"),
        },
        CommandMetadata {
            name: "/route",
            aliases: &[],
            category: CommandCategory::ExternalAgents,
            description: "Show swarm route decision for current input",
            usage: "/route",
            examples: &["/route"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: Some("swarm"),
        },
        // ── UI / Views ─────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/view",
            aliases: &[],
            category: CommandCategory::Ui,
            description: "Switch active view: chat|tasks|repo|patches|tools|memory|skills|subagents|external",
            usage: "/view <name>",
            examples: &["/view chat", "/view repo", "/view subagents"],
            shortcut: Some("Ctrl+1..9"),
            surface: CommandSurface::tui_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/palette",
            aliases: &["/command"],
            category: CommandCategory::Ui,
            description: "Open command palette. Optional search: /palette <query>",
            usage: "/palette [<query>]",
            examples: &["/palette", "/palette tools", "/palette memory"],
            shortcut: Some("Ctrl+P"),
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── System ─────────────────────────────────────────────────────────────
        CommandMetadata {
            name: "/exit",
            aliases: &["/quit"],
            category: CommandCategory::System,
            description: "Exit GOAT (headless only; use Ctrl+C in TUI)",
            usage: "/exit",
            examples: &["/exit"],
            shortcut: Some("Ctrl+C"),
            surface: CommandSurface::headless_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        // ── Phase 3.2: Layout & UI ──────────────────────────────────────────────
        CommandMetadata {
            name: "/layout",
            aliases: &[],
            category: CommandCategory::Ui,
            description: "Switch TUI layout mode (focus/dashboard/compact)",
            usage: "/layout [focus|dashboard|compact]",
            examples: &[
                "/layout",
                "/layout focus",
                "/layout dashboard",
                "/layout compact",
            ],
            shortcut: None,
            surface: CommandSurface::tui_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/logs",
            aliases: &["/log"],
            category: CommandCategory::Ui,
            description: "View system and tool logs",
            usage: "/logs [clear]",
            examples: &["/logs", "/view logs", "/logs clear"],
            shortcut: None,
            surface: CommandSurface::tui_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/agents",
            aliases: &["/agent-selector", "/subagent-selector", "/agent-select"],
            category: CommandCategory::Subagents,
            description: "Open agent & skill selector modal",
            usage: "/agents",
            examples: &["/agents", "/agent-selector", "/subagent-selector"],
            shortcut: None,
            surface: CommandSurface::tui_only(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/theme",
            aliases: &[],
            category: CommandCategory::Ui,
            description: "View or switch TUI color theme",
            usage: "/theme [name]",
            examples: &["/theme", "/theme goat-dark"],
            shortcut: None,
            surface: CommandSurface::all(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: None,
        },
        // ══════════════════════════════════════════════════════════════════════
        // FUTURE / PLANNED COMMANDS — NOT YET IMPLEMENTED
        // These are registered for documentation purposes only.
        // DO NOT route execution for these commands.
        // ══════════════════════════════════════════════════════════════════════
        CommandMetadata {
            name: "/branch",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Create or switch git branch",
            usage: "/branch <name>",
            examples: &["/branch feature/new-auth"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Low,
            status: CommandStatus::Planned,
            related: Some("git"),
        },
        CommandMetadata {
            name: "/commit",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Stage and commit changes with AI-generated message",
            usage: "/commit [<message>]",
            examples: &["/commit", "/commit fix auth bug"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Planned,
            related: Some("git"),
        },
        CommandMetadata {
            name: "/pr",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Create a pull request with AI-generated description",
            usage: "/pr",
            examples: &["/pr"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Planned,
            related: Some("git"),
        },
        CommandMetadata {
            name: "/checkpoint",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Create a session checkpoint (save/restore point)",
            usage: "/checkpoint [<label>]",
            examples: &["/checkpoint before-refactor"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/rollback",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Restore to a previous checkpoint",
            usage: "/rollback [<label>]",
            examples: &["/rollback"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Planned,
            related: Some("workflow"),
        },
        CommandMetadata {
            name: "/undo",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Undo the last file write or patch application",
            usage: "/undo",
            examples: &["/undo"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Planned,
            related: Some("patches"),
        },
        CommandMetadata {
            name: "/redo",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Redo the last undone action",
            usage: "/redo",
            examples: &["/redo"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("patches"),
        },
        CommandMetadata {
            name: "/retry",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Retry the last failed message or tool call",
            usage: "/retry",
            examples: &["/retry"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/compact",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Compact context — summarize history to reduce token usage",
            usage: "/compact",
            examples: &["/compact"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("memory"),
        },
        CommandMetadata {
            name: "/cost",
            aliases: &["/tokens"],
            category: CommandCategory::Future,
            description: "Show estimated token usage and cost for this session",
            usage: "/cost",
            examples: &["/cost"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/context",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Show current context window contents and token budget",
            usage: "/context",
            examples: &["/context"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/schedule",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Schedule a recurring task or reminder",
            usage: "/schedule <cron> <task>",
            examples: &["/schedule '0 9 * * 1' run tests"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/jobs",
            aliases: &["@jobs", "@run", "@status", "@resume", "@cancel", "@retry"],
            category: CommandCategory::System,
            description: "Manage Agent Execution Runtime jobs (run, list, pause, resume)",
            usage: "/jobs [run|list|show|pause|resume|cancel|retry] <args>",
            examples: &[
                "/jobs run learner 'Create roadmap'",
                "/jobs list",
                "@status",
                "@run cofounder 'Validate idea'",
            ],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/daemon",
            aliases: &[],
            category: CommandCategory::System,
            description: "Manage local GOAT daemon",
            usage: "/daemon [status|doctor]",
            examples: &["/daemon status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/dashboard",
            aliases: &[],
            category: CommandCategory::System,
            description: "Manage GOAT web dashboard",
            usage: "/dashboard [status|doctor|path|chat|repo|diffs|commands|audit|approvals]",
            examples: &["/dashboard path", "/dashboard commands"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/audit",
            aliases: &[],
            category: CommandCategory::System,
            description: "View system audit logs",
            usage: "/audit [recent]",
            examples: &["/audit recent"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("/jobs"),
        },
        CommandMetadata {
            name: "/approvals",
            aliases: &[],
            category: CommandCategory::System,
            description: "Manage approval queue and history",
            usage: "/approvals [history]",
            examples: &["/approvals history"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: Some("/dashboard"),
        },
        CommandMetadata {
            name: "/api",
            aliases: &[],
            category: CommandCategory::System,
            description: "Manage local API",
            usage: "/api [status]",
            examples: &["/api status"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/browser-auto",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Launch browser automation for a web task",
            usage: "/browser <url>",
            examples: &["/browser https://docs.rs/ratatui"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/computer-use",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Enable computer use mode (screen + mouse + keyboard control)",
            usage: "/computer-use",
            examples: &["/computer-use"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Critical,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/dashboard",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Open local web dashboard in browser",
            usage: "/dashboard",
            examples: &["/dashboard"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/voice",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Enable voice input mode (push-to-talk, local STT)",
            usage: "/voice",
            examples: &["/voice"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/export",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Export session transcript as Markdown or JSON",
            usage: "/export [json|md]",
            examples: &["/export md", "/export json"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("brain"),
        },
        CommandMetadata {
            name: "/share",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Share a session or skill via secure link",
            usage: "/share",
            examples: &["/share"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/permissions",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Manage tool and path permissions for this session",
            usage: "/permissions [list|allow <tool>|deny <tool>]",
            examples: &["/permissions list", "/permissions allow bash"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("approval"),
        },
        CommandMetadata {
            name: "/sandbox",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Enable isolated sandbox mode for all tool execution",
            usage: "/sandbox [on|off]",
            examples: &["/sandbox on"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("approval"),
        },
        CommandMetadata {
            name: "/install-tool",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Install a new MCP or community tool",
            usage: "/install-tool <name>",
            examples: &["/install-tool playwright-mcp"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::High,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/marketplace",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Browse and install community skills and tools",
            usage: "/marketplace",
            examples: &["/marketplace"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
        CommandMetadata {
            name: "/hooks",
            aliases: &[],
            category: CommandCategory::Hooks,
            description: "Configure lifecycle hooks (on-submit, on-tool-call, etc.)",
            usage: "/hooks [list|show|enable|disable|run]",
            examples: &["/hooks list", "/hooks run format-after-patch"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: None,
        },
        CommandMetadata {
            name: "/fork",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Fork the current session into a new branch",
            usage: "/fork",
            examples: &["/fork"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("brain"),
        },
        CommandMetadata {
            name: "/message",
            aliases: &[],
            category: CommandCategory::Future,
            description: "Send a direct message to another GOAT agent instance",
            usage: "/message <agent-id> <text>",
            examples: &["/message agent-2 continue the auth task"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Planned,
            related: Some("subagents"),
        },
        // ── Phase 5.3 Studio Commands ──
        CommandMetadata {
            name: "/studio",
            aliases: &[],
            category: CommandCategory::System,
            description: "Open the AI Studio in dashboard or print info",
            usage: "/studio",
            examples: &["/studio"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/studio drafts",
            aliases: &[],
            category: CommandCategory::System,
            description: "List all drafts in the AI Studio",
            usage: "/studio drafts",
            examples: &["/studio drafts"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/studio show",
            aliases: &[],
            category: CommandCategory::System,
            description: "Show a specific Studio draft",
            usage: "/studio show <id>",
            examples: &["/studio show 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Working,
            related: None,
        },
        CommandMetadata {
            name: "/studio create-skill",
            aliases: &[],
            category: CommandCategory::System,
            description: "Create a skill from a Studio draft",
            usage: "/studio create-skill <id>",
            examples: &["/studio create-skill 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/studio create-agent",
            aliases: &[],
            category: CommandCategory::System,
            description: "Create an agent from a Studio draft",
            usage: "/studio create-agent <id>",
            examples: &["/studio create-agent 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/studio create-workflow",
            aliases: &[],
            category: CommandCategory::System,
            description: "Create a workflow from a Studio draft",
            usage: "/studio create-workflow <id>",
            examples: &["/studio create-workflow 123"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: true,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: None,
        },
        CommandMetadata {
            name: "/models compare",
            aliases: &[],
            category: CommandCategory::Models,
            description: "Compare multiple models side-by-side",
            usage: "/models compare",
            examples: &["/models compare"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: None,
        },
        CommandMetadata {
            name: "/skill-builder",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Launch the interactive skill builder",
            usage: "/skill-builder",
            examples: &["/skill-builder"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("skills"),
        },
        CommandMetadata {
            name: "/agent-builder",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Launch the interactive agent builder",
            usage: "/agent-builder",
            examples: &["/agent-builder"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: Some("subagents"),
        },
        CommandMetadata {
            name: "/workflow-builder",
            aliases: &[],
            category: CommandCategory::Subagents,
            description: "Launch the interactive workflow builder",
            usage: "/workflow-builder",
            examples: &["/workflow-builder"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Planned,
            related: None,
        },
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> CommandRegistry {
        CommandRegistry::build()
    }

    #[test]
    fn test_registry_builds_without_panic() {
        let r = registry();
        assert!(!r.commands.is_empty());
    }

    #[test]
    fn test_find_by_exact_name() {
        let r = registry();
        assert!(r.find("/help").is_some());
        assert!(r.find("/status").is_some());
        assert!(r.find("/memory").is_some());
        assert!(r.find("/nonexistent").is_none());
    }

    #[test]
    fn test_find_by_alias() {
        let r = registry();
        let cmd = r.find("/command");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().name, "/palette");
    }

    #[test]
    fn test_prefix_suggest_slash_only() {
        let r = registry();
        let suggestions = r.suggest("/");
        // Should return all non-planned commands
        assert!(!suggestions.is_empty());
        // Planned commands should not appear
        for s in &suggestions {
            assert_ne!(s.status, CommandStatus::Planned);
        }
    }

    #[test]
    fn test_prefix_suggest_help() {
        let r = registry();
        let suggestions = r.suggest("/he");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|c| c.name == "/help"));
    }

    #[test]
    fn test_prefix_suggest_empty_returns_all_working() {
        let r = registry();
        let s = r.suggest("");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_search_finds_by_description() {
        let r = registry();
        let results = r.search("session", false);
        assert!(!results.is_empty());
        // /sessions and /new should both appear
        let names: Vec<&str> = results.iter().map(|c| c.name).collect();
        assert!(names.contains(&"/sessions") || names.contains(&"/new"));
    }

    #[test]
    fn test_search_finds_by_category() {
        let r = registry();
        let results = r.search("memory", false);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_include_planned() {
        let r = registry();
        let without = r.search("future planned", false);
        let with_planned = r.search("git", true);
        // /branch, /commit, /pr are planned git commands
        assert!(with_planned.iter().any(|c| c.name == "/branch"));
        let _ = without; // just ensure no panic
    }

    #[test]
    fn test_planned_commands_not_visible() {
        let r = registry();
        for cmd in r.all(false) {
            assert_ne!(
                cmd.status,
                CommandStatus::Planned,
                "Planned command {} should not appear in non-planned listing",
                cmd.name
            );
        }
    }

    #[test]
    fn test_all_include_planned_has_more() {
        let r = registry();
        let without = r.all(false).len();
        let with_planned = r.all(true).len();
        assert!(with_planned > without);
    }

    #[test]
    fn test_category_grouping() {
        let r = registry();
        let groups = r.grouped(false);
        assert!(!groups.is_empty());
        // All groups should have at least one command
        for (cat, cmds) in &groups {
            assert!(!cmds.is_empty(), "Category {:?} is empty", cat);
        }
    }

    #[test]
    fn test_complete_returns_first_match() {
        let r = registry();
        let c = r.complete("/st");
        assert_eq!(c, Some("/status"));
    }

    #[test]
    fn test_complete_no_match_returns_none() {
        let r = registry();
        let c = r.complete("/xyznonexistent");
        assert!(c.is_none());
    }

    #[test]
    fn test_no_planned_command_in_suggestions() {
        let r = registry();
        // /fork is planned — should NOT appear in suggestions
        let suggestions = r.suggest("/fork");
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_format_help_output_not_empty() {
        let r = registry();
        let help = r.format_help(false);
        assert!(!help.is_empty());
    }

    #[test]
    fn test_format_palette_output_not_empty() {
        let r = registry();
        let palette = r.format_palette(None);
        assert!(!palette.is_empty());
    }

    #[test]
    fn test_format_palette_with_filter() {
        let r = registry();
        let palette = r.format_palette(Some("memory"));
        assert!(!palette.is_empty());
        // Should contain memory-related commands
        let output = palette.join("\n");
        assert!(output.contains("memory") || output.contains("recall"));
    }

    #[test]
    fn test_matches_prefix_case_insensitive() {
        let r = registry();
        let cmd = r.find("/help").unwrap();
        assert!(cmd.matches_prefix("/he"));
        assert!(cmd.matches_prefix("/HE"));
        assert!(cmd.matches_prefix("/HELP"));
        assert!(!cmd.matches_prefix("/status"));
    }

    #[test]
    fn test_all_commands_have_name_and_description() {
        let r = registry();
        for cmd in r.all(true) {
            assert!(!cmd.name.is_empty(), "Command has empty name");
            assert!(
                cmd.name.starts_with('/'),
                "Command {} doesn't start with /",
                cmd.name
            );
            assert!(
                !cmd.description.is_empty(),
                "Command {} has empty description",
                cmd.name
            );
        }
    }

    #[test]
    fn test_dangerous_commands_have_approval() {
        let r = registry();
        for cmd in r.all(true) {
            if matches!(cmd.risk, CommandRisk::High | CommandRisk::Critical) {
                assert!(
                    cmd.requires_approval,
                    "High/Critical command {} must require approval",
                    cmd.name
                );
            }
        }
    }
}
