//! CLI argument parsing for GOAT using `clap`.
//!
//! Defines the top-level CLI structure and handles all non-TUI subcommands.
//!
//! # Mode selection
//!
//! | Invocation                            | Mode                      |
//! |---------------------------------------|---------------------------|
//! | `goat`                                | Interactive TUI           |
//! | `goat --headless`                     | Headless stdin/stdout     |
//! | `goat --profile <name>`               | TUI with specific profile |
//! | `goat --headless --profile <name>`    | Headless + profile        |
//! | `goat doctor`                         | Print readiness report    |
//! | `goat config-path`                    | Print config path         |
//! | `goat data-path`                      | Print data dir            |
//! | `goat db-path`                        | Print database path       |
//! | `goat sessions`                       | List recent sessions      |
//! | `goat new-session`                    | Create a new session      |
//! | `goat migrate-db`                     | Migrate legacy DB         |
//! | `goat models`                         | List providers/profiles   |

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// GOAT — General Omniscient Agentic Tool
///
/// Universal AI CLI/TUI agent platform.
/// Run without arguments to launch the interactive TUI.
/// Use --headless for non-TUI mode.
#[derive(Parser, Debug)]
#[command(
    name = "goat",
    version = env!("CARGO_PKG_VERSION"),
    about = "GOAT — Universal AI CLI/TUI agent platform",
    long_about = "GOAT (General Omniscient Agentic Tool) is a Rust-first, terminal-native \
                  AI agent platform.\n\n\
                  Modes:\n  \
                    goat              Start interactive TUI\n  \
                    goat --headless   Start headless stdin/stdout mode\n  \
                    goat doctor       System readiness check\n  \
                    goat sessions     List recent sessions\n\n\
                  Paths:\n  \
                    Config:   ~/.config/goat/goat.toml\n  \
                    Data:     ~/.local/share/goat/\n  \
                    Database: ~/.local/share/goat/goat.db\n  \
                    Logs:     ~/.local/share/goat/logs/"
)]
pub struct Cli {
    /// Path to a custom config file (overrides ~/.config/goat/goat.toml).
    #[arg(long, value_name = "PATH", global = true)]
    pub config: Option<PathBuf>,

    /// Path to a custom brain database file (overrides XDG data path).
    #[arg(long, value_name = "PATH", global = true)]
    pub db: Option<PathBuf>,

    /// Run in headless mode: read from stdin, print to stdout. No TUI.
    #[arg(long, global = true)]
    pub headless: bool,

    /// Disable brain (SQLite memory). Runs without persistent session storage.
    /// History is ephemeral and lost when GOAT exits.
    #[arg(long, global = true)]
    pub no_brain: bool,

    /// Select a model profile by name (e.g. balanced, coding, cheap, powerful).
    /// Overrides the default profile from goat.toml.
    /// Run `goat models` to list available profiles.
    #[arg(long, value_name = "PROFILE", global = true)]
    pub profile: Option<String>,

    /// Subcommand to run. If omitted, the TUI (or --headless) mode is used.
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print the path of the active config file and exit.
    #[command(name = "config-path")]
    ConfigPath,

    /// Print the active data directory path and exit.
    #[command(name = "data-path")]
    DataPath,

    /// Print the active brain database path and exit.
    #[command(name = "db-path")]
    DbPath,

    /// Check system readiness and print a health report.
    ///
    /// Checks: OS, GOAT version, config file + permissions, data directory,
    /// database, legacy DB migration status, provider keys, profile + chain,
    /// ApprovalGate, headless readiness, log directory.
    #[command(name = "doctor")]
    Doctor,

    /// Migrate the legacy project-root goat_brain.db to the XDG data path.
    ///
    /// Copies ./goat_brain.db → XDG path. Original is NOT deleted.
    #[command(name = "migrate-db")]
    MigrateDb,

    /// List recent sessions from the brain database.
    ///
    /// Shows session ID, title, timestamps, and UUID/legacy classification.
    #[command(name = "sessions")]
    Sessions,

    /// Create a new session and print its ID.
    ///
    /// Does not destroy old sessions. The new session UUID is printed to stdout.
    #[command(name = "new-session")]
    NewSession,

    /// List and switch model profiles and providers.
    #[command(name = "models")]
    Models {
        /// Optional specific action (e.g., 'list', 'status')
        action: Option<String>,
    },

    /// Manage tools, permissions, and tool registry.
    #[command(name = "tools")]
    Tools {
        /// Action to perform: list, show, categories, doctor, audit.
        #[arg(default_value = "list")]
        action: String,
        /// Optional argument for the action (e.g. tool name).
        arg: Option<String>,
    },

    /// Internal Subagent Framework management.
    Subagents {
        /// Action to perform: list, show, audit.
        #[arg(default_value = "list")]
        action: String,
        /// Optional argument for the action (e.g. subagent name).
        arg: Option<String>,
    },

    /// Run a single internal subagent turn.
    AskAgent {
        /// The name of the subagent to run.
        name: String,
        /// Task for the agent.
        task: String,
    },

    /// Manage External Agent Adapters (Phase 2.8).
    #[command(name = "external-agents")]
    ExternalAgents {
        /// Action: list, detect, doctor, audit, show.
        #[arg(default_value = "list")]
        action: String,
        /// Target agent name for 'show'.
        arg: Option<String>,
    },

    /// Delegate a task to an external agent.
    #[command(name = "delegate-external")]
    DelegateExternal {
        /// External agent name.
        agent: String,
        /// Task summary/prompt.
        task: String,
    },

    #[command(name = "mcp")]
    Mcp {
        /// Action to perform: status, list, show, doctor, start, stop, restart.
        #[arg(default_value = "status")]
        action: String,
        /// Target server name for 'show', 'start', 'stop', 'restart'.
        arg: Option<String>,
    },

    /// Manage hooks
    #[command(name = "hooks")]
    Hooks {
        /// Action to perform: list, show, enable, disable, run
        #[arg(default_value = "list")]
        action: String,
        /// Hook name
        arg: Option<String>,
    },

