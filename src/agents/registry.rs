use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::agents::manifest::{AgentManifest, AgentStatus, AgentTier};

pub struct AgentRegistry {
    pub agents: HashMap<String, AgentManifest>,
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            agents: HashMap::new(),
        };
        registry.load_builtins();
        registry.load_local();
        registry
    }

    pub fn register(&mut self, manifest: AgentManifest) {
        self.agents.insert(manifest.id.clone(), manifest);
    }

    pub fn get(&self, id: &str) -> Option<&AgentManifest> {
        self.agents.get(id)
    }

    pub fn list(&self) -> Vec<&AgentManifest> {
        let mut list: Vec<_> = self.agents.values().collect();
        list.sort_by_key(|a| &a.id);
        list
    }

    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let cloned_agent = {
            let agent = self
                .agents
                .get_mut(id)
                .ok_or_else(|| format!("Agent {} not found", id))?;
            agent.status = AgentStatus::Active;
            agent.clone()
        };
        self.save_local(&cloned_agent)?;
        Ok(())
    }

    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        let cloned_agent = {
            let agent = self
                .agents
                .get_mut(id)
                .ok_or_else(|| format!("Agent {} not found", id))?;
            agent.status = AgentStatus::Disabled;
            agent.clone()
        };
        self.save_local(&cloned_agent)?;
        Ok(())
    }

    fn load_builtins(&mut self) {
        // Prime Agents
        self.register(AgentManifest {
            id: "cofounder".to_string(),
            name: "Cofounder".to_string(),
            description: "High-level strategic partner".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Experimental,
            prime_affinity: None,
            traits: vec!["Strategic".to_string(), "Visionary".to_string()],
            capabilities: vec!["Strategy".to_string(), "Planning".to_string()],
        });
        self.register(AgentManifest {
            id: "socializer".to_string(),
            name: "Socializer".to_string(),
            description: "Community and social media expert".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Planned,
            prime_affinity: None,
            traits: vec!["Extroverted".to_string(), "Persuasive".to_string()],
            capabilities: vec!["Marketing".to_string(), "Social Media".to_string()],
        });
        self.register(AgentManifest {
            id: "designer".to_string(),
            name: "Designer".to_string(),
            description: "UI/UX and creative design".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Planned,
            prime_affinity: None,
            traits: vec!["Creative".to_string(), "Visual".to_string()],
            capabilities: vec!["UI/UX".to_string(), "Graphics".to_string()],
        });
        self.register(AgentManifest {
            id: "researcher".to_string(),
            name: "Researcher".to_string(),
            description: "Information gathering and analysis".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Experimental,
            prime_affinity: None,
            traits: vec!["Analytical".to_string(), "Thorough".to_string()],
            capabilities: vec!["Search".to_string(), "Synthesis".to_string()],
        });
        self.register(AgentManifest {
            id: "builder".to_string(),
            name: "Builder".to_string(),
            description: "Software engineering and development".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Experimental,
            prime_affinity: None,
            traits: vec!["Logical".to_string(), "Precise".to_string()],
            capabilities: vec!["Coding".to_string(), "Architecture".to_string()],
        });
        self.register(AgentManifest {
            id: "operator".to_string(),
            name: "Operator".to_string(),
            description: "DevOps and system operations".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Planned,
            prime_affinity: None,
            traits: vec!["Reliable".to_string(), "Systematic".to_string()],
            capabilities: vec!["Deployment".to_string(), "Monitoring".to_string()],
        });
        self.register(AgentManifest {
            id: "learner".to_string(),
            name: "Learner".to_string(),
            description: "Knowledge acquisition and indexing".to_string(),
            tier: AgentTier::Prime,
            status: AgentStatus::Planned,
            prime_affinity: None,
            traits: vec!["Curious".to_string(), "Adaptable".to_string()],
            capabilities: vec!["Learning".to_string(), "Memory".to_string()],
        });

        // Specialist Agents
        self.register(AgentManifest {
            id: "finance_analyst".to_string(),
            name: "Finance Analyst".to_string(),
            description: "Financial planning and reporting".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("cofounder".to_string()),
            traits: vec!["Analytical".to_string(), "Detail-oriented".to_string()],
            capabilities: vec!["Financial Modeling".to_string(), "Accounting".to_string()],
        });
        self.register(AgentManifest {
            id: "reddit_strategist".to_string(),
            name: "Reddit Strategist".to_string(),
            description: "Reddit marketing and community building".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("socializer".to_string()),
            traits: vec!["Trend-aware".to_string(), "Engaging".to_string()],
            capabilities: vec![
                "Content Strategy".to_string(),
                "Community Management".to_string(),
            ],
        });
        self.register(AgentManifest {
            id: "ui_critic".to_string(),
            name: "UI Critic".to_string(),
            description: "UI/UX analysis and feedback".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("designer".to_string()),
            traits: vec!["Observant".to_string(), "Constructive".to_string()],
            capabilities: vec![
                "Usability Testing".to_string(),
                "Design Critique".to_string(),
            ],
        });
        self.register(AgentManifest {
            id: "source_checker".to_string(),
            name: "Source Checker".to_string(),
            description: "Fact-checking and source verification".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("researcher".to_string()),
            traits: vec!["Skeptical".to_string(), "Rigorous".to_string()],
            capabilities: vec!["Verification".to_string(), "Fact-checking".to_string()],
        });
        self.register(AgentManifest {
            id: "code_reviewer".to_string(),
            name: "Code Reviewer".to_string(),
            description: "Code quality and security analysis".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Experimental,
            prime_affinity: Some("builder".to_string()),
            traits: vec!["Pedantic".to_string(), "Security-minded".to_string()],
            capabilities: vec!["Code Review".to_string(), "Security Auditing".to_string()],
        });
        self.register(AgentManifest {
            id: "devops_specialist".to_string(),
            name: "DevOps Specialist".to_string(),
            description: "CI/CD and infrastructure management".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("operator".to_string()),
            traits: vec!["Automator".to_string(), "Resilient".to_string()],
            capabilities: vec!["CI/CD".to_string(), "Infrastructure as Code".to_string()],
        });
        self.register(AgentManifest {
            id: "dsa_coach".to_string(),
            name: "DSA Coach".to_string(),
            description: "Data structures and algorithms training".to_string(),
            tier: AgentTier::Specialist,
            status: AgentStatus::Planned,
            prime_affinity: Some("learner".to_string()),
            traits: vec!["Patient".to_string(), "Structured".to_string()],
            capabilities: vec!["Algorithm Design".to_string(), "Teaching".to_string()],
        });
    }

    fn load_local(&mut self) {
        if let Some(config_dir) = dirs::config_dir() {
            let agents_dir = config_dir.join("goat").join("agents");
            self.load_from_dir(&agents_dir);
        }
        if let Some(data_dir) = dirs::data_dir() {
            let agents_dir = data_dir.join("goat").join("agents");
            self.load_from_dir(&agents_dir);
        }
    }

    fn load_from_dir(&mut self, dir: &Path) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "json" {
                                if let Ok(contents) = fs::read_to_string(entry.path()) {
                                    if let Ok(manifest) =
                                        serde_json::from_str::<AgentManifest>(&contents)
                                    {
                                        // Overwrite builtin if found locally
                                        self.register(manifest);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn save_local(&self, manifest: &AgentManifest) -> Result<(), String> {
        if let Some(data_dir) = dirs::data_dir() {
            let agents_dir = data_dir.join("goat").join("agents");
            if let Err(e) = fs::create_dir_all(&agents_dir) {
                return Err(format!("Failed to create agents directory: {}", e));
            }
            let file_path = agents_dir.join(format!("{}.json", manifest.id));
            let json = serde_json::to_string_pretty(manifest).map_err(|e| e.to_string())?;
            fs::write(file_path, json).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Data directory not found".to_string())
        }
    }
}
