# GOAT v0.14.0-alpha.1 — Public Alpha

## What is GOAT?
GOAT is a local-first AI Agent OS for building, learning, research, automation, workflows, and safe multi-agent collaboration. It provides a unified system combining local AI models, specialized Prime Agents, a secure ApprovalGate, persistent Memory Galaxy, and a rich Next.js dashboard.

## What works in this alpha 🟢
* **Web Dashboard**: An overhauled, modular Next.js 15 application.
* **TUI / Daemon**: A fully functional local Rust server and terminal UI.
* **Cofounder (UI)**: Formulate and scope ideas, with an actionable scorecard.
* **Learner (UI)**: Multi-track roadmap generation and daily practice modules.
* **PromptForge**: Automatic context refinement and compiler.
* **Memory Galaxy**: SQLite-based memory blocks and file indexing.
* **ApprovalGate**: Correctly intercepts and blocks terminal commands until human review.

## What is experimental 🧪
* **Agent Collaboration**: The multi-agent workflow backend exists but is not fully exposed in the web dashboard.
* **Socializer, Designer, Operator**: UI shells are built, but the backend implementation is pending.
* **Vector Embeddings**: Real semantic search embeddings are still in progress (currently using mocked/text fallback).

## Quick start
```bash
git clone https://github.com/ziuus/GOAT.git
cd GOAT
cargo build --release
cargo run --release -- daemon start
```
Then, in another terminal:
```bash
cd apps/dashboard
npm install
npm run dev
```
Visit `http://localhost:3000`. 

## Safety notes 🛡️
* **ApprovalGate** intercepts dangerous shell commands and API actions.
* The system is designed to be **Local-first**, meaning your data stays on your machine.
* Cloud model integrations (like OpenAI) are entirely **opt-in**.

## Known limitations
* This is an active Alpha release. The API, paths, and DB schemas will change rapidly.
* Some complex agent routines (e.g. Designer or Operator logic) use simple generations or mocks while their full backend pipelines are being finalized.

## Feedback wanted
We are looking for testers, builders, and developers! If you run into UX friction, missing docs, or bugs, please use the new **Alpha Feedback** issue template in the repo.

## Screenshots
*(See the attached images below for a preview of the dashboard and agents).*

## Full changelog
See `CHANGELOG.md` in the source repository for detailed commits.
