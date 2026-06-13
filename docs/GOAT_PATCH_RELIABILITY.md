# GOAT Patch Reliability & Safety

This document describes the safety mechanisms enforced by GOAT during patch generation and application to ensure enterprise-grade reliability and prevent accidental corruption of workspaces.

## Preflight Checks

Before any patch is applied, GOAT performs the following preflight checks:

1. **Path Containment:** Ensures all target files are within the project root. Traversal attempts (e.g., `../`, absolute paths outside root) are strictly blocked.
2. **Secret File Protection:** Modifications to known secret files (like `.env`, `credentials.json`, `id_rsa`) are automatically denied.
3. **Protected Directories:** GOAT refuses to modify files within critical build/dependency directories like `node_modules`, `target`, `vendor`, and `.git`.
4. **Lockfile Protection:** Prevents modifications to dependency lockfiles (e.g., `Cargo.lock`, `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`) since these should only be managed by package managers.

## Mandatory Checkpointing

GOAT enforces a mandatory checkpointing system. Before a patch is applied, a `git`-based checkpoint of the current workspace state is automatically created. If checkpointing fails, the patch application is aborted.

## Patch Quality & Risk Scoring

GOAT calculates a deterministic risk score for every proposed patch. 
- **Risk Level:** Evaluated as `low`, `medium`, or `high` based on the files being modified and the size of the diff. (e.g., changes to `Cargo.toml` or `package.json` increase risk).
- **Estimated Impact:** GOAT provides an English description of the impact, such as "Minor modification", "Large modification", or "New file creation".

## Validation Recommendations

After a patch is successfully applied, GOAT intelligently recommends validation commands based on the project's detected framework and toolchain.
- For Rust projects, it might recommend `cargo test` or `cargo check`.
- For Node.js projects, it might recommend `npm test` or `npm run build`.
- If no automated commands are detected, it falls back to recommending "Manual review required".

During the interactive CLI patch flow, users are prompted to run these validation commands directly from the prompt.
