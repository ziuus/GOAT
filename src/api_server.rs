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
        .route("/v1/approvals/:id", get(approval_detail_handler))
        .route("/v1/approvals/:id/approve", post(approval_approve_handler))
        .route("/v1/approvals/:id/deny", post(approval_deny_handler))
        .route("/v1/chat", post(chat_handler))
        .route("/v1/sessions", get(sessions_list_handler).post(session_create_handler))
        .route("/v1/sessions/:id", get(session_detail_handler))
        .route("/v1/repo/tree", get(repo_tree_handler))
        .route("/v1/repo/file", get(repo_file_handler))
        .route("/v1/diffs", get(diffs_handler))
        .route("/v1/context", get(context_get_handler))
        .route("/v1/context/add", post(context_add_handler))
        .route("/v1/context/remove", post(context_remove_handler))
        .route("/v1/context/clear", post(context_clear_handler))
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

    // Minimal safe command handling
    let cmd = req.command.trim();
    if cmd == "/status" {
        return status_handler(headers, State(state)).await;
    } else if cmd == "/jobs" {
        return jobs_list_handler(headers, State(state)).await;
    } else if cmd == "/schedule" {
        return schedule_list_handler(headers, State(state)).await;
    } else if cmd == "/hooks" {
        return hooks_list_handler(headers, State(state)).await;
    }

    // Risky command handling -> defer to approval queue in future
    Err((
        StatusCode::FORBIDDEN,
        Json(json!({
            "approval_required": true,
            "risk": "High",
            "message": "Use TUI/headless to approve, API approval workflow planned.",
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
    // Placeholder for actual LLM execution since we don't block the daemon loop yet.
    // To satisfy "safe command/chat foundation and document partial":
    let msg = req.message.trim();
    if msg.starts_with('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Commands through chat not yet supported via API" })),
        ));
    }

    Ok(Json(json!({
        "status": "queued",
        "message": "Chat routing from Web Dashboard is partially implemented. Full streaming chat is planned for Phase 4.4.",
        "input": msg,
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
            let sessions: Vec<_> = records.into_iter().map(|s| json!({
                "id": s.id,
                "title": s.title,
                "created_at": s.created_at,
                "updated_at": s.updated_at
            })).collect();
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
            let messages: Vec<_> = history.into_iter().map(|(r, c)| json!({
                "role": r,
                "content": c
            })).collect();
            return Ok(Json(json!({ "id": id, "history": messages })));
        }
    }
    Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Session not found" }))))
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
        return Ok(Json(json!({ "id": new_id, "title": "New Dashboard Session" })));
    }
    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Brain disabled" }))))
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
    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to scan repo" }))))
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
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Path outside project" }))));
    }
    if crate::repo_map::looks_like_secret_file(&p) {
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Refusing to read potential secret file" }))));
    }
    
    if let Ok(content) = std::fs::read_to_string(&p) {
        let redacted = crate::approval::redact_secrets(&content);
        Ok(Json(json!({ "path": query.path, "content": redacted })))
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({ "error": "File not found or not UTF-8" }))))
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
        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Git diff failed" }))))
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
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Cannot add secret files to context" }))));
    }
    if !rt.selected_files.contains(&req.path) {
        rt.selected_files.push(req.path.clone());
    }
    Ok(Json(json!({ "status": "added", "selected_files": rt.selected_files })))
}

async fn context_remove_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(req): Json<ContextAddReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.selected_files.retain(|f| f != &req.path);
    Ok(Json(json!({ "status": "removed", "selected_files": rt.selected_files })))
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
