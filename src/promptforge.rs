use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PromptForgeMode {
    Mock,
    Model,
    Cli,
    Api,
}

impl Default for PromptForgeMode {
    fn default() -> Self {
        PromptForgeMode::Mock
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeAgentMapping {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub target: String,
}

fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeRules {
    #[serde(default)]
    pub min_complexity: String,
    #[serde(default = "default_true")]
    pub skip_simple_commands: bool,
    #[serde(default)]
    pub require_confirmation_for_refined_prompt: bool,
    #[serde(default)]
    pub strict_mode: bool,
    #[serde(default = "default_max_prompt_chars")]
    pub max_prompt_chars: usize,
    #[serde(default)]
    pub send_project_summary: bool,
    #[serde(default)]
    pub send_brain_context: bool,
    #[serde(default)]
    pub send_timeline_context: bool,
}

fn default_max_prompt_chars() -> usize { 12000 }

impl Default for PromptForgeRules {
    fn default() -> Self {
        Self {
            min_complexity: "medium".to_string(),
            skip_simple_commands: true,
            require_confirmation_for_refined_prompt: false,
            strict_mode: false,
            max_prompt_chars: 12000,
            send_project_summary: false,
            send_brain_context: false,
            send_timeline_context: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub mode: PromptForgeMode,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub api_url: String,
    #[serde(default)]
    pub default_target: String,
    #[serde(default)]
    pub auto_refine: bool,
    #[serde(default = "default_true")]
    pub store_history: bool,
    #[serde(default = "default_true")]
    pub fail_open: bool,
    #[serde(default)]
    pub provider_profile: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub allow_browser_chat: bool,

    #[serde(default)]
    pub rules: PromptForgeRules,
    
    #[serde(default)]
    pub agents: HashMap<String, PromptForgeAgentMapping>,
}

impl Default for PromptForgeConfig {
    fn default() -> Self {
        let mut agents = HashMap::new();
        agents.insert("builder".to_string(), PromptForgeAgentMapping { enabled: true, target: "coding".to_string() });
        agents.insert("cofounder".to_string(), PromptForgeAgentMapping { enabled: true, target: "product".to_string() });
        agents.insert("researcher".to_string(), PromptForgeAgentMapping { enabled: true, target: "research".to_string() });
        agents.insert("designer".to_string(), PromptForgeAgentMapping { enabled: true, target: "design".to_string() });
        agents.insert("socializer".to_string(), PromptForgeAgentMapping { enabled: true, target: "social".to_string() });
        agents.insert("operator".to_string(), PromptForgeAgentMapping { enabled: true, target: "operations".to_string() });
        agents.insert("learner".to_string(), PromptForgeAgentMapping { enabled: true, target: "learning".to_string() });

        Self {
            enabled: false,
            mode: PromptForgeMode::Mock,
            command: "promptforge".to_string(),
            api_url: "http://localhost:4567".to_string(),
            default_target: "goat".to_string(),
            auto_refine: false,
            store_history: true,
            fail_open: true,
            provider_profile: "default".to_string(),
            model: "".to_string(),
            allow_browser_chat: false,
            rules: PromptForgeRules::default(),
            agents,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeStatus {
    pub enabled: bool,
    pub mode: PromptForgeMode,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeRefineRequest {
    pub original_prompt: String,
    pub target_agent: String,
    pub target_format: String,
    pub domain: String,
    pub complexity: String,
    pub safe_context: String,
    pub constraints: Vec<String>,
    pub mode: PromptForgeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeRefineResponse {
    pub original_prompt: String,
    pub refined_prompt: String,
    pub original_score: Option<u8>,
    pub refined_score: Option<u8>,
    pub target_agent: String,
    pub target: String,
    pub improvements: Vec<String>,
    pub warnings: Vec<String>,
    pub provider_used: String,
    pub mode_used: PromptForgeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeDecision {
    pub should_refine: bool,
    pub reason: String,
    pub complexity: String, // simple | medium | complex
    pub domain: String, // coding | design | research | startup | social | operations | learning | general
    pub target_agent: String,
    pub promptforge_target: String,
    pub risk_level: String,
    pub user_confirmation_required: bool,
    pub mode_used: PromptForgeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeError {
    pub message: String,
}

impl std::fmt::Display for PromptForgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PromptForgeError: {}", self.message)
    }
}

impl std::error::Error for PromptForgeError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeHistoryEntry {
    pub id: String,
    pub timestamp: i64,
    pub agent: String,
    pub target: String,
    pub mode: PromptForgeMode,
    pub original_prompt: String,
    pub refined_prompt: String,
    pub original_score: Option<u8>,
    pub refined_score: Option<u8>,
    pub improvements: Vec<String>,
    pub warnings: Vec<String>,
    pub used_auto_refine: bool,
    pub user_override: Option<bool>,
    pub status: String,
    pub error_if_any: Option<String>,
}

pub struct PromptForgeClient {
    config: PromptForgeConfig,
    base_dir: PathBuf,
}

impl PromptForgeClient {
    pub fn new(config: PromptForgeConfig) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/promptforge");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(base_dir.join("sessions"));
        let _ = fs::create_dir_all(base_dir.join("logs"));
        
        Self { config, base_dir }
    }

    pub fn should_refine(&self, task: &str, agent: &str, is_user_override: Option<bool>) -> PromptForgeDecision {
        let mut decision = PromptForgeDecision {
            should_refine: false,
            reason: "Refinement not required".to_string(),
            complexity: "simple".to_string(),
            domain: "general".to_string(),
            target_agent: agent.to_string(),
            promptforge_target: self.config.default_target.clone(),
            risk_level: "low".to_string(),
            user_confirmation_required: self.config.rules.require_confirmation_for_refined_prompt,
            mode_used: self.config.mode.clone(),
        };

        if let Some(mapping) = self.config.agents.get(agent) {
            if !mapping.enabled {
                decision.reason = format!("Agent {} has PromptForge disabled.", agent);
                return decision;
            }
            decision.promptforge_target = mapping.target.clone();
            decision.domain = mapping.target.clone();
        }

        if task.len() > 100 {
            decision.complexity = "complex".to_string();
        } else {
            let lower = task.to_lowercase();
            if lower.contains("build") || lower.contains("architect") || lower.contains("research") || lower.contains("plan") || lower.contains("design") {
                decision.complexity = "medium".to_string();
            }
        }

        let is_simple = decision.complexity == "simple";
        if self.config.rules.skip_simple_commands && is_simple {
            decision.reason = "Simple command, bypassing refinement.".to_string();
            return decision;
        }

        if !self.config.enabled {
            decision.reason = "PromptForge is globally disabled.".to_string();
            return decision;
        }

        if let Some(user_ov) = is_user_override {
            decision.should_refine = user_ov;
            decision.reason = if user_ov { "User explicitly requested refinement.".to_string() } else { "User explicitly bypassed refinement.".to_string() };
            return decision;
        }

        if self.config.auto_refine {
            decision.should_refine = true;
            decision.reason = "Auto-refine is enabled and task meets complexity threshold.".to_string();
        } else {
            decision.reason = "Auto-refine is disabled, waiting for manual request.".to_string();
        }

        decision
    }

    pub async fn refine(&self, req: PromptForgeRefineRequest) -> Result<PromptForgeRefineResponse, PromptForgeError> {
        let start = chrono::Utc::now().timestamp();
        
        let res = match req.mode {
            PromptForgeMode::Mock => {
                let refined = format!("REFINED: {}", req.original_prompt);
                Ok(PromptForgeRefineResponse {
                    original_prompt: req.original_prompt.clone(),
                    refined_prompt: refined.clone(),
                    original_score: Some(50),
                    refined_score: Some(95),
                    target_agent: req.target_agent.clone(),
                    target: req.target_format.clone(),
                    improvements: vec!["Added structure".to_string(), "Clarified goals".to_string()],
                    warnings: vec![],
                    provider_used: "mock_local".to_string(),
                    mode_used: PromptForgeMode::Mock,
                })
            }
            PromptForgeMode::Model => {
                // Uses internal provider in real implementation.
                // For now, doing a safe deterministic fallback.
                let mut refined = format!("{}\n\n### Constraints & Context\n- Target: {}\n- Domain: {}", req.original_prompt, req.target_format, req.domain);
                Ok(PromptForgeRefineResponse {
                    original_prompt: req.original_prompt.clone(),
                    refined_prompt: refined.clone(),
                    original_score: None,
                    refined_score: None,
                    target_agent: req.target_agent.clone(),
                    target: req.target_format.clone(),
                    improvements: vec!["Added context".to_string()],
                    warnings: vec![],
                    provider_used: "model_fallback".to_string(),
                    mode_used: PromptForgeMode::Model,
                })
            }
            _ => {
                if self.config.fail_open {
                    Ok(PromptForgeRefineResponse {
                        original_prompt: req.original_prompt.clone(),
                        refined_prompt: req.original_prompt.clone(),
                        original_score: None,
                        refined_score: None,
                        target_agent: req.target_agent.clone(),
                        target: req.target_format.clone(),
                        improvements: vec![],
                        warnings: vec![format!("Mode {:?} not available, returning original prompt (fail-open)", req.mode)],
                        provider_used: "none".to_string(),
                        mode_used: req.mode.clone(),
                    })
                } else {
                    Err(PromptForgeError { message: format!("Mode {:?} not supported and strict mode enabled.", req.mode) })
                }
            }
        };

        if let Ok(ref response) = res {
            if self.config.store_history {
                let entry = PromptForgeHistoryEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: start,
                    agent: req.target_agent.clone(),
                    target: req.target_format.clone(),
                    mode: req.mode.clone(),
                    original_prompt: req.original_prompt.clone(),
                    refined_prompt: response.refined_prompt.clone(),
                    original_score: response.original_score,
                    refined_score: response.refined_score,
                    improvements: response.improvements.clone(),
                    warnings: response.warnings.clone(),
                    used_auto_refine: true, // simplified for now
                    user_override: None,
                    status: "success".to_string(),
                    error_if_any: None,
                };
                let _ = self.save_history(entry);
            }
        } else if let Err(ref err) = res {
            if self.config.store_history {
                 let entry = PromptForgeHistoryEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: start,
                    agent: req.target_agent.clone(),
                    target: req.target_format.clone(),
                    mode: req.mode.clone(),
                    original_prompt: req.original_prompt.clone(),
                    refined_prompt: "".to_string(),
                    original_score: None,
                    refined_score: None,
                    improvements: vec![],
                    warnings: vec![],
                    used_auto_refine: true,
                    user_override: None,
                    status: "failed".to_string(),
                    error_if_any: Some(err.message.clone()),
                };
                let _ = self.save_history(entry);
            }
        }

        res
    }

    fn save_history(&self, entry: PromptForgeHistoryEntry) -> Result<(), std::io::Error> {
        let history_file = self.base_dir.join("history.jsonl");
        let line = serde_json::to_string(&entry)? + "\n";
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&history_file)?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }

    pub fn get_history(&self) -> Vec<PromptForgeHistoryEntry> {
        let mut history = Vec::new();
        let history_file = self.base_dir.join("history.jsonl");
        if let Ok(content) = fs::read_to_string(&history_file) {
            for line in content.lines() {
                if let Ok(entry) = serde_json::from_str::<PromptForgeHistoryEntry>(line) {
                    history.push(entry);
                }
            }
        }
        history
    }
}
