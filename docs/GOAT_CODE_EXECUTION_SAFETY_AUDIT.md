# GOAT Code Execution Safety Audit

## 1. Current File Mutation Paths
- **Current State:** GOAT can execute file changes via the `run_command` and `edit_file` tool integrations in `ToolRegistry` and external shell outputs. Builder currently acts as a planner, relying on other agents or manual user action to apply `BuilderPatchPlan`.
- **Risks/Gaps:** File mutations are not centrally managed through a strict lifecycle. Patch plans exist, but the direct application is unmanaged and not automatically wrapped in checkpoints or validation steps.

## 2. Current Checkpoint Support
- **Current State:** `src/checkpoint.rs` exists and creates checkpoints using `git diff HEAD` and `git status`. It does not create explicit standalone file snapshots but leverages Git's internal tracking for full project state.
- **Risks/Gaps:** We need file-level or explicitly managed snapshots for changes made outside the context of large Git commits, especially for rolling back partial changes gracefully without losing unrelated work.

## 3. Current Rollback Support
- **Current State:** `BuilderPatchPlan` contains a `BuilderRollbackPlan` generated during planning. `src/checkpoint.rs` supports creating checkpoints, but no integrated rollback orchestrator exists to systematically reverse an applied patch or restore an earlier `checkpoint.rs` state if validation fails.
- **Risks/Gaps:** No built-in orchestration to detect hash mismatches/conflicts on rollback.

## 4. Current Diff Preview Support
- **Current State:** The API exposes `/v1/diffs` which executes `git diff`. `BuilderDiffReview` is currently static or relies on unstructured string diffs. 
- **Risks/Gaps:** Lacks a structured `CodeDiffPreview` model tightly integrated with execution sessions before they are approved and applied.

## 5. Current Validation Command Support
- **Current State:** There is no dedicated validation orchestrator. Test plans are generated via `BuilderTestPlan`, but they are not executed automatically as part of a governed patch application lifecycle.
- **Risks/Gaps:** Validation is purely manual. If a patch breaks the build, GOAT currently does not automatically catch it via a trusted validation runner before committing to the change.

## 6. ApprovalGate Coverage
- **Current State:** `ApprovalGate` protects dangerous tool calls (e.g. destructive shell commands or file edits), ensuring the user explicitly authorizes them.
- **Risks/Gaps:** `ApprovalGate` needs to be extended to natively support the entire `CodeExecutionSession`, reviewing unified diffs and execution plans, rather than just isolated low-level tool calls.

## 7. What is Fixed in Phase 7.2
- Creation of `CodeExecutionManager` in `src/code_execution.rs`.
- Unified `CodeExecutionSession` handling patch plans from Draft to Completion.
- Integration of a governed Validation Runner executing allowlisted commands (e.g. `cargo check`, `npm run build`).
- Advanced rollback integration detecting conflicts before restoring files.
- Refined Dashboard (`/builder`) and API integration mapping execution states.

## 8. What Remains Partial
- Automatic AST-level self-healing edits when validations fail (targeted for Phase 7.3).
- Fine-grained unified diff application (using basic file replacement or patch libraries for now).
