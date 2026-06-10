use anyhow::{Result, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

use crate::config::BrainIndexConfig;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSearchQuery {
    pub q: String,
    pub limit: usize,
    pub kind_filter: Option<Vec<BrainDocumentKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSearchResult {
    pub document: BrainDocument,
    pub score: f32,
    pub match_reason: String,
}

pub struct BrainIndexManager {
    paths: GoatPaths,
    config: BrainIndexConfig,
    index_file: PathBuf,
}

impl BrainIndexManager {
    pub fn new(paths: GoatPaths, config: BrainIndexConfig) -> Self {
        let index_file = paths.brain_index_dir.join("index.jsonl");
        Self {
            paths,
            config,
            index_file,
        }
    }

    pub fn status(&self) -> Result<BrainIndexStats> {
        if !self.config.enabled {
            return Ok(BrainIndexStats {
                total_documents: 0,
                last_indexed_at: None,
                storage_size_bytes: 0,
            });
        }

        if !self.index_file.exists() {
            return Ok(BrainIndexStats {
                total_documents: 0,
                last_indexed_at: None,
                storage_size_bytes: 0,
            });
        }

        let meta = match fs::metadata(&self.index_file) {
            Ok(m) => m,
            Err(_) => return Err(anyhow!("Failed to read index metadata")),
        };

        let content = fs::read_to_string(&self.index_file).unwrap_or_default();
        let docs_count = content.lines().filter(|l| !l.trim().is_empty()).count();

        Ok(BrainIndexStats {
            total_documents: docs_count,
            last_indexed_at: Some(Utc::now().to_rfc3339()), // Mock for now
            storage_size_bytes: meta.len(),
        })
    }

    pub fn index_all(&self) -> Result<()> {
        if !self.config.enabled {
            return Err(anyhow!("Brain index is disabled in config"));
        }

        if !self.paths.brain_index_dir.exists() {
            fs::create_dir_all(&self.paths.brain_index_dir)?;
        }

        let mut docs = Vec::new();

        // 1. Index skills
        if self.config.index_skills && self.paths.skills_dir.exists() {
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

        // 2. Index memory
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

        // Write docs to jsonl
        let mut out = String::new();
        for doc in docs {
            if let Ok(line) = serde_json::to_string(&doc) {
                out.push_str(&line);
                out.push('\n');
            }
        }
        fs::write(&self.index_file, out)?;

        info!("Brain indexing completed.");
        Ok(())
    }

    pub fn search(&self, query: &BrainSearchQuery) -> Result<Vec<BrainSearchResult>> {
        if !self.config.enabled || !self.index_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.index_file)?;
        let mut results = Vec::new();
        let q = query.q.to_lowercase();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(doc) = serde_json::from_str::<BrainDocument>(line) {
                if let Some(filters) = &query.kind_filter {
                    if !filters.contains(&doc.kind) {
                        continue;
                    }
                }

                let mut score = 0.0;
                let mut reasons = Vec::new();

                if doc.title.to_lowercase().contains(&q) {
                    score += 10.0;
                    reasons.push("Title match");
                }
                if doc.summary.to_lowercase().contains(&q) {
                    score += 5.0;
                    reasons.push("Summary match");
                }
                if doc.body.to_lowercase().contains(&q) {
                    score += 2.0;
                    reasons.push("Body match");
                }
                if doc.tags.iter().any(|t| t.to_lowercase() == q) {
                    score += 8.0;
                    reasons.push("Tag match");
                }

                if score > 0.0 {
                    results.push(BrainSearchResult {
                        document: doc,
                        score,
                        match_reason: reasons.join(", "),
                    });
                }
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

    pub fn recall(&self, q: &str) -> Result<serde_json::Value> {
        let results = self.search(&BrainSearchQuery {
            q: q.to_string(),
            limit: 20,
            kind_filter: None,
        })?;

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

    fn contains_secrets(&self, content: &str) -> bool {
        let lower = content.to_lowercase();
        // Naive fast redaction logic
        lower.contains("api_key")
            || lower.contains("secret")
            || lower.contains("password")
            || lower.contains("-----begin private key-----")
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