    /// Manage scheduled tasks
    #[command(name = "schedule")]
    Schedule {
        /// Action to perform: list, add, show, enable, disable, run, delete
        #[arg(default_value = "list")]
        action: String,
        /// Additional arguments depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Manage background jobs
    #[command(name = "jobs")]
    Jobs {
        /// Action to perform: list, show, cancel
        #[arg(default_value = "list")]
        action: String,
        /// Job ID
        arg: Option<String>,
    },

    /// Manage GOAT Daemon
    #[command(name = "daemon")]
    Daemon {
        /// Action to perform: start, status, stop, doctor
        #[arg(default_value = "start")]
        action: String,
    },

    /// Show project awareness status or scan the current directory.
    #[command(name = "project")]
    Project {
        /// "status" (default) or "scan"
        #[arg(default_value = "status")]
        action: String,
    },

    /// Manage GOAT curated memory files.
    #[command(name = "memory")]
    Memory {
        /// "status", "show", "path", "edit", "add-user", or "add-note"
        action: String,
        /// The text to add (for add-user and add-note)
        text: Option<String>,
    },

    /// Search past conversation interactions.
    #[command(name = "recall")]
    Recall { query: String },

    /// Manage GOAT reusable skills.
    #[command(name = "skills")]
    Skills {
        /// "list", "show", "path", "create", "validate", "search", "create-from-session"
        #[arg(default_value = "list")]
        action: String,
        /// The name or query
        arg: Option<String>,
        /// Session ID to extract from (for create-from-session)
        #[arg(long)]
        session: Option<String>,
    },

    /// Show or refresh the repo map for the current project.
    ///
    /// goat repo-map          → show cached or auto-scan
    /// goat repo-map refresh  → force rescan
    /// goat repo-map show     → show compact repo map
    #[command(name = "repo-map")]
    RepoMap {
        /// "show" (default), "refresh"
        #[arg(default_value = "show")]
        action: String,
    },

    /// Run the project's check command (e.g. cargo check, tsc, go build).
    ///
    /// Command is detected from the project. Requires approval before execution.
    #[command(name = "check")]
    Check,

    /// Run the project's test command (e.g. cargo test, pytest, npm test).
    ///
    /// Command is detected from the project. Requires approval before execution.
    #[command(name = "test")]
    Test {
        /// Optional test filter / extra args passed to the test runner.
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Run the project's lint command (e.g. cargo clippy, eslint, ruff).
    ///
    /// Command is detected from the project. Requires approval before execution.
    #[command(name = "lint")]
    Lint,

    /// Run the project's format command (e.g. cargo fmt, prettier, ruff format).
    ///
    /// Command is detected from the project. Requires approval before execution.
    #[command(name = "format")]
    Format,

    /// Inspect or manage pending code patches.
    ///
    /// goat patch          → show pending patch (if any)
    /// goat patch apply    → apply the pending patch (requires approval)
    /// goat patch discard  → discard pending patch
    #[command(name = "patch")]
    Patch {
        /// "show" (default), "apply", or "discard"
        #[arg(default_value = "show")]
        action: String,
    },

    /// Manage safety checkpoints.
    #[command(name = "checkpoint")]
    Checkpoint {
        /// "list" (default), "create", "show", "diff"
        #[arg(default_value = "list")]
        action: String,
        /// Optional argument (e.g. label for create, ID for show)
        arg: Option<String>,
    },

    /// Rollback to a specific checkpoint.
    #[command(name = "rollback")]
    Rollback {
        /// Checkpoint ID
        id: String,
    },

    /// Manage git branches safely.
    #[command(name = "branch")]
    Branch {
        /// "current" (default), "create"
        #[arg(default_value = "current")]
        action: String,
        /// Branch name
        name: Option<String>,
    },

    /// Prepare and create git commits.
    #[command(name = "commit")]
    Commit {
        /// "message" (default), "create"
        #[arg(default_value = "message")]
        action: String,
    },
}

/// Handle CLI subcommands that do not need TUI or headless mode.
///
/// Returns `true` if a subcommand was handled (caller should exit after),
/// `false` if the TUI or headless loop should be launched.
pub async fn handle_subcommand(
    cli: &Cli,
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
) -> anyhow::Result<bool> {
    let Some(ref cmd) = cli.command else {
        return Ok(false);
    };

    match cmd {
        Command::ConfigPath => {
            println!("{}", paths.config_file.display());
            Ok(true)
        }

        Command::DataPath => {
            println!("{}", paths.data_dir.display());
            Ok(true)
        }

        Command::DbPath => {
            println!("{}", paths.db_file.display());
            Ok(true)
        }

        Command::Doctor => {
            let checks = crate::paths::run_doctor(paths, config, cli.headless);
            crate::paths::print_doctor_results(&checks);
            Ok(true)
        }

        Command::MigrateDb => {
            handle_migrate_db(paths)?;
            Ok(true)
        }

        Command::Sessions => {
            handle_sessions_command(paths)?;
            Ok(true)
        }

        Command::NewSession => {
            handle_new_session_command(paths)?;
            Ok(true)
        }

        Command::Models { action: _ } => {
            handle_models_command(config);
            Ok(true)
        }
        Command::Project { action } => {
            handle_project_command(paths, config, action)?;
            Ok(true)
        }
        Command::Memory { action, text } => {
            handle_memory_command(paths, config, action, text.as_deref())?;
            Ok(true)
        }
        Command::Recall { query } => {
            handle_recall_command(paths, query)?;
            Ok(true)
        }

        Command::Skills {
            action,
            arg,
            session,
        } => {
            handle_skills_command(paths, config, action, arg.as_deref(), session.as_deref())
                .await?;
            Ok(true)
        }

        Command::RepoMap { action } => {
            handle_repo_map_command(paths, config, action)?;
            Ok(true)
        }

        Command::Check => {
            handle_dev_command("check")?;
            Ok(true)
        }

        Command::Test { args } => {
            let extra = args.join(" ");
            handle_dev_command_with_args(
                "test",
                if extra.is_empty() { None } else { Some(&extra) },
            )?;
            Ok(true)
        }

        Command::Lint => {
            handle_dev_command("lint")?;
            Ok(true)
        }

        Command::Format => {
            handle_dev_command("format")?;
            Ok(true)
        }

        Command::Patch { action } => {
            handle_patch_command(action);
            Ok(true)
        }

        Command::Daemon { action } => {
            handle_daemon_command(paths, config, action).await?;
            Ok(true)
        }

        Command::Checkpoint { action, arg } => {
            let root = std::env::current_dir().unwrap_or_default();
            let manager = crate::checkpoint::CheckpointManager::new(&paths.data_dir);
            match action.as_str() {
                "create" => {
                    let label = arg.as_deref().unwrap_or("manual");
                    match manager.create_checkpoint(&root, label) {
                        Ok(cp) => println!("Created checkpoint {} ({})", cp.id, cp.label),
                        Err(e) => eprintln!("Failed to create checkpoint: {}", e),
                    }
                }
                "list" => match manager.list_checkpoints() {
                    Ok(cps) => {
                        if cps.is_empty() {
                            println!("No checkpoints found.");
                        } else {
                            println!("{} checkpoints:", cps.len());
                            for cp in cps {
                                println!(
                                    "  {} | {} | {} files | {}",
                                    cp.id,
                                    cp.branch,
                                    cp.changed_files.len(),
                                    cp.label
                                );
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to list checkpoints: {}", e),
                },
                "show" | "diff" => {
                    if let Some(id) = arg {
                        match manager.get_checkpoint(id) {
                            Ok(Some(cp)) => {
                                println!("{} | {} | dirty: {}", cp.id, cp.branch, cp.is_dirty);
                                println!("Label: {}", cp.label);
                                println!("Changed files: {}", cp.changed_files.join(", "));
                                if action == "diff" && !cp.diff_snapshot.is_empty() {
                                    println!("\nDiff Snapshot:\n{}", cp.diff_snapshot);
                                }
                            }
                            Ok(None) => println!("Checkpoint ID {} not found.", id),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        println!("Please provide a checkpoint ID.");
                    }
                }
                _ => println!("Unknown action. Use list, create, show, diff."),
            }
            Ok(true)
        }

        Command::Rollback { id } => {
            println!("Rollback via CLI defaults to 'plan' mode to prevent accidental data loss.");
            println!(
                "To safely restore or perform a destructive rollback, launch GOAT (cargo run) and type:"
            );
            println!("  /rollback plan {}", id);
            println!("  /rollback restore {}", id);
            println!("  /rollback destructive {}", id);
            Ok(true)
        }

        Command::Branch { action, name } => {
            let root = std::env::current_dir().unwrap_or_default();
            match action.as_str() {
                "current" => {
                    if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                        println!("Current branch: {}", git.branch);
                    } else {
                        println!("Not in a git repository.");
                    }
                }
                "create" => {
                    if let Some(n) = name {
                        println!(
                            "Branch creation requires approval. Please run GOAT interactively and type /branch create {}",
                            n
                        );
                    } else {
                        println!("Please specify a branch name.");
                    }
                }
                _ => println!("Unknown action. Use current or create."),
            }
            Ok(true)
        }

        Command::Commit { action } => {
            match action.as_str() {
                "message" => {
                    let root = std::env::current_dir().unwrap_or_default();
                    let status_out = std::process::Command::new("git")
                        .args(["-C", &root.to_string_lossy(), "status", "--short"])
                        .output()
                        .ok()
                        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                        .unwrap_or_default();

                    let diff_out = std::process::Command::new("git")
                        .args(["-C", &root.to_string_lossy(), "diff", "--cached", "--stat"])
                        .output()
                        .ok()
                        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                        .unwrap_or_default();

                    if status_out.trim().is_empty() {
                        println!("No changes detected. Working tree clean.");
                    } else {
                        println!("Proposed deterministic commit message:\n");
                        println!("feat: Update project files\n");
                        for line in status_out.lines().filter(|l| !l.trim().is_empty()) {
                            println!("- {}", line.trim());
                        }
                        if !diff_out.trim().is_empty() {
                            println!("\nDiff stat:\n{}", diff_out.trim());
                        }
                    }
                }
                "create" => {
                    println!(
                        "Commit creation requires approval. Please run GOAT interactively and type /commit create"
                    );
                }
                _ => println!("Unknown action. Use message or create."),
            }
            Ok(true)
        }

        Command::Tools { action, arg } => {
            handle_tools_command(paths, config, &action, arg.as_deref())?;
            Ok(true)
        }
        Command::Subagents { action, arg } => {
            handle_subagents_command(paths, config, &action, arg.as_deref())?;
            Ok(true)
        }
        Command::AskAgent { name, task } => {
            let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                config.clone(),
                paths.clone(),
                vec![],
                false,
                None,
            );
            handle_ask_agent_command(&name, &task, &rt).await?;
            Ok(true)
        }
        Command::ExternalAgents { action, arg } => {
            let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                config.clone(),
                paths.clone(),
                vec![],
                false,
                None,
            );
            handle_external_agents_command(rt, &action, arg.as_deref());
            Ok(true)
        }

        Command::DelegateExternal { agent, task } => {
            let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                config.clone(),
                paths.clone(),
                vec![],
                false,
                None,
            );
            handle_delegate_external_command(rt, &agent, &task).await;
            Ok(true)
        }
        Command::Mcp { action, arg } => {
            handle_mcp_command(paths, config, action, arg)?;
            Ok(true)
        }
        Command::Hooks { action, arg } => {
            handle_hooks_command(paths, config, &action, arg.as_deref())?;
            Ok(true)
        }
        Command::Schedule { action, args } => {
            handle_schedule_command(paths, config, &action, &args)?;
            Ok(true)
        }
        Command::Jobs { action, arg } => {
            handle_jobs_command(paths, config, &action, arg.as_deref())?;
            Ok(true)
        }
    }
}

// ── sessions command ──────────────────────────────────────────────────────────

fn handle_sessions_command(paths: &crate::paths::GoatPaths) -> anyhow::Result<()> {
    use anyhow::Context;

    if !paths.db_file.exists() {
        println!("No brain database found at {}", paths.db_file.display());
        println!("Run `goat` to create your first session.");
        return Ok(());
    }

    let brain = crate::brain::Brain::new(&paths.db_file)
        .with_context(|| format!("could not open database: {}", paths.db_file.display()))?;

    let records = brain
        .get_session_records()
        .context("could not read sessions from database")?;

    if records.is_empty() {
        println!("No sessions found in {}", paths.db_file.display());
        return Ok(());
    }

    println!("Sessions ({}):", records.len());
    println!("{}", "─".repeat(78));
    println!(
        "  {:<10}  {:<5}  {:<20}  {:<20}  {}",
        "ID", "Type", "Created", "Updated", "Title"
    );
    println!("{}", "─".repeat(78));
    for rec in &records {
        let short_id = if rec.id.len() > 8 {
            format!("{}…", &rec.id[..8])
        } else {
            rec.id.clone()
        };
        let kind = if rec.is_uuid() { "uuid" } else { "legacy" };
        // Trim datetime to just the date+time without fractional seconds.
        let created = rec.created_at.get(..16).unwrap_or(&rec.created_at);
        let updated = rec.updated_at.get(..16).unwrap_or(&rec.updated_at);
        let title = if rec.title.len() > 28 {
            format!("{}…", &rec.title[..27])
        } else {
            rec.title.clone()
        };
        println!(
            "  {:<10}  {:<5}  {:<20}  {:<20}  {}",
            short_id, kind, created, updated, title
        );
    }
    println!("{}", "─".repeat(78));
    println!("Database: {}", paths.db_file.display());
    Ok(())
}

// ── new-session command ────────────────────────────────────────────────────────

fn handle_new_session_command(paths: &crate::paths::GoatPaths) -> anyhow::Result<()> {
    use anyhow::Context;
    use uuid::Uuid;

    let session_id = Uuid::new_v4().to_string();

    if paths.db_file.exists() {
        let brain = crate::brain::Brain::new(&paths.db_file)
            .with_context(|| format!("could not open database: {}", paths.db_file.display()))?;
        brain
            .create_session(&session_id, "New Session")
            .context("could not create session")?;
        println!("{}", session_id);
        eprintln!("[GOAT] New session created: {}", session_id);
        eprintln!("[GOAT] Database: {}", paths.db_file.display());
    } else {
        // No DB yet — just print the UUID (it will be created on first run).
        println!("{}", session_id);
        eprintln!(
            "[GOAT] No brain database yet. Session ID reserved: {}",
            session_id
        );
        eprintln!("[GOAT] Run `goat` to start and persist this session.");
    }

    Ok(())
}

// ── tools command ─────────────────────────────────────────────────────────────

fn handle_tools_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
) -> anyhow::Result<()> {
    let registry = crate::tool_registry::ToolRegistry::new();

    match action {
        "list" => {
            println!("GOAT Tool Registry ({} tools)", registry.list_all().len());
            println!("{:-<80}", "");
            println!(
                "{:<20} | {:<15} | {:<10} | {:<10} | {}",
                "Name", "Category", "Risk", "Approval", "Permission"
            );
            println!("{:-<80}", "");

            for tool in registry.list_all() {
                let perm = registry.get_permission(&tool.name, &config.tools);
                let approval = if tool.requires_approval {
                    "Required"
                } else {
                    "None"
                };
                println!(
                    "{:<20} | {:<15} | {:<10} | {:<10} | {:?}",
                    tool.name,
                    tool.category.to_string(),
                    tool.risk_level.to_string(),
                    approval,
                    perm
                );
            }
        }
        "show" => {
            if let Some(name) = arg {
                if let Some(tool) = registry.get(name) {
                    println!("Tool: {}", tool.name);
                    println!("Description: {}", tool.description);
                    println!("Category: {}", tool.category);
                    println!("Risk Level: {}", tool.risk_level);
                    println!("Requires Approval: {}", tool.requires_approval);
                    println!("Read Only: {}", tool.read_only);
                    println!("Permission Group: {}", tool.permission_group);
                    println!(
                        "Effective Permission: {:?}",
                        registry.get_permission(&tool.name, &config.tools)
                    );
                    println!(
                        "Effective Action: {:?}",
                        registry.evaluate_action(&tool.name, &config.tools)
                    );
                } else {
                    println!("Tool '{}' not found.", name);
                }
            } else {
                println!("Please provide a tool name. Example: goat tools show bash");
            }
        }
        "categories" => {
            println!("Tool Categories:");
            println!("- filesystem");
            println!("- shell");
            println!("- project");
            println!("- subagent");
            // etc
        }
        "doctor" => {
            let tools = registry.list_all();
            println!("Tool Registry Doctor:");
            println!("  Total tools: {}", tools.len());
            println!(
                "  High/Critical risk tools: {}",
                tools
                    .iter()
                    .filter(|t| t.risk_level == crate::approval::RiskLevel::High
                        || t.risk_level == crate::approval::RiskLevel::Critical)
                    .count()
            );
            println!(
                "  Tool audit log path: {}",
                paths.tool_audit_log_file.display()
            );
            println!(
                "  Permission configuration enabled: {}",
                config.tools.enabled
            );
        }
        "audit" => {
            if paths.tool_audit_log_file.exists() {
                match std::fs::read_to_string(&paths.tool_audit_log_file) {
                    Ok(content) => println!("{}", content),
                    Err(e) => println!("Failed to read audit log: {}", e),
                }
            } else {
                println!(
                    "No audit log found at {}.",
                    paths.tool_audit_log_file.display()
                );
            }
        }
        "catalog" => {
            println!("GOAT Tool Catalog (Phase 3.7 Foundation)");
            println!("Status: Informational only. No automatic installation yet.");
            if paths.tool_catalog_file.exists() {
                println!("Catalog loaded from: {}", paths.tool_catalog_file.display());
            } else {
                println!(
                    "Catalog not found at {}. Using default docs catalog.",
                    paths.tool_catalog_file.display()
                );
            }
            if let Some(a) = arg {
                let parts: Vec<&str> = a.splitn(2, ' ').collect();
                if parts[0] == "search" {
                    println!("Searching catalog for: {}", parts.get(1).unwrap_or(&""));
                } else if parts[0] == "show" {
                    println!("Showing catalog entry for: {}", parts.get(1).unwrap_or(&""));
                } else {
                    println!("Unknown catalog action: {}", parts[0]);
                }
            } else {
                println!("Available Planned Categories:");
                println!("- filesystem MCP, git tools, browser automation, web search,");
                println!("  Playwright/browser-use, image generation, TTS/STT,");
                println!("  database tools, GitHub tools, calendar/email tools, local shell");
            }
        }
        "install" | "enable" | "disable" | "remove" => {
            println!("Tool/MCP {} is planned for Phase 3.8.", action);
            println!(
                "No automatic installation yet. Future installs require approval and sandbox checks."
            );
            if let Some(a) = arg {
                println!("Target: {}", a);
            }
        }
        _ => {
            println!(
                "Unknown action '{}'. Expected: list, show, categories, doctor, audit, catalog, install, enable, disable.",
                action
            );
        }
    }

    Ok(())
}

// ── mcp command ───────────────────────────────────────────────────────────────

fn handle_mcp_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &String,
    arg: &Option<String>,
) -> anyhow::Result<()> {
    match action.as_str() {
        "status" => {
            println!("MCP Status (Phase 3.7 Foundation)");
            let mcp_conf_exists = paths.mcp_json_file.exists() || paths.mcp_toml_file.exists();
            println!(
                "MCP config paths: {} / {}",
                paths.mcp_json_file.display(),
                paths.mcp_toml_file.display()
            );
            println!(
                "MCP config exists: {}",
                if mcp_conf_exists { "yes" } else { "no" }
            );
            let enabled_count = config.mcp_servers.values().filter(|s| s.enabled).count();
            let risky_count = config
                .mcp_servers
                .values()
                .filter(|s| s.risk == "ask" || s.risk == "deny")
                .count();
            println!("Configured servers: {}", config.mcp_servers.len());
            println!("Enabled servers: {}", enabled_count);
            println!("Risky servers: {}", risky_count);
            println!("Execution status: allowed (requires ApprovalGate)");
        }
        "list" => {
            if config.mcp_servers.is_empty() {
                println!("No MCP servers configured.");
                return Ok(());
            }
            println!("{:-<80}", "");
            println!(
                "{:<15} | {:<8} | {:<10} | {:<8} | {}",
                "Server Name", "Enabled", "Transport", "Risk", "Command"
            );
            println!("{:-<80}", "");
            for (name, srv) in &config.mcp_servers {
                println!(
                    "{:<15} | {:<8} | {:<10} | {:<8} | {}",
                    name, srv.enabled, srv.transport, srv.risk, srv.command
                );
            }
            println!("{:-<80}", "");
        }
        "show" => {
            let Some(name) = arg else {
                println!("Usage: goat mcp show <name>");
                return Ok(());
            };
            if let Some(srv) = config.mcp_servers.get(name) {
                println!("MCP Server: {}", name);
                println!("Enabled: {}", srv.enabled);
                println!("Transport: {}", srv.transport);
                println!("Risk Policy: {}", srv.risk);
                println!("Command: {}", srv.command);
                println!("Args: {:?}", srv.args);
                println!(
                    "Env Vars Configured: {:?}",
                    srv.env.keys().collect::<Vec<_>>()
                );
            } else {
                println!("MCP server '{}' not found in config.", name);
            }
        }
        "start" | "stop" | "restart" => {
            let Some(name) = arg else {
                println!("Usage: goat mcp {} <name>", action);
                return Ok(());
            };
            println!(
                "Lifecycle action '{}' for MCP server '{}' is planned/partial.",
                action, name
            );
            println!(
                "Currently waiting for full MCP client lifecycle + ApprovalGate integration in Phase 3.8."
            );
        }
        "doctor" => {
            println!("MCP Doctor (Phase 3.7)");
            let mcp_conf_exists = paths.mcp_json_file.exists() || paths.mcp_toml_file.exists();
            println!(
                "[*] Config paths checked: {} / {}",
                paths.mcp_json_file.display(),
                paths.mcp_toml_file.display()
            );
            println!(
                "[*] Config exists: {}",
                if mcp_conf_exists { "yes" } else { "no" }
            );
            println!("[*] Configured servers: {}", config.mcp_servers.len());
            println!(
                "[*] Tool catalog path: {}",
                paths.tool_catalog_file.display()
            );
            println!(
                "[*] Tool catalog exists: {}",
                if paths.tool_catalog_file.exists() {
                    "yes"
                } else {
                    "no"
                }
            );
        }
        _ => {
            println!(
                "Unknown action '{}'. Expected: status, list, show, start, stop, restart, doctor.",
                action
            );
        }
    }

    Ok(())
}

