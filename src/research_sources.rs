use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchSourceStatus {
    Pending,
    Ingested,
    Graded,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchSourceTrustLevel {
    Official,
    Primary,
    Secondary,
    Community,
    Unverified,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchSourceKind {
    Webpage,
    BrowserArtifact,
    Report,
    Paper,
    Documentation,
    GithubRepo,
    ApiDoc,
    CompanyPage,
    NewsArticle,
    BlogPost,
    ForumPost,
    SocialPost,
    Dataset,
    LocalFile,
    UserNote,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSourceMetadata {
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSourceProvenance {
    pub collected_by: String, // e.g. "agent:researcher"
    pub original_url: Option<String>,
    pub extraction_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSourceExcerpt {
    pub id: String,
    pub text: String,
    pub location_in_source: Option<String>, // e.g., "paragraph 2", "section 1.1"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub kind: ResearchSourceKind,
    pub status: ResearchSourceStatus,
    pub trust_level: ResearchSourceTrustLevel,
    pub quality_score: u8, // 0-100
    pub summary: String,
    pub notes: String,
    pub metadata: ResearchSourceMetadata,
    pub provenance: ResearchSourceProvenance,
    pub excerpts: Vec<ResearchSourceExcerpt>,
    pub tags: Vec<String>,
    pub captured_at: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub source_hash: String,
    pub browser_artifact_refs: Vec<String>,
    pub brain_refs: Vec<String>,
    pub timeline_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStrength {
    Strong,
    Moderate,
    Weak,
    Anecdotal,
    Unsupported,
    Conflicting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceGradeResult {
    pub strength: EvidenceStrength,
    pub score: u8,
    pub reasoning: String,
    pub independent_source_count: usize,
    pub conflict_flag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimEvidence {
    pub source_id: String,
    pub excerpt_id: Option<String>,
    pub context: String,
    pub grade: EvidenceGradeResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub statement: String,
    pub is_assumption: bool,
    pub evidence: Vec<ClaimEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationRef {
    pub id: String,
    pub claim_id: String,
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    pub id: String,
    pub name: String,
    pub question: String,
    pub scope: String,
    pub project_type: String, // market_research, competitor_research, etc.
    pub sources: Vec<String>, // source IDs
    pub claims: Vec<String>,  // claim IDs
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorProfile {
    pub id: String,
    pub name: String,
    pub website_url: Option<String>,
    pub positioning: String,
    pub core_features: Vec<String>,
    pub pricing_notes: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub evidence_refs: Vec<String>, // Claim IDs or Source IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyOption {
    pub name: String,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyComparisonMatrix {
    pub id: String,
    pub criteria: Vec<String>,
    pub options: Vec<TechnologyOption>,
    pub recommendation: String,
    pub uncertainty_notes: String,
}

pub struct ResearchSourceManager {
    base_dir: std::path::PathBuf,
}

impl ResearchSourceManager {
    pub fn new(data_dir: &std::path::Path) -> Self {
        let base_dir = data_dir.join("agents/prime/researcher");
        let _ = std::fs::create_dir_all(&base_dir);
        Self { base_dir }
    }

    pub fn save_project(&self, project: &ResearchProject) -> anyhow::Result<()> {
        let path = self.base_dir.join("projects.jsonl");
        let content = serde_json::to_string(project)? + "\n";
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn save_source(&self, source: &ResearchSource) -> anyhow::Result<()> {
        let path = self.base_dir.join("sources.jsonl");
        let content = serde_json::to_string(source)? + "\n";
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(content.as_bytes())?;
        Ok(())
    }
}

// Dummy methods to simulate functionality
pub struct EvidenceGrader;

impl EvidenceGrader {
    pub fn grade_source(_source: &ResearchSource) -> EvidenceGradeResult {
        EvidenceGradeResult {
            strength: EvidenceStrength::Moderate,
            score: 70,
            reasoning: "Automated grading not fully implemented; defaulted to moderate".into(),
            independent_source_count: 1,
            conflict_flag: false,
        }
    }
}
