use crate::brain::Brain;
use crate::llm::{LlmRouter, Message, Tool, FunctionDeclaration, ToolCall};
use crate::config::Config;
use serde_json::Value;

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
    pub mcp_client: Option<crate::mcp::McpClient>,
    pub history: Vec<Message>,
    pub config: Config,
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
            mcp_client: None,
            history: Vec::new(),
            config,
        }
    }
    
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn handle_user_input(&mut self, msg: String) {
        self.logs.push(format!("[USER] {}", msg));
        
        self.history.push(Message {
            role: "user".to_string(),
            content: Some(msg),
            tool_calls: None,
            tool_call_id: None,
        });

        // Agentic loop max iterations
        for _iteration in 0..10 {
            let mut tools: Option<Vec<Tool>> = None;
            
            // 1. Fetch tools from MCP if client exists
            if let Some(ref mut client) = self.mcp_client {
                if let Ok(mcp_tools) = client.list_tools().await {
                    if let Some(tools_array) = mcp_tools.get("tools").and_then(|t| t.as_array()) {
                        let mut mapped_tools = Vec::new();
                        for t in tools_array {
                            if let (Some(name), Some(desc), Some(schema)) = (
                                t.get("name").and_then(|v| v.as_str()),
                                t.get("description").and_then(|v| v.as_str()),
                                t.get("inputSchema")
                            ) {
                                mapped_tools.push(Tool {
                                    r#type: "function".to_string(),
                                    function: FunctionDeclaration {
                                        name: name.to_string(),
                                        description: desc.to_string(),
                                        parameters: schema.clone(),
                                    }
                                });
                            }
                        }
                        if !mapped_tools.is_empty() {
                            tools = Some(mapped_tools);
                        }
                    }
                }
            }

            // 2. Call LLM
            match self.llm_router.completion("openai", "gpt-4o-mini", self.history.clone(), tools).await {
                Ok(response) => {
                    // Record assistant message
                    self.history.push(Message {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                        tool_calls: response.tool_calls.clone(),
                        tool_call_id: None,
                    });

                    // Log text content
                    if let Some(content) = &response.content {
                        self.logs.push(format!("[LLM] {}", content));
                        if let Some(ref brain) = self.brain {
                            let _ = brain.log_interaction("assistant", content);
                        }
                    }

                    // 3. Handle Tool Calls
                    if let Some(tool_calls) = response.tool_calls {
                        for tc in tool_calls {
                            self.logs.push(format!("[AGENT] Executing tool: {}", tc.function.name));
                            
                            let mut tool_result = String::new();
                            
                            if let Some(ref mut client) = self.mcp_client {
                                let args: Value = serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::json!({}));
                                match client.call_tool(&tc.function.name, args).await {
                                    Ok(res) => {
                                        tool_result = serde_json::to_string(&res).unwrap_or_else(|_| "[]".to_string());
                                    }
                                    Err(e) => {
                                        tool_result = format!("Error calling tool: {}", e);
                                    }
                                }
                            } else {
                                tool_result = "Error: MCP Client not connected.".to_string();
                            }

                            self.logs.push(format!("[TOOL] Result: {}", tool_result));
                            
                            // Append tool result to history
                            self.history.push(Message {
                                role: "tool".to_string(),
                                content: Some(tool_result),
                                tool_calls: None,
                                tool_call_id: Some(tc.id),
                            });
                        }
                    } else {
                        // No tool calls, interaction complete
                        break;
                    }
                }
                Err(e) => {
                    self.logs.push(format!("[ERROR] LLM Failed: {}", e));
                    break;
                }
            }
        }
    }
}
