# GOAT Socializer Agent Plan

## Vision
The Socializer Agent is a Prime Agent in GOAT focused on distribution, community engagement, and personal branding. It acts as an ethical marketing assistant that values high-signal content over automated spam.

## Core Responsibilities
1. **Content Strategy:** Planning a content calendar across multiple platforms (X, LinkedIn, Reddit, Hacker News).
2. **Draft Generation:** Creating drafts for posts, launch announcements, and community replies.
3. **Outreach Formatting:** Structuring cold outreach emails or DMs to be concise, polite, and personalized.
4. **Platform Etiquette:** Adhering to the unwritten rules of platforms (e.g., no promotional links in the main Reddit post).

## Attached Specialists
The Socializer Agent can delegate platform-specific tasks to:
- **Reddit Strategist:** Analyzes subreddit rules and tone.
- **LinkedIn Writer:** Formats for professional storytelling.
- **X/Twitter Writer:** Optimizes for hooks, brevity, and thread pacing.
- **Community Manager:** Drafts empathetic and helpful replies to user questions.
- **Outreach Writer:** Crafts B2B cold emails.
- **Launch Planner:** Coordinates Product Hunt / HN launch materials.

## Interaction Model
- **Trigger:** Initiated via `/agents run socializer draft-launch` or `@socializer what's the best way to post this to Reddit?`.
- **Outputs:** Generates structured plans (`social_content_plan.md`) or draft text in the AI Studio / Chat.
- **Ethics & Safety:**
  - **ABSOLUTE RULE:** No spam automation.
  - The agent CANNOT autonomously post to social media APIs or send emails. It only creates *drafts*.
  - The user must explicitly copy-paste the output or approve it via the ApprovalGate if a safe API integration is later added.
  - Ethical distribution: focuses on providing value, not manipulating algorithms.

## Current Status (Phase 5.16)
- The architecture and manifest exist.
- State: `Planned`.
- The full generative behavior loop is scheduled for Phase 5.17+.
