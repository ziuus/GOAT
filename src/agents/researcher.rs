use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use crate::paths::GoatPaths;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResearchWorkflowState {
    New,
    Planning,
    Sourcing,
    Analyzing,
    Drafting,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchSourceKind {
    OfficialDocs,
    ResearchPaper,
    CompanyPage,
    PricingPage,
    BlogPost,
    NewsArticle,
    ForumThread,
    SocialPost,
    GithubRepo,
    DocsPage,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQuestion {
    pub main_question: String,
    pub subquestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPlan {
    pub question: ResearchQuestion,
    pub scope: String,
    pub source_types_needed: Vec<ResearchSourceKind>,
    pub source_priority: Vec<String>,
    pub search_keywords: Vec<String>,
    pub comparison_criteria: Vec<String>,
    pub known_assumptions: Vec<String>,
    pub uncertainty_areas: Vec<String>,
    pub expected_output_format: String,
    pub handoff_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTopic {
    pub id: String,
    pub title: String,
    pub research_question: String,
    pub domain: String,
    pub scope: String,
    pub constraints: Vec<String>,
    pub source_requirements: Vec<String>,
    pub status: ResearchWorkflowState,
    pub created_at: u64,
    pub updated_at: u64,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
    pub linked_project: Option<String>,
    pub linked_cofounder_idea: Option<String>,
    pub linked_socializer_campaign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub source_type: ResearchSourceKind,
    pub publisher: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub retrieved_at: Option<u64>,
    pub credibility_notes: String,
    pub relevance_score: u8,
    pub summary: String,
    pub key_claims: Vec<String>,
    pub limitations: Vec<String>,
    pub citation_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchEvidenceNote {
    pub id: String,
    pub claim: String,
    pub source_refs: Vec<String>,
    pub confidence: String, // "low", "medium", "high"
    pub supporting_evidence: Vec<String>,
    pub contradicting_evidence: Vec<String>,
    pub uncertainty: String,
    pub implications: Vec<String>,
    pub next_verification_step: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchClaim {
    pub text: String,
    pub source_id: Option<String>,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchUncertainty {
    pub description: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchCompetitorScan {
    pub id: String,
    pub competitor_name: String,
    pub url: Option<String>,
    pub positioning: String,
    pub target_users: Vec<String>,
    pub pricing_notes: String,
    pub core_features: Vec<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub distribution_channels: Vec<String>,
    pub trust_signals: Vec<String>,
    pub gaps: Vec<String>,
    pub threat_level: String,
    pub evidence_confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTechnologyComparison {
    pub id: String,
    pub options: Vec<String>,
    pub use_cases: Vec<String>,
    pub criteria: Vec<String>,
    pub pros_cons: HashMap<String, Vec<String>>,
    pub ecosystem_maturity: HashMap<String, String>,
    pub learning_curve: HashMap<String, String>,
    pub cost: HashMap<String, String>,
    pub deployment_complexity: HashMap<String, String>,
    pub security_privacy: HashMap<String, String>,
    pub compatibility: HashMap<String, String>,
    pub recommendation: String,
    pub uncertainty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchComparison {
    pub id: String,
    pub title: String,
    pub competitors: Option<Vec<ResearchCompetitorScan>>,
    pub technologies: Option<ResearchTechnologyComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchBrief {
    pub id: String,
    pub topic_id: String,
    pub executive_summary: String,
    pub research_question: String,
    pub scope: String,
    pub key_findings: Vec<String>,
    pub evidence_table: Vec<ResearchEvidenceNote>,
    pub source_list: Vec<ResearchSource>,
    pub disagreements: Vec<String>,
    pub assumptions: Vec<String>,
    pub risks: Vec<String>,
    pub implications: Vec<String>,
    pub recommendations: Vec<String>,
    pub next_research_steps: Vec<String>,
    pub handoff_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub id: String,
    pub topic_id: String,
    pub content: String,
}

pub struct ResearcherAgent {
    pub topics_dir: PathBuf,
}

impl Default for ResearcherAgent {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self { topics_dir: PathBuf::new() })
    }
}

impl ResearcherAgent {
    pub fn new() -> Result<Self> {
        let paths = GoatPaths::resolve()?;
        let topics_dir = paths.data_dir.join("agents/prime/researcher");
        if !topics_dir.exists() {
            fs::create_dir_all(&topics_dir)?;
        }
        Ok(Self { topics_dir })
    }

    fn topics_file(&self) -> PathBuf {
        self.topics_dir.join("topics.jsonl")
    }

    fn plans_file(&self) -> PathBuf {
        self.topics_dir.join("plans.jsonl")
    }

    fn sources_file(&self) -> PathBuf {
        self.topics_dir.join("sources.jsonl")
    }

    fn evidence_notes_file(&self) -> PathBuf {
        self.topics_dir.join("evidence_notes.jsonl")
    }

    pub fn list_topics(&self) -> Result<Vec<ResearchTopic>> {
        let path = self.topics_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let mut topics = Vec::new();
        for line in content.lines() {
            if let Ok(t) = serde_json::from_str::<ResearchTopic>(line) {
                topics.push(t);
            }
        }
        Ok(topics)
    }

    pub fn create_topic(&self, title: &str, question: &str, domain: &str) -> Result<ResearchTopic> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let topic = ResearchTopic {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            research_question: question.to_string(),
            domain: domain.to_string(),
            scope: "General".to_string(),
            constraints: vec![],
            source_requirements: vec![],
            status: ResearchWorkflowState::New,
            created_at: now,
            updated_at: now,
            timeline_refs: vec![],
            brain_refs: vec![],
            linked_project: None,
            linked_cofounder_idea: None,
            linked_socializer_campaign: None,
        };

        let json = serde_json::to_string(&topic)?;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.topics_file())?;
        use std::io::Write;
        writeln!(file, "{}", json)?;

        Ok(topic)
    }

    pub fn get_topic(&self, id: &str) -> Result<Option<ResearchTopic>> {
        let topics = self.list_topics()?;
        Ok(topics.into_iter().find(|t| t.id == id))
    }

    pub fn create_plan(&self, topic_id: &str) -> Result<ResearchPlan> {
        let plan = ResearchPlan {
            question: ResearchQuestion {
                main_question: "Generated Main Question".to_string(),
                subquestions: vec!["Generated Subquestion 1".to_string()],
            },
            scope: "Generated Scope".to_string(),
            source_types_needed: vec![ResearchSourceKind::OfficialDocs],
            source_priority: vec!["Primary".to_string()],
            search_keywords: vec!["keyword".to_string()],
            comparison_criteria: vec!["criteria".to_string()],
            known_assumptions: vec!["assumption".to_string()],
            uncertainty_areas: vec!["uncertainty".to_string()],
            expected_output_format: "Markdown".to_string(),
            handoff_target: None,
        };

        let mut map: HashMap<String, ResearchPlan> = HashMap::new();
        if self.plans_file().exists() {
            let content = fs::read_to_string(self.plans_file())?;
            for line in content.lines() {
                if let Ok((tid, p)) = serde_json::from_str::<(String, ResearchPlan)>(line) {
                    map.insert(tid, p);
                }
            }
        }
        map.insert(topic_id.to_string(), plan.clone());

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.plans_file())?;
        use std::io::Write;
        for (k, v) in map {
            writeln!(file, "{}", serde_json::to_string(&(k, v))?)?;
        }

        Ok(plan)
    }

    pub fn list_sources(&self, topic_id: &str) -> Result<Vec<ResearchSource>> {
        let path = self.sources_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let mut sources = Vec::new();
        for line in content.lines() {
            if let Ok((tid, s)) = serde_json::from_str::<(String, ResearchSource)>(line) {
                if tid == topic_id {
                    sources.push(s);
                }
            }
        }
        Ok(sources)
    }

    pub fn add_source(&self, topic_id: &str, title: &str) -> Result<ResearchSource> {
        let source = ResearchSource {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            url: None,
            source_type: ResearchSourceKind::Unknown,
            publisher: "Unknown".to_string(),
            author: None,
            date: None,
            retrieved_at: None,
            credibility_notes: "".to_string(),
            relevance_score: 5,
            summary: "Generated summary".to_string(),
            key_claims: vec![],
            limitations: vec![],
            citation_text: None,
        };

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.sources_file())?;
        use std::io::Write;
        writeln!(file, "{}", serde_json::to_string(&(topic_id.to_string(), source.clone()))?)?;

        Ok(source)
    }

    pub fn list_notes(&self, topic_id: &str) -> Result<Vec<ResearchEvidenceNote>> {
        let path = self.evidence_notes_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let mut notes = Vec::new();
        for line in content.lines() {
            if let Ok((tid, n)) = serde_json::from_str::<(String, ResearchEvidenceNote)>(line) {
                if tid == topic_id {
                    notes.push(n);
                }
            }
        }
        Ok(notes)
    }

    pub fn add_note(&self, topic_id: &str, claim: &str) -> Result<ResearchEvidenceNote> {
        let note = ResearchEvidenceNote {
            id: Uuid::new_v4().to_string(),
            claim: claim.to_string(),
            source_refs: vec![],
            confidence: "medium".to_string(),
            supporting_evidence: vec![],
            contradicting_evidence: vec![],
            uncertainty: "Generated uncertainty".to_string(),
            implications: vec![],
            next_verification_step: None,
        };

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.evidence_notes_file())?;
        use std::io::Write;
        writeln!(file, "{}", serde_json::to_string(&(topic_id.to_string(), note.clone()))?)?;

        Ok(note)
    }

    pub fn generate_competitors(&self, _topic_id: &str) -> Result<Vec<ResearchCompetitorScan>> {
        Ok(vec![ResearchCompetitorScan {
            id: Uuid::new_v4().to_string(),
            competitor_name: "Competitor A".to_string(),
            url: None,
            positioning: "Unknown".to_string(),
            target_users: vec![],
            pricing_notes: "Unknown".to_string(),
            core_features: vec![],
            strengths: vec![],
            weaknesses: vec![],
            distribution_channels: vec![],
            trust_signals: vec![],
            gaps: vec![],
            threat_level: "low".to_string(),
            evidence_confidence: "low".to_string(),
        }])
    }

    pub fn generate_compare(&self, _topic_id: &str) -> Result<ResearchTechnologyComparison> {
        Ok(ResearchTechnologyComparison {
            id: Uuid::new_v4().to_string(),
            options: vec!["Option A".to_string(), "Option B".to_string()],
            use_cases: vec![],
            criteria: vec![],
            pros_cons: HashMap::new(),
            ecosystem_maturity: HashMap::new(),
            learning_curve: HashMap::new(),
            cost: HashMap::new(),
            deployment_complexity: HashMap::new(),
            security_privacy: HashMap::new(),
            compatibility: HashMap::new(),
            recommendation: "Option A based on assumptions".to_string(),
            uncertainty: "High".to_string(),
        })
    }

    pub fn generate_market(&self, _topic_id: &str) -> Result<String> {
        Ok("Market trends generated.".to_string())
    }

    pub fn generate_brief(&self, topic_id: &str) -> Result<ResearchBrief> {
        Ok(ResearchBrief {
            id: Uuid::new_v4().to_string(),
            topic_id: topic_id.to_string(),
            executive_summary: "Generated brief summary.".to_string(),
            research_question: "Original question".to_string(),
            scope: "General".to_string(),
            key_findings: vec![],
            evidence_table: vec![],
            source_list: vec![],
            disagreements: vec![],
            assumptions: vec![],
            risks: vec![],
            implications: vec![],
            recommendations: vec![],
            next_research_steps: vec![],
            handoff_notes: None,
        })
    }

    pub fn generate_report(&self, topic_id: &str) -> Result<ResearchReport> {
        Ok(ResearchReport {
            id: Uuid::new_v4().to_string(),
            topic_id: topic_id.to_string(),
            content: "Generated report content.".to_string(),
        })
    }
}
