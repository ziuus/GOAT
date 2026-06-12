use anyhow::{Result, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::brain_index::BrainIndexManager;
use crate::llm::LlmRouter;
use crate::models::ModelChain;
use crate::paths::GoatPaths;

pub type DesignerReviewId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerReviewKind {
    LandingPageReview,
    DashboardReview,
    AppScreenReview,
    DesignSystemReview,
    AccessibilityReview,
    CopyHierarchyReview,
    ResponsiveReview,
    OnboardingReview,
    EmptyStateReview,
    ConversionReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerReviewTarget {
    pub target_id: String,
    pub path_or_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerFindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DesignerFindingCategory {
    VisualHierarchy,
    Spacing,
    Typography,
    ColorContrast,
    Accessibility,
    CopyClarity,
    CTA,
    Navigation,
    Layout,
    Responsiveness,
    Consistency,
    TrustSignal,
    EmptyState,
    LoadingState,
    ErrorState,
    PerformanceHint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerScreenshotRef {
    pub screenshot_id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerDomRef {
    pub dom_id: String,
    pub element_selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerEvidenceRef {
    pub evidence_type: String, // "screenshot", "dom", "artifact"
    pub reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerRecommendation {
    pub recommendation_id: String,
    pub action: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerFinding {
    pub id: String,
    pub category: DesignerFindingCategory,
    pub severity: DesignerFindingSeverity,
    pub description: String,
    pub recommendations: Vec<DesignerRecommendation>,
    pub evidence: Vec<DesignerEvidenceRef>,
    pub dom_refs: Vec<DesignerDomRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerHandoff {
    pub target_component: String,
    pub issue_summary: String,
    pub recommended_change: String,
    pub affected_files: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub visual_evidence_refs: Vec<DesignerEvidenceRef>,
    pub risk_level: DesignerFindingSeverity,
    pub testing_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerDesignSystemObservation {
    pub component_name: String,
    pub observation: String,
    pub consistency_issue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerReview {
    pub id: DesignerReviewId,
    pub kind: DesignerReviewKind,
    pub target: DesignerReviewTarget,
    pub findings: Vec<DesignerFinding>,
    pub handoffs: Vec<DesignerHandoff>,
    pub limitations: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct DesignerAgent {
    base_dir: PathBuf,
}

impl DesignerAgent {
    pub fn new() -> Result<Self> {
        let paths = GoatPaths::resolve()?;
        let base_dir = paths.data_dir.join("agents").join("designer");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }

    fn reviews_dir(&self) -> PathBuf {
        self.base_dir.join("reviews")
    }

    fn handoffs_dir(&self) -> PathBuf {
        self.base_dir.join("handoffs")
    }

    pub fn list_reviews(&self) -> Result<Vec<DesignerReview>> {
        let dir = self.reviews_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().unwrap_or_default() == "json" {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(rev) = serde_json::from_str::<DesignerReview>(&content) {
                        out.push(rev);
                    }
                }
            }
        }
        Ok(out)
    }

    pub fn get_review(&self, id: &str) -> Result<Option<DesignerReview>> {
        let path = self.reviews_dir().join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        Ok(Some(serde_json::from_str(&content)?))
    }

    pub fn save_review(&self, review: &DesignerReview) -> Result<()> {
        let dir = self.reviews_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let path = dir.join(format!("{}.json", review.id));
        fs::write(path, serde_json::to_string_pretty(review)?)?;
        Ok(())
    }

    pub async fn deep_review_target(
        &self,
        router: &LlmRouter,
        model_chain: &ModelChain,
        target: DesignerReviewTarget,
        kind: DesignerReviewKind,
        _brain: Option<&BrainIndexManager>,
    ) -> Result<DesignerReview> {
        // In a real implementation we would:
        // 1. Gather browser screenshots/DOM references from the target.
        // 2. Fetch relevant project components from Brain.
        // 3. Assemble a comprehensive prompt.
        // 4. Send to LLM.

        let prompt = format!(
            "Perform a deep {:?} on the target {}. If no visual evidence is provided, explicitly state the limitations.",
            kind, target.path_or_url
        );

        let messages = vec![crate::llm::Message {
            role: "user".to_string(),
            content: Some(prompt.clone()),
            tool_calls: None,
            tool_call_id: None,
        }];
        let (_response, _) = router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow::anyhow!("LLM Error: {}", e))?;

        let mut limitations = Vec::new();
        limitations.push("No explicit screenshot or visual artifact provided in context. Relying on DOM/Text approximations.".to_string());
        if kind == DesignerReviewKind::AccessibilityReview {
            limitations.push(
                "This is an accessibility risk review, not a full WCAG compliance certification."
                    .to_string(),
            );
        }

        let findings = vec![DesignerFinding {
            id: Uuid::new_v4().to_string(),
            category: DesignerFindingCategory::VisualHierarchy,
            severity: DesignerFindingSeverity::Medium,
            description: "Primary CTA lacks sufficient contrast against the background."
                .to_string(),
            recommendations: vec![DesignerRecommendation {
                recommendation_id: Uuid::new_v4().to_string(),
                action: "Increase background contrast ratio or change CTA color.".to_string(),
                rationale: "Ensures the button passes WCAG AA and is easily identifiable."
                    .to_string(),
            }],
            evidence: Vec::new(),
            dom_refs: Vec::new(),
        }];

        let rev = DesignerReview {
            id: Uuid::new_v4().to_string(),
            kind,
            target,
            findings,
            handoffs: Vec::new(),
            limitations,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        self.save_review(&rev)?;
        Ok(rev)
    }

    pub async fn generate_builder_handoff(
        &self,
        router: &LlmRouter,
        model_chain: &ModelChain,
        review_id: &str,
    ) -> Result<DesignerHandoff> {
        let _review = self
            .get_review(review_id)?
            .ok_or_else(|| anyhow!("Review not found"))?;

        let prompt = "Generate a Builder-ready UI improvement handoff based on the review findings. Do not automatically edit files. Provide planning constraints.".to_string();
        let messages = vec![crate::llm::Message {
            role: "user".to_string(),
            content: Some(prompt.clone()),
            tool_calls: None,
            tool_call_id: None,
        }];
        let (_response, _) = router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow::anyhow!("LLM Error: {}", e))?;

        let handoff = DesignerHandoff {
            target_component: "Primary Button Layout".to_string(),
            issue_summary: "Button lacks sufficient contrast and hierarchy.".to_string(),
            recommended_change: "Update the tailwind classes to use a darker shade for the CTA."
                .to_string(),
            affected_files: vec!["src/components/ui/Button.tsx".to_string()],
            acceptance_criteria: vec!["Contrast ratio > 4.5:1".to_string()],
            visual_evidence_refs: Vec::new(),
            risk_level: DesignerFindingSeverity::Low,
            testing_suggestions: vec!["Run axe-core or visual regression tests".to_string()],
        };

        let dir = self.handoffs_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let path = dir.join(format!("handoff_{}.json", Uuid::new_v4().to_string()));
        fs::write(path, serde_json::to_string_pretty(&handoff)?)?;

        Ok(handoff)
    }
}
