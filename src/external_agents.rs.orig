use crate::config::Config;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

// ── Types ────────────────────────────────────────────────────────────────────

pub type ExternalAgentId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalAgentKind {
    OpenCode,
    ClaudeCode,
    GeminiCli,
    CodexCli,
    Aider,
    Cline,
    Hermes,
    JCode,
    Goose,
    OpenHands,
    Custom(String),
}

impl fmt::Display for ExternalAgentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenCode => write!(f, "opencode"),
            Self::ClaudeCode => write!(f, "claude-code"),
            Self::GeminiCli => write!(f, "gemini-cli"),
            Self::CodexCli => write!(f, "codex-cli"),
            Self::Aider => write!(f, "aider"),
            Self::Cline => write!(f, "cline"),
            Self::Hermes => write!(f, "hermes"),
            Self::JCode => write!(f, "jcode"),
            Self::Goose => write!(f, "goose"),
            Self::OpenHands => write!(f, "openhands"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalAgentCapabilities {
    pub supports_non_interactive: bool,
    pub supports_json_output: bool,
    pub supports_file_edits: bool,
    pub supports_approval_internally: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalAgentStatus {
    Detected,
    Missing,
    Unsupported,
    NeedsConfig,
    Disabled,
}

impl fmt::Display for ExternalAgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Detected => write!(f, "Detected"),
            Self::Missing => write!(f, "Missing"),
            Self::Unsupported => write!(f, "Unsupported"),
            Self::NeedsConfig => write!(f, "Needs Config"),
            Self::Disabled => write!(f, "Disabled"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalAgentAdapter {
    pub name: String,
    pub kind: ExternalAgentKind,
    pub command_name: String,
    pub detected_path: Option<PathBuf>,
    pub version: Option<String>,
    pub capabilities: ExternalAgentCapabilities,
    pub risk_level: crate::approval::RiskLevel,
    pub workspace_behavior: String,
    pub license_note: String,
    pub status: ExternalAgentStatus,
}

pub struct ExternalAgentRequest {
    pub task: String,
    pub timeout: Duration,
    pub workspace_mode: String,
}

pub struct ExternalAgentResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub success: bool,
}

pub struct ExternalAgentRun {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub agent_name: String,
    pub command_path: PathBuf,
    pub task: String,
    pub mode: String,
    pub duration: Duration,
    pub exit_code: Option<i32>,
    pub success: bool,
}

// ── Registry ─────────────────────────────────────────────────────────────────

pub struct ExternalAgentRegistry {
    pub adapters: HashMap<ExternalAgentId, ExternalAgentAdapter>,
}

impl ExternalAgentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            adapters: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        self.register(ExternalAgentAdapter {
            name: "opencode".to_string(),
            kind: ExternalAgentKind::OpenCode,
            command_name: "opencode".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: true,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "claude-code".to_string(),
            kind: ExternalAgentKind::ClaudeCode,
            command_name: "claude".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: true,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Proprietary".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "gemini-cli".to_string(),
            kind: ExternalAgentKind::GeminiCli,
            command_name: "gemini".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: true,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Google".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "codex-cli".to_string(),
            kind: ExternalAgentKind::CodexCli,
            command_name: "codex".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "aider".to_string(),
            kind: ExternalAgentKind::Aider,
            command_name: "aider".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "commits to git directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "cline".to_string(),
            kind: ExternalAgentKind::Cline,
            command_name: "cline".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "hermes".to_string(),
            kind: ExternalAgentKind::Hermes,
            command_name: "hermes".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "jcode".to_string(),
            kind: ExternalAgentKind::JCode,
            command_name: "jcode".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: true,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });
        
        self.register(ExternalAgentAdapter {
            name: "goose".to_string(),
            kind: ExternalAgentKind::Goose,
            command_name: "goose".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "modifies files directly".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });
        
        self.register(ExternalAgentAdapter {
            name: "openhands".to_string(),
            kind: ExternalAgentKind::OpenHands,
            command_name: "openhands".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "sandbox / docker execution".to_string(),
            license_note: "Open source".to_string(),
            status: ExternalAgentStatus::Missing,
        });
    }

    pub fn register(&mut self, adapter: ExternalAgentAdapter) {
        self.adapters.insert(adapter.name.clone(), adapter);
    }

    pub fn get(&self, name: &str) -> Option<&ExternalAgentAdapter> {
        self.adapters.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ExternalAgentAdapter> {
        self.adapters.get_mut(name)
    }

    pub fn list_all(&self) -> Vec<&ExternalAgentAdapter> {
        let mut list: Vec<_> = self.adapters.values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}

// ── Manager ──────────────────────────────────────────────────────────────────

pub struct ExternalAgentManager {
    pub registry: ExternalAgentRegistry,
    audit_log_path: PathBuf,
}

impl ExternalAgentManager {
    pub fn new(audit_log_path: PathBuf) -> Self {
        Self {
            registry: ExternalAgentRegistry::new(),
            audit_log_path,
        }
    }

    pub fn detect_all(&mut self, config: &Config) {
        let global_enabled = config.external_agents.enabled;
        
        for (_, adapter) in self.registry.adapters.iter_mut() {
            if !global_enabled {
                adapter.status = ExternalAgentStatus::Disabled;
                continue;
            }

            // Check if explicitly disabled in config
            if let Some(agent_config) = config.external_agents.agents.get(&adapter.name) {
                if !agent_config.enabled {
                    adapter.status = ExternalAgentStatus::Disabled;
                    continue;
                }
            }

            // Safe detection using 'which'
            match which::which(&adapter.command_name) {
                Ok(path) => {
                    adapter.detected_path = Some(path);
                    adapter.status = ExternalAgentStatus::Detected;
                    
                    // Note: Version detection is intentionally deferred for Phase 2.8 to avoid spawning subprocesses
                    // during detection. `which` is purely an fs check.
                }
                Err(_) => {
                    adapter.detected_path = None;
                    adapter.status = ExternalAgentStatus::Missing;
                }
            }
        }
    }

    pub fn delegate(
        &self,
        name: &str,
        task: &str,
        config: &Config,
    ) -> Result<ExternalAgentResponse> {
        let adapter = self.registry.get(name).ok_or_else(|| anyhow!("External agent '{}' not found in registry", name))?;

        if !config.external_agents.enabled {
            return Err(anyhow!("External agent framework is disabled in configuration."));
        }

        if adapter.status == ExternalAgentStatus::Disabled {
            return Err(anyhow!("External agent '{}' is disabled.", name));
        }

        if adapter.status == ExternalAgentStatus::Missing {
            return Err(anyhow!("External agent '{}' binary '{}' was not detected in PATH.", name, adapter.command_name));
        }

        if !config.external_agents.allow_execution {
            return Err(anyhow!("External agent execution is disabled globally (allow_execution = false). Detected path: {:?}", adapter.detected_path));
        }

        let agent_config_allow = config.external_agents.agents.get(name).map(|c| c.allow_execution).unwrap_or(false);
        if !agent_config_allow {
            return Err(anyhow!("External agent '{}' execution is not explicitly allowed in config.", name));
        }

        if !adapter.capabilities.supports_non_interactive {
            return Err(anyhow!("External agent '{}' does not support safe non-interactive execution.", name));
        }

        // For Phase 2.8, we implement a safe dry-run / minimal launch
        // In real execution, we would stream stdout/stderr, but for now we capture.
        
        let binary_path = adapter.detected_path.as_ref().unwrap();
        
        let timeout_secs = config.external_agents.default_timeout_secs;
        
        let start = Instant::now();
        
        // This is a placeholder for actual argument mapping per agent.
        // In Phase 2.8 we are just verifying the subprocess architecture.
        // We will pass the task as an argument.
        
        let mut cmd = Command::new(binary_path);
        
        // Custom argument formats per agent:
        match adapter.kind {
            ExternalAgentKind::OpenCode | ExternalAgentKind::JCode => {
                cmd.arg(task);
            }
            ExternalAgentKind::Aider => {
                cmd.arg("--message").arg(task);
            }
            ExternalAgentKind::ClaudeCode => {
                cmd.arg("-p").arg(task);
            }
            _ => {
                cmd.arg(task);
            }
        }

        let run_mode = config.external_agents.workspace_mode.clone();

        // Note: For Phase 2.8, since we don't fully trust agents yet, if they are modifying files
        // we should technically isolate them, but the user must opt-in via config.
        
        // Log execution start
        self.log_execution(&ExternalAgentRun {
            timestamp: chrono::Utc::now(),
            agent_name: adapter.name.clone(),
            command_path: binary_path.clone(),
            task: task.to_string(),
            mode: run_mode.clone(),
            duration: Duration::from_secs(0),
            exit_code: None,
            success: false, // not done yet
        });

        // Normally we'd use tokio timeout here, but since this is sync/blocking for now or we spawn:
        // We will just do standard process for Phase 2.8.
        let output = cmd.output().map_err(|e| anyhow!("Failed to spawn external agent: {}", e))?;
        
        let duration = start.elapsed();
        let success = output.status.success();
        
        let run_record = ExternalAgentRun {
            timestamp: chrono::Utc::now(),
            agent_name: adapter.name.clone(),
            command_path: binary_path.clone(),
            task: task.to_string(),
            mode: run_mode,
            duration,
            exit_code: output.status.code(),
            success,
        };
        
        self.log_execution(&run_record);

        Ok(ExternalAgentResponse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success,
        })
    }

    fn log_execution(&self, run: &ExternalAgentRun) {
        use std::fs::OpenOptions;
        use std::io::Write;

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_log_path)
        {
            let log_entry = format!(
                "[{}] Agent: {} | Path: {} | Mode: {} | Task: {} | Duration: {}s | ExitCode: {:?} | Success: {}\n",
                run.timestamp.to_rfc3339(),
                run.agent_name,
                run.command_path.display(),
                run.mode,
                crate::approval::redact_secrets(&run.task),
                run.duration.as_secs(),
                run.exit_code,
                run.success,
            );
            let _ = file.write_all(log_entry.as_bytes());
        }
    }
}
