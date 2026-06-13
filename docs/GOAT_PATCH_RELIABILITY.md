# GOAT Patch Reliability & Safety

Phase 10.2 hardens the repository editing and patch application workflows, providing migration-grade reliability for users transitioning from other agentic coding tools like Claude Code, Aider, or Cline.

## Core Reliability Guarantees

1. **Path Containment (No Traversal)**
   - All proposed file changes are strictly canonicalized.
   - Any attempt to write or patch files outside the project's root directory is blocked at the patch application layer.

2. **Immutable Dependency & Generated Directories**
   - GOAT refuses to modify critical locked directories:
     - `node_modules/`
     - `target/`
     - `vendor/`
     - `.git/`
   - Lockfiles (`package-lock.json`, `Cargo.lock`, `poetry.lock`) are also protected from manual agent patches.

3. **Secret Protection**
   - `.env`, `.pem`, and other heuristic-based secret files are blacklisted. The agent cannot propose edits to them.

4. **Mandatory Checkpoints Before Apply**
   - Patches configured with `checkpoint_required = true` force the creation of a Git-based checkpoint *before* the patch is written to disk.
   - Rollbacks (`goat code rollback <session_id>`) leverage these checkpoints, restoring only the affected files via `git checkout HEAD -- <file>`.

## Patch Quality Scoring (Risk Assessment)

GOAT deterministically evaluates patch risk using `assess_patch_risk()`. Risk levels are classified as **Low**, **Medium**, or **High** based on:
- **File Type**: Modifications to build config files (`Cargo.toml`, `package.json`, `vite.config`) heavily increase the score.
- **File Core Role**: Changes to entry points (`main.rs`, `index.ts`, `app.tsx`), database schemas, or authentication components elevate risk.
- **Diff Size**: Modifications exceeding 50 lines flag as "Large modification", and over 200 lines flag as "Massive modification".

*When reviewing a patch, the user is presented with the explicit Risk Level and the Estimated Impact reasoning (e.g., "Build/Config file modification, Large modification").*

## Post-Patch Validation Recommendations

When an agent proposes a patch, `suggest_validation_command()` dynamically recommends a post-patch verification command based on the file extension and the `ProjectIntelligence` metadata.
- **Rust (`.rs`)**: `cargo check && cargo test`
- **TypeScript (`.ts`, `.tsx`)**: `npm run build && npm run test`
- **Go (`.go`)**: `go build ./... && go test ./...`
- **Fallback**: Inherits directly from the detected `test_commands` or `build_commands` in `ProjectIntelligence`.

## CLI Workflows for Patch Review

The CLI provides a polished interface for inspecting and applying patches proposed by the agent:

```bash
# List all generated patches in the session
goat patch list

# Show detailed patch diff, risk level, and suggested validation command
goat patch show <patch_id>

# Interactively apply a proposed patch
goat patch apply <patch_id>
```

When applying, the CLI summarizes the impact and requests explicit user confirmation `[y/N]`. If approved, a checkpoint is cut, the files are written, and the patch status changes to `applied`.
