use crate::config::EmbeddingsConfig;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingVector {
    pub document_id: String,
    pub provider: String,
    pub model: String,
    pub dimensions: usize,
    pub vector: Vec<f32>,
    pub created_at: String,
    pub content_hash: String,
}

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Embeddings are disabled")]
    Disabled,
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingProviderKind {
    None,
    Mock,
    Ollama,
}

pub struct NoneProvider;

impl NoneProvider {
    async fn generate(&self, _text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Err(EmbeddingError::Disabled)
    }

    fn dimensions(&self) -> usize {
        0
    }

    fn kind(&self) -> EmbeddingProviderKind {
        EmbeddingProviderKind::None
    }
}

pub struct MockProvider {
    pub dimensions: usize,
}

impl MockProvider {
    async fn generate(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let mut vec = vec![0.0; self.dimensions];
        let hash_val = text.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));
        for (i, v) in vec.iter_mut().enumerate() {
            *v = ((hash_val.wrapping_add(i as u32)) as f32 / 1000.0).sin();
        }
        Ok(vec)
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn kind(&self) -> EmbeddingProviderKind {
        EmbeddingProviderKind::Mock
    }
}

pub struct OllamaProvider {
    base_url: String,
    model: String,
    dimensions: usize,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(config: &EmbeddingsConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap_or_default();
        Self {
            base_url: config.base_url.clone(),
            model: config.model.clone(),
            dimensions: config.dimensions,
            client,
        }
    }

    async fn generate(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        #[derive(Serialize)]
        struct OllamaReq<'a> {
            model: &'a str,
            prompt: &'a str,
        }
        #[derive(Deserialize)]
        struct OllamaRes {
            embedding: Vec<f32>,
        }

        let req = OllamaReq {
            model: &self.model,
            prompt: text,
        };

        let url = format!("{}/api/embeddings", self.base_url.trim_end_matches('/'));
        let res = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| {
                EmbeddingError::NetworkError(format!(
                    "Failed to connect to Ollama at {}: {}",
                    url, e
                ))
            })?;

        if !res.status().is_success() {
            return Err(EmbeddingError::InvalidResponse(format!(
                "Ollama returned status: {}",
                res.status()
            )));
        }

        let data: OllamaRes = res.json().await.map_err(|e| {
            EmbeddingError::InvalidResponse(format!("Failed to parse Ollama response: {}", e))
        })?;

        let mut v = data.embedding;
        if v.len() != self.dimensions {
            v.resize(self.dimensions, 0.0);
        }

        Ok(v)
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn kind(&self) -> EmbeddingProviderKind {
        EmbeddingProviderKind::Ollama
    }
}

pub enum EmbeddingProvider {
    None(NoneProvider),
    Mock(MockProvider),
    Ollama(OllamaProvider),
}

impl EmbeddingProvider {
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        match self {
            Self::None(p) => p.generate(text).await,
            Self::Mock(p) => p.generate(text).await,
            Self::Ollama(p) => p.generate(text).await,
        }
    }

    pub fn dimensions(&self) -> usize {
        match self {
            Self::None(p) => p.dimensions(),
            Self::Mock(p) => p.dimensions(),
            Self::Ollama(p) => p.dimensions(),
        }
    }

    pub fn kind(&self) -> EmbeddingProviderKind {
        match self {
            Self::None(p) => p.kind(),
            Self::Mock(p) => p.kind(),
            Self::Ollama(p) => p.kind(),
        }
    }
}

pub fn create_provider(config: &EmbeddingsConfig) -> EmbeddingProvider {
    if !config.enabled {
        return EmbeddingProvider::None(NoneProvider);
    }
    match config.provider.to_lowercase().as_str() {
        "ollama" => EmbeddingProvider::Ollama(OllamaProvider::new(config)),
        "mock" => EmbeddingProvider::Mock(MockProvider {
            dimensions: config.dimensions,
        }),
        _ => EmbeddingProvider::None(NoneProvider),
    }
}
