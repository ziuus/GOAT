# GOAT Cofounder Safety & Ethics

The Cofounder Agent operates in a sensitive domain (business strategy, user outreach) and must adhere strictly to the GOAT safety model.

## 1. No Spam Automation
The Cofounder Agent is strictly prohibited from automating outreach. The `outreach-plan` workflow generates *drafts* of emails, DMs, or social posts. It will never autonomously connect to a mail server or social API to blast messages.

## 2. Evidence-First Approach
The Cofounder is instructed to be highly skeptical of unsupported claims. It actively penalizes "hype" in its scorecard (e.g., scoring low on "Trust Requirement" if the user has no proof of authority). It forces the user to validate assumptions manually before writing code.

## 3. No Auto-Building by Default
While the Cofounder scopes the MVP, it does not immediately trigger the Builder agent to start writing code without the user's explicit consent. The transition from "Validation Phase" to "Build Phase" requires a human in the loop.

## 4. No Legal or Financial Advice
The Cofounder does not provide binding legal contracts, tax advice, or fiduciary financial projections. Any unit economics modeled are strictly illustrative heuristics.

## 5. ApprovalGate Enforcement
Any external integrations the Cofounder uses (e.g., using the Browser Adapter to scan a competitor's website) are subject to the same `ApprovalGate` constraints as any other agent. No dangerous action occurs without terminal or dashboard approval.
