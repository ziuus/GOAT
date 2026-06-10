//! Shared agent runtime — usable by TUI, headless, daemon, and future surfaces.
//!
//! `GoatRuntime` holds all state that is independent of how input/output is
//! presented to the user.  The TUI (`App`) and the headless loop both consume
//! a `GoatRuntime` instance so they share identical session, provider, brain,
//! and approval-gate setup without code duplication.
//!
//! # Design
//!
//! ```text
//!  main()
//!   ├─ GoatRuntime::bootstrap(config, paths, warnings, no_brain, profile_override)
//!   │     ├─ ProfileRegistry::from_config()  → profile/chain setup
//!   │     ├─ Brain::new(paths.db_file)        → SQLite (optional via --no-brain)
//!   │     ├─ LlmRouter::from_config(config)   → uses LlmConfig for retry/timeout
//!   │     ├─ SwarmRouter::default()
//!   │     ├─ ApprovalGate::new()
//!   │     └─ session resume / UUID create
//!   │
//!   ├─ TUI path → App::from_runtime(runtime) → run_app()
//!   └─ Headless path → headless::run(runtime)
//! ```
//!
//! Future surfaces (web dashboard, Tauri, daemon) follow the same pattern.

use crate::approval::{ApprovalGate, ApprovalRequest};
use crate::brain::Brain;
use crate::config::Config;
use crate::llm::{LlmRouter, Message};
use crate::mcp::McpManager;
use crate::models::{ModelChain, ProfileRegistry};
use crate::paths::GoatPaths;
use crate::swarm::SwarmRouter;
use crate::task::WorkflowState;
use tracing::info;
use uuid::Uuid;

// ── GoatRuntime ───────────────────────────────────────────────────────────────

/// Shared agent runtime.  Surface-agnostic — contains no TUI or headless I/O.
pub struct GoatRuntime {
    /// Resolved filesystem paths for this session.
    pub paths: GoatPaths,
    /// Loaded configuration.
    pub config: Config,
    /// Non-fatal startup warnings (e.g. insecure config permissions).
    pub startup_warnings: Vec<String>,

    // ── Session ───────────────────────────────────────────────────────────────
    /// Active session identifier (UUID for new sessions, legacy ID for resumed).
    pub session_id: String,
    /// Whether this session was resumed from the brain (true) or is fresh (false).
    pub session_resumed: bool,
    /// Whether brain (SQLite) is disabled via `--no-brain`.
    pub brain_disabled: bool,

    // ── Agent components ──────────────────────────────────────────────────────
    /// SQLite brain (memory/session store).
    /// `None` if DB is unavailable OR if `--no-brain` was passed.
    pub brain: Option<Brain>,
    /// LLM provider router.
    pub llm_router: LlmRouter,
    /// Model profile registry (built-in defaults + user config).
    pub profile_registry: ProfileRegistry,
    /// Keyword-based swarm router.
    pub swarm_router: SwarmRouter,
    /// Approval gate for dangerous tools.
    pub approval_gate: ApprovalGate,
    /// MCP server manager.
    pub mcp_manager: McpManager,
    /// MCP runtime state manager (Phase 3.8).
    pub mcp_runtime: crate::mcp_runtime::McpRuntimeManager,

    /// Conversation history (sent to LLM each turn).
    pub history: Vec<Message>,

    // ── Provider display ──────────────────────────────────────────────────────
    /// Human-readable provider:model label for the currently active entry.
    pub provider_label: String,
    /// Name of the active profile (e.g. "balanced", "coding").
    pub active_profile: String,
    /// The active fallback chain (from the active profile).
    pub model_chain: ModelChain,
    /// Number of running MCP servers.
    pub mcp_server_count: usize,

    // ── Workflow state ────────────────────────────────────────────────────────
    /// Phase 2.5 Agentic Coding workflow state.
    pub workflow: WorkflowState,
    pub tool_registry: crate::tool_registry::ToolRegistry,
    /// Phase 2.7 Subagent Manager.
    pub subagent_manager: crate::subagents::SubagentManager,
    /// Phase 3.6 Selected file context
    pub selected_files: Vec<String>,
    /// Phase 2.8 External Agent Manager.
    pub external_agent_manager: crate::external_agents::ExternalAgentManager,

    // ── Phase 3.9 Managers ───────────────────────────────────────────────────
    pub hooks_manager: crate::hooks::HooksManager,
    pub scheduler_manager: crate::scheduler::SchedulerManager,
    pub job_tracker: crate::jobs::JobTracker,
}

