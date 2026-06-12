# GOAT Alpha Manual QA Checklist

Use this checklist before creating new public alpha releases to verify the integrity and safety of the local-first environment.

## 1. Installation & Environment
- [ ] `git clone` works cleanly without missing submodules.
- [ ] `cargo build --release` compiles without errors on a fresh machine.
- [ ] `goat setup` runs the interactive wizard correctly and creates `goat.toml`.
- [ ] `goat doctor` correctly reports paths, node availability, and git presence.

## 2. Mock & Seed Data
- [ ] `goat seed-demo` successfully populates all demo artifacts (Cofounder, Learner, Reports, etc.).
- [ ] `goat seed-demo --clear` correctly clears ONLY the demo artifacts (leaves user data intact).
- [ ] All seed data accurately displays the `[DEMO]` prefix.

## 3. Daemon & TUI
- [ ] `goat daemon start` binds to `localhost:3000` cleanly and handles SIGINT.
- [ ] `goat` (TUI) starts cleanly and displays the Command Center.
- [ ] TUI fetches and displays local provider configurations correctly.

## 4. Dashboard Quality
- [ ] `npm run build` succeeds inside `apps/dashboard`.
- [ ] **Home Page:** Correctly handles "Daemon Disconnected" state if the backend is down.
- [ ] **Providers Page:** Correctly reads from `goat.toml`.
- [ ] **Extensions Page:** Lists installed extensions and accurately displays enable/disable UI.
- [ ] **Browser Page:** Navigating visual workflows warns if headless browser backend is missing.
- [ ] **Reports Page:** Renders semantic search and generated reports correctly.
- [ ] **Builder Page:** Shows code plans with safety-first approval states correctly.

## 5. Security & Safety
- [ ] **ApprovalGate Check:** `goat builder plan` (or equivalent code modification commands) ALWAYS halts for user approval.
- [ ] **No Auto-Deploy:** `goat operator` does not execute bash deploy commands automatically.
- [ ] **No Auto-Post:** `goat socializer` does not dispatch requests to live social APIs automatically.
- [ ] **No Secrets Leakage:** Running demo flows does not expose API keys to generated reports or logs.
- [ ] **Local Defaults:** No external LLM usage occurs without explicit opt-in in `goat.toml`.

## 6. Documentation
- [ ] `README.md` clearly states Alpha constraints and provides Quick Start.
- [ ] `docs/README.md` accurately indexes all deep architectures.
