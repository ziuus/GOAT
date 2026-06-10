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
use tower_http::cors::{Any, CorsLayer};
use tokio_stream::StreamExt;

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
) -> Result<axum::response::sse::Sse<impl tokio_stream::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>>, (StatusCode, Json<serde_json::Value>)> {
    if state.auth_required {
        let auth_valid = if let Some(auth_header) = headers.get("Authorization") {
            auth_header.to_str().unwrap_or("") == format!("Bearer {}", state.token)
        } else if let Some(token) = query.get("token") {
            token == &state.token
        } else {
            false
        };

        if !auth_valid {
            return Err((StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))));
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

    Ok(axum::response::sse::Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new()))
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
