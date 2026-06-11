# GOAT Public Alpha Demo Script

This script outlines a 3–5 minute recorded or live demo flow to introduce GOAT's core capabilities in the Alpha release.

## Prep
* Run `cargo run --release -- daemon start`.
* Run `npm run dev` in `apps/dashboard`.
* Open `http://localhost:3000`.
* Ensure Demo Data is loaded via Settings.

## Flow

**1. Open Dashboard (0:00 - 0:30)**
* "Welcome to GOAT, a local-first AI Agent OS."
* Show the Home Dashboard. Point out the clear visual separation of tasks.
* Highlight the `Local Daemon` connection status widget.

**2. Local-First Safety & ApprovalGate (0:30 - 1:00)**
* Open the **Sidebar Safety** or **Approvals** area.
* "GOAT runs entirely locally. It features an `ApprovalGate` that intercepts destructive terminal commands or API calls before agents execute them."

**3. Learner OS Roadmap (1:00 - 2:00)**
* Navigate to **Learner**.
* "This isn't just for coding. For example, if you want to learn System Design, the Learner agent breaks down a multi-day roadmap and generates practice questions locally."
* Show the active track and levels.

**4. Cofounder Idea Validation (2:00 - 2:45)**
* Navigate to **Cofounder**.
* "If you're building a product, the Cofounder agent reviews your idea, scopes an MVP, and provides an actionable scorecard."
* Show the structured Idea Validation form and Scorecard.

**5. PromptForge Refinement (2:45 - 3:30)**
* Navigate to **PromptForge**.
* "Vague prompts produce bad results. PromptForge takes a rough draft and compiles it into a strict, structured agent instruction."
* Type a rough prompt and click **Improve Prompt**. Show the output.

**6. Reports & Timeline (3:30 - 4:00)**
* Navigate to **Reports** and then **Timeline**.
* "All agent activities, decisions, and handoffs are recorded in the Timeline. You can export structured reports anytime."

**7. Experimental Agents (4:00 - 4:30)**
* Navigate to **Agents** Command Center.
* Honestly point out the `Experimental` labels on Socializer, Designer, and Operator.
* "We are actively building out more Prime Agents, but we want to be honest about what is fully wired up today."

**8. Conclusion & Contribution (4:30 - 5:00)**
* "GOAT is open-source and in active Alpha."
* Point viewers to the GitHub repo.
* "We need your feedback and contributions. Check out the issues, spin up the local daemon, and let's build the ultimate local Agent OS."
