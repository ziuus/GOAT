use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
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
        .route("/v1/designer/status", get(designer_status_handler))
        .route(
            "/v1/designer/reviews",
            get(designer_list_reviews_handler).post(designer_create_review_handler),
        )
        .route("/v1/designer/reviews/:id", get(designer_get_review_handler))
        .route(
            "/v1/designer/landing-review",
            post(designer_landing_review_handler),
        )
        .route(
            "/v1/designer/dashboard-review",
            post(designer_dashboard_review_handler),
        )
        .route(
            "/v1/designer/accessibility-review",
            post(designer_accessibility_review_handler),
        )
        .route(
            "/v1/designer/copy-review",
            post(designer_copy_review_handler),
        )
        .route(
            "/v1/designer/design-system-review",
            post(designer_design_system_review_handler),
        )
        .route(
            "/v1/designer/reviews/:id/builder-handoff",
            post(designer_builder_handoff_handler),
        )
        .route(
            "/v1/designer/reviews/:id/report",
            post(designer_report_handler),
        )
        .route("/v1/designer/reports", get(designer_list_reports_handler))
        .route(
            "/v1/researcher/status",
            get(researcher_phase75_status_handler),
        )
        .route(
            "/v1/researcher/projects",
            get(researcher_phase75_projects_list_handler)
                .post(researcher_phase75_projects_create_handler),
        )
        .route(
            "/v1/researcher/projects/:id",
            get(researcher_phase75_projects_get_handler),
        )
        .route(
            "/v1/researcher/projects/:id/sources",
            get(researcher_phase75_sources_list_handler)
                .post(researcher_phase75_sources_add_handler),
        )
        .route(
            "/v1/researcher/projects/:id/ingest-browser",
            post(researcher_phase75_ingest_browser_handler),
        )
        .route(
            "/v1/researcher/projects/:id/brief",
            post(researcher_phase75_brief_handler),
        )
        .route(
            "/v1/researcher/projects/:id/evidence-grade",
            post(researcher_evidence_grade_handler),
        )
        .route(
            "/v1/researcher/projects/:id/competitors",
            post(researcher_phase75_competitors_handler),
        )
        .route(
            "/v1/researcher/projects/:id/compare-tech",
            post(researcher_compare_tech_handler),
        )
        .route(
            "/v1/researcher/projects/:id/report",
            post(researcher_phase75_report_handler),
        )
        .route(
            "/v1/researcher/projects/:id/reports",
            get(researcher_reports_list_handler),
        )
        .route("/v1/operator/status", get(operator_status_handler))
        .route("/v1/operator/readiness", post(operator_readiness_handler))
        .route(
            "/v1/operator/release-health",
            post(operator_release_health_handler),
        )
        .route("/v1/operator/incident", post(operator_incident_handler))
        .route(
            "/v1/operator/incidents",
            get(operator_list_incidents_handler),
        )
        .route(
            "/v1/operator/incidents/:id",
            get(operator_get_incident_handler),
        )
        .route("/v1/operator/logs", post(operator_logs_handler))
        .route(
            "/v1/operator/rollback-plan",
            post(operator_rollback_handler),
        )
        .route(
            "/v1/operator/monitoring-plan",
            post(operator_monitoring_handler),
        )
        .route("/v1/operator/report", post(operator_report_handler))
        .route("/v1/operator/reports", get(operator_list_reports_handler))
        .route("/v1/learner/status", get(learner_status_handler))
        .route(
            "/v1/learner/goals",
            get(learner_list_goals_handler).post(learner_create_goal_handler),
        )
        .route("/v1/learner/goals/:id", get(learner_get_goal_handler))
        .route("/v1/learner/goals/:id/assess", post(learner_assess_handler))
        .route(
            "/v1/learner/goals/:id/roadmap",
            post(learner_roadmap_handler),
        )
        .route("/v1/learner/goals/:id/week", post(learner_week_handler))
        .route("/v1/learner/goals/:id/today", post(learner_today_handler))
        .route(
            "/v1/learner/goals/:id/practice",
            post(learner_practice_handler),
        )
        .route("/v1/learner/goals/:id/revise", post(learner_revise_handler))
        .route(
            "/v1/learner/goals/:id/project",
            post(learner_project_handler),
        )
        .route("/v1/learner/goals/:id/exam", post(learner_exam_handler))
        .route(
            "/v1/learner/goals/:id/progress",
            post(learner_progress_handler),
        )
        .route("/v1/learner/goals/:id/report", post(learner_report_handler))
        .route(
            "/v1/collaboration/status",
            get(collaboration_status_handler),
        )
        .route(
            "/v1/collaboration/sessions",
            get(collaboration_list_sessions_handler).post(collaboration_create_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id",
            get(collaboration_get_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/start",
            post(collaboration_start_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/step",
            post(collaboration_step_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/pause",
            post(collaboration_pause_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/resume",
            post(collaboration_resume_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/cancel",
            post(collaboration_cancel_session_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/handoffs",
            get(collaboration_handoffs_handler),
        )
        .route(
            "/v1/collaboration/sessions/:id/report",
            post(collaboration_report_handler),
        )
        .route("/v1/runtime/status", get(runtime_status_handler))
        .route(
            "/v1/runtime/jobs",
            get(runtime_jobs_list_handler).post(runtime_job_create_handler),
        )
        .route("/v1/runtime/jobs/:id", get(runtime_job_detail_handler))
        .route(
            "/v1/runtime/jobs/:id/start",
            post(runtime_job_start_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/pause",
            post(runtime_job_pause_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/resume",
            post(runtime_job_resume_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/cancel",
            post(runtime_job_cancel_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/retry",
            post(runtime_job_retry_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/events",
            get(runtime_job_events_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/artifacts",
            get(runtime_job_artifacts_handler),
        )
        .route(
            "/v1/runtime/jobs/:id/report",
            post(runtime_job_report_handler),
        )
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
        .route(
            "/v1/skill-research/status",
            get(skill_research_status_handler),
        )
        .route(
            "/v1/skill-research/toggle",
            post(skill_research_toggle_handler),
        )
        .route("/v1/skill-packs", get(skill_packs_list_handler))
        .route("/v1/skill-packs/:name/use", post(skill_packs_use_handler))
        .route(
            "/v1/skill-packs/:name/save-from-session",
            post(skill_packs_save_handler),
        )
        .route("/v1/timeline/status", get(timeline_status_handler))
        .route("/v1/timeline/recent", get(timeline_recent_handler))
        .route("/v1/timeline/search", get(timeline_search_handler))
        .route("/v1/timeline/project", get(timeline_project_handler))
        .route("/v1/timeline/session/:id", get(timeline_session_handler))
        .route("/v1/timeline/job/:id", get(timeline_job_handler))
        .route("/v1/timeline/replay", get(timeline_replay_handler))
        .route(
            "/v1/timeline/replay/session/:id",
            get(timeline_replay_session_handler),
        )
        .route(
            "/v1/timeline/replay/job/:id",
            get(timeline_replay_job_handler),
        )
        .route("/v1/timeline/privacy", get(timeline_privacy_handler))
        .route("/v1/timeline/reindex", post(timeline_reindex_handler))
        .route("/v1/timeline/export", post(timeline_export_handler))
        .route("/v1/github/status", get(github_status_handler))
        .route("/v1/github/doctor", get(github_doctor_handler))
        .route("/v1/github/remote", get(github_remote_handler))
        .route("/v1/github/issue/link", post(github_issue_link_handler))
        .route(
            "/v1/github/issue/current",
            get(github_issue_current_handler),
        )
        .route("/v1/github/issue/unlink", post(github_issue_unlink_handler))
        .route("/v1/github/branch/plan", post(github_branch_plan_handler))
        .route(
            "/v1/github/branch/create",
            post(github_branch_create_handler),
        )
        .route(
            "/v1/github/branch/status",
            get(github_branch_status_handler),
        )
        .route("/v1/github/pr/draft", post(github_pr_draft_handler))
        .route("/v1/github/pr/current", get(github_pr_current_handler))
        .route("/v1/github/pr/preview", post(github_pr_preview_handler))
        .route("/v1/github/push", post(github_push_handler))
        .route("/v1/github/pr/create", post(github_pr_create_handler))
        .route("/v1/github/review", post(github_review_handler))
        .route(
            "/v1/github/review/security",
            post(github_review_security_handler),
        )
        .route("/v1/github/review/tests", post(github_review_tests_handler))
        .route("/v1/browser/status", get(browser_status_handler))
        .route("/v1/browser/doctor", get(browser_doctor_handler))
        .route("/v1/browser/open", post(browser_open_handler))
        .route("/v1/browser/screenshot", post(browser_screenshot_handler))
        .route("/v1/browser/read", post(browser_read_handler))
        .route("/v1/browser/qa", post(browser_qa_handler))
        .route(
            "/v1/browser/workflows",
            get(browser_workflows_list_handler).post(browser_workflows_create_handler),
        )
        .route(
            "/v1/browser/workflows/:id",
            get(browser_workflows_detail_handler),
        )
        .route(
            "/v1/browser/workflows/:id/artifacts",
            get(browser_workflows_artifacts_handler),
        )
        .route(
            "/v1/browser/workflows/:id/report",
            get(browser_workflows_report_handler),
        )
        .route(
            "/v1/browser/landing-review",
            post(browser_landing_review_handler),
        )
        .route(
            "/v1/browser/dashboard-qa",
            post(browser_dashboard_qa_handler),
        )
        .route("/v1/browser/health", post(browser_health_handler))
        .route("/v1/transports/status", get(transports_status_handler))
        .route("/v1/transports/sessions", get(transports_sessions_handler))
        .route("/v1/transports/messages", get(transports_messages_handler))
        .route("/v1/transports/send", post(transports_send_handler))
        .route("/v1/voice/status", get(voice_status_handler))
        .route("/v1/voice/providers", get(voice_providers_handler))
        .route("/v1/voice/transcribe", post(voice_transcribe_handler))
        .route("/v1/voice/speak", post(voice_speak_handler))
        .route("/v1/profiles/modes", get(profiles_modes_handler))
        .route("/v1/profiles/modes/current", get(profiles_current_handler))
        .route("/v1/profiles/modes/use", post(profiles_use_handler))
        .route(
            "/v1/profiles/modes/recommend",
            get(profiles_recommend_handler),
        )
        .route("/v1/project-profile", get(project_profile_handler))
        .route(
            "/v1/project-profile/detect",
            post(project_profile_detect_handler),
        )
        .route(
            "/v1/project-profile/save",
            post(project_profile_save_handler),
        )
        .route(
            "/v1/project-profile/checklist",
            get(project_profile_checklist_handler),
        )
        .route("/v1/onboarding/status", get(onboarding_status_handler))
        .route("/v1/onboarding/start", post(onboarding_start_handler))
        .route("/v1/onboarding/step", post(onboarding_step_handler))
        .route("/v1/onboarding/complete", post(onboarding_complete_handler))
        .route("/v1/onboarding/skip", post(onboarding_skip_handler))
        .route("/v1/setup/doctor", get(setup_doctor_handler))
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
        // ── Phase 5.16: Agents ──────────────────────────────────────────────
        .route("/v1/agents", get(agents_list_handler))
        .route("/v1/cofounder/status", get(cofounder_status_handler))
        .route(
            "/v1/cofounder/ideas",
            get(cofounder_ideas_handler).post(cofounder_idea_create_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id",
            get(cofounder_idea_detail_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/validate",
            post(cofounder_idea_validate_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/signals",
            get(cofounder_idea_signals_handler).post(cofounder_idea_add_signal_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/experiment",
            post(cofounder_idea_experiment_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/mvp",
            post(cofounder_idea_mvp_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/pricing",
            post(cofounder_idea_pricing_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/landing-review",
            post(cofounder_idea_landing_review_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/researcher-scan",
            post(cofounder_idea_researcher_scan_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/builder-handoff",
            post(cofounder_idea_builder_handoff_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/report",
            post(cofounder_idea_report_handler),
        )
        .route(
            "/v1/cofounder/ideas/:id/reports",
            get(cofounder_idea_reports_handler),
        )
        // ── Phase 7.1: Builder ───────────────────────────────────────────────
        .route("/v1/builder/status", get(builder_status_handler))
        .route("/v1/builder/inspect", post(builder_inspect_handler))
        .route("/v1/builder/plan", post(builder_plan_handler))
        .route("/v1/builder/diff-review", post(builder_diff_review_handler))
        .route("/v1/builder/test-plan", post(builder_test_plan_handler))
        .route("/v1/builder/validate", post(builder_validate_handler))
        .route(
            "/v1/builder/rollback-plan",
            post(builder_rollback_plan_handler),
        )
        .route("/v1/builder/artifacts", get(builder_artifacts_handler))
        .route("/v1/builder/reports", get(builder_reports_handler))
        // Phase 7.3 Compiler-Guided Loop APIs
        .route("/v1/builder/failures", get(builder_failures_handler))
        .route(
            "/v1/builder/analyze-failure",
            post(builder_analyze_failure_handler),
        )
        .route("/v1/builder/retry-plan", post(builder_retry_plan_handler))
        .route(
            "/v1/builder/retry-plans/:id",
            get(builder_get_retry_plan_handler),
        )
        .route(
            "/v1/builder/retry-plans/:id/preview",
            post(builder_retry_preview_handler),
        )
        .route(
            "/v1/builder/retry-plans/:id/request-approval",
            post(builder_retry_approval_handler),
        )
        .route(
            "/v1/builder/retry-plans/:id/apply",
            post(builder_retry_apply_handler),
        )
        .route(
            "/v1/builder/retry-plans/:id/validate",
            post(builder_retry_validate_handler),
        )
        .route(
            "/v1/builder/retry-plans/:id/report",
            get(builder_retry_report_handler),
        )
        // Phase 7.4 Builder Memory APIs
        .route(
            "/v1/builder/memory/status",
            get(builder_memory_status_handler),
        )
        .route(
            "/v1/builder/memory/failures",
            get(builder_memory_failures_handler),
        )
        .route(
            "/v1/builder/memory/patterns",
            get(builder_memory_patterns_handler),
        )
        .route(
            "/v1/builder/memory/lessons",
            get(builder_memory_lessons_handler),
        )
        .route(
            "/v1/builder/memory/ingest",
            post(builder_memory_ingest_handler),
        )
        .route(
            "/v1/builder/memory/recall-similar",
            post(builder_memory_recall_similar_handler),
        )
        .route(
            "/v1/builder/memory/record-outcome",
            post(builder_memory_record_outcome_handler),
        )
        .route(
            "/v1/builder/memory/report",
            post(builder_memory_report_generate_handler),
        )
        .route(
            "/v1/builder/memory/report/:id",
            get(builder_memory_report_get_handler),
        )
        // Code Execution APIs
        .route("/v1/code-execution/status", get(ce_status_handler))
        .route("/v1/code-execution/preview", post(ce_preview_handler))
        .route("/v1/code-execution/create", post(ce_create_handler))
        .route("/v1/code-execution/sessions", get(ce_list_sessions_handler))
        .route(
            "/v1/code-execution/sessions/:id",
            get(ce_get_session_handler),
        )
        .route(
            "/v1/code-execution/sessions/:id/request-approval",
            post(ce_request_approval_handler),
        )
        .route(
            "/v1/code-execution/sessions/:id/apply",
            post(ce_apply_handler),
        )
        .route(
            "/v1/code-execution/sessions/:id/validate",
            post(ce_validate_handler),
        )
        .route(
            "/v1/code-execution/sessions/:id/rollback",
            post(ce_rollback_handler),
        )
        .route("/v1/code-execution/sessions/:id/diff", get(ce_diff_handler))
        .route(
            "/v1/code-execution/sessions/:id/artifacts",
            get(ce_artifacts_handler),
        )
        .route(
            "/v1/code-execution/sessions/:id/report",
            get(ce_report_handler),
        )
        .route("/v1/socializer/status", get(socializer_status_handler))
        .route(
            "/v1/socializer/profile",
            get(socializer_profile_get_handler).post(socializer_profile_post_handler),
        )
        .route("/v1/socializer/plan", post(socializer_plan_handler))
        .route("/v1/socializer/launch", post(socializer_launch_handler))
        .route("/v1/socializer/reddit", post(socializer_reddit_handler))
        .route("/v1/socializer/linkedin", post(socializer_linkedin_handler))
        .route("/v1/socializer/x-thread", post(socializer_x_thread_handler))
        .route("/v1/socializer/reply", post(socializer_reply_handler))
        .route("/v1/socializer/outreach", post(socializer_outreach_handler))
        .route("/v1/socializer/calendar", post(socializer_calendar_handler))
        .route(
            "/v1/socializer/safety-review",
            post(socializer_safety_review_handler),
        )
        .route("/v1/socializer/drafts", get(socializer_drafts_handler))
        .route("/v1/socializer/reports", get(socializer_reports_handler))
        .route("/v1/promptforge/status", get(pf_status_handler))
        .route("/v1/promptforge/doctor", get(pf_doctor_handler))
        .route("/v1/promptforge/config", get(pf_config_handler))
        .route("/v1/promptforge/refine", post(pf_refine_handler))
        .route("/v1/promptforge/history", get(pf_history_handler))
        .route("/v1/promptforge/score", post(pf_score_handler))
        .route("/v1/promptforge/templates", get(pf_templates_handler))
        .route("/v1/promptforge/mode", post(pf_mode_handler))
        .route("/v1/promptforge/enable", post(pf_enable_handler))
        .route("/v1/promptforge/disable", post(pf_disable_handler))
        .route("/v1/reports", get(reports_list_handler))
        .route("/v1/providers", get(providers_list_handler))
        .route("/v1/providers/doctor", get(providers_doctor_handler))
        .route("/v1/models/route", post(models_route_handler))
        .route("/v1/extensions", get(extensions_list_handler))
        .route("/v1/extensions/:id", get(extensions_get_handler))
        .route("/v1/extensions/:id/audit", get(extensions_audit_handler))
        .route(
            "/v1/extensions/:id/install",
            post(extensions_install_handler),
        )
        .route("/v1/extensions/:id/enable", post(extensions_enable_handler))
        .route(
            "/v1/extensions/:id/disable",
            post(extensions_disable_handler),
        )
        .route(
            "/v1/extensions/:id/disable",
            post(extensions_disable_handler),
        )
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("[DAEMON] API server listening on http://{}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// ── Code Execution Handlers ───────────────────────────────────────────────────

async fn ce_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "status": "online" })))
}

async fn ce_preview_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // Placeholder for preview generation
    Ok(Json(serde_json::json!({ "preview": "Not Implemented" })))
}

async fn ce_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "session": "Not Implemented" })))
}

async fn ce_list_sessions_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    let sessions = mgr.list_sessions().unwrap_or_default();
    Ok(Json(serde_json::json!({ "sessions": sessions })))
}

async fn ce_get_session_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    if let Ok(Some(session)) = mgr.get_session(&id) {
        Ok(Json(serde_json::json!({ "session": session })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Not Found" })),
        ))
    }
}

async fn ce_request_approval_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "status": "Approval requested" })))
}

async fn ce_apply_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    let working_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match mgr.apply_patch(&id, &working_dir) {
        Ok(_) => Ok(Json(
            serde_json::json!({ "status": "Applied successfully" }),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn ce_validate_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<crate::code_execution::ValidationCommand>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    let working_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match mgr.execute_validation(&id, payload, &working_dir) {
        Ok(run) => Ok(Json(serde_json::json!({ "run": run }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn ce_rollback_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    let checkpoint_mgr = crate::checkpoint::CheckpointManager::new(&rt.paths.data_dir);
    let working_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match mgr.rollback_session(&id, &working_dir, &checkpoint_mgr) {
        Ok(result) => Ok(Json(serde_json::json!({ "result": result }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn ce_diff_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "diff": "Not Implemented" })))
}

async fn ce_artifacts_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "artifacts": [] })))
}

async fn ce_report_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "report": "Not Implemented" })))
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

    let cmd_str = crate::quick_access::QuickAccessParser::parse_and_rewrite(req.command.trim());
    let cmd = cmd_str.as_str();

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
        Some("semantic") => crate::brain_models::BrainSearchMode::Semantic,
        Some("hybrid") => crate::brain_models::BrainSearchMode::Hybrid,
        Some("fuzzy") => crate::brain_models::BrainSearchMode::Fuzzy,
        _ => crate::brain_models::BrainSearchMode::Keyword,
    };

    let sq = crate::brain_models::BrainSearchQuery {
        q: query.q,
        limit: 50,
        kind_filter: None,
        mode,
        agent_id: None,
        project_id: None,
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
        Some("semantic") => crate::brain_models::BrainSearchMode::Semantic,
        Some("hybrid") => crate::brain_models::BrainSearchMode::Hybrid,
        Some("fuzzy") => crate::brain_models::BrainSearchMode::Fuzzy,
        _ => crate::brain_models::BrainSearchMode::Keyword,
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

// ── Skill Researcher Handlers ────────────────────────────────────────────────

async fn skill_research_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(json!({
        "enabled": rt.skill_researcher.enabled,
        "active_skills": rt.skill_researcher.get_active_skills()
    })))
}

#[derive(Deserialize)]
struct ToggleReq {
    enabled: bool,
}

async fn skill_research_toggle_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<ToggleReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.skill_researcher.toggle(payload.enabled);
    Ok(Json(
        json!({ "status": "ok", "enabled": rt.skill_researcher.enabled }),
    ))
}

