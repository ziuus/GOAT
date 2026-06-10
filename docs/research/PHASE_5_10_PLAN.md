# Phase 5.10: Project Timeline + Work History Replay Plan

## 1. Inspect EventBus, JobTracker, ApprovalQueue/history, audit logs, checkpoints, recipes, skills, brain index, AI Studio, Memory Galaxy.
The GOAT ecosystem emits events across many domains. Currently, we have `audit logs` (e.g. `tool-audit.log`, `subagent-audit.log`), `JobTracker` in `task.rs`, `ApprovalQueue` in `approval.rs`, and custom structs for memories and skills. The `TimelineManager` will need to hook into these either by subscribing to an event bus (if `events.rs` has one) or by manually writing timeline events when key actions occur.

## 2. Decide JSONL storage format.
We will store events in `~/.local/share/goat/timeline/timeline.jsonl`.
Each line is a JSON object `TimelineEvent`.
```json
{
  "id": "uuid",
  "timestamp": 1690000000,
  "project_path": "/home/zius/Projects/GOAT",
  "session_id": "uuid",
  "source": "approval_gate",
  "kind": "approval_approved",
  "title": "Approved write to src/main.rs",
  "summary": "User approved the file write for Phase 5.10",
  "actor": "user",
  "related_ids": ["job_123"],
  "file_refs": ["src/main.rs"],
  "risk_level": "medium",
  "privacy_level": "public",
  "redaction_status": "clean"
}
```

## 3. Explain timeline architecture.
The `TimelineManager` will be a struct in `src/timeline.rs`. It provides methods to `record_event`, `query`, and `export`.
It will be integrated into `GoatRuntime` similarly to `SkillResearcher` so it's accessible across the app.
It will feature memory-mapped or buffered appending to `timeline.jsonl`.

## 4. Explain ingestion sources.
Sources will push to `TimelineManager::record_event()`.
- **Jobs**: Inside `JobTracker` hooks or manual calls when jobs start/end.
- **Approvals**: Inside `ApprovalGate`.
- **Skills/Recipes**: When attached/executed.
- **Commits**: Can be inferred from `repo_map` or executed via git hooks/native tool calls.
- *Partial implementation*: Deep scraping of past audit logs will be partial. We will focus on capturing new events moving forward.

## 5. Explain replay modes.
- **narrative**: A human-readable story of the session.
- **step_list**: A bulleted list of actions.
- **audit**: Detailed logs with risk levels and timestamps.
- **reconstruct_context**: Outputs context ready for an LLM prompt.
- **dry_run_plan**: A generated plan to recreate the work.
Replay does *not* execute commands. It just queries the timeline and formats it.

## 6. Explain dashboard timeline design.
A new page `/timeline` will be added to the Next.js dashboard.
It will feature a chronological feed. Items will have icons based on `kind` (e.g. Shield for approval, Terminal for job).
Clicking an item opens a drawer with full metadata. A "Replay" button generates a narrative view.

## 7. Explain Brain Search integration.
(Partial implementation). `BrainIndexManager` will be updated to recognize `timeline.jsonl`. It will parse events and index their `summary` and `title` fields.
Semantic search will find timeline events if `store_vectors` is enabled.

## 8. Explain privacy/redaction model.
The timeline stores *metadata and summaries*, not raw file contents or secrets.
Commands will be sanitized (e.g., stripping `--api-key` values).
Exporting the timeline will filter out any events marked `privacy_level: Private`.

## 9. Explain what is implemented now vs partial/planned.
**Implemented Now**:
- Architecture & JSONL storage.
- Active event recording for Approvals, Sessions, Skills, and Jobs.
- Replay command (Narrative & Step List).
- Dashboard UI feed.
- API endpoints.
**Partial / Planned**:
- Deep retroactive ingestion of all past git commits.
- Semantic Memory Galaxy suggestions from timeline patterns.
- AI Studio direct integration (mocked in UI).
