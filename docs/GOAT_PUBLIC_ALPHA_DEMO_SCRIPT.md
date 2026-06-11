# GOAT Public Alpha Demo Script

This script outlines a 3–5 minute recorded demo flow to introduce GOAT's core capabilities in the Alpha release.

## ⚠️ Pre-Flight Setup
1. **Clear Logs:** `rm -rf logs/*` to start fresh.
2. **Start Daemon:** `cargo run --release -- daemon start`
3. **Start Dashboard:** `cd apps/dashboard && npm run dev`
4. **Open Browser:** Navigate to `http://localhost:3000`. Full screen. Dark mode only.
5. **Load Demo Data:** Go to Settings -> "Load Demo Data".

## ❌ What NOT to show
* Do not attempt to click "Run Validation" in Cofounder (it's marked Coming Soon).
* Do not try to open the Designer or Socializer "New Task" screens (disabled).
* Do not open the "Agents" chat interface (backend collaboration isn't fully wired to the UI yet).

---

## 🎬 The Flow

### 1. Intro & Architecture (0:00 - 0:45)
* **Action:** Show the Home Dashboard. Point out the clear visual separation of tasks. Scroll down to show the quick actions.
* **Script:** "Welcome to the first alpha of GOAT, a local-first AI Agent OS. Unlike typical chat interfaces, GOAT is an orchestration system. Notice the 'Local Daemon' widget—the backend is entirely local Rust, powering this Next.js dashboard."

### 2. Local-First Safety & ApprovalGate (0:45 - 1:15)
* **Action:** Click on the **Sidebar Safety** or **Approvals** area. 
* **Script:** "Security is critical when giving agents tools. GOAT features an `ApprovalGate`. If an agent tries to run a bash script or format your drive, the daemon halts execution and forces you to explicitly approve it here."

### 3. Learner OS Roadmap (1:15 - 2:00)
* **Action:** Navigate to **Learner**. Click on the "DSA Masterclass" active track.
* **Script:** "We're building Prime Agents for specific domains. The Learner agent isn't just a tutor; it acts as an OS. It breaks down complex topics into multi-day roadmaps, tracks your level, and generates localized practice modules."

### 4. Cofounder Idea Validation (2:00 - 2:45)
* **Action:** Navigate to **Cofounder**. Show the Scorecard and structured form.
* **Script:** "If you're building a product, the Cofounder agent reviews your idea, scopes an MVP, and provides an actionable scorecard. Notice how the UI is structured data, not just a wall of generated text."

### 5. PromptForge Refinement (2:45 - 3:30)
* **Action:** Navigate to **PromptForge**. Type a vague prompt (e.g., "build me a weather app").
* **Script:** "Agents need rigorous instructions. PromptForge takes your rough idea and compiles it into a strict, structured agent instruction. You can see the compiled output here."

### 6. Transparency & Timeline (3:30 - 4:00)
* **Action:** Navigate to **Reports** and then **Timeline**.
* **Script:** "We don't want hidden agent swarms. Every action, handoff, and decision is recorded chronologically in the Timeline, stored locally in SQLite."

### 7. Honesty & Conclusion (4:00 - 5:00)
* **Action:** Navigate to **Agents** Command Center. Hover over the "Experimental" tags.
* **Script:** "We are in active Alpha. Agents like Socializer and Operator are structurally planned but explicitly marked experimental today. We're open source, and we need your help to build this local-first future. Check out the GitHub repo, read the `CONTRIBUTING.md`, and let's get building."
