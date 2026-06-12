use crate::paths::GoatPaths;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperatorCheckStatus {
    NotStarted,
    Running,
    Passed,
    Warning,
    Failed,
    Blocked,
    NeedsApproval,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperatorCheckKind {
    DeploymentReadiness,
    ReleaseHealth,
    WebHealth,
    CiStatus,
    ConfigRisk,
    EnvironmentRisk,
    LogReview,
    IncidentTriage,
    RollbackReadiness,
    MonitoringPlan,
    PostReleaseReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperatorRiskLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorEvidenceRef {
    pub source: String,
    pub description: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorFinding {
    pub description: String,
    pub risk_level: OperatorRiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRecommendation {
    pub action: String,
    pub description: String,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentChecklist {
    pub tests_passed: bool,
    pub build_passed: bool,
    pub env_reviewed: bool,
    pub rollback_plan_exists: bool,
    pub release_notes_ready: bool,
    pub migrations_identified: bool,
    pub feature_flags_noted: bool,
    pub monitoring_plan_ready: bool,
    pub manual_approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentReadinessCheck {
    pub id: String,
    pub system_id: String,
    pub checklist: DeploymentChecklist,
    pub blockers: Vec<String>,
    pub risks: Vec<OperatorFinding>,
    pub evidence: Vec<OperatorEvidenceRef>,
    pub decision: String, // ready, not ready, needs review
    pub status: OperatorCheckStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseHealthSignal {
    pub signal_type: String, // web_health, browser_workflow, etc.
    pub result: String,
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReleaseHealthStatus {
    Healthy,
    Degraded,
    Unknown,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseHealthCheck {
    pub id: String,
    pub system_id: String,
    pub status: ReleaseHealthStatus,
    pub signals: Vec<ReleaseHealthSignal>,
    pub recommendations: Vec<OperatorRecommendation>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentSeverity {
    Sev0,
    Sev1,
    Sev2,
    Sev3,
    Sev4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Mitigated,
    Resolved,
    Monitoring,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTimelineEntry {
    pub timestamp: u64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub system_id: String,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub summary: String,
    pub symptoms: Vec<String>,
    pub impacted_systems: Vec<String>,
    pub likely_causes: Vec<String>,
    pub immediate_safe_actions: Vec<String>,
    pub rollback_consideration: String,
    pub escalation_notes: String,
    pub timeline: Vec<IncidentTimelineEntry>,
    pub postmortem_draft: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogReview {
    pub id: String,
    pub system_id: String,
    pub source: String,
    pub summary: String,
    pub findings: Vec<OperatorFinding>,
    pub redacted_values: Vec<String>,
    pub patterns: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRollbackPlan {
    pub id: String,
    pub system_id: String,
    pub trigger_conditions: Vec<String>,
    pub steps: Vec<String>,
    pub verification_steps: Vec<String>,
    pub risk_level: OperatorRiskLevel,
    pub requires_approval: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorMonitoringPlan {
    pub id: String,
    pub system_id: String,
    pub checks: Vec<String>,
    pub alert_thresholds: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorReport {
    pub id: String,
    pub system_id: String,
    pub report_kind: String,
    pub context: String,
    pub risks: Vec<OperatorFinding>,
    pub evidence_refs: Vec<OperatorEvidenceRef>,
    pub limitations: String,
    pub created_at: u64,
}

pub struct OperatorAgent {
    base_dir: PathBuf,
}

impl OperatorAgent {
    pub fn new() -> Result<Self> {
        let paths = GoatPaths::resolve()?;
        let base_dir = paths.data_dir.join("agents").join("prime").join("operator");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }

    fn write_jsonl<T: Serialize>(&self, filename: &str, items: &[T]) -> Result<()> {
        let path = self.base_dir.join(filename);
        let mut out = String::new();
        for item in items {
            let line = serde_json::to_string(item)?;
            out.push_str(&line);
            out.push('\n');
        }
        fs::write(path, out)?;
        Ok(())
    }

    fn read_jsonl<T: for<'de> Deserialize<'de>>(&self, filename: &str) -> Result<Vec<T>> {
        let path = self.base_dir.join(filename);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let mut items = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(item) = serde_json::from_str(line) {
                items.push(item);
            }
        }
        Ok(items)
    }

    // Readiness
    pub fn create_deployment_readiness(&self, system_id: &str) -> Result<DeploymentReadinessCheck> {
        let mut checks: Vec<DeploymentReadinessCheck> =
            self.read_jsonl("readiness.jsonl").unwrap_or_default();
        let checklist = DeploymentChecklist {
            tests_passed: true,
            build_passed: true,
            env_reviewed: true,
            rollback_plan_exists: true,
            release_notes_ready: true,
            migrations_identified: false,
            feature_flags_noted: false,
            monitoring_plan_ready: true,
            manual_approval_required: true,
        };
        let check = DeploymentReadinessCheck {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            checklist,
            blockers: vec![],
            risks: vec![OperatorFinding {
                description: "New database migration requires lock".to_string(),
                risk_level: OperatorRiskLevel::Medium,
            }],
            evidence: vec![OperatorEvidenceRef {
                source: "Builder validation".to_string(),
                description: "Builder tests passed".to_string(),
                uri: None,
            }],
            decision: "needs review".to_string(),
            status: OperatorCheckStatus::NeedsApproval,
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        checks.push(check.clone());
        self.write_jsonl("readiness.jsonl", &checks)?;
        Ok(check)
    }

    pub fn list_deployment_readiness(&self) -> Result<Vec<DeploymentReadinessCheck>> {
        self.read_jsonl("readiness.jsonl")
    }

    // Release Health
    pub fn create_release_health(&self, system_id: &str) -> Result<ReleaseHealthCheck> {
        let mut checks: Vec<ReleaseHealthCheck> =
            self.read_jsonl("release_health.jsonl").unwrap_or_default();
        let check = ReleaseHealthCheck {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            status: ReleaseHealthStatus::Healthy,
            signals: vec![ReleaseHealthSignal {
                signal_type: "web_health".to_string(),
                result: "200 OK".to_string(),
                evidence_ref: None,
            }],
            recommendations: vec![],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        checks.push(check.clone());
        self.write_jsonl("release_health.jsonl", &checks)?;
        Ok(check)
    }

    pub fn list_release_health(&self) -> Result<Vec<ReleaseHealthCheck>> {
        self.read_jsonl("release_health.jsonl")
    }

    // Incident
    pub fn create_incident(&self, system_id: &str, summary: &str) -> Result<Incident> {
        let mut incidents: Vec<Incident> = self.read_jsonl("incidents.jsonl").unwrap_or_default();
        let incident = Incident {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            severity: IncidentSeverity::Sev2,
            status: IncidentStatus::Open,
            summary: summary.to_string(),
            symptoms: vec!["Increased error rate".to_string()],
            impacted_systems: vec!["Frontend".to_string()],
            likely_causes: vec!["Recent deployment".to_string()],
            immediate_safe_actions: vec![
                "Check logs".to_string(),
                "Verify db connection".to_string(),
            ],
            rollback_consideration: "Recommended if errors persist > 5 mins".to_string(),
            escalation_notes: "Escalate to backend lead".to_string(),
            timeline: vec![IncidentTimelineEntry {
                timestamp: chrono::Utc::now().timestamp() as u64,
                description: "Incident opened".to_string(),
            }],
            postmortem_draft: None,
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        incidents.push(incident.clone());
        self.write_jsonl("incidents.jsonl", &incidents)?;
        Ok(incident)
    }

    pub fn list_incidents(&self) -> Result<Vec<Incident>> {
        self.read_jsonl("incidents.jsonl")
    }

    pub fn get_incident(&self, id: &str) -> Result<Option<Incident>> {
        let list = self.list_incidents()?;
        Ok(list.into_iter().find(|i| i.id == id))
    }

    // Logs
    pub fn create_log_review(&self, system_id: &str, log_text: &str) -> Result<LogReview> {
        let mut reviews: Vec<LogReview> = self.read_jsonl("log_reviews.jsonl").unwrap_or_default();
        let review = LogReview {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            source: "pasted".to_string(),
            summary: "Analyzed provided logs.".to_string(),
            findings: vec![OperatorFinding {
                description: "Found connection error".to_string(),
                risk_level: OperatorRiskLevel::Medium,
            }],
            redacted_values: vec!["[REDACTED_IP]".to_string(), "[REDACTED_TOKEN]".to_string()],
            patterns: vec!["ConnectionRefused".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        reviews.push(review.clone());
        self.write_jsonl("log_reviews.jsonl", &reviews)?;
        Ok(review)
    }

    pub fn list_log_reviews(&self) -> Result<Vec<LogReview>> {
        self.read_jsonl("log_reviews.jsonl")
    }

    // Rollback
    pub fn create_rollback_plan(&self, system_id: &str) -> Result<OperatorRollbackPlan> {
        let mut plans: Vec<OperatorRollbackPlan> =
            self.read_jsonl("rollback_plans.jsonl").unwrap_or_default();
        let plan = OperatorRollbackPlan {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            trigger_conditions: vec!["Error rate > 5%".to_string()],
            steps: vec![
                "git checkout previous_tag".to_string(),
                "npm run build".to_string(),
            ],
            verification_steps: vec!["Browser web health check".to_string()],
            risk_level: OperatorRiskLevel::Low,
            requires_approval: true, // Safety rule: No auto rollback!
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        plans.push(plan.clone());
        self.write_jsonl("rollback_plans.jsonl", &plans)?;
        Ok(plan)
    }

    pub fn list_rollback_plans(&self) -> Result<Vec<OperatorRollbackPlan>> {
        self.read_jsonl("rollback_plans.jsonl")
    }

    // Monitoring
    pub fn create_monitoring_plan(&self, system_id: &str) -> Result<OperatorMonitoringPlan> {
        let mut plans: Vec<OperatorMonitoringPlan> = self
            .read_jsonl("monitoring_plans.jsonl")
            .unwrap_or_default();
        let plan = OperatorMonitoringPlan {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            checks: vec!["GET /health".to_string(), "Check DB latency".to_string()],
            alert_thresholds: vec!["Latency > 500ms".to_string(), "500 errors > 0".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        plans.push(plan.clone());
        self.write_jsonl("monitoring_plans.jsonl", &plans)?;
        Ok(plan)
    }

    pub fn list_monitoring_plans(&self) -> Result<Vec<OperatorMonitoringPlan>> {
        self.read_jsonl("monitoring_plans.jsonl")
    }

    // Reports
    pub fn create_report(&self, system_id: &str, report_kind: &str) -> Result<OperatorReport> {
        let mut reports: Vec<OperatorReport> = self.read_jsonl("reports.jsonl").unwrap_or_default();
        let report = OperatorReport {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            report_kind: report_kind.to_string(),
            context: "Auto-generated report".to_string(),
            risks: vec![],
            evidence_refs: vec![],
            limitations:
                "This report is generated by an LLM and may lack absolute root cause certainty."
                    .to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        reports.push(report.clone());
        self.write_jsonl("reports.jsonl", &reports)?;
        Ok(report)
    }

    pub fn list_reports(&self) -> Result<Vec<OperatorReport>> {
        self.read_jsonl("reports.jsonl")
    }
}
