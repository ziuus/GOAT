use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustLevel {
    TrustedLocal,
    InstalledRemote,
    LearnedReviewed,
    SessionOnly,
    RemoteUntrusted,
    GeneratedDraft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionSkillState {
    Suggested,
    Attached,
    Detached,
    Rejected,
    PromotedToPack,
    InstalledGlobally,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSkillCandidate {
    pub id: String,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub reason: String,
    pub trust_level: TrustLevel,
    pub risk_level: RiskLevel,
    pub state: SessionSkillState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPackItem {
    pub skill_id: String,
    pub source_ref: String,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPackManifest {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub skills: Vec<SkillPackItem>,
    pub created_from_session: Option<String>,
    pub created_from_task: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct SkillResearcher {
    pub enabled: bool,
    pub session_id: String,
    pub session_skills: HashMap<String, SessionSkillCandidate>,
}

impl SkillResearcher {
    pub fn new(session_id: String) -> Self {
        Self {
            enabled: false,
            session_id,
            session_skills: HashMap::new(),
        }
    }

    pub fn toggle(&mut self, state: bool) {
        self.enabled = state;
    }

    pub fn suggest_mock(&mut self) -> Vec<SessionSkillCandidate> {
        let candidate = SessionSkillCandidate {
            id: Uuid::new_v4().to_string(),
            source: "mock_marketplace".to_string(),
            title: "React Review Skill".to_string(),
            summary: "Reviews React components for best practices.".to_string(),
            reason: "Matches keyword 'react' in current task.".to_string(),
            trust_level: TrustLevel::RemoteUntrusted,
            risk_level: RiskLevel::Low,
            state: SessionSkillState::Suggested,
        };
        self.session_skills.insert(candidate.id.clone(), candidate.clone());
        vec![candidate]
    }

    pub fn attach(&mut self, id: &str) -> Result<()> {
        if let Some(skill) = self.session_skills.get_mut(id) {
            skill.state = SessionSkillState::Attached;
            Ok(())
        } else {
            anyhow::bail!("Skill candidate not found")
        }
    }

    pub fn detach(&mut self, id: &str) -> Result<()> {
        if let Some(skill) = self.session_skills.get_mut(id) {
            skill.state = SessionSkillState::Detached;
            Ok(())
        } else {
            anyhow::bail!("Skill not found in session")
        }
    }

    pub fn clear(&mut self) {
        self.session_skills.clear();
    }

    pub fn get_active_skills(&self) -> Vec<SessionSkillCandidate> {
        self.session_skills
            .values()
            .filter(|s| matches!(s.state, SessionSkillState::Attached))
            .cloned()
            .collect()
    }
}
