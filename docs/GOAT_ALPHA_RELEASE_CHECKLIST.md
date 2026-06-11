# GOAT Alpha Release Checklist

Before tagging and releasing `0.14.0-alpha.1` (or any subsequent alpha), ensure the following strict checklist is completed:

- [ ] **README Polished:** Positioning is clear, warnings are present, badges are accurate, and quick start commands are verified.
- [ ] **Screenshots Ready:** All required screenshots in `docs/assets/screenshots/README.md` are captured and named correctly.
- [ ] **Smoke Test Completed:** Ran through all steps in `docs/GOAT_ALPHA_SMOKE_TEST.md`.
- [ ] **Feature Matrix Honest:** `docs/GOAT_FEATURE_MATRIX.md` strictly labels what is Working vs. Experimental.
- [ ] **Known Limitations:** `docs/GOAT_KNOWN_LIMITATIONS.md` exists and is up-to-date.
- [ ] **Issue Templates:** `alpha-feedback.yml` template exists.
- [ ] **Docs Links:** All links in the documentation index are working.
- [ ] **No Fake Claims:** The system does exactly what it says it does.
- [ ] **No Secrets:** Verified no hardcoded API keys, tokens, or personal paths are in the codebase.
- [ ] **Dashboard Build Passes:** `npm run build` succeeds locally.
- [ ] **Cargo Tests Pass:** `cargo test` executes successfully.
- [ ] **Release Notes:** `docs/releases/GOAT_0.14.0_ALPHA_1.md` is drafted and accurate.
