use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionKind {
    Tool,
    McpServer,
    SkillPack,
    RecipePack,
    PromptPack,
    WorkflowPack,
    ProviderProfile,
    AgentExtension,
    DashboardCard,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionStatus {
    Discovered,
    Installed,
    Enabled,
    Disabled,
    Quarantined,
    Deprecated,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionTrustLevel {
    LocalBuiltin,
    LocalUser,
    RemoteUntrusted,
    RemoteAudited,
    VerifiedLater,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionPermission {
    ReadProject,
    WriteProject,
    ReadHomeLimited,
    ShellCommand,
    NetworkAccess,
    BrowserAccess,
    GithubAccess,
    TransportAccess,
    MemoryRead,
    MemoryWrite,
    ProviderAccess,
    SecretEnvAccess,
    FileSystemWide,
    ExternalSideEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSource {
    LocalBuiltin,
    LocalFolder(PathBuf),
    RemoteIndexLater(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub kind: ExtensionKind,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
    pub license: Option<String>,
    pub entry_type: Option<String>,
    pub entry_ref: Option<String>,
    #[serde(default)]
    pub permissions: Vec<ExtensionPermission>,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub recipes: Vec<String>,
    #[serde(default)]
    pub mcp_servers: Vec<serde_json::Value>,
    #[serde(default)]
    pub provider_profiles: Vec<serde_json::Value>,
    #[serde(default)]
    pub prompt_templates: Vec<String>,
    pub safety_notes: Option<String>,
    pub install_notes: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRecord {
    pub manifest: ExtensionManifest,
    pub status: ExtensionStatus,
    pub trust_level: ExtensionTrustLevel,
    pub source: ExtensionSource,
    pub install_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAuditFinding {
    pub severity: AuditSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAuditResult {
    pub extension_id: String,
    pub findings: Vec<ExtensionAuditFinding>,
    pub passed: bool,
}

pub struct ExtensionRegistry {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    records: HashMap<String, ExtensionRecord>,
}

impl ExtensionRegistry {
    pub fn new(config_dir: &Path, data_dir: &Path) -> Result<Self> {
        let registry = Self {
            config_dir: config_dir.join("extensions"),
            data_dir: data_dir.join("extensions"),
            records: HashMap::new(),
        };
        registry.init_directories()?;
        Ok(registry)
    }

    fn init_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(self.data_dir.join("installed"))?;
        fs::create_dir_all(self.data_dir.join("catalog"))?;
        fs::create_dir_all(self.data_dir.join("audit"))?;
        fs::create_dir_all(self.data_dir.join("artifacts"))?;
        Ok(())
    }

    pub fn load_state(&mut self) -> Result<()> {
        let enabled_path = self.config_dir.join("enabled.json");
        let trust_path = self.config_dir.join("trust.json");

        // This is a stub for loading the state from disk
        // In a real implementation we would load all ExtensionRecords

        // Let's add some built-in extensions for demonstration
        self.add_builtin_catalog();

        Ok(())
    }

    pub fn save_state(&self) -> Result<()> {
        // Stub for saving
        Ok(())
    }

    fn add_builtin_catalog(&mut self) {
        let builtin_manifests = vec![
            ExtensionManifest {
                id: "goat.builtin.github-pr-review".to_string(),
                name: "GitHub PR Review Pack".to_string(),
                version: "1.0.0".to_string(),
                description: "Skill pack to review PRs automatically".to_string(),
                kind: ExtensionKind::SkillPack,
                author: Some("GOAT".to_string()),
                homepage: None,
                source_url: None,
                license: Some("MIT".to_string()),
                entry_type: None,
                entry_ref: None,
                permissions: vec![
                    ExtensionPermission::GithubAccess,
                    ExtensionPermission::ReadProject,
                ],
                commands: vec![],
                tools: vec![],
                skills: vec!["github_pr_reviewer".to_string()],
                recipes: vec![],
                mcp_servers: vec![],
                provider_profiles: vec![],
                prompt_templates: vec![],
                safety_notes: Some("Requires GitHub read-only access".to_string()),
                install_notes: None,
                created_at: Some(Utc::now().to_rfc3339()),
            },
            ExtensionManifest {
                id: "goat.builtin.react-ui-review".to_string(),
                name: "React UI Review Skill Pack".to_string(),
                version: "1.0.0".to_string(),
                description: "Audits React UI against best practices".to_string(),
                kind: ExtensionKind::SkillPack,
                author: Some("GOAT".to_string()),
                homepage: None,
                source_url: None,
                license: Some("MIT".to_string()),
                entry_type: None,
                entry_ref: None,
                permissions: vec![ExtensionPermission::ReadProject],
                commands: vec![],
                tools: vec![],
                skills: vec!["react_ui_audit".to_string()],
                recipes: vec![],
                mcp_servers: vec![],
                provider_profiles: vec![],
                prompt_templates: vec![],
                safety_notes: None,
                install_notes: None,
                created_at: Some(Utc::now().to_rfc3339()),
            },
        ];

        for m in builtin_manifests {
            self.records.insert(
                m.id.clone(),
                ExtensionRecord {
                    manifest: m,
                    status: ExtensionStatus::Discovered,
                    trust_level: ExtensionTrustLevel::LocalBuiltin,
                    source: ExtensionSource::LocalBuiltin,
                    install_path: None,
                },
            );
        }
    }

    pub fn list_extensions(&self) -> Vec<&ExtensionRecord> {
        self.records.values().collect()
    }

    pub fn get_extension(&self, id: &str) -> Option<&ExtensionRecord> {
        self.records.get(id)
    }

    pub fn discover_local(&mut self, path: &Path) -> Result<String> {
        if !path.exists() {
            return Err(anyhow!("Path does not exist"));
        }

        let manifest_path = if path.is_file() {
            path.to_path_buf()
        } else {
            let json_path = path.join("manifest.json");
            let toml_path = path.join("manifest.toml");
            if json_path.exists() {
                json_path
            } else if toml_path.exists() {
                toml_path
            } else {
                return Err(anyhow!(
                    "No manifest.json or manifest.toml found in directory"
                ));
            }
        };

        let content = fs::read_to_string(&manifest_path)?;
        let manifest: ExtensionManifest =
            if manifest_path.extension().and_then(|e| e.to_str()) == Some("toml") {
                toml::from_str(&content)?
            } else {
                serde_json::from_str(&content)?
            };

        let id = manifest.id.clone();

        self.records.insert(
            id.clone(),
            ExtensionRecord {
                manifest,
                status: ExtensionStatus::Discovered,
                trust_level: ExtensionTrustLevel::LocalUser,
                source: ExtensionSource::LocalFolder(path.to_path_buf()),
                install_path: None,
            },
        );

        self.save_state()?;
        Ok(id)
    }

    pub fn audit_extension(&self, id: &str) -> Result<ExtensionAuditResult> {
        let record = self
            .records
            .get(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        let mut findings = Vec::new();

        if record.manifest.author.is_none() {
            findings.push(ExtensionAuditFinding {
                severity: AuditSeverity::Info,
                message: "Author is not specified".to_string(),
            });
        }

        if record.manifest.license.is_none() {
            findings.push(ExtensionAuditFinding {
                severity: AuditSeverity::Low,
                message: "No license specified".to_string(),
            });
        }

        let dangerous_perms = [
            ExtensionPermission::ShellCommand,
            ExtensionPermission::WriteProject,
            ExtensionPermission::NetworkAccess,
            ExtensionPermission::BrowserAccess,
            ExtensionPermission::GithubAccess,
            ExtensionPermission::SecretEnvAccess,
            ExtensionPermission::FileSystemWide,
            ExtensionPermission::ExternalSideEffect,
        ];

        let mut has_critical = false;
        for perm in &record.manifest.permissions {
            if dangerous_perms.contains(perm) {
                has_critical = true;
                findings.push(ExtensionAuditFinding {
                    severity: AuditSeverity::High,
                    message: format!("Requests dangerous permission: {:?}", perm),
                });
            }
        }

        match record.source {
            ExtensionSource::RemoteIndexLater(_) => {
                findings.push(ExtensionAuditFinding {
                    severity: AuditSeverity::Medium,
                    message: "Source is remote and untrusted".to_string(),
                });
            }
            _ => {}
        }

        Ok(ExtensionAuditResult {
            extension_id: id.to_string(),
            findings,
            passed: !has_critical,
        })
    }

    pub fn install_extension(&mut self, id: &str) -> Result<()> {
        let mut record = self
            .records
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;

        if record.status != ExtensionStatus::Discovered
            && record.status != ExtensionStatus::Disabled
        {
            return Err(anyhow!("Extension is not in a state to be installed"));
        }

        // Simulating install logic
        let target_path = self.data_dir.join("installed").join(&record.manifest.id);
        fs::create_dir_all(&target_path)?;

        record.install_path = Some(target_path);
        record.status = ExtensionStatus::Disabled; // Always default to disabled

        self.save_state()?;
        Ok(())
    }

    pub fn enable_extension(&mut self, id: &str) -> Result<()> {
        let mut record = self
            .records
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        if record.status != ExtensionStatus::Disabled && record.status != ExtensionStatus::Installed
        {
            return Err(anyhow!(
                "Extension must be installed and disabled before enabling"
            ));
        }

        record.status = ExtensionStatus::Enabled;
        self.save_state()?;
        Ok(())
    }

    pub fn disable_extension(&mut self, id: &str) -> Result<()> {
        let mut record = self
            .records
            .get_mut(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        if record.status != ExtensionStatus::Enabled {
            return Err(anyhow!("Extension is not enabled"));
        }

        record.status = ExtensionStatus::Disabled;
        self.save_state()?;
        Ok(())
    }

    pub fn remove_extension(&mut self, id: &str) -> Result<()> {
        let record = self
            .records
            .get(id)
            .ok_or_else(|| anyhow!("Extension not found"))?;
        if record.status == ExtensionStatus::Enabled {
            return Err(anyhow!(
                "Cannot remove an enabled extension. Disable it first."
            ));
        }

        if let Some(path) = &record.install_path {
            if path.exists() {
                fs::remove_dir_all(path)?;
            }
        }

        self.records.remove(id);
        self.save_state()?;
        Ok(())
    }
}
