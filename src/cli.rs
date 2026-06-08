//! CLI argument parsing for GOAT using `clap`.
//!
//! This module defines the top-level CLI structure and handles all
//! non-TUI subcommands (doctor, config-path, data-path, db-path, etc.).
//!
//! The TUI is launched only when no subcommand is given (or `run` is used).
//! All other commands print to stdout and exit immediately, so they work
//! without a terminal emulator (CI, scripts, health checks, etc.).

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// GOAT — General Omniscient Agentic Tool
///
/// Universal AI CLI/TUI agent platform.
/// Run without arguments to launch the interactive TUI.
#[derive(Parser, Debug)]
#[command(
    name = "goat",
    version = env!("CARGO_PKG_VERSION"),
    about = "GOAT — Universal AI CLI/TUI agent platform",
    long_about = "GOAT (General Omniscient Agentic Tool) is a Rust-first, terminal-native \
                  AI agent platform. Run without arguments to start the interactive TUI.\n\n\
                  Config:   ~/.config/goat/goat.toml\n\
                  Data:     ~/.local/share/goat/\n\
                  Database: ~/.local/share/goat/goat.db\n\
                  Logs:     ~/.local/share/goat/logs/"
)]
pub struct Cli {
    /// Path to a custom config file (overrides default ~/.config/goat/goat.toml).
    #[arg(long, value_name = "PATH", global = true)]
    pub config: Option<PathBuf>,

    /// Path to a custom brain database file (overrides XDG data path).
    #[arg(long, value_name = "PATH", global = true)]
    pub db: Option<PathBuf>,

    /// Subcommand to run. If omitted, the TUI is launched.
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
    /// Checks: config file, config permissions, data directory, database,
    /// legacy DB migration, provider keys, approval gate, log directory.
    #[command(name = "doctor")]
    Doctor,

    /// Migrate the legacy project-root goat_brain.db to the XDG data path.
    ///
    /// Detects `./goat_brain.db` and copies it to the new location.
    /// The original file is NOT deleted (manual removal required).
    #[command(name = "migrate-db")]
    MigrateDb,
}

/// Handle CLI subcommands that do not need the TUI.
///
/// Returns `true` if a subcommand was handled (program should exit after),
/// `false` if the TUI should be launched.
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
            );
            crate::paths::print_doctor_results(&checks);
            Ok(true)
        }

        Command::MigrateDb => {
            handle_migrate_db(paths)?;
            Ok(true)
        }
    }
}

/// Copy the legacy `./goat_brain.db` to the XDG data path if found.
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

    // Ensure data dir exists.
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
