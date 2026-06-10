# GOAT skills.sh Integration

Phase 5.4 connects GOAT to `skills.sh`.

## Auth and Caching
- `skills.sh` requires authentication (e.g. Vercel OIDC) to pull proprietary or rate-limited skills.
- The `[skill_marketplace]` config section handles endpoint URL and auth mechanism.
- If auth is missing, GOAT gracefully defaults to offline mode or mock mode.
- Remote searches are cached in `~/.local/share/goat/skill-marketplace-cache/` to limit API calls.

## Core Principle
- `skills.sh` is solely for **discovery**. It cannot remotely trigger actions on the user's GOAT instance.
- Users must manually explicitly invoke `/skills remote install <id>`, which initiates the local `SkillAuditPipeline`.
