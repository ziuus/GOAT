# GOAT Alpha Release Checklist

Before tagging and releasing `0.1.0-alpha.1` (or any subsequent alpha), ensure the following checklist is completed:

- [ ] **README Polished:** Positioning is clear ("Not just for developers"), badges are accurate, and quick start commands are correct.
- [ ] **Repo Settings:** GitHub description and topics are set manually according to `docs/GITHUB_REPO_SETUP.md`.
- [ ] **Screenshots Collected:** All required screenshots in `docs/assets/screenshots/README.md` are captured and linked.
- [ ] **Install Works:** `git clone` and `cargo build` run cleanly.
- [ ] **Doctor Works:** `cargo run --release -- doctor` executes without crashing.
- [ ] **Dashboard Builds:** `cd apps/dashboard && npm run build` completes with 0 errors.
- [ ] **Core Workflows Tested:** Ran through all 5 workflows in `docs/GOAT_DOGFOODING_GUIDE.md`.
- [ ] **No Obvious Dead Buttons:** "Coming soon" features are explicitly disabled or labeled in the UI. No silent failures.
- [ ] **Feature Matrix Honest:** `docs/GOAT_FEATURE_MATRIX.md` strictly labels what is Working vs. Experimental.
- [ ] **Known Limitations Listed:** Clearly stated in the README and Matrix.
- [ ] **No Secrets:** Ran `git grep` to ensure no hardcoded API keys or tokens are in the codebase.
- [ ] **Release Notes Drafted:** `CHANGELOG.md` is updated with the unreleased section moved to a versioned tag.