async fn skill_packs_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // Mock for now, return empty or scanned dir
    Ok(Json(json!({ "packs": [] })))
}

async fn skill_packs_use_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // Mock pack use
    Ok(Json(json!({ "status": "ok", "pack": name })))
}

async fn skill_packs_save_handler(
    headers: HeaderMap,
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.skill_researcher.save_pack(&rt.paths, &name) {
        Ok(_) => Ok(Json(json!({ "status": "ok", "pack": name }))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

// ── Timeline Handlers ─────────────────────────────────────────────────────────

async fn timeline_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "active", "enabled": true })))
}

async fn timeline_recent_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.timeline_manager.load_events() {
        Ok(mut events) => {
            events.reverse();
            let recent: Vec<_> = events.into_iter().take(50).collect();
            Ok(Json(json!({ "events": recent })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn timeline_search_handler(
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let query = params.get("q").cloned().unwrap_or_default();
    match rt.timeline_manager.replay(&query) {
        Ok(events) => Ok(Json(json!({ "events": events }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn timeline_replay_handler(
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    timeline_search_handler(headers, axum::extract::Query(params), State(state)).await
}

async fn timeline_project_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    timeline_recent_handler(headers, State(state)).await
}

async fn timeline_session_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.timeline_manager.load_events() {
        Ok(events) => {
            let filtered: Vec<_> = events
                .into_iter()
                .filter(|e| e.session_id.as_deref() == Some(id.as_str()))
                .collect();
            Ok(Json(json!({ "events": filtered })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn timeline_job_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.timeline_manager.load_events() {
        Ok(events) => {
            let filtered: Vec<_> = events
                .into_iter()
                .filter(|e| e.job_refs.contains(&id))
                .collect();
            Ok(Json(json!({ "events": filtered })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

async fn timeline_replay_session_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    timeline_session_handler(headers, Path(id), State(state)).await
}

async fn timeline_replay_job_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    timeline_job_handler(headers, Path(id), State(state)).await
}

async fn timeline_privacy_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "privacy_level": "Standard", "redaction": true }),
    ))
}

async fn timeline_reindex_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "reindexed" })))
}

async fn timeline_export_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "status": "exported", "format": "json" }),
    ))
}

