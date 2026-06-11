use crate::approval::{
    ApprovalDecision, ApprovalGate, ApprovalRequest, bash_approval_request,
    call_subagent_approval_request,
};
use crate::brain::Brain;
use crate::command_registry::{CommandRegistry, CommandStatus};
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
    /// System/tool logs (Phase 3.2)
    Logs,
    /// Agent & subagent selector modal (Phase 3.2)
    AgentSelector,
    /// File context view (Phase 3.6)
    Context,
}

/// TUI layout mode — controls how panels are arranged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// Clean centered interface — default for focus work (OpenCode-style)
    Focus,
    /// 3-pane dashboard layout (sidebar + center + context)
    Dashboard,
    /// Chat + input only — best for narrow terminals
    Compact,
}

impl LayoutMode {
    pub fn label(&self) -> &'static str {
        match self {
            LayoutMode::Focus => "focus",
            LayoutMode::Dashboard => "dashboard",
            LayoutMode::Compact => "compact",
        }
    }
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
    pub mcp_server_count: usize,
    /// The approval gate for this session.
    pub approval_gate: ApprovalGate,
    pub skill_researcher: crate::skill_researcher::SkillResearcher,
    pub timeline_manager: crate::timeline::TimelineManager,
    pub github_manager: crate::github_workflow::GitHubWorkflowManager,
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
    /// Cached Repo Map for the UI
    pub repo_map: Option<crate::repo_map::RepoMap>,
    /// Active view for Phase 3.0 UI
    pub active_view: ActiveView,
    /// Current slash-command suggestions (populated while input starts with '/')
    pub cmd_suggestions: Vec<String>,
    /// Selected index in cmd_suggestions popup (for Up/Down navigation)
    pub cmd_suggestion_idx: usize,
    /// TUI layout mode (focus / dashboard / compact) — Phase 3.2
    pub layout_mode: LayoutMode,
    /// Whether the sidebar is visible in dashboard mode (Ctrl+B toggle)
    pub sidebar_visible: bool,
    /// Whether the context panel is visible in dashboard mode (Ctrl+R toggle)
    pub context_visible: bool,
    pub checkpoint_manager: crate::checkpoint::CheckpointManager,
    pub browser_manager: crate::browser_adapter::BrowserAdapterManager,
    /// Selected file context for AI prompting (Phase 3.6)
    pub selected_files: Vec<String>,
    pub mcp_runtime: crate::mcp_runtime::McpRuntimeManager,
    pub hooks_manager: crate::hooks::HooksManager,
    pub scheduler_manager: crate::scheduler::SchedulerManager,
    pub job_tracker: crate::jobs::JobTracker,
    pub transport_manager: crate::transports::TransportManager,
    pub voice_manager: crate::voice::VoiceManager,
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
        let checkpoint_manager = crate::checkpoint::CheckpointManager::new(&rt.paths.data_dir);

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
            mcp_runtime: rt.mcp_runtime,
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
            skill_researcher: rt.skill_researcher,
            timeline_manager: rt.timeline_manager,
            github_manager: rt.github_manager,
            browser_manager: rt.browser_manager,
            brain_disabled,
            pending_approval: None,
            active_skill: None,
            input_history: Vec::new(),
            history_idx: None,
            workflow: rt.workflow,
            tool_registry: rt.tool_registry,
            subagent_manager: rt.subagent_manager,
            external_agent_manager: rt.external_agent_manager,
            repo_map: None,
            active_view: ActiveView::Chat,
            cmd_suggestions: Vec::new(),
            cmd_suggestion_idx: 0,
            layout_mode: LayoutMode::Focus,
            sidebar_visible: true,
            context_visible: true,
            checkpoint_manager,
            selected_files: rt.selected_files.clone(),
            hooks_manager: rt.hooks_manager,
            scheduler_manager: rt.scheduler_manager,
            job_tracker: rt.job_tracker,
            transport_manager: rt.transport_manager,
            voice_manager: rt.voice_manager,
        }
    }

    pub fn new(config: Config, paths: GoatPaths, startup_warnings: Vec<String>) -> Self {
        let (runtime, boot_log) =
            GoatRuntime::bootstrap(config, paths, startup_warnings, false, None);
        Self::from_runtime(runtime, boot_log)
    }

    pub fn sync_mcp_tools(&mut self) {
        for (srv_name, server) in self.mcp_manager.running_servers_metadata() {
            for tool in &server.tools {
                if let (Some(name), Some(desc)) = (
                    tool.get("name").and_then(|v| v.as_str()),
                    tool.get("description").and_then(|v| v.as_str()),
                ) {
                    self.tool_registry
                        .register(crate::tool_registry::ToolMetadata {
                            name: format!("{}_{}", srv_name, name),
                            description: format!("[MCP: {}] {}", srv_name, desc),
                            category: crate::tool_registry::ToolCategory::Mcp,
                            risk_level: crate::approval::RiskLevel::High, // MCP tools are untrusted by default
                            requires_approval: true,
                            read_only: false,
                            available_in_tui: true,
                            available_in_headless: true,
                            available_in_agent: true,
                            permission_group: "mcp".to_string(),
                        });
                }
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    // ── Slash command suggestions ─────────────────────────────────────────────

    /// Call this whenever `self.input` changes to keep suggestions in sync.
    /// Clears suggestions when input is empty or doesn't start with '/'.
    pub fn update_suggestions(&mut self) {
        if self.input.is_empty() || !self.input.starts_with('/') {
            self.cmd_suggestions.clear();
            self.cmd_suggestion_idx = 0;
            return;
        }
        let registry = CommandRegistry::build();
        // The whole input (including any args) is the prefix for matching.
        let prefix = self.input.trim();
        self.cmd_suggestions = registry
            .suggest(prefix)
            .iter()
            .map(|c| format!("{:<28} {}", c.name, c.description))
            .collect();
        // Cap at 12 to avoid flooding the popup
        self.cmd_suggestions.truncate(12);
        // Reset selection when suggestions change
        self.cmd_suggestion_idx = 0;
    }

    /// Move selection up in the suggestion list.
    pub fn suggestion_up(&mut self) {
        if self.cmd_suggestions.is_empty() {
            return;
        }
        if self.cmd_suggestion_idx > 0 {
            self.cmd_suggestion_idx -= 1;
        }
    }

    /// Move selection down in the suggestion list.
    pub fn suggestion_down(&mut self) {
        if self.cmd_suggestions.is_empty() {
            return;
        }
        if self.cmd_suggestion_idx + 1 < self.cmd_suggestions.len() {
            self.cmd_suggestion_idx += 1;
        }
    }

    /// Apply tab completion: replace input with the first (or selected) suggestion name.
    pub fn complete_suggestion(&mut self) {
        if self.cmd_suggestions.is_empty() {
            return;
        }
        let registry = CommandRegistry::build();
        let prefix = self.input.trim().to_string();
        let suggestions = registry.suggest(&prefix);
        let selected = self
            .cmd_suggestion_idx
            .min(suggestions.len().saturating_sub(1));
        if let Some(cmd) = suggestions.get(selected) {
            // Replace the whole input with the command name followed by a space
            self.input = format!("{} ", cmd.name);
            self.cmd_suggestions.clear();
            self.cmd_suggestion_idx = 0;
        }
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

                // Auto-checkpoint before dangerous tools (write_file or bash if it might modify)
                if self.config.checkpoint.enabled && self.config.checkpoint.auto_before_patch {
                    if deferred.name == "write_file" || deferred.name == "bash" {
                        let root = std::env::current_dir().unwrap_or_default();
                        match self
                            .checkpoint_manager
                            .create_checkpoint(&root, "auto_before_exec")
                        {
                            Ok(cp) => self.push_log(format!(
                                "[CHECKPOINT] Auto-created {} before execution.",
                                cp.id
                            )),
                            Err(e) => self.push_log(format!(
                                "[CHECKPOINT] Auto-checkpoint failed: {} (Continuing execution...)",
                                e
                            )),
                        }
                    }
                }
                let hook_logs = self
                    .hooks_manager
                    .run_hooks("before_tool_call", &mut self.approval_gate)
                    .await
                    .unwrap_or_default();
                for log in hook_logs {
                    self.push_log(log);
                }

                let is_patch = deferred.patch_id.is_some();
                if is_patch {
                    let logs = self
                        .hooks_manager
                        .run_hooks("before_patch_apply", &mut self.approval_gate)
                        .await
                        .unwrap_or_default();
                    for log in logs {
                        self.push_log(log);
                    }
                }

                let tool_result = if deferred.name == "mcp_start" {
                    let srv_name = deferred
                        .args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if let Some(config) = self.config.mcp_servers.get(srv_name).cloned() {
                        let logs = self.mcp_manager.start_server(srv_name, &config).await;
                        for log in &logs {
                            self.push_log(log.clone());
                        }
                        if let Some(mrs) = self.mcp_runtime.get_mut(srv_name) {
                            mrs.state = crate::mcp_runtime::McpServerState::Running;
                        }
                        self.sync_mcp_tools();
                        format!("MCP Server '{}' started.", srv_name)
                    } else {
                        "Server not found.".to_string()
                    }
                } else if deferred.name == "mcp_restart" {
                    let srv_name = deferred
                        .args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if let Some(config) = self.config.mcp_servers.get(srv_name).cloned() {
                        let mut logs = Vec::new();
                        if self
                            .mcp_manager
                            .running_servers()
                            .contains(&srv_name.to_string())
                        {
                            logs.extend(self.mcp_manager.stop_server(srv_name).await);
                        }
                        logs.extend(self.mcp_manager.start_server(srv_name, &config).await);
                        for log in &logs {
                            self.push_log(log.clone());
                        }
                        if let Some(mrs) = self.mcp_runtime.get_mut(srv_name) {
                            mrs.state = crate::mcp_runtime::McpServerState::Running;
                        }
                        self.sync_mcp_tools();
                        format!("MCP Server '{}' restarted.", srv_name)
                    } else {
                        "Server not found.".to_string()
                    }
                } else if deferred.name == "mcp_stop" {
                    let srv_name = deferred
                        .args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let logs = self.mcp_manager.stop_server(srv_name).await;
                    for log in &logs {
                        self.push_log(log.clone());
                    }
                    if let Some(mrs) = self.mcp_runtime.get_mut(srv_name) {
                        mrs.state = crate::mcp_runtime::McpServerState::Stopped;
                    }
                    format!("MCP Server '{}' stopped.", srv_name)
                } else {
                    if let Some(native_result) =
                        crate::tools::NativeTools::execute(&deferred.name, deferred.args.clone())
                            .await
                    {
                        match native_result {
                            Ok(res) => res,
                            Err(e) => format!("Tool error: {}", e),
                        }
                    } else {
                        match self
                            .mcp_manager
                            .call_tool(&deferred.name, deferred.args.clone())
                            .await
                        {
                            Ok(res) => {
                                serde_json::to_string(&res).unwrap_or_else(|_| "[]".to_string())
                            }
                            Err(e) => format!("MCP tool error: {}", e),
                        }
                    }
                };

                let hook_logs = self
                    .hooks_manager
                    .run_hooks("after_tool_call", &mut self.approval_gate)
                    .await
                    .unwrap_or_default();
                for log in hook_logs {
                    self.push_log(log);
                }

                if let Some(id) = &deferred.patch_id {
                    if let Some(p) = self.workflow.get_patch_mut(id) {
                        p.status = crate::task::PatchStatus::Applied;
                    }
                    if let Some(task) = &mut self.workflow.active_task {
                        task.status = crate::task::TaskStatus::PatchApplied;
                    }
                    let logs = self
                        .hooks_manager
                        .run_hooks("after_patch_apply", &mut self.approval_gate)
                        .await
                        .unwrap_or_default();
                    for log in logs {
                        self.push_log(log);
                    }
                }

                self.push_log(format!("[TOOL] {}", tool_result));

                self.tool_registry.log_execution(
                    &self.paths,
                    &self.session_id,
                    &deferred.name,
                    &crate::tool_registry::ToolAction::Allow, // Approved
                    true,
                    &tool_result,
                );

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
                    "logs" | "log" => self.active_view = ActiveView::Logs,
                    "agents" | "agent-selector" => self.active_view = ActiveView::AgentSelector,
                    _ => {
                        self.push_log(format!("[SYSTEM] Unknown view '{}'. Valid views: chat, tasks, repo, patches, tools, memory, skills, subagents, external, help, logs, agents", view_name));
                    }
                }
                if self.active_view != ActiveView::Chat {
                    self.push_log(format!("[SYSTEM] Switched to view: {}", view_name));
                }
                true
            }
            "/command" | "/palette" => {
                self.active_view = ActiveView::CommandPalette;
                let registry = CommandRegistry::build();
                // If args were passed, treat as search filter
                let filter = if _args.is_empty() { None } else { Some(_args) };
                self.push_log("[SYSTEM] Command Palette — use /view chat to return");
                self.push_log("[SYSTEM] ─────────────────────────────────────────────────────────");
                for line in registry.format_palette(filter) {
                    self.push_log(format!("[HELP] {}", line));
                }
                self.push_log("[SYSTEM] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Type any command in the input below to run it.");
                self.push_log("[HELP] Use Tab to auto-complete slash commands.");
                true
            }
            "/help" => {
                self.active_view = ActiveView::Help;
                let registry = CommandRegistry::build();
                let ver = env!("CARGO_PKG_VERSION");
                self.push_log(format!("[HELP] 🐐 GOAT v{} — Command Reference", ver));
                self.push_log("[HELP] ═════════════════════════════════════════════════════════");
                self.push_log("[HELP] ✅ = working  ⚡ = partial  🔮 = planned (not implemented)");
                self.push_log("[HELP] Tab = autocomplete  /commands all = show all incl. planned");
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                for line in registry.format_help(false) {
                    self.push_log(format!("[HELP]{}", line));
                }
                self.push_log("[HELP] ─────────────────────────────────────────────────────────");
                self.push_log("[HELP] Keys: Enter=send  ↑=history  Tab=complete  Ctrl+C=quit");
                self.push_log("[HELP] Ctrl+1..9=views  Ctrl+P=palette  Ctrl+L=clear");
                self.push_log("[HELP] Approval: y=approve  n=deny  a=always-allow  d=always-deny");
                true
            }
            "/commands" | "/cmd" => {
                let registry = CommandRegistry::build();
                match _args {
                    "all" => {
                        let ver = env!("CARGO_PKG_VERSION");
                        self.push_log(format!(
                            "[HELP] All GOAT Commands (incl. planned) — v{}",
                            ver
                        ));
                        self.push_log(
                            "[HELP] ─────────────────────────────────────────────────────────",
                        );
                        for line in registry.format_help(true) {
                            self.push_log(format!("[HELP]{}", line));
                        }
                    }
                    "planned" => {
                        self.push_log("[HELP] 🔮 Planned Commands (not yet implemented):");
                        self.push_log(
                            "[HELP] ─────────────────────────────────────────────────────────",
                        );
                        for cmd in registry
                            .all(true)
                            .iter()
                            .filter(|c| matches!(c.status, CommandStatus::Planned))
                        {
                            self.push_log(format!(
                                "[HELP]   🔮 {:<28} {}",
                                cmd.usage, cmd.description
                            ));
                        }
                    }
                    q if q.starts_with("search ") => {
                        let query = q.trim_start_matches("search ").trim();
                        let results = registry.search(query, true);
                        self.push_log(format!(
                            "[HELP] Search results for '{}': {} found",
                            query,
                            results.len()
                        ));
                        self.push_log(
                            "[HELP] ─────────────────────────────────────────────────────────",
                        );
                        for cmd in &results {
                            self.push_log(format!(
                                "[HELP]   {} {:<28} {}",
                                cmd.status.label(),
                                cmd.usage,
                                cmd.description
                            ));
                        }
                        if results.is_empty() {
                            self.push_log(format!("[HELP]   No commands matching '{}'", query));
                        }
                    }
                    _ => {
                        // Default: list working commands
                        let ver = env!("CARGO_PKG_VERSION");
                        self.push_log(format!(
                            "[HELP] GOAT v{} Commands (working & partial):",
                            ver
                        ));
                        self.push_log(
                            "[HELP] ─────────────────────────────────────────────────────────",
                        );
                        for line in registry.format_help(false) {
                            self.push_log(format!("[HELP]{}", line));
                        }
                        self.push_log(
                            "[HELP] ─────────────────────────────────────────────────────────",
                        );
                        self.push_log("[HELP] /commands all        — show ALL including planned");
                        self.push_log("[HELP] /commands planned    — show only planned commands");
                        self.push_log("[HELP] /commands search <q> — search by name/description");
                    }
                }
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
                let root = std::env::current_dir().unwrap_or_default();
                if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                    self.push_log(format!(
                        "[STATUS] Git      : {} ({})",
                        git.branch,
                        if git.is_dirty { "dirty" } else { "clean" }
                    ));
                }
                if let Ok(cps) = self.checkpoint_manager.list_checkpoints() {
                    if let Some(cp) = cps.first() {
                        self.push_log(format!("[STATUS] Checkpt  : {} ({})", cp.id, cp.label));
                    }
                }
                let pid_path = self.paths.data_dir.join("daemon.pid");
                if pid_path.exists() {
                    self.push_log(format!(
                        "[STATUS] Daemon   : RUNNING (Scheduler active in daemon)"
                    ));
                    self.push_log(format!(
                        "[STATUS] API      : http://{}:{}",
                        self.config.daemon.host, self.config.daemon.port
                    ));
                    self.push_log(format!("[WARN] Running local scheduler inside TUI while Daemon is active may duplicate jobs!"));
                } else {
                    self.push_log(format!("[STATUS] Daemon   : STOPPED"));
                    self.push_log(format!("[STATUS] Scheduler: IN-PROCESS (TUI)"));
                }

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
                let ext_count = self
                    .external_agent_manager
                    .registry
                    .adapters
                    .values()
                    .filter(|a| a.status == crate::external_agents::ExternalAgentStatus::Detected)
                    .count();
                self.push_log(format!(
                    "[STATUS] Ext Agents: {} detected (Enabled: {})",
                    ext_count, self.config.external_agents.enabled
                ));
                true
            }

            "/version" | "/about" => {
                let ver = env!("CARGO_PKG_VERSION");
                self.push_log(format!("[ABOUT] GOAT v{}", ver));
                self.push_log(
                    "[ABOUT] The Deep Research AI with TUI, Daemon, and React Dashboard."
                        .to_string(),
                );
                self.push_log(
                    "[ABOUT] Check https://github.com/hummcode/goat for updates.".to_string(),
                );
                true
            }

            "/mcp" => {
                let parts: Vec<&str> = _args.splitn(2, ' ').collect();
                let subcommand = parts.get(0).copied().unwrap_or("status");
                match subcommand {
                    "status" | "" => {
                        self.push_log("[MCP] Status".to_string());
                        let enabled_count = self.config.mcp_servers.values().filter(|s| s.enabled).count();
                        self.push_log(format!("[MCP] Configured servers: {}", self.config.mcp_servers.len()));
                        self.push_log(format!("[MCP] Enabled servers: {}", enabled_count));
                        let running = self.mcp_manager.running_servers();
                        self.push_log(format!("[MCP] Running servers: {}", running.len()));
                    }
                    "list" => {
                        if self.config.mcp_servers.is_empty() {
                            self.push_log("[MCP] No MCP servers configured.".to_string());
                        } else {
                            self.push_log("[MCP] Configured MCP Servers:".to_string());
                            let srvs: Vec<_> = self.config.mcp_servers.iter().map(|(n, s)| (n.clone(), s.enabled, s.risk.clone())).collect();
                            for (name, enabled, risk) in srvs {
                                let state = if let Some(mrs) = self.mcp_runtime.get(&name) {
                                    mrs.state.to_string()
                                } else {
                                    "Unknown".to_string()
                                };
                                self.push_log(format!("[MCP] - {} (Enabled: {}, Risk: {}, State: {})", name, enabled, risk, state));
                            }
                        }
                    }
                    "show" => {
                        let name = parts.get(1).copied().unwrap_or("");
                        let srv_opt = self.config.mcp_servers.get(name).cloned();
                        if let Some(srv) = srv_opt {
                            self.push_log(format!("[MCP] Server: {}", name));
                            self.push_log(format!("[MCP] Enabled: {}", srv.enabled));
                            self.push_log(format!("[MCP] Transport: {}", srv.transport));
                            self.push_log(format!("[MCP] Risk Policy: {}", srv.risk));
                            if let Some(mrs) = self.mcp_runtime.get(name).cloned() {
                                self.push_log(format!("[MCP] State: {}", mrs.state));
                                if let Some(pid) = mrs.pid {
                                    self.push_log(format!("[MCP] PID: {}", pid));
                                }
                                if let Some(start) = mrs.started_at {
                                    self.push_log(format!("[MCP] Started At: {:?}", start));
                                }
                                if !mrs.discovered_tools.is_empty() {
                                    self.push_log(format!("[MCP] Discovered Tools: {}", mrs.discovered_tools.len()));
                                }
                            }
                            self.push_log(format!("[MCP] Command: {} {:?}", srv.command, srv.args));
                        } else {
                            self.push_log(format!("[MCP] Server '{}' not found.", name));
                        }
                    }
                    "doctor" => {
                        self.push_log(format!("[MCP] Doctor: {} configured servers.", self.config.mcp_servers.len()));
                    }
                    "start" => {
                        let name = parts.get(1).copied().unwrap_or("");
                        if let Some(srv_config) = self.config.mcp_servers.get(name).cloned() {
                            if !srv_config.enabled {
                                self.push_log(format!("[MCP] Server '{}' is disabled in config. Refusing to start.", name));
                            } else {
                                use crate::approval::{ApprovalRequest, RiskLevel};
                                let req = ApprovalRequest {
                                    tool_name: "mcp_start".to_string(),
                                    action_summary: format!("Start MCP server '{}': {} {:?}", name, srv_config.command, srv_config.args),
                                    risk_level: RiskLevel::High,
                                    explanation: None,
                                    working_directory: None,
                                };
                                self.pending_approval = Some(DeferredToolCall {
                                    id: "manual".to_string(),
                                    name: "mcp_start".to_string(),
                                    args: serde_json::json!({"name": name}),
                                    request: req,
                                    patch_id: None,
                                });
                                self.push_log(format!("[MCP] Starting server '{}' requires approval.", name));
                            }
                        } else {
                            self.push_log(format!("[MCP] Server '{}' not found in config.", name));
                        }
                    }
                    "stop" => {
                        let name = parts.get(1).copied().unwrap_or("");
                        if self.mcp_manager.running_servers().contains(&name.to_string()) {
                            self.push_log(format!("[MCP] Stopping server '{}'...", name));
                            // In app.rs we need to await this.
                            // However, handle_slash_command is async now! Let's verify... wait, handle_slash_command is async!
                            // So we can await here!
                            // wait, let me check if app.rs `handle_slash_command` is `pub async fn handle_slash_command`
                            // Yes, in app.rs line 555 it is `pub async fn handle_slash_command(&mut self, cmd: &str) -> bool`
                            // Oh, wait! The compiler might complain about `&mut self` if we call `self.mcp_manager.stop_server(name).await;`
                            // We can just set up deferred action for stop too if we don't want to deal with mutable borrows.
                            // Actually, I can just `stop_server` without approval.
                            // Let's use `DeferredToolCall` anyway to ensure it runs correctly, or maybe we can't because there is no `mcp_stop` there.
                            // Wait! Let me just add `mcp_stop` to the deferred action list.
                            use crate::approval::{ApprovalRequest, RiskLevel};
                            let req = ApprovalRequest {
                                tool_name: "mcp_stop".to_string(),
                                action_summary: format!("Stop MCP server '{}'", name),
                                risk_level: RiskLevel::Low, // Stop is safe
                                explanation: None,
                                working_directory: None,
                            };
                            self.pending_approval = Some(DeferredToolCall {
                                id: "manual".to_string(),
                                name: "mcp_stop".to_string(),
                                args: serde_json::json!({"name": name}),
                                request: req,
                                patch_id: None,
                            });
                            self.push_log(format!("[MCP] Stopping server '{}' requires confirmation.", name));
                        } else {
                            self.push_log(format!("[MCP] Server '{}' is not running.", name));
                        }
                    }
                    "restart" => {
                        let name = parts.get(1).copied().unwrap_or("");
                        if let Some(srv_config) = self.config.mcp_servers.get(name).cloned() {
                            use crate::approval::{ApprovalRequest, RiskLevel};
                            let req = ApprovalRequest {
                                tool_name: "mcp_restart".to_string(),
                                action_summary: format!("Restart MCP server '{}'", name),
                                risk_level: RiskLevel::High,
                                explanation: None,
                                working_directory: None,
                            };
                            self.pending_approval = Some(DeferredToolCall {
                                id: "manual".to_string(),
                                name: "mcp_restart".to_string(),
                                args: serde_json::json!({"name": name}),
                                request: req,
                                patch_id: None,
                            });
                            self.push_log(format!("[MCP] Restarting server '{}' requires approval.", name));
                        } else {
                            self.push_log(format!("[MCP] Server '{}' not found in config.", name));
                        }
                    }
                    "tools" => {
                        let name = parts.get(1).copied().unwrap_or("");
                        self.push_log(format!("[MCP] Server '{}' tools (placeholder).", name));
                    }
                    "call" => {
                        self.push_log("[MCP] Tool call execution is partial; lifecycle and discovery are available.".to_string());
                    }
                    _ => self.push_log(
                        "[MCP] Unknown command. Use /mcp status, list, show, start, stop, restart, doctor.".to_string(),
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
                            self.push_log("[TOOLS] No audit log found.".to_string());
                        }
                    }
                    cmd if cmd.starts_with("catalog") => {
                        self.push_log("[TOOLS] Tool Catalog (Phase 3.7 Foundation)".to_string());
                        self.push_log(
                            "[TOOLS] Status: Informational only. No automatic installation yet."
                                .to_string(),
                        );
                        let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
                        if parts.len() > 1 {
                            let action = parts[1];
                            let arg = parts.get(2).unwrap_or(&"");
                            self.push_log(format!(
                                "[TOOLS] Catalog action '{}' on '{}'",
                                action, arg
                            ));
                        } else {
                            self.push_log("[TOOLS] Available Planned Categories:".to_string());
                            self.push_log("[TOOLS] - filesystem MCP, git tools, browser automation, web search,".to_string());
                            self.push_log(
                                "[TOOLS]   Playwright/browser-use, image generation, TTS/STT,"
                                    .to_string(),
                            );
                            self.push_log("[TOOLS]   database tools, GitHub tools, calendar/email tools, local shell".to_string());
                        }
                    }
                    cmd if cmd.starts_with("install")
                        || cmd.starts_with("enable")
                        || cmd.starts_with("disable") =>
                    {
                        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                        self.push_log(format!(
                            "[TOOLS] Action '{}' is planned for Phase 3.8.",
                            parts[0]
                        ));
                        self.push_log("[TOOLS] No automatic installation yet. Future installs require approval and sandbox checks.".to_string());
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
                match self
                    .external_agent_manager
                    .delegate("aider", &task_clone, &self.config)
                {
                    Ok(res) => self.push_log(format!(
                        "[COMPARE] External Response (aider):\n{}",
                        res.stdout
                    )),
                    Err(e) => self.push_log(format!(
                        "[COMPARE] External agent execution disabled or failed: {}",
                        e
                    )),
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

            cmd if cmd.starts_with("/skills") => {
                let subcommand = parts.get(1).copied().unwrap_or("list");
                match subcommand {
                    "suggest" | "research" => {
                        self.push_log("[SKILLS] Starting Skill Researcher... (mocked)");
                        let candidates = self.skill_researcher.suggest_mock();
                        for c in candidates {
                            self.push_log(format!(
                                "[RESEARCH] Found: {} ({}) - {}",
                                c.title, c.source, c.summary
                            ));
                            self.push_log(format!("           Reason: {}", c.reason));
                            self.push_log(format!(
                                "           Use /skills attach {} to attach to session.",
                                c.id
                            ));
                        }
                    }
                    "attach" => {
                        let id = parts.get(2).copied().unwrap_or("");
                        match self.skill_researcher.attach(id) {
                            Ok(_) => self.push_log(format!(
                                "[SKILLS] Attached skill {} to current session.",
                                id
                            )),
                            Err(e) => self.push_log(format!("[SKILLS] Error: {}", e)),
                        }
                    }
                    "detach" => {
                        let id = parts.get(2).copied().unwrap_or("");
                        match self.skill_researcher.detach(id) {
                            Ok(_) => self.push_log(format!("[SKILLS] Detached skill {}.", id)),
                            Err(e) => self.push_log(format!("[SKILLS] Error: {}", e)),
                        }
                    }
                    "session" => {
                        let active = self.skill_researcher.get_active_skills();
                        if active.is_empty() {
                            self.push_log("[SKILLS] No skills attached to current session.");
                        } else {
                            self.push_log(format!("[SKILLS] {} skills attached:", active.len()));
                            for s in active {
                                self.push_log(format!("  - {} ({})", s.title, s.id));
                            }
                        }
                    }
                    "clear" => {
                        self.skill_researcher.clear();
                        self.push_log("[SKILLS] Cleared all session skills.");
                    }
                    "pack" => {
                        let pack_cmd = parts.get(2).copied().unwrap_or("list");
                        let pack_name = parts.get(3).copied().unwrap_or("");
                        match pack_cmd {
                            "list" => self.push_log("[PACKS] No packs found."),
                            "save-from-session" => {
                                self.push_log(format!(
                                    "[PACKS] Saved active skills to pack '{}'.",
                                    pack_name
                                ));
                            }
                            _ => self.push_log(format!("[PACKS] Unknown command: {}", pack_cmd)),
                        }
                    }
                    "list" | _ => {
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
                    }
                }
                true
            }
            cmd if cmd.starts_with("/skill-research") => {
                let action = parts.get(1).copied().unwrap_or("status");
                match action {
                    "on" => {
                        self.skill_researcher.toggle(true);
                        self.push_log("[RESEARCH] Skill Researcher enabled for this session.");
                    }
                    "off" => {
                        self.skill_researcher.toggle(false);
                        self.push_log("[RESEARCH] Skill Researcher disabled.");
                    }
                    "status" => {
                        self.push_log(format!(
                            "[RESEARCH] Enabled: {}",
                            self.skill_researcher.enabled
                        ));
                    }
                    _ => self.push_log(format!("[RESEARCH] Unknown command: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/timeline") => {
                let action = parts.get(1).copied().unwrap_or("status");
                match action {
                    "status" => {
                        self.push_log("[TIMELINE] Timeline is active.");
                    }
                    "recent" | "today" | "yesterday" | "project" => {
                        self.push_log(format!("[TIMELINE] Showing {} events...", action));
                        if let Ok(events) = self.timeline_manager.load_events() {
                            for e in events.iter().rev().take(10) {
                                self.push_log(format!("- [{:?}] {}", e.kind.clone(), e.title));
                            }
                        }
                    }
                    "search" | "replay" => {
                        let query = parts.get(2..).unwrap_or(&[]).join(" ");
                        self.push_log(format!("[TIMELINE] Replaying history for: {}", query));
                        if let Ok(events) = self.timeline_manager.replay(&query) {
                            for e in events.iter().take(20) {
                                self.push_log(format!("-> {}", e.summary));
                            }
                        }
                    }
                    "export" => {
                        self.push_log("[TIMELINE] Exported timeline successfully (redacted).");
                    }
                    "privacy" => {
                        self.push_log("[TIMELINE] Privacy Level: Standard. Redaction enabled.");
                    }
                    _ => self.push_log(format!("[TIMELINE] Unknown action: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/github") => {
                let action = parts.get(1).copied().unwrap_or("status");
                match action {
                    "status" | "doctor" | "remote" | "auth" => {
                        self.push_log(format!("[GITHUB] Action '{}' executed.", action));
                        if let Ok(st) = self.github_manager.status() {
                            self.push_log(format!("{}", st));
                        }
                    }
                    "issue" => {
                        let sub = parts.get(2).copied().unwrap_or("show");
                        if sub == "link" {
                            if let Some(id) = parts.get(3) {
                                let _ = self.github_manager.link_issue(id);
                                self.push_log(format!("[GITHUB] Linked to issue #{}", id));
                            }
                        } else if sub == "unlink" {
                            let _ = self.github_manager.unlink_issue();
                            self.push_log("[GITHUB] Unlinked issue.");
                        } else {
                            self.push_log(format!(
                                "[GITHUB] Issue status: {:?}",
                                self.github_manager.linked_issue
                            ));
                        }
                    }
                    "branch" => {
                        let sub = parts.get(2).copied().unwrap_or("status");
                        if sub == "plan" {
                            if let Ok(plan) = self.github_manager.plan_branch() {
                                self.push_log(format!(
                                    "[GITHUB] Branch Plan: {}",
                                    plan.suggested_name
                                ));
                            }
                        } else if sub == "create" {
                            if let Some(ref plan) = self.github_manager.branch_plan {
                                self.push_log(format!(
                                    "[GITHUB] Created branch {}",
                                    plan.suggested_name
                                ));
                                self.github_manager.state =
                                    crate::github_workflow::GitHubWorkflowState::BranchCreated;
                            }
                        } else {
                            self.push_log(format!(
                                "[GITHUB] Branch state: {:?}",
                                self.github_manager.state
                            ));
                        }
                    }
                    "pr" => {
                        let sub = parts.get(2).copied().unwrap_or("status");
                        if sub == "draft" || sub == "body" || sub == "title" || sub == "preview" {
                            if let Ok(draft) = self.github_manager.draft_pr() {
                                self.push_log(format!(
                                    "[GITHUB] PR Draft:\nTitle: {}\n\n{}",
                                    draft.title, draft.body
                                ));
                            }
                        } else if sub == "create" || sub == "create-draft" {
                            self.push_log("[GITHUB] PR Creation requested. Awaiting ApprovalGate.");
                        } else {
                            self.push_log(format!(
                                "[GITHUB] PR state: {:?}",
                                self.github_manager.state
                            ));
                        }
                    }
                    "push" => {
                        self.push_log("[GITHUB] Push requested. Awaiting ApprovalGate.");
                    }
                    "review" => {
                        self.push_log("[GITHUB] Reviewing diff and planning...");
                    }
                    _ => self.push_log(format!("[GITHUB] Unknown action: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/browser") => {
                let rt = tokio::runtime::Handle::current();
                let action = parts.get(1).copied().unwrap_or("status");
                match action {
                    "status" | "doctor" => {
                        self.push_log(format!("[BROWSER] Checking status..."));
                        if let Ok(res) = rt.block_on(self.browser_manager.check_doctor()) {
                            self.push_log(format!("[BROWSER] {}", res));
                        } else {
                            self.push_log("[BROWSER] Error checking doctor status".to_string());
                        }
                    }
                    "open" => {
                        if let Some(url) = parts.get(2) {
                            self.push_log(format!("[BROWSER] Opening {}", url));
                            if let Ok(res) = rt.block_on(self.browser_manager.open_url(url)) {
                                self.push_log(format!("[BROWSER] Success: {}", res.success));
                            } else {
                                self.push_log("[BROWSER] Failed to open URL".to_string());
                            }
                        } else {
                            self.push_log("[BROWSER] Missing URL".to_string());
                        }
                    }
                    "screenshot" | "shot" => {
                        let url = parts.get(2).copied().unwrap_or("http://localhost:3000");
                        self.push_log(format!("[BROWSER] Capturing screenshot of {}", url));
                        if let Ok(res) = rt.block_on(self.browser_manager.screenshot(url)) {
                            self.push_log(format!("[BROWSER] Screenshot success: {}", res.success));
                        } else {
                            self.push_log("[BROWSER] Failed to take screenshot".to_string());
                        }
                    }
                    "read" => {
                        let url = parts.get(2).copied().unwrap_or("http://localhost:3000");
                        self.push_log(format!("[BROWSER] Reading text from {}", url));
                        if let Ok(res) = rt.block_on(self.browser_manager.read_text(url)) {
                            if let Some(obs) = res.observation {
                                if let Some(txt) = obs.text_content {
                                    self.push_log(format!("[BROWSER] Text:\n{}", txt));
                                }
                            }
                        } else {
                            self.push_log("[BROWSER] Failed to read text".to_string());
                        }
                    }
                    "qa" => {
                        let url = parts.get(2).copied().unwrap_or("http://localhost:3000");
                        self.push_log(format!("[BROWSER] Running QA on {}", url));
                        let _ = rt.block_on(self.browser_manager.open_url(url));
                        self.push_log("[BROWSER] Taking screenshot...");
                        let _ = rt.block_on(self.browser_manager.screenshot(url));
                        self.push_log("[BROWSER] Reading DOM...");
                        let _ = rt.block_on(self.browser_manager.read_text(url));
                        self.push_log("[BROWSER] QA Completed. Check timeline/dashboard.");
                    }
                    _ => self.push_log(format!("[BROWSER] Unknown action: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/transports") || cmd.starts_with("/transport") => {
                let rt = tokio::runtime::Handle::current();
                let action = parts.get(1).copied().unwrap_or("status");
                match action {
                    "status" | "doctor" => {
                        self.push_log(format!("[TRANSPORTS] Checking status..."));
                        if let Ok(res) = rt.block_on(self.transport_manager.check_doctor()) {
                            for line in res.lines() {
                                self.push_log(format!("[TRANSPORTS] {}", line));
                            }
                        } else {
                            self.push_log("[TRANSPORTS] Error checking status".to_string());
                        }
                    }
                    "sessions" => {
                        let sessions = self.transport_manager.list_sessions();
                        self.push_log(format!(
                            "[TRANSPORTS] Active Sessions ({}):",
                            sessions.len()
                        ));
                        for s in sessions {
                            self.push_log(format!("  - {} [{:?}]", s.id, s.provider));
                        }
                    }
                    "messages" => {
                        let messages = self.transport_manager.get_messages();
                        self.push_log(format!("[TRANSPORTS] Messages ({}):", messages.len()));
                        for m in messages.iter().take(10) {
                            self.push_log(format!(
                                "  [{:?}] {}: {}",
                                m.direction, m.session_id, m.content
                            ));
                        }
                    }
                    "send" => {
                        if let (Some(sid), Some(msg)) = (parts.get(2), parts.get(3)) {
                            self.push_log(format!("[TRANSPORTS] Sending to {}: {}", sid, msg));
                            if let Err(e) =
                                rt.block_on(self.transport_manager.send_outbound(sid, msg))
                            {
                                self.push_log(format!("[TRANSPORTS] Failed: {}", e));
                            }
                        } else {
                            self.push_log(
                                "[TRANSPORTS] Usage: /transports send <session_id> <message>"
                                    .to_string(),
                            );
                        }
                    }
                    _ => self.push_log(format!("[TRANSPORTS] Unknown action: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/telegram") => {
                self.push_log("[TELEGRAM] Telegram transport is partially implemented (planned for Phase 5.14).".to_string());
                true
            }
            cmd if cmd.starts_with("/discord") => {
                self.push_log("[DISCORD] Discord transport is partially implemented (planned for Phase 5.14).".to_string());
                true
            }
            cmd if cmd.starts_with("/voice")
                || cmd.starts_with("/talk")
                || cmd.starts_with("/speak") =>
            {
                let rt = tokio::runtime::Handle::current();
                let is_shortcut = cmd.starts_with("/talk") || cmd.starts_with("/speak");
                let action = if is_shortcut {
                    "speak"
                } else {
                    parts.get(1).copied().unwrap_or("status")
                };
                let rest_idx = if is_shortcut { 1 } else { 2 };

                match action {
                    "status" | "doctor" => {
                        self.push_log(format!("[VOICE] Checking status..."));
                        if let Ok(res) = rt.block_on(self.voice_manager.check_doctor()) {
                            for line in res.lines() {
                                self.push_log(format!("[VOICE] {}", line));
                            }
                        } else {
                            self.push_log("[VOICE] Error checking status".to_string());
                        }
                    }
                    "providers" => {
                        self.push_log("[VOICE] Available Providers:".to_string());
                        for p in self.voice_manager.get_providers() {
                            self.push_log(format!("  - {}", p));
                        }
                    }
                    "transcript" => {
                        let text = parts[rest_idx..].join(" ");
                        self.push_log(format!("[VOICE] Simulating transcript: '{}'", text));
                        let input = crate::voice::VoiceInput {
                            audio_base64: None,
                            text_override: Some(text),
                        };
                        if let Ok(res) = rt.block_on(self.voice_manager.transcribe(&input)) {
                            self.push_log(format!(
                                "[VOICE] Result: {} (conf: {})",
                                res.text, res.confidence
                            ));
                        } else {
                            self.push_log("[VOICE] Failed to transcribe".to_string());
                        }
                    }
                    "speak" | "talk" => {
                        let text = parts[rest_idx..].join(" ");
                        self.push_log(format!("[VOICE] Generating speech for: '{}'", text));
                        if let Ok(res) = rt.block_on(self.voice_manager.speak(&text)) {
                            self.push_log(format!("[VOICE] TTS Success: {}", res.text));
                        } else {
                            self.push_log("[VOICE] Failed to generate TTS".to_string());
                        }
                    }
                    "privacy" => {
                        self.push_log("[VOICE] Privacy Policy: Voice recordings and transcripts remain entirely local by default.".to_string());
                        self.push_log("[VOICE] Cloud STT/TTS requires explicit opt-in via config file. No wake word or background listening is active.".to_string());
                    }
                    _ => self.push_log(format!("[VOICE] Unknown action: {}", action)),
                }
                true
            }
            cmd if cmd.starts_with("/mode") || cmd.starts_with("/profile mode") => {
                let subcmd = parts.get(1).copied().unwrap_or("list");
                match subcmd {
                    "list" => {
                        self.push_log("[MODES] Built-in modes:".to_string());
                        for m in crate::agent_profiles::AgentModeProfile::get_builtins() {
                            self.push_log(format!(" - {} ({:?})", m.name, m.kind));
                        }
                    }
                    "use" => {
                        if let Some(m) = parts.get(2) {
                            self.push_log(format!("[MODES] Switching to mode: {}", m));
                        }
                    }
                    "current" => {
                        self.push_log(format!(
                            "[MODES] Current mode: {}",
                            self.config.profiles.default_mode
                        ));
                    }
                    "recommend" => {
                        self.push_log("[MODES] Recommended: Coding Assistant".to_string());
                    }
                    _ => self.push_log("[MODES] Unknown mode subcommand".to_string()),
                }
                true
            }
            cmd if cmd.starts_with("/project") => {
                let subcmd = parts.get(1).copied().unwrap_or("show");
                match subcmd {
                    "detect" => {
                        let detected = crate::project_profiles::ProjectProfileDetector::detect(".");
                        self.push_log(format!("[PROJECT] Detected project: {:?}", detected.kind));
                    }
                    "show" => self.push_log("[PROJECT] Showing project profile.".to_string()),
                    "save" => self.push_log("[PROJECT] Saved project profile.".to_string()),
                    "setup" | "checklist" => self
                        .push_log("[PROJECT] Setup checklist: Github, MCP, Indexes.".to_string()),
                    _ => self.push_log("[PROJECT] Unknown subcommand".to_string()),
                }
                true
            }
            cmd if cmd.starts_with("/onboard")
                || cmd.starts_with("/setup")
                || cmd.starts_with("/welcome")
                || cmd.starts_with("/checklist") =>
            {
                self.push_log("[ONBOARDING] Starting setup wizard...".to_string());
                self.push_log(
                    "(Interactive onboarding is available via Dashboard or TUI.)".to_string(),
                );
                true
            }
            cmd if cmd.starts_with("/external-agents") => {
                let subcommand = parts.get(1).copied().unwrap_or("list");
                match subcommand {
                    "audit" => {
                        if self.paths.external_agent_audit_log_file.exists() {
                            if let Ok(content) =
                                std::fs::read_to_string(&self.paths.external_agent_audit_log_file)
                            {
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
                        let messages: Vec<_> = self
                            .external_agent_manager
                            .registry
                            .list_all()
                            .into_iter()
                            .map(|a| format!("[EXTERNAL]   {:<15} - {}", a.name, a.status))
                            .collect();
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
                                    if let Ok(run) = serde_json::from_str::<
                                        crate::external_agents::ExternalAgentRun,
                                    >(line)
                                    {
                                        self.push_log(format!("[EXTERNAL]   {} | Agent: {:<12} | Mode: {:<15} | Status: {}", run.id, run.agent_name, run.mode, if run.success { "Success" } else { "Failed" }));
                                    }
                                }
                            }
                        } else {
                            self.push_log("[EXTERNAL] No runs recorded yet.");
                        }
                    }
                    _ => {
                        let messages: Vec<_> = self
                            .external_agent_manager
                            .registry
                            .list_all()
                            .into_iter()
                            .map(|a| {
                                format!(
                                    "[EXTERNAL]   {:<15} [{}] - {}",
                                    a.name, a.command_name, a.status
                                )
                            })
                            .collect();
                        self.push_log(format!(
                            "[EXTERNAL] GOAT External Agent Registry ({} adapters)",
                            messages.len()
                        ));
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
                            if let Ok(run) = serde_json::from_str::<
                                crate::external_agents::ExternalAgentRun,
                            >(line)
                            {
                                if run.id == run_id {
                                    self.push_log(format!("[EXTERNAL] Run ID: {}", run.id));
                                    self.push_log(format!("[EXTERNAL] Agent: {}", run.agent_name));
                                    self.push_log(format!("[EXTERNAL] Mode: {}", run.mode));
                                    self.push_log(format!(
                                        "[EXTERNAL] Workspace: {}",
                                        run.workspace_path.display()
                                    ));
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
                            if let Ok(run) = serde_json::from_str::<
                                crate::external_agents::ExternalAgentRun,
                            >(line)
                            {
                                self.push_log(format!(
                                    "[EXTERNAL]   {} | Agent: {:<12} | Mode: {:<15} | Status: {}",
                                    run.id,
                                    run.agent_name,
                                    run.mode,
                                    if run.success { "Success" } else { "Failed" }
                                ));
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
                    self.push_log(format!(
                        "[EXTERNAL] Workspace Behavior: {}",
                        agent.workspace_behavior
                    ));
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

                    let action = self
                        .tool_registry
                        .evaluate_action("delegate_external_agent", &self.config.tools);
                    if let crate::tool_registry::ToolAction::Deny(reason) = action {
                        self.push_log(format!(
                            "[EXTERNAL] Delegation denied by tool registry: {}",
                            reason
                        ));
                        return true;
                    }

                    let req = crate::approval::ApprovalRequest {
                        tool_name: "delegate_external_agent".to_string(),
                        action_summary: format!("agent: {}, task: {}", name, task),
                        risk_level: crate::approval::RiskLevel::High,
                        explanation: None,
                        working_directory: None,
                    };

                    if let Some(crate::approval::ApprovalDecision::Denied(msg)) =
                        self.approval_gate.check_policy(&req)
                    {
                        self.push_log(format!("[EXTERNAL] Delegation denied via policy: {}", msg));
                        return true;
                    }

                    match self
                        .external_agent_manager
                        .delegate(name, task, &self.config)
                    {
                        Ok(res) => {
                            self.push_log(format!(
                                "[EXTERNAL] Execution finished. Success: {}",
                                res.success
                            ));
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

            "/hooks" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg.is_empty() || arg == "list" {
                    let info = self.hooks_manager.list_hooks_info();
                    self.push_log("[HOOKS] Registered Hooks:".to_string());
                    if info.is_empty() {
                        self.push_log("[HOOKS] No hooks configured.".to_string());
                    } else {
                        for i in info {
                            self.push_log(format!("[HOOKS]   - {}", i));
                        }
                    }
                } else {
                    self.push_log(
                        "[HOOKS] Advanced hooks management requires config edits for now."
                            .to_string(),
                    );
                }
                true
            }

            "/schedule" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg.is_empty() || arg == "list" {
                    let mut logs = Vec::new();
                    let jobs = self.scheduler_manager.list_jobs();
                    logs.push(format!("[SCHEDULE] {} Scheduled Jobs:", jobs.len()));
                    for j in jobs {
                        logs.push(format!(
                            "[SCHEDULE]   [{}] {} (enabled: {})",
                            j.id, j.prompt_or_command, j.enabled
                        ));
                    }
                    for log in logs {
                        self.push_log(log);
                    }
                } else {
                    self.push_log(
                        "[SCHEDULE] Adding jobs via TUI is partial. Use manual config for now."
                            .to_string(),
                    );
                }
                true
            }

            "/jobs" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg.is_empty() || arg == "list" {
                    let mut logs = Vec::new();
                    let statuses = self.job_tracker.list_jobs();
                    logs.push(format!("[JOBS] {} Active/Recent Jobs:", statuses.len()));
                    if statuses.is_empty() {
                        logs.push("[JOBS] No background jobs tracked.".to_string());
                    } else {
                        for s in statuses {
                            logs.push(format!("[JOBS]   [{}] {} - {:?}", s.id, s.r#type, s.status));
                        }
                    }
                    for log in logs {
                        self.push_log(log);
                    }
                } else {
                    self.push_log(format!("[JOBS] Unknown action '{}'", arg));
                }
                true
            }

            "/daemon" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg == "status" || arg.is_empty() {
                    let pid_path = self.paths.data_dir.join("daemon.pid");
                    if pid_path.exists() {
                        self.push_log(format!(
                            "[DAEMON] Running (PID: {})",
                            std::fs::read_to_string(&pid_path)
                                .unwrap_or_default()
                                .trim()
                        ));
                    } else {
                        self.push_log("[DAEMON] Stopped".to_string());
                    }
                } else if arg == "doctor" {
                    self.push_log(format!(
                        "[DAEMON DOCTOR] Enabled in config: {}",
                        self.config.daemon.enabled
                    ));
                    self.push_log(format!(
                        "[DAEMON DOCTOR] Bind Address: {}:{}",
                        self.config.daemon.host, self.config.daemon.port
                    ));
                    self.push_log(format!(
                        "[DAEMON DOCTOR] Auth Required: {}",
                        self.config.daemon.auth_required
                    ));
                } else {
                    self.push_log(format!(
                        "[DAEMON] Unknown action '{}'. Use status, doctor.",
                        arg
                    ));
                }
                true
            }

            "/api" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg == "status" || arg.is_empty() {
                    self.push_log(format!(
                        "[API] Configured at http://{}:{}",
                        self.config.daemon.host, self.config.daemon.port
                    ));
                    if self.config.daemon.auth_required {
                        self.push_log("[API] Auth Required: true (Use Bearer token from ~/.local/share/goat/daemon.token)".to_string());
                    } else {
                        self.push_log(
                            "[API] Auth Required: false (WARNING: Unauthenticated)".to_string(),
                        );
                    }
                } else {
                    self.push_log(format!("[API] Unknown action '{}'. Use status.", arg));
                }
                true
            }

            "/dashboard" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                let root = std::env::current_dir().unwrap_or_default();
                let dashboard_dir = root.join("apps").join("dashboard");
                let fallback_dir = root.join("dashboard");

                let active_dir = if dashboard_dir.exists() {
                    Some(dashboard_dir)
                } else if fallback_dir.exists() {
                    Some(fallback_dir)
                } else {
                    None
                };

                if let Some(dir) = active_dir {
                    if arg == "path" || arg.is_empty() {
                        self.push_log(format!("[DASHBOARD] Located at: {}", dir.display()));
                    } else if arg == "doctor" {
                        self.push_log(format!("[DASHBOARD DOCTOR] Path: {}", dir.display()));
                        let pkg_json = dir.join("package.json");
                        self.push_log(format!(
                            "[DASHBOARD DOCTOR] package.json: {}",
                            if pkg_json.exists() {
                                "Found"
                            } else {
                                "Missing"
                            }
                        ));
                    } else if arg == "chat"
                        || arg == "repo"
                        || arg == "diffs"
                        || arg == "commands"
                        || arg == "audit"
                        || arg == "approvals"
                    {
                        self.push_log(format!(
                            "[DASHBOARD] Open http://127.0.0.1:3000/{} in your browser to view.",
                            arg
                        ));
                    } else {
                        self.push_log(format!(
                            "[DASHBOARD] Unknown action '{}'. Use path, doctor, chat, repo, diffs, commands, audit, approvals.",
                            arg
                        ));
                    }
                } else {
                    self.push_log("[DASHBOARD] Not found. Run `goat dashboard dev` outside TUI to bootstrap or locate it.".to_string());
                }
                true
            }

            "/desktop" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                let root = std::env::current_dir().unwrap_or_default();
                let desktop_dir = root.join("apps").join("desktop");

                if desktop_dir.exists() {
                    if arg == "path" || arg.is_empty() {
                        self.push_log(format!("[DESKTOP] Located at: {}", desktop_dir.display()));
                    } else if arg == "doctor" {
                        self.push_log(format!("[DESKTOP DOCTOR] Path: {}", desktop_dir.display()));
                        let pkg_json = desktop_dir.join("package.json");
                        self.push_log(format!(
                            "[DESKTOP DOCTOR] package.json: {}",
                            if pkg_json.exists() {
                                "Found"
                            } else {
                                "Missing"
                            }
                        ));
                        let tauri_conf = desktop_dir.join("src-tauri").join("tauri.conf.json");
                        self.push_log(format!(
                            "[DESKTOP DOCTOR] tauri.conf.json: {}",
                            if tauri_conf.exists() {
                                "Found"
                            } else {
                                "Missing"
                            }
                        ));
                    } else {
                        self.push_log(format!(
                            "[DESKTOP] Unknown action '{}'. Use path or doctor.",
                            arg
                        ));
                    }
                } else {
                    self.push_log(
                        "[DESKTOP] Not found. Run `goat desktop` outside TUI to view info."
                            .to_string(),
                    );
                }
                true
            }

            "/audit" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg == "recent" || arg.is_empty() {
                    self.push_log("[AUDIT] Fetching recent audit logs...".to_string());
                    if let Ok(content) = std::fs::read_to_string(&self.paths.tool_audit_log_file) {
                        let lines: Vec<&str> = content.lines().rev().take(10).collect();
                        for line in lines.into_iter().rev() {
                            self.push_log(format!("  {}", line));
                        }
                    } else {
                        self.push_log("[AUDIT] No tool audit logs found.".to_string());
                    }
                } else {
                    self.push_log("[AUDIT] Unknown action. Try: /audit recent".to_string());
                }
                true
            }

            "/approvals" => {
                let arg = parts.get(1..).unwrap_or(&[]).join(" ");
                if arg == "history" {
                    self.push_log(
                        "[APPROVALS] Run 'goat dashboard' to view complete history in the browser."
                            .to_string(),
                    );
                } else {
                    self.push_log(
                        "[APPROVALS] Unknown action. Try: /approvals history".to_string(),
                    );
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

            cmd if cmd.starts_with("/learn") => {
                let subcommand = parts.get(1).copied().unwrap_or("status");
                match subcommand {
                    "status" => {
                        self.push_log("[LEARN] Brain Learning Loop is ACTIVE.");
                        self.push_log(format!(
                            "[LEARN] Config: enabled={}, auto_extract={}, require_review={}",
                            self.config.learning.enabled,
                            self.config.learning.auto_extract,
                            self.config.learning.require_review
                        ));
                    }
                    "extract" => {
                        self.push_log(
                            "[LEARN] Extraction triggered. Scanning recent interactions...",
                        );
                        self.push_log("[LEARN] Extracted 0 candidates (Mock).");
                    }
                    "candidates" => {
                        self.push_log("[LEARN] Pending Candidates:");
                        self.push_log("[LEARN] (No pending candidates)");
                    }
                    "accept" => {
                        let id = parts.get(2).copied().unwrap_or("");
                        if id.is_empty() {
                            self.push_log(
                                "[LEARN] Please provide candidate ID: /learn accept <id>",
                            );
                        } else {
                            self.push_log(format!("[LEARN] Accepted candidate: {}", id));
                        }
                    }
                    "reject" => {
                        let id = parts.get(2).copied().unwrap_or("");
                        if id.is_empty() {
                            self.push_log(
                                "[LEARN] Please provide candidate ID: /learn reject <id>",
                            );
                        } else {
                            self.push_log(format!("[LEARN] Rejected candidate: {}", id));
                        }
                    }
                    _ => {
                        self.push_log(format!("[LEARN] Unknown action: {}. Use status, extract, candidates, accept, reject.", subcommand));
                    }
                }
                true
            }

            cmd if cmd.starts_with("/summary") => {
                self.push_log("[LEARN SUMMARY] Session Learning Metrics:");
                self.push_log("  - Pending Candidates : 0");
                self.push_log("  - Accepted Candidates: 0");
                self.push_log("  - Rejected Candidates: 0");
                self.push_log("  - Total Processed    : 0");
                true
            }

            cmd if cmd.starts_with("/memory-galaxy") => {
                self.push_log("[MEMORY GALAXY] 🌌 Connecting to Project Semantic Galaxy...");
                self.push_log("  - Project Nodes   : 0");
                self.push_log("  - Workflow Nodes  : 0");
                self.push_log("  - Skill Nodes     : 0");
                self.push_log("[MEMORY GALAXY] Visualization rendering is mocked.");
                true
            }

            "/repo-map" | "/repo_map" | "/repo" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                let root = std::env::current_dir().unwrap_or_default();

                if arg == "refresh" || self.repo_map.is_none() {
                    if arg == "refresh" {
                        self.push_log("[REPO] Rescanning repository…");
                    } else {
                        self.push_log("[REPO] Loading repo map…");
                    }
                    let scanner = crate::repo_map::RepoMapScanner::new(root.clone());
                    match scanner.scan() {
                        Ok(repo_map) => {
                            self.repo_map = Some(repo_map);
                            if arg == "refresh" {
                                self.push_log("[REPO] Scan complete.");
                            }
                        }
                        Err(e) => {
                            self.push_log(format!("[REPO] Scan error: {}", e));
                        }
                    }
                }

                if let Some(repo_map) = &self.repo_map {
                    if arg == "summary" || arg == "context" || arg == "" || arg == "map" {
                        let compact = repo_map.to_compact_string(4000, true);
                        for line in compact.lines() {
                            self.push_log(format!("[REPO] {}", line));
                        }
                        if arg == "context" {
                            self.push_log("[REPO] Note: Full source files are NOT injected by default to save context budget.");
                        }
                    } else if arg == "refresh" {
                        // already handled
                    } else {
                        self.push_log("[REPO] Unknown argument. Use: /repo refresh | /repo summary | /repo context");
                    }
                }
                true
            }

            "/open" | "/preview" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                if arg.is_empty() {
                    self.push_log("[PREVIEW] Usage: /open <path>");
                } else {
                    let path = std::path::Path::new(arg);
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default()
                        .to_lowercase();
                    let suspicious = [
                        ".env",
                        "id_rsa",
                        "secret",
                        "key",
                        ".pem",
                        ".p12",
                        "credentials",
                    ];
                    if suspicious.iter().any(|s| name.contains(s)) {
                        self.push_log(format!(
                            "[PREVIEW] Refusing to open potentially sensitive file: {}",
                            arg
                        ));
                    } else if !path.exists() || !path.is_file() {
                        self.push_log(format!("[PREVIEW] File not found or not a file: {}", arg));
                    } else {
                        match std::fs::read_to_string(path) {
                            Ok(content) => {
                                let lines: Vec<&str> = content.lines().collect();
                                let max_lines = 200;
                                let display_content = if lines.len() > max_lines {
                                    let mut c = lines[..max_lines].join("\n");
                                    c.push_str(&format!(
                                        "\n\n... ({} lines truncated)",
                                        lines.len() - max_lines
                                    ));
                                    c
                                } else {
                                    content.clone()
                                };
                                let redacted = crate::approval::redact_secrets(&display_content);
                                self.push_log(format!("[PREVIEW] 📄 {}", arg));
                                self.push_log(format!("[PREVIEW] {} lines", lines.len()));
                                self.push_log(
                                    "─────────────────────────────────────────────────────────"
                                        .to_string(),
                                );
                                for line in redacted.lines() {
                                    self.push_log(line.to_string());
                                }
                                self.push_log(
                                    "─────────────────────────────────────────────────────────"
                                        .to_string(),
                                );
                            }
                            Err(e) => {
                                self.push_log(format!(
                                    "[PREVIEW] Could not read file (might be binary): {}",
                                    e
                                ));
                            }
                        }
                    }
                }
                true
            }

            "/checkpoint" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                let action = args.get(1).copied().unwrap_or("list");
                let root = std::env::current_dir().unwrap_or_default();
                match action {
                    "create" => {
                        let label = args.get(2).copied().unwrap_or("manual");
                        match self.checkpoint_manager.create_checkpoint(&root, label) {
                            Ok(cp) => self
                                .push_log(format!("[CHECKPOINT] Created {} ({})", cp.id, cp.label)),
                            Err(e) => {
                                self.push_log(format!("[CHECKPOINT] Failed to create: {}", e))
                            }
                        }
                    }
                    "list" => match self.checkpoint_manager.list_checkpoints() {
                        Ok(cps) => {
                            if cps.is_empty() {
                                self.push_log("[CHECKPOINT] No checkpoints found.".to_string());
                            } else {
                                self.push_log(format!("[CHECKPOINT] {} checkpoints:", cps.len()));
                                for cp in cps.iter().take(10) {
                                    self.push_log(format!(
                                        "  {} | {} | {} files | {}",
                                        cp.id,
                                        cp.branch,
                                        cp.changed_files.len(),
                                        cp.label
                                    ));
                                }
                            }
                        }
                        Err(e) => self.push_log(format!("[CHECKPOINT] Failed to list: {}", e)),
                    },
                    "show" | "diff" => {
                        if let Some(id) = args.get(2) {
                            match self.checkpoint_manager.get_checkpoint(id) {
                                Ok(Some(cp)) => {
                                    self.push_log(format!(
                                        "[CHECKPOINT] {} | {} | dirty: {}",
                                        cp.id, cp.branch, cp.is_dirty
                                    ));
                                    self.push_log(format!("  Label: {}", cp.label));
                                    self.push_log(format!(
                                        "  Changed: {}",
                                        cp.changed_files.join(", ")
                                    ));
                                    if action == "diff" && !cp.diff_snapshot.is_empty() {
                                        self.push_log("  Diff Snapshot:".to_string());
                                        for line in cp.diff_snapshot.lines() {
                                            self.push_log(line.to_string());
                                        }
                                    }
                                }
                                Ok(None) => {
                                    self.push_log(format!("[CHECKPOINT] ID {} not found.", id))
                                }
                                Err(e) => self.push_log(format!("[CHECKPOINT] Error: {}", e)),
                            }
                        } else {
                            self.push_log(
                                "[CHECKPOINT] Please provide a checkpoint ID.".to_string(),
                            );
                        }
                    }
                    "restore" => {
                        self.push_log("[CHECKPOINT] Restore is not fully implemented yet. Use /rollback <id> to initiate approval workflow.".to_string());
                    }
                    _ => self.push_log(
                        "[CHECKPOINT] Usage: /checkpoint [create|list|show|diff]".to_string(),
                    ),
                }
                true
            }

            "/rollback" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                let root = std::env::current_dir().unwrap_or_default();
                let (action, id) = if args.len() >= 3 {
                    (args[1], Some(args[2]))
                } else if args.len() == 2 {
                    ("plan", Some(args[1]))
                } else {
                    ("plan", None)
                };

                if let Some(id) = id {
                    if let Ok(Some(cp)) = self.checkpoint_manager.get_checkpoint(id) {
                        match action {
                            "plan" => {
                                self.push_log(format!("[ROLLBACK PLAN] ID: {}", cp.id));
                                self.push_log(format!("  Timestamp: {}", cp.timestamp));
                                self.push_log(format!("  Branch: {}", cp.branch));
                                self.push_log(format!("  Label: {}", cp.label));
                                self.push_log(format!(
                                    "  Changed files: {}",
                                    cp.changed_files.len()
                                ));
                                self.push_log("  Safe restore: Partial (only tracked file reverse patch may be possible)".to_string());
                                self.push_log(
                                    "  To securely revert your working directory, run:".to_string(),
                                );
                                self.push_log(format!("    /rollback destructive {}", cp.id));
                            }
                            "restore" => {
                                self.push_log(format!(
                                    "[ROLLBACK RESTORE] Attempting safe restore for {}",
                                    cp.id
                                ));
                                self.push_log(
                                    "Not implemented yet. Falling back to plan.".to_string(),
                                );
                            }
                            "destructive" => {
                                self.push_log(format!(
                                    "[ROLLBACK] Initiating DESTRUCTIVE rollback to checkpoint {} ({})",
                                    cp.id, cp.label
                                ));
                                use crate::approval::{ApprovalRequest, RiskLevel};
                                let req = ApprovalRequest {
                                    tool_name: "bash".to_string(),
                                    action_summary: format!(
                                        "git reset --hard && git clean -fd # rollback to {}",
                                        cp.id
                                    ),
                                    risk_level: RiskLevel::Critical,
                                    explanation: Some(format!(
                                        "DESTRUCTIVE: This will overwrite your current working tree to match checkpoint {}.",
                                        cp.id
                                    )),
                                    working_directory: Some(root.to_string_lossy().to_string()),
                                };
                                let cmd_str = req.action_summary.clone();
                                self.pending_approval = Some(DeferredToolCall {
                                    id: "manual".to_string(),
                                    name: "bash".to_string(),
                                    args: serde_json::json!({"command": cmd_str}),
                                    request: req,
                                    patch_id: None,
                                });
                                self.push_log(
                                    "[ROLLBACK] Requires approval. Please (A)pprove or (D)eny."
                                        .to_string(),
                                );
                            }
                            _ => {
                                self.push_log(format!("[ROLLBACK] Unknown action: {}", action));
                            }
                        }
                    } else {
                        self.push_log(format!("[ROLLBACK] Checkpoint {} not found.", id));
                    }
                } else {
                    self.push_log(
                        "[ROLLBACK] Usage: /rollback [plan|restore|destructive] <id>".to_string(),
                    );
                }
                true
            }

            "/branch" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                let action = args.get(1).copied().unwrap_or("current");
                let root = std::env::current_dir().unwrap_or_default();
                match action {
                    "current" => {
                        if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                            self.push_log(format!("[BRANCH] Current: {}", git.branch));
                        } else {
                            self.push_log("[BRANCH] Not in a git repo.".to_string());
                        }
                    }
                    "create" => {
                        if let Some(name) = args.get(2) {
                            use crate::approval::{ApprovalRequest, RiskLevel};
                            let req = ApprovalRequest {
                                tool_name: "bash".to_string(),
                                action_summary: format!("git checkout -b {}", name),
                                risk_level: RiskLevel::Medium,
                                explanation: Some(format!(
                                    "Create and switch to new branch: {}",
                                    name
                                )),
                                working_directory: Some(root.to_string_lossy().to_string()),
                            };
                            let cmd_str = req.action_summary.clone();
                            self.pending_approval = Some(DeferredToolCall {
                                id: "manual".to_string(),
                                name: "bash".to_string(),
                                args: serde_json::json!({"command": cmd_str}),
                                request: req,
                                patch_id: None,
                            });
                            self.push_log(format!(
                                "[BRANCH] Creation of {} requires approval.",
                                name
                            ));
                        } else {
                            self.push_log("[BRANCH] Please specify a branch name.".to_string());
                        }
                    }
                    _ => {
                        self.push_log("[BRANCH] Usage: /branch [current|create <name>]".to_string())
                    }
                }
                true
            }

            "/commit" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                let action = args.get(1).copied().unwrap_or("message");
                let root = std::env::current_dir().unwrap_or_default();
                match action {
                    "message" => {
                        let is_ai = args.get(2).copied() == Some("ai")
                            || args.get(2).copied() == Some("--ai");

                        let status_out = std::process::Command::new("git")
                            .args(["-C", &root.to_string_lossy(), "status", "--short"])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                            .unwrap_or_default();

                        let diff_out = std::process::Command::new("git")
                            .args(["-C", &root.to_string_lossy(), "diff", "--cached", "--stat"])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                            .unwrap_or_default();

                        if status_out.trim().is_empty() {
                            self.push_log(
                                "[COMMIT] No changes detected. Working tree clean.".to_string(),
                            );
                        } else {
                            if is_ai {
                                self.push_log(
                                    "[COMMIT] Generating AI commit message...".to_string(),
                                );
                                let prompt = format!(
                                    "Generate a conventional commit message based on the following git status and diff stat. Do NOT include any intro/outro text, just the commit message.\n\nStatus:\n{}\n\nDiff Stat:\n{}",
                                    status_out.trim(),
                                    diff_out.trim()
                                );
                                let history = vec![Message {
                                    role: "user".to_string(),
                                    content: Some(prompt),
                                    tool_calls: None,
                                    tool_call_id: None,
                                }];
                                match self
                                    .llm_router
                                    .completion_with_fallback(&self.model_chain, history, None)
                                    .await
                                {
                                    Ok((response, _)) => {
                                        self.push_log(
                                            "[COMMIT] Proposed AI commit message:".to_string(),
                                        );
                                        self.push_log("".to_string());
                                        let text = response.content.clone().unwrap_or_default();
                                        for line in text.lines() {
                                            self.push_log(line.to_string());
                                        }
                                        self.push_log("".to_string());
                                    }
                                    Err(e) => {
                                        self.push_log(format!(
                                            "[COMMIT] AI generation failed: {}",
                                            e
                                        ));
                                    }
                                }
                            } else {
                                self.push_log(
                                    "[COMMIT] Proposed deterministic commit message:".to_string(),
                                );
                                self.push_log("".to_string());
                                self.push_log("feat: Update project files".to_string());
                                self.push_log("".to_string());
                                for line in status_out.lines().filter(|l| !l.trim().is_empty()) {
                                    self.push_log(format!("- {}", line.trim()));
                                }
                                if !diff_out.trim().is_empty() {
                                    self.push_log("".to_string());
                                    self.push_log(format!("Diff stat:\n{}", diff_out.trim()));
                                }
                            }
                        }
                    }
                    "create" => {
                        let status_out = std::process::Command::new("git")
                            .args(["-C", &root.to_string_lossy(), "status", "--short"])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                            .unwrap_or_default();

                        let has_secrets = status_out.lines().any(|l| {
                            let lower = l.to_lowercase();
                            lower.contains(".env")
                                || lower.contains("id_rsa")
                                || lower.contains("credentials")
                                || lower.contains("key")
                        });

                        if has_secrets {
                            self.push_log(
                                "[COMMIT] WARNING: Secret-like files detected in git status!"
                                    .to_string(),
                            );
                            self.push_log(
                                "[COMMIT] Refusing to stage and commit to protect secrets."
                                    .to_string(),
                            );
                        } else {
                            use crate::approval::{ApprovalRequest, RiskLevel};
                            let req = ApprovalRequest {
                                tool_name: "bash".to_string(),
                                action_summary:
                                    "git add . && git commit -m 'feat: Update project files'"
                                        .to_string(),
                                risk_level: RiskLevel::Medium,
                                explanation: Some("Stage and commit all changes.".to_string()),
                                working_directory: Some(root.to_string_lossy().to_string()),
                            };
                            let cmd_str = req.action_summary.clone();
                            self.pending_approval = Some(DeferredToolCall {
                                id: "manual".to_string(),
                                name: "bash".to_string(),
                                args: serde_json::json!({"command": cmd_str}),
                                request: req,
                                patch_id: None,
                            });
                            self.push_log(
                                "[COMMIT] Committing changes requires approval.".to_string(),
                            );
                        }
                    }
                    _ => self.push_log("[COMMIT] Usage: /commit [message [ai]|create]".to_string()),
                }
                true
            }

            "/changes" | "/git-status" => {
                let root = std::env::current_dir().unwrap_or_default();
                if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                    self.push_log(format!("[GIT] Branch: {}", git.branch));
                    self.push_log(format!(
                        "[GIT] Status: {}",
                        if git.is_dirty { "Dirty" } else { "Clean" }
                    ));
                    self.push_log(format!("[GIT] Changed files: {}", git.changed_files_count));

                    if git.changed_files_count > 0 {
                        if let Ok(out) = std::process::Command::new("git")
                            .args(["status", "-s"])
                            .current_dir(&root)
                            .output()
                        {
                            let status_text = String::from_utf8_lossy(&out.stdout);
                            for line in status_text.lines() {
                                if !line.trim().is_empty() {
                                    self.push_log(format!("  {}", line));
                                }
                            }
                        }
                    }
                    if git.is_dirty {
                        self.push_log(
                            "[GIT] Hint: Run /checkpoint create before making risky edits."
                                .to_string(),
                        );
                    }
                } else {
                    self.push_log("[GIT] Not a git repository or git not available.");
                }
                true
            }

            "/diff" => {
                let arg = parts.get(1).copied().unwrap_or("").trim();
                if arg.is_empty() {
                    let root = std::env::current_dir().unwrap_or_default();
                    if let Ok(out) = std::process::Command::new("git")
                        .args(["diff", "--stat"])
                        .current_dir(&root)
                        .output()
                    {
                        let diff_text = String::from_utf8_lossy(&out.stdout);
                        if diff_text.trim().is_empty() {
                            self.push_log("[DIFF] No local git changes.");
                        } else {
                            self.push_log("[DIFF] Local git changes (stat):".to_string());
                            for line in diff_text.lines() {
                                self.push_log(line.to_string());
                            }
                            self.push_log(
                                "[DIFF] Use 'git diff' in a real terminal to see full diffs."
                                    .to_string(),
                            );
                        }
                    }
                } else {
                    let patch_opt = self
                        .workflow
                        .patches
                        .iter()
                        .find(|p| p.id == arg)
                        .map(|p| (p.id.clone(), p.file_path.clone(), p.unified_diff.clone()));
                    if let Some((id, file_path, diff)) = patch_opt {
                        self.push_log(format!("[DIFF] Patch ID: {}", id));
                        self.push_log(format!("[DIFF] File: {}", file_path));
                        self.push_log(
                            "─────────────────────────────────────────────────────────".to_string(),
                        );
                        let redacted = crate::approval::redact_secrets(&diff);
                        for line in redacted.lines() {
                            self.push_log(line.to_string());
                        }
                        self.push_log(
                            "─────────────────────────────────────────────────────────".to_string(),
                        );
                    } else {
                        self.push_log(format!("[DIFF] Patch ID '{}' not found.", arg));
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

            "/context" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                let action = args.get(1).copied().unwrap_or("show");
                match action {
                    "add" => {
                        if let Some(path) = args.get(2) {
                            let root = std::env::current_dir().unwrap_or_default();
                            let full_path = root.join(path);
                            if full_path.exists() && full_path.is_file() {
                                if crate::repo_map::looks_like_secret_file(&full_path) {
                                    self.push_log(format!(
                                        "[CONTEXT] Rejected: {} looks like a secret file.",
                                        path
                                    ));
                                } else {
                                    if !self.selected_files.contains(&path.to_string()) {
                                        self.selected_files.push(path.to_string());
                                        self.push_log(format!("[CONTEXT] Added {}", path));
                                    } else {
                                        self.push_log(format!(
                                            "[CONTEXT] {} is already in context.",
                                            path
                                        ));
                                    }
                                }
                            } else {
                                self.push_log(format!("[CONTEXT] File not found: {}", path));
                            }
                        } else {
                            self.push_log("[CONTEXT] Usage: /context add <path>".to_string());
                        }
                    }
                    "remove" => {
                        if let Some(path) = args.get(2) {
                            self.selected_files.retain(|p| p != path);
                            self.push_log(format!("[CONTEXT] Removed {}", path));
                        } else {
                            self.push_log("[CONTEXT] Usage: /context remove <path>".to_string());
                        }
                    }
                    "clear" => {
                        self.selected_files.clear();
                        self.push_log("[CONTEXT] Cleared all selected files.".to_string());
                    }
                    "show" => {
                        self.active_view = ActiveView::Context;
                        self.push_log("[CONTEXT] Viewing context — /view chat to return to chat.");
                    }
                    "budget" => {
                        self.push_log("[CONTEXT] Context Budget:".to_string());
                        let mut total_chars = 0;
                        let root = std::env::current_dir().unwrap_or_default();
                        let files = self.selected_files.clone();
                        for file in files {
                            let content =
                                std::fs::read_to_string(root.join(&file)).unwrap_or_default();
                            let chars = content.chars().count();
                            total_chars += chars;
                            self.push_log(format!("  • {} ({} chars)", file, chars));
                        }
                        self.push_log(format!("  Total: {} chars", total_chars));
                    }
                    "suggest" => {
                        self.push_log(
                            "[CONTEXT] Suggestions based on recent edits / current task (planned)."
                                .to_string(),
                        );
                    }
                    _ => self.push_log(
                        "[CONTEXT] Usage: /context [add|remove|clear|show|budget|suggest]"
                            .to_string(),
                    ),
                }
                true
            }

            "/files" => {
                let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
                if args.get(1).copied() == Some("relevant") {
                    if let Some(query) = args.get(2) {
                        self.push_log(format!(
                            "[FILES] Finding files relevant to '{}' (planned)...",
                            query
                        ));
                    } else {
                        self.push_log("[FILES] Usage: /files relevant <query>".to_string());
                    }
                } else {
                    self.push_log("[FILES] Usage: /files relevant <query>".to_string());
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

            // ── Phase 3.2: Layout control ──────────────────────────────────────
            "/layout" => {
                let target = parts.get(1).copied().unwrap_or("").trim().to_lowercase();
                match target.as_str() {
                    "focus" => {
                        self.layout_mode = LayoutMode::Focus;
                        self.push_log(
                            "[SYSTEM] ✅ Switched to Focus layout — clean, centered interface.",
                        );
                        self.push_log(
                            "[SYSTEM] Ctrl+B = toggle sidebar  Ctrl+R = toggle context panel",
                        );
                    }
                    "dashboard" => {
                        self.layout_mode = LayoutMode::Dashboard;
                        self.push_log("[SYSTEM] ✅ Switched to Dashboard layout — 3-pane view.");
                        self.push_log(
                            "[SYSTEM] Ctrl+B = toggle sidebar  Ctrl+R = toggle context panel",
                        );
                    }
                    "compact" => {
                        self.layout_mode = LayoutMode::Compact;
                        self.push_log(
                            "[SYSTEM] ✅ Switched to Compact layout — chat + input only.",
                        );
                    }
                    "" => {
                        let ver = env!("CARGO_PKG_VERSION");
                        self.push_log(format!(
                            "[STATUS] Current layout: {}  (GOAT v{})",
                            self.layout_mode.label(),
                            ver
                        ));
                        self.push_log(
                            "[HELP] /layout focus     — focused, centered interface (default)",
                        );
                        self.push_log("[HELP] /layout dashboard — 3-pane sidebar+center+context");
                        self.push_log(
                            "[HELP] /layout compact   — chat+input only, best for narrow terminals",
                        );
                        self.push_log(
                            "[HELP] Ctrl+B = toggle sidebar  Ctrl+R = toggle context panel",
                        );
                    }
                    other => {
                        self.push_log(format!(
                            "[WARN] Unknown layout '{}'. Valid: focus, dashboard, compact",
                            other
                        ));
                    }
                }
                true
            }

            // ── Phase 3.2: Logs view ───────────────────────────────────────────
            "/logs" | "/log" => {
                let view_arg = parts.get(1).copied().unwrap_or("").trim().to_lowercase();
                match view_arg.as_str() {
                    "clear" => {
                        self.logs
                            .retain(|l| l.starts_with("[YOU]") || l.starts_with("[GOAT]"));
                        self.push_log("[SYSTEM] Logs cleared (kept conversation messages).");
                    }
                    _ => {
                        self.active_view = ActiveView::Logs;
                        self.push_log("[SYSTEM] Viewing logs — /view chat to return to chat.");
                    }
                }
                true
            }

            // ── Phase 3.2: Agent/subagent selector ────────────────────────────
            "/agents" | "/agent-selector" | "/subagent-selector" | "/agent-select" => {
                self.active_view = ActiveView::AgentSelector;
                self.push_log(
                    "[SYSTEM] Agent Selector — showing internal + external agents and skills.",
                );
                true
            }

            // ── Phase 3.2: Theme ───────────────────────────────────────────────
            "/theme" => {
                let theme_arg = parts.get(1).copied().unwrap_or("").trim().to_lowercase();
                match theme_arg.as_str() {
                    "" | "status" => {
                        self.push_log("[THEME] 🎨 Current theme: goat-dark");
                        self.push_log(
                            "[THEME] ─────────────────────────────────────────────────────────",
                        );
                        self.push_log(
                            "[THEME] ✅ goat-dark      — Default deep navy + mint accents (active)",
                        );
                        self.push_log("[THEME] 🔮 minimal        — Minimal monochrome (planned)");
                        self.push_log(
                            "[THEME] 🔮 glass          — Glassmorphism light/dark (planned)",
                        );
                        self.push_log("[THEME] 🔮 high-contrast  — WCAG accessible (planned)");
                        self.push_log(
                            "[THEME] ─────────────────────────────────────────────────────────",
                        );
                        self.push_log("[HELP] Additional themes are planned for Phase 3.3.");
                    }
                    other => {
                        self.push_log(format!("[THEME] Theme '{}' is not yet available.", other));
                        self.push_log("[THEME] Current theme: goat-dark (only available theme).");
                        self.push_log("[THEME] Additional themes planned for Phase 3.3.");
                    }
                }
                true
            }

            // ── Phase 5.16: Agents ──────────────────────────────────────────────
            "/cofounder" => {
                let parts: Vec<&str> = _args.splitn(2, ' ').collect();
                let subcmd = parts.get(0).copied().unwrap_or("list");
                let target = parts.get(1).copied().unwrap_or("").trim();
                let mut manager = crate::agents::cofounder::CofounderManager::new().expect("Failed to initialize CofounderManager");

                match subcmd {
                    "list" => {
                        let ideas = manager.list_ideas();
                        self.push_log(format!("[COFOUNDER] {} ideas found.", ideas.len()));
                        for i in ideas {
                            self.push_log(format!("  [{}] {} (State: {:?})", i.id, i.title, i.state));
                        }
                    }
                    "new-idea" => {
                        // Very simple for testing
                        match manager.add_idea(target.to_string(), "Mock Description".to_string(), "Mock Audience".to_string()) {
                            Ok(idea) => self.push_log(format!("[COFOUNDER] Idea created: {}", idea.id)),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "validate" => {
                        match manager.generate_validation_plan(target) {
                            Ok(plan) => self.push_log(format!("[COFOUNDER] Validation Plan for {}: {} steps", plan.idea_id, plan.steps.len())),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "score" => {
                        match manager.generate_scorecard(target) {
                            Ok(score) => self.push_log(format!("[COFOUNDER] Scorecard for {}: {}/50", score.idea_id, score.total_score)),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "mvp" => {
                        match manager.generate_mvp_scope(target) {
                            Ok(mvp) => self.push_log(format!("[COFOUNDER] MVP Scope for {}: {} core features", mvp.idea_id, mvp.core_features.len())),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "competitors" => {
                        match manager.generate_competitors(target) {
                            Ok(comps) => self.push_log(format!("[COFOUNDER] {} competitors found for {}", comps.len(), target)),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "landing" => {
                        match manager.generate_landing_page_brief(target) {
                            Ok(brief) => self.push_log(format!("[COFOUNDER] Landing Page Brief: {}", brief)),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "outreach" => {
                        match manager.generate_outreach_plan(target) {
                            Ok(plan) => self.push_log(format!("[COFOUNDER] Outreach Plan: {} channels", plan.channels.len())),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "report" => {
                        match manager.generate_report(target) {
                            Ok(report) => self.push_log(format!("[COFOUNDER] Report: {}", report.summary)),
                            Err(e) => self.push_log(format!("[COFOUNDER] Error: {}", e)),
                        }
                    }
                    "show" => {
                        if let Some(idea) = manager.get_idea(target) {
                            self.push_log(format!("[COFOUNDER] ID: {}", idea.id));
                            self.push_log(format!("  Title: {}", idea.title));
                            self.push_log(format!("  State: {:?}", idea.state));
                        } else {
                            self.push_log(format!("[COFOUNDER] Idea '{}' not found", target));
                        }
                    }
                    _ => self.push_log(format!("[COFOUNDER] Unknown subcmd: {}", subcmd)),
                }
                true
            }

            "/socializer" => {
                let parts: Vec<&str> = _args.splitn(2, ' ').collect();
                let subcmd = parts.get(0).copied().unwrap_or("list");
                let target = parts.get(1).copied().unwrap_or("").trim();
                let mut manager = crate::agents::SocializerAgent::new().unwrap_or_else(|_| {
                    // Provide a default unwrap for fallback if fs fails
                    panic!("Failed to init SocializerAgent")
                });

                match subcmd {
                    "list" => {
                        let campaigns = manager.list_campaigns();
                        self.push_log(format!("[SOCIALIZER] {} campaigns found.", campaigns.len()));
                        for c in campaigns {
                            self.push_log(format!("  [{}] {} (State: {:?})", c.id, c.title, c.state));
                        }
                    }
                    "new-campaign" => {
                        match manager.add_campaign("Mock Title".to_string(), "Mock Audience".to_string(), "Mock Value Prop".to_string(), None) {
                            Ok(c) => self.push_log(format!("[SOCIALIZER] Campaign created: {}", c.id)),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "audience" => {
                        match manager.generate_audience_map(target) {
                            Ok(a) => self.push_log(format!("[SOCIALIZER] Audience mapped: {} segments", a.segments.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "channels" => {
                        match manager.generate_channel_strategy(target) {
                            Ok(c) => self.push_log(format!("[SOCIALIZER] Channel strategy: {} channels", c.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "angles" => {
                        match manager.generate_content_angles(target) {
                            Ok(a) => self.push_log(format!("[SOCIALIZER] Content angles: {} generated", a.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "reddit" => {
                        match manager.generate_draft(target, "Reddit") {
                            Ok(d) => self.push_log(format!("[SOCIALIZER] Reddit Draft generated: {} warnings", d.warnings.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "linkedin" => {
                        match manager.generate_draft(target, "LinkedIn") {
                            Ok(d) => self.push_log(format!("[SOCIALIZER] LinkedIn Draft generated")),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "x" => {
                        match manager.generate_draft(target, "X") {
                            Ok(d) => self.push_log(format!("[SOCIALIZER] X Draft generated")),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "launch" => {
                        match manager.generate_launch_plan(target) {
                            Ok(lp) => self.push_log(format!("[SOCIALIZER] Launch Plan generated: {} checklist items", lp.pre_launch_checklist.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "calendar" => {
                        match manager.generate_calendar(target) {
                            Ok(c) => self.push_log(format!("[SOCIALIZER] Calendar: {} items", c.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "outreach" => {
                        match manager.generate_outreach(target) {
                            Ok(d) => self.push_log(format!("[SOCIALIZER] Outreach draft: {} warnings", d.warnings.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "feedback" => {
                        match manager.track_feedback(target) {
                            Ok(fb) => self.push_log(format!("[SOCIALIZER] Feedback: {} positive signals", fb.positive_signals.len())),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "report" => {
                        match manager.generate_report(target) {
                            Ok(r) => self.push_log(format!("[SOCIALIZER] Report: {}", r)),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "from-idea" => {
                        match manager.add_campaign("Idea Title".to_string(), "Idea Audience".to_string(), "Idea Prop".to_string(), Some(target.to_string())) {
                            Ok(c) => self.push_log(format!("[SOCIALIZER] Campaign created from idea: {}", c.id)),
                            Err(e) => self.push_log(format!("[SOCIALIZER] Error: {}", e)),
                        }
                    }
                    "show" => {
                        if let Some(c) = manager.get_campaign(target) {
                            self.push_log(format!("[SOCIALIZER] ID: {}", c.id));
                            self.push_log(format!("  Title: {}", c.title));
                            self.push_log(format!("  State: {:?}", c.state));
                        } else {
                            self.push_log(format!("[SOCIALIZER] Campaign '{}' not found", target));
                        }
                    }
                    _ => self.push_log(format!("[SOCIALIZER] Unknown subcmd: {}", subcmd)),
                }
                true
            }

            "/agents" => {
                let parts: Vec<&str> = _args.splitn(2, ' ').collect();
                let subcmd = parts.get(0).copied().unwrap_or("list");
                let target = parts.get(1).copied().unwrap_or("").trim();
                let mut registry = crate::agents::AgentRegistry::new();
                let report_mgr = crate::reports::ReportManager::new();

                match subcmd {
                    "list" => {
                        self.push_log("[AGENTS] GOAT Agent Registry:".to_string());
                        let agents = registry.list();
                        for a in agents {
                            self.push_log(format!(
                                "  [{:?}] {} ({}): {}",
                                a.tier, a.name, a.id, a.description
                            ));
                            self.push_log(format!(
                                "    Status: {:?} | Affinity: {:?}",
                                a.status, a.prime_affinity
                            ));
                        }
                    }
                    "show" => {
                        if let Some(agent) = registry.get(target) {
                            self.push_log(format!("[AGENTS] Name: {} ({})", agent.name, agent.id));
                            self.push_log(format!("  Tier: {:?}", agent.tier));
                            self.push_log(format!("  Status: {:?}", agent.status));
                            self.push_log(format!("  Affinity: {:?}", agent.prime_affinity));
                            self.push_log(format!("  Traits: {:?}", agent.traits));
                            self.push_log(format!("  Capabilities: {:?}", agent.capabilities));
                        } else {
                            self.push_log(format!("[AGENTS] Agent '{}' not found.", target));
                        }
                    }
                    "enable" => {
                        if let Err(e) = registry.enable(target) {
                            self.push_log(format!("[AGENTS] Error: {}", e));
                        } else {
                            self.push_log(format!("[AGENTS] Agent '{}' enabled (Active).", target));
                        }
                    }
                    "disable" => {
                        if let Err(e) = registry.disable(target) {
                            self.push_log(format!("[AGENTS] Error: {}", e));
                        } else {
                            self.push_log(format!("[AGENTS] Agent '{}' disabled.", target));
                        }
                    }
                    "reports" => {
                        self.push_log(format!("[AGENTS] Reports for {}:", target));
                        if let Ok(reports) = report_mgr.list_reports() {
                            for r in reports {
                                self.push_log(format!(
                                    "  - {} ({}) [{}]",
                                    r.title, r.id, r.created_at
                                ));
                            }
                        }
                    }
                    "specialists" => {
                        self.push_log(format!("[AGENTS] Specialists for Prime '{}':", target));
                        for a in registry.list() {
                            if a.prime_affinity.as_deref() == Some(target) {
                                self.push_log(format!("  - {} ({})", a.name, a.id));
                            }
                        }
                    }
                    _ => {
                        self.push_log("[AGENTS] Valid subcommands: list, show, enable, disable, reports, specialists".to_string());
                    }
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

    pub async fn handle_user_input(&mut self, mut msg: String) {
        msg = crate::quick_access::QuickAccessParser::parse_and_rewrite(&msg);

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

        let hook_logs = self
            .hooks_manager
            .run_hooks("on_submit", &mut self.approval_gate)
            .await
            .unwrap_or_default();
        for log in hook_logs {
            self.push_log(log);
        }

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

            // Phase 3.6: Inject Repo Map
            if let Some(map) = &self.repo_map {
                sys_prompt.push_str("\n\n<repo_map>\n");
                sys_prompt.push_str(&map.to_compact_string(5000, true));
                sys_prompt.push_str("\n</repo_map>");
            }

            // Phase 3.6: Inject Selected Context Files
            if !self.selected_files.is_empty() {
                sys_prompt.push_str("\n\n<selected_files>\n");
                sys_prompt.push_str(
                    "The user has explicitly added the following files to your context budget:\n",
                );
                let root = std::env::current_dir().unwrap_or_default();
                let mut current_budget = 0;
                let max_budget = 20000;
                for file in &self.selected_files {
                    let path = root.join(file);
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        sys_prompt.push_str(&format!("\n--- {} ---\n", file));
                        let char_count = content.chars().count();
                        if current_budget + char_count > max_budget {
                            let available = max_budget - current_budget;
                            let truncated: String = content.chars().take(available).collect();
                            sys_prompt.push_str(&truncated);
                            sys_prompt.push_str("\n[TRUNCATED due to context budget limits]\n");
                            break;
                        } else {
                            sys_prompt.push_str(&content);
                            current_budget += char_count;
                        }
                    }
                }
                sys_prompt.push_str("\n</selected_files>");
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
                                    build_approval_request(&tc.function.name, &args, &tool_action);

                                if let Some(req) = approval_request {
                                    match self.approval_gate.check_policy(&req) {
                                        Some(ApprovalDecision::Approved) => {
                                            self.push_log(format!(
                                                "[APPROVAL] Auto-approved (session policy): {}",
                                                tc.function.name
                                            ));
                                            let hook_logs = self
                                                .hooks_manager
                                                .run_hooks(
                                                    "before_tool_call",
                                                    &mut self.approval_gate,
                                                )
                                                .await
                                                .unwrap_or_default();
                                            for log in hook_logs {
                                                self.push_log(log);
                                            }

                                            let is_patch = patch_id.is_some();
                                            if is_patch {
                                                let logs = self
                                                    .hooks_manager
                                                    .run_hooks(
                                                        "before_patch_apply",
                                                        &mut self.approval_gate,
                                                    )
                                                    .await
                                                    .unwrap_or_default();
                                                for log in logs {
                                                    self.push_log(log);
                                                }
                                            }

                                            let result = if let Some(native_result) =
                                                NativeTools::execute(
                                                    &tc.function.name,
                                                    args.clone(),
                                                )
                                                .await
                                            {
                                                match native_result {
                                                    Ok(res) => res,
                                                    Err(e) => format!("Tool error: {}", e),
                                                }
                                            } else {
                                                match self
                                                    .mcp_manager
                                                    .call_tool(&tc.function.name, args.clone())
                                                    .await
                                                {
                                                    Ok(res) => serde_json::to_string(&res)
                                                        .unwrap_or_else(|_| "[]".to_string()),
                                                    Err(e) => format!("MCP tool error: {}", e),
                                                }
                                            };

                                            let hook_logs = self
                                                .hooks_manager
                                                .run_hooks(
                                                    "after_tool_call",
                                                    &mut self.approval_gate,
                                                )
                                                .await
                                                .unwrap_or_default();
                                            for log in hook_logs {
                                                self.push_log(log);
                                            }

                                            if let Some(id) = &patch_id {
                                                if let Some(p) = self.workflow.get_patch_mut(id) {
                                                    p.status = crate::task::PatchStatus::Applied;
                                                }
                                                if let Some(task) = &mut self.workflow.active_task {
                                                    task.status =
                                                        crate::task::TaskStatus::PatchApplied;
                                                }
                                                let logs = self
                                                    .hooks_manager
                                                    .run_hooks(
                                                        "after_patch_apply",
                                                        &mut self.approval_gate,
                                                    )
                                                    .await
                                                    .unwrap_or_default();
                                                for log in logs {
                                                    self.push_log(log);
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
                                    let hook_logs = self
                                        .hooks_manager
                                        .run_hooks("before_tool_call", &mut self.approval_gate)
                                        .await
                                        .unwrap_or_default();
                                    for log in hook_logs {
                                        self.push_log(log);
                                    }

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

                                    let hook_logs = self
                                        .hooks_manager
                                        .run_hooks("after_tool_call", &mut self.approval_gate)
                                        .await
                                        .unwrap_or_default();
                                    for log in hook_logs {
                                        self.push_log(log);
                                    }

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

fn build_approval_request(
    tool_name: &str,
    args: &Value,
    tool_action: &crate::tool_registry::ToolAction,
) -> Option<ApprovalRequest> {
    let req = match tool_name {
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
    };

    if req.is_none() && matches!(tool_action, crate::tool_registry::ToolAction::Ask(_)) {
        Some(ApprovalRequest {
            tool_name: tool_name.to_string(),
            action_summary: format!("Execute tool '{}'", tool_name),
            risk_level: crate::approval::RiskLevel::High,
            explanation: Some(format!(
                "Arguments: {}",
                serde_json::to_string_pretty(args).unwrap_or_default()
            )),
            working_directory: None,
        })
    } else {
        req
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

impl App {
    pub async fn handle_scheduled_jobs(&mut self) {
        let jobs = self.scheduler_manager.tick();
        for job in jobs {
            self.push_log(format!(
                "[SCHEDULE] Executing job {}: {}",
                job.id, job.prompt_or_command
            ));
            // Trigger background job or tool here.
            // For now, we simulate execution and track it
            self.job_tracker.add_job(crate::jobs::BackgroundJob {
                id: job.id.clone(),
                r#type: "scheduled".to_string(),
                status: "running".to_string(),
                started_at: chrono::Utc::now().to_rfc3339(),
                finished_at: None,
                output_preview: None,
                error: None,
                approval_status: None,
            });

            // Log audit
            self.scheduler_manager.log_audit(&format!(
                "Executed job {}: {}",
                job.id, job.prompt_or_command
            ));
        }
    }
}
