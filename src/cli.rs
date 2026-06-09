//! CLI argument parsing for GOAT using `clap`.
//!
//! Defines the top-level CLI structure and handles all non-TUI subcommands.
//!
//! # Mode selection
//!
//! | Invocation                    | Mode                      |
//! |-------------------------------|---------------------------|
//! | `goat`                        | Interactive TUI           |
//! | `goat --headless`             | Headless stdin/stdout     |
//! | `goat doctor`                 | Print readiness report    |
//! | `goat config-path`            | Print config path         |
//! | `goat data-path`              | Print data dir            |
//! | `goat db-path`                | Print database path       |
//! | `goat sessions`               | List recent sessions      |
//! | `goat migrate-db`             | Migrate legacy DB         |

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
    /// Shows session ID, title, and (if available) the first user message.
    #[command(name = "sessions")]
    Sessions,

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
            let checks = crate::paths::run_doctor(
                paths,
                config.keys.openai_api_key.is_some(),
                config.keys.groq_api_key.is_some(),
                cli.headless,
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

    let sessions = brain
        .get_sessions()
        .context("could not read sessions from database")?;

    if sessions.is_empty() {
        println!("No sessions found in {}", paths.db_file.display());
        return Ok(());
    }

    println!("Sessions ({}):", sessions.len());
    println!("{}", "─".repeat(70));
    for (id, title) in &sessions {
        // Shorten UUID for readability.
        let short_id = if id.len() > 8 { &id[..8] } else { id.as_str() };
        println!("  {}…  {}", short_id, title);
    }
    println!("{}", "─".repeat(70));
    println!("Database: {}", paths.db_file.display());
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

    // Build a temporary LlmRouter just to check provider availability.
    let router = crate::llm::LlmRouter::new(
        config.keys.openai_api_key.clone(),
        config.keys.groq_api_key.clone(),
    );

    println!("GOAT Model Profiles");
    println!("{}", "─".repeat(70));

    // Provider status (never print keys).
    println!("Providers:");
    for provider in &[
        "openai",
        "groq",
        "anthropic",
        "gemini",
        "ollama",
        "openrouter",
    ] {
        println!(
            "  {:12} {}",
            provider,
            router.provider_status_label(provider)
        );
    }
    println!();

    // Profile list.
    println!("Default profile: {}", registry.default_profile);
    println!();
    println!("Profiles:");
    println!("{}", "─".repeat(70));
    for name in registry.profile_names() {
        let (_, chain) = registry.resolve(name);
        let primary = chain.primary_display();
        let fallback = chain.fallback_display();
        // Mark each entry as ready or unavailable.
        let primary_status = if let Some(e) = chain.entries.first() {
            if router.is_provider_available(&e.provider) {
                "✓"
            } else {
                "✗"
            }
        } else {
            "✗"
        };
        println!("  {:12} primary: {} {}", name, primary_status, primary);
        if fallback != "none" {
            println!("  {:12} fallback: {}", "", fallback);
        }
    }
    println!("{}", "─".repeat(70));
    println!();
    println!("Legend: ✓ = provider ready   ✗ = provider not configured");
    println!("To configure a provider, add its API key to ~/.config/goat/goat.toml [keys]");
    println!("To customize profiles, add a [profiles] section to goat.toml");
}
