use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEdit {
    pub path: String,
    pub original_content: String,
    pub new_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProposal {
    pub patch_id: String,
    pub mission_id: String,
    pub project_id: String,
    pub title: String,
    pub summary: String,
    pub edits: Vec<PatchEdit>,
    pub diff_preview: String,
    pub risk_level: String,
    pub status: String,
    pub created_at: u64,
    pub applied_at: Option<u64>,
    pub checkpoint_id: Option<String>,
}

pub struct PatchManager {
    storage_dir: PathBuf,
}

impl PatchManager {
    pub fn new() -> Self {
        let storage_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("goat")
            .join("patches");
        let _ = fs::create_dir_all(&storage_dir);
        Self { storage_dir }
    }

    pub fn get_patches(&self) -> Vec<PatchProposal> {
        let mut patches = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(patch) = serde_json::from_str::<PatchProposal>(&content) {
                            patches.push(patch);
                        }
                    }
                }
            }
        }
        patches.sort_by_key(|p| std::cmp::Reverse(p.created_at));
        patches
    }

    pub fn get_patch(&self, id: &str) -> Option<PatchProposal> {
        let path = self.storage_dir.join(format!("{}.json", id));
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(patch) = serde_json::from_str::<PatchProposal>(&content) {
                    return Some(patch);
                }
            }
        }
        None
    }

    pub fn save_patch(&self, patch: &PatchProposal) -> Result<()> {
        let path = self.storage_dir.join(format!("{}.json", patch.patch_id));
        let content = serde_json::to_string_pretty(patch)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn generate_patch_proposal(
        &self,
        mission: &crate::mission_control::Mission,
        project: &crate::project_intelligence::ProjectIntelligence,
    ) -> Result<PatchProposal> {
        let mut target_file = "README.md".to_string();
        
        let goal_lower = mission.raw_goal.to_lowercase();
        if !goal_lower.contains("readme") {
            for f in &project.important_files {
                let f_lower = f.to_lowercase();
                let is_match = goal_lower.split_whitespace().any(|word| f_lower.contains(word) && word.len() > 3);
                if is_match {
                    target_file = f.clone();
                    break;
                }
            }
        }

        let file_path = project.root_path.join(&target_file);
        
        // Safety checks
        if crate::repo_map::looks_like_secret_file(&file_path) {
            anyhow::bail!("Refusing to patch sensitive file: {}", target_file);
        }
        
        let original_content = if file_path.exists() {
            fs::read_to_string(&file_path).unwrap_or_default()
        } else {
            String::new()
        };

        let new_content = if target_file.ends_with(".md") {
            format!("{}\n\n<!-- GOAT Checkpoint: {} -->\n", original_content, mission.raw_goal)
        } else if target_file.ends_with(".rs") {
            format!("// GOAT: {}\n{}", mission.raw_goal, original_content)
        } else if target_file.ends_with(".ts") || target_file.ends_with(".js") || target_file.ends_with(".tsx") {
            format!("// GOAT: {}\n{}", mission.raw_goal, original_content)
        } else {
            format!("{}\n# GOAT: {}\n", original_content, mission.raw_goal)
        };

        let edit = PatchEdit {
            path: target_file.clone(),
            original_content: original_content.clone(),
            new_content: new_content.clone(),
        };

        // Basic diff preview
        let diff_preview = format!("--- {}\n+++ {}\n@@ ... @@\n+ // GOAT Edit", target_file, target_file);

        use std::time::SystemTime;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let patch = PatchProposal {
            patch_id: uuid::Uuid::new_v4().to_string()[..8].to_string(),
            mission_id: mission.mission_id.clone(),
            project_id: project.project_id.clone(),
            title: format!("Update {}", target_file),
            summary: format!("Proposing changes to {} based on mission goal.", target_file),
            edits: vec![edit],
            diff_preview,
            risk_level: "low".to_string(),
            status: "proposed".to_string(),
            created_at: now,
            applied_at: None,
            checkpoint_id: None,
        };

        Ok(patch)
    }

    pub fn apply_patch(
        &self,
        patch: &mut PatchProposal,
        project_root: &std::path::Path,
    ) -> Result<()> {
        for edit in &patch.edits {
            let file_path = project_root.join(&edit.path);
            
            // Re-verify path containment
            let canonical_root = project_root.canonicalize()?;
            // Allow creating new files by checking parent
            let parent_dir = file_path.parent().unwrap_or(project_root);
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?;
            }
            let canonical_parent = parent_dir.canonicalize()?;
            if !canonical_parent.starts_with(&canonical_root) {
                anyhow::bail!("Path traversal attempt blocked: {}", edit.path);
            }
            if crate::repo_map::looks_like_secret_file(&file_path) {
                anyhow::bail!("Refusing to patch sensitive file: {}", edit.path);
            }

            fs::write(&file_path, &edit.new_content).context("Failed to write patched file")?;
        }

        use std::time::SystemTime;
        patch.status = "applied".to_string();
        patch.applied_at = Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
        self.save_patch(patch)?;

        Ok(())
    }
}