impl GoatRuntime {
    /// Bootstrap the shared agent runtime from config and paths.
    ///
    /// - `no_brain`: when `true`, skip SQLite entirely (ephemeral session).
    /// - `profile_override`: when `Some(name)`, use that profile instead of the config default.
    pub fn bootstrap(
        config: Config,
        paths: GoatPaths,
        startup_warnings: Vec<String>,
        no_brain: bool,
        profile_override: Option<String>,
    ) -> (Self, Vec<String>) {
        let mut boot_log: Vec<String> = Vec::new();

        // ── Profile registry ──────────────────────────────────────────────────
        let profile_registry = ProfileRegistry::from_config(&config.profiles);

        // If a profile was specified on the CLI, validate it early.
        let active_profile = if let Some(ref name) = profile_override {
            if profile_registry.profiles.contains_key(name.as_str()) {
                name.clone()
            } else {
                boot_log.push(format!(
                    "[WARN] Profile '{}' not found — falling back to default '{}'",
                    name, profile_registry.default_profile
                ));
                profile_registry.default_profile.clone()
            }
        } else {
            profile_registry.default_profile.clone()
        };

        let model_chain = profile_registry.resolve(&active_profile).1.clone();

        boot_log.push(format!(
            "[SYSTEM] Profile: {} | Chain: {}{}",
            active_profile,
            model_chain.primary_display(),
            if model_chain.len() > 1 {
                format!(" → {}", model_chain.fallback_display())
            } else {
                String::new()
            }
        ));

        // ── Brain / DB ────────────────────────────────────────────────────────
        let brain = if no_brain {
            boot_log
                .push("[SYSTEM] Brain disabled (--no-brain) — running without memory.".to_string());
            None
        } else {
            match Brain::new(&paths.db_file) {
                Ok(b) => {
                    boot_log.push(format!(
                        "[SYSTEM] Brain connected: {}",
                        paths.db_file.display()
                    ));
                    Some(b)
                }
                Err(e) => {
                    boot_log.push(format!(
                        "[WARN] Brain (SQLite) unavailable — running without memory: {}",
                        e
                    ));
                    None
                }
            }
        };

        // ── Session ───────────────────────────────────────────────────────────
        let mut session_id = Uuid::new_v4().to_string();
        let mut session_resumed = false;
        let mut history: Vec<Message> = Vec::new();

        if let Some(ref b) = brain {
            match b.get_sessions() {
                Ok(sessions) => {
                    if let Some((latest_id, _)) = sessions.first() {
                        session_id = latest_id.clone();
                        session_resumed = true;
                        boot_log.push(format!("[SYSTEM] Resumed session: {}", session_id));

                        match b.load_session_history(&session_id) {
                            Ok(loaded) => {
                                for (role, content) in loaded {
                                    history.push(Message {
                                        role,
                                        content: Some(content),
                                        tool_calls: None,
                                        tool_call_id: None,
                                    });
                                }
                                if !history.is_empty() {
                                    boot_log.push(format!(
                                        "[SYSTEM] Loaded {} history messages.",
                                        history.len()
                                    ));
                                }
                            }
                            Err(e) => {
                                boot_log
                                    .push(format!("[WARN] Could not load session history: {}", e));
                            }
                        }
                    } else {
                        let _ = b.create_session(&session_id, "New Session");
                        boot_log.push(format!("[SYSTEM] Created session: {}", session_id));
                    }
                }
                Err(e) => {
                    boot_log.push(format!("[WARN] Could not query sessions: {}", e));
                }
            }
        } else if no_brain {
            boot_log.push(format!("[SYSTEM] Ephemeral session: {}", session_id));
        }

        // ── Provider / LLM ────────────────────────────────────────────────────
        let llm_router = LlmRouter::from_config(&config);

        // Build the display label from the first available entry in the chain.
        let provider_label = model_chain
            .entries
            .iter()
            .find(|e| llm_router.is_provider_available(&e.provider))
            .map(|e| e.display())
            .unwrap_or_else(|| "no provider configured".to_string());

        info!(
            session_id = %session_id,
            provider = %provider_label,
            profile = %active_profile,
            chain_len = model_chain.len(),
            resumed = session_resumed,
            no_brain,
            "runtime bootstrapped"
        );

        boot_log.push(
            "[SECURITY] Approval gate active — bash, write_file, call_subagent require confirmation."
                .to_string(),
        );

        let mut external_agent_manager = crate::external_agents::ExternalAgentManager::new(
            paths.external_agent_audit_log_file.clone(),
            paths.data_dir.clone(),
        );
        external_agent_manager.detect_all(&config);

        let runtime = GoatRuntime {
            subagent_manager: crate::subagents::SubagentManager::new(paths.clone()),
            external_agent_manager,
            paths: paths.clone(),
            config: config.clone(),
            startup_warnings,
            session_id,
            session_resumed,
            brain_disabled: no_brain,
            brain,
            llm_router,
            profile_registry,
            swarm_router: SwarmRouter::default(),
            approval_gate: ApprovalGate::new(),
            mcp_manager: McpManager::new(),
            mcp_runtime: {
                let mut mgr = crate::mcp_runtime::McpRuntimeManager::new();
                mgr.init_from_config(&config);
                mgr
            },
            history,
            provider_label,
            active_profile,
            model_chain,
            mcp_server_count: 0,
            workflow: WorkflowState::default(),
            tool_registry: crate::tool_registry::ToolRegistry::new(),
            selected_files: Vec::new(),
            hooks_manager: crate::hooks::HooksManager::new(config.hooks.clone(), paths.clone()),
            scheduler_manager: crate::scheduler::SchedulerManager::new(
                config.scheduler.clone(),
                paths.clone(),
            ),
            job_tracker: crate::jobs::JobTracker::new(),
        };

        (runtime, boot_log)
    }

