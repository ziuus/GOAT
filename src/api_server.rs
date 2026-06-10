use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tower_http::cors::{Any, CorsLayer};

use crate::runtime::GoatRuntime;

pub struct ApiState {
    pub runtime: Arc<Mutex<GoatRuntime>>,
    pub token: String,
    pub auth_required: bool,
}

pub async fn start_server(
    host: &str,
    port: u16,
    auth_required: bool,
    token: String,
    runtime: Arc<Mutex<GoatRuntime>>,
) -> anyhow::Result<()> {
    let state = Arc::new(ApiState {
        runtime,
        token,
        auth_required,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/v1/status", get(status_handler))
        .route("/v1/jobs", get(jobs_list_handler))
        .route("/v1/jobs/:id", get(job_detail_handler))
        .route("/v1/hooks", get(hooks_list_handler))
        .route("/v1/schedule", get(schedule_list_handler))
        .route("/v1/mcp/status", get(mcp_status_handler))
        .route("/v1/command", post(command_handler))
        .route("/v1/logs/recent", get(logs_handler))
        .route("/v1/events/stream", get(events_stream_handler))
        .route("/v1/approvals", get(approvals_list_handler))
        .route("/v1/approvals/history", get(approvals_history_handler))
        .route("/v1/approvals/:id", get(approval_detail_handler))
        .route("/v1/approvals/:id/approve", post(approval_approve_handler))
        .route("/v1/approvals/:id/deny", post(approval_deny_handler))
        .route("/v1/chat", post(chat_handler))
        .route(
            "/v1/sessions",
            get(sessions_list_handler).post(session_create_handler),
        )
        .route("/v1/sessions/:id", get(session_detail_handler))
        .route("/v1/repo/tree", get(repo_tree_handler))
        .route("/v1/repo/file", get(repo_file_handler))
        .route("/v1/diffs", get(diffs_handler))
        .route("/v1/context", get(context_get_handler))
        .route("/v1/context/add", post(context_add_handler))
        .route("/v1/context/remove", post(context_remove_handler))
        .route("/v1/context/clear", post(context_clear_handler))
        .route("/v1/audit", get(audit_handler))
        .route("/v1/audit/recent", get(audit_recent_handler))
        .route("/v1/learning/status", get(learning_status_handler))
        .route("/v1/learning/candidates", get(learning_candidates_handler))
        .route(
            "/v1/learning/candidates/:id",
            get(learning_candidate_detail_handler),
        )
        .route(
            "/v1/learning/candidates/:id/accept",
            post(learning_candidate_accept_handler),
        )
        .route(
            "/v1/learning/candidates/:id/reject",
            post(learning_candidate_reject_handler),
        )
        .route("/v1/learning/extract", post(learning_extract_handler))
        .route("/v1/learning/summary", get(learning_summary_handler))
        .route("/v1/memory/galaxy", get(memory_galaxy_handler))
        .route("/v1/studio", get(studio_handler))
        .route("/v1/studio/drafts", get(studio_drafts_handler))
        .route("/v1/studio/drafts/:id", get(studio_draft_detail_handler))
        .route("/v1/studio/prompt", post(studio_prompt_handler))
        .route("/v1/studio/profiles", get(studio_profiles_handler))
        .route("/v1/studio/compare", post(studio_compare_handler))
        .route("/v1/studio/skills/draft", post(studio_skills_draft_handler))
        .route(
            "/v1/studio/skills/create",
            post(studio_skills_create_handler),
        )
        .route("/v1/studio/agents/draft", post(studio_agents_draft_handler))
        .route(
            "/v1/studio/agents/create",
            post(studio_agents_create_handler),
        )
        .route(
            "/v1/studio/workflows/draft",
            post(studio_workflows_draft_handler),
        )
        .route(
            "/v1/studio/workflows/create",
            post(studio_workflows_create_handler),
        )
        .route("/v1/skills/sources", get(skills_sources_handler))
        .route("/v1/skills/installed", get(skills_installed_handler))
        .route(
            "/v1/skills/provenance/:name",
            get(skills_provenance_handler),
        )
        .route(
            "/v1/skills/remote/search",
            get(skills_remote_search_handler),
        )
        .route("/v1/skills/remote/:id", get(skills_remote_detail_handler))
        .route(
            "/v1/skills/remote/:id/audit",
            post(skills_remote_audit_handler),
        )
        .route(
            "/v1/skills/remote/:id/install",
            post(skills_remote_install_handler),
        )
        .route("/v1/skills/:name/uninstall", post(skills_uninstall_handler))
        .route("/v1/recipes", get(recipes_list_handler))
        .route("/v1/recipes/built-in", get(recipes_builtin_handler))
        .route("/v1/recipes/installed", get(recipes_installed_handler))
        .route("/v1/recipes/drafts", get(recipes_drafts_handler))
        .route("/v1/recipes/:id", get(recipes_detail_handler))
        .route("/v1/recipes/:id/audit", post(recipes_audit_handler))
        .route("/v1/recipes/:id/install", post(recipes_install_handler))
        .route("/v1/recipes/:name/enable", post(recipes_enable_handler))
        .route("/v1/recipes/:name/disable", post(recipes_disable_handler))
        .route(
            "/v1/recipes/:name/uninstall",
            post(recipes_uninstall_handler),
        )
        .route(
            "/v1/recipes/:name/provenance",
            get(recipes_provenance_handler),
        )
        .route(
            "/v1/recipes/from-memory/:candidate_id",
            post(recipes_from_memory_handler),
        )
        .route("/v1/recipes/:name/activate", post(recipes_activate_handler))
        .route(
            "/v1/recipes/:name/deactivate",
            post(recipes_deactivate_handler),
        )
        .route("/v1/recipes/:name/run", post(recipes_run_handler))
        .route("/v1/recipes/:name/plan", get(recipes_plan_handler))
        .route("/v1/recipes/runs", get(recipes_runs_list_handler))
        .route("/v1/recipes/:name/runs", get(recipes_runs_handler))
        .route("/v1/recipes/:name/status", get(recipes_status_handler))
        .route("/v1/brain/status", get(brain_status_handler))
        .route("/v1/brain/index", post(brain_index_handler))
        .route("/v1/brain/reindex", post(brain_index_handler))
        .route("/v1/brain/search", get(brain_search_handler))
        .route("/v1/brain/recall", get(brain_recall_handler))
        .route("/v1/brain/related/:id", get(brain_related_handler))
        .route("/v1/brain/sources", get(brain_sources_handler))
        .route("/v1/brain/privacy", get(brain_privacy_handler))
        .route(
            "/v1/brain/embeddings/status",
            get(brain_embeddings_status_handler),
        )
        .route(
            "/v1/brain/embeddings/rebuild",
            post(brain_embeddings_rebuild_handler),
        )
        .route("/v1/agent-templates", get(agent_templates_list_handler))
        .route(
            "/v1/agent-templates/:id/draft",
            post(agent_templates_draft_handler),
        )
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("[DAEMON] API server listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

fn check_auth(
    headers: &HeaderMap,
    state: &ApiState,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if !state.auth_required {
        return Ok(());
    }
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", state.token) {
                return Ok(());
            }
        }
    }
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized" })),
    ))
}

