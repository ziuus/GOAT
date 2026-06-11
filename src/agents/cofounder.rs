use crate::error::GoatError;
use crate::reports::ReportManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CofounderWorkflowState {
    #[default]
    IdeaLogged,
    Validating,
    Validated,
    Scored,
    MvpScoped,
    OutreachPlanned,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderIdea {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_audience: String,
    pub created_at: i64,
    pub state: CofounderWorkflowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderAssumption {
    pub id: String,
    pub idea_id: String,
    pub description: String,
    pub is_critical: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderValidationPlan {
    pub idea_id: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderCompetitorRef {
    pub idea_id: String,
    pub name: String,
    pub url: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderMvpScope {
    pub idea_id: String,
    pub core_features: Vec<String>,
    pub excluded_features: Vec<String>,
    pub estimated_timeline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderOutreachPlan {
    pub idea_id: String,
    pub channels: Vec<String>,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderScorecard {
    pub idea_id: String,
    pub pain_intensity: u8,
    pub frequency: u8,
    pub willingness_to_pay: u8,
    pub reachability: u8,
    pub competition: u8,
    pub build_complexity: u8,
    pub trust_requirement: u8,
    pub distribution_difficulty: u8,
    pub speed_to_validate: u8,
    pub founder_fit: u8,
    pub total_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderReport {
    pub idea_id: String,
    pub summary: String,
}

pub struct CofounderManager {
    base_dir: PathBuf,
    ideas: HashMap<String, CofounderIdea>,
}

impl CofounderManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/agents/prime/cofounder");
        fs::create_dir_all(&base_dir)?;

        let mut manager = Self {
            base_dir,
            ideas: HashMap::new(),
        };
        manager.load_ideas()?;
        Ok(manager)
    }

    fn ideas_file(&self) -> PathBuf {
        self.base_dir.join("ideas.jsonl")
    }

    fn load_ideas(&mut self) -> Result<()> {
        let path = self.ideas_file();
        if !path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&path)?;
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(idea) = serde_json::from_str::<CofounderIdea>(line) {
                self.ideas.insert(idea.id.clone(), idea);
            }
        }
        Ok(())
    }

    fn save_ideas(&self) -> Result<()> {
        let path = self.ideas_file();
        let mut lines = Vec::new();
        for idea in self.ideas.values() {
            if let Ok(json) = serde_json::to_string(idea) {
                lines.push(json);
            }
        }
        fs::write(&path, lines.join("\n"))?;
        Ok(())
    }

    pub fn list_ideas(&self) -> Vec<CofounderIdea> {
        self.ideas.values().cloned().collect()
    }

    pub fn get_idea(&self, id: &str) -> Option<CofounderIdea> {
        self.ideas.get(id).cloned()
    }

    pub fn add_idea(
        &mut self,
        title: String,
        description: String,
        target_audience: String,
    ) -> Result<CofounderIdea> {
        let id = Uuid::new_v4().to_string();
        let idea = CofounderIdea {
            id: id.clone(),
            title,
            description,
            target_audience,
            created_at: chrono::Utc::now().timestamp(),
            state: CofounderWorkflowState::IdeaLogged,
        };
        self.ideas.insert(id, idea.clone());
        self.save_ideas()?;
        Ok(idea)
    }

    pub fn generate_validation_plan(&mut self, id: &str) -> Result<CofounderValidationPlan> {
        if let Some(idea) = self.ideas.get_mut(id) {
            idea.state = CofounderWorkflowState::Validating;
            self.save_ideas()?;
        } else {
            return Err(anyhow::anyhow!("Idea {} not found", id));
        }

        Ok(CofounderValidationPlan {
            idea_id: id.to_string(),
            steps: vec![
                "Identify 5 potential customers".to_string(),
                "Conduct user interviews".to_string(),
            ],
        })
    }

    pub fn generate_mvp_scope(&mut self, id: &str) -> Result<CofounderMvpScope> {
        if let Some(idea) = self.ideas.get_mut(id) {
            idea.state = CofounderWorkflowState::MvpScoped;
            self.save_ideas()?;
        } else {
            return Err(anyhow::anyhow!("Idea {} not found", id));
        }

        Ok(CofounderMvpScope {
            idea_id: id.to_string(),
            core_features: vec!["Landing page".to_string(), "Email capture".to_string()],
            excluded_features: vec!["Mobile app".to_string(), "Payments".to_string()],
            estimated_timeline: "2 weeks".to_string(),
        })
    }

    pub fn generate_competitors(&self, id: &str) -> Result<Vec<CofounderCompetitorRef>> {
        Ok(vec![CofounderCompetitorRef {
            idea_id: id.to_string(),
            name: "Example Corp".to_string(),
            url: "https://example.com".to_string(),
            strengths: vec!["Big brand".to_string()],
            weaknesses: vec!["Slow".to_string()],
        }])
    }

    pub fn generate_landing_page_brief(&self, id: &str) -> Result<String> {
        Ok(format!("Landing page brief for idea {}", id))
    }

    pub fn generate_outreach_plan(&mut self, id: &str) -> Result<CofounderOutreachPlan> {
        if let Some(idea) = self.ideas.get_mut(id) {
            idea.state = CofounderWorkflowState::OutreachPlanned;
            self.save_ideas()?;
        } else {
            return Err(anyhow::anyhow!("Idea {} not found", id));
        }

        Ok(CofounderOutreachPlan {
            idea_id: id.to_string(),
            channels: vec!["Twitter".to_string(), "LinkedIn".to_string()],
            messages: vec!["Hi, we are building something new...".to_string()],
        })
    }

    pub fn generate_scorecard(&mut self, id: &str) -> Result<CofounderScorecard> {
        if let Some(idea) = self.ideas.get_mut(id) {
            idea.state = CofounderWorkflowState::Scored;
            self.save_ideas()?;
        } else {
            return Err(anyhow::anyhow!("Idea {} not found", id));
        }

        let score = CofounderScorecard {
            idea_id: id.to_string(),
            pain_intensity: 4,
            frequency: 3,
            willingness_to_pay: 3,
            reachability: 4,
            competition: 2,
            build_complexity: 2,
            trust_requirement: 3,
            distribution_difficulty: 3,
            speed_to_validate: 4,
            founder_fit: 5,
            total_score: 33, // Example total
        };

        Ok(score)
    }

    pub fn generate_report(&self, id: &str) -> Result<CofounderReport> {
        let report_manager = crate::reports::ReportManager::new();
        let template = crate::reports::ReportTemplate {
            kind: crate::reports::ReportKind::General,
            title: format!("Cofounder Idea {}", id),
            sections: vec![crate::reports::ReportSection {
                heading: "Executive Summary".to_string(),
                body: format!("Summary for idea {}", id),
            }],
        };
        let report_output = report_manager
            .generate_report(template)
            .map_err(|e| anyhow::anyhow!("Report error: {}", e))?;

        Ok(CofounderReport {
            idea_id: id.to_string(),
            summary: format!("Report {} generated for idea {}", report_output.id, id),
        })
    }

    pub async fn deep_evaluate_idea(
        &mut self,
        id: &str,
        brain_manager: &crate::brain_index::BrainIndexManager,
    ) -> Result<CofounderScorecard> {
        let idea = self.ideas.get(id).ok_or_else(|| anyhow::anyhow!("Idea not found"))?;
        
        let packer = crate::agent_quality::AgentContextPacker::new(brain_manager, "cofounder");
        let _context = packer.pack_for_task(&idea.description).await?;

        // In full implementation, we use PromptForge / LLM here with ProviderRouting
        let provider = crate::agent_quality::ProviderRouting::select_provider(&crate::agent_quality::TaskKind::Strategic);
        
        // Mocking the result of the LLM for now
        let score = CofounderScorecard {
            idea_id: id.to_string(),
            pain_intensity: 8,
            frequency: 7,
            willingness_to_pay: 6,
            reachability: 8,
            competition: 5,
            build_complexity: 4,
            trust_requirement: 6,
            distribution_difficulty: 7,
            speed_to_validate: 9,
            founder_fit: 9,
            total_score: 69,
        };

        crate::agent_quality::QualityGate::evaluate_scorecard(score.total_score)?;
        
        if let Some(idea_mut) = self.ideas.get_mut(id) {
            idea_mut.state = CofounderWorkflowState::Scored;
            self.save_ideas()?;
        }

        Ok(score)
    }
}
