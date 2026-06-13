use anyhow::{Result, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    SkillPack,
    McpPack,
    AgentAdapter,
    WorkflowPack,
    CommandPack,
    ValidationPack,
    MemoryPack,
    DashboardWidget,
    ProjectTemplate,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionStatus {
    Discovered,
    Installed,
    Enabled,
    Disabled,
    Blocked,
    Invalid,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: Option<String>,
    #[serde(rename = "type")]
    pub ext_type: ExtensionType,
    #[serde(default = "default_risk")]
    pub risk_level: ExtensionRiskLevel,
}

fn default_risk() -> ExtensionRiskLevel {
    ExtensionRiskLevel::Medium
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionCapabilities {
    #[serde(default)]
    pub skills: bool,
    #[serde(default)]
    pub validation_recipes: bool,
    #[serde(default)]
    pub commands: bool,
    #[serde(default)]
    pub mcp_servers: bool,
    #[serde(default)]
    pub external_agents: bool,
    #[serde(default)]
    pub dashboard_widgets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionPermissions {
    #[serde(default)]
    pub read_project: bool,
    #[serde(default)]
    pub write_project: bool,
    #[serde(default)]
    pub run_commands: bool,
    #[serde(default)]
    pub network: bool,
    #[serde(default)]
    pub access_memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionEntrypoints {
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub validation_recipes: Vec<String>,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub mcp_servers: Vec<String>,
    #[serde(default)]
    pub external_agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoatExtensionManifest {
    pub extension: ExtensionMeta,
    #[serde(default)]
    pub capabilities: ExtensionCapabilities,
    #[serde(default)]
    pub permissions: ExtensionPermissions,
    #[serde(default)]
    pub entrypoints: ExtensionEntrypoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRegistryEntry {
    pub manifest: GoatExtensionManifest,
    pub status: ExtensionStatus,
    pub install_source: String,
    pub installed_path: Option<PathBuf>,
    pub installed_at: String,
}

pub struct ExtensionManager {
    pub base_dir: PathBuf,
    pub installed_dir: PathBuf,
    pub registry_file: PathBuf,
    pub entries: HashMap<String, ExtensionRegistryEntry>,
}

impl ExtensionManager {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let base_dir = data_dir.join("extensions");
        let installed_dir = base_dir.join("installed");
        let registry_file = base_dir.join("registry.jsonl");

        fs::create_dir_all(&installed_dir)?;

        let mut manager = Self {
            base_dir,
            installed_dir,
            registry_file,
            entries: HashMap::new(),
        };
        manager.load_registry()?;
        Ok(manager)
    }

    fn load_registry(&mut self) -> Result<()> {
        if !self.registry_file.exists() {
            return Ok(());
        }
        let content = fs::read_to_string(&self.registry_file)?;
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(entry) = serde_json::from_str::<ExtensionRegistryEntry>(line) {
                self.entries
                    .insert(entry.manifest.extension.id.clone(), entry);
            }
        }
        Ok(())
    }

    fn save_registry(&self) -> Result<()> {
        let mut content = String::new();
        for entry in self.entries.values() {
            content.push_str(&serde_json::to_string(entry)?);
            content.push('\n');
        }
        fs::write(&self.registry_file, content)?;
        Ok(())
    }

    pub fn validate_manifest(&self, path: &Path) -> Result<GoatExtensionManifest> {
        if !path.exists() {
            return Err(anyhow!("Extension path does not exist: {}", path.display()));
        }

        let manifest_path = if path.is_file() {
            path.to_path_buf()
        } else {
            let toml_path = path.join("GOAT_EXTENSION.toml");
            if toml_path.exists() {
                toml_path
            } else {
                let alt_path = path.join("goat.extension.toml");
                if alt_path.exists() {
                    alt_path
                } else {
                    return Err(anyhow!("No GOAT_EXTENSION.toml found in directory"));
                }
            }
        };

        let content = fs::read_to_string(&manifest_path)?;
        let manifest: GoatExtensionManifest =
            toml::from_str(&content).map_err(|e| anyhow!("Failed to parse manifest: {}", e))?;

        // Path traversal checks
        let check_paths = |paths: &Vec<String>, kind: &str| -> Result<()> {
            for p in paths {
                if p.contains("..") || p.starts_with('/') {
                    return Err(anyhow!(
                        "Path traversal detected in {} entrypoints: {}",
                        kind,
                        p
                    ));
                }
            }
            Ok(())
        };

        check_paths(&manifest.entrypoints.skills, "skills")?;
        check_paths(
            &manifest.entrypoints.validation_recipes,
            "validation_recipes",
        )?;
        check_paths(&manifest.entrypoints.commands, "commands")?;
        check_paths(&manifest.entrypoints.mcp_servers, "mcp_servers")?;
        check_paths(&manifest.entrypoints.external_agents, "external_agents")?;

        Ok(manifest)
    }

    pub fn install_local(&mut self, path: &Path) -> Result<String> {
        let manifest = self.validate_manifest(path)?;
        let id = manifest.extension.id.clone();

        if let Some(existing) = self.entries.get(&id) {
            if existing.status != ExtensionStatus::Archived {
                return Err(anyhow!("Extension with ID {} is already installed", id));
            }
        }

        let source_dir = if path.is_file() {
            path.parent().unwrap_or(Path::new(""))
        } else {
            path
        };

        let target_dir = self.installed_dir.join(&id);
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }

        // Deep copy the directory
        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        fs_extra::dir::copy(source_dir, &target_dir, &options)
            .map_err(|e| anyhow!("Failed to copy extension files: {}", e))?;

        let entry = ExtensionRegistryEntry {
            manifest,
            status: ExtensionStatus::Disabled,
            install_source: path.to_string_lossy().to_string(),
            installed_path: Some(target_dir),
            installed_at: Utc::now().to_rfc3339(),
        };

        self.entries.insert(id.clone(), entry);
        self.save_registry()?;

        Ok(id)
    }

    pub fn enable(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        if entry.status == ExtensionStatus::Enabled {
            return Ok(());
        }
        entry.status = ExtensionStatus::Enabled;
        self.save_registry()?;
        Ok(())
    }

    pub fn disable(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        if entry.status == ExtensionStatus::Disabled {
            return Ok(());
        }
        entry.status = ExtensionStatus::Disabled;
        self.save_registry()?;
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        entry.status = ExtensionStatus::Archived;
        if let Some(path) = &entry.installed_path {
            if path.exists() {
                fs::remove_dir_all(path)?;
            }
        }
        entry.installed_path = None;
        self.save_registry()?;
        Ok(())
    }

    pub fn list(&self) -> Vec<&ExtensionRegistryEntry> {
        self.entries
            .values()
            .filter(|e| e.status != ExtensionStatus::Archived)
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&ExtensionRegistryEntry> {
        self.entries
            .get(id)
            .filter(|e| e.status != ExtensionStatus::Archived)
    }

    pub fn doctor(&self) -> Result<Vec<String>> {
        let mut findings = Vec::new();

        findings.push(format!("Registry Path: {}", self.registry_file.display()));
        findings.push(format!("Installed Dir: {}", self.installed_dir.display()));

        for entry in self.entries.values() {
            if entry.status == ExtensionStatus::Archived {
                continue;
            }

            let id = &entry.manifest.extension.id;

            if entry.status == ExtensionStatus::Enabled {
                match entry.manifest.extension.risk_level {
                    ExtensionRiskLevel::High | ExtensionRiskLevel::Critical => {
                        findings.push(format!(
                            "WARNING: High/Critical risk extension '{}' is enabled.",
                            id
                        ));
                    }
                    _ => {}
                }
            }

            if let Some(path) = &entry.installed_path {
                if !path.exists() {
                    findings.push(format!(
                        "ERROR: Installed path for '{}' is missing: {}",
                        id,
                        path.display()
                    ));
                } else {
                    for sk in &entry.manifest.entrypoints.skills {
                        if !path.join(sk).exists() {
                            findings.push(format!(
                                "ERROR: Extension '{}' missing skill entrypoint: {}",
                                id, sk
                            ));
                        }
                    }
                }
            } else {
                findings.push(format!("ERROR: Installed path for '{}' is None", id));
            }
        }

        if findings.len() == 2 {
            findings.push("No issues found. Extensions are healthy.".to_string());
        }

        Ok(findings)
    }
}
