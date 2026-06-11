use crate::agent_quality::{AgentContextPacker, QualityGate, TaskKind};
use crate::brain_index::BrainIndexManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuilderWorkflowState {
    ContextGathered,
    PlanDrafted,
    CodeGenerated,
    TestsWritten,
    Reviewed,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFeaturePlan {
    pub id: String,
    pub title: String,
    pub description: String,
    pub architecture_notes: String,
    pub implementation_steps: Vec<String>,
    pub test_plan: Vec<String>,
    pub status: BuilderWorkflowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderCodeReview {
    pub plan_id: String,
    pub feedback: Vec<String>,
    pub is_approved: bool,
}

pub struct BuilderAgent {
    pub base_dir: PathBuf,
}

impl Default for BuilderAgent {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            base_dir: PathBuf::new(),
        })
    }
}

impl BuilderAgent {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/agents/prime/builder");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }

    pub async fn draft_plan(
        &self,
        title: &str,
        description: &str,
        brain_manager: &BrainIndexManager,
    ) -> Result<BuilderFeaturePlan> {
        // Integrate Context Pack
        let packer = AgentContextPacker::new(brain_manager, "builder");
        let context_pack = packer.pack_for_task(description).await?;

        let plan = BuilderFeaturePlan {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            architecture_notes: format!(
                "Using context pack with {} items (size: {})",
                context_pack.items.len(),
                context_pack.estimated_size
            ),
            implementation_steps: vec![
                "Step 1: Setup module".to_string(),
                "Step 2: Implement logic".to_string(),
            ],
            test_plan: vec!["Write unit tests".to_string()],
            status: BuilderWorkflowState::PlanDrafted,
        };

        // Quality Gate check
        QualityGate::evaluate_markdown(&plan.description)?;

        let plan_file = self.base_dir.join(format!("{}.json", plan.id));
        fs::write(plan_file, serde_json::to_string_pretty(&plan)?)?;

        Ok(plan)
    }

    pub fn review_code(&self, plan_id: &str) -> Result<BuilderCodeReview> {
        Ok(BuilderCodeReview {
            plan_id: plan_id.to_string(),
            feedback: vec!["LGTM".to_string()],
            is_approved: true,
        })
    }
}
