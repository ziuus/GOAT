# Phase 5.12: Browser / Computer-Use Adapter Layer Plan

## 1. Browser Adapter Architecture
The adapter layer (`src/browser_adapter.rs`) will define the core abstractions:
- `BrowserAdapterManager`: The central manager orchestrating browser tasks.
- `BrowserProvider`: Trait defining the capabilities of a provider (e.g., `manual_stub`, `playwright`, `browser_use`).
- `BrowserSession`: Represents an active browser session.
- `BrowserTask`: Represents a complete QA/test sequence containing multiple `BrowserAction`s.
- `BrowserAction`: A single command (e.g., `open_url`, `screenshot`, `click`).
- `BrowserActionResult`: Output of an action containing `BrowserObservation` (e.g., extracted text, visual summaries).

## 2. Action Risk Model
Actions are classified into three risk levels:
- **Low**: `read_text`, `screenshot` (localhost), `open_url` (localhost). These typically bypass `ApprovalGate` if configured safely.
- **Medium**: `screenshot` (external), `open_url` (external), `click`. These require `ApprovalGate` confirmation.
- **High/Critical**: `type_text` (external), `submit_form`, `download_file`, `upload_file`, `evaluate_readonly_js`. These always require strict `ApprovalGate` confirmation and cannot be bypassed.

## 3. Provider Detection
GOAT will check for installed browser automation tools:
- `manual_stub` is always available.
- `playwright` is detected by checking if `npx playwright --version` succeeds.
- `browser_use` is detected by checking for the CLI/server.
The command `/browser doctor` will run these checks and report back.

## 4. Dashboard Browser Page Design
The `apps/dashboard/src/app/browser/page.tsx` will feature:
- A control panel for launching tasks (e.g., `/browser qa http://localhost:3000`).
- A status section showing the detected provider.
- A Timeline / Logs view displaying recent screenshots, observations, and extracted text.
- Security badges denoting required approvals.
- Premium Antigravity styling focusing on transparency, glassmorphism, and smooth layout transitions.

## 5. Security Model
- **Disabled by default**: Users must explicitly opt-in via config (`[browser] enabled = true`).
- **ApprovalGate Integration**: Every medium/high risk action passes through `ApprovalGate.request_approval()`.
- **No Persistence**: Cookies and secrets are not stored by default.
- **Redaction**: Page text and task summaries are scrubbed before storage.
- **Local-Only**: Screenshots remain local to `~/.local/share/goat/browser-tasks/`.

## 6. Implementation Scope (Now vs Partial)
- **Implemented Now**: `BrowserAdapterManager`, config schema, provider detection framework, `manual_stub` provider, Risk Model integration with `ApprovalGate`, Dashboard `/browser` UI, `/v1/browser/*` API endpoints, CLI slash commands.
- **Partial/Planned**: Full Playwright and `browser_use` integration (stubbed for now to prevent breaking dependencies), deep AI visual evaluation (stubbed to mock LLM responses), cloud browser hosting (deferred).
