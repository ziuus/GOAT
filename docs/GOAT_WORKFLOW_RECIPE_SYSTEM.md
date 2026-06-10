# GOAT Workflow Recipe System

Workflows in GOAT are chained sequences of tools, skills, or model invocations.
A Workflow Recipe is an encapsulation of these chains.

## Conversion to Templates
Installed Workflow Recipes can be converted into:
- **Hook Templates**: Attach to `git-pre-commit` or file changes.
- **Schedule Templates**: Attach to the internal Cron scheduler.
- **Job Templates**: Create Async Jobs.

Enabling a template creates the corresponding Hook, Schedule, or Job.

## Security
Workflow steps execute via the existing `ToolRegistry` and `ApprovalGate`. High-risk operations (e.g. `write_file`) will block and ask for user approval unless explicitly overridden.
