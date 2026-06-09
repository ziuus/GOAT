use crate::approval::{
    ApprovalDecision, ApprovalGate, ApprovalRequest, bash_approval_request,
    call_subagent_approval_request, write_file_approval_request,
};
use crate::brain::Brain;
use crate::config::Config;
use crate::llm::{FunctionDeclaration, LlmRouter, Message, Tool};
use crate::mcp::McpManager;
use crate::models::ModelChain;
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
    /// Number of running MCP servers.
    pub mcp_server_count: usize,
    /// The approval gate for this session.
    pub approval_gate: ApprovalGate,
    /// Whether brain was disabled via --no-brain.
    pub brain_disabled: bool,
    /// Pending approval (Some ↔ approval overlay visible).
    pending_approval: Option<DeferredToolCall>,
}

impl App {
    /// Create `App` from a pre-bootstrapped `GoatRuntime`.
    ///
    /// This is the preferred constructor used in production.
    /// `boot_log` contains messages from the bootstrap phase (brain connection,
    /// session resume, security notices) to display in the TUI at startup.
    pub fn from_runtime(rt: GoatRuntime, boot_log: Vec<String>) -> Self {
        let mut logs: Vec<String> = Vec::new();

        // TUI splash header.
        logs.push(
            "[GOAT] v0.3 — Universal AI Agent Platform | Type your message and press Enter"
                .to_string(),
        );
        logs.push(
            "[GOAT] Slash commands: /help /status /mcp /learn /route /clear /tools /sessions"
                .to_string(),
        );
        logs.push("[GOAT] Keys: Enter send · Ctrl+C quit · ↑↓ scroll log · Esc cancel".to_string());

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
            mcp_server_count,
            approval_gate: rt.approval_gate,
            brain_disabled,
            pending_approval: None,
        }
    }

    /// Create a new `App` directly from config + paths.
    ///
    /// Used in tests and as a convenience wrapper.
    /// Production code should prefer `from_runtime()` to avoid duplicating
    /// bootstrap logic.
    pub fn new(config: Config, paths: GoatPaths, startup_warnings: Vec<String>) -> Self {
        let (runtime, boot_log) = GoatRuntime::bootstrap(config, paths, startup_warnings, false);
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
            "/help" => {
                self.push_log("[HELP] Available commands:");
                self.push_log("[HELP]   /help         — show this help");
                self.push_log("[HELP]   /status       — show system status");
                self.push_log("[HELP]   /mcp          — start configured MCP servers");
                self.push_log("[HELP]   /learn        — index project files into brain");
                self.push_log("[HELP]   /route        — show current swarm route for input");
                self.push_log("[HELP]   /clear        — clear the log display");
                self.push_log("[HELP]   /tools        — list available tools");
                self.push_log("[HELP]   /sessions     — show session info");
                self.push_log("[HELP]");
                self.push_log("[HELP] Keys:");
                self.push_log("[HELP]   Enter         — send message");
                self.push_log("[HELP]   Ctrl+C        — quit");
                self.push_log("[HELP]   ↑/↓           — scroll log");
                self.push_log("[HELP]   Page Up/Down  — fast scroll");
                self.push_log("[HELP]   Esc           — cancel input");
                self.push_log("[HELP]");
                self.push_log("[HELP] Approval (when overlay appears):");
                self.push_log("[HELP]   y — approve once");
                self.push_log("[HELP]   n — deny");
                self.push_log("[HELP]   a — always allow this tool (session)");
                self.push_log("[HELP]   d — always deny this tool (session)");
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
                    "[STATUS] History  : {} messages",
                    self.history.len()
                ));
                self.push_log(format!(
                    "[STATUS] MCP      : {} server(s)",
                    self.mcp_server_count
                ));
                true
            }

            "/mcp" => {
                self.push_log("[MCP] Starting configured MCP servers...");
                info!("starting configured MCP servers via slash command");
                self.start_configured_mcp_servers().await;
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

            "/clear" => {
                self.logs.clear();
                self.log_scroll = 0;
                self.push_log("[GOAT] Log cleared. Type a message to continue.");
                true
            }

            "/tools" => {
                let tools = NativeTools::all_tools();
                self.push_log(format!("[TOOLS] {} native tools available:", tools.len()));
                for t in &tools {
                    self.push_log(format!(
                        "[TOOLS]   {} — {}",
                        t.function.name, t.function.description
                    ));
                }
                let mcp_tools = self.mcp_manager.all_tools();
                if !mcp_tools.is_empty() {
                    self.push_log(format!("[TOOLS] {} MCP tools available:", mcp_tools.len()));
                    for t in &mcp_tools {
                        if let Some(name) = t.get("name").and_then(|v| v.as_str()) {
                            self.push_log(format!("[TOOLS]   {}", name));
                        }
                    }
                }
                true
            }

            "/sessions" => {
                self.push_log(format!("[SESSION] Current session ID: {}", self.session_id));
                if let Some(ref brain) = self.brain {
                    match brain.get_sessions() {
                        Ok(sessions) => {
                            self.push_log(format!(
                                "[SESSION] {} session(s) in brain:",
                                sessions.len()
                            ));
                            for (id, label) in sessions.iter().take(10) {
                                self.push_log(format!("[SESSION]   {} — {}", id, label));
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

            _ => false,
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

            let mut routed_history = vec![Message {
                role: "system".to_string(),
                content: Some(route.profile.system_prompt.to_string()),
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
                                            self.push_log(format!("[TOOL] {}", result));
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
                                            self.history.push(Message {
                                                role: "tool".to_string(),
                                                content: Some(format!(
                                                    "Tool execution denied (session policy). Reason: {}",
                                                    reason
                                                )),
                                                tool_calls: None,
                                                tool_call_id: Some(tc.id),
                                            });
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
            Some(write_file_approval_request(path, content))
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
