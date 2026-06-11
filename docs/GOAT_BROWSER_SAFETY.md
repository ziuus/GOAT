# GOAT Browser Safety Model (Phase 6.9)

## 1. Safety Policy
Browser actions are categorized into four levels of risk:
- **Low**: Inspecting local/public DOM, reading text, capturing local screenshots. No approval required.
- **Medium**: Interactive tasks like clicking or entering text. Checked against session policies.
- **High**: Submitting forms, navigating multiple pages, file uploads, log-in attempts. Checked against policies.
- **Critical**: Remote side-effects, payment actions, or credential usage. Always blocked unless explicitly verified.

## 2. Constraints & Protections
- **Visible Sessions Only**: Hidden browser sessions are blocked. The Dashboard /browser reflects live operations.
- **Approval Gate**: ApprovalGate validates Medium and High risk actions.
- **No Secret Storage**: Cookies, sessions, passwords, and tokens are never persisted in workflows or database state.
