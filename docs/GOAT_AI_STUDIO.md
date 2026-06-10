# GOAT AI Studio

## Overview
Phase 5.3 introduces the GOAT AI Studio, a unified environment within the dashboard for designing, testing, and deploying intelligent assets before they become active in the main agent's brain. 

## Features
- **Prompt Lab**: An interactive playground to test prompts against different LLM profiles and operational modes (Chat, Plan, Act).
- **Model Compare**: A side-by-side view to compare the output of different models and profiles for a given prompt.
- **Builders**: Dedicated interfaces for constructing Skills, Agents, and Workflows.
- **From Memory**: An interface to convert learning candidates from the memory galaxy directly into durable skills or agents.

## Persistence & Safety
- All drafts are stored locally at `~/.local/share/goat/studio/`.
- Creating active skills, agents, or workflows from drafts requires explicit user approval via `ApprovalGate` to ensure safety and prevent silent or malicious asset injection.

## API Endpoints
- `GET /v1/studio/drafts`: List all drafts.
- `GET /v1/studio/drafts/:id`: Get a specific draft.
- `POST /v1/studio/prompt`: Execute a prompt test.
- `POST /v1/studio/skills/draft`: Save a skill draft.
- `POST /v1/studio/skills/create`: Instantiate a real skill from a draft.
