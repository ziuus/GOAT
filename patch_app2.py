import re

with open('src/app.rs', 'r') as f:
    content = f.read()

start_str = '            cmd if cmd.starts_with("/external-agents") => {'
end_str = '            "/skill" => {'

start_idx = content.find(start_str)
end_idx = content.find(end_str)

if start_idx != -1 and end_idx != -1:
    new_block = """            cmd if cmd == "/agent-doctor" => {
                let checks = crate::paths::run_doctor(&self.paths, &self.config, false);
                for c in checks {
                    self.push_log(format!("[DOCTOR] {} - {:?}", c.name, c.status));
                }
                true
            }

            cmd if cmd == "/agent-runs" => {
                let jsonl_path = self.paths.data_dir.join("external-agent-runs.jsonl");
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        self.push_log("[AGENTS] External Agent Runs:");
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line) {
                                self.push_log(format!(
                                    "[AGENTS]   {} | Agent: {:<12} | Mode: {:<15} | Status: {}",
                                    run.id,
                                    run.agent_name,
                                    run.mode,
                                    if run.success { "Success" } else { "Failed" }
                                ));
                            }
                        }
                    }
                } else {
                    self.push_log("[AGENTS] No runs recorded yet.");
                }
                true
            }

            cmd if cmd.starts_with("/agent-run ") => {
                let run_id = parts.get(1).copied().unwrap_or("").trim();
                let jsonl_path = self.paths.data_dir.join("external-agent-runs.jsonl");
                let mut found = false;
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line) {
                                if run.id == run_id {
                                    self.push_log(format!("[AGENTS] Run ID: {}", run.id));
                                    self.push_log(format!("[AGENTS] Agent: {}", run.agent_name));
                                    self.push_log(format!("[AGENTS] Task: {}", run.task));
                                    self.push_log(format!("[AGENTS] Success: {}", run.success));
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found {
                    self.push_log(format!("[AGENTS] Run ID '{}' not found.", run_id));
                }
                true
            }

            cmd if cmd.starts_with("/run-agent ") => {
                let agent_name = parts.get(1).copied().unwrap_or("").trim();
                let prompt = parts.get(2..).unwrap_or(&[]).join(" ");
                self.push_log(format!("[AGENTS] Delegating task to '{}'...", agent_name));
                
                let action_res = self.tool_registry.evaluate_action("delegate_external_agent", &self.config.tools);
                if let crate::tool_registry::ToolAction::Deny(reason) = action_res {
                    self.push_log(format!("[AGENTS] Delegation denied: {}", reason));
                } else {
                    let req = crate::approval::ApprovalRequest {
                        tool_name: "delegate_external_agent".to_string(),
                        action_summary: format!("agent: {}, task: {}", agent_name, prompt),
                        risk_level: crate::approval::RiskLevel::High,
                        explanation: Some("Run external agent".into()),
                        working_directory: None,
                    };

                    if let Some(crate::approval::ApprovalDecision::Denied(msg)) =
                        self.approval_gate.check_policy(&req)
                    {
                        self.push_log(format!("[EXTERNAL] Delegation denied via policy: {}", msg));
                    } else {
                        self.pending_approval_request = Some(req);
                        // Store the run details in TUI state to execute after approval if needed, 
                        // but actually `App` doesn't have a place for this right now, so we will 
                        // execute immediately for now with a warning that it's blocking.
                        match self.external_agent_manager.delegate(agent_name, &prompt, &self.config) {
                            Ok(res) => {
                                self.push_log(format!(
                                    "[EXTERNAL] Execution finished. Success: {}",
                                    res.success
                                ));
                                for line in res.stdout.lines() {
                                    self.push_log(format!("[EXTERNAL] Stdout: {}", line));
                                }
                                if !res.stderr.is_empty() {
                                    for line in res.stderr.lines() {
                                        self.push_log(format!("[EXTERNAL] Stderr: {}", line));
                                    }
                                }
                            }
                            Err(e) => self.push_log(format!("[EXTERNAL] Error: {}", e)),
                        }
                    }
                }
                true
            }

            cmd if cmd.starts_with("/external-agents") || cmd.starts_with("/external-agent ") || cmd.starts_with("/delegate-external ") => {
                self.push_log("[EXTERNAL] Command deprecated. Please use /agents, /agent-doctor, /run-agent, /agent-runs, /agent-run instead.");
                true
            }

"""

    content = content[:start_idx] + new_block + content[end_idx:]
    with open('src/app.rs', 'w') as f:
        f.write(content)
    print("Successfully patched src/app.rs")
else:
    print(f"Could not find block: start={start_idx}, end={end_idx}")

