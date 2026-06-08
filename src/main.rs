mod app;
mod approval;
mod brain;
mod cli;
mod config;
mod error;
mod llm;
mod mcp;
mod paths;
mod swarm;
mod tools;
mod ui;

use anyhow::{Context, Result};
use app::App;
use clap::Parser;
use cli::Cli;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;

/// Initialise file-based logging.
///
/// Log files are written to the GOAT data directory (`<data_dir>/logs/`).
/// Falls back to `./logs` if the data directory is not yet available.
fn init_logging(log_dir: &std::path::Path) -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(log_dir, "goat.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .init();

    guard
}

#[tokio::main]
async fn main() -> Result<()> {
    // ── 1. Parse CLI arguments ────────────────────────────────────────────────
    let cli = Cli::parse();

    // ── 2. Resolve paths (XDG + CLI overrides) ────────────────────────────────
    let mut goat_paths =
        paths::GoatPaths::resolve().context("failed to determine GOAT data paths")?;

    if let Some(cfg) = &cli.config {
        goat_paths = goat_paths.with_config(cfg.clone());
    }
    if let Some(db) = &cli.db {
        goat_paths = goat_paths.with_db(db.clone());
    }

    // Ensure data and log directories exist before we try to write logs.
    // These are non-fatal — we fall back to ./logs if needed.
    if goat_paths.ensure_data_dir().is_err() {
        eprintln!(
            "[WARN] Could not create data directory: {} — falling back to ./logs",
            goat_paths.data_dir.display()
        );
    }
    let _ = goat_paths.ensure_log_dir();

    // ── 3. Initialise logging ─────────────────────────────────────────────────
    let log_dir = if goat_paths.log_dir.exists() {
        goat_paths.log_dir.clone()
    } else {
        std::path::PathBuf::from("logs")
    };
    let _log_guard = init_logging(&log_dir);
    info!("GOAT starting — version {}", env!("CARGO_PKG_VERSION"));
    info!(
        config = %goat_paths.config_file.display(),
        data_dir = %goat_paths.data_dir.display(),
        db = %goat_paths.db_file.display(),
        "resolved paths"
    );

    // ── 4. Load configuration ─────────────────────────────────────────────────
    let config_result = config::Config::load_from(&goat_paths.config_file).with_context(|| {
        format!(
            "failed to load config from {}",
            goat_paths.config_file.display()
        )
    })?;
    let goat_config = config_result.config;
    let config_warnings = config_result.warnings;
    info!("configuration loaded");

    // ── 5. Handle non-TUI subcommands ─────────────────────────────────────────
    // These run before the terminal enters raw mode so they print cleanly.
    if cli::handle_subcommand(&cli, &goat_paths, &goat_config)
        .await
        .context("subcommand failed")?
    {
        return Ok(());
    }

    // ── 6. Detect legacy DB and warn before entering TUI ──────────────────────
    // This is printed to stderr (before raw mode) so it is visible even if
    // the TUI fails to start.
    if let Some(legacy) = paths::GoatPaths::detect_legacy_db() {
        eprintln!(
            "[WARN] Legacy database detected: {}\n  New database location: {}\n  To migrate: cargo run -- migrate-db",
            legacy.display(),
            goat_paths.db_file.display()
        );
    }

    // ── 7. Launch TUI ─────────────────────────────────────────────────────────
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to create terminal")?;
    info!("terminal initialized");

    let mut app = App::new(goat_config, goat_paths, config_warnings);

    let res = run_app(&mut terminal, &mut app).await;

    // ── 8. Restore terminal ───────────────────────────────────────────────────
    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();
    info!("terminal restored");

    if let Err(ref err) = res {
        error!(error = ?err, "application loop exited with an error");
        eprintln!("Error: {err:?}");
    }

    res.context("TUI event loop failed")?;
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // ── Global exits — always active ───────────────────────────────
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    info!("Ctrl+C received — shutting down");
                    app.push_log("[GOAT] Shutting down… goodbye!");
                    terminal.draw(|f| ui::render(f, app))?;
                    app.shutdown_mcp_servers().await;
                    app.quit();
                    break;
                }

                // ── Approval mode: intercept y/n/a/d when prompt is pending ───
                if app.has_pending_approval() {
                    match key.code {
                        KeyCode::Char(c) if matches!(c, 'y' | 'n' | 'a' | 'd') => {
                            terminal.draw(|f| ui::render(f, app))?;
                            app.resolve_approval(c).await;
                        }
                        KeyCode::Esc => {
                            terminal.draw(|f| ui::render(f, app))?;
                            app.resolve_approval('n').await;
                        }
                        _ => {}
                    }
                } else {
                    // ── Normal input handling — always-active composer ─────────
                    match key.code {
                        KeyCode::Enter => {
                            let msg = app.input.trim().to_string();
                            if !msg.is_empty() {
                                app.input.clear();
                                app.scroll_to_bottom();
                                info!(length = msg.len(), "submitting user input");
                                terminal.draw(|f| ui::render(f, app))?;
                                app.handle_user_input(msg).await;
                            }
                        }

                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }

                        KeyCode::Esc => {
                            if !app.input.is_empty() {
                                app.input.clear();
                            } else {
                                app.scroll_to_bottom();
                            }
                        }

                        KeyCode::Up => app.scroll_up(1),
                        KeyCode::Down => app.scroll_down(1),
                        KeyCode::PageUp => app.scroll_up(10),
                        KeyCode::PageDown => app.scroll_down(10),
                        KeyCode::Home => app.scroll_up(usize::MAX),
                        KeyCode::End => app.scroll_to_bottom(),

                        _ => {}
                    }
                }
            }
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}
