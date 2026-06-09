use crate::app::{ActiveView, App, AppStatus, LayoutMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Current GOAT version shown in the header.
pub const GOAT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Number of log lines passed to ratatui for rendering.
/// We cap this so the widget doesn't slow down on very large logs.
const MAX_RENDER_LINES: usize = 300;

// ── Colour palette (GOAT Design System v1) ──────────────────────────────────
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
const COLOR_HEADER_BG: Color = Color::Rgb(18, 22, 42);
/// Main text on header.
const COLOR_HEADER_FG: Color = Color::Rgb(170, 185, 225);
/// Accent text on header (e.g. version, skill label).
const COLOR_HEADER_ACCENT: Color = Color::Rgb(80, 210, 155);
/// Border color — normal state.
const COLOR_BORDER: Color = Color::Rgb(50, 65, 105);
/// Border color — active/focused panel.
const COLOR_BORDER_FOCUS: Color = Color::Rgb(70, 110, 190);
/// Border color — approval pending.
const COLOR_BORDER_APPROVAL: Color = Color::Rgb(255, 160, 50);
/// Input border — normal.
const COLOR_INPUT_BORDER: Color = Color::Rgb(60, 95, 165);
/// Input border — active typing.
const COLOR_INPUT_ACTIVE: Color = Color::Rgb(90, 140, 220);

// ── Main render entry ─────────────────────────────────────────────────────────

pub fn render(f: &mut Frame, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header bar
            Constraint::Min(5),    // workspace panel
            Constraint::Length(3), // input composer
        ])
        .split(f.area());

    render_header(f, app, rows[0]);

    match app.layout_mode {
        LayoutMode::Focus => render_focus_layout(f, app, rows[1]),
        LayoutMode::Dashboard => render_dashboard_layout(f, app, rows[1]),
        LayoutMode::Compact => render_compact_layout(f, app, rows[1]),
    }

    render_input(f, app, rows[2]);

    // Approval overlay floats on top of the workspace when active.
    if let Some(approval_lines) = app.pending_approval_lines() {
        render_approval_overlay(f, &approval_lines, rows[1]);
    }

    // Slash command suggestion popup — appears just above the input composer.
    if !app.cmd_suggestions.is_empty() {
        render_suggestion_popup(f, app, rows[2]);
    }

    // Command palette modal — centered overlay.
    if app.active_view == ActiveView::CommandPalette {
        render_palette_modal(f, app, rows[1]);
    }

    // Agent selector modal — centered overlay.
    if app.active_view == ActiveView::AgentSelector {
        render_agent_selector_modal(f, app, rows[1]);
    }
}

// ── Layout modes ─────────────────────────────────────────────────────────────

/// Focus layout — clean, centered, OpenCode-style.
/// No always-visible sidebar/context panel. Just the chat or active view.
fn render_focus_layout(f: &mut Frame, app: &App, area: Rect) {
    // In focus mode with chat view and no messages: show premium empty state.
    let is_empty_chat = app.active_view == ActiveView::Chat
        && app
            .logs
            .iter()
            .filter(|l| l.starts_with("[YOU]") || l.starts_with("[GOAT]"))
            .count()
            == 0;

    if is_empty_chat {
        render_empty_state(f, app, area);
        return;
    }

    match app.active_view {
        ActiveView::Chat => {
            // In focus mode, give chat a clean max-width centered column
            let content_area = center_area(area, 120, 100);
            render_log(f, app, content_area)
        }
        ActiveView::Logs => render_logs_view(f, app, area),
        ActiveView::CommandPalette | ActiveView::AgentSelector => {
            // Modals rendered separately after main layout
            render_log(f, app, area)
        }
        _ => render_view(f, app, area),
    }
}

