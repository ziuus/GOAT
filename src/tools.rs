//! Native tools available to the GOAT agent.
//!
//! # Security note
//!
//! The [`NativeTools::execute`] function is the **post-approval** execution
//! path. All dangerous tools (`bash`, `write_file`, `call_subagent`) must pass
//! through the [`crate::approval::ApprovalGate`] **before** this function is
//! called. Callers in [`crate::app`] enforce this invariant.
//!
//! `read_file` is the only tool here that does not currently require interactive
//! approval. It will be gated by a path-based risk check in Phase 2.

use crate::llm::{FunctionDeclaration, Tool};
use serde_json::Value;
use std::fs;
use std::process::Stdio;
use tokio::process::Command;

pub struct NativeTools;

impl NativeTools {
    /// Return the tool schemas that are advertised to the LLM.
    pub fn all_tools() -> Vec<Tool> {
        vec![
            Tool {
                r#type: "function".to_string(),
                function: FunctionDeclaration {
                    name: "bash".to_string(),
                    description: "Execute a bash command. Requires user approval before execution."
                        .to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "The bash command to execute"
                            }
                        },
                        "required": ["command"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: FunctionDeclaration {
                    name: "read_file".to_string(),
                    description: "Read a file from the filesystem.".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Absolute path to the file"
                            }
                        },
                        "required": ["path"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: FunctionDeclaration {
                    name: "write_file".to_string(),
                    description: "Write content to a file. Requires user approval before execution."
                        .to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Absolute path to write to"
                            },
                            "content": {
                                "type": "string",
                                "description": "Content to write"
                            }
                        },
                        "required": ["path", "content"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: FunctionDeclaration {
                    name: "call_subagent".to_string(),
                    description: "Spawn an external CLI agent and delegate a task. Requires user approval before execution.".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "agent_cli": {
                                "type": "string",
                                "description": "The CLI command of the agent (e.g. 'opencode', 'claude')"
                            },
                            "prompt": {
                                "type": "string",
                                "description": "The task prompt to pass to the agent"
                            }
                        },
                        "required": ["agent_cli", "prompt"]
                    }),
                },
            },
        ]
    }

    /// Execute a named tool with the given arguments.
    ///
    /// # Safety invariant
    ///
    /// **This must only be called after the approval gate has granted permission
    /// for dangerous tools (`bash`, `write_file`, `call_subagent`).**
    ///
    /// Returns `None` if the tool name is not recognised by the native tool set
    /// (the caller should then try MCP tools).
    pub async fn execute(name: &str, args: Value) -> Option<Result<String, String>> {
        match name {
            "bash" => {
                let cmd = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
                if cmd.is_empty() {
                    return Some(Err("No command provided".to_string()));
                }

                let output = Command::new("bash")
                    .arg("-c")
                    .arg(cmd)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .await;

                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                        let result = if stderr.is_empty() {
                            stdout
                        } else {
                            format!("{}\nSTDERR:\n{}", stdout, stderr)
                        };
                        Some(Ok(result))
                    }
                    Err(e) => Some(Err(format!("Failed to execute command: {}", e))),
                }
            }

            "read_file" => {
                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                if path.is_empty() {
                    return Some(Err("No path provided".to_string()));
                }
                match fs::read_to_string(path) {
                    Ok(content) => Some(Ok(content)),
                    Err(e) => Some(Err(format!("Failed to read file: {}", e))),
                }
            }

            "write_file" => {
                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                if path.is_empty() {
                    return Some(Err("No path provided".to_string()));
                }
                match fs::write(path, content) {
                    Ok(_) => Some(Ok(format!("Successfully wrote to {}", path))),
                    Err(e) => Some(Err(format!("Failed to write file: {}", e))),
                }
            }

            "call_subagent" => {
                let agent_cli = args.get("agent_cli").and_then(|v| v.as_str()).unwrap_or("");
                let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");

                if agent_cli.is_empty() || prompt.is_empty() {
                    return Some(Err("agent_cli and prompt are required".to_string()));
                }

                let output = Command::new(agent_cli)
                    .arg(prompt)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .await;

                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                        let result = if stderr.is_empty() {
                            stdout
                        } else {
                            format!("{}\nSTDERR:\n{}", stdout, stderr)
                        };
                        Some(Ok(format!("Agent '{}' completed:\n{}", agent_cli, result)))
                    }
                    Err(e) => Some(Err(format!(
                        "Failed to execute agent '{}': {}",
                        agent_cli, e
                    ))),
                }
            }

            _ => None,
        }
    }
}
