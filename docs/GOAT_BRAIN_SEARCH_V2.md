# GOAT Brain Search V2

GOAT Brain Search V2 represents a significant upgrade from the basic searchable memory into a useful local intelligence layer that holds the entire history of the project, agents, decisions, and system interactions.

## Key Features

1. **Hybrid Retrieval**: Combines keyword search (BM25 or similar basic keyword index) with semantic search (Embeddings) to provide highly accurate, relevant results.
2. **Context Packs**: Combines results from Brain Search into a single, cohesive context payload that agents can ingest instantly when entering a new task or conversation.
3. **Deep Ingestion**: Ingests not only user messages but also system reports, timeline events, and runtime artifacts to provide a full 360-degree view of system activity.
4. **Source Attribution**: Every piece of data returned from the Brain includes exact source attribution, providing traceability back to the original source (`source_id`, `source_kind`, `source_path`, `content_hash`).

## Search Modes

- **Keyword**: Standard keyword search across indexed data.
- **Semantic**: Search based on the meaning of the query using embeddings.
- **Hybrid**: A combined approach using both Keyword and Semantic search to provide maximum relevance.

## CLI Commands

You can interact with Brain Memory via CLI:
- `goat brain dedupe`: Deduplicates redundant entries in the Brain index.
- `goat brain pack <query>`: Generates a Context Pack based on the given query.

## Dashboard

The Dashboard includes a `/brain` page for interactively querying memory using various search modes, visualizing index size, rebuilding embeddings, and deep-reindexing data.
