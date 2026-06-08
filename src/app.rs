use crate::brain::Brain;
use crate::llm::{FunctionDeclaration, LlmRouter, Message, Tool};
use crate::swarm::{RouteDecision, SwarmRouter};
use crate::config::Config;
use serde_json::Value;
use std::path::PathBuf;

const MAX_LOG_LINES: usize = 500;
const MAX_HISTORY_MESSAGES: usize = 80;

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
    pub mcp_manager: crate::mcp::McpManager,
    pub history: Vec<Message>,
    pub config: Config,
    pub swarm_router: SwarmRouter,
    pub active_route: Option<RouteDecision>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let brain = Brain::new("goat_brain.db").ok();
        let mut logs = vec![
            "[SYSTEM] GOAT Engine Initialized.".to_string(),
            "[SYSTEM] Awaiting MCP connections...".to_string(),
        ];
        
        if brain.is_none() {
            logs.push("[ERROR] Failed to initialize Brain (SQLite).".to_string());
        } else {
            logs.push("[SYSTEM] Brain connected (SQLite).".to_string());
        }

        let llm_router = LlmRouter::new(config.keys.openai_api_key.clone(), config.keys.groq_api_key.clone());

        Self {
            running: true,
            logs,
            current_task: "Ready for mission...".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            brain,
            llm_router,
            mcp_manager: crate::mcp::McpManager::new(),
            history: Vec::new(),
            config,
            swarm_router: SwarmRouter::default(),
            active_route: None,
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

    pub async fn start_configured_mcp_servers(&mut self) {
        let logs = self.mcp_manager.start_configured(&self.config.mcp_servers).await;
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

    pub fn learn_about_me(&mut self) {
        let paths = default_index_paths();
        if paths.is_empty() {
            self.push_log("[BRAIN] No default paths found to index.");
            return;
        }

        self.push_log(format!("[BRAIN] Indexing {} local knowledge roots...", paths.len()));
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

    pub fn route_current_input(&mut self) {
        let candidate = if self.input.trim().is_empty() {
            self.current_task.as_str()
        } else {
            self.input.as_str()
        };

        let decision = self.swarm_router.route(candidate);
        self.push_log(format!(
            "[SWARM] Routed to {} ({:?}) confidence {}%: {}",
            decision.profile.name,
            decision.profile.kind,
            decision.confidence,
            decision.reason
        ));
        self.current_task = format!("{} agent selected", decision.profile.name);
        self.active_route = Some(decision);
    }

    pub async fn handle_user_input(&mut self, msg: String) {
        self.push_log(format!("[USER] {}", msg));
        
        self.history.push(Message {
            role: "user".to_string(),
            content: Some(msg),
            tool_calls: None,
            tool_call_id: None,
        });
        self.trim_history();

        for _iteration in 0..10 {
            let mut tools: Option<Vec<Tool>> = None;

            let mcp_tools = self.mcp_manager.all_tools();
            if !mcp_tools.is_empty() {
                let mut mapped_tools = Vec::new();
                for tool in mcp_tools {
                    if let (Some(name), Some(desc), Some(schema)) = (
                        tool.get("name").and_then(|value| value.as_str()),
                        tool.get("description").and_then(|value| value.as_str()),
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
                if !mapped_tools.is_empty() {
                    tools = Some(mapped_tools);
                }
            }

            let route = self.swarm_router.route(self.history.last().and_then(|message| message.content.as_deref()).unwrap_or_default());
            self.active_route = Some(route.clone());
            self.current_task = format!("{} agent working", route.profile.name);

            let mut routed_history = vec![Message {
                role: "system".to_string(),
                content: Some(route.profile.system_prompt.to_string()),
                tool_calls: None,
                tool_call_id: None,
            }];
            routed_history.extend(self.history.clone());

            match self.llm_router.completion(route.profile.provider, route.profile.model, routed_history, tools).await {
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
                            let _ = brain.log_interaction("assistant", content);
                        }
                    }

                    if let Some(tool_calls) = response.tool_calls {
                        for tc in tool_calls {
                            self.push_log(format!("[AGENT] Executing tool: {}", tc.function.name));

                            let tool_result = {
                                let args: Value = serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::json!({}));
                                match self.mcp_manager.call_tool(&tc.function.name, args).await {
                                    Ok(res) => serde_json::to_string(&res).unwrap_or_else(|_| "[]".to_string()),
                                    Err(e) => format!("Error calling tool: {}", e),
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
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    self.push_log(format!("[ERROR] LLM Failed: {}", e));
                    break;
                }
            }
        }
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
