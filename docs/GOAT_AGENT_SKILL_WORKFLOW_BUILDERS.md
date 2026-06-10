# Agent, Skill, and Workflow Builders

## Agent Builder
Allows you to define subagent profiles.
- **Inputs**: Name, Description, System Prompt, Required Tools, Model Requirements.
- **Safety**: By default, subagents are isolated and have no tools assigned. Any tool injection requires manual review.

## Skill Builder
Allows you to build functional blocks of prompt chains and tool sequences.
- **Inputs**: Skill Name, Trigger patterns, Workflow sequence, Dependencies.
- **Memory Integration**: You can convert successful interaction patterns from the `Memory Galaxy` directly into reusable skills.

## Workflow Builder
Allows you to orchestrate sequences involving multiple agents or skills.
- **Inputs**: Workflow Name, Trigger Events, Directed Acyclic Graph (DAG) of steps.
- **Safety**: Workflows that perform actions in `Act` mode require `ApprovalGate` clearance before deployment.

## Architecture
Drafts are serialized as JSON in the local storage path. The `StudioManager` in the backend reads and writes these drafts. When a user finalizes a draft, the backend validates it against security constraints, requests approval via `ApprovalGate` (for actions), and copies the asset into the active `src/skills/` or `src/subagents/` registry.
