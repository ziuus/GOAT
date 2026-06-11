# GOAT PromptForge Model Mode

The `model` mode uses GOAT's internal LLM provider abstraction to refine prompts without relying on fragile browser automation.

- It sends minimal safe context.
- It restructures the prompt based on the target domain (e.g., coding, product).
- It is the preferred real implementation mode.
