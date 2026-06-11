# GOAT Context Packs

Context Packs are a powerful feature in GOAT Phase 6.7 that bundle relevant information into a single, cohesive object. Agents can instantly ingest these Context Packs at the beginning of a task to get up to speed without needing to perform multiple individual queries.

## Structure

A `ContextPack` consists of:
- `title`: The query or intent behind the pack.
- `summary`: A synthesized summary of the packed items.
- `items`: The matching documents (`BrainDocument`) retrieved from the Brain.
- `estimated_size`: Approximate character length of the pack (useful for context-window management).

## How They Work

When a user or an agent requests a Context Pack (`goat brain pack <query>`), the `BrainContextPackBuilder`:
1. Executes a hybrid search against the Brain index using the query.
2. Filters out low-relevance results.
3. Groups the results by `BrainDocumentKind` (e.g., Skills, Memory, Timeline, Reports).
4. Generates a synthesis/summary of why these items are relevant.
5. Returns a structured JSON-ready `ContextPack`.

## Usage in Agents

During initialization or task assignment, an Agent can be supplied with a `ContextPack`. Instead of the Agent asking "What happened yesterday?", the Agent's system prompt or initialization payload will already contain the pre-assembled pack, significantly saving LLM round-trips and tokens.
