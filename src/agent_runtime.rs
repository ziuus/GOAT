use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub type AgentJobId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeConfig {
    pub enabled: bool,
    pub max_active_jobs: usize,
    pub max_steps_per_job: usize,
    pub require_approval_for_medium_risk: bool,
    pub require_approval_for_high_risk: bool,
    pub persist_events: bool,
    pub live_updates: bool,
}

impl Default for AgentRuntimeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_active_jobs: 3,
            max_steps_per_job: 12,
            require_approval_for_medium_risk: true,
            require_approval_for_high_risk: true,
            persist_events: true,
            live_updates: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentJobStatus {
    Queued,
    WaitingForApproval,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentJobStepStatus {
    Pending,
    WaitingForApproval,
    Running,
    Completed,
    Skipped,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentJobKind {
    CofounderValidation,
    LearnerRoadmap,
    PromptforgeRefinement,
    ResearcherBrief,
    DesignerReview,
    OperatorHealthCheck,
    BuilderPlan,
    ReportGeneration,
    GenericAgentTask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentJobRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobPolicy {
    pub allow_file_writes: bool,
    pub allow_shell_commands: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobInput {
    pub task: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobOutput {
    pub result_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobArtifact {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobStep {
    pub id: String,
    pub name: String,
    pub status: AgentJobStepStatus,
    pub risk_level: AgentJobRiskLevel,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobResumeState {
    pub next_step_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobCheckpoint {
    pub id: String,
    pub job_id: String,
    pub step_index: usize,
    pub resume_state: AgentJobResumeState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJob {
    pub id: AgentJobId,
    pub title: String,
    pub agent_id: String,
    pub job_kind: AgentJobKind,
    pub status: AgentJobStatus,
    pub input_summary: String,
    pub risk_level: AgentJobRiskLevel,
    pub steps: Vec<AgentJobStep>,
    pub artifacts: Vec<AgentJobArtifact>,
    pub timeline_refs: Vec<String>,
    pub report_refs: Vec<String>,
    pub brain_refs: Vec<String>,
    pub approval_refs: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentJobEventKind {
    JobCreated,
    JobQueued,
    JobStarted,
    JobStepStarted,
    JobWaitingForApproval,
    JobStepCompleted,
    JobArtifactCreated,
    JobPaused,
    JobResumed,
    JobCompleted,
    JobFailed,
    JobCancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJobEvent {
    pub id: String,
    pub job_id: AgentJobId,
    pub kind: AgentJobEventKind,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

pub struct AgentRuntime {
    pub config: AgentRuntimeConfig,
    pub runtime_dir: PathBuf,
    pub jobs: HashMap<AgentJobId, AgentJob>,
}

impl AgentRuntime {
    pub fn new(config: AgentRuntimeConfig, runtime_dir: PathBuf) -> Result<Self> {
        if !runtime_dir.exists() {
            fs::create_dir_all(&runtime_dir)?;
        }

        let mut runtime = Self {
            config,
            runtime_dir,
            jobs: HashMap::new(),
        };

        let jobs_path = runtime.jobs_file();
        if jobs_path.exists() {
            let content = fs::read_to_string(&jobs_path)?;
            for line in content.lines() {
                if let Ok(job) = serde_json::from_str::<AgentJob>(line) {
                    runtime.jobs.insert(job.id.clone(), job);
                }
            }
        }

        Ok(runtime)
    }

    fn jobs_file(&self) -> PathBuf {
        self.runtime_dir.join("jobs.jsonl")
    }

    fn events_file(&self) -> PathBuf {
        self.runtime_dir.join("events.jsonl")
    }

    fn artifacts_file(&self) -> PathBuf {
        self.runtime_dir.join("artifacts.jsonl")
    }

    fn checkpoints_file(&self) -> PathBuf {
        self.runtime_dir.join("checkpoints.jsonl")
    }

    fn append_jsonl<T: Serialize>(path: &PathBuf, item: &T) -> Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        let json = serde_json::to_string(item)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    pub fn save_jobs(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.jobs_file())?;
        for job in self.jobs.values() {
            let json = serde_json::to_string(job)?;
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }

    pub fn push_event(&self, job_id: &str, kind: AgentJobEventKind, message: &str) -> Result<()> {
        let event = AgentJobEvent {
            id: Uuid::new_v4().to_string(),
            job_id: job_id.to_string(),
            kind,
            message: message.to_string(),
            created_at: Utc::now(),
        };
        Self::append_jsonl(&self.events_file(), &event)?;
        // TODO: propagate to system event bus for SSE
        Ok(())
    }

    pub fn create_job(
        &mut self,
        title: String,
        agent_id: String,
        job_kind: AgentJobKind,
        task: String,
    ) -> Result<AgentJobId> {
        let job_id = Uuid::new_v4().to_string();
        let mut steps = Vec::new();

        // Setup deterministic steps based on job kind
        match job_kind {
            AgentJobKind::PromptforgeRefinement => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Collect Input".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Refine".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Score".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Store Artifact".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
            AgentJobKind::LearnerRoadmap => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Create Goal".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Generate Roadmap".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Generate Today Plan".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Store Artifact".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
            AgentJobKind::CofounderValidation => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Create Idea".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Generate Checklist".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Generate MVP Scope".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Store Artifact".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
            AgentJobKind::ResearcherBrief => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Create Topic".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Create Research Plan".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Generate Brief Skeleton".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
            AgentJobKind::OperatorHealthCheck => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Create Checklist".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Store Artifact".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
            _ => {
                steps.push(AgentJobStep {
                    id: Uuid::new_v4().to_string(),
                    name: "Execute Task".into(),
                    status: AgentJobStepStatus::Pending,
                    risk_level: AgentJobRiskLevel::Low,
                    started_at: None,
                    completed_at: None,
                    error: None,
                });
            }
        }

        let job = AgentJob {
            id: job_id.clone(),
            title,
            agent_id,
            job_kind,
            status: AgentJobStatus::Queued,
            input_summary: task,
            risk_level: AgentJobRiskLevel::Low,
            steps,
            artifacts: Vec::new(),
            timeline_refs: Vec::new(),
            report_refs: Vec::new(),
            brain_refs: Vec::new(),
            approval_refs: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error: None,
        };

        self.jobs.insert(job_id.clone(), job);
        self.save_jobs()?;
        self.push_event(
            &job_id,
            AgentJobEventKind::JobCreated,
            "Job created successfully",
        )?;

        Ok(job_id)
    }

    pub fn get_job(&self, id: &str) -> Option<AgentJob> {
        self.jobs.get(id).cloned()
    }

    pub fn list_jobs(&self) -> Vec<AgentJob> {
        let mut jobs: Vec<AgentJob> = self.jobs.values().cloned().collect();
        jobs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        jobs
    }

    pub fn start_job(&mut self, id: &str) -> Result<()> {
        if let Some(job) = self.jobs.get_mut(id) {
            job.status = AgentJobStatus::Running;
            job.started_at = Some(Utc::now());
            job.updated_at = Utc::now();
            self.push_event(id, AgentJobEventKind::JobStarted, "Job started")?;
        }
        self.save_jobs()?;
        Ok(())
    }

    pub fn pause_job(&mut self, id: &str) -> Result<()> {
        if let Some(job) = self.jobs.get_mut(id) {
            job.status = AgentJobStatus::Paused;
            job.updated_at = Utc::now();
            self.push_event(id, AgentJobEventKind::JobPaused, "Job paused")?;
        }
        self.save_jobs()?;
        Ok(())
    }

    pub fn resume_job(&mut self, id: &str) -> Result<()> {
        if let Some(job) = self.jobs.get_mut(id) {
            job.status = AgentJobStatus::Running;
            job.updated_at = Utc::now();
            self.push_event(id, AgentJobEventKind::JobResumed, "Job resumed")?;
        }
        self.save_jobs()?;
        Ok(())
    }

    pub fn cancel_job(&mut self, id: &str) -> Result<()> {
        if let Some(job) = self.jobs.get_mut(id) {
            job.status = AgentJobStatus::Cancelled;
            job.updated_at = Utc::now();
            self.push_event(id, AgentJobEventKind::JobCancelled, "Job cancelled")?;
        }
        self.save_jobs()?;
        Ok(())
    }

    pub fn retry_job(&mut self, id: &str) -> Result<()> {
        if let Some(job) = self.jobs.get_mut(id) {
            job.status = AgentJobStatus::Queued;
            job.error = None;
            for step in &mut job.steps {
                if step.status == AgentJobStepStatus::Failed
                    || step.status == AgentJobStepStatus::Cancelled
                {
                    step.status = AgentJobStepStatus::Pending;
                    step.error = None;
                }
            }
            job.updated_at = Utc::now();
            self.push_event(id, AgentJobEventKind::JobQueued, "Job retried")?;
        }
        self.save_jobs()?;
        Ok(())
    }
}
