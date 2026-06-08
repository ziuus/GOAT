mod app;
mod approval;
mod brain;
mod config;
mod llm;
mod mcp;
mod swarm;
mod tools;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Duration;
use std::{error::Error, io};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;

fn init_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "goat.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .init();

    guard
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _log_guard = init_logging();
    info!("starting GOAT");

    let config = match config::Config::load() {
        Ok(config) => {
            info!("configuration loaded");
            config
        }
        Err(err) => {
            error!(error = %err, "failed to load configuration");
            return Err(err);
        }
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    info!("terminal initialized");

    let mut app = App::new(config);

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    info!("terminal restored");

    if let Err(err) = res {
        error!(error = ?err, "application loop exited with an error");
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // ── Global exits — always active ───────────────────────────────
                // Ctrl+C: safe exit regardless of any other mode.
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
                            // Esc during approval = deny (safe default).
                            terminal.draw(|f| ui::render(f, app))?;
                            app.resolve_approval('n').await;
                        }
                        // Any other key ignored during approval — user must respond.
                        _ => {}
                    }
                } else {
                    // ── Normal input handling — always-active composer ─────────
                    match key.code {
                        // ── Send message ────────────────────────────────────
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

                        // ── Typing ──────────────────────────────────────────
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }

                        // ── Cancel / clear input ────────────────────────────
                        KeyCode::Esc => {
                            if !app.input.is_empty() {
                                app.input.clear();
                            } else {
                                app.scroll_to_bottom();
                            }
                        }

                        // ── Log scrolling ────────────────────────────────────
                        KeyCode::Up => {
                            app.scroll_up(1);
                        }
                        KeyCode::Down => {
                            app.scroll_down(1);
                        }
                        KeyCode::PageUp => {
                            app.scroll_up(10);
                        }
                        KeyCode::PageDown => {
                            app.scroll_down(10);
                        }
                        KeyCode::Home => {
                            // Jump to oldest message.
                            app.scroll_up(usize::MAX);
                        }
                        KeyCode::End => {
                            app.scroll_to_bottom();
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
