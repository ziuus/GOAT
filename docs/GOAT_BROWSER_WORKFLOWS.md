# GOAT Browser Workflows (Phase 6.9)

## 1. Overview
The Browser Workflow system enables GOAT to execute multi-step browser actions such as UI QA, landing page review, dashboard testing, and health checks safely. Workflows are composed of discrete steps, and their status is stored locally.

## 2. Predefined Workflows
- **`ui-qa`**: Open URL, capture screenshot, inspect DOM, check links, check forms, check accessibility, check responsive layout, generate report.
- **`landing-review`**: Inspect CTA sections, trust signals, headings, sections, and handoff to Designer.
- **`dashboard-qa`**: Validate local dashboard routes and capture states.
- **`web-health-check`**: Check site availability, record screenshots, and audit risks.

## 3. Workflow Steps & State
Each workflow progresses through statuses: `draft`, `queued`, `waiting_for_approval`, `running`, `paused`, `completed`, `failed`, and `cancelled`.
Workflow steps map to specific browser adapter calls.
Results are saved to `~/.local/share/goat/browser_workflows/workflows.jsonl`.
