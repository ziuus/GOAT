# GOAT Cofounder Agent Plan

## Vision
The Cofounder Agent is a Prime Agent in GOAT designed to act as an objective, evidence-first partner for makers, hackers, and entrepreneurs. It prevents the most common failure mode in software: building things nobody wants.

## Core Responsibilities
1. **Idea Validation:** Force the user to define the problem, the target audience, and the existing alternatives before writing code.
2. **Market Research:** Utilize tools (web search, browser adapter) to analyze competitors and market size.
3. **MVP Scoping:** Ruthlessly cut features to arrive at the smallest testable iteration (v1.0).
4. **Weekly Founder Reports:** Generate structured summaries (`founder_weekly_report`) reflecting on progress, roadblocks, and alignment with the initial goal.
5. **Devil's Advocate:** Challenge overly optimistic assumptions.

## Attached Specialists
The Cofounder Agent can delegate deep analysis to:
- **Finance Analyst:** To model burn rate or unit economics.
- **Market Validator:** To synthesize user interview transcripts.
- **Competitor Analyst:** To build a feature matrix of existing solutions.
- **Pricing Strategist:** To recommend pricing models (SaaS, one-time, usage-based).

## Interaction Model
- **Trigger:** Initiated via `/agents run cofounder validate-idea` or `@cofounder check this assumption`.
- **Outputs:** Instead of endless chat, the Cofounder generates structured Reports (e.g., `validation_report.md`) saved to the Report System.
- **Ethics & Safety:** 
  - Cannot execute financial transactions.
  - Cannot auto-send emails to potential customers (must draft for approval).
  - Demands real evidence; penalizes hype-driven development.

## Current Status (Phase 5.16)
- The architecture and manifest exist.
- State: `Planned / Experimental`.
- The full autonomous behavior loop is scheduled for Phase 5.17+.
