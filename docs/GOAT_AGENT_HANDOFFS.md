# GOAT Agent Handoffs

Handoffs are explicit JSONL records passing context and intent from one Prime Agent to another.

## Fields
- `from_agent`: Source agent.
- `to_agent`: Destination agent.
- `context_summary`: The output of the source agent.
- `output_expected`: What the destination agent must produce.
- `safety_notes`: Specific safety constraints for this handoff.
