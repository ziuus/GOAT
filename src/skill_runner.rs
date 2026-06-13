use crate::skills::Skill;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SkillExecutionStatus {
    Planned,
    WaitingForApproval,
    Running,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SkillStepType {
    InspectFile,
    ProposePatch,
    ApplyPatch,
    RunValidation,
    RecordNote,
    AskUser,
    ManualStep,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillStep {
    pub step_type: SkillStepType,
    pub description: String,
    pub command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillExecution {
    pub execution_id: String,
    pub skill_id: String,
    pub skill_name: String,
    pub mission_id: Option<String>,
    pub project_id: Option<String>,
    pub status: SkillExecutionStatus,
    pub current_step: usize,
    pub total_steps: usize,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub events: Vec<String>,
    pub approvals_requested: usize,
    pub actions_taken: usize,
    pub errors: Vec<String>,
    pub summary: Option<String>,
}

pub struct SkillRunner {
    storage_dir: PathBuf,
}

impl SkillRunner {
    pub fn new(base_dir: &Path) -> Self {
        let storage_dir = base_dir.join("skill-runs");
        let _ = fs::create_dir_all(&storage_dir);
        Self { storage_dir }
    }

    pub fn parse_steps(skill_content: &str) -> Vec<SkillStep> {
        let mut steps = Vec::new();
        let mut current_step_type = SkillStepType::ManualStep;
        let mut current_desc = String::new();
        let mut current_cmd = None;
        let mut in_code_block = false;

        for line in skill_content.lines() {
            let line = line.trim();
            if line.starts_with("```bash") || line.starts_with("```sh") {
                in_code_block = true;
                current_cmd = Some(String::new());
            } else if line.starts_with("```") && in_code_block {
                in_code_block = false;
                if let Some(cmd) = &mut current_cmd {
                    *cmd = cmd.trim().to_string();
                }
            } else if in_code_block {
                if let Some(cmd) = &mut current_cmd {
                    cmd.push_str(line);
                    cmd.push('\n');
                }
            } else if line.starts_with("- ") || line.starts_with("* ") {
                if !current_desc.is_empty() {
                    steps.push(SkillStep {
                        step_type: current_step_type.clone(),
                        description: current_desc.clone(),
                        command: current_cmd.take(),
                    });
                }
                let text = line[2..].to_string();
                current_step_type = if text.to_lowercase().contains("inspect")
                    || text.to_lowercase().contains("read")
                {
                    SkillStepType::InspectFile
                } else if text.to_lowercase().contains("patch") {
                    SkillStepType::ProposePatch
                } else if text.to_lowercase().contains("validate")
                    || text.to_lowercase().contains("test")
                {
                    SkillStepType::RunValidation
                } else if text.to_lowercase().contains("ask") {
                    SkillStepType::AskUser
                } else if text.to_lowercase().contains("note") {
                    SkillStepType::RecordNote
                } else {
                    SkillStepType::ManualStep
                };
                current_desc = text;
            }
        }

        if !current_desc.is_empty() {
            steps.push(SkillStep {
                step_type: current_step_type,
                description: current_desc,
                command: current_cmd,
            });
        }

        if steps.is_empty() {
            steps.push(SkillStep {
                step_type: SkillStepType::ManualStep,
                description: "Execute the skill as described in the documentation.".to_string(),
                command: None,
            });
        }

        steps
    }

    pub fn start_execution(
        &self,
        skill: &Skill,
        mission_id: Option<String>,
        project_id: Option<String>,
    ) -> Result<SkillExecution> {
        let steps = Self::parse_steps(&skill.content);
        let execution = SkillExecution {
            execution_id: uuid::Uuid::new_v4().to_string(),
            skill_id: skill.name.clone(),
            skill_name: skill.name.clone(),
            mission_id,
            project_id,
            status: SkillExecutionStatus::Planned,
            current_step: 0,
            total_steps: steps.len(),
            started_at: chrono::Utc::now().timestamp(),
            finished_at: None,
            events: vec![format!("Execution started for skill '{}'", skill.name)],
            approvals_requested: 0,
            actions_taken: 0,
            errors: vec![],
            summary: None,
        };
        self.save_execution(&execution)?;
        Ok(execution)
    }

    pub fn get_execution(&self, id: &str) -> Result<Option<SkillExecution>> {
        let path = self.storage_dir.join(format!("{}.json", id));
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let exec = serde_json::from_str(&content)?;
            Ok(Some(exec))
        } else {
            Ok(None)
        }
    }

    pub fn list_executions(&self) -> Vec<SkillExecution> {
        let mut execs = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(exec) = serde_json::from_str::<SkillExecution>(&content) {
                            execs.push(exec);
                        }
                    }
                }
            }
        }
        execs.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        execs
    }

    pub fn save_execution(&self, exec: &SkillExecution) -> Result<()> {
        let path = self.storage_dir.join(format!("{}.json", exec.execution_id));
        fs::write(path, serde_json::to_string_pretty(exec)?)?;
        Ok(())
    }
}
