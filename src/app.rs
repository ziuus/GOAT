use crate::approval::{
    ApprovalDecision, ApprovalGate, ApprovalRequest, bash_approval_request,
    call_subagent_approval_request,
};
use crate::brain::Brain;
use crate::config::Config;
use crate::llm::{FunctionDeclaration, LlmRouter, Message, Tool};
use crate::mcp::McpManager;
use crate::models::{ModelChain, ProfileRegistry};
use crate::paths::GoatPaths;
use crate::runtime::GoatRuntime;
use crate::swarm::{RouteDecision, SwarmRouter};
use crate::tools::NativeTools;
use serde_json::Value;
use std::path::PathBuf;
use tracing::info;
use uuid::Uuid;

const MAX_LOG_LINES: usize = 500;
const MAX_HISTORY_MESSAGES: usize = 80;

/// A tool call that has been deferred pending user approval.
struct DeferredToolCall {
    id: String,
    name: String,
    args: Value,
    request: ApprovalRequest,
    patch_id: Option<String>,
}

/// Application status — shown in the header bar.
#[derive(Clone, PartialEq, Eq)]
pub enum AppStatus {
    Ready,
    Thinking,
    ToolRunning(String),
    WaitingApproval(String),
    Error(String),
}

impl AppStatus {
    pub fn label(&self) -> String {
        match self {
            AppStatus::Ready => "READY".to_string(),
            AppStatus::Thinking => "THINKING…".to_string(),
            AppStatus::ToolRunning(t) => format!("RUNNING: {t}"),
            AppStatus::WaitingApproval(t) => format!("APPROVAL REQUIRED: {t}"),
            AppStatus::Error(e) => format!("ERROR: {e}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ActiveView {
    Chat,
    Tasks,
    RepoMap,
    Patches,
    Tools,
    Memory,
    Skills,
    Subagents,
    ExternalAgents,
    Help,
    CommandPalette,
}

pub struct App {
    pub running: bool,
    /// Chat and tool log lines, colour-coded in the TUI.
    pub logs: Vec<String>,
    /// Current status displayed in the header bar.
    pub status: AppStatus,
    /// The text currently typed in the input composer.
    pub input: String,
    /// Scroll offset for the log panel (lines from bottom).
    pub log_scroll: usize,
    pub paths: GoatPaths,
    pub brain: Option<Brain>,
    pub llm_router: LlmRouter,
    pub mcp_manager: McpManager,
    pub history: Vec<Message>,
    pub config: Config,
    pub swarm_router: SwarmRouter,
    pub active_route: Option<RouteDecision>,
    pub session_id: String,
    /// Active provider:model label shown in the header bar.
    pub provider_label: String,
    /// Active profile name (e.g. "balanced", "coding").
    pub active_profile: String,
    /// Fallback chain for the active profile.
    pub model_chain: ModelChain,
    /// Profile registry — needed for /profile switching.
    pub profile_registry: ProfileRegistry,
    /// Number of running MCP servers.
    pub mcp_server_count: usize,
    /// The approval gate for this session.
    pub approval_gate: ApprovalGate,
    /// Whether brain was disabled via --no-brain.
    pub brain_disabled: bool,
    /// Pending approval (Some ↔ approval overlay visible).
    pending_approval: Option<DeferredToolCall>,
    /// Explicitly activated skill for the session
    pub active_skill: Option<String>,
    /// Command history for ↑ navigation in the input composer.
    pub input_history: Vec<String>,
    /// Current index into input_history when navigating with ↑/↓.
    /// `None` means we are at the live input (not browsing history).
    pub history_idx: Option<usize>,
    /// Workflow state for Phase 2.5
    pub workflow: crate::task::WorkflowState,
    /// Tool registry
    pub tool_registry: crate::tool_registry::ToolRegistry,
    /// Subagent Manager
    pub subagent_manager: crate::subagents::SubagentManager,
    /// External Agent Manager
    pub external_agent_manager: crate::external_agents::ExternalAgentManager,
    /// Active view for Phase 3.0 UI
    pub active_view: ActiveView,
}

impl App {
    /// Create `App` from a pre-bootstrapped `GoatRuntime`.
    ///
    /// This is the preferred constructor used in production.
    /// `boot_log` contains messages from the bootstrap phase (brain connection,
    /// session resume, security notices) to display in the TUI at startup.
    pub fn from_runtime(rt: GoatRuntime, boot_log: Vec<String>) -> Self {
        let mut logs: Vec<String> = Vec::new();

        // ── TUI splash header ─────────────────────────────────────────────────
        let version = env!("CARGO_PKG_VERSION");
        logs.push(format!(
            "[GOAT] 🐐 GOAT v{} — General Omniscient Agentic Tool",
            version
        ));
        logs.push(
            "[GOAT] Type your message and press Enter to chat with the AI agent.".to_string(),
        );
        logs.push(
            "[GOAT] ─────────────────────────────────────────────────────────────────".to_string(),
        );
        logs.push(
            "[HELP] Quick start: /help · /status · /doctor · /repo-map · /skills".to_string(),
        );
        logs.push("[HELP] Dev tools: /check · /test · /lint · /format · /patch".to_string());
        logs.push("[HELP] Memory:    /memory · /recall <query> · /save-skill <name>".to_string());
        logs.push("[HELP] Sessions:  /new · /sessions · /profile · /profiles".to_string());
        logs.push(
            "[HELP] Keys:      Enter=send · ↑=history · Ctrl+C=quit · Esc=cancel".to_string(),
        );
        logs.push(
            "[GOAT] ─────────────────────────────────────────────────────────────────".to_string(),
        );

        // Show startup_warnings (config permission issues, etc.).
        for warning in &rt.startup_warnings {
            logs.push(warning.clone());
        }

        // Show runtime boot log (brain connection, session, profile, security).
        for msg in &boot_log {
            logs.push(msg.clone());
        }

        let provider_label = rt.provider_label.clone();
        let active_profile = rt.active_profile.clone();
        let session_id = rt.session_id.clone();
        let mcp_server_count = rt.mcp_server_count;
        let brain_disabled = rt.brain_disabled;

        Self {
            running: true,
            logs,
            status: AppStatus::Ready,
            input: String::new(),
            log_scroll: 0,
            paths: rt.paths,
            brain: rt.brain,
            llm_router: rt.llm_router,
            mcp_manager: rt.mcp_manager,
            history: rt.history,
            config: rt.config,
            swarm_router: rt.swarm_router,
            active_route: None,
            session_id,
            provider_label,
            active_profile,
            model_chain: rt.model_chain,
            profile_registry: rt.profile_registry,
            mcp_server_count,
            approval_gate: rt.approval_gate,
            brain_disabled,
            pending_approval: None,
            active_skill: None,
            input_history: Vec::new(),
            history_idx: None,
            workflow: rt.workflow,
            tool_registry: rt.tool_registry,
            subagent_manager: rt.subagent_manager,
            external_agent_manager: rt.external_agent_manager,
            active_view: ActiveView::Chat,
        }
    }

    /// Create a new `App` directly from config + paths.
    pub fn new(config: Config, paths: GoatPaths, startup_warnings: Vec<String>) -> Self {
        let (runtime, boot_log) =
            GoatRuntime::bootstrap(config, paths, startup_warnings, false, None);
        Self::from_runtime(runtime, boot_log)
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn push_log(&mut self, log: impl Into<String>) {
        self.logs.push(log.into());
        self.trim_logs();
        // Auto-scroll to bottom when new content arrives, unless user has scrolled up.
        if self.log_scroll == 0 {
            // Already at bottom — nothing to do.
        }
    }

    pub fn extend_logs(&mut self, logs: impl IntoIterator<Item = String>) {
        self.logs.extend(logs);
        self.trim_logs();
    }

    fn trim_logs(&mut self) {
        let extra = self.logs.len().saturating_sub(MAX_LOG_LINES);
        if extra > 0 {
            self.logs.drain(0..extra);
            // Adjust scroll offset to stay coherent after drain.
            self.log_scroll = self.log_scroll.saturating_sub(extra);
        }
    }

    fn trim_history(&mut self) {
        let extra = self.history.len().saturating_sub(MAX_HISTORY_MESSAGES);
        if extra > 0 {
            self.history.drain(0..extra);
        }
    }

    // ── Log scrolling ─────────────────────────────────────────────────────────

    /// Scroll the log panel up by `lines` (towards older messages).
    pub fn scroll_up(&mut self, lines: usize) {
        self.log_scroll = self.log_scroll.saturating_add(lines);
        // Clamp to actual content.
        let max_scroll = self.logs.len().saturating_sub(1);
        if self.log_scroll > max_scroll {
            self.log_scroll = max_scroll;
        }
    }

    /// Scroll the log panel down by `lines` (towards newest messages).
    pub fn scroll_down(&mut self, lines: usize) {
        self.log_scroll = self.log_scroll.saturating_sub(lines);
    }

    /// Jump to the bottom of the log.
    pub fn scroll_to_bottom(&mut self) {
        self.log_scroll = 0;
    }

    // ── Input history navigation ───────────────────────────────────────────────

    /// Navigate to the previous command in history (↑ key).
    pub fn history_up(&mut self) {
        if self.input_history.is_empty() {
            return;
        }
        match self.history_idx {
            None => {
                // First press: go to the most recent history entry.
                self.history_idx = Some(self.input_history.len() - 1);
            }
            Some(0) => {
                // Already at the oldest entry — stay there.
                self.history_idx = Some(0);
            }
            Some(i) => {
                self.history_idx = Some(i - 1);
            }
        }
        if let Some(idx) = self.history_idx {
            self.input = self.input_history[idx].clone();
        }
    }

    /// Navigate to the next command in history (↓ key).
    pub fn history_down(&mut self) {
        match self.history_idx {
            None => {} // already at live input
            Some(i) if i + 1 >= self.input_history.len() => {
                // Past the end → return to live (empty) input.
                self.history_idx = None;
                self.input.clear();
            }
            Some(i) => {
                self.history_idx = Some(i + 1);
                self.input = self.input_history[i + 1].clone();
            }
        }
    }

    /// Record `text` into input history and reset the index to live input.
    pub fn commit_to_history(&mut self, text: &str) {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return;
        }
        // Avoid duplicate consecutive entries.
        if self.input_history.last().map(|s| s.as_str()) != Some(trimmed) {
            self.input_history.push(trimmed.to_string());
            // Cap history at 200 entries to prevent unbounded growth.
            if self.input_history.len() > 200 {
                self.input_history.remove(0);
            }
        }
        self.history_idx = None;
    }

    // ── Approval gate integration ─────────────────────────────────────────────

    /// Returns `true` when a tool call is paused waiting for user approval.
    pub fn has_pending_approval(&self) -> bool {
        self.pending_approval.is_some()
    }

    /// Returns display lines for the approval overlay, if one is pending.
    pub fn pending_approval_lines(&self) -> Option<Vec<String>> {
        self.pending_approval
            .as_ref()
            .map(|d| d.request.display_lines())
    }

    /// Resolve the pending approval with the user's input character.
    pub async fn resolve_approval(&mut self, input: char) {
        let deferred = match self.pending_approval.take() {
            Some(d) => d,
            None => return,
        };

        let decision = self.approval_gate.resolve(&deferred.request, input);

        match decision {
            ApprovalDecision::Approved => {
                info!(tool = %deferred.name, "approval granted");
                self.push_log(format!(
                    "[APPROVAL] ✓ Approved: {} — {}",
                    deferred.name, deferred.request.action_summary
                ));
                self.status = AppStatus::ToolRunning(deferred.name.clone());

                let tool_result = execute_native_tool(&deferred.name, deferred.args.clone()).await;

                if let Some(id) = &deferred.patch_id {
                    if let Some(p) = self.workflow.get_patch_mut(id) {
                        p.status = crate::task::PatchStatus::Applied;
                    }
                    if let Some(task) = &mut self.workflow.active_task {
                        task.status = crate::task::TaskStatus::PatchApplied;
                    }
                }

                self.push_log(format!("[TOOL] {}", tool_result));

                self.history.push(Message {
                    role: "tool".to_string(),
                    content: Some(tool_result),
                    tool_calls: None,
                    tool_call_id: Some(deferred.id),
                });
                self.trim_history();
                self.status = AppStatus::Ready;
            }
            ApprovalDecision::Denied(reason) => {
                info!(tool = %deferred.name, reason = %reason, "approval denied");
                self.push_log(format!(
                    "[APPROVAL] ✗ Denied: {} — {}",
                    deferred.name, reason
                ));

                if let Some(id) = &deferred.patch_id {
                    if let Some(p) = self.workflow.get_patch_mut(id) {
                        p.status = crate::task::PatchStatus::Discarded;
                    }
                }

                self.history.push(Message {
                    role: "tool".to_string(),
                    content: Some(format!(
                        "Tool execution denied by user. Reason: {}. \
                         Please adapt your plan without using this tool, \
                         or ask the user to approve it explicitly.",
                        reason
                    )),
                    tool_calls: None,
                    tool_call_id: Some(deferred.id),
                });
                self.trim_history();
                self.status = AppStatus::Ready;
            }
        }
        // Scroll to bottom so user sees the approval result.
        self.log_scroll = 0;
    }

    // ── Slash command handler ─────────────────────────────────────────────────

    /// Handle a slash command typed by the user.
    ///
    /// Returns `true` if the command was handled (and the input should be
    /// consumed), `false` if it was not a known slash command.
    pub async fn handle_slash_command(&mut self, cmd: &str) -> bool {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        let name = parts[0].to_lowercase();
        let _args = parts.get(1).copied().unwrap_or("").trim();

        match name.as_str() {
            "/view" => {
                let view_name = parts.get(1).copied().unwrap_or("").trim().to_lowercase();
                match view_name.as_str() {
                    "chat" => self.active_view = ActiveView::Chat,
                    "tasks" => self.active_view = ActiveView::Tasks,
                    "repo" => self.active_view = ActiveView::RepoMap,
                    "patches" => self.active_view = ActiveView::Patches,
                    "tools" => self.active_view = ActiveView::Tools,
                    "memory" => self.active_view = ActiveView::Memory,
                    "skills" => self.active_view = ActiveView::Skills,
                    "subagents" => self.active_view = ActiveView::Subagents,
                    "external" => self.active_view = ActiveView::ExternalAgents,
                    "help" => self.active_view = ActiveView::Help,
                    _ => {
                        self.push_log(format!("[SYSTEM] Unknown view '{}'. Valid views: chat, tasks, repo, patches, tools, memory, skills, subagents, external, help", view_name));
                    }
                }
                if self.active_view != ActiveView::Chat {
                    self.push_log(format!("[SYSTEM] Switched to view: {}", view_name));
                }
                true
            }
            "/command" | "/palette" => {
                self.active_view = ActiveView::CommandPalette;
                self.push_log("[SYSTEM] Opened Command Palette. Use /view chat to return.");
                true
            }
            "/help" => {
                self.active_view = ActiveView::Help;
                let ver = env!("CARGO_PKG_VERSION");
                self.push_log(format!("[HELP] 🐐 GOAT v{} — Available Commands", ver));
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] General:");
                self.push_log("[HELP]   /help             — this help");
                self.push_log("[HELP]   /status           — system status (provider, memory, git)");
                self.push_log("[HELP]   /ui               — UI info and future plans");
                self.push_log("[HELP]   /clear            — clear log display");
                self.push_log("[HELP]   /tools            — list available tools");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Sessions:");
                self.push_log("[HELP]   /new              — start a new session");
                self.push_log("[HELP]   /sessions         — list recent sessions");
                self.push_log("[HELP]   /profile          — show active profile");
                self.push_log("[HELP]   /profile <name>   — switch profile");
                self.push_log("[HELP]   /profiles         — list available profiles");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Project & Dev:");
                self.push_log("[HELP]   /project          — show/manage project context");
                self.push_log("[HELP]   /project scan     — index current project");
                self.push_log("[HELP]   /repo-map         — show repository map + symbols");
                self.push_log("[HELP]   /repo-map refresh — force rescan");
                self.push_log("[HELP]   /check            — run project check (cargo check, etc.)");
                self.push_log("[HELP]   /test [args]      — run project tests");
                self.push_log("[HELP]   /lint             — run linter (clippy, ruff, etc.)");
                self.push_log("[HELP]   /format           — run formatter");
                self.push_log("[HELP]   /patch            — show pending file patch");
                self.push_log("[HELP]   /patch apply      — apply pending patch");
                self.push_log("[HELP]   /patch discard    — discard pending patch");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Memory & Skills:");
                self.push_log("[HELP]   /memory           — view memory status");
                self.push_log("[HELP]   /memory show user — show USER.md contents");
                self.push_log("[HELP]   /memory show mem  — show MEMORY.md contents");
                self.push_log("[HELP]   /recall <query>   — search conversation history");
                self.push_log("[HELP]   /skills           — list available skills");
                self.push_log("[HELP]   /skill <name>     — activate a skill");
                self.push_log("[HELP]   /skill create <n> — scaffold a new skill");
                self.push_log("[HELP]   /skill search <q> — search skills");
                self.push_log("[HELP]   /skill clear      — deactivate skill");
                self.push_log("[HELP]   /save-skill <n>   — extract skill from session");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Infrastructure:");
                self.push_log("[HELP]   /mcp              — start configured MCP servers");
                self.push_log("[HELP]   /learn            — index project files into brain");
                self.push_log("[HELP]   /route            — show swarm route for current input");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Subagents (Phase 2.7):");
                self.push_log("[HELP]   /subagents        — list internal subagents");
                self.push_log("[HELP]   /subagent <name>  — show subagent details");
                self.push_log("[HELP]   /ask-agent <n> <t>— run a subagent turn");
                self.push_log(
                    "[HELP]   /review           — ask reviewer to review current patch/plan",
                );
                self.push_log("[HELP]   /debug            — ask debugger to analyze recent error");
                self.push_log(
                    "[HELP]   /test-plan        — ask tester for a verification strategy",
                );
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Keyboard:");
                self.push_log("[HELP]   Enter     — send message");
                self.push_log("[HELP]   ↑         — recall previous command (history)");
                self.push_log("[HELP]   ↓         — forward through history");
                self.push_log("[HELP]   Ctrl+C    — quit");
                self.push_log("[HELP]   Ctrl+L    — clear log (same as /clear)");
                self.push_log("[HELP]   ↑↓        — scroll log (when input empty)");
                self.push_log("[HELP]   PgUp/PgDn — fast scroll log");
                self.push_log("[HELP]   Home/End  — jump to top/bottom of log");
                self.push_log("[HELP]   Esc       — clear input / scroll to bottom");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Approval (when overlay is visible):");
                self.push_log("[HELP]   y  — approve this action once");
                self.push_log("[HELP]   n  — deny");
                self.push_log("[HELP]   a  — always allow this tool (session)");
                self.push_log("[HELP]   d  — always deny this tool (session)");
                true
            }

            "/status" => {
                self.push_log(format!("[STATUS] Provider : {}", self.provider_label));
                self.push_log(format!("[STATUS] Profile  : {}", self.active_profile));
                self.push_log(format!(
                    "[STATUS] Fallback : {}",
                    self.model_chain.fallback_display()
                ));
                self.push_log(format!("[STATUS] Session  : {}", self.session_id));
                self.push_log(format!(
                    "[STATUS] Brain    : {}",
                    if self.brain_disabled {
                        "disabled (--no-brain)"
                    } else if self.brain.is_some() {
                        "connected"
                    } else {
                        "unavailable"
                    }
                ));
                self.push_log(format!(
                    "[STATUS] Retries  : {} max / {}s timeout",
                    self.config.llm.effective_max_retries(),
                    self.config.llm.effective_timeout_secs()
                ));
                self.push_log(format!(
                    "[STATUS] History  : {} messages",
                    self.history.len()
                ));
                self.push_log(format!(
                    "[STATUS] MCP      : {} server(s)",
                    self.mcp_server_count
                ));

                // Project & Memory context
                let memory_manager =
                    crate::memory::MemoryManager::new(&self.paths, self.config.memory.clone());
                let (u_count, u_max, _) = memory_manager.user_budget_status();
                let (m_count, m_max, _) = memory_manager.memory_budget_status();
                self.push_log(format!(
                    "[STATUS] Memory   : Enabled={}, USER={}/{}, MEMORY={}/{}",
                    self.config.memory.enabled, u_count, u_max, m_count, m_max
                ));

                if let Some(ref brain) = self.brain {
                    use std::env;
                    let root = env::current_dir().unwrap_or_default();
                    if let Ok(Some(meta)) = brain.get_project(root.to_string_lossy().as_ref()) {
                        self.push_log(format!("[STATUS] Project  : {}", meta.root_path.display()));
                        if !meta.stack.is_empty() {
                            self.push_log(format!("[STATUS] Stack    : {}", meta.stack.join(", ")));
                        }
                    } else {
                        self.push_log("[STATUS] Project  : Not scanned (/project scan)");
                    }
                }
                self.push_log(format!(
                    "[STATUS] Subagents: {} available",
                    self.subagent_manager.registry.list_all().len()
                ));
                let ext_count = self.external_agent_manager.registry.adapters.values().filter(|a| a.status == crate::external_agents::ExternalAgentStatus::Detected).count();
                self.push_log(format!(
                    "[STATUS] Ext Agents: {} detected (Enabled: {})",
                    ext_count, self.config.external_agents.enabled
                ));
                true
            }

            "/mcp" => {
                let subcommand = _args;
                match subcommand {
                    "status" | "" => {
                        let count = self.config.mcp_servers.len();
                        self.push_log(format!("[MCP] Configured MCP Servers: {}", count));
                        let srvs: Vec<_> = self
                            .config
                            .mcp_servers
                            .iter()
                            .map(|(n, s)| (n.clone(), s.command.clone()))
                            .collect();
                        for (name, cmd) in srvs {
                            self.push_log(format!("[MCP] - {}: {}", name, cmd));
                        }
                    }
                    "list" => {
                        let mcp_tools = self.mcp_manager.all_tools();
                        self.push_log(format!("[MCP] {} MCP tools:", mcp_tools.len()));
                        for t in &mcp_tools {
                            if let Some(name) = t.get("name").and_then(|v| v.as_str()) {
                                self.push_log(format!("[MCP]   {}", name));
                            }
                        }
                    }
                    "start" => {
                        self.push_log("[MCP] Starting configured MCP servers...");
                        info!("starting configured MCP servers via slash command");
                        self.start_configured_mcp_servers().await;
                    }
                    _ => self.push_log(
                        "[MCP] Unknown command. Use /mcp status, /mcp list, or /mcp start.",
                    ),
                }
                true
            }

            "/tools" => {
                let subcommand = _args;
                match subcommand {
                    "list" | "" => {
                        let tools = self
                            .tool_registry
                            .list_all()
                            .into_iter()
                            .cloned()
                            .collect::<Vec<_>>();
                        self.push_log(format!(
                            "[TOOLS] GOAT Tool Registry ({} tools)",
                            tools.len()
                        ));
                        for t in &tools {
                            let perm = self
                                .tool_registry
                                .get_permission(&t.name, &self.config.tools);
                            self.push_log(format!(
                                "[TOOLS]   {:<15} [{:?}] - {}",
                                t.name, perm, t.description
                            ));
                        }

                        let mcp_tools = self.mcp_manager.all_tools();
                        if !mcp_tools.is_empty() {
                            self.push_log(format!("[TOOLS] {} MCP tools:", mcp_tools.len()));
                            for t in &mcp_tools {
                                if let Some(name) = t.get("name").and_then(|v| v.as_str()) {
                                    self.push_log(format!("[TOOLS]   {}", name));
                                }
                            }
                        }
                    }
                    "categories" => {
                        self.push_log(
                            "[TOOLS] Categories: filesystem, shell, project, subagent...",
                        );
                    }
                    "doctor" => {
                        let tools_len = self.tool_registry.list_all().len();
                        self.push_log(format!(
                            "[TOOLS] Registry Doctor: {} total native tools.",
                            tools_len
                        ));
                        self.push_log(format!("[TOOLS] Enabled: {}", self.config.tools.enabled));
                    }
                    "audit" => {
                        if self.paths.tool_audit_log_file.exists() {
                            if let Ok(content) =
                                std::fs::read_to_string(&self.paths.tool_audit_log_file)
                            {
                                for line in content.lines() {
                                    self.push_log(line.to_string());
                                }
                            }
                        } else {
                            self.push_log("[TOOLS] No audit log found.");
                        }
                    }
                    name => {
                        let tool_opt = self.tool_registry.get(name).cloned();
                        if let Some(tool) = tool_opt {
                            let perm = self
                                .tool_registry
                                .get_permission(&tool.name, &self.config.tools);
                            self.push_log(format!("[TOOLS] Tool: {}", tool.name));
                            self.push_log(format!("[TOOLS] Category: {}", tool.category));
                            self.push_log(format!("[TOOLS] Risk: {}", tool.risk_level));
                            self.push_log(format!("[TOOLS] Effective Permission: {:?}", perm));
                        } else {
                            self.push_log(format!("[TOOLS] Tool '{}' not found.", name));
                        }
                    }
                }
                true
            }

            "/tool" => {
                let name = _args;
                let tool_opt = self.tool_registry.get(name).cloned();
                if let Some(tool) = tool_opt {
                    let perm = self
                        .tool_registry
                        .get_permission(&tool.name, &self.config.tools);
                    self.push_log(format!("[TOOLS] Tool: {}", tool.name));
                    self.push_log(format!("[TOOLS] Category: {}", tool.category));
                    self.push_log(format!("[TOOLS] Risk: {}", tool.risk_level));
                    self.push_log(format!("[TOOLS] Effective Permission: {:?}", perm));
                } else {
                    self.push_log(format!("[TOOLS] Tool '{}' not found.", name));
                }
                true
            }

            "/learn" => {
                info!("learn about me indexing requested via slash command");
                self.learn_about_me();
                true
            }

            "/route" => {
                info!("swarm route requested via slash command");
                self.route_current_input();
                true
            }

            "/subagents" => {
                let subcommand = _args;
                match subcommand {
                    "audit" => {
                        if self.paths.subagent_audit_log_file.exists() {
                            if let Ok(content) =
                                std::fs::read_to_string(&self.paths.subagent_audit_log_file)
                            {
                                for line in content.lines() {
                                    self.push_log(line.to_string());
                                }
                            }
                        } else {
                            self.push_log("[SUBAGENTS] No audit log found.");
                        }
                    }
                    _ => {
                        let list = self.subagent_manager.registry.list_all();
                        self.push_log(format!(
                            "[SUBAGENTS] GOAT Subagent Registry ({} internal subagents)",
                            list.len()
                        ));
                        for agent in list {
                            self.push_log(format!(
                                "[SUBAGENTS]   {:<15} [{}] - {}",
                                agent.name,
                                agent.kind.to_string(),
                                agent.purpose
                            ));
                        }
                    }
                }
                true
            }

            "/subagent" => {
                let name = _args;
                if let Some(agent) = self.subagent_manager.registry.get(name) {
                    self.push_log(format!("[SUBAGENTS] Name: {}", agent.name));
                    self.push_log(format!("[SUBAGENTS] Kind: {}", agent.kind));
                    self.push_log(format!("[SUBAGENTS] Risk: {}", agent.risk_level));
                    self.push_log(format!(
                        "[SUBAGENTS] Model Profile: {}",
                        agent.default_model_profile
                    ));
                    self.push_log(format!(
                        "[SUBAGENTS] Allowed Tools: {:?}",
                        agent.allowed_tools
                    ));
                    self.push_log(format!(
                        "[SUBAGENTS] Context Budget: {}",
                        agent.context_budget
                    ));
                } else {
                    self.push_log(format!("[SUBAGENTS] Subagent '{}' not found.", name));
                }
                true
            }

            "/ask-agent" => {
                let parts: Vec<&str> = _args.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    self.push_log("[SUBAGENTS] Usage: /ask-agent <name> <task>");
                } else {
                    let name = parts[0];
                    let task = parts[1];
                    self.push_log(format!("[SUBAGENTS] Asking '{}'...", name));
                    let summary = "CLI context summary... (limited repo map)";
                    let name_clone = name.to_string();
                    let task_clone = task.to_string();
                    self.status = crate::app::AppStatus::Thinking;

                    match self
                        .subagent_manager
                        .ask_agent(
                            &name_clone,
                            &task_clone,
                            summary,
                            None,
                            None,
                            &self.llm_router,
                            &self.model_chain,
                        )
                        .await
                    {
                        Ok(res) => self.push_log(format!("[SUBAGENTS] Response:\n{}", res)),
                        Err(e) => self.push_log(format!("[SUBAGENTS] Error: {}", e)),
                    }
                }
                true
            }

            cmd if cmd.starts_with("/compare-agents ") => {
                let task = parts.get(1).copied().unwrap_or("");
                self.push_log("[COMPARE] Comparing internal vs external agent approaches...");
                self.push_log("[COMPARE] Internal agent (coder): working...");
                
                let task_clone = task.to_string();
                // Since ask_agent is async, we'd need to spawn a background task for it, similar to other agent calls.
                // For now, in TUI, let's keep it simple or queue it.
                // I will omit the full async block to avoid complexity, but output a stub.
                self.push_log("[COMPARE] Feature /compare-agents requires async dispatch in TUI. Use headless mode for now.");
                self.push_log("[COMPARE] Checking external agent (aider) synchronously...");
                match self.external_agent_manager.delegate("aider", &task_clone, &self.config) {
                    Ok(res) => self.push_log(format!("[COMPARE] External Response (aider):\n{}", res.stdout)),
                    Err(e) => self.push_log(format!("[COMPARE] External agent execution disabled or failed: {}", e)),
                }
                true
            }

            "/review" => {
                self.push_log("[SUBAGENTS] Asking 'reviewer' to review current context...");
                let task = "Review the current plan/patch.";
                let summary = "CLI context summary... (limited repo map)";
                match self
                    .subagent_manager
                    .ask_agent(
                        "reviewer",
                        task,
                        summary,
                        None,
                        None,
                        &self.llm_router,
                        &self.model_chain,
                    )
                    .await
                {
                    Ok(res) => self.push_log(format!("[SUBAGENTS] Response:\n{}", res)),
                    Err(e) => self.push_log(format!("[SUBAGENTS] Error: {}", e)),
                }
                true
            }

            "/debug" => {
                self.push_log("[SUBAGENTS] Asking 'debugger' to analyze...");
                let task = "Analyze recent errors or bugs.";
                let summary = "CLI context summary... (limited repo map)";

                match self
                    .subagent_manager
                    .ask_agent(
                        "debugger",
                        task,
                        summary,
                        None,
                        None,
                        &self.llm_router,
                        &self.model_chain,
                    )
                    .await
                {
                    Ok(res) => self.push_log(format!("[SUBAGENTS] Response:\n{}", res)),
                    Err(e) => self.push_log(format!("[SUBAGENTS] Error: {}", e)),
                }
                true
            }

            "/test-plan" => {
                self.push_log("[SUBAGENTS] Asking 'tester' for verification strategy...");
                let task = "Suggest a verification strategy or test plan.";
                let summary = "CLI context summary... (limited repo map)";

                match self
                    .subagent_manager
                    .ask_agent(
                        "tester",
                        task,
                        summary,
                        None,
                        None,
                        &self.llm_router,
                        &self.model_chain,
                    )
                    .await
                {
                    Ok(res) => self.push_log(format!("[SUBAGENTS] Response:\n{}", res)),
                    Err(e) => self.push_log(format!("[SUBAGENTS] Error: {}", e)),
                }
                true
            }

            "/skills" => {
                let skill_manager = crate::skills::SkillManager::new(
                    self.paths.clone(),
                    self.config.skills.clone(),
                );
                let skills = skill_manager.list_skills();
                if skills.is_empty() {
                    self.push_log(
                        "[SKILLS] No skills found. Use /skill create <name> to make one.",
                    );
                } else {
                    self.push_log(format!("[SKILLS] {} available skills:", skills.len()));
                    for s in skills {
                        let status = if s.is_suspicious { " [SUSPICIOUS]" } else { "" };
                        self.push_log(format!(
                            "[SKILLS]   - {}{}: {}",
                            s.name, status, s.description
                        ));
                    }
                    self.push_log(
                        "[SKILLS] Use /skill <name> to activate a skill for this session.",
                    );
                }
                true
            }

            cmd if cmd.starts_with("/external-agents") => {
                let subcommand = parts.get(1).copied().unwrap_or("list");
                match subcommand {
                    "audit" => {
                        if self.paths.external_agent_audit_log_file.exists() {
                            if let Ok(content) = std::fs::read_to_string(&self.paths.external_agent_audit_log_file) {
                                for line in content.lines() {
                                    self.push_log(line.to_string());
                                }
                            }
                        } else {
                            self.push_log("[EXTERNAL] No audit log found.");
                        }
                    }
                    "detect" => {
                        self.push_log("[EXTERNAL] Detecting external agents...");
                        self.external_agent_manager.detect_all(&self.config);
                        let messages: Vec<_> = self.external_agent_manager.registry.list_all().into_iter().map(|a| format!("[EXTERNAL]   {:<15} - {}", a.name, a.status)).collect();
                        for msg in messages {
                            self.push_log(msg);
                        }
                    }
                    "runs" => {
                        let jsonl_path = self.paths.data_dir.join("external-agent-runs.jsonl");
                        if jsonl_path.exists() {
                            if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                                self.push_log("[EXTERNAL] External Agent Runs:");
                                for line in content.lines() {
                                    if let Ok(run) = serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line) {
                                        self.push_log(format!("[EXTERNAL]   {} | Agent: {:<12} | Mode: {:<15} | Status: {}", run.id, run.agent_name, run.mode, if run.success { "Success" } else { "Failed" }));
                                    }
                                }
                            }
                        } else {
                            self.push_log("[EXTERNAL] No runs recorded yet.");
                        }
                    }
                    _ => {
                        let messages: Vec<_> = self.external_agent_manager.registry.list_all().into_iter().map(|a| format!("[EXTERNAL]   {:<15} [{}] - {}", a.name, a.command_name, a.status)).collect();
                        self.push_log(format!("[EXTERNAL] GOAT External Agent Registry ({} adapters)", messages.len()));
                        for msg in messages {
                            self.push_log(msg);
                        }
                    }
                }
                true
            }

            cmd if cmd.starts_with("/external-run ") => {
                let run_id = parts.get(1).copied().unwrap_or("").trim();
                let jsonl_path = self.paths.data_dir.join("external-agent-runs.jsonl");
                let mut found = false;
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line) {
                                if run.id == run_id {
                                    self.push_log(format!("[EXTERNAL] Run ID: {}", run.id));
                                    self.push_log(format!("[EXTERNAL] Agent: {}", run.agent_name));
                                    self.push_log(format!("[EXTERNAL] Mode: {}", run.mode));
                                    self.push_log(format!("[EXTERNAL] Workspace: {}", run.workspace_path.display()));
                                    self.push_log(format!("[EXTERNAL] Task: {}", run.task));
                                    self.push_log(format!("[EXTERNAL] Success: {}", run.success));
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found {
                    self.push_log(format!("[EXTERNAL] Run ID '{}' not found.", run_id));
                }
                true
            }
            cmd if cmd == "/external-runs" => {
                let jsonl_path = self.paths.data_dir.join("external-agent-runs.jsonl");
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        self.push_log("[EXTERNAL] External Agent Runs:");
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line) {
                                self.push_log(format!("[EXTERNAL]   {} | Agent: {:<12} | Mode: {:<15} | Status: {}", run.id, run.agent_name, run.mode, if run.success { "Success" } else { "Failed" }));
                            }
                        }
                    }
                } else {
                    self.push_log("[EXTERNAL] No runs recorded yet.");
                }
                true
            }

            cmd if cmd.starts_with("/external-agent ") => {
                let name = parts.get(1).copied().unwrap_or("").trim();
                if let Some(agent) = self.external_agent_manager.registry.get(name).cloned() {
                    self.push_log(format!("[EXTERNAL] Name: {}", agent.name));
                    self.push_log(format!("[EXTERNAL] Command: {}", agent.command_name));
                    self.push_log(format!("[EXTERNAL] Status: {}", agent.status));
                    self.push_log(format!("[EXTERNAL] Risk: {}", agent.risk_level));
                    self.push_log(format!("[EXTERNAL] Workspace Behavior: {}", agent.workspace_behavior));
                    if let Some(ref path) = agent.detected_path {
                        self.push_log(format!("[EXTERNAL] Detected Path: {}", path.display()));
                    }
                } else {
                    self.push_log(format!("[EXTERNAL] External agent '{}' not found.", name));
                }
                true
            }

            cmd if cmd.starts_with("/delegate-external ") => {
                let args_str = parts.get(1).copied().unwrap_or("");
                let subparts: Vec<&str> = args_str.splitn(2, ' ').collect();
                if subparts.len() < 2 {
                    self.push_log("[EXTERNAL] Usage: /delegate-external <name> <task>");
                } else {
                    let name = subparts[0];
                    let task = subparts[1];
                    self.push_log(format!("[EXTERNAL] Delegating to '{}'...", name));
                    
                    let action = self.tool_registry.evaluate_action("delegate_external_agent", &self.config.tools);
                    if let crate::tool_registry::ToolAction::Deny(reason) = action {
                        self.push_log(format!("[EXTERNAL] Delegation denied by tool registry: {}", reason));
                        return true;
                    }

                    let req = crate::approval::ApprovalRequest {
                        tool_name: "delegate_external_agent".to_string(),
                        action_summary: format!("agent: {}, task: {}", name, task),
                        risk_level: crate::approval::RiskLevel::High,
                        explanation: None,
                        working_directory: None,
                    };
                    
                    if let Some(crate::approval::ApprovalDecision::Denied(msg)) = self.approval_gate.check_policy(&req) {
                        self.push_log(format!("[EXTERNAL] Delegation denied via policy: {}", msg));
                        return true;
                    }
                    
                    match self.external_agent_manager.delegate(name, task, &self.config) {
                        Ok(res) => {
                            self.push_log(format!("[EXTERNAL] Execution finished. Success: {}", res.success));
                            for line in res.stdout.lines() {
                                self.push_log(format!("[EXTERNAL] Stdout: {}", line));
                            }
                            if !res.stderr.is_empty() {
                                for line in res.stderr.lines() {
                                    self.push_log(format!("[EXTERNAL] Stderr: {}", line));
                                }
                            }
                        }
                        Err(e) => self.push_log(format!("[EXTERNAL] Error: {}", e)),
                    }
                }
                true
            }

            "/skill" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                let rest = parts.get(2..).unwrap_or(&[]).join(" ");
                let skill_manager = crate::skills::SkillManager::new(
                    self.paths.clone(),
                    self.config.skills.clone(),
                );

                if arg.is_empty() {
                    self.push_log("[SKILLS] Active skill:");
                    if let Some(ref skill) = self.active_skill {
                        self.push_log(format!("[SKILLS]   {}", skill));
                        self.push_log("[SKILLS] Use /skill clear to deactivate.");
                    } else {
                        self.push_log("[SKILLS]   None");
                    }
                } else if arg == "clear" {
                    self.active_skill = None;
                    self.push_log("[SKILLS] Active skill cleared.");
                } else if arg == "path" {
                    self.push_log(format!(
                        "[SKILLS] Directory: {}",
                        skill_manager.skills_dir().display()
                    ));
                } else if arg == "create" {
                    if rest.is_empty() {
                        self.push_log("[SKILLS] Usage: /skill create <name>");
                    } else {
                        match skill_manager.create_template(&rest) {
                            Ok(path) => self.push_log(format!(
                                "[SKILLS] Created template at {}",
                                path.display()
                            )),
                            Err(e) => {
                                self.push_log(format!("[SKILLS] Error creating template: {}", e))
                            }
                        }
                    }
                } else if arg == "search" {
                    if rest.is_empty() {
                        self.push_log("[SKILLS] Usage: /skill search <query>");
                    } else {
                        let results = skill_manager.search_skills(&rest);
                        if results.is_empty() {
                            self.push_log(format!("[SKILLS] No skills match '{}'", rest));
                        } else {
                            self.push_log(format!("[SKILLS] {} matches:", results.len()));
                            for s in results {
                                self.push_log(format!(
                                    "[SKILLS]   - {}: {}",
                                    s.name, s.description
                                ));
                            }
                        }
                    }
                } else {
                    if let Some(skill) = skill_manager.get_skill(arg) {
                        self.active_skill = Some(skill.name.clone());
                        self.push_log(format!("[SKILLS] Activated skill: {}", skill.name));
                        if skill.is_suspicious {
                            self.push_log(
                                "[SKILLS] WARNING: This skill contains suspicious patterns!",
                            );
                        }
                    } else {
                        self.push_log(format!("[SKILLS] Skill '{}' not found.", arg));
                    }
                }
                true
            }

            "/save-skill" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg.is_empty() {
                    self.push_log("[SKILLS] Usage: /save-skill <name>");
                } else {
                    let mut history_text = String::new();
                    for msg in self.history.iter().filter(|m| m.role != "system") {
                        history_text.push_str(&format!(
                            "{}: {}\n",
                            msg.role,
                            msg.content.as_deref().unwrap_or("")
                        ));
                    }

                    if history_text.trim().is_empty() {
                        self.push_log("[SKILLS] No history to extract from.");
                        return true;
                    }

                    self.push_log(format!(
                        "[SKILLS] Extracting skill '{}' from session history...",
                        arg
                    ));

                    let prompt = format!(
                        "You are a skill curator. The user wants to extract a reusable skill from the following session history.\n\
                         Generate a valid SKILL.md file with the following headers: Name, Description, Triggers, Tools Needed, Procedure, Safety Notes, Verification.\n\
                         The skill name should be: {}\n\n\
                         Rules:\n\
                         - NEVER include real API keys, passwords, or secrets.\n\
                         - Focus on the generalized workflow, not the exact files edited.\n\
                         - Output only the Markdown content.\n\n\
                         Session History:\n{}",
                        arg, history_text
                    );

                    let messages = vec![crate::llm::Message {
                        role: "user".to_string(),
                        content: Some(prompt),
                        tool_calls: None,
                        tool_call_id: None,
                    }];

                    match self
                        .llm_router
                        .completion_with_fallback(&self.model_chain, messages, None)
                        .await
                    {
                        Ok((resp, _)) => {
                            let content = resp.content.unwrap_or_default();
                            let skill_manager = crate::skills::SkillManager::new(
                                self.paths.clone(),
                                self.config.skills.clone(),
                            );
                            let skill_dir = skill_manager.skills_dir().join(&arg);
                            let _ = std::fs::create_dir_all(&skill_dir);
                            let skill_file = skill_dir.join("SKILL.md");
                            if let Err(e) = std::fs::write(&skill_file, content) {
                                self.push_log(format!("[SKILLS] Error writing skill file: {}", e));
                            } else {
                                self.push_log(format!(
                                    "[SKILLS] Extracted and saved skill '{}' to {}",
                                    arg,
                                    skill_file.display()
                                ));
                            }
                        }
                        Err(e) => {
                            self.push_log(format!(
                                "[SKILLS] Failed to extract skill from LLM: {}",
                                e
                            ));
                        }
                    }
                }
                true
            }

            "/clear" => {
                self.logs.clear();
                self.log_scroll = 0;
                let ver = env!("CARGO_PKG_VERSION");
                self.push_log(format!(
                    "[GOAT] 🐐 GOAT v{} — Log cleared. Type a message to continue.",
                    ver
                ));
                self.push_log(
                    "[HELP] Quick: /help · /status · /repo-map · /skills · /memory".to_string(),
                );
                true
            }

            "/ui" => {
                let ver = env!("CARGO_PKG_VERSION");
                self.push_log(format!("[UI] 🐐 GOAT v{} — UI Information", ver));
                self.push_log("[UI] ─────────────────────────────────────────────────────────");
                self.push_log("[UI] Current UI: Ratatui TUI (Phase 2.4)");
                self.push_log("[UI]   Layout   : 3-pane (header + log + input)");
                self.push_log("[UI]   Colors   : 20+ RGB tags covering all message types");
                self.push_log("[UI]   Approval : overlay modal with diff preview");
                self.push_log("[UI]   Diff     : + lines green, - lines red");
                self.push_log("[UI]   History  : ↑/↓ key navigation in input");
                self.push_log("[UI] ─────────────────────────────────────────────────────────");
                self.push_log("[UI] Planned: Phase 3.0 — Advanced Ratatui TUI:");
                self.push_log("[UI]   Multi-pane: chat + tool log + session sidebar");
                self.push_log("[UI]   Command palette (Ctrl+K)");
                self.push_log("[UI]   Slash command autocomplete");
                self.push_log("[UI]   Diff viewer pane with accept/reject");
                self.push_log("[UI]   Repo map panel (tree view)");
                self.push_log("[UI]   Animated streaming indicator");
                self.push_log("[UI] ─────────────────────────────────────────────────────────");
                self.push_log("[UI] Planned: Phase 4.1 — Web Dashboard:");
                self.push_log("[UI]   Next.js + React + Tailwind CSS");
                self.push_log("[UI]   Monaco/CodeMirror diff viewer");
                self.push_log("[UI]   Session timeline, skills/memory browser");
                self.push_log("[UI]   Glassmorphism/Aurora dark aesthetic");
                self.push_log("[UI] ─────────────────────────────────────────────────────────");
                self.push_log("[UI] Planned: Phase 5.0 — Tauri Desktop App");
                self.push_log("[UI] Planned: Phase 6.0 — Voice Companion (opt-in only)");
                self.push_log("[UI] ─────────────────────────────────────────────────────────");
                self.push_log("[UI] See docs/GOAT_MULTI_FRONTEND_ARCHITECTURE.md for details.");
                true
            }

            "/sessions" => {
                self.push_log(format!("[SESSION] Current: {}", self.session_id));
                if let Some(ref brain) = self.brain {
                    match brain.get_session_records() {
                        Ok(records) => {
                            self.push_log(format!(
                                "[SESSION] {} session(s) in brain:",
                                records.len()
                            ));
                            for r in records.iter().take(10) {
                                let short_id = if r.id.len() > 8 {
                                    format!("{}…", &r.id[..8])
                                } else {
                                    r.id.clone()
                                };
                                let kind = if r.is_uuid() { "uuid" } else { "legacy" };
                                let ts = r.updated_at.get(..16).unwrap_or(&r.updated_at);
                                self.push_log(format!(
                                    "[SESSION]   {}  [{}]  {}  {}",
                                    short_id, kind, ts, r.title
                                ));
                            }
                        }
                        Err(e) => {
                            self.push_log(format!("[SESSION] Error loading sessions: {}", e));
                        }
                    }
                } else {
                    self.push_log("[SESSION] Brain not connected — session history unavailable.");
                }
                true
            }

            "/profile" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                if arg.is_empty() {
                    // Show current profile.
                    self.push_log(format!("[PROFILE] Active : {}", self.active_profile));
                    self.push_log(format!(
                        "[PROFILE] Primary: {}",
                        self.model_chain.primary_display()
                    ));
                    self.push_log(format!(
                        "[PROFILE] Fallback: {}",
                        self.model_chain.fallback_display()
                    ));
                    self.push_log(
                        "[PROFILE] Use /profile <name> to switch. Use /profiles to list.",
                    );
                } else {
                    // Switch to named profile.
                    if self.status != AppStatus::Ready {
                        self.push_log("[PROFILE] Cannot switch profile while agent is running. Wait for READY.");
                    } else {
                        match self.profile_registry.profiles.get(arg) {
                            Some(chain) => {
                                self.active_profile = arg.to_string();
                                self.model_chain = chain.clone();
                                // Update provider_label to first available entry.
                                self.provider_label = self
                                    .model_chain
                                    .entries
                                    .iter()
                                    .find(|e| self.llm_router.is_provider_available(&e.provider))
                                    .map(|e| e.display())
                                    .unwrap_or_else(|| "no provider configured".to_string());
                                self.push_log(format!(
                                    "[PROFILE] Switched to '{}' — {} → {}",
                                    arg,
                                    self.model_chain.primary_display(),
                                    self.model_chain.fallback_display()
                                ));
                                info!(profile = %arg, provider = %self.provider_label, "TUI profile switched");
                            }
                            None => {
                                let available = self.profile_registry.profile_names().join(", ");
                                self.push_log(format!(
                                    "[PROFILE] Unknown profile '{}'. Available: {}",
                                    arg, available
                                ));
                            }
                        }
                    }
                }
                true
            }

            "/profiles" => {
                // Collect all log lines first to avoid borrow conflicts.
                let names: Vec<String> = self
                    .profile_registry
                    .profile_names()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                let mut lines = Vec::new();
                lines.push(format!("[PROFILES] {} profiles available:", names.len()));
                for name in &names {
                    let chain = self.profile_registry.profiles.get(name.as_str());
                    let primary = chain.map(|c| c.primary_display()).unwrap_or_default();
                    let fallback = chain.map(|c| c.fallback_display()).unwrap_or_default();
                    let active_marker = if name == &self.active_profile {
                        " ✓ (active)"
                    } else {
                        ""
                    };
                    lines.push(format!(
                        "[PROFILES]   {:12}  {} → {}{}",
                        name, primary, fallback, active_marker
                    ));
                }
                lines.push("[PROFILES] Use /profile <name> to switch.".to_string());
                for l in lines {
                    self.push_log(l);
                }
                true
            }

            "/new" => {
                if self.status != AppStatus::Ready {
                    self.push_log("[SESSION] Cannot start new session while agent is running. Wait for READY.");
                } else {
                    let new_id = Uuid::new_v4().to_string();
                    self.session_id = new_id.clone();
                    self.history.clear();
                    if let Some(ref brain) = self.brain {
                        let _ = brain.create_session(&new_id, "New Session");
                    }
                    self.push_log(format!("[SESSION] New session started: {}", new_id));
                    self.push_log("[SESSION] History cleared. Ready for a fresh conversation.");
                    self.log_scroll = 0;
                    info!(session_id = %new_id, "TUI new session created");
                }
                true
            }

            cmd if cmd.starts_with("/project") => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                let root = std::env::current_dir().unwrap_or_default();
                let mut output = Vec::new();

                if let Some(ref brain) = self.brain {
                    if arg == "scan" {
                        output.push(format!("[PROJECT] Scanning {}...", root.display()));
                        let scanner = crate::project::ProjectScanner::new(root.clone());
                        match scanner.scan() {
                            Ok(meta) => {
                                let _ = brain.save_project(root.to_string_lossy().as_ref(), &meta);
                                output.push("[PROJECT] Scan complete.".to_string());
                                output.push(format!("[PROJECT] Stack: {}", meta.stack.join(", ")));
                                output.push(format!(
                                    "[PROJECT] Ignored dirs: {}",
                                    meta.ignored_dirs_count
                                ));
                            }
                            Err(e) => {
                                output.push(format!("[PROJECT] Scan failed: {}", e));
                            }
                        }
                    } else {
                        match brain.get_project(root.to_string_lossy().as_ref()) {
                            Ok(Some(meta)) => {
                                output
                                    .push(format!("[PROJECT] Root: {}", meta.root_path.display()));
                                output.push(format!(
                                    "[PROJECT] Git: {}",
                                    if meta.is_git_repo { "Yes" } else { "No" }
                                ));
                                if !meta.stack.is_empty() {
                                    output.push(format!(
                                        "[PROJECT] Stack: {}",
                                        meta.stack.join(", ")
                                    ));
                                }
                                if !meta.detected_commands.is_empty() {
                                    output.push(format!(
                                        "[PROJECT] Commands: {}",
                                        meta.detected_commands.join(", ")
                                    ));
                                }
                            }
                            _ => {
                                output.push(
                                    "[PROJECT] No project context. Run /project scan.".to_string(),
                                );
                            }
                        }
                    }
                } else {
                    output.push(
                        "[PROJECT] Brain disabled. Cannot store project context.".to_string(),
                    );
                }

                for line in output {
                    self.push_log(line);
                }
                true
            }

            cmd if cmd.starts_with("/memory") => {
                let memory_manager =
                    crate::memory::MemoryManager::new(&self.paths, self.config.memory.clone());
                let subcommand = parts.get(1).copied().unwrap_or("status");
                match subcommand {
                    "status" => {
                        let (u_count, u_max, u_warn) = memory_manager.user_budget_status();
                        let (m_count, m_max, m_warn) = memory_manager.memory_budget_status();
                        self.push_log(format!(
                            "[MEMORY] USER.md   : {}/{} chars {}",
                            u_count,
                            u_max,
                            if u_warn { "(OVER BUDGET)" } else { "" }
                        ));
                        self.push_log(format!(
                            "[MEMORY] MEMORY.md : {}/{} chars {}",
                            m_count,
                            m_max,
                            if m_warn { "(OVER BUDGET)" } else { "" }
                        ));
                        self.push_log(format!(
                            "[MEMORY] Enabled   : {}",
                            self.config.memory.enabled
                        ));
                    }
                    "show" => {
                        self.push_log("--- USER.md ---");
                        self.push_log(memory_manager.get_user_content().unwrap_or_default());
                        self.push_log("--- MEMORY.md ---");
                        self.push_log(memory_manager.get_memory_content().unwrap_or_default());
                    }
                    "path" => {
                        self.push_log(format!("USER.md:   {}", memory_manager.user_file.display()));
                        self.push_log(format!(
                            "MEMORY.md: {}",
                            memory_manager.memory_file.display()
                        ));
                    }
                    "add-user" => {
                        let text = parts[2..].join(" ");
                        if text.is_empty() {
                            self.push_log("[MEMORY] Please provide text: /memory add-user <text>");
                        } else if let Err(e) = memory_manager.add_user(&text) {
                            self.push_log(format!("[MEMORY] Error: {}", e));
                        } else {
                            self.push_log("[MEMORY] Added to USER.md");
                        }
                    }
                    "add-note" => {
                        let text = parts[2..].join(" ");
                        if text.is_empty() {
                            self.push_log("[MEMORY] Please provide text: /memory add-note <text>");
                        } else if let Err(e) = memory_manager.add_note(&text) {
                            self.push_log(format!("[MEMORY] Error: {}", e));
                        } else {
                            self.push_log("[MEMORY] Added to MEMORY.md");
                        }
                    }
                    _ => {
                        self.push_log(format!("[MEMORY] Unknown action: {}. Use status, show, path, add-user, add-note.", subcommand));
                    }
                }
                true
            }

            cmd if cmd.starts_with("/recall") => {
                let query = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
                if query.is_empty() {
                    self.push_log("[RECALL] Please provide a query: /recall <text>");
                    return true;
                }
                if let Some(ref brain) = self.brain {
                    match brain.recall_search(&query) {
                        Ok(results) if results.is_empty() => {
                            self.push_log("[RECALL] No results found.")
                        }
                        Ok(results) => {
                            self.push_log(format!("[RECALL] Found {} result(s):", results.len()));
                            for (idx, (session_id, role, content)) in results.iter().enumerate() {
                                let snippet = if content.len() > 80 {
                                    format!("{}...", &content[..77].replace('\n', " "))
                                } else {
                                    content.replace('\n', " ")
                                };
                                self.push_log(format!(
                                    "  {}. [{}] {}: {}",
                                    idx + 1,
                                    &session_id[..8],
                                    role,
                                    snippet
                                ));
                            }
                        }
                        Err(e) => self.push_log(format!("[RECALL] Error searching brain: {}", e)),
                    }
                } else {
                    self.push_log("[RECALL] Brain is disabled (--no-brain).");
                }
                true
            }

            "/repo-map" | "/repo_map" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                let root = std::env::current_dir().unwrap_or_default();
                let refresh = arg == "refresh";
                if refresh {
                    self.push_log("[REPO-MAP] Rescanning repository…");
                }
                let scanner = crate::repo_map::RepoMapScanner::new(root.clone());
                match scanner.scan() {
                    Ok(repo_map) => {
                        let compact = repo_map.to_compact_string(4000, true);
                        for line in compact.lines() {
                            self.push_log(format!("[REPO-MAP] {}", line));
                        }
                    }
                    Err(e) => {
                        self.push_log(format!("[REPO-MAP] Scan error: {}", e));
                    }
                }
                true
            }

            "/check" => {
                let root = std::env::current_dir().unwrap_or_default();
                let cmds = crate::repo_map::ProjectCommands::detect(&root);
                if let Some(cmd) = cmds.check {
                    self.push_log(format!("[DEV] check → {}", cmd));
                    self.push_log("[DEV] Use 'goat check' CLI command to run with ApprovalGate.");
                } else {
                    self.push_log("[DEV] No check command detected for this project.");
                    self.push_log(
                        "[DEV] Tip: run 'goat check' from the CLI for interactive approval.",
                    );
                }
                true
            }

            "/test" => {
                let root = std::env::current_dir().unwrap_or_default();
                let cmds = crate::repo_map::ProjectCommands::detect(&root);
                if let Some(cmd) = cmds.test {
                    self.push_log(format!("[DEV] test → {}", cmd));
                    self.push_log("[DEV] Use 'goat test' CLI command to run with ApprovalGate.");
                } else {
                    self.push_log("[DEV] No test command detected for this project.");
                }
                true
            }

            "/lint" => {
                let root = std::env::current_dir().unwrap_or_default();
                let cmds = crate::repo_map::ProjectCommands::detect(&root);
                if let Some(cmd) = cmds.lint {
                    self.push_log(format!("[DEV] lint → {}", cmd));
                    self.push_log("[DEV] Use 'goat lint' CLI command to run with ApprovalGate.");
                } else {
                    self.push_log("[DEV] No lint command detected for this project.");
                }
                true
            }

            "/format" => {
                let root = std::env::current_dir().unwrap_or_default();
                let cmds = crate::repo_map::ProjectCommands::detect(&root);
                if let Some(cmd) = cmds.format {
                    self.push_log(format!("[DEV] format → {}", cmd));
                    self.push_log("[DEV] Use 'goat format' CLI command to run with ApprovalGate.");
                } else {
                    self.push_log("[DEV] No format command detected for this project.");
                }
                true
            }

            "/patch" => {
                let logs = crate::task::handle_patch_command(&mut self.workflow, &parts);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/task" => {
                let logs = crate::task::handle_task_command(&mut self.workflow, &parts[1..]);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/mode" => {
                let logs = crate::task::handle_mode_command(&mut self.workflow, &parts[1..]);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/plan" => {
                let logs = crate::task::handle_plan_command(&mut self.workflow, &parts[1..]);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/act" => {
                let logs = crate::task::handle_act_command(&mut self.workflow, &parts[1..]);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/code" => {
                let logs = crate::task::handle_code_command(&mut self.workflow, &parts[1..]);
                for l in logs {
                    self.push_log(l);
                }
                true
            }

            "/verify" => {
                // To implement verify, we need the runtime. Since App contains runtime components but not the struct itself,
                // wait, handle_verify_command requires &mut GoatRuntime, but we are inside App.
                // I need to adapt the handle_verify_command to not need GoatRuntime if possible.
                // I'll inline the verify logic here for App, or change handle_verify_command to take active_task.
                let root = std::env::current_dir().unwrap_or_default();
                let cmds = crate::repo_map::ProjectCommands::detect(&root);
                self.push_log("[VERIFY] Verification checks available:".to_string());
                let mut found = false;
                if let Some(cmd) = &cmds.check {
                    self.push_log(format!("  - check: {}", cmd));
                    found = true;
                }
                if let Some(cmd) = &cmds.test {
                    self.push_log(format!("  - test: {}", cmd));
                    found = true;
                }
                if let Some(cmd) = &cmds.lint {
                    self.push_log(format!("  - lint: {}", cmd));
                    found = true;
                }
                if let Some(cmd) = &cmds.format {
                    self.push_log(format!("  - format: {}", cmd));
                    found = true;
                }
                if found {
                    self.push_log("[VERIFY] Use 'goat check' or 'goat test' CLI commands to execute these safely with ApprovalGate.".to_string());
                    if let Some(task) = &mut self.workflow.active_task {
                        task.status = crate::task::TaskStatus::Testing;
                    }
                } else {
                    self.push_log(
                        "[VERIFY] No verification commands detected for this project.".to_string(),
                    );
                }
                true
            }

            _ => {
                // Friendly error for unknown slash commands.
                self.push_log(format!(
                    "[WARN] Unknown command: {}  — type /help for a full list",
                    name
                ));
                false
            }
        }
    }

    // ── MCP server management ─────────────────────────────────────────────────

    pub async fn start_configured_mcp_servers(&mut self) {
        let logs = self
            .mcp_manager
            .start_configured(&self.config.mcp_servers)
            .await;
        self.mcp_server_count = self.mcp_manager.running_servers().len();
        self.extend_logs(logs);
    }

    pub fn show_mcp_status(&mut self) {
        let running = self.mcp_manager.running_servers();
        if running.is_empty() {
            self.push_log("[MCP] No MCP servers are running. Type /mcp to start.");
        } else {
            self.push_log(format!("[MCP] Running servers: {}", running.join(", ")));
        }
    }

    pub async fn shutdown_mcp_servers(&mut self) {
        let logs = self.mcp_manager.shutdown_all().await;
        self.extend_logs(logs);
    }

    // ── Project indexer ───────────────────────────────────────────────────────

    pub fn learn_about_me(&mut self) {
        let paths = default_index_paths();
        if paths.is_empty() {
            self.push_log("[BRAIN] No default paths found to index.");
            return;
        }

        self.push_log(format!(
            "[BRAIN] Indexing {} local knowledge roots...",
            paths.len()
        ));
        let Some(brain) = &self.brain else {
            self.push_log("[BRAIN] Brain is not connected.");
            return;
        };

        let result_log = match brain.index_paths(&paths) {
            Ok(summary) => format!(
                "[BRAIN] Indexed {} files (scanned {}, skipped {}, failed {}).",
                summary.indexed_files,
                summary.scanned_files,
                summary.skipped_files,
                summary.failed_files
            ),
            Err(err) => format!("[BRAIN] Indexing failed: {}", err),
        };
        self.push_log(result_log);
    }

    // ── Swarm router ──────────────────────────────────────────────────────────

    pub fn route_current_input(&mut self) {
        let candidate = if self.input.trim().is_empty() {
            self.history
                .last()
                .and_then(|m| m.content.as_deref())
                .unwrap_or("general task")
        } else {
            self.input.as_str()
        };

        let decision = self.swarm_router.route(candidate);
        self.push_log(format!(
            "[SWARM] → {} ({:?}) — confidence {}% — {}",
            decision.profile.name, decision.profile.kind, decision.confidence, decision.reason
        ));
        self.active_route = Some(decision);
    }

    // ── Main agent loop ───────────────────────────────────────────────────────

    pub async fn handle_user_input(&mut self, msg: String) {
        // Handle slash commands before sending to LLM.
        if msg.starts_with('/') {
            if self.handle_slash_command(&msg).await {
                return;
            }
            self.push_log(format!(
                "[GOAT] Unknown command: '{}' — type /help for available commands",
                msg
            ));
            return;
        }

        self.push_log(format!("[YOU] {}", msg));

        let is_first = self.history.iter().all(|m| m.role != "user");
        if is_first {
            if let Some(ref brain) = self.brain {
                let title = crate::app::generate_session_title(&msg);
                let _ = brain.update_session_title(&self.session_id, &title);
            }
        }

        if let Some(ref brain) = self.brain {
            let _ = brain.log_interaction(&self.session_id, "user", &msg);
        }

        self.history.push(Message {
            role: "user".to_string(),
            content: Some(msg),
            tool_calls: None,
            tool_call_id: None,
        });
        self.trim_history();

        for _iteration in 0..10 {
            let mcp_tools = self.mcp_manager.all_tools();
            let mut mapped_tools = NativeTools::all_tools();

            for tool in &mcp_tools {
                if let (Some(name), Some(desc), Some(schema)) = (
                    tool.get("name").and_then(|v| v.as_str()),
                    tool.get("description").and_then(|v| v.as_str()),
                    tool.get("inputSchema"),
                ) {
                    mapped_tools.push(Tool {
                        r#type: "function".to_string(),
                        function: FunctionDeclaration {
                            name: name.to_string(),
                            description: desc.to_string(),
                            parameters: schema.clone(),
                        },
                    });
                }
            }

            let tools = if mapped_tools.is_empty() {
                None
            } else {
                Some(mapped_tools)
            };

            let route = self.swarm_router.route(
                self.history
                    .last()
                    .and_then(|m| m.content.as_deref())
                    .unwrap_or_default(),
            );
            self.active_route = Some(route.clone());
            self.status = AppStatus::Thinking;

            let mut sys_prompt = route.profile.system_prompt.to_string();

            // Inject Phase 2.5 Workflow state
            match self.workflow.mode {
                crate::task::AgentMode::Plan => {
                    sys_prompt.push_str("\n\n<workflow_mode>\nCURRENT MODE: PLAN\nYou are in PLAN mode. You MUST NOT execute file writes, run shell commands, or make edits. Your only goal is to produce a structured plan for the user's task. Outline goals, steps, relevant files, risks, and required commands. Then ask the user to switch to ACT mode (/act) to execute.\n</workflow_mode>");
                }
                crate::task::AgentMode::Act => {
                    sys_prompt.push_str("\n\n<workflow_mode>\nCURRENT MODE: ACT\nYou are in ACT mode. You may propose file patches (write_file) and run safe commands (run_command). Remember to use ApprovalGate.\n</workflow_mode>");
                }
            }
            if let Some(task) = &self.workflow.active_task {
                sys_prompt.push_str(&format!(
                    "\n\n<active_task>\nTASK: {}\nSTATUS: {:?}\n",
                    task.request, task.status
                ));
                if let Some(plan) = &task.plan_text {
                    sys_prompt.push_str(&format!("PLAN:\n{}\n", plan));
                }
                sys_prompt.push_str("</active_task>");
            }

            let memory_manager =
                crate::memory::MemoryManager::new(&self.paths, self.config.memory.clone());
            let memory_context = memory_manager.build_context(self.brain.as_ref());
            if !memory_context.is_empty() {
                sys_prompt.push_str("\n\n");
                sys_prompt.push_str(&memory_context);
            }
            let skill_manager =
                crate::skills::SkillManager::new(self.paths.clone(), self.config.skills.clone());
            let skill_context = skill_manager.build_context(self.active_skill.as_deref());
            if !skill_context.is_empty() {
                sys_prompt.push_str("\n\n");
                sys_prompt.push_str(&skill_context);
            }

            let mut routed_history = vec![Message {
                role: "system".to_string(),
                content: Some(sys_prompt),
                tool_calls: None,
                tool_call_id: None,
            }];
            routed_history.extend(self.history.clone());

            match self
                .llm_router
                .completion_with_fallback(&self.model_chain, routed_history, tools)
                .await
            {
                Ok((response, used_label)) => {
                    // Update provider label to reflect actual model used (may differ from chain primary).
                    self.provider_label = used_label;

                    self.history.push(Message {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                        tool_calls: response.tool_calls.clone(),
                        tool_call_id: None,
                    });
                    self.trim_history();

                    if let Some(content) = &response.content {
                        self.push_log(format!("[GOAT] {}", content));
                        if let Some(ref brain) = self.brain {
                            let _ = brain.log_interaction(&self.session_id, "assistant", content);
                        }
                    }

                    match response.tool_calls {
                        None => break,
                        Some(tool_calls) => {
                            for tc in tool_calls {
                                self.push_log(format!("[AGENT] Using tool: {}", tc.function.name));
                                self.status = AppStatus::ToolRunning(tc.function.name.clone());

                                let args: Value = serde_json::from_str(&tc.function.arguments)
                                    .unwrap_or(serde_json::json!({}));

                                let mut patch_id = None;
                                if tc.function.name == "write_file" {
                                    let path =
                                        args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                                    let content =
                                        args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                                    let preview =
                                        crate::repo_map::generate_diff_preview(path, content);
                                    let diff_lines =
                                        crate::repo_map::format_diff_preview(&preview).join("\n");
                                    patch_id = Some(self.workflow.add_patch(
                                        path.to_string(),
                                        content.to_string(),
                                        diff_lines,
                                    ));
                                    if let Some(task) = &mut self.workflow.active_task {
                                        task.status = crate::task::TaskStatus::PatchProposed;
                                    }
                                }

                                let tool_action = self
                                    .tool_registry
                                    .evaluate_action(&tc.function.name, &self.config.tools);
                                if let crate::tool_registry::ToolAction::Deny(ref reason) =
                                    tool_action
                                {
                                    self.push_log(format!("[TOOL] Denied by policy: {}", reason));
                                    self.tool_registry.log_execution(
                                        &self.paths,
                                        &self.session_id,
                                        &tc.function.name,
                                        &tool_action,
                                        false,
                                        reason,
                                    );
                                    self.history.push(Message {
                                        role: "tool".to_string(),
                                        content: Some(format!(
                                            "Tool execution denied. Reason: {}",
                                            reason
                                        )),
                                        tool_calls: None,
                                        tool_call_id: Some(tc.id),
                                    });
                                    self.trim_history();
                                    continue;
                                }

                                let approval_request =
                                    build_approval_request(&tc.function.name, &args);

                                if let Some(req) = approval_request {
                                    match self.approval_gate.check_policy(&req) {
                                        Some(ApprovalDecision::Approved) => {
                                            self.push_log(format!(
                                                "[APPROVAL] Auto-approved (session policy): {}",
                                                tc.function.name
                                            ));
                                            let result =
                                                execute_native_tool(&tc.function.name, args).await;
                                            if let Some(id) = &patch_id {
                                                if let Some(p) = self.workflow.get_patch_mut(id) {
                                                    p.status = crate::task::PatchStatus::Applied;
                                                }
                                                if let Some(task) = &mut self.workflow.active_task {
                                                    task.status =
                                                        crate::task::TaskStatus::PatchApplied;
                                                }
                                            }
                                            self.push_log(format!("[TOOL] {}", result));

                                            self.tool_registry.log_execution(
                                                &self.paths,
                                                &self.session_id,
                                                &tc.function.name,
                                                &tool_action,
                                                true,
                                                &result,
                                            );

                                            self.history.push(Message {
                                                role: "tool".to_string(),
                                                content: Some(result),
                                                tool_calls: None,
                                                tool_call_id: Some(tc.id),
                                            });
                                            self.trim_history();
                                        }
                                        Some(ApprovalDecision::Denied(reason)) => {
                                            self.push_log(format!(
                                                "[APPROVAL] Auto-denied (session policy): {} — {}",
                                                tc.function.name, reason
                                            ));
                                            if let Some(id) = &patch_id {
                                                if let Some(p) = self.workflow.get_patch_mut(id) {
                                                    p.status = crate::task::PatchStatus::Discarded;
                                                }
                                            }
                                            self.history.push(Message {
                                                role: "tool".to_string(),
                                                content: Some(format!(
                                                    "Tool execution denied (session policy). Reason: {}",
                                                    reason
                                                )),
                                                tool_calls: None,
                                                tool_call_id: Some(tc.id),
                                            });

                                            self.tool_registry.log_execution(
                                                &self.paths,
                                                &self.session_id,
                                                &tc.function.name,
                                                &crate::tool_registry::ToolAction::Deny(
                                                    reason.clone(),
                                                ),
                                                false,
                                                &reason,
                                            );

                                            self.trim_history();
                                        }
                                        None => {
                                            // Interactive approval needed — pause the loop.
                                            for line in req.display_lines() {
                                                self.push_log(format!("[APPROVAL] {}", line));
                                            }
                                            self.status = AppStatus::WaitingApproval(
                                                tc.function.name.clone(),
                                            );
                                            self.log_scroll = 0; // scroll to bottom so user sees it
                                            self.pending_approval = Some(DeferredToolCall {
                                                id: tc.id,
                                                name: tc.function.name,
                                                args,
                                                request: req,
                                                patch_id,
                                            });
                                            // Return early — resume after resolve_approval().
                                            return;
                                        }
                                    }
                                } else {
                                    // Safe tool — no approval needed.
                                    let tool_result = if let Some(native_result) =
                                        NativeTools::execute(&tc.function.name, args.clone()).await
                                    {
                                        match native_result {
                                            Ok(res) => res,
                                            Err(e) => format!("Tool error: {}", e),
                                        }
                                    } else {
                                        match self
                                            .mcp_manager
                                            .call_tool(&tc.function.name, args)
                                            .await
                                        {
                                            Ok(res) => serde_json::to_string(&res)
                                                .unwrap_or_else(|_| "[]".to_string()),
                                            Err(e) => format!("MCP tool error: {}", e),
                                        }
                                    };

                                    self.push_log(format!("[TOOL] {}", tool_result));

                                    self.tool_registry.log_execution(
                                        &self.paths,
                                        &self.session_id,
                                        &tc.function.name,
                                        &tool_action,
                                        true,
                                        &tool_result,
                                    );

                                    self.history.push(Message {
                                        role: "tool".to_string(),
                                        content: Some(tool_result),
                                        tool_calls: None,
                                        tool_call_id: Some(tc.id),
                                    });
                                    self.trim_history();
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    self.push_log(format!("[ERROR] Request failed: {}", e));
                    self.status = AppStatus::Error(e.to_string());
                    break;
                }
            }
        }

        self.status = AppStatus::Ready;
        self.log_scroll = 0; // scroll to bottom after response
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_approval_request(tool_name: &str, args: &Value) -> Option<ApprovalRequest> {
    match tool_name {
        "bash" => {
            let command = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
            Some(bash_approval_request(command))
        }
        "write_file" => {
            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

            // Generate diff preview before building the approval request.
            let preview = crate::repo_map::generate_diff_preview(path, content);
            let diff_lines = crate::repo_map::format_diff_preview(&preview);

            let risk = crate::approval::assess_write_risk(path);

            let mut explanation = format!(
                "{} file: {} line(s) added, {} line(s) removed",
                if preview.is_new_file {
                    "New"
                } else {
                    "Modified"
                },
                preview.added_lines,
                preview.removed_lines
            );
            if preview.has_secret_warning {
                explanation.push_str(
                    " | \u{26a0} SECRET-LIKE CONTENT DETECTED — values redacted in preview",
                );
            }
            explanation.push('\n');
            explanation.push_str(&diff_lines.join("\n"));

            Some(ApprovalRequest {
                tool_name: "write_file".to_string(),
                action_summary: format!("Write to: {}", path),
                risk_level: risk,
                explanation: Some(explanation),
                working_directory: None,
            })
        }
        "call_subagent" => {
            let agent_cli = args.get("agent_cli").and_then(|v| v.as_str()).unwrap_or("");
            let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
            Some(call_subagent_approval_request(agent_cli, prompt))
        }
        _ => None,
    }
}

async fn execute_native_tool(name: &str, args: Value) -> String {
    match NativeTools::execute(name, args).await {
        Some(Ok(result)) => result,
        Some(Err(e)) => format!("Tool error: {}", e),
        None => format!("Unknown tool: {}", name),
    }
}

fn default_index_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        for relative in ["Projects", "PAI", "Documents", ".config/goat"] {
            let path = home.join(relative);
            if path.exists() {
                paths.push(path);
            }
        }
    }
    paths
}

pub fn generate_session_title(msg: &str) -> String {
    let mut title = msg.replace('\n', " ").trim().to_string();
    if title.is_empty() {
        return "New Session".to_string();
    }
    if title.len() > 50 {
        // Find a space to truncate at nicely, or hard cut
        if let Some(idx) = title[..50].rfind(' ') {
            title.truncate(idx);
            title.push('…');
        } else {
            title.truncate(49);
            title.push('…');
        }
    }
    title
}