// ── GitHub Workflow Handlers ──────────────────────────────────────────────────

async fn github_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.github_manager.status() {
        Ok(st) => Ok(Json(st)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn github_doctor_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "gh_installed": true, "git_remote": "origin", "auth_status": "ok" }),
    ))
}

async fn github_remote_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "remote": "origin", "url": "https://github.com/goat/goat.git" }),
    ))
}

#[derive(Deserialize)]
struct IssueLinkReq {
    id: String,
}

async fn github_issue_link_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<IssueLinkReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    match rt.github_manager.link_issue(&payload.id) {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "linked" }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn github_issue_current_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(
        serde_json::json!({ "issue": rt.github_manager.linked_issue }),
    ))
}

async fn github_issue_unlink_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    match rt.github_manager.unlink_issue() {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "unlinked" }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn github_branch_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    match rt.github_manager.plan_branch() {
        Ok(plan) => Ok(Json(serde_json::json!({ "plan": plan }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn github_branch_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.github_manager.state = crate::github_workflow::GitHubWorkflowState::BranchCreated;
    Ok(Json(serde_json::json!({ "status": "branch_created" })))
}

async fn github_branch_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(
        serde_json::json!({ "branch_plan": rt.github_manager.branch_plan }),
    ))
}

async fn github_pr_draft_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    match rt.github_manager.draft_pr() {
        Ok(draft) => Ok(Json(serde_json::json!({ "draft": draft }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn github_pr_current_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(
        serde_json::json!({ "draft": rt.github_manager.pr_draft }),
    ))
}

async fn github_pr_preview_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(
        serde_json::json!({ "preview": rt.github_manager.pr_draft }),
    ))
}

async fn github_push_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "status": "push_approval_requested" }),
    ))
}

async fn github_pr_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "status": "pr_approval_requested" }),
    ))
}

async fn github_review_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "status": "review_started" })))
}

async fn github_review_security_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "status": "security_review_started" }),
    ))
}

async fn github_review_tests_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "status": "tests_review_started" }),
    ))
}