async fn health_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    })))
}

// ── Learning endpoints ────────────────────────────────────────────────────────

async fn learning_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "enabled": true, "status": "active" })))
}

async fn learning_candidates_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!([]))) // Mock empty candidates
}

async fn learning_candidate_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Err((StatusCode::NOT_FOUND, Json(json!({ "error": "not found" }))))
}

async fn learning_candidate_accept_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "accepted", "id": id })))
}

async fn learning_candidate_reject_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "rejected", "id": id })))
}

async fn learning_extract_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "extract_started" })))
}

async fn learning_summary_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({
        "total_candidates": 0,
        "accepted": 0,
        "rejected": 0,
        "pending": 0,
    })))
}

async fn memory_galaxy_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "memories": [] })))
}

async fn status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let jobs_count = rt.job_tracker.list_jobs().len();
    let scheduled_count = rt.scheduler_manager.list_jobs().len();

    Ok(Json(json!({
        "session_id": rt.session_id,
        "active_profile": rt.active_profile,
        "provider": rt.provider_label,
        "jobs_count": jobs_count,
        "scheduled_jobs_count": scheduled_count,
    })))
}

async fn jobs_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let jobs = rt.job_tracker.list_jobs();
    Ok(Json(json!(jobs)))
}

async fn job_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    if let Some(job) = rt.job_tracker.get_job(&id) {
        Ok(Json(json!(job)))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Job not found" })),
        ))
    }
}

async fn hooks_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let hooks = rt.hooks_manager.list_hooks_info();
    Ok(Json(json!(hooks)))
}

async fn schedule_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let jobs = rt.scheduler_manager.list_jobs();
    Ok(Json(json!(jobs)))
}

