use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderMemorySourceRef {
    pub session_id: String,
    pub validation_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFailureSignature {
    pub hash: String,
    pub normalized_message: String,
    pub file_path_pattern: String,
    pub framework: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuilderMemoryConfidence {
    Low,
    Medium,
    High,
    Certain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFixAttempt {
    pub plan_id: String,
    pub fix_hypothesis: String,
    pub patch_intent: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFixOutcome {
    pub attempt: BuilderFixAttempt,
    pub success: bool,
    pub validation_result_summary: String,
    pub rollback_occurred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFixLesson {
    pub id: String,
    pub failure_signature_hash: String,
    pub successful_fix_intent: String,
    pub explanation: String,
    pub confidence: BuilderMemoryConfidence,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFailurePattern {
    pub signature: BuilderFailureSignature,
    pub occurrences: usize,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderRecurringMistake {
    pub pattern: BuilderFailurePattern,
    pub recommended_avoidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderProjectLearning {
    pub common_failure_types: std::collections::HashMap<String, usize>,
    pub most_repeated_files: Vec<String>,
    pub successful_fix_patterns: Vec<String>,
    pub risky_areas: Vec<String>,
    pub avoid_mistakes_list: Vec<String>,
    pub generated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderFailureMemory {
    pub id: String,
    pub signature: BuilderFailureSignature,
    pub original_failure_id: String,
    pub kind: crate::code_retry::ValidationFailureKind,
    pub source_ref: BuilderMemorySourceRef,
    pub outcomes: Vec<BuilderFixOutcome>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct SignatureGenerator;

impl SignatureGenerator {
    pub fn generate(failure: &crate::code_retry::ValidationFailure) -> BuilderFailureSignature {
        let mut msg = failure.evidence.source.normalized_message.clone();

        // Strip line numbers e.g. "at line 10" -> "at line X"
        // For Rust: `error[E0432]: ...`
        // We can just take the first line and remove numbers and quotes
        let first_line = msg.lines().next().unwrap_or("").to_string();
        let normalized: String = first_line
            .chars()
            .filter(|c| !c.is_numeric() && *c != '\'' && *c != '"')
            .collect();

        let file_pattern = if let Some(ref path) = failure.location.file_path {
            let path_obj = std::path::Path::new(path);
            path_obj
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        } else {
            "unknown".to_string()
        };

        let framework = format!("{:?}", failure.kind);
        let raw_hash = format!("{}:{}:{}", framework, file_pattern, normalized);

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        raw_hash.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());

        BuilderFailureSignature {
            hash,
            normalized_message: normalized,
            file_path_pattern: file_pattern,
            framework,
        }
    }
}

pub struct FailureMemoryManager {
    base_dir: std::path::PathBuf,
}

impl FailureMemoryManager {
    pub fn new(data_dir: &std::path::Path) -> Self {
        let base_dir = data_dir.join("builder_memory");
        let _ = std::fs::create_dir_all(&base_dir);
        Self { base_dir }
    }

    pub fn ingest_failure(
        &self,
        failure: &crate::code_retry::ValidationFailure,
        session_id: &str,
    ) -> anyhow::Result<BuilderFailureMemory> {
        let sig = SignatureGenerator::generate(failure);
        let mem_path = self.base_dir.join(format!("{}.json", sig.hash));

        let mut mem = if mem_path.exists() {
            let content = std::fs::read_to_string(&mem_path)?;
            serde_json::from_str(&content)?
        } else {
            BuilderFailureMemory {
                id: uuid::Uuid::new_v4().to_string(),
                signature: sig.clone(),
                original_failure_id: failure.id.clone(),
                kind: failure.kind.clone(),
                source_ref: BuilderMemorySourceRef {
                    session_id: session_id.to_string(),
                    validation_run_id: "unknown".to_string(),
                },
                outcomes: Vec::new(),
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                updated_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            }
        };

        mem.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        std::fs::write(mem_path, serde_json::to_string_pretty(&mem)?)?;
        Ok(mem)
    }

    pub fn record_outcome(
        &self,
        signature_hash: &str,
        outcome: BuilderFixOutcome,
    ) -> anyhow::Result<()> {
        let mem_path = self.base_dir.join(format!("{}.json", signature_hash));
        if mem_path.exists() {
            let content = std::fs::read_to_string(&mem_path)?;
            let mut mem: BuilderFailureMemory = serde_json::from_str(&content)?;
            mem.outcomes.push(outcome);
            mem.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            std::fs::write(mem_path, serde_json::to_string_pretty(&mem)?)?;
        }
        Ok(())
    }

    pub fn recall_similar(
        &self,
        failure: &crate::code_retry::ValidationFailure,
    ) -> anyhow::Result<Option<BuilderFailureMemory>> {
        let sig = SignatureGenerator::generate(failure);
        let mem_path = self.base_dir.join(format!("{}.json", sig.hash));
        if mem_path.exists() {
            let content = std::fs::read_to_string(&mem_path)?;
            let mem = serde_json::from_str(&content)?;
            Ok(Some(mem))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_memories(&self) -> anyhow::Result<Vec<BuilderFailureMemory>> {
        let mut memories = Vec::new();
        if !self.base_dir.exists() {
            return Ok(memories);
        }
        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(mem) = serde_json::from_str::<BuilderFailureMemory>(&content) {
                        memories.push(mem);
                    }
                }
            }
        }
        Ok(memories)
    }
}
