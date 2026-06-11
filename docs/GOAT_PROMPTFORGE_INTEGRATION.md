# GOAT PromptForge Integration

PromptForge acts as an optional prompt refinement layer inside GOAT. 
It converts rough human requests into clearer, structured, agent-ready instructions before execution.

## Core Flow
1. User provides a rough request.
2. GOAT receives the task.
3. If PromptForge is enabled, the request is refined.
4. GOAT receives the refined prompt.
5. GOAT sends the refined prompt to the selected Prime Agent.

## Important Notes
- PromptForge is OPTIONAL and disabled by default.
- It must not bypass ApprovalGate.
- It does not automate browser web-chat (ChatGPT/Claude/Gemini) directly due to fragility.
