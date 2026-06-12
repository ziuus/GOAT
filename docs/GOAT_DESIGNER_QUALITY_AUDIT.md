# GOAT Designer Quality Audit

## Current State of Designer
Currently, the `DesignerAgent` is essentially a stub implementation. It defines basic structures for `DesignerReview`, `DesignerScorecard`, `DesignerIssue`, `DesignerImprovementPlan`, and `DesignerHandoffBrief`. However, all methods (`run_scorecard`, `run_accessibility_check`, `run_responsive_check`, `create_improvement_plan`, `create_handoff_brief`) are entirely hardcoded to return dummy data.

## What is shallow/simple
- **Visual Evidence:** Designer currently does not process or even link to any visual evidence.
- **Accessibility:** It fakes an accessibility check by blindly returning a hardcoded missing ARIA label issue.
- **Design System:** It does not review design system consistency at all.
- **Builder Handoff:** It generates a mock Builder handoff brief.

## What screenshot/browser artifact support exists
- `BrowserWorkflow` generates DOM extracts, screenshots, and visual artifacts. These can be wired into the Designer so it actually grounds its reviews in reality.

## What accessibility checks exist
- The browser workflows can perform accessibility risk extraction, but the Designer doesn't currently use them.

## What design system checks exist
- None currently.

## What Builder handoff exists
- A stub exists. A real Builder handoff should be planning only, without auto-editing files, focusing on providing actionable UI/UX requirements for the Builder.

## What dashboard actions are wired
- There is a dashboard placeholder (`/designer`) but it doesn't do anything yet.

## What is implemented in this phase
1. **Designer Review Model:** A new data model based on `DesignerReviewKind`, `DesignerFinding`, `DesignerRecommendation`, `DesignerEvidenceRef`, etc.
2. **Visual Hierarchy & Accessibility Risk Reviews:** Asynchronous logic utilizing the LLM router to generate structured critique backed by DOM/Screenshot artifacts if available, and explicitly stating limitations if not.
3. **Design System & Copy Hierarchy Reviews.**
4. **Builder Handoff:** Generating targeted UI improvement plans.
5. **Runtime Jobs:** Wiring the Designer into the `AgentRuntime` for real orchestration.
6. **Dashboard:** Upgrading `/designer` to be fully interactive.

## What remains partial
- Automatic image processing (screenshot analysis) might be limited by the configured LLM's vision capabilities. If the LLM doesn't support vision natively, Designer will rely strictly on DOM extracts and layout bounding boxes provided by the Browser workflow, clearly stating this limitation in its reports.
- Full WCAG compliance certification is impossible via automated LLMs, so we explicitly call this an "Accessibility Risk Review".
- Cofounder and Socializer integrations will be partial in this phase, largely acting as consumers of the Designer's output rather than driving autonomous loops.
