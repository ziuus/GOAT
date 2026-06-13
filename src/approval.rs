//! Approval gate for dangerous tool operations.
//!
//! This module implements the security approval layer that must be consulted
//! before any dangerous operation (shell commands, file writes, subagent spawns)
//! is executed. It is intentionally decoupled from the TUI so that future
//! rendering backends can replace the prompt without touching this logic.
//!
//! # Design
//!
//! - [`ApprovalRequest`] describes a proposed dangerous action.
//! - [`RiskLevel`] classifies how dangerous an operation is.
//! - [`ApprovalDecision`] is the outcome: `Approved` or `Denied`.
//! - [`SessionPolicy`] stores per-tool session-level overrides (always allow / always deny).
//! - [`ApprovalGate`] holds the session policy and applies it to new requests.
//!
//! # Deny-by-default
//!
//! If the gate cannot determine approval (missing context, I/O error, unknown
//! input), it returns [`ApprovalDecision::Denied`]. Execution is never
//! permitted silently.

use std::collections::HashMap;
use tracing::{info, warn};

// ── Risk classification ──────────────────────────────────────────────────────

/// How dangerous a proposed operation is.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum RiskLevel {
    /// Low risk — informational or read-only.
    Low,
    /// Medium risk — modifies state but is reversible.
    Medium,
    /// High risk — potentially destructive or irreversible.
    High,
    /// Critical risk — strongly destructive patterns detected (e.g. `rm -rf /`).
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "LOW"),
            RiskLevel::Medium => write!(f, "MEDIUM"),
            RiskLevel::High => write!(f, "HIGH"),
            RiskLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

// ── Approval request ─────────────────────────────────────────────────────────

/// A request for the user to approve or deny a dangerous operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApprovalRequest {
    /// The name of the tool being invoked (e.g. `"bash"`, `"write_file"`).
    pub tool_name: String,
    /// Human-readable summary of the action (e.g. the command or path).
    pub action_summary: String,
    /// The risk level assessed for this specific request.
    pub risk_level: RiskLevel,
    /// Optional extra context shown to the user.
    pub explanation: Option<String>,
    /// The working directory, if relevant (bash tool).
    pub working_directory: Option<String>,
}

impl ApprovalRequest {
    /// Format a multi-line approval prompt suitable for display in the TUI log
    /// panel or a terminal banner.
    pub fn display_lines(&self) -> Vec<String> {
        let mut lines = vec![
            "╔══════════════ APPROVAL REQUIRED ══════════════╗".to_string(),
            format!("  Tool   : {}", self.tool_name),
            format!("  Action : {}", self.action_summary),
            format!("  Risk   : {}", self.risk_level),
        ];
        if let Some(ref exp) = self.explanation {
            lines.push(format!("  Note   : {}", exp));
        }
        if let Some(ref wd) = self.working_directory {
            lines.push(format!("  Cwd    : {}", wd));
        }
        lines.push(
            "  [y] Approve once  [n] Deny  [a] Always allow (session)  [d] Always deny (session)"
                .to_string(),
        );
        lines.push("╚════════════════════════════════════════════════╝".to_string());
        lines
    }
}

// ── Decision ─────────────────────────────────────────────────────────────────

/// The result of consulting the approval gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalDecision {
    /// The operation is approved and may proceed.
    Approved,
    /// The operation is denied. The provided message can be forwarded to the LLM
    /// so it can adapt its plan.
    Denied(String),
}

impl ApprovalDecision {
    /// Returns `true` if the decision is [`ApprovalDecision::Approved`].
    pub fn is_approved(&self) -> bool {
        matches!(self, ApprovalDecision::Approved)
    }
}

// ── Session policy ────────────────────────────────────────────────────────────

/// A per-tool session-level override set by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionPolicy {
    /// Always approve this tool for the rest of the session without prompting.
    AlwaysApprove,
    /// Always deny this tool for the rest of the session without prompting.
    AlwaysDeny,
}

// ── Approval gate ─────────────────────────────────────────────────────────────

