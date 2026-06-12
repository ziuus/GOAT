# GOAT Phase 7.9 Implementation Plan (Deep Operator Quality)

## Readiness / Release / Incident / Log Model Explanation
- **Deployment Readiness**: Evaluates if a system is prepared for a release. Driven by check lists (tests, builds, environment config, manual approvals, rollback plan existence). Crucially, this acts as a gatekeeper but never auto-deploys.
- **Release Health**: Post-deployment status evaluation. Analyzes signals like Builder validation reports, web health checks, and CI statuses to declare a system healthy, degraded, or failed.
- **Incident Workflow**: Safe triage state machine. Tracks incidents through Open -> Investigating -> Mitigated -> Resolved -> Closed. Defines severities (Sev0-Sev4) and tracks symptoms/hypotheses. Does NOT restart services or execute rollback commands automatically; instead, outputs recommended safe actions and Draft Postmortems.
- **Log Review Model**: Safe ingestion of logs. Identifies error/warning patterns, redacts sensitive values (like tokens or secrets), and generates debugging steps without claiming absolute root cause certainty without solid evidence.

## Browser / Builder Integration
- **Browser**: Operator will pull `browser_web_health_check` and `browser_dashboard_qa` results. Broken page risks and response/DOM/screenshot evidence refs will be linked into the `ReleaseHealthReport`.
- **Builder**: Operator will read Builder's test plans, validation results, and rollback plans. A `DeploymentReadinessCheck` will actively require a Builder rollback plan before declaring a system "ready" for release.

## What will be implemented vs partial
- **Implemented**: All core models (`OperatorCheck`, `OperatorIncident`, `OperatorRisk`, `OperatorDeploymentPlan`, etc.), integration with Brain (indexing), Provider Routing (for safe, non-external incident summary and log review), API routes with token auth, Dashboard Operator UI upgrade. Commands and aliases will be fully functional.
- **Partial/Omitted for Safety**: No actual executing of rollbacks. The `OperatorRollbackPlan` is generated and presented for manual ApprovalGate confirmation. No direct modification of infra configs or scraping of arbitrary huge logs. No active continuous monitoring background loops unless handled safely via Runtime jobs; mostly request-based generation for now.

## Task for Cline
1. Implement the new core models in `src/agents/operator.rs` (e.g., `DeploymentReadinessCheck`, `ReleaseHealthCheck`, `Incident`, `LogReview`, `OperatorRollbackPlan`). Use robust Rust structs with serde.
2. Integrate Runtime jobs (add `operator_deployment_readiness`, etc. to `AgentRuntime`).
3. Add/Upgrade API endpoints in `src/api_server.rs` (`GET /v1/operator/status`, `POST /v1/operator/readiness`, `POST /v1/operator/release-health`, `POST /v1/operator/web-health`, `POST /v1/operator/incident`, `GET /v1/operator/incidents`, `GET /v1/operator/incidents/:id`, `POST /v1/operator/logs`, `POST /v1/operator/rollback-plan`, `POST /v1/operator/monitoring-plan`, `POST /v1/operator/report`, `GET /v1/operator/reports`). All require token auth.
4. Upgrade the TUI (`src/app.rs`) to show Operator tasks properly (no blockings).
5. Upgrade the Dashboard (`apps/dashboard/src/app/operator/page.tsx` and related components) to have:
   - Deployment readiness panel
   - Release health panel
   - Incident intake form
   - Log review input
   - Safety notices (e.g. “Operator plans releases and incidents safely. It does not deploy or rollback without approval.”)
6. Add commands in `src/command_registry.rs` (`goat operator status`, etc. and slash commands `/operator`).
7. Update documentation (`docs/GOAT_OPERATOR_AGENT.md`, `README.md`, `CHANGELOG.md`, etc.).
8. Run Rust tests and ensure they pass.
9. Ensure Dashboard builds successfully.
