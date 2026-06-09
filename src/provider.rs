//! Provider abstraction layer for GOAT's LLM backend.
//!
//! Defines the types used across the provider system:
//! - Error classification (recoverable vs non-recoverable)
//! - Provider status
//! - Completion request/response wrappers
//!
//! # Recoverable vs Non-Recoverable Errors
//!
//! The fallback chain only advances on **recoverable** errors:
//!
//! | Error kind          | Recoverable? | Notes                          |
//! |---------------------|--------------|--------------------------------|
//! | Rate limit (429)    | Yes          | Try next provider/model        |
//! | Network / timeout   | Yes          | Retry then try next            |
//! | Server error (5xx)  | Yes          | Provider temporarily down      |
//! | Auth error (401)    | No           | Bad API key — stop immediately |
//! | Bad request (400)   | No           | Invalid payload — stop         |
//! | Not found (404)     | No           | Wrong model name — stop        |
//! | Tool denied by user | No           | User decision — no retry       |
//! | No providers left   | No           | All exhausted                  |
//!
//! # Provider IDs
//!
//! GOAT uses string identifiers for providers:
//! - `"openai"` — OpenAI API (working)
//! - `"groq"` — Groq API (working)
//! - `"anthropic"` — Planned (not implemented)
//! - `"gemini"` — Planned (not implemented)
//! - `"ollama"` — Planned (not implemented)
//! - `"openrouter"` — Planned (not implemented)
//!
//! Only `"openai"` and `"groq"` are functional in this release.

use thiserror::Error;

// ── Provider error ────────────────────────────────────────────────────────────

/// Error returned from a provider completion attempt.
#[derive(Debug, Error, Clone)]
pub enum ProviderError {
    /// HTTP 429 — rate limit hit. Recoverable.
    #[error("rate limit hit on {provider}:{model}")]
    RateLimit { provider: String, model: String },

    /// HTTP 401/403 — authentication failure. Non-recoverable.
    #[error("authentication failed for {provider} — check your API key")]
    AuthFailed { provider: String },

    /// HTTP 400 — bad request (invalid model, bad payload). Non-recoverable.
    #[error("bad request to {provider}:{model}: {detail}")]
    BadRequest {
        provider: String,
        model: String,
        detail: String,
    },

    /// HTTP 404 — model not found. Non-recoverable (wrong model name).
    #[error("model not found: {provider}:{model}")]
    ModelNotFound { provider: String, model: String },

    /// HTTP 5xx — server error. Recoverable.
    #[error("server error from {provider} (status {status})")]
    ServerError { provider: String, status: u16 },

    /// Network / timeout error. Recoverable.
    #[error("network or timeout error talking to {provider}: {detail}")]
    NetworkError { provider: String, detail: String },

    /// Provider not configured (missing API key). Non-recoverable for this provider.
    #[error("{provider} not configured — API key is missing")]
    NotConfigured { provider: String },

    /// The requested provider name is unknown to GOAT. Non-recoverable.
    #[error("unknown provider '{provider}'")]
    UnknownProvider { provider: String },

    /// All models in the fallback chain have been exhausted. Non-recoverable.
    #[error("all {count} models in chain exhausted — no response available")]
    ChainExhausted { count: usize },

    /// Any other error not classified above. Treated as non-recoverable by default.
    #[error("provider error from {provider}: {detail}")]
    Other { provider: String, detail: String },
}

