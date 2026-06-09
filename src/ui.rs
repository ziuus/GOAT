use crate::app::{App, AppStatus};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Current GOAT version shown in the header.
pub const GOAT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Number of log lines passed to ratatui for rendering.
/// We cap this so the widget doesn't slow down on very large logs.
const MAX_RENDER_LINES: usize = 300;

// ── Colour palette (GOAT Design System v1) ─────────────────────────────────
//
// Palette philosophy:
//   - Deep dark navy base (not pure black)
//   - Soft, desaturated accents for readability
//   - Bold/bright only for user input, errors, approvals
//   - Covers all [TAG] prefixes used in the codebase

const COLOR_USER: Color = Color::Rgb(130, 200, 255); // soft sky blue  — [YOU]
const COLOR_GOAT: Color = Color::Rgb(100, 220, 160); // mint green     — [GOAT]
const COLOR_TOOL: Color = Color::Rgb(190, 130, 255); // purple         — [TOOL], [TOOLS]
const COLOR_AGENT: Color = Color::Rgb(80, 150, 255); // darker blue    — [AGENT], [SWARM], [BRAIN], [MCP]
const COLOR_APPROVAL: Color = Color::Rgb(255, 190, 60); // amber       — [APPROVAL] pending, [WARN]
const COLOR_APPROVAL_OK: Color = Color::Rgb(80, 220, 120); // green    — [APPROVAL]✓
const COLOR_APPROVAL_DENIED: Color = Color::Rgb(240, 80, 80); // red   — [APPROVAL]✗
const COLOR_SECURITY: Color = Color::Rgb(255, 130, 175); // pink       — [SECURITY]
const COLOR_SYSTEM: Color = Color::Rgb(120, 125, 150); // dim grey     — [SYSTEM]
const COLOR_ERROR: Color = Color::Rgb(240, 70, 70); // bright red      — [ERROR]
const COLOR_HELP: Color = Color::Rgb(130, 205, 205); // teal           — [HELP]
const COLOR_STATUS: Color = Color::Rgb(200, 200, 100); // yellow-green — [STATUS], [SESSION]
const COLOR_MEMORY: Color = Color::Rgb(130, 200, 220); // cyan         — [MEMORY]
const COLOR_SKILL: Color = Color::Rgb(160, 230, 170); // light green   — [SKILL], [SKILLS]
const COLOR_PROJECT: Color = Color::Rgb(160, 200, 255); // light blue  — [PROJECT]
const COLOR_REPOMAP: Color = Color::Rgb(140, 190, 240); // pale blue   — [REPO-MAP]
const COLOR_DEV: Color = Color::Rgb(220, 160, 255); // lavender        — [DEV]
const COLOR_PATCH: Color = Color::Rgb(255, 210, 100); // golden        — [PATCH]
const COLOR_RESEARCH: Color = Color::Rgb(100, 220, 210); // teal-green — [RESEARCH]
const COLOR_DIFF_ADD: Color = Color::Rgb(80, 220, 100); // green       — diff + lines
const COLOR_DIFF_REMOVE: Color = Color::Rgb(240, 80, 80); // red       — diff - lines
const COLOR_DIFF_HUNK: Color = Color::Rgb(100, 160, 255); // blue      — diff @@ lines
const COLOR_DIM: Color = Color::Rgb(95, 100, 125); // fallback dim grey

/// Background for the header bar.
const COLOR_HEADER_BG: Color = Color::Rgb(22, 26, 48);
/// Main text on header.
const COLOR_HEADER_FG: Color = Color::Rgb(180, 195, 235);
/// Accent text on header (e.g. version, skill label).
const COLOR_HEADER_ACCENT: Color = Color::Rgb(100, 220, 160);
/// Border color — normal state.
const COLOR_BORDER: Color = Color::Rgb(55, 70, 110);
/// Border color — approval pending.
const COLOR_BORDER_APPROVAL: Color = Color::Rgb(255, 160, 50);
/// Input border — normal.
const COLOR_INPUT_BORDER: Color = Color::Rgb(65, 100, 175);

// ── Main render entry ─────────────────────────────────────────────────────────

