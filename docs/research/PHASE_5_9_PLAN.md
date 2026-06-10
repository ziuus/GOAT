# Phase 5.9 Implementation Plan

## 1. Quick Access Prefix Architecture
We will implement a `QuickAccessParser` in `src/quick_access.rs` that takes any input string and normalizes prefix grammar (`@`, `#`, `~`, `|`, `\`) into canonical `/` slash commands. This parser will be invoked at the very beginning of user input handling in the TUI (`src/app.rs`), Headless mode (`src/headless.rs`), and the Dashboard API (`src/api_server.rs`). This ensures all layers of the app seamlessly support the new syntax without duplicating logic. Prefix suggestions will map directly to their canonical `/` counterparts.

## 2. Skill Researcher Architecture
A new `SkillResearcher` struct will manage the discovery of skills across various sources (local, marketplace, brain search, memory galaxy). The mode's state (ON/OFF) will be kept in the current session's memory. It will fetch from multiple providers asynchronously and rank results. Configuration will be added to `src/config.rs` under `SkillResearcherConfig`.

## 3. Session Skill Lifecycle
Session skills will be stored in memory during the session and backed up to `~/.local/share/goat/session-skills/<session_id>.json`. The lifecycle is:
1. Suggested (by Researcher)
2. Attached (active in session context)
3. Detached (removed from session)
4. Promoted (saved to a Skill Pack or installed globally).

## 4. Skill Pack Model
Stored in `~/.config/goat/skill-packs/<pack_name>/`. A pack contains a `pack.meta.json` (metadata, references to skills) and `PACK.md` (description/docs). Reusing a pack simply attaches its referenced skills to the current session.

## 5. Cross-Session Reuse Design
Since session skills are strictly tied to the session ID, reusing them in a new session requires explicit promotion to a Skill Pack or using `/skills from-session <id>`. No silent bleed-over.

## 6. Dashboard UX
A new `/skills/research` page will be added to the Next.js app. It will allow searching, toggling Researcher mode, displaying suggested skills with trust badges, and managing session skills/packs.

## 7. Privacy/Security Model
Skill Researcher is OFF by default. No remote skills are attached without explicit user review. Web research is disabled by default. No secrets are stored in packs or session state.

