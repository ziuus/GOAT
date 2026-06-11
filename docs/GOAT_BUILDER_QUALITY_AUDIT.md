# GOAT Builder Quality Audit (Phase 7.1)

## Overview
This quality audit details the capabilities of the GOAT Builder agent, its stubs, architecture plans, and safety gates.

## Current Builder Capabilities
- Can create a basic `BuilderFeaturePlan` containing stubbed implementation steps and test plans.
- Integrates with `AgentContextPacker` to fetch basic brain context.
- Runs `QualityGate` markdown validations.
- Exports a dummy code review via `review_code`.

## Shallow or Stubbed Components
- **Repo Inspection:** No real folder traversal or language detection existed.
- **Patch Planning:** Hardcoded steps ("Step 1: Setup module", "Step 2: Implement logic").
- **Diff Review:** Absent. A hardcoded review is returned.
- **Test Planning:** Hardcoded ("Write unit tests").
- **Safety / ApprovalGate:** FS write is protected, but granular Builder actions (planning, repo-scanning, test-planning, and applying patches) did not have safety categorization or integration.

## Proposed Upgrades in Phase 7.1
1. **Repo Inspection Model:** Traverse the workspace, detect stack (Rust, NextJS, etc.), respect ignore patterns (`.git`, `node_modules`, `target`), summarize files/risk areas.
2. **Patch Planning Model:** Structured plan including goals, affected files, proposed changes, risks, rollback plans, and acceptance criteria.
3. **Diff Review Model:** Review patches for logical bugs, import breaks, security issues, missing tests, and classify by severity (info, low, medium, high, critical).
4. **Test Planning Model:** Generate check/test command sequences tailored to stack (e.g. `cargo check`, `npm run lint`).
5. **Runtime Jobs & Endpoint Actions:** Define 7 specific jobs (`builder_repo_inspection`, etc.) and expose them via API and CLI.
6. **Tool-Use Policy & Safety:** Require explicit reasoning before high-risk commands and enforce strict ApprovalGate checking.
7. **Dashboard Workspace:** Create a dedicated `/builder` dashboard interface displaying plan, snapshot, risks, diffs, and validation commands.