/// Dashboard layout — 3-pane: sidebar + center + context.
/// Respects sidebar_visible and context_visible toggles.
fn render_dashboard_layout(f: &mut Frame, app: &App, area: Rect) {
    // Narrow terminal fallback — behave like compact.
    if area.width < 90 {
        render_compact_layout(f, app, area);
        return;
    }

    let show_sidebar = app.sidebar_visible;
    let show_context = app.context_visible;

    let constraints = match (show_sidebar, show_context) {
        (true, true) => vec![
            Constraint::Length(26),
            Constraint::Min(40),
            Constraint::Length(30),
        ],
        (true, false) => vec![Constraint::Length(26), Constraint::Min(40)],
        (false, true) => vec![Constraint::Min(40), Constraint::Length(30)],
        (false, false) => vec![Constraint::Min(40)],
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let (sidebar_col, center_col, context_col) = match (show_sidebar, show_context) {
        (true, true) => (Some(cols[0]), cols[1], Some(cols[2])),
        (true, false) => (Some(cols[0]), cols[1], None),
        (false, true) => (None, cols[0], Some(cols[1])),
        (false, false) => (None, cols[0], None),
    };

    if let Some(col) = sidebar_col {
        render_sidebar(f, app, col);
    }

    match app.active_view {
        ActiveView::Chat => render_log(f, app, center_col),
        ActiveView::Logs => render_logs_view(f, app, center_col),
        ActiveView::CommandPalette | ActiveView::AgentSelector => render_log(f, app, center_col),
        _ => render_view(f, app, center_col),
    }

    if let Some(col) = context_col {
        render_context_panel(f, app, col);
    }
}

/// Compact layout — chat + input only. Best for narrow terminals.
fn render_compact_layout(f: &mut Frame, app: &App, area: Rect) {
    match app.active_view {
        ActiveView::Chat => render_log(f, app, area),
        ActiveView::Logs => render_logs_view(f, app, area),
        _ => render_view(f, app, area),
    }
}

// ── Premium empty state ───────────────────────────────────────────────────────

/// Renders the premium centered empty/landing state.
/// Shown in Focus layout when no conversation has started.
fn render_empty_state(f: &mut Frame, app: &App, area: Rect) {
    let content_area = center_area(area, 72, 28);
    f.render_widget(Clear, content_area);

    // Build the empty state content.
    let profile = &app.active_profile;
    let provider = &app.provider_label;
    let layout = app.layout_mode.label();
    let skill_line = match &app.active_skill {
        Some(s) => format!("  🎯 Skill: {}  ", s),
        None => String::new(),
    };

    let mode_str = match app.workflow.mode {
        crate::task::AgentMode::Plan => "📝 PLAN",
        crate::task::AgentMode::Act => "⚡ ACT",
    };

    let project_str = String::new(); // Project name injected via context; not stored in App directly

    let tool_count = if app.config.tools.enabled {
        "Tools ✅"
    } else {
        "Tools ✗"
    };
    let subagent_count = app.subagent_manager.registry.list_all().len();
    let brain_str = if app.brain_disabled {
        "Brain ✗"
    } else {
        "Brain ✅"
    };

    // Status row info
    let status_line = format!(
        " {} │ {} │ {} {}{}│ {} │ {} subagents ",
        profile, provider, mode_str, project_str, skill_line, tool_count, subagent_count
    );

    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "    🐐 GOAT",
                Style::default()
                    .fg(COLOR_HEADER_ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  v{}", GOAT_VERSION),
                Style::default().fg(Color::Rgb(80, 95, 140)),
            ),
        ]),
        Line::from(Span::styled(
            "    General Omniscient Agentic Tool",
            Style::default().fg(Color::Rgb(100, 115, 160)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    ─────────────────────────────────────────────────",
            Style::default().fg(Color::Rgb(45, 55, 90)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    Ask anything… \"Fix broken tests\" or \"/help\"",
            Style::default()
                .fg(Color::Rgb(155, 170, 210))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    ─────────────────────────────────────────────────",
            Style::default().fg(Color::Rgb(45, 55, 90)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("   {}", status_line),
            Style::default().fg(Color::Rgb(90, 105, 150)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("    {} │ Layout: {}", brain_str, layout),
            Style::default().fg(Color::Rgb(70, 85, 125)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    ─────────────────────────────────────────────────",
            Style::default().fg(Color::Rgb(45, 55, 90)),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "    /  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("commands  ", Style::default().fg(Color::Rgb(110, 125, 170))),
            Span::styled(
                "Tab  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("complete  ", Style::default().fg(Color::Rgb(110, 125, 170))),
            Span::styled(
                "Ctrl+P  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("palette  ", Style::default().fg(Color::Rgb(110, 125, 170))),
            Span::styled(
                "Ctrl+D  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("dashboard", Style::default().fg(Color::Rgb(110, 125, 170))),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "    /status  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "system check  ",
                Style::default().fg(Color::Rgb(110, 125, 170)),
            ),
            Span::styled(
                "/help  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "all commands  ",
                Style::default().fg(Color::Rgb(110, 125, 170)),
            ),
            Span::styled(
                "/agents  ",
                Style::default()
                    .fg(Color::Rgb(100, 200, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "agent selector",
                Style::default().fg(Color::Rgb(110, 125, 170)),
            ),
        ]),
        Line::from(""),
        Line::from(""),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(40, 55, 95)))
        .title(Span::styled(
            " Welcome ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left),
        content_area,
    );
}

// ── Sidebar (Dashboard mode) ──────────────────────────────────────────────────

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let view_item = |v: ActiveView, icon: &str, label: &str| -> Line<'static> {
        if app.active_view == v {
            Line::from(Span::styled(
                format!(" ▶ {} {}", icon, label),
                Style::default()
                    .fg(COLOR_HEADER_ACCENT)
                    .add_modifier(Modifier::BOLD),
            ))
        } else {
            Line::from(Span::styled(
                format!("   {} {}", icon, label),
                Style::default().fg(Color::Rgb(120, 135, 175)),
            ))
        }
    };

    let lines = vec![
        Line::from(Span::styled(
            " VIEWS",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        view_item(ActiveView::Chat, "💬", "Chat"),
        view_item(ActiveView::Tasks, "📋", "Tasks"),
        view_item(ActiveView::RepoMap, "🗂", "Repo Map"),
        view_item(ActiveView::Patches, "🩹", "Patches"),
        view_item(ActiveView::Tools, "🔧", "Tools"),
        view_item(ActiveView::Memory, "🧠", "Memory"),
        view_item(ActiveView::Skills, "🎯", "Skills"),
        view_item(ActiveView::Subagents, "🤖", "Subagents"),
        view_item(ActiveView::ExternalAgents, "🔗", "External"),
        view_item(ActiveView::Logs, "📜", "Logs"),
        view_item(ActiveView::Help, "❓", "Help"),
        Line::from(""),
        Line::from(Span::styled(
            " INFO",
            Style::default()
                .fg(Color::Rgb(80, 95, 140))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                " 👤 {}",
                if app.session_id.len() > 8 {
                    &app.session_id[..8]
                } else {
                    &app.session_id
                }
            ),
            Style::default().fg(COLOR_DIM),
        )),
        Line::from(Span::styled(
            format!(" 🔀 {}", app.active_profile),
            Style::default().fg(COLOR_DIM),
        )),
        Line::from(Span::styled(
            format!(" 📐 {}", app.layout_mode.label()),
            Style::default().fg(COLOR_DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " KEYS",
            Style::default()
                .fg(Color::Rgb(80, 95, 140))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " Ctrl+1-9 views",
            Style::default().fg(Color::Rgb(65, 80, 120)),
        )),
        Line::from(Span::styled(
            " Ctrl+P  palette",
            Style::default().fg(Color::Rgb(65, 80, 120)),
        )),
        Line::from(Span::styled(
            " Ctrl+B  sidebar",
            Style::default().fg(Color::Rgb(65, 80, 120)),
        )),
        Line::from(Span::styled(
            " Ctrl+F  focus",
            Style::default().fg(Color::Rgb(65, 80, 120)),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_BORDER))
        .title(Span::styled(
            " 🐐 Menu ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(Paragraph::new(lines).block(block), area);
}

// ── Context panel (Dashboard mode) ───────────────────────────────────────────

fn render_context_panel(f: &mut Frame, app: &App, area: Rect) {
    let mode_label = match app.workflow.mode {
        crate::task::AgentMode::Plan => "📝 PLAN",
        crate::task::AgentMode::Act => "⚡ ACT",
    };

    let mut lines = vec![
        Line::from(Span::styled(
            " WORKFLOW",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(" Mode: {}", mode_label),
            Style::default().fg(Color::Rgb(140, 160, 210)),
        )),
        Line::from(""),
    ];

    if let Some(task) = &app.workflow.active_task {
        lines.push(Line::from(Span::styled(
            " ACTIVE TASK",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!(" Status: {:?}", task.status),
            Style::default().fg(Color::Rgb(140, 160, 210)),
        )));
        lines.push(Line::from(Span::styled(
            format!(" Patches: {}", app.workflow.patches.len()),
            Style::default().fg(COLOR_PATCH),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            " No Active Task",
            Style::default().fg(COLOR_DIM),
        )));
        lines.push(Line::from(Span::styled(
            " /code <task> to start",
            Style::default().fg(Color::Rgb(60, 75, 115)),
        )));
    }

    lines.push(Line::from(""));

    // Skill
    if let Some(skill) = &app.active_skill {
        lines.push(Line::from(Span::styled(
            " ACTIVE SKILL",
            Style::default()
                .fg(Color::Rgb(80, 95, 140))
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!(" 🎯 {}", skill),
            Style::default().fg(COLOR_SKILL),
        )));
        lines.push(Line::from(""));
    }

    // Provider info
    lines.push(Line::from(Span::styled(
        " PROVIDER",
        Style::default()
            .fg(Color::Rgb(80, 95, 140))
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!(" {}", app.provider_label),
        Style::default().fg(Color::Rgb(110, 130, 175)),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_BORDER))
        .title(Span::styled(
            " Context ",
            Style::default().fg(Color::Rgb(80, 100, 155)),
        ));

    f.render_widget(Paragraph::new(lines).block(block), area);
}

// ── View panel (non-Chat views) ───────────────────────────────────────────────

fn render_view(f: &mut Frame, app: &App, area: Rect) {
    let (title, lines) = build_view_content(app);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_BORDER_FOCUS))
        .title(Span::styled(
            title,
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn build_view_content(app: &App) -> (&'static str, Vec<Line<'static>>) {
    match app.active_view {
        ActiveView::Tasks => {
            let lines = if let Some(t) = &app.workflow.active_task {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        " ACTIVE TASK",
                        Style::default()
                            .fg(COLOR_HEADER_ACCENT)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        format!(" ID:      {}", t.id),
                        Style::default().fg(Color::Rgb(140, 160, 210)),
                    )),
                    Line::from(Span::styled(
                        format!(" Status:  {:?}", t.status),
                        Style::default().fg(COLOR_STATUS),
                    )),
                    Line::from(Span::styled(
                        format!(" Patches: {}", app.workflow.patches.len()),
                        Style::default().fg(COLOR_PATCH),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " Request:",
                        Style::default()
                            .fg(Color::Rgb(110, 130, 175))
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        format!(" {}", t.request),
                        Style::default().fg(Color::Rgb(180, 195, 235)),
                    )),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        " 📋  No Active Task",
                        Style::default()
                            .fg(COLOR_HEADER_ACCENT)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " No coding task is currently active.",
                        Style::default().fg(Color::Rgb(110, 125, 170)),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " To start one, run:",
                        Style::default().fg(COLOR_DIM),
                    )),
                    Line::from(Span::styled(
                        "   /code <description of task>",
                        Style::default().fg(COLOR_HELP).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        "   /plan <task>",
                        Style::default().fg(COLOR_HELP),
                    )),
                    Line::from(Span::styled(
                        "   /mode act   (switch to Act mode)",
                        Style::default().fg(COLOR_HELP),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " Proposed code edits will appear here as patches.",
                        Style::default().fg(COLOR_DIM),
                    )),
                ]
            };
            (" 📋 Tasks ", lines)
        }

        ActiveView::RepoMap => {
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🗂  Repository Map",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " No repo map is loaded in this view.",
                    Style::default().fg(Color::Rgb(110, 125, 170)),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Run in chat to load the map:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   /repo-map           show current map",
                    Style::default().fg(COLOR_HELP).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    "   /repo-map refresh   force rescan",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " The map shows your project's files, symbols,",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    " dependencies, and detected stack.",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Interactive tree view is planned for Phase 3.3.",
                    Style::default().fg(Color::Rgb(60, 75, 115)),
                )),
            ];
            (" 🗂 Repo Map ", lines)
        }

        ActiveView::Patches => {
            let lines = if app.workflow.patches.is_empty() {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        " 🩹  No Pending Patches",
                        Style::default()
                            .fg(COLOR_HEADER_ACCENT)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " Proposed code edits will appear here.",
                        Style::default().fg(Color::Rgb(110, 125, 170)),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " Patch controls:",
                        Style::default().fg(COLOR_DIM),
                    )),
                    Line::from(Span::styled(
                        "   /patch          show pending patches",
                        Style::default().fg(COLOR_HELP),
                    )),
                    Line::from(Span::styled(
                        "   /patch apply    apply the pending patch",
                        Style::default().fg(COLOR_HELP),
                    )),
                    Line::from(Span::styled(
                        "   /patch discard  discard and try again",
                        Style::default().fg(COLOR_HELP),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        " Use /code <task> to ask GOAT to make changes.",
                        Style::default().fg(COLOR_DIM),
                    )),
                ]
            } else {
                let mut l = vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        format!(" 🩹 {} Pending Patch(es)", app.workflow.patches.len()),
                        Style::default()
                            .fg(COLOR_PATCH)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                ];
                for p in &app.workflow.patches {
                    l.push(Line::from(Span::styled(
                        format!(" ─ ID: {}", p.id),
                        Style::default().fg(Color::Rgb(130, 150, 200)),
                    )));
                    l.push(Line::from(Span::styled(
                        format!("   Status: {:?}", p.status),
                        Style::default().fg(COLOR_STATUS),
                    )));
                    l.push(Line::from(""));
                }
                l.push(Line::from(Span::styled(
                    "   /patch apply    to apply",
                    Style::default().fg(COLOR_HELP),
                )));
                l.push(Line::from(Span::styled(
                    "   /patch discard  to discard",
                    Style::default().fg(COLOR_HELP),
                )));
                l
            };
            (" 🩹 Patches ", lines)
        }

        ActiveView::Tools => {
            let enabled = app.config.tools.enabled;
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🔧  Tool Registry",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!(" Enabled:  {}", if enabled { "✅ Yes" } else { "❌ No" }),
                    Style::default().fg(if enabled {
                        COLOR_APPROVAL_OK
                    } else {
                        COLOR_ERROR
                    }),
                )),
                Line::from(Span::styled(
                    " Timeout:  60s (default)",
                    Style::default().fg(Color::Rgb(120, 140, 190)),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Built-in tools:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   bash         Shell execution (approval required)",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(Span::styled(
                    "   read_file    Read any file from disk",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(Span::styled(
                    "   write_file   Write/modify files (approval required)",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(Span::styled(
                    "   list_dir     List directory contents",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(Span::styled(
                    "   call_subagent  Spawn a subagent (approval required)",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " All dangerous operations require ApprovalGate.",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    " Use /tools for full list in chat.",
                    Style::default().fg(COLOR_HELP),
                )),
            ];
            (" 🔧 Tools ", lines)
        }

        ActiveView::Memory => {
            let brain_ok = app.brain.is_some();
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🧠  Memory & Brain",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!(
                        " Brain DB: {}",
                        if app.brain_disabled {
                            "❌ Disabled (--no-brain)"
                        } else if brain_ok {
                            "✅ Connected"
                        } else {
                            "⚠ Not connected"
                        }
                    ),
                    Style::default().fg(if brain_ok && !app.brain_disabled {
                        COLOR_APPROVAL_OK
                    } else {
                        COLOR_APPROVAL
                    }),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Memory files:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   USER.md    User profile & preferences",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(Span::styled(
                    "   MEMORY.md  Project context & notes",
                    Style::default().fg(Color::Rgb(110, 130, 175)),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Memory commands:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   /memory status      show memory status",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /memory add <note>  add a note",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /recall <query>     search history",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /save-skill <name>  save current skill",
                    Style::default().fg(COLOR_HELP),
                )),
            ];
            (" 🧠 Memory ", lines)
        }

        ActiveView::Skills => {
            let active = app.active_skill.as_deref().unwrap_or("None");
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🎯  Skills",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!(" Active skill: {}", active),
                    Style::default().fg(if active == "None" {
                        COLOR_DIM
                    } else {
                        COLOR_SKILL
                    }),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Skills allow GOAT to follow reusable instruction sets.",
                    Style::default().fg(Color::Rgb(100, 115, 160)),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Skill commands:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   /skills              list available skills",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /skill <name>        activate a skill",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /skill deactivate    deactivate current skill",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /save-skill <name>   save current context as skill",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Skills are stored in ~/.config/goat/skills/",
                    Style::default().fg(COLOR_DIM),
                )),
            ];
            (" 🎯 Skills ", lines)
        }

        ActiveView::Subagents => {
            let subagents = app.subagent_manager.registry.list_all();
            let mut lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🤖  Internal Subagents",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];
            for sa in &subagents {
                lines.push(Line::from(Span::styled(
                    format!(" ● {}", sa.name),
                    Style::default()
                        .fg(COLOR_AGENT)
                        .add_modifier(Modifier::BOLD),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                " Use /subagents for details in chat.",
                Style::default().fg(COLOR_HELP),
            )));
            lines.push(Line::from(Span::styled(
                " /ask-agent <name> <task> to run one.",
                Style::default().fg(COLOR_HELP),
            )));
            lines.push(Line::from(Span::styled(
                " /agents for selector modal.",
                Style::default().fg(COLOR_HELP),
            )));
            (" 🤖 Subagents ", lines)
        }

        ActiveView::ExternalAgents => {
            let agents = app.external_agent_manager.registry.list_all();
            let exec_ok = app.config.external_agents.allow_execution;
            let mut lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " 🔗  External Agents",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!(
                        " Execution: {}",
                        if exec_ok {
                            "✅ Allowed"
                        } else {
                            "❌ Disabled by default"
                        }
                    ),
                    Style::default().fg(if exec_ok {
                        COLOR_APPROVAL_OK
                    } else {
                        COLOR_DIM
                    }),
                )),
                Line::from(""),
            ];

            if agents.is_empty() {
                lines.push(Line::from(Span::styled(
                    " No external agents detected.",
                    Style::default().fg(COLOR_DIM),
                )));
            } else {
                for a in &agents {
                    lines.push(Line::from(Span::styled(
                        format!(" {} [{}] — {}", a.name, a.command_name, a.status),
                        Style::default().fg(Color::Rgb(110, 130, 175)),
                    )));
                }
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                " External agent is disabled by default for safety.",
                Style::default().fg(COLOR_DIM),
            )));
            lines.push(Line::from(Span::styled(
                " /external-agents doctor   check readiness",
                Style::default().fg(COLOR_HELP),
            )));
            lines.push(Line::from(Span::styled(
                " /delegate-external <name> <task>",
                Style::default().fg(COLOR_HELP),
            )));
            (" 🔗 External Agents ", lines)
        }

        ActiveView::Help => {
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    " ❓  Help & Quick Reference",
                    Style::default()
                        .fg(COLOR_HEADER_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Quick start:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   /help           all commands",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /status         system status",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   /doctor         health check",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Keyboard shortcuts:",
                    Style::default().fg(COLOR_DIM),
                )),
                Line::from(Span::styled(
                    "   Ctrl+P  command palette",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Ctrl+F  focus layout",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Ctrl+D  dashboard layout",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Ctrl+B  toggle sidebar",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Ctrl+1-9  switch views",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Ctrl+L  clear log",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   Tab     complete command",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(Span::styled(
                    "   ↑↓      input history / navigate suggestions",
                    Style::default().fg(COLOR_HELP),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    " Type /view chat to return to chat.",
                    Style::default().fg(COLOR_DIM),
                )),
            ];
            (" ❓ Help ", lines)
        }

        _ => (
            " View ",
            vec![Line::from(Span::styled(
                " This view is not rendered here.",
                Style::default().fg(COLOR_DIM),
            ))],
        ),
    }
}

