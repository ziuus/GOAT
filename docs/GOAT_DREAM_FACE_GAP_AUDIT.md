# GOAT Dream Face Gap Audit

## Introduction
This audit compares the current state of GOAT against the "Dream Face" vision—a unified, local-first AI Agent OS. The goal is to identify where the experience feels fragmented and disconnected, and map out the changes required to bring it all together into a cohesive Mission Control workspace.

## Audit Areas

### 1. Dashboard Home & Agent Pages
**Current State:** The dashboard acts primarily as a launcher with links to distinct pages (Cofounder, Researcher, etc.).
**Gap:** It feels like a collection of separate tools rather than a unified OS. Project context is lost when navigating between these pages.
**Action:** Replace the primary focus with a "Mission Control" that acts as a central hub for all project-related activity.

### 2. Runtime/Jobs, Reports, Timeline & Browser
**Current State:** Jobs, reports, timeline events, and browser artifacts are siloed into their respective pages. The user must hunt for outputs.
**Gap:** No unified feed or artifact panel. A user cannot easily see a chronological history of a project's evolution across all these dimensions at once.
**Action:** Consolidate these into a Unified Activity Feed and Unified Artifact Panel within Mission Control.

### 3. Agent Subsystems (Cofounder, Researcher, Socializer, Designer, Operator, Builder)
**Current State:** Each agent has deep individual workflows, but they operate somewhat in isolation.
**Gap:** The user has to manually play "orchestrator," moving from one agent to another and carrying context manually.
**Action:** Introduce Global GOAT Goal Input to classify user intents and automatically recommend the right team of agents, along with the required workflow.

### 4. Command System & Memory
**Current State:** Commands are powerful in the TUI, and Memory (Brain) is robust but abstract.
**Gap:** The dashboard lacks a clear, persistent connection to the active project's memory.
**Action:** Introduce a Project Workspace concept that binds memories, jobs, and artifacts to a specific `GoatProjectId`.

### 5. AgentFlow, ApprovalGate & Provider Routing
**Current State:** ApprovalGate protects risky actions, but pending approvals are only visible when the user happens to look.
**Gap:** Pending approvals can stall progress without the user realizing it.
**Action:** Add a persistent Pending Approvals Panel to Mission Control.

## Conclusion
The core backend systems of GOAT (ApprovalGate, Agent Runtime, Provider Routing) are mature. However, the frontend presentation fails to leverage this maturity. The creation of a unified `Mission Control` will transform GOAT from a "tool collection" into an "Agent OS".
