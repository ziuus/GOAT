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
        self.session_skills
            .insert(candidate.id.clone(), candidate.clone());
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

impl SkillResearcher {
    pub fn save_pack(&self, paths: &crate::paths::GoatPaths, name: &str) -> Result<()> {
        let active = self.get_active_skills();
        if active.is_empty() {
            anyhow::bail!("No active skills to save");
        }

        let pack_dir = paths.skill_packs_dir.join(name);
        std::fs::create_dir_all(&pack_dir)?;

        let mut items = Vec::new();
        let mut md_content = format!("# Skill Pack: {}\n\n", name);
        for skill in active {
            items.push(SkillPackItem {
                skill_id: skill.id.clone(),
                source_ref: skill.source.clone(),
                trust_level: skill.trust_level.clone(),
            });
            md_content.push_str(&format!("- **{}** ({})\n", skill.title, skill.source));
        }

        let manifest = SkillPackManifest {
            name: name.to_string(),
            description: "Saved from session".to_string(),
            tags: vec![],
            skills: items,
            created_from_session: Some(self.session_id.clone()),
            created_from_task: None,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        std::fs::write(pack_dir.join("PACK.md"), md_content)?;
        std::fs::write(
            pack_dir.join("pack.meta.json"),
            serde_json::to_string_pretty(&manifest)?,
        )?;

        Ok(())
    }
}
