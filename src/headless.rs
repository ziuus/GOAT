//! Headless (non-TUI) GOAT interaction loop.
//!
//! When started with `goat --headless`, this module runs the agent using
//! plain stdin/stdout instead of the ratatui TUI.  All core features —
//! config, paths, brain/session, LLM provider, native tools, ApprovalGate —
//! are identical to the TUI path.  Only the I/O layer is different.
//!
//! # Approval in headless mode
//!
//! When the agent wants to run a dangerous tool (bash, write_file,
//! call_subagent), the headless loop prints the approval prompt to stdout
//! and reads a single character from stdin, exactly like a classic CLI
//! `y/n` prompt.  The same `ApprovalGate` logic applies.
//!
//! # Exiting
//!
//! - `Ctrl+D` (EOF on stdin) → clean exit.
//! - `Ctrl+C` (SIGINT) → clean exit via signal handler.
//! - Empty line → ignored, prompt re-shown.

use crate::approval::{ApprovalDecision, ApprovalRequest};
use crate::llm::{FunctionDeclaration, Message, Tool};
use crate::runtime::GoatRuntime;
use crate::tools::NativeTools;
use anyhow::Result;
use serde_json::Value;
use std::io::{self, BufRead, Write};
use tracing::info;

const MAX_HISTORY_MESSAGES: usize = 80;

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the headless agent loop.
///
/// Blocks until the user sends EOF (Ctrl+D) or the process receives SIGINT.
pub async fn run(mut rt: GoatRuntime) -> Result<()> {
    print_banner(&rt);

    // Print any startup warnings.
    for w in &rt.startup_warnings {
        eprintln!("{}", w);
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        // Show the prompt.
        print!("> ");
        io::stdout().flush().ok();

        // Read a line from stdin.
        let line = match lines.next() {
            Some(Ok(l)) => l,
            Some(Err(e)) => {
                eprintln!("[ERROR] stdin read error: {}", e);
                break;
            }
            None => {
                // EOF (Ctrl+D)
                println!("\n[GOAT] EOF received — goodbye!");
                break;
            }
        };

        let input = line.trim().to_string();
        if input.is_empty() {
            continue;
        }

        // Handle built-in headless commands (subset of slash commands).
        if input.starts_with('/') {
            if handle_slash_command(&input, &mut rt).await {
                continue;
            }
            println!(
                "[GOAT] Unknown command '{}'. Type /help for available commands.",
                input
            );
            continue;
        }

        // Run the agent loop for this prompt.
        run_agent_turn(&mut rt, input).await;
    }

    // Shutdown MCP servers on exit.
    let shutdown_logs = rt.mcp_manager.shutdown_all().await;
    for l in shutdown_logs {
        println!("{}", l);
    }

    info!("headless session ended: {}", rt.session_id);
    Ok(())
}

// ── Banner ────────────────────────────────────────────────────────────────────

fn print_banner(rt: &GoatRuntime) {
    println!("╔══════════════════════════════════════════════════╗");
    println!(
        "║  GOAT Headless v{}                         ║",
        env!("CARGO_PKG_VERSION")
    );
    println!("╚══════════════════════════════════════════════════╝");
    println!("Provider : {}", rt.provider_label);
    println!("Profile  : {}", rt.active_profile);
    println!("Fallback : {}", rt.model_chain.fallback_display());
    println!(
        "Session  : {}",
        if rt.session_id.len() > 20 {
            &rt.session_id[..20]
        } else {
            &rt.session_id
        }
    );
    println!(
        "Brain    : {}",
        if rt.brain_disabled {
            "disabled (--no-brain)".to_string()
        } else if rt.brain.is_some() {
            rt.paths.db_file.to_string_lossy().to_string()
        } else {
            "unavailable (running without memory)".to_string()
        }
    );
    println!(
        "Mode     : {} session",
        if rt.session_resumed { "resumed" } else { "new" }
    );
    println!();
    println!("Type a message and press Enter. Ctrl+D or Ctrl+C to exit.");
    println!("Slash commands: /help /status /profile /profiles /clear /sessions /tools /new /exit");
    println!();
}

// ── Slash command handling (headless subset) ──────────────────────────────────

