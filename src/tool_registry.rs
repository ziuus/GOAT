use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolCategory {
    Filesystem,
    Shell,
    Git,
    Project,
    RepoMap,
    Memory,
    Skills,
    Provider,
    CodingWorkflow,
    Mcp,
    Browser,
    Subagent,
    System,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolAction {
    Allow,
    Ask(crate::approval::RiskLevel),
    Deny(String),
}

impl ToolAction {
    pub fn as_str(&self) -> &str {
        match self {
            ToolAction::Allow => "Allow",
            ToolAction::Ask(_) => "Ask",
            ToolAction::Deny(_) => "Deny",
        }
    }
}

impl std::fmt::Display for ToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ToolCategory::Filesystem => "filesystem",
            ToolCategory::Shell => "shell",
            ToolCategory::Git => "git",
            ToolCategory::Project => "project",
            ToolCategory::RepoMap => "repo-map",
            ToolCategory::Memory => "memory",
            ToolCategory::Skills => "skills",
            ToolCategory::Provider => "provider",
            ToolCategory::CodingWorkflow => "coding-workflow",
            ToolCategory::Mcp => "mcp",
            ToolCategory::Browser => "browser",
            ToolCategory::Subagent => "subagent",
            ToolCategory::System => "system",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolPermission {
    Allow,
    Ask,
    Deny,
}

impl ToolPermission {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "allow" => ToolPermission::Allow,
            "ask" => ToolPermission::Ask,
            "deny" => ToolPermission::Deny,
            _ => ToolPermission::Ask, // Default to safe
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub risk_level: crate::approval::RiskLevel,
    pub requires_approval: bool,
    pub read_only: bool,
    pub available_in_tui: bool,
    pub available_in_headless: bool,
    pub available_in_agent: bool,
    pub permission_group: String,
}

pub struct ToolRegistry {
    tools: HashMap<String, ToolMetadata>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register_builtins();
        registry
    }

    pub fn get_permission(
        &self,
        tool_name: &str,
        config: &crate::config::ToolsConfig,
    ) -> ToolPermission {
        if !config.enabled {
            return ToolPermission::Deny;
        }

        let tool = match self.get(tool_name) {
            Some(t) => t,
            None => return ToolPermission::Ask, // Unknown tools should always ask by default
        };

        let group_perm_str = match tool.permission_group.as_str() {
            "shell" => &config.permissions.shell,
            "filesystem_write" => &config.permissions.filesystem_write,
            "filesystem_read" => &config.permissions.filesystem_read,
            "network" => &config.permissions.network,
            "git" => &config.permissions.git,
            "memory" => &config.permissions.memory,
            "skills" => &config.permissions.skills,
            "subagent" => &config.permissions.subagent,
            _ => "ask",
        };

        ToolPermission::from_str(group_perm_str)
    }

    pub fn evaluate_action(&self, name: &str, config: &crate::config::ToolsConfig) -> ToolAction {
        let perm = self.get_permission(name, config);

        let tool = match self.get(name) {
            Some(t) => t,
            None => return ToolAction::Ask(crate::approval::RiskLevel::High), // unknown tools ask by default
        };

        match perm {
            ToolPermission::Deny => {
                ToolAction::Deny("Tool disabled by permission policy.".to_string())
            }
            ToolPermission::Ask => ToolAction::Ask(tool.risk_level.clone()),
            ToolPermission::Allow => {
                if tool.read_only && !tool.requires_approval {
                    ToolAction::Allow
                } else {
                    // Safe by default: downgrade 'Allow' to 'Ask' for dangerous tools
                    ToolAction::Ask(tool.risk_level.clone())
                }
            }
        }
    }

    pub fn log_execution(
        &self,
        paths: &crate::paths::GoatPaths,
        session_id: &str,
        tool_name: &str,
        action: &ToolAction,
        success: bool,
        output: &str,
    ) {
        use std::io::Write;

        let tool = self.get(tool_name);
        let category = tool
            .map(|t| t.category.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let risk = tool
            .map(|t| t.risk_level.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Redact secrets and truncate output
        let redacted_output = crate::approval::redact_secrets(output);
        let preview = if redacted_output.len() > 100 {
            format!("{}...", &redacted_output[..100].replace("\n", "\\n"))
        } else {
            redacted_output.replace("\n", "\\n")
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();

        let log_line = format!(
            "[{}] session={} tool={} category={} risk={} action={} success={} output=\"{}\"\n",
            timestamp,
            session_id,
            tool_name,
            category,
            risk,
            action.as_str(),
            success,
            preview
        );

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&paths.tool_audit_log_file)
        {
            let _ = file.write_all(log_line.as_bytes());
        }
    }

    fn register_builtins(&mut self) {
        self.register(ToolMetadata {
            name: "bash".to_string(),
            description: "Execute a bash command. Requires user approval before execution."
                .to_string(),
            category: ToolCategory::Shell,
            risk_level: crate::approval::RiskLevel::High,
            requires_approval: true,
            read_only: false,
            available_in_tui: true,
            available_in_headless: true,
            available_in_agent: true,
            permission_group: "shell".to_string(),
        });

        self.register(ToolMetadata {
            name: "read_file".to_string(),
            description: "Read a file from the filesystem.".to_string(),
            category: ToolCategory::Filesystem,
            risk_level: crate::approval::RiskLevel::Low,
            requires_approval: false,
            read_only: true,
            available_in_tui: true,
            available_in_headless: true,
            available_in_agent: true,
            permission_group: "filesystem_read".to_string(),
        });

        self.register(ToolMetadata {
            name: "write_file".to_string(),
            description: "Write content to a file. Requires user approval before execution."
                .to_string(),
            category: ToolCategory::Filesystem,
            risk_level: crate::approval::RiskLevel::High,
            requires_approval: true,
            read_only: false,
            available_in_tui: true,
            available_in_headless: true,
            available_in_agent: true,
            permission_group: "filesystem_write".to_string(),
        });

        self.register(ToolMetadata {
            name: "call_subagent".to_string(),
            description: "Spawn an external CLI agent and delegate a task. Requires user approval before execution.".to_string(),
            category: ToolCategory::Subagent,
            risk_level: crate::approval::RiskLevel::High,
            requires_approval: true,
            read_only: false,
            available_in_tui: true,
            available_in_headless: true,
            available_in_agent: true,
            permission_group: "subagent".to_string(),
        });

        self.register(ToolMetadata {
            name: "check".to_string(),
            description: "Run project type checks.".to_string(),
            category: ToolCategory::Project,
            risk_level: crate::approval::RiskLevel::Medium,
            requires_approval: true,
            read_only: true,
            available_in_tui: true,
            available_in_headless: true,
            available_in_agent: false,
            permission_group: "shell".to_string(),
        });
    }

    pub fn register(&mut self, metadata: ToolMetadata) {
        self.tools.insert(metadata.name.clone(), metadata);
    }

    pub fn get(&self, name: &str) -> Option<&ToolMetadata> {
        self.tools.get(name)
    }

    pub fn list_all(&self) -> Vec<&ToolMetadata> {
        let mut list: Vec<_> = self.tools.values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}
