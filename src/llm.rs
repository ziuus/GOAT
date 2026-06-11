//! LLM provider router with fallback chain and retry policy.
//!
//! `LlmRouter` dispatches completion requests to OpenAI-compatible providers.
//!
//! # Working providers
//!
//! | Provider    | API Style      | Requires          |
//! |-------------|----------------|-------------------|
//! | openai      | OpenAI API     | OPENAI_API_KEY    |
//! | groq        | OpenAI-compat  | GROQ_API_KEY      |
//! | openrouter  | OpenAI-compat  | OPENROUTER_API_KEY|
//! | ollama      | OpenAI-compat  | local server      |
//!
//! Planned (not implemented): `anthropic`, `gemini`.
//!
//! # Retry policy (configurable via `[llm]` in goat.toml)
//!
//! - Retryable (NetworkError, ServerError): retry same entry up to `max_retries`.
//! - Recoverable (RateLimit, ServerError, NetworkError): advance chain after retries.
//! - Non-recoverable (AuthFailed, BadRequest, ModelNotFound): stop immediately.
//! - Fallback can be disabled per error class via `fallback_on_*` config flags.

use crate::config::{Config, LlmConfig};
use crate::models::{ModelChain, ModelEntry};
use crate::provider::ProviderError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn};

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Clone)]
pub struct Tool {
    pub r#type: String,
    pub function: FunctionDeclaration,
}

#[derive(Serialize, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: MessageContent,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageContent {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: Option<String>,
}

// ── LlmRouter ─────────────────────────────────────────────────────────────────

pub struct LlmRouter {
    pub client: Client,
    pub llm_config: LlmConfig,
    pub registry: crate::providers::ModelProviderRegistry,
    // We keep config around to resolve API keys dynamically at request time
    pub config: Config,
}

