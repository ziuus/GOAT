import re

with open('src/cli.rs', 'r') as f:
    content = f.read()

old_func_start = content.find("fn handle_external_agents_command(")
if old_func_start == -1:
    print("Could not find handle_external_agents_command")
    exit(1)

old_func_end = content.find("fn handle_hooks_command(", old_func_start)
if old_func_end == -1:
    print("Could not find end of delegate function")
    exit(1)

new_func = """fn handle_agent_command(
    mut rt: crate::runtime::GoatRuntime,
    action: &str,
    arg: Option<&str>,
    prompt: Option<&str>,
    mission: Option<&str>,
) {
    let mut ext_mgr = rt.external_agent_manager;
    ext_mgr.detect_all(&rt.config);

    match action {
        "list" | "agents" => {
            let agents = ext_mgr.registry.list_all();
            println!("GOAT External Agent Registry ({} adapters)", agents.len());
            for agent in agents {
                println!(
                    "  {:<15} [{}] - {}",
                    agent.name, agent.command_name, agent.status
                );
            }
        }
        "detect" => {
            println!("Detecting external agents...");
            for agent in ext_mgr.registry.list_all() {
                println!("  {:<15} - {}", agent.name, agent.status);
            }
        }
        "show" => {
            let name = arg.unwrap_or("");
            if let Some(agent) = ext_mgr.registry.get(name) {
                println!("Name: {}", agent.name);
                println!("Command: {}", agent.command_name);
                println!("Status: {}", agent.status);
                println!("Risk: {:?}", agent.risk_level);
                println!("Workspace Behavior: {}", agent.workspace_behavior);
                if let Some(ref path) = agent.detected_path {
                    println!("Detected Path: {}", path.display());
                }
            } else {
                println!("External agent '{}' not found.", name);
            }
        }
        "audit" => {
            if rt.paths.external_agent_audit_log_file.exists() {
                if let Ok(content) =
                    std::fs::read_to_string(&rt.paths.external_agent_audit_log_file)
                {
                    println!("{}", content);
                }
            } else {
                println!("No external agent audit log found.");
            }
        }
        "doctor" => {
            let checks = crate::paths::run_doctor(&rt.paths, &rt.config, false);
            crate::paths::print_doctor_results(&checks);
        }
        "runs" => {
            let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
            if jsonl_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                    println!("External Agent Runs:");
                    for line in content.lines() {
                        if let Ok(run) =
                            serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line)
                        {
                            println!(
                                "  {} | Agent: {:<12} | Mode: {:<15} | Status: {}",
                                run.id,
                                run.agent_name,
                                run.mode,
                                if run.success { "Success" } else { "Failed" }
                            );
                        }
                    }
                }
            } else {
                println!("No runs recorded yet.");
            }
        }
        "run-show" => {
            if let Some(run_id) = arg {
                let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
                let mut found = false;
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<
                                crate::external_agents::ExternalAgentRun,
                            >(line)
                            {
                                if run.id == run_id {
                                    println!("Run ID: {}", run.id);
                                    println!("Agent: {}", run.agent_name);
                                    println!("Timestamp: {}", run.timestamp);
                                    println!("Mode: {}", run.mode);
                                    println!("Workspace: {}", run.workspace_path.display());
                                    println!("Task: {}", run.task);
                                    println!("Success: {}", run.success);
                                    println!("Duration: {:?}", run.duration);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found {
                    println!("Run ID '{}' not found.", run_id);
                }
            } else {
                println!("Usage: goat agent run-show <id>");
            }
        }
        "run" => {
            let name = arg.unwrap_or("");
            let task = prompt.unwrap_or("Test run");
            println!("Delegating task to external agent '{}'...", name);

            let action_res = rt
                .tool_registry
                .evaluate_action("delegate_external_agent", &rt.config.tools);
            if let crate::tool_registry::ToolAction::Deny(reason) = action_res {
                println!("Delegation denied by tool registry: {}", reason);
                return;
            }

            let req = crate::approval::ApprovalRequest {
                tool_name: "delegate_external_agent".to_string(),
                action_summary: format!("agent: {}, task: {}", name, task),
                risk_level: crate::approval::RiskLevel::High,
                explanation: Some("Running external agent command".into()),
                working_directory: None,
            };

            let decision = if let Some(decision) = rt.approval_gate.check_policy(&req) {
                decision
            } else {
                let mut lines = req.display_lines();
                for line in lines {
                    println!("{}", line);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let char_in = input.trim().chars().next().unwrap_or('n');
                rt.approval_gate.resolve(&req, char_in)
            };

            if let crate::approval::ApprovalDecision::Approved = decision {
                match ext_mgr.delegate(name, task, &rt.config) {
                    Ok(res) => {
                        println!("Execution finished. Success: {}", res.success);
                        println!("Stdout:\\n{}", res.stdout);
                        if !res.stderr.is_empty() {
                            println!("Stderr:\\n{}", res.stderr);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Execution denied.");
            }
        }
        "compare" => {
            println!("Compare feature is planned for Phase 8.9.");
        }
        _ => {
            println!("Unknown agent action: {}", action);
            println!("Valid actions: list, doctor, runs, run-show <id>, run <name> --prompt <...>, compare");
        }
    }
}

"""

new_content = content[:old_func_start] + new_func + content[old_func_end:]
with open('src/cli.rs', 'w') as f:
    f.write(new_content)

print("Updated CLI")
