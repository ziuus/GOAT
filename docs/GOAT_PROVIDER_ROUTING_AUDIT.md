# GOAT Provider Routing Audit

## 1. Existing Providers
Currently, the LLM routing system supports:
- **openai**: Works via the OpenAI API, configurable via `OPENAI_API_KEY`.
- **groq**: Works via the OpenAI-compatible API, configurable via `GROQ_API_KEY`.
- **openrouter**: Works via the OpenAI-compatible API, configurable via `OPENROUTER_API_KEY`.
- **ollama**: Works via the OpenAI-compatible local API without authentication.

Planned, but not fully implemented (currently skipping them or causing failure):
- **anthropic**: Planned, not working.
- **gemini**: Planned, not working.

## 2. Hardcoded Assumptions
- `LlmRouter` (in `src/llm.rs`) contains hardcoded fields for keys and base URLs (`openai_key`, `groq_key`, `openrouter_key`, `ollama_base_url`, etc.).
- There is no unified provider abstraction for configuration and capabilities. `llm_config` deals only with retries, timeout, and simple fallback toggles.
- The `completion_with_fallback` loop manually matches on string constants (`"openai"`, `"groq"`, `"openrouter"`, `"ollama"`) inside `dispatch_to_provider()`.
- The `ProviderError` classification handles OpenAI-like responses well but may not map correctly for APIs like Anthropic and Gemini.

## 3. Provider Abstraction (Resolved in Phase 6.6)
Implemented in Phase 6.6:
- A `ModelProviderRegistry` that aggregates an arbitrary number of defined providers dynamically.
- Dynamic `capabilities` (e.g., `vision`, `streaming`, `long_context`, `embedding`) are now associated with providers.
- Routing context like `local_only` vs `cloud` is fully handled by `ModelRouteRequest` in `providers.rs`.
- Extensibility for other OpenAI-compatible and LiteLLM gateways natively via the generic `call_openai_compatible` pattern without hardcoded `dispatch` cases.

## 4. Current Fallback Behavior
- Controlled by `[llm]` section (`max_retries`, `fallback_on_rate_limit`, etc.).
- If an API key is missing for a provider in the chain, it logs a warning and gracefully falls back to the next one.
- If all providers are exhausted, it throws `ChainExhausted`.
- `LlmRouter::is_error_fallback_allowed` checks if an error is transient (e.g., rate limit, network, server errors). 4xx errors (except 429) stop the chain immediately.

## 5. How Providers Should Plug In
- **Direct Provider Adapters**: Native implementations for Google Gemini, Anthropic Claude, etc.
- **OpenAI-Compatible Adapters**: A unified adapter taking arbitrary `base_url` and `api_key_env` (e.g., LiteLLM, Groq, OpenRouter, LM Studio, vLLM).
- **Registry**: Configs from `goat.toml` should map directly into a registry of loaded provider structs.
- **Gateway Abstraction**: Should permit transparent use of LiteLLM without needing a hardcoded switch block.

## 6. What is Implemented Now
- Fallback chain mechanics over `ProviderError`.
- Profile parsing (`balanced`, `coding`, `cheap`) into ordered `ModelChain`s.
- `goat.toml` configuration loading for static LLM settings and basic OpenRouter / Ollama settings.

## 7. What is Deferred (Phase 6.7 and beyond)
- Complex native protocols for non-OpenAI compliant clouds (Anthropic native SDK features, Google AI Studio native endpoints).
- Advanced embeddings generation routing (currently placeholder).
- Image generation and Text-to-Speech provider routing.
