# GOAT Context System

This document outlines the design and usage of the GOAT Context System introduced in Phase 3.6.

## Overview
GOAT automatically manages token budget limits and injects context into AI prompts to enable multi-file contextual awareness. To prevent overwhelming the model or leaking sensitive data, GOAT enforces a strict budget limit and utilizes an automated secret detection system via `repo_map.rs`.

## Context Flow
1. **Selection:** Users add files to the context via `/context add <path>`.
2. **Security Check:** Files are checked against the `.gitignore` tree and secret-detection heuristics (`looks_like_secret_file`).
3. **Budget Calculation:** Before generating an AI prompt, the context budget is calculated. The system has a hard limit (e.g. 20,000 chars) to prevent context exhaustion. 
4. **Injection:** The repo map summary is injected first, followed by the selected files (truncated if necessary).
5. **Prompt:** The system runs the standard workflow `plan` / `act` with the injected context explicitly isolated within XML tags (`<selected_files>`).

## Slash Commands
- `/context show`: Switches the UI view to the dedicated Context sidebar.
- `/context add <path>`: Adds a file to the prompt builder after validating it's not a secret.
- `/context remove <path>`: Removes the file from the current selection.
- `/context clear`: Flushes all context files.
- `/context budget`: Analyzes the total characters and bytes of the current selection.
- `/files relevant <query>`: (Planned) Will use local vector search / BM25 to suggest relevant files to add to context.

## AI Commit Generation
`/commit message ai` extracts `git diff` and `git status --short` and runs an LLM generation step to output a conventional commit message. The prompt enforces no markdown wrappers and zero fluff. This is fully integrated into both TUI and Headless environments.

## Dashboard Integration
Phase 4.3 exposes the `/v1/context` API endpoints (GET, POST `/add`, POST `/remove`, POST `/clear`) allowing users to interactively construct the context selection safely via the Dashboard's Repo Explorer.