async fn mcp_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let servers: Vec<serde_json::Value> = rt
        .mcp_runtime
        .list_all()
        .iter()
        .map(|s| {
            json!({
                "name": s.name,
                "state": format!("{:?}", s.state),
                "tool_count": s.discovered_tools.len(),
                "pid": s.pid,
            })
        })
        .collect();
    Ok(Json(json!({
        "servers": servers,
    })))
}

#[derive(Deserialize)]
struct CommandReq {
    command: String,
}

async fn command_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<CommandReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;

    let cmd = req.command.trim();

    // Safe read-only commands
    let safe_commands = [
        "/status",
        "/jobs",
        "/jobs list",
        "/schedule",
        "/schedule list",
        "/hooks",
        "/hooks list",
        "/mcp status",
        "/mcp list",
        "/repo",
        "/changes",
        "/context show",
        "/checkpoint list",
        "/logs recent",
    ];

    if safe_commands.contains(&cmd) || cmd.starts_with("/commands search") {
        if cmd == "/status" {
            return status_handler(headers, State(state)).await;
        }
        if cmd == "/jobs" || cmd == "/jobs list" {
            return jobs_list_handler(headers, State(state)).await;
        }
        if cmd == "/schedule" || cmd == "/schedule list" {
            return schedule_list_handler(headers, State(state)).await;
        }
        if cmd == "/hooks" || cmd == "/hooks list" {
            return hooks_list_handler(headers, State(state)).await;
        }
        if cmd == "/mcp status" || cmd == "/mcp list" {
            return mcp_status_handler(headers, State(state)).await;
        }
        if cmd == "/repo" {
            return repo_tree_handler(headers, State(state)).await;
        }
        if cmd == "/changes" {
            return diffs_handler(headers, State(state)).await;
        }
        if cmd == "/context show" {
            return context_get_handler(headers, State(state)).await;
        }
        if cmd == "/logs recent" {
            return logs_handler(headers, State(state)).await;
        }
        return Ok(Json(
            json!({ "message": format!("Safe command executed: {}", cmd) }),
        ));
    }

    // Attempt to parse as risky command
    let rt = state.runtime.lock().await;

    // Check if it's unknown/unsupported
    let known_risky = [
        "/bash",
        "/write",
        "/git",
        "/commit",
        "/mcp start",
        "/rollback",
    ];
    if !known_risky.iter().any(|r| cmd.starts_with(r)) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Unknown or unsupported command" })),
        ));
    }

    // Create approval request for risky command
    let req_obj = crate::approval::ApprovalRequest {
        tool_name: "dashboard_command".to_string(),
        action_summary: cmd.to_string(),
        risk_level: crate::approval::RiskLevel::High,
        explanation: Some("Risky command initiated from dashboard".to_string()),
        working_directory: None,
    };

    let (pending, _) = rt.approval_queue.add(req_obj, "dashboard").await;

    // Broadcast event
    let _ = rt
        .event_bus
        .push(crate::events::GoatEvent::new(
            "approval_requested",
            crate::events::EventSeverity::Warning,
            &format!("Dashboard command requires approval: {}", cmd),
            Some(json!({ "id": pending.id, "source": pending.source })),
        ))
        .await;

    Err((
        StatusCode::FORBIDDEN,
        Json(json!({
            "approval_required": true,
            "approval_id": pending.id,
            "risk": "high",
            "message": "Approval required to execute this command.",
        })),
    ))
}

async fn logs_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;

    let rt = state.runtime.lock().await;
    let log_path = rt.paths.data_dir.join("daemon.log");

    if let Ok(content) = std::fs::read_to_string(&log_path) {
        // Redact simple tokens if necessary (basic)
        let safe_content = content.replace(&state.token, "[REDACTED]");
        let lines: Vec<&str> = safe_content.lines().rev().take(100).collect();
        Ok(Json(json!({ "logs": lines })))
    } else {
        Ok(Json(json!({ "logs": [] })))
    }
}

async fn events_stream_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    State(state): State<Arc<ApiState>>,
) -> Result<
    axum::response::sse::Sse<
        impl tokio_stream::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>,
    >,
    (StatusCode, Json<serde_json::Value>),
