use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use crate::approval::RiskLevel;
use crate::paths::GoatPaths;

/// Unique identifier for an internal subagent.
pub type SubagentId = String;

/// The role/kind of an internal subagent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubagentKind {
    Planner,
    Coder,
    Reviewer,
    Tester,
    Debugger,
    Documenter,
    Researcher,
    SecurityAuditor,
    UiDesigner,
    Refactorer,
}

impl std::fmt::Display for SubagentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Planner => "planner",
            Self::Coder => "coder",
            Self::Reviewer => "reviewer",
            Self::Tester => "tester",
            Self::Debugger => "debugger",
            Self::Documenter => "documenter",
            Self::Researcher => "researcher",
            Self::SecurityAuditor => "security-auditor",
            Self::UiDesigner => "ui-designer",
            Self::Refactorer => "refactorer",
        };
        write!(f, "{}", s)
    }
}

/// The state of a subagent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubagentStatus {
    Idle,
    Running,
    Finished,
    Failed,
}

/// The profile and capabilities of an internal subagent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentProfile {
    pub name: String,
    pub kind: SubagentKind,
    pub purpose: String,
    pub system_prompt: String,
    pub allowed_tools: Vec<String>,
    pub default_model_profile: String,
    pub risk_level: RiskLevel,
    pub context_budget: usize,
    pub can_propose_patches: bool,
    pub can_run_tools: bool,
    pub requires_approval: bool,
}

/// A request to an internal subagent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentRequest {
    pub task: String,
    pub context_summary: String,
    pub active_skill: Option<String>,
    pub memory_snapshot: Option<String>,
}

/// A response from an internal subagent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentResponse {
    pub content: String,
    pub tools_called: Vec<String>,
    pub success: bool,
}

/// Tracks an active subagent run (future-proofing for background runs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentRun {
    pub id: String,
    pub subagent: String,
    pub status: SubagentStatus,
    pub request: SubagentRequest,
}

/// Central registry of available internal subagents.
pub struct SubagentRegistry {
    pub subagents: HashMap<String, SubagentProfile>,
}

