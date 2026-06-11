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
use crate::command_registry::{CommandRegistry, CommandStatus};
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
    let mut active_skill: Option<String> = None;

    loop {
        handle_scheduled_jobs(&mut rt).await;

        // Show the prompt.
        print!("> ");
        if let Some(ref skill) = active_skill {
            print!("[{}] ", skill);
        }
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

        let mut input = line.trim().to_string();
        if input.is_empty() {
            continue;
        }

        input = crate::quick_access::QuickAccessParser::parse_and_rewrite(&input);

        // Handle built-in headless commands (subset of slash commands).
        if input.starts_with('/') {
            if handle_slash_command(&input, &mut rt, &mut active_skill).await {
                continue;
            }
            println!(
                "[GOAT] Unknown command '{}'. Type /help for available commands.",
                input
            );
            continue;
        }

        // Run the agent loop for this prompt.
        run_agent_turn(&mut rt, input, active_skill.as_deref()).await;
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
    println!(
        "Slash commands: /help /status /repo-map /check /test /lint /format /patch /profile /profiles /clear /sessions /tools /new /exit"
    );
    println!();
}

// ── Slash command handling (headless subset) ──────────────────────────────────

/// Handle headless slash commands.  Returns `true` if the command was handled.
async fn handle_slash_command(
    cmd: &str,
    rt: &mut GoatRuntime,
    active_skill: &mut Option<String>,
) -> bool {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let name = parts[0].to_lowercase();

    match name.as_str() {
        "/help" => {
            let registry = CommandRegistry::build();
            let ver = env!("CARGO_PKG_VERSION");
            println!("[HELP] 🐐 GOAT v{} — Headless Command Reference", ver);
            println!("[HELP] ═════════════════════════════════════════════════════════");
            println!("[HELP] ✅ = working  ⚡ = partial  🔮 = planned (not yet implemented)");
            println!("[HELP] /commands all = show all including planned future commands");
            println!("[HELP] ─────────────────────────────────────────────────────────");
            for line in registry.format_help(false) {
                println!("[HELP]{}", line);
            }
            println!("[HELP] ─────────────────────────────────────────────────────────");
            println!("[HELP] Approval: y=approve  n=deny  a=always-allow  d=always-deny");
            println!("[HELP] Use /exit or Ctrl+D to quit.");
            true
        }

        "/commands" | "/cmd" => {
            let registry = CommandRegistry::build();
            let args = parts.get(1).copied().unwrap_or("").trim();
            match args {
                "all" => {
                    let ver = env!("CARGO_PKG_VERSION");
                    println!("[HELP] All GOAT Commands (incl. planned) — v{}", ver);
                    for line in registry.format_help(true) {
                        println!("[HELP]{}", line);
                    }
                }
                "planned" => {
                    println!("[HELP] 🔮 Planned Commands (not yet implemented):");
                    for cmd in registry
                        .all(true)
                        .iter()
                        .filter(|c| matches!(c.status, CommandStatus::Planned))
                    {
                        println!("[HELP]   🔮 {:<28} {}", cmd.usage, cmd.description);
                    }
                }
                q if q.starts_with("search ") => {
                    let query = q.trim_start_matches("search ").trim();
                    let results = registry.search(query, true);
                    println!("[HELP] {} result(s) for '{}':", results.len(), query);
                    for cmd in &results {
                        println!(
                            "[HELP]   {} {:<28} {}",
                            cmd.status.label(),
                            cmd.usage,
                            cmd.description
                        );
                    }
                    if results.is_empty() {
                        println!("[HELP]   No commands matching '{}'.", query);
                    }
                }
                _ => {
                    let ver = env!("CARGO_PKG_VERSION");
                    println!("[HELP] GOAT v{} Commands (working & partial):", ver);
                    for line in registry.format_help(false) {
                        println!("[HELP]{}", line);
                    }
                    println!("[HELP] ─────────────────────────────────────────────────────────");
                    println!("[HELP] /commands all        — show ALL including planned");
                    println!("[HELP] /commands planned    — show only planned commands");
                    println!("[HELP] /commands search <q> — search by name/description");
                }
            }
            true
        }

        "/version" | "/about" => {
            let ver = env!("CARGO_PKG_VERSION");
            println!("[ABOUT] GOAT v{}", ver);
            println!("[ABOUT] The Deep Research AI with TUI, Daemon, and React Dashboard.");
            println!("[ABOUT] Check https://github.com/hummcode/goat for updates.");
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

            let pid_path = rt.paths.data_dir.join("daemon.pid");
            if pid_path.exists() {
                println!("[STATUS] Daemon   : RUNNING (Scheduler active in daemon)");
                println!(
                    "[STATUS] API      : http://{}:{}",
                    rt.config.daemon.host, rt.config.daemon.port
                );
                println!(
                    "[WARN] Running local scheduler inside Headless while Daemon is active may duplicate jobs!"
                );
            } else {
                println!("[STATUS] Daemon   : STOPPED");
                println!("[STATUS] Scheduler: IN-PROCESS (Headless)");
            }

            // Project & Memory context
            let memory_manager =
                crate::memory::MemoryManager::new(&rt.paths, rt.config.memory.clone());
            let (u_count, u_max, _) = memory_manager.user_budget_status();
            let (m_count, m_max, _) = memory_manager.memory_budget_status();
            println!(
                "[STATUS] Memory   : Enabled={}, USER={}/{}, MEMORY={}/{}",
                rt.config.memory.enabled, u_count, u_max, m_count, m_max
            );

            if let Some(ref brain) = rt.brain {
                use std::env;
                let root = env::current_dir().unwrap_or_default();
                if let Ok(Some(meta)) = brain.get_project(root.to_string_lossy().as_ref()) {
                    println!("[STATUS] Project  : {}", meta.root_path.display());
                    if !meta.stack.is_empty() {
                        println!("[STATUS] Stack    : {}", meta.stack.join(", "));
                    }
                    // Git status
                    if meta.is_git_repo {
                        if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                            println!("[STATUS] Git      : {}", git.summary());
                        } else {
                            println!("[STATUS] Git      : repo (status unavailable)");
                        }
                    }
                    // Detected commands
                    let cmds = crate::repo_map::ProjectCommands::detect(&root);
                    println!(
                        "[STATUS] Dev cmds : check={}, test={}, lint={}",
                        cmds.check.as_deref().unwrap_or("none"),
                        cmds.test.as_deref().unwrap_or("none"),
                        cmds.lint.as_deref().unwrap_or("none"),
                    );
                } else {
                    println!("[STATUS] Project  : Not scanned (/project scan or /repo-map)");
                    // Still show git status for current dir
                    let root = env::current_dir().unwrap_or_default();
                    if root.join(".git").exists() {
                        if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                            println!("[STATUS] Git      : {}", git.summary());
                        }
                    }
                }
            }
            println!(
                "[STATUS] Subagents: {} available",
                rt.subagent_manager.registry.list_all().len()
            );

            let ext_count = rt
                .external_agent_manager
                .registry
                .adapters
                .values()
                .filter(|a| a.status == crate::external_agents::ExternalAgentStatus::Detected)
                .count();
            println!(
                "[STATUS] Ext Agents: {} detected (Enabled: {})",
                ext_count, rt.config.external_agents.enabled
            );

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

        cmd if cmd.starts_with("/tools") => {
            let subcommand = parts.get(1).copied().unwrap_or("list");
            match subcommand {
                "list" => {
                    let tools = rt.tool_registry.list_all();
                    println!("[TOOLS] GOAT Tool Registry ({} tools)", tools.len());
                    for t in &tools {
                        let perm = rt.tool_registry.get_permission(&t.name, &rt.config.tools);
                        println!("[TOOLS]   {:<15} [{:?}] - {}", t.name, perm, t.description);
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
                }
                "categories" => {
                    println!("[TOOLS] Categories: filesystem, shell, project, subagent...");
                }
                "doctor" => {
                    let tools = rt.tool_registry.list_all();
                    println!(
                        "[TOOLS] Registry Doctor: {} total native tools.",
                        tools.len()
                    );
                    println!("[TOOLS] Enabled: {}", rt.config.tools.enabled);
                }
                "audit" => {
                    if rt.paths.tool_audit_log_file.exists() {
                        if let Ok(content) = std::fs::read_to_string(&rt.paths.tool_audit_log_file)
                        {
                            println!("{}", content);
                        }
                    } else {
                        println!("[TOOLS] No audit log found.");
                    }
                }
                "catalog" | "catalog search" | "catalog show" => {
                    println!("[TOOLS] Tool Catalog (Phase 3.7 Foundation)");
                    println!("[TOOLS] Status: Informational only. No automatic installation yet.");
                    let parts: Vec<&str> = subcommand.splitn(3, ' ').collect();
                    if parts.len() > 1 {
                        let action = parts[1];
                        let arg = parts.get(2).unwrap_or(&"");
                        println!("[TOOLS] Catalog action '{}' on '{}'", action, arg);
                    } else {
                        println!("[TOOLS] Available Planned Categories:");
                        println!(
                            "[TOOLS] - filesystem MCP, git tools, browser automation, web search,"
                        );
                        println!("[TOOLS]   Playwright/browser-use, image generation, TTS/STT,");
                        println!(
                            "[TOOLS]   database tools, GitHub tools, calendar/email tools, local shell"
                        );
                    }
                }
                cmd if cmd.starts_with("install")
                    || cmd.starts_with("enable")
                    || cmd.starts_with("disable") =>
                {
                    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                    println!("[TOOLS] Action '{}' is planned for Phase 3.8.", parts[0]);
                    println!(
                        "[TOOLS] No automatic installation yet. Future installs require approval and sandbox checks."
                    );
                }
                name => {
                    if let Some(tool) = rt.tool_registry.get(name) {
                        println!("[TOOLS] Tool: {}", tool.name);
                        println!("[TOOLS] Category: {}", tool.category);
                        println!("[TOOLS] Risk: {}", tool.risk_level);
                        println!(
                            "[TOOLS] Effective Permission: {:?}",
                            rt.tool_registry
                                .get_permission(&tool.name, &rt.config.tools)
                        );
                    } else {
                        println!("[TOOLS] Tool '{}' not found.", name);
                    }
                }
            }
            true
        }

        cmd if cmd.starts_with("/tool ") => {
            let name = parts.get(1).copied().unwrap_or("").trim();
            if let Some(tool) = rt.tool_registry.get(name) {
                println!("[TOOLS] Tool: {}", tool.name);
                println!("[TOOLS] Category: {}", tool.category);
                println!("[TOOLS] Risk: {}", tool.risk_level);
                println!(
                    "[TOOLS] Effective Permission: {:?}",
                    rt.tool_registry
                        .get_permission(&tool.name, &rt.config.tools)
                );
            } else {
                println!("[TOOLS] Tool '{}' not found.", name);
            }
            true
        }

        cmd if cmd.starts_with("/mcp") => {
            let subcommand = parts.get(1).copied().unwrap_or("status");
            match subcommand {
                "status" => {
                    println!("[MCP] Status");
                    let enabled_count =
                        rt.config.mcp_servers.values().filter(|s| s.enabled).count();
                    println!("[MCP] Configured servers: {}", rt.config.mcp_servers.len());
                    println!("[MCP] Enabled servers: {}", enabled_count);
                    let running = rt.mcp_manager.running_servers();
                    println!("[MCP] Running servers: {}", running.len());
                }
                "list" => {
                    if rt.config.mcp_servers.is_empty() {
                        println!("[MCP] No MCP servers configured.");
                    } else {
                        println!("[MCP] Configured MCP Servers:");
                        for (name, srv) in &rt.config.mcp_servers {
                            let state = if let Some(mrs) = rt.mcp_runtime.get(name) {
                                mrs.state.to_string()
                            } else {
                                "Unknown".to_string()
                            };
                            println!(
                                "[MCP] - {} (Enabled: {}, Risk: {}, State: {})",
                                name, srv.enabled, srv.risk, state
                            );
                        }
                    }
                }
                "show" => {
                    let name = parts.get(2).copied().unwrap_or("");
                    if let Some(srv) = rt.config.mcp_servers.get(name) {
                        println!("[MCP] Server: {}", name);
                        println!("[MCP] Enabled: {}", srv.enabled);
                        println!("[MCP] Transport: {}", srv.transport);
                        println!("[MCP] Risk Policy: {}", srv.risk);
                        if let Some(mrs) = rt.mcp_runtime.get(name) {
                            println!("[MCP] State: {}", mrs.state);
                            if let Some(pid) = mrs.pid {
                                println!("[MCP] PID: {}", pid);
                            }
                            if let Some(start) = mrs.started_at {
                                println!("[MCP] Started At: {:?}", start);
                            }
                            if !mrs.discovered_tools.is_empty() {
                                println!("[MCP] Discovered Tools: {}", mrs.discovered_tools.len());
                            }
                        }
                        println!("[MCP] Command: {} {:?}", srv.command, srv.args);
                    } else {
                        println!("[MCP] Server '{}' not found.", name);
                    }
                }
                "start" => {
                    let name = parts.get(2).copied().unwrap_or("");
                    if let Some(srv_config) = rt.config.mcp_servers.get(name).cloned() {
                        if !srv_config.enabled {
                            println!(
                                "[MCP] Server '{}' is disabled in config. Refusing to start.",
                                name
                            );
                        } else {
                            use crate::approval::{ApprovalRequest, RiskLevel};
                            let req = ApprovalRequest {
                                tool_name: "mcp_start".to_string(),
                                action_summary: format!(
                                    "Start MCP server '{}': {} {:?}",
                                    name, srv_config.command, srv_config.args
                                ),
                                risk_level: RiskLevel::High,
                                explanation: None,
                                working_directory: None,
                            };
                            let decision = prompt_approval_stdin(&req, &mut rt.approval_gate);
                            if let crate::approval::ApprovalDecision::Approved = decision {
                                println!("[MCP] Starting server '{}'...", name);
                                let logs = rt.mcp_manager.start_server(name, &srv_config).await;
                                for log in &logs {
                                    println!("{}", log);
                                }
                                if let Some(mrs) = rt.mcp_runtime.get_mut(name) {
                                    mrs.state = crate::mcp_runtime::McpServerState::Running;
                                }
                                rt.sync_mcp_tools();
                                rt.tool_registry.log_execution(
                                    &rt.paths,
                                    &rt.session_id,
                                    "mcp_start",
                                    &crate::tool_registry::ToolAction::Allow,
                                    true,
                                    &logs.join("\n"),
                                );
                            } else {
                                println!("[MCP] Start request for '{}' denied.", name);
                            }
                        }
                    } else {
                        println!("[MCP] Server '{}' not found in config.", name);
                    }
                }
                "stop" => {
                    let name = parts.get(2).copied().unwrap_or("");
                    if rt.mcp_manager.running_servers().contains(&name.to_string()) {
                        println!("[MCP] Stopping server '{}'...", name);
                        let logs = rt.mcp_manager.stop_server(name).await;
                        for log in &logs {
                            println!("{}", log);
                        }
                        if let Some(mrs) = rt.mcp_runtime.get_mut(name) {
                            mrs.state = crate::mcp_runtime::McpServerState::Stopped;
                        }
                        rt.tool_registry.log_execution(
                            &rt.paths,
                            &rt.session_id,
                            "mcp_stop",
                            &crate::tool_registry::ToolAction::Allow,
                            true,
                            &logs.join("\n"),
                        );
                    } else {
                        println!("[MCP] Server '{}' is not running.", name);
                    }
                }
                "restart" => {
                    let name = parts.get(2).copied().unwrap_or("");
                    if let Some(srv_config) = rt.config.mcp_servers.get(name).cloned() {
                        use crate::approval::{ApprovalRequest, RiskLevel};
                        let req = ApprovalRequest {
                            tool_name: "mcp_restart".to_string(),
                            action_summary: format!("Restart MCP server '{}'", name),
                            risk_level: RiskLevel::High,
                            explanation: None,
                            working_directory: None,
                        };
                        let decision = prompt_approval_stdin(&req, &mut rt.approval_gate);
                        if let crate::approval::ApprovalDecision::Approved = decision {
                            println!("[MCP] Restarting server '{}'...", name);
                            let mut all_logs = Vec::new();
                            if rt.mcp_manager.running_servers().contains(&name.to_string()) {
                                let stop_logs = rt.mcp_manager.stop_server(name).await;
                                for log in &stop_logs {
                                    println!("{}", log);
                                }
                                all_logs.extend(stop_logs);
                            }
                            let start_logs = rt.mcp_manager.start_server(name, &srv_config).await;
                            for log in &start_logs {
                                println!("{}", log);
                            }
                            all_logs.extend(start_logs);
                            if let Some(mrs) = rt.mcp_runtime.get_mut(name) {
                                mrs.state = crate::mcp_runtime::McpServerState::Running;
                            }
                            rt.sync_mcp_tools();
                            rt.tool_registry.log_execution(
                                &rt.paths,
                                &rt.session_id,
                                "mcp_restart",
                                &crate::tool_registry::ToolAction::Allow,
                                true,
                                &all_logs.join("\n"),
                            );
                        } else {
                            println!("[MCP] Restart request for '{}' denied.", name);
                        }
                    } else {
                        println!("[MCP] Server '{}' not found in config.", name);
                    }
                }
                "tools" => {
                    let name = parts.get(2).copied().unwrap_or("");
                    // TODO tool listing for server
                    println!("[MCP] Server '{}' tools (placeholder).", name);
                }
                "call" => {
                    println!(
                        "[MCP] Tool call execution is partial; lifecycle and discovery are available."
                    );
                }
                "doctor" => {
                    println!(
                        "[MCP] Doctor: {} configured servers.",
                        rt.config.mcp_servers.len()
                    );
                }
                _ => println!(
                    "[MCP] Unknown command. Use /mcp status, list, show, start, stop, restart, doctor."
                ),
            }
            true
        }

        cmd if cmd.starts_with("/subagents") => {
            let subcommand = parts.get(1).copied().unwrap_or("list");
            match subcommand {
                "audit" => {
                    if rt.paths.subagent_audit_log_file.exists() {
                        if let Ok(content) =
                            std::fs::read_to_string(&rt.paths.subagent_audit_log_file)
                        {
                            println!("{}", content);
                        }
                    } else {
                        println!("[SUBAGENTS] No audit log found.");
                    }
                }
                _ => {
                    let list = rt.subagent_manager.registry.list_all();
                    println!(
                        "[SUBAGENTS] GOAT Subagent Registry ({} internal subagents)",
                        list.len()
                    );
                    for agent in list {
                        println!(
                            "[SUBAGENTS]   {:<15} [{}] - {}",
                            agent.name,
                            agent.kind.to_string(),
                            agent.purpose
                        );
                    }
                }
            }
            true
        }

        cmd if cmd.starts_with("/subagent ") => {
            let name = parts.get(1).copied().unwrap_or("").trim();
            if let Some(agent) = rt.subagent_manager.registry.get(name) {
                println!("[SUBAGENTS] Name: {}", agent.name);
                println!("[SUBAGENTS] Kind: {}", agent.kind);
                println!("[SUBAGENTS] Risk: {}", agent.risk_level);
                println!("[SUBAGENTS] Model Profile: {}", agent.default_model_profile);
                println!("[SUBAGENTS] Allowed Tools: {:?}", agent.allowed_tools);
                println!("[SUBAGENTS] Context Budget: {}", agent.context_budget);
            } else {
                println!("[SUBAGENTS] Subagent '{}' not found.", name);
            }
            true
        }

        cmd if cmd.starts_with("/ask-agent ") => {
            let args_str = parts.get(1).copied().unwrap_or("");
            let subparts: Vec<&str> = args_str.splitn(2, ' ').collect();
            if subparts.len() < 2 {
                println!("[SUBAGENTS] Usage: /ask-agent <name> <task>");
            } else {
                let name = subparts[0];
                let task = subparts[1];

                println!("[SUBAGENTS] Asking '{}'...", name);
                let summary = "Headless context summary... (limited repo map)";
                match rt
                    .subagent_manager
                    .ask_agent(
                        name,
                        task,
                        summary,
                        active_skill.clone(),
                        None,
                        &rt.llm_router,
                        &rt.model_chain,
                    )
                    .await
                {
                    Ok(res) => println!("[SUBAGENTS] Response:\n{}", res),
                    Err(e) => println!("[SUBAGENTS] Error: {}", e),
                }
            }
            true
        }

        cmd if cmd.starts_with("/delegate-external ") => {
            let args_str = parts.get(1).copied().unwrap_or("");
            let subparts: Vec<&str> = args_str.splitn(2, ' ').collect();
            if subparts.len() < 2 {
                println!("[EXTERNAL] Usage: /delegate-external <agent_name> <task>");
            } else {
                let name = subparts[0];
                let task = subparts[1];

                let action = rt
                    .tool_registry
                    .evaluate_action("delegate_external_agent", &rt.config.tools);
                if let crate::tool_registry::ToolAction::Deny(reason) = action {
                    println!("[EXTERNAL] Delegation denied by tool registry: {}", reason);
                    return true;
                }

                let req = crate::approval::ApprovalRequest {
                    tool_name: "delegate_external_agent".to_string(),
                    action_summary: format!("agent: {}, task: {}", name, task),
                    risk_level: crate::approval::RiskLevel::High,
                    explanation: None,
                    working_directory: None,
                };

                if let Some(crate::approval::ApprovalDecision::Denied(msg)) =
                    rt.approval_gate.check_policy(&req)
                {
                    println!("[EXTERNAL] Delegation denied via policy: {}", msg);
                    return true;
                }

                println!("[EXTERNAL] Delegating to '{}'...", name);
                match rt.external_agent_manager.delegate(name, task, &rt.config) {
                    Ok(res) => {
                        println!("[EXTERNAL] Done. Success: {}", res.success);
                        println!("[EXTERNAL] STDOUT:\n{}", res.stdout);
                        if !res.stderr.is_empty() {
                            println!("[EXTERNAL] STDERR:\n{}", res.stderr);
                        }
                    }
                    Err(e) => {
                        println!("[EXTERNAL] Execution failed: {}", e);
                        println!(
                            "[EXTERNAL] (To enable, check your config: allow_execution = true, workspace_mode = \"isolated-copy\")"
                        );
                    }
                }
            }
            true
        }

        cmd if cmd.starts_with("/compare-agents ") => {
            let task = parts.get(1).copied().unwrap_or("");
            println!("[COMPARE] Comparing internal vs external agent approaches...");
            println!("[COMPARE] Internal agent (coder): working...");
            let summary = "Headless context summary... (limited repo map)";
            match rt
                .subagent_manager
                .ask_agent(
                    "coder",
                    task,
                    summary,
                    active_skill.clone(),
                    None,
                    &rt.llm_router,
                    &rt.model_chain,
                )
                .await
            {
                Ok(res) => println!("[COMPARE] Internal Response:\n{}", res),
                Err(e) => println!("[COMPARE] Internal Error: {}", e),
            }
            println!("[COMPARE] Checking external agent (aider)...");
            if rt.config.external_agents.allow_execution {
                match rt
                    .external_agent_manager
                    .delegate("aider", task, &rt.config)
                {
                    Ok(res) => println!("[COMPARE] External Response (aider):\n{}", res.stdout),
                    Err(e) => println!(
                        "[COMPARE] External agent execution disabled or failed: {}",
                        e
                    ),
                }
            } else {
                println!(
                    "[COMPARE] External agent execution is disabled in config. Cannot compare."
                );
            }
            true
        }

        cmd if cmd.starts_with("/builder")
            || cmd.starts_with("@builder")
            || cmd.starts_with("@code")
            || cmd.starts_with("@plan")
            || cmd.starts_with("@review")
            || cmd.starts_with("@diff")
            || cmd.starts_with("@tests")
            || cmd.starts_with("@patch") =>
        {
            let action = parts.get(1).copied().unwrap_or("inspect");
            let agent = match crate::agents::builder::BuilderAgent::new() {
                Ok(a) => a,
                Err(e) => {
                    println!("[BUILDER] Failed to create agent: {}", e);
                    return true;
                }
            };
            let handle = tokio::runtime::Handle::current();

            match action {
                "inspect" => {
                    println!("[BUILDER] Inspecting repository...");
                    match agent.inspect_repo(crate::agents::builder::BuilderInspectionScope {
                        max_depth: 3,
                        include_tests: true,
                    }) {
                        Ok(res) => {
                            println!("Root: {}", res.snapshot.root_path);
                            println!("Main Language: {}", res.snapshot.tech_stack.main_language);
                            println!("Files: {}", res.snapshot.file_count);
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                "plan" => {
                    let goal = parts.get(2..).unwrap_or(&[""]).join(" ");
                    if goal.is_empty() {
                        println!("[BUILDER] Goal missing.");
                    } else {
                        let brain_mgr = crate::brain_index::BrainIndexManager::new(
                            rt.paths.clone(),
                            rt.config.brain_index.clone(),
                            &rt.config.embeddings,
                        );
                        match handle.block_on(agent.plan_patch(&goal, &brain_mgr)) {
                            Ok(plan) => {
                                println!("Plan ID: {}", plan.id);
                                println!("Goal: {}", plan.goal);
                                println!("Risk: {}", plan.risk_level);
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }
                "diff-review" => {
                    println!("[BUILDER] Running diff review...");
                    match agent.diff_review("active_plan") {
                        Ok(rev) => {
                            println!("Severity: {:?}", rev.overall_severity);
                            for f in rev.findings {
                                println!("- [{}]: {}", f.file_path, f.issue_description);
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                "test-plan" => {
                    let goal = parts.get(2..).unwrap_or(&[""]).join(" ");
                    match agent.test_plan(&goal) {
                        Ok(p) => {
                            println!("Validation plan generated.");
                            for c in p.commands {
                                println!("Command: {}", c.command);
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                "validate" => match agent.validate("active_plan") {
                    Ok(res) => {
                        println!("Validation Finished. Valid: {}", res.is_valid);
                        println!("Logs:\n{}", res.test_logs);
                    }
                    Err(e) => println!("Error: {}", e),
                },
                "rollback-plan" => match agent.rollback_plan("active_plan") {
                    Ok(p) => {
                        println!("Fallback Command: {}", p.command_fallback);
                    }
                    Err(e) => println!("Error: {}", e),
                },
                _ => println!(
                    "Unknown builder action: {}. Use inspect, plan, diff-review, test-plan, validate, rollback-plan",
                    action
                ),
            }
            true
        }

        cmd if cmd.starts_with("/transports") || cmd.starts_with("/transport") => {
            let action = parts.get(1).copied().unwrap_or("status");
            match action {
                "status" | "doctor" => {
                    println!("[TRANSPORTS] Checking status...");
                    let handle = tokio::runtime::Handle::current();
                    if let Ok(res) = handle.block_on(rt.transport_manager.check_doctor()) {
                        println!("{}", res);
                    } else {
                        println!("[TRANSPORTS] Error checking status");
                    }
                }
                "sessions" => {
                    let sessions = rt.transport_manager.list_sessions();
                    println!("[TRANSPORTS] Active Sessions ({}):", sessions.len());
                    for s in sessions {
                        println!("  - {} [{:?}]", s.id, s.provider);
                    }
                }
                "messages" => {
                    let messages = rt.transport_manager.get_messages();
                    println!("[TRANSPORTS] Messages ({}):", messages.len());
                    for m in messages.iter().take(10) {
                        println!("  [{:?}] {}: {}", m.direction, m.session_id, m.content);
                    }
                }
                "send" => {
                    if let (Some(sid), Some(msg)) = (parts.get(2), parts.get(3)) {
                        println!("[TRANSPORTS] Sending to {}: {}", sid, msg);
                        let handle = tokio::runtime::Handle::current();
                        if let Err(e) =
                            handle.block_on(rt.transport_manager.send_outbound(sid, msg))
                        {
                            println!("[TRANSPORTS] Failed: {}", e);
                        }
                    } else {
                        println!("[TRANSPORTS] Usage: /transports send <session_id> <message>");
                    }
                }
                _ => println!("[TRANSPORTS] Unknown action: {}", action),
            }
            true
        }
        cmd if cmd.starts_with("/telegram") => {
            println!(
                "[TELEGRAM] Telegram transport is partially implemented (planned for Phase 5.14)."
            );
            true
        }
        cmd if cmd.starts_with("/discord") => {
            println!(
                "[DISCORD] Discord transport is partially implemented (planned for Phase 5.14)."
            );
            true
        }
        cmd if cmd.starts_with("/voice")
            || cmd.starts_with("/talk")
            || cmd.starts_with("/speak") =>
        {
            let is_shortcut = cmd.starts_with("/talk") || cmd.starts_with("/speak");
            let action = if is_shortcut {
                "speak"
            } else {
                parts.get(1).copied().unwrap_or("status")
            };
            let rest_idx = if is_shortcut { 1 } else { 2 };
            let handle = tokio::runtime::Handle::current();

            match action {
                "status" | "doctor" => {
                    println!("[VOICE] Checking status...");
                    if let Ok(res) = handle.block_on(rt.voice_manager.check_doctor()) {
                        println!("{}", res);
                    } else {
                        println!("[VOICE] Error checking status");
                    }
                }
                "providers" => {
                    println!("[VOICE] Available Providers:");
                    for p in rt.voice_manager.get_providers() {
                        println!("  - {}", p);
                    }
                }
                "transcript" => {
                    let text = parts[rest_idx..].join(" ");
                    println!("[VOICE] Simulating transcript: '{}'", text);
                    let input = crate::voice::VoiceInput {
                        audio_base64: None,
                        text_override: Some(text),
                    };
                    if let Ok(res) = handle.block_on(rt.voice_manager.transcribe(&input)) {
                        println!("[VOICE] Result: {} (conf: {})", res.text, res.confidence);
                    } else {
                        println!("[VOICE] Failed to transcribe");
                    }
                }
                "speak" | "talk" => {
                    let text = parts[rest_idx..].join(" ");
                    println!("[VOICE] Generating speech for: '{}'", text);
                    if let Ok(res) = handle.block_on(rt.voice_manager.speak(&text)) {
                        println!("[VOICE] TTS Success: {}", res.text);
                    } else {
                        println!("[VOICE] Failed to generate TTS");
                    }
                }
                "privacy" => {
                    println!(
                        "[VOICE] Privacy Policy: Voice recordings and transcripts remain entirely local by default."
                    );
                    println!(
                        "[VOICE] Cloud STT/TTS requires explicit opt-in via config file. No wake word or background listening is active."
                    );
                }
                _ => println!("[VOICE] Unknown action: {}", action),
            }
            true
        }

        cmd if cmd.starts_with("/mode") || cmd.starts_with("/profile mode") => {
            let subcmd = parts.get(1).copied().unwrap_or("list");
            match subcmd {
                "list" => {
                    println!("[MODES] Built-in modes:");
                    for m in crate::agent_profiles::AgentModeProfile::get_builtins() {
                        println!(" - {} ({:?})", m.name, m.kind);
                    }
                }
                "use" => {
                    if let Some(m) = parts.get(2) {
                        println!("[MODES] Switching to mode: {}", m);
                    }
                }
                "current" => {
                    println!("[MODES] Current mode: {}", rt.config.profiles.default_mode);
                }
                "recommend" => {
                    println!("[MODES] Recommended: Coding Assistant");
                }
                _ => println!("[MODES] Unknown mode subcommand"),
            }
            true
        }
        cmd if cmd.starts_with("/project") => {
            let subcmd = parts.get(1).copied().unwrap_or("show");
            match subcmd {
                "detect" => {
                    let detected = crate::project_profiles::ProjectProfileDetector::detect(".");
                    println!("[PROJECT] Detected project: {:?}", detected.kind);
                }
                "show" => println!("[PROJECT] Showing project profile."),
                "save" => println!("[PROJECT] Saved project profile."),
                "setup" | "checklist" => {
                    println!("[PROJECT] Setup checklist: Github, MCP, Indexes.")
                }
                _ => println!("[PROJECT] Unknown subcommand"),
            }
            true
        }
        cmd if cmd.starts_with("/onboard")
            || cmd.starts_with("/setup")
            || cmd.starts_with("/welcome")
            || cmd.starts_with("/checklist") =>
        {
            println!("[ONBOARDING] Starting setup wizard...");
            println!("(Interactive onboarding is available via Dashboard or TUI.)");
            true
        }
        cmd if cmd.starts_with("/external-agents") => {
            let subcmd = parts.get(1).copied().unwrap_or("list");
            match subcmd {
                "detect" => {
                    println!("[EXTERNAL] Detecting...");
                    rt.external_agent_manager.detect_all(&rt.config);
                    for a in rt.external_agent_manager.registry.list_all() {
                        println!("  {} - {}", a.name, a.status);
                    }
                }
                "runs" => {
                    let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
                    if jsonl_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                            for line in content.lines() {
                                if let Ok(run) = serde_json::from_str::<
                                    crate::external_agents::ExternalAgentRun,
                                >(line)
                                {
                                    println!("{} | {} | {}", run.id, run.agent_name, run.mode);
                                }
                            }
                        }
                    } else {
                        println!("[EXTERNAL] No runs.");
                    }
                }
                _ => {
                    for a in rt.external_agent_manager.registry.list_all() {
                        println!("  {} [{}] - {}", a.name, a.command_name, a.status);
                    }
                }
            }
            true
        }

        cmd if cmd == "/external-runs" => {
            let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
            if jsonl_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                    for line in content.lines() {
                        if let Ok(run) =
                            serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line)
                        {
                            println!("{} | {} | {}", run.id, run.agent_name, run.mode);
                        }
                    }
                }
            } else {
                println!("[EXTERNAL] No runs.");
            }
            true
        }

        cmd if cmd.starts_with("/external-run ") => {
            let run_id = parts.get(1).copied().unwrap_or("").trim();
            let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
            if jsonl_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                    for line in content.lines() {
                        if let Ok(run) =
                            serde_json::from_str::<crate::external_agents::ExternalAgentRun>(line)
                        {
                            if run.id == run_id {
                                println!(
                                    "Run ID: {}\nAgent: {}\nWorkspace: {}\nTask: {}",
                                    run.id,
                                    run.agent_name,
                                    run.workspace_path.display(),
                                    run.task
                                );
                            }
                        }
                    }
                }
            }
            true
        }

        "/review" => {
            println!("[SUBAGENTS] Asking 'reviewer' to review current context...");
            let task = "Review the current plan/patch.";
            let summary = "Headless context summary... (limited repo map)";
            match rt
                .subagent_manager
                .ask_agent(
                    "reviewer",
                    task,
                    summary,
                    active_skill.clone(),
                    None,
                    &rt.llm_router,
                    &rt.model_chain,
                )
                .await
            {
                Ok(res) => println!("[SUBAGENTS] Response:\n{}", res),
                Err(e) => println!("[SUBAGENTS] Error: {}", e),
            }
            true
        }

        "/debug" => {
            println!("[SUBAGENTS] Asking 'debugger' to analyze...");
            let task = "Analyze recent errors or bugs.";
            let summary = "Headless context summary... (limited repo map)";
            match rt
                .subagent_manager
                .ask_agent(
                    "debugger",
                    task,
                    summary,
                    active_skill.clone(),
                    None,
                    &rt.llm_router,
                    &rt.model_chain,
                )
                .await
            {
                Ok(res) => println!("[SUBAGENTS] Response:\n{}", res),
                Err(e) => println!("[SUBAGENTS] Error: {}", e),
            }
            true
        }

        "/test-plan" => {
            println!("[SUBAGENTS] Asking 'tester' for verification strategy...");
            let task = "Suggest a verification strategy or test plan.";
            let summary = "Headless context summary... (limited repo map)";
            match rt
                .subagent_manager
                .ask_agent(
                    "tester",
                    task,
                    summary,
                    active_skill.clone(),
                    None,
                    &rt.llm_router,
                    &rt.model_chain,
                )
                .await
            {
                Ok(res) => println!("[SUBAGENTS] Response:\n{}", res),
                Err(e) => println!("[SUBAGENTS] Error: {}", e),
            }
            true
        }

        "/exit" => {
            println!("[GOAT] Goodbye!");
            std::process::exit(0);
        }

        cmd if cmd.starts_with("/project") => {
            let arg = parts.get(1).copied().unwrap_or("").trim();
            let root = std::env::current_dir().unwrap_or_default();
            if let Some(ref brain) = rt.brain {
                if arg == "scan" {
                    println!("[PROJECT] Scanning {}...", root.display());
                    let scanner = crate::project::ProjectScanner::new(root.clone());
                    match scanner.scan() {
                        Ok(meta) => {
                            let _ = brain.save_project(root.to_string_lossy().as_ref(), &meta);
                            println!("[PROJECT] Scan complete.");
                            println!("[PROJECT] Stack: {}", meta.stack.join(", "));
                            println!("[PROJECT] Ignored dirs: {}", meta.ignored_dirs_count);
                        }
                        Err(e) => {
                            println!("[PROJECT] Scan failed: {}", e);
                        }
                    }
                } else {
                    match brain.get_project(root.to_string_lossy().as_ref()) {
                        Ok(Some(meta)) => {
                            println!("[PROJECT] Root: {}", meta.root_path.display());
                            println!(
                                "[PROJECT] Git: {}",
                                if meta.is_git_repo { "Yes" } else { "No" }
                            );
                            if !meta.stack.is_empty() {
                                println!("[PROJECT] Stack: {}", meta.stack.join(", "));
                            }
                            if !meta.detected_commands.is_empty() {
                                println!(
                                    "[PROJECT] Commands: {}",
                                    meta.detected_commands.join(", ")
                                );
                            }
                        }
                        _ => {
                            println!("[PROJECT] No project context. Run /project scan.");
                        }
                    }
                }
            } else {
                println!("[PROJECT] Brain disabled. Cannot store project context.");
            }
            true
        }

        "/memory" => {
            let memory_manager =
                crate::memory::MemoryManager::new(&rt.paths, rt.config.memory.clone());
            let subcommand = parts.get(1).copied().unwrap_or("status");
            match subcommand {
                "status" => {
                    let (u_count, u_max, u_warn) = memory_manager.user_budget_status();
                    let (m_count, m_max, m_warn) = memory_manager.memory_budget_status();
                    println!(
                        "[MEMORY] USER.md   : {}/{} chars {}",
                        u_count,
                        u_max,
                        if u_warn { "(OVER BUDGET)" } else { "" }
                    );
                    println!(
                        "[MEMORY] MEMORY.md : {}/{} chars {}",
                        m_count,
                        m_max,
                        if m_warn { "(OVER BUDGET)" } else { "" }
                    );
                    println!("[MEMORY] Enabled   : {}", rt.config.memory.enabled);
                }
                "show" => {
                    println!("--- USER.md ---");
                    println!("{}", memory_manager.get_user_content().unwrap_or_default());
                    println!("--- MEMORY.md ---");
                    println!(
                        "{}",
                        memory_manager.get_memory_content().unwrap_or_default()
                    );
                }
                "path" => {
                    println!("USER.md:   {}", memory_manager.user_file.display());
                    println!("MEMORY.md: {}", memory_manager.memory_file.display());
                }
                "add-user" => {
                    let text = parts[2..].join(" ");
                    if text.is_empty() {
                        println!("[MEMORY] Please provide text: /memory add-user <text>");
                    } else if let Err(e) = memory_manager.add_user(&text) {
                        println!("[MEMORY] Error: {}", e);
                    } else {
                        println!("[MEMORY] Added to USER.md");
                    }
                }
                "add-note" => {
                    let text = parts[2..].join(" ");
                    if text.is_empty() {
                        println!("[MEMORY] Please provide text: /memory add-note <text>");
                    } else if let Err(e) = memory_manager.add_note(&text) {
                        println!("[MEMORY] Error: {}", e);
                    } else {
                        println!("[MEMORY] Added to MEMORY.md");
                    }
                }
                _ => {
                    println!(
                        "[MEMORY] Unknown action: {}. Use status, show, path, add-user, add-note.",
                        subcommand
                    );
                }
            }
            true
        }

        "/skills" => {
            let skill_manager =
                crate::skills::SkillManager::new(rt.paths.clone(), rt.config.skills.clone());
            let skills = skill_manager.list_skills();
            if skills.is_empty() {
                println!("[SKILLS] No skills found. Use /skill create <name> to make one.");
            } else {
                println!("[SKILLS] {} available skills:", skills.len());
                for s in skills {
                    let status = if s.is_suspicious { " [SUSPICIOUS]" } else { "" };
                    println!("[SKILLS]   - {}{}: {}", s.name, status, s.description);
                }
                println!("[SKILLS] Use /skill <name> to activate a skill for this session.");
            }
            true
        }

        "/skill" => {
            let arg = parts.get(1).copied().unwrap_or("").trim();
            let rest = parts.get(2..).unwrap_or(&[]).join(" ");
            let skill_manager =
                crate::skills::SkillManager::new(rt.paths.clone(), rt.config.skills.clone());

            if arg.is_empty() {
                println!("[SKILLS] Active skill:");
                if let Some(skill) = active_skill {
                    println!("[SKILLS]   {}", skill);
                    println!("[SKILLS] Use /skill clear to deactivate.");
                } else {
                    println!("[SKILLS]   None");
                }
            } else if arg == "clear" {
                *active_skill = None;
                println!("[SKILLS] Active skill cleared.");
            } else if arg == "path" {
                println!(
                    "[SKILLS] Directory: {}",
                    skill_manager.skills_dir().display()
                );
            } else if arg == "create" {
                if rest.is_empty() {
                    println!("[SKILLS] Usage: /skill create <name>");
                } else {
                    match skill_manager.create_template(&rest) {
                        Ok(path) => println!("[SKILLS] Created template at {}", path.display()),
                        Err(e) => println!("[SKILLS] Error creating template: {}", e),
                    }
                }
            } else if arg == "search" {
                if rest.is_empty() {
                    println!("[SKILLS] Usage: /skill search <query>");
                } else {
                    let results = skill_manager.search_skills(&rest);
                    if results.is_empty() {
                        println!("[SKILLS] No skills match '{}'", rest);
                    } else {
                        println!("[SKILLS] {} matches:", results.len());
                        for s in results {
                            println!("[SKILLS]   - {}: {}", s.name, s.description);
                        }
                    }
                }
            } else {
                if let Some(skill) = skill_manager.get_skill(arg) {
                    *active_skill = Some(skill.name.clone());
                    println!("[SKILLS] Activated skill: {}", skill.name);
                    if skill.is_suspicious {
                        println!("[SKILLS] WARNING: This skill contains suspicious patterns!");
                    }
                } else {
                    println!("[SKILLS] Skill '{}' not found.", arg);
                }
            }
            true
        }

        "/save-skill" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg.is_empty() {
                println!("[SKILLS] Usage: /save-skill <name>");
            } else {
                let mut history_text = String::new();
                for msg in rt.history.iter().filter(|m| m.role != "system") {
                    history_text.push_str(&format!(
                        "{}: {}\n",
                        msg.role,
                        msg.content.as_deref().unwrap_or("")
                    ));
                }

                if history_text.trim().is_empty() {
                    println!("[SKILLS] No history to extract from.");
                    return true;
                }

                println!(
                    "[SKILLS] Extracting skill '{}' from session history...",
                    arg
                );

                let prompt = format!(
                    "You are a skill curator. The user wants to extract a reusable skill from the following session history.\n\
                     Generate a valid SKILL.md file with the following headers: Name, Description, Triggers, Tools Needed, Procedure, Safety Notes, Verification.\n\
                     The skill name should be: {}\n\n\
                     Rules:\n\
                     - NEVER include real API keys, passwords, or secrets.\n\
                     - Focus on the generalized workflow, not the exact files edited.\n\
                     - Output only the Markdown content.\n\n\
                     Session History:\n{}",
                    arg, history_text
                );

                let messages = vec![crate::llm::Message {
                    role: "user".to_string(),
                    content: Some(prompt),
                    tool_calls: None,
                    tool_call_id: None,
                }];

                match rt
                    .llm_router
                    .completion_with_fallback(&rt.model_chain, messages, None)
                    .await
                {
                    Ok((resp, _)) => {
                        let content = resp.content.unwrap_or_default();
                        let skill_manager = crate::skills::SkillManager::new(
                            rt.paths.clone(),
                            rt.config.skills.clone(),
                        );
                        let skill_dir = skill_manager.skills_dir().join(&arg);
                        let _ = std::fs::create_dir_all(&skill_dir);
                        let skill_file = skill_dir.join("SKILL.md");
                        if let Err(e) = std::fs::write(&skill_file, content) {
                            println!("[SKILLS] Error writing skill file: {}", e);
                        } else {
                            println!(
                                "[SKILLS] Extracted and saved skill '{}' to {}",
                                arg,
                                skill_file.display()
                            );
                        }
                    }
                    Err(e) => {
                        println!("[SKILLS] Failed to extract skill from LLM: {}", e);
                    }
                }
            }
            true
        }
        "/hooks" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg.is_empty() || arg == "list" {
                let info = rt.hooks_manager.list_hooks_info();
                println!("[HOOKS] Registered Hooks:");
                if info.is_empty() {
                    println!("[HOOKS] No hooks configured.");
                } else {
                    for i in info {
                        println!("[HOOKS]   - {}", i);
                    }
                }
            } else {
                println!("[HOOKS] Advanced hooks management requires config edits for now.");
            }
            true
        }

        "/schedule" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg.is_empty() || arg == "list" {
                let jobs = rt.scheduler_manager.list_jobs();
                println!("[SCHEDULE] {} Scheduled Jobs:", jobs.len());
                for j in jobs {
                    println!(
                        "[SCHEDULE]   [{}] {} (enabled: {})",
                        j.id, j.prompt_or_command, j.enabled
                    );
                }
            } else {
                println!(
                    "[SCHEDULE] Adding jobs via Headless is partial. Use manual config for now."
                );
            }
            true
        }

        "/jobs" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg.is_empty() || arg == "list" {
                let statuses = rt.job_tracker.list_jobs();
                println!("[JOBS] {} Active/Recent Jobs:", statuses.len());
                if statuses.is_empty() {
                    println!("[JOBS] No background jobs tracked.");
                } else {
                    for s in statuses {
                        println!("[JOBS]   [{}] {} - {:?}", s.id, s.r#type, s.status);
                    }
                }
            } else {
                println!("[JOBS] Unknown action '{}'", arg);
            }
            true
        }

        "/daemon" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg == "status" || arg.is_empty() {
                let pid_path = rt.paths.data_dir.join("daemon.pid");
                if pid_path.exists() {
                    println!(
                        "[DAEMON] Running (PID: {})",
                        std::fs::read_to_string(&pid_path)
                            .unwrap_or_default()
                            .trim()
                    );
                } else {
                    println!("[DAEMON] Stopped");
                }
            } else if arg == "doctor" {
                println!(
                    "[DAEMON DOCTOR] Enabled in config: {}",
                    rt.config.daemon.enabled
                );
                println!(
                    "[DAEMON DOCTOR] Bind Address: {}:{}",
                    rt.config.daemon.host, rt.config.daemon.port
                );
                println!(
                    "[DAEMON DOCTOR] Auth Required: {}",
                    rt.config.daemon.auth_required
                );
            } else {
                println!("[DAEMON] Unknown action '{}'. Use status, doctor.", arg);
            }
            true
        }

        "/api" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg == "status" || arg.is_empty() {
                println!(
                    "[API] Configured at http://{}:{}",
                    rt.config.daemon.host, rt.config.daemon.port
                );
                if rt.config.daemon.auth_required {
                    println!(
                        "[API] Auth Required: true (Use Bearer token from ~/.local/share/goat/daemon.token)"
                    );
                } else {
                    println!("[API] Auth Required: false (WARNING: Unauthenticated)");
                }
            } else {
                println!("[API] Unknown action '{}'. Use status.", arg);
            }
            true
        }

        "/dashboard" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            let root = std::env::current_dir().unwrap_or_default();
            let dashboard_dir = root.join("apps").join("dashboard");
            let fallback_dir = root.join("dashboard");

            let active_dir = if dashboard_dir.exists() {
                Some(dashboard_dir)
            } else if fallback_dir.exists() {
                Some(fallback_dir)
            } else {
                None
            };

            if let Some(dir) = active_dir {
                if arg == "path" || arg.is_empty() {
                    println!("[DASHBOARD] Located at: {}", dir.display());
                } else if arg == "doctor" {
                    println!("[DASHBOARD DOCTOR] Path: {}", dir.display());
                    let pkg_json = dir.join("package.json");
                    println!(
                        "[DASHBOARD DOCTOR] package.json: {}",
                        if pkg_json.exists() {
                            "Found"
                        } else {
                            "Missing"
                        }
                    );
                } else if arg == "chat"
                    || arg == "repo"
                    || arg == "diffs"
                    || arg == "commands"
                    || arg == "audit"
                    || arg == "approvals"
                {
                    println!(
                        "[DASHBOARD] Open http://127.0.0.1:3000/{} in your browser to view.",
                        arg
                    );
                } else {
                    println!(
                        "[DASHBOARD] Unknown action '{}'. Use path, doctor, chat, repo, diffs, commands, audit, approvals.",
                        arg
                    );
                }
            } else {
                println!(
                    "[DASHBOARD] Not found. Run `goat dashboard dev` outside TUI to bootstrap or locate it."
                );
            }
            true
        }

        "/desktop" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            let root = std::env::current_dir().unwrap_or_default();
            let desktop_dir = root.join("apps").join("desktop");

            if desktop_dir.exists() {
                if arg == "path" || arg.is_empty() {
                    println!("[DESKTOP] Located at: {}", desktop_dir.display());
                } else if arg == "doctor" {
                    println!("[DESKTOP DOCTOR] Path: {}", desktop_dir.display());
                    let pkg_json = desktop_dir.join("package.json");
                    println!(
                        "[DESKTOP DOCTOR] package.json: {}",
                        if pkg_json.exists() {
                            "Found"
                        } else {
                            "Missing"
                        }
                    );
                    let tauri_conf = desktop_dir.join("src-tauri").join("tauri.conf.json");
                    println!(
                        "[DESKTOP DOCTOR] tauri.conf.json: {}",
                        if tauri_conf.exists() {
                            "Found"
                        } else {
                            "Missing"
                        }
                    );
                } else {
                    println!("[DESKTOP] Unknown action '{}'. Use path or doctor.", arg);
                }
            } else {
                println!("[DESKTOP] Not found. Run `goat desktop` outside TUI to view info.");
            }
            true
        }

        "/audit" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg == "recent" || arg.is_empty() {
                println!("[AUDIT] Fetching recent audit logs...");
                if let Ok(content) = std::fs::read_to_string(&rt.paths.tool_audit_log_file) {
                    let lines: Vec<&str> = content.lines().rev().take(10).collect();
                    for line in lines.into_iter().rev() {
                        println!("  {}", line);
                    }
                } else {
                    println!("[AUDIT] No tool audit logs found.");
                }
            } else {
                println!("[AUDIT] Unknown action. Try: /audit recent");
            }
            true
        }

        "/approvals" => {
            let arg = parts.get(1..).unwrap_or(&[]).join(" ");
            if arg == "history" {
                println!(
                    "[APPROVALS] Run 'goat dashboard' to view complete history in the browser."
                );
            } else {
                println!("[APPROVALS] Unknown action. Try: /approvals history");
            }
            true
        }
        "/recall" => {
            let query = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
            if query.is_empty() {
                println!("[RECALL] Please provide a query: /recall <text>");
                return true;
            }
            if let Some(ref brain) = rt.brain {
                match brain.recall_search(&query) {
                    Ok(results) if results.is_empty() => println!("[RECALL] No results found."),
                    Ok(results) => {
                        println!("[RECALL] Found {} result(s):", results.len());
                        for (idx, (session_id, role, content)) in results.iter().enumerate() {
                            let snippet = if content.len() > 80 {
                                format!("{}...", &content[..77].replace('\n', " "))
                            } else {
                                content.replace('\n', " ")
                            };
                            println!(
                                "  {}. [{}] {}: {}",
                                idx + 1,
                                &session_id[..8],
                                role,
                                snippet
                            );
                        }
                    }
                    Err(e) => println!("[RECALL] Error searching brain: {}", e),
                }
            } else {
                println!("[RECALL] Brain is disabled (--no-brain).");
            }
            true
        }

        // ── Context files ──────────────────────────────────────────────────────
        cmd if cmd.starts_with("/context") => {
            let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
            let action = args.get(1).copied().unwrap_or("show");
            match action {
                "add" => {
                    if let Some(path) = args.get(2) {
                        let root = std::env::current_dir().unwrap_or_default();
                        let full_path = root.join(path);
                        if full_path.exists() && full_path.is_file() {
                            if crate::repo_map::looks_like_secret_file(&full_path) {
                                println!("[CONTEXT] Rejected: {} looks like a secret file.", path);
                            } else {
                                if !rt.selected_files.contains(&path.to_string()) {
                                    rt.selected_files.push(path.to_string());
                                    println!("[CONTEXT] Added {}", path);
                                } else {
                                    println!("[CONTEXT] {} is already in context.", path);
                                }
                            }
                        } else {
                            println!("[CONTEXT] File not found: {}", path);
                        }
                    } else {
                        println!("[CONTEXT] Usage: /context add <path>");
                    }
                }
                "remove" => {
                    if let Some(path) = args.get(2) {
                        rt.selected_files.retain(|p| p != path);
                        println!("[CONTEXT] Removed {}", path);
                    } else {
                        println!("[CONTEXT] Usage: /context remove <path>");
                    }
                }
                "clear" => {
                    rt.selected_files.clear();
                    println!("[CONTEXT] Cleared all selected files.");
                }
                "show" => {
                    println!("[CONTEXT] Selected files:");
                    if rt.selected_files.is_empty() {
                        println!("  (None)");
                    } else {
                        for file in &rt.selected_files {
                            println!("  • {}", file);
                        }
                    }
                }
                "budget" => {
                    println!("[CONTEXT] Context Budget:");
                    let mut total_chars = 0;
                    let root = std::env::current_dir().unwrap_or_default();
                    for file in &rt.selected_files {
                        let content = std::fs::read_to_string(root.join(file)).unwrap_or_default();
                        let chars = content.chars().count();
                        total_chars += chars;
                        println!("  • {} ({} chars)", file, chars);
                    }
                    println!("  Total: {} chars", total_chars);
                }
                "suggest" => {
                    println!(
                        "[CONTEXT] Suggestions based on recent edits / current task (planned)."
                    );
                }
                _ => println!("[CONTEXT] Usage: /context [add|remove|clear|show|budget|suggest]"),
            }
            true
        }

        cmd if cmd.starts_with("/files") => {
            let args = cmd.trim().split_whitespace().collect::<Vec<_>>();
            if args.get(1).copied() == Some("relevant") {
                if let Some(query) = args.get(2) {
                    println!("[FILES] Finding files relevant to '{}' (planned)...", query);
                } else {
                    println!("[FILES] Usage: /files relevant <query>");
                }
            } else {
                println!("[FILES] Usage: /files relevant <query>");
            }
            true
        }

        // ── Repo map ──────────────────────────────────────────────────────────
        cmd if cmd.starts_with("/repo-map") => {
            let sub = name.split_whitespace().nth(1).unwrap_or("show");
            let root = std::env::current_dir().unwrap_or_default();
            let max_chars = rt.config.repo_map.max_chars;
            let include_symbols = rt.config.repo_map.include_symbols;

            if sub == "refresh" {
                println!("[REPO-MAP] Refreshing repo map for {}...", root.display());
            } else {
                println!("[REPO-MAP] Scanning {}...", root.display());
            }

            let scanner = crate::repo_map::RepoMapScanner::new(root.clone());
            match scanner.scan() {
                Ok(map) => {
                    println!("{}", map.to_compact_string(max_chars, include_symbols));
                    let cmds = crate::repo_map::ProjectCommands::detect(&root);
                    println!(
                        "[REPO-MAP] Commands: check={}, test={}, lint={}, format={}",
                        cmds.check.as_deref().unwrap_or("none"),
                        cmds.test.as_deref().unwrap_or("none"),
                        cmds.lint.as_deref().unwrap_or("none"),
                        cmds.format.as_deref().unwrap_or("none"),
                    );
                    if let Some(ref brain) = rt.brain {
                        let meta = crate::project::ProjectScanner::new(root.clone())
                            .scan()
                            .ok();
                        if let Some(meta) = meta {
                            let _ = brain.save_project(root.to_string_lossy().as_ref(), &meta);
                        }
                    }
                }
                Err(e) => println!("[REPO-MAP] Scan failed: {}", e),
            }
            true
        }

        // ── Check/Test/Lint/Format (dev commands via ApprovalGate) ────────────
        cmd if matches!(cmd, "/check" | "/test" | "/lint" | "/format") => {
            let kind = &cmd[1..]; // strip leading /
            let root = std::env::current_dir().unwrap_or_default();
            let cmds = crate::repo_map::ProjectCommands::detect(&root);

            let base_cmd = match kind {
                "check" => cmds.check,
                "test" => cmds.test,
                "lint" => cmds.lint,
                "format" => cmds.format,
                _ => None,
            };

            let extra_args = parts.get(1).copied().unwrap_or("").trim();

            match base_cmd {
                None => {
                    println!("[DEV] No {} command detected for {}.", kind, root.display());
                    println!(
                        "[DEV] Supported: Rust (Cargo.toml), Node (package.json), Python (pyproject.toml), Go (go.mod)."
                    );
                }
                Some(c) => {
                    let full_cmd = if extra_args.is_empty() {
                        c.clone()
                    } else {
                        format!("{} {}", c, extra_args)
                    };

                    println!("[DEV] Detected {} command: {}", kind, full_cmd);

                    // Build approval request for ApprovalGate
                    let req = crate::approval::bash_approval_request(&full_cmd);
                    if let Some(decision) = rt.approval_gate.check_policy(&req) {
                        if decision.is_approved() {
                            println!("[DEV] Auto-approved by session policy. Running...");
                            let output = std::process::Command::new("bash")
                                .arg("-c")
                                .arg(&full_cmd)
                                .output();
                            match output {
                                Ok(out) => {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    if !stdout.is_empty() {
                                        print!("{}", stdout);
                                    }
                                    if !stderr.is_empty() {
                                        eprint!("{}", stderr);
                                    }
                                    if out.status.success() {
                                        println!("[DEV] ✓ {} completed.", kind);
                                    } else {
                                        println!(
                                            "[DEV] ✗ {} failed (exit {:?}).",
                                            kind,
                                            out.status.code()
                                        );
                                    }
                                }
                                Err(e) => println!("[DEV] Error: {}", e),
                            }
                        } else {
                            println!("[DEV] {} denied by session policy.", kind);
                        }
                    } else {
                        // Interactive approval prompt
                        for line in req.display_lines() {
                            println!("{}", line);
                        }
                        print!("Your choice: ");
                        use std::io::{self, BufRead, Write};
                        io::stdout().flush().ok();
                        let mut input = String::new();
                        io::stdin().lock().read_line(&mut input).ok();
                        let ch = input.trim().chars().next().unwrap_or('n');
                        let decision = rt.approval_gate.resolve(&req, ch);
                        if decision.is_approved() {
                            println!("[DEV] Running: {}", full_cmd);
                            let output = std::process::Command::new("bash")
                                .arg("-c")
                                .arg(&full_cmd)
                                .output();
                            match output {
                                Ok(out) => {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    if !stdout.is_empty() {
                                        print!("{}", stdout);
                                    }
                                    if !stderr.is_empty() {
                                        eprint!("{}", stderr);
                                    }
                                    if out.status.success() {
                                        println!("[DEV] ✓ {} completed.", kind);
                                    } else {
                                        println!(
                                            "[DEV] ✗ {} failed (exit {:?}).",
                                            kind,
                                            out.status.code()
                                        );
                                    }
                                }
                                Err(e) => println!("[DEV] Error: {}", e),
                            }
                        } else {
                            println!("[DEV] {} denied.", kind);
                        }
                    }
                }
            }
            true
        }

        // ── Patch ────────────────────────────────────────────────────────────
        cmd if cmd.starts_with("/patch") => {
            let logs = crate::task::handle_patch_command(&mut rt.workflow, &parts);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/task") => {
            let logs = crate::task::handle_task_command(&mut rt.workflow, &parts[1..]);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/mode") => {
            let logs = crate::task::handle_mode_command(&mut rt.workflow, &parts[1..]);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/plan") => {
            let logs = crate::task::handle_plan_command(&mut rt.workflow, &parts[1..]);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/act") => {
            let logs = crate::task::handle_act_command(&mut rt.workflow, &parts[1..]);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/code") => {
            let logs = crate::task::handle_code_command(&mut rt.workflow, &parts[1..]);
            for l in logs {
                println!("{}", l);
            }
            true
        }
        cmd if cmd.starts_with("/verify") => {
            let root = std::env::current_dir().unwrap_or_default();
            let cmds = crate::repo_map::ProjectCommands::detect(&root);
            println!("[VERIFY] Verification checks available:");
            let mut found = false;
            if let Some(cmd) = &cmds.check {
                println!("  - check: {}", cmd);
                found = true;
            }
            if let Some(cmd) = &cmds.test {
                println!("  - test: {}", cmd);
                found = true;
            }
            if let Some(cmd) = &cmds.lint {
                println!("  - lint: {}", cmd);
                found = true;
            }
            if let Some(cmd) = &cmds.format {
                println!("  - format: {}", cmd);
                found = true;
            }
            if found {
                println!(
                    "[VERIFY] Use 'goat check' or 'goat test' CLI commands to execute these safely with ApprovalGate."
                );
                if let Some(task) = &mut rt.workflow.active_task {
                    task.status = crate::task::TaskStatus::Testing;
                }
            } else {
                println!("[VERIFY] No verification commands detected for this project.");
            }
            true
        }

        // ── Phase 5.16: Agents ──────────────────────────────────────────────
        "/cofounder" => {
            let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
            let subcmd = parts.get(1).copied().unwrap_or("list");
            let target = parts.get(2).copied().unwrap_or("").trim();
            if let Ok(mut manager) = crate::agents::cofounder::CofounderManager::new() {
                match subcmd {
                    "list" => {
                        println!("[COFOUNDER] Ideas:");
                        for i in manager.list_ideas() {
                            println!("  [{}] {} ({:?})", i.id, i.title, i.state);
                        }
                    }
                    "new-idea" => {
                        match manager.add_idea(target.to_string(), "Mock".into(), "Mock".into()) {
                            Ok(i) => println!("[COFOUNDER] Created {}", i.id),
                            Err(e) => println!("[COFOUNDER] Error: {}", e),
                        }
                    }
                    "validate" => {
                        if let Ok(plan) = manager.generate_validation_plan(target) {
                            println!("[COFOUNDER] Validation Plan: {} steps", plan.steps.len());
                        }
                    }
                    "score" => {
                        if let Ok(score) = manager.generate_scorecard(target) {
                            println!("[COFOUNDER] Scorecard: {}/50", score.total_score);
                        }
                    }
                    "mvp" => {
                        if let Ok(mvp) = manager.generate_mvp_scope(target) {
                            println!(
                                "[COFOUNDER] MVP Scope: {} features",
                                mvp.core_features.len()
                            );
                        }
                    }
                    "competitors" => {
                        if let Ok(comps) = manager.generate_competitors(target) {
                            println!("[COFOUNDER] Competitors: {}", comps.len());
                        }
                    }
                    "landing" => {
                        if let Ok(brief) = manager.generate_landing_page_brief(target) {
                            println!("[COFOUNDER] Landing: {}", brief);
                        }
                    }
                    "outreach" => {
                        if let Ok(plan) = manager.generate_outreach_plan(target) {
                            println!("[COFOUNDER] Outreach: {} channels", plan.channels.len());
                        }
                    }
                    "report" => {
                        if let Ok(r) = manager.generate_report(target) {
                            println!("[COFOUNDER] Report: {}", r.summary);
                        }
                    }
                    "show" => {
                        if let Some(idea) = manager.get_idea(target) {
                            println!("[COFOUNDER] Idea: {:?}", idea);
                        }
                    }
                    _ => println!("[COFOUNDER] Unknown: {}", subcmd),
                }
            } else {
                println!("[COFOUNDER] Failed to init manager");
            }
            true
        }
        "/agents" => {
            let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
            let subcmd = parts.get(1).copied().unwrap_or("list");
            let target = parts.get(2).copied().unwrap_or("").trim();
            let mut registry = crate::agents::AgentRegistry::new();
            let report_mgr = crate::reports::ReportManager::new();

            match subcmd {
                "list" => {
                    println!("[AGENTS] GOAT Agent Registry:");
                    let agents = registry.list();
                    for a in agents {
                        println!("  [{:?}] {} ({}): {}", a.tier, a.name, a.id, a.description);
                        println!(
                            "    Status: {:?} | Affinity: {:?}",
                            a.status, a.prime_affinity
                        );
                    }
                }
                "show" => {
                    if let Some(agent) = registry.get(target) {
                        println!("[AGENTS] Name: {} ({})", agent.name, agent.id);
                        println!("  Tier: {:?}", agent.tier);
                        println!("  Status: {:?}", agent.status);
                        println!("  Affinity: {:?}", agent.prime_affinity);
                        println!("  Traits: {:?}", agent.traits);
                        println!("  Capabilities: {:?}", agent.capabilities);
                    } else {
                        println!("[AGENTS] Agent '{}' not found.", target);
                    }
                }
                "enable" => {
                    if let Err(e) = registry.enable(target) {
                        println!("[AGENTS] Error: {}", e);
                    } else {
                        println!("[AGENTS] Agent '{}' enabled (Active).", target);
                        // Using TimelineManager directly if needed, or omit for headless if not readily available
                        println!("[TIMELINE] Agent {} enabled", target);
                    }
                }
                "disable" => {
                    if let Err(e) = registry.disable(target) {
                        println!("[AGENTS] Error: {}", e);
                    } else {
                        println!("[AGENTS] Agent '{}' disabled.", target);
                    }
                }
                "reports" => {
                    println!("[AGENTS] Reports for {}:", target);
                    if let Ok(reports) = report_mgr.list_reports() {
                        for r in reports {
                            println!("  - {} ({}) [{}]", r.title, r.id, r.created_at);
                        }
                    }
                }
                "specialists" => {
                    println!("[AGENTS] Specialists for Prime '{}':", target);
                    for a in registry.list() {
                        if a.prime_affinity.as_deref() == Some(target) {
                            println!("  - {} ({})", a.name, a.id);
                        }
                    }
                }
                _ => {
                    println!(
                        "[AGENTS] Valid subcommands: list, show, enable, disable, reports, specialists"
                    );
                }
            }
            true
        }

        _ => false,
    }
}

