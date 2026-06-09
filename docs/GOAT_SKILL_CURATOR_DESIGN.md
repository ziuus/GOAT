# GOAT Skill Curator Design

*Status: Planned (Phase 2.2+)*

## Motivation
Currently, skills are created explicitly by the user (either manually or via `/save-skill <name>`). 
However, as GOAT handles more sessions, there are successful workflows that naturally emerge but are lost to the history logs unless the user remembers to extract them.

The **Skill Curator** is an automated background process that runs asynchronously after a session completes, identifying and proposing new skills based on successful interactions.

## Core Flow
1. **Trigger**: When a session ends or the user explicitly runs `/curate`.
2. **Analysis**: The Curator LLM (using a cheaper, fast model like Claude Haiku or GPT-4o-mini) reads the `MEMORY.md` and the `brain.db` interaction history.
3. **Detection**: It looks for repeated workflows, multi-step tool sequences, or successful debugging sessions that have generalized value.
4. **Drafting**: It generates a drafted `SKILL.md` (similar to `/save-skill`) and places it in `~/.local/share/goat/skills/.drafts/`.
5. **Approval**: On the next startup, GOAT notifies the user:
   `"The Curator noticed a successful workflow from your last session and drafted a new skill: 'rust-clippy-fixer'. Run /skills draft to review."`
6. **Promotion**: If the user approves, the skill is moved to the main skills directory and becomes available.

## Architecture
- **Curator Task**: A detached Tokio task spawned at application shutdown, or run in the background during idle time.
- **Drafts Directory**: `~/.local/share/goat/skills/.drafts/` keeps unapproved skills out of the active context.
- **Cost Management**: Since this runs automatically, it MUST use a low-cost profile from the `ProfileRegistry`.
- **Secret Scrubbing**: The Curator must run its output through the same regex patterns used in `SkillManager` validation to strip any accidental secrets before proposing the draft.
