use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMode {
    Plan,
    Act,
}

impl Default for AgentMode {
    fn default() -> Self {
        AgentMode::Act
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Planning,
    AwaitingApproval,
    PatchProposed,
    PatchApplied,
    Testing,
    Failed,
    Completed,
}

#[derive(Debug, Clone)]
pub struct CodingTask {
    pub id: String,
    pub request: String,
    pub plan_text: Option<String>,
    pub status: TaskStatus,
    pub created_at: SystemTime,
}

impl CodingTask {
    pub fn new(request: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            request,
            plan_text: None,
            status: TaskStatus::Planning,
            created_at: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchStatus {
    Pending,
    Applied,
    Discarded,
}

#[derive(Debug, Clone)]
pub struct Patch {
    pub id: String,
    pub file_path: String,
    pub proposed_content: String,
    pub unified_diff: String,
    pub status: PatchStatus,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowState {
    pub mode: AgentMode,
    pub active_task: Option<CodingTask>,
    pub patches: Vec<Patch>,
}

impl WorkflowState {
    pub fn add_patch(&mut self, path: String, content: String, diff: String) -> String {
        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let patch = Patch {
            id: id.clone(),
            file_path: path,
            proposed_content: content,
            unified_diff: diff,
            status: PatchStatus::Pending,
            created_at: SystemTime::now(),
        };
        self.patches.push(patch);
        id
    }
    
    pub fn get_patch_mut(&mut self, id: &str) -> Option<&mut Patch> {
        self.patches.iter_mut().find(|p| p.id == id)
    }

    pub fn get_pending_patches(&self) -> Vec<&Patch> {
        self.patches.iter().filter(|p| matches!(p.status, PatchStatus::Pending)).collect()
    }
    
    pub fn clear_patches(&mut self) {
        self.patches.clear();
    }
}
use crate::runtime::GoatRuntime;
use crate::approval::{RiskLevel, ApprovalRequest};

pub fn handle_mode_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() {
        let current = match workflow.mode {
            crate::task::AgentMode::Plan => "Plan",
            crate::task::AgentMode::Act => "Act",
        };
        logs.push(format!("[MODE] Current mode: {}", current));
        logs.push("[MODE] Use /mode plan or /mode act to switch.".to_string());
    } else {
        match args[0].to_lowercase().as_str() {
            "plan" => {
                workflow.mode = crate::task::AgentMode::Plan;
                logs.push("[MODE] Switched to Plan mode.".to_string());
                logs.push("[MODE] GOAT will only create plans and will not modify files.".to_string());
            }
            "act" => {
                workflow.mode = crate::task::AgentMode::Act;
                logs.push("[MODE] Switched to Act mode.".to_string());
                logs.push("[MODE] GOAT can propose patches (requires ApprovalGate).".to_string());
            }
            _ => {
                logs.push(format!("[MODE] Unknown mode: {}. Use 'plan' or 'act'.", args[0]));
            }
        }
    }
    logs
}

pub fn handle_task_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() || args[0] == "status" {
        if let Some(task) = &workflow.active_task {
            logs.push(format!("[TASK] Active Task: {}", task.id));
            logs.push(format!("[TASK] Request: {}", task.request));
            let status_str = match task.status {
                crate::task::TaskStatus::Planning => "Planning",
                crate::task::TaskStatus::AwaitingApproval => "Awaiting Approval",
                crate::task::TaskStatus::PatchProposed => "Patch Proposed",
                crate::task::TaskStatus::PatchApplied => "Patch Applied",
                crate::task::TaskStatus::Testing => "Testing",
                crate::task::TaskStatus::Failed => "Failed",
                crate::task::TaskStatus::Completed => "Completed",
            };
            logs.push(format!("[TASK] Status: {}", status_str));
            if let Some(plan) = &task.plan_text {
                logs.push("[TASK] Plan:".to_string());
                for line in plan.lines() {
                    logs.push(format!("  {}", line));
                }
            }
        } else {
            logs.push("[TASK] No active task.".to_string());
            logs.push("[TASK] Use /plan <request> or /code <request> to start a task.".to_string());
        }
    } else if args[0] == "clear" {
        workflow.active_task = None;
        logs.push("[TASK] Active task cleared.".to_string());
    } else {
        logs.push("[TASK] Unknown task command. Use /task, /task status, or /task clear.".to_string());
    }
    logs
}

pub fn handle_patch_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() || args[0] == "list" {
        let pending = workflow.get_pending_patches();
        if pending.is_empty() {
            logs.push("[PATCH] No pending patches.".to_string());
        } else {
            logs.push(format!("[PATCH] {} pending patches:", pending.len()));
            for p in pending {
                logs.push(format!("  [{}] {}", p.id, p.file_path));
            }
            logs.push("[PATCH] Commands: /patch show <id> · /patch apply <id> · /patch discard <id>".to_string());
        }
    } else if args[0] == "show" {
        if args.len() < 2 {
            logs.push("[PATCH] Please specify a patch ID.".to_string());
        } else {
            let id = args[1];
            if let Some(patch) = workflow.patches.iter().find(|p| p.id == id) {
                logs.push(format!("[PATCH] Showing patch {}:", id));
                logs.push(format!("[PATCH] File: {}", patch.file_path));
                for line in patch.unified_diff.lines() {
                    logs.push(format!("{}", line));
                }
            } else {
                logs.push(format!("[PATCH] Patch {} not found.", id));
            }
        }
    } else if args[0] == "apply" {
        if args.len() < 2 {
            logs.push("[PATCH] Please specify a patch ID.".to_string());
        } else {
            let id = args[1];
            if let Some(_patch) = workflow.patches.iter().find(|p| p.id == id) {
                // To apply, we just need to send it to the ApprovalGate. 
                // But from here we return log lines. The actual approval must be requested by the UI.
                logs.push(format!("[PATCH] Found patch {}. Use write_file tool to apply it with ApprovalGate.", id));
                logs.push("[PATCH] (Direct /patch apply without tool call requires injecting an ApprovalRequest directly into the app state.)".to_string());
            } else {
                logs.push(format!("[PATCH] Patch {} not found.", id));
            }
        }
    } else if args[0] == "discard" {
        if args.len() < 2 {
            logs.push("[PATCH] Please specify a patch ID.".to_string());
        } else {
            let id = args[1];
            if let Some(patch) = workflow.get_patch_mut(id) {
                patch.status = crate::task::PatchStatus::Discarded;
                logs.push(format!("[PATCH] Discarded patch {}.", id));
            } else {
                logs.push(format!("[PATCH] Patch {} not found.", id));
            }
        }
    } else if args[0] == "clear" {
        workflow.clear_patches();
        logs.push("[PATCH] All patches cleared.".to_string());
    } else {
        logs.push("[PATCH] Unknown command. Commands: list, show <id>, apply <id>, discard <id>, clear".to_string());
    }
    logs
}

pub fn handle_plan_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() {
        logs.push("[PLAN] Please specify a task request. Example: /plan add a README section".to_string());
        return logs;
    }
    let request = args.join(" ");
    workflow.mode = crate::task::AgentMode::Plan;
    workflow.active_task = Some(crate::task::CodingTask::new(request.clone()));
    logs.push(format!("[PLAN] Switched to Plan mode. Created new task: {}", request));
    logs.push("[PLAN] GOAT will now analyze the project and produce a structured plan.".to_string());
    logs
}

