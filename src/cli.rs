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
            let has_openrouter = config.provider_api_key("openrouter").is_some();
            let ollama_enabled = config.providers.contains_key("ollama");
            let checks = crate::paths::run_doctor(
                paths,
                config.provider_api_key("openai").is_some(),
                config.provider_api_key("groq").is_some(),
                has_openrouter,
                ollama_enabled,
                cli.headless,
                Some(&config.llm),
            );
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