    pub fn sync_mcp_tools(&mut self) {
        for (srv_name, server) in self.mcp_manager.running_servers_metadata() {
            if let Some(mrs) = self.mcp_runtime.get_mut(&srv_name) {
                mrs.state = crate::mcp_runtime::McpServerState::Running;
                mrs.pid = server.client.pid();
                if mrs.started_at.is_none() {
                    mrs.started_at = Some(std::time::SystemTime::now());
                }
                mrs.discovered_tools = server.tools.clone();
            }

            for tool in &server.tools {
                if let (Some(name), Some(desc)) = (
                    tool.get("name").and_then(|v| v.as_str()),
                    tool.get("description").and_then(|v| v.as_str()),
                ) {
                    self.tool_registry
                        .register(crate::tool_registry::ToolMetadata {
                            name: format!("{}_{}", srv_name, name),
                            description: format!("[MCP: {}] {}", srv_name, desc),
                            category: crate::tool_registry::ToolCategory::Mcp,
                            risk_level: crate::approval::RiskLevel::High, // MCP tools are untrusted by default
                            requires_approval: true,
                            read_only: false,
                            available_in_tui: true,
                            available_in_headless: true,
                            available_in_agent: true,
                            permission_group: "mcp".to_string(),
                        });
                }
            }
        }

        // Handle stopped servers
        let running_names = self.mcp_manager.running_servers();
        for mut mrs in self.mcp_runtime.list_all_mut() {
            if !running_names.contains(&mrs.name) {
                if mrs.state == crate::mcp_runtime::McpServerState::Running {
                    mrs.state = crate::mcp_runtime::McpServerState::Stopped;
                    mrs.pid = None;
                }
            }
        }
    }
    // ── Runtime mutations ─────────────────────────────────────────────────────

    /// Switch to a different profile at runtime.
    ///
    /// Returns `Ok(())` on success, `Err(reason)` if the profile name is unknown.
    pub fn switch_profile(&mut self, name: &str) -> Result<(), String> {
        if let Some(chain) = self.profile_registry.profiles.get(name) {
            self.active_profile = name.to_string();
            self.model_chain = chain.clone();
            // Update provider_label to reflect new chain's primary entry.
            self.provider_label = self
                .model_chain
                .entries
                .iter()
                .find(|e| self.llm_router.is_provider_available(&e.provider))
                .map(|e| e.display())
                .unwrap_or_else(|| "no provider configured".to_string());
            info!(profile = %name, provider = %self.provider_label, "profile switched");
            Ok(())
        } else {
            let available = self.profile_registry.profile_names().join(", ");
            Err(format!(
                "profile '{}' not found. Available: {}",
                name, available
            ))
        }
    }

    /// Create a new session and switch to it.
    ///
    /// Saves the current session (already persisted via log_interaction) then
    /// creates a fresh UUID session in the brain (if available).
    /// Returns the new session ID.
    pub fn create_new_session(&mut self) -> String {
        use uuid::Uuid;
        let new_id = Uuid::new_v4().to_string();
        self.session_id = new_id.clone();
        self.session_resumed = false;
        self.history.clear();

        if let Some(ref brain) = self.brain {
            let _ = brain.create_session(&new_id, "New Session");
        }
        info!(session_id = %new_id, "new session created");
        new_id
    }
}

// ── Approval prompt formatting (shared between TUI and headless) ──────────────

/// Format an approval request as lines of text for display in any surface.
pub fn format_approval_prompt(req: &ApprovalRequest) -> Vec<String> {
    req.display_lines()
}
