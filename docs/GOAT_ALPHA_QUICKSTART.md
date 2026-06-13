# GOAT Alpha 1 Quickstart

Welcome to GOAT (General Objective Agentic Task-engine). This guide is tailored for users participating in the Alpha 1 phase who may be migrating from other AI coding assistants (like Claude Code, Aider, or Cline).

GOAT emphasizes **safety, transparency, and explicit permission** over autonomous guesswork. Every action is audited and, by default, verified via the `ApprovalGate`.

## 1. Install & Verify

If you are building from source:
```bash
cargo build --release --bin goat
# Or place it in your path:
cargo install --path .
```

Verify your installation:
```bash
goat --version
goat doctor alpha
```

## 2. Learn a Project

GOAT relies on an intelligent repository map instead of just sending all files to an LLM. When entering a new workspace:
```bash
cd /your/project
goat learn
```
This parses the AST and builds a local-first graph of your code structure.

## 3. Create a Mission

Missions track ongoing objectives and help subagents stay focused.
```bash
goat mission plan "Refactor the database adapter to use connection pooling"
```

## 4. Launching the TUI (Optional but Recommended)

You can use GOAT entirely via single CLI commands, or drop into the interactive Terminal User Interface:
```bash
goat
```
Inside the TUI, type `/tools` to see the dashboard, or `/help` to see available slash commands.

## 5. Propose and Apply Edits

GOAT does not write to your files autonomously behind your back.
```bash
# Propose a change
goat patch propose --mission <mission-id>

# Review patches
goat patch list
goat diff analyze patch-<id>

# Apply the patch if safe
goat patch apply patch-<id>
```

## 6. Run Validations and Skills

You can safely run recipes configured for your environment:
```bash
goat validate --recipe cargo-test
```
For full reusable skill workflows from extensions:
```bash
goat tools prepare ext-skill-format
goat tools invoke ext-skill-format
```

## Migration Notes

* **No direct shell execution:** If you type `npm install` in the chat, GOAT won't blindly run it unless it passes `ApprovalGate` or is triggered through a validated Capability.
* **Fail-Closed:** When in doubt, GOAT halts and asks for approval.
* **Persistent Memory:** GOAT remembers project architecture decisions across sessions. You can query it via `goat memory search "how does auth work"`.

Enjoy building with GOAT!
