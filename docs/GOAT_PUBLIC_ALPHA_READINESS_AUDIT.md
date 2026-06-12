# GOAT Public Alpha Readiness Audit

## 1. Install & Setup Flow
* **Install:** Standard `cargo build --release` works well, but users need to know about the Node prerequisite for the dashboard.
* **Setup Flow:** `goat setup` provides an interactive prompt to configure API keys and paths. This is solid.
* **First-user friction:** The initial launch doesn't explicitly link the daemon and dashboard workflows well enough for a non-technical user. This phase addresses it with better output for `goat setup`.

## 2. Doctor Output
* Currently checks config, paths, providers, and dashboard build.
* **Improvement needed:** Add checks for `ExtensionRegistry` health, `Brain` index, `Reports` directory, `Timeline` directory, `Browser` workflow backend, and `Git` availability. Use `OK`, `WARN`, `ERROR`, `OPTIONAL`, and `PARTIAL`.

## 3. Daemon & TUI Startup
* **Daemon:** Starts cleanly on `localhost:3000`. 
* **TUI:** Starts cleanly, falls back to mock logic if the daemon isn't reachable for some endpoints.
* **Limitations:** Some TUI features do not surface deep errors effectively if the daemon crashes.

## 4. Local-First Defaults
* All data is stored in the local XDG data directory (`~/.local/share/goat` on Linux).
* `goat.toml` uses local file providers by default.
* External API integrations (OpenAI, Groq) are opt-in.

## 5. Demo Data Flow (`goat seed-demo`)
* Currently deletes entire JSONL files when `--clear` is used, which risks overwriting user data!
* **Fixed in this phase:** Modify `seed-demo` to tag entries with `[DEMO]` and ensure `--clear` only removes lines containing the `[DEMO]` tag. Expand seed data to cover all Prime Agents, Runtime jobs, AgentFlow, and Timeline events.

## 6. Dashboard Disconnected & Error States
* **Current state:** Fails silently or hangs on loading if the daemon is disconnected.
* **Fixed in this phase:** Add global or page-level error states indicating "Daemon Disconnected" or "API Error" across top-level pages. Provide "Requires Daemon" messaging.

## 7. Known Limitations & Partial Features
* **Browser Workflows:** Dependent on the local Chrome instance and playwright-like backend.
* **External Providers:** Disabled by default to protect privacy.
* **Safety:** ApprovalGate requires terminal access; the dashboard currently requires the terminal to approve dangerous actions.
* **Desktop Scaffold:** Tauri desktop wrapper is experimental and partial.

## 8. Summary of Fixes in Phase 8.0
* Upgraded `goat doctor` with comprehensive checks.
* Fixed `goat seed-demo` to protect user data and seed all domains.
* Hardened the Dashboard with "Daemon Disconnected" states.
* Centralized and organized documentation via `docs/README.md`.
* Provided a `smoke-alpha.sh` script to verify local installations.
