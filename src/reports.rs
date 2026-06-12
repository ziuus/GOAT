use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportKind {
    Research,
    CodeReview,
    Finance,
    General,
    FounderValidationReport,
    FounderMarketSignalReport,
    ValidationExperimentReport,
    MvpScopeReport,
    PricingHypothesisReport,
    FounderDecisionReport,
    BuilderHandoffReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub heading: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub kind: ReportKind,
    pub title: String,
    pub sections: Vec<ReportSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportOutput {
    pub id: String,
    pub kind: ReportKind,
    pub title: String,
    pub markdown: String,
    pub created_at: String,
}

pub struct ReportManager {
    reports_dir: PathBuf,
}

impl Default for ReportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportManager {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        let reports_dir = data_dir.join("goat").join("reports");
        let _ = fs::create_dir_all(&reports_dir);
        Self { reports_dir }
    }

    pub fn generate_report(&self, template: ReportTemplate) -> Result<ReportOutput, String> {
        let id = format!(
            "{}-{}",
            match template.kind {
                ReportKind::Research => "research",
                ReportKind::CodeReview => "review",
                ReportKind::Finance => "finance",
                ReportKind::General => "report",
                ReportKind::FounderValidationReport => "founder_validation",
                ReportKind::FounderMarketSignalReport => "market_signal",
                ReportKind::ValidationExperimentReport => "experiment",
                ReportKind::MvpScopeReport => "mvp_scope",
                ReportKind::PricingHypothesisReport => "pricing",
                ReportKind::FounderDecisionReport => "decision",
                ReportKind::BuilderHandoffReport => "handoff",
            },
            Utc::now().format("%Y%m%d%H%M%S")
        );
        let created_at = Utc::now().to_rfc3339();

        let mut markdown = String::new();
        markdown.push_str(&format!("# {}\n\n", template.title));
        markdown.push_str(&format!("**Generated at:** {}\n\n", created_at));

        for section in template.sections {
            markdown.push_str(&format!("## {}\n\n{}\n\n", section.heading, section.body));
        }

        let output = ReportOutput {
            id: id.clone(),
            kind: template.kind,
            title: template.title,
            markdown: markdown.clone(),
            created_at,
        };

        let file_path = self.reports_dir.join(format!("{}.md", id));
        fs::write(&file_path, markdown).map_err(|e| format!("Failed to write report: {}", e))?;

        let meta_path = self.reports_dir.join(format!("{}.json", id));
        let meta_json = serde_json::to_string_pretty(&output).map_err(|e| e.to_string())?;
        fs::write(&meta_path, meta_json)
            .map_err(|e| format!("Failed to write report metadata: {}", e))?;

        Ok(output)
    }

    pub fn list_reports(&self) -> Result<Vec<ReportOutput>, String> {
        let mut reports = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.reports_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "json" {
                                if let Ok(contents) = fs::read_to_string(entry.path()) {
                                    if let Ok(report) =
                                        serde_json::from_str::<ReportOutput>(&contents)
                                    {
                                        reports.push(report);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        reports.sort_by(|a, b| b.created_at.cmp(&a.created_at)); // newest first
        Ok(reports)
    }

    pub fn get_report(&self, id: &str) -> Result<Option<ReportOutput>, String> {
        let meta_path = self.reports_dir.join(format!("{}.json", id));
        if meta_path.exists() {
            let contents = fs::read_to_string(meta_path).map_err(|e| e.to_string())?;
            let report =
                serde_json::from_str::<ReportOutput>(&contents).map_err(|e| e.to_string())?;
            Ok(Some(report))
        } else {
            Ok(None)
        }
    }
}
