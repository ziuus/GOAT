use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Checkpoint {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub label: String,
    pub branch: String,
    pub is_dirty: bool,
    pub changed_files: Vec<String>,
    pub diff_snapshot: String,
}

pub struct CheckpointManager {
    pub checkpoints_dir: PathBuf,
}

impl CheckpointManager {
    pub fn new(data_dir: &Path) -> Self {
        let checkpoints_dir = data_dir.join("checkpoints");
        let _ = fs::create_dir_all(&checkpoints_dir);
        Self { checkpoints_dir }
    }

    pub fn list_checkpoints(&self) -> Result<Vec<Checkpoint>> {
        let mut checkpoints = Vec::new();
        if !self.checkpoints_dir.exists() {
            return Ok(checkpoints);
        }
        for entry in fs::read_dir(&self.checkpoints_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cp) = serde_json::from_str::<Checkpoint>(&content) {
                        checkpoints.push(cp);
                    }
                }
            }
        }
        checkpoints.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(checkpoints)
    }

    pub fn get_checkpoint(&self, id: &str) -> Result<Option<Checkpoint>> {
        let path = self.checkpoints_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let cp = serde_json::from_str(&content)?;
        Ok(Some(cp))
    }

    pub fn create_checkpoint(&self, root: &Path, label: &str) -> Result<Checkpoint> {
        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();

        let branch = Command::new("git")
            .args([
                "-C",
                &root.to_string_lossy(),
                "rev-parse",
                "--abbrev-ref",
                "HEAD",
            ])
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string());

        let status_out = Command::new("git")
            .args(["-C", &root.to_string_lossy(), "status", "--porcelain"])
            .output()
            .ok();

        let mut is_dirty = false;
        let mut changed_files = Vec::new();
        if let Some(out) = status_out {
            let status = String::from_utf8_lossy(&out.stdout);
            for line in status.lines() {
                if !line.trim().is_empty() {
                    is_dirty = true;
                    changed_files.push(line.trim().to_string());
                }
            }
        }

        let diff_out = Command::new("git")
            .args(["-C", &root.to_string_lossy(), "diff", "HEAD"])
            .output()
            .ok();

        let diff_snapshot = if let Some(out) = diff_out {
            String::from_utf8_lossy(&out.stdout).to_string()
        } else {
            String::new()
        };

        let cp = Checkpoint {
            id: id.clone(),
            timestamp: Utc::now(),
            label: label.to_string(),
            branch,
            is_dirty,
            changed_files,
            diff_snapshot,
        };

        let path = self.checkpoints_dir.join(format!("{}.json", id));
        let json = serde_json::to_string_pretty(&cp)?;
        fs::write(path, json)?;

        Ok(cp)
    }
}
