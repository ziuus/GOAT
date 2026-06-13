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
    about = "GOAT — Local-first Agent OS",
    long_about = "GOAT is a Rust-first, terminal-native AI agent platform.\n\n\
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

    /// Seed demo data for the dashboard (Phase 6.5).
    /// Generates local-first JSONL mock data to visualize all Prime Agent UI flows.
    #[command(name = "seed-demo")]
    SeedDemo {
        /// Clear existing demo data before seeding.
        #[arg(long)]
        clear: bool,
    },

    /// List and switch model profiles and providers.
    #[command(name = "models")]
    Models {
        /// Optional specific action (e.g., 'list', 'route')
        #[arg(default_value = "list")]
        action: String,
        /// Additional arguments depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Manage Universal Model Providers (Phase 6.6)
    #[command(name = "providers")]
    Providers {
        /// Action to perform: list, doctor
        #[arg(default_value = "list")]
        action: String,
    },

    /// Manage Brain Memory and Context Packs (Phase 6.7)
    #[command(name = "brain")]
    Brain {
        /// Action to perform: dedupe, ingest, pack
        action: String,
        /// Additional arguments depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Manage Safe Extensions and Plugin Marketplace (Phase 6.8)
    #[command(name = "extensions")]
    Extensions {
        /// Action to perform: list, discover, audit, install, enable, disable, remove
        #[arg(default_value = "list")]
        action: String,
        /// ID or Path to extension depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Safe approval-gated browser automation workflows (Phase 6.9)
    #[command(name = "browser")]
    Browser {
        /// Subcommand: workflows, screenshot, inspect, qa, landing-review, dashboard-qa, health
        action: String,
        /// URL or workflow-id depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Builder agent workspace operations (Phase 7.1)
    #[command(name = "builder")]
    Builder {
        /// Subcommand: inspect, plan, diff-review, test-plan, validate, rollback-plan
        action: String,
        /// Goal or argument depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Researcher agent operations (Phase 7.5)
    #[command(name = "researcher")]
    Researcher {
        /// Subcommand: projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report
        action: String,
        /// Goal or argument depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
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

    /// Manage external AI agents.
    #[command(name = "agent", alias = "agents")]
    Agent {
        /// "list", "doctor", "run", "runs", "run-show", "compare"
        action: String,
        /// The agent name or run ID
        arg: Option<String>,
        /// Prompt for the agent
        #[arg(long)]
        prompt: Option<String>,
        /// Mission ID
        #[arg(long)]
        mission: Option<String>,
    },

    /// Mission Control workspace operations.
    #[command(name = "mission")]
    Mission {
        /// Action to perform: plan, status.
        #[arg(default_value = "status")]
        action: String,
        /// Additional arguments depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Learn the current or specified project folder.
    #[command(name = "learn")]
    Learn {
        /// Target path (defaults to current directory).
        #[arg(default_value = ".")]
        path: String,
    },

    /// Context-Aware Diff Analyzer (Phase 8.9)
    #[command(name = "diff")]
    Diff {
        /// Subcommand: analyze, list, show
        action: String,
        /// Patch ID, Agent Run ID, or "git"
        arg: Option<String>,
        /// Target project for git diff
        #[arg(long)]
        project: Option<String>,
        /// Type of analysis (patch, git, agent-run)
        #[arg(long)]
        source: Option<String>,
    },

    /// Manage proposed code changes (patches).
    #[command(name = "patch")]
    Patch {
        /// Action: propose, list, show, apply
        #[arg(default_value = "list")]
        action: String,
        /// Additional arguments: mission_id or patch_id
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Manage project checkpoints.
    #[command(name = "checkpoint")]
    CheckpointCmd {
        /// Action: list, restore
        #[arg(default_value = "list")]
        action: String,
        /// Additional arguments: checkpoint_id
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Run validation commands for a project.
    #[command(name = "validate")]
    Validate {
        /// Project ID to validate
        project_id: Option<String>,
        /// Associated mission ID
        #[arg(long)]
        mission: Option<String>,
        /// Associated patch ID
        #[arg(long)]
        patch: Option<String>,
        /// Automatically approve the validation command (opt-in)
        #[arg(long, default_value_t = false)]
        auto_approve: bool,
    },

    /// Manage validation results.
    #[command(name = "validation")]
    Validation {
        /// Action: list, show
        #[arg(default_value = "list")]
        action: String,
        /// Validation ID to show
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Project workspace operations.
    #[command(name = "projects")]
    Projects {
        /// Action to perform: list, new, show.
        #[arg(default_value = "list")]
        action: String,
        /// Additional arguments depending on action
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
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

    /// Manage GOAT Web Dashboard
    #[command(name = "dashboard")]
    Dashboard {
        /// Action to perform: dev, path, doctor
        #[arg(default_value = "dev")]
        action: String,
    },

    /// Manage GOAT Desktop App
    #[command(name = "desktop")]
    Desktop {
        /// Action to perform: run, dev, path, doctor
        #[arg(default_value = "run")]
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
        /// "list", "search", "show", "archive", "extract", "add", "user" or "project"
        action: String,
        /// The query, ID, or text
        #[arg(default_value = "")]
        arg: String,
        /// Scope for add (user, project, mission, skill, system)
        #[arg(long)]
        scope: Option<String>,
        /// Kind for add (preference, architecture_note, etc.)
        #[arg(long)]
        kind: Option<String>,
        /// Text for add
        #[arg(long)]
        text: Option<String>,
        /// Mission ID for extract
        #[arg(long)]
        mission: Option<String>,
        /// Project ID
        #[arg(long)]
        project: Option<String>,
    },

    /// Search past conversation interactions.
    #[command(name = "recall")]
    Recall { query: String },

    /// Manage GOAT reusable skills.
    #[command(name = "skills", alias = "skill")]
    Skills {
        /// "list", "show", "path", "new", "create", "validate", "search", "create-from-mission", "run", "runs", "run-show", "recommend", "curate"
        #[arg(default_value = "list")]
        action: String,
        /// The name, query, or mission ID
        arg: Option<String>,
        /// Skill name for create commands
        #[arg(long)]
        name: Option<String>,
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

        Command::SeedDemo { clear } => {
            handle_seed_demo_command(paths, *clear).await?;
            Ok(true)
        }

        Command::Models { action, args } => {
            handle_models_command(config, action, args)?;
            Ok(true)
        }
        Command::Providers { action } => {
            handle_providers_command(config, action)?;
            Ok(true)
        }
        Command::Brain { action, args } => {
            let manager = crate::brain_index::BrainIndexManager::new(
                paths.clone(),
                config.brain_index.clone(),
                &config.embeddings,
            );
            match action.as_str() {
                "dedupe" => {
                    println!("[BRAIN] Starting deduplication...");
                    let count = manager.dedupe()?;
                    println!(
                        "[BRAIN] Deduplication complete. Removed {} duplicates.",
                        count
                    );
                }
                "pack" => {
                    let query = args.join(" ");
                    if query.is_empty() {
                        println!("[BRAIN] Please provide a query for the context pack.");
                        return Ok(true);
                    }
                    let builder =
                        crate::brain_context::BrainContextPackBuilder::new(&manager, query)
                            .limit_items(5);
                    let pack = builder.build().await?;
                    println!("[BRAIN] Context Pack Generated:");
                    println!("Title: {}", pack.title);
                    println!("Summary: {}", pack.summary);
                    println!("Size: {} characters", pack.estimated_size);
                    println!("Items: {}", pack.items.len());
                    for (i, doc) in pack.items.iter().enumerate() {
                        println!("  {}) [{:?}] {}", i + 1, doc.kind, doc.title);
                    }
                }
                _ => {
                    println!("[BRAIN] Unknown action: {}", action);
                }
            }
            Ok(true)
        }
        Command::Project { action } => {
            handle_project_command(paths, config, action)?;
            Ok(true)
        }
        Command::Memory {
            action,
            arg,
            scope,
            kind,
            text,
            mission,
            project,
        } => {
            handle_memory_command(
                paths, config, action.clone(), arg.clone(), scope.clone(), kind.clone(), text.clone(), mission.clone(), project.clone(),
            )?;
            Ok(true)
        }
        Command::Recall { query } => {
            handle_recall_command(paths, query)?;
            Ok(true)
        }

        Command::Skills {
            action,
            arg,
            name,
            session,
        } => {
            handle_skills_command(
                paths,
                config,
                action,
                arg.as_deref(),
                name.as_deref(),
                session.as_deref(),
            )
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

        Command::Daemon { action } => {
            handle_daemon_command(paths, config, action).await?;
            Ok(true)
        }

        Command::Dashboard { action } => {
            handle_dashboard_command(action);
            Ok(true)
        }

        Command::Desktop { action } => {
            handle_desktop_command(action);
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
        Command::Extensions { action, args } => {
            handle_extensions_command(paths, config, &action, &args)?;
            Ok(true)
        }
        Command::Browser { action, args } => {
            handle_browser_command(paths, config, &action, &args)?;
            Ok(true)
        }
        Command::Builder { action, args } => {
            handle_builder_command(paths, config, &action, &args)?;
            Ok(true)
        }
        Command::Researcher { action, args } => {
            handle_researcher_command(paths, config, &action, &args)?;
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
        Command::Agent {
            action,
            arg,
            prompt,
            mission,
        } => {
            let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                config.clone(),
                paths.clone(),
                vec![],
                false,
                None,
            );
            handle_agent_command(
                rt,
                &action,
                arg.as_deref(),
                prompt.as_deref(),
                mission.as_deref(),
            );
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
        Command::Mission { action, args } => {
            let mc = crate::mission_control::MissionControlManager::new();
            if action == "plan" && !args.is_empty() {
                let goal = args.join(" ");
                let req = crate::mission_control::MissionPlanReq {
                    goal,
                    project_id: None,
                    constraints: None,
                };
                let plan = mc.plan_goal(&req);
                println!(
                    "Created Mission: {} (Type: {:?})",
                    plan.title, plan.mission_type
                );
                for step in plan.plan_steps {
                    println!(
                        "  - [{}] {} (Agent: {:?})",
                        step.status, step.title, step.assigned_agent
                    );
                }
            } else {
                let missions = mc.get_missions();
                if let Some(m) = missions.first() {
                    println!("Active Mission: {} ({:?})", m.title, m.status);
                    println!("  Goal: {}", m.raw_goal);
                    println!("  Progress: {}%", m.progress);
                } else {
                    println!(
                        "No active missions found. Run `goat mission plan \"<goal>\"` to plan one."
                    );
                }
            }
            println!(
                "\nView the full Mission Control workspace at http://127.0.0.1:3000/mission-control"
            );
            Ok(true)
        }
        Command::Learn { path } => {
            let target_path = path;
            let target_path_buf = std::path::PathBuf::from(&target_path);
            let canonical = target_path_buf
                .canonicalize()
                .unwrap_or_else(|_| target_path_buf.clone());

            println!("You are about to scan: {}", canonical.display());
            println!("This will analyze files for tech stack, commands, and project context.");
            println!(
                "Sensitive files (secrets, .env) and large directories (.git, node_modules) will be ignored."
            );

            let mut prompt = String::new();
            println!("Do you want to proceed? [y/N]: ");
            std::io::stdin().read_line(&mut prompt).ok();
            if prompt.trim().to_lowercase() != "y" {
                println!("Scan aborted.");
                return Ok(true);
            }

            let scanner = crate::project_intelligence::DeepProjectScanner::new(canonical);
            match scanner.scan() {
                Ok(pi) => {
                    let manager = crate::project_intelligence::ProjectIntelligenceManager::new();
                    manager.save_project(&pi)?;
                    println!("\nProject learned successfully!");
                    println!("Name: {}", pi.name);
                    println!("ID: {}", pi.project_id);
                    println!("Stack: {}", pi.detected_stack.join(", "));
                    println!("Summary: {}", pi.architecture_summary);
                    if !pi.risk_notes.is_empty() {
                        println!("Notes: {} sensitive files ignored.", pi.risk_notes.len());
                    }
                }
                Err(e) => println!("Failed to scan project: {}", e),
            }
            Ok(true)
        }
        Command::Diff {
            action,
            arg,
            project,
            source,
        } => {
            let analyzer = crate::diff_analyzer::DiffAnalyzer::new();
            if action == "list" {
                match analyzer.list_analyses() {
                    Ok(analyses) => {
                        if analyses.is_empty() {
                            println!("No diff analyses found.");
                        } else {
                            for a in analyses {
                                println!(
                                    "- {} | {} | Risk: {:?} | Source: {:?}",
                                    a.analysis_id, a.title, a.risk_level, a.source_type
                                );
                            }
                        }
                    }
                    Err(e) => println!("Error listing diff analyses: {}", e),
                }
            } else if action == "show" {
                if let Some(id) = arg {
                    match analyzer.get_analysis(&id) {
                        Ok(a) => {
                            println!("Analysis ID: {}", a.analysis_id);
                            println!("Title: {}", a.title);
                            println!("Source: {:?}", a.source_type);
                            println!("Risk Level: {:?}", a.risk_level);
                            println!("Recommendation: {:?}", a.recommendation);
                            println!("Summary: {}", a.summary);
                            if !a.findings.is_empty() {
                                println!("\nFindings:");
                                for f in a.findings {
                                    println!(
                                        "  [{:?}] {}: {}",
                                        f.severity,
                                        f.file_path.unwrap_or_default(),
                                        f.message
                                    );
                                }
                            }
                        }
                        Err(e) => println!("Error fetching analysis: {}", e),
                    }
                } else {
                    println!("Usage: goat diff show <analysis_id>");
                }
            } else if action == "analyze" {
                let src = source.clone().unwrap_or_else(|| "patch".to_string());
                if src == "patch" {
                    if let Some(id) = arg {
                        let patch_manager = crate::patch_manager::PatchManager::new();
                        if let Some(patch) = patch_manager.get_patch(&id) {
                            match analyzer.analyze_patch(&patch) {
                                Ok(a) => println!(
                                    "Patch analyzed successfully. ID: {} | Risk: {:?}",
                                    a.analysis_id, a.risk_level
                                ),
                                Err(e) => println!("Error analyzing patch: {}", e),
                            }
                        } else {
                            println!("Patch not found.");
                        }
                    } else {
                        println!("Usage: goat diff analyze <patch_id> --source patch");
                    }
                } else if src == "git" {
                    let project_path = project.clone().unwrap_or_else(|| ".".to_string());
                    let path = std::path::PathBuf::from(project_path);
                    if let Ok(output) = std::process::Command::new("git")
                        .arg("diff")
                        .current_dir(&path)
                        .output()
                    {
                        let diff_text = String::from_utf8_lossy(&output.stdout);
                        match analyzer.analyze_git_diff(&path, &diff_text) {
                            Ok(a) => println!(
                                "Git diff analyzed successfully. ID: {} | Risk: {:?}",
                                a.analysis_id, a.risk_level
                            ),
                            Err(e) => println!("Error analyzing git diff: {}", e),
                        }
                    } else {
                        println!("Failed to run git diff");
                    }
                } else if src == "agent-run" {
                    if let Some(id) = arg {
                        let paths = crate::paths::GoatPaths::resolve().unwrap();
                        let em = crate::external_agents::ExternalAgentManager::new(
                            paths.external_agent_audit_log_file.clone(),
                            paths.data_dir.clone(),
                        );
                        if let Some(run) = em.get_run(&id) {
                            match analyzer.analyze_agent_run(&run) {
                                Ok(a) => println!(
                                    "Agent run analyzed successfully. ID: {} | Risk: {:?}",
                                    a.analysis_id, a.risk_level
                                ),
                                Err(e) => println!("Error analyzing agent run: {}", e),
                            }
                        } else {
                            println!("Agent run not found.");
                        }
                    } else {
                        println!("Usage: goat diff analyze <run_id> --source agent-run");
                    }
                } else {
                    println!("Unknown source. Use 'patch', 'git', or 'agent-run'.");
                }
            } else {
                println!("Unknown action. Use analyze, list, or show.");
            }
            Ok(true)
        }
        Command::Patch { action, args } => {
            let patch_manager = crate::patch_manager::PatchManager::new();
            if action == "list" {
                let patches = patch_manager.get_patches();
                if patches.is_empty() {
                    println!("No patches found.");
                } else {
                    for p in patches {
                        println!(
                            "- {} [{}] ({}) : {}",
                            p.patch_id, p.status, p.project_id, p.title
                        );
                    }
                }
            } else if action == "show" {
                if let Some(id) = args.first() {
                    if let Some(p) = patch_manager.get_patch(id) {
                        println!("Patch: {} ({})", p.patch_id, p.status);
                        println!("Title: {}", p.title);
                        println!("Project ID: {}", p.project_id);
                        println!("Mission ID: {}", p.mission_id);
                        println!("Diff Preview:\n{}", p.diff_preview);
                    } else {
                        println!("Patch not found.");
                    }
                } else {
                    println!("Usage: goat patch show <patch_id>");
                }
            } else if action == "propose" {
                if let Some(mission_id) = args.first() {
                    let mc = crate::mission_control::MissionControlManager::new();
                    if let Some(mission) = mc
                        .get_missions()
                        .into_iter()
                        .find(|m| m.mission_id == *mission_id)
                    {
                        if let Some(linked_project_id) = &mission.linked_project {
                            let pi_mgr =
                                crate::project_intelligence::ProjectIntelligenceManager::new();
                            if let Some(project) = pi_mgr.get_project(linked_project_id) {
                                match patch_manager.generate_patch_proposal(&mission, &project) {
                                    Ok(patch) => {
                                        patch_manager.save_patch(&patch).unwrap();
                                        println!(
                                            "Patch proposed successfully! ID: {}",
                                            patch.patch_id
                                        );
                                        println!("Title: {}", patch.title);
                                        println!(
                                            "Review it with `goat patch show {}`",
                                            patch.patch_id
                                        );
                                        println!(
                                            "Apply it with `goat patch apply {}`",
                                            patch.patch_id
                                        );
                                    }
                                    Err(e) => println!("Failed to propose patch: {}", e),
                                }
                            } else {
                                println!(
                                    "Project intelligence not found for ID: {}",
                                    linked_project_id
                                );
                            }
                        } else {
                            println!("Mission is not linked to a project.");
                        }
                    } else {
                        println!("Mission not found.");
                    }
                } else {
                    println!("Usage: goat patch propose <mission_id>");
                }
            } else if action == "apply" {
                if let Some(id) = args.first() {
                    if let Some(mut patch) = patch_manager.get_patch(id) {
                        if patch.status != "proposed" {
                            println!("Patch status is '{}', cannot apply.", patch.status);
                            return Ok(true);
                        }
                        let pi_mgr = crate::project_intelligence::ProjectIntelligenceManager::new();
                        if let Some(project) = pi_mgr.get_project(&patch.project_id) {
                            println!(
                                "You are about to apply patch '{}' to project '{}'.",
                                patch.patch_id, project.name
                            );
                            println!("Diff Preview:\n{}", patch.diff_preview);

                            use std::io::Write;
                            print!("Do you approve this patch? [y/N]: ");
                            std::io::stdout().flush().unwrap();
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();

                            if input.trim().eq_ignore_ascii_case("y") {
                                // Create Checkpoint
                                let cp_mgr = crate::checkpoint::CheckpointManager::new(
                                    &crate::paths::GoatPaths::resolve().unwrap().data_dir,
                                );
                                match cp_mgr.create_checkpoint(
                                    &project.root_path,
                                    &format!("Pre-patch {}", patch.patch_id),
                                ) {
                                    Ok(cp) => {
                                        println!("Checkpoint created: {}", cp.id);
                                        patch.checkpoint_id = Some(cp.id.clone());
                                    }
                                    Err(e) => {
                                        println!("Failed to create checkpoint: {}", e);
                                        println!("Aborting patch application.");
                                        return Ok(true);
                                    }
                                }

                                match patch_manager.apply_patch(&mut patch, &project.root_path) {
                                    Ok(_) => {
                                        println!("Patch applied successfully.");

                                        // Command Validation Loop
                                        let mut commands_to_suggest = Vec::new();
                                        commands_to_suggest.extend(project.test_commands.clone());
                                        commands_to_suggest.extend(project.lint_commands.clone());
                                        commands_to_suggest.extend(project.build_commands.clone());
                                        if !commands_to_suggest.is_empty() {
                                            println!("\nDetected validation commands:");
                                            for cmd in &commands_to_suggest {
                                                println!("- {}", cmd);
                                            }
                                            print!("Run these commands now? [y/N]: ");
                                            std::io::stdout().flush().unwrap();
                                            let mut run_input = String::new();
                                            std::io::stdin().read_line(&mut run_input).unwrap();
                                            if run_input.trim().eq_ignore_ascii_case("y") {
                                                for cmd in &commands_to_suggest {
                                                    println!("Running: {}", cmd);
                                                    let mut parts = cmd.split_whitespace();
                                                    if let Some(prog) = parts.next() {
                                                        let args: Vec<&str> = parts.collect();
                                                        let mut child =
                                                            std::process::Command::new(prog)
                                                                .args(args)
                                                                .current_dir(&project.root_path)
                                                                .spawn();
                                                        if let Ok(mut c) = child {
                                                            let _ = c.wait();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => println!("Failed to apply patch: {}", e),
                                }
                            } else {
                                println!("Patch application cancelled.");
                            }
                        } else {
                            println!("Project not found.");
                        }
                    } else {
                        println!("Patch not found.");
                    }
                } else {
                    println!("Usage: goat patch apply <patch_id>");
                }
            } else {
                println!("Unknown patch action.");
            }
            Ok(true)
        }
        Command::CheckpointCmd { action, args } => {
            let cp_mgr = crate::checkpoint::CheckpointManager::new(
                &crate::paths::GoatPaths::resolve().unwrap().data_dir,
            );
            if action == "list" {
                if let Ok(checkpoints) = cp_mgr.list_checkpoints() {
                    if checkpoints.is_empty() {
                        println!("No checkpoints found.");
                    } else {
                        for cp in checkpoints {
                            println!(
                                "- {} [{}] {} (Files changed: {})",
                                cp.id,
                                cp.timestamp,
                                cp.label,
                                cp.changed_files.len()
                            );
                        }
                    }
                } else {
                    println!("Failed to list checkpoints.");
                }
            } else if action == "restore" {
                println!("Restore functionality will be implemented in the next phase.");
            } else {
                println!("Unknown checkpoint action.");
            }
            Ok(true)
        }
        Command::Validate {
            project_id,
            mission,
            patch,
            auto_approve,
        } => {
            let val_mgr = crate::validation::ValidationManager::new();
            let pi_mgr = crate::project_intelligence::ProjectIntelligenceManager::new();

            let pid = if let Some(id) = project_id {
                id.clone()
            } else {
                println!("No project ID provided. Usage: goat validate <project_id>");
                return Ok(true);
            };

            if let Some(project) = pi_mgr.get_project(&pid) {
                let mut cmds = val_mgr.generate_commands(&project);
                if cmds.is_empty() {
                    println!("No validation commands detected for project: {}", pid);
                } else {
                    println!("Found {} validation commands. Executing...", cmds.len());
                    // Since handle_subcommand isn't passed approval_queue, we'll instantiate it
                    // Or if we can't easily get it here, wait, do we have access to it?
                    // We can just use an ephemeral approval queue for CLI test execution,
                    // or skip if we have no approval queue. Let's see what is available.
                    let approval_queue = std::sync::Arc::new(crate::approval::ApprovalQueue::new());

                    // Accept approvals automatically in CLI headless context for now, or just ask user.
                    // Wait, ApprovalQueue allows CLI approval via terminal prompting.

                    // Actually, we should spawn a task to auto-approve in headless if we want,
                    // or better yet, loop through and run them.
                    for mut val in cmds {
                        val.mission_id = mission.clone();
                        val.patch_id = patch.clone();

                        println!("Validating: {}", val.command);
                        let q = approval_queue.clone();
                        let auto_approve_flag = *auto_approve;
                        tokio::spawn(async move {
                            loop {
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                let pending = q.list().await;
                                for p in pending {
                                    if auto_approve_flag {
                                        q.resolve(&p.id, 'y').await;
                                    } else {
                                        for line in p.request.display_lines() {
                                            println!("{}", line);
                                        }
                                        if let Some(wd) = &p.request.working_directory {
                                            println!("  Dir    : {}", wd);
                                        }
                                        println!(
                                            "╚══════════════════════════════════════════════════════╝"
                                        );
                                        print!("Allow execution? (y/n/a): ");
                                        let dec = tokio::task::spawn_blocking(|| {
                                            use std::io::Write;
                                            let _ = std::io::stdout().flush();
                                            let mut input = String::new();
                                            if std::io::stdin().read_line(&mut input).is_ok() {
                                                let input = input.trim().to_lowercase();
                                                if input == "y" || input == "yes" {
                                                    'y'
                                                } else {
                                                    'n'
                                                }
                                            } else {
                                                'n'
                                            }
                                        })
                                        .await
                                        .unwrap_or('n');
                                        q.resolve(&p.id, dec).await;
                                    }
                                }
                            }
                        });

                        match val_mgr.run_validation(val, &approval_queue).await {
                            Ok(res) => {
                                println!("Status: {:?}", res.status);
                                if let Some(sum) = res.summary {
                                    println!("Summary: {}", sum);
                                }
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }
            } else {
                println!("Project not found: {}", pid);
            }
            Ok(true)
        }
        Command::Validation { action, args } => {
            let val_mgr = crate::validation::ValidationManager::new();
            if action == "list" {
                if let Ok(vals) = val_mgr.list_validations() {
                    if vals.is_empty() {
                        println!("No validation runs found.");
                    } else {
                        for v in vals {
                            println!(
                                "- {} | {} | {:?} | Project: {:?}",
                                v.validation_id, v.command, v.status, v.project_id
                            );
                        }
                    }
                } else {
                    println!("Failed to list validations.");
                }
            } else if action == "show" {
                if let Some(id) = args.first() {
                    if let Ok(Some(val)) = val_mgr.get_validation(id) {
                        println!("Validation ID: {}", val.validation_id);
                        println!("Command: {}", val.command);
                        println!("Status: {:?}", val.status);
                        println!("Log: {:?}", val.full_log_path);
                    } else {
                        println!("Validation not found.");
                    }
                } else {
                    println!("Usage: goat validation show <id>");
                }
            } else {
                println!("Unknown validation action.");
            }
            Ok(true)
        }
        Command::Projects { action, args } => {
            let manager = crate::project_intelligence::ProjectIntelligenceManager::new();
            if action == "list" {
                let projects = manager.get_projects();
                if projects.is_empty() {
                    println!("No projects learned yet. Run `goat learn <path>` to add one.");
                } else {
                    println!("Learned Projects:");
                    for p in projects {
                        println!(
                            "- {} ({}) | {}",
                            p.name, p.project_id, p.architecture_summary
                        );
                    }
                }
            } else if action == "show" {
                if let Some(id) = args.first() {
                    if let Some(p) = manager.get_project(id) {
                        println!("Project: {} ({})", p.name, p.project_id);
                        println!("Path: {}", p.root_path.display());
                        println!("Stack: {}", p.detected_stack.join(", "));
                        println!("Commands:");
                        for cmd in p.available_commands {
                            println!("  - {}", cmd);
                        }
                    } else {
                        println!("Project not found.");
                    }
                } else {
                    println!("Usage: goat projects show <id>");
                }
            } else if action == "scan" {
                println!("Use `goat learn <path>` instead.");
            } else {
                println!("Unknown action: {}", action);
            }
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

async fn handle_seed_demo_command(
    paths: &crate::paths::GoatPaths,
    clear: bool,
) -> anyhow::Result<()> {
    use std::fs;
    println!("Seeding demo data for dashboard flows...");

    let prime_dir = paths.data_dir.join("agents").join("prime");
    let cofounder_file = prime_dir.join("cofounder").join("ideas.jsonl");
    let learner_goals = prime_dir.join("learner").join("goals.jsonl");
    let learner_roadmaps = prime_dir.join("learner").join("roadmaps.jsonl");
    let promptforge_hist = paths.data_dir.join("promptforge").join("history.jsonl");
    let reports_dir = paths.data_dir.join("reports");

    if clear {
        println!("Clearing existing demo data...");
        let clear_jsonl = |path: &std::path::PathBuf| {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    let filtered: Vec<&str> =
                        content.lines().filter(|l| !l.contains("[DEMO]")).collect();
                    let _ = fs::write(path, filtered.join("\n"));
                }
            }
        };
        clear_jsonl(&cofounder_file);
        clear_jsonl(&learner_goals);
        clear_jsonl(&learner_roadmaps);
        clear_jsonl(&promptforge_hist);
    } else {
        // Seed Cofounder
        println!("Seeding Cofounder ideas...");
        if let Ok(mut cofounder) = crate::agents::cofounder::CofounderManager::new() {
            let _ = cofounder.add_idea(
                "[DEMO] AI Developer CLI".to_string(),
                "A terminal-native AI agent platform written in Rust".to_string(),
                "Developers".to_string(),
            );
            let _ = cofounder.add_idea(
                "[DEMO] HyperFrames Video Studio".to_string(),
                "Create programmatic videos using React and HTML".to_string(),
                "Creators".to_string(),
            );
        }

        // Seed Learner
        println!("Seeding Learner goals...");
        if let Ok(learner) = crate::agents::learner::LearnerAgent::new() {
            if let Ok(goal) = learner.create_goal(
                "[DEMO] Master Rust Concurrency",
                crate::agents::learner::LearningDomain::Rust,
            ) {
                let _ = learner.create_roadmap(&goal.id);
            }
        }

        // Seed Reports
        println!("Seeding Reports...");
        let report_mgr = crate::reports::ReportManager::new();
        let _ = report_mgr.generate_report(crate::reports::ReportTemplate {
            kind: crate::reports::ReportKind::Research,
            title: "[DEMO] Rust Async Ecosystem".into(),
            sections: vec![crate::reports::ReportSection {
                heading: "Overview".into(),
                body: "Tokio remains the dominant runtime for async Rust.".into(),
            }],
        });
        let _ = report_mgr.generate_report(crate::reports::ReportTemplate {
            kind: crate::reports::ReportKind::CodeReview,
            title: "[DEMO] Phase 6.5 Audit".into(),
            sections: vec![crate::reports::ReportSection {
                heading: "Security".into(),
                body: "Passed all automated checks.".into(),
            }],
        });
    }

    println!("Demo seed/clear complete! Run `goat dashboard` to see the changes.");
    Ok(())
}

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

fn handle_extensions_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    use crate::extensions::ExtensionRegistry;
    let mut registry = ExtensionRegistry::new(
        paths
            .config_file
            .parent()
            .unwrap_or(std::path::Path::new("/")),
        &paths.data_dir,
    )?;
    registry.load_state()?;

    match action {
        "list" => {
            println!("Extension Registry (Phase 6.8)");
            println!("{:-<80}", "");
            println!(
                "{:<30} | {:<15} | {:<15} | {:<10}",
                "ID", "Kind", "Status", "Trust"
            );
            println!("{:-<80}", "");

            let mut records = registry.list_extensions();
            records.sort_by_key(|r| r.manifest.id.clone());

            for r in records {
                println!(
                    "{:<30} | {:<15?} | {:<15?} | {:<10?}",
                    r.manifest.id, r.manifest.kind, r.status, r.trust_level
                );
            }
        }
        "discover" => {
            if args.is_empty() {
                println!("Usage: goat extensions discover <path>");
                return Ok(());
            }
            let path = std::path::Path::new(&args[0]);
            match registry.discover_local(path) {
                Ok(id) => println!("Discovered extension: {}", id),
                Err(e) => println!("Error discovering extension: {}", e),
            }
        }
        "audit" => {
            if args.is_empty() {
                println!("Usage: goat extensions audit <id>");
                return Ok(());
            }
            match registry.audit_extension(&args[0]) {
                Ok(result) => {
                    println!("Audit Results for {}: ", result.extension_id);
                    println!("Passed: {}", result.passed);
                    if result.findings.is_empty() {
                        println!("No findings.");
                    } else {
                        for finding in result.findings {
                            println!("- [{:?}] {}", finding.severity, finding.message);
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        "install" => {
            if args.is_empty() {
                println!("Usage: goat extensions install <id>");
                return Ok(());
            }
            let id = &args[0];

            // For CLI we assume user interaction is outside or explicitly trusted
            if let Some(record) = registry.get_extension(id) {
                if record.trust_level != crate::extensions::ExtensionTrustLevel::LocalBuiltin {
                    println!("Warning: Installing untrusted extension.");
                }
            }

            match registry.install_extension(id) {
                Ok(_) => println!("Successfully installed {}. It is currently DISABLED.", id),
                Err(e) => println!("Error installing: {}", e),
            }
        }
        "enable" => {
            if args.is_empty() {
                println!("Usage: goat extensions enable <id>");
                return Ok(());
            }
            match registry.enable_extension(&args[0]) {
                Ok(_) => println!("Successfully enabled {}.", args[0]),
                Err(e) => println!("Error enabling: {}", e),
            }
        }
        "disable" => {
            if args.is_empty() {
                println!("Usage: goat extensions disable <id>");
                return Ok(());
            }
            match registry.disable_extension(&args[0]) {
                Ok(_) => println!("Successfully disabled {}.", args[0]),
                Err(e) => println!("Error disabling: {}", e),
            }
        }
        "remove" => {
            if args.is_empty() {
                println!("Usage: goat extensions remove <id>");
                return Ok(());
            }
            match registry.remove_extension(&args[0]) {
                Ok(_) => println!("Successfully removed {}.", args[0]),
                Err(e) => println!("Error removing: {}", e),
            }
        }
        _ => {
            println!("Unknown action: {}", action);
            println!("Supported actions: list, discover, audit, install, enable, disable, remove");
        }
    }

    Ok(())
}

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

fn handle_models_command(
    config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    use crate::providers::{ModelProviderCapability, ModelProviderRegistry, ModelRouteRequest};

    let mut registry = ModelProviderRegistry::new(config.model_routing.clone());
    for (_, p_cfg) in &config.providers {
        registry.register(p_cfg.clone());
    }

    match action {
        "list" => {
            println!("GOAT Available Models");
            println!("{}", "─".repeat(72));
            for provider in registry.providers.values() {
                if !provider.enabled {
                    continue;
                }
                println!("Provider: {} ({})", provider.name, provider.id);
                println!("  Default Model: {}", provider.default_model);
                if !provider.available_models.is_empty() {
                    println!(
                        "  Available Models: {}",
                        provider.available_models.join(", ")
                    );
                }
                println!();
            }
        }
        "route" => {
            let task_kind = args.get(0).map(|s| s.as_str()).unwrap_or("general");
            let req = ModelRouteRequest {
                agent_id: "cli_user".to_string(),
                task_kind: task_kind.to_string(),
                required_capabilities: vec![],
                local_only: false,
                allow_external: true,
                preferred_provider: None,
                preferred_model: None,
                quality_preference: "balanced".to_string(),
                latency_preference: "balanced".to_string(),
                cost_preference: "balanced".to_string(),
                fallback_allowed: true,
            };
            let decision = registry.route(&req);
            println!("Routing decision for task '{}':", task_kind);
            println!("  Provider: {}", decision.provider_id);
            println!("  Model: {}", decision.model);
            println!("  Local Only: {}", decision.local_only);
            println!("  Notes: {}", decision.notes);
        }
        _ => {
            println!("Unknown action: {}", action);
            println!("Usage: goat models <list|route>");
        }
    }
    Ok(())
}

fn handle_providers_command(config: &crate::config::Config, action: &str) -> anyhow::Result<()> {
    use crate::providers::{
        ModelProviderAdapter, ModelProviderRegistry, ModelProviderStatus, OpenAiCompatibleAdapter,
    };

    let mut registry = ModelProviderRegistry::new(config.model_routing.clone());
    for (_, p_cfg) in &config.providers {
        registry.register(p_cfg.clone());
    }

    match action {
        "list" => {
            println!("GOAT Model Providers");
            println!("{}", "─".repeat(72));
            for provider in registry.providers.values() {
                let status_icon = if provider.enabled { "✓" } else { "✗" };
                println!("  {} {:15} ({})", status_icon, provider.name, provider.id);
            }
        }
        "doctor" => {
            println!("GOAT Provider Doctor");
            println!("{}", "─".repeat(72));
            for provider in registry.providers.values() {
                if !provider.enabled {
                    continue;
                }
                let adapter = OpenAiCompatibleAdapter::new(
                    provider.base_url.clone().unwrap_or_default(),
                    config.provider_api_key(&provider.id),
                    provider.timeout_secs,
                );
                let status = adapter.status();
                let status_str = match status {
                    ModelProviderStatus::Ready => "Ready",
                    ModelProviderStatus::NotConfigured => "Not Configured",
                    ModelProviderStatus::MissingKey => "Missing API Key",
                    ModelProviderStatus::Unreachable => "Unreachable",
                    ModelProviderStatus::Unknown => "Unknown",
                };
                let status_icon = if status == ModelProviderStatus::Ready {
                    "✓"
                } else {
                    "!"
                };
                println!("  {} {:15} {}", status_icon, provider.name, status_str);
            }
        }
        _ => {
            println!("Unknown action: {}", action);
            println!("Usage: goat providers <list|doctor>");
        }
    }
    Ok(())
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
    action: String,
    arg: String,
    scope: Option<String>,
    kind: Option<String>,
    text: Option<String>,
    mission: Option<String>,
    project: Option<String>,
) -> anyhow::Result<()> {
    use crate::memory::{MemoryItem, MemoryKind, MemoryManager, MemoryScope, MemoryStatus};
    let manager = MemoryManager::new(paths, config.memory.clone());

    match action.as_str() {
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
            if arg.is_empty() {
                println!("--- USER.md ---");
                println!("{}", manager.get_user_content().unwrap_or_default());
                println!("--- MEMORY.md ---");
                println!("{}", manager.get_memory_content().unwrap_or_default());
            } else {
                if let Ok(Some(mem)) = manager.get_structured_memory(&arg) {
                    println!("[MEMORY] ID: {}", mem.memory_id);
                    println!("Scope: {:?} | Kind: {:?}", mem.scope, mem.kind);
                    println!("Title: {}", mem.title);
                    println!("Content: {}", mem.content);
                } else {
                    println!("Memory not found: {}", arg);
                }
            }
        }
        "path" => {
            println!("USER.md:   {}", manager.user_file.display());
            println!("MEMORY.md: {}", manager.memory_file.display());
            println!("Structured:{}", manager.structured_store.display());
        }
        "edit" => {
            println!("To edit memory files, open these in your editor:");
            println!("  {}", manager.user_file.display());
            println!("  {}", manager.memory_file.display());
        }
        "user" => {
            if arg == "edit" {
                println!(
                    "Open {} in your editor to edit.",
                    manager.user_file.display()
                );
            } else if arg == "show" {
                println!("--- USER.md ---");
                println!("{}", manager.get_user_content().unwrap_or_default());
            } else {
                println!("Usage: goat memory user [show|edit]");
            }
        }
        "project" => {
            if arg == "show" {
                if let Some(pid) = project {
                    println!("--- PROJECT_MEMORY.md for {} ---", pid);
                    println!("{}", manager.get_project_memory(&pid).unwrap_or_default());
                } else {
                    println!("Please provide --project <id>");
                }
            } else if arg == "update" {
                if let (Some(pid), Some(txt)) = (project, text) {
                    manager.update_project_memory(&pid, &txt)?;
                    println!("Updated PROJECT_MEMORY.md for {}", pid);
                } else {
                    println!("Please provide --project <id> and --text \"...\"");
                }
            } else {
                println!("Usage: goat memory project [show|update] --project <id>");
            }
        }
        "add" => {
            if let Some(t) = text {
                let scope_enum = match scope.as_deref().unwrap_or("system") {
                    "user" => MemoryScope::User,
                    "project" => MemoryScope::Project,
                    "mission" => MemoryScope::Mission,
                    "skill" => MemoryScope::Skill,
                    _ => MemoryScope::System,
                };
                let kind_enum = match kind.as_deref().unwrap_or("unknown") {
                    "preference" => MemoryKind::Preference,
                    "architecture_note" => MemoryKind::ArchitectureNote,
                    "project_decision" => MemoryKind::ProjectDecision,
                    "command" => MemoryKind::Command,
                    "workflow" => MemoryKind::Workflow,
                    "bug_fix" => MemoryKind::BugFix,
                    _ => MemoryKind::Unknown,
                };
                let item = MemoryItem {
                    memory_id: "".to_string(),
                    scope: scope_enum,
                    project_id: project,
                    mission_id: mission,
                    source: "manual".to_string(),
                    kind: kind_enum,
                    title: format!("Manual entry"),
                    content: t.clone(),
                    tags: vec![],
                    confidence: 100,
                    status: MemoryStatus::Active,
                    created_at: 0,
                    updated_at: 0,
                    last_used_at: None,
                    use_count: 0,
                };
                let id = manager.add_structured_memory(item)?;
                println!("Added structured memory: {}", id);
            } else {
                println!("Please provide --text \"...\"");
            }
        }
        "list" => {
            let mems = manager.list_structured_memories()?;
            if mems.is_empty() {
                println!("No memories found.");
            } else {
                for m in mems {
                    println!("- [{}] ({:?}) {}", m.memory_id, m.scope, m.title);
                }
            }
        }
        "search" => {
            let mems = manager.search_structured_memories(&arg)?;
            if mems.is_empty() {
                println!("No matches found for: {}", arg);
            } else {
                for m in mems {
                    println!("- [{}] ({:?}) {}", m.memory_id, m.scope, m.title);
                }
            }
        }
        "archive" => {
            if arg.is_empty() {
                println!("Please provide a memory ID to archive.");
            } else {
                manager.archive_memory(&arg)?;
                println!("Archived memory: {}", arg);
            }
        }
        "extract" => {
            if let Some(mid) = mission {
                let mc = crate::mission_control::MissionControlManager::new();
                let missions = mc.get_missions();
                if let Some(m) = missions.into_iter().find(|m| m.mission_id == mid) {
                    println!("[MEMORY] Extracting insights from mission: {}", m.title);
                    let content = format!("Goal: {}\nOutcome: {:?}\nNotes: {}", m.raw_goal, m.status, m.notes.join("\n"));
                    let item = MemoryItem {
                        memory_id: "".to_string(),
                        scope: MemoryScope::Mission,
                        project_id: m.linked_project.clone(),
                        mission_id: Some(m.mission_id.clone()),
                        source: "extract".to_string(),
                        kind: MemoryKind::ProjectDecision,
                        title: format!("Mission Insight: {}", m.title),
                        content,
                        tags: vec!["mission-extraction".to_string()],
                        confidence: 80,
                        status: MemoryStatus::Active,
                        created_at: 0,
                        updated_at: 0,
                        last_used_at: None,
                        use_count: 0,
                    };
                    let id = manager.add_structured_memory(item)?;
                    println!("Saved memory: {}", id);
                } else {
                    println!("Mission not found: {}", mid);
                }
            } else {
                println!("Please provide --mission <id> to extract memory.");
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
    name_arg: Option<&str>,
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
        "new" => {
            let name = name_arg
                .or(arg)
                .ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let path = skill_manager.create_template(name)?;
            println!("Created skill template at: {}", path.display());
            println!("Edit this file to implement your skill.");
            let _ = skill_manager.list_skills();
        }
        "create" => {
            let name = name_arg
                .or(arg)
                .ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let path = skill_manager.create_template(name)?;
            println!("Created skill template at: {}", path.display());
            println!("Edit this file to implement your skill.");
            let _ = skill_manager.list_skills();
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
        "run" => {
            let name = arg.ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let skill = skill_manager
                .get_skill(name)
                .ok_or_else(|| anyhow::anyhow!("Skill not found"))?;

            let runner = crate::skill_runner::SkillRunner::new(&paths.data_dir);
            let exec = runner.start_execution(&skill, None, None)?;
            println!(
                "Started execution {} for skill '{}'",
                exec.execution_id, skill.name
            );
            println!("Steps to execute: {}", exec.total_steps);
            // We just print status here. A real TUI/CLI would loop and ask for approvals.
        }
        "runs" => {
            let runner = crate::skill_runner::SkillRunner::new(&paths.data_dir);
            let runs = runner.list_executions();
            if runs.is_empty() {
                println!("No skill runs found.");
            } else {
                println!("Skill Runs ({}):", runs.len());
                for r in runs {
                    println!(
                        "  - {} (Skill: {}, Status: {:?})",
                        r.execution_id, r.skill_name, r.status
                    );
                }
            }
        }
        "run-show" => {
            let id = arg.ok_or_else(|| anyhow::anyhow!("Expected run ID"))?;
            let runner = crate::skill_runner::SkillRunner::new(&paths.data_dir);
            if let Some(run) = runner.get_execution(id)? {
                println!("Execution ID: {}", run.execution_id);
                println!("Skill: {}", run.skill_name);
                println!("Status: {:?}", run.status);
                println!("Steps: {} / {}", run.current_step, run.total_steps);
            } else {
                println!("Run '{}' not found", id);
            }
        }
        "recommend" => {
            // naive recommendation based on stack/goals (which could be the arg)
            let query = arg.unwrap_or("");
            let results = skill_manager.search_skills(query);
            println!("Recommended skills for '{}':", query);
            for s in results.iter().take(5) {
                println!("  - {name:<20} {desc}", name = s.name, desc = s.description);
            }
        }
        "curate" => {
            let runner = crate::skill_runner::SkillRunner::new(&paths.data_dir);
            let runs = runner.list_executions();
            println!("Skill Curator Report");
            println!("====================");
            println!("Total runs recorded: {}", runs.len());
            // compute some stats
            let mut completed = 0;
            let mut failed = 0;
            for r in runs {
                match r.status {
                    crate::skill_runner::SkillExecutionStatus::Completed => completed += 1,
                    crate::skill_runner::SkillExecutionStatus::Failed => failed += 1,
                    _ => {}
                }
            }
            println!("Completed runs: {}", completed);
            println!("Failed runs: {}", failed);
        }
        "create-from-mission" => {
            let mission_id = arg.ok_or_else(|| anyhow::anyhow!("Expected mission ID"))?;
            let mission_manager = crate::mission_control::MissionControlManager::new();

            let mission = mission_manager
                .get_missions()
                .into_iter()
                .find(|m| m.mission_id == mission_id)
                .ok_or_else(|| anyhow::anyhow!("Mission '{}' not found", mission_id))?;

            let name = name_arg.unwrap_or_else(|| "mission-skill").to_string();

            // Ask for confirmation
            println!(
                "Do you want to save this mission as a reusable skill '{}'? (y/N)",
                name
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Skill creation cancelled.");
                return Ok(());
            }

            println!("Extracting skill '{}' from mission {}...", name, mission_id);

            let mut registry = crate::models::ProfileRegistry::from_config(&config.profiles);
            let mut router = crate::llm::LlmRouter::from_config(config);

            let profile_name = &registry.default_profile;
            let (_, chain) = registry.resolve(profile_name);

            let steps: Vec<String> = mission
                .plan_steps
                .iter()
                .map(|s| format!("- {}: {}", s.title, s.description))
                .collect();
            let steps_str = steps.join("\n");

            let prompt = format!(
                "You are a skill curator. The user wants to extract a reusable skill from the following completed mission.\n\
                 Generate a valid SKILL.md file.\n\
                 The skill name should be: {}\n\
                 Mission Goal: {}\n\
                 Steps Executed:\n{}\n\
                 Artifacts Produced: {}\n\n\
                 Use the following format strictly:\n\
                 ---\n\
                 name: {}\n\
                 description: <short summary>\n\
                 version: 0.1.0\n\
                 status: active\n\
                 source: mission\n\
                 source_mission_id: {}\n\
                 risk_level: <low|medium|high>\n\
                 ---\n\n\
                 # Skill: {}\n\n\
                 ## When to use\n\
                 <triggers>\n\n\
                 ## Required context\n\
                 <context>\n\n\
                 ## Procedure\n\
                 <step by step>\n\n\
                 ## Success criteria\n\
                 <criteria>\n\n\
                 Output only the Markdown content.",
                name,
                mission.raw_goal,
                steps_str,
                mission.expected_artifacts.join(", "),
                name,
                mission_id,
                name
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
                    let skill_dir = skill_manager.skills_dir().join(&name);
                    std::fs::create_dir_all(&skill_dir)?;
                    let skill_file = skill_dir.join("SKILL.md");
                    std::fs::write(&skill_file, content)?;
                    println!(
                        "Extracted and saved skill '{}' to {}",
                        name,
                        skill_file.display()
                    );
                    let _ = skill_manager.list_skills();
                }
                Err(e) => anyhow::bail!("Failed to extract skill from LLM: {}", e),
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

fn handle_agent_command(
    mut rt: crate::runtime::GoatRuntime,
    action: &str,
    arg: Option<&str>,
    prompt: Option<&str>,
    mission: Option<&str>,
) {
    let mut ext_mgr = rt.external_agent_manager;
    ext_mgr.detect_all(&rt.config);

    match action {
        "list" | "agents" => {
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
                println!("Risk: {:?}", agent.risk_level);
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
            let mut checks = Vec::new();
            use crate::paths::{DoctorCheck, DoctorStatus};

            checks.push(DoctorCheck {
                status: DoctorStatus::Info,
                label: "External Agents global config".to_string(),
                detail: if rt.config.external_agents.enabled {
                    "Enabled".to_string()
                } else {
                    "Disabled".to_string()
                },
            });

            for agent in ext_mgr.registry.list_all() {
                let status = match agent.status {
                    crate::external_agents::ExternalAgentStatus::Detected => DoctorStatus::Ok,
                    crate::external_agents::ExternalAgentStatus::Missing => DoctorStatus::Warn,
                    crate::external_agents::ExternalAgentStatus::Disabled => DoctorStatus::Info,
                    crate::external_agents::ExternalAgentStatus::NeedsConfig => DoctorStatus::Warn,
                    crate::external_agents::ExternalAgentStatus::Unsupported => DoctorStatus::Warn,
                };
                let detail = match agent.status {
                    crate::external_agents::ExternalAgentStatus::Detected => {
                        format!(
                            "Found at {}",
                            agent.detected_path.as_ref().unwrap().display()
                        )
                    }
                    crate::external_agents::ExternalAgentStatus::Missing => {
                        "Command not found in PATH".to_string()
                    }
                    crate::external_agents::ExternalAgentStatus::Disabled => {
                        "Disabled by configuration".to_string()
                    }
                    crate::external_agents::ExternalAgentStatus::NeedsConfig => {
                        "Needs configuration (API key, etc)".to_string()
                    }
                    crate::external_agents::ExternalAgentStatus::Unsupported => {
                        "Unsupported environment or version".to_string()
                    }
                };
                checks.push(DoctorCheck {
                    status,
                    label: format!("Adapter: {}", agent.name),
                    detail,
                });
            }

            let audit_log = &rt.paths.external_agent_audit_log_file;
            checks.push(DoctorCheck {
                status: if audit_log.exists() {
                    DoctorStatus::Ok
                } else {
                    DoctorStatus::Info
                },
                label: "Audit Log".to_string(),
                detail: if audit_log.exists() {
                    audit_log.display().to_string()
                } else {
                    "Not created yet".to_string()
                },
            });

            let runs_file = rt.paths.data_dir.join("external-agent-runs.jsonl");
            checks.push(DoctorCheck {
                status: if runs_file.exists() {
                    DoctorStatus::Ok
                } else {
                    DoctorStatus::Info
                },
                label: "Runs storage".to_string(),
                detail: if runs_file.exists() {
                    runs_file.display().to_string()
                } else {
                    "Not created yet".to_string()
                },
            });

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
                                "  {} | Agent: {:<12} | Profile: {:<15} | Status: {}",
                                run.run_id, run.agent_name, run.permission_profile, run.status
                            );
                        }
                    }
                }
            } else {
                println!("No runs recorded yet.");
            }
        }
        "run-show" => {
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
                                if run.run_id == run_id {
                                    println!("Run ID: {}", run.run_id);
                                    println!("Agent: {}", run.agent_name);
                                    println!("Timestamp: {}", run.started_at);
                                    println!("Profile: {}", run.permission_profile);
                                    println!("Workspace: {}", run.working_directory.display());
                                    println!("Task: {}", run.task_summary);
                                    println!("Status: {}", run.status);
                                    if let Some(finished_at) = run.finished_at {
                                        println!(
                                            "Duration: {:?}",
                                            finished_at.signed_duration_since(run.started_at)
                                        );
                                    }
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
                println!("Usage: goat agent run-show <id>");
            }
        }
        "run" => {
            let name = arg.unwrap_or("");
            let task = prompt.unwrap_or("Test run");
            println!("Delegating task to external agent '{}'...", name);

            let action_res = rt
                .tool_registry
                .evaluate_action("delegate_external_agent", &rt.config.tools);
            if let crate::tool_registry::ToolAction::Deny(reason) = action_res {
                println!("Delegation denied by tool registry: {}", reason);
                return;
            }

            let req = crate::approval::ApprovalRequest {
                tool_name: "delegate_external_agent".to_string(),
                action_summary: format!("agent: {}, task: {}", name, task),
                risk_level: crate::approval::RiskLevel::High,
                explanation: Some("Running external agent command".into()),
                working_directory: None,
            };

            let decision = if let Some(decision) = rt.approval_gate.check_policy(&req) {
                decision
            } else {
                let mut lines = req.display_lines();
                for line in lines {
                    println!("{}", line);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let char_in = input.trim().chars().next().unwrap_or('n');
                rt.approval_gate.resolve(&req, char_in)
            };

            match ext_mgr.delegate(
                name,
                task,
                &rt.config,
                decision.clone(),
                mission.map(|s| s.to_string()),
            ) {
                Ok(res) => {
                    println!("Execution finished. Success: {}", res.success);
                    println!("Stdout:\n{}", res.stdout);
                    if !res.stderr.is_empty() {
                        println!("Stderr:\n{}", res.stderr);
                    }
                }
                Err(e) => {
                    if let crate::approval::ApprovalDecision::Approved = decision {
                        println!("Error: {}", e);
                    } else {
                        println!("Execution denied.");
                    }
                }
            }
        }
        "compare" => {
            println!("Compare feature is planned for Phase 8.9.");
        }
        _ => {
            println!("Unknown agent action: {}", action);
            println!(
                "Valid actions: list, doctor, runs, run-show <id>, run <name> --prompt <...>, compare"
            );
        }
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
    paths: &crate::paths::GoatPaths,
    _config: &crate::config::Config,
    action: &str,
    arg: Option<&str>,
) -> anyhow::Result<()> {
    let mut rt = crate::agent_runtime::AgentRuntime::new(
        crate::agent_runtime::AgentRuntimeConfig::default(),
        paths.runtime_dir.clone(),
    )?;
    match action {
        "list" => {
            let jobs = rt.list_jobs();
            println!("[RUNTIME] Active jobs:");
            if jobs.is_empty() {
                println!("  No jobs found.");
            } else {
                for job in jobs {
                    println!(
                        "  - [{}] {} ({:?}) - {}",
                        job.id, job.agent_id, job.status, job.input_summary
                    );
                }
            }
        }
        "show" => {
            if let Some(id) = arg {
                if let Some(job) = rt.get_job(id) {
                    println!("Job ID: {}", job.id);
                    println!("Agent: {}", job.agent_id);
                    println!("Status: {:?}", job.status);
                    println!("Task: {}", job.input_summary);
                    println!("Artifacts: {:?}", job.artifacts);
                } else {
                    println!("Job {} not found.", id);
                }
            } else {
                println!("Usage: goat jobs show <id>");
            }
        }
        "pause" => {
            if let Some(id) = arg {
                let _ = rt.pause_job(id);
                println!("Job {} paused.", id);
            }
        }
        "resume" => {
            if let Some(id) = arg {
                let _ = rt.resume_job(id);
                println!("Job {} resumed.", id);
            }
        }
        "cancel" => {
            if let Some(id) = arg {
                let _ = rt.cancel_job(id);
                println!("Job {} cancelled.", id);
            }
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

fn handle_dashboard_command(action: &str) {
    let root = std::env::current_dir().unwrap_or_default();
    let dashboard_dir = root.join("apps").join("dashboard");
    let fallback_dir = root.join("dashboard");

    let active_dir = if dashboard_dir.exists() {
        dashboard_dir
    } else if fallback_dir.exists() {
        fallback_dir
    } else {
        println!("[DASHBOARD] Cannot find dashboard/ or apps/dashboard/ directory.");
        return;
    };

    match action {
        "path" => {
            println!("{}", active_dir.display());
        }
        "doctor" => {
            println!("[DASHBOARD DOCTOR]");
            println!("  Dashboard Path: {}", active_dir.display());
            let pkg_json = active_dir.join("package.json");
            println!(
                "  package.json: {}",
                if pkg_json.exists() {
                    "Found"
                } else {
                    "Missing"
                }
            );
            println!(
                "  To run the dashboard, navigate to the path, run `npm install`, then `npm run dev`."
            );
        }
        "dev" => {
            println!("[DASHBOARD] Dashboard code is at: {}", active_dir.display());
            println!("  Please run the following in a new terminal:");
            println!("    cd {}", active_dir.display());
            println!("    npm install");
            println!("    npm run dev");
        }
        _ => {
            println!(
                "[DASHBOARD] Unknown action '{}'. Use dev, path, or doctor.",
                action
            );
        }
    }
}

fn handle_desktop_command(action: &str) {
    let root = std::env::current_dir().unwrap_or_default();
    let desktop_dir = root.join("apps").join("desktop");

    if !desktop_dir.exists() {
        println!("[DESKTOP] Cannot find apps/desktop/ directory.");
        return;
    }

    match action {
        "path" => {
            println!("{}", desktop_dir.display());
        }
        "doctor" => {
            println!("[DESKTOP DOCTOR]");
            println!("  Desktop Path: {}", desktop_dir.display());
            let pkg_json = desktop_dir.join("package.json");
            println!(
                "  package.json: {}",
                if pkg_json.exists() {
                    "Found"
                } else {
                    "Missing"
                }
            );
            let tauri_conf = desktop_dir.join("src-tauri").join("tauri.conf.json");
            println!(
                "  tauri.conf.json: {}",
                if tauri_conf.exists() {
                    "Found"
                } else {
                    "Missing"
                }
            );
            println!(
                "  To run the desktop app, navigate to the path, run `npm install`, then `npm run tauri dev`."
            );
        }
        "run" | "dev" => {
            println!(
                "[DESKTOP] Desktop app code is at: {}",
                desktop_dir.display()
            );
            println!("  Please run the following in a new terminal:");
            println!("    cd {}", desktop_dir.display());
            println!("    npm install");
            println!("    npm run dev");
        }
        _ => {
            println!(
                "[DESKTOP] Unknown action '{}'. Use dev, run, path, or doctor.",
                action
            );
        }
    }
}

fn handle_browser_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    use crate::browser_adapter::BrowserAdapterManager;
    use crate::browser_workflows::BrowserWorkflowManager;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let manager = BrowserWorkflowManager::new(&paths.data_dir);
    let browser_config = config.browser.clone();
    let mut browser_adapter = BrowserAdapterManager::new(browser_config);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    match action {
        "workflows" => {
            let list = manager.list_workflows()?;
            println!("Browser Workflows ({}):", list.len());
            for w in list {
                println!("- {} [{}] -> Status: {:?}", w.id, w.title, w.status);
            }
        }
        "screenshot" => {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            println!("Creating workflow for screenshot of {}", url);
            let w = manager.create_workflow("Screenshot Capture", &url, "screenshot");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        "inspect" => {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            println!("Creating workflow for inspection of {}", url);
            let w = manager.create_workflow("DOM Inspection", &url, "inspect");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        "qa" => {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            println!("Creating workflow for QA of {}", url);
            let w = manager.create_workflow("UI QA", &url, "ui-qa");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        "landing-review" => {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            println!("Creating workflow for Landing Page Review of {}", url);
            let w = manager.create_workflow("Landing Page Review", &url, "landing-review");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        "dashboard-qa" => {
            println!("Creating workflow for Dashboard QA");
            let w =
                manager.create_workflow("Dashboard QA", "http://localhost:3000", "dashboard-qa");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        "health" => {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            println!("Creating workflow for Health Check of {}", url);
            let w = manager.create_workflow("Web Health Check", &url, "web-health-check");
            manager.save_workflow(&w)?;
            let updated = rt.block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
            println!("Workflow Completed. Status: {:?}", updated.status);
        }
        _ => {
            println!("Unknown action: {}", action);
        }
    }
    Ok(())
}

fn handle_builder_command(
    paths: &crate::paths::GoatPaths,
    config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    use crate::agents::builder::{BuilderAgent, BuilderInspectionScope};
    use crate::brain_index::BrainIndexManager;

    let agent = BuilderAgent::new()?;
    let brain_mgr = BrainIndexManager::new(
        paths.clone(),
        config.brain_index.clone(),
        &config.embeddings,
    );

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    match action {
        "inspect" => {
            let result = agent.inspect_repo(BuilderInspectionScope {
                max_depth: 3,
                include_tests: true,
            })?;
            println!("[BUILDER] Inspection complete. Snapshot generated.");
            println!("Root: {}", result.snapshot.root_path);
            println!(
                "Main Language: {}",
                result.snapshot.tech_stack.main_language
            );
            println!("Files scanned: {}", result.snapshot.file_count);
        }
        "plan" => {
            let goal = args.join(" ");
            if goal.is_empty() {
                println!("[BUILDER] Please provide a goal.");
                return Ok(());
            }
            let plan = rt.block_on(agent.plan_patch(&goal, &brain_mgr))?;
            println!("[BUILDER] Patch Plan Generated (ID: {})", plan.id);
            println!("Goal: {}", plan.goal);
            println!("Risk Level: {}", plan.risk_level);
        }
        "diff-review" => {
            let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
            let review = agent.diff_review(plan_id)?;
            println!("[BUILDER] Diff Review Complete.");
            println!("Overall Severity: {:?}", review.overall_severity);
            for finding in review.findings {
                println!("- [{}]: {}", finding.file_path, finding.issue_description);
            }
        }
        "test-plan" => {
            let goal = args.join(" ");
            let plan = agent.test_plan(&goal)?;
            println!("[BUILDER] Test Plan Created (ID: {})", plan.plan_id);
            for cmd in plan.commands {
                println!("Command: {}", cmd.command);
            }
        }
        "validate" => {
            let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
            let result = agent.validate(plan_id)?;
            println!("[BUILDER] Validation Finished. Valid: {}", result.is_valid);
            println!("Logs:\n{}", result.test_logs);
        }
        "rollback-plan" => {
            let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
            let rollback = agent.rollback_plan(plan_id)?;
            println!("[BUILDER] Rollback Plan generated.");
            println!("Fallback command: {}", rollback.command_fallback);
        }
        _ => {
            println!(
                "Unknown action: {}. Use inspect, plan, diff-review, test-plan, validate, rollback-plan",
                action
            );
        }
    }
    Ok(())
}

fn handle_researcher_command(
    _paths: &crate::paths::GoatPaths,
    _config: &crate::config::Config,
    action: &str,
    args: &[String],
) -> anyhow::Result<()> {
    match action {
        "projects" => println!("[RESEARCHER] Projects list:"),
        "new" => {
            let q = args.join(" ");
            println!("[RESEARCHER] Creating project: {}", q);
        }
        "add-source" => println!("[RESEARCHER] Adding source to project"),
        "ingest-browser" => println!("[RESEARCHER] Ingesting browser artifact"),
        "brief" => println!("[RESEARCHER] Generating brief for project"),
        "competitors" => println!("[RESEARCHER] Scanning competitors"),
        "compare-tech" => println!("[RESEARCHER] Comparing technology options"),
        "report" => println!("[RESEARCHER] Generating report"),
        _ => println!(
            "[RESEARCHER] Unknown action. Use projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report"
        ),
    }
    Ok(())
}