> {
    if state.auth_required {
        let auth_valid = if let Some(auth_header) = headers.get("Authorization") {
            auth_header.to_str().unwrap_or("") == format!("Bearer {}", state.token)
        } else if let Some(token) = query.get("token") {
            token == &state.token
        } else {
            false
        };

        if !auth_valid {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" })),
            ));
        }
    }

    let rt = state.runtime.lock().await;
    let rx = rt.event_bus.subscribe();
    drop(rt);

    let stream = tokio_stream::wrappers::BroadcastStream::new(rx)
        .filter_map(|res| res.ok())
        .map(|evt| {
            let json = serde_json::to_string(&evt).unwrap_or_default();
            Ok(axum::response::sse::Event::default()
                .event(evt.kind)
                .data(json))
        });

    Ok(axum::response::sse::Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new()))
}

async fn approvals_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let approvals = rt.approval_queue.list().await;
    Ok(Json(json!({ "approvals": approvals })))
}

async fn approval_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    if let Some(approval) = rt.approval_queue.get(&id).await {
        Ok(Json(json!({ "approval": approval })))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))))
    }
}

async fn approval_approve_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let ok = rt.approval_queue.resolve(&id, 'y').await;

    if ok {
        let _ = rt
            .event_bus
            .push(crate::events::GoatEvent::new(
                "approval_resolved",
                crate::events::EventSeverity::Info,
                &format!("Approval {} resolved (approved)", id),
                Some(json!({ "id": id, "decision": "y" })),
            ))
            .await;
    }

    Ok(Json(json!({ "success": ok })))
}

async fn approval_deny_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let ok = rt.approval_queue.resolve(&id, 'n').await;

    if ok {
        let _ = rt
            .event_bus
            .push(crate::events::GoatEvent::new(
                "approval_resolved",
                crate::events::EventSeverity::Info,
                &format!("Approval {} resolved (denied)", id),
                Some(json!({ "id": id, "decision": "n" })),
            ))
            .await;
    }

    Ok(Json(json!({ "success": ok })))
}

// ── Phase 4.3 Chat ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ChatReq {
    message: String,
    session_id: Option<String>,
    mode: Option<String>,
    selected_context_files: Option<Vec<String>>,
}

