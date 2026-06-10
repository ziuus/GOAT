use crate::config::TransportConfig;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportProviderKind {
    LocalWebhook,
    Desktop,
    Telegram,
    Discord,
    Slack,
    Email,
    VoiceNote,
    ManualStub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportSession {
    pub id: String,
    pub provider: TransportProviderKind,
    pub active: bool,
    pub started_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportMessageKind {
    Text,
    Audio,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportMessage {
    pub id: String,
    pub session_id: String,
    pub kind: TransportMessageKind,
    pub direction: TransportDirection,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportAction {
    SendMessage(String, String), // session_id, message
    CloseSession(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportActionResult {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportAuditRecord {
    pub timestamp: u64,
    pub provider: TransportProviderKind,
    pub event_type: String,
    pub description: String,
}

#[async_trait::async_trait]
pub trait TransportProvider: Send + Sync {
    fn kind(&self) -> TransportProviderKind;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn check_health(&self) -> Result<String>;
    async fn send_message(&self, session_id: &str, content: &str) -> Result<TransportActionResult>;
}

struct ManualStubProvider;

#[async_trait::async_trait]
impl TransportProvider for ManualStubProvider {
    fn kind(&self) -> TransportProviderKind {
        TransportProviderKind::ManualStub
    }

    async fn start(&self) -> Result<()> {
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    async fn check_health(&self) -> Result<String> {
        Ok("ManualStub Transport is healthy".into())
    }

    async fn send_message(
        &self,
        _session_id: &str,
        _content: &str,
    ) -> Result<TransportActionResult> {
        Ok(TransportActionResult {
            success: true,
            error: None,
        })
    }
}

pub struct TransportManager {
    config: TransportConfig,
    providers: HashMap<String, Arc<dyn TransportProvider>>,
    sessions: HashMap<String, TransportSession>,
    messages: Vec<TransportMessage>,
    audit_logs: Vec<TransportAuditRecord>,
}

impl TransportManager {
    pub fn new(config: TransportConfig) -> Self {
        let mut providers: HashMap<String, Arc<dyn TransportProvider>> = HashMap::new();
        providers.insert("manual_stub".to_string(), Arc::new(ManualStubProvider));

        Self {
            config,
            providers,
            sessions: HashMap::new(),
            messages: Vec::new(),
            audit_logs: Vec::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub async fn check_doctor(&self) -> Result<String> {
        if !self.config.enabled {
            return Ok("Transports disabled in config.".into());
        }
        let mut status = String::new();
        for (name, provider) in &self.providers {
            match provider.check_health().await {
                Ok(h) => status.push_str(&format!("{}: {}\n", name, h)),
                Err(e) => status.push_str(&format!("{}: Error - {}\n", name, e)),
            }
        }
        Ok(status.trim().to_string())
    }

    pub fn list_sessions(&self) -> Vec<TransportSession> {
        self.sessions.values().cloned().collect()
    }

    pub fn get_messages(&self) -> Vec<TransportMessage> {
        self.messages.clone()
    }

    pub fn get_session_messages(&self, session_id: &str) -> Vec<TransportMessage> {
        self.messages
            .iter()
            .filter(|m| m.session_id == session_id)
            .cloned()
            .collect()
    }

    pub fn create_session(&mut self, provider: TransportProviderKind) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let session = TransportSession {
            id: id.clone(),
            provider: provider.clone(),
            active: true,
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.sessions.insert(id.clone(), session);
        self.audit_logs.push(TransportAuditRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            provider,
            event_type: "session_created".into(),
            description: format!("Session {} created", id),
        });
        id
    }

    pub fn record_inbound(&mut self, session_id: &str, content: &str) {
        let msg = TransportMessage {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            kind: TransportMessageKind::Text,
            direction: TransportDirection::Inbound,
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.messages.push(msg);
    }

    pub async fn send_outbound(&mut self, session_id: &str, content: &str) -> Result<()> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| anyhow!("Session not found"))?;
        let provider_name = match session.provider {
            TransportProviderKind::ManualStub => "manual_stub",
            _ => return Err(anyhow!("Provider not implemented for outbound")),
        };

        if let Some(provider) = self.providers.get(provider_name) {
            let _ = provider.send_message(session_id, content).await?;
            let msg = TransportMessage {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                kind: TransportMessageKind::Text,
                direction: TransportDirection::Outbound,
                content: content.to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            self.messages.push(msg);
            Ok(())
        } else {
            Err(anyhow!("Provider missing"))
        }
    }

    pub fn clear_session(&mut self, session_id: &str) {
        self.sessions.remove(session_id);
        self.messages.retain(|m| m.session_id != session_id);
    }
}
