# GOAT Installation Guide

GOAT is currently in Alpha. The primary method of installation is building from source. Future releases will include pre-compiled binaries for major operating systems.

## System Requirements

To install and run GOAT, you need:
- **Rust Toolchain:** Version 1.70.0 or later (installed via [rustup](https://rustup.rs/)).
- **Node.js:** Version 18 or later (for the web dashboard and desktop app).
- **Package Manager:** `npm` or `pnpm`.
- *(Optional)* **GitHub CLI (`gh`):** Required for GitHub Workflow integrations.
- *(Optional)* **Playwright / Puppeteer:** Required if using Browser Automation capabilities.
- *(Optional)* **Local LLMs (e.g. Ollama):** For full local, privacy-first AI generation.

## Building from Source

For detailed instructions on compiling GOAT from source, please refer to the [BUILD_FROM_SOURCE.md](BUILD_FROM_SOURCE.md) document.

## Initial Setup

After installation, configure GOAT by running:

```bash
goat setup
```
*(If you built from source, use `cargo run --release -- setup`)*

This interactive wizard will help you configure:
1. Agent Profiles and Workspaces
2. AI Provider Settings (OpenAI, Anthropic, Ollama, etc.)
3. Memory Galaxy Indexing preferences

## Health Check

To verify your installation is healthy and all components are functioning, run:

```bash
goat doctor
```

This will report the status of:
* Config, Data, and Cache paths
* LLM provider connections
* Daemon and Dashboard services
* External tool availability (like `gh`)
