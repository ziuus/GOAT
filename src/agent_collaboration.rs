use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentCollaborationStatus {
    Draft,
    Planned,
    WaitingForApproval,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentCollaborationRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCollaborationSession {
    pub id: String,
    pub title: String,
    pub user_goal: String,
    pub template: Option<String>,
    pub participating_agents: Vec<String>,
    pub status: AgentCollaborationStatus,
    pub current_step_index: usize,
    pub steps: Vec<AgentCollaborationStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub timeline_refs: Vec<String>,
    pub report_refs: Vec<String>,
    pub brain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCollaborationStep {
    pub id: String,
    pub session_id: String,
    pub agent: String,
    pub action: String,
    pub description: String,
    pub status: AgentCollaborationStatus,
    pub required_approval: bool,
    pub expected_output: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub artifacts_produced: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandoff {
    pub id: String,
    pub session_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub title: String,
    pub context_summary: String,
    pub input_refs: Vec<String>,
    pub output_expected: String,
    pub constraints: Vec<String>,
    pub safety_notes: String,
    pub acceptance_criteria: Vec<String>,
    pub status: AgentCollaborationStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCollaborationArtifact {
    pub id: String,
    pub session_id: String,
    pub agent: String,
    pub kind: String,
    pub title: String,
    pub content_summary: String,
    pub content_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCollaborationReport {
    pub id: String,
    pub session_id: String,
    pub original_goal: String,
    pub participating_agents: Vec<String>,
    pub step_summary: String,
    pub handoffs_count: usize,
    pub decisions: Vec<String>,
    pub risks: Vec<String>,
    pub next_actions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub struct AgentCollaborationManager {
    pub storage_dir: PathBuf,
}

impl AgentCollaborationManager {
    pub fn new() -> Result<Self, String> {
        let storage_dir = dirs::data_local_dir()
            .ok_or("No local data dir")?
            .join("goat")
            .join("collaborations");
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir).map_err(|e| e.to_string())?;
        }
        let reports_dir = storage_dir.join("reports");
        if !reports_dir.exists() {
            fs::create_dir_all(&reports_dir).map_err(|e| e.to_string())?;
        }
        Ok(Self { storage_dir })
    }

    fn sessions_file(&self) -> PathBuf {
        self.storage_dir.join("sessions.jsonl")
    }

    fn handoffs_file(&self) -> PathBuf {
        self.storage_dir.join("handoffs.jsonl")
    }

    fn artifacts_file(&self) -> PathBuf {
        self.storage_dir.join("artifacts.jsonl")
    }

    fn append_jsonl<T: Serialize>(path: &PathBuf, item: &T) -> Result<(), String> {
        let line = serde_json::to_string(item).map_err(|e| e.to_string())?;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn read_jsonl<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<Vec<T>, String> {
        if !path.exists() {
            return Ok(vec![]);
        }
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut items = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(item) = serde_json::from_str(line) {
                items.push(item);
            }
        }
        Ok(items)
    }

    pub fn list_sessions(&self) -> Result<Vec<AgentCollaborationSession>, String> {
        let sessions: Vec<AgentCollaborationSession> = Self::read_jsonl(&self.sessions_file())?;
        let mut latest = HashMap::new();
        for session in sessions {
            latest.insert(session.id.clone(), session);
        }
        let mut result: Vec<AgentCollaborationSession> = latest.into_values().collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(result)
    }

    pub fn get_session(&self, id: &str) -> Result<Option<AgentCollaborationSession>, String> {
        let sessions = Self::read_jsonl::<AgentCollaborationSession>(&self.sessions_file())?;
        let latest = sessions.into_iter().filter(|s| s.id == id).last();
        Ok(latest)
    }

    pub fn create_session(
        &self,
        title: &str,
        goal: &str,
        template: Option<&str>,
    ) -> Result<AgentCollaborationSession, String> {
        let (participating_agents, steps) =
            self.build_plan_from_template(template.unwrap_or("custom"));

        let session = AgentCollaborationSession {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            user_goal: goal.to_string(),
            template: template.map(|s| s.to_string()),
            participating_agents,
            status: AgentCollaborationStatus::Planned,
            current_step_index: 0,
            steps,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            timeline_refs: vec![],
            report_refs: vec![],
            brain_refs: vec![],
        };
        Self::append_jsonl(&self.sessions_file(), &session)?;
        Ok(session)
    }

    fn build_plan_from_template(
        &self,
        template: &str,
    ) -> (Vec<String>, Vec<AgentCollaborationStep>) {
        let mut agents = vec![];
        let mut steps = vec![];

        match template {
            "startup-validation-flow" => {
                agents = vec![
                    "Cofounder".to_string(),
                    "Researcher".to_string(),
                    "Designer".to_string(),
                    "Socializer".to_string(),
                    "Builder".to_string(),
                ];
                steps = vec![
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Cofounder".to_string(),
                        action: "Validate Idea".to_string(),
                        description: "Clarify idea and assumptions".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Idea Validation Brief".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Researcher".to_string(),
                        action: "Market Scan".to_string(),
                        description: "Competitor and market scan".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Market Research Report".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Designer".to_string(),
                        action: "Design Critique".to_string(),
                        description: "Landing page design critique".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Design Guidelines".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Socializer".to_string(),
                        action: "Launch Plan".to_string(),
                        description: "Ethical launch distribution plan".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Launch Strategy".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Builder".to_string(),
                        action: "Builder Handoff".to_string(),
                        description: "MVP Handoff Brief".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Implementation Plan".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                ];
            }
            "launch-readiness-flow" => {
                agents = vec![
                    "Cofounder".to_string(),
                    "Designer".to_string(),
                    "Socializer".to_string(),
                    "Operator".to_string(),
                ];
                steps = vec![
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Cofounder".to_string(),
                        action: "Positioning Check".to_string(),
                        description: "Verify value proposition".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Positioning Check Pass".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Designer".to_string(),
                        action: "Onboarding Check".to_string(),
                        description: "Landing and onboarding UX pass".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("UX Audit Report".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Socializer".to_string(),
                        action: "Launch Calendar".to_string(),
                        description: "Prepare social calendar".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Calendar Output".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Operator".to_string(),
                        action: "Readiness Checklist".to_string(),
                        description: "Deployment readiness checks".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Deployment Go/No-Go".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                ];
            }
            "build-and-release-flow" => {
                agents = vec![
                    "Builder".to_string(),
                    "Operator".to_string(),
                    "Researcher".to_string(),
                ];
                steps = vec![
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Builder".to_string(),
                        action: "Implementation".to_string(),
                        description: "Implementation brief".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Code Changes".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Operator".to_string(),
                        action: "Release Plan".to_string(),
                        description: "Deployment plan".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Deployment Plan".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Researcher".to_string(),
                        action: "Risk Check".to_string(),
                        description: "Technology risk scan".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Risk Scan Result".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                ];
            }
            "learning-project-flow" => {
                agents = vec![
                    "Learner".to_string(),
                    "Researcher".to_string(),
                    "Builder".to_string(),
                    "Designer".to_string(),
                ];
                steps = vec![
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Learner".to_string(),
                        action: "Scope Project".to_string(),
                        description: "Learning goal and project scope".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Project Scope".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Researcher".to_string(),
                        action: "Topic Scan".to_string(),
                        description: "Learning resources scan".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Resource Links".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Builder".to_string(),
                        action: "Implementation Handoff".to_string(),
                        description: "Project implementation brief".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Code Scaffold".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Designer".to_string(),
                        action: "UX Review".to_string(),
                        description: "UI/UX review if applicable".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Design Feedback".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                ];
            }
            "incident-response-flow" => {
                agents = vec![
                    "Operator".to_string(),
                    "Researcher".to_string(),
                    "Builder".to_string(),
                ];
                steps = vec![
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Operator".to_string(),
                        action: "Incident Intake".to_string(),
                        description: "Intake and severity check".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Incident Summary".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Researcher".to_string(),
                        action: "Error Investigation".to_string(),
                        description: "Source investigation".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: false,
                        expected_output: Some("Root Cause Hypothesis".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Builder".to_string(),
                        action: "Fix Handoff".to_string(),
                        description: "Implementation fix brief".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Patch Details".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                    AgentCollaborationStep {
                        id: Uuid::new_v4().to_string(),
                        session_id: "".to_string(),
                        agent: "Operator".to_string(),
                        action: "Postmortem".to_string(),
                        description: "Rollback or postmortem report".to_string(),
                        status: AgentCollaborationStatus::Planned,
                        required_approval: true,
                        expected_output: Some("Incident Report".to_string()),
                        started_at: None,
                        completed_at: None,
                        artifacts_produced: vec![],
                    },
                ];
            }
            _ => {
                agents = vec!["Cofounder".to_string(), "Builder".to_string()];
                steps = vec![AgentCollaborationStep {
                    id: Uuid::new_v4().to_string(),
                    session_id: "".to_string(),
                    agent: "Cofounder".to_string(),
                    action: "Analyze".to_string(),
                    description: "Initial analysis".to_string(),
                    status: AgentCollaborationStatus::Planned,
                    required_approval: false,
                    expected_output: Some("Analysis Result".to_string()),
                    started_at: None,
                    completed_at: None,
                    artifacts_produced: vec![],
                }];
            }
        }
        (agents, steps)
    }

    pub fn update_session(&self, session: &AgentCollaborationSession) -> Result<(), String> {
        Self::append_jsonl(&self.sessions_file(), session)
    }

    pub fn start_session(&self, id: &str) -> Result<AgentCollaborationSession, String> {
        let mut session = self.get_session(id)?.ok_or("Session not found")?;
        session.status = AgentCollaborationStatus::Running;
        session.updated_at = Utc::now();
        if let Some(step) = session.steps.get_mut(0) {
            step.status = AgentCollaborationStatus::Running;
            step.started_at = Some(Utc::now());
        }
        self.update_session(&session)?;
        Ok(session)
    }

    pub fn advance_step(&self, id: &str) -> Result<AgentCollaborationSession, String> {
        let mut session = self.get_session(id)?.ok_or("Session not found")?;
        let current_index = session.current_step_index;
        if current_index >= session.steps.len() {
            session.status = AgentCollaborationStatus::Completed;
            session.updated_at = Utc::now();
            self.update_session(&session)?;
            return Ok(session);
        }

        let has_next = current_index + 1 < session.steps.len();
        let next_agent = if has_next {
            Some(session.steps[current_index + 1].agent.clone())
        } else {
            None
        };
        let next_desc = if has_next {
            Some(session.steps[current_index + 1].description.clone())
        } else {
            None
        };

        {
            let step = &mut session.steps[current_index];
            step.status = AgentCollaborationStatus::Completed;
            step.completed_at = Some(Utc::now());

            // Create a handoff if there is a next step
            if let (Some(na), Some(nd)) = (next_agent, next_desc) {
                let handoff = AgentHandoff {
                    id: Uuid::new_v4().to_string(),
                    session_id: id.to_string(),
                    from_agent: step.agent.clone(),
                    to_agent: na.clone(),
                    title: format!("Handoff from {} to {}", step.agent, na),
                    context_summary: "Automated step transition".to_string(),
                    input_refs: vec![],
                    output_expected: nd,
                    constraints: vec![],
                    safety_notes: "Proceed safely according to Prime Agent guidelines.".to_string(),
                    acceptance_criteria: vec![],
                    status: AgentCollaborationStatus::Completed,
                    created_at: Utc::now(),
                    completed_at: Some(Utc::now()),
                };
                Self::append_jsonl(&self.handoffs_file(), &handoff)?;
            }
        }

        session.current_step_index += 1;
        if session.current_step_index >= session.steps.len() {
            session.status = AgentCollaborationStatus::Completed;
            session.updated_at = Utc::now();
        } else {
            let next_step = &mut session.steps[session.current_step_index];
            if next_step.required_approval {
                session.status = AgentCollaborationStatus::WaitingForApproval;
                next_step.status = AgentCollaborationStatus::WaitingForApproval;
            } else {
                next_step.status = AgentCollaborationStatus::Running;
                next_step.started_at = Some(Utc::now());
            }
        }

        self.update_session(&session)?;
        Ok(session)
    }

    pub fn pause_session(&self, id: &str) -> Result<AgentCollaborationSession, String> {
        let mut session = self.get_session(id)?.ok_or("Session not found")?;
        session.status = AgentCollaborationStatus::Paused;
        session.updated_at = Utc::now();
        self.update_session(&session)?;
        Ok(session)
    }

    pub fn resume_session(&self, id: &str) -> Result<AgentCollaborationSession, String> {
        let mut session = self.get_session(id)?.ok_or("Session not found")?;
        session.status = AgentCollaborationStatus::Running;
        session.updated_at = Utc::now();
        self.update_session(&session)?;
        Ok(session)
    }

    pub fn cancel_session(&self, id: &str) -> Result<AgentCollaborationSession, String> {
        let mut session = self.get_session(id)?.ok_or("Session not found")?;
        session.status = AgentCollaborationStatus::Cancelled;
        session.updated_at = Utc::now();
        self.update_session(&session)?;
        Ok(session)
    }

    pub fn list_handoffs(&self, session_id: &str) -> Result<Vec<AgentHandoff>, String> {
        let handoffs = Self::read_jsonl::<AgentHandoff>(&self.handoffs_file())?;
        Ok(handoffs
            .into_iter()
            .filter(|h| h.session_id == session_id)
            .collect())
    }

    pub fn generate_report(&self, session_id: &str) -> Result<AgentCollaborationReport, String> {
        let session = self.get_session(session_id)?.ok_or("Session not found")?;
        let handoffs = self.list_handoffs(session_id)?;

        let report = AgentCollaborationReport {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            original_goal: session.user_goal.clone(),
            participating_agents: session.participating_agents.clone(),
            step_summary: format!(
                "Completed {} / {} steps",
                session.current_step_index,
                session.steps.len()
            ),
            handoffs_count: handoffs.len(),
            decisions: vec!["Automatically approved steps".to_string()],
            risks: vec!["Pending full autonomy analysis".to_string()],
            next_actions: vec!["Review timeline events".to_string()],
            created_at: Utc::now(),
        };

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let m = AgentCollaborationManager::new().unwrap();
        let session = m
            .create_session("Test Flow", "Goal", Some("startup-validation-flow"))
            .unwrap();
        assert_eq!(session.title, "Test Flow");
        assert_eq!(session.steps.len(), 5);
        assert_eq!(session.status, AgentCollaborationStatus::Planned);
    }
}