pub fn handle_act_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() {
        if workflow.active_task.is_some() {
            workflow.mode = crate::task::AgentMode::Act;
            logs.push("[ACT] Switched to Act mode for current task.".to_string());
        } else {
            logs.push("[ACT] No active task to act on. Please specify a task request or run /plan first.".to_string());
        }
        return logs;
    }
    let request = args.join(" ");
    workflow.mode = crate::task::AgentMode::Act;
    workflow.active_task = Some(crate::task::CodingTask::new(request.clone()));
    logs.push(format!("[ACT] Switched to Act mode. Created new task: {}", request));
    logs.push("[ACT] GOAT will now propose patches via ApprovalGate.".to_string());
    logs
}

pub fn handle_code_command(workflow: &mut WorkflowState, args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    if args.is_empty() {
        logs.push("[CODE] Please specify a task request. Example: /code improve README formatting".to_string());
        return logs;
    }
    let request = args.join(" ");
    // Produce a plan first
    workflow.mode = crate::task::AgentMode::Plan;
    workflow.active_task = Some(crate::task::CodingTask::new(request.clone()));
    logs.push(format!("[CODE] Started coding loop for: {}", request));
    logs.push("[CODE] Step 1: Planning (Plan Mode active).".to_string());
    logs.push("[CODE] Once the plan is ready, you can type /act to execute it.".to_string());
    logs
}

pub fn handle_verify_command(workflow: &mut WorkflowState, _args: &[&str]) -> Vec<String> {
    let mut logs = Vec::new();
    let root = std::env::current_dir().unwrap_or_default();
    let cmds = crate::repo_map::ProjectCommands::detect(&root);
    logs.push("[VERIFY] Verification checks available:".to_string());
    
    let mut found = false;
    if let Some(cmd) = &cmds.check {
        logs.push(format!("  - check: {}", cmd));
        found = true;
    }
    if let Some(cmd) = &cmds.test {
        logs.push(format!("  - test: {}", cmd));
        found = true;
    }
    if let Some(cmd) = &cmds.lint {
        logs.push(format!("  - lint: {}", cmd));
        found = true;
    }
    if let Some(cmd) = &cmds.format {
        logs.push(format!("  - format: {}", cmd));
        found = true;
    }
    
    if found {
        logs.push("[VERIFY] Use 'goat check' or 'goat test' CLI commands to execute these safely with ApprovalGate.".to_string());
        if let Some(task) = &mut workflow.active_task {
            task.status = TaskStatus::Testing;
        }
    } else {
        logs.push("[VERIFY] No verification commands detected for this project.".to_string());
    }
    logs
}
