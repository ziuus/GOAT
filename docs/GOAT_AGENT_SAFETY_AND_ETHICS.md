# GOAT Agent Safety & Ethics

GOAT is designed as a powerful, local-first Agent OS. With that power comes the responsibility to ensure agents act predictably, safely, and ethically. 

## Architectural Safety

1. **ApprovalGate as the Bedrock**
   No agent—whether Prime, Specialist, or Subagent—can execute a dangerous action (file write, network request, command execution) without passing through the `ApprovalGate`. Agents operate with "least privilege" tool policies.

2. **Visibility & Inspection**
   There are no hidden background agents. All agent activities are logged to the Timeline, and their session histories are viewable in the Dashboard.

3. **Scoped Memory**
   Agents do not duplicate or maliciously scan global memory. They only have access to what they are explicitly provided via the `AgentMemoryScope`.

## Ethical Principles by Domain

### 1. Social & Distribution (Socializer)
- **No Spam Automation:** GOAT will not implement autonomous spam bots.
- **Draft-Only Outreach:** The Socializer Agent must generate *drafts* for outreach. It will not autonomously send cold emails or mass-DM users.
- **Platform Respect:** Output is optimized for high-signal value, respecting the etiquette of communities like Reddit and Hacker News.

### 2. Business & Strategy (Cofounder)
- **Evidence-First:** The Cofounder Agent is explicitly instructed to demand evidence for assumptions. It pushes back on hype-driven feature bloat.
- **Honest Feedback:** It acts as an objective mirror, not a sycophant.

### 3. Future Legal & Admin Agents
- **No Unlicensed Advice:** Legal or Admin agents must prominently display disclaimers that they are not lawyers. They are tools for formatting and analyzing text, not providing binding legal counsel.

### 4. Future Finance Agents
- **No Fiduciary Role:** Finance Analyst specialists model data based on user inputs. They do not provide certified financial advice or execute trades.

### 5. Security Agents
- **No Aggressive Scanning:** Security Reviewers and Bug Hunters are restricted to local static analysis or user-owned local dev servers. They will not launch aggressive payloads against external domains.

## Enforcement
These rules are enforced via:
- Hardcoded `AgentManifest` constraints.
- System prompts defining the agent's persona.
- The `ApprovalGate` intercepting the actual tool execution.
