# GOAT Alpha Usability Audit

**Date:** June 2026

## Focus
Inspect the app as a real first-time user and identify friction points, dead ends, and confusing UX.

## Findings

1. **Dashboard Home:**
   - Improved in Phase 5.26, but needs a more prominent "Start Here" path.
   - If the daemon is down, the user sees generic fetch errors. Needs a clear full-page "Daemon Disconnected" error state with instructions (`cargo run -- daemon start`).

2. **Sidebar:**
   - Reorganized successfully in Phase 5.26. Connection state is visible.

3. **Prime Agents (Command Center):**
   - Clean layout. Most buttons link correctly to agent workspaces.
   - Some agents (Designer, Researcher, Operator, Socializer) are largely empty shells with placeholder empty states. These need clear "Experimental / Coming Soon" labels to set honest expectations.

4. **Cofounder:**
   - The "Create Idea" flow works in the UI but doesn't persist to the backend yet.
   - "Run Validation" button does nothing. It should show a "Not yet wired" toast or disable state.

5. **Learner:**
   - Solid UI from Phase 5.25. The mock data feels somewhat realistic, but starting a new track only saves to local React state.

6. **PromptForge:**
   - The "Improve Prompt" action calls a real endpoint but the backend implementation might be stubbed depending on config.
   - "History" shows mock data if API fails. 

7. **Reports & Timeline:**
   - Currently show static mock data after a fake 600ms loading delay. Need to clarify this is a UI preview.

## Action Items for Speed Sprint
- [x] Add global Daemon connection check and Error state.
- [x] Label Designer, Researcher, Operator, Socializer as "Experimental".
- [x] Disable dead buttons (e.g. "Run Validation" in Cofounder) with clear "Coming soon" helper text.
- [x] Provide a clear Demo Data path.
