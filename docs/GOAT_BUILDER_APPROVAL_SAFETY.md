# GOAT Builder Approval & Safety

The Builder Agent strictly enforces `ApprovalGate` constraints before making any destructive or code-mutating actions.

## Security Policies
- **No Silent Mutations:** Patches or writes require explicit user validation.
- **Risk Grading:**
  - Low (e.g. read repo map) -> Auto-approved.
  - Medium/High (e.g. run test suite, modify files) -> Prompts the user via `ApprovalGate` or the Dashboard Approval queue.
- **Restricted Directories:** Writes to XDG configuration directory or SSH key locations are blocked immediately.