// ── Logs view ─────────────────────────────────────────────────────────────────

fn render_logs_view(f: &mut Frame, app: &App, area: Rect) {
    let total = app.logs.len();
    let visible_height = area.height.saturating_sub(2) as usize;
    let bottom = total.saturating_sub(app.log_scroll);
    let top = bottom.saturating_sub(visible_height.min(MAX_RENDER_LINES));
    let visible_slice = &app.logs[top..bottom];

    // In logs view, show all log lines (not just [YOU]/[GOAT]) with full color coding.
    let logs_text: Vec<Line> = visible_slice
        .iter()
        .map(|line| color_log_line(line))
        .collect();

    let scroll_hint = if app.log_scroll > 0 {
        format!(
            " 📜 Logs  [↑↓ scroll │ {} above │ /logs clear to clear] ",
            app.log_scroll
        )
    } else {
        " 📜 System & Tool Logs  [↑↓ to scroll │ /logs clear to clear] ".to_string()
    };

    let block = Block::default()
        .title(scroll_hint)
        .title_style(Style::default().fg(Color::Rgb(100, 115, 160)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_BORDER_FOCUS));

    f.render_widget(
        Paragraph::new(logs_text)
            .block(block)
            .wrap(Wrap { trim: false }),
        area,
    );
}

