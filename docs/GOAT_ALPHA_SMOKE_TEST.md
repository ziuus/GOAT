# GOAT Alpha Smoke Test Checklist

Before any Alpha release, ensure the following steps complete successfully on a fresh clone.

## 1. System Setup
- [ ] `git clone https://github.com/ziuus/GOAT.git && cd GOAT`
- [ ] `cargo build --release` compiles without errors.
- [ ] `cargo run --release -- setup` completes configuration without panics.
- [ ] `cargo run --release -- doctor` shows valid system health and configurations.

## 2. Daemon & Dashboard
- [ ] `cargo run --release -- daemon start` successfully binds and listens on port 47647.
- [ ] `cd apps/dashboard && npm install && npm run build` completes with 0 errors.
- [ ] `npm run dev` serves the dashboard on port 3000.

## 3. UI Functionality
- [ ] Open Dashboard (`http://localhost:3000`).
- [ ] Disconnected state appears correctly if the Daemon is stopped.
- [ ] Dashboard reconnects and shows Home correctly when the Daemon is started.
- [ ] Demo Data can be loaded via Settings.
- [ ] **Learner Page**: Tracks and UI render correctly.
- [ ] **Cofounder Page**: Forms are usable, Scorecard renders.
- [ ] **PromptForge Page**: Refinement mock works.
- [ ] **Reports Page**: Can view timeline and reports.
- [ ] Check experimental badges are present on Socializer, Designer, Operator.

## 4. Documentation & Secrets
- [ ] `docs/GOAT_FEATURE_MATRIX.md` matches the true state of the branch.
- [ ] No hardcoded API keys or secrets are committed.
