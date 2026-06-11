# GOAT Ethical Distribution Policy

As an agentic framework, GOAT possesses the capability to generate and distribute content at a superhuman scale. The **Ethical Distribution Policy** exists to ensure this capability is used to create value, not spam.

## The Problem
"AI Spam" ruins communities. Automated bots scraping content, spinning it, and blasting it across Reddit, Hacker News, X, and LinkedIn degrades the signal-to-noise ratio of the internet.

## The GOAT Solution
The `Socializer` Prime Agent is hardcoded with constraints that prevent it from engaging in these behaviors.

### 1. No Auto-Posting
The `Socializer` is physically disconnected from direct platform APIs by default. It outputs `SocializerContentDraft` objects. It is up to the human operator to copy, refine, and publish the draft. Even if a custom API transport is added, the `ApprovalGate` requires explicit cryptographic or manual sign-off for every outbound payload.

### 2. Platform-Specific Etiquette
The `Socializer` models platform etiquette natively:
- **Reddit**: It knows that promotional links in the main post body will get you banned. It defaults to "Story/Architecture" formats that provide standalone value.
- **Hacker News**: It understands the need for extreme technical depth and the rejection of marketing speak.
- **LinkedIn**: It optimizes for professional storytelling without crossing into algorithmic "cringe."

### 3. Value-First
Every campaign requires a defined `value_proposition`. The Socializer measures its drafts against this proposition. If a draft is pure self-promotion without offering the reader a takeaway, lesson, or resource, the agent will flag it internally and suggest a rewrite.

### 4. Non-Promotional Alternatives
Every `SocializerContentDraft` includes a `non_promotional_version`. This encourages users to test the waters in strict communities by sharing the core learning first, rather than immediately linking to their product.

## Summary
GOAT is a tool for builders to amplify their reach, not a tool for spammers to pollute the commons. The Socializer Agent enforces this philosophy at the architectural level.