// ── Command palette modal ─────────────────────────────────────────────────────

fn render_palette_modal(f: &mut Frame, app: &App, area: Rect) {
    let modal_area = center_area(area, 76, 36);
    f.render_widget(Clear, modal_area);

    let registry = crate::command_registry::CommandRegistry::build();
    let palette_lines = registry.format_palette(None);

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Type a command below to run it. Tab=autocomplete  Esc=close",
            Style::default()
                .fg(Color::Rgb(110, 130, 175))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
    ];

    for raw in &palette_lines {
        // Color code the palette lines: group headers vs entries vs risk warnings
        let line_style = if raw.starts_with("──") || raw.contains("━") {
            Style::default().fg(Color::Rgb(45, 60, 100))
        } else if raw.ends_with("──") || raw.starts_with("  ✅") || raw.starts_with("  ⚡") {
            Style::default().fg(Color::Rgb(75, 90, 140))
        } else if raw.contains("🔮") {
            Style::default().fg(Color::Rgb(60, 70, 110))
        } else if raw.contains("⚠") || raw.contains("CRITICAL") || raw.contains("HIGH") {
            Style::default()
                .fg(Color::Rgb(220, 150, 60))
                .add_modifier(Modifier::BOLD)
        } else if raw.starts_with("  /") || raw.contains("│") {
            // Command entry — split command vs description
            let trimmed = raw.trim_start();
            if let Some(space_pos) = trimmed.find("  ") {
                let cmd = &trimmed[..space_pos];
                let desc = &trimmed[space_pos..];
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        cmd.to_string(),
                        Style::default()
                            .fg(Color::Rgb(120, 190, 255))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        desc.to_string(),
                        Style::default().fg(Color::Rgb(100, 115, 160)),
                    ),
                ]));
                continue;
            }
            Style::default().fg(Color::Rgb(120, 190, 255))
        } else {
            Style::default().fg(Color::Rgb(80, 100, 155))
        };
        lines.push(Line::from(Span::styled(raw.clone(), line_style)));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(COLOR_BORDER_FOCUS)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " ⌨  Command Palette  — Ctrl+P to toggle │ /view chat to exit ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        modal_area,
    );
}