// ── migrate-db command ────────────────────────────────────────────────────────

fn handle_migrate_db(paths: &crate::paths::GoatPaths) -> anyhow::Result<()> {
    use anyhow::Context;

    let legacy = crate::paths::GoatPaths::detect_legacy_db();

    let Some(legacy_path) = legacy else {
        println!("No legacy database found at ./goat_brain.db — nothing to migrate.");
        return Ok(());
    };

    if paths.db_file.exists() {
        println!(
            "Target database already exists: {}",
            paths.db_file.display()
        );
        println!("To replace it, manually delete it first and re-run migrate-db.");
        return Ok(());
    }

    paths.ensure_data_dir().with_context(|| {
        format!(
            "could not create data directory: {}",
            paths.data_dir.display()
        )
    })?;

    std::fs::copy(&legacy_path, &paths.db_file).with_context(|| {
        format!(
            "failed to copy {} to {}",
            legacy_path.display(),
            paths.db_file.display()
        )
    })?;

    println!(
        "Migration successful: {} → {}",
        legacy_path.display(),
        paths.db_file.display()
    );
    println!(
        "The original file at {} was NOT deleted. Remove it manually when ready.",
        legacy_path.display()
    );
    Ok(())
}

// ── models command ────────────────────────────────────────────────────────────

