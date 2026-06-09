//! LLM provider router with fallback chain and retry policy.
//!
//! `LlmRouter` dispatches completion requests to OpenAI-compatible providers.
//! The public API:
//!
//! - [`LlmRouter::completion`] — single-provider call (raw, no fallback)
//! - [`LlmRouter::completion_with_fallback`] — tries each entry in a
//!   [`ModelChain`] until one succeeds or all are exhausted
//!
//! # Retry policy
//!
//! On retryable errors (network timeout, 5xx), the same provider/model is
//! retried up to `MAX_RETRIES` times before advancing to the next chain entry.
//! Non-retryable errors (401 auth, 400 bad request, 404 model not found) stop
//! immediately — never retried, never advance the chain for that entry.
//!
//! # Fallback chain
//!
//! On recoverable errors (rate limit, server error, network) AFTER retries are
//! exhausted, the chain advances to the next `provider:model` entry.
//! Auth failures (401) are non-recoverable and stop the chain immediately to
//! avoid burning time on subsequent providers.
//!
//! # Working providers
//!
//! - `openai` — OpenAI API (https://api.openai.com/v1)
//! - `groq` — Groq API (https://api.groq.com/openai/v1)
//!
//! Planned (not implemented): `anthropic`, `gemini`, `ollama`, `openrouter`.

use crate::models::{ModelChain, ModelEntry};
use crate::provider::ProviderError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Maximum times to retry the *same* provider/model on retryable errors
/// before advancing the chain.
const MAX_RETRIES: u32 = 2;

/// Request timeout per attempt.
const REQUEST_TIMEOUT_SECS: u64 = 120;

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

// Internal OpenAI-compatible request body.
#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

// OpenAI-compatible response body.
#[derive(Deserialize, Debug)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: MessageContent,
    pub finish_reason: Option<String>,
}

/// Response content from the model — returned to the agent loop.
#[derive(Deserialize, Debug)]
pub struct MessageContent {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: Option<String>,
}

// ── LlmRouter ─────────────────────────────────────────────────────────────────

pub struct LlmRouter {
    client: Client,
    pub openai_key: Option<String>,
    pub openai_base_url: Option<String>,
    pub groq_key: Option<String>,
}

impl LlmRouter {
    /// Create a new router from explicit keys.
    ///
    /// Falls back to `OPENAI_API_KEY` / `GROQ_API_KEY` env vars if not provided.
    /// Falls back to the OpenCode/freellmapi key if nothing else is configured.
    pub fn new(openai_key: Option<String>, groq_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| Client::new());

        let mut final_openai_key = openai_key.or_else(|| std::env::var("OPENAI_API_KEY").ok());
        let mut final_base_url = None;

        if final_openai_key.is_none() {
            if let Some((key, url)) = crate::config::Config::get_fallback_api_key() {
                final_openai_key = Some(key);
                final_base_url = Some(url);
            }
        }

        Self {
            client,
            openai_key: final_openai_key,
            openai_base_url: final_base_url,
            groq_key: groq_key.or_else(|| std::env::var("GROQ_API_KEY").ok()),
        }
    }

    // ── Single-provider completion (raw, no fallback) ─────────────────────────

    /// Call a specific provider/model directly.
    ///
    /// Returns a typed [`ProviderError`] on failure — callers can decide
    /// whether to fall back or propagate the error.
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
            other => Err(ProviderError::UnknownProvider {
                provider: other.to_string(),
            }),
        }
    }

    // ── Fallback chain ────────────────────────────────────────────────────────

    /// Try each entry in `chain` in order until one succeeds.
    ///
    /// Returns the successful response and the `provider:model` string that
    /// was used (so the caller can log/display it).
    ///
    /// # Fallback rules
    /// - Retryable errors (network, 5xx): retry same entry up to `MAX_RETRIES`.
    /// - Recoverable errors (rate limit, 5xx after retries): advance chain.
    /// - Non-recoverable (401 auth, 400 bad request, 404): stop immediately.
    /// - `ollama` / unknown providers: skip with a warning (not implemented).
    /// - All entries exhausted: return [`ProviderError::ChainExhausted`].
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
            if !self.is_provider_available(&entry.provider) {
                warn!(
                    entry = %label,
                    "skipping unimplemented provider in chain"
                );
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
                Err(ref e) if !e.is_recoverable() => {
                    // Non-recoverable — stop chain immediately.
                    warn!(
                        entry = %label,
                        error = %e,
                        "non-recoverable error — stopping chain"
                    );
                    return Err(e.clone());
                }
                Err(ref e) => {
                    // Recoverable — log and try next entry.
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

    /// Retry a single chain entry up to `MAX_RETRIES` times on retryable errors.
    async fn try_with_retry(
        &self,
        entry: &ModelEntry,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, ProviderError> {
        let mut last_err: Option<ProviderError> = None;

        for attempt in 0..=MAX_RETRIES {
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
                Err(e) if e.is_retryable() && attempt < MAX_RETRIES => {
                    warn!(entry = %entry.display(), attempt, error = %e, "retryable error");
                    last_err = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_err.unwrap_or(ProviderError::ChainExhausted { count: 0 }))
    }

    // ── Provider status helpers ───────────────────────────────────────────────

    /// Whether a provider is implemented and has a key configured.
    pub fn is_provider_available(&self, provider: &str) -> bool {
        match provider {
            "openai" => self.openai_key.is_some(),
            "groq" => self.groq_key.is_some(),
            // These are planned but not implemented — always unavailable.
            "anthropic" | "gemini" | "ollama" | "openrouter" => false,
            _ => false,
        }
    }

    /// Whether a provider is implemented (has code, regardless of key).
    pub fn is_provider_implemented(provider: &str) -> bool {
        matches!(provider, "openai" | "groq")
    }

    /// Human-readable status for a provider (for doctor/models output).
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
            "anthropic" => "planned — not implemented",
            "gemini" => "planned — not implemented",
            "ollama" => "planned — not implemented",
            "openrouter" => "planned — not implemented",
            _ => "unknown provider",
        }
    }

    // ── Provider-specific implementations ─────────────────────────────────────

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
            .map_err(|e| {
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
            })?;

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
}
