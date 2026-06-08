// Agent Manager
// Handles the ReAct loop and tool execution.

use crate::agent::litellm::{LlmRouter, Message, Tool, FunctionDeclaration};
use crate::mcp::McpManager;
use crate::tools::NativeTools;
use serde_json::Value;
use crate::swarm::RouteDecision;

pub struct AgentManager {
    pub llm_router: LlmRouter,
}

impl AgentManager {
    pub fn new(openai_key: Option<String>, groq_key: Option<String>) -> Self {
        Self {
            llm_router: LlmRouter::new(openai_key, groq_key),
        }
    }

    pub async fn execute_task<F>(
        &mut self,
        route: &RouteDecision,
        history: &mut Vec<Message>,
        mcp_manager: &mut McpManager,
        mut logger: F,
    ) -> Result<(), String>
    where
        F: FnMut(String),
    {
        for _iteration in 0..10 {
            let mut tools: Option<Vec<Tool>> = None;

            let mcp_tools = mcp_manager.all_tools();
            let mut mapped_tools = NativeTools::all_tools();

            if !mcp_tools.is_empty() {
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
            }
            if !mapped_tools.is_empty() {
                tools = Some(mapped_tools);
            }

            let mut routed_history = vec![Message {
                role: "system".to_string(),
                content: Some(route.profile.system_prompt.to_string()),
                tool_calls: None,
                tool_call_id: None,
            }];
            routed_history.extend(history.clone());

            match self.llm_router.completion(route.profile.provider, route.profile.model, routed_history, tools).await {
                Ok(response) => {
                    history.push(Message {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                        tool_calls: response.tool_calls.clone(),
                        tool_call_id: None,
                    });

                    if let Some(content) = &response.content {
                        logger(format!("[LLM] {}", content));
                    }

                    if let Some(tool_calls) = response.tool_calls {
                        for tc in tool_calls {
                            logger(format!("[AGENT] Executing tool: {}", tc.function.name));

                            let tool_result = {
                                let args: Value = serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::json!({}));
                                if let Some(native_result) = NativeTools::execute(&tc.function.name, args.clone()).await {
                                    match native_result {
                                        Ok(res) => res,
                                        Err(e) => format!("Error executing native tool: {}", e),
                                    }
                                } else {
                                    match mcp_manager.call_tool(&tc.function.name, args).await {
                                        Ok(res) => serde_json::to_string(&res).unwrap_or_else(|_| "[]".to_string()),
                                        Err(e) => format!("Error calling MCP tool: {}", e),
                                    }
                                }
                            };

                            logger(format!("[TOOL] Result: {}", tool_result));

                            history.push(Message {
                                role: "tool".to_string(),
                                content: Some(tool_result),
                                tool_calls: None,
                                tool_call_id: Some(tc.id),
                            });
                        }
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    logger(format!("[ERROR] LLM Failed: {}", e));
                    return Err(e.to_string());
                }
            }
        }
        Ok(())
    }
}