impl LlmRouter {
    /// Build from a loaded `Config`.
    pub fn from_config(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.llm.effective_timeout_secs()))
            .build()
            .unwrap_or_else(|_| Client::new());

        let mut registry = crate::providers::ModelProviderRegistry::new(config.model_routing.clone());
        for (_, p_cfg) in &config.providers {
            registry.register(p_cfg.clone());
        }

        Self {
            client,
            llm_config: config.llm.clone(),
            registry,
            config: config.clone(),
        }
    }

    /// Legacy constructor (kept for backward compat with tests).
    pub fn new(openai_key: Option<String>, groq_key: Option<String>) -> Self {
        let mut config = Config::default();
        config.keys.openai_api_key = openai_key;
        config.keys.groq_api_key = groq_key;
        Self::from_config(&config)
    }

    // ── Fallback chain ────────────────────────────────────────────────────────

    /// Try each entry in `chain` in order until one succeeds.
    ///
    /// Returns `(MessageContent, used_provider_label)`.
    pub async fn completion_with_fallback(
        &self,
        chain: &ModelChain,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<(MessageContent, String), ProviderError> {
        if chain.is_empty() {
            return Err(ProviderError::ChainExhausted { count: 0 });
        }

        let total = chain.len();
        for (i, entry) in chain.entries.iter().enumerate() {
            let label = entry.display();

            // Skip providers that are planned but not implemented.
            if !self.is_provider_implemented(&entry.provider) {
                warn!(entry = %label, "skipping unimplemented provider in chain");
                continue;
            }

            // Skip providers that are not available (no key / not running).
            if !self.is_provider_available(&entry.provider) {
                warn!(entry = %label, "provider not available (no key/connection) — skipping");
                continue;
            }

            let result = self
                .try_with_retry(entry, messages.clone(), tools.clone())
                .await;

            match result {
                Ok(content) => {
                    info!(entry = %label, attempt = i + 1, total, "chain entry succeeded");
                    return Ok((content, label));
                }
                Err(ref e) if !self.is_error_fallback_allowed(e) => {
                    warn!(entry = %label, error = %e, "non-recoverable or fallback-disabled error — stopping chain");
                    return Err(e.clone());
                }
                Err(ref e) => {
                    warn!(
                        entry = %label,
                        error = %e,
                        remaining = total.saturating_sub(i + 1),
                        "recoverable error — advancing chain"
                    );
                }
            }
        }

        Err(ProviderError::ChainExhausted { count: total })
    }

    /// Whether the LLM config policy permits advancing the chain for this error.
    fn is_error_fallback_allowed(&self, e: &ProviderError) -> bool {
        if !e.is_recoverable() {
            return false;
        }
        match e {
            ProviderError::RateLimit { .. } => self.llm_config.fallback_on_rate_limit,
            ProviderError::NetworkError { .. } => self.llm_config.fallback_on_network,
            ProviderError::ServerError { .. } => self.llm_config.fallback_on_server_error,
            _ => e.is_recoverable(),
        }
    }

    /// Single-provider call with retry.
    async fn try_with_retry(
        &self,
        entry: &ModelEntry,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let max = self.llm_config.effective_max_retries();
        let mut last_err: Option<ProviderError> = None;

        for attempt in 0..=max {
            if attempt > 0 {
                let delay = Duration::from_millis(500 * u64::from(attempt));
                info!(
                    entry = %entry.display(),
                    attempt,
                    delay_ms = delay.as_millis(),
                    "retrying after transient error"
                );
                tokio::time::sleep(delay).await;
            }

            match self
                .completion(
                    &entry.provider,
                    &entry.model,
                    messages.clone(),
                    tools.clone(),
                )
                .await
            {
                Ok(content) => return Ok(content),
                Err(e) if e.is_retryable() && attempt < max => {
                    warn!(entry = %entry.display(), attempt, error = %e, "retryable error");
                    last_err = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_err.unwrap_or(ProviderError::ChainExhausted { count: 0 }))
    }

    // ── Single-provider completion (raw) ──────────────────────────────────────

    pub async fn completion(
        &self,
        provider: &str,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let p_cfg = self.registry.get_provider(provider).ok_or_else(|| ProviderError::UnknownProvider {
            provider: provider.to_string(),
        })?;

        // Extract base URL and API key
        let base_url = p_cfg.base_url.clone().unwrap_or_default();
        let api_key = self.config.provider_api_key(provider).unwrap_or_default();

        let mut url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
        
        // OpenRouter needs /api/v1
        if p_cfg.kind == crate::providers::ModelProviderKind::Openrouter && !url.contains("/api/v1") && !url.contains("/chat/completions") {
            // we assume openrouter uses openai compatible url if the url is default
            if url.is_empty() {
                url = "https://openrouter.ai/api/v1/chat/completions".to_string();
            }
        }
        // OpenAI default
        if p_cfg.kind == crate::providers::ModelProviderKind::Openai && url == "/chat/completions" {
            url = "https://api.openai.com/v1/chat/completions".to_string();
        }
        // Groq default
        if p_cfg.kind == crate::providers::ModelProviderKind::Groq && url == "/chat/completions" {
            url = "https://api.groq.com/openai/v1/chat/completions".to_string();
        }

        self.call_openai_compatible(provider, &url, &api_key, model, messages, tools).await
    }

    // ── Provider status helpers ───────────────────────────────────────────────

    /// Whether a provider is coded and can receive requests.
    pub fn is_provider_implemented(&self, provider: &str) -> bool {
        self.registry.get_provider(provider).is_some()
    }

    /// Whether a provider has a key (or no key needed) and is ready to use.
    pub fn is_provider_available(&self, provider: &str) -> bool {
        if let Some(p_cfg) = self.registry.get_provider(provider) {
            if p_cfg.local_only || p_cfg.kind == crate::providers::ModelProviderKind::Ollama {
                true
            } else {
                self.config.provider_api_key(provider).is_some()
            }
        } else {
            false
        }
    }

    /// Human-readable status label for a provider (no secrets).
    pub fn provider_status_label(&self, provider: &str) -> String {
        if let Some(p_cfg) = self.registry.get_provider(provider) {
            if p_cfg.local_only || p_cfg.kind == crate::providers::ModelProviderKind::Ollama {
                "local (no key required — server must be running)".to_string()
            } else {
                if self.config.provider_api_key(provider).is_some() {
                    "ready".to_string()
                } else {
                    format!("not configured (set {}_API_KEY in env or config)", provider.to_uppercase())
                }
            }
        } else {
            "unknown provider".to_string()
        }
    }

    // ── Provider implementations (removed, now uses call_openai_compatible dynamically) ──────────────────────────────────────────────


    // ── Shared HTTP helpers ───────────────────────────────────────────────────

    async fn call_openai_compatible(
        &self,
        provider: &str,
        url: &str,
        key: &str,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let req_body = OpenAiRequest {
            model: model.to_string(),
            messages,
            tools,
        };

        let response = self
            .client
            .post(url)
            .bearer_auth(key)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| self.classify_network_error(provider, e))?;

        self.parse_openai_response(provider, model, response).await
    }

    async fn parse_openai_response(
        &self,
        provider: &str,
        model: &str,
        response: reqwest::Response,
    ) -> Result<MessageContent, ProviderError> {
        let status = response.status().as_u16();
        if !response.status().is_success() {
            let err_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::from_http(provider, model, status, &err_text));
        }
        let res_body: OpenAiResponse = response.json().await.map_err(|e| ProviderError::Other {
            provider: provider.to_string(),
            detail: format!("failed to parse response: {}", e),
        })?;
        res_body
            .choices
            .into_iter()
            .next()
            .map(|c| c.message)
            .ok_or(ProviderError::Other {
                provider: provider.to_string(),
                detail: "no choices returned".to_string(),
            })
    }

    fn classify_network_error(&self, provider: &str, e: reqwest::Error) -> ProviderError {
        if e.is_timeout() {
            ProviderError::NetworkError {
                provider: provider.to_string(),
                detail: "request timed out".to_string(),
            }
        } else if e.is_connect() {
            ProviderError::NetworkError {
                provider: provider.to_string(),
                detail: "connection refused or unreachable".to_string(),
            }
        } else {
            ProviderError::NetworkError {
                provider: provider.to_string(),
                detail: e.to_string(),
            }
        }
    }
}
