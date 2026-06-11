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

fn default_true() -> bool {
    true
}

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

fn default_max_prompt_chars() -> usize {
    12000
}

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
        agents.insert(
            "builder".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "coding".to_string(),
            },
        );
        agents.insert(
            "cofounder".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "product".to_string(),
            },
        );
        agents.insert(
            "researcher".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "research".to_string(),
            },
        );
        agents.insert(
            "designer".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "design".to_string(),
            },
        );
        agents.insert(
            "socializer".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "social".to_string(),
            },
        );
        agents.insert(
            "operator".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "operations".to_string(),
            },
        );
        agents.insert(
            "learner".to_string(),
            PromptForgeAgentMapping {
                enabled: true,
                target: "learning".to_string(),
            },
        );

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PromptForgeTemplateKind {
    CodingFeature,
    CodingDebug,
    ProductValidation,
    ProductMvpScope,
    ResearchReport,
    DesignReview,
    SocialLaunch,
    OperationsIncident,
    LearningPlan,
    GeneralAgentTask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeTemplate {
    pub id: String,
    pub kind: PromptForgeTemplateKind,
    pub name: String,
    pub description: String,
    pub structure: String,
}

pub struct PromptForgeTemplateLibrary {
    pub templates: Vec<PromptForgeTemplate>,
}

impl PromptForgeTemplateLibrary {
    pub fn new() -> Self {
        Self {
            templates: vec![
                PromptForgeTemplate {
                    id: "coding_feature".to_string(),
                    kind: PromptForgeTemplateKind::CodingFeature,
                    name: "Coding Feature Refinement".to_string(),
                    description: "Structures a rough coding request into a clear feature spec"
                        .to_string(),
                    structure: "### Goal
[...]
### Repo Inspection Steps
[...]
### Constraints
[...]
### Tests/Checks
[...]"
                        .to_string(),
                },
                PromptForgeTemplate {
                    id: "product_validation".to_string(),
                    kind: PromptForgeTemplateKind::ProductValidation,
                    name: "Product Validation".to_string(),
                    description: "Structures a product idea for cofounder validation".to_string(),
                    structure: "### Problem Statement
[...]
### Assumptions
[...]
### Validation Questions
[...]"
                        .to_string(),
                },
                // Add more stubs if necessary
            ],
        }
    }

    pub fn get(&self, id: &str) -> Option<PromptForgeTemplate> {
        self.templates.iter().find(|t| t.id == id).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeScoreResult {
    pub total_score: u8,
    pub clarity: u8,
    pub specificity: u8,
    pub context: u8,
    pub constraints: u8,
    pub acceptance_criteria: u8,
    pub safety: u8,
    pub agent_fit: u8,
    pub weak_areas: Vec<String>,
    pub suggestions: Vec<String>,
    pub refinement_recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptForgeHistoryEntry {
    pub id: String,
    pub timestamp: i64,
    pub agent: String,
    pub target: String,
    pub mode: PromptForgeMode,
    pub template: Option<String>,
    pub original_prompt: String,
    pub refined_prompt: String,
    pub original_score: Option<u8>,
    pub refined_score: Option<u8>,
    pub improvements: Vec<String>,
    pub warnings: Vec<String>,
    pub decision_reason: Option<String>,
    pub used_auto_refine: bool,
    pub user_override: Option<bool>,
    pub status: String,
    pub error_if_any: Option<String>,
    pub timeline_refs: Option<Vec<String>>,
    pub brain_refs: Option<Vec<String>>,
}

pub struct PromptForgeClient {
    pub pf_config: PromptForgeConfig,
    pub global_config: crate::config::Config,
    pub base_dir: PathBuf,
}

impl PromptForgeClient {
    pub fn new(global_config: crate::config::Config) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/promptforge");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(base_dir.join("sessions"));
        let _ = fs::create_dir_all(base_dir.join("logs"));

        Self {
            pf_config: global_config.promptforge.clone(),
            global_config,
            base_dir,
        }
    }

    pub fn should_refine(
        &self,
        task: &str,
        agent: &str,
        is_user_override: Option<bool>,
    ) -> PromptForgeDecision {
        let mut decision = PromptForgeDecision {
            should_refine: false,
            reason: "Refinement not required".to_string(),
            complexity: "simple".to_string(),
            domain: "general".to_string(),
            target_agent: agent.to_string(),
            promptforge_target: self.pf_config.default_target.clone(),
            risk_level: "low".to_string(),
            user_confirmation_required: self
                .pf_config
                .rules
                .require_confirmation_for_refined_prompt,
            mode_used: self.pf_config.mode.clone(),
        };

        if let Some(mapping) = self.pf_config.agents.get(agent) {
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
            if lower.contains("build")
                || lower.contains("architect")
                || lower.contains("research")
                || lower.contains("plan")
                || lower.contains("design")
            {
                decision.complexity = "medium".to_string();
            }
        }

        let is_simple = decision.complexity == "simple";
        if self.pf_config.rules.skip_simple_commands && is_simple {
            decision.reason = "Simple command, bypassing refinement.".to_string();
            return decision;
        }

        if !self.pf_config.enabled {
            decision.reason = "PromptForge is globally disabled.".to_string();
            return decision;
        }

        if let Some(user_ov) = is_user_override {
            decision.should_refine = user_ov;
            decision.reason = if user_ov {
                "User explicitly requested refinement.".to_string()
            } else {
                "User explicitly bypassed refinement.".to_string()
            };
            return decision;
        }

        if self.pf_config.auto_refine {
            decision.should_refine = true;
            decision.reason =
                "Auto-refine is enabled and task meets complexity threshold.".to_string();
        } else {
            decision.reason = "Auto-refine is disabled, waiting for manual request.".to_string();
        }

        decision
    }

    pub async fn refine(
        &self,
        req: PromptForgeRefineRequest,
    ) -> Result<PromptForgeRefineResponse, PromptForgeError> {
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
                    improvements: vec![
                        "Added structure".to_string(),
                        "Clarified goals".to_string(),
                    ],
                    warnings: vec![],
                    provider_used: "mock_local".to_string(),
                    mode_used: PromptForgeMode::Mock,
                })
            }
            PromptForgeMode::Model => {
                use crate::providers::{ModelProviderRegistry, ModelRouteRequest};
                let mut registry =
                    ModelProviderRegistry::new(self.global_config.model_routing.clone());
                for (_, p_cfg) in &self.global_config.providers {
                    registry.register(p_cfg.clone());
                }

                let route_req = ModelRouteRequest {
                    agent_id: req.target_agent.clone(),
                    task_kind: "prompt_refinement".to_string(),
                    required_capabilities: vec![],
                    local_only: false,
                    allow_external: true,
                    preferred_provider: None,
                    preferred_model: None,
                    quality_preference: "balanced".to_string(),
                    latency_preference: "balanced".to_string(),
                    cost_preference: "balanced".to_string(),
                    fallback_allowed: true,
                };
                let decision = registry.route(&route_req);
                let llm = crate::llm::LlmRouter::from_config(&self.global_config);

                let sys_msg = crate::llm::Message {
                    role: "system".to_string(),
                    content: Some(format!(
                        "You are a prompt refinement specialist. Refine the given prompt for {} (domain: {}). Output ONLY the refined prompt text.",
                        req.target_format, req.domain
                    )),
                    tool_calls: None,
                    tool_call_id: None,
                };
                let user_msg = crate::llm::Message {
                    role: "user".to_string(),
                    content: Some(req.original_prompt.clone()),
                    tool_calls: None,
                    tool_call_id: None,
                };

                let provider_used = decision.provider_id.clone();

                match llm
                    .completion(
                        &decision.provider_id,
                        &decision.model,
                        vec![sys_msg, user_msg],
                        None,
                    )
                    .await
                {
                    Ok(content) => Ok(PromptForgeRefineResponse {
                        original_prompt: req.original_prompt.clone(),
                        refined_prompt: content.content.unwrap_or_default(),
                        original_score: None,
                        refined_score: None,
                        target_agent: req.target_agent.clone(),
                        target: req.target_format.clone(),
                        improvements: vec!["Refined by model".to_string()],
                        warnings: vec![],
                        provider_used,
                        mode_used: PromptForgeMode::Model,
                    }),
                    Err(e) => {
                        if self.pf_config.fail_open {
                            Ok(PromptForgeRefineResponse {
                                original_prompt: req.original_prompt.clone(),
                                refined_prompt: req.original_prompt.clone(),
                                original_score: None,
                                refined_score: None,
                                target_agent: req.target_agent.clone(),
                                target: req.target_format.clone(),
                                improvements: vec![],
                                warnings: vec![format!(
                                    "Model refinement failed, falling back: {}",
                                    e
                                )],
                                provider_used: "fallback".to_string(),
                                mode_used: PromptForgeMode::Model,
                            })
                        } else {
                            Err(PromptForgeError {
                                message: e.to_string(),
                            })
                        }
                    }
                }
            }
            _ => {
                if self.pf_config.fail_open {
                    Ok(PromptForgeRefineResponse {
                        original_prompt: req.original_prompt.clone(),
                        refined_prompt: req.original_prompt.clone(),
                        original_score: None,
                        refined_score: None,
                        target_agent: req.target_agent.clone(),
                        target: req.target_format.clone(),
                        improvements: vec![],
                        warnings: vec![format!(
                            "Mode {:?} not available, returning original prompt (fail-open)",
                            req.mode
                        )],
                        provider_used: "none".to_string(),
                        mode_used: req.mode.clone(),
                    })
                } else {
                    Err(PromptForgeError {
                        message: format!(
                            "Mode {:?} not supported and strict mode enabled.",
                            req.mode
                        ),
                    })
                }
            }
        };

        if let Ok(ref response) = res {
            if self.pf_config.store_history {
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
                    template: None,
                    decision_reason: None,
                    used_auto_refine: true,
                    user_override: None,
                    status: "success".to_string(),
                    error_if_any: None,
                    timeline_refs: None,
                    brain_refs: None,
                };
                let _ = self.save_history(entry);
            }
        } else if let Err(ref err) = res {
            if self.pf_config.store_history {
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
                    template: None,
                    decision_reason: None,
                    used_auto_refine: true,
                    user_override: None,
                    status: "failed".to_string(),
                    error_if_any: Some(err.message.clone()),
                    timeline_refs: None,
                    brain_refs: None,
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
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&history_file)?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }

    pub async fn score(&self, prompt: &str) -> PromptForgeScoreResult {
        // Mock scoring logic for phase 5.20
        let len = prompt.len();
        let total = if len > 200 { 85 } else { 45 };
        PromptForgeScoreResult {
            total_score: total,
            clarity: if len > 50 { 80 } else { 40 },
            specificity: if len > 100 { 80 } else { 30 },
            context: if len > 150 { 90 } else { 20 },
            constraints: 50,
            acceptance_criteria: 40,
            safety: 90,
            agent_fit: 70,
            weak_areas: if total < 50 {
                vec!["Too short".to_string(), "Missing constraints".to_string()]
            } else {
                vec![]
            },
            suggestions: if total < 50 {
                vec!["Add more details".to_string()]
            } else {
                vec![]
            },
            refinement_recommended: total < 75,
        }
    }

    pub async fn maybe_refine_for_agent(
        &self,
        agent: &str,
        task: &str,
        context: &str,
        is_user_override: Option<bool>,
    ) -> String {
        let decision = self.should_refine(task, agent, is_user_override);
        if !decision.should_refine {
            return task.to_string();
        }

        let req = PromptForgeRefineRequest {
            original_prompt: task.to_string(),
            target_agent: agent.to_string(),
            target_format: decision.promptforge_target.clone(),
            domain: decision.domain.clone(),
            complexity: decision.complexity.clone(),
            safe_context: context.to_string(),
            constraints: vec![],
            mode: self.pf_config.mode.clone(),
        };

        match self.refine(req).await {
            Ok(resp) => {
                if !resp.refined_prompt.is_empty() {
                    resp.refined_prompt
                } else {
                    task.to_string()
                }
            }
            Err(_) => task.to_string(),
        }
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
