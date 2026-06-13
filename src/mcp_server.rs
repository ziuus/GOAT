use anyhow::Result;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

use crate::mcp::{JsonRpcRequest, JsonRpcResponse};
use crate::runtime::GoatRuntime;

/// Run the MCP server loop over stdin/stdout.
pub async fn run(mut rt: GoatRuntime) -> Result<()> {
    info!("Starting GOAT MCP Server on stdio");

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        debug!("MCP Server received: {}", line);

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse MCP request: {}", e);
                let err_resp = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                });
                let out = serde_json::to_string(&err_resp).unwrap();
                stdout.write_all(out.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                continue;
            }
        };

        let response = match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "goat",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
                error: None,
            }),
            "notifications/initialized" => None,
            "tools/list" => Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "tools": [
                        {
                            "name": "goat_memory_add",
                            "description": "Add a safe, redacted memory to the GOAT structured memory store.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "content": { "type": "string", "description": "The memory content to store. Will be redacted for secrets." }
                                },
                                "required": ["content"]
                            }
                        },
                        {
                            "name": "goat_repo_status",
                            "description": "Get the GOAT generated repository map and status safely.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                })),
                error: None,
            }),
            "tools/call" => {
                let params = req.params.unwrap_or(json!({}));
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let default_args = json!({});
                let args = params.get("arguments").unwrap_or(&default_args);

                let (result, is_tool_error) = match name {
                    "goat_memory_add" => {
                        let raw_content =
                            args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                        if raw_content.trim().is_empty() {
                            (
                                json!({ "isError": true, "content": [{ "type": "text", "text": "Missing content" }] }),
                                false,
                            )
                        } else {
                            let redacted = crate::memory::redact_secrets(raw_content);
                            if redacted.trim().is_empty() || redacted == "[REDACTED]" {
                                (
                                    json!({ "isError": true, "content": [{ "type": "text", "text": "Content appears to be entirely sensitive and was rejected." }] }),
                                    false,
                                )
                            } else {
                                let mm = crate::memory::MemoryManager::new(
                                    &rt.paths,
                                    rt.config.memory.clone(),
                                );
                                match mm.add_note(&redacted) {
                                    Ok(_) => (
                                        json!({ "content": [{ "type": "text", "text": "Memory safely added." }] }),
                                        false,
                                    ),
                                    Err(e) => (
                                        json!({ "isError": true, "content": [{ "type": "text", "text": format!("Failed to add memory: {}", e) }] }),
                                        false,
                                    ),
                                }
                            }
                        }
                    }
                    "goat_repo_status" => {
                        let current_dir =
                            std::env::current_dir().unwrap_or(rt.paths.data_dir.clone());
                        let scanner = crate::repo_map::RepoMapScanner::new(current_dir);
                        match scanner.scan() {
                            Ok(map) => {
                                let map_str = map.to_compact_string(8000, false);
                                (
                                    json!({ "content": [{ "type": "text", "text": map_str }] }),
                                    false,
                                )
                            }
                            Err(e) => (
                                json!({ "isError": true, "content": [{ "type": "text", "text": format!("Failed to generate repo map: {}", e) }] }),
                                false,
                            ),
                        }
                    }
                    _ => (json!({}), true),
                };

                if is_tool_error {
                    Some(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(json!({
                            "code": -32601,
                            "message": format!("Unknown tool: {}", name)
                        })),
                    })
                } else {
                    Some(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(result),
                        error: None,
                    })
                }
            }
            _ => {
                debug!("Unknown MCP method: {}", req.method);
                Some(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: None,
                    error: Some(json!({
                        "code": -32601,
                        "message": format!("Method not found: {}", req.method)
                    })),
                })
            }
        };

        if let Some(resp) = response {
            let out = serde_json::to_string(&resp).unwrap();
            stdout.write_all(out.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
            debug!("MCP Server sent: {}", out);
        }
    }

    info!("GOAT MCP Server shutting down");
    Ok(())
}
