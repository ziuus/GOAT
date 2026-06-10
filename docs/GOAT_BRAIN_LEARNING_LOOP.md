# GOAT Brain Learning Loop

## Overview
The Brain Learning Loop is GOAT's continuous process of extracting durable knowledge from everyday interactions. Instead of relying solely on immediate context or manual instruction, GOAT observes chat sessions, job outcomes, tool executions, and file modifications to propose memory candidates.

## Principles
1. **Review-First**: GOAT never silently stores sensitive information or automatically mutates your preferences. All learning is proposed as a "candidate" that requires explicit user approval.
2. **Local-First**: Learning is conducted and stored entirely locally. No telemetry or memory sync is sent to the cloud.
3. **Secret Redaction**: Any text that looks like a secret, token, or password is automatically redacted before a candidate is even proposed.

## Learning Config
The learning behavior is controlled in `goat.toml` under `[learning]`:
- `enabled`: Toggles the entire learning system.
- `auto_extract`: Toggles whether GOAT automatically parses events for candidates.
- `require_review`: If true, candidates are placed in a queue. If false, non-user candidates might be automatically accepted (NOT RECOMMENDED).
- `allow_llm_summarization`: Toggles whether an LLM is used to summarize events (requires external API calls).

## Candidate Types
- **user_preference**: Notes about how the user prefers to code or communicate.
- **project_fact**: Key architectural decisions or context about the repository.
- **workflow_pattern**: Repeated sequences of commands (can be promoted to Skills).
- **skill_candidate**: A proposed reusable automation or prompt template.
- **decision**: Recorded outcomes of significant approvals or architectural choices.

## Commands
- `/learn status` - View the status of the learning system.
- `/learn candidates` - View pending candidates.
- `/learn accept <id>` - Accept a candidate into long-term memory.
- `/learn reject <id>` - Reject a candidate.
- `/summary project` - Generate a summary of project activity based on learned context.
