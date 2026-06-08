use crate::approval::{
    ApprovalDecision, ApprovalGate, ApprovalRequest, bash_approval_request,
    call_subagent_approval_request, write_file_approval_request,
};
use crate::brain::Brain;
use crate::config::Config;
use crate::llm::{FunctionDeclaration, LlmRouter, Message, Tool};
use crate::mcp::McpManager;
use crate::swarm::{RouteDecision, SwarmRouter};
use crate::tools::NativeTools;
use serde_json::Value;
use std::path::PathBuf;
use tracing::info;

const MAX_LOG_LINES: usize = 500;
const MAX_HISTORY_MESSAGES: usize = 80;

/// A tool call that has been deferred pending user approval.
///
/// When the agent wants to run a dangerous tool, we store the call here,
/// surface the approval prompt in the TUI, and resume execution once the user
/// responds (in the main event loop via [`App::resolve_approval`]).
struct DeferredToolCall {
    /// The JSON-RPC-style tool call ID (used to build the tool-result message).
    id: String,
    /// The tool name.
    name: String,
    /// The parsed arguments.
    args: Value,
    /// The approval request that was surfaced to the user.
    request: ApprovalRequest,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    pub current_task: String,
    pub input: String,
    pub input_mode: InputMode,
    pub brain: Option<Brain>,
    pub llm_router: LlmRouter,
    pub mcp_manager: McpManager,
    pub history: Vec<Message>,
    pub config: Config,
    pub swarm_router: SwarmRouter,
    pub active_route: Option<RouteDecision>,
    pub session_id: String,
    /// The approval gate for this session.  Holds per-tool session policies.
    pub approval_gate: ApprovalGate,
    /// When `Some`, a tool call is paused waiting for the user to approve/deny.
    /// The TUI renders an approval banner and the event loop routes y/n/a/d here.
    pending_approval: Option<DeferredToolCall>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let brain = Brain::new("goat_brain.db").ok();
        let mut logs = vec![
            "[SYSTEM] GOAT Engine Initialized.".to_string(),
            "[SYSTEM] Awaiting MCP connections...".to_string(),
        ];

        let mut session_id = format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let mut history = Vec::new();

        if let Some(ref b) = brain {
            logs.push("[SYSTEM] Brain connected (SQLite).".to_string());
            if let Ok(sessions) = b.get_sessions() {
                if let Some((latest_id, _)) = sessions.first() {
                    session_id = latest_id.clone();
                    logs.push(format!("[SYSTEM] Resumed session: {}", session_id));
                    if let Ok(loaded_history) = b.load_session_history(&session_id) {
                        for (role, content) in loaded_history {
                            history.push(Message {
                                role,
                                content: Some(content),
                                tool_calls: None,
                                tool_call_id: None,
                            });
                        }
                    }
                } else {
                    let _ = b.create_session(&session_id, "New Session");
                    logs.push(format!("[SYSTEM] Created session: {}", session_id));
                }
            }
        } else {
            logs.push("[ERROR] Failed to initialize Brain (SQLite).".to_string());
        }

        let llm_router = LlmRouter::new(
            config.keys.openai_api_key.clone(),
            config.keys.groq_api_key.clone(),
        );

        // Security notice in startup log
        logs.push("[SECURITY] Approval gate active: bash, write_file, call_subagent require your confirmation.".to_string());
        logs.push(
            "[SECURITY] Keys: [y] approve  [n] deny  [a] always allow  [d] always deny".to_string(),
        );

