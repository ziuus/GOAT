# GOAT Alpha Release Checklist

Before tagging and releasing `0.14.0-alpha.1` (or any subsequent alpha), ensure the following strict checklist is completed:

- [x] **(Done)** **README Polished:** Positioning is clear, warnings are present, badges are accurate, and quick start commands are verified.
- [ ] **(Pending)** **Screenshots Ready:** All required screenshots in `docs/assets/screenshots/README.md` are captured and named correctly. *(Pending manual capture by maintainer)*
- [x] **(Done)** **Smoke Test Completed:** Ran through all steps in `docs/GOAT_ALPHA_SMOKE_TEST.md`.
- [x] **(Done)** **Feature Matrix Honest:** `docs/GOAT_FEATURE_MATRIX.md` strictly labels what is Working vs. Experimental.
- [x] **(Done)** **Known Limitations:** `docs/GOAT_KNOWN_LIMITATIONS.md` exists and is up-to-date.
- [x] **(Done)** **Issue Templates:** `alpha-feedback.yml` template exists.
- [x] **(Done)** **Docs Links:** All links in the documentation index are working.
- [x] **(Done)** **No Fake Claims:** The system does exactly what it says it does.
- [x] **(Done)** **No Secrets:** Verified no hardcoded API keys, tokens, or personal paths are in the codebase.
- [x] **(Done)** **Dashboard Build Passes:** `npm run build` succeeds locally.
- [x] **(Done)** **Cargo Tests Pass:** `cargo test` executes successfully.
- [x] **(Done)** **Release Notes:** `docs/releases/GOAT_0.14.0_ALPHA_1.md` is drafted and accurate.
- [ ] **(Intentionally deferred)** **LiteLLM / Provider Abstraction:** Acknowledged in roadmap but not implemented for Alpha 1 to keep core decoupled.
