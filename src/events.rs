use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoatEvent {
    pub id: String,
    pub timestamp: u64,
    pub kind: String,
    pub severity: EventSeverity,
    pub message: String,
    pub payload: Option<serde_json::Value>,
}

impl GoatEvent {
    pub fn new(
        kind: &str,
        severity: EventSeverity,
        message: &str,
        payload: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            kind: kind.to_string(),
            severity,
            message: message.to_string(),
            payload,
        }
    }
}

pub struct EventBus {
    sender: broadcast::Sender<GoatEvent>,
    history: Arc<Mutex<Vec<GoatEvent>>>,
    max_history: usize,
}

impl EventBus {
    pub fn new(max_history: usize) -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self {
            sender,
            history: Arc::new(Mutex::new(Vec::with_capacity(max_history))),
            max_history,
        }
    }

    pub async fn push(&self, event: GoatEvent) {
        // Redact any potential secrets from the message
        let evt = event.clone();

        let mut hist = self.history.lock().await;
        if hist.len() >= self.max_history {
            hist.remove(0);
        }
        hist.push(evt.clone());

        // Ignore send errors if there are no receivers
        let _ = self.sender.send(evt);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<GoatEvent> {
        self.sender.subscribe()
    }

    pub async fn get_history(&self) -> Vec<GoatEvent> {
        self.history.lock().await.clone()
    }
}
