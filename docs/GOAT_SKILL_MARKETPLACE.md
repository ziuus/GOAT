# GOAT Skill Marketplace

## Overview
Phase 5.4 introduces the Skill Marketplace pipeline. It allows GOAT to query remote skill sources (e.g. skills.sh), fetch skill details, audit them for malicious patterns, and prompt the user via `ApprovalGate` before importing them into the local environment.

## Trust Boundaries
- **Remote Skills**: Untrusted by default. GOAT will never execute a remote skill directly.
- **Installed Skills**: Trusted after passing Audit and ApprovalGate. They are stored locally at `~/.config/goat/skills/`.

## Architecture
- `src/skill_marketplace.rs`: Connects to configured remote endpoints (auth handled via OIDC or fallback). Caches metadata in `~/.local/share/goat/skill-marketplace-cache/`.
- `SkillAuditReport`: Parses skill code to detect `curl | sh`, `rm -rf`, or other unsafe constructs. Outputs risk levels: `low`, `medium`, `high`, `critical`.
- Local installation attaches provenance metadata in `skill.meta.json` inside the skill's directory.

## Commands
- `/skills remote search <query>`
- `/skills remote audit <id>`
- `/skills remote install <id>`
- `/skills provenance <name>`
