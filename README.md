<div align="center">
  <img src="./namelogo.png" alt="GOAT logo" width="420" />
  <h1>GOAT (v0.1.0-alpha.1)</h1>
  <p><strong>The fail-closed, local-first AI Agent OS for building, learning, and safe workflows.</strong></p>

  GOAT is an open-source AI agent system designed for developers who want powerful assistance without surrendering their terminal. Unlike simple CLI wrappers that blindly execute arbitrary shell scripts, GOAT insists on explicit permission, structural understanding, and safety.
  
  <p>
    <a href="#quick-start">Quick Start</a> •
    <a href="#who-is-goat-for">Audiences</a> •
    <a href="#what-goat-can-do">Features</a> •
    <a href="#migration--comparison">Migration</a> •
    <a href="#safety-model">Security</a> •
    <a href="docs/README.md">Docs</a>
  </p>

  ![Alpha](https://img.shields.io/badge/Status-Alpha_1-orange)
  ![Local-first](https://img.shields.io/badge/Architecture-Local--first-blue)
  ![Rust](https://img.shields.io/badge/Backend-Rust-red)
  ![ApprovalGate](https://img.shields.io/badge/Security-ApprovalGate-brightgreen)
</div>

---

## 🚀 Quick Start (Golden Path)

To get started with GOAT Alpha 1:

```bash
# 1. Build the release binary
cargo build --release --bin goat

# 2. Check alpha readiness
./target/release/goat doctor alpha

# 3. View the quickstart guide
./target/release/goat quickstart

# 4. Learn your current project (builds AST-aware repo map)
./target/release/goat learn .

# 5. Plan a new mission
./target/release/goat mission new

# 6. View available capabilities & MCP tools
./target/release/goat tools list
```

*(Alternatively, use `bash scripts/install-local.sh` to install to `~/.local/bin/goat`)*

---

## 👥 Who is GOAT for?

GOAT is for:
* **Developers migrating from Aider/Claude Code** who want more control over when and how code changes are applied.
* **Engineers** who need their agent to understand project architecture (`goat learn`) before writing code.
* **Power users** who want to define repeatable, deterministic `Skills` instead of crossing their fingers on autonomous loops.
* **Security-conscious teams** who demand a strict `ApprovalGate` and fail-closed execution.

---

## 🌟 Core Features (What Works Today)

1. **Local-First Brain:** SQLite-backed memory preserves missions, checkpoints, and context across sessions.
2. **Project Intelligence (`repo-map`):** AST-aware project mapping to feed agents accurate context.
3. **Safe Patch Flow:** Agents propose patches which are validated before applying, easily reversible via checkpoints.
4. **ApprovalGate:** Strict blocking of unexpected terminal execution.
5. **Validation Runner:** Native integration for running test suites (`goat validate`).
6. **Capability Extensions:** Safely load predefined workflows, MCP metadata, and tools without silent background execution.
7. **Terminal UI (TUI):** A rich, widget-based dashboard accessible directly in your terminal.

---

## ⚖️ Migration & Comparison

If you're coming from other tools, here is how GOAT differs:

* **vs. Claude Code / Codex CLI:** GOAT interrupts you more often for approval. It does not blindly run bash loops.
* **vs. Aider:** GOAT doesn't auto-commit. Changes are staged as patches (`goat patch propose`) and applied via `goat patch apply`.
* **vs. Cline / Cursor:** GOAT is CLI-first right now. It does not actively read unsaved IDE tabs in real-time.

Read the full [Migration Parity Audit](docs/GOAT_MIGRATION_PARITY.md) for a detailed breakdown.

---

## 🔒 Safety Model (Fail-Closed)

* **ApprovalGate:** Blocks system/terminal actions until explicitly reviewed.
* **No hidden autonomous swarms:** All multi-agent workflows are fully visible and require explicit steps.
* **Validation requires manual approval:** Shell execution for test suites or build commands will not auto-approve without explicit flags.
* **Opt-in Extensibility:** MCP servers and third-party tools are declarative capabilities. They don't autostart uninvited.

---

## 🚧 What is NOT Implemented Yet (Alpha Limitations)

* **Native IDE Extensions:** Full VS Code / IntelliJ integration is on the roadmap but missing today.
* **Approval Profiles:** Running complex builds can trigger multiple approval prompts. Future versions will introduce tiered profiles to reduce approval fatigue.
* **Advanced Capability Scheduling:** Cron-like autonomous capability execution is planned for post-alpha phases.

---

## 📚 Important Docs

* [Alpha Quickstart](docs/GOAT_ALPHA_QUICKSTART.md)
* [Alpha Blockers & Roadmap](docs/GOAT_ALPHA_BLOCKERS.md)
* [Migration Parity Audit](docs/GOAT_MIGRATION_PARITY.md)
* [Feature Matrix](docs/GOAT_FEATURE_MATRIX.md)
* [Security Model](docs/GOAT_SECURITY_MODEL.md)

---

## 🤝 Contributing

GOAT is in active Alpha. We welcome bug reports, documentation fixes, and focused integrations. Please open an issue before starting large feature work!

Review our [Contributing Guidelines](docs/CONTRIBUTING.md).

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