/// The central approval gate. Holds session-level policies and processes
/// [`ApprovalRequest`]s.
///
/// In TUI mode, the gate does not read stdin itself. Instead it:
/// 1. Checks the session policy for an immediate decision.
/// 2. Returns [`ApprovalDecision::Denied`] with a pending marker so the caller
///    can surface the prompt to the user and call [`ApprovalGate::resolve`] with
///    the user's answer.
///
/// The [`ApprovalGate`] is `Clone` so the App can store it and the tools module
/// can receive a shared reference without lifetime issues.
#[derive(Debug, Clone)]
pub struct ApprovalGate {
    /// Per-tool name overrides for the current session.
    policies: HashMap<String, SessionPolicy>,
    /// The active approval profile.
    profile: crate::config::ApprovalProfile,
    /// Cache of previously approved validation commands in this session.
    approved_validations: std::collections::HashSet<String>,
}

impl Default for ApprovalGate {
    fn default() -> Self {
        Self {
            policies: HashMap::new(),
            profile: crate::config::ApprovalProfile::Strict,
            approved_validations: std::collections::HashSet::new(),
        }
    }
}

impl ApprovalGate {
    /// Create a new gate with default strict profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new gate with a specific profile.
    pub fn with_profile(profile: crate::config::ApprovalProfile) -> Self {
        Self {
            policies: HashMap::new(),
            profile,
            approved_validations: std::collections::HashSet::new(),
        }
    }

    /// Get the current approval profile.
    pub fn profile(&self) -> &crate::config::ApprovalProfile {
        &self.profile
    }

    /// Apply a session policy override for a given tool name.
    ///
    /// Called when the user answers `'a'` (always allow) or `'d'` (always deny).
    pub fn set_policy(&mut self, tool_name: &str, policy: SessionPolicy) {
        info!(tool = tool_name, ?policy, "session policy updated");
        self.policies.insert(tool_name.to_string(), policy);
    }

    /// Remove a session policy for a given tool (revert to interactive).
    pub fn clear_policy(&mut self, tool_name: &str) {
        self.policies.remove(tool_name);
    }

