# GOAT (General Omniscient Agentic Tool)

<div align="center">
  <p><strong>The Local-First AI Agent OS for Coding, Automation, and Workflows</strong></p>
  <p>
    <a href="#quick-start">Quick Start</a> •
    <a href="#features">Features</a> •
    <a href="#architecture">Architecture</a> •
    <a href="#security">Security</a> •
    <a href="docs/README.md">Documentation</a>
  </p>
</div>

---

GOAT is a comprehensive, local-first AI Agent Operating System designed for developers. It bridges the gap between powerful LLM reasoning and safe, local execution. Whether you need a simple CLI coding assistant, an autonomous desktop daemon, or a web-based AI studio, GOAT provides a unified architecture to manage it all.

> **Status:** Active Alpha Development 🚀
> GOAT is actively evolving. Expect rough edges and rapid updates.

## 🌟 Key Features

* **Local-First & Secure by Default:** GOAT runs locally. All dangerous actions (file writes, command execution, API calls) are blocked by an interactive `ApprovalGate`. 
* **Multi-Interface Support:** Interact with GOAT via a rich TUI, a headless CLI, a persistent background daemon, a modern web dashboard, or a Tauri desktop app.
* **Agent OS Architecture:** GOAT acts as the operating system for a hierarchy of intelligent agents:
  - **Prime Agents:** Built-in domains (e.g., Cofounder, Socializer, Researcher, Builder) that own large-scale strategies.
  - **Specialist Agents:** Highly-focused experts (e.g., SEO Analyst, UI Critic, Finance Analyst) attached to Prime Agents.
  - **Subagents:** Temporary executors spawned for single tasks.
* **Agent Mode & Project Profiles:** Tailor GOAT to your workflow. Are you debugging Rust, refactoring React, or writing a PRD? GOAT automatically detects your project stack and switches to the optimal agent profile.
* **Memory Galaxy & Brain Search:** A persistent SQLite-backed vector search system stores your code, conversations, and learned skills, acting as a long-term memory system.
* **Extensive Integrations:**
  - **Browser QA:** Automate browser testing via Playwright/Puppeteer adapters.
  - **GitHub Workflows:** Native integration for PR reviews, issue management, and CI interactions.
  - **Transports & Voice:** Command your agent via Discord, Telegram, or Voice (TTS/STT).
  - **MCP & Tools:** A robust Tool Registry that supports arbitrary shell commands, Model Context Protocol (MCP) servers, and custom recipes.

## 📸 Screenshots

*Currently gathering screenshots for the Alpha release. Placeholder examples:*

* `[TUI Interface Placeholder]`
* `[Dashboard AI Studio Placeholder]`
* `[Memory Galaxy Visualization Placeholder]`

*(See [docs/assets/screenshots](docs/assets/screenshots/) for all UI references once available).*

## 🚀 Quick Start

### Prerequisites
* Rust (stable)
* Node.js (for the Dashboard)
* `pnpm` or `npm`

### Install from Source
```bash
git clone https://github.com/ziuus/GOAT.git
cd GOAT
cargo build --release
```

### 1. Setup Wizard
Run the interactive onboarding to configure paths, database, and LLM providers:
```bash
cargo run --release -- setup
```
Alternatively, open the TUI and type `/onboard`.

### 2. Verify Health
Check if GOAT is configured correctly:
```bash
cargo run --release -- doctor
```

### 3. Run the Interfaces

**Run the TUI (Terminal User Interface):**
```bash
cargo run --release -- tui
```

**Run the Daemon (Background API Server):**
```bash
cargo run --release -- daemon start
```

**Run the Web Dashboard (requires Daemon to be running):**
```bash
cd apps/dashboard
npm install
npm run dev
```
*(The dashboard runs on `http://localhost:3000`)*

## 🛡️ Security Model

GOAT is designed to be secure on your local machine:
1. **ApprovalGate:** Every file modification or system command proposed by the AI must be explicitly approved by you (unless you bypass it with `--auto-approve`, which is highly discouraged).
2. **Local Memory:** Your project data and conversations remain on your local SQLite database (`goat_brain.db`).
3. **Opt-In Cloud:** External connections to browsers, cloud APIs, and transports are disabled by default.

For full details, read our [Security Documentation](docs/SECURITY.md).

## 🗺️ Roadmap & Current Limitations

We are currently wrapping up Phase 5 (Production Hardening & Multi-Agent Topologies). 
* See [docs/GOAT_IMPLEMENTATION_ROADMAP.md](docs/GOAT_IMPLEMENTATION_ROADMAP.md) for the detailed phase breakdown.
* See [docs/GOAT_FEATURE_MATRIX.md](docs/GOAT_FEATURE_MATRIX.md) for an honest assessment of what works, what is partial, and what is experimental.

## 🤝 Contributing

We welcome early testers and contributors! 
Please review our [Contributing Guidelines](docs/CONTRIBUTING.md) and [Code of Conduct](docs/CODE_OF_CONDUCT.md).

## 📄 License

MIT License. See `LICENSE` for details.
