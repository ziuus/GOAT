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
    client: Client,
    /// Configuration for LLM retries, timeouts, and fallback policy.
    pub llm_config: LlmConfig,
    // ── OpenAI ────────────────────────────────────────────────────────────────
    pub openai_key: Option<String>,
    pub openai_base_url: Option<String>,
    // ── Groq ─────────────────────────────────────────────────────────────────
    pub groq_key: Option<String>,
    // ── OpenRouter ────────────────────────────────────────────────────────────
    pub openrouter_key: Option<String>,
    pub openrouter_base_url: String,
    // ── Ollama ────────────────────────────────────────────────────────────────
    pub ollama_base_url: String,
}

impl LlmRouter {
    /// Build from a loaded `Config`.
    pub fn from_config(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.llm.effective_timeout_secs()))
            .build()
            .unwrap_or_else(|_| Client::new());

        // OpenAI key — config → env → OpenCode fallback.
        let mut openai_key = config.provider_api_key("openai");
        let mut openai_base_url = config.provider_base_url("openai");

        if openai_key.is_none() {
            if let Some((key, url)) = Config::get_fallback_api_key() {
                openai_key = Some(key);
                openai_base_url = Some(url);
            }
        }

        // OpenRouter defaults.
        let openrouter_base_url = config
            .provider_base_url("openrouter")
            .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string());

        // Ollama defaults.
        let ollama_base_url = config
            .provider_base_url("ollama")
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        Self {
            client,
            llm_config: config.llm.clone(),
            openai_key,
            openai_base_url,
            groq_key: config.provider_api_key("groq"),
            openrouter_key: config.provider_api_key("openrouter"),
            openrouter_base_url,
            ollama_base_url,
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
        match provider {
            "openai" => self.openai_completion(model, messages, tools).await,
            "groq" => self.groq_completion(model, messages, tools).await,
            "openrouter" => self.openrouter_completion(model, messages, tools).await,
            "ollama" => self.ollama_completion(model, messages, tools).await,
            other => Err(ProviderError::UnknownProvider {
                provider: other.to_string(),
            }),
        }
    }

    // ── Provider status helpers ───────────────────────────────────────────────

    /// Whether a provider is coded and can receive requests.
    pub fn is_provider_implemented(&self, provider: &str) -> bool {
        matches!(provider, "openai" | "groq" | "openrouter" | "ollama")
    }

    /// Whether a provider has a key (or no key needed) and is ready to use.
    pub fn is_provider_available(&self, provider: &str) -> bool {
        match provider {
            "openai" => self.openai_key.is_some(),
            "groq" => self.groq_key.is_some(),
            "openrouter" => self.openrouter_key.is_some(),
            "ollama" => true, // local — no key needed; may still fail at call time
            _ => false,
        }
    }

    /// Human-readable status label for a provider (no secrets).
    pub fn provider_status_label(&self, provider: &str) -> &'static str {
        match provider {
            "openai" => {
                if self.openai_key.is_some() {
                    "ready"
                } else {
                    "not configured (set OPENAI_API_KEY or openai_api_key in config)"
                }
            }
            "groq" => {
                if self.groq_key.is_some() {
                    "ready"
                } else {
                    "not configured (set GROQ_API_KEY or groq_api_key in config)"
                }
            }
            "openrouter" => {
                if self.openrouter_key.is_some() {
                    "ready"
                } else {
                    "not configured (set OPENROUTER_API_KEY or openrouter_api_key in config)"
                }
            }
            "ollama" => "local (no key required — server must be running)",
            "anthropic" => "planned — not implemented",
            "gemini" => "planned — not implemented",
            _ => "unknown provider",
        }
    }

    // ── Provider implementations ──────────────────────────────────────────────

    async fn openai_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let key = self
            .openai_key
            .as_ref()
            .ok_or(ProviderError::NotConfigured {
                provider: "openai".to_string(),
            })?;
        let base_url = self
            .openai_base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
        self.call_openai_compatible("openai", &url, key, model, messages, tools)
            .await
    }

    async fn groq_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let key = self.groq_key.as_ref().ok_or(ProviderError::NotConfigured {
            provider: "groq".to_string(),
        })?;
        self.call_openai_compatible(
            "groq",
            "https://api.groq.com/openai/v1/chat/completions",
            key,
            model,
            messages,
            tools,
        )
        .await
    }

    /// OpenRouter — OpenAI-compatible API with required HTTP headers.
    ///
    /// OpenRouter requires `HTTP-Referer` and `X-Title` headers for routing.
    async fn openrouter_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let key = self
            .openrouter_key
            .as_ref()
            .ok_or(ProviderError::NotConfigured {
                provider: "openrouter".to_string(),
            })?;
        let url = format!(
            "{}/chat/completions",
            self.openrouter_base_url.trim_end_matches('/')
        );

        let req_body = OpenAiRequest {
            model: model.to_string(),
            messages,
            tools,
        };

        let response = self
            .client
            .post(&url)
            .bearer_auth(key)
            .header("HTTP-Referer", "https://github.com/ziuus/GOAT")
            .header("X-Title", "GOAT")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| self.classify_network_error("openrouter", e))?;

        self.parse_openai_response("openrouter", model, response)
            .await
    }

    /// Ollama — OpenAI-compatible endpoint at `/v1/chat/completions`.
    ///
    /// Ollama must be running locally. No API key required.
    async fn ollama_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let url = format!(
            "{}/v1/chat/completions",
            self.ollama_base_url.trim_end_matches('/')
        );

        let req_body = OpenAiRequest {
            model: model.to_string(),
            messages,
            // Ollama tool call support varies by model — pass through.
            tools,
        };

        let response = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| {
                // Connection refused → Ollama is not running.
                if e.is_connect() {
                    ProviderError::NetworkError {
                        provider: "ollama".to_string(),
                        detail: format!(
                            "cannot connect to Ollama at {} — is Ollama running?",
                            self.ollama_base_url
                        ),
                    }
                } else {
                    self.classify_network_error("ollama", e)
                }
            })?;

        self.parse_openai_response("ollama", model, response).await
    }

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
