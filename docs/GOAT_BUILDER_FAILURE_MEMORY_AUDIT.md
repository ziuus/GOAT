# GOAT Builder Failure Memory Audit

## Current State (Post Phase 7.3)

### Validation Failures
- **Where are they stored?** Currently embedded inside `CodeExecutionSession` (`.goat/code_executions/<id>.json`) as `ValidationRun`s. When analyzed, they are clustered and stored in `ValidationFailureAnalysis` within `.goat/code_analyses/<id>.json`.
- **Are they indexed in Brain?** No. They are purely local to the execution run.
- **Does Builder recall similar failures?** No. Every session starts from scratch.

### Retry Plans
- **Where are they stored?** In `.goat/code_retries/<plan_id>.json` via `CodeExecutionManager::save_retry_plan()`.
- **Are successful fixes recorded?** No. When a retry loop succeeds, it simply marks the execution as `Completed`.
- **Are failed retry attempts recorded?** No. Only the active retry plan is known, and exceeding max retries just sets the state to `MaxRetriesExceeded`.

### Gaps in Memory/Learning
1. **No Deduplication:** The same `import missing` error in the same file will generate a fresh analysis and retry plan every time.
2. **No Success Linking:** If a specific fix pattern worked for a `MissingDependency` error, the Builder does not remember it next time.
3. **No Brain Integration:** Brain V2 has no knowledge of `BuilderFixLesson` or `BuilderValidationFailure`.
4. **No Project Learning:** We don't generate aggregate reports of "files that fail to compile most often" or "most common error patterns".

## Phase 7.4 Scope

### Fixed in this Phase
1. **Failure Memory Model:** Introducing `BuilderFailureMemory`, `BuilderFailureSignature`, `BuilderFixOutcome`, and `BuilderFixLesson`.
2. **Deduplication:** Normalizing error messages to group similar failures (e.g., removing specific line numbers for the signature).
3. **Brain Ingestion:** Indexing failures, retry outcomes, and lessons into the Brain so they are searchable.
4. **Recall Integration:** Builder will query the Brain for similar past failures before creating a retry plan.
5. **Outcome Recording:** Automatically generating a `BuilderFixLesson` on success and logging failed attempts on failure.
6. **Project Learning Summaries:** Generating `builder_learning_report` summarizing recurring mistakes and successful patterns.
7. **Dashboard/API Integration:** Expanding the Builder and Brain UIs to surface these memory artifacts.

### What Remains Partial
- Automatic inference of generalized rules without human review (we stick to explicitly generated lessons from specific fix events).
- Deep AST-level structural matching of mistakes (we rely on text-based signatures, regex normalization, and LLM semantic similarity via Brain).
- Bypassing approvals based on high-confidence memory (Rule: Memory never bypasses `ApprovalGate`).