fn handle_models_command(config: &crate::config::Config) {
    use crate::models::ProfileRegistry;

    let registry = ProfileRegistry::from_config(&config.profiles);

    // Build router from full config (includes OpenRouter, Ollama keys).
    let router = crate::llm::LlmRouter::from_config(config);

    println!("GOAT Model Providers & Profiles");
    println!("{}", "─".repeat(72));

    // Provider status (never print keys).
    println!("Providers:");
    for provider in &["openai", "groq", "openrouter", "ollama"] {
        let implemented = router.is_provider_implemented(provider);
        let available = router.is_provider_available(provider);
        let status_icon = if available {
            "✓"
        } else if implemented {
            "✗"
        } else {
            "~"
        };
        println!(
            "  {} {:12} {}",
            status_icon,
            provider,
            router.provider_status_label(provider)
        );
    }
    println!("  ~ {:12} planned — not implemented", "anthropic");
    println!("  ~ {:12} planned — not implemented", "gemini");
    println!();

    // LLM retry/timeout config.
    println!("LLM config:");
    println!(
        "  max_retries           : {}",
        config.llm.effective_max_retries()
    );
    println!(
        "  timeout_secs          : {}",
        config.llm.effective_timeout_secs()
    );
    println!(
        "  fallback_on_rate_limit: {}",
        config.llm.fallback_on_rate_limit
    );
    println!(
        "  fallback_on_network   : {}",
        config.llm.fallback_on_network
    );
    println!(
        "  fallback_on_5xx       : {}",
        config.llm.fallback_on_server_error
    );
    println!();

    // Profile list.
    println!("Default profile: {}", registry.default_profile);
    println!();
    println!("Profiles:");
    println!("{}", "─".repeat(72));
    for name in registry.profile_names() {
        let (_, chain) = registry.resolve(name);
        let primary = chain.primary_display();
        let fallback = chain.fallback_display();
        let primary_status = if let Some(e) = chain.entries.first() {
            if router.is_provider_available(&e.provider) {
                "✓"
            } else {
                "✗"
            }
        } else {
            "✗"
        };
        let default_marker = if name == registry.default_profile {
            " (default)"
        } else {
            ""
        };
        println!(
            "  {:12} primary: {} {}{}",
            name, primary_status, primary, default_marker
        );
        if fallback != "none" {
            let fallback_status = if let Some(e) = chain.entries.get(1) {
                if router.is_provider_available(&e.provider) {
                    "✓"
                } else {
                    "✗"
                }
            } else {
                "✗"
            };
            println!("  {:12} fallback: {} {}", "", fallback_status, fallback);
        }
    }
    println!("{}", "─".repeat(72));
    println!();
    println!("Legend: ✓ = ready  ✗ = key missing  ~ = planned/not-implemented");
    println!("Config:  {}", "~/.config/goat/goat.toml");
    println!("Usage:   goat --profile <name>  |  TUI: /profile <name>  |  /profiles");
}

