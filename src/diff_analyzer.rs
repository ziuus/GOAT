use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffAnalysisSource {
    PatchProposal,
    AppliedPatch,
    GitDiff,
    ExternalAgentOutput,
    ManualDiffFile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DiffRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffRecommendation {
    Apply,
    Revise,
    ValidateFirst,
    Reject,
    ManualReviewRequired,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffAnalysisStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffImpactArea {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffFinding {
    pub severity: FindingSeverity,
    pub file_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffAnalysis {
    pub analysis_id: String,
    pub source_type: DiffAnalysisSource,
    pub patch_id: Option<String>,
    pub mission_id: Option<String>,
    pub project_id: Option<String>,
    pub external_agent_run_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub files_changed: Vec<String>,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub affected_languages: Vec<String>,
    pub affected_frameworks: Vec<String>,
    pub impacted_project_areas: Vec<DiffImpactArea>,
    pub risk_level: DiffRiskLevel,
    pub findings: Vec<DiffFinding>,
    pub recommended_validation_commands: Vec<String>,
    pub recommendation: DiffRecommendation,
    pub created_at: DateTime<Utc>,
    pub status: DiffAnalysisStatus,
}

pub struct DiffAnalyzer {
    storage_dir: PathBuf,
}

impl DiffAnalyzer {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let storage_dir = home.join(".local/share/goat/diff-analyses");
        let _ = fs::create_dir_all(&storage_dir);
        Self { storage_dir }
    }

    pub fn save_analysis(&self, analysis: &DiffAnalysis) -> Result<()> {
        let path = self
            .storage_dir
            .join(format!("{}.json", analysis.analysis_id));
        let json = serde_json::to_string_pretty(analysis)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn get_analysis(&self, id: &str) -> Result<DiffAnalysis> {
        let path = self.storage_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Err(anyhow!("Analysis not found: {}", id));
        }
        let json = fs::read_to_string(path)?;
        let analysis = serde_json::from_str(&json)?;
        Ok(analysis)
    }

    pub fn list_analyses(&self) -> Result<Vec<DiffAnalysis>> {
        let mut analyses = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(analysis) = serde_json::from_str::<DiffAnalysis>(&content) {
                            analyses.push(analysis);
                        }
                    }
                }
            }
        }
        analyses.sort_by_key(|a| std::cmp::Reverse(a.created_at));
        Ok(analyses)
    }

    pub fn analyze_patch(
        &self,
        patch: &crate::patch_manager::PatchProposal,
    ) -> Result<DiffAnalysis> {
        let analysis_id = uuid::Uuid::new_v4().to_string();

        let mut files_changed = Vec::new();
        let mut lines_added = 0;
        let mut lines_removed = 0;

        for edit in &patch.edits {
            files_changed.push(edit.path.clone());
            lines_added += edit.new_content.lines().count();
            lines_removed += edit.original_content.lines().count();
        }

        let (risk_level, findings, recommendation, commands) =
            self.run_deterministic_rules(&files_changed, DiffAnalysisSource::PatchProposal);

        let mut analysis = DiffAnalysis {
            analysis_id,
            source_type: DiffAnalysisSource::PatchProposal,
            patch_id: Some(patch.patch_id.clone()),
            mission_id: Some(patch.mission_id.clone()),
            project_id: Some(patch.project_id.clone()),
            external_agent_run_id: None,
            title: format!("Analysis for Patch {}", patch.patch_id),
            summary: patch.summary.clone(),
            files_changed,
            lines_added,
            lines_removed,
            affected_languages: vec![],
            affected_frameworks: vec![],
            impacted_project_areas: vec![],
            risk_level,
            findings,
            recommended_validation_commands: commands,
            recommendation,
            created_at: Utc::now(),
            status: DiffAnalysisStatus::Completed,
        };

        self.save_analysis(&analysis)?;
        Ok(analysis)
    }

    pub fn analyze_git_diff(
        &self,
        _project_root: &Path,
        diff_output: &str,
    ) -> Result<DiffAnalysis> {
        let analysis_id = uuid::Uuid::new_v4().to_string();

        let mut files_changed = Vec::new();
        for line in diff_output.lines() {
            if line.starts_with("+++ b/") {
                files_changed.push(line.trim_start_matches("+++ b/").to_string());
            }
        }

        let (risk_level, findings, recommendation, commands) =
            self.run_deterministic_rules(&files_changed, DiffAnalysisSource::GitDiff);

        let analysis = DiffAnalysis {
            analysis_id,
            source_type: DiffAnalysisSource::GitDiff,
            patch_id: None,
            mission_id: None,
            project_id: None,
            external_agent_run_id: None,
            title: "Git Diff Analysis".to_string(),
            summary: "Analyzed local git modifications".to_string(),
            files_changed,
            lines_added: 0,
            lines_removed: 0,
            affected_languages: vec![],
            affected_frameworks: vec![],
            impacted_project_areas: vec![],
            risk_level,
            findings,
            recommended_validation_commands: commands,
            recommendation,
            created_at: Utc::now(),
            status: DiffAnalysisStatus::Completed,
        };

        self.save_analysis(&analysis)?;
        Ok(analysis)
    }

    pub fn analyze_agent_run(
        &self,
        run: &crate::external_agents::ExternalAgentRun,
    ) -> Result<DiffAnalysis> {
        let analysis_id = uuid::Uuid::new_v4().to_string();

        let findings = vec![
            DiffFinding {
                severity: FindingSeverity::Warning,
                file_path: None,
                message: "External agent output detected. Output is treated as untrusted and requires manual review.".to_string(),
            }
        ];

        let analysis = DiffAnalysis {
            analysis_id,
            source_type: DiffAnalysisSource::ExternalAgentOutput,
            patch_id: None,
            mission_id: run.mission_id.clone(),
            project_id: run.project_id.clone(),
            external_agent_run_id: Some(run.run_id.clone()),
            title: format!("Agent Run Analysis ({})", run.run_id),
            summary: "External agent modifications require manual verification.".to_string(),
            files_changed: vec![],
            lines_added: 0,
            lines_removed: 0,
            affected_languages: vec![],
            affected_frameworks: vec![],
            impacted_project_areas: vec![],
            risk_level: DiffRiskLevel::High,
            findings,
            recommended_validation_commands: vec![],
            recommendation: DiffRecommendation::ManualReviewRequired,
            created_at: Utc::now(),
            status: DiffAnalysisStatus::Completed,
        };

        self.save_analysis(&analysis)?;
        Ok(analysis)
    }

    fn run_deterministic_rules(
        &self,
        files: &[String],
        source: DiffAnalysisSource,
    ) -> (
        DiffRiskLevel,
        Vec<DiffFinding>,
        DiffRecommendation,
        Vec<String>,
    ) {
        let mut risk = DiffRiskLevel::Low;
        let mut findings = Vec::new();
        let mut commands = Vec::new();
        let mut recommendation = DiffRecommendation::Apply;

        if source == DiffAnalysisSource::ExternalAgentOutput {
            return (
                DiffRiskLevel::High,
                vec![],
                DiffRecommendation::ManualReviewRequired,
                vec![],
            );
        }

        let mut requires_cargo = false;
        let mut requires_npm = false;

        for file in files {
            if file == ".env"
                || file.starts_with(".env.")
                || file.ends_with(".pem")
                || file.ends_with(".key")
            {
                risk = DiffRiskLevel::Critical;
                recommendation = DiffRecommendation::Reject;
                findings.push(DiffFinding {
                    severity: FindingSeverity::Critical,
                    file_path: Some(file.clone()),
                    message: "Sensitive file modified (credentials/secrets).".to_string(),
                });
            }

            if file.contains("..") || file.starts_with("/") {
                risk = DiffRiskLevel::Critical;
                recommendation = DiffRecommendation::Reject;
                findings.push(DiffFinding {
                    severity: FindingSeverity::Critical,
                    file_path: Some(file.clone()),
                    message: "Path traversal or out-of-root path detected.".to_string(),
                });
            }

            if file.ends_with(".sh")
                || file == "package-lock.json"
                || file == "yarn.lock"
                || file.starts_with(".github/workflows/")
            {
                if risk < DiffRiskLevel::High {
                    risk = DiffRiskLevel::High;
                }
                if recommendation != DiffRecommendation::Reject {
                    recommendation = DiffRecommendation::ManualReviewRequired;
                }
                findings.push(DiffFinding {
                    severity: FindingSeverity::Warning,
                    file_path: Some(file.clone()),
                    message: "High-risk file modified (scripts, CI, or lockfiles).".to_string(),
                });
            }

            if file.ends_with(".rs") || file == "Cargo.toml" {
                if risk < DiffRiskLevel::Medium {
                    risk = DiffRiskLevel::Medium;
                }
                requires_cargo = true;
            }
            if file.ends_with(".ts")
                || file.ends_with(".tsx")
                || file.ends_with(".js")
                || file == "package.json"
            {
                if risk < DiffRiskLevel::Medium {
                    risk = DiffRiskLevel::Medium;
                }
                requires_npm = true;
            }
        }

        if requires_cargo {
            commands.push("cargo check".to_string());
            commands.push("cargo test".to_string());
            if recommendation == DiffRecommendation::Apply {
                recommendation = DiffRecommendation::ValidateFirst;
            }
        }
        if requires_npm {
            commands.push("npm run build".to_string());
            commands.push("npm run lint".to_string());
            if recommendation == DiffRecommendation::Apply {
                recommendation = DiffRecommendation::ValidateFirst;
            }
        }

        if files.is_empty() {
            findings.push(DiffFinding {
                severity: FindingSeverity::Info,
                file_path: None,
                message: "No files changed.".to_string(),
            });
        }

        (risk, findings, recommendation, commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_sensitive_files() {
        let analyzer = DiffAnalyzer::new();
        let files = vec!["src/main.rs".to_string(), ".env".to_string()];
        let (risk, findings, recommendation, _) =
            analyzer.run_deterministic_rules(&files, DiffAnalysisSource::PatchProposal);
        assert_eq!(risk, DiffRiskLevel::Critical);
        assert_eq!(recommendation, DiffRecommendation::Reject);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("Sensitive file modified"))
        );
    }

    #[test]
    fn test_analyze_npm_files() {
        let analyzer = DiffAnalyzer::new();
        let files = vec!["package.json".to_string(), "src/app.tsx".to_string()];
        let (risk, _, recommendation, commands) =
            analyzer.run_deterministic_rules(&files, DiffAnalysisSource::PatchProposal);
        assert_eq!(risk, DiffRiskLevel::Medium);
        assert_eq!(recommendation, DiffRecommendation::ValidateFirst);
        assert!(commands.contains(&"npm run build".to_string()));
    }
}
