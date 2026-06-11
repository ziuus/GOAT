use crate::paths::GoatPaths;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperatorWorkflowState {
    New,
    Planning,
    Inspecting,
    Reviewing,
    Active,
    Mitigated,
    Resolved,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSystem {
    pub id: String,
    pub name: String,
    pub project_ref: Option<String>,
    pub repo_path: Option<String>,
    pub system_type: String,
    pub environment: String,
    pub deploy_target: Option<String>,
    pub health_urls: Option<Vec<String>>,
    pub log_paths: Option<Vec<String>>,
    pub ci_provider: Option<String>,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorHealthCheck {
    pub id: String,
    pub system_id: String,
    pub system_summary: String,
    pub expected_services: Vec<String>,
    pub health_endpoints: Vec<String>,
    pub local_checks: Vec<String>,
    pub build_test_checks: Vec<String>,
    pub dependency_checks: Vec<String>,
    pub config_checks: Vec<String>,
    pub resource_checks: Vec<String>,
    pub risk_findings: Vec<String>,
    pub next_safe_actions: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorLogSource {
    pub path_or_url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorLogFinding {
    pub id: String,
    pub system_id: String,
    pub error_patterns: Vec<String>,
    pub warning_patterns: Vec<String>,
    pub repeated_failures: Vec<String>,
    pub timestamps: Vec<String>,
    pub likely_root_causes: Vec<String>,
    pub severity: String,
    pub redacted_sensitive_values: Vec<String>,
    pub next_debugging_steps: Vec<String>,
    pub recommended_fixes: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperatorIncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorIncident {
    pub id: String,
    pub system_id: String,
    pub incident_summary: String,
    pub impact: String,
    pub timeline: Vec<String>,
    pub symptoms: Vec<String>,
    pub suspected_root_cause: String,
    pub evidence: Vec<String>,
    pub immediate_mitigation: String,
    pub permanent_fix: String,
    pub rollback_option: String,
    pub monitoring_improvement: String,
    pub postmortem_notes: String,
    pub severity: OperatorIncidentSeverity,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDeploymentPlan {
    pub id: String,
    pub system_id: String,
    pub pre_deploy_checklist: Vec<String>,
    pub build_commands: Vec<String>,
    pub test_commands: Vec<String>,
    pub migration_notes: String,
    pub environment_checks: Vec<String>,
    pub backup_notes: String,
    pub deploy_steps: Vec<String>,
    pub smoke_tests: Vec<String>,
    pub rollback_trigger: String,
    pub post_deploy_monitoring: Vec<String>,
    pub approval_requirements: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorCiCdReview {
    pub id: String,
    pub system_id: String,
    pub current_ci_summary: String,
    pub missing_checks: Vec<String>,
    pub flaky_risk: Vec<String>,
    pub security_concerns: Vec<String>,
    pub build_cache_suggestions: Vec<String>,
    pub test_coverage_gaps: Vec<String>,
    pub release_workflow_notes: String,
    pub recommended_actions_improvements: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRollbackPlan {
    pub id: String,
    pub system_id: String,
    pub rollback_trigger_conditions: Vec<String>,
    pub backup_restore_approach: String,
    pub version_tag_to_return_to: String,
    pub database_rollback_warnings: String,
    pub config_rollback: String,
    pub verification_steps: Vec<String>,
    pub communication_notes: String,
    pub risk_level: String,
    pub requires_approval: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorReliabilityCheck {
    pub id: String,
    pub system_id: String,
    pub findings: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRunbook {
    pub id: String,
    pub system_id: String,
    pub system_overview: String,
    pub common_commands: Vec<String>,
    pub health_checks: Vec<String>,
    pub log_locations: Vec<String>,
    pub common_failures: Vec<String>,
    pub restart_procedure: String,
    pub deployment_procedure: String,
    pub rollback_procedure: String,
    pub escalation_notes: String,
    pub safety_warnings: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorReport {
    pub id: String,
    pub system_id: String,
    pub report_kind: String, // operator_health_report, incident_report, etc.
    pub system_summary: String,
    pub findings: Vec<String>,
    pub risks: Vec<String>,
    pub evidence: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub approval_requirements: Vec<String>,
    pub rollback_notes: String,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
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

    pub fn list_systems(&self) -> Result<Vec<OperatorSystem>> {
        self.read_jsonl("systems.jsonl")
    }

    pub fn get_system(&self, id: &str) -> Result<Option<OperatorSystem>> {
        let systems = self.list_systems()?;
        Ok(systems.into_iter().find(|s| s.id == id))
    }

    pub fn create_system(
        &self,
        name: &str,
        system_type: &str,
        environment: &str,
    ) -> Result<OperatorSystem> {
        let mut systems = self.list_systems()?;
        let sys = OperatorSystem {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            project_ref: None,
            repo_path: None,
            system_type: system_type.to_string(),
            environment: environment.to_string(),
            deploy_target: None,
            health_urls: None,
            log_paths: None,
            ci_provider: None,
            status: "Monitoring".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            timeline_refs: vec![],
            brain_refs: vec![],
        };
        systems.push(sys.clone());
        self.write_jsonl("systems.jsonl", &systems)?;
        Ok(sys)
    }

    pub fn create_health_check(&self, system_id: &str) -> Result<OperatorHealthCheck> {
        let mut checks: Vec<OperatorHealthCheck> =
            self.read_jsonl("health_checks.jsonl").unwrap_or_default();
        let hc = OperatorHealthCheck {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            system_summary: "System appears operational".to_string(),
            expected_services: vec!["db".to_string(), "api".to_string(), "web".to_string()],
            health_endpoints: vec!["/health".to_string()],
            local_checks: vec!["Process running".to_string()],
            build_test_checks: vec!["cargo test passed".to_string()],
            dependency_checks: vec!["postgres active".to_string()],
            config_checks: vec!["env vars valid".to_string()],
            resource_checks: vec!["memory < 80%".to_string()],
            risk_findings: vec![],
            next_safe_actions: vec!["Continue monitoring".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        checks.push(hc.clone());
        self.write_jsonl("health_checks.jsonl", &checks)?;
        Ok(hc)
    }

    pub fn create_log_finding(&self, system_id: &str) -> Result<OperatorLogFinding> {
        let mut finds: Vec<OperatorLogFinding> =
            self.read_jsonl("log_findings.jsonl").unwrap_or_default();
        let finding = OperatorLogFinding {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            error_patterns: vec!["Connection refused".to_string()],
            warning_patterns: vec!["High latency".to_string()],
            repeated_failures: vec![],
            timestamps: vec![chrono::Utc::now().to_rfc3339()],
            likely_root_causes: vec!["DB restart".to_string()],
            severity: "Medium".to_string(),
            redacted_sensitive_values: vec!["[REDACTED_TOKEN]".to_string()],
            next_debugging_steps: vec!["Check db logs".to_string()],
            recommended_fixes: vec!["Increase connection pool".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        finds.push(finding.clone());
        self.write_jsonl("log_findings.jsonl", &finds)?;
        Ok(finding)
    }

    pub fn create_incident(&self, system_id: &str) -> Result<OperatorIncident> {
        let mut incs: Vec<OperatorIncident> =
            self.read_jsonl("incidents.jsonl").unwrap_or_default();
        let inc = OperatorIncident {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            incident_summary: "Service downtime".to_string(),
            impact: "Users cannot login".to_string(),
            timeline: vec![],
            symptoms: vec!["500 errors".to_string()],
            suspected_root_cause: "Unknown".to_string(),
            evidence: vec![],
            immediate_mitigation: "Restart service".to_string(),
            permanent_fix: "TBD".to_string(),
            rollback_option: "Rollback to prev version".to_string(),
            monitoring_improvement: "Add auth alarms".to_string(),
            postmortem_notes: "".to_string(),
            severity: OperatorIncidentSeverity::High,
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        incs.push(inc.clone());
        self.write_jsonl("incidents.jsonl", &incs)?;
        Ok(inc)
    }

    pub fn create_deployment_plan(&self, system_id: &str) -> Result<OperatorDeploymentPlan> {
        let mut plans: Vec<OperatorDeploymentPlan> = self
            .read_jsonl("deployment_plans.jsonl")
            .unwrap_or_default();
        let plan = OperatorDeploymentPlan {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            pre_deploy_checklist: vec!["Tests pass".to_string(), "Build ok".to_string()],
            build_commands: vec!["npm run build".to_string()],
            test_commands: vec!["npm test".to_string()],
            migration_notes: "None".to_string(),
            environment_checks: vec!["env vars set".to_string()],
            backup_notes: "DB backup taken".to_string(),
            deploy_steps: vec!["Restart service".to_string()],
            smoke_tests: vec!["curl /health".to_string()],
            rollback_trigger: "High error rate".to_string(),
            post_deploy_monitoring: vec!["Watch logs".to_string()],
            approval_requirements: vec!["Lead Dev Approval".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        plans.push(plan.clone());
        self.write_jsonl("deployment_plans.jsonl", &plans)?;
        Ok(plan)
    }

    pub fn create_ci_review(&self, system_id: &str) -> Result<OperatorCiCdReview> {
        let mut revs: Vec<OperatorCiCdReview> =
            self.read_jsonl("ci_reviews.jsonl").unwrap_or_default();
        let rev = OperatorCiCdReview {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            current_ci_summary: "GitHub Actions active".to_string(),
            missing_checks: vec!["E2E tests".to_string()],
            flaky_risk: vec!["low".to_string()],
            security_concerns: vec!["none".to_string()],
            build_cache_suggestions: vec!["cache node_modules".to_string()],
            test_coverage_gaps: vec!["auth service".to_string()],
            release_workflow_notes: "Looks OK".to_string(),
            recommended_actions_improvements: vec!["Add rust cache".to_string()],
            acceptance_criteria: vec!["Passes checks".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        revs.push(rev.clone());
        self.write_jsonl("ci_reviews.jsonl", &revs)?;
        Ok(rev)
    }

    pub fn create_rollback_plan(&self, system_id: &str) -> Result<OperatorRollbackPlan> {
        let mut plans: Vec<OperatorRollbackPlan> =
            self.read_jsonl("rollback_plans.jsonl").unwrap_or_default();
        let plan = OperatorRollbackPlan {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            rollback_trigger_conditions: vec!["500 errors > 1%".to_string()],
            backup_restore_approach: "Restore from automated snapshot".to_string(),
            version_tag_to_return_to: "previous-stable".to_string(),
            database_rollback_warnings: "Ensure schema backwards compatibility".to_string(),
            config_rollback: "Revert env vars".to_string(),
            verification_steps: vec!["Check /health".to_string()],
            communication_notes: "Notify team in #ops".to_string(),
            risk_level: "Low".to_string(),
            requires_approval: true,
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        plans.push(plan.clone());
        self.write_jsonl("rollback_plans.jsonl", &plans)?;
        Ok(plan)
    }

    pub fn create_runbook(&self, system_id: &str) -> Result<OperatorRunbook> {
        let mut books: Vec<OperatorRunbook> = self.read_jsonl("runbooks.jsonl").unwrap_or_default();
        let book = OperatorRunbook {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            system_overview: "Main web API".to_string(),
            common_commands: vec!["cargo run".to_string()],
            health_checks: vec!["curl /health".to_string()],
            log_locations: vec!["/var/log/api.log".to_string()],
            common_failures: vec!["OOM".to_string(), "DB disconnect".to_string()],
            restart_procedure: "systemctl restart api".to_string(),
            deployment_procedure: "npm run deploy".to_string(),
            rollback_procedure: "npm run rollback".to_string(),
            escalation_notes: "Call on-call engineer".to_string(),
            safety_warnings: vec![
                "Do NOT run migrations directly on prod db without approval".to_string(),
            ],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        books.push(book.clone());
        self.write_jsonl("runbooks.jsonl", &books)?;
        Ok(book)
    }

    pub fn create_report(&self, system_id: &str, report_kind: &str) -> Result<OperatorReport> {
        let r_dir = self.base_dir.join("reports");
        if !r_dir.exists() {
            fs::create_dir_all(&r_dir)?;
        }
        let report = OperatorReport {
            id: Uuid::new_v4().to_string(),
            system_id: system_id.to_string(),
            report_kind: report_kind.to_string(),
            system_summary: "System operates normally".to_string(),
            findings: vec!["All checks pass".to_string()],
            risks: vec!["None".to_string()],
            evidence: vec!["Logs clean".to_string()],
            recommended_actions: vec!["Maintain".to_string()],
            approval_requirements: vec![],
            rollback_notes: "N/A".to_string(),
            timeline_refs: vec![],
            brain_refs: vec![],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        let path = r_dir.join(format!("{}.json", report.id));
        fs::write(path, serde_json::to_string_pretty(&report)?)?;
        Ok(report)
    }
}