pub fn render(f: &mut Frame, app: &App) {
    // ── Layout: header (1 line) + log (fill) + input (3 lines) ───────────────
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

    // Approval overlay floats on top of the log panel when active.
    if let Some(approval_lines) = app.pending_approval_lines() {
        render_approval_overlay(f, &approval_lines, rows[1]);
    }
}

// ── Header bar ────────────────────────────────────────────────────────────────

fn render_header(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (status_label, status_color) = match &app.status {
        AppStatus::Ready => ("● READY", Color::Rgb(80, 210, 80)),
        AppStatus::Thinking => ("◌ THINKING…", Color::Rgb(255, 200, 50)),
        AppStatus::ToolRunning(t) => {
            // Truncate long tool names in header.
            let _ = t; // used via dynamic dispatch below
            ("⚙ RUNNING", Color::Rgb(100, 180, 255))
        }
        AppStatus::WaitingApproval(_) => ("⚠ APPROVAL", Color::Rgb(255, 160, 50)),
        AppStatus::Error(_) => ("✕ ERROR", Color::Rgb(240, 70, 70)),
    };

    // Session display: prefer title over UUID snippet.
    let session_display = if app.session_id.len() > 12 {
        format!("s:{}", &app.session_id[..8])
    } else {
        format!("s:{}", &app.session_id)
    };

    // Active skill tag — shown only when a skill is active.
    let skill_tag = match &app.active_skill {
        Some(s) => {
            let short = if s.len() > 12 { &s[..12] } else { s.as_str() };
            format!(" │ 🎯 {}", short)
        }
        None => String::new(),
    };

    // MCP server count (show even when 0 to confirm daemon state).
    let mcp_tag = if app.mcp_server_count > 0 {
        format!(" │ MCP:{}", app.mcp_server_count)
    } else {
        String::new()
    };

    // Build header spans with distinct styling.
    let spans = vec![
        // Brand mark + version
        Span::styled(
            " 🐐 GOAT ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .bg(COLOR_HEADER_BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("v{}", GOAT_VERSION),
            Style::default()
                .fg(Color::Rgb(100, 115, 160))
                .bg(COLOR_HEADER_BG),
        ),
        Span::styled(
            format!(
                " │ {} │ {}{}{} │ ",
                app.active_profile, app.provider_label, mcp_tag, skill_tag
            ),
            Style::default().fg(COLOR_HEADER_FG).bg(COLOR_HEADER_BG),
        ),
        Span::styled(
            status_label,
            Style::default()
                .fg(status_color)
                .bg(COLOR_HEADER_BG)
                .add_modifier(Modifier::BOLD),
        ),
        // Fill the rest of the line with the header background.
        Span::styled(
            "                                                                                ",
            Style::default().bg(COLOR_HEADER_BG),
        ),
    ];

    let header_line = Paragraph::new(Line::from(spans));
    f.render_widget(header_line, area);
}

// ── Chat / Log panel ──────────────────────────────────────────────────────────

fn render_log(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
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

    // Panel title: show scroll state when user has scrolled up.
    let scroll_hint = if app.log_scroll > 0 {
        format!(
            " Chat & Logs  [↑↓ PgUp/PgDn scroll │ End=bottom │ {} above] ",
            app.log_scroll
        )
    } else {
        " Chat & Logs  [↑↓ to scroll │ /help for commands] ".to_string()
    };

    // Log panel border pulses amber when approval is pending.
    let border_style = if app.has_pending_approval() {
        Style::default()
            .fg(COLOR_BORDER_APPROVAL)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_BORDER)
    };

    let log_block = Paragraph::new(logs_text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(scroll_hint)
            .title_style(Style::default().fg(Color::Rgb(110, 130, 175)))
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    f.render_widget(log_block, area);
}

