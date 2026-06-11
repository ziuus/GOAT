# Contributing to GOAT

Thank you for your interest in contributing to GOAT! We are in early alpha and welcome all forms of contribution: bug reports, feature requests, documentation improvements, and code.

## How to Contribute

1. **Fork the Repository:** Create your own fork and work on a feature branch.
2. **Setup your environment:** See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).
3. **Write Tests:** Ensure any new features include appropriate unit tests.
4. **Follow Formatting:** Run `cargo fmt` and `cargo check` before committing.
5. **Open a PR:** Use the provided Pull Request template to describe your changes.

## Architectural Guidelines

- **Do Not Bypass ApprovalGate:** Security is our top priority. Never bypass user consent for dangerous actions.
- **Local-First:** Features must work without external cloud dependencies where possible.
- **Modular Design:** Keep the core agent logic decoupled from specific UI implementations.

See [docs/README.md](docs/README.md) for more architectural resources.