// ── Main agent turn ───────────────────────────────────────────────────────────

/// Run a single user-prompt → agent-response turn.
async fn run_agent_turn(rt: &mut GoatRuntime, user_msg: String, active_skill: Option<&str>) {
    println!("[YOU] {}", user_msg);

    let hook_logs = rt
        .hooks_manager
        .run_hooks("on_submit", &mut rt.approval_gate)
        .await
        .unwrap_or_default();
    for log in hook_logs {
        println!("[HOOKS] {}", log);
    }

    let is_first = rt.history.iter().all(|m| m.role != "user");
    if is_first {
        if let Some(ref brain) = rt.brain {
            let title = crate::app::generate_session_title(&user_msg);
            let _ = brain.update_session_title(&rt.session_id, &title);
        }
    }

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

        let memory_manager = crate::memory::MemoryManager::new(&rt.paths, rt.config.memory.clone());
        let mut sys_prompt = route.profile.system_prompt.to_string();

        match rt.workflow.mode {
            crate::task::AgentMode::Plan => {
                sys_prompt.push_str("\n\n<workflow_mode>\nCURRENT MODE: PLAN\nYou are in PLAN mode. You MUST NOT execute file writes, run shell commands, or make edits. Your only goal is to produce a structured plan for the user's task. Outline goals, steps, relevant files, risks, and required commands. Then ask the user to switch to ACT mode (/act) to execute.\n</workflow_mode>");
            }
            crate::task::AgentMode::Act => {
                sys_prompt.push_str("\n\n<workflow_mode>\nCURRENT MODE: ACT\nYou are in ACT mode. You may propose file patches (write_file) and run safe commands (run_command). Remember to use ApprovalGate.\n</workflow_mode>");
            }
        }
        if let Some(task) = &rt.workflow.active_task {
            sys_prompt.push_str(&format!(
                "\n\n<active_task>\nTASK: {}\nSTATUS: {:?}\n",
                task.request, task.status
            ));
            if let Some(plan) = &task.plan_text {
                sys_prompt.push_str(&format!("PLAN:\n{}\n", plan));
            }
            sys_prompt.push_str("</active_task>");
        }

        let memory_context = memory_manager.build_context(rt.brain.as_ref());
        if !memory_context.is_empty() {
            sys_prompt.push_str("\n\n");
            sys_prompt.push_str(&memory_context);
        }
        let skill_manager =
            crate::skills::SkillManager::new(rt.paths.clone(), rt.config.skills.clone());
        let skill_context = skill_manager.build_context(active_skill);
        if !skill_context.is_empty() {
            sys_prompt.push_str("\n\n");
            sys_prompt.push_str(&skill_context);
        }

        let mut routed_history = vec![Message {
            role: "system".to_string(),
            content: Some(sys_prompt),
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

                            let mut patch_id = None;
                            if tc.function.name == "write_file" {
                                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                                let content =
                                    args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                                let preview = crate::repo_map::generate_diff_preview(path, content);
                                let diff_lines =
                                    crate::repo_map::format_diff_preview(&preview).join("\n");
                                patch_id = Some(rt.workflow.add_patch(
                                    path.to_string(),
                                    content.to_string(),
                                    diff_lines,
                                ));
                                if let Some(task) = &mut rt.workflow.active_task {
                                    task.status = crate::task::TaskStatus::PatchProposed;
                                }
                            }

                            let tool_action = rt
                                .tool_registry
                                .evaluate_action(&tc.function.name, &rt.config.tools);
                            if let crate::tool_registry::ToolAction::Deny(ref reason) = tool_action
                            {
                                println!("[TOOL] Denied by policy: {}", reason);
                                rt.tool_registry.log_execution(
                                    &rt.paths,
                                    "headless",
                                    &tc.function.name,
                                    &tool_action,
                                    false,
                                    reason,
                                );
                                rt.history.push(Message {
                                    role: "tool".to_string(),
                                    content: Some(format!(
                                        "Tool execution denied. Reason: {}",
                                        reason
                                    )),
                                    tool_calls: None,
                                    tool_call_id: Some(tc.id),
                                });
                                trim_history(&mut rt.history);
                                continue;
                            }

                            let approval_req =
                                build_approval_request(&tc.function.name, &args, &tool_action);

                            if let Some(req) = approval_req {
                                // Check session policy first.
                                match rt.approval_gate.check_policy(&req) {
                                    Some(ApprovalDecision::Approved) => {
                                        println!(
                                            "[APPROVAL] Auto-approved (session policy): {}",
                                            tc.function.name
                                        );
                                        let hook_logs = rt
                                            .hooks_manager
                                            .run_hooks("before_tool_call", &mut rt.approval_gate)
                                            .await
                                            .unwrap_or_default();
                                        for log in hook_logs {
                                            println!("[HOOKS] {}", log);
                                        }

                                        let is_patch = patch_id.is_some();
                                        if is_patch {
                                            let logs = rt
                                                .hooks_manager
                                                .run_hooks(
                                                    "before_patch_apply",
                                                    &mut rt.approval_gate,
                                                )
                                                .await
                                                .unwrap_or_default();
                                            for log in logs {
                                                println!("[HOOKS] {}", log);
                                            }
                                        }

                                        let result =
                                            execute_tool(rt, &tc.function.name, args).await;

                                        let hook_logs = rt
                                            .hooks_manager
                                            .run_hooks("after_tool_call", &mut rt.approval_gate)
                                            .await
                                            .unwrap_or_default();
                                        for log in hook_logs {
                                            println!("[HOOKS] {}", log);
                                        }

                                        if let Some(id) = &patch_id {
                                            if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                p.status = crate::task::PatchStatus::Applied;
                                            }
                                            if let Some(task) = &mut rt.workflow.active_task {
                                                task.status = crate::task::TaskStatus::PatchApplied;
                                            }
                                            let logs = rt
                                                .hooks_manager
                                                .run_hooks(
                                                    "after_patch_apply",
                                                    &mut rt.approval_gate,
                                                )
                                                .await
                                                .unwrap_or_default();
                                            for log in logs {
                                                println!("[HOOKS] {}", log);
                                            }
                                        }
                                        println!("[TOOL] {}", result);

                                        rt.tool_registry.log_execution(
                                            &rt.paths,
                                            "headless",
                                            &tc.function.name,
                                            &tool_action,
                                            true,
                                            &result,
                                        );

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
                                        if let Some(id) = &patch_id {
                                            if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                p.status = crate::task::PatchStatus::Discarded;
                                            }
                                        }
                                        rt.history.push(Message {
                                            role: "tool".to_string(),
                                            content: Some(format!(
                                                "Tool execution denied (session policy). Reason: {}",
                                                reason
                                            )),
                                            tool_calls: None,
                                            tool_call_id: Some(tc.id),
                                        });

                                        rt.tool_registry.log_execution(
                                            &rt.paths,
                                            "headless",
                                            &tc.function.name,
                                            &crate::tool_registry::ToolAction::Deny(reason.clone()),
                                            false,
                                            &reason,
                                        );

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
                                                let hook_logs = rt
                                                    .hooks_manager
                                                    .run_hooks(
                                                        "before_tool_call",
                                                        &mut rt.approval_gate,
                                                    )
                                                    .await
                                                    .unwrap_or_default();
                                                for log in hook_logs {
                                                    println!("[HOOKS] {}", log);
                                                }

                                                let is_patch = patch_id.is_some();
                                                if is_patch {
                                                    let logs = rt
                                                        .hooks_manager
                                                        .run_hooks(
                                                            "before_patch_apply",
                                                            &mut rt.approval_gate,
                                                        )
                                                        .await
                                                        .unwrap_or_default();
                                                    for log in logs {
                                                        println!("[HOOKS] {}", log);
                                                    }
                                                }

                                                let result =
                                                    execute_tool(rt, &tc.function.name, args).await;

                                                let hook_logs = rt
                                                    .hooks_manager
                                                    .run_hooks(
                                                        "after_tool_call",
                                                        &mut rt.approval_gate,
                                                    )
                                                    .await
                                                    .unwrap_or_default();
                                                for log in hook_logs {
                                                    println!("[HOOKS] {}", log);
                                                }

                                                if let Some(id) = &patch_id {
                                                    if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                        p.status =
                                                            crate::task::PatchStatus::Applied;
                                                    }
                                                    if let Some(task) = &mut rt.workflow.active_task
                                                    {
                                                        task.status =
                                                            crate::task::TaskStatus::PatchApplied;
                                                    }
                                                    let logs = rt
                                                        .hooks_manager
                                                        .run_hooks(
                                                            "after_patch_apply",
                                                            &mut rt.approval_gate,
                                                        )
                                                        .await
                                                        .unwrap_or_default();
                                                    for log in logs {
                                                        println!("[HOOKS] {}", log);
                                                    }
                                                }
                                                println!("[TOOL] {}", result);

                                                rt.tool_registry.log_execution(
                                                    &rt.paths,
                                                    "headless",
                                                    &tc.function.name,
                                                    &tool_action,
                                                    true,
                                                    &result,
                                                );

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
                                                if let Some(id) = &patch_id {
                                                    if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                        p.status =
                                                            crate::task::PatchStatus::Discarded;
                                                    }
                                                }
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
                                let hook_logs = rt
                                    .hooks_manager
                                    .run_hooks("before_tool_call", &mut rt.approval_gate)
                                    .await
                                    .unwrap_or_default();
                                for log in hook_logs {
                                    println!("[HOOKS] {}", log);
                                }

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

                                let hook_logs = rt
                                    .hooks_manager
                                    .run_hooks("after_tool_call", &mut rt.approval_gate)
                                    .await
                                    .unwrap_or_default();
                                for log in hook_logs {
                                    println!("[HOOKS] {}", log);
                                }

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
pub fn prompt_approval_stdin(
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

fn build_approval_request(
    tool_name: &str,
    args: &Value,
    tool_action: &crate::tool_registry::ToolAction,
) -> Option<ApprovalRequest> {
    use crate::approval::{ApprovalRequest, bash_approval_request, call_subagent_approval_request};
    let req = match tool_name {
        "bash" => {
            let command = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
            Some(bash_approval_request(command))
        }
        "write_file" => {
            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

            // Generate diff preview before showing the approval gate.
            let preview = crate::repo_map::generate_diff_preview(path, content);
            let diff_lines = crate::repo_map::format_diff_preview(&preview);

            let risk = crate::approval::assess_write_risk(path);

            let mut explanation = format!(
                "{} file: {} line(s) added, {} line(s) removed",
                if preview.is_new_file {
                    "New"
                } else {
                    "Modified"
                },
                preview.added_lines,
                preview.removed_lines
            );
            if preview.has_secret_warning {
                explanation.push_str(
                    " | \u{26a0} SECRET-LIKE CONTENT DETECTED \u{2014} values redacted in preview",
                );
            }
            explanation.push('\n');
            explanation.push_str(&diff_lines.join("\n"));

            Some(ApprovalRequest {
                tool_name: "write_file".to_string(),
                action_summary: format!("Write to: {}", path),
                risk_level: risk,
                explanation: Some(explanation),
                working_directory: None,
            })
        }
        "call_subagent" => {
            let agent_cli = args.get("agent_cli").and_then(|v| v.as_str()).unwrap_or("");
            let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
            Some(call_subagent_approval_request(agent_cli, prompt))
        }
        _ => None,
    };

    if req.is_none() && matches!(tool_action, crate::tool_registry::ToolAction::Ask(_)) {
        Some(ApprovalRequest {
            tool_name: tool_name.to_string(),
            action_summary: format!("Execute tool '{}'", tool_name),
            risk_level: crate::approval::RiskLevel::High,
            explanation: Some(format!(
                "Arguments: {}",
                serde_json::to_string_pretty(args).unwrap_or_default()
            )),
            working_directory: None,
        })
    } else {
        req
    }
}

async fn execute_tool(rt: &mut crate::runtime::GoatRuntime, name: &str, args: Value) -> String {
    if let Some(native_result) = NativeTools::execute(name, args.clone()).await {
        match native_result {
            Ok(result) => result,
            Err(e) => format!("Tool error: {}", e),
        }
    } else {
        match rt.mcp_manager.call_tool(name, args).await {
            Ok(res) => serde_json::to_string(&res).unwrap_or_else(|_| "[]".to_string()),
            Err(e) => format!("MCP tool error: {}", e),
        }
    }
}

fn trim_history(history: &mut Vec<Message>) {
    let extra = history.len().saturating_sub(MAX_HISTORY_MESSAGES);
    if extra > 0 {
        history.drain(0..extra);
    }
}

async fn handle_scheduled_jobs(rt: &mut crate::runtime::GoatRuntime) {
    let jobs = rt.scheduler_manager.tick();
    for job in jobs {
        println!(
            "[SCHEDULE] Executing job {}: {}",
            job.id, job.prompt_or_command
        );
        rt.job_tracker.add_job(crate::jobs::BackgroundJob {
            id: job.id.clone(),
            r#type: "scheduled".to_string(),
            status: "running".to_string(),
            started_at: chrono::Utc::now().to_rfc3339(),
            finished_at: None,
            output_preview: None,
            error: None,
            approval_status: None,
        });
        rt.scheduler_manager.log_audit(&format!(
            "Executed job {}: {}",
            job.id, job.prompt_or_command
        ));
    }
}