async fn chat_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<ChatReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;

    let msg = req.message.trim().to_string();
    if msg.starts_with('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Commands through chat not yet supported via API" })),
        ));
    }

    let job_id = uuid::Uuid::new_v4().to_string();
    let session_id = req
        .session_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let mode = req.mode.clone().unwrap_or_else(|| "chat".to_string());

    let job = crate::jobs::BackgroundJob {
        id: job_id.clone(),
        r#type: "chat".to_string(),
        status: "queued".to_string(),
        started_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
        finished_at: None,
        output_preview: None,
        error: None,
        approval_status: None,
    };

    let (config, chain, sys_prompt, db_file, event_bus) = {
        let mut rt_guard = state.runtime.lock().await;
        rt_guard.job_tracker.add_job(job);

        let evt = crate::events::GoatEvent::new(
            "job_created",
            crate::events::EventSeverity::Info,
            &format!("Chat job {} created", job_id),
            Some(json!({"job_id": job_id, "session_id": session_id})),
        );
        let bus = rt_guard.event_bus.clone();
        // Fire and forget is okay, but we await to ensure order if we want, or just spawn.

        let sys_prompt = rt_guard
            .swarm_router
            .route(&msg)
            .profile
            .system_prompt
            .to_string();
        let (_, chain) = rt_guard.profile_registry.resolve("balanced");
        (
            rt_guard.config.clone(),
            chain.clone(),
            sys_prompt,
            rt_guard.paths.db_file.clone(),
            bus,
        )
    };
    event_bus
        .push(crate::events::GoatEvent::new(
            "job_created",
            crate::events::EventSeverity::Info,
            &format!("Chat job {} created", job_id),
            Some(json!({"job_id": job_id, "session_id": session_id})),
        ))
        .await;

    // Clone state for async task
    let state_clone = state.clone();
    let msg_clone = msg.clone();
    let sid_clone = session_id.clone();
    let job_id_clone = job_id.clone();

    tokio::spawn(async move {
        // Emit job_started
        {
            let mut rt_guard = state_clone.runtime.lock().await;
            if let Some(j) = rt_guard.job_tracker.get_job_mut(&job_id_clone) {
                j.status = "running".to_string();
            }
            rt_guard
                .event_bus
                .push(crate::events::GoatEvent::new(
                    "job_started",
                    crate::events::EventSeverity::Info,
                    &format!("Chat job {} started", job_id_clone),
                    Some(json!({"job_id": job_id_clone, "session_id": sid_clone})),
                ))
                .await;
        }

        let brain = crate::brain::Brain::new(&db_file).ok();
        let mut history = vec![];

        if let Some(ref b) = brain {
            let _ = b.create_session(&sid_clone, "Dashboard Chat Session");
            let _ = b.log_interaction(&sid_clone, "user", &msg_clone);
            if let Ok(hist) = b.load_session_history(&sid_clone) {
                history = hist
                    .into_iter()
                    .map(|(r, c)| crate::llm::Message {
                        role: r,
                        content: Some(c),
                        tool_calls: None,
                        tool_call_id: None,
                    })
                    .collect();
            }
        }

        if history.is_empty() {
            history.push(crate::llm::Message {
                role: "user".to_string(),
                content: Some(msg_clone.clone()),
                tool_calls: None,
                tool_call_id: None,
            });
        }

        let mut sys_prompt_actual = sys_prompt.clone();
        if mode == "act" {
            sys_prompt_actual.push_str("\n\n<workflow_mode>\nCURRENT MODE: ACT\nYou may propose file patches and run safe commands.\n</workflow_mode>");
        }

        let mut routed_history = vec![crate::llm::Message {
            role: "system".to_string(),
            content: Some(sys_prompt_actual),
            tool_calls: None,
            tool_call_id: None,
        }];
        routed_history.extend(history);

        let llm_router = crate::llm::LlmRouter::from_config(&config);

        let res = llm_router
            .completion_with_fallback(&chain, routed_history, None)
            .await;

        let mut is_approval_req = false;
        let mut final_content = String::new();
        let mut error_msg = None;

        match res {
            Ok((response, _)) => {
                if let Some(c) = response.content {
                    final_content = c.clone();
                    if let Some(ref b) = brain {
                        let _ = b.log_interaction(&sid_clone, "assistant", &c);
                    }

                    let rt_guard = state_clone.runtime.lock().await;
                    rt_guard.event_bus.push(crate::events::GoatEvent::new(
                        "chat_message",
                        crate::events::EventSeverity::Info,
                        "New message",
                        Some(json!({"job_id": job_id_clone, "session_id": sid_clone, "content": c})),
                    )).await;
                }

                // If mode act and we would have passed tools, we trigger approval partial flow.
                if mode == "act" {
                    is_approval_req = true;
                }
            }
            Err(e) => {
                error_msg = Some(e.to_string());
            }
        }

        let mut pending_approval = None;
        let mut final_status_val = String::new();
        {
            let mut rt_guard = state_clone.runtime.lock().await;
            if let Some(j) = rt_guard.job_tracker.get_job_mut(&job_id_clone) {
                j.finished_at = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        .to_string(),
                );
                if let Some(e) = error_msg {
                    j.status = "failed".to_string();
                    j.error = Some(e);
                } else if is_approval_req {
                    j.status = "approval_required".to_string();
                    j.output_preview = Some(final_content.clone());

                    let req = crate::approval::ApprovalRequest {
                        tool_name: "async_chat_tools".to_string(),
                        action_summary: "LLM requested tool execution".to_string(),
                        risk_level: crate::approval::RiskLevel::High,
                        explanation: Some(
                            "Manual approval required for web async tools in Phase 4.5".to_string(),
                        ),
                        working_directory: Some(
                            std::env::current_dir()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                        ),
                    };
                    pending_approval = Some(req);
                } else {
                    j.status = "completed".to_string();
                    j.output_preview = Some(final_content);
                }
                final_status_val = j.status.clone();
            }
        }

        if let Some(req) = pending_approval {
            let aq = state_clone.runtime.lock().await.approval_queue.clone();
            aq.add(req, "dashboard").await;
        }

        let rt_guard = state_clone.runtime.lock().await;
        rt_guard.event_bus.push(crate::events::GoatEvent::new(
            if final_status_val == "completed" { "job_completed" } else if final_status_val == "failed" { "job_failed" } else { "approval_required" },
            crate::events::EventSeverity::Info,
            &format!("Chat job {} finished", job_id_clone),
            Some(json!({"job_id": job_id_clone, "session_id": sid_clone, "status": final_status_val})),
        )).await;
    });

    Ok(Json(json!({
        "status": "queued",
        "job_id": job_id,
        "session_id": session_id,
        "message": "Chat job queued successfully.",
    })))
}

// ── Phase 4.3 Sessions ────────────────────────────────────────────────────

async fn sessions_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    if let Some(brain) = &rt.brain {
        if let Ok(records) = brain.get_session_records() {
            let sessions: Vec<_> = records
                .into_iter()
                .map(|s| {
                    json!({
                        "id": s.id,
                        "title": s.title,
                        "created_at": s.created_at,
                        "updated_at": s.updated_at
                    })
                })
                .collect();
            return Ok(Json(json!({ "sessions": sessions })));
        }
    }
    Ok(Json(json!({ "sessions": [] })))
}

