# GOAT Agent Collaboration

Phase 5.26 introduces the Prime Agent Collaboration Layer + Live Execution Feedback.

## Architecture
The `AgentCollaborationManager` connects GOAT’s Prime Agents (Cofounder, Socializer, Designer, Researcher, Operator, Learner, Builder) into a visible, safe collaboration system.
This is not a fully autonomous swarm. It is a controlled collaboration system.

## Handoff Model
Agents communicate through `AgentHandoff` records which include:
- `from_agent` and `to_agent`
- `context_summary`
- `output_expected`
- `safety_notes`

## Built-in Templates
- `startup-validation-flow`: Cofounder -> Researcher -> Designer -> Socializer -> Builder
- `launch-readiness-flow`: Cofounder -> Designer -> Socializer -> Operator
- `build-and-release-flow`: Builder -> Operator -> Researcher -> Report
- `learning-project-flow`: Learner -> Researcher -> Builder -> Designer -> Report
- `incident-response-flow`: Operator -> Researcher -> Builder -> Operator

## Live Feedback
Events are broadcasted via `src/events.rs`. Dashboard consumes these for real-time status.

## ApprovalGate
Risky steps are marked `required_approval: true` and will pause the session until user approves.
