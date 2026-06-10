# GOAT Setup and Troubleshooting

This document outlines the setup process for GOAT and common troubleshooting steps.

## Quickstart

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/ziuus/GOAT.git
   cd GOAT
   ```

2. **Configure API Keys:**
   Create a `.env` file in the root directory and add your LLM API keys:
   ```env
   OPENAI_API_KEY=sk-...
   ANTHROPIC_API_KEY=sk-...
   ```

3. **Start the Daemon (Backend):**
   ```bash
   cargo run -- daemon
   ```
   *This starts the local API server and event bus on `127.0.0.1:47647`.*

4. **Start the Dashboard (Frontend):**
   ```bash
   cd apps/dashboard
   npm install
   npm run dev
   ```
   *Open `http://localhost:3000` in your browser. Copy the daemon token from `~/.local/share/goat/daemon.token` and paste it into the Dashboard Settings to authenticate.*

5. **Alternative: Use the TUI:**
   ```bash
   cargo run -- tui
   ```

## Common Issues & Troubleshooting

### 1. "Dashboard Cannot Connect to Daemon"
- **Cause:** The Daemon is not running, or the token is incorrect.
- **Fix:** Ensure `cargo run -- daemon` is actively running in a terminal. Check your token by running `cat ~/.local/share/goat/daemon.token`. Verify the URL in the Dashboard Settings is `http://127.0.0.1:47647`.

### 2. "Approval Required" but nothing happens
- **Cause:** You requested an `Act` mode operation that triggered the `ApprovalGate`.
- **Fix:** Open the Dashboard and navigate to the `/approvals` page, or check the TUI Approvals pane to explicitly approve or deny the action.

### 3. "API Key Missing" Error
- **Cause:** GOAT cannot find a valid LLM API key.
- **Fix:** Run the built-in diagnostic tool: `cargo run -- doctor`. This will check your `.env` file and system variables.

### 4. "npm install ETIMEDOUT"
- **Cause:** Network restrictions or proxy issues.
- **Fix:** Verify your network connectivity. GOAT is designed to fallback gracefully to custom viewers if heavy frontend dependencies like Monaco fail to download.
