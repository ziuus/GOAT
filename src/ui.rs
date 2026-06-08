use crate::app::{App, AppStatus};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Number of log lines passed to ratatui for rendering.
/// We cap this so the widget doesn't slow down on very large logs.
const MAX_RENDER_LINES: usize = 300;

// ── Colour palette ─────────────────────────────────────────────────────────────

const COLOR_USER: Color = Color::Rgb(130, 200, 255); // soft blue
const COLOR_GOAT: Color = Color::Rgb(140, 230, 140); // soft green
const COLOR_TOOL: Color = Color::Rgb(180, 130, 255); // purple
const COLOR_AGENT: Color = Color::Rgb(100, 180, 255); // lighter blue
const COLOR_APPROVAL: Color = Color::Rgb(255, 200, 80); // amber
const COLOR_APPROVAL_DENIED: Color = Color::Rgb(255, 100, 100); // red
const COLOR_SECURITY: Color = Color::Rgb(255, 140, 180); // pink
const COLOR_SYSTEM: Color = Color::Rgb(140, 140, 160); // dim grey
const COLOR_ERROR: Color = Color::Rgb(255, 80, 80); // bright red
const COLOR_HELP: Color = Color::Rgb(160, 200, 200); // teal
const COLOR_STATUS: Color = Color::Rgb(200, 200, 100); // yellow-ish
const COLOR_DIM: Color = Color::Rgb(100, 100, 120);

pub fn render(f: &mut Frame, app: &App) {
    // ── Layout: 3 rows ────────────────────────────────────────────────────────
    //   [0] Header bar         — 1 line  (status, provider, session)
    //   [1] Chat / Log panel   — fill
    //   [2] Input composer     — 3 lines
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header bar
            Constraint::Min(5),    // chat / log panel
            Constraint::Length(3), // input composer
        ])
        .split(f.area());

    render_header(f, app, rows[0]);
    render_log(f, app, rows[1]);
    render_input(f, app, rows[2]);

    // Approval overlay on top of the log when active.
    if let Some(approval_lines) = app.pending_approval_lines() {
        render_approval_overlay(f, &approval_lines, rows[1]);
    }
}

// ── Header bar ────────────────────────────────────────────────────────────────

