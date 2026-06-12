# GOAT Cofounder Quality Audit

## 1. What Cofounder can actually do now
Currently, `CofounderManager` in `src/agents/cofounder.rs` maintains an extremely shallow model of an idea. It supports storing a `title`, `description`, and `target_audience`. Workflow states are basic (IdeaLogged, Validating, Validated, Scored, MvpScoped, OutreachPlanned, Rejected).

It can "generate" validation plans, MVP scopes, competitors, landing page briefs, outreach plans, and scorecards, but all of these are completely hardcoded and faked (e.g., pain_intensity is always 4, totally fake competitor "Example Corp").

## 2. What is shallow/simple
Almost everything. There is no real validation logic. Evidence is not tracked. MVP scopes are completely fake string vectors. There is no semantic difference between a strong signal and a weak signal. Landing page reviews return a static string.

## 3. What validation logic exists
Only a stub method `deep_evaluate_idea` that mocks a score of 69 out of 100 without actually verifying any evidence or constraints.

## 4. What evidence model exists
There is no Cofounder evidence model yet. It does not reference Researcher claims.

## 5. What Researcher/Builder handoffs exist
Handoffs do not exist. There is a fake method `generate_landing_page_brief` that returns a string format.

## 6. What reports exist
A mock `CofounderReport` generation is wired to `crate::reports::ReportManager` but only creates a shallow "Executive Summary" with zero actionable evidence.

## 7. What dashboard actions are wired
The `/cofounder` page exists in the dashboard but likely expects very basic data.

## 8. What is implemented in this phase
In Phase 7.6, we will implement:
- Deep `CofounderIdea` models that capture pain, urgency, and pricing hypotheses.
- `MarketSignal` tracking to track real validation (interviews, waitlists, etc.).
- `ValidationExperiment` models to plan ethical tests.
- `MVPScope` models that prevent feature creep and require evidence.
- `PricingHypothesis` models to test willingness to pay.
- Builder handoffs that integrate with `BuilderAcceptanceCriteria`.
- Brain indexing and Timeline events for all these new concepts.
- Provider metadata and Researcher integration.

## 9. What remains partial
- Generating actual landing page DOM reviews via Browser workflow (we will map the structure but actual vision/DOM extraction requires the Browser adapter to be run).
- Generating actual competitor scans (will be handed off to Researcher, but execution depends on AgentFlow runtime orchestrating it).