    /// Check if a session policy exists for a tool and return an immediate
    /// decision without requiring interactive input.
    ///
    /// Returns `None` if the user must be prompted interactively.
    pub fn check_policy(&self, request: &ApprovalRequest) -> Option<ApprovalDecision> {
        // Fast path for validation-fast profile
        if matches!(self.profile, crate::config::ApprovalProfile::ValidationFast) {
            // Only applies to safe validation commands (not extensions or arbitrary shells)
            // Assuming tools module prefixes these or we can identify them by tool_name.
            // For example, if tool_name is "code_execution" but it's a known validation.
            // A safer heuristic: we check if the request was specifically marked or if
            // it's in our approved_validations cache.
            if request.tool_name == "validate" || request.tool_name == "code_execution" {
                // If it's a validation command and we already approved it this session:
                let cache_key = format!("{}::{}", request.tool_name, request.action_summary);
                if self.approved_validations.contains(&cache_key) {
                    info!(
                        tool = %request.tool_name,
                        action = %request.action_summary,
                        "auto-approved by validation-fast profile (previously approved)"
                    );
                    return Some(ApprovalDecision::Approved);
                }
            }
        }

        // Audit-only profile always approves (dry-run mode handled elsewhere or logs it)
        if matches!(self.profile, crate::config::ApprovalProfile::AuditOnly) {
            info!(
                tool = %request.tool_name,
                action = %request.action_summary,
                "auto-approved by audit-only profile (dry-run)"
            );
            return Some(ApprovalDecision::Approved);
        }

        match self.policies.get(&request.tool_name) {
            Some(SessionPolicy::AlwaysApprove) => {
                info!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    "auto-approved by session policy"
                );
                Some(ApprovalDecision::Approved)
            }
            Some(SessionPolicy::AlwaysDeny) => {
                warn!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    "auto-denied by session policy"
                );
                Some(ApprovalDecision::Denied(format!(
                    "Tool '{}' is set to always-deny this session.",
                    request.tool_name
                )))
            }
            None => None,
        }
    }

    /// Resolve a pending approval request based on the user's raw input character.
    ///
    /// | Input | Outcome |
    /// |-------|---------|
    /// | `'y'` | `Approved` once |
    /// | `'a'` | `Approved` + set `AlwaysApprove` for this tool in session |
    /// | `'n'` | `Denied` once |
    /// | `'d'` | `Denied` + set `AlwaysDeny` for this tool in session |
    /// | anything else | `Denied` (safe default) |
    ///
    /// The decision is logged via `tracing`.
    pub fn resolve(&mut self, request: &ApprovalRequest, input: char) -> ApprovalDecision {
        match input {
            'y' | 'Y' => {
                info!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    risk = %request.risk_level,
                    "approved once by user"
                );

                // If validation-fast profile is active and it's a validation command, cache it
                if matches!(self.profile, crate::config::ApprovalProfile::ValidationFast) {
                    if request.tool_name == "validate" || request.tool_name == "code_execution" {
                        // Assuming risk level is low for typical validations
                        if request.risk_level == RiskLevel::Low
                            || request.risk_level == RiskLevel::Medium
                        {
                            let cache_key =
                                format!("{}::{}", request.tool_name, request.action_summary);
                            self.approved_validations.insert(cache_key);
                            info!("cached approved validation for fast reuse");
                        }
                    }
                }

                ApprovalDecision::Approved
            }
            'a' | 'A' => {
                self.set_policy(&request.tool_name, SessionPolicy::AlwaysApprove);
                info!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    risk = %request.risk_level,
                    "approved — session policy set to AlwaysApprove"
                );
                ApprovalDecision::Approved
            }
            'n' | 'N' => {
                warn!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    risk = %request.risk_level,
                    "denied by user"
                );
                ApprovalDecision::Denied(format!(
                    "User denied execution of '{}': {}",
                    request.tool_name, request.action_summary
                ))
            }
            'd' | 'D' => {
                self.set_policy(&request.tool_name, SessionPolicy::AlwaysDeny);
                warn!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    risk = %request.risk_level,
                    "denied — session policy set to AlwaysDeny"
                );
                ApprovalDecision::Denied(format!(
                    "User denied execution of '{}' and set session policy to always-deny.",
                    request.tool_name
                ))
            }
            other => {
                warn!(
                    tool = %request.tool_name,
                    action = %request.action_summary,
                    input = %other,
                    "invalid input — defaulting to deny"
                );
                ApprovalDecision::Denied(format!(
                    "User provided invalid input '{}', defaulting to deny.",
                    other
                ))
            }
        }
    }
}

// ── Phase 4.2: Approval Queue ────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct PendingApproval {
    pub id: String,
    pub created_at: u64,
    pub request: ApprovalRequest,
    pub source: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ApprovalHistoryEntry {
    pub id: String,
    pub created_at: u64,
    pub resolved_at: u64,
    pub request: ApprovalRequest,
    pub source: String,
    pub decision: char,
}

pub struct ApprovalQueue {
    pending: std::sync::Arc<
        tokio::sync::Mutex<HashMap<String, (PendingApproval, tokio::sync::oneshot::Sender<char>)>>,
    >,
    history: std::sync::Arc<tokio::sync::Mutex<Vec<ApprovalHistoryEntry>>>,
}

