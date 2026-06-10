use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CandidateType {
    Memory,
    Project,
    Skill,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CandidateStatus {
    Pending,
    Accepted,
    Rejected,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RedactionStatus {
    Unredacted,
    Redacted,
    NotNeeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCandidate {
    pub id: String,
    pub created_at: u64,
    pub source: String,
    pub candidate_type: CandidateType,
    pub title: String,
    pub summary: String,
    pub confidence: f32,
    pub suggested_destination: String,
    pub status: CandidateStatus,
    pub redaction_status: RedactionStatus,
    pub raw_data: String,
}

impl MemoryCandidate {
    pub fn new(source: String, candidate_type: CandidateType, title: String, summary: String, confidence: f32, suggested_destination: String, raw_data: String) -> Self {
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at,
            source,
            candidate_type,
            title,
            summary,
            confidence,
            suggested_destination,
            status: CandidateStatus::Pending,
            redaction_status: RedactionStatus::NotNeeded,
            raw_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub event_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMemory {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCandidate {
    pub id: String,
    pub skill_name: String,
    pub description: String,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCandidate {
    pub id: String,
    pub workflow_name: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSummary {
    pub total_candidates: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub pending: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningDecision {
    Accept,
    Reject,
    Edit(String),
}

pub struct BrainLearningManager {
    pub candidates: Vec<MemoryCandidate>,
}

impl BrainLearningManager {
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }

    pub fn get_candidates(&self) -> Vec<MemoryCandidate> {
        self.candidates.clone()
    }

    pub fn add_candidate(&mut self, candidate: MemoryCandidate) {
        self.candidates.push(candidate);
    }

    pub fn get_candidate(&self, id: &str) -> Option<MemoryCandidate> {
        self.candidates.iter().find(|c| c.id == id).cloned()
    }

    pub fn handle_decision(&mut self, candidate_id: &str, decision: LearningDecision) -> Result<(), String> {
        let mut idx = None;
        for (i, c) in self.candidates.iter().enumerate() {
            if c.id == candidate_id {
                idx = Some(i);
                break;
            }
        }
        let idx = idx.ok_or_else(|| "Candidate not found".to_string())?;

        match decision {
            LearningDecision::Accept => {
                self.candidates[idx].status = CandidateStatus::Accepted;
                let c = self.candidates[idx].clone();
                self.write_to_destination(&c)?;
            }
            LearningDecision::Reject => {
                self.candidates[idx].status = CandidateStatus::Rejected;
            }
            LearningDecision::Edit(new_summary) => {
                self.candidates[idx].summary = new_summary;
                self.candidates[idx].status = CandidateStatus::Accepted;
                let c = self.candidates[idx].clone();
                self.write_to_destination(&c)?;
            }
        }
        Ok(())
    }

    fn write_to_destination(&self, candidate: &MemoryCandidate) -> Result<(), String> {
        // Safe write logic to the suggested_destination.
        // In reality, this would safely write to DB or filesystem, ensuring no secrets are leaked.
        if candidate.redaction_status == RedactionStatus::Unredacted {
            // Refuse to write if it hasn't passed redaction verification
            return Err("Cannot write unredacted candidate".to_string());
        }
        // Mocking safe write
        println!("Safely wrote candidate {} to {}", candidate.id, candidate.suggested_destination);
        Ok(())
    }

    pub fn get_summary(&self) -> LearningSummary {
        let mut total = 0;
        let mut accepted = 0;
        let mut rejected = 0;
        let mut pending = 0;

        for c in &self.candidates {
            total += 1;
            match c.status {
                CandidateStatus::Accepted => accepted += 1,
                CandidateStatus::Rejected => rejected += 1,
                CandidateStatus::Pending => pending += 1,
                CandidateStatus::Archived => {}
            }
        }

        LearningSummary {
            total_candidates: total,
            accepted,
            rejected,
            pending,
        }
    }
}

impl Default for BrainLearningManager {
    fn default() -> Self {
        Self::new()
    }
}
