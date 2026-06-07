mod app;
mod mcp;
mod brain;
mod llm;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
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
                            app.quit();
                        }
                        KeyCode::Char('c') => {
                            app.logs.push("[MCP] Spawning 'uvx mcp-server-sqlite --db goat_brain.db'...".to_string());
                            terminal.draw(|f| ui::render(f, app))?;
                            
                            match crate::mcp::McpClient::spawn("uvx", &["mcp-server-sqlite", "--db", "goat_brain.db"]).await {
                                Ok(mut client) => {
                                    match client.initialize().await {
                                        Ok(init_res) => {
                                            app.logs.push(format!("[MCP] Initialized: {:?}", init_res));
                                            
                                            match client.list_tools().await {
                                                Ok(tools) => {
                                                    app.logs.push(format!("[MCP] Tools: {:?}", tools));
                                                }
                                                Err(e) => {
                                                    app.logs.push(format!("[MCP ERROR] List tools failed: {}", e));
                                                }
                                            }
                                            
                                            app.mcp_client = Some(client);
                                        }
                                        Err(e) => {
                                            app.logs.push(format!("[MCP ERROR] Initialize failed: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    app.logs.push(format!("[MCP ERROR] Spawn failed: {}", e));
                                }
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let msg = app.input.clone();
                            if !msg.is_empty() {
                                app.logs.push(format!("[USER] {}", msg));
                                app.input.clear();
                                
                                let messages = vec![
                                    llm::Message {
                                        role: "user".to_string(),
                                        content: msg,
                                    }
                                ];
                                
                                terminal.draw(|f| ui::render(f, app))?; // update UI with loading state

                                match app.llm_router.completion("openai", "gpt-4o-mini", messages).await {
                                    Ok(response) => {
                                        app.logs.push(format!("[LLM] {}", response));
                                        if let Some(ref brain) = app.brain {
                                            let _ = brain.log_interaction("assistant", &response);
                                            app.logs.push("[SYSTEM] Saved to Brain.".to_string());
                                        }
                                    }
                                    Err(e) => {
                                        app.logs.push(format!("[ERROR] LLM Failed: {}", e));
                                    }
                                }
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
