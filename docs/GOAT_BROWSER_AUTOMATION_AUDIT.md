# GOAT Browser Automation Audit (Phase 6.9)

## 1. What Browser Automation Exists Now
Currently, GOAT implements a lightweight `BrowserAdapterManager` (defined in `src/browser_adapter.rs`) with:
- **`BrowserProvider` Trait**: Outlines `start_session`, `close_session`, `execute_action`, and `check_health`.
- **`ManualStubProvider`**: A mock provider that simulates actions and returns pre-defined observations.
- **API Endpoints**: `/v1/browser/status`, `/v1/browser/doctor`, `/v1/browser/open`, `/v1/browser/screenshot`, `/v1/browser/read`, `/v1/browser/qa`.
- **Slash Commands**: `/browser open`, `/browser screenshot`, `/browser read`, `/browser qa`.
- **Approval Gate check**: Routes `BrowserRiskLevel` actions through `ApprovalGate` validation.

## 2. What Is Stubbed / Partial
- Playwright and `browser-use` backends are mapped to `ManualStubProvider`.
- Active screenshot capture returns `None` and prints simulation logs.
- DOM and text inspection return simulated page structure.
- Desktop automation (screen, mouse, keyboard control) is stubbed/planned.

## 3. Security & Approval Flow
- `BrowserActionKind` categorizes actions into `Low`, `Medium`, `High`, and `Critical` risks.
- `ApprovalGate` validates `Medium` and `High` risk operations before running.
- Hidden browser sessions are prohibited; sessions must report to the UI.

## 4. Planned in Phase 6.9
- **Structured Workflows**: Introducing `BrowserWorkflow` supporting steps (`open_url`, `capture_screenshot`, `inspect_dom`, `run_ui_qa`, etc.).
- **Specific Workflows**: `ui-qa`, `landing-review`, `dashboard-qa`, and `web-health-check`.
- **Improved UI Page**: An upgraded dashboard interface allowing interactive workflow runs, screenshot previews, and artifact lists.
- **Local persistence**: Save workflows, steps, and screenshots in `~/.local/share/goat/browser_workflows/`.
