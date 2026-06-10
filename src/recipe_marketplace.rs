use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::paths::GoatPaths;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecipeSource {
    Local,
    Learned,
    StudioDraft,
    Imported,
    BuiltIn,
    RemoteMarketplace,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub source: RecipeSource,
    pub risk_level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub source: RecipeSource,
    pub content: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAuditReport {
    pub risk_level: String,
    pub warnings: Vec<String>,
    pub recommended_action: String,
    pub required_approvals: Vec<String>,
    pub suggested_sandbox_policy: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeInstallCandidate {
    pub details: RecipeDetails,
    pub audit: RecipeAuditReport,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeInstallDecision {
    pub approved: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstalledRecipeMeta {
    pub name: String,
    pub source: RecipeSource,
    pub version: String,
    pub installed_at: String,
    pub enabled: bool,
    pub audit_result: RecipeAuditReport,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentTemplate {
    pub id: String,
    pub name: String,
    pub role: String,
    pub purpose: String,
    pub model_recommendation: Option<String>,
    pub tools_requested: Vec<String>,
    pub risk_level: String,
    pub instructions: String,
    pub examples: Vec<String>,
    pub when_to_use: String,
}

pub struct RecipeMarketplaceManager {
    paths: GoatPaths,
}

impl RecipeMarketplaceManager {
    pub fn new(paths: GoatPaths) -> Self {
        let _ = fs::create_dir_all(paths.data_dir.join("recipes"));
        Self { paths }
    }

    pub fn built_in_recipes(&self) -> Vec<RecipeSummary> {
        vec![
            RecipeSummary {
                id: "builtin_1".to_string(),
                name: "cargo-check-on-save".to_string(),
                description: "Run cargo check after Rust file edits".to_string(),
                author: "goat-core".to_string(),
                source: RecipeSource::BuiltIn,
                risk_level: "low".to_string(),
            },
            RecipeSummary {
                id: "builtin_2".to_string(),
                name: "checkpoint-before-write".to_string(),
                description: "Create checkpoint before risky write".to_string(),
                author: "goat-core".to_string(),
                source: RecipeSource::BuiltIn,
                risk_level: "low".to_string(),
            },
            RecipeSummary {
                id: "builtin_3".to_string(),
                name: "summarize-jobs-daily".to_string(),
                description: "Summarize recent jobs daily".to_string(),
                author: "goat-core".to_string(),
                source: RecipeSource::BuiltIn,
                risk_level: "low".to_string(),
            },
            RecipeSummary {
                id: "builtin_4".to_string(),
                name: "npm-build-dashboard".to_string(),
                description: "Run npm build after dashboard edits".to_string(),
                author: "goat-core".to_string(),
                source: RecipeSource::BuiltIn,
                risk_level: "low".to_string(),
            },
        ]
    }

    pub fn audit(&self, content: &str) -> RecipeAuditReport {
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
        if lower.contains("git push --force") {
            risk_level = "high".to_string();
            warnings.push("Contains destructive git push".to_string());
        }
        
        let action = if risk_level == "critical" || risk_level == "high" {
            "review_required".to_string()
        } else {
            "safe_to_install".to_string()
        };

        RecipeAuditReport {
            risk_level,
            warnings,
            recommended_action: action,
            required_approvals: vec!["admin".to_string()],
            suggested_sandbox_policy: "strict".to_string(),
        }
    }

    pub fn install(&self, candidate: &RecipeInstallCandidate) -> Result<()> {
        let safe_name = candidate.details.name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let recipe_dir = self.paths.data_dir.join("recipes").join("installed").join(&safe_name);
        fs::create_dir_all(&recipe_dir)?;
        
        let md_path = recipe_dir.join("recipe.toml");
        fs::write(&md_path, &candidate.details.content)?;
        
        let meta_path = recipe_dir.join("recipe.meta.json");
        let meta = InstalledRecipeMeta {
            name: safe_name,
            source: candidate.details.source.clone(),
            version: candidate.details.version.clone(),
            installed_at: chrono::Utc::now().to_rfc3339(),
            enabled: false, // ALWAYS false by default
            audit_result: candidate.audit.clone(),
        };
        fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
        
        Ok(())
    }

    pub fn enable(&self, name: &str) -> Result<()> {
        let safe_name = name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let meta_path = self.paths.data_dir.join("recipes").join("installed").join(&safe_name).join("recipe.meta.json");
        if !meta_path.exists() {
            return Err(anyhow::anyhow!("Recipe not found"));
        }
        let content = fs::read_to_string(&meta_path)?;
        let mut meta: InstalledRecipeMeta = serde_json::from_str(&content)?;
        meta.enabled = true;
        fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
        Ok(())
    }

    pub fn disable(&self, name: &str) -> Result<()> {
        let safe_name = name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let meta_path = self.paths.data_dir.join("recipes").join("installed").join(&safe_name).join("recipe.meta.json");
        if !meta_path.exists() {
            return Err(anyhow::anyhow!("Recipe not found"));
        }
        let content = fs::read_to_string(&meta_path)?;
        let mut meta: InstalledRecipeMeta = serde_json::from_str(&content)?;
        meta.enabled = false;
        fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
        Ok(())
    }

    pub fn uninstall(&self, name: &str) -> Result<()> {
        let safe_name = name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let recipe_dir = self.paths.data_dir.join("recipes").join("installed").join(&safe_name);
        if recipe_dir.exists() {
            fs::remove_dir_all(recipe_dir)?;
        }
        Ok(())
    }
}
