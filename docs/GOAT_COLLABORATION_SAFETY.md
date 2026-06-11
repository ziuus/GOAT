# GOAT Collaboration Safety

- **No hidden background agents**: All collaboration steps are explicitly visible.
- **ApprovalGate Integration**: Any step requiring destructive commands or external actions is paused in `WaitingForApproval` state until explicitly approved.
- **Handoff validation**: No agent can override another agent's safety policy.
- **Resumable**: User can pause, resume, or cancel any session.