async fn session_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    if let Some(brain) = &rt.brain {
        if let Ok(history) = brain.load_session_history(&id) {
            let messages: Vec<_> = history
                .into_iter()
                .map(|(r, c)| {
                    json!({
                        "role": r,
                        "content": c
                    })
                })
                .collect();
            return Ok(Json(json!({ "id": id, "history": messages })));
        }
    }
    Err((
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Session not found" })),
    ))
}

async fn session_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let new_id = uuid::Uuid::new_v4().to_string();
    if let Some(brain) = &rt.brain {
        let _ = brain.create_session(&new_id, "New Dashboard Session");
        return Ok(Json(
            json!({ "id": new_id, "title": "New Dashboard Session" }),
        ));
    }
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Brain disabled" })),
    ))
}

// ── Phase 4.3 Repo Explorer ───────────────────────────────────────────────

async fn repo_tree_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let root_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let scanner = crate::repo_map::RepoMapScanner::new(root_dir.clone()).with_no_symbols();
    if let Ok(repo_map) = scanner.scan() {
        return Ok(Json(json!(repo_map)));
    }
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Failed to scan repo" })),
    ))
}

#[derive(Deserialize)]
struct RepoFileQuery {
    path: String,
}

async fn repo_file_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<RepoFileQuery>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let root_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let p = root_dir.join(&query.path);

    // Safety check: must be inside project and not look like a secret
    if !p.starts_with(&root_dir) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Path outside project" })),
        ));
    }
    if crate::repo_map::looks_like_secret_file(&p) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Refusing to read potential secret file" })),
        ));
    }

    if let Ok(content) = std::fs::read_to_string(&p) {
        let redacted = crate::approval::redact_secrets(&content);
        Ok(Json(json!({ "path": query.path, "content": redacted })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "File not found or not UTF-8" })),
        ))
    }
}

// ── Phase 4.3 Diffs ───────────────────────────────────────────────────────

async fn diffs_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let root_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    // Attempt git diff
    let out = std::process::Command::new("git")
        .args(["-C", &root_dir.to_string_lossy(), "diff"])
        .output();

    if let Ok(o) = out {
        let diff_str = String::from_utf8_lossy(&o.stdout);
        // Truncate if too huge
        let truncated = if diff_str.len() > 100_000 {
            format!("{}\n... (truncated)", &diff_str[..100_000])
        } else {
            diff_str.to_string()
        };
        Ok(Json(json!({ "diff": truncated })))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Git diff failed" })),
        ))
    }
}

// ── Phase 4.3 Context API ─────────────────────────────────────────────────

async fn context_get_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(json!({ "selected_files": rt.selected_files })))
}

#[derive(Deserialize)]
struct ContextAddReq {
    path: String,
}

async fn context_add_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<ContextAddReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let root_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let p = root_dir.join(&req.path);
    if crate::repo_map::looks_like_secret_file(&p) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Cannot add secret files to context" })),
        ));
    }
    if !rt.selected_files.contains(&req.path) {
        rt.selected_files.push(req.path.clone());
    }
    Ok(Json(
        json!({ "status": "added", "selected_files": rt.selected_files }),
    ))
}

async fn context_remove_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<ContextAddReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.selected_files.retain(|f| f != &req.path);
    Ok(Json(
        json!({ "status": "removed", "selected_files": rt.selected_files }),
    ))
}

async fn context_clear_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.selected_files.clear();
    Ok(Json(json!({ "status": "cleared" })))
}

async fn approvals_history_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let history = rt.approval_queue.history().await;
    Ok(Json(json!({ "history": history })))
}

#[derive(Deserialize)]
struct AuditQuery {
    category: Option<String>,
}

async fn audit_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<AuditQuery>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;

    let target_file = match query.category.as_deref() {
        Some("tool") => rt.paths.tool_audit_log_file.clone(),
        Some("scheduler") => rt.paths.data_dir.join("scheduler-audit.log"),
        _ => rt.paths.tool_audit_log_file.clone(), // Default to tool audit
    };

    if let Ok(content) = std::fs::read_to_string(&target_file) {
        let safe_content = crate::approval::redact_secrets(&content);
        let safe_content = safe_content.replace(&state.token, "[REDACTED]");
        let lines: Vec<&str> = safe_content.lines().rev().take(500).collect();
        Ok(Json(json!({ "audit": lines })))
    } else {
        Ok(Json(json!({ "audit": [] })))
    }
}