/// Colour-code a single log line based on its `[TAG]` prefix.
///
/// Extended to cover all tags used across the codebase, including diff lines.
fn color_log_line(line: &str) -> Line<'static> {
    let owned = line.to_owned();

    // Diff line coloring: `+` added, `-` removed, `@@` hunk headers.
    // These appear inside [PATCH] sections as free-standing lines starting with +/-/@@.
    if line.starts_with("+ ") || line == "+" {
        return Line::from(Span::styled(owned, Style::default().fg(COLOR_DIFF_ADD)));
    }
    if line.starts_with("- ") || line == "-" {
        return Line::from(Span::styled(owned, Style::default().fg(COLOR_DIFF_REMOVE)));
    }
    if line.starts_with("@@ ") {
        return Line::from(Span::styled(owned, Style::default().fg(COLOR_DIFF_HUNK)));
    }

    let style = if line.starts_with("[YOU]") {
        Style::default().fg(COLOR_USER).add_modifier(Modifier::BOLD)
    } else if line.starts_with("[GOAT]") {
        Style::default().fg(COLOR_GOAT)
    } else if line.starts_with("[TOOL]") || line.starts_with("[TOOLS]") {
        Style::default().fg(COLOR_TOOL)
    } else if line.starts_with("[AGENT]") {
        Style::default().fg(COLOR_AGENT)
    } else if line.starts_with("[APPROVAL] ✓") || line.starts_with("[APPROVAL] Auto-approved") {
        Style::default()
            .fg(COLOR_APPROVAL_OK)
            .add_modifier(Modifier::BOLD)
    } else if line.starts_with("[APPROVAL] ✗") || line.starts_with("[APPROVAL] Auto-denied") {
        Style::default()
            .fg(COLOR_APPROVAL_DENIED)
            .add_modifier(Modifier::BOLD)
    } else if line.starts_with("[APPROVAL]") {
        Style::default()
            .fg(COLOR_APPROVAL)
            .add_modifier(Modifier::BOLD)
    } else if line.starts_with("[SECURITY]") {
        Style::default().fg(COLOR_SECURITY)
    } else if line.starts_with("[SYSTEM]") {
        Style::default()
            .fg(COLOR_SYSTEM)
            .add_modifier(Modifier::DIM)
    } else if line.starts_with("[ERROR]") {
        Style::default()
            .fg(COLOR_ERROR)
            .add_modifier(Modifier::BOLD)
    } else if line.starts_with("[WARN]") {
        Style::default().fg(COLOR_APPROVAL)
    } else if line.starts_with("[HELP]") {
        Style::default().fg(COLOR_HELP)
    } else if line.starts_with("[STATUS]") || line.starts_with("[SESSION]") {
        Style::default().fg(COLOR_STATUS)
    } else if line.starts_with("[BRAIN]")
        || line.starts_with("[SWARM]")
        || line.starts_with("[MCP]")
    {
        Style::default().fg(COLOR_AGENT)
    } else if line.starts_with("[MEMORY]") {
        Style::default().fg(COLOR_MEMORY)
    } else if line.starts_with("[SKILL]") || line.starts_with("[SKILLS]") {
        Style::default().fg(COLOR_SKILL)
    } else if line.starts_with("[PROJECT]") {
        Style::default().fg(COLOR_PROJECT)
    } else if line.starts_with("[REPO-MAP]") || line.starts_with("[REPOMAP]") {
        Style::default().fg(COLOR_REPOMAP)
    } else if line.starts_with("[DEV]") {
        Style::default().fg(COLOR_DEV)
    } else if line.starts_with("[PATCH]") {
        Style::default().fg(COLOR_PATCH)
    } else if line.starts_with("[RESEARCH]") {
        Style::default().fg(COLOR_RESEARCH)
    } else if line.starts_with("[UI]") {
        Style::default().fg(COLOR_HELP)
    } else if line.starts_with("[RECALL]") || line.starts_with("[BRAIN-RECALL]") {
        Style::default().fg(COLOR_MEMORY)
    } else {
        Style::default().fg(COLOR_DIM)
    };

    Line::from(Span::styled(owned, style))
}

// ── Input composer ────────────────────────────────────────────────────────────

