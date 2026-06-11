# GOAT Cofounder Agent Phase 1

The Cofounder Agent is a Prime Agent built into GOAT, designed to be an objective, evidence-first partner for makers, hackers, and entrepreneurs. Its primary purpose is to prevent the most common failure mode in software: building things nobody wants.

## Phase 1 Focus

Phase 1 of the Cofounder Agent implements the foundation of Idea Validation, MVP Scoping, and Founder Reports. It uses the `AgentRegistry`, `ReportManager`, and the new Cofounder specific logic located in `src/agents/cofounder.rs`.

### Core Capabilities

1. **Idea Intake & Storage**: Safely records startup ideas, preserving the target user, pain point, and proposed solution.
2. **Transparent Scoring**: Evaluates ideas across 10 dimensions (Pain Intensity, Frequency, Willingness to Pay, Reachability, Competition, Build Complexity, Trust Requirement, Distribution Difficulty, Speed to Validate, Founder Fit) using a 1-5 scale.
3. **Validation Planning**: Generates testable, non-building validation plans (e.g. Concierge tests, landing pages, interviews).
4. **MVP Scoping**: Aggressively scopes down ideas to their absolute minimum testable version. Excludes non-essential features.
5. **Competitor Scanning**: Tracks known alternatives and maps the gap/opportunity.
6. **Outreach Planning**: Drafts ethical, non-spam outreach templates for platforms like Reddit, X, and LinkedIn.
7. **Founder Reporting**: Synthesizes the above into structured markdown reports (`validation_report`, `mvp_scope_report`, `founder_weekly_report`).

## Storage Model

Cofounder data is stored locally in:
`~/.local/share/goat/agents/prime/cofounder/`

Using JSONL format for simple, append-only durability:
- `ideas.jsonl`
- `validations.jsonl`
- `scorecards.jsonl`
- `competitors.jsonl`
- `outreach_plans.jsonl`

## Interaction
The Cofounder Agent can be interacted with via the Dashboard (`/cofounder`), the CLI (`/cofounder` or `@cofounder`), or the REST API (`/v1/cofounder/*`).
