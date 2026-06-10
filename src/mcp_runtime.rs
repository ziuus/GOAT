use crate::config::McpServerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lifecycle state of an MCP server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum McpServerState {
    Configured,
    Disabled,
    Starting,
    Running,
    Stopped,
    Failed,
    Unsupported,
}

impl std::fmt::Display for McpServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Configured => write!(f, "Configured"),
            Self::Disabled => write!(f, "Disabled"),
            Self::Starting => write!(f, "Starting"),
            Self::Running => write!(f, "Running"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Failed => write!(f, "Failed"),
            Self::Unsupported => write!(f, "Unsupported"),
        }
    }
}

/// Holds the runtime state of an MCP server.
#[derive(Debug, Clone)]
pub struct McpRuntimeState {
    pub name: String,
    pub config: McpServerConfig,
    pub state: McpServerState,
    pub pid: Option<u32>,
    pub started_at: Option<std::time::SystemTime>,
    pub last_error: Option<String>,
    pub health_status: Option<String>,
    pub discovered_tools: Vec<serde_json::Value>,
}

impl McpRuntimeState {
    pub fn new(name: String, config: McpServerConfig) -> Self {
        let state = if config.enabled {
            McpServerState::Configured
        } else {
            McpServerState::Disabled
        };

        Self {
            name,
            config,
            state,
            pid: None,
            started_at: None,
            last_error: None,
            health_status: None,
            discovered_tools: Vec::new(),
        }
    }
}

/// Manages MCP servers across their lifecycle.
#[derive(Debug, Default)]
pub struct McpRuntimeManager {
    servers: HashMap<String, McpRuntimeState>,
}

impl McpRuntimeManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    /// Loads configured servers into the runtime manager.
    pub fn init_from_config(&mut self, config: &crate::config::Config) {
        for (name, srv_config) in &config.mcp_servers {
            if !self.servers.contains_key(name) {
                self.servers.insert(
                    name.clone(),
                    McpRuntimeState::new(name.clone(), srv_config.clone()),
                );
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&McpRuntimeState> {
        self.servers.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut McpRuntimeState> {
        self.servers.get_mut(name)
    }

    pub fn list_all(&self) -> Vec<&McpRuntimeState> {
        let mut list: Vec<_> = self.servers.values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    pub fn list_all_mut(&mut self) -> Vec<&mut McpRuntimeState> {
        let mut list: Vec<_> = self.servers.values_mut().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}