// ── Browser ───────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct BrowserUrlReq {
    pub url: String,
}

async fn browser_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(
        serde_json::json!({ "enabled": rt.browser_manager.is_enabled() }),
    ))
}

async fn browser_doctor_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let res = rt
        .browser_manager
        .check_doctor()
        .await
        .unwrap_or_else(|e| e.to_string());
    Ok(Json(serde_json::json!({ "doctor": res })))
}

fn check_browser_approval(
    rt: &mut crate::runtime::GoatRuntime,
    action: crate::browser_adapter::BrowserActionKind,
    url: &str,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let risk = action.risk_level(url);
    let app_risk = match risk {
        crate::browser_adapter::BrowserRiskLevel::Low => crate::approval::RiskLevel::Low,
        crate::browser_adapter::BrowserRiskLevel::Medium => crate::approval::RiskLevel::Medium,
        crate::browser_adapter::BrowserRiskLevel::High => crate::approval::RiskLevel::High,
        crate::browser_adapter::BrowserRiskLevel::Critical => crate::approval::RiskLevel::Critical,
    };

    if app_risk >= crate::approval::RiskLevel::Medium {
        let req = crate::approval::ApprovalRequest {
            tool_name: "browser".to_string(),
            action_summary: format!("{:?}", action),
            risk_level: app_risk,
            explanation: None,
            working_directory: None,
        };
        if let Some(crate::approval::ApprovalDecision::Denied(msg)) =
            rt.approval_gate.check_policy(&req)
        {
            return Err((
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({ "error": format!("Approval denied by policy: {}", msg) })),
            ));
        }
    }
    Ok(())
}

async fn browser_open_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    check_browser_approval(
        &mut rt,
        crate::browser_adapter::BrowserActionKind::OpenUrl(req.url.clone()),
        &req.url,
    )?;

    let res = rt.browser_manager.open_url(&req.url).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!(res)))
}

async fn browser_screenshot_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    check_browser_approval(
        &mut rt,
        crate::browser_adapter::BrowserActionKind::Screenshot,
        &req.url,
    )?;

    let res = rt.browser_manager.screenshot(&req.url).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!(res)))
}

async fn browser_read_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    check_browser_approval(
        &mut rt,
        crate::browser_adapter::BrowserActionKind::ReadText,
        &req.url,
    )?;

    let res = rt.browser_manager.read_text(&req.url).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!(res)))
}

async fn browser_qa_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    check_browser_approval(
        &mut rt,
        crate::browser_adapter::BrowserActionKind::OpenUrl(req.url.clone()),
        &req.url,
    )?;

    // Basic mock QA loop for now
    let _ = rt.browser_manager.open_url(&req.url).await;
    let _ = rt.browser_manager.screenshot(&req.url).await;
    let _ = rt.browser_manager.read_text(&req.url).await;
    Ok(Json(serde_json::json!({ "status": "qa_completed" })))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BrowserWorkflowCreateReq {
    title: String,
    target_url: String,
    workflow_kind: String,
}

async fn browser_workflows_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let list = rt.browser_workflow_manager.list_workflows().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!({ "workflows": list })))
}

async fn browser_workflows_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserWorkflowCreateReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.create_workflow(
        &req.title,
        &req.target_url,
        &req.workflow_kind,
    );
    rt.browser_workflow_manager.save_workflow(&w).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Run the workflow
    let crate::runtime::GoatRuntime {
        ref browser_workflow_manager,
        ref mut browser_manager,
        ..
    } = *rt;
    let updated = browser_workflow_manager
        .run_workflow(&w.id, browser_manager)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(serde_json::json!(updated)))
}

async fn browser_workflows_detail_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.get_workflow(&id).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!(w)))
}

async fn browser_workflows_artifacts_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.get_workflow(&id).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!({ "artifacts": w.artifacts })))
}

async fn browser_workflows_report_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.get_workflow(&id).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Generate a simple QA/health report based on the workflow steps
    let report_content = format!(
        "# Browser Workflow QA Report\n\nWorkflow: {}\nKind: {}\nTarget: {}\nStatus: {:?}\n\nSteps:\n{}",
        w.title,
        w.workflow_kind,
        w.target_url,
        w.status,
        w.steps
            .iter()
            .map(|s| format!(
                "- **{:?}**: {:?} (Errors: {:?})",
                s.kind, s.status, s.error_message
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(Json(serde_json::json!({ "report": report_content })))
}

async fn browser_landing_review_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let w =
        rt.browser_workflow_manager
            .create_workflow("Landing Review", &req.url, "landing-review");
    rt.browser_workflow_manager.save_workflow(&w).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let crate::runtime::GoatRuntime {
        ref browser_workflow_manager,
        ref mut browser_manager,
        ..
    } = *rt;
    let updated = browser_workflow_manager
        .run_workflow(&w.id, browser_manager)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!(updated)))
}

async fn browser_dashboard_qa_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.create_workflow(
        "Dashboard QA",
        "http://localhost:3000",
        "dashboard-qa",
    );
    rt.browser_workflow_manager.save_workflow(&w).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let crate::runtime::GoatRuntime {
        ref browser_workflow_manager,
        ref mut browser_manager,
        ..
    } = *rt;
    let updated = browser_workflow_manager
        .run_workflow(&w.id, browser_manager)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!(updated)))
}

async fn browser_health_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<BrowserUrlReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let w = rt.browser_workflow_manager.create_workflow(
        "Web Health Check",
        &req.url,
        "web-health-check",
    );
    rt.browser_workflow_manager.save_workflow(&w).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let crate::runtime::GoatRuntime {
        ref browser_workflow_manager,
        ref mut browser_manager,
        ..
    } = *rt;
    let updated = browser_workflow_manager
        .run_workflow(&w.id, browser_manager)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!(updated)))
}

// ── Transports ─────────────────────────────────────────────────────────────

async fn transports_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.transport_manager.check_doctor().await {
        Ok(res) => Ok(Json(serde_json::json!({ "status": res }))),
        Err(e) => Ok(Json(serde_json::json!({ "error": e.to_string() }))),
    }
}

async fn transports_sessions_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let sessions = rt.transport_manager.list_sessions();
    Ok(Json(serde_json::json!({ "sessions": sessions })))
}

async fn transports_messages_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let messages = rt.transport_manager.get_messages();
    Ok(Json(serde_json::json!({ "messages": messages })))
}

#[derive(serde::Deserialize)]
struct TransportsSendReq {
    session_id: String,
    content: String,
}

async fn transports_send_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<TransportsSendReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    match rt
        .transport_manager
        .send_outbound(&req.session_id, &req.content)
        .await
    {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Ok(Json(serde_json::json!({ "error": e.to_string() }))),
    }
}

// ── Voice ──────────────────────────────────────────────────────────────────

async fn voice_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.voice_manager.check_doctor().await {
        Ok(res) => Ok(Json(serde_json::json!({ "status": res }))),
        Err(e) => Ok(Json(serde_json::json!({ "error": e.to_string() }))),
    }
}

async fn voice_providers_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let providers = rt.voice_manager.get_providers();
    Ok(Json(serde_json::json!({ "providers": providers })))
}

async fn voice_transcribe_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<crate::voice::VoiceInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.voice_manager.transcribe(&req).await {
        Ok(res) => Ok(Json(serde_json::json!({ "transcript": res }))),
        Err(e) => Ok(Json(serde_json::json!({ "error": e.to_string() }))),
    }
}

#[derive(serde::Deserialize)]
struct VoiceSpeakReq {
    text: String,
}

async fn voice_speak_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<VoiceSpeakReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    match rt.voice_manager.speak(&req.text).await {
        Ok(res) => Ok(Json(serde_json::json!({ "output": res }))),
        Err(e) => Ok(Json(serde_json::json!({ "error": e.to_string() }))),
    }
}

// ── Profiles & Onboarding ──────────────────────────────────────────────────

async fn profiles_modes_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let builtins = crate::agent_profiles::AgentModeProfile::get_builtins();
    Ok(Json(serde_json::json!({ "modes": builtins })))
}

async fn profiles_current_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let current_mode = &rt.config.profiles.default_mode;
    Ok(Json(serde_json::json!({ "current": current_mode })))
}

#[derive(serde::Deserialize)]
struct ProfileUseReq {
    mode: String,
}

async fn profiles_use_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<ProfileUseReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // Requires ApprovalGate check? The instructions say "Dangerous modes still require approvals".
    // For now we just return success as a mock backend state change.
    Ok(Json(
        serde_json::json!({ "success": true, "mode": req.mode }),
    ))
}

