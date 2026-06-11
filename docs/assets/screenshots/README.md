# GOAT Screenshots

This directory contains screenshots of the GOAT UI/UX for use in documentation, marketing, and the main README.

## Required Screenshots Checklist
Before the first public alpha release, ensure the following screenshots are captured and placed in this directory:

- [ ] `01-home-dashboard.png`
- [ ] `02-agent-command-center.png`
- [ ] `03-learner-os.png`
- [ ] `04-cofounder.png`
- [ ] `05-promptforge.png`
- [ ] `06-reports.png`
- [ ] `07-timeline.png`
- [ ] `08-brain-search.png`
- [ ] `09-sidebar-safety.png`

## Capture Instructions (Manual)
1. Start the Daemon (`cargo run --release -- daemon start`).
2. Start the Dashboard (`npm run dev`).
3. Click "Load Demo Data" in Settings to populate the UI.
4. Take full-window screenshots (or crop nicely) in Dark Mode.
5. Save exactly with the names above to this directory.

## Guidelines
* Do NOT fake screenshots.
* Use realistic mock data or actual usage data.
* Ensure the `ApprovalGate Protected` badge is visible in shots where relevant.
* Use the default Dark Theme (`#0A0A0A`).
