//! Phase 9.3 — Safe Runtime Wiring for Approved Extension Capabilities
//!
//! This module bridges the `CapabilityRegistry` (metadata discovery) and
//! GOAT's runtime/tool environment with explicit safety contracts:
//!
//! - Extension capabilities are **metadata first**. No code is executed by
//!   reading or discovering a capability.
//! - Every invocation of a risky capability MUST pass through `ApprovalGate`.
//! - MCP servers discovered from extensions are **never auto-started**.
//! - Disabled or unknown capabilities are rejected immediately.
//! - All prepare/invoke attempts are logged to the capability runtime log.
//! - Secrets are redacted from logs.
//!
//! # Capability lifecycle
//!
//! ```text
//!   Discoverable → Available → RequiresApproval
//!       ↓                              ↓
//!   (disabled)                ApprovedForThisRun
//!                                      ↓
//!                              Executed | Failed | Blocked
//! ```

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

use crate::approval::{ApprovalDecision, ApprovalGate, ApprovalRequest, RiskLevel};
use crate::capability_registry::{CapabilitySource, CapabilityType, ToolCapability};
use crate::paths::GoatPaths;

// ── Capability runtime status ─────────────────────────────────────────────────

/// The lifecycle state of a capability as it moves through the runtime system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    /// Known but not yet validated. Extension may be disabled.
    Discoverable,
    /// All pre-conditions met; can be prepared/invoked.
    Available,
    /// A user-facing approval prompt is required before invocation.
    RequiresApproval,
    /// User approved this invocation (single run, not persistent).
    ApprovedForThisRun,
    /// Successfully executed.
    Executed,
    /// Execution failed.
    Failed(String),
    /// Blocked — disabled extension, bad path, ApprovalGate denied, etc.
    Blocked(String),
}

impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityStatus::Discoverable => write!(f, "discoverable"),
            CapabilityStatus::Available => write!(f, "available"),
            CapabilityStatus::RequiresApproval => write!(f, "requires_approval"),
            CapabilityStatus::ApprovedForThisRun => write!(f, "approved_for_this_run"),
            CapabilityStatus::Executed => write!(f, "executed"),
            CapabilityStatus::Failed(msg) => write!(f, "failed: {}", msg),
            CapabilityStatus::Blocked(reason) => write!(f, "blocked: {}", reason),
        }
    }
}

// ── Runtime capability view ───────────────────────────────────────────────────

/// A runtime-visible projection of a `ToolCapability` enriched with its
/// current `CapabilityStatus`.
#[derive(Debug, Clone)]
pub struct RuntimeCapability {
    pub capability: ToolCapability,
    pub status: CapabilityStatus,
    pub risk_level: RiskLevel,
}

