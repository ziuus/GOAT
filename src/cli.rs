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

    /// List configured providers, model profiles, and fallback chains.
    #[command(name = "models")]
    Models,

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

        Command::Models => {
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

        Command::Skills { action, arg, session } => {
            handle_skills_command(paths, config, action, arg.as_deref(), session.as_deref()).await?;
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
                println!("No skills found in {}", skill_manager.skills_dir().display());
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
                println!("  - {name:<20} {desc}", name=s.name, desc=s.description);
            }
        }
        "create-from-session" => {
            let name = arg.ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let sid = session_id.ok_or_else(|| anyhow::anyhow!("Expected --session <id> for create-from-session"))?;
            
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
            
            match router.completion_with_fallback(&chain, messages, None).await {
                Ok((resp, _)) => {
                    let content = resp.content.unwrap_or_default();
                    let skill_dir = skill_manager.skills_dir().join(name);
                    std::fs::create_dir_all(&skill_dir)?;
                    let skill_file = skill_dir.join("SKILL.md");
                    std::fs::write(&skill_file, content)?;
                    println!("Extracted and saved skill '{}' to {}", name, skill_file.display());
                }
                Err(e) => anyhow::bail!("Failed to extract skill from LLM: {}", e),
            }
        }
        _ => {
            println!("Unknown action '{}'. Expected: list, show, path, create, validate, search, create-from-session.", action);
        }
    }
    Ok(())
}

