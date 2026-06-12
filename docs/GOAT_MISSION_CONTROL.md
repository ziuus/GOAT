# GOAT Mission Control Design

## Concept
Mission Control is the primary workspace of the GOAT Agent OS. Instead of users hopping between disparate agent pages (Builder, Cofounder, Researcher), they start here. It acts as the "Desktop" for the OS, unifying all interactions.

## Key Components

1. **Global Goal Input**:
   A single, central input box: "What are we building / solving today?"
   - Instead of executing directly, this triggers a **safe planning function**.
   - It outputs a structured local plan: classifying the goal, recommending a team of agents, and outlining required approvals.

2. **Unified Activity Feed**:
   A chronological, combined feed pulling from:
   - Timeline events
   - Runtime jobs
   - AgentFlow sessions
   - Builder activity

3. **Unified Artifact Panel**:
   A dedicated panel displaying recent outputs from various agents:
   - Researcher briefs
   - Cofounder validation reports
   - Builder plans
   - Browser artifacts

4. **Pending Approvals Panel**:
   A persistent widget showing any pending actions waiting at the ApprovalGate. This builds trust by making risky actions highly visible.

5. **Brain Context Summary**:
   A quick overview of recent memory ingestion and context pertinent to the active project.

## Implementation Details
- **Route**: `apps/dashboard/src/app/mission-control/page.tsx`
- **Backend APIs**:
  - `GET /v1/mission-control/status`
  - `GET /v1/mission-control/feed`
  - `GET /v1/mission-control/artifacts`
  - `GET /v1/mission-control/projects`
  - `POST /v1/mission-control/projects`
  - `POST /v1/mission-control/plan-goal`
  - `GET /v1/mission-control/recommendations`
- **UI Architecture**: Next.js React Server Components with live SSE updates (if applicable) or standard REST polling for feed data.

---

# What is Partial vs Implemented

**Implemented**:
- Mission Control dashboard layout.
- Real API endpoints fetching from existing subsystems (Timeline, Reports, Runtime Jobs).
- Safe deterministic planning for goals without autonomous side-effects.
- Sidebar and Home page navigational updates.

**Partial**:
- Deep Brain integration (will use metadata-based recent context rather than full semantic retrieval).
- Pending Approvals Panel (will show placeholder if a real approval REST API isn't fully ready yet, but will hook into the ApprovalGate structure).
