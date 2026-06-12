# GOAT Unified Activity Feed

## Concept
The Unified Activity Feed solves the "where did my output go?" problem. It provides a single chronological view of everything GOAT is doing, has done, and is waiting to do.

## Sources
The feed aggregates data from multiple distinct backend systems:
1. **Timeline**: General system events (`mission_goal_planned`, `project_created`).
2. **Agent Runtime**: Background execution statuses (e.g., Builder compiling, Researcher searching).
3. **Reports**: Completed documents and artifacts.
4. **AgentFlow**: Collaboration and session interactions.
5. **ApprovalGate**: Security and execution prompts requiring human intervention.
6. **Browser Adapter**: Web automation steps and captured screenshots.

## Card Design
Each item in the feed is rendered as a clean UI card containing:
- **Action**: What happened (e.g., "Report Generated", "Build Failed", "Pending Approval").
- **Agent**: Which agent performed the action (e.g., `@builder`, `@researcher`).
- **Project**: The associated project context.
- **Link**: Direct link to the full output/report or approval interface.
- **Status**: Success, Failed, Pending, Active.
- **Timestamp**: When the event occurred.

## Value Proposition
By merging these streams, the user gets the true "Team OS" experience. They can watch the Operator agent diagnose a bug, the Builder agent apply a patch, and the ApprovalGate pause the process for a security check—all flowing through one central feed in Mission Control.