fn handle_project_command(
    paths: &crate::paths::GoatPaths,
    _config: &crate::config::Config,
    action: &str,
) -> anyhow::Result<()> {
    use crate::brain::Brain;
    use crate::project::ProjectScanner;
    use std::env;

    let root = env::current_dir().unwrap_or_default();
    let brain = Brain::new(&paths.db_file)?;

    if action == "scan" {
        println!("Scanning project at {}...", root.display());
        let scanner = ProjectScanner::new(root.clone());
        let meta = scanner.scan()?;
        brain.save_project(root.to_string_lossy().as_ref(), &meta)?;
        println!("Scan complete.");
        print_project_summary(&meta);
    } else {
        // Status
        if let Ok(Some(meta)) = brain.get_project(root.to_string_lossy().as_ref()) {
            println!("Project context found for {}", root.display());
            print_project_summary(&meta);
        } else {
            println!("No project context found for {}", root.display());
            println!("Run `goat project scan` to index this directory.");
        }
    }
    Ok(())
}

fn print_project_summary(meta: &crate::project::ProjectMetadata) {
    println!("{}", "─".repeat(60));
    println!("Root: {}", meta.root_path.display());
    println!("Git Repo: {}", if meta.is_git_repo { "Yes" } else { "No" });
    if !meta.stack.is_empty() {
        println!("Stack: {}", meta.stack.join(", "));
    }
    if !meta.package_files.is_empty() {
        println!("Packages: {}", meta.package_files.join(", "));
    }
    if !meta.detected_commands.is_empty() {
        println!("Commands: {}", meta.detected_commands.join(", "));
    }
    println!("Ignored directories: {}", meta.ignored_dirs_count);
    println!("{}", "─".repeat(60));
}

