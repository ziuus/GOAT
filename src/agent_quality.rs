use crate::brain_context::BrainContextPackBuilder;
use crate::brain_index::BrainIndexManager;
use crate::brain_models::BrainContextPack;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskKind {
    Strategic,
    Coding,
    Synthesis,
    Creative,
    FastDataExtraction,
}

pub struct ProviderRouting;

impl ProviderRouting {
    pub fn select_provider(kind: &TaskKind) -> String {
        match kind {
            TaskKind::Strategic | TaskKind::Synthesis | TaskKind::Coding => "openai".to_string(),
            TaskKind::Creative => "openai".to_string(), // Or anthropic in future
            TaskKind::FastDataExtraction => "groq".to_string(),
        }
    }
}

pub struct QualityGate;

impl QualityGate {
    pub fn evaluate_scorecard(total_score: u8) -> Result<()> {
        if total_score > 100 {
            anyhow::bail!("Scorecard total score exceeds 100");
        }
        Ok(())
    }

    pub fn evaluate_markdown(content: &str) -> Result<()> {
        if content.trim().is_empty() {
            anyhow::bail!("Generated content is empty");
        }
        if !content.contains("#") {
            anyhow::bail!("Content lacks required markdown headers");
        }
        Ok(())
    }
}

pub struct AgentContextPacker<'a> {
    manager: &'a BrainIndexManager,
    agent_id: String,
}

impl<'a> AgentContextPacker<'a> {
    pub fn new(manager: &'a BrainIndexManager, agent_id: &str) -> Self {
        Self {
            manager,
            agent_id: agent_id.to_string(),
        }
    }

    pub async fn pack_for_task(&self, task_query: &str) -> Result<BrainContextPack> {
        BrainContextPackBuilder::new(self.manager, task_query.to_string())
            .with_agent(self.agent_id.clone())
            .limit_items(15)
            .build()
            .await
    }
}
