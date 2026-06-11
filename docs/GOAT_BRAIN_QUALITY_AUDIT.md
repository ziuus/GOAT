# GOAT Brain Quality Audit

## Current State Analysis (Pre-Phase 6.7)

### 1. Semantic Capabilities
- **What is actually semantic now:** Very little. The system is designed to support embeddings, but it relies on an `OllamaProvider` or `MockProvider`.
- **Where embeddings are mocked/simple:** The `MockProvider` creates a fake vector using a sine wave generation based on a hash of the text. This provides absolutely zero semantic value.
- **What is keyword/fuzzy only:** The existing search in `src/brain_index.rs` defaults to `BrainSearchMode::Keyword` and its internal matching likely relies on simple text inclusion or basic scoring if semantic mode isn't explicitly configured and connected to Ollama.
- **Provider Routing:** Embeddings do NOT use the Phase 6.6 `ModelProviderRegistry`. They use a standalone hardcoded `EmbeddingsConfig` (Ollama or mock).

### 2. Data Indexed
- **What data is indexed:**
  - Memory Blocks (`src/memory.rs` notes)
  - Skills (`SKILL.md` files)
  - PromptForge templates and history
  - Optionally: Recipes, Studio Drafts, Jobs, Approvals, Audit Logs, Checkpoints (via `deep_ingestion` config).
- **What data is not indexed:**
  - Timeline events (`timeline.jsonl`)
  - Reports (`reports/`)
  - Cofounder ideas, Learner goals, Researcher briefs
  - Collaboration handoffs and AgentRuntime session contexts in a structured way.

### 3. Gaps and Weaknesses
- **Ranking Weaknesses:** The current ranking simply returns `BrainSearchResult` with `score`, `keyword_score`, and `semantic_score`. But if embeddings are mock, the score is arbitrary. There is no hybrid ranking logic combining recency, source reliability, or context matches.
- **Source Attribution Gaps:** `BrainDocument` has a basic `source_path` and `project_id`. However, it lacks `source_kind`, agent attribution, timestamps (beyond creation/update), content hashes, and specific relations to timeline events or reports.
- **Deduplication Gaps:** The system only avoids re-embedding exact content hashes in `rebuild_embeddings`. It does not deduplicate across sources (e.g. if the same prompt history is ingested twice or a report is updated).
- **Privacy Risks:** The system attempts to block ingestion if `contains_secrets` is true (checking for "sk-", "password=", etc.), but this is a naive text check. External embeddings are somewhat disabled unless configured, but a better structured default is needed.

### 4. Upgrade Plan (Fixed in Phase 6.7)
- **Document Model:** Introduce `BrainSourceRef`, `BrainChunk`, improved `BrainDocumentKind`, and metadata like `source_agent`, `source_project`.
- **Source Attribution:** Results will return detailed source information, reason for match, and agent/project links.
- **Deduplication:** A proper content hash and dedupe command (`/brain dedupe`) will be built to safely mark duplicates.
- **Embedding Routing:** Replace the standalone Ollama/Mock embedding clients with the `ModelProviderRegistry` from Phase 6.6. It will default to a local/mock mechanism unless explicitly requested.
- **Hybrid Ranking:** Combine keyword score, fuzzy match, semantic (if available), recency, and source boosts.
- **Context Packs:** Build `BrainContextPackBuilder` to produce tailored contexts for specific agents.
- **Ingestion:** Expand to Reports, Timeline, and Runtime Jobs.

### 5. What Remains Partial (Deferred to Phase 6.8+)
- Full Cofounder/Learner specific visual integration.
- Deep visual memory galaxy upgrades (graph UI will use improved metadata but remain conceptually similar).
- Automated background semantic cleanup beyond manual `/brain dedupe`.