impl ProviderError {
    /// Whether GOAT's fallback chain should attempt the next model in the chain
    /// after seeing this error.  Conservative: when in doubt, do NOT advance.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ProviderError::RateLimit { .. }
                | ProviderError::ServerError { .. }
                | ProviderError::NetworkError { .. }
        )
    }

    /// Whether it is worth retrying the *same* provider/model (e.g. transient timeout).
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ProviderError::NetworkError { .. } | ProviderError::ServerError { .. }
        )
    }

    /// Classify an HTTP status code + error text into a `ProviderError`.
    pub fn from_http(provider: &str, model: &str, status: u16, body: &str) -> Self {
        match status {
            429 => ProviderError::RateLimit {
                provider: provider.to_string(),
                model: model.to_string(),
            },
            401 | 403 => ProviderError::AuthFailed {
                provider: provider.to_string(),
            },
            400 => ProviderError::BadRequest {
                provider: provider.to_string(),
                model: model.to_string(),
                detail: truncate(body, 120),
            },
            404 => ProviderError::ModelNotFound {
                provider: provider.to_string(),
                model: model.to_string(),
            },
            500..=599 => ProviderError::ServerError {
                provider: provider.to_string(),
                status,
            },
            _ => ProviderError::Other {
                provider: provider.to_string(),
                detail: format!("HTTP {} — {}", status, truncate(body, 120)),
            },
        }
    }
}

// ── Provider status ───────────────────────────────────────────────────────────

/// Runtime status of a configured provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderStatus {
    /// API key is present and the provider is ready to use.
    Ready,
    /// No API key configured — provider is unusable.
    NotConfigured,
    /// Provider is planned but not yet implemented.
    Planned,
}

impl ProviderStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ProviderStatus::Ready => "ready",
            ProviderStatus::NotConfigured => "not configured",
            ProviderStatus::Planned => "planned (not implemented)",
        }
    }
}

/// Summary of a single provider for display in `goat doctor` or `goat models`.
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    /// Short identifier (e.g. "openai").
    pub id: String,
    /// Human-readable name (e.g. "OpenAI").
    pub name: String,
    /// Current runtime status.
    pub status: ProviderStatus,
    /// Models supported or planned for this provider.
    pub models: Vec<String>,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_is_recoverable() {
        let e = ProviderError::RateLimit {
            provider: "openai".into(),
            model: "gpt-4o-mini".into(),
        };
        assert!(e.is_recoverable());
        assert!(!e.is_retryable()); // rate limit → advance chain, not retry same
    }

    #[test]
    fn test_network_error_is_recoverable_and_retryable() {
        let e = ProviderError::NetworkError {
            provider: "groq".into(),
            detail: "timeout".into(),
        };
        assert!(e.is_recoverable());
        assert!(e.is_retryable());
    }

    #[test]
    fn test_auth_error_is_not_recoverable() {
        let e = ProviderError::AuthFailed {
            provider: "openai".into(),
        };
        assert!(!e.is_recoverable());
        assert!(!e.is_retryable());
    }

    #[test]
    fn test_bad_request_not_recoverable() {
        let e = ProviderError::BadRequest {
            provider: "openai".into(),
            model: "gpt-4o".into(),
            detail: "invalid parameter".into(),
        };
        assert!(!e.is_recoverable());
    }

    #[test]
    fn test_server_error_is_recoverable_and_retryable() {
        let e = ProviderError::ServerError {
            provider: "openai".into(),
            status: 503,
        };
        assert!(e.is_recoverable());
        assert!(e.is_retryable());
    }

    #[test]
    fn test_from_http_429() {
        let e = ProviderError::from_http("openai", "gpt-4o", 429, "rate limited");
        assert!(matches!(e, ProviderError::RateLimit { .. }));
        assert!(e.is_recoverable());
    }

    #[test]
    fn test_from_http_401() {
        let e = ProviderError::from_http("openai", "gpt-4o", 401, "unauthorized");
        assert!(matches!(e, ProviderError::AuthFailed { .. }));
        assert!(!e.is_recoverable());
    }

    #[test]
    fn test_from_http_500() {
        let e = ProviderError::from_http("groq", "llama3", 500, "internal error");
        assert!(matches!(e, ProviderError::ServerError { .. }));
        assert!(e.is_recoverable());
    }

    #[test]
    fn test_chain_exhausted_not_recoverable() {
        let e = ProviderError::ChainExhausted { count: 2 };
        assert!(!e.is_recoverable());
    }
}