impl Default for ApprovalQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ApprovalQueue {
    pub fn new() -> Self {
        Self {
            pending: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            history: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn add(
        &self,
        request: ApprovalRequest,
        source: &str,
    ) -> (PendingApproval, tokio::sync::oneshot::Receiver<char>) {
        let id = uuid::Uuid::new_v4().to_string();
        let pending = PendingApproval {
            id: id.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            request,
            source: source.to_string(),
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.pending.lock().await.insert(id, (pending.clone(), tx));
        (pending, rx)
    }

    pub async fn list(&self) -> Vec<PendingApproval> {
        self.pending
            .lock()
            .await
            .values()
            .map(|(p, _)| p.clone())
            .collect()
    }

    pub async fn get(&self, id: &str) -> Option<PendingApproval> {
        self.pending.lock().await.get(id).map(|(p, _)| p.clone())
    }

    pub async fn resolve(&self, id: &str, decision: char) -> bool {
        if let Some((p, tx)) = self.pending.lock().await.remove(id) {
            let history_entry = ApprovalHistoryEntry {
                id: p.id,
                created_at: p.created_at,
                resolved_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                request: p.request,
                source: p.source,
                decision,
            };
            self.history.lock().await.push(history_entry);
            tx.send(decision).is_ok()
        } else {
            false
        }
    }

    pub async fn history(&self) -> Vec<ApprovalHistoryEntry> {
        self.history.lock().await.clone()
    }
}

// ── Risk assessment helpers ───────────────────────────────────────────────────

/// Patterns that are Critical only when the `/` targets root (i.e. `rm -rf /`
/// followed by end-of-string, space, or another option flag — not a sub-path
/// like `/tmp/foo`).
const CRITICAL_BASH_ROOT_TARGETS: &[&str] =
    &["rm -rf /", "rm -fr /", "chmod 777 /", "chmod -r 777 /"];

/// Patterns in shell commands that indicate a [`RiskLevel::Critical`] operation
/// via pure substring match (risk regardless of what follows).
const CRITICAL_BASH_PATTERNS: &[&str] = &[
    ":(){ :|:& };:",
    "mkfs",
    "dd if=",
    "chown -r root",
    "> /dev/sda",
    ">> /dev/sda",
    "shred",
    "wipefs",
];

/// Patterns in shell commands that indicate a [`RiskLevel::High`] operation.
const HIGH_BASH_PATTERNS: &[&str] = &[
    "rm -rf",
    "rm -fr",
    "sudo",
    "su ",
    "chmod -r",
    "chown -r",
    "curl | sh",
    "curl|sh",
    "wget | sh",
    "wget|sh",
    "| sh",
    "|sh",
    "| bash",
    "|bash",
    "bash <(",
    "sh <(",
    "pip uninstall",
    "npm uninstall",
    "apt remove",
    "apt purge",
    "apt-get remove",
    "apt-get purge",
    "yum remove",
    "dnf remove",
    "pacman -r",
    "brew uninstall",
    ".ssh",
    ".gnupg",
    "id_rsa",
    "id_ed25519",
    ".env",
    "credentials",
    "secret",
    "token",
    "api_key",
    "password",
    "passwd",
    "shadow",
    "/etc/sudoers",
    "iptables",
    "ufw",
    "firewall",
    "kill -9",
    "killall",
    "pkill",
    "systemctl stop",
    "systemctl disable",
    "service stop",
];

/// Returns `true` if `input` contains `pattern` where the character immediately
/// after the match is either end-of-string, whitespace, or `'"'`/`'\''`.
///
/// Used for patterns like `rm -rf /` to avoid matching `rm -rf /tmp/foo`.
fn contains_root_targeted(input: &str, pattern: &str) -> bool {
    let Some(pos) = input.find(pattern) else {
        return false;
    };
    let after = &input[pos + pattern.len()..];
    after.is_empty() || after.starts_with([' ', '\t', '\n', '"', '\''])
}

/// Assess the risk level of a bash command string.
pub fn assess_bash_risk(command: &str) -> RiskLevel {
    let lower = command.to_lowercase();

    // Root-targeted patterns: only critical when targeting root, not sub-paths.
    for pattern in CRITICAL_BASH_ROOT_TARGETS {
        if contains_root_targeted(&lower, pattern) {
            return RiskLevel::Critical;
        }
    }

    // Pure-substring critical patterns.
    for pattern in CRITICAL_BASH_PATTERNS {
        if lower.contains(pattern) {
            return RiskLevel::Critical;
        }
    }

    for pattern in HIGH_BASH_PATTERNS {
        if lower.contains(pattern) {
            return RiskLevel::High;
        }
    }

    // All bash commands are at least Medium risk — they execute arbitrary code.
    RiskLevel::Medium
}

/// Path prefixes that are always considered [`RiskLevel::Critical`] to write.
const CRITICAL_WRITE_PATHS: &[&str] = &[
    "/etc/", "/usr/", "/bin/", "/sbin/", "/boot/", "/sys/", "/proc/", "/dev/",
];

/// Path prefixes and substrings that are [`RiskLevel::High`] to write.
const HIGH_WRITE_PATHS: &[&str] = &[
    ".ssh",
    ".gnupg",
    ".aws",
    "id_rsa",
    "id_ed25519",
    ".env",
    "credentials",
    "secret",
    "token",
    "passwd",
    "shadow",
    "sudoers",
];

/// Assess the risk level of writing to a file path.
pub fn assess_write_risk(path: &str) -> RiskLevel {
    let lower = path.to_lowercase();

    for prefix in CRITICAL_WRITE_PATHS {
        if lower.starts_with(prefix) {
            return RiskLevel::Critical;
        }
    }

    for pattern in HIGH_WRITE_PATHS {
        if lower.contains(pattern) {
            return RiskLevel::High;
        }
    }

    // All file writes are at least Medium risk.
    RiskLevel::Medium
}

/// Assess the risk level of spawning an external agent subprocess.
pub fn assess_subagent_risk(agent_cli: &str) -> RiskLevel {
    // External agent spawns are always at least High — they run arbitrary code.
    let lower = agent_cli.to_lowercase();
    if lower.contains("sudo") || lower.contains("rm") || lower.contains("dd") {
        RiskLevel::Critical
    } else {
        RiskLevel::High
    }
}

/// Build an [`ApprovalRequest`] for a `bash` tool call.
pub fn bash_approval_request(command: &str) -> ApprovalRequest {
    let risk = assess_bash_risk(command);
    let explanation = if risk >= RiskLevel::High {
        Some(
            "This command matches patterns associated with destructive or privileged operations."
                .to_string(),
        )
    } else {
        Some("All shell commands require approval before execution.".to_string())
    };

    ApprovalRequest {
        tool_name: "bash".to_string(),
        action_summary: redact_secrets(command),
        risk_level: risk,
        explanation,
        working_directory: std::env::current_dir()
            .ok()
            .map(|p| p.display().to_string()),
    }
}

/// Build an [`ApprovalRequest`] for a `write_file` tool call.
pub fn write_file_approval_request(path: &str, content_preview: &str) -> ApprovalRequest {
    let risk = assess_write_risk(path);
    let preview = if content_preview.len() > 120 {
        format!(
            "{}… ({} bytes total)",
            &content_preview[..120],
            content_preview.len()
        )
    } else {
        content_preview.to_string()
    };

    ApprovalRequest {
        tool_name: "write_file".to_string(),
        action_summary: format!("Write to: {}", path),
        risk_level: risk,
        explanation: Some(format!("Content preview: {}", preview)),
        working_directory: None,
    }
}

/// Build an [`ApprovalRequest`] for a `call_subagent` tool call.
pub fn call_subagent_approval_request(agent_cli: &str, prompt: &str) -> ApprovalRequest {
    let risk = assess_subagent_risk(agent_cli);
    let prompt_preview = if prompt.len() > 120 {
        format!("{}… ({} chars)", &prompt[..120], prompt.len())
    } else {
        prompt.to_string()
    };

    ApprovalRequest {
        tool_name: "call_subagent".to_string(),
        action_summary: format!("Spawn external agent: {}", agent_cli),
        risk_level: risk,
        explanation: Some(format!("Prompt: {}", prompt_preview)),
        working_directory: None,
    }
}

// ── Secret redaction ─────────────────────────────────────────────────────────

/// Simple secret redaction: replaces values that look like API keys or tokens
/// with `[REDACTED]` before displaying them to the user or writing to logs.
///
/// This is a best-effort heuristic, not a guarantee.
pub fn redact_secrets(input: &str) -> String {
    // Redact anything that looks like an API key or token assignment.
    // Patterns: KEY=value, key: value, "key": "value"
    let mut result = input.to_string();

    let secret_keywords = [
        "api_key",
        "apikey",
        "api-key",
        "secret",
        "token",
        "password",
        "passwd",
        "authorization",
        "bearer",
        "sk-",
        "gsk_",
        "ghp_",
        "xoxb-",
    ];

    for keyword in secret_keywords {
        // Redact assignment patterns: KEY=XXXX (up to 120 chars)
        if let Some(pos) = result.to_lowercase().find(keyword) {
            let after = &result[pos..];
            // Find the value part after =, :, or space
            if let Some(sep) = after.find(['=', ':', '"']) {
                let value_start = pos + sep + 1;
                if value_start < result.len() {
                    // Find end of value (whitespace, quote, newline)
                    let value_portion = &result[value_start..];
                    let value_end = value_portion
                        .find([' ', '\n', '\r', '\t', '"', '\'', ','])
                        .unwrap_or(value_portion.len().min(120));
                    if value_end > 4 {
                        let actual_end = value_start + value_end;
                        result
                            .replace_range(value_start..actual_end.min(result.len()), "[REDACTED]");
                    }
                }
            }
        }
    }

    result
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
    }

    #[test]
    fn test_bash_risk_critical() {
        assert_eq!(assess_bash_risk("rm -rf /"), RiskLevel::Critical);
        assert_eq!(assess_bash_risk("mkfs.ext4 /dev/sda"), RiskLevel::Critical);
        assert_eq!(
            assess_bash_risk("dd if=/dev/zero of=/dev/sda"),
            RiskLevel::Critical
        );
    }

    #[test]
    fn test_bash_risk_high() {
        assert_eq!(assess_bash_risk("sudo apt-get update"), RiskLevel::High);
        assert_eq!(assess_bash_risk("rm -rf /tmp/foo"), RiskLevel::High);
        assert_eq!(
            assess_bash_risk("curl http://x.com/script.sh | sh"),
            RiskLevel::High
        );
        assert_eq!(assess_bash_risk("cat ~/.ssh/id_rsa"), RiskLevel::High);
        assert_eq!(assess_bash_risk("chmod -R 755 /home"), RiskLevel::High);
    }

    #[test]
    fn test_bash_risk_medium() {
        assert_eq!(assess_bash_risk("ls -la /tmp"), RiskLevel::Medium);
        assert_eq!(assess_bash_risk("echo hello"), RiskLevel::Medium);
        assert_eq!(assess_bash_risk("cargo build"), RiskLevel::Medium);
        assert_eq!(assess_bash_risk("git status"), RiskLevel::Medium);
    }

    #[test]
    fn test_write_risk_critical() {
        assert_eq!(assess_write_risk("/etc/passwd"), RiskLevel::Critical);
        assert_eq!(assess_write_risk("/usr/bin/ls"), RiskLevel::Critical);
        assert_eq!(assess_write_risk("/bin/sh"), RiskLevel::Critical);
    }

    #[test]
    fn test_write_risk_high() {
        assert_eq!(
            assess_write_risk("/home/user/.ssh/authorized_keys"),
            RiskLevel::High
        );
        assert_eq!(assess_write_risk("/home/user/.env"), RiskLevel::High);
        assert_eq!(
            assess_write_risk("/home/user/.aws/credentials"),
            RiskLevel::High
        );
    }

    #[test]
    fn test_write_risk_medium() {
        assert_eq!(
            assess_write_risk("/home/user/project/main.rs"),
            RiskLevel::Medium
        );
        assert_eq!(assess_write_risk("/tmp/output.txt"), RiskLevel::Medium);
    }

    #[test]
    fn test_approval_gate_policy() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls -la".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        // No policy → None (interactive required)
        assert_eq!(gate.check_policy(&req), None);

        // Set always-approve → immediate Approved
        gate.set_policy("bash", SessionPolicy::AlwaysApprove);
        assert_eq!(gate.check_policy(&req), Some(ApprovalDecision::Approved));

        // Set always-deny → immediate Denied
        gate.set_policy("bash", SessionPolicy::AlwaysDeny);
        assert!(matches!(
            gate.check_policy(&req),
            Some(ApprovalDecision::Denied(_))
        ));

        // Clear policy → back to interactive
        gate.clear_policy("bash");
        assert_eq!(gate.check_policy(&req), None);
    }

    #[test]
    fn test_resolve_approved_once() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        assert_eq!(gate.resolve(&req, 'y'), ApprovalDecision::Approved);
        // Should NOT have set a session policy
        assert_eq!(gate.check_policy(&req), None);
    }