impl RuntimeCapability {
    pub fn parse_risk(s: &str) -> RiskLevel {
        match s.to_lowercase().as_str() {
            "critical" => RiskLevel::Critical,
            "high" => RiskLevel::High,
            "medium" => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }

    pub fn from_capability(cap: ToolCapability) -> Self {
        let risk_level = Self::parse_risk(&cap.risk_level);
        let status = if cap.enabled {
            CapabilityStatus::Available
        } else {
            CapabilityStatus::Blocked("Capability is disabled.".to_string())
        };
        RuntimeCapability {
            capability: cap,
            status,
            risk_level,
        }
    }

    pub fn needs_approval(&self) -> bool {
        self.risk_level >= RiskLevel::Medium
    }
}

// ── Prepare result ────────────────────────────────────────────────────────────

/// The result of a `prepare` check — never executes anything.
#[derive(Debug, Clone)]
pub struct PrepareResult {
    pub capability_id: String,
    pub capability_name: String,
    pub source_extension: Option<String>,
    pub capability_type: String,
    pub status: CapabilityStatus,
    pub risk_level: RiskLevel,
    pub approval_required: bool,
    pub approval_reason: Option<String>,
    pub checks: Vec<PrepareCheck>,
    pub safe_to_invoke: bool,
}

/// A single pre-flight check item.
#[derive(Debug, Clone)]
pub struct PrepareCheck {
    pub label: String,
    pub passed: bool,
    pub message: String,
}

// ── Invocation record ─────────────────────────────────────────────────────────

struct InvocationRecord {
    timestamp: u64,
    capability_id: String,
    capability_name: String,
    source: String,
    extension_id: Option<String>,
    action_requested: String,
    risk_level: String,
    approval_required: bool,
    approval_result: Option<String>,
    execution_result: Option<String>,
    error: Option<String>,
}

impl InvocationRecord {
    fn to_log_line(&self) -> String {
        format!(
            "[{}] id={} name={} source={} ext={} action={} risk={} approval_req={} approval_result={} exec_result={} error={}\n",
            self.timestamp,
            self.capability_id,
            self.capability_name,
            self.source,
            self.extension_id.as_deref().unwrap_or("none"),
            crate::approval::redact_secrets(&self.action_requested),
            self.risk_level,
            self.approval_required,
            self.approval_result.as_deref().unwrap_or("n/a"),
            self.execution_result.as_deref().unwrap_or("n/a"),
            self.error.as_deref().unwrap_or("none"),
        )
    }
}

// ── Capability Runtime Adapter ────────────────────────────────────────────────

/// The main runtime adapter. Reads from `CapabilityRegistry` and provides
/// safe, auditable preparation and invocation of extension capabilities.
pub struct CapabilityRuntimeAdapter {
    pub paths: GoatPaths,
    pub runtime_log: PathBuf,
}

impl CapabilityRuntimeAdapter {
    pub fn new(paths: GoatPaths) -> Self {
        let runtime_log = paths.data_dir.join("capability-runtime.log");
        CapabilityRuntimeAdapter { paths, runtime_log }
    }

    pub fn resolve(&self, id: &str) -> anyhow::Result<Option<RuntimeCapability>> {
        let reg = crate::capability_registry::CapabilityRegistry::new(&self.paths.data_dir)?;
        Ok(reg
            .get(id)
            .map(|c| RuntimeCapability::from_capability(c.clone())))
    }

    pub fn list_all(&self) -> anyhow::Result<Vec<RuntimeCapability>> {
        let reg = crate::capability_registry::CapabilityRegistry::new(&self.paths.data_dir)?;
        Ok(reg
            .list()
            .into_iter()
            .map(|c| RuntimeCapability::from_capability(c.clone()))
            .collect())
    }