impl Default for SubagentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SubagentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            subagents: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    pub fn register(&mut self, profile: SubagentProfile) {
        self.subagents.insert(profile.name.to_lowercase(), profile);
    }

    pub fn get(&self, name: &str) -> Option<SubagentProfile> {
        self.subagents.get(&name.to_lowercase()).cloned()
    }

    pub fn list_all(&self) -> Vec<SubagentProfile> {
        let mut list: Vec<_> = self.subagents.values().cloned().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    fn register_defaults(&mut self) {
        self.register(SubagentProfile {
            name: "reviewer".to_string(),
            kind: SubagentKind::Reviewer,
            purpose: "Review code changes, patches, or plans for correctness and best practices.".to_string(),
            system_prompt: "You are the GOAT Reviewer subagent. Your job is to analyze provided plans or code patches, point out flaws, suggest improvements, and verify correctness. Be concise and precise. Do not act, only advise.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "balanced".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 16000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "tester".to_string(),
            kind: SubagentKind::Tester,
            purpose: "Analyze code and suggest testing strategies or specific unit tests.".to_string(),
            system_prompt: "You are the GOAT Tester subagent. Analyze the codebase or task and suggest tests. You can advise on testing strategy but do not execute tests yourself. Provide code snippets for tests when appropriate.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "fast".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 16000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "debugger".to_string(),
            kind: SubagentKind::Debugger,
            purpose: "Analyze error logs, tracebacks, or failing test output to identify root causes.".to_string(),
            system_prompt: "You are the GOAT Debugger subagent. Read the error messages and the related code context. Identify the root cause of the bug and explain how to fix it.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "deep".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 32000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "planner".to_string(),
            kind: SubagentKind::Planner,
            purpose: "Deconstruct complex tasks into step-by-step implementation plans.".to_string(),
            system_prompt: "You are the GOAT Planner subagent. Break down the user's objective into a logical sequence of actionable steps. Point out potential blockers and dependencies.".to_string(),
            allowed_tools: vec!["read_file".to_string()],
            default_model_profile: "deep".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 64000,
            can_propose_patches: false,
            can_run_tools: true,
            requires_approval: true,
        });

        self.register(SubagentProfile {
            name: "coder".to_string(),
            kind: SubagentKind::Coder,
            purpose: "Write code to accomplish a specific task or implement a specific plan.".to_string(),
            system_prompt: "You are the GOAT Coder subagent. Write the exact code needed. Output code blocks. You may propose a patch if appropriate.".to_string(),
            allowed_tools: vec!["read_file".to_string(), "grep_search".to_string()],
            default_model_profile: "balanced".to_string(),
            risk_level: RiskLevel::Medium,
            context_budget: 32000,
            can_propose_patches: true,
            can_run_tools: true,
            requires_approval: true,
        });

        self.register(SubagentProfile {
            name: "documenter".to_string(),
            kind: SubagentKind::Documenter,
            purpose: "Write or update documentation, docstrings, and READMEs.".to_string(),
            system_prompt: "You are the GOAT Documenter subagent. Your goal is to write clear, comprehensive documentation based on the provided code or context.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "fast".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 16000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "researcher".to_string(),
            kind: SubagentKind::Researcher,
            purpose: "Search documentation, web resources, and deep project context to find answers.".to_string(),
            system_prompt: "You are the GOAT Researcher subagent. Analyze the context and find the missing information required to proceed. Summarize your findings clearly.".to_string(),
            allowed_tools: vec!["search_web".to_string()],
            default_model_profile: "deep".to_string(),
            risk_level: RiskLevel::Medium,
            context_budget: 64000,
            can_propose_patches: false,
            can_run_tools: true,
            requires_approval: true,
        });

        self.register(SubagentProfile {
            name: "security-auditor".to_string(),
            kind: SubagentKind::SecurityAuditor,
            purpose: "Review code or plans for security vulnerabilities, secret leakage, and unsafe patterns.".to_string(),
            system_prompt: "You are the GOAT Security Auditor subagent. Examine the provided context strictly for security vulnerabilities, hardcoded secrets, unsafe deserialization, injection risks, etc.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "deep".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 32000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "ui-designer".to_string(),
            kind: SubagentKind::UiDesigner,
            purpose: "Suggest UI/UX layouts, styles, and frontend component structures.".to_string(),
            system_prompt: "You are the GOAT UI/UX Designer subagent. Suggest beautiful, modern frontend layouts. Think about accessibility, responsiveness, and user flow.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "balanced".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 16000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });

        self.register(SubagentProfile {
            name: "refactorer".to_string(),
            kind: SubagentKind::Refactorer,
            purpose: "Suggest codebase refactoring to improve maintainability and performance.".to_string(),
            system_prompt: "You are the GOAT Refactorer subagent. Look for code smells, duplicated logic, and performance bottlenecks, and suggest structural improvements.".to_string(),
            allowed_tools: vec![],
            default_model_profile: "balanced".to_string(),
            risk_level: RiskLevel::Low,
            context_budget: 32000,
            can_propose_patches: false,
            can_run_tools: false,
            requires_approval: false,
        });
    }
}

/// Orchestrates the execution and auditing of internal subagents.
pub struct SubagentManager {
    pub registry: SubagentRegistry,
    pub paths: GoatPaths,
}

impl SubagentManager {
    pub fn new(paths: GoatPaths) -> Self {
        Self {
            registry: SubagentRegistry::new(),
            paths,
        }
    }

    /// Run a subagent by asking it a task and providing scoped context.
    /// Returns the textual response from the subagent.
    pub async fn ask_agent(
        &self,
        name: &str,
        task: &str,
        context_summary: &str,
        active_skill: Option<String>,
        memory_snapshot: Option<String>,
        router: &crate::llm::LlmRouter,
        model_chain: &crate::models::ModelChain,
    ) -> Result<String> {
        let profile = self
            .registry
            .get(name)
            .ok_or_else(|| anyhow!("Subagent '{}' not found in registry", name))?;

        let mut sys_prompt = profile.system_prompt.clone();

        sys_prompt.push_str("\n\n--- SUBAGENT CONTEXT ---");

        if let Some(skill) = active_skill {
            sys_prompt.push_str(&format!("\nActive Skill:\n{}\n", skill));
        }

        if let Some(mem) = memory_snapshot {
            sys_prompt.push_str(&format!("\nUser Memory:\n{}\n", mem));
        }

        sys_prompt.push_str(&format!(
            "\nProject Context / Repo Map:\n{}\n",
            context_summary
        ));
        sys_prompt.push_str("------------------------");

        let messages = vec![
            crate::llm::Message {
                role: "system".to_string(),
                content: Some(sys_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
            crate::llm::Message {
                role: "user".to_string(),
                content: Some(format!("Task for subagent {}:\n{}", profile.name, task)),
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        // Note: For full tool execution, we would pass native tools filtered by `profile.allowed_tools`,
        // but for safety in Phase 2.7, we let the LLM just return text. If the LLM proposes tool calls in text,
        // it must be parsed manually by the user or outer loop.

        let (response, _) = router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow::anyhow!("Subagent LLM error: {:?}", e))?;

        let text_content = response.content.clone().unwrap_or_default();

        // Audit the execution
        self.log_execution(
            &profile.name,
            task,
            &profile.default_model_profile,
            true,
            &text_content,
        );

        Ok(text_content)
    }

    /// Appends a subagent execution record to `subagent-audit.log`.
    pub fn log_execution(
        &self,
        subagent_name: &str,
        task: &str,
        model_profile: &str,
        success: bool,
        output: &str,
    ) {
        let task_summary = if task.len() > 100 {
            format!("{}...", &task[..97]).replace("\n", " ")
        } else {
            task.replace("\n", " ")
        };

        let redacted_output = crate::approval::redact_secrets(output);
        let output_preview = if redacted_output.len() > 200 {
            format!("{}...", &redacted_output[..197]).replace("\n", "\\n")
        } else {
            redacted_output.replace("\n", "\\n")
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();

        let log_line = format!(
            "[{}] subagent={} task=\"{}\" model={} success={} output=\"{}\"\n",
            timestamp, subagent_name, task_summary, model_profile, success, output_preview
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.paths.subagent_audit_log_file)
        {
            let _ = file.write_all(log_line.as_bytes());
        }
    }
}
