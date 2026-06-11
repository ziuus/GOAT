# GOAT Compiler-Guided Builder Audit

This document outlines the state of the Code Execution and Validation systems before implementing Phase 7.3 (Compiler-Guided Builder Loop) and describes what will be added.

## 1. Current State of Validation Output Storage
- **Location:** Managed by `CodeExecutionManager` in `src/code_execution.rs`.
- **Storage:** Persisted in `CodeExecutionSession` under `.goat/code_executions/<session-id>.json`.
- **Format:** `ValidationRun` captures `stdout`, `stderr`, `exit_code`, and a simple `findings` array (containing `message` and `severity`).
- **Shortcomings:** 
  - `ValidationFinding` is too simplistic (no file, no line number, no suggested fix, no error cluster).
  - No dedicated parsing for Rust compiler/test errors or Next.js/TypeScript errors.

## 2. Failure Metadata
- Currently, failures are loosely represented by `ValidationStatus::Failed` and an exit code.
- No sophisticated taxonomy for error origins (e.g., `ValidationFailureKind::rust_compile_error`).
- **Planned in 7.3:** Introduction of `ValidationFailure`, `ValidationFailureCluster`, `ValidationFailureAnalysis`, capturing deep semantics of *why* validation failed.

## 3. Parsing and Error Mapping
- **Current:** Basic string capture (`stdout`/`stderr`). No file/line mapping.
- **Planned in 7.3:**
  - Rust output parsers (`cargo check`, `cargo test`, `cargo fmt`).
  - JS/TS parsers (Next.js build errors, TS errors, Eslint).
  - Generic/fallback heuristics to map failures to files when parsing is imperfect.

## 4. Retry Plans
- **Current:** Non-existent. The loop stops at `ValidationStatus::Failed`.
- **Planned in 7.3:** 
  - `BuilderRetryPlan` struct.
  - Safe retry loops that require ApprovalGate.
  - Max retries limit (3).
  - Checkpoint creation *before* the retry mutation.

## 5. Scope of Phase 7.3 vs Future
- **Implemented Here:** 
  - Structured failure representation and clustering.
  - Parsers for Rust and JS/TS.
  - Retry plan generation and safe application (approval-gated).
  - Builder subcommands for retries.
  - Dashboard panels for failures and retries.
- **Remains Partial / Future:**
  - Unsupervised autonomous "self-healing" loop (intentionally avoided in 7.3 for safety).
  - Extremely deep AST-level semantic analysis (will rely on generic + regex for now).
  - Auto-installation of missing external system dependencies.

## 6. Security & Safety Rules
- **No unapproved retries.** All `BuilderRetryPatchCandidate`s go through ApprovalGate.
- **Checkpoint-guarded:** The loop strictly snapshots the codebase before any retry.
- **No infinite loops:** Capped at 3 retries max per execution session.
