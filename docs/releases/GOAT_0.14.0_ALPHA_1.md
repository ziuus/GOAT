# GOAT v0.14.0-alpha.1 Release Notes

Welcome to the first public alpha of **GOAT**: the local-first AI Agent OS for building, learning, research, automation, and safe multi-agent workflows.

## What is GOAT?
GOAT is a cohesive platform that brings local AI agents, memory (Memory Galaxy), tools, an ApprovalGate for safety, and an extensible web dashboard into a single system. It is designed to be completely local, inspectable, and explicit.

## What Works Today 🟢
* **Web Dashboard**: An overhauled Next.js 15 application.
* **TUI / Daemon**: A fully functional local rust server and terminal UI.
* **Cofounder (UI)**: Formulate and scope ideas, with an actionable scorecard.
* **Learner (UI)**: Multi-track roadmap generation and daily practice modules.
* **PromptForge**: Automatic context refinement and compiler.
* **Memory Galaxy**: SQLite-based memory blocks and file indexing.
* **Safety First**: The `ApprovalGate` correctly blocks terminal commands until human review.

## What is Experimental / Partial 🧪
* **Agent Collaboration**: The multi-agent workflow backend exists but is not fully exposed in the web dashboard.
* **Socializer, Designer, Operator**: UI shells are built but backend implementation is pending.
* **Vector Embeddings**: Real semantic search embeddings are still in progress.

## Installation / Quick Start

```bash
git clone https://github.com/ziuus/GOAT.git
cd GOAT

# Build the Rust Daemon
cargo build --release

# Start the Daemon
cargo run --release -- daemon start

# In a separate terminal, start the Dashboard
cd apps/dashboard
npm install
npm run dev
```

Visit `http://localhost:3000`. You can click "Load Demo Data" in Settings to pre-populate views if no live data exists yet.

## Known Limitations
* This is an active Alpha. Not a production autonomous agent.
* Integrations with external model providers are currently experimental.
* Certain complex workflows might still stub outputs in the dashboard.

## Feedback Wanted!
We need your feedback! If you find a bug, have a feature request, or are confused by the setup, please use our new **Alpha Feedback** issue template on GitHub.

## Next Roadmap
* Wiring up the backend persistence layer for Cofounder and Learner states.
* Expanding the multi-agent collaboration UI in the web dashboard.
* Full vector embeddings support in Memory Galaxy.
