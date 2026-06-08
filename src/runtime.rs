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
//!   ├─ GoatRuntime::bootstrap(config, paths, warnings)
//!   │     ├─ Brain::new(paths.db_file)
//!   │     ├─ LlmRouter::new(keys)
//!   │     ├─ SwarmRouter::default()
//!   │     ├─ ApprovalGate::new()
//!   │     └─ session resume / UUID create
//!   │
//!   ├─ TUI path → App::from_runtime(runtime) → run_app()
//!   └─ Headless path → headless::run(runtime)
//! ```
//!
//! Future surfaces (web dashboard, Tauri, daemon) follow the same pattern:
//! `GoatRuntime::bootstrap()` once, then pass to the surface-specific loop.

use crate::approval::{ApprovalGate, ApprovalRequest};
use crate::brain::Brain;
use crate::config::Config;
use crate::llm::{LlmRouter, Message};
use crate::mcp::McpManager;
use crate::paths::GoatPaths;
use crate::swarm::SwarmRouter;
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

    // ── Agent components ──────────────────────────────────────────────────────
    /// SQLite brain (memory/session store). `None` if DB is unavailable.
    pub brain: Option<Brain>,
    /// LLM provider router.
    pub llm_router: LlmRouter,
    /// Keyword-based swarm router.
    pub swarm_router: SwarmRouter,
    /// Approval gate for dangerous tools.
    pub approval_gate: ApprovalGate,
    /// MCP server manager.
    pub mcp_manager: McpManager,

    /// Conversation history (sent to LLM each turn).
    pub history: Vec<Message>,

    /// Human-readable provider label (e.g. "openai:gpt-4o-mini").
    pub provider_label: String,
    /// Number of running MCP servers.
    pub mcp_server_count: usize,
}

impl GoatRuntime {
    /// Bootstrap the shared agent runtime from config and paths.
    ///
    /// This is the single place where session resume / UUID creation,
    /// brain connection, provider setup, and gate initialization happen.
    /// Both the TUI and headless mode call this.
    pub fn bootstrap(
        config: Config,
        paths: GoatPaths,
        startup_warnings: Vec<String>,
    ) -> (Self, Vec<String>) {
        let mut boot_log: Vec<String> = Vec::new();

        // ── Brain / DB ────────────────────────────────────────────────────────
        let brain = match Brain::new(&paths.db_file) {
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
        };

        // ── Session ───────────────────────────────────────────────────────────
        let mut session_id = Uuid::new_v4().to_string();
        let mut session_resumed = false;
        let mut history: Vec<Message> = Vec::new();

        if let Some(ref b) = brain {
            match b.get_sessions() {
                Ok(sessions) => {
                    if let Some((latest_id, _)) = sessions.first() {
                        // Resume the most recent session.
                        session_id = latest_id.clone();
                        session_resumed = true;
                        boot_log.push(format!("[SYSTEM] Resumed session: {}", session_id));

                        // Load history for resumed session.
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
                        // Fresh database — create first session.
                        let _ = b.create_session(&session_id, "New Session");
                        boot_log.push(format!("[SYSTEM] Created session: {}", session_id));
                    }
                }
                Err(e) => {
                    boot_log.push(format!("[WARN] Could not query sessions: {}", e));
                }
            }
        }

        // ── Provider / LLM ────────────────────────────────────────────────────
        let llm_router = LlmRouter::new(
            config.keys.openai_api_key.clone(),
            config.keys.groq_api_key.clone(),
        );

        let provider_label = if config.keys.openai_api_key.is_some() {
            "openai:gpt-4o-mini".to_string()
        } else if config.keys.groq_api_key.is_some() {
            "groq:llama3-8b".to_string()
        } else {
            "no provider configured".to_string()
        };

        info!(
            session_id = %session_id,
            provider = %provider_label,
            resumed = session_resumed,
            "runtime bootstrapped"
        );

        boot_log.push(
            "[SECURITY] Approval gate active — bash, write_file, call_subagent require confirmation."
                .to_string(),
        );

        let runtime = GoatRuntime {
            paths,
            config,
            startup_warnings,
            session_id,
            session_resumed,
            brain,
            llm_router,
            swarm_router: SwarmRouter::default(),
            approval_gate: ApprovalGate::new(),
            mcp_manager: McpManager::new(),
            history,
            provider_label,
            mcp_server_count: 0,
        };

        (runtime, boot_log)
    }
}

// ── Approval prompt formatting (shared between TUI and headless) ──────────────

/// Format an approval request as lines of text for display in any surface.
pub fn format_approval_prompt(req: &ApprovalRequest) -> Vec<String> {
    req.display_lines()
}