fn handle_memory_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    text: Option<&str>,
) -> anyhow::Result<()> {
    use crate::memory::MemoryManager;
    let manager = MemoryManager::new(paths, config.memory.clone());

    match action {
        "status" => {
            let (u_count, u_max, u_warn) = manager.user_budget_status();
            let (m_count, m_max, m_warn) = manager.memory_budget_status();
            println!("[MEMORY] Status:");
            println!(
                "  USER.md   : {}/{} chars {}",
                u_count,
                u_max,
                if u_warn { "(OVER BUDGET)" } else { "" }
            );
            println!(
                "  MEMORY.md : {}/{} chars {}",
                m_count,
                m_max,
                if m_warn { "(OVER BUDGET)" } else { "" }
            );
            println!("  Enabled   : {}", config.memory.enabled);
        }
        "show" => {
            println!("--- USER.md ---");
            println!("{}", manager.get_user_content().unwrap_or_default());
            println!("--- MEMORY.md ---");
            println!("{}", manager.get_memory_content().unwrap_or_default());
        }
        "path" => {
            println!("USER.md:   {}", manager.user_file.display());
            println!("MEMORY.md: {}", manager.memory_file.display());
        }
        "edit" => {
            println!("To edit memory files, open these in your editor:");
            println!("  {}", manager.user_file.display());
            println!("  {}", manager.memory_file.display());
        }
        "add-user" => {
            if let Some(t) = text {
                manager.add_user(t)?;
                println!("Added to USER.md");
            } else {
                println!("Please provide text to add.");
            }
        }
        "add-note" => {
            if let Some(t) = text {
                manager.add_note(t)?;
                println!("Added to MEMORY.md");
            } else {
                println!("Please provide text to add.");
            }
        }
        _ => {
            println!("Unknown memory action: {}", action);
        }
    }
    Ok(())
}

fn handle_recall_command(paths: &crate::paths::GoatPaths, query: &str) -> anyhow::Result<()> {
    use crate::brain::Brain;
    let brain = Brain::new(&paths.db_file)?;
    let results = brain.recall_search(query)?;

    if results.is_empty() {
        println!("No recall results found for: {}", query);
    } else {
        println!("Found {} result(s) for '{}':", results.len(), query);
        for (idx, (session_id, role, content)) in results.iter().enumerate() {
            let snippet = if content.len() > 80 {
                format!("{}...", &content[..77].replace('\n', " "))
            } else {
                content.replace('\n', " ")
            };
            println!(
                "  {}. [{}] {}: {}",
                idx + 1,
                &session_id[..8],
                role,
                snippet
            );
        }
    }
    Ok(())
}

// ── skills command ────────────────────────────────────────────────────────────

async fn handle_skills_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
    session_id: Option<&str>,
) -> anyhow::Result<()> {
    let skill_manager = crate::skills::SkillManager::new(paths.clone(), config.skills.clone());

    match action {
        "path" => {
            println!("{}", skill_manager.skills_dir().display());
        }
        "list" => {
            let skills = skill_manager.list_skills();
            if skills.is_empty() {
                println!(
                    "No skills found in {}",
                    skill_manager.skills_dir().display()
                );
                return Ok(());
            }
            println!("Skills ({}):", skills.len());
            for s in skills {
                let name = if s.is_suspicious {
                    format!("{} [SUSPICIOUS]", s.name)
                } else {
                    s.name
                };
                println!("  - {name:<20} {}", s.description);
            }
        }
        "show" => {
            let name = arg.ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            if let Some(skill) = skill_manager.get_skill(name) {
                if skill.is_suspicious {
                    println!("WARNING: This skill contains suspicious patterns:");
                    for w in &skill.warnings {
                        println!("  - {}", w);
                    }
                    println!();
                }
                println!("{}", skill.content);
            } else {
                println!("Skill '{}' not found.", name);
            }
        }
        "create" => {
            let name = arg.ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let path = skill_manager.create_template(name)?;
            println!("Created skill template at: {}", path.display());
            println!("Edit this file to implement your skill.");
        }
        "validate" => {
            let skills = skill_manager.list_skills();
            let mut invalid = 0;
            for s in skills {
                if s.is_suspicious {
                    invalid += 1;
                    println!("WARNING: Skill '{}' is suspicious:", s.name);
                    for w in s.warnings {
                        println!("  - {}", w);
                    }
                }
            }
            if invalid == 0 {
                println!("All skills passed validation.");
            } else {
                println!("{} skills failed validation.", invalid);
            }
        }
        "search" => {
            let query = arg.ok_or_else(|| anyhow::anyhow!("Expected search query"))?;
            let results = skill_manager.search_skills(query);
            if results.is_empty() {
                println!("No skills found matching '{}'", query);
                return Ok(());
            }
            println!("Found {} matching skills:", results.len());
            for s in results {
                println!("  - {name:<20} {desc}", name = s.name, desc = s.description);
            }
        }
        "create-from-session" => {
            let name = arg.ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let sid = session_id.ok_or_else(|| {
                anyhow::anyhow!("Expected --session <id> for create-from-session")
            })?;

            let brain = crate::brain::Brain::new(&paths.db_file)
                .map_err(|e| anyhow::anyhow!("Could not open brain db: {}", e))?;

            let history = brain.load_session_history(sid)?;
            if history.is_empty() {
                anyhow::bail!("No history found for session {}", sid);
            }

            let mut history_text = String::new();
            for msg in history {
                if msg.0 != "system" {
                    history_text.push_str(&format!("{}: {}\n", msg.0, msg.1));
                }
            }

            println!("Extracting skill '{}' from session {}...", name, sid);

            let mut registry = crate::models::ProfileRegistry::from_config(&config.profiles);
            let mut router = crate::llm::LlmRouter::from_config(config);

            let profile_name = &registry.default_profile;
            let (_, chain) = registry.resolve(profile_name);

            let prompt = format!(
                "You are a skill curator. The user wants to extract a reusable skill from the following session history.\n\
                 Generate a valid SKILL.md file with the following headers: Name, Description, Triggers, Tools Needed, Procedure, Safety Notes, Verification.\n\
                 The skill name should be: {}\n\n\
                 Rules:\n\
                 - NEVER include real API keys, passwords, or secrets.\n\
                 - Focus on the generalized workflow, not the exact files edited.\n\
                 - Output only the Markdown content.\n\n\
                 Session History:\n{}",
                name, history_text
            );

            let messages = vec![crate::llm::Message {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
            }];

            match router
                .completion_with_fallback(&chain, messages, None)
                .await
            {
                Ok((resp, _)) => {
                    let content = resp.content.unwrap_or_default();
                    let skill_dir = skill_manager.skills_dir().join(name);
                    std::fs::create_dir_all(&skill_dir)?;
                    let skill_file = skill_dir.join("SKILL.md");
                    std::fs::write(&skill_file, content)?;
                    println!(
                        "Extracted and saved skill '{}' to {}",
                        name,
                        skill_file.display()
                    );
                }
                Err(e) => anyhow::bail!("Failed to extract skill from LLM: {}", e),
            }
        }
        _ => {
            println!(
                "Unknown action '{}'. Expected: list, show, path, create, validate, search, create-from-session.",
                action
            );
        }
    }
    Ok(())
}

// ── repo-map command ──────────────────────────────────────────────────────────

