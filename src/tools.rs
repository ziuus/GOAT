use crate::llm::{FunctionDeclaration, Tool};
use serde_json::Value;
use std::fs;
use std::process::Stdio;
use tokio::process::Command;

pub struct NativeTools;

impl NativeTools {
    pub fn all_tools() -> Vec<Tool> {
        vec![
            Tool {
                r#type: "function".to_string(),
                function: FunctionDeclaration {
                    name: "bash".to_string(),
                    description: "Execute a bash command".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "The command to execute"
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
                    description: "Read a file from the filesystem".to_string(),
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
                    description: "Write content to a file".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Absolute path to the file"
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
        ]
    }

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
                    },
                    Err(e) => Some(Err(format!("Failed to execute command: {}", e))),
                }
            },
            "read_file" => {
                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                if path.is_empty() {
                    return Some(Err("No path provided".to_string()));
                }
                match fs::read_to_string(path) {
                    Ok(content) => Some(Ok(content)),
                    Err(e) => Some(Err(format!("Failed to read file: {}", e))),
                }
            },
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
            },
            _ => None,
        }
    }
}
