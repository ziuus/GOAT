use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{App, InputMode};

const VISIBLE_LOG_LINES: usize = 200;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(3),
            Constraint::Percentage(80)
        ].as_ref())
        .split(f.area());

    let task_block = Paragraph::new(app.current_task.as_str())
        .block(Block::default().title(" Active Mission ").borders(Borders::ALL).style(Style::default().fg(Color::Cyan)));
    f.render_widget(task_block, chunks[0]);

    let input_title = match app.input_mode {
        InputMode::Normal => " Input (i edit | c MCP | l learn | r route | m status | q quit) ",
        InputMode::Editing => " Input (Press 'Esc' to stop, 'Enter' to send) ",
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let input_block = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(Block::default().title(input_title).borders(Borders::ALL));
    f.render_widget(input_block, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor_position((
                chunks[1].x + app.input.chars().count() as u16 + 1,
                chunks[1].y + 1,
            ));
        }
    }

    let log_start = app.logs.len().saturating_sub(VISIBLE_LOG_LINES);
    let logs_text = app.logs[log_start..].join("\n");
    let logs_block = Paragraph::new(logs_text)
        .block(Block::default().title(" Execution Logs ").borders(Borders::ALL));
    f.render_widget(logs_block, chunks[2]);
}
