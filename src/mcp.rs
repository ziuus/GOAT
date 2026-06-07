use std::collections::HashMap;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::McpServerConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpClient {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

pub struct ManagedMcpServer {
    pub client: McpClient,
    pub tools: Vec<Value>,
}

#[derive(Default)]
pub struct McpManager {
    servers: HashMap<String, ManagedMcpServer>,
    tool_index: HashMap<String, String>,
}

impl McpClient {
    pub async fn spawn_with_env(
        command: &str,
        args: &[&str],
        env: &HashMap<String, String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut child_command = Command::new(command);
        child_command.args(args);
        for (key, value) in env {
            child_command.env(key, value);
        }

        let mut child = child_command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = BufReader::new(child.stdout.take().ok_or("Failed to open stdout")?);

        Ok(Self { child, stdin, stdout })
    }

    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running() {
            self.child.kill().await?;
        }
        Ok(())
    }

    pub async fn initialize(&mut self) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
        let init_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "initialize".to_string(),
            params: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "goat",
                    "version": "0.1.0"
                }
            })),
        };

        let mut req_str = serde_json::to_string(&init_req)?;
        req_str.push('\n');
        self.stdin.write_all(req_str.as_bytes()).await?;
        self.stdin.flush().await?;

        let mut line = String::new();
        self.stdout.read_line(&mut line).await?;

        let resp: JsonRpcResponse = serde_json::from_str(&line)?;
        Ok(resp)
    }

    pub async fn send_request(&mut self, method: &str, params: Option<Value>) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: rand::random::<u64>(),
            method: method.to_string(),
            params,
        };

        let mut req_str = serde_json::to_string(&req)?;
        req_str.push('\n');
        self.stdin.write_all(req_str.as_bytes()).await?;
        self.stdin.flush().await?;

        let mut line = String::new();
        self.stdout.read_line(&mut line).await?;

        let resp: JsonRpcResponse = serde_json::from_str(&line)?;
        Ok(resp)
    }

    pub async fn list_tools(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        let resp = self.send_request("tools/list", None).await?;
        Ok(resp.result.unwrap_or(json!({})))
    }

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let resp = self.send_request("tools/call", Some(json!({
            "name": name,
            "arguments": arguments
        }))).await?;
        Ok(resp.result.unwrap_or(json!({})))
    }
}

impl McpManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start_configured(
        &mut self,
        configs: &HashMap<String, McpServerConfig>,
    ) -> Vec<String> {
        let mut logs = Vec::new();

        for (name, config) in configs {
            if self.servers.contains_key(name) {
                logs.push(format!("[MCP] Server '{name}' already running."));
                continue;
            }

            match McpClient::spawn_with_env(&config.command, &config.args.iter().map(String::as_str).collect::<Vec<_>>(), &config.env).await {
                Ok(mut client) => match client.initialize().await {
                    Ok(_) => match client.list_tools().await {
                        Ok(tool_payload) => {
                            let tools = tool_payload
                                .get("tools")
                                .and_then(Value::as_array)
                                .cloned()
                                .unwrap_or_default();

                            for tool in &tools {
                                if let Some(tool_name) = tool.get("name").and_then(Value::as_str) {
                                    self.tool_index.insert(tool_name.to_string(), name.to_string());
                                }
                            }

                            self.servers.insert(
                                name.to_string(),
                                ManagedMcpServer { client, tools },
                            );
                            logs.push(format!("[MCP] Server '{name}' started."));
                        }
                        Err(err) => logs.push(format!("[MCP ERROR] Server '{name}' tool listing failed: {err}")),
                    },
                    Err(err) => logs.push(format!("[MCP ERROR] Server '{name}' initialize failed: {err}")),
                },
                Err(err) => logs.push(format!("[MCP ERROR] Server '{name}' spawn failed: {err}")),
            }
        }

        if configs.is_empty() {
            logs.push("[MCP] No configured MCP servers found.".to_string());
        }

        logs
    }

    pub fn all_tools(&self) -> Vec<Value> {
        self.servers
            .values()
            .flat_map(|server| server.tools.iter().cloned())
            .collect()
    }

    pub fn running_servers(&mut self) -> Vec<String> {
        self.servers
            .iter_mut()
            .filter_map(|(name, server)| server.client.is_running().then(|| name.clone()))
            .collect()
    }

    pub async fn shutdown_all(&mut self) -> Vec<String> {
        let mut logs = Vec::new();

        for (name, server) in &mut self.servers {
            match server.client.shutdown().await {
                Ok(()) => logs.push(format!("[MCP] Server '{name}' stopped.")),
                Err(err) => logs.push(format!("[MCP ERROR] Server '{name}' shutdown failed: {err}")),
            }
        }

        self.servers.clear();
        self.tool_index.clear();
        logs
    }

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let server_name = self
            .tool_index
            .get(name)
            .ok_or_else(|| format!("No MCP server registered tool '{name}'"))?
            .clone();

        let server = self
            .servers
            .get_mut(&server_name)
            .ok_or_else(|| format!("MCP server '{server_name}' is not running"))?;

        server.client.call_tool(name, arguments).await
    }
}