/// Handle headless slash commands.  Returns `true` if the command was handled.
async fn handle_slash_command(cmd: &str, rt: &mut GoatRuntime) -> bool {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let name = parts[0].to_lowercase();

    match name.as_str() {
        "/help" => {
            println!("[HELP] Headless commands:");
            println!("[HELP]   /help            — show this help");
            println!("[HELP]   /status          — show provider/session/brain status");
            println!("[HELP]   /profile         — show current profile");
            println!("[HELP]   /profile <name>  — switch to a profile");
            println!("[HELP]   /profiles        — list available profiles");
            println!("[HELP]   /new             — start a new session");
            println!("[HELP]   /clear           — clear screen and reset");
            println!("[HELP]   /sessions        — list recent sessions");
            println!("[HELP]   /tools           — list available tools");
            println!("[HELP]   /exit            — exit GOAT headless");
            println!("[HELP]");
            println!("[HELP] Approval (when prompted):");
            println!("[HELP]   y — approve once");
            println!("[HELP]   n — deny");
            println!("[HELP]   a — always allow this tool (session)");
            println!("[HELP]   d — always deny this tool (session)");
            true
        }

        "/status" => {
            println!("[STATUS] Provider : {}", rt.provider_label);
            println!("[STATUS] Profile  : {}", rt.active_profile);
            println!("[STATUS] Fallback : {}", rt.model_chain.fallback_display());
            println!("[STATUS] Session  : {}", rt.session_id);
            println!(
                "[STATUS] Brain    : {}",
                if rt.brain_disabled {
                    "disabled (--no-brain)"
                } else if rt.brain.is_some() {
                    "connected"
                } else {
                    "unavailable"
                }
            );
            println!(
                "[STATUS] Retries  : {} max / {}s timeout",
                rt.config.llm.effective_max_retries(),
                rt.config.llm.effective_timeout_secs()
            );
            println!("[STATUS] History  : {} messages", rt.history.len());
            println!("[STATUS] MCP      : {} server(s)", rt.mcp_server_count);
            true
        }

        "/clear" => {
            // Clear screen and reprint banner.
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().ok();
            print_banner(rt);
            true
        }

        "/sessions" => {
            println!("[SESSION] Current: {}", rt.session_id);
            if let Some(ref brain) = rt.brain {
                match brain.get_session_records() {
                    Ok(records) => {
                        println!("[SESSION] {} session(s) in brain:", records.len());
                        for r in records.iter().take(10) {
                            let short_id = if r.id.len() > 8 {
                                format!("{}…", &r.id[..8])
                            } else {
                                r.id.clone()
                            };
                            let kind = if r.is_uuid() { "uuid" } else { "legacy" };
                            let ts = r.updated_at.get(..16).unwrap_or(&r.updated_at);
                            println!("[SESSION]   {}  [{}]  {}  {}", short_id, kind, ts, r.title);
                        }
                    }
                    Err(e) => println!("[SESSION] Error: {}", e),
                }
            } else {
                println!("[SESSION] Brain not connected.");
            }
            true
        }

        "/profile" => {
            let arg = parts.get(1).copied().unwrap_or("").trim();
            if arg.is_empty() {
                println!("[PROFILE] Active : {}", rt.active_profile);
                println!("[PROFILE] Primary: {}", rt.model_chain.primary_display());
                println!("[PROFILE] Fallback: {}", rt.model_chain.fallback_display());
                println!("[PROFILE] Use /profile <name> to switch. Use /profiles to list.");
            } else {
                match rt.switch_profile(arg) {
                    Ok(()) => {
                        println!(
                            "[PROFILE] Switched to '{}' — {} → {}",
                            arg,
                            rt.model_chain.primary_display(),
                            rt.model_chain.fallback_display()
                        );
                    }
                    Err(e) => println!("[PROFILE] {}", e),
                }
            }
            true
        }

        "/profiles" => {
            let names = rt.profile_registry.profile_names();
            println!("[PROFILES] {} profiles available:", names.len());
            for name in &names {
                let chain = rt.profile_registry.profiles.get(*name);
                let primary = chain.map(|c| c.primary_display()).unwrap_or_default();
                let fallback = chain.map(|c| c.fallback_display()).unwrap_or_default();
                let active_marker = if *name == rt.active_profile.as_str() {
                    " ✓ (active)"
                } else {
                    ""
                };
                println!(
                    "[PROFILES]   {:12}  {} → {}{}",
                    name, primary, fallback, active_marker
                );
            }
            println!("[PROFILES] Use /profile <name> to switch.");
            true
        }

        "/new" => {
            let new_id = rt.create_new_session();
            println!("[SESSION] New session started: {}", new_id);
            println!("[SESSION] History cleared. Ready for a fresh conversation.");
            true
        }

        "/tools" => {
            let tools = NativeTools::all_tools();
            println!("[TOOLS] {} native tools:", tools.len());
            for t in &tools {
                println!("[TOOLS]   {} — {}", t.function.name, t.function.description);
            }
            let mcp_tools = rt.mcp_manager.all_tools();
            if !mcp_tools.is_empty() {
                println!("[TOOLS] {} MCP tools:", mcp_tools.len());
                for t in &mcp_tools {
                    if let Some(name) = t.get("name").and_then(|v| v.as_str()) {
                        println!("[TOOLS]   {}", name);
                    }
                }
            }
            true
        }

        "/exit" => {
            println!("[GOAT] Goodbye!");
            std::process::exit(0);
        }

        _ => false,
    }
}

