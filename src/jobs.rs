use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundJob {
    pub id: String,
    pub r#type: String, // "scheduled", "mcp", "hook", "tool"
    pub status: String, // "queued", "running", "completed", "failed", "cancelled", "approval_required"
    pub started_at: String,
    pub finished_at: Option<String>,
    pub output_preview: Option<String>,
    pub error: Option<String>,
    pub approval_status: Option<String>,
}

pub struct JobTracker {
    jobs: HashMap<String, BackgroundJob>,
}

impl JobTracker {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn add_job(&mut self, job: BackgroundJob) {
        self.jobs.insert(job.id.clone(), job);
    }

    pub fn get_job(&self, id: &str) -> Option<&BackgroundJob> {
        self.jobs.get(id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut BackgroundJob> {
        self.jobs.get_mut(id)
    }

    pub fn list_jobs(&self) -> Vec<&BackgroundJob> {
        self.jobs.values().collect()
    }
}
