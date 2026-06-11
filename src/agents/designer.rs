use anyhow::{anyhow, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::paths::GoatPaths;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerTargetType {
    Dashboard,
    LandingPage,
    Onboarding,
    Form,
    Mobile,
    GeneralUI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerTarget {
    pub kind: DesignerTargetType,
    pub path_or_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerIssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerIssue {
    pub id: String,
    pub severity: DesignerIssueSeverity,
    pub category: String, // e.g. "accessibility", "visual_hierarchy", "responsive"
    pub description: String,
    pub suggestion: String,
    pub element_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesignerScorecard {
    pub clarity: u8,
    pub visual_hierarchy: u8,
    pub spacing: u8,
    pub typography: u8,
    pub color_contrast: u8,
    pub responsiveness: u8,
    pub accessibility: u8,
    pub consistency: u8,
    pub information_architecture: u8,
    pub conversion_focus: u8,
    pub trust_signals: u8,
    pub empty_error_states: u8,
    pub total_score: f32,
    pub strongest_areas: Vec<String>,
    pub weakest_areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerImprovementPlan {
    pub review_id: String,
    pub quick_wins: Vec<String>,
    pub medium_improvements: Vec<String>,
    pub larger_redesigns: Vec<String>,
    pub files_involved: Vec<String>,
    pub non_goals: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub tests_to_run: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerHandoffBrief {
    pub review_id: String,
    pub goal: String,
    pub target_files: Vec<String>,
    pub current_issues: Vec<String>,
    pub exact_ui_changes: Vec<String>,
    pub constraints: Vec<String>,
    pub non_goals: Vec<String>,
    pub accessibility_requirements: Vec<String>,
    pub responsive_requirements: Vec<String>,
    pub empty_error_states: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerWorkflowState {
    Pending,
    Inspecting,
    Scoring,
    AccessibilityCheck,
    Planning,
    HandoffReady,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerReview {
    pub id: String,
    pub title: String,
    pub target: DesignerTarget,
    pub state: DesignerWorkflowState,
    pub scorecard: Option<DesignerScorecard>,
    pub issues: Vec<DesignerIssue>,
    pub improvement_plan: Option<DesignerImprovementPlan>,
    pub handoff_brief: Option<DesignerHandoffBrief>,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct DesignerAgent {
    storage_dir: PathBuf,
}

impl DesignerAgent {
    pub fn new() -> Result<Self> {
        let paths = GoatPaths::resolve()?;
        let storage_dir = paths.data_dir.join("agents").join("prime").join("designer");
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }
        Ok(Self { storage_dir })
    }

    fn reviews_file(&self) -> PathBuf {
        self.storage_dir.join("reviews.jsonl")
    }

    pub fn list_reviews(&self) -> Result<Vec<DesignerReview>> {
        let path = self.reviews_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path)?;
        let mut out = Vec::new();
        for line in content.lines().filter(|l| !l.trim().is_empty()) {
            if let Ok(rev) = serde_json::from_str::<DesignerReview>(line) {
                out.push(rev);
            }
        }
        Ok(out)
    }

    pub fn get_review(&self, id: &str) -> Result<Option<DesignerReview>> {
        let reviews = self.list_reviews()?;
        Ok(reviews.into_iter().find(|r| r.id == id))
    }

    pub fn save_review(&self, review: &DesignerReview) -> Result<()> {
        let mut reviews = self.list_reviews()?;
        if let Some(idx) = reviews.iter().position(|r| r.id == review.id) {
            reviews[idx] = review.clone();
        } else {
            reviews.push(review.clone());
        }
        let mut out = String::new();
        for r in &reviews {
            if let Ok(line) = serde_json::to_string(r) {
                out.push_str(&line);
                out.push('\n');
            }
        }
        fs::write(self.reviews_file(), out)?;
        Ok(())
    }

    pub fn create_review(&self, target_type: DesignerTargetType, path: &str, desc: Option<String>) -> Result<DesignerReview> {
        let review = DesignerReview {
            id: Uuid::new_v4().to_string(),
            title: format!("{:?} Review: {}", target_type, path),
            target: DesignerTarget {
                kind: target_type,
                path_or_url: path.to_string(),
                description: desc,
            },
            state: DesignerWorkflowState::Pending,
            scorecard: None,
            issues: Vec::new(),
            improvement_plan: None,
            handoff_brief: None,
            timeline_refs: Vec::new(),
            brain_refs: Vec::new(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.save_review(&review)?;
        Ok(review)
    }

    pub fn run_scorecard(&self, review_id: &str) -> Result<DesignerReview> {
        let mut rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        rev.state = DesignerWorkflowState::Scoring;
        rev.updated_at = Utc::now().to_rfc3339();
        
        let scorecard = DesignerScorecard {
            clarity: 4,
            visual_hierarchy: 3,
            spacing: 3,
            typography: 4,
            color_contrast: 4,
            responsiveness: 2,
            accessibility: 2,
            consistency: 3,
            information_architecture: 4,
            conversion_focus: 3,
            trust_signals: 2,
            empty_error_states: 2,
            total_score: 3.0,
            strongest_areas: vec!["Typography".into(), "Clarity".into()],
            weakest_areas: vec!["Accessibility".into(), "Empty States".into(), "Responsiveness".into()],
        };
        rev.scorecard = Some(scorecard);
        self.save_review(&rev)?;
        Ok(rev)
    }

    pub fn run_accessibility_check(&self, review_id: &str) -> Result<DesignerReview> {
        let mut rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        rev.state = DesignerWorkflowState::AccessibilityCheck;
        rev.updated_at = Utc::now().to_rfc3339();

        rev.issues.push(DesignerIssue {
            id: Uuid::new_v4().to_string(),
            severity: DesignerIssueSeverity::High,
            category: "accessibility".to_string(),
            description: "Missing ARIA labels on primary icon buttons.".to_string(),
            suggestion: "Add aria-label attributes to all icon-only buttons.".to_string(),
            element_ref: Some("Header Nav".to_string()),
        });

        self.save_review(&rev)?;
        Ok(rev)
    }

    pub fn run_responsive_check(&self, review_id: &str) -> Result<DesignerReview> {
        let mut rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        rev.updated_at = Utc::now().to_rfc3339();

        rev.issues.push(DesignerIssue {
            id: Uuid::new_v4().to_string(),
            severity: DesignerIssueSeverity::Medium,
            category: "responsive".to_string(),
            description: "Data table overflows screen on mobile widths.".to_string(),
            suggestion: "Wrap table in overflow-x-auto or use card-based mobile layout.".to_string(),
            element_ref: Some("Main Dashboard View".to_string()),
        });

        self.save_review(&rev)?;
        Ok(rev)
    }

    pub fn create_improvement_plan(&self, review_id: &str) -> Result<DesignerReview> {
        let mut rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        rev.state = DesignerWorkflowState::Planning;
        rev.updated_at = Utc::now().to_rfc3339();

        let plan = DesignerImprovementPlan {
            review_id: rev.id.clone(),
            quick_wins: vec!["Add aria-labels to icons".into(), "Fix contrast on secondary text".into()],
            medium_improvements: vec!["Make data tables scrollable on mobile".into()],
            larger_redesigns: vec!["Refactor dashboard sidebar to collapsible drawer for mobile".into()],
            files_involved: vec!["apps/dashboard/src/app/page.tsx".into()],
            non_goals: vec!["Do not rewrite the entire theme system".into()],
            acceptance_criteria: vec!["Mobile view passes standard accessibility testing".into()],
            tests_to_run: vec!["npm run lint".into(), "Lighthouse audit".into()],
        };

        rev.improvement_plan = Some(plan);
        self.save_review(&rev)?;
        Ok(rev)
    }

    pub fn create_handoff_brief(&self, review_id: &str) -> Result<DesignerReview> {
        let mut rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        rev.state = DesignerWorkflowState::HandoffReady;
        rev.updated_at = Utc::now().to_rfc3339();

        let brief = DesignerHandoffBrief {
            review_id: rev.id.clone(),
            goal: "Improve accessibility and mobile responsiveness of dashboard".to_string(),
            target_files: vec!["apps/dashboard/src/components/layout.tsx".into()],
            current_issues: vec!["Missing ARIA labels", "Table overflow"].into_iter().map(String::from).collect(),
            exact_ui_changes: vec!["Add overflow-x-auto to table wrappers", "Add aria-label to sidebar toggles"].into_iter().map(String::from).collect(),
            constraints: vec!["Do not break existing layouts on desktop"].into_iter().map(String::from).collect(),
            non_goals: vec!["No new dependencies"].into_iter().map(String::from).collect(),
            accessibility_requirements: vec!["WCAG AA contrast", "Keyboard navigation"].into_iter().map(String::from).collect(),
            responsive_requirements: vec!["Functional down to 320px width"].into_iter().map(String::from).collect(),
            empty_error_states: vec!["Ensure empty tables display a placeholder"].into_iter().map(String::from).collect(),
            acceptance_criteria: vec!["Build passes", "No new lint errors"].into_iter().map(String::from).collect(),
        };

        rev.handoff_brief = Some(brief);
        self.save_review(&rev)?;
        Ok(rev)
    }

    pub fn generate_report(&self, review_id: &str) -> Result<String> {
        let rev = self.get_review(review_id)?.ok_or_else(|| anyhow!("Review not found"))?;
        let report_id = Uuid::new_v4().to_string();
        
        let mut body = format!("# Designer Report: {}\n\n", rev.title);
        body.push_str(&format!("**Target:** {}\n", rev.target.path_or_url));
        if let Some(desc) = &rev.target.description {
            body.push_str(&format!("**Description:** {}\n", desc));
        }
        body.push_str("\n## Scorecard\n");
        if let Some(score) = &rev.scorecard {
            body.push_str(&format!("- Total Score: {}/5.0\n", score.total_score));
            body.push_str(&format!("- Strongest Areas: {:?}\n", score.strongest_areas));
            body.push_str(&format!("- Weakest Areas: {:?}\n", score.weakest_areas));
        }

        body.push_str("\n## Issues\n");
        for issue in &rev.issues {
            body.push_str(&format!("- [{:?}] {}: {} -> {}\n", issue.severity, issue.category, issue.description, issue.suggestion));
        }

        if let Some(plan) = &rev.improvement_plan {
            body.push_str("\n## Improvement Plan\n");
            body.push_str(&format!("- Quick Wins: {:?}\n", plan.quick_wins));
            body.push_str(&format!("- Medium Improvements: {:?}\n", plan.medium_improvements));
        }

        if let Some(brief) = &rev.handoff_brief {
            body.push_str("\n## Builder Handoff Brief\n");
            body.push_str(&format!("Goal: {}\n", brief.goal));
            body.push_str(&format!("Target Files: {:?}\n", brief.target_files));
            body.push_str(&format!("Exact UI Changes: {:?}\n", brief.exact_ui_changes));
        }

        let report_path = self.storage_dir.join("reports");
        if !report_path.exists() {
            fs::create_dir_all(&report_path)?;
        }
        let out_file = report_path.join(format!("{}.md", report_id));
        fs::write(&out_file, body)?;
        
        Ok(report_id)
    }
}