// ── Main agent turn ───────────────────────────────────────────────────────────

/// Run a single user-prompt → agent-response turn.
async fn run_agent_turn(rt: &mut GoatRuntime, user_msg: String) {
    println!("[YOU] {}", user_msg);

    // Persist to brain.
    if let Some(ref brain) = rt.brain {
        let _ = brain.log_interaction(&rt.session_id, "user", &user_msg);
    }

    rt.history.push(Message {
        role: "user".to_string(),
        content: Some(user_msg),
        tool_calls: None,
        tool_call_id: None,
    });
    trim_history(&mut rt.history);

    // ReAct loop — up to 10 iterations.
    for _iteration in 0..10 {
        // Build tool list.
        let mcp_tools = rt.mcp_manager.all_tools();
        let mut mapped_tools = NativeTools::all_tools();
        for tool in &mcp_tools {
            if let (Some(name), Some(desc), Some(schema)) = (
                tool.get("name").and_then(|v| v.as_str()),
                tool.get("description").and_then(|v| v.as_str()),
                tool.get("inputSchema"),
            ) {
                mapped_tools.push(Tool {
                    r#type: "function".to_string(),
                    function: FunctionDeclaration {
                        name: name.to_string(),
                        description: desc.to_string(),
                        parameters: schema.clone(),
                    },
                });
            }
        }
        let tools = if mapped_tools.is_empty() {
            None
        } else {
            Some(mapped_tools)
        };

        // Route and call LLM.
        let route = rt.swarm_router.route(
            rt.history
                .last()
                .and_then(|m| m.content.as_deref())
                .unwrap_or_default(),
        );

        let mut routed_history = vec![Message {
            role: "system".to_string(),
            content: Some(route.profile.system_prompt.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }];
        routed_history.extend(rt.history.clone());

        print!("[GOAT] Thinking… ");
        io::stdout().flush().ok();

        match rt
            .llm_router
            .completion_with_fallback(&rt.model_chain, routed_history, tools)
            .await
        {
            Ok((response, used_label)) => {
                // Update provider label to show which model actually responded.
                rt.provider_label = used_label;

                rt.history.push(Message {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                    tool_calls: response.tool_calls.clone(),
                    tool_call_id: None,
                });
                trim_history(&mut rt.history);

                if let Some(content) = &response.content {
                    // Clear the "Thinking…" line.
                    println!("\r[GOAT] {}", content);
                    if let Some(ref brain) = rt.brain {
                        let _ = brain.log_interaction(&rt.session_id, "assistant", content);
                    }
                } else {
                    println!(); // clear the "Thinking…" line
                }

                match response.tool_calls {
                    None => break, // No more tool calls — done.
                    Some(tool_calls) => {
                        for tc in tool_calls {
                            println!("[AGENT] Using tool: {}", tc.function.name);

                            let args: Value = serde_json::from_str(&tc.function.arguments)
                                .unwrap_or(serde_json::json!({}));

                            let approval_req = build_approval_request(&tc.function.name, &args);

                            if let Some(req) = approval_req {
                                // Check session policy first.
                                match rt.approval_gate.check_policy(&req) {
                                    Some(ApprovalDecision::Approved) => {
                                        println!(
                                            "[APPROVAL] Auto-approved (session policy): {}",
                                            tc.function.name
                                        );
                                        let result =
                                            execute_native_tool(&tc.function.name, args).await;
                                        println!("[TOOL] {}", result);
                                        rt.history.push(Message {
                                            role: "tool".to_string(),
                                            content: Some(result),
                                            tool_calls: None,
                                            tool_call_id: Some(tc.id),
                                        });
                                        trim_history(&mut rt.history);
                                    }
                                    Some(ApprovalDecision::Denied(reason)) => {
                                        println!(
                                            "[APPROVAL] Auto-denied (session policy): {} — {}",
                                            tc.function.name, reason
                                        );
                                        rt.history.push(Message {
                                            role: "tool".to_string(),
                                            content: Some(format!(
                                                "Tool execution denied (session policy). Reason: {}",
                                                reason
                                            )),
                                            tool_calls: None,
                                            tool_call_id: Some(tc.id),
                                        });
                                        trim_history(&mut rt.history);
                                    }
                                    None => {
                                        // Interactive approval — block on stdin.
                                        let decision =
                                            prompt_approval_stdin(&req, &mut rt.approval_gate);

                                        match decision {
                                            ApprovalDecision::Approved => {
                                                println!(
                                                    "[APPROVAL] ✓ Approved: {}",
                                                    tc.function.name
                                                );
                                                let result =
                                                    execute_native_tool(&tc.function.name, args)
                                                        .await;
                                                println!("[TOOL] {}", result);
                                                rt.history.push(Message {
                                                    role: "tool".to_string(),
                                                    content: Some(result),
                                                    tool_calls: None,
                                                    tool_call_id: Some(tc.id),
                                                });
                                                trim_history(&mut rt.history);
                                            }
                                            ApprovalDecision::Denied(reason) => {
                                                println!(
                                                    "[APPROVAL] ✗ Denied: {} — {}",
                                                    tc.function.name, reason
                                                );
                                                rt.history.push(Message {
                                                    role: "tool".to_string(),
                                                    content: Some(format!(
                                                        "Tool execution denied. Reason: {}. \
                                                         Please adapt without this tool.",
                                                        reason
                                                    )),
                                                    tool_calls: None,
                                                    tool_call_id: Some(tc.id),
                                                });
                                                trim_history(&mut rt.history);
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Safe tool — no approval needed.
                                let tool_result = if let Some(native_result) =
                                    NativeTools::execute(&tc.function.name, args.clone()).await
                                {
                                    match native_result {
                                        Ok(res) => res,
                                        Err(e) => format!("Tool error: {}", e),
                                    }
                                } else {
                                    match rt.mcp_manager.call_tool(&tc.function.name, args).await {
                                        Ok(res) => serde_json::to_string(&res)
                                            .unwrap_or_else(|_| "[]".to_string()),
                                        Err(e) => format!("MCP tool error: {}", e),
                                    }
                                };

                                println!("[TOOL] {}", tool_result);
                                rt.history.push(Message {
                                    role: "tool".to_string(),
                                    content: Some(tool_result),
                                    tool_calls: None,
                                    tool_call_id: Some(tc.id),
                                });
                                trim_history(&mut rt.history);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("\n[ERROR] Request failed: {}", e);
                break;
            }
        }
    }

    println!(); // blank line after each turn
}

// ── Stdin approval prompt ─────────────────────────────────────────────────────

/// Print approval prompt to stdout and read y/n/a/d from stdin (blocking).
///
/// Re-prompts on invalid input. Defaults to Denied on EOF.
fn prompt_approval_stdin(
    req: &ApprovalRequest,
    gate: &mut crate::approval::ApprovalGate,
) -> ApprovalDecision {
    // Print the approval box.
    println!();
    for line in req.display_lines() {
        println!("{}", line);
    }
    println!();

    loop {
        print!("Approve? [y] yes  [n] no  [a] always allow  [d] always deny: ");
        io::stdout().flush().ok();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => {
                // EOF or error → deny (safe default).
                println!("[APPROVAL] EOF received — denying (safe default).");
                return ApprovalDecision::Denied("EOF on stdin".to_string());
            }
            Ok(_) => {}
        }

        let ch = input.trim().chars().next().unwrap_or('\0');
        match ch {
            'y' | 'n' | 'a' | 'd' => {
                return gate.resolve(req, ch);
            }
            _ => {
                println!("Invalid input. Please press y, n, a, or d.");
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_approval_request(tool_name: &str, args: &Value) -> Option<ApprovalRequest> {
    use crate::approval::{
        bash_approval_request, call_subagent_approval_request, write_file_approval_request,
    };
    match tool_name {
        "bash" => {
            let command = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
            Some(bash_approval_request(command))
        }
        "write_file" => {
            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
            Some(write_file_approval_request(path, content))
        }
        "call_subagent" => {
            let agent_cli = args.get("agent_cli").and_then(|v| v.as_str()).unwrap_or("");
            let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
            Some(call_subagent_approval_request(agent_cli, prompt))
        }
        _ => None,
    }
}

async fn execute_native_tool(name: &str, args: Value) -> String {
    match NativeTools::execute(name, args).await {
        Some(Ok(result)) => result,
        Some(Err(e)) => format!("Tool error: {}", e),
        None => format!("Unknown tool: {}", name),
    }
}

fn trim_history(history: &mut Vec<Message>) {
    let extra = history.len().saturating_sub(MAX_HISTORY_MESSAGES);
    if extra > 0 {
        history.drain(0..extra);
    }
}
