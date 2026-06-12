# GOAT Socializer Quality Audit

## 1. Current State of Socializer
The `SocializerAgent` currently exists as a stubbed proof-of-concept. All of its methods (`generate_audience_map`, `generate_channel_strategy`, `generate_content_angles`, `generate_draft`, `generate_launch_plan`, `generate_calendar`, `generate_outreach`, `track_feedback`) return hardcoded dummy data.

**What it can actually do now:**
- Maintain basic state for `SocializerCampaign` (saving/loading from `campaigns.jsonl`).
- Advance state linearly through a basic enum (`Draft`, `AudienceMapped`, etc.).
- Expose endpoints via `src/api_server.rs` that serve these hardcoded mock responses.
- Generate generic `CofounderReport` entries using the `ReportManager`.

**What is shallow/simple:**
- The content generation is entirely mocked, lacking real LLM integration, prompt structures, or contextual awareness of the user's project/idea.
- Audience mapping and channel strategies ignore project data.
- Spam risk and ethical constraints are just hardcoded strings in the output structs, not actual logic or policies.
- Tone, brand constraints, and user-specific profiles do not exist.

## 2. Platform Support
**Current:**
- Mock data references "Reddit", "X", "LinkedIn", and "Email", but there is no explicit platform-aware data structure dictating the constraints or etiquette per platform.

**Needed:**
- Explicit `SocialPlatform` mapping with content limits, link handling etiquette, and anti-spam norms per platform.

## 3. Anti-Spam Safety
**Current:**
- Hardcoded warnings (`warnings: vec!["Do not post this in self-promotion free zones."]`).

**Needed:**
- Deep `SocialDistributionPolicy`, `SocialSpamRisk` calculation, and a `SocialSafetyCheck` that prevents the creation of risky drafts (e.g., auto-DMs, fake testimonials, astroturfing).

## 4. Integration Context
**Current:**
- No actual use of `Cofounder` validation reports, `Researcher` sources, or `Builder` reports. The campaign takes an optional `project_or_idea_ref` but doesn't resolve it via `BrainIndexManager`.

**Needed:**
- Fetching context packs from `BrainIndexManager` (Cofounder ideas, Researcher citations, Builder release notes).
- Generating source-backed drafts where claims are linked to `SocialContentSourceRef`.

## 5. Dashboard / Runtime Integration
**Current:**
- Dashboard endpoints exist in `api_server.rs` but return mocked values.
- Dashboard UI (/socializer) likely renders these mocked structs.

**Needed:**
- Implementation of the deep workflows in `SocializerAgent` using `LlmRouter` and `BrainIndexManager`.
- Exposing the new endpoints and structures (Content Calendar, Profiles, Review logic) in the API server.
- Creating the `socializer_*` job kinds in `AgentRuntime`.
- `PromptForge` templates for social content.

## 6. Phase 7.7 Implementation Focus
We will replace the stubbed logic with LLM-orchestrated distribution assets that focus heavily on:
1. **Source-backed claims** (using Cofounder & Researcher artifacts).
2. **Ethical constraints** (No automated DMs, no fake testimonials).
3. **Platform-specific formatting** (Reddit rules, X constraints, LinkedIn styles).
4. **Content Calendars & Launch Planning**.
