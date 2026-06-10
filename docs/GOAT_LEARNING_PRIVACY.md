# GOAT Learning Privacy

## Core Philosophy
GOAT is a strictly local-first tool. Its learning capabilities are designed to augment your productivity without compromising your privacy, security, or autonomy.

## What GOAT Learns
- Project structure, tech stack, and typical commands used.
- Repetitive workflows (e.g., "always run tests after editing a rust file").
- Errors encountered and how they were resolved.
- Explicit preferences (e.g., "prefer concise answers").

## What GOAT Does NOT Learn
- Raw API keys, tokens, or passwords (stripped at the memory boundary).
- Large chunks of proprietary source code beyond necessary functional context.
- Personal data outside the context of the workspace.

## Privacy Safeguards
1. **Explicit Review**: By default, `require_review` is enabled. GOAT will never write a learned memory to your permanent database without you clicking "Accept" or running `/learn accept`.
2. **Local Storage Only**: Memory files (`USER.md`, `MEMORY.md`, SQLite databases) never leave your local machine. There is no cloud sync or external backup feature.
3. **Secret Redaction Pipeline**: Before any event is analyzed for memory extraction, it passes through a regex and entropy-based secret scrubber to ensure no tokens are persisted.
4. **LLM Summarization Opt-In**: Summarizing memories via an external LLM (like OpenAI or Groq) requires `allow_llm_summarization = true`. By default, this is disabled, meaning all summaries are deterministic and rule-based.
5. **Kill Switch**: The entire system can be turned off instantly with `/learn off` or `enabled = false` in `goat.toml`.
