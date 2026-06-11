# GOAT PromptForge Safety

PromptForge prioritizes safety and stability:
- **No Direct Execution**: PromptForge only refines prompts, it never executes commands.
- **ApprovalGate**: All high-impact tasks still require ApprovalGate clearance after refinement.
- **Secrets**: PromptForge does not receive environment variables, tokens, or private messages.
- **Fail Open**: If PromptForge fails, GOAT can safely fall back to the original prompt.