fn render_header(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (status_label, status_color) = match &app.status {
        AppStatus::Ready => ("● READY", Color::Rgb(80, 200, 80)),
        AppStatus::Thinking => ("◌ THINKING…", Color::Rgb(255, 200, 50)),
        AppStatus::ToolRunning(_) => ("⚙ RUNNING", Color::Rgb(100, 180, 255)),
        AppStatus::WaitingApproval(_) => ("⚠ APPROVAL", Color::Rgb(255, 160, 50)),
        AppStatus::Error(_) => ("✕ ERROR", Color::Rgb(255, 80, 80)),
    };

    let mcp_text = if app.mcp_server_count > 0 {
        format!(" │ MCP:{}", app.mcp_server_count)
    } else {
        String::new()
    };

    // Truncate session ID for display
    let session_short = if app.session_id.len() > 12 {
        &app.session_id[..12]
    } else {
        &app.session_id
    };

    let header_text = format!(
        " GOAT v0.1 │ {} │ Session:{}{} │ {}",
        app.provider_label, session_short, mcp_text, status_label
    );
    let _ = header_text; // retained for potential future use in non-colored fallback

    // Render the header content with status color for the status indicator.
    // Build it as styled spans for the status part.
    let spans = vec![
        Span::styled(
            format!(
                " GOAT v0.1 │ {} │ Session:{}{} │ ",
                app.provider_label, session_short, mcp_text
            ),
            Style::default()
                .fg(Color::Rgb(200, 220, 255))
                .bg(Color::Rgb(30, 40, 70)),
        ),
        Span::styled(
            status_label,
            Style::default()
                .fg(status_color)
                .bg(Color::Rgb(30, 40, 70))
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let header_line = Paragraph::new(Line::from(spans));
    f.render_widget(header_line, area);
}

// ── Chat / Log panel ──────────────────────────────────────────────────────────

fn render_log(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // Determine visible window based on scroll offset.
    // log_scroll = 0 → show the last N lines (newest at bottom)
    // log_scroll = N → scroll up N lines from bottom
    let total = app.logs.len();
    let visible_height = area.height.saturating_sub(2) as usize; // subtract border lines

    // Compute which slice of logs to show.
    let bottom = total.saturating_sub(app.log_scroll);
    let top = bottom.saturating_sub(visible_height.min(MAX_RENDER_LINES));
    let visible_slice = &app.logs[top..bottom];

    let logs_text: Vec<Line> = visible_slice
        .iter()
        .map(|line| color_log_line(line))
        .collect();

    // Build scroll indicator text if user has scrolled up.
    let scroll_hint = if app.log_scroll > 0 {
        format!(
            " Chat & Logs  [↑↓ scroll | End = bottom | {} lines above] ",
            app.log_scroll
        )
    } else {
        " Chat & Logs ".to_string()
    };

    let border_style = if app.has_pending_approval() {
        Style::default()
            .fg(Color::Rgb(255, 160, 50))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(60, 80, 120))
    };

    let log_block = Paragraph::new(logs_text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(scroll_hint)
            .title_style(Style::default().fg(Color::Rgb(120, 140, 180)))
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    f.render_widget(log_block, area);
}

/// Apply colour-coding to a single log line based on its prefix.
fn color_log_line(line: &str) -> Line<'static> {
    let (style, content) = if line.starts_with("[YOU]") {
        (
            Style::default().fg(COLOR_USER).add_modifier(Modifier::BOLD),
            line.to_owned(),
        )
    } else if line.starts_with("[GOAT]") {
        (Style::default().fg(COLOR_GOAT), line.to_owned())
    } else if line.starts_with("[TOOL]") {
        (Style::default().fg(COLOR_TOOL), line.to_owned())
    } else if line.starts_with("[AGENT]") {
        (Style::default().fg(COLOR_AGENT), line.to_owned())
    } else if line.starts_with("[APPROVAL] ✓") || line.starts_with("[APPROVAL] Auto-approved") {
        (
            Style::default().fg(COLOR_GOAT).add_modifier(Modifier::BOLD),
            line.to_owned(),
        )
    } else if line.starts_with("[APPROVAL] ✗") || line.starts_with("[APPROVAL] Auto-denied") {
        (
            Style::default()
                .fg(COLOR_APPROVAL_DENIED)
                .add_modifier(Modifier::BOLD),
            line.to_owned(),
        )
    } else if line.starts_with("[APPROVAL]") {
        (
            Style::default()
                .fg(COLOR_APPROVAL)
                .add_modifier(Modifier::BOLD),
            line.to_owned(),
        )
    } else if line.starts_with("[SECURITY]") {
        (Style::default().fg(COLOR_SECURITY), line.to_owned())
    } else if line.starts_with("[SYSTEM]") {
        (
            Style::default()
                .fg(COLOR_SYSTEM)
                .add_modifier(Modifier::DIM),
            line.to_owned(),
        )
    } else if line.starts_with("[ERROR]") {
        (
            Style::default()
                .fg(COLOR_ERROR)
                .add_modifier(Modifier::BOLD),
            line.to_owned(),
        )
    } else if line.starts_with("[WARN]") {
        (Style::default().fg(COLOR_APPROVAL), line.to_owned())
    } else if line.starts_with("[HELP]") {
        (Style::default().fg(COLOR_HELP), line.to_owned())
    } else if line.starts_with("[STATUS]") || line.starts_with("[SESSION]") {
        (Style::default().fg(COLOR_STATUS), line.to_owned())
    } else if line.starts_with("[BRAIN]")
        || line.starts_with("[SWARM]")
        || line.starts_with("[MCP]")
    {
        (Style::default().fg(COLOR_AGENT), line.to_owned())
    } else if line.starts_with("[TOOLS]") {
        (Style::default().fg(COLOR_TOOL), line.to_owned())
    } else {
        (Style::default().fg(COLOR_DIM), line.to_owned())
    };

    Line::from(Span::styled(content, style))
}

// ── Input composer ────────────────────────────────────────────────────────────

fn render_input(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.has_pending_approval() {
        // While approval overlay is open, replace the input with a clear hint.
        let hint = Paragraph::new(
            "  ⚠  Approval required — press  [y] approve  [n] deny  [a] always allow  [d] always deny  [Esc] deny",
        )
        .style(
            Style::default()
                .fg(Color::Rgb(255, 200, 80))
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title(" Action Required ")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 160, 50))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::Rgb(255, 120, 30))
                        .add_modifier(Modifier::BOLD),
                ),
        );
        f.render_widget(hint, area);
        return;
    }

    // Normal input composer.
    let display_text = if app.input.is_empty() {
        // Placeholder text when nothing typed.
        Span::styled(
            "  Ask GOAT anything… (Enter to send · Ctrl+C to quit · /help for commands)",
            Style::default()
                .fg(Color::Rgb(80, 90, 110))
                .add_modifier(Modifier::ITALIC),
        )
    } else {
        Span::styled(
            format!("  {}", app.input),
            Style::default().fg(Color::Rgb(220, 230, 255)),
        )
    };

    let border_style = Style::default().fg(Color::Rgb(70, 110, 180));

    let input_block = Paragraph::new(Line::from(display_text)).block(
        Block::default()
            .title(" Message ")
            .title_style(Style::default().fg(Color::Rgb(100, 140, 220)))
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(input_block, area);

    // Show cursor position in the input box.
    #[allow(clippy::cast_possible_truncation)]
    let cursor_x = area.x + app.input.chars().count() as u16 + 3; // 3 = border + 2 spaces
    let cursor_y = area.y + 1;
    // Clamp cursor within the input area.
    if cursor_x < area.x + area.width.saturating_sub(1) {
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

// ── Approval overlay ──────────────────────────────────────────────────────────

fn render_approval_overlay(f: &mut Frame, lines: &[String], area: ratatui::layout::Rect) {
    let overlay_height = (lines.len() as u16 + 4).min(area.height.saturating_sub(4));
    let overlay_width = 78u16.min(area.width.saturating_sub(4));

    if overlay_width < 20 || overlay_height < 4 {
        return; // terminal too small
    }

    let x = area.x + (area.width.saturating_sub(overlay_width)) / 2;
    let y = area.y + (area.height.saturating_sub(overlay_height)) / 2;

    let overlay_area = ratatui::layout::Rect {
        x,
        y,
        width: overlay_width,
        height: overlay_height,
    };

    f.render_widget(Clear, overlay_area);

    let content: Vec<Line> = lines
        .iter()
        .map(|l| {
            let style = if l.contains("CRITICAL") {
                Style::default()
                    .fg(Color::Rgb(255, 60, 60))
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("HIGH") {
                Style::default()
                    .fg(Color::Rgb(255, 180, 50))
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("[y]") || l.contains("approve") {
                Style::default().fg(Color::Rgb(100, 220, 100))
            } else if l.contains("[n]") || l.contains("deny") || l.contains("[d]") {
                Style::default().fg(Color::Rgb(255, 100, 100))
            } else {
                Style::default().fg(Color::Rgb(220, 220, 240))
            };
            Line::from(Span::styled(l.clone(), style))
        })
        .collect();

    let overlay_block = Paragraph::new(content)
        .block(
            Block::default()
                .title(" ⚠  APPROVAL REQUIRED — Review before executing ⚠ ")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 180, 50))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::Rgb(255, 120, 30))
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(overlay_block, overlay_area);
}