async fn audit_recent_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;

    let mut recent_lines = Vec::new();

    // Read tool audit
    if let Ok(content) = std::fs::read_to_string(&rt.paths.tool_audit_log_file) {
        let safe_content = crate::approval::redact_secrets(&content);
        let safe_content = safe_content.replace(&state.token, "[REDACTED]");
        recent_lines.extend(safe_content.lines().rev().take(50).map(|s| s.to_string()));
    }

    Ok(Json(json!({ "audit": recent_lines })))
}

// ── Phase 5.3 Studio ────────────────────────────────────────────────────────

async fn studio_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "active", "version": "0.1.0" })))
}

async fn studio_drafts_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let sm = crate::studio::StudioManager::new();
    let drafts = sm.list_drafts();
    Ok(Json(json!({ "drafts": drafts })))
}

async fn studio_draft_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let sm = crate::studio::StudioManager::new();
    if let Some(draft) = sm.get_draft(&id) {
        Ok(Json(json!({ "draft": draft })))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))))
    }
}

#[derive(Deserialize)]
struct StudioPromptReq {
    prompt: String,
    profile: Option<String>,
    mode: Option<String>,
    context_files: Option<Vec<String>>,
    save_as: Option<String>,
}

async fn studio_prompt_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<StudioPromptReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;

    // Add simple implementation wrapping around LLM
    let rt = state.runtime.lock().await;
    let _ = req.profile.clone();
    let _ = req.mode.clone();
    let _ = req.context_files.clone();
    let _ = req.save_as.clone();

    // Mock implementation for Phase 5.3 as partial endpoint without full wiring
    Ok(Json(
        json!({ "output": format!("Simulated response for: {}", req.prompt) }),
    ))
}

async fn studio_profiles_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let names: Vec<String> = rt.profile_registry.profiles.keys().cloned().collect();
    Ok(Json(json!({ "profiles": names })))
}

async fn studio_compare_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "status": "partial", "message": "Compare mock response" }),
    ))
}

async fn studio_skills_draft_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let sm = crate::studio::StudioManager::new();
    let draft = sm.save_draft(
        crate::studio::DraftType::Skill,
        json!({"name": "Draft Skill"}),
    );
    Ok(Json(json!({ "draft": draft })))
}

async fn studio_skills_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "created" })))
}

async fn studio_agents_draft_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let sm = crate::studio::StudioManager::new();
    let draft = sm.save_draft(
        crate::studio::DraftType::Agent,
        json!({"name": "Draft Agent"}),
    );
    Ok(Json(json!({ "draft": draft })))
}

async fn studio_agents_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "created" })))
}

async fn studio_workflows_draft_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let sm = crate::studio::StudioManager::new();
    let draft = sm.save_draft(
        crate::studio::DraftType::Workflow,
        json!({"name": "Draft Workflow"}),
    );
    Ok(Json(json!({ "draft": draft })))
}

async fn studio_workflows_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "created" })))
}

// ── Skills Marketplace Handlers ──────────────────────────────────────────────

async fn skills_sources_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "sources": ["local", "learned", "studio_draft", "remote_marketplace"] }),
    ))
}

async fn skills_installed_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "installed": [] })))
}

async fn skills_provenance_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "name": name, "provenance": "local" })))
}

async fn skills_remote_search_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "results": [] })))
}

async fn skills_remote_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "name": "dummy_skill" })))
}

async fn skills_remote_audit_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "id": id, "audit": { "risk_level": "low", "warnings": [], "recommended_action": "safe_to_install" } }),
    ))
}

async fn skills_remote_install_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "status": "approval_required" })))
}

async fn skills_update_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "name": name, "status": "updated" })))
}

async fn skills_uninstall_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({ "name": name, "status": "uninstalled" })))
}

// ── Recipe Marketplace Handlers ──────────────────────────────────────────────

async fn recipes_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "recipes": [] })))
}

async fn recipes_builtin_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let paths = rt.paths.clone();
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(paths);
    Ok(Json(json!({ "built_in": sm.built_in_recipes() })))
}

async fn recipes_installed_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "installed": [] })))
}

async fn recipes_drafts_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "drafts": [] })))
}

async fn recipes_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "name": "dummy_recipe" })))
}

async fn recipes_audit_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "id": id, "audit": { "risk_level": "low", "warnings": [], "recommended_action": "safe_to_install" } }),
    ))
}

