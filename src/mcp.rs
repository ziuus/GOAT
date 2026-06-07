use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

impl McpClient {
    pub async fn spawn(command: &str, args: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = BufReader::new(child.stdout.take().ok_or("Failed to open stdout")?);

        Ok(Self { child, stdin, stdout })
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
