use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CodeExecutionStatus {
    Draft,
    WaitingForApproval,
    Approved,
    CheckpointCreated,
    Applying,
    Validating,
    Completed,
    Failed,
    RolledBack,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeExecutionRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDiffHunk {
    pub header: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDiffFile {
    pub file_path: String,
    pub hunks: Vec<CodeDiffHunk>,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDiffSummary {
    pub total_files_changed: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDiffPreview {
    pub files: Vec<CodeDiffFile>,
    pub summary: CodeDiffSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Pending,
    Running,
    Passed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCommand {
    pub command: String,
    pub args: Vec<String>,
    pub run_at_root: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRun {
    pub id: String,
    pub command: ValidationCommand,
    pub status: ValidationStatus,
    pub output: Option<ValidationOutput>,
    pub findings: Vec<ValidationFinding>,
    pub executed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionStep {
    pub order: usize,
    pub action: String,
    pub target_file: String,
    pub step_risk: String,
    pub new_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionArtifact {
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConflict {
    pub file_path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub success: bool,
    pub conflicts: Vec<RollbackConflict>,
    pub restored_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionSession {
    pub id: String,
    pub builder_plan_id: Option<String>,
    pub goal: String,
    pub status: CodeExecutionStatus,
    pub affected_files: Vec<String>,
    pub patch_preview: Option<CodeDiffPreview>,
    pub checkpoint_id: Option<String>,
    pub approval_ref: Option<String>,
    pub validation_commands: Vec<ValidationCommand>,
    pub validation_results: Vec<ValidationRun>,
    pub execution_steps: Vec<CodeExecutionStep>,
    pub artifacts: Vec<CodeExecutionArtifact>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct CodeExecutionManager {
    base_dir: PathBuf,
    retry_dir: PathBuf,
    analysis_dir: PathBuf,
}

impl CodeExecutionManager {
    pub fn new(data_dir: &Path) -> Self {
        let base_dir = data_dir.join("code_executions");
        let retry_dir = data_dir.join("code_retries");
        let analysis_dir = data_dir.join("code_analyses");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(&retry_dir);
        let _ = fs::create_dir_all(&analysis_dir);
        Self {
            base_dir,
            retry_dir,
            analysis_dir,
        }
    }

    pub fn create_session(
        &self,
        goal: &str,
        builder_plan_id: Option<String>,
        affected_files: Vec<String>,
        execution_steps: Vec<CodeExecutionStep>,
    ) -> Result<CodeExecutionSession> {
        let session = CodeExecutionSession {
            id: Uuid::new_v4().to_string(),
            builder_plan_id,
            goal: goal.to_string(),
            status: CodeExecutionStatus::Draft,
            affected_files,
            patch_preview: None,
            checkpoint_id: None,
            approval_ref: None,
            validation_commands: vec![],
            validation_results: vec![],
            execution_steps,
            artifacts: vec![],
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        self.save_session(&session)?;
        Ok(session)
    }

    pub fn save_session(&self, session: &CodeExecutionSession) -> Result<()> {
        let path = self.base_dir.join(format!("{}.json", session.id));
        fs::write(path, serde_json::to_string_pretty(session)?)?;
        Ok(())
    }

    pub fn get_session(&self, id: &str) -> Result<Option<CodeExecutionSession>> {
        let path = self.base_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let session = serde_json::from_str(&content)?;
        Ok(Some(session))
    }

    pub fn list_sessions(&self) -> Result<Vec<CodeExecutionSession>> {
        let mut sessions = Vec::new();
        if !self.base_dir.exists() {
            return Ok(sessions);
        }
        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(session) = serde_json::from_str::<CodeExecutionSession>(&content) {
                        sessions.push(session);
                    }
                }
            }
        }
        sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(sessions)
    }

    pub fn set_status(&self, id: &str, status: CodeExecutionStatus) -> Result<()> {
        if let Some(mut session) = self.get_session(id)? {
            session.status = status;
            session.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.save_session(&session)?;
        }
        Ok(())
    }

    pub fn save_retry_plan(&self, plan: &crate::code_retry::BuilderRetryPlan) -> Result<()> {
        let path = self.retry_dir.join(format!("{}.json", plan.id));
        fs::write(path, serde_json::to_string_pretty(plan)?)?;
        Ok(())
    }

    pub fn get_retry_plan(&self, id: &str) -> Result<Option<crate::code_retry::BuilderRetryPlan>> {
        let path = self.retry_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let plan = serde_json::from_str(&content)?;
        Ok(Some(plan))
    }

    pub fn save_analysis(
        &self,
        analysis: &crate::code_retry::ValidationFailureAnalysis,
    ) -> Result<()> {
        let path = self
            .analysis_dir
            .join(format!("{}.json", analysis.session_id));
        fs::write(path, serde_json::to_string_pretty(analysis)?)?;
        Ok(())
    }

    pub fn get_analysis(
        &self,
        session_id: &str,
    ) -> Result<Option<crate::code_retry::ValidationFailureAnalysis>> {
        let path = self.analysis_dir.join(format!("{}.json", session_id));
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let analysis = serde_json::from_str(&content)?;
        Ok(Some(analysis))
    }

    pub fn execute_validation(
        &self,
        id: &str,
        mut cmd: ValidationCommand,
        working_dir: &Path,
    ) -> Result<ValidationRun> {
        let run_id = Uuid::new_v4().to_string();

        let mut proc = Command::new(&cmd.command);
        proc.args(&cmd.args);

        if cmd.run_at_root {
            proc.current_dir(working_dir);
        }

        let output = proc.output().context("Failed to run validation command")?;

        let val_out = ValidationOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code(),
        };

        let passed = output.status.success();

        let run = ValidationRun {
            id: run_id,
            command: cmd,
            status: if passed {
                ValidationStatus::Passed
            } else {
                ValidationStatus::Failed
            },
            output: Some(val_out),
            findings: vec![],
            executed_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        if let Some(mut session) = self.get_session(id)? {
            session.validation_results.push(run.clone());
            self.save_session(&session)?;
        }

        Ok(run)
    }

    pub fn apply_patch(&self, id: &str, working_dir: &Path) -> Result<()> {
        let mut session = self.get_session(id)?.context("Session not found")?;

        if session.status != CodeExecutionStatus::Approved
            && session.status != CodeExecutionStatus::CheckpointCreated
        {
            anyhow::bail!("Session is not approved or checkpointed");
        }

        // Apply steps
        for step in &session.execution_steps {
            if let Some(content) = &step.new_content {
                let target_path = working_dir.join(&step.target_file);

                // Ensure path containment
                let canonical_root = working_dir.canonicalize()?;
                let parent_dir = target_path.parent().unwrap_or(working_dir);
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir)?;
                }
                let canonical_parent = parent_dir.canonicalize()?;
                if !canonical_parent.starts_with(&canonical_root) {
                    anyhow::bail!("Path traversal attempt blocked: {}", step.target_file);
                }

                // Block risky paths
                let path_str = step.target_file.to_lowercase();
                if path_str.contains("package-lock.json")
                    || path_str.contains("cargo.lock")
                    || path_str.contains("poetry.lock")
                {
                    anyhow::bail!("Modification of lockfiles is blocked.");
                }

                if crate::repo_map::looks_like_secret_file(&target_path) {
                    anyhow::bail!("Refusing to patch sensitive file: {}", step.target_file);
                }

                if path_str.contains("node_modules/")
                    || path_str.contains("target/")
                    || path_str.contains("vendor/")
                    || path_str.contains(".git/")
                {
                    anyhow::bail!(
                        "Refusing to patch vendor/generated directory: {}",
                        step.target_file
                    );
                }

                fs::write(&target_path, content)?;
            }
        }

        session.status = CodeExecutionStatus::Completed;
        session.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.save_session(&session)?;

        Ok(())
    }

    pub fn rollback_session(
        &self,
        id: &str,
        working_dir: &Path,
        checkpoint_mgr: &crate::checkpoint::CheckpointManager,
    ) -> Result<RollbackResult> {
        let mut session = self.get_session(id)?.context("Session not found")?;

        let mut result = RollbackResult {
            success: false,
            conflicts: vec![],
            restored_files: vec![],
        };

        if let Some(cp_id) = &session.checkpoint_id {
            if let Some(cp) = checkpoint_mgr.get_checkpoint(cp_id)? {
                // For a real rollback without git checking out everything, we could just checkout the specific affected files
                // Currently checkpoint.rs captures git diff and status, but to rollback we need to restore.
                // Assuming we can use git checkout for the affected files from the checkpoint branch/commit:
                // Since checkpoint.rs relies on git, we execute git restore or git checkout.
                for file in &session.affected_files {
                    let mut cmd = Command::new("git");
                    cmd.current_dir(working_dir)
                        .args(["checkout", "HEAD", "--", file]);

                    let out = cmd.output()?;
                    if out.status.success() {
                        result.restored_files.push(file.clone());
                    } else {
                        result.conflicts.push(RollbackConflict {
                            file_path: file.clone(),
                            reason: String::from_utf8_lossy(&out.stderr).into_owned(),
                        });
                    }
                }

                if result.conflicts.is_empty() {
                    result.success = true;
                    session.status = CodeExecutionStatus::RolledBack;
                } else {
                    session.status = CodeExecutionStatus::Failed; // conflict
                }
                session.updated_at = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                self.save_session(&session)?;
                return Ok(result);
            }
        }

        anyhow::bail!("No valid checkpoint found for rollback");
    }
}
