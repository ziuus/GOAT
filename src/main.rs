mod app;
mod mcp;
mod brain;
mod llm;
mod config;
mod swarm;
mod ui;
mod tools;

use app::{App, InputMode};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};
use std::time::Duration;
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
        println!("{:?}", err);
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
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            info!("quit requested");
                            app.shutdown_mcp_servers().await;
                            app.quit();
                        }
                        KeyCode::Char('c') => {
                            app.push_log("[MCP] Starting configured MCP servers...");
                            info!("starting configured MCP servers");
                            terminal.draw(|f| ui::render(f, app))?;
                            app.start_configured_mcp_servers().await;
                            info!("configured MCP startup finished");
                        }
                        KeyCode::Char('l') => {
                            info!("learn about me indexing requested");
                            app.learn_about_me();
                        }
                        KeyCode::Char('r') => {
                            info!(input_length = app.input.len(), "swarm route requested");
                            app.route_current_input();
                        }
                        KeyCode::Char('m') => {
                            info!("MCP status requested");
                            app.show_mcp_status();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let msg = app.input.clone();
                            if !msg.is_empty() {
                                app.input.clear();
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
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
        if !app.running {
            break;
        }
    }
    Ok(())
}
