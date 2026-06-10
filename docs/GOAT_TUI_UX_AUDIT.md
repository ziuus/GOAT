# GOAT TUI UX Audit

**Date:** June 2026

## Overview
The Terminal User Interface (TUI) acts as the primary power-user interface for GOAT. It relies on `ratatui` and provides a multi-pane layout for chat, context, system logs, and approvals.

## Layout Analysis
- **Focus Layout:** Clean, but the border weights can occasionally feel too heavy, taking up valuable screen real estate.
- **Dashboard Layout:** Functional but crowded on terminal windows smaller than 120x40. 
- **Compact Layout:** Good for split screens, but log output can overflow ungracefully.

## Specific Views
- **Chat/Slash Suggestions:** Suggestions pop up cleanly, but the contrast on selected items needs slight tweaking for readability.
- **Approvals:** The TUI approval flow is secure, but the "Press Y/N" prompt could be highlighted more clearly to draw attention.
- **Repo/Diff Views:** Sufficient for quick checks, but long lines wrap aggressively.
- **Context View:** Functional. Adding/removing files is intuitive via commands.
- **MCP/Tools View:** Lists available tools properly.

## Discovered Issues (Small/Medium)
1. **Ugly Spacing:** Some block borders lack padding, causing text to jam against lines.
2. **Weak Empty States:** The "No active jobs" or "No pending approvals" screens are just blank lists rather than helpful text.
3. **Noisy Logs:** The event bus dumps every minor tick into the log view, which can push important system warnings out of sight too quickly.
4. **Weak Error Messages:** API connection failures print a generic network error rather than a helpful "Is the daemon running?" prompt.

## Verdict
**Is the TUI clean?** Yes, but it borders on cluttered during heavy operations.
**Is it robust?** Highly robust, rarely crashes.
**Is it user-friendly?** For developers, yes. For casual users, the learning curve for slash commands remains steep.
**What still needs improvement?** Smarter log filtering, softer borders, and friendlier empty-state messages.
