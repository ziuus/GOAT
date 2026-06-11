# GOAT Persistence Audit

## Overview
This document audits the local-first persistent storage strategy for GOAT's Prime Agents and Runtime flows (Phase 6.5).

## Core Directives
- **Do not fake persistence:** State must not reset when the dashboard restarts.
- **Do not overwrite user data:** Avoid generic rewrites; use atomic or append-safe operations.
- **Keep local-first storage visible:** Data must reside in accessible local files (XDG paths).

## Storage Locations (XDG Data Directory)
By default, this is `~/.local/share/goat/`.

| Component / Flow       | Storage Strategy               | File Path                                       |
|------------------------|--------------------------------|-------------------------------------------------|
| **Runtime Jobs**       | SQLite / JSON Append           | `jobs.db` or `runtime/jobs.jsonl`               |
| **Cofounder Agent**    | Append-safe JSONL              | `agents/prime/cofounder/ideas.jsonl`            |
| **Learner Agent**      | Append-safe JSONL              | `agents/prime/learner/goals.jsonl`              |
| **Learner Roadmaps**   | Append-safe JSONL              | `agents/prime/learner/roadmaps.jsonl`           |
| **PromptForge**        | Append-safe JSONL              | `promptforge/history.jsonl`                     |
| **Reports**            | Markdown + JSON Meta           | `reports/<id>.md`, `reports/<id>.json`          |
| **Timeline**           | SQLite / System Events         | `timeline.db` or `timeline/events.jsonl`        |

## Implementation Details
1. **JSONL Append-Only Structure:** Most Prime Agents use `.jsonl` files. This allows safe concurrent writes (appends) without full file locking and prevents race conditions that overwrite entire states. 
2. **Dashboard Real Wiring:** The React Dashboard communicates exclusively through `goatApi` / `daemonFetch`. There is no local React state overriding the backend data—when the page mounts, it fetches from `/v1/...` endpoints.
3. **Demo Data Management:** A `goat seed-demo --clear` CLI command is provided to initialize or wipe the demo state without touching core user databases like the memory index or the SQLite sessions.

## Verification
- Restarting the daemon preserves all ideas, goals, prompt history, and reports.
- Restarting the dashboard Next.js dev server does not reset state.
- Demo data can be securely wiped without affecting production usage.
