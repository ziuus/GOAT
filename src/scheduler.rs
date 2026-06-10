use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::SchedulerConfig;
use crate::paths::GoatPaths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub prompt_or_command: String,
    pub schedule_type: String, // "manual", "once", "interval_minutes"
    pub interval_minutes: Option<u64>,
    pub enabled: bool,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub status: String,
    pub risk: String,
    pub created_at: String,
}

pub struct SchedulerManager {
    config: SchedulerConfig,
    paths: GoatPaths,
    jobs: HashMap<String, ScheduledJob>,
}

impl SchedulerManager {
    pub fn new(config: SchedulerConfig, paths: GoatPaths) -> Self {
        let mut sm = Self {
            config,
            paths,
            jobs: HashMap::new(),
        };
        sm.load_jobs();
        sm
    }

    fn jobs_file(&self) -> PathBuf {
        self.paths.data_dir.join("scheduled_jobs.json")
    }

    pub fn load_jobs(&mut self) {
        if let Ok(content) = fs::read_to_string(self.jobs_file()) {
            if let Ok(jobs) = serde_json::from_str::<Vec<ScheduledJob>>(&content) {
                self.jobs.clear();
                for job in jobs {
                    self.jobs.insert(job.id.clone(), job);
                }
            }
        }
    }

    pub fn save_jobs(&self) {
        let jobs_vec: Vec<&ScheduledJob> = self.jobs.values().collect();
        if let Ok(content) = serde_json::to_string_pretty(&jobs_vec) {
            let _ = fs::write(self.jobs_file(), content);
        }
    }

    pub fn add_job(&mut self, job: ScheduledJob) {
        self.jobs.insert(job.id.clone(), job);
        self.save_jobs();
    }

    pub fn delete_job(&mut self, id: &str) -> bool {
        let removed = self.jobs.remove(id).is_some();
        if removed {
            self.save_jobs();
        }
        removed
    }

    pub fn get_job(&self, id: &str) -> Option<&ScheduledJob> {
        self.jobs.get(id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut ScheduledJob> {
        self.jobs.get_mut(id)
    }

    pub fn list_jobs(&self) -> Vec<&ScheduledJob> {
        self.jobs.values().collect()
    }

    pub fn log_audit(&self, msg: &str) {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.paths.data_dir.join("scheduler-audit.log"))
        {
            use std::io::Write;
            let now = chrono::Utc::now().to_rfc3339();
            let _ = writeln!(file, "[{}] {}", now, msg);
        }
    }
}

impl SchedulerManager {
    pub fn tick(&mut self) -> Vec<ScheduledJob> {
        // Find jobs that need to run
        let mut to_run = Vec::new();
        let now = chrono::Utc::now();

        for job in self.jobs.values_mut() {
            if !job.enabled { continue; }
            
            if let Some(ref next) = job.next_run {
                if let Ok(next_time) = chrono::DateTime::parse_from_rfc3339(next) {
                    if now >= next_time.with_timezone(&chrono::Utc) {
                        to_run.push(job.clone());
                        
                        // Update next_run if interval
                        if let Some(interval) = job.interval_minutes {
                            job.last_run = Some(now.to_rfc3339());
                            let next = now + chrono::Duration::minutes(interval as i64);
                            job.next_run = Some(next.to_rfc3339());
                        } else {
                            // One-off job, disable it
                            job.enabled = false;
                        }
                    }
                }
            }
        }
        
        if !to_run.is_empty() {
            self.save_jobs();
        }
        
        to_run
    }
}
