use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::config::MemoryConfig;
use crate::paths::GoatPaths;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScope {
    User,
    Project,
    Mission,
    Skill,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    Preference,
    ProjectDecision,
    ArchitectureNote,
    Command,
    Workflow,
    BugFix,
    ValidationResult,
    DiffInsight,
    AgentNote,
    Reminder,
    Caution,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryStatus {
    Active,
    Archived,
    NeedsReview,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub memory_id: String,
    pub scope: MemoryScope,
    pub project_id: Option<String>,
    pub mission_id: Option<String>,
    pub source: String,
    pub kind: MemoryKind,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub confidence: u32,
    pub status: MemoryStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: Option<i64>,
    pub use_count: u32,
}

pub fn redact_secrets(content: &str) -> String {
    let mut redacted = content.to_string();
    let patterns = [
        "api_key",
        "apikey",
        "password",
        "token",
        "secret",
        "private_key",
        "client_secret",
    ];

    // Simple line-based redaction
    let lines: Vec<&str> = redacted.split('\n').collect();
    let mut out = Vec::new();
    for line in lines {
        let lower = line.to_lowercase();
        let mut line_redacted = false;
        for p in patterns.iter() {
            if lower.contains(p) && (line.contains('=') || line.contains(':')) {
                out.push("[REDACTED SECRET]");
                line_redacted = true;
                break;
            }
        }
        if !line_redacted {
            out.push(line);
        }
    }
    out.join("\n")
}

pub struct MemoryManager {
    pub user_file: PathBuf,
    pub memory_file: PathBuf,
    pub projects_dir: PathBuf,
    pub structured_store: PathBuf,
    pub config: MemoryConfig,
}

impl MemoryManager {
    pub fn new(paths: &GoatPaths, config: MemoryConfig) -> Self {
        let projects_dir = paths.data_dir.join("projects");
        let memory_dir = paths.data_dir.join("memory");
        let structured_store = memory_dir.join("memories.jsonl");

        Self {
            user_file: paths.user_file.clone(),
            memory_file: paths.memory_file.clone(),
            projects_dir,
            structured_store,
            config,
        }
    }

    pub fn ensure_files(&self) -> Result<()> {
        Self::ensure_file(
            &self.user_file,
            "# USER.md\n\nUser preferences, communication style, and goals.\n\n",
        )?;
        Self::ensure_file(
            &self.memory_file,
            "# MEMORY.md\n\nGOAT's learned notes about the environment, projects, and workflows.\n\n",
        )?;
        if let Some(parent) = self.structured_store.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    fn ensure_file(path: &Path, default_header: &str) -> Result<()> {
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, default_header)?;
        }
        Ok(())
    }

    pub fn add_user(&self, text: &str) -> Result<()> {
        self.ensure_files()?;
        self.check_secrets(text)?;
        let mut file = OpenOptions::new().append(true).open(&self.user_file)?;
        writeln!(file, "- {}", text)?;
        Ok(())
    }

    pub fn add_note(&self, text: &str) -> Result<()> {
        self.ensure_files()?;
        self.check_secrets(text)?;
        let mut file = OpenOptions::new().append(true).open(&self.memory_file)?;
        writeln!(file, "- {}", text)?;
        Ok(())
    }

    pub fn get_user_content(&self) -> Result<String> {
        if !self.user_file.exists() {
            let template = "# User Memory\n\nAdd your core preferences and profile details here.\n";
            fs::write(&self.user_file, template)?;
            return Ok(template.to_string());
        }
        let content = fs::read_to_string(&self.user_file)?;
        Ok(content)
    }

    pub fn get_memory_content(&self) -> Result<String> {
        if !self.memory_file.exists() {
            return Ok(String::new());
        }
        let content = fs::read_to_string(&self.memory_file)?;
        Ok(content)
    }

    pub fn get_project_memory(&self, project_id: &str) -> Result<String> {
        let path = self.projects_dir.join(project_id).join("PROJECT_MEMORY.md");
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let template = format!(
                "# Project Memory: {}\n\nAdd architecture notes and decisions here.\n",
                project_id
            );
            fs::write(&path, &template)?;
            return Ok(template);
        }
        let content = fs::read_to_string(&path)?;
        Ok(content)
    }

    pub fn update_project_memory(&self, project_id: &str, content: &str) -> Result<()> {
        self.check_secrets(content)?;
        let path = self.projects_dir.join(project_id).join("PROJECT_MEMORY.md");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_user_char_count(&self) -> usize {
        self.get_user_content().map(|c| c.len()).unwrap_or(0)
    }

    pub fn get_memory_char_count(&self) -> usize {
        self.get_memory_content().map(|c| c.len()).unwrap_or(0)
    }

    pub fn user_budget_status(&self) -> (usize, usize, bool) {
        let count = self.get_user_char_count();
        (
            count,
            self.config.max_user_chars,
            count > self.config.max_user_chars,
        )
    }

    pub fn memory_budget_status(&self) -> (usize, usize, bool) {
        let count = self.get_memory_char_count();
        (
            count,
            self.config.max_memory_chars,
            count > self.config.max_memory_chars,
        )
    }

    fn check_secrets(&self, text: &str) -> Result<()> {
        let text_lower = text.to_lowercase();
        if text.contains("sk-")
            || text.contains("gsk_")
            || text.contains("AKIA")
            || text.contains("-----BEGIN")
        {
            return Err(anyhow!(
                "Secret detected. Refusing to save sensitive data to memory."
            ));
        }
        if text_lower.contains("password=")
            || text_lower.contains("api_key=")
            || text_lower.contains("secret=")
        {
            return Err(anyhow!(
                "Secret detected. Refusing to save sensitive data to memory."
            ));
        }
        Ok(())
    }

    pub fn add_structured_memory(&self, mut item: MemoryItem) -> Result<String> {
        self.ensure_files()?;
        self.check_secrets(&item.content)?;
        item.memory_id = uuid::Uuid::new_v4().to_string();
        item.created_at = chrono::Utc::now().timestamp();
        item.updated_at = item.created_at;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.structured_store)?;

        let json = serde_json::to_string(&item)?;
        writeln!(file, "{}", json)?;
        Ok(item.memory_id)
    }

    pub fn list_structured_memories(&self) -> Result<Vec<MemoryItem>> {
        if !self.structured_store.exists() {
            return Ok(vec![]);
        }

        let file = fs::File::open(&self.structured_store)?;
        let reader = BufReader::new(file);
        let mut memories = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Ok(item) = serde_json::from_str::<MemoryItem>(&line) {
                memories.push(item);
            }
        }
        Ok(memories)
    }

    pub fn search_structured_memories(&self, query: &str) -> Result<Vec<MemoryItem>> {
        let memories = self.list_structured_memories()?;
        let query_lower = query.to_lowercase();

        let matched: Vec<MemoryItem> = memories
            .into_iter()
            .filter(|m| {
                m.status == MemoryStatus::Active
                    && (m.content.to_lowercase().contains(&query_lower)
                        || m.title.to_lowercase().contains(&query_lower)
                        || m.tags
                            .iter()
                            .any(|t| t.to_lowercase().contains(&query_lower))
                        || (m.project_id.as_deref().unwrap_or("") == query)
                        || (m.mission_id.as_deref().unwrap_or("") == query))
            })
            .collect();

        Ok(matched)
    }

    pub fn get_structured_memory(&self, id: &str) -> Result<Option<MemoryItem>> {
        let memories = self.list_structured_memories()?;
        Ok(memories.into_iter().find(|m| m.memory_id == id))
    }

    pub fn update_structured_memory(&self, item: &MemoryItem) -> Result<()> {
        let mut memories = self.list_structured_memories()?;
        let mut found = false;
        for m in memories.iter_mut() {
            if m.memory_id == item.memory_id {
                *m = item.clone();
                m.updated_at = chrono::Utc::now().timestamp();
                found = true;
                break;
            }
        }

        if found {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.structured_store)?;

            for m in memories {
                let json = serde_json::to_string(&m)?;
                writeln!(file, "{}", json)?;
            }
        }
        Ok(())
    }

    pub fn archive_memory(&self, id: &str) -> Result<()> {
        if let Some(mut m) = self.get_structured_memory(id)? {
            m.status = MemoryStatus::Archived;
            self.update_structured_memory(&m)?;
        }
        Ok(())
    }

    pub fn build_context(&self, brain: Option<&crate::brain::Brain>) -> String {
        if !self.config.enabled {
            return String::new();
        }

        let mut context = String::new();

        if self.config.inject_user {
            let u_content = self.get_user_content().unwrap_or_default();
            let u_content = if u_content.len() > self.config.max_user_chars {
                &u_content[..self.config.max_user_chars]
            } else {
                &u_content
            };
            if !u_content.trim().is_empty() {
                context.push_str("<USER_PREFERENCES>\n");
                context.push_str(u_content);
                context.push_str("\n</USER_PREFERENCES>\n\n");
            }
        }

        if self.config.inject_memory {
            let m_content = self.get_memory_content().unwrap_or_default();
            let m_content = if m_content.len() > self.config.max_memory_chars {
                &m_content[..self.config.max_memory_chars]
            } else {
                &m_content
            };
            if !m_content.trim().is_empty() {
                context.push_str("<MEMORY_NOTES>\n");
                context.push_str(m_content);
                context.push_str("\n</MEMORY_NOTES>\n\n");
            }
        }

        if self.config.inject_project {
            if let Some(brain) = brain {
                let root = std::env::current_dir().unwrap_or_default();
                if let Ok(Some(meta)) = brain.get_project(root.to_string_lossy().as_ref()) {
                    context.push_str("<PROJECT_CONTEXT>\n");
                    context.push_str(&format!("Root: {}\n", meta.root_path.display()));
                    if !meta.stack.is_empty() {
                        context.push_str(&format!("Stack: {}\n", meta.stack.join(", ")));
                    }
                    if !meta.detected_commands.is_empty() {
                        context.push_str(&format!(
                            "Commands: {}\n",
                            meta.detected_commands.join(", ")
                        ));
                    }

                    let pid = meta
                        .root_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    if let Ok(proj_mem) = self.get_project_memory(&pid) {
                        if !proj_mem.trim().is_empty() {
                            context.push_str("\n-- PROJECT_MEMORY.md --\n");
                            let len = proj_mem.len().min(2000);
                            context.push_str(&proj_mem[..len]);
                        }
                    }

                    if let Ok(memories) = self.search_structured_memories(&pid) {
                        let active_mems: Vec<_> = memories
                            .iter()
                            .filter(|m| m.status == MemoryStatus::Active)
                            .collect();
                        if !active_mems.is_empty() {
                            context.push_str("\n-- Structured Project Memories --\n");
                            for mem in active_mems.iter().take(5) {
                                context.push_str(&format!(
                                    "* [{:?}] {}: {}\n",
                                    mem.kind, mem.title, mem.content
                                ));
                            }
                        }
                    }

                    context.push_str("\n</PROJECT_CONTEXT>\n\n");
                }
            }
        }

        context
    }
}