// ── Agent selector modal ──────────────────────────────────────────────────────

fn render_agent_selector_modal(f: &mut Frame, app: &App, area: Rect) {
    let modal_area = center_area(area, 72, 32);
    f.render_widget(Clear, modal_area);

    let subagents = app.subagent_manager.registry.list_all();
    let ext_agents = app.external_agent_manager.registry.list_all();
    let exec_allowed = app.config.external_agents.allow_execution;

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            " Select an agent to run. Type the command below and press Enter.",
            Style::default()
                .fg(Color::Rgb(110, 130, 175))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " ── INTERNAL SUBAGENTS ─────────────────────────────────────────",
            Style::default().fg(Color::Rgb(55, 70, 115)),
        )),
        Line::from(""),
    ];

    for sa in &subagents {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  🤖 {:<20}", sa.name),
                Style::default()
                    .fg(Color::Rgb(100, 170, 255))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  /ask-agent {} <task>", sa.name),
                Style::default().fg(Color::Rgb(80, 100, 155)),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " ── EXTERNAL AGENTS ────────────────────────────────────────────",
        Style::default().fg(Color::Rgb(55, 70, 115)),
    )));
    lines.push(Line::from(""));

    if !exec_allowed {
        lines.push(Line::from(Span::styled(
            "  ❌ External execution is disabled by default.",
            Style::default().fg(COLOR_DIM),
        )));
        lines.push(Line::from(Span::styled(
            "     Run /external-agents doctor to check readiness.",
            Style::default().fg(Color::Rgb(70, 85, 130)),
        )));
    } else if ext_agents.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No external agents detected.",
            Style::default().fg(COLOR_DIM),
        )));
    } else {
        for a in &ext_agents {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  🔗 {:<20}", a.name),
                    Style::default()
                        .fg(Color::Rgb(100, 200, 150))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  /delegate-external {} <task>", a.command_name),
                    Style::default().fg(Color::Rgb(80, 100, 155)),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " ── ACTIVE SKILL ───────────────────────────────────────────────",
        Style::default().fg(Color::Rgb(55, 70, 115)),
    )));
    lines.push(Line::from(""));

    match &app.active_skill {
        Some(skill) => {
            lines.push(Line::from(Span::styled(
                format!("  🎯 Active: {}", skill),
                Style::default()
                    .fg(COLOR_SKILL)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "     /skill deactivate   to deactivate",
                Style::default().fg(COLOR_DIM),
            )));
        }
        None => {
            lines.push(Line::from(Span::styled(
                "  No active skill. Use /skill <name> to activate one.",
                Style::default().fg(COLOR_DIM),
            )));
            lines.push(Line::from(Span::styled(
                "  /skills to list available skills.",
                Style::default().fg(Color::Rgb(70, 85, 130)),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  /view chat or Esc to return to chat",
        Style::default()
            .fg(COLOR_DIM)
            .add_modifier(Modifier::ITALIC),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Rgb(70, 110, 190))
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " 🤖  Agent & Skill Selector ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        modal_area,
    );
}

// ── Header bar ────────────────────────────────────────────────────────────────

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let (status_label, status_color) = match &app.status {
        AppStatus::Ready => ("● READY", Color::Rgb(70, 200, 110)),
        AppStatus::Thinking => ("◌ THINKING…", Color::Rgb(255, 200, 50)),
        AppStatus::ToolRunning(_) => ("⚙ RUNNING", Color::Rgb(100, 180, 255)),
        AppStatus::WaitingApproval(_) => ("⚠ APPROVAL", Color::Rgb(255, 160, 50)),
        AppStatus::Error(_) => ("✕ ERROR", Color::Rgb(240, 70, 70)),
    };

    let skill_tag = match &app.active_skill {
        Some(s) => {
            let short = if s.len() > 10 { &s[..10] } else { s.as_str() };
            format!(" │ 🎯 {}", short)
        }
        None => String::new(),
    };

    let mcp_tag = if app.mcp_server_count > 0 {
        format!(" │ MCP:{}", app.mcp_server_count)
    } else {
        String::new()
    };

    let mode_str = match app.workflow.mode {
        crate::task::AgentMode::Plan => " │ 📝 PLAN",
        crate::task::AgentMode::Act => " │ ⚡ ACT",
    };

    let task_tag = if let Some(task) = &app.workflow.active_task {
        let status = match task.status {
            crate::task::TaskStatus::Planning => "Planning",
            crate::task::TaskStatus::AwaitingApproval => "Awaiting",
            crate::task::TaskStatus::PatchProposed => "Patch?",
            crate::task::TaskStatus::PatchApplied => "Patched",
            crate::task::TaskStatus::Testing => "Testing",
            crate::task::TaskStatus::Failed => "Failed",
            crate::task::TaskStatus::Completed => "Done",
        };
        format!(" │ T: {}", status)
    } else {
        String::new()
    };

    let layout_tag = format!(" │ 📐 {}", app.layout_mode.label());

    let spans = vec![
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
                .fg(Color::Rgb(80, 95, 140))
                .bg(COLOR_HEADER_BG),
        ),
        Span::styled(
            format!(
                " │ {} │ {}{}{}{}{}{}",
                app.active_profile,
                app.provider_label,
                mcp_tag,
                skill_tag,
                mode_str,
                task_tag,
                layout_tag
            ),
            Style::default().fg(COLOR_HEADER_FG).bg(COLOR_HEADER_BG),
        ),
        Span::styled(
            " │ ",
            Style::default()
                .fg(Color::Rgb(50, 65, 105))
                .bg(COLOR_HEADER_BG),
        ),
        Span::styled(
            status_label,
            Style::default()
                .fg(status_color)
                .bg(COLOR_HEADER_BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "                                                                                ",
            Style::default().bg(COLOR_HEADER_BG),
        ),
    ];

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

// ── Chat / Log panel ──────────────────────────────────────────────────────────

fn render_log(f: &mut Frame, app: &App, area: Rect) {
    let total = app.logs.len();
    let visible_height = area.height.saturating_sub(2) as usize;

    let bottom = total.saturating_sub(app.log_scroll);
    let top = bottom.saturating_sub(visible_height.min(MAX_RENDER_LINES));
    let visible_slice = &app.logs[top..bottom];

    let logs_text: Vec<Line> = visible_slice
        .iter()
        .map(|line| color_log_line(line))
        .collect();

    let scroll_hint = if app.log_scroll > 0 {
        format!(
            " Chat  [↑↓ PgUp/PgDn scroll │ End=bottom │ {} above] ",
            app.log_scroll
        )
    } else {
        " Chat  [↑↓ scroll │ / for commands │ Tab=autocomplete] ".to_string()
    };

    // Panel border pulses amber when approval is pending.
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
            .title_style(Style::default().fg(Color::Rgb(90, 110, 165)))
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    f.render_widget(log_block, area);
}

/// Colour-code a single log line based on its `[TAG]` prefix.
fn color_log_line(line: &str) -> Line<'static> {
    let owned = line.to_owned();

    // Diff line coloring: `+` added, `-` removed, `@@` hunk headers.
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
    } else if line.starts_with("[THEME]") {
        Style::default().fg(Color::Rgb(180, 140, 255))
    } else {
        Style::default().fg(COLOR_DIM)
    };

    Line::from(Span::styled(owned, style))
}

