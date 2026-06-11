# GOAT PromptForge Config

PromptForge is configured via `goat.toml` under the `[promptforge]` section.

```toml
[promptforge]
enabled = false
mode = "model" # mock | model | cli | api
auto_refine = false
fail_open = true
allow_browser_chat = false

[promptforge.rules]
min_complexity = "medium"
skip_simple_commands = true
require_confirmation_for_refined_prompt = false

[promptforge.agents.builder]
enabled = true
target = "coding"
```

PromptForge can be enabled globally, and further tuned per-agent.
