# Security Policy

Security and user sovereignty are core to GOAT's design. Because GOAT executes code and commands on your behalf, we enforce a strict local-first and consent-driven model.

## Reporting a Vulnerability

**DO NOT report security vulnerabilities via public GitHub issues.**

Please email security reports to the maintainers directly (contact info will be provided upon full release, for now please create a placeholder issue asking for a private security contact). We will coordinate a fix and release it responsibly.

**Do not include secrets, API keys, or sensitive logs in any bug reports.**

## Local-First Security Model

* **ApprovalGate:** All dangerous actions (e.g., executing shell commands, modifying files, making network requests) are intercepted by the `ApprovalGate`. The user is prompted to approve or deny the action.
* **No Telemetry:** We do not send your data, code, or conversations to any central server.
* **Opt-in Cloud:** Connecting to external services (like OpenAI or Discord) requires explicit configuration by the user.

## Risky Tool Execution

GOAT will occasionally propose running shell commands or installing dependencies. It is your responsibility to review the proposed commands in the ApprovalGate prompt before accepting them.
