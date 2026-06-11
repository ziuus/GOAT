# GOAT Brain Privacy & Security

GOAT's Brain is built on a strict "Local-First, Privacy-First" architecture.

## Local Embedding Requirement

By default, external embedding providers (like OpenAI or Gemini embeddings) are **disabled**.
- GOAT Brain relies on local BM25 keyword search if embeddings are disabled.
- To enable semantic search, a local embedding model (e.g., `all-MiniLM-L6-v2` via ONNX or a local provider) should be configured.
- If a user wishes to use a cloud provider for embeddings, they must explicitly opt-in via `goat.toml` under `[embeddings.provider]`.

## Secret Redaction

The Brain features an automatic redaction step during ingestion. 
Before any string of text is placed into the index or sent to an embedding model:
- Regex patterns detect high-entropy strings, API keys, tokens, and passwords.
- Identified secrets are replaced with `[REDACTED]`.
- The `redaction_status` of the resulting `BrainDocument` is flagged.

## No Secret Indexing

Under no circumstances are secrets stored in the `goat_brain.db`, the Tantivy index, or the Vector DB. The redaction happens *before* ingestion.

## Manual Reindexing and Deduplication

Users have full control over the Brain index.
- `goat brain dedupe`: Will safely remove duplicates without leaking data.
- `goat brain reindex`: Will completely rebuild the index from the filesystem, ensuring that if a user deletes a sensitive file locally, it is permanently removed from the Brain on the next reindex.
