pub mod agent_collaboration;
pub mod agent_profiles;
pub mod agent_quality;
pub mod agent_runtime;
pub mod agents;
pub mod api_server;
mod app;
mod approval;
mod brain;
pub mod brain_context;
pub mod brain_index;
pub mod brain_learning;
pub mod brain_models;
pub mod browser_adapter;
pub mod browser_workflows;
pub mod checkpoint;
mod cli;
pub mod code_execution;
pub mod command_registry;
pub mod config;
pub mod daemon;
pub mod embeddings;
pub mod error;
pub mod events;
pub mod extensions;
pub mod external_agents;
pub mod github_workflow;
pub mod headless;
pub mod hooks;
pub mod jobs;
pub mod llm;
pub mod mcp;
pub mod mcp_runtime;
pub mod memory;
pub mod models;
pub mod onboarding;
pub mod paths;
pub mod project;
pub mod project_profiles;
pub mod promptforge;
pub mod provider;
pub mod providers;
pub mod quick_access;
pub mod recipe_marketplace;
pub mod repo_map;
pub mod reports;
pub mod runtime;
pub mod scheduler;
pub mod skill_marketplace;
pub mod skill_researcher;
mod skills;
pub mod studio;
pub mod subagents;
pub mod swarm;
pub mod task;
pub mod timeline;
pub mod tool_registry;
mod tools;
pub mod transports;
mod ui;
pub mod voice;

use anyhow::{Context, Result};
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

// ── Logging ───────────────────────────────────────────────────────────────────

/// Initialise file-based rolling daily logs.
///
/// Logs go to `<data_dir>/logs/` (XDG).  Falls back to `./logs` if the XDG
/// directory could not be created.
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

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    // ── 1. Parse CLI arguments ─────────────────────────────────────────────────
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
    if goat_paths.ensure_data_dir().is_err() {
        eprintln!(
            "[WARN] Could not create data directory: {} — falling back to ./logs",
            goat_paths.data_dir.display()
        );
    }
    let _ = goat_paths.ensure_log_dir();

    // ── 3. Initialise logging ──────────────────────────────────────────────────
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
        headless = cli.headless,
        "resolved paths"
    );

    // ── 4. Load configuration ──────────────────────────────────────────────────
    let config_result = config::Config::load_from(&goat_paths.config_file).with_context(|| {
        format!(
            "failed to load config from {}",
            goat_paths.config_file.display()
        )
    })?;
    let goat_config = config_result.config;
    let config_warnings = config_result.warnings;
    info!("configuration loaded");

    // ── 5. Handle non-TUI/non-headless subcommands ────────────────────────────
    // These run before raw mode and print cleanly to stdout.
    if cli::handle_subcommand(&cli, &goat_paths, &goat_config)
        .await
        .context("subcommand failed")?
    {
        return Ok(());
    }

    // ── 6. Detect legacy DB and warn before entering UI ───────────────────────
    // Printed to stderr so it is visible even if the TUI/headless fails to start.
    if let Some(legacy) = paths::GoatPaths::detect_legacy_db() {
        eprintln!(
            "[WARN] Legacy database detected: {}\n  New location: {}\n  Migrate: goat migrate-db",
            legacy.display(),
            goat_paths.db_file.display()
        );
    }

    // ── 7. Bootstrap shared runtime ───────────────────────────────────────────
    // Shared between TUI and headless: brain, LLM, session, approval gate,
    // model profiles, fallback chain, retry/timeout config.
    let (runtime, boot_log) = runtime::GoatRuntime::bootstrap(
        goat_config,
        goat_paths,
        config_warnings,
        cli.no_brain,
        cli.profile.clone(),
    );

    // ── 8. Route to TUI or headless ───────────────────────────────────────────
    if cli.headless {
        info!("launching headless mode");
        headless::run(runtime)
            .await
            .context("headless mode failed")?;
    } else {
        info!("launching TUI");
        run_tui(runtime, boot_log).await.context("TUI failed")?;
    }

    Ok(())
}

// ── TUI runner ────────────────────────────────────────────────────────────────

