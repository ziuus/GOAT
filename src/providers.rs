use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderKind {
    LocalMock,
    Ollama,
    Openai,
    Anthropic,
    Gemini,
    Openrouter,
    Groq,
    Mistral,
    Deepseek,
    Cohere,
    Together,
    Fireworks,
    Perplexity,
    Xai,
    OpenaiCompatible,
    LitellmGateway,
    LmStudio,
    Vllm,
    LlamaCppServer,
    Localai,
    AzureOpenai,
    AwsBedrock,
    GoogleVertex,
    CustomHttp,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderCapability {
    Chat,
    Completion,
    Code,
    Reasoning,
    Embedding,
    Vision,
    Audio,
    ImageGeneration,
    ToolCalling,
    JsonMode,
    Streaming,
    LongContext,
    CheapFast,
    HighQuality,
    LocalOnly,
    OpenaiCompatible,
    Batch,
    FunctionCalling,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelCapabilitySet {
    pub caps: HashSet<ModelProviderCapability>,
}

impl Default for ModelCapabilitySet {
    fn default() -> Self {
        Self {
            caps: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderStatus {
    Ready,
    NotConfigured,
    MissingKey,
    Unreachable,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderHealth {
    Healthy,
    Degraded,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderConfig {
    pub id: String,
    pub name: String,
    pub kind: ModelProviderKind,
    #[serde(default)]
    pub enabled: bool,
    pub base_url: Option<String>,
    pub api_key_env: Option<String>,
    pub default_model: String,
    #[serde(default)]
    pub available_models: Vec<String>,
    #[serde(default)]
    pub capabilities: ModelCapabilitySet,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    #[serde(default)]
    pub local_only: bool,
    #[serde(default)]
    pub privacy_level: String,
    #[serde(default)]
    pub notes: String,
}

fn default_timeout() -> u64 {
    60
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelGatewayConfig {
    #[serde(default)]
    pub default_gateway: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRoutingConfig {
    #[serde(default = "default_true")]
    pub local_first: bool,
    #[serde(default)]
    pub allow_external_by_default: bool,
    #[serde(default = "default_true")]
    pub fallback_to_mock: bool,
    #[serde(default = "default_local_mock")]
    pub default_provider: String,
    #[serde(default = "default_mock_local")]
    pub default_model: String,
    #[serde(default = "default_local_mock")]
    pub default_coding_provider: String,
    #[serde(default = "default_local_mock")]
    pub default_research_provider: String,
    #[serde(default = "default_local_mock")]
    pub default_promptforge_provider: String,
    #[serde(default = "default_local_mock")]
    pub default_embedding_provider: String,
}

impl Default for ModelRoutingConfig {
    fn default() -> Self {
        Self {
            local_first: true,
            allow_external_by_default: false,
            fallback_to_mock: true,
            default_provider: "local_mock".to_string(),
            default_model: "mock-local".to_string(),
            default_coding_provider: "local_mock".to_string(),
            default_research_provider: "local_mock".to_string(),
            default_promptforge_provider: "local_mock".to_string(),
            default_embedding_provider: "local_mock".to_string(),
        }
    }
}

fn default_true() -> bool { true }
fn default_local_mock() -> String { "local_mock".to_string() }
fn default_mock_local() -> String { "mock-local".to_string() }

#[derive(Debug, Clone)]
pub struct ModelRouteRequest {
    pub agent_id: String,
    pub task_kind: String,
    pub required_capabilities: Vec<ModelProviderCapability>,
    pub local_only: bool,
    pub allow_external: bool,
    pub preferred_provider: Option<String>,
    pub preferred_model: Option<String>,
    pub quality_preference: String,
    pub latency_preference: String,
    pub cost_preference: String,
    pub fallback_allowed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelRouteDecision {
    pub provider_id: String,
    pub model: String,
    pub local_only: bool,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct ModelRoutingPolicy {
    pub config: ModelRoutingConfig,
}

impl ModelRoutingPolicy {
    pub fn new(config: ModelRoutingConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone)]
pub struct ModelFallbackPolicy {
    pub max_retries: u32,
}

pub struct ModelInvocationRequest {
    pub messages: Vec<crate::llm::Message>,
    pub tools: Option<Vec<crate::llm::Tool>>,
}

pub struct ModelInvocationResult {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<crate::llm::ToolCall>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelInvocationError {
    #[error("API Key Missing")]
    MissingKey,
    #[error("Network Error")]
    NetworkError,
    #[error("Unknown Provider")]
    UnknownProvider,
}

#[async_trait::async_trait]
pub trait ModelProviderAdapter: Send + Sync {
    fn status(&self) -> ModelProviderStatus;
    fn health(&self) -> ModelProviderHealth;
    async fn invoke(
        &self,
        model: &str,
        req: ModelInvocationRequest,
    ) -> Result<ModelInvocationResult, ModelInvocationError>;
}

pub struct ModelProviderRegistry {
    pub providers: HashMap<String, ModelProviderConfig>,
    pub routing_config: ModelRoutingConfig,
}

impl ModelProviderRegistry {
    pub fn new(routing_config: ModelRoutingConfig) -> Self {
        Self {
            providers: HashMap::new(),
            routing_config,
        }
    }

    pub fn register(&mut self, config: ModelProviderConfig) {
        self.providers.insert(config.id.clone(), config);
    }

    pub fn get_provider(&self, id: &str) -> Option<&ModelProviderConfig> {
        self.providers.get(id)
    }

    pub fn route(&self, req: &ModelRouteRequest) -> ModelRouteDecision {
        // Fallback logic
        let mut chosen_provider = self.routing_config.default_provider.clone();
        let mut chosen_model = self.routing_config.default_model.clone();
        let mut is_local = true;

        if let Some(pref_prov) = &req.preferred_provider {
            if let Some(p) = self.providers.get(pref_prov) {
                if !req.local_only || p.local_only {
                    chosen_provider = p.id.clone();
                    chosen_model = req.preferred_model.clone().unwrap_or(p.default_model.clone());
                    is_local = p.local_only;
                }
            }
        } else {
            // Task-based routing mapping defaults
            let mapped_prov = match req.task_kind.as_str() {
                "coding" | "tool_planning" => &self.routing_config.default_coding_provider,
                "research" | "summarization" => &self.routing_config.default_research_provider,
                "prompt_refinement" => &self.routing_config.default_promptforge_provider,
                "embedding" => &self.routing_config.default_embedding_provider,
                _ => &self.routing_config.default_provider,
            };

            if let Some(p) = self.providers.get(mapped_prov) {
                if !req.local_only || p.local_only {
                    chosen_provider = p.id.clone();
                    chosen_model = p.default_model.clone();
                    is_local = p.local_only;
                }
            }
        }

        ModelRouteDecision {
            provider_id: chosen_provider,
            model: chosen_model,
            local_only: is_local,
            notes: "Routed automatically".to_string(),
        }
    }
}

pub struct OpenAiCompatibleAdapter {
    pub client: reqwest::Client,
    pub base_url: String,
    pub api_key: Option<String>,
}

impl OpenAiCompatibleAdapter {
    pub fn new(base_url: String, api_key: Option<String>, timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .unwrap_or_default();
        Self {
            client,
            base_url,
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl ModelProviderAdapter for OpenAiCompatibleAdapter {
    fn status(&self) -> ModelProviderStatus {
        if self.api_key.is_some() || self.base_url.contains("localhost") {
            ModelProviderStatus::Ready
        } else {
            ModelProviderStatus::MissingKey
        }
    }

    fn health(&self) -> ModelProviderHealth {
        ModelProviderHealth::Unknown
    }

    async fn invoke(
        &self,
        model: &str,
        req: ModelInvocationRequest,
    ) -> Result<ModelInvocationResult, ModelInvocationError> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let mut body = serde_json::json!({
            "model": model,
            "messages": req.messages,
        });

        if let Some(tools) = req.tools {
            body["tools"] = serde_json::to_value(tools).unwrap_or_default();
        }

        let mut request = self.client.post(&url);
        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }

        let response = request
            .json(&body)
            .send()
            .await
            .map_err(|_| ModelInvocationError::NetworkError)?;

        if !response.status().is_success() {
            return Err(ModelInvocationError::NetworkError);
        }

        let openai_res: crate::llm::OpenAiResponse = response
            .json()
            .await
            .map_err(|_| ModelInvocationError::NetworkError)?;

        if let Some(choice) = openai_res.choices.into_iter().next() {
            Ok(ModelInvocationResult {
                content: choice.message.content,
                tool_calls: choice.message.tool_calls,
            })
        } else {
            Ok(ModelInvocationResult {
                content: None,
                tool_calls: None,
            })
        }
    }
}