// ── Input composer ────────────────────────────────────────────────────────────

fn render_input(f: &mut Frame, app: &App, area: Rect) {
    if app.has_pending_approval() {
        let hint = Paragraph::new(
            "  ⚠  APPROVAL — [y] approve  [n] deny  [a] always allow  [d] always deny  [Esc] deny",
        )
        .style(
            Style::default()
                .fg(Color::Rgb(255, 200, 80))
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title(Span::styled(
                    " ⚠ Action Required — See overlay above ",
                    Style::default()
                        .fg(Color::Rgb(255, 160, 50))
                        .add_modifier(Modifier::BOLD),
                ))
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

    // Premium placeholder: context-aware hint.
    let placeholder = match &app.status {
        AppStatus::Thinking => "  🧠 GOAT is thinking…",
        AppStatus::ToolRunning(_) => "  ⚙  Running tool…",
        AppStatus::Error(_) => "  Error — type a new message or /status",
        _ => "  Ask GOAT anything…  (/ for commands · Tab=complete · ↑=history)",
    };

    // Mode hint displayed in title
    let mode_hint = match app.workflow.mode {
        crate::task::AgentMode::Plan => "📝 PLAN",
        crate::task::AgentMode::Act => "⚡ ACT",
    };

    let profile_hint = &app.active_profile;

    let display_text = if app.input.is_empty() {
        Span::styled(
            placeholder,
            Style::default()
                .fg(Color::Rgb(65, 78, 115))
                .add_modifier(Modifier::ITALIC),
        )
    } else {
        Span::styled(
            format!("  {}", app.input),
            Style::default().fg(Color::Rgb(210, 225, 255)),
        )
    };

    let border_style = if app.input.is_empty() {
        Style::default().fg(COLOR_INPUT_BORDER)
    } else {
        Style::default()
            .fg(COLOR_INPUT_ACTIVE)
            .add_modifier(Modifier::BOLD)
    };

    // In focus layout on wide terminals, center the input box.
    let input_area = if app.layout_mode == LayoutMode::Focus && area.width > 130 {
        center_area(area, 120, area.height)
    } else {
        area
    };

    let title = format!(" {} │ {} │ Message ", mode_hint, profile_hint);

    let input_block = Paragraph::new(Line::from(display_text)).block(
        Block::default()
            .title(Span::styled(
                title,
                Style::default().fg(Color::Rgb(80, 100, 170)),
            ))
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(input_block, input_area);

    // Position cursor in the input box.
    #[allow(clippy::cast_possible_truncation)]
    let cursor_x = input_area.x + app.input.chars().count() as u16 + 3;
    let cursor_y = input_area.y + 1;
    if cursor_x < input_area.x + input_area.width.saturating_sub(1) {
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

// ── Approval overlay ──────────────────────────────────────────────────────────

fn render_approval_overlay(f: &mut Frame, lines: &[String], area: Rect) {
    let content_lines = lines.len() as u16;
    let overlay_height = (content_lines + 4).min(area.height.saturating_sub(4));
    let overlay_width = 88u16.min(area.width.saturating_sub(4));

    if overlay_width < 20 || overlay_height < 4 {
        return;
    }

    let x = area.x + (area.width.saturating_sub(overlay_width)) / 2;
    let y = area.y + (area.height.saturating_sub(overlay_height)) / 2;

    let overlay_area = Rect {
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
                Style::default().fg(COLOR_DIFF_ADD)
            } else if l.starts_with("- ") || l == "-" {
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

    let has_critical = lines.iter().any(|l| l.contains("CRITICAL"));
    let border_color = if has_critical {
        Color::Rgb(240, 60, 60)
    } else {
        Color::Rgb(255, 130, 35)
    };

    let overlay_block = Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(
                    " ⚠  APPROVAL REQUIRED — Review before executing ",
                    Style::default()
                        .fg(if has_critical {
                            Color::Rgb(255, 80, 80)
                        } else {
                            Color::Rgb(255, 190, 55)
                        })
                        .add_modifier(Modifier::BOLD),
                ))
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

// ── Slash command suggestion popup ────────────────────────────────────────────

/// Renders a floating suggestion popup just above the input composer.
///
/// Phase 3.2 improvements:
/// - Better position (respects focus layout centering)
/// - Shows command name + short description
/// - Planned commands shown differently (dimmed)
/// - Risk marker for dangerous commands
/// - Cleaner selected row with ▶ indicator
fn render_suggestion_popup(f: &mut Frame, app: &App, input_area: Rect) {
    let suggestions = &app.cmd_suggestions;
    if suggestions.is_empty() {
        return;
    }

    let popup_height = (suggestions.len() as u16 + 2).min(16);
    let popup_width = 76u16.min(input_area.width.saturating_sub(4));

    if popup_width < 24 || input_area.y < popup_height + 1 {
        return;
    }

    // In focus mode on wide terminals, align popup with the centered input area.
    let base_x = if app.layout_mode == LayoutMode::Focus && input_area.width > 130 {
        let centered_start = input_area.x + (input_area.width.saturating_sub(120)) / 2;
        centered_start + 2
    } else {
        input_area.x + 2
    };

    let popup_area = Rect {
        x: base_x,
        y: input_area.y.saturating_sub(popup_height),
        width: popup_width,
        height: popup_height,
    };

    f.render_widget(Clear, popup_area);

    let selected = app.cmd_suggestion_idx;
    let registry = crate::command_registry::CommandRegistry::build();

    let items: Vec<Line> = suggestions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            // Parse the suggestion string to find command name
            let cmd_name = s.split_whitespace().next().unwrap_or(s.as_str());

            // Look up metadata for risk/status markers
            let meta = registry.find(cmd_name);
            let risk_marker = meta
                .and_then(|m| {
                    use crate::command_registry::CommandRisk;
                    match m.risk {
                        CommandRisk::Critical => Some(" ⛔"),
                        CommandRisk::High => Some(" ⚠"),
                        _ => None,
                    }
                })
                .unwrap_or("");

            let is_planned = meta
                .map(|m| matches!(m.status, crate::command_registry::CommandStatus::Planned))
                .unwrap_or(false);

            if i == selected {
                Line::from(Span::styled(
                    format!(" ▶ {}{} ", s, risk_marker),
                    Style::default()
                        .fg(Color::Rgb(20, 25, 45))
                        .bg(Color::Rgb(70, 210, 130))
                        .add_modifier(Modifier::BOLD),
                ))
            } else if is_planned {
                Line::from(Span::styled(
                    format!("   {} 🔮", s),
                    Style::default().fg(Color::Rgb(70, 80, 120)),
                ))
            } else {
                // Split command vs description part
                let (cmd_part, desc_part) = s
                    .find("  ")
                    .map(|pos| s.split_at(pos))
                    .unwrap_or((s.as_str(), ""));

                let cmd_color = if risk_marker.is_empty() {
                    Color::Rgb(110, 190, 255)
                } else {
                    Color::Rgb(255, 170, 80)
                };

                Line::from(vec![
                    Span::styled(
                        format!("   {}", cmd_part),
                        Style::default().fg(cmd_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{}{}", desc_part, risk_marker),
                        Style::default().fg(Color::Rgb(90, 105, 155)),
                    ),
                ])
            }
        })
        .collect();

    let block = Block::default()
        .title(Span::styled(
            " / Commands — Tab:complete  ↑↓:navigate  Esc:close ",
            Style::default()
                .fg(COLOR_HEADER_ACCENT)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Rgb(60, 110, 190))
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(Paragraph::new(items).block(block), popup_area);
}

// ── Layout helpers ────────────────────────────────────────────────────────────

/// Center an area within a parent, clamped to parent bounds.
///
/// `max_w` and `max_h` are the maximum size of the centered area.
/// If parent is smaller, the full parent area is returned.
fn center_area(parent: Rect, max_w: u16, max_h: u16) -> Rect {
    let w = max_w.min(parent.width);
    let h = max_h.min(parent.height);
    let x = parent.x + (parent.width.saturating_sub(w)) / 2;
    let y = parent.y + (parent.height.saturating_sub(h)) / 2;
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}