    /// Run pre-flight checks for a capability without executing anything.
    pub fn prepare(&self, id: &str) -> anyhow::Result<PrepareResult> {
        let reg = crate::capability_registry::CapabilityRegistry::new(&self.paths.data_dir)?;

        let cap = match reg.get(id) {
            Some(c) => c.clone(),
            None => {
                self.log_invocation(InvocationRecord {
                    timestamp: now_secs(),
                    capability_id: id.to_string(),
                    capability_name: "unknown".to_string(),
                    source: "unknown".to_string(),
                    extension_id: None,
                    action_requested: "prepare".to_string(),
                    risk_level: "unknown".to_string(),
                    approval_required: false,
                    approval_result: None,
                    execution_result: None,
                    error: Some(format!("Capability '{}' not found", id)),
                });
                return Ok(PrepareResult {
                    capability_id: id.to_string(),
                    capability_name: "unknown".to_string(),
                    source_extension: None,
                    capability_type: "unknown".to_string(),
                    status: CapabilityStatus::Blocked(format!(
                        "Capability '{}' not found in registry.",
                        id
                    )),
                    risk_level: RiskLevel::High,
                    approval_required: true,
                    approval_reason: None,
                    checks: vec![PrepareCheck {
                        label: "Registry lookup".to_string(),
                        passed: false,
                        message: format!("No capability with id '{}' exists.", id),
                    }],
                    safe_to_invoke: false,
                });
            }
        };

        let runtime_cap = RuntimeCapability::from_capability(cap.clone());
        let mut checks: Vec<PrepareCheck> = Vec::new();
        let mut safe = true;

        // Check 1: enabled
        checks.push(PrepareCheck {
            label: "Capability enabled".to_string(),
            passed: cap.enabled,
            message: if cap.enabled {
                "Capability is enabled.".to_string()
            } else {
                "Capability is DISABLED. Enable the source extension first.".to_string()
            },
        });
        if !cap.enabled {
            safe = false;
        }

        // Check 2: source extension
        let source_ext_id: Option<String> = match &cap.source {
            CapabilitySource::Extension(ext_id) => Some(ext_id.clone()),
            CapabilitySource::Core => None,
        };

        if let Some(ref ext_id) = source_ext_id {
            let ext_mgr = crate::extensions::ExtensionManager::new(&self.paths.data_dir)?;
            let ext_enabled = ext_mgr.list().iter().any(|e| {
                &e.manifest.extension.id == ext_id
                    && e.status == crate::extensions::ExtensionStatus::Enabled
            });

            checks.push(PrepareCheck {
                label: "Source extension enabled".to_string(),
                passed: ext_enabled,
                message: if ext_enabled {
                    format!("Extension '{}' is enabled.", ext_id)
                } else {
                    format!(
                        "Source extension '{}' is NOT enabled. Run: goat extensions enable {}",
                        ext_id, ext_id
                    )
                },
            });
            if !ext_enabled {
                safe = false;
            }
        }

        // Check 3: type-specific checks
        match &cap.capability_type {
            CapabilityType::Command => {
                if let Some(cmd) = cap.metadata.get("command").and_then(|v| v.as_str()) {
                    let first_word = cmd.split_whitespace().next().unwrap_or(cmd);
                    let injection_risk = cmd.contains(';')
                        || cmd.contains('|')
                        || cmd.contains('&')
                        || cmd.contains('`')
                        || cmd.contains("$(");
                    checks.push(PrepareCheck {
                        label: "Command safety check".to_string(),
                        passed: !injection_risk,
                        message: if injection_risk {
                            format!("Command '{}' contains shell meta-characters. Manual review required.", crate::approval::redact_secrets(cmd))
                        } else {
                            format!("Command executable: '{}'", first_word)
                        },
                    });
                    if injection_risk {
                        safe = false;
                    }
                } else {
                    checks.push(PrepareCheck {
                        label: "Command metadata".to_string(),
                        passed: false,
                        message: "No 'command' field in capability metadata.".to_string(),
                    });
                    safe = false;
                }
            }
            CapabilityType::McpServer => {
                checks.push(PrepareCheck {
                    label: "MCP server auto-start".to_string(),
                    passed: true,
                    message: "MCP servers are NEVER auto-started. Metadata only.".to_string(),
                });
                if let Some(cmd) = cap.metadata.get("command").and_then(|v| v.as_str()) {
                    checks.push(PrepareCheck {
                        label: "MCP server command".to_string(),
                        passed: true,
                        message: format!(
                            "Server command: '{}' (not executed)",
                            crate::approval::redact_secrets(cmd)
                        ),
                    });
                }
            }
            CapabilityType::Skill => {
                if let Some(path_val) = cap.metadata.get("path").and_then(|v| v.as_str()) {
                    let path = PathBuf::from(path_val);
                    let is_safe_path = !path.is_absolute()
                        || path.starts_with(&self.paths.skills_dir)
                        || path.starts_with(&self.paths.data_dir);
                    checks.push(PrepareCheck {
                        label: "Skill path safety".to_string(),
                        passed: is_safe_path,
                        message: if is_safe_path {
                            format!("Skill path: '{}' is within a safe scope.", path_val)
                        } else {
                            format!(
                                "Skill path '{}' is outside safe directories. Blocked.",
                                path_val
                            )
                        },
                    });
                    if !is_safe_path {
                        safe = false;
                    }
                }
                checks.push(PrepareCheck {
                    label: "Skill execution model".to_string(),
                    passed: true,
                    message:
                        "Skills are guided workflows. Use: goat skills run --from-extension <id>"
                            .to_string(),
                });
            }
            CapabilityType::ValidationRecipe => {
                if let Some(path_val) = cap.metadata.get("path").and_then(|v| v.as_str()) {
                    checks.push(PrepareCheck {
                        label: "Validation recipe path".to_string(),
                        passed: true,
                        message: format!(
                            "Recipe path: '{}'. Use: goat validate --recipe <id>",
                            path_val
                        ),
                    });
                }
                checks.push(PrepareCheck {
                    label: "Validation execution model".to_string(),
                    passed: true,
                    message: "Validation recipes require explicit run + ApprovalGate approval."
                        .to_string(),
                });
            }
            CapabilityType::NativeTool => {
                checks.push(PrepareCheck {
                    label: "Native tool".to_string(),
                    passed: true,
                    message:
                        "Native GOAT core tool. Managed by ToolRegistry, not extension runtime."
                            .to_string(),
                });
            }
        }

        // Check 4: risk level
        let approval_required = runtime_cap.needs_approval();
        let approval_reason = if approval_required {
            Some(format!(
                "Risk level '{}' requires ApprovalGate confirmation before execution.",
                cap.risk_level
            ))
        } else {
            None
        };
        checks.push(PrepareCheck {
            label: "Risk level".to_string(),
            passed: true,
            message: format!(
                "Risk: {} — {}",
                cap.risk_level,
                if approval_required {
                    "APPROVAL REQUIRED before invocation"
                } else {
                    "Low risk — approval recommended"
                }
            ),
        });

        let final_status = if !safe {
            CapabilityStatus::Blocked(
                checks
                    .iter()
                    .filter(|c| !c.passed)
                    .map(|c| c.message.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        } else if approval_required {
            CapabilityStatus::RequiresApproval
        } else {
            CapabilityStatus::Available
        };

        self.log_invocation(InvocationRecord {
            timestamp: now_secs(),
            capability_id: cap.id.clone(),
            capability_name: cap.name.clone(),
            source: format!("{:?}", cap.source),
            extension_id: source_ext_id.clone(),
            action_requested: "prepare".to_string(),
            risk_level: cap.risk_level.clone(),
            approval_required,
            approval_result: None,
            execution_result: Some(format!("{}", final_status)),
            error: None,
        });

        Ok(PrepareResult {
            capability_id: cap.id,
            capability_name: cap.name,
            source_extension: source_ext_id,
            capability_type: format!("{:?}", cap.capability_type),
            status: final_status,
            risk_level: runtime_cap.risk_level,
            approval_required,
            approval_reason,
            checks,
            safe_to_invoke: safe,
        })
    }

    /// Attempt to invoke a capability after ApprovalGate check.
    /// ONLY `CapabilityType::Command` capabilities can be executed from the CLI.
    pub fn invoke_sync(
        &self,
        id: &str,
        gate: &mut ApprovalGate,
        session_id: &str,
    ) -> anyhow::Result<CapabilityStatus> {
        let prep = self.prepare(id)?;

        if !prep.safe_to_invoke {
            let reason = format!("{}", prep.status);
            self.log_invocation(InvocationRecord {
                timestamp: now_secs(),
                capability_id: id.to_string(),
                capability_name: prep.capability_name.clone(),
                source: prep
                    .source_extension
                    .clone()
                    .unwrap_or_else(|| "core".to_string()),
                extension_id: prep.source_extension.clone(),
                action_requested: "invoke".to_string(),
                risk_level: format!("{}", prep.risk_level),
                approval_required: prep.approval_required,
                approval_result: Some("blocked".to_string()),
                execution_result: Some("blocked".to_string()),
                error: Some(reason.clone()),
            });
            return Ok(CapabilityStatus::Blocked(reason));
        }

        let reg = crate::capability_registry::CapabilityRegistry::new(&self.paths.data_dir)?;
        let cap = match reg.get(id) {
            Some(c) => c.clone(),
            None => {
                return Ok(CapabilityStatus::Blocked(format!(
                    "Capability '{}' disappeared from registry.",
                    id
                )));
            }
        };

        let command = match &cap.capability_type {
            CapabilityType::Command => match cap.metadata.get("command").and_then(|v| v.as_str()) {
                Some(cmd) => cmd.to_string(),
                None => {
                    return Ok(CapabilityStatus::Blocked(
                        "No 'command' field in metadata.".to_string(),
                    ));
                }
            },
            CapabilityType::McpServer => {
                let msg = "MCP servers are not auto-started. Use your MCP client to connect.";
                self.log_invocation(InvocationRecord {
                    timestamp: now_secs(),
                    capability_id: id.to_string(),
                    capability_name: cap.name.clone(),
                    source: format!("{:?}", cap.source),
                    extension_id: prep.source_extension.clone(),
                    action_requested: "invoke".to_string(),
                    risk_level: cap.risk_level.clone(),
                    approval_required: true,
                    approval_result: Some("blocked_mcp_no_autostart".to_string()),
                    execution_result: Some("blocked".to_string()),
                    error: Some(msg.to_string()),
                });
                return Ok(CapabilityStatus::Blocked(msg.to_string()));
            }
            CapabilityType::Skill => {
                let msg = format!(
                    "Skills require guided execution. Run: goat skills run --from-extension {}",
                    id
                );
                self.log_invocation(InvocationRecord {
                    timestamp: now_secs(),
                    capability_id: id.to_string(),
                    capability_name: cap.name.clone(),
                    source: format!("{:?}", cap.source),
                    extension_id: prep.source_extension.clone(),
                    action_requested: "invoke".to_string(),
                    risk_level: cap.risk_level.clone(),
                    approval_required: true,
                    approval_result: Some("blocked_skill_guided_only".to_string()),
                    execution_result: Some("blocked".to_string()),
                    error: Some(msg.clone()),
                });
                return Ok(CapabilityStatus::Blocked(msg));
            }
            CapabilityType::ValidationRecipe => {
                let msg = format!(
                    "Validation recipes require explicit run. Use: goat validate --recipe {}",
                    id
                );
                self.log_invocation(InvocationRecord {
                    timestamp: now_secs(),
                    capability_id: id.to_string(),
                    capability_name: cap.name.clone(),
                    source: format!("{:?}", cap.source),
                    extension_id: prep.source_extension.clone(),
                    action_requested: "invoke".to_string(),
                    risk_level: cap.risk_level.clone(),
                    approval_required: true,
                    approval_result: Some("blocked_validator_explicit_only".to_string()),
                    execution_result: Some("blocked".to_string()),
                    error: Some(msg.clone()),
                });
                return Ok(CapabilityStatus::Blocked(msg));
            }
            CapabilityType::NativeTool => {
                return Ok(CapabilityStatus::Blocked(
                    "Native tools are managed by the GOAT ToolRegistry, not the extension runtime."
                        .to_string(),
                ));
            }
        };

        // Build ApprovalRequest
        let risk = prep.risk_level;
        let req = ApprovalRequest {
            tool_name: format!("capability:{}", cap.id),
            action_summary: format!(
                "Run extension capability '{}': {}",
                cap.name,
                crate::approval::redact_secrets(&command)
            ),
            risk_level: risk,
            explanation: Some(format!(
                "Source: {:?}\nExtension: {}\nCapability Type: {:?}\nRequired Permissions: {:?}",
                cap.source,
                prep.source_extension.as_deref().unwrap_or("core"),
                cap.capability_type,
                cap.required_permissions
            )),
            working_directory: None,
        };

        // Check session policy first, then interactive prompt
        let decision = match gate.check_policy(&req) {
            Some(d) => d,
            None => {
                println!();
                for line in req.display_lines() {
                    println!("{}", line);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap_or(0);
                let ch = input.trim().chars().next().unwrap_or('n');
                gate.resolve(&req, ch)
            }
        };

        let approval_result_str = match &decision {
            ApprovalDecision::Approved => "approved",
            ApprovalDecision::Denied(_) => "denied",
        };

        if let ApprovalDecision::Denied(reason) = &decision {
            self.log_invocation(InvocationRecord {
                timestamp: now_secs(),
                capability_id: id.to_string(),
                capability_name: cap.name.clone(),
                source: format!("{:?}", cap.source),
                extension_id: prep.source_extension.clone(),
                action_requested: format!("invoke: {}", crate::approval::redact_secrets(&command)),
                risk_level: format!("{}", req.risk_level),
                approval_required: true,
                approval_result: Some(approval_result_str.to_string()),
                execution_result: Some("denied".to_string()),
                error: Some(reason.clone()),
            });
            return Ok(CapabilityStatus::Blocked(format!("Denied: {}", reason)));
        }

        // Execute — non-shell, split by whitespace (no bash expansion)
        let parts: Vec<&str> = command.split_whitespace().collect();
        let (cmd_exec, args) = parts
            .split_first()
            .ok_or_else(|| anyhow::anyhow!("Empty command string"))?;

        let output = std::process::Command::new(cmd_exec).args(args).output();

        let (status, exec_result, error) = match output {
            Ok(out) => {
                let success = out.status.success();
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let combined = format!(
                    "exit={} stdout={} stderr={}",
                    out.status.code().unwrap_or(-1),
                    &stdout[..stdout.len().min(200)],
                    &stderr[..stderr.len().min(200)]
                );
                if success {
                    println!("{}", stdout);
                    (CapabilityStatus::Executed, combined, None)
                } else {
                    eprintln!("stderr: {}", stderr);
                    (
                        CapabilityStatus::Failed(stderr.clone()),
                        combined,
                        Some(stderr),
                    )
                }
            }
            Err(e) => {
                let msg = format!("Failed to spawn '{}': {}", cmd_exec, e);
                (
                    CapabilityStatus::Failed(msg.clone()),
                    "spawn_error".to_string(),
                    Some(msg),
                )
            }
        };

        // Reuse ToolRegistry audit log infrastructure
        {
            let tool_reg = crate::tool_registry::ToolRegistry::new();
            let action = crate::tool_registry::ToolAction::Allow;
            let success = matches!(&status, CapabilityStatus::Executed);
            tool_reg.log_execution(
                &self.paths,
                session_id,
                &format!("capability:{}", cap.id),
                &action,
                success,
                &exec_result,
            );
        }

        self.log_invocation(InvocationRecord {
            timestamp: now_secs(),
            capability_id: id.to_string(),
            capability_name: cap.name.clone(),
            source: format!("{:?}", cap.source),
            extension_id: prep.source_extension.clone(),
            action_requested: format!("invoke: {}", crate::approval::redact_secrets(&command)),
            risk_level: format!("{}", req.risk_level),
            approval_required: true,
            approval_result: Some(approval_result_str.to_string()),
            execution_result: Some(exec_result),
            error,
        });

        Ok(status)
    }

    fn log_invocation(&self, record: InvocationRecord) {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.runtime_log)
        {
            let _ = file.write_all(record.to_log_line().as_bytes());
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approval::{ApprovalGate, SessionPolicy};
    use crate::capability_registry::{CapabilitySource, CapabilityType, ToolCapability};

    fn make_cap(id: &str, cap_type: CapabilityType, risk: &str, enabled: bool) -> ToolCapability {
        ToolCapability {
            id: id.to_string(),
            name: format!("Test {}", id),
            source: CapabilitySource::Extension("test-ext".to_string()),
            capability_type: cap_type,
            risk_level: risk.to_string(),
            enabled,
            description: "Test capability".to_string(),
            required_permissions: vec![],
            metadata: serde_json::json!({ "command": "echo hello" }),
            discovered_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_disabled_capability_blocked() {
        let cap = make_cap("test:tool:disabled", CapabilityType::Command, "low", false);
        let rt = RuntimeCapability::from_capability(cap);
        assert!(matches!(rt.status, CapabilityStatus::Blocked(_)));
    }

    #[test]
    fn test_enabled_low_risk_available() {
        let cap = make_cap("test:tool:enabled", CapabilityType::Command, "low", true);
        let rt = RuntimeCapability::from_capability(cap);
        assert_eq!(rt.status, CapabilityStatus::Available);
        assert!(!rt.needs_approval());
    }

    #[test]
    fn test_high_risk_needs_approval() {
        let cap = make_cap("test:tool:high", CapabilityType::Command, "high", true);
        let rt = RuntimeCapability::from_capability(cap);
        assert!(rt.needs_approval());
    }

    #[test]
    fn test_risk_parse() {
        assert_eq!(
            RuntimeCapability::parse_risk("critical"),
            RiskLevel::Critical
        );
        assert_eq!(
            RuntimeCapability::parse_risk("CRITICAL"),
            RiskLevel::Critical
        );
        assert_eq!(RuntimeCapability::parse_risk("high"), RiskLevel::High);
        assert_eq!(RuntimeCapability::parse_risk("medium"), RiskLevel::Medium);
        assert_eq!(RuntimeCapability::parse_risk("low"), RiskLevel::Low);
        assert_eq!(
            RuntimeCapability::parse_risk("unknown_junk"),
            RiskLevel::Low
        );
    }

    #[test]
    fn test_capability_status_display() {
        assert_eq!(format!("{}", CapabilityStatus::Available), "available");
        assert_eq!(
            format!("{}", CapabilityStatus::RequiresApproval),
            "requires_approval"
        );
        assert_eq!(format!("{}", CapabilityStatus::Executed), "executed");
        assert_eq!(
            format!("{}", CapabilityStatus::Blocked("bad".to_string())),
            "blocked: bad"
        );
        assert_eq!(
            format!("{}", CapabilityStatus::Failed("oops".to_string())),
            "failed: oops"
        );
    }

    #[test]
    fn test_approval_gate_deny_blocks() {
        let mut gate = ApprovalGate::new();
        gate.set_policy("capability:test:tool:hi", SessionPolicy::AlwaysDeny);
        let req = ApprovalRequest {
            tool_name: "capability:test:tool:hi".to_string(),
            action_summary: "Run test".to_string(),
            risk_level: RiskLevel::High,
            explanation: None,
            working_directory: None,
        };
        let decision = gate.check_policy(&req);
        assert!(matches!(decision, Some(ApprovalDecision::Denied(_))));
    }

    #[test]
    fn test_mcp_capability_type_identified() {
        let cap = make_cap("ext:mcp:srv", CapabilityType::McpServer, "high", true);
        let rt = RuntimeCapability::from_capability(cap);
        assert_eq!(rt.status, CapabilityStatus::Available);
        assert!(matches!(
            rt.capability.capability_type,
            CapabilityType::McpServer
        ));
    }

    #[test]
    fn test_skill_capability_type_identified() {
        let cap = make_cap("ext:skill:s1", CapabilityType::Skill, "low", true);
        let rt = RuntimeCapability::from_capability(cap);
        assert!(matches!(
            rt.capability.capability_type,
            CapabilityType::Skill
        ));
    }

    #[test]
    fn test_validation_recipe_type_identified() {
        let cap = make_cap(
            "ext:validator:v1",
            CapabilityType::ValidationRecipe,
            "medium",
            true,
        );
        let rt = RuntimeCapability::from_capability(cap);
        assert!(matches!(
            rt.capability.capability_type,
            CapabilityType::ValidationRecipe
        ));
    }

    #[test]
    fn test_redacted_log_line_no_secrets() {
        let record = InvocationRecord {
            timestamp: 0,
            capability_id: "ext:tool:t1".to_string(),
            capability_name: "Test Tool".to_string(),
            source: "extension".to_string(),
            extension_id: Some("my-ext".to_string()),
            action_requested: "invoke: echo api_key=secret123 hello".to_string(),
            risk_level: "medium".to_string(),
            approval_required: true,
            approval_result: Some("approved".to_string()),
            execution_result: Some("exit=0".to_string()),
            error: None,
        };
        let line = record.to_log_line();
        assert!(
            !line.contains("secret123"),
            "Secrets must be redacted from logs"
        );
        assert!(line.contains("[REDACTED]"), "Redaction marker must appear");
    }
}