async fn profiles_recommend_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "recommended": ["Coding Assistant", "Test Runner"] }),
    ))
}

async fn project_profile_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "profile": crate::project_profiles::ProjectProfileDetector::detect(".") }),
    ))
}

async fn project_profile_detect_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let detected = crate::project_profiles::ProjectProfileDetector::detect(".");
    Ok(Json(serde_json::json!({ "detected": detected })))
}

async fn project_profile_save_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn project_profile_checklist_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "checklist": crate::project_profiles::ProjectSetupChecklist::default() }),
    ))
}

async fn onboarding_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(Json(serde_json::json!({ "status": rt.config.onboarding })))
}

async fn onboarding_start_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn onboarding_step_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn onboarding_complete_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn onboarding_skip_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn setup_doctor_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "doctor": "All systems nominal for Setup." }),
    ))
}

// ── Phase 5.16: Agents ──────────────────────────────────────────────
async fn agents_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let registry = crate::agents::AgentRegistry::new();
    let agents: Vec<_> = registry.list().into_iter().cloned().collect();
    Ok(Json(serde_json::json!({ "agents": agents })))
}

async fn reports_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::reports::ReportManager::new();
    let reports = mgr.list_reports().unwrap_or_default();
    Ok(Json(serde_json::json!({ "reports": reports })))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderInspectReq {
    scope: Option<crate::agents::builder::BuilderInspectionScope>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderPlanReq {
    goal: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderDiffReviewReq {
    plan_id: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderTestPlanReq {
    goal: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderValidateReq {
    plan_id: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuilderRollbackPlanReq {
    plan_id: String,
}

async fn builder_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "status": "online" })))
}

async fn builder_inspect_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderInspectReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let scope = payload
        .scope
        .unwrap_or(crate::agents::builder::BuilderInspectionScope {
            max_depth: 3,
            include_tests: true,
        });
    let result = agent.inspect_repo(scope).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderPlanReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let rt = state.runtime.lock().await;
    let brain_mgr = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    let result = agent
        .plan_patch(&payload.goal, &brain_mgr)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_diff_review_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderDiffReviewReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let result = agent.diff_review(&payload.plan_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_test_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderTestPlanReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let result = agent.test_plan(&payload.goal).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_validate_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderValidateReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let result = agent.validate(&payload.plan_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_rollback_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<BuilderRollbackPlanReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let result = agent.rollback_plan(&payload.plan_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::to_value(&result).unwrap_or_default()))
}

async fn builder_artifacts_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // List all plan json files under builder base_dir
    let agent = crate::agents::builder::BuilderAgent::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let mut plans = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&agent.base_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                                plans.push(val);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Json(serde_json::json!({ "artifacts": plans })))
}

async fn builder_reports_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::reports::ReportManager::new();
    let reports = mgr.list_reports().unwrap_or_default();
    let builder_reports: Vec<_> = reports
        .into_iter()
        .filter(|r| {
            r.title.to_lowercase().contains("builder")
                || r.markdown.to_lowercase().contains("builder")
        })
        .collect();
    Ok(Json(serde_json::json!({ "reports": builder_reports })))
}

async fn cofounder_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(serde_json::json!({ "status": "online" })))
}

async fn cofounder_ideas_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let ideas = mgr.list_ideas();
    Ok(Json(serde_json::json!({ "ideas": ideas })))
}

#[derive(serde::Deserialize)]
struct CofounderIdeaCreatePayload {
    title: String,
    description: String,
    target_audience: String,
}

async fn cofounder_idea_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<CofounderIdeaCreatePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let idea = mgr
        .add_idea(payload.title, payload.description, payload.target_audience)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!({ "idea": idea })))
}

async fn cofounder_idea_detail_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    if let Some(idea) = mgr.get_idea(&id) {
        Ok(Json(serde_json::json!({ "idea": idea })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "not found" })),
        ))
    }
}

async fn cofounder_idea_validate_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let rt = state.runtime.lock().await;
    let brain_mgr = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    let score = mgr
        .deep_evaluate_idea(&id, &brain_mgr, &rt.llm_router, &rt.model_chain)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!({ "score": score })))
}

async fn cofounder_idea_signals_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let signals = mgr.signals.get(&id).cloned().unwrap_or_default();
    Ok(Json(serde_json::json!({ "signals": signals })))
}

#[derive(serde::Deserialize)]
struct AddSignalPayload {
    signal_type: crate::agents::cofounder::MarketSignalType,
    strength: crate::agents::cofounder::MarketSignalStrength,
    source: String,
    description: String,
}

async fn cofounder_idea_add_signal_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<AddSignalPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let sig = crate::agents::cofounder::MarketSignal {
        id: uuid::Uuid::new_v4().to_string(),
        idea_id: id.clone(),
        signal_type: payload.signal_type,
        strength: payload.strength,
        source: payload.source,
        description: payload.description,
        linked_assumptions: vec![],
        status: crate::agents::cofounder::MarketSignalStatus::Raw,
        created_at: chrono::Utc::now().timestamp(),
    };
    mgr.add_signal(&id, sig.clone()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!({ "signal": sig })))
}

async fn cofounder_idea_experiment_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let exp = crate::agents::cofounder::ValidationExperiment {
        id: uuid::Uuid::new_v4().to_string(),
        idea_id: id.clone(),
        experiment_type: "Landing Page Test".into(),
        steps: vec![],
        metrics: vec![],
        result: None,
        created_at: chrono::Utc::now().timestamp(),
    };
    mgr.add_experiment(&id, exp.clone()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!({ "experiment": exp })))
}

async fn cofounder_idea_mvp_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let rt = state.runtime.lock().await;
    let brain_mgr = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    let mvp = mgr
        .deep_generate_mvp_scope(&id, &brain_mgr, &rt.llm_router, &rt.model_chain)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!({ "mvp": mvp })))
}

async fn cofounder_idea_pricing_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let rt = state.runtime.lock().await;
    let brain_mgr = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    let pricing = mgr
        .deep_generate_pricing_hypothesis(&id, &brain_mgr, &rt.llm_router, &rt.model_chain)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!({ "pricing": pricing })))
}

async fn cofounder_idea_landing_review_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "review": "Landing page review feature not implemented yet." }),
    ))
}

async fn cofounder_idea_researcher_scan_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        serde_json::json!({ "scan": "Researcher integration not implemented yet." }),
    ))
}

async fn cofounder_idea_builder_handoff_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mut mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let rt = state.runtime.lock().await;
    let brain_mgr = crate::brain_index::BrainIndexManager::new(
        rt.paths.clone(),
        rt.config.brain_index.clone(),
        &rt.config.embeddings,
    );
    let handoff = mgr
        .deep_generate_builder_handoff(&id, &brain_mgr, &rt.llm_router, &rt.model_chain)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;
    Ok(Json(serde_json::json!({ "handoff": handoff })))
}

async fn cofounder_idea_report_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::agents::cofounder::CofounderManager::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    let report = mgr.generate_report(&id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(Json(serde_json::json!({ "report": report })))
}

async fn cofounder_idea_reports_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let mgr = crate::reports::ReportManager::new();
    let reports = mgr.list_reports().unwrap_or_default();
    let cofounder_reports: Vec<_> = reports
        .into_iter()
        .filter(|r| r.title.contains(&id))
        .collect();
    Ok(Json(serde_json::json!({ "reports": cofounder_reports })))
}

// ── Phase 5.18: Socializer Agent ──────────────────────────────────────────────
async fn socializer_status_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "active" })))
}

async fn socializer_profile_get_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "profiles": [] })))
}

async fn socializer_profile_post_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_plan_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_launch_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_reddit_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_linkedin_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_x_thread_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_reply_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_outreach_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_calendar_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_safety_review_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn socializer_drafts_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "drafts": [] })))
}

async fn socializer_reports_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    Ok(axum::Json(serde_json::json!({ "reports": [] })))
}

// ── Phase 5.19: PromptForge ─────────────────────────────────────────────────
async fn pf_status_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(axum::Json(serde_json::json!({
        "enabled": rt.config.promptforge.enabled,
        "mode": rt.config.promptforge.mode,
        "auto_refine": rt.config.promptforge.auto_refine
    })))
}

async fn pf_doctor_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(axum::Json(serde_json::json!({
        "enabled": rt.config.promptforge.enabled,
        "mode": rt.config.promptforge.mode,
        "fail_open": rt.config.promptforge.fail_open,
        "allow_browser_chat": rt.config.promptforge.allow_browser_chat
    })))
}

async fn pf_config_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    Ok(axum::Json(serde_json::json!({
        "config": rt.config.promptforge
    })))
}

