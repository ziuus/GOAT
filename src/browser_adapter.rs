#![allow(dead_code)]
use crate::config::BrowserConfig;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserProviderKind {
    None,
    ManualStub,
    Playwright,
    BrowserUse,
    ExternalCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserSessionStatus {
    Starting,
    Active,
    Idle,
    Closed,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    pub id: String,
    pub provider: BrowserProviderKind,
    pub status: BrowserSessionStatus,
    pub started_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserActionKind {
    OpenUrl(String),
    Reload,
    Back,
    Forward,
    Screenshot,
    ReadText,
    InspectDom,
    Click(String),
    TypeText(String, String),
    SubmitForm(String),
    Wait(u64),
    EvaluateReadonlyJs(String),
    DownloadFile(String),
    CloseSession,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl BrowserActionKind {
    pub fn risk_level(&self, url: &str) -> BrowserRiskLevel {
        let is_local = url.starts_with("http://localhost") || url.starts_with("http://127.0.0.1");

        match self {
            BrowserActionKind::ReadText | BrowserActionKind::InspectDom => BrowserRiskLevel::Low,
            BrowserActionKind::Wait(_)
            | BrowserActionKind::Reload
            | BrowserActionKind::Back
            | BrowserActionKind::Forward => BrowserRiskLevel::Low,
            BrowserActionKind::CloseSession => BrowserRiskLevel::Low,
            BrowserActionKind::Screenshot => {
                if is_local {
                    BrowserRiskLevel::Low
                } else {
                    BrowserRiskLevel::Medium
                }
            }
            BrowserActionKind::OpenUrl(_) => {
                if is_local {
                    BrowserRiskLevel::Low
                } else {
                    BrowserRiskLevel::Medium
                }
            }
            BrowserActionKind::Click(_) => BrowserRiskLevel::Medium,
            BrowserActionKind::TypeText(_, _) => BrowserRiskLevel::High,
            BrowserActionKind::SubmitForm(_) => BrowserRiskLevel::High,
            BrowserActionKind::DownloadFile(_) => BrowserRiskLevel::High,
            BrowserActionKind::EvaluateReadonlyJs(_) => BrowserRiskLevel::High,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserAction {
    pub kind: BrowserActionKind,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserActionPlan {
    pub actions: Vec<BrowserAction>,
    pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserObservation {
    pub text_content: Option<String>,
    pub dom_summary: Option<String>,
    pub console_errors: Vec<String>,
    pub current_url: String,
    pub page_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserScreenshot {
    pub path: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserActionResult {
    pub success: bool,
    pub observation: Option<BrowserObservation>,
    pub screenshot: Option<BrowserScreenshot>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserTaskStatus {
    Pending,
    Running,
    WaitingForApproval,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTask {
    pub id: String,
    pub session_id: String,
    pub goal: String,
    pub status: BrowserTaskStatus,
    pub url: String,
    pub plan: Option<BrowserActionPlan>,
    pub actions_taken: Vec<BrowserAction>,
    pub results: Vec<BrowserActionResult>,
    pub summary: Option<String>,
}

#[async_trait::async_trait]
pub trait BrowserProvider: Send + Sync {
    fn kind(&self) -> BrowserProviderKind;
    async fn start_session(&self) -> Result<BrowserSession>;
    async fn close_session(&self, session_id: &str) -> Result<()>;
    async fn execute_action(
        &self,
        session_id: &str,
        action: &BrowserAction,
    ) -> Result<BrowserActionResult>;
    async fn check_health(&self) -> Result<String>;
}

pub struct BrowserSafetyPolicy {
    config: BrowserConfig,
}

impl BrowserSafetyPolicy {
    pub fn new(config: BrowserConfig) -> Self {
        Self { config }
    }

    pub fn is_action_allowed(&self, action: &BrowserActionKind, url: &str) -> bool {
        let risk = action.risk_level(url);
        match risk {
            BrowserRiskLevel::Low => true,
            BrowserRiskLevel::Medium => true,
            BrowserRiskLevel::High => true, // ApprovalGate handles actual blocking
            BrowserRiskLevel::Critical => false,
        }
    }
}

pub struct BrowserAdapterManager {
    config: BrowserConfig,
    provider: Option<Arc<dyn BrowserProvider>>,
    policy: BrowserSafetyPolicy,
    active_sessions: HashMap<String, BrowserSession>,
    tasks: HashMap<String, BrowserTask>,
}

use std::collections::HashMap;

impl BrowserAdapterManager {
    pub fn new(config: BrowserConfig) -> Self {
        let policy = BrowserSafetyPolicy::new(config.clone());

        let provider: Option<Arc<dyn BrowserProvider>> = if !config.enabled {
            None
        } else {
            match config.provider.as_str() {
                "manual_stub" => Some(Arc::new(ManualStubProvider)),
                // Other providers would be instantiated here
                _ => Some(Arc::new(ManualStubProvider)),
            }
        };

        Self {
            config,
            provider,
            policy,
            active_sessions: HashMap::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.provider.is_some()
    }

    pub async fn check_doctor(&self) -> Result<String> {
        if !self.config.enabled {
            return Ok("Browser adapter is disabled in config.".into());
        }

        if let Some(prov) = &self.provider {
            prov.check_health().await
        } else {
            Ok("No provider configured.".into())
        }
    }

    pub async fn open_url(&mut self, url: &str) -> Result<BrowserActionResult> {
        let action = BrowserAction {
            kind: BrowserActionKind::OpenUrl(url.to_string()),
            description: format!("Open URL: {}", url),
        };
        self.execute_action_internal(&action, url).await
    }

    pub async fn screenshot(&mut self, url: &str) -> Result<BrowserActionResult> {
        let action = BrowserAction {
            kind: BrowserActionKind::Screenshot,
            description: "Capture screenshot".to_string(),
        };
        self.execute_action_internal(&action, url).await
    }

    pub async fn read_text(&mut self, url: &str) -> Result<BrowserActionResult> {
        let action = BrowserAction {
            kind: BrowserActionKind::ReadText,
            description: "Read page text".to_string(),
        };
        self.execute_action_internal(&action, url).await
    }

    async fn execute_action_internal(
        &mut self,
        action: &BrowserAction,
        url: &str,
    ) -> Result<BrowserActionResult> {
        if !self.is_enabled() {
            return Err(anyhow!("Browser is disabled"));
        }

        if !self.policy.is_action_allowed(&action.kind, url) {
            return Err(anyhow!("Action blocked by safety policy"));
        }

        if let Some(prov) = &self.provider {
            prov.execute_action("default_session", action).await
        } else {
            Err(anyhow!("No provider active"))
        }
    }
}

// ── Manual Stub Provider ──────────────────────────────────────────────────────

struct ManualStubProvider;

#[async_trait::async_trait]
impl BrowserProvider for ManualStubProvider {
    fn kind(&self) -> BrowserProviderKind {
        BrowserProviderKind::ManualStub
    }

    async fn start_session(&self) -> Result<BrowserSession> {
        Ok(BrowserSession {
            id: "stub_session_1".into(),
            provider: BrowserProviderKind::ManualStub,
            status: BrowserSessionStatus::Active,
            started_at: 0,
        })
    }

    async fn close_session(&self, _session_id: &str) -> Result<()> {
        Ok(())
    }

    async fn execute_action(
        &self,
        _session_id: &str,
        action: &BrowserAction,
    ) -> Result<BrowserActionResult> {
        Ok(BrowserActionResult {
            success: true,
            observation: Some(BrowserObservation {
                text_content: Some(format!("Simulated output for {:?}", action.kind)),
                dom_summary: Some("Simulated DOM".into()),
                console_errors: vec![],
                current_url: "http://localhost:3000".into(),
                page_title: "Stubbed Page".into(),
            }),
            screenshot: None,
            error_message: None,
        })
    }

    async fn check_health(&self) -> Result<String> {
        Ok("ManualStubProvider is healthy and active. (No real browser attached)".into())
    }
}