fn handle_repo_map_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
) -> anyhow::Result<()> {
    use crate::repo_map::{ProjectCommands, RepoMapScanner};
    use std::env;

    let root = env::current_dir().unwrap_or_default();
    let max_chars = config.repo_map.max_chars;
    let include_symbols = config.repo_map.include_symbols;

    match action {
        "show" | "refresh" => {
            if action == "refresh" {
                println!("Refreshing repo map for {}...", root.display());
            } else {
                println!("Repo map for {}:", root.display());
            }

            let scanner = if include_symbols {
                RepoMapScanner::new(root.clone())
            } else {
                RepoMapScanner::new(root.clone()).with_no_symbols()
            };

            let map = scanner
                .scan()
                .map_err(|e| anyhow::anyhow!("Scan failed: {}", e))?;
            println!("{}", "─".repeat(60));
            println!("{}", map.to_compact_string(max_chars, include_symbols));
            println!("{}", "─".repeat(60));

            let cmds = ProjectCommands::detect(&root);
            println!("Detected commands:");
            println!(
                "  check  : {}",
                cmds.check.as_deref().unwrap_or("not detected")
            );
            println!(
                "  test   : {}",
                cmds.test.as_deref().unwrap_or("not detected")
            );
            println!(
                "  lint   : {}",
                cmds.lint.as_deref().unwrap_or("not detected")
            );
            println!(
                "  format : {}",
                cmds.format.as_deref().unwrap_or("not detected")
            );

            // Save project scan to brain if available
            if paths.db_file.exists() {
                if let Ok(brain) = crate::brain::Brain::new(&paths.db_file) {
                    let meta = crate::project::ProjectScanner::new(root.clone())
                        .scan()
                        .ok();
                    if let Some(meta) = meta {
                        let _ = brain.save_project(root.to_string_lossy().as_ref(), &meta);
                    }
                }
            }
        }
        _ => {
            println!("Unknown action '{}'. Expected: show, refresh.", action);
        }
    }

    Ok(())
}

// ── dev command runner (check/test/lint/format) ───────────────────────────────

fn handle_dev_command(kind: &str) -> anyhow::Result<()> {
    handle_dev_command_with_args(kind, None)
}

fn handle_dev_command_with_args(kind: &str, extra: Option<&str>) -> anyhow::Result<()> {
    use std::io::{self, BufRead, Write};

    let root = std::env::current_dir().unwrap_or_default();
    let cmds = crate::repo_map::ProjectCommands::detect(&root);

    let base_cmd = match kind {
        "check" => cmds.check,
        "test" => cmds.test,
        "lint" => cmds.lint,
        "format" => cmds.format,
        _ => None,
    };

    let cmd = match base_cmd {
        Some(c) => {
            if let Some(extra_args) = extra {
                format!("{} {}", c, extra_args)
            } else {
                c
            }
        }
        None => {
            println!("[DEV] No {} command detected for this project.", kind);
            println!("[DEV] GOAT scanned: {}", root.display());
            println!(
                "[DEV] Supported: Rust (Cargo.toml), Node (package.json), Python (pyproject.toml), Go (go.mod)."
            );
            return Ok(());
        }
    };

    println!("[DEV] Detected {} command: {}", kind, cmd);
    println!("[DEV] \u{26a0} This command will run in your terminal. Confirm to proceed.");
    println!("[DEV] (In TUI/headless mode, the ApprovalGate prompt will appear instead.)");
    print!("[DEV] Execute '{}' now? [y/N]: ", cmd);
    io::stdout().flush().ok();

    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok();
    let answer = line.trim().to_lowercase();

    if answer == "y" || answer == "yes" {
        println!("[DEV] Running: {}", cmd);
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .status();
        match status {
            Ok(s) if s.success() => println!("[DEV] \u{2713} {} completed successfully.", kind),
            Ok(s) => println!("[DEV] \u{2717} {} exited with code: {:?}", kind, s.code()),
            Err(e) => println!("[DEV] Error running command: {}", e),
        }
    } else {
        println!("[DEV] Cancelled.");
    }

    Ok(())
}

// ── patch command ─────────────────────────────────────────────────────────────

fn handle_patch_command(action: &str) {
    match action {
        "show" => {
            println!("[PATCH] No pending patch in current session.");
            println!(
                "[PATCH] Patches are created when GOAT proposes a file write during an agent session."
            );
            println!("[PATCH] Use /patch in TUI or headless mode to inspect pending diffs.");
            println!("[PATCH] Full patch queue is planned for Phase 2.4.");
        }
        "apply" => {
            println!("[PATCH] No pending patch to apply.");
            println!("[PATCH] Start a session and let the agent propose a file write.");
        }
        "discard" => {
            println!("[PATCH] No pending patch to discard.");
        }
        _ => {
            println!(
                "Unknown patch action '{}'. Expected: show, apply, discard.",
                action
            );
        }
    }
}

fn handle_subagents_command(
    paths: &crate::paths::GoatPaths,
    _config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
) -> anyhow::Result<()> {
    let registry = crate::subagents::SubagentRegistry::new();

    match action {
        "list" | "" => {
            let list = registry.list_all();
            println!("GOAT Subagent Registry ({} subagents)", list.len());
            println!("{:-<80}", "");
            println!(
                "{:<15} | {:<15} | {:<15} | {}",
                "Name", "Kind", "Risk Level", "Profile"
            );
            println!("{:-<80}", "");
            for agent in list {
                println!(
                    "{:<15} | {:<15} | {:<15} | {}",
                    agent.name,
                    agent.kind.to_string(),
                    agent.risk_level.to_string(),
                    agent.default_model_profile
                );
            }
        }
        "show" => {
            if let Some(name) = arg {
                if let Some(agent) = registry.get(name) {
                    println!("Subagent: {}", agent.name);
                    println!("Kind: {}", agent.kind);
                    println!("Purpose: {}", agent.purpose);
                    println!("Risk Level: {}", agent.risk_level);
                    println!("Model Profile: {}", agent.default_model_profile);
                    println!("Allowed Tools: {:?}", agent.allowed_tools);
                    println!("Context Budget: {}", agent.context_budget);
                    println!("Requires Approval: {}", agent.requires_approval);
                    println!("Can Propose Patches: {}", agent.can_propose_patches);
                } else {
                    println!("Subagent '{}' not found.", name);
                }
            } else {
                println!("Please specify a subagent name to show.");
            }
        }
        "audit" => {
            if paths.subagent_audit_log_file.exists() {
                if let Ok(content) = std::fs::read_to_string(&paths.subagent_audit_log_file) {
                    println!("{}", content);
                } else {
                    println!("Error reading subagent audit log.");
                }
            } else {
                println!("No subagent audit log found.");
            }
        }
        _ => {
            println!("Unknown action: {}. Available: list, show, audit.", action);
        }
    }
    Ok(())
}

async fn handle_ask_agent_command(
    name: &str,
    task: &str,
    rt: &crate::runtime::GoatRuntime,
) -> anyhow::Result<()> {
    println!("Asking subagent '{}'...", name);
    let manager = &rt.subagent_manager;
    let summary = "CLI context summary... (limited repo map)";
    let result = manager
        .ask_agent(
            name,
            task,
            summary,
            None,
            None,
            &rt.llm_router,
            &rt.model_chain,
        )
        .await?;
    println!("\nResponse:\n{}\n", result);
    Ok(())
}

// ── External Agent Commands ───────────────────────────────────────────────────

