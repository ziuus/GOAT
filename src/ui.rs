use crate::app::{App, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

const VISIBLE_LOG_LINES: usize = 200;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(3),
            Constraint::Percentage(80),
        ])
        .split(f.area());

    // ── Active Mission panel ─────────────────────────────────────────────────
    let task_style = if app.has_pending_approval() {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let task_block = Paragraph::new(app.current_task.as_str()).block(
        Block::default()
            .title(" Active Mission ")
            .borders(Borders::ALL)
            .style(task_style),
    );
    f.render_widget(task_block, chunks[0]);

    // ── Input panel ──────────────────────────────────────────────────────────
    let input_title = match app.input_mode {
        InputMode::Normal => " Input (i edit | c MCP | l learn | r route | m status | q quit) ",
        InputMode::Editing => " Input (Press 'Esc' to stop, 'Enter' to send) ",
    };

    let input_style = if app.has_pending_approval() {
        // During approval, show special hint and dim the input box.
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        }
    };

    let input_content = if app.has_pending_approval() {
        "⚠  APPROVAL REQUIRED — press [y] approve  [n] deny  [a] always allow  [d] always deny"
    } else {
        app.input.as_str()
    };

    let input_block = Paragraph::new(input_content)
        .style(input_style)
        .block(Block::default().title(input_title).borders(Borders::ALL));
    f.render_widget(input_block, chunks[1]);

    // Show cursor in editing mode (only when NOT in approval mode).
    if !app.has_pending_approval() {
        if let InputMode::Editing = app.input_mode {
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor_position((
                chunks[1].x + app.input.chars().count() as u16 + 1,
                chunks[1].y + 1,
            ));
        }
    }

    // ── Execution logs panel ─────────────────────────────────────────────────
    let log_start = app.logs.len().saturating_sub(VISIBLE_LOG_LINES);
    let logs_text: Vec<Line> = app.logs[log_start..]
        .iter()
        .map(|line| {
            // Colour-code log lines by prefix for readability.
            let style = if line.starts_with("[ERROR]") || line.starts_with("[APPROVAL] Denied") {
                Style::default().fg(Color::Red)
            } else if line.starts_with("[APPROVAL]") {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if line.starts_with("[SECURITY]") {
                Style::default().fg(Color::Magenta)
            } else if line.starts_with("[LLM]") {
                Style::default().fg(Color::Green)
            } else if line.starts_with("[TOOL]") {
                Style::default().fg(Color::Cyan)
            } else if line.starts_with("[AGENT]") {
                Style::default().fg(Color::Blue)
            } else if line.starts_with("[SYSTEM]") {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM)
            } else {
                Style::default()
            };
            Line::from(Span::styled(line.clone(), style))
        })
        .collect();

    let logs_block = Paragraph::new(logs_text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(" Execution Logs ")
            .borders(Borders::ALL),
    );
    f.render_widget(logs_block, chunks[2]);

    // ── Approval overlay ─────────────────────────────────────────────────────
    // When a dangerous tool is pending approval, draw a centred overlay box
    // over the bottom portion of the log panel so the user cannot miss it.
    if let Some(approval_lines) = app.pending_approval_lines() {
        render_approval_overlay(f, &approval_lines, chunks[2]);
    }
}

/// Render a centred approval overlay box inside `area`.
fn render_approval_overlay(f: &mut Frame, lines: &[String], area: ratatui::layout::Rect) {
    // Calculate overlay size: as wide as possible (max 70 cols), 2 + lines tall.
    let overlay_height = (lines.len() as u16 + 2).min(area.height.saturating_sub(2));
    let overlay_width = 72u16.min(area.width.saturating_sub(4));

    if overlay_width == 0 || overlay_height == 0 {
        return; // Terminal too small to render overlay.
    }

    let x = area.x + (area.width.saturating_sub(overlay_width)) / 2;
    let y = area.y + (area.height.saturating_sub(overlay_height)) / 2;

    let overlay_area = ratatui::layout::Rect {
        x,
        y,
        width: overlay_width,
        height: overlay_height,
    };

    // Clear the background behind the overlay.
    f.render_widget(Clear, overlay_area);

    let content: Vec<Line> = lines
        .iter()
        .map(|l| {
            let style = if l.contains("CRITICAL") {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if l.contains("HIGH") {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("[y]") {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(l.clone(), style))
        })
        .collect();

    let overlay_block = Paragraph::new(content)
        .block(
            Block::default()
                .title(" ⚠  APPROVAL REQUIRED ⚠ ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(overlay_block, overlay_area);
}