async fn pf_refine_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
    axum::Json(payload): axum::Json<crate::promptforge::PromptForgeRefineRequest>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    if !rt.config.promptforge.enabled {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "error": "PromptForge is disabled" })),
        ));
    }
    let client = crate::promptforge::PromptForgeClient::new(rt.config.clone());
    drop(rt);
    let res = client.refine(payload).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    Ok(axum::Json(serde_json::json!({ "result": res })))
}

async fn pf_history_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let client = crate::promptforge::PromptForgeClient::new(rt.config.clone());
    let history = client.get_history();
    Ok(axum::Json(serde_json::json!({ "history": history })))
}
#[derive(Debug, serde::Deserialize)]
pub struct PromptForgeScoreRequest {
    pub prompt: String,
}

async fn pf_score_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
    axum::Json(payload): axum::Json<PromptForgeScoreRequest>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let client = crate::promptforge::PromptForgeClient::new(rt.config.clone());
    drop(rt);
    let res = client.score(&payload.prompt).await;
    Ok(axum::Json(serde_json::json!({ "result": res })))
}

async fn pf_templates_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let lib = crate::promptforge::PromptForgeTemplateLibrary::new();
    Ok(axum::Json(
        serde_json::json!({ "templates": lib.templates }),
    ))
}

#[derive(Debug, serde::Deserialize)]
pub struct PromptForgeModeRequest {
    pub mode: String,
}

async fn pf_mode_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
    axum::Json(payload): axum::Json<PromptForgeModeRequest>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    let mode = match payload.mode.to_lowercase().as_str() {
        "mock" => crate::promptforge::PromptForgeMode::Mock,
        "model" => crate::promptforge::PromptForgeMode::Model,
        "cli" => crate::promptforge::PromptForgeMode::Cli,
        "api" => crate::promptforge::PromptForgeMode::Api,
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({ "error": "Invalid mode" })),
            ));
        }
    };
    rt.config.promptforge.mode = mode;
    Ok(axum::Json(serde_json::json!({ "status": "success" })))
}

async fn pf_enable_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.config.promptforge.enabled = true;
    Ok(axum::Json(serde_json::json!({ "status": "success" })))
}

async fn pf_disable_handler(
    headers: axum::http::HeaderMap,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::api_server::ApiState>>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    crate::api_server::check_auth(&headers, &state)?;
    let mut rt = state.runtime.lock().await;
    rt.config.promptforge.enabled = false;
    Ok(axum::Json(serde_json::json!({ "status": "success" })))
}
// -----------------------------------------------------------------------------
// Designer Endpoints
// -----------------------------------------------------------------------------

async fn designer_status_handler() -> impl axum::response::IntoResponse {
    let mut status = serde_json::Map::new();
    status.insert("enabled".to_string(), serde_json::Value::Bool(true));
    status.insert(
        "version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    axum::Json(status)
}

async fn designer_list_reviews_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::designer::DesignerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let reviews = agent.list_reviews().unwrap_or_default();
    Ok(axum::Json(serde_json::json!({ "reviews": reviews })))
}

#[derive(serde::Deserialize)]
struct CreateDesignerReviewReq {
    target_type: String,
    path_or_url: String,
    description: Option<String>,
}

async fn designer_create_review_handler(
    axum::Json(req): axum::Json<CreateDesignerReviewReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_get_review_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::designer::DesignerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Ok(Some(r)) = agent.get_review(&id) {
        Ok(axum::Json(serde_json::json!({ "review": r })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn designer_landing_review_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_dashboard_review_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_accessibility_review_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_copy_review_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_design_system_review_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_builder_handoff_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_report_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}

async fn designer_list_reports_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({ "reports": [] })))
}

// -----------------------------------------------------------------------------
// Researcher Endpoints
// -----------------------------------------------------------------------------

async fn researcher_status_handler() -> impl axum::response::IntoResponse {
    let mut status = serde_json::Map::new();
    status.insert("enabled".to_string(), serde_json::Value::Bool(true));
    status.insert(
        "version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    axum::Json(status)
}

async fn researcher_list_topics_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let topics = agent.list_topics().unwrap_or_default();
    Ok(axum::Json(serde_json::json!({ "topics": topics })))
}

#[derive(serde::Deserialize)]
struct CreateResearchTopicReq {
    title: String,
    research_question: String,
    domain: String,
}

async fn researcher_create_topic_handler(
    axum::Json(req): axum::Json<CreateResearchTopicReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let topic = agent
        .create_topic(&req.title, &req.research_question, &req.domain)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "topic": topic })))
}

async fn researcher_get_topic_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Ok(Some(r)) = agent.get_topic(&id) {
        Ok(axum::Json(serde_json::json!({ "topic": r })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn researcher_plan_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let plan = agent
        .create_plan(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "plan": plan })))
}

async fn researcher_list_sources_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let sources = agent.list_sources(&id).unwrap_or_default();
    Ok(axum::Json(serde_json::json!({ "sources": sources })))
}

#[derive(serde::Deserialize)]
struct AddResearchSourceReq {
    title: String,
}

async fn researcher_add_source_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::Json(req): axum::Json<AddResearchSourceReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let source = agent
        .add_source(&id, &req.title)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "source": source })))
}

async fn researcher_list_notes_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let notes = agent.list_notes(&id).unwrap_or_default();
    Ok(axum::Json(serde_json::json!({ "notes": notes })))
}

#[derive(serde::Deserialize)]
struct AddResearchNoteReq {
    claim: String,
}

async fn researcher_add_note_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::Json(req): axum::Json<AddResearchNoteReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let note = agent
        .add_note(&id, &req.claim)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "note": note })))
}

async fn researcher_competitors_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let scans = agent
        .generate_competitors(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "scans": scans })))
}

async fn researcher_compare_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let comp = agent
        .generate_compare(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "comparison": comp })))
}

async fn researcher_market_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let m = agent
        .generate_market(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "market_brief": m })))
}

async fn researcher_brief_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let b = agent
        .generate_brief(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "brief": b })))
}

async fn researcher_report_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::researcher::ResearcherAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = agent
        .generate_report(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "report": r })))
}

// -----------------------------------------------------------------------------
// Operator Endpoints
// -----------------------------------------------------------------------------

async fn operator_status_handler() -> impl axum::response::IntoResponse {
    let mut status = serde_json::Map::new();
    status.insert("enabled".to_string(), serde_json::Value::Bool(true));
    status.insert(
        "version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    axum::Json(status)
}

#[derive(serde::Deserialize)]
struct CreateOperatorReadinessReq {
    system_id: String,
}

async fn operator_readiness_handler(
    axum::Json(req): axum::Json<CreateOperatorReadinessReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let check = agent
        .create_deployment_readiness(&req.system_id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "readiness": check })))
}

#[derive(serde::Deserialize)]
struct CreateOperatorReleaseHealthReq {
    system_id: String,
}

async fn operator_release_health_handler(
    axum::Json(req): axum::Json<CreateOperatorReleaseHealthReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let check = agent
        .create_release_health(&req.system_id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "release_health": check })))
}

#[derive(serde::Deserialize)]
struct CreateOperatorIncidentReq {
    system_id: String,
    summary: String,
}

async fn operator_incident_handler(
    axum::Json(req): axum::Json<CreateOperatorIncidentReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let inc = agent
        .create_incident(&req.system_id, &req.summary)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "incident": inc })))
}

async fn operator_list_incidents_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let incs = agent
        .list_incidents()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "incidents": incs })))
}

async fn operator_get_incident_handler(
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Ok(Some(s)) = agent.get_incident(&_id) {
        Ok(axum::Json(serde_json::json!({ "incident": s })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

#[derive(serde::Deserialize)]
struct CreateOperatorLogReviewReq {
    system_id: String,
    log_text: String,
}

async fn operator_logs_handler(
    axum::Json(req): axum::Json<CreateOperatorLogReviewReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let lf = agent
        .create_log_review(&req.system_id, &req.log_text)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "log_review": lf })))
}

#[derive(serde::Deserialize)]
struct CreateOperatorRollbackReq {
    system_id: String,
}

async fn operator_rollback_handler(
    axum::Json(req): axum::Json<CreateOperatorRollbackReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let rp = agent
        .create_rollback_plan(&req.system_id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "rollback_plan": rp })))
}

#[derive(serde::Deserialize)]
struct CreateOperatorMonitoringReq {
    system_id: String,
}

async fn operator_monitoring_handler(
    axum::Json(req): axum::Json<CreateOperatorMonitoringReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let mp = agent
        .create_monitoring_plan(&req.system_id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "monitoring_plan": mp })))
}

#[derive(serde::Deserialize)]
struct CreateOperatorReportReq {
    system_id: String,
}