        Self {
            running: true,
            logs,
            current_task: "Ready for mission...".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            brain,
            llm_router,
            mcp_manager: McpManager::new(),
            history,
            config,
            swarm_router: SwarmRouter::default(),
            active_route: None,
            session_id,
            approval_gate: ApprovalGate::new(),
            pending_approval: None,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn push_log(&mut self, log: impl Into<String>) {
        self.logs.push(log.into());
        self.trim_logs();
    }

    pub fn extend_logs(&mut self, logs: impl IntoIterator<Item = String>) {
        self.logs.extend(logs);
        self.trim_logs();
    }

    fn trim_logs(&mut self) {
        let extra = self.logs.len().saturating_sub(MAX_LOG_LINES);
        if extra > 0 {
            self.logs.drain(0..extra);
        }
    }

    fn trim_history(&mut self) {
        let extra = self.history.len().saturating_sub(MAX_HISTORY_MESSAGES);
        if extra > 0 {
            self.history.drain(0..extra);
        }
    }

    // ── Approval gate integration ────────────────────────────────────────────

    /// Returns `true` when a tool call is paused waiting for user approval.
    pub fn has_pending_approval(&self) -> bool {
        self.pending_approval.is_some()
    }

    /// Returns a reference to the current approval request display lines, if any.
    /// Used by `ui.rs` to render the approval banner.
    pub fn pending_approval_lines(&self) -> Option<Vec<String>> {
        self.pending_approval
            .as_ref()
            .map(|d| d.request.display_lines())
    }

    /// Resolve the pending approval with the user's input character.
    ///
    /// If approved: execute the deferred tool call, push the result to history,
    /// and continue the agent loop.
    /// If denied: push a denial message to history so the LLM can adapt.
    ///
    /// This is called from the TUI event loop in `main.rs` when the user presses
    /// a key while `has_pending_approval()` is true.
    pub async fn resolve_approval(&mut self, input: char) {
        let deferred = match self.pending_approval.take() {
            Some(d) => d,
            None => return,
        };

        let decision = self.approval_gate.resolve(&deferred.request, input);

        match decision {
            ApprovalDecision::Approved => {
                info!(
                    tool = %deferred.name,
                    "approval granted — executing deferred tool call"
                );
                self.push_log(format!(
                    "[APPROVAL] Approved: {} — {}",
                    deferred.name, deferred.request.action_summary
                ));

                // Execute the tool now that we have approval.
                let tool_result = execute_native_tool(&deferred.name, deferred.args.clone()).await;
                self.push_log(format!("[TOOL] Result: {}", tool_result));

                self.history.push(Message {
                    role: "tool".to_string(),
                    content: Some(tool_result),
                    tool_calls: None,
                    tool_call_id: Some(deferred.id),
                });
                self.trim_history();

                self.current_task = "Agent resumed after approval".to_string();
            }
            ApprovalDecision::Denied(reason) => {
                info!(
                    tool = %deferred.name,
                    reason = %reason,
                    "approval denied — tool call blocked"
                );
                self.push_log(format!("[APPROVAL] Denied: {} — {}", deferred.name, reason));

                // Push a tool-result message explaining the denial so the LLM
                // knows to adapt its plan.
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

                self.current_task = "Ready for mission...".to_string();
            }
        }
    }

    // ── MCP server management ────────────────────────────────────────────────

    pub async fn start_configured_mcp_servers(&mut self) {
        let logs = self
            .mcp_manager
            .start_configured(&self.config.mcp_servers)
            .await;
        self.extend_logs(logs);
    }

    pub fn show_mcp_status(&mut self) {
        let running = self.mcp_manager.running_servers();
        if running.is_empty() {
            self.push_log("[MCP] No MCP servers are running.");
        } else {
            self.push_log(format!("[MCP] Running servers: {}", running.join(", ")));
        }
    }

    pub async fn shutdown_mcp_servers(&mut self) {
        let logs = self.mcp_manager.shutdown_all().await;
        self.extend_logs(logs);
    }

    // ── Project indexer ──────────────────────────────────────────────────────

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
            self.push_log("[BRAIN ERROR] Brain is not connected.");
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
            Err(err) => format!("[BRAIN ERROR] Learn About Me failed: {}", err),
        };
        self.push_log(result_log);
    }

    // ── Swarm router ─────────────────────────────────────────────────────────

    pub fn route_current_input(&mut self) {
        let candidate = if self.input.trim().is_empty() {
            self.current_task.as_str()
        } else {
            self.input.as_str()
        };

        let decision = self.swarm_router.route(candidate);
        self.push_log(format!(
            "[SWARM] Routed to {} ({:?}) confidence {}%: {}",
            decision.profile.name, decision.profile.kind, decision.confidence, decision.reason
        ));
        self.current_task = format!("{} agent selected", decision.profile.name);
        self.active_route = Some(decision);
    }

    // ── Main agent loop ──────────────────────────────────────────────────────

    /// Handle a user message: route it through the swarm, call the LLM, and
    /// dispatch any tool calls the LLM requests — pausing for approval on
    /// dangerous tools.
    pub async fn handle_user_input(&mut self, msg: String) {
        self.push_log(format!("[USER] {}", msg));

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
            // Build tool list from native + MCP tools.
            let mcp_tools = self.mcp_manager.all_tools();
            let mut mapped_tools = NativeTools::all_tools();

            if !mcp_tools.is_empty() {
                for tool in mcp_tools {
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
            }

            let tools = if mapped_tools.is_empty() {
                None
            } else {
                Some(mapped_tools)
            };

            // Route and build history with system prompt.
            let route = self.swarm_router.route(
                self.history
                    .last()
                    .and_then(|m| m.content.as_deref())
                    .unwrap_or_default(),
            );
            self.active_route = Some(route.clone());
            self.current_task = format!("{} agent working", route.profile.name);

            let mut routed_history = vec![Message {
                role: "system".to_string(),
                content: Some(route.profile.system_prompt.to_string()),
                tool_calls: None,
                tool_call_id: None,
            }];
            routed_history.extend(self.history.clone());

            // ── Call LLM ──────────────────────────────────────────────────────
            match self
                .llm_router
                .completion(
                    route.profile.provider,
                    route.profile.model,
                    routed_history,
                    tools,
                )
                .await
            {
                Ok(response) => {
                    self.history.push(Message {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                        tool_calls: response.tool_calls.clone(),
                        tool_call_id: None,
                    });
                    self.trim_history();

                    if let Some(content) = &response.content {
                        self.push_log(format!("[LLM] {}", content));
                        if let Some(ref brain) = self.brain {
                            let _ = brain.log_interaction(&self.session_id, "assistant", content);
                        }
                    }

                    match response.tool_calls {
                        None => break, // LLM finished — no more tool calls.
                        Some(tool_calls) => {
                            for tc in tool_calls {
                                self.push_log(format!(
                                    "[AGENT] Tool requested: {}",
                                    tc.function.name
                                ));

                                let args: Value = serde_json::from_str(&tc.function.arguments)
                                    .unwrap_or(serde_json::json!({}));

                                // ── Approval check ────────────────────────────
                                let approval_request =
                                    build_approval_request(&tc.function.name, &args);

                                if let Some(req) = approval_request {
                                    // This tool requires approval.
                                    // Check session policy first for immediate decisions.
                                    match self.approval_gate.check_policy(&req) {
                                        Some(ApprovalDecision::Approved) => {
                                            // Session policy says always allow.
                                            self.push_log(format!(
                                                "[APPROVAL] Auto-approved by session policy: {}",
                                                tc.function.name
                                            ));
                                            let result =
                                                execute_native_tool(&tc.function.name, args).await;
                                            self.push_log(format!("[TOOL] Result: {}", result));
                                            self.history.push(Message {
                                                role: "tool".to_string(),
                                                content: Some(result),
                                                tool_calls: None,
                                                tool_call_id: Some(tc.id),
                                            });
                                            self.trim_history();
                                        }
                                        Some(ApprovalDecision::Denied(reason)) => {
                                            // Session policy says always deny.
                                            self.push_log(format!(
                                                "[APPROVAL] Auto-denied by session policy: {} — {}",
                                                tc.function.name, reason
                                            ));
                                            self.history.push(Message {
                                                role: "tool".to_string(),
                                                content: Some(format!(
                                                    "Tool execution denied (session policy). \
                                                     Reason: {}",
                                                    reason
                                                )),
                                                tool_calls: None,
                                                tool_call_id: Some(tc.id),
                                            });
                                            self.trim_history();
                                        }
                                        None => {
                                            // Interactive approval needed: surface prompt in TUI
                                            // and pause the agent loop.
                                            for line in req.display_lines() {
                                                self.push_log(format!("[APPROVAL] {}", line));
                                            }
                                            self.current_task = format!(
                                                "Waiting for approval: {}",
                                                tc.function.name
                                            );
                                            self.pending_approval = Some(DeferredToolCall {
                                                id: tc.id,
                                                name: tc.function.name,
                                                args,
                                                request: req,
                                            });
                                            // Return early — the event loop will call
                                            // resolve_approval() when the user responds.
                                            return;
                                        }
                                    }
                                } else {
                                    // Tool does not require approval (e.g. read_file).
                                    let tool_result = if let Some(native_result) =
                                        NativeTools::execute(&tc.function.name, args.clone()).await
                                    {
                                        match native_result {
                                            Ok(res) => res,
                                            Err(e) => format!("Error executing tool: {}", e),
                                        }
                                    } else {
                                        match self
                                            .mcp_manager
                                            .call_tool(&tc.function.name, args)
                                            .await
                                        {
                                            Ok(res) => serde_json::to_string(&res)
                                                .unwrap_or_else(|_| "[]".to_string()),
                                            Err(e) => format!("Error calling MCP tool: {}", e),
                                        }
                                    };

                                    self.push_log(format!("[TOOL] Result: {}", tool_result));
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
                    self.push_log(format!("[ERROR] LLM Failed: {}", e));
                    break;
                }
            }
        }

        self.current_task = "Ready for mission...".to_string();
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build an [`ApprovalRequest`] for a tool call if the tool is dangerous.
///
/// Returns `None` for safe tools (like `read_file`) that do not require
/// interactive approval.
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
        // read_file and MCP tools do not currently require interactive approval.
        // They may be added in Phase 2 with path-based risk assessment.
        _ => None,
    }
}

/// Execute a native tool by name and args, returning the result as a string.
///
/// This is the post-approval execution path — approval has already been granted
/// by the time this is called.
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
