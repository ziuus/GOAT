use anyhow::{Result, anyhow};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::MemoryConfig;
use crate::paths::GoatPaths;

pub struct MemoryManager {
    pub user_file: PathBuf,
    pub memory_file: PathBuf,
    pub config: MemoryConfig,
}

impl MemoryManager {
    pub fn new(paths: &GoatPaths, config: MemoryConfig) -> Self {
        Self {
            user_file: paths.user_file.clone(),
            memory_file: paths.memory_file.clone(),
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
            return Ok(String::new());
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
                    context.push_str("</PROJECT_CONTEXT>\n\n");
                }
            }
        }

        context
    }
}
