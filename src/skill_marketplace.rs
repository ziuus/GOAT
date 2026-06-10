use crate::config::SkillMarketplaceConfig;
use crate::paths::GoatPaths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteSkillSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub downloads: u64,
    pub rating: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteSkillDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub content: String,
    pub url: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillAuditReport {
    pub risk_level: String,
    pub warnings: Vec<String>,
    pub recommended_action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillInstallCandidate {
    pub details: RemoteSkillDetails,
    pub audit: SkillAuditReport,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillInstallDecision {
    pub approved: bool,
    pub reason: Option<String>,
}

pub struct SkillMarketplaceManager {
    paths: GoatPaths,
    config: SkillMarketplaceConfig,
}

impl SkillMarketplaceManager {
    pub fn new(paths: GoatPaths, config: SkillMarketplaceConfig) -> Self {
        let cache_dir = paths.data_dir.join("skill-marketplace-cache");
        let _ = fs::create_dir_all(&cache_dir);
        Self { paths, config }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<RemoteSkillSummary>> {
        if !self.config.enabled {
            return Err(anyhow::anyhow!("Skill marketplace is disabled. Check config or auth."));
        }
        // TODO: Implement actual HTTP client
        Ok(vec![])
    }

    pub async fn get_details(&self, id: &str) -> Result<RemoteSkillDetails> {
        if !self.config.enabled {
            return Err(anyhow::anyhow!("Skill marketplace is disabled."));
        }
        // TODO: Implement actual HTTP client
        Err(anyhow::anyhow!("Skill not found"))
    }

    pub fn audit(&self, content: &str) -> SkillAuditReport {
        let mut warnings = Vec::new();
        let mut risk_level = "low".to_string();
        let lower = content.to_lowercase();
        
        if lower.contains("rm -rf") || lower.contains("sudo") {
            risk_level = "critical".to_string();
            warnings.push("Contains destructive or privileged commands".to_string());
        }
        if lower.contains("curl | sh") {
            risk_level = "high".to_string();
            warnings.push("Contains arbitrary remote script execution".to_string());
        }
        
        let action = if risk_level == "critical" || risk_level == "high" {
            "review_required".to_string()
        } else {
            "safe_to_install".to_string()
        };

        SkillAuditReport {
            risk_level,
            warnings,
            recommended_action: action,
        }
    }

    pub async fn install(&self, candidate: &SkillInstallCandidate) -> Result<()> {
        // Assume ApprovalGate has passed before calling this.
        let safe_name = candidate.details.name.replace(|c: char| !c.is_alphanumeric(), "_");
        let skill_dir = self.paths.skills_dir.join(&safe_name);
        fs::create_dir_all(&skill_dir)?;
        
        let md_path = skill_dir.join("SKILL.md");
        fs::write(&md_path, &candidate.details.content)?;
        
        // Write metadata
        let meta_path = skill_dir.join("skill.meta.json");
        let meta = serde_json::json!({
            "source": "skills.sh",
            "remote_id": candidate.details.id,
            "version": candidate.details.version,
            "installed_at": chrono::Utc::now().to_rfc3339(),
            "audit_result": candidate.audit,
        });
        fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
        
        Ok(())
    }
}
