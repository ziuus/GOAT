# GOAT Provider Roadmap

GOAT is currently tightly coupled with a basic OpenAI/Groq API client natively built into the rust daemon. As we mature the project, we want to expand the model provider abstraction.

## Future Direction (LiteLLM / Gateways)
A contributor recently asked about integrating **LiteLLM**. 

* **The Goal**: A provider abstraction is definitely part of the GOAT direction.
* **LiteLLM**: LiteLLM could be an excellent optional provider backend or model gateway later.
* **Core Philosophy**: There will be no hard dependency on external API servers in the core Rust binary.
* **Configuration**: Any external provider gateway will be config-based (enable/disable in `goat.toml`).
* **Fallback**: We will maintain a strong local-first and fail-open/fallback behavior so users without cloud APIs can still run local Ollama models.

If you are interested in contributing to the provider interface, please open a discussion or issue before writing a large PR.
