use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::extensions::{ExtensionManager, ExtensionStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilitySource {
    Core,
    Extension(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityType {
    Command,
    McpServer,
    Skill,
    ValidationRecipe,
    NativeTool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    pub id: String,
    pub name: String,
    pub source: CapabilitySource,
    pub capability_type: CapabilityType,
    pub risk_level: String,
    pub enabled: bool,
    pub description: String,
    #[serde(default)]
    pub required_permissions: Vec<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub discovered_at: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CapabilityRegistryState {
    pub capabilities: HashMap<String, ToolCapability>,
}

pub struct CapabilityRegistry {
    pub registry_file: PathBuf,
    pub state: CapabilityRegistryState,
}

impl CapabilityRegistry {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let registry_file = data_dir.join("capability_registry.json");
        let mut registry = Self {
            registry_file,
            state: CapabilityRegistryState::default(),
        };
        registry.load()?;
        Ok(registry)
    }

    fn load(&mut self) -> Result<()> {
        if !self.registry_file.exists() {
            return Ok(());
        }
        let content = fs::read_to_string(&self.registry_file)?;
        self.state = serde_json::from_str(&content).unwrap_or_default();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.state)?;
        fs::write(&self.registry_file, content)?;
        Ok(())
    }

    pub fn refresh(&mut self, extension_manager: &ExtensionManager) -> Result<()> {
        let mut new_capabilities = HashMap::new();

        // 1. Add Core capabilities (Mocked/Static for now, should mirror src/tool_registry.rs long term)
        let native_tools = vec![
            ("bash", "Execute a bash command", "shell"),
            (
                "read_file",
                "Read a file from the filesystem",
                "filesystem_read",
            ),
            ("write_file", "Write content to a file", "filesystem_write"),
        ];

        for (id, desc, perm) in native_tools {
            let cap = ToolCapability {
                id: id.to_string(),
                name: id.to_string(),
                source: CapabilitySource::Core,
                capability_type: CapabilityType::NativeTool,
                risk_level: "low".to_string(), // Simplified for discovery
                enabled: true,
                description: desc.to_string(),
                required_permissions: vec![perm.to_string()],
                metadata: serde_json::json!({}),
                discovered_at: Utc::now().to_rfc3339(),
            };
            new_capabilities.insert(cap.id.clone(), cap);
        }

        // 2. Add Extension Capabilities (from Enabled extensions only)
        for entry in extension_manager.list() {
            if entry.status != ExtensionStatus::Enabled {
                continue;
            }

            let ext_id = &entry.manifest.extension.id;
            let ext_risk = format!("{:?}", entry.manifest.extension.risk_level).to_lowercase();

            // Tools (Commands)
            for t in &entry.manifest.tools {
                let id = format!("{}:tool:{}", ext_id, t.id);
                let cap = ToolCapability {
                    id: id.clone(),
                    name: t.name.clone(),
                    source: CapabilitySource::Extension(ext_id.clone()),
                    capability_type: CapabilityType::Command,
                    risk_level: format!("{:?}", t.risk_level).to_lowercase(),
                    enabled: true,
                    description: t.description.clone(),
                    required_permissions: vec![],
                    metadata: serde_json::json!({ "command": t.command }),
                    discovered_at: Utc::now().to_rfc3339(),
                };
                new_capabilities.insert(id, cap);
            }

            // MCP Servers
            for m in &entry.manifest.mcp_servers {
                let id = format!("{}:mcp:{}", ext_id, m.id);
                let cap = ToolCapability {
                    id: id.clone(),
                    name: m.name.clone(),
                    source: CapabilitySource::Extension(ext_id.clone()),
                    capability_type: CapabilityType::McpServer,
                    risk_level: format!("{:?}", m.risk_level).to_lowercase(),
                    enabled: true, // Only metadata is enabled, server is NOT running
                    description: format!("MCP Server from {}", ext_id),
                    required_permissions: vec![],
                    metadata: serde_json::json!({
                        "command": m.command,
                        "args": m.args
                    }),
                    discovered_at: Utc::now().to_rfc3339(),
                };
                new_capabilities.insert(id, cap);
            }

            // Skills
            for s in &entry.manifest.skills {
                let id = format!("{}:skill:{}", ext_id, s.id);
                let cap = ToolCapability {
                    id: id.clone(),
                    name: s.name.clone(),
                    source: CapabilitySource::Extension(ext_id.clone()),
                    capability_type: CapabilityType::Skill,
                    risk_level: ext_risk.clone(), // inherit from extension
                    enabled: true,
                    description: s.description.clone(),
                    required_permissions: vec![],
                    metadata: serde_json::json!({ "path": s.path }),
                    discovered_at: Utc::now().to_rfc3339(),
                };
                new_capabilities.insert(id, cap);
            }

            // Validation Recipes
            for v in &entry.manifest.validation_recipes {
                let id = format!("{}:validator:{}", ext_id, v.id);
                let cap = ToolCapability {
                    id: id.clone(),
                    name: v.name.clone(),
                    source: CapabilitySource::Extension(ext_id.clone()),
                    capability_type: CapabilityType::ValidationRecipe,
                    risk_level: ext_risk.clone(),
                    enabled: true,
                    description: v.description.clone(),
                    required_permissions: vec![],
                    metadata: serde_json::json!({ "path": v.path }),
                    discovered_at: Utc::now().to_rfc3339(),
                };
                new_capabilities.insert(id, cap);
            }
        }

        // Retain discovered_at for existing capabilities
        for (id, new_cap) in new_capabilities.iter_mut() {
            if let Some(existing) = self.state.capabilities.get(id) {
                new_cap.discovered_at = existing.discovered_at.clone();
            }
        }

        self.state.capabilities = new_capabilities;
        self.save()?;

        Ok(())
    }

    pub fn list(&self) -> Vec<&ToolCapability> {
        let mut caps: Vec<_> = self.state.capabilities.values().collect();
        caps.sort_by(|a, b| a.id.cmp(&b.id));
        caps
    }

    pub fn get(&self, id: &str) -> Option<&ToolCapability> {
        self.state.capabilities.get(id)
    }

    pub fn doctor(&self) -> Result<Vec<String>> {
        let mut findings = Vec::new();

        findings.push(format!("Registry Path: {}", self.registry_file.display()));
        findings.push(format!(
            "Total Capabilities: {}",
            self.state.capabilities.len()
        ));

        for cap in self.state.capabilities.values() {
            if cap.risk_level == "high" || cap.risk_level == "critical" {
                findings.push(format!(
                    "WARNING: Capability '{}' has {} risk.",
                    cap.id, cap.risk_level
                ));
            }

            // Validate extension sources
            if let CapabilitySource::Extension(ext_id) = &cap.source {
                if cap.id.split(':').next().unwrap_or("") != ext_id {
                    findings.push(format!(
                        "ERROR: Capability ID '{}' does not match source extension '{}'",
                        cap.id, ext_id
                    ));
                }
            }
        }

        if findings.len() == 2 {
            findings.push("No issues found. Capability Registry is healthy.".to_string());
        }

        Ok(findings)
    }
}
