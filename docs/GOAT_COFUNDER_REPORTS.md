# GOAT Cofounder Reports

The Cofounder Agent leverages the GOAT `ReportManager` to persist its findings and strategies in standard Markdown.

## Generated Reports

### 1. Validation Report (`validation_report`)
Contains the results of the `validate-idea` workflow. Outlines the riskiest assumptions and proposes multiple manual or low-code tests (e.g., Concierge MVP, Landing page signups) to prove or disprove them.

### 2. MVP Scope Report (`mvp_scope_report`)
The result of the `mvp` workflow. Explicitly lists the core user problem, the smallest possible feature set, and a crucial "Excluded Features" section to combat scope creep. Can be handed off to the Builder agent.

### 3. Competitor Report (`competitor_report`)
Synthesizes the competitor scan into a structured comparison. Highlights the gaps in the market and how the current idea should position itself against existing alternatives.

### 4. Founder Weekly Report (`founder_weekly_report`)
A master synthesis of the idea's state. Includes:
- Idea Summary
- Latest Scorecard
- Current Validation Plan
- MVP Scope
- Next Actions

## Storage & Retrieval
Cofounder reports are saved in:
`~/.local/share/goat/reports/cofounder/`

They can be viewed in the Dashboard under the `/reports` route or via the `/cofounder show` command in the CLI.
