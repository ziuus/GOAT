use anyhow::{Result, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

use crate::config::BrainIndexConfig;
use crate::embeddings::{EmbeddingProvider, EmbeddingVector, create_provider};
use crate::paths::GoatPaths;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrainDocumentKind {
    Memory,
    MemoryCandidate,
    Skill,
    SkillProvenance,
    Recipe,
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
    PromptForgeTemplate,
    PromptForgeHistory,
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
pub struct BrainDocument {
    pub id: String,
    pub kind: BrainDocumentKind,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tags: Vec<String>,
    pub source_path: Option<String>,
    pub project_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSearchResult {
    pub document: BrainDocument,
    pub score: f32,
    pub keyword_score: f32,
    pub semantic_score: f32,
    pub match_reason: String,
}

pub struct BrainIndexManager {
    paths: GoatPaths,
    config: BrainIndexConfig,
    index_file: PathBuf,
    vector_file: PathBuf,
    provider: EmbeddingProvider,
}

impl BrainIndexManager {
    pub fn new(
        paths: GoatPaths,
        config: BrainIndexConfig,
        embeddings_config: &crate::config::EmbeddingsConfig,
    ) -> Self {
        let index_file = paths.brain_index_dir.join("index.jsonl");
        let vector_file = paths.brain_index_dir.join("vectors.jsonl");
        let provider = create_provider(embeddings_config);
        Self {
            paths,
            config,
            index_file,
            vector_file,
            provider,
        }
    }

    pub fn status(&self) -> Result<BrainIndexStats> {
        let docs_count = if self.index_file.exists() {
            let content = fs::read_to_string(&self.index_file).unwrap_or_default();
            content.lines().filter(|l| !l.trim().is_empty()).count()
        } else {
            0
        };

        let storage_size = if self.index_file.exists() {
            fs::metadata(&self.index_file).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        let vec_count = if self.vector_file.exists() {
            let content = fs::read_to_string(&self.vector_file).unwrap_or_default();
            content.lines().filter(|l| !l.trim().is_empty()).count()
        } else {
            0
        };

        Ok(BrainIndexStats {
            total_documents: docs_count,
            last_indexed_at: Some(Utc::now().to_rfc3339()),
            storage_size_bytes: storage_size,
            total_vectors: vec_count,
            embedding_provider: format!("{:?}", self.provider.kind()),
        })
    }

    pub async fn index_all(&self) -> Result<()> {
        if !self.config.enabled {
            return Err(anyhow!("Brain index is disabled in config"));
        }

        if !self.paths.brain_index_dir.exists() {
            fs::create_dir_all(&self.paths.brain_index_dir)?;
        }

        let mut docs = Vec::new();

        self.ingest_skills(&mut docs);
        self.ingest_memory(&mut docs);
        self.ingest_promptforge(&mut docs);

        if self.config.deep_ingestion {
            if self.config.index_recipes {
                self.ingest_dir(
                    &self.paths.data_dir.join("recipes").join("installed"),
                    BrainDocumentKind::Recipe,
                    "recipe.toml",
                    "Recipe",
                    TrustLevel::Installed,
                    &mut docs,
                );
            }
            if self.config.index_studio_drafts {
                self.ingest_dir(
                    &self.paths.data_dir.join("studio"),
                    BrainDocumentKind::StudioDraft,
                    "draft.json",
                    "Studio Draft",
                    TrustLevel::GeneratedDraft,
                    &mut docs,
                );
            }
            if self.config.index_jobs {
                self.ingest_dir(
                    &self.paths.data_dir.join("jobs"),
                    BrainDocumentKind::Job,
                    "job.json",
                    "Job Run",
                    TrustLevel::TrustedLocal,
                    &mut docs,
                );
            }
            if self.config.index_approvals {
                self.ingest_dir(
                    &self.paths.data_dir.join("approvals"),
                    BrainDocumentKind::Approval,
                    "history.json",
                    "Approval History",
                    TrustLevel::AuditOnly,
                    &mut docs,
                );
            }
            if self.config.index_audit_logs {
                self.ingest_dir(
                    &self.paths.data_dir.join("audit"),
                    BrainDocumentKind::AuditLog,
                    "log.jsonl",
                    "Audit Log",
                    TrustLevel::AuditOnly,
                    &mut docs,
                );
            }
            if self.config.index_checkpoints {
                self.ingest_dir(
                    &self.paths.data_dir.join("checkpoints"),
                    BrainDocumentKind::Checkpoint,
                    "metadata.json",
                    "Checkpoint",
                    TrustLevel::TrustedLocal,
                    &mut docs,
                );
            }
        }

        // Save index
        let mut out = String::new();
        for doc in &docs {
            if let Ok(line) = serde_json::to_string(doc) {
                out.push_str(&line);
                out.push('\n');
            }
        }
        fs::write(&self.index_file, out)?;

        // Update embeddings
        if self.provider.kind() != crate::embeddings::EmbeddingProviderKind::None {
            self.rebuild_embeddings(&docs).await?;
        }

        info!("Brain indexing completed.");
        Ok(())
    }

    async fn rebuild_embeddings(&self, docs: &[BrainDocument]) -> Result<()> {
        let mut existing_vectors = HashMap::new();
        if self.vector_file.exists() {
            if let Ok(content) = fs::read_to_string(&self.vector_file) {
                for line in content.lines().filter(|l| !l.trim().is_empty()) {
                    if let Ok(vec) = serde_json::from_str::<EmbeddingVector>(line) {
                        existing_vectors.insert(vec.document_id.clone(), vec);
                    }
                }
            }
        }

        let mut out_vectors = Vec::new();

        for doc in docs {
            let combined_text = format!(
                "{} {} {} {}",
                doc.title,
                doc.summary,
                doc.body,
                doc.tags.join(" ")
            );
            let hash_val = self.hash_text(&combined_text);

            if let Some(existing) = existing_vectors.get(&doc.id) {
                if existing.content_hash == hash_val {
                    out_vectors.push(existing.clone());
                    continue;
                }
            }

            match self.provider.generate(&combined_text).await {
                Ok(vec) => {
                    out_vectors.push(EmbeddingVector {
                        document_id: doc.id.clone(),
                        provider: format!("{:?}", self.provider.kind()),
                        model: "model".to_string(),
                        dimensions: self.provider.dimensions(),
                        vector: vec,
                        created_at: Utc::now().to_rfc3339(),
                        content_hash: hash_val,
                    });
                }
                Err(e) => {
                    warn!("Failed to embed doc {}: {}", doc.id, e);
                }
            }
        }

        let mut out = String::new();
        for vec in &out_vectors {
            if let Ok(line) = serde_json::to_string(vec) {
                out.push_str(&line);
                out.push('\n');
            }
        }
        fs::write(&self.vector_file, out)?;

        Ok(())
    }

    fn hash_text(&self, text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }


    fn ingest_promptforge(&self, docs: &mut Vec<BrainDocument>) {
        let pf_history_file = self.paths.data_dir.join("promptforge").join("history.jsonl");
        if pf_history_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&pf_history_file) {
                if !self.contains_secrets(&content) {
                    docs.push(BrainDocument {
                        id: "global_promptforge_history".to_string(),
                        kind: BrainDocumentKind::PromptForgeHistory,
                        title: "PromptForge History".to_string(),
                        summary: "Global system PromptForge history".to_string(),
                        body: self.truncate(&content),
                        tags: vec!["promptforge".to_string(), "history".to_string()],
                        source_path: Some(pf_history_file.to_string_lossy().to_string()),
                        project_id: None,
                        created_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                        redaction_status: "clean".to_string(),
                        trust_level: TrustLevel::TrustedLocal,
                    });
                }
            }
        }
        
        let pf_templates = crate::promptforge::PromptForgeTemplateLibrary::new();
        for tpl in pf_templates.templates {
            let body = format!("Template: {}\nKind: {:?}\nDescription: {}\nStructure:\n{}", tpl.name, tpl.kind, tpl.description, tpl.structure);
            if !self.contains_secrets(&body) {
                docs.push(BrainDocument {
                    id: format!("pf_template_{}", tpl.id),
                    kind: BrainDocumentKind::PromptForgeTemplate,
                    title: format!("PromptForge Template: {}", tpl.name),
                    summary: tpl.description.clone(),
                    body: self.truncate(&body),
                    tags: vec!["promptforge".to_string(), "template".to_string(), format!("{:?}", tpl.kind)],
                    source_path: None,
                    project_id: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                    redaction_status: "clean".to_string(),
                    trust_level: TrustLevel::TrustedLocal,
                });
            }
        }
    }

    fn ingest_skills(&self, docs: &mut Vec<BrainDocument>) {
        if !self.paths.skills_dir.exists() {
            return;
        }
        for entry in walkdir::WalkDir::new(&self.paths.skills_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "SKILL.md" {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if !self.contains_secrets(&content) {
                        docs.push(BrainDocument {
                            id: format!("skill_{}", docs.len()),
                            kind: BrainDocumentKind::Skill,
                            title: "Local Skill".to_string(),
                            summary: "A local skill definition".to_string(),
                            body: self.truncate(&content),
                            tags: vec!["skill".to_string()],
                            source_path: Some(entry.path().to_string_lossy().to_string()),
                            project_id: None,
                            created_at: Utc::now().to_rfc3339(),
                            updated_at: Utc::now().to_rfc3339(),
                            redaction_status: "clean".to_string(),
                            trust_level: TrustLevel::TrustedLocal,
                        });
                    }
                }
            }
        }
    }

    fn ingest_memory(&self, docs: &mut Vec<BrainDocument>) {
        if self.paths.memory_file.exists() {
            if let Ok(content) = fs::read_to_string(&self.paths.memory_file) {
                if !self.contains_secrets(&content) {
                    docs.push(BrainDocument {
                        id: "global_memory".to_string(),
                        kind: BrainDocumentKind::Memory,
                        title: "Global Memory".to_string(),
                        summary: "Global system memory context".to_string(),
                        body: self.truncate(&content),
                        tags: vec!["memory".to_string()],
                        source_path: Some(self.paths.memory_file.to_string_lossy().to_string()),
                        project_id: None,
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                        redaction_status: "clean".to_string(),
                        trust_level: TrustLevel::TrustedLocal,
                    });
                }
            }
        }
    }

    fn ingest_dir(
        &self,
        dir: &Path,
        kind: BrainDocumentKind,
        target_file: &str,
        title_prefix: &str,
        trust_level: TrustLevel,
        docs: &mut Vec<BrainDocument>,
    ) {
        if !dir.exists() {
            return;
        }
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name().to_string_lossy() == target_file {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if !self.contains_secrets(&content) {
                        docs.push(BrainDocument {
                            id: format!("{:?}_{}", kind, docs.len()),
                            kind: kind.clone(),
                            title: format!("{} Source", title_prefix),
                            summary: format!("Indexed content from {}", target_file),
                            body: self.truncate(&content),
                            tags: vec![format!("{:?}", kind).to_lowercase()],
                            source_path: Some(entry.path().to_string_lossy().to_string()),
                            project_id: None,
                            created_at: Utc::now().to_rfc3339(),
                            updated_at: Utc::now().to_rfc3339(),
                            redaction_status: "clean".to_string(),
                            trust_level: trust_level.clone(),
                        });
                    }
                }
            }
        }
    }

    pub async fn search(&self, query: &BrainSearchQuery) -> Result<Vec<BrainSearchResult>> {
        if !self.config.enabled || !self.index_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.index_file)?;
        let mut all_docs = Vec::new();
        let q = query.q.to_lowercase();

        for line in content.lines().filter(|l| !l.trim().is_empty()) {
            if let Ok(doc) = serde_json::from_str::<BrainDocument>(line) {
                if let Some(filters) = &query.kind_filter {
                    if !filters.contains(&doc.kind) {
                        continue;
                    }
                }
                all_docs.push(doc);
            }
        }

        let mut results = Vec::new();

        let mut query_vec = None;
        if (query.mode == BrainSearchMode::Semantic || query.mode == BrainSearchMode::Hybrid)
            && self.provider.kind() != crate::embeddings::EmbeddingProviderKind::None
        {
            if let Ok(v) = self.provider.generate(&query.q).await {
                query_vec = Some(v);
            }
        }

        let mut vectors_map = HashMap::new();
        if query_vec.is_some() && self.vector_file.exists() {
            if let Ok(vc) = fs::read_to_string(&self.vector_file) {
                for line in vc.lines().filter(|l| !l.trim().is_empty()) {
                    if let Ok(vec) = serde_json::from_str::<EmbeddingVector>(line) {
                        vectors_map.insert(vec.document_id.clone(), vec.vector);
                    }
                }
            }
        }

        for doc in all_docs {
            let mut keyword_score = 0.0;
            let mut reasons = Vec::new();

            if doc.title.to_lowercase().contains(&q) {
                keyword_score += 10.0;
                reasons.push("Title match");
            }
            if doc.summary.to_lowercase().contains(&q) {
                keyword_score += 5.0;
                reasons.push("Summary match");
            }
            if doc.body.to_lowercase().contains(&q) {
                keyword_score += 2.0;
                reasons.push("Body match");
            }
            if doc.tags.iter().any(|t| t.to_lowercase() == q) {
                keyword_score += 8.0;
                reasons.push("Tag match");
            }

            let mut semantic_score = 0.0;
            if let Some(qv) = &query_vec {
                if let Some(dv) = vectors_map.get(&doc.id) {
                    semantic_score = self.cosine_similarity(qv, dv);
                    if semantic_score > 0.5 {
                        reasons.push("Semantic match");
                    }
                }
            }

            let score = match query.mode {
                BrainSearchMode::Keyword | BrainSearchMode::Fuzzy => keyword_score,
                BrainSearchMode::Semantic => {
                    if query_vec.is_none() {
                        keyword_score
                    } else {
                        semantic_score * 100.0
                    }
                }
                BrainSearchMode::Hybrid => {
                    if query_vec.is_none() {
                        keyword_score
                    } else {
                        keyword_score + (semantic_score * 50.0)
                    }
                }
            };

            if score > 0.0 || semantic_score > 0.3 {
                results.push(BrainSearchResult {
                    document: doc,
                    score,
                    keyword_score,
                    semantic_score,
                    match_reason: reasons.join(", "),
                });
            }
        }

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(query.limit);

        Ok(results)
    }

    pub async fn recall(&self, q: &str, mode: BrainSearchMode) -> Result<serde_json::Value> {
        let results = self
            .search(&BrainSearchQuery {
                q: q.to_string(),
                limit: 20,
                kind_filter: None,
                mode,
            })
            .await?;

        let mut groups = serde_json::Map::new();
        for r in results {
            let kind_str = format!("{:?}", r.document.kind);
            let arr = groups
                .entry(kind_str)
                .or_insert(serde_json::Value::Array(Vec::new()));
            if let serde_json::Value::Array(list) = arr {
                list.push(serde_json::to_value(r.document).unwrap());
            }
        }

        Ok(serde_json::Value::Object(groups))
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|y| y * y).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    fn contains_secrets(&self, content: &str) -> bool {
        let lower = content.to_lowercase();
        // Naive fast redaction logic
        lower.contains("api_key")
            || lower.contains("secret")
            || lower.contains("password")
            || lower.contains("-----begin private key-----")
            || lower.contains("bearer ")
            || lower.contains("sk-")
    }

    fn truncate(&self, content: &str) -> String {
        if content.len() > self.config.max_document_chars {
            format!(
                "{}... [TRUNCATED]",
                &content[..self.config.max_document_chars]
            )
        } else {
            content.to_string()
        }
    }
}