async fn recipes_install_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "status": "approval_required" })))
}

async fn recipes_enable_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let paths = rt.paths.clone();
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(paths);
    if let Err(e) = sm.enable(&name) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ));
    }
    Ok(Json(json!({ "name": name, "status": "enabled" })))
}

async fn recipes_disable_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let paths = rt.paths.clone();
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(paths);
    if let Err(e) = sm.disable(&name) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ));
    }
    Ok(Json(json!({ "name": name, "status": "disabled" })))
}

async fn recipes_uninstall_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let paths = rt.paths.clone();
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(paths);
    if let Err(e) = sm.uninstall(&name) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ));
    }
    Ok(Json(json!({ "name": name, "status": "uninstalled" })))
}

async fn recipes_provenance_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "name": name, "provenance": "local" })))
}

async fn recipes_from_memory_handler(
    headers: HeaderMap,
    Path(candidate_id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "status": "draft_created", "candidate_id": candidate_id }),
    ))
}

async fn recipes_activate_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(rt.paths.clone());
    if let Err(e) = sm.activate(&name, "hook") {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ));
    }
    Ok(Json(json!({ "name": name, "status": "activated" })))
}

async fn recipes_deactivate_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(rt.paths.clone());
    if let Err(e) = sm.deactivate(&name) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ));
    }
    Ok(Json(json!({ "name": name, "status": "deactivated" })))
}

async fn recipes_plan_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(rt.paths.clone());
    match sm.plan(&name) {
        Ok(plan) => Ok(Json(json!({ "plan": plan }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn recipes_run_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let sm = crate::recipe_marketplace::RecipeMarketplaceManager::new(rt.paths.clone());
    match sm.run(&name) {
        Ok(record) => Ok(Json(json!({ "record": record }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn recipes_runs_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "runs": [] })))
}

async fn recipes_runs_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "name": name, "runs": [] })))
}

async fn recipes_status_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "name": name, "status": "unknown" })))
}

async fn agent_templates_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "templates": [] })))
}

async fn agent_templates_draft_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "status": "draft_created" })))
}

// ── Brain Index ──────────────────────────────────────────────────────────────

async fn brain_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    match manager.status() {
        Ok(stats) => Ok(Json(json!(stats))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn brain_index_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    match manager.index_all().await {
        Ok(_) => Ok(Json(json!({ "status": "indexed" }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[derive(serde::Deserialize)]
struct BrainSearchQueryArgs {
    q: String,
    mode: Option<String>,
}

async fn brain_search_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<BrainSearchQueryArgs>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );

    let mode = match query.mode.as_deref() {
        Some("semantic") => crate::brain_index::BrainSearchMode::Semantic,
        Some("hybrid") => crate::brain_index::BrainSearchMode::Hybrid,
        Some("fuzzy") => crate::brain_index::BrainSearchMode::Fuzzy,
        _ => crate::brain_index::BrainSearchMode::Keyword,
    };

    let sq = crate::brain_index::BrainSearchQuery {
        q: query.q,
        limit: 50,
        kind_filter: None,
        mode,
    };

    match manager.search(&sq).await {
        Ok(res) => Ok(Json(json!({ "results": res }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn brain_recall_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<BrainSearchQueryArgs>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );

    let mode = match query.mode.as_deref() {
        Some("semantic") => crate::brain_index::BrainSearchMode::Semantic,
        Some("hybrid") => crate::brain_index::BrainSearchMode::Hybrid,
        Some("fuzzy") => crate::brain_index::BrainSearchMode::Fuzzy,
        _ => crate::brain_index::BrainSearchMode::Keyword,
    };

    match manager.recall(&query.q, mode).await {
        Ok(res) => Ok(Json(json!({ "recall": res }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn brain_related_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": id, "related": [] })))
}

async fn brain_sources_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "sources": ["memory", "skills", "recipes", "studio_drafts"] }),
    ))
}

async fn brain_privacy_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({
        "status": "active",
        "redaction": "enabled",
        "semantic_embeddings": "local_only",
        "skipped": [".env", "tokens", "private_keys"]
    })))
}

async fn brain_embeddings_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    match manager.status() {
        Ok(stats) => Ok(Json(json!({
            "provider": stats.embedding_provider,
            "total_vectors": stats.total_vectors,
            "enabled": rt.config.embeddings.enabled
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn brain_embeddings_rebuild_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let manager = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    // In a real app, this should be a background task
    match manager.index_all().await {
        Ok(_) => Ok(Json(json!({ "status": "rebuilt" }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