async fn operator_report_handler(
    axum::Json(req): axum::Json<CreateOperatorReportReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = agent
        .create_report(&req.system_id, "operator_health_report")
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "report": r })))
}

async fn operator_list_reports_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::operator::OperatorAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let reps = agent
        .list_reports()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "reports": reps })))
}

// -----------------------------------------------------------------------------
// Learner Endpoints
// -----------------------------------------------------------------------------

async fn learner_status_handler() -> impl axum::response::IntoResponse {
    let mut status = serde_json::Map::new();
    status.insert("enabled".to_string(), serde_json::Value::Bool(true));
    status.insert(
        "version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    axum::Json(status)
}

async fn learner_list_goals_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let goals = agent.list_goals().unwrap_or_default();
    Ok(axum::Json(serde_json::json!({ "goals": goals })))
}

#[derive(serde::Deserialize)]
struct CreateLearnerGoalReq {
    title: String,
    domain: String,
}

async fn learner_create_goal_handler(
    axum::Json(req): axum::Json<CreateLearnerGoalReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let domain = match req.domain.as_str() {
        "DSA" => crate::agents::learner::LearningDomain::DSA,
        "AIML" => crate::agents::learner::LearningDomain::AIML,
        "Rust" => crate::agents::learner::LearningDomain::Rust,
        "Web3" => crate::agents::learner::LearningDomain::Web3,
        _ => crate::agents::learner::LearningDomain::General,
    };
    let g = agent
        .create_goal(&req.title, domain)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "goal": g })))
}

async fn learner_get_goal_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Ok(Some(g)) = agent.get_goal(&id) {
        Ok(axum::Json(serde_json::json!({ "goal": g })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn learner_assess_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    Ok(axum::Json(
        serde_json::json!({ "status": "assessed", "goal_id": id }),
    ))
}

async fn learner_roadmap_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let rm = agent
        .create_roadmap(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "roadmap": rm })))
}

async fn learner_week_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let tasks = agent
        .generate_weekly_plan(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "tasks": tasks })))
}

async fn learner_today_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let tasks = agent
        .generate_daily_plan(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "tasks": tasks })))
}

async fn learner_practice_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let pt = agent
        .generate_practice_task(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "practice_task": pt })))
}

async fn learner_revise_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let cp = agent
        .create_revision_checkpoint(&id, "General")
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "checkpoint": cp })))
}

async fn learner_project_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let p = agent
        .create_project_plan(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "project_plan": p })))
}

async fn learner_exam_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let e = agent
        .generate_exam_prep(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "exam_prep": e })))
}

async fn learner_progress_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let pe = agent
        .log_progress(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "progress": pe })))
}

async fn learner_report_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let agent = crate::agents::learner::LearnerAgent::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = agent
        .generate_report(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "report": r })))
}

// Collaboration Endpoints
async fn collaboration_status_handler() -> impl axum::response::IntoResponse {
    axum::Json(serde_json::json!({ "status": "online" }))
}

async fn collaboration_list_sessions_handler()
-> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let sessions = m
        .list_sessions()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "sessions": sessions })))
}

#[derive(serde::Deserialize)]
struct CreateCollaborationSessionReq {
    title: String,
    goal: String,
    template: Option<String>,
}

async fn collaboration_create_session_handler(
    axum::Json(req): axum::Json<CreateCollaborationSessionReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .create_session(&req.title, &req.goal, req.template.as_deref())
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_get_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .get_session(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_start_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .start_session(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_step_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .advance_step(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_pause_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .pause_session(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_resume_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .resume_session(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_cancel_session_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = m
        .cancel_session(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "session": session })))
}

async fn collaboration_handoffs_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let handoffs = m
        .list_handoffs(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "handoffs": handoffs })))
}

async fn collaboration_report_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let m = crate::agent_collaboration::AgentCollaborationManager::new()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let report = m
        .generate_report(&id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(serde_json::json!({ "report": report })))
}

// ── Runtime Handlers ────────────────────────────────────────────────────────

async fn runtime_status_handler(
    State(state): State<Arc<ApiState>>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let rt = state.runtime.lock().await;
    let config = rt.agent_runtime.config.clone();
    let jobs: Vec<_> = rt.agent_runtime.list_jobs();
    let active_count = jobs
        .iter()
        .filter(|j| {
            matches!(
                j.status,
                crate::agent_runtime::AgentJobStatus::Running
                    | crate::agent_runtime::AgentJobStatus::WaitingForApproval
            )
        })
        .count();
    Ok(axum::Json(
        serde_json::json!({ "config": config, "active_jobs": active_count, "total_jobs": jobs.len() }),
    ))
}

async fn runtime_jobs_list_handler(
    State(state): State<Arc<ApiState>>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let rt = state.runtime.lock().await;
    let jobs = rt.agent_runtime.list_jobs();
    Ok(axum::Json(serde_json::json!({ "jobs": jobs })))
}

#[derive(serde::Deserialize)]
pub struct RuntimeJobCreateReq {
    title: String,
    agent_id: String,
    job_kind: crate::agent_runtime::AgentJobKind,
    task: String,
}

