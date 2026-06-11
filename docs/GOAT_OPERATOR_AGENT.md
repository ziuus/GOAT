# GOAT Prime Operator Agent

**Status:** Active
**Phase:** 5.23 Complete
**Tier:** Prime Agent

## Overview

The `OperatorAgent` is a Prime Agent responsible for operational workflows: System Health, Log Reviews, CI/CD Analysis, Incident Handling, Deployment Planning, and Rollback Preparation.

The defining characteristic of the Operator Agent is its **Safety-First** design:
1. **No Destructive Actions**: It does not restart production services automatically.
2. **ApprovalGate Native**: All deployment, rollback, and runbook executions generate plans that are routed through ApprovalGate.
3. **Secret Redaction**: Sensitive strings (API keys, tokens, passwords) are scrubbed from log findings.

## Storage
All data is stored in `~/.local/share/goat/agents/prime/operator/`:
- `systems.jsonl`: Monitored systems/projects.
- `health_checks.jsonl`: Point-in-time health evaluations.
- `log_findings.jsonl`: Redacted log anomaly reports.
- `incidents.jsonl`: Outage root cause and mitigation plans.
- `deployment_plans.jsonl`: Safe pre-flight and deploy steps.
- `rollback_plans.jsonl`: Condition-based revert instructions.
- `runbooks.jsonl`: Standard operating procedures.
- `reports/`: Full Markdown reliability reports.

## Architecture

- Uses `GoatPaths` for file-based JSONL storage.
- Intercepted by `PromptForge` (Optional Layer) for dynamic plan refinement.
- Dashboard accessible at `/operator` for point-and-click health reviews.
- Subcommands: `/operator list`, `system`, `health`, `logs`, `incident`, `deploy-plan`, `ci`, `rollback`, `runbook`, `reliability`, `report`.
- Aliases: `@operator`, `@ops`, `@health`, `@logs`, `@incident`, `@deploy`, `@rollback`.

## Security Rules

- **Bypass Prohibition**: Never bypass `ApprovalGate`.
- **Auto-Deploy**: Prohibited in Phase 1.
- **Log Exposure**: Must redact secrets before returning strings to timeline or UI.
- **Evidence-based Claims**: Cannot claim "system is 100% healthy" without running checks.

## Future Phases

- Automated safe execution via ApprovalGate hooks (Phase 6).
- Deeper Prometheus/Datadog integration.