    #[test]
    fn test_resolve_always_approve() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        assert_eq!(gate.resolve(&req, 'a'), ApprovalDecision::Approved);
        // Should have set AlwaysApprove policy
        assert_eq!(gate.check_policy(&req), Some(ApprovalDecision::Approved));
    }

    #[test]
    fn test_resolve_denied() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        assert!(matches!(
            gate.resolve(&req, 'n'),
            ApprovalDecision::Denied(_)
        ));
        // Should NOT have set a session policy
        assert_eq!(gate.check_policy(&req), None);
    }

    #[test]
    fn test_resolve_always_deny() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        assert!(matches!(
            gate.resolve(&req, 'd'),
            ApprovalDecision::Denied(_)
        ));
        // Should have set AlwaysDeny policy
        assert!(matches!(
            gate.check_policy(&req),
            Some(ApprovalDecision::Denied(_))
        ));
    }

    #[test]
    fn test_resolve_unknown_input_defaults_to_denied() {
        let mut gate = ApprovalGate::new();
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "ls".to_string(),
            risk_level: RiskLevel::Medium,
            explanation: None,
            working_directory: None,
        };

        // Any unrecognized input → Denied
        assert!(matches!(
            gate.resolve(&req, 'x'),
            ApprovalDecision::Denied(_)
        ));
        assert!(matches!(
            gate.resolve(&req, ' '),
            ApprovalDecision::Denied(_)
        ));
        assert!(matches!(
            gate.resolve(&req, '\n'),
            ApprovalDecision::Denied(_)
        ));
    }

    #[test]
    fn test_redact_secrets_basic() {
        let input = "export OPENAI_API_KEY=sk-abc123def456";
        let redacted = redact_secrets(input);
        assert!(
            !redacted.contains("sk-abc123def456"),
            "Secret not redacted: {}",
            redacted
        );
    }

    #[test]
    fn test_redact_no_false_positives() {
        // Normal commands should not be mangled
        let input = "ls -la /tmp";
        let redacted = redact_secrets(input);
        assert_eq!(redacted, input);
    }

    #[test]
    fn test_display_lines_format() {
        let req = ApprovalRequest {
            tool_name: "bash".to_string(),
            action_summary: "rm -rf /tmp/test".to_string(),
            risk_level: RiskLevel::High,
            explanation: Some("Destructive pattern detected".to_string()),
            working_directory: Some("/home/user/project".to_string()),
        };
        let lines = req.display_lines();
        assert!(lines.iter().any(|l| l.contains("bash")));
        assert!(lines.iter().any(|l| l.contains("HIGH")));
        assert!(lines.iter().any(|l| l.contains("Destructive")));
        assert!(lines.iter().any(|l| l.contains("[y]")));
    }

    #[test]
    fn test_validation_fast_profile() {
        let mut gate = ApprovalGate::with_profile(crate::config::ApprovalProfile::ValidationFast);
        let req = ApprovalRequest {
            tool_name: "validate".to_string(),
            action_summary: "cargo test".to_string(),
            risk_level: RiskLevel::Low,
            explanation: None,
            working_directory: None,
        };

        // First time, no policy yet, should require interactive
        assert_eq!(gate.check_policy(&req), None);

        // Approve it interactively
        let decision = gate.resolve(&req, 'y');
        assert_eq!(decision, ApprovalDecision::Approved);

        // Next time, same command should be auto-approved by validation-fast
        assert_eq!(gate.check_policy(&req), Some(ApprovalDecision::Approved));
    }
}
