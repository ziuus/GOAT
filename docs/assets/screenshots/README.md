# GOAT Screenshots

This directory contains screenshots of the GOAT UI/UX for use in documentation, marketing, and the main README.

## Required Screenshots Checklist
Before the first public alpha release, ensure the following screenshots are captured and placed in this directory:

- [x] `01-home-dashboard.png`
- [x] `02-agent-command-center.png`
- [x] `03-learner-os.png`
- [x] `04-cofounder.png`
- [x] `05-promptforge.png`
- [x] `06-reports.png`
- [x] `07-timeline.png`
- [x] `08-brain-search.png`
- [x] `09-sidebar-safety.png`

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
