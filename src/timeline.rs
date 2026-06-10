use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineEventKind {
    SessionStarted,
    SessionCompleted,
    ChatMessage,
    PlanCreated,
    ActStarted,
    JobStarted,
    JobCompleted,
    JobFailed,
    ApprovalRequested,
    ApprovalApproved,
    ApprovalDenied,
    ToolUsed,
    McpToolUsed,
    ExternalAgentRun,
    CheckpointCreated,
    RollbackPlanned,
    RollbackExecuted,
    CommitCreated,
    BranchCreated,
    PatchCreated,
    PatchApplied,
    MemoryCandidateCreated,
    MemoryCandidateAccepted,
    SkillCreated,
    SkillAttached,
    SkillPackUsed,
    RecipeInstalled,
    RecipeActivated,
    RecipeRunStarted,
    RecipeRunCompleted,
    StudioDraftCreated,
    BrainIndexCompleted,
    EmbeddingRebuildCompleted,
    DaemonStarted,
    DesktopStarted,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineSource {
    System,
    User,
    Agent(String),
    Daemon,
    Dashboard,
    Desktop,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TimelineRiskLevel {
    #[default]
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TimelinePrivacyLevel {
    #[default]
    Standard,
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    pub timestamp: i64,
    pub project_path: Option<String>,
    pub session_id: Option<String>,
    pub source: TimelineSource,
    pub kind: TimelineEventKind,
    pub title: String,
    pub summary: String,
    pub actor: String,
    pub related_ids: Vec<String>,
    pub file_refs: Vec<String>,
    pub git_refs: Vec<String>,
    pub checkpoint_refs: Vec<String>,
    pub job_refs: Vec<String>,
    pub approval_refs: Vec<String>,
    pub skill_refs: Vec<String>,
    pub recipe_refs: Vec<String>,
    pub memory_refs: Vec<String>,
    pub risk_level: TimelineRiskLevel,
    pub privacy_level: TimelinePrivacyLevel,
    pub redaction_status: String,
}

pub struct TimelineManager {
    timeline_dir: PathBuf,
    jsonl_path: PathBuf,
}

impl TimelineManager {
    pub fn new(data_dir: &Path) -> Self {
        let timeline_dir = data_dir.join("timeline");
        let _ = std::fs::create_dir_all(&timeline_dir);
        let jsonl_path = timeline_dir.join("timeline.jsonl");

        Self {
            timeline_dir,
            jsonl_path,
        }
    }

    pub fn record_event(&self, event: TimelineEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.jsonl_path)?;

        let line = serde_json::to_string(&event)?;
        writeln!(file, "{}", line)?;
        Ok(())
    }

    pub fn load_events(&self) -> Result<Vec<TimelineEvent>> {
        let mut events = Vec::new();
        if !self.jsonl_path.exists() {
            return Ok(events);
        }

        let content = std::fs::read_to_string(&self.jsonl_path)?;
        for line in content.lines() {
            if let Ok(event) = serde_json::from_str::<TimelineEvent>(line) {
                events.push(event);
            }
        }
        Ok(events)
    }

    pub fn replay(&self, query: &str) -> Result<Vec<TimelineEvent>> {
        // Simple mock replay query handling for now
        let events = self.load_events()?;
        let query_lower = query.to_lowercase();
        
        let filtered: Vec<TimelineEvent> = events
            .into_iter()
            .filter(|e| e.summary.to_lowercase().contains(&query_lower) || e.title.to_lowercase().contains(&query_lower))
            .collect();
            
        Ok(filtered)
    }
}
