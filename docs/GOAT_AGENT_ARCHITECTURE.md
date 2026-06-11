# GOAT Agent Architecture

GOAT is designed as an Agent Operating System (OS). It provides the foundational runtime, memory, safety, and coordination layers. On top of this OS, GOAT runs a hierarchy of intelligent agents designed to handle specific domains and tasks.

## The Hierarchy

1. **Prime Agents**
   - **Role:** Own large domains of work and coordinate long-term strategies.
   - **Capabilities:** Coordinate workflows, attach specialist agents, use skills, generate structured reports, and manage long-term project memory.
   - **Examples:** Cofounder, Builder, Researcher, Socializer, Designer, Operator, Learner.

2. **Specialist Agents**
   - **Role:** Focused domain experts that attach to Prime Agents to handle specific niches.
   - **Capabilities:** Provide deep expertise on a narrow topic (e.g., SEO, security, finance) and advise or generate outputs for the Prime Agent.
   - **Examples:** SEO Analyst (attaches to Socializer), Unit Economics Analyst (attaches to Cofounder), DevOps Specialist (attaches to Operator).

3. **Subagents**
   - **Role:** Temporary task executors spawned for a single session, task, or job.
   - **Capabilities:** Execute well-defined instructions in isolation. Destroyed or archived after the task is completed.
   - **Examples:** Bug-hunter, Refactorer, Documenter.

## How Agents Differ from Other Primitives

- **Prime/Specialist/Subagents:** Autonomous or semi-autonomous entities with domain responsibilities and state.
- **Skills:** Reusable, stateless packages of know-how (e.g., "How to scrape a React website"). Agents *use* skills.
- **Recipes / Workflows:** Repeatable procedural steps (e.g., "Weekly codebase audit"). Agents *execute* recipes.
- **Tools (MCP):** External abilities (e.g., `git commit`, `execute_query`). Agents *call* tools.

## Architecture & Interfaces

Agents in GOAT plug into the OS via a unified interface:
- **Manifests:** Defined in `manifest.toml` (ID, name, tier, allowed tools, safety policies).
- **Storage:** Data is scoped cleanly:
  - Configuration: `~/.config/goat/agents/`
  - State/Data: `~/.local/share/goat/agents/`
- **Memory & Brain Search:** Agents do not duplicate global memory. They reference global memory but maintain scoped summaries and context.
- **ApprovalGate:** All risky actions (e.g., network access, file mutation, social posting) must pass through the ApprovalGate.
- **Reports:** Agents produce structured outputs via the ReportSystem, avoiding unstructured terminal spam.

## Future Extensibility
This architecture is built to be horizontally scalable. While GOAT ships with a set of built-in Prime and Specialist agents, users can define their own Custom Agents via the AI Studio or by placing a valid `manifest.toml` in the configuration directory.