fn handle_external_agents_command(
    mut rt: crate::runtime::GoatRuntime,
    action: &str,
    arg: Option<&str>,
) {
    let mut ext_mgr = rt.external_agent_manager;
    ext_mgr.detect_all(&rt.config);

    match action {
        "list" => {
            let agents = ext_mgr.registry.list_all();
            println!("GOAT External Agent Registry ({} adapters)", agents.len());
            for agent in agents {
                println!(
                    "  {:<15} [{}] - {}",
                    agent.name, agent.command_name, agent.status
                );
            }
        }
        "detect" => {
            println!("Detecting external agents...");
            for agent in ext_mgr.registry.list_all() {
                println!("  {:<15} - {}", agent.name, agent.status);
            }
        }
        "show" => {
            let name = arg.unwrap_or("");
            if let Some(agent) = ext_mgr.registry.get(name) {
                println!("Name: {}", agent.name);
                println!("Command: {}", agent.command_name);
                println!("Status: {}", agent.status);
                println!("Risk: {}", agent.risk_level);
                println!("Workspace Behavior: {}", agent.workspace_behavior);
                if let Some(ref path) = agent.detected_path {
                    println!("Detected Path: {}", path.display());
                }
            } else {
                println!("External agent '{}' not found.", name);
            }
        }
        "audit" => {
            if rt.paths.external_agent_audit_log_file.exists() {
                if let Ok(content) =
                    std::fs::read_to_string(&rt.paths.external_agent_audit_log_file)
                {
                    println!("{}", content);
                }
            } else {
                println!("No external agent audit log found.");
            }
        }
        "doctor" => {
            let checks = crate::paths::run_doctor(&rt.paths, &rt.config, false);
            crate::paths::print_doctor_results(&checks);
        }
        "runs" => {
            let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
            if jsonl_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                    println!("External Agent Runs:");
                    for line in content.lines() {
                        if let Ok(run) =
                            serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line)
                        {
                            println!(
                                "  {} | Agent: {:<12} | Mode: {:<15} | Status: {}",
                                run.id,
                                run.agent_name,
                                run.mode,
                                if run.success { "Success" } else { "Failed" }
                            );
                        }
                    }
                }
            } else {
                println!("No runs recorded yet.");
            }
        }
        "run" => {
            if let Some(run_id) = arg {
                let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
                let mut found = false;
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<
                                crate::external_agents::ExternalAgentRun,
                            >(line)
                            {
                                if run.id == run_id {
                                    println!("Run ID: {}", run.id);
                                    println!("Agent: {}", run.agent_name);
                                    println!("Timestamp: {}", run.timestamp);
                                    println!("Mode: {}", run.mode);
                                    println!("Workspace: {}", run.workspace_path.display());
                                    println!("Task: {}", run.task);
                                    println!("Success: {}", run.success);
                                    println!("Duration: {:?}", run.duration);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found {
                    println!("Run ID '{}' not found.", run_id);
                }
            } else {
                println!("Usage: goat external-agents run <id>");
            }
        }
        _ => {
            println!("Unknown external-agents action: {}", action);
            println!("Valid actions: list, detect, show <name>, audit, doctor, runs, run <id>");
        }
    }
}

async fn handle_delegate_external_command(
    mut rt: crate::runtime::GoatRuntime,
    name: &str,
    task: &str,
) {
    println!("Delegating task to external agent '{}'...", name);
    rt.external_agent_manager.detect_all(&rt.config);

    let action = rt
        .tool_registry
        .evaluate_action("delegate_external_agent", &rt.config.tools);
    if let crate::tool_registry::ToolAction::Deny(reason) = action {
        println!("Delegation denied by tool registry: {}", reason);
        return;
    }

    let req = crate::approval::ApprovalRequest {
        tool_name: "delegate_external_agent".to_string(),
        action_summary: format!("agent: {}, task: {}", name, task),
        risk_level: crate::approval::RiskLevel::High,
        explanation: None,
        working_directory: None,
    };

    if let Some(crate::approval::ApprovalDecision::Denied(msg)) =
        rt.approval_gate.check_policy(&req)
    {
        match rt.external_agent_manager.delegate(name, task, &rt.config) {
            Ok(res) => {
                println!("Execution finished. Success: {}", res.success);
                println!("Stdout:\n{}", res.stdout);
                if !res.stderr.is_empty() {
                    println!("Stderr:\n{}", res.stderr);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    match rt.external_agent_manager.delegate(name, task, &rt.config) {
        Ok(res) => {
            println!("Execution finished. Success: {}", res.success);
            println!("Stdout:\n{}", res.stdout);
            if !res.stderr.is_empty() {
                println!("Stderr:\n{}", res.stderr);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_hooks_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
) -> anyhow::Result<()> {
    // Basic wrapper to print output from hooks manager for CLI usage.
    let mut hm = crate::hooks::HooksManager::new(config.hooks.clone(), paths.clone());

    match action {
        "list" => {
            let info = hm.list_hooks_info();
            println!("[HOOKS] Registered Hooks:");
            for i in info {
                println!("  - {}", i);
            }
        }
        "show" => {
            if let Some(name) = arg {
                println!("[HOOKS] Show hook not fully implemented in CLI.");
            } else {
                println!("Usage: goat hooks show <name>");
            }
        }
        _ => {
            println!("Unknown hooks action: {}", action);
        }
    }
    Ok(())
}

fn handle_schedule_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    let mut sm = crate::scheduler::SchedulerManager::new(config.scheduler.clone(), paths.clone());

    match action {
        "list" => {
            let jobs = sm.list_jobs();
            println!("[SCHEDULE] Scheduled Jobs:");
            for j in jobs {
                println!(
                    "  - [{}] {} (enabled: {})",
                    j.id, j.prompt_or_command, j.enabled
                );
            }
        }
        "add" => {
            println!("[SCHEDULE] Adding jobs via CLI is not fully implemented yet.");
        }
        _ => {
            println!("Unknown schedule action: {}", action);
        }
    }
    Ok(())
}

fn handle_jobs_command(
    _paths: &crate::paths::GoatPaths,
    _config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
) -> anyhow::Result<()> {
    match action {
        "list" => {
            println!("[JOBS] Listing jobs via CLI requires an active runtime session.");
            println!("Please use `goat` TUI or `/jobs` slash command.");
        }
        _ => {
            println!("Unknown jobs action: {}", action);
        }
    }
    Ok(())
}

async fn handle_daemon_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
) -> anyhow::Result<()> {
    match action {
        "start" => {
            // Setup runtime and start daemon
            let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                config.clone(),
                paths.clone(),
                vec![],
                false,
                None,
            );
            crate::daemon::run(rt).await?;
        }
        "status" => {
            crate::daemon::get_status(paths);
        }
        "doctor" => {
            crate::daemon::print_doctor(paths, config);
        }
        "stop" => {
            println!(
                "[DAEMON] Stop command is partial. Use Ctrl+C on the start terminal or kill the PID directly for now."
            );
        }
        _ => {
            println!(
                "[DAEMON] Unknown action '{}'. Use start, status, stop, or doctor.",
                action
            );
        }
    }
    Ok(())
}