fn render_input(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.has_pending_approval() {
        // During approval, replace input with a prominent key-hint bar.
        let hint = Paragraph::new(
            "  ⚠  APPROVAL — [y] approve  [n] deny  [a] always allow  [d] always deny  [Esc] deny & cancel",
        )
        .style(
            Style::default()
                .fg(Color::Rgb(255, 200, 80))
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title(" ⚠ Action Required — See overlay above ")
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

    // Determine a contextual placeholder hint based on current state.
    let placeholder = match &app.status {
        AppStatus::Thinking => "  GOAT is thinking…",
        AppStatus::ToolRunning(t) => {
            let _ = t;
            "  Running tool…"
        }
        AppStatus::Error(_) => "  Error occurred — type a new message or /status to check",
        _ => "  Ask GOAT anything… (/help · ↑ history · Ctrl+C quit)",
    };

    let display_text = if app.input.is_empty() {
        Span::styled(
            placeholder,
            Style::default()
                .fg(Color::Rgb(75, 85, 115))
                .add_modifier(Modifier::ITALIC),
        )
    } else {
        Span::styled(
            format!("  {}", app.input),
            Style::default().fg(Color::Rgb(215, 225, 255)),
        )
    };

    // Input border brightens during active typing vs idle.
    let border_style = if app.input.is_empty() {
        Style::default().fg(COLOR_INPUT_BORDER)
    } else {
        Style::default()
            .fg(Color::Rgb(100, 150, 230))
            .add_modifier(Modifier::BOLD)
    };

    let input_block = Paragraph::new(Line::from(display_text)).block(
        Block::default()
            .title(" Message ")
            .title_style(Style::default().fg(Color::Rgb(95, 135, 215)))
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(input_block, area);

    // Position cursor in the input box.
    #[allow(clippy::cast_possible_truncation)]
    let cursor_x = area.x + app.input.chars().count() as u16 + 3; // border + 2 spaces
    let cursor_y = area.y + 1;
    if cursor_x < area.x + area.width.saturating_sub(1) {
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

// ── Approval overlay ──────────────────────────────────────────────────────────

fn render_approval_overlay(f: &mut Frame, lines: &[String], area: ratatui::layout::Rect) {
    // Size the overlay to content, capped at 90% of available space.
    let content_lines = lines.len() as u16;
    let overlay_height = (content_lines + 4).min(area.height.saturating_sub(4));
    let overlay_width = 86u16.min(area.width.saturating_sub(4));

    if overlay_width < 20 || overlay_height < 4 {
        return; // Terminal too small to show overlay.
    }

    // Center the overlay in the log panel.
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
            // Style lines based on content.
            let style = if l.contains("CRITICAL") {
                Style::default()
                    .fg(Color::Rgb(255, 50, 50))
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("HIGH") {
                Style::default()
                    .fg(Color::Rgb(255, 170, 40))
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("MEDIUM") {
                Style::default().fg(Color::Rgb(100, 160, 255))
            } else if l.contains("LOW") {
                Style::default().fg(Color::Rgb(80, 210, 80))
            } else if l.starts_with("+ ") || l == "+" {
                // Diff added lines inside approval overlay.
                Style::default().fg(COLOR_DIFF_ADD)
            } else if l.starts_with("- ") || l == "-" {
                // Diff removed lines inside approval overlay.
                Style::default().fg(COLOR_DIFF_REMOVE)
            } else if l.starts_with("@@ ") {
                Style::default().fg(COLOR_DIFF_HUNK)
            } else if l.contains("[y]") || l.contains("approve") {
                Style::default()
                    .fg(Color::Rgb(80, 220, 100))
                    .add_modifier(Modifier::BOLD)
            } else if l.contains("[n]") || l.contains("deny") || l.contains("[d]") {
                Style::default().fg(Color::Rgb(240, 90, 90))
            } else if l.contains("SECRET") || l.contains("⚠") {
                Style::default()
                    .fg(Color::Rgb(255, 130, 175))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(210, 220, 245))
            };
            Line::from(Span::styled(l.clone(), style))
        })
        .collect();

    // Choose border color based on content — CRITICAL = red, else amber.
    let has_critical = lines.iter().any(|l| l.contains("CRITICAL"));
    let border_color = if has_critical {
        Color::Rgb(240, 60, 60)
    } else {
        Color::Rgb(255, 130, 35)
    };

    let overlay_block = Paragraph::new(content)
        .block(
            Block::default()
                .title(" ⚠  APPROVAL REQUIRED — Review before executing ")
                .title_style(
                    Style::default()
                        .fg(if has_critical {
                            Color::Rgb(255, 80, 80)
                        } else {
                            Color::Rgb(255, 190, 55)
                        })
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(overlay_block, overlay_area);
}