async fn run_tui(runtime: runtime::GoatRuntime, boot_log: Vec<String>) -> Result<()> {
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to create terminal")?;
    info!("terminal initialized");

    let mut app = app::App::from_runtime(runtime, boot_log);

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();
    info!("terminal restored");

    if let Err(ref err) = res {
        error!(error = ?err, "TUI loop exited with an error");
        eprintln!("Error: {err:?}");
    }

    res.context("TUI event loop failed")?;
    Ok(())
}

// ── TUI event loop ────────────────────────────────────────────────────────────

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        app.handle_scheduled_jobs().await;

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
                            // If there are suggestions and user presses Enter,
                            // only submit if input is a complete command or not a slash prefix.
                            let msg = app.input.trim().to_string();
                            if !msg.is_empty() {
                                app.cmd_suggestions.clear();
                                app.commit_to_history(&msg);
                                app.input.clear();
                                app.scroll_to_bottom();
                                info!(length = msg.len(), "submitting user input");
                                terminal.draw(|f| ui::render(f, app))?;
                                app.handle_user_input(msg).await;
                            }
                        }

                        KeyCode::Esc => {
                            if !app.cmd_suggestions.is_empty() {
                                // Esc closes suggestions without clearing input
                                app.cmd_suggestions.clear();
                            } else if !app.input.is_empty() {
                                app.input.clear();
                                app.update_suggestions();
                            } else {
                                app.scroll_to_bottom();
                            }
                        }

                        // Tab — complete the selected or first suggestion
                        KeyCode::Tab => {
                            app.complete_suggestion();
                            app.update_suggestions();
                        }

                        // ↑ / ↓ — suggestion navigation when popup is open;
                        // input history when input non-empty; log scroll when input is empty.
                        KeyCode::Up => {
                            if !app.cmd_suggestions.is_empty() {
                                app.suggestion_up();
                            } else if !app.input.is_empty() || app.history_idx.is_some() {
                                app.history_up();
                            } else {
                                app.scroll_up(1);
                            }
                        }
                        KeyCode::Down => {
                            if !app.cmd_suggestions.is_empty() {
                                app.suggestion_down();
                            } else if app.history_idx.is_some() {
                                app.history_down();
                            } else {
                                app.scroll_down(1);
                            }
                        }
                        KeyCode::PageUp => app.scroll_up(10),
                        KeyCode::PageDown => app.scroll_down(10),
                        KeyCode::Home => app.scroll_up(usize::MAX),
                        KeyCode::End => app.scroll_to_bottom(),

                        // Ctrl+L — clear log (same as /clear)
                        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/clear").await;
                        }

                        // View shortcuts
                        KeyCode::Char('1') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view chat").await;
                        }
                        KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view tasks").await;
                        }
                        KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view repo").await;
                        }
                        KeyCode::Char('4') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view patches").await;
                        }
                        KeyCode::Char('5') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view tools").await;
                        }
                        KeyCode::Char('6') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view memory").await;
                        }
                        KeyCode::Char('7') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view skills").await;
                        }
                        KeyCode::Char('8') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view subagents").await;
                        }
                        KeyCode::Char('9') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/view external").await;
                        }

                        // Command Palette
                        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/palette").await;
                        }

                        // Phase 3.2: Layout shortcuts
                        // Ctrl+B — toggle sidebar (in dashboard mode)
                        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.sidebar_visible = !app.sidebar_visible;
                            let state = if app.sidebar_visible {
                                "visible"
                            } else {
                                "hidden"
                            };
                            app.push_log(format!("[SYSTEM] Sidebar {}", state));
                        }
                        // Ctrl+R — toggle context panel (in dashboard mode)
                        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.context_visible = !app.context_visible;
                            let state = if app.context_visible {
                                "visible"
                            } else {
                                "hidden"
                            };
                            app.push_log(format!("[SYSTEM] Context panel {}", state));
                        }
                        // Ctrl+F — switch to Focus layout
                        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/layout focus").await;
                        }
                        // Ctrl+D — switch to Dashboard layout
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let _ = app.handle_slash_command("/layout dashboard").await;
                        }

                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.update_suggestions();
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                            app.update_suggestions();
                        }

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
