use crate::config::SkillsConfig;
use crate::paths::GoatPaths;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a parsed skill from a SKILL.md file.
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub triggers: String,
    pub content: String,
    pub is_suspicious: bool,
    pub warnings: Vec<String>,
}

pub struct SkillManager {
    paths: GoatPaths,
    config: SkillsConfig,
}

impl SkillManager {
    pub fn new(paths: GoatPaths, config: SkillsConfig) -> Self {
        let _ = paths.ensure_skills_dir();
        Self { paths, config }
    }

    /// List all skills found in the skills directory.
    pub fn list_skills(&self) -> Vec<Skill> {
        let mut skills = Vec::new();
        if !self.paths.skills_dir.exists() {
            return skills;
        }

        let dir = match fs::read_dir(&self.paths.skills_dir) {
            Ok(d) => d,
            Err(_) => return skills,
        };

        for entry in dir.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if skill_md.exists() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if let Ok(skill) = self.parse_skill(&skill_md, name) {
                        skills.push(skill);
                    }
                }
            } else if path.is_file() && path.extension().unwrap_or_default() == "md" {
                // Also support standalone .md files in the skills directory
                let name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if let Ok(skill) = self.parse_skill(&path, name) {
                    skills.push(skill);
                }
            }
        }
        
        // Sort by name for consistency
        skills.sort_by(|a, b| a.name.cmp(&b.name));
        skills
    }

    /// Parse a single SKILL.md file.
    fn parse_skill(&self, path: &Path, name: String) -> Result<Skill> {
        let content = fs::read_to_string(path).context("Failed to read skill file")?;
        
        let mut description = String::new();
        let mut triggers = String::new();
        let mut is_suspicious = false;
        let mut warnings = Vec::new();

        // Very basic markdown parsing
        let mut in_description = false;
        let mut in_triggers = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("## Description") {
                in_description = true;
                in_triggers = false;
                continue;
            } else if trimmed.starts_with("## Triggers") {
                in_description = false;
                in_triggers = true;
                continue;
            } else if trimmed.starts_with("## ") {
                in_description = false;
                in_triggers = false;
            }

            if in_description && !trimmed.is_empty() {
                if description.is_empty() {
                    description = trimmed.to_string();
                } else {
                    description.push(' ');
                    description.push_str(trimmed);
                }
            } else if in_triggers && !trimmed.is_empty() {
                if triggers.is_empty() {
                    triggers = trimmed.to_string();
                } else {
                    triggers.push(' ');
                    triggers.push_str(trimmed);
                }
            }
        }

        // Basic secret detection
        let lower_content = content.to_lowercase();
        if lower_content.contains("sk-") || lower_content.contains("gsk_") {
            is_suspicious = true;
            warnings.push("Suspicious API key pattern detected (sk- or gsk_)".to_string());
        }
        if lower_content.contains("rm -rf") {
            is_suspicious = true;
            warnings.push("Suspicious command: rm -rf".to_string());
        }
        if lower_content.contains("curl | sh") || lower_content.contains("wget -qO-") {
            is_suspicious = true;
            warnings.push("Suspicious command: curl | sh (arbitrary script execution)".to_string());
        }
        if lower_content.contains("sudo ") {
            is_suspicious = true;
            warnings.push("Suspicious command: sudo (privilege escalation)".to_string());
        }
        if lower_content.contains("password=") || lower_content.contains("secret=") {
            is_suspicious = true;
            warnings.push("Suspicious string: password= or secret=".to_string());
        }
        if lower_content.contains("-----begin rsa") || lower_content.contains("-----begin openssh") {
            is_suspicious = true;
            warnings.push("Suspicious string: RSA/SSH private key".to_string());
        }

        Ok(Skill {
            name,
            description,
            triggers,
            content,
            is_suspicious,
            warnings,
        })
    }

    /// Load a specific skill by name.
    pub fn get_skill(&self, name: &str) -> Option<Skill> {
        let skills = self.list_skills();
        skills.into_iter().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    /// Search skills by query
    pub fn search_skills(&self, query: &str) -> Vec<Skill> {
        let query_lower = query.to_lowercase();
        self.list_skills()
            .into_iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
                    || s.triggers.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get the path to the skills directory
    pub fn skills_dir(&self) -> PathBuf {
        self.paths.skills_dir.clone()
    }

    /// Create a template SKILL.md file
    pub fn create_template(&self, name: &str) -> Result<PathBuf> {
        let skill_dir = self.paths.skills_dir.join(name);
        fs::create_dir_all(&skill_dir)?;
        let skill_file = skill_dir.join("SKILL.md");
        
        if skill_file.exists() {
            anyhow::bail!("Skill '{}' already exists at {}", name, skill_file.display());
        }

        let template = format!(
            r#"# {name}

## Description
Short description of what this skill does.

## Triggers
- When the user asks about X
- Trigger alias: `run {name}`

## Tools Needed
- `bash` (for running scripts)
- `read_file` (for reading code)

## Procedure
1. Step 1...
2. Step 2...

## Safety Notes
- Always check XYZ before proceeding.
- Approval required for write operations.

## Verification
- How to verify it worked.
"#,
            name = name
        );

        fs::write(&skill_file, template)?;
        Ok(skill_file)
    }

    /// Build the skills context injection block for the LLM.
    /// Uses progressive disclosure: index by default, full content if active_skill is set.
    pub fn build_context(&self, active_skill: Option<&str>) -> String {
        if !self.config.enabled {
            return String::new();
        }

        let mut out = String::new();
        let skills = self.list_skills();
        
        if skills.is_empty() {
            return out;
        }

        out.push_str("\n<GOAT_SKILLS>\n");

        if let Some(skill_name) = active_skill {
            if let Some(skill) = self.get_skill(skill_name) {
                out.push_str(&format!(
                    "User has explicitly activated the skill: {}\n\n",
                    skill.name
                ));
                
                if skill.is_suspicious {
                    out.push_str("WARNING: This skill contains suspicious patterns and may be unsafe.\n");
                }
                
                let mut content = skill.content.clone();
                if content.len() > self.config.max_skill_chars {
                    content.truncate(self.config.max_skill_chars);
                    content.push_str("\n... [SKILL TRUNCATED DUE TO BUDGET]\n");
                }
                out.push_str(&content);
                out.push_str("\n</GOAT_SKILLS>\n");
                return out;
            } else {
                out.push_str(&format!("WARNING: Requested active skill '{}' was not found.\n\n", skill_name));
            }
        }

        if self.config.inject_index {
            out.push_str("Available skills (use to assist with related tasks):\n\n");
            let mut index_str = String::new();
            
            for skill in skills {
                if skill.is_suspicious { continue; } // Do not advertise suspicious skills by default
                let entry = format!(
                    "- **{}**: {}\n  Triggers: {}\n",
                    skill.name,
                    if skill.description.is_empty() { "No description" } else { &skill.description },
                    if skill.triggers.is_empty() { "None specified" } else { &skill.triggers }
                );
                index_str.push_str(&entry);
            }
            
            if index_str.len() > self.config.max_index_chars {
                index_str.truncate(self.config.max_index_chars);
                index_str.push_str("\n... [INDEX TRUNCATED DUE TO BUDGET]\n");
            }
            
            out.push_str(&index_str);
            out.push_str("\n(If a skill is relevant, refer to it explicitly to load its full procedure on the next turn.)\n");
        }

        out.push_str("</GOAT_SKILLS>\n");
        out
    }
}
