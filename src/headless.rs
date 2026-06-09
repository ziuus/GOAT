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
    let mut active_skill: Option<String> = None;

    loop {
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

        let input = line.trim().to_string();
        if input.is_empty() {
            continue;
        }

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
            println!("[HELP] Headless commands:");
            println!("[HELP]   /help            — show this help");
            println!("[HELP]   /status          — show provider/session/brain/repo status");
            println!("[HELP]   /repo-map        — show repo map for current project");
            println!("[HELP]   /repo-map refresh — force rescan repo map");
            println!("[HELP]   /check           — run project check command");
            println!("[HELP]   /test [filter]   — run project test command");
            println!("[HELP]   /lint            — run project lint command");
            println!("[HELP]   /format          — run project format command");
            println!("[HELP]   /patch           — show pending code patches");
            println!("[HELP]   /patch apply     — apply pending patch");
            println!("[HELP]   /patch discard   — discard pending patch");
            println!("[HELP]   /profile         — show current profile");
            println!("[HELP]   /profile <name>  — switch to a profile");
            println!("[HELP]   /profiles        — list available profiles");
            println!("[HELP]   /new             — start a new session");
            println!("[HELP]   /clear           — clear screen and reset");
            println!("[HELP]   /sessions        — list recent sessions");
            println!("[HELP]   /tools           — list available tools");
            println!("[HELP]   /project         — manage project context");
            println!("[HELP]   /memory          — view or manage curated memory");
            println!("[HELP]   /recall <query>  — search conversation history");
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
            let sub = name.split_whitespace().nth(1).unwrap_or("show");
            match sub {
                "show" => {
                    println!("[PATCH] No pending patch in this session.");
                    println!(
                        "[PATCH] Diff-before-write is shown automatically when the agent proposes a file write."
                    );
                    println!("[PATCH] Full patch queue is planned for Phase 2.4.");
                }
                "apply" => {
                    println!(
                        "[PATCH] No pending patch to apply. Propose a file write via the agent first."
                    );
                }
                "discard" => {
                    println!("[PATCH] No pending patch to discard.");
                }
                _ => {
                    println!(
                        "[PATCH] Unknown: {}. Use /patch, /patch apply, /patch discard.",
                        sub
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
    use crate::approval::{ApprovalRequest, bash_approval_request, call_subagent_approval_request};
    match tool_name {
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
