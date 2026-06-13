use crate::config::{Config, ExternalWorkspaceMode};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
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
    Generic,
    OpenClaw,
    Littlebird,
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
            Self::Generic => write!(f, "generic"),
            Self::OpenClaw => write!(f, "openclaw"),
            Self::Littlebird => write!(f, "littlebird"),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAgentRun {
    pub run_id: String,
    pub agent_name: String,
    pub command: String,
    pub task_summary: String,
    pub working_directory: PathBuf,
    pub permission_profile: String,
    pub status: String,
    pub approval_decision: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub exit_code: Option<i32>,
    pub stdout_log_path: Option<PathBuf>,
    pub stderr_log_path: Option<PathBuf>,
    pub mission_id: Option<String>,
    pub project_id: Option<String>,
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

        self.register(ExternalAgentAdapter {
            name: "generic".to_string(),
            kind: ExternalAgentKind::Generic,
            command_name: "bash".to_string(),
            detected_path: None,
            version: None,
            capabilities: ExternalAgentCapabilities {
                supports_non_interactive: true,
                supports_json_output: false,
                supports_file_edits: true,
                supports_approval_internally: false,
            },
            risk_level: crate::approval::RiskLevel::High,
            workspace_behavior: "runs in current terminal".to_string(),
            license_note: "System".to_string(),
            status: ExternalAgentStatus::Missing,
        });

        self.register(ExternalAgentAdapter {
            name: "openclaw".to_string(),
            kind: ExternalAgentKind::OpenClaw,
            command_name: "openclaw".to_string(),
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
            name: "littlebird".to_string(),
            kind: ExternalAgentKind::Littlebird,
            command_name: "littlebird".to_string(),
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
    data_dir: PathBuf,
}

impl ExternalAgentManager {
    pub fn new(audit_log_path: PathBuf, data_dir: PathBuf) -> Self {
        Self {
            registry: ExternalAgentRegistry::new(),
            audit_log_path,
            data_dir,
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
        approval_decision: crate::approval::ApprovalDecision,
        mission_id: Option<String>,
    ) -> Result<ExternalAgentResponse> {
        let adapter = self
            .registry
            .get(name)
            .ok_or_else(|| anyhow!("External agent '{}' not found in registry", name))?;

        if !config.external_agents.enabled {
            return Err(anyhow!(
                "External agent framework is disabled in configuration."
            ));
        }

        if adapter.status == ExternalAgentStatus::Disabled {
            return Err(anyhow!("External agent '{}' is disabled.", name));
        }

        if adapter.status == ExternalAgentStatus::Missing {
            return Err(anyhow!(
                "External agent '{}' binary '{}' was not detected in PATH.",
                name,
                adapter.command_name
            ));
        }

        if !config.external_agents.allow_execution {
            return Err(anyhow!(
                "External agent execution is disabled globally (allow_execution = false)."
            ));
        }

        let agent_config_allow = config
            .external_agents
            .agents
            .get(name)
            .map(|c| c.allow_execution)
            .unwrap_or(false);
        if !agent_config_allow {
            return Err(anyhow!(
                "External agent '{}' execution is not explicitly allowed in config.",
                name
            ));
        }

        if !adapter.capabilities.supports_non_interactive {
            return Err(anyhow!(
                "External agent '{}' does not support safe non-interactive execution.",
                name
            ));
        }

        let binary_path = adapter.detected_path.as_ref().unwrap();
        let timeout_secs = config.external_agents.default_timeout_secs;
        let start = Instant::now();

        let mut cmd = Command::new(binary_path);
        let mut cmd_str = binary_path.to_string_lossy().to_string();

        match adapter.kind {
            ExternalAgentKind::OpenCode | ExternalAgentKind::JCode => {
                cmd.arg(task);
                cmd_str = format!("{} {}", cmd_str, task);
            }
            ExternalAgentKind::Aider => {
                cmd.arg("--message").arg(task);
                cmd_str = format!("{} --message {}", cmd_str, task);
            }
            ExternalAgentKind::ClaudeCode => {
                cmd.arg("-p").arg(task);
                cmd_str = format!("{} -p {}", cmd_str, task);
            }
            ExternalAgentKind::Generic => {
                cmd.arg("-c").arg(task);
                cmd_str = format!("{} -c \"{}\"", cmd_str, task);
            }
            _ => {
                cmd.arg(task);
                cmd_str = format!("{} {}", cmd_str, task);
            }
        }

        let run_mode = config.external_agents.workspace_mode.clone();
        let run_id = uuid::Uuid::new_v4().to_string();
        let mut workspace_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        if run_mode == ExternalWorkspaceMode::IsolatedCopy {
            let target_dir = self.data_dir.join("external-runs").join(&run_id);
            if let Err(e) = copy_safe_workspace(&workspace_path, &target_dir) {
                return Err(anyhow!("Failed to create isolated workspace: {}", e));
            }
            workspace_path = target_dir;
            cmd.current_dir(&workspace_path);
        } else if run_mode == ExternalWorkspaceMode::DetectOnly {
            return Err(anyhow!(
                "External execution is configured to detect-only. Execution aborted."
            ));
        }

        let permission_profile = config.external_agents.workspace_mode.to_string();

        let mut run_record = ExternalAgentRun {
            run_id: run_id.clone(),
            agent_name: adapter.name.clone(),
            command: cmd_str,
            task_summary: task.to_string(),
            working_directory: workspace_path.clone(),
            permission_profile,
            status: "pending".to_string(),
            approval_decision: format!("{:?}", approval_decision),
            started_at: chrono::Utc::now(),
            finished_at: None,
            exit_code: None,
            stdout_log_path: None,
            stderr_log_path: None,
            mission_id,
            project_id: None,
        };

        if let crate::approval::ApprovalDecision::Approved = approval_decision {
            run_record.status = "running".to_string();
            self.record_run(&run_record, None, None);

            let output = cmd
                .output()
                .map_err(|e| anyhow!("Failed to spawn external agent: {}", e))?;

            let success = output.status.success();
            let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

            run_record.status = if success {
                "success".to_string()
            } else {
                "failed".to_string()
            };
            run_record.finished_at = Some(chrono::Utc::now());
            run_record.exit_code = output.status.code();

            self.record_run(&run_record, Some(&stdout_str), Some(&stderr_str));

            Ok(ExternalAgentResponse {
                stdout: stdout_str,
                stderr: stderr_str,
                exit_code: output.status.code(),
                success,
            })
        } else {
            run_record.status = "denied".to_string();
            run_record.finished_at = Some(chrono::Utc::now());
            self.record_run(&run_record, None, None);
            Err(anyhow!("Execution denied by ApprovalGate."))
        }
    }

    pub fn record_run(&self, run: &ExternalAgentRun, stdout: Option<&str>, stderr: Option<&str>) {
        use std::fs::{self, OpenOptions};
        use std::io::Write;

        let run_dir = self.data_dir.join("external-agent-runs").join(&run.run_id);
        if !run_dir.exists() {
            let _ = fs::create_dir_all(&run_dir);
        }

        let mut run_to_save = run.clone();

        if let Some(out) = stdout {
            let stdout_path = run_dir.join("stdout.log");
            let _ = fs::write(&stdout_path, out);
            run_to_save.stdout_log_path = Some(stdout_path);
        }

        if let Some(err) = stderr {
            let stderr_path = run_dir.join("stderr.log");
            let _ = fs::write(&stderr_path, err);
            run_to_save.stderr_log_path = Some(stderr_path);
        }

        let meta_path = run_dir.join("metadata.json");
        if let Ok(json_str) = serde_json::to_string_pretty(&run_to_save) {
            let _ = fs::write(&meta_path, json_str);
        }

        // Also append to the JSONL global log
        let jsonl_path = self.data_dir.join("external-agent-runs.jsonl");
        if let Ok(mut json_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&jsonl_path)
        {
            if let Ok(json_str) = serde_json::to_string(&run_to_save) {
                let _ = writeln!(json_file, "{}", json_str);
            }
        }
    }

    pub fn get_run(&self, id: &str) -> Option<ExternalAgentRun> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        if let Ok(file) = File::open(&self.audit_log_path) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|l| l.ok()) {
                if let Ok(run) = serde_json::from_str::<ExternalAgentRun>(&line) {
                    if run.run_id == id {
                        return Some(run);
                    }
                }
            }
        }
        None
    }
}

fn copy_safe_workspace(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    let ignore_names = [
        ".git",
        "node_modules",
        "target",
        "dist",
        "build",
        ".next",
        ".turbo",
        ".cache",
        "venv",
        ".venv",
        "__pycache__",
        ".env",
        "secrets",
        "credentials",
        "keys",
    ];
    fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path, ignore: &[&str]) -> Result<()> {
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ft = entry.file_type()?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().to_lowercase();
            if ignore.iter().any(|&i| file_name_str.contains(i)) {
                continue;
            }
            let dst_path = dst.join(file_name);
            if ft.is_dir() {
                std::fs::create_dir_all(&dst_path)?;
                copy_dir_all(&entry.path(), &dst_path, ignore)?;
            } else {
                let _ = std::fs::copy(&entry.path(), &dst_path);
            }
        }
        Ok(())
    }
    copy_dir_all(src, dst, &ignore_names)
}
