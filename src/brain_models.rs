use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrainDocumentKind {
    Conversation,
    TimelineEvent,
    Report,
    RuntimeJob,
    RuntimeArtifact,
    CollaborationSession,
    CollaborationHandoff,
    PromptForgeHistory,
    PromptForgeTemplate,
    CofounderIdea,
    LearnerGoal,
    LearnerRoadmap,
    ResearcherBrief,
    OperatorReport,
    DesignerReview,
    File,
    Skill,
    Recipe,
    Memory,
    Unknown,

    // Existing kinds for backward compatibility
    MemoryCandidate,
    SkillProvenance,
    RecipeRun,
    AgentTemplate,
    StudioDraft,
    Job,
    Approval,
    AuditLog,
    SessionSummary,
    ProjectSummary,
    Checkpoint,
    McpTool,
    ExternalAgentRun,
    CommandHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    TrustedLocal,
    Installed,
    LearnedPending,
    RemoteUntrusted,
    GeneratedDraft,
    AuditOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSourceRef {
    pub source_kind: BrainDocumentKind,
    pub source_id: String,
    pub source_path: Option<String>,
    pub source_title: String,
    pub source_agent: Option<String>,
    pub source_project: Option<String>,
    pub content_hash: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub timeline_refs: Vec<String>,
    #[serde(default)]
    pub report_refs: Vec<String>,
    #[serde(default)]
    pub runtime_refs: Vec<String>,
    #[serde(default)]
    pub collaboration_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainDocument {
    pub id: String,
    pub kind: BrainDocumentKind,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tags: Vec<String>,
    pub source: BrainSourceRef,
    pub redaction_status: String,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainIndexStats {
    pub total_documents: usize,
    pub last_indexed_at: Option<String>,
    pub storage_size_bytes: u64,
    pub total_vectors: usize,
    pub embedding_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrainSearchMode {
    Keyword,
    Fuzzy,
    Semantic,
    Hybrid,
    Recent,
    Agent,
    Project,
}

impl Default for BrainSearchMode {
    fn default() -> Self {
        BrainSearchMode::Keyword
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSearchQuery {
    pub q: String,
    pub limit: usize,
    pub kind_filter: Option<Vec<BrainDocumentKind>>,
    #[serde(default)]
    pub mode: BrainSearchMode,
    pub agent_id: Option<String>,
    pub project_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSearchResult {
    pub document: BrainDocument,
    pub score: f32,
    pub keyword_score: f32,
    pub fuzzy_score: f32,
    pub semantic_score: f32,
    pub match_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainRecallTrace {
    pub query: String,
    pub mode: BrainSearchMode,
    pub returned_results: usize,
    pub top_score: f32,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainContextPack {
    pub title: String,
    pub items: Vec<BrainDocument>,
    pub source_refs: Vec<String>,
    pub summary: String,
    pub warnings: Vec<String>,
    pub estimated_size: usize,
    pub recall_trace: Vec<BrainRecallTrace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainDedupKey {
    pub content_hash: String,
    pub title: String,
    pub kind: BrainDocumentKind,
}

// Ensure old fields can be deserialized and mapped (via serde features or manually if needed, but we'll try to just migrate new data gracefully).
