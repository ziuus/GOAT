use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

use crate::approval::{ApprovalDecision, ApprovalGate, ApprovalRequest, RiskLevel};
use crate::config::{HookRuleConfig, HooksConfig};
use crate::paths::GoatPaths;

pub struct HooksManager {
    config: HooksConfig,
    paths: GoatPaths,
}

impl HooksManager {
    pub fn new(config: HooksConfig, paths: GoatPaths) -> Self {
        Self { config, paths }
    }

    pub async fn run_hooks(&self, event: &str, gate: &mut ApprovalGate) -> Result<Vec<String>> {
        let mut logs = Vec::new();
        if !self.config.enabled {
            return Ok(logs);
        }

        let hooks: Vec<_> = self
            .config
            .rules
            .iter()
            .filter(|r| r.event == event && r.enabled)
            .collect();

        for hook in hooks {
            logs.push(format!(
                "[HOOK] Triggered '{}' (event: {})",
                hook.name, event
            ));
            let log_msg = match hook.r#type.as_str() {
                "log_only" => format!("[HOOK] Log-only hook '{}' executed.", hook.name),
                "command" => {
                    if let Some(cmd) = &hook.command {
                        if self.config.require_approval {
                            let risk = match hook.risk.as_str() {
                                "deny" => RiskLevel::Critical,
                                "allow" => RiskLevel::Low,
                                _ => RiskLevel::High, // ask
                            };
                            let req = ApprovalRequest {
                                tool_name: "hook_command".to_string(),
                                action_summary: format!("Run hook '{}'", hook.name),
                                risk_level: risk,
                                explanation: Some(format!("Command: {}", cmd)),
                                working_directory: None,
                            };
                            if let Some(decision) = gate.check_policy(&req) {
                                match decision {
                                    ApprovalDecision::Approved => self.execute_command(cmd).await,
                                    ApprovalDecision::Denied(r) => {
                                        format!("[HOOK] Denied by policy: {}", r)
                                    }
                                }
                            } else {
                                // Manual approval required, but we can't block here asynchronously cleanly
                                // without breaking the flow if we're mid-LLM loop.
                                // Actually we will assume hooks requiring approval in non-interactive modes
                                // are denied if they don't meet policy.
                                // For interactive modes, we prompt via stdin.
                                let decision = crate::headless::prompt_approval_stdin(&req, gate);
                                match decision {
                                    ApprovalDecision::Approved => self.execute_command(cmd).await,
                                    ApprovalDecision::Denied(r) => {
                                        format!("[HOOK] Denied: {}", r)
                                    }
                                }
                            }
                        } else {
                            self.execute_command(cmd).await
                        }
                    } else {
                        format!(
                            "[HOOK] Command hook '{}' has no command configured.",
                            hook.name
                        )
                    }
                }
                "internal_action" => format!(
                    "[HOOK] Internal action '{}' not fully implemented yet.",
                    hook.name
                ),
                _ => format!(
                    "[HOOK] Unknown type '{}' for hook '{}'",
                    hook.r#type, hook.name
                ),
            };
            logs.push(log_msg.clone());
            self.log_audit(&hook.name, event, &log_msg);
        }

        Ok(logs)
    }

    async fn execute_command(&self, cmd: &str) -> String {
        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                if stderr.is_empty() {
                    format!("[HOOK] Executed successfully. Output:\n{}", stdout)
                } else {
                    format!(
                        "[HOOK] Executed with stderr.\nSTDOUT:\n{}\nSTDERR:\n{}",
                        stdout, stderr
                    )
                }
            }
            Err(e) => format!("[HOOK] Failed to execute command: {}", e),
        }
    }

    fn log_audit(&self, name: &str, event: &str, msg: &str) {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.paths.data_dir.join("hook-audit.log"))
        {
            use std::io::Write;
            let now = chrono::Utc::now().to_rfc3339();
            let _ = writeln!(
                file,
                "[{}] Hook: {} | Event: {} | {}",
                now, name, event, msg
            );
        }
    }
}

impl HooksManager {
    pub fn list_hooks_info(&self) -> Vec<String> {
        self.config
            .rules
            .iter()
            .map(|r| {
                format!(
                    "{} (event: {}, type: {}, enabled: {})",
                    r.name, r.event, r.r#type, r.enabled
                )
            })
            .collect()
    }
}
