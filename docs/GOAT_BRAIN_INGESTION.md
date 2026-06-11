# GOAT Brain Ingestion

The Brain in GOAT Phase 6.7 automatically ingests data from various internal systems to provide a deep, contextual memory layer.

## Ingestion Sources

1. **Skills**: Ingests files from `~/.gemini/antigravity/skills/` (excluding nested subdirectories that don't belong to the core skill file).
2. **Memory**: Ingests historical conversational memories and memory blocks stored in the Brain.
3. **PromptForge**: Ingests PromptForge templates and snippets.
4. **Reports**: Ingests system reports, agent reports, and execution summaries generated in the `data/reports/` directory.
5. **Timeline**: Ingests chronological timeline events (such as task started, PR merged, etc.) from `data/timeline/`.
6. **Runtime**: Ingests `data/runtime_jobs/` and `data/runtime_artifacts/` for traceability of executed jobs and generated outputs.

## Ingestion Process

When the index is rebuilt (`goat brain reindex` or `index_all()` at startup):
- Files are parsed based on their type.
- Secrets are automatically redacted before the file content enters the index.
- A `BrainSourceRef` is attached to every ingested document, ensuring strict attribution.
- The `content_hash` is computed to prevent duplicate indexing.
- Documents are pushed to the search index (Tantivy/BM25) and, if embeddings are enabled, pushed to the Vector index.

## Deep Ingestion

If `deep_ingestion` is enabled in `goat.toml`, the system will also traverse and index:
- Recipes
- Codebase paths (if configured)
- Additional external directories

This allows GOAT to have an unparalleled understanding of the entire workspace and operational history.
