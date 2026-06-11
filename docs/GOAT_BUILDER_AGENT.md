# GOAT Builder Agent

The Builder Agent acts as a local developer companion that reads the workspace structure, drafts detailed patch plans, evaluates code modifications, recommends validation steps, and runs tests securely.

## Workflow States
1. **ContextGathered:** Repository inspection has scanned the project.
2. **PlanDrafted:** A feature implementation plan or patch plan is written.
3. **CodeGenerated:** Draft patches have been prepared.
4. **TestsWritten / Recommended:** A clear validation/test plan has been established.
5. **Reviewed:** Diffs have been scrutinized for risk, logic, or syntax bugs.
6. **Complete:** The changes are fully validated and checked in.
