use anyhow::{Result, bail, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::SystemTime;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationType {
    Test,
    Build,
    Lint,
    Format,
    Typecheck,
    Dev,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Pending,
    ApprovalRequired,
    Running,
    Passed,
    Failed,
    Cancelled,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_id: String,
    pub mission_id: Option<String>,
    pub project_id: Option<String>,
    pub patch_id: Option<String>,
    pub command: String,
    pub command_type: ValidationType,
    pub working_directory: PathBuf,
    pub status: ValidationStatus,
    pub exit_code: Option<i32>,
    pub stdout_preview: Option<String>,
    pub stderr_preview: Option<String>,
    pub full_log_path: Option<PathBuf>,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
    pub duration_ms: Option<u64>,
    pub summary: Option<String>,
    pub suggested_next_action: Option<String>,
}

#[derive(Clone)]
pub struct ValidationManager {
    base_dir: PathBuf,
}

impl ValidationManager {
    pub fn new() -> Self {
        let base_dir = crate::paths::GoatPaths::resolve()
            .map(|p| p.data_dir.join("validation-logs"))
            .unwrap_or_else(|_| PathBuf::from("~/.local/share/goat/validation-logs"));
        
        if !base_dir.exists() {
            let _ = fs::create_dir_all(&base_dir);
        }

        Self { base_dir }
    }

    pub fn generate_commands(
        &self,
        project: &crate::project_intelligence::ProjectIntelligence,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        let root = project.root_path.clone();
        let proj_id = project.project_id.clone();

        for lang in &project.languages {
            let lang_lower = lang.to_lowercase();
            if lang_lower == "rust" {
                results.push(self.create_pending(&proj_id, "cargo test", ValidationType::Test, &root));
                results.push(self.create_pending(&proj_id, "cargo check", ValidationType::Typecheck, &root));
                results.push(self.create_pending(&proj_id, "cargo fmt --check", ValidationType::Format, &root));
            } else if lang_lower == "typescript" || lang_lower == "javascript" {
                if project.package_managers.contains(&"npm".to_string()) {
                    results.push(self.create_pending(&proj_id, "npm run build", ValidationType::Build, &root));
                    results.push(self.create_pending(&proj_id, "npm run lint", ValidationType::Lint, &root));
                } else if project.package_managers.contains(&"pnpm".to_string()) {
                    results.push(self.create_pending(&proj_id, "pnpm build", ValidationType::Build, &root));
                    results.push(self.create_pending(&proj_id, "pnpm lint", ValidationType::Lint, &root));
                }
            } else if lang_lower == "python" {
                results.push(self.create_pending(&proj_id, "python -m pytest", ValidationType::Test, &root));
            }
        }

        results
    }

    pub fn create_pending(
        &self,
        project_id: &str,
        command: &str,
        command_type: ValidationType,
        working_dir: &Path,
    ) -> ValidationResult {
        ValidationResult {
            validation_id: Uuid::new_v4().to_string(),
            mission_id: None,
            project_id: Some(project_id.to_string()),
            patch_id: None,
            command: command.to_string(),
            command_type,
            working_directory: working_dir.to_path_buf(),
            status: ValidationStatus::Pending,
            exit_code: None,
            stdout_preview: None,
            stderr_preview: None,
            full_log_path: None,
            started_at: None,
            finished_at: None,
            duration_ms: None,
            summary: None,
            suggested_next_action: None,
        }
    }

    pub fn save_validation(&self, res: &ValidationResult) -> Result<()> {
        let path = self.base_dir.join(format!("{}.json", res.validation_id));
        let data = serde_json::to_string_pretty(res)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn get_validation(&self, id: &str) -> Result<Option<ValidationResult>> {
        let path = self.base_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(&path)?;
        let val: ValidationResult = serde_json::from_str(&data)?;
        Ok(Some(val))
    }

    pub fn list_validations(&self) -> Result<Vec<ValidationResult>> {
        let mut items = Vec::new();
        if !self.base_dir.exists() {
            return Ok(items);
        }
        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(val) = serde_json::from_str::<ValidationResult>(&data) {
                        items.push(val);
                    }
                }
            }
        }
        items.sort_by(|a, b| b.started_at.unwrap_or(0).cmp(&a.started_at.unwrap_or(0)));
        Ok(items)
    }

    pub async fn run_validation(
        &self,
        mut val: ValidationResult,
        approval_queue: &crate::approval::ApprovalQueue,
    ) -> Result<ValidationResult> {
        // 1. Approval Gate
        let req = crate::approval::ApprovalRequest {
            tool_name: "ValidationRunner".to_string(),
            action_summary: format!("Run {} in {}", val.command, val.working_directory.display()),
            risk_level: crate::approval::RiskLevel::Medium,
            explanation: Some(format!("Execute {:?} command automatically", val.command_type)),
            working_directory: Some(val.working_directory.to_string_lossy().to_string()),
        };

        let (pending, rx) = approval_queue.add(req, "validation").await;
        
        let decision = rx.await.unwrap_or('n');

        if decision != 'y' && decision != 'a' {
            val.status = ValidationStatus::Cancelled;
            val.summary = Some(format!("Approval denied"));
            self.save_validation(&val)?;
            bail!("Validation denied");
        }

        // 2. Prepare Execution
        val.status = ValidationStatus::Running;
        let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
        val.started_at = Some(start_time / 1000);
        self.save_validation(&val)?;

        let mut parts = val.command.split_whitespace();
        let cmd_str = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        let mut child = Command::new(cmd_str)
            .args(&args)
            .current_dir(&val.working_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn validation command")?;

        let mut stdout = child.stdout.take().expect("Failed to open stdout");
        let mut stderr = child.stderr.take().expect("Failed to open stderr");

        let mut out_str = String::new();
        let mut err_str = String::new();

        let out_task = tokio::spawn(async move {
            stdout.read_to_string(&mut out_str).await.unwrap_or(0);
            out_str
        });

        let err_task = tokio::spawn(async move {
            stderr.read_to_string(&mut err_str).await.unwrap_or(0);
            err_str
        });

        // Add 2 minute timeout
        let status_res = tokio::time::timeout(std::time::Duration::from_secs(120), child.wait()).await;
        
        let out_str = out_task.await.unwrap_or_default();
        let err_str = err_task.await.unwrap_or_default();

        let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
        val.finished_at = Some(end_time / 1000);
        val.duration_ms = Some(end_time - start_time);

        let log_path = self.base_dir.join(format!("{}.log", val.validation_id));
        fs::write(&log_path, format!("STDOUT:\n{}\n\nSTDERR:\n{}", out_str, err_str))?;
        val.full_log_path = Some(log_path);

        val.stdout_preview = Some(out_str.chars().take(500).collect());
        val.stderr_preview = Some(err_str.chars().take(500).collect());

        match status_res {
            Ok(Ok(status)) => {
                val.exit_code = status.code();
                if status.success() {
                    val.status = ValidationStatus::Passed;
                    val.summary = Some("Command completed successfully".to_string());
                } else {
                    val.status = ValidationStatus::Failed;
                    val.summary = Some("Command failed with errors".to_string());
                    val.suggested_next_action = Some(format!("Check logs and fix errors in {}", val.working_directory.display()));
                }
            }
            Ok(Err(e)) => {
                val.status = ValidationStatus::Failed;
                val.summary = Some(format!("Failed to wait on command: {}", e));
            }
            Err(_) => {
                val.status = ValidationStatus::Failed;
                val.summary = Some("Command timed out after 120 seconds".to_string());
                let _ = child.kill().await;
            }
        }

        self.save_validation(&val)?;

        // If part of a mission, update mission control
        if let Some(mission_id) = &val.mission_id {
            let mc = crate::mission_control::MissionControlManager::new();
            if let Some(mut mission) = mc.get_missions().into_iter().find(|m| m.mission_id == *mission_id) {
                // Determine step or action
                let event = format!("Validation {} ({}) finished with status: {:?}", val.validation_id, val.command, val.status);
                mission.plan_steps.push(crate::mission_control::MissionPlanStep {
                    id: Uuid::new_v4().to_string(),
                    title: format!("Validation: {}", val.command),
                    description: event,
                    assigned_agent: None,
                    status: if val.status == ValidationStatus::Passed { 
                        "Completed".to_string()
                    } else { 
                        "Failed".to_string()
                    },
                });
                if val.status == ValidationStatus::Failed {
                    if let Some(na) = &val.suggested_next_action {
                        mission.next_actions.push(na.clone());
                    }
                } else {
                    mission.next_actions.push("Proceed with next mission steps".to_string());
                }
                mc.update_mission(&mission);
            }
        }

        Ok(val)
    }
}
