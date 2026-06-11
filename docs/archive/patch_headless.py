import sys

content = open("src/headless.rs").read()

# Replace slash commands
slash_old = """        // ── Patch ────────────────────────────────────────────────────────────
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
        }"""

slash_new = """        // ── Patch ────────────────────────────────────────────────────────────
        cmd if cmd.starts_with("/patch") => {
            let logs = crate::task::handle_patch_command(&mut rt.workflow, &parts);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/task") => {
            let logs = crate::task::handle_task_command(&mut rt.workflow, &parts[1..]);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/mode") => {
            let logs = crate::task::handle_mode_command(&mut rt.workflow, &parts[1..]);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/plan") => {
            let logs = crate::task::handle_plan_command(&mut rt.workflow, &parts[1..]);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/act") => {
            let logs = crate::task::handle_act_command(&mut rt.workflow, &parts[1..]);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/code") => {
            let logs = crate::task::handle_code_command(&mut rt.workflow, &parts[1..]);
            for l in logs { println!("{}", l); }
            true
        }
        cmd if cmd.starts_with("/verify") => {
            let root = std::env::current_dir().unwrap_or_default();
            let cmds = crate::repo_map::ProjectCommands::detect(&root);
            println!("[VERIFY] Verification checks available:");
            let mut found = false;
            if let Some(cmd) = &cmds.check { println!("  - check: {}", cmd); found = true; }
            if let Some(cmd) = &cmds.test { println!("  - test: {}", cmd); found = true; }
            if let Some(cmd) = &cmds.lint { println!("  - lint: {}", cmd); found = true; }
            if let Some(cmd) = &cmds.format { println!("  - format: {}", cmd); found = true; }
            if found {
                println!("[VERIFY] Use 'goat check' or 'goat test' CLI commands to execute these safely with ApprovalGate.");
                if let Some(task) = &mut rt.workflow.active_task {
                    task.status = crate::task::TaskStatus::Testing;
                }
            } else {
                println!("[VERIFY] No verification commands detected for this project.");
            }
            true
        }"""
content = content.replace(slash_old, slash_new)

# Patch Tool logic
tool_old = """                            let args: Value = serde_json::from_str(&tc.function.arguments)
                                .unwrap_or(serde_json::json!({}));

                            let approval_req = build_approval_request(&tc.function.name, &args);"""

tool_new = """                            let args: Value = serde_json::from_str(&tc.function.arguments)
                                .unwrap_or(serde_json::json!({}));

                            let mut patch_id = None;
                            if tc.function.name == "write_file" {
                                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                                let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                                let preview = crate::repo_map::generate_diff_preview(path, content);
                                let diff_lines = crate::repo_map::format_diff_preview(&preview).join("\\n");
                                patch_id = Some(rt.workflow.add_patch(path.to_string(), content.to_string(), diff_lines));
                                if let Some(task) = &mut rt.workflow.active_task {
                                    task.status = crate::task::TaskStatus::PatchProposed;
                                }
                            }

                            let approval_req = build_approval_request(&tc.function.name, &args);"""
content = content.replace(tool_old, tool_new)

# Session policy Auto-approved
auto_approved_old = """                                    Some(ApprovalDecision::Approved) => {
                                        println!(
                                            "[APPROVAL] Auto-approved (session policy): {}",
                                            tc.function.name
                                        );
                                        let result =
                                            execute_native_tool(&tc.function.name, args).await;
                                        println!("[TOOL] {}", result);"""

auto_approved_new = """                                    Some(ApprovalDecision::Approved) => {
                                        println!(
                                            "[APPROVAL] Auto-approved (session policy): {}",
                                            tc.function.name
                                        );
                                        let result =
                                            execute_native_tool(&tc.function.name, args).await;
                                        if let Some(id) = &patch_id {
                                            if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                p.status = crate::task::PatchStatus::Applied;
                                            }
                                            if let Some(task) = &mut rt.workflow.active_task {
                                                task.status = crate::task::TaskStatus::PatchApplied;
                                            }
                                        }
                                        println!("[TOOL] {}", result);"""
content = content.replace(auto_approved_old, auto_approved_new)

# Session policy Auto-denied
auto_denied_old = """                                    Some(ApprovalDecision::Denied(reason)) => {
                                        println!(
                                            "[APPROVAL] Auto-denied (session policy): {} — {}",
                                            tc.function.name, reason
                                        );
                                        rt.history.push(Message {"""
auto_denied_new = """                                    Some(ApprovalDecision::Denied(reason)) => {
                                        println!(
                                            "[APPROVAL] Auto-denied (session policy): {} — {}",
                                            tc.function.name, reason
                                        );
                                        if let Some(id) = &patch_id {
                                            if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                p.status = crate::task::PatchStatus::Discarded;
                                            }
                                        }
                                        rt.history.push(Message {"""
content = content.replace(auto_denied_old, auto_denied_new)

# Interactive Approved
interactive_approved_old = """                                            ApprovalDecision::Approved => {
                                                println!(
                                                    "[APPROVAL] ✓ Approved: {}",
                                                    tc.function.name
                                                );
                                                let result =
                                                    execute_native_tool(&tc.function.name, args)
                                                        .await;
                                                println!("[TOOL] {}", result);"""

interactive_approved_new = """                                            ApprovalDecision::Approved => {
                                                println!(
                                                    "[APPROVAL] ✓ Approved: {}",
                                                    tc.function.name
                                                );
                                                let result =
                                                    execute_native_tool(&tc.function.name, args)
                                                        .await;
                                                if let Some(id) = &patch_id {
                                                    if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                        p.status = crate::task::PatchStatus::Applied;
                                                    }
                                                    if let Some(task) = &mut rt.workflow.active_task {
                                                        task.status = crate::task::TaskStatus::PatchApplied;
                                                    }
                                                }
                                                println!("[TOOL] {}", result);"""
content = content.replace(interactive_approved_old, interactive_approved_new)

# Interactive Denied
interactive_denied_old = """                                            ApprovalDecision::Denied(reason) => {
                                                println!(
                                                    "[APPROVAL] ✗ Denied: {} — {}",
                                                    tc.function.name, reason
                                                );
                                                rt.history.push(Message {"""
interactive_denied_new = """                                            ApprovalDecision::Denied(reason) => {
                                                println!(
                                                    "[APPROVAL] ✗ Denied: {} — {}",
                                                    tc.function.name, reason
                                                );
                                                if let Some(id) = &patch_id {
                                                    if let Some(p) = rt.workflow.get_patch_mut(id) {
                                                        p.status = crate::task::PatchStatus::Discarded;
                                                    }
                                                }
                                                rt.history.push(Message {"""
content = content.replace(interactive_denied_old, interactive_denied_new)

open("src/headless.rs", "w").write(content)

