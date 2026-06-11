# GOAT Cofounder Workflows

The Cofounder Agent exposes several primary workflows designed to take an idea from conception to validation and scoping.

## Workflow List

### 1. Idea Intake
- **Trigger**: `/cofounder new-idea` or Dashboard "New Idea" form.
- **Action**: Captures the raw idea, target user, pain point, and proposed solution.
- **Output**: Stores a `CofounderIdea` record.

### 2. Validate Idea
- **Trigger**: `/cofounder validate <id>` or `@cofounder validate <idea>`.
- **Action**: Generates a `CofounderValidationPlan`. Identifies the riskiest assumptions and proposes tests (concierge, landing page, interviews) to validate them without writing code.
- **Output**: Saves validation plan, emits timeline event.

### 3. Score Idea
- **Trigger**: `/cofounder score <id>`.
- **Action**: Evaluates the idea across 10 dimensions (Pain Intensity, Frequency, etc.).
- **Output**: Generates a `CofounderScorecard` with total score, strengths, risks, and a final recommendation (e.g. "Validate First", "Avoid").

### 4. MVP Scope
- **Trigger**: `/cofounder mvp <id>` or `@cofounder mvp <idea>`.
- **Action**: Aggressively scopes down the proposed solution to its absolute core. Defines explicit "Excluded Features".
- **Output**: Generates a `CofounderMvpScope`.

### 5. Competitor Scan
- **Trigger**: `/cofounder competitors <id>`.
- **Action**: Records known competitors, their positioning, pricing, strengths, and weaknesses. Identifies the "Gap/Opportunity".
- **Output**: Saves `CofounderCompetitorRef` entries.

### 6. Landing Page Brief
- **Trigger**: `/cofounder landing <id>`.
- **Action**: Drafts the structural elements of a landing page (Headline, Subheadline, Promise, CTA, Trust Section).
- **Output**: Can be handed off to the Designer agent later.

### 7. Outreach Plan
- **Trigger**: `/cofounder outreach <id>`.
- **Action**: Drafts ethical, non-spammy outreach messages for Reddit, LinkedIn, X, or cold email.
- **Output**: Saves a `CofounderOutreachPlan`. *Note: These are drafts only. Cofounder does not auto-send.*

### 8. Founder Weekly Report
- **Trigger**: `/cofounder report <id>`.
- **Action**: Synthesizes the current state of the idea, scorecard, validations, and MVP scope into a structured markdown report.
- **Output**: Generates a `founder_weekly_report` using the `ReportManager`.
