# Good First Issues for Alpha 1

This document lists the immediate, scoped tasks perfect for first-time contributors post public Alpha launch. 

---

### 1. Add Screenshots
* **Description:** Run the demo script locally, capture the 9 required screenshots from `docs/assets/screenshots/README.md`, and open a PR adding them to the repository.
* **Difficulty:** Easy
* **Area:** Documentation

### 2. Improve Dashboard Empty States
* **Description:** Some dashboard pages (like specific project views) still look slightly bare when no data exists. Add better placeholder graphics or "Get Started" call-to-actions.
* **Difficulty:** Easy
* **Area:** Frontend (Next.js)

### 3. Improve Learner Demo Data
* **Description:** Update the `Settings` page's "Load Demo Data" logic to populate more realistic, robust mock JSON into local storage for the Learner OS view.
* **Difficulty:** Easy
* **Area:** Frontend (Next.js)

### 4. Wire Cofounder Persistence
* **Description:** The Cofounder UI uses local state. Map the Cofounder form submissions to the Rust backend API to store ideas persistently in the SQLite `Memory Galaxy`.
* **Difficulty:** Medium
* **Area:** Fullstack (Next.js + Rust)

### 5. Add LiteLLM Provider Investigation
* **Description:** Investigate creating a flexible LLM provider trait in the Rust daemon that could optionally use LiteLLM as an abstraction layer, strictly behind an opt-in config flag.
* **Difficulty:** Hard
* **Area:** Backend (Rust)

### 6. Add More Smoke Tests
* **Description:** Expand the Rust unit tests and add bash-scripted smoke tests to automatically verify daemon startup and port bindings in CI.
* **Difficulty:** Medium
* **Area:** Testing

### 7. Improve Docs Navigation
* **Description:** The markdown files in `/docs` are growing. Add a simple static site generator (like mdBook) or just improve cross-linking between the MD files.
* **Difficulty:** Easy
* **Area:** Documentation

### 8. Add Dashboard E2E Tests
* **Description:** Introduce Playwright or Cypress to the Next.js app to automatically verify that the dashboard renders without crashing.
* **Difficulty:** Medium
* **Area:** Frontend / Testing

### 9. Improve Install Script
* **Description:** Write a simple `install.sh` bash script that checks for Rust and Node, runs the cargo builds, and sets up the daemon as a systemd/launchd service.
* **Difficulty:** Medium
* **Area:** DevOps / CLI