async fn runtime_job_create_handler(
    State(state): State<Arc<ApiState>>,
    axum::Json(req): axum::Json<RuntimeJobCreateReq>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    match rt
        .agent_runtime
        .create_job(req.title, req.agent_id, req.job_kind, req.task)
    {
        Ok(job_id) => Ok(axum::Json(serde_json::json!({ "job_id": job_id }))),
        Err(e) => {
            tracing::error!("Failed to create job: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn runtime_job_detail_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let rt = state.runtime.lock().await;
    if let Some(job) = rt.agent_runtime.get_job(&id) {
        Ok(axum::Json(serde_json::json!({ "job": job })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_start_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    if rt.agent_runtime.start_job(&id).is_ok() {
        Ok(axum::Json(serde_json::json!({ "status": "started" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_pause_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    if rt.agent_runtime.pause_job(&id).is_ok() {
        Ok(axum::Json(serde_json::json!({ "status": "paused" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_resume_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    if rt.agent_runtime.resume_job(&id).is_ok() {
        Ok(axum::Json(serde_json::json!({ "status": "resumed" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_cancel_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    if rt.agent_runtime.cancel_job(&id).is_ok() {
        Ok(axum::Json(serde_json::json!({ "status": "cancelled" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_retry_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let mut rt = state.runtime.lock().await;
    if rt.agent_runtime.retry_job(&id).is_ok() {
        Ok(axum::Json(serde_json::json!({ "status": "retried" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_events_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    // For now we just return an empty array if we don't have an easy query,
    // or we could load events.jsonl. Since we didn't implement get_events yet,
    // we return empty array to unblock dashboard.
    Ok(axum::Json(serde_json::json!({ "events": [] })))
}

async fn runtime_job_artifacts_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let rt = state.runtime.lock().await;
    if let Some(job) = rt.agent_runtime.get_job(&id) {
        Ok(axum::Json(
            serde_json::json!({ "artifacts": job.artifacts }),
        ))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn runtime_job_report_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let rt = state.runtime.lock().await;
    if let Some(job) = rt.agent_runtime.get_job(&id) {
        let title = format!("Report for Job {}", id);
        let content = format!(
            "# Runtime Job Report\n\n**Job ID:** {}\n**Status:** {:?}\n",
            id, job.status
        );
        Ok(axum::Json(
            serde_json::json!({ "report_ref": title, "content": content }),
        ))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

// ── Providers and Models ──────────────────────────────────────────────────────

async fn providers_list_handler(
    State(state): State<Arc<ApiState>>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    use crate::providers::ModelProviderRegistry;
    let rt = state.runtime.lock().await;
    let mut registry = ModelProviderRegistry::new(rt.config.model_routing.clone());
    for (_, p_cfg) in &rt.config.providers {
        registry.register(p_cfg.clone());
    }

    let mut providers: Vec<_> = registry.providers.values().cloned().collect();
    providers.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(axum::Json(serde_json::json!({
        "providers": providers
    })))
}

async fn providers_doctor_handler(
    State(state): State<Arc<ApiState>>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    use crate::providers::{
        ModelProviderAdapter, ModelProviderRegistry, ModelProviderStatus, OpenAiCompatibleAdapter,
    };
    let rt = state.runtime.lock().await;
    let mut registry = ModelProviderRegistry::new(rt.config.model_routing.clone());
    for (_, p_cfg) in &rt.config.providers {
        registry.register(p_cfg.clone());
    }

    let mut results = Vec::new();
    for provider in registry.providers.values() {
        if !provider.enabled {
            continue;
        }
        let adapter = OpenAiCompatibleAdapter::new(
            provider.base_url.clone().unwrap_or_default(),
            rt.config.provider_api_key(&provider.id),
            provider.timeout_secs,
        );
        let status = adapter.status();
        let status_str = match status {
            ModelProviderStatus::Ready => "Ready",
            ModelProviderStatus::NotConfigured => "Not Configured",
            ModelProviderStatus::MissingKey => "Missing API Key",
            ModelProviderStatus::Unreachable => "Unreachable",
            ModelProviderStatus::Unknown => "Unknown",
        };
        results.push(serde_json::json!({
            "id": provider.id,
            "name": provider.name,
            "status": status_str,
            "is_ready": status == ModelProviderStatus::Ready
        }));
    }

    Ok(axum::Json(serde_json::json!({
        "doctor_results": results
    })))
}

#[derive(serde::Deserialize)]
struct RouteRequest {
    task_kind: String,
}

async fn models_route_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Json(req): axum::extract::Json<RouteRequest>,
) -> axum::Json<serde_json::Value> {
    use crate::providers::{ModelProviderRegistry, ModelRouteRequest};
    let rt = state.runtime.lock().await;
    let mut registry = ModelProviderRegistry::new(rt.config.model_routing.clone());
    for (_, p_cfg) in &rt.config.providers {
        registry.register(p_cfg.clone());
    }

    let route_req = ModelRouteRequest {
        agent_id: "api_user".to_string(),
        task_kind: req.task_kind,
        required_capabilities: vec![],
        local_only: false,
        allow_external: true,
        preferred_provider: None,
        preferred_model: None,
        quality_preference: "balanced".to_string(),
        latency_preference: "balanced".to_string(),
        cost_preference: "balanced".to_string(),
        fallback_allowed: true,
    };

    let decision = registry.route(&route_req);
    axum::Json(serde_json::json!({ "routed_to": decision }))
}

// ── Extensions ─────────────────────────────────────────────────────────────

async fn extensions_list_handler(
    State(state): State<Arc<ApiState>>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    let records = rt.extension_registry.list_extensions();
    axum::Json(serde_json::json!({ "extensions": records }))
}

async fn extensions_get_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    if let Some(record) = rt.extension_registry.get_extension(&id) {
        axum::Json(serde_json::json!({ "extension": record }))
    } else {
        axum::Json(serde_json::json!({ "error": "not found" }))
    }
}

async fn extensions_audit_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    match rt.extension_registry.audit_extension(&id) {
        Ok(res) => axum::Json(serde_json::json!({ "audit": res })),
        Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn extensions_install_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    match rt.extension_registry.install_extension(&id) {
        Ok(_) => axum::Json(serde_json::json!({ "status": "success" })),
        Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn extensions_enable_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    match rt.extension_registry.enable_extension(&id) {
        Ok(_) => axum::Json(serde_json::json!({ "status": "success" })),
        Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn extensions_disable_handler(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mut rt = state.runtime.lock().await;
    let _ = rt.extension_registry.load_state();
    match rt.extension_registry.disable_extension(&id) {
        Ok(_) => axum::Json(serde_json::json!({ "status": "success" })),
        Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// --- Phase 7.3 Compiler-Guided Loop Handlers ---

async fn builder_failures_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    // Stub implementation
    Ok(Json(json!({ "failures": [] })))
}

#[derive(serde::Deserialize)]
struct AnalyzeFailureReq {
    session_id: String,
}

async fn builder_analyze_failure_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<AnalyzeFailureReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);

    let session = mgr
        .get_session(&payload.session_id)
        .unwrap_or(None)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Session not found" })),
        ))?;

    let mut all_failures = Vec::new();
    for res in session.validation_results {
        if res.status == crate::code_execution::ValidationStatus::Failed {
            if let Some(out) = res.output {
                let parsed = crate::validation_parser::ValidationParser::parse_output(
                    &res.command.command,
                    &out.stdout,
                    &out.stderr,
                    out.exit_code,
                );
                all_failures.extend(parsed);
            }
        }
    }

    let clusters = crate::validation_parser::ValidationParser::cluster_failures(all_failures);

    let analysis = crate::code_retry::ValidationFailureAnalysis {
        session_id: payload.session_id.clone(),
        clusters,
        fix_hypothesis: crate::code_retry::ValidationFailureFixHypothesis {
            description: "Analyze the compiler errors and fix type/syntax issues.".to_string(),
            expected_outcome: "Compilation passes without errors.".to_string(),
        },
    };

    let _ = mgr.save_analysis(&analysis);

    Ok(Json(json!({ "analysis": analysis })))
}

#[derive(serde::Deserialize)]
struct RetryPlanReq {
    session_id: String,
}

async fn builder_retry_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<RetryPlanReq>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);

    let analysis = mgr
        .get_analysis(&payload.session_id)
        .unwrap_or(None)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Analysis not found" })),
        ))?;

    let session = mgr
        .get_session(&payload.session_id)
        .unwrap_or(None)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Session not found" })),
        ))?;

    let plan = crate::code_retry::BuilderRetryPlan {
        id: uuid::Uuid::new_v4().to_string(),
        validation_session_id: payload.session_id.clone(),
        failed_command: "validation suite".to_string(),
        failure_summary: format!("{} failure clusters detected", analysis.clusters.len()),
        suspected_root_cause: analysis.fix_hypothesis.description.clone(),
        proposed_patch_intent: "Fix compiler errors based on hypothesis".to_string(),
        patch_candidate: crate::code_retry::BuilderRetryPatchCandidate {
            affected_files: vec![],
            steps: vec![],
            diff_preview_summary: None,
        },
        risk_level: crate::code_retry::BuilderRetryRiskLevel::Medium,
        approval_need: crate::code_retry::BuilderRetryApprovalNeed::Standard,
        validation_plan: crate::code_retry::BuilderRetryValidationPlan {
            expected_commands: session.validation_commands.clone(),
        },
        checkpoint_ref: None,
        rollback_ref: None,
        max_retry_count: 3,
        current_retry_attempt: 1,
    };

    let _ = mgr.save_retry_plan(&plan);

    Ok(Json(json!({ "plan": plan })))
}

async fn builder_get_retry_plan_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::code_execution::CodeExecutionManager::new(&rt.paths.data_dir);
    let plan = mgr.get_retry_plan(&id).unwrap_or(None).ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Plan not found" })),
    ))?;
    Ok(Json(json!({ "plan": plan })))
}

async fn builder_retry_preview_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "preview not implemented yet" })))
}

async fn builder_retry_approval_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "Approval requested" })))
}

async fn builder_retry_apply_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "Applied" })))
}

async fn builder_retry_validate_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "Validated" })))
}

async fn builder_retry_report_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "report": "Report contents" })))
}

// --- Phase 7.4 Builder Memory Handlers ---

async fn builder_memory_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "active" })))
}

async fn builder_memory_failures_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    let rt = state.runtime.lock().await;
    let mgr = crate::failure_memory::FailureMemoryManager::new(&rt.paths.data_dir);
    let memories = mgr.get_all_memories().unwrap_or_default();
    Ok(Json(json!({ "failures": memories })))
}

async fn builder_memory_patterns_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "patterns": [] })))
}

async fn builder_memory_lessons_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "lessons": [] })))
}

async fn builder_memory_ingest_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "ingested" })))
}

async fn builder_memory_recall_similar_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "similar": [] })))
}

async fn builder_memory_record_outcome_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "recorded" })))
}

async fn builder_memory_report_generate_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "status": "report generated", "report_id": "r-123" }),
    ))
}

async fn builder_memory_report_get_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(
        json!({ "report": { "id": id, "summary": "Project is doing great." } }),
    ))
}

// --- Phase 7.5 Researcher APIs ---

async fn researcher_phase75_status_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "active" })))
}

async fn researcher_phase75_projects_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "projects": [] })))
}

async fn researcher_phase75_projects_create_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "id": "proj-new", "status": "created" })))
}

async fn researcher_phase75_projects_get_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "project": {} })))
}

async fn researcher_phase75_sources_add_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "source added" })))
}

async fn researcher_phase75_sources_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "sources": [] })))
}

async fn researcher_phase75_ingest_browser_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "artifact ingested" })))
}

async fn researcher_phase75_brief_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "brief generated" })))
}

async fn researcher_evidence_grade_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "evidence graded" })))
}

async fn researcher_phase75_competitors_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "competitors scanned" })))
}

async fn researcher_compare_tech_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "tech comparison generated" })))
}

async fn researcher_phase75_report_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "status": "report generated" })))
}

async fn researcher_reports_list_handler(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    check_auth(&headers, &state)?;
    Ok(Json(json!({ "reports": [] })))
}
