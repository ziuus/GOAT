# GOAT Prime Learner Agent

**Status:** Active
**Phase:** 5.24 Complete
**Tier:** Prime Agent

## Overview

The `LearnerAgent` is a Prime Agent designed to facilitate structured, project-based learning. It replaces generic chatbot tutoring with trackable, realistic study plans across domains such as DSA, AI/ML, Rust, Web3, Full-stack, and System Design.

The defining characteristic of the Learner Agent is its **Realistic Scheduling**:
1. **No Unrealistic Mastery**: It will not generate 12-hour/day burnout schedules unless explicitly forced.
2. **Modular Progression**: Roadmaps are broken into phases, weekly plans, and daily tasks.
3. **Practice & Revision Focus**: It tracks what you know, what you got wrong, and schedules revision.

## Storage
All data is stored in `~/.local/share/goat/agents/prime/learner/`:
- `goals.jsonl`: User-defined learning goals.
- `roadmaps.jsonl`: Structured learning paths.
- `practice_tasks.jsonl`: Generated hands-on exercises.
- `revision_checkpoints.jsonl`: Points of review and self-assessment.
- `progress.jsonl`: Historical progression entries.
- `reports/`: Aggregated learning reports.

## Architecture

- Uses `GoatPaths` for file-based JSONL storage.
- Dashbaord accessible at `/learner` for goal tracking.
- CLI/TUI accessible via `/learner` subcommands or `@learn`, `@dsa`, `@rust` aliases.
- Subcommands: `list`, `new-goal`, `assess`, `roadmap`, `week`, `today`, `practice`, `revise`, `project`, `exam`, `progress`, `report`.

## Supported Domains
- DSA (Data Structures & Algorithms)
- AI/ML (Artificial Intelligence & Machine Learning)
- Rust (Systems Programming)
- Web3 (Decentralized Apps)
- Full-Stack (Web Development)
- System Design
- Exam Prep
- Project-Based
- General

## Security and Quality Rules

- **Syllabus Accuracy**: The Learner will clearly state if it is assuming a syllabus vs following a provided one. It must not hallucinate official exam details.
- **Privacy**: Does not store unnecessary sensitive personal details in the progress logs.
- **Handoffs**: Learner can prepare "Builder Handoffs" for project-based tasks, but will not automatically execute code generation.

## Future Phases

- Integration with PromptForge for highly customized, external API-driven syllabus generation.
- Deep Brain Search integration to retrieve specific past code mistakes to turn into revision exercises.
