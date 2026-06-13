# GOAT Alpha Release Checklist (v0.1.0-alpha.1)

Before tagging and publishing the Alpha 1 release, ensure all checks below pass.

## 1. Build Checks
- [x] `cargo fmt` passes.
- [x] `cargo check` passes without critical warnings.
- [x] `cargo build --release --bin goat` produces a functional binary.
- [x] Version in `Cargo.toml` is `0.1.0-alpha.1`.

## 2. Test Checks
- [x] `cargo test` passes all 100+ unit tests.
- [x] `scripts/smoke-test-alpha.sh` executes successfully.

## 3. Documentation Checks
- [x] `README.md` is updated with honest Alpha status, features, and Quickstart.
- [x] `docs/GOAT_ALPHA_QUICKSTART.md` is present and accurate.
- [x] `docs/GOAT_MIGRATION_PARITY.md` outlines how GOAT differs from other agents.
- [x] `docs/releases/v0.1.0-alpha.1.md` release notes are created.
- [x] `docs/GOAT_ALPHA_BLOCKERS.md` documents remaining hurdles for Public Alpha.

## 4. Safety & Approval Checks
- [x] `ApprovalGate` is strictly enforced and has not been bypassed globally.
- [x] "Approval Fatigue" handling is documented as a future planned feature (Tiered Approval Profiles) in release notes/blockers, rather than using an unsafe global `--auto-approve`.

## 5. Sample Extension Checks
- [x] `examples/capabilities/` contains sample extension payloads.
- [x] Capabilities fail closed if dependencies (like MCP tools) are not present.

## 6. Known Limitations (Documented)
- [x] No live IDE Context syncing (CLI/TUI only).
- [x] Constant approval prompts may cause fatigue.
- [x] Advanced capability scheduling not implemented yet.

## 7. Tag Checklist
- [x] Commit all finalized files.
- [x] Run `git tag -a v0.1.0-alpha.1 -m "GOAT v0.1.0-alpha.1 — first alpha release"`.
- [x] Run `git push origin master` and `git push origin v0.1.0-alpha.1`.

## 8. GitHub Release
- [ ] Create a GitHub Release using `docs/releases/v0.1.0-alpha.1.md` as the description.
- [ ] Attach `target/release/goat` (optional, as users will primarily build from source for Alpha 1).
