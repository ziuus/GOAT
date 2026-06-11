# GOAT Prime Researcher Agent

**Status:** Active
**Phase:** 5.22 Complete
**Tier:** Prime Agent

## Overview

The `ResearcherAgent` is a Prime Agent responsible for conducting deep, source-grounded research, technology comparisons, competitor scans, and generating comprehensive briefs for other agents (like Designer or Builder).

It is strictly instructed to avoid hallucinations: it must ground all claims in verifiable sources, clearly distinguish between facts, weak signals, assumptions, and unknowns.

## Storage
All data is stored in `~/.local/share/goat/agents/prime/researcher/`:
- `topics.jsonl`: Managed research topics
- `plans.jsonl`: Execution plans for each topic
- `sources.jsonl`: Collected references, links, or file paths
- `evidence_notes.jsonl`: Extracted notes strictly tied to sources
- `reports.jsonl`: Final generated briefs, market scans, or competitor analysis

## Workflow

1. **Topic Creation**: User creates a `ResearchTopic` outlining the core question and domain.
2. **Plan Generation**: Agent proposes a `ResearchPlan` detailing sub-questions, keywords, and priority sources.
3. **Source Collection**: Agent gathers `ResearchSource` records (URLs, papers, internal docs).
4. **Evidence Extraction**: Agent parses sources and creates `ResearchEvidenceNote` entries linking claims to sources.
5. **Synthesis**:
   - `generate_competitors`: Produces a `ResearchCompetitorScan`.
   - `generate_compare`: Produces a `ResearchTechnologyComparison`.
   - `generate_brief`: Generates a high-level `ResearchBrief` for humans/agents.
6. **Handoff**: Research artifacts are formatted for the Builder, Designer, or Cofounder to consume.

## Architecture

- Standard Prime Agent structure using `GoatPaths` and `serde`.
- `PromptForge` Optional Layer (if enabled).
- Timeline Events: `ResearcherTopicCreated`, `ResearcherPlanCreated`, etc.
- Dashboard Integration at `/researcher`.
- Subcommands: `/researcher list`, `new-topic`, `plan`, `sources`, `notes`, `competitors`, `compare`, `market`, `brief`, `report`.
- Aliases: `@researcher`, `@research`, `@sources`, `@competitors`, `@compare`.

## Future

- Subagent Spawning for large parallel web scraping (Phase 6+).
- Real-time Deep Research Integration via external APIs.
