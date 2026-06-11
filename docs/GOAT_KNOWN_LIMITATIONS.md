# GOAT Known Limitations

Welcome to the GOAT Alpha! Please read this before filing bug reports, as some features are intentionally partial at this stage.

## 1. Active Alpha Status
GOAT is not a production autonomous agent. It is a local-first system under rapid development. Things will break, and APIs will change without warning.

## 2. Dashboard UI vs Backend
Some dashboard pages (like Learner and Cofounder) have robust, polished user interfaces but are currently relying on mocked or unstructured data on the backend. True SQLite persistence for these specific views is planned for Phase 6.1.

## 3. Experimental Agents
The **Socializer**, **Designer**, and **Operator** agents have UI placeholders but are disabled. They are not fully wired to the local Daemon yet.

## 4. Vector Embeddings
While `Memory Galaxy` exists, real semantic vector search embeddings are partial. We are actively working on finalizing the local embedding generation pipeline.

## 5. Agent Collaboration Layer
The backend Rust logic for multi-agent collaboration (handoffs, structured chat) is implemented, but the Dashboard Web UI does not fully expose this yet.

## 6. External Integrations
External cloud provider integrations (like OpenAI or Anthropic via LiteLLM/direct APIs) are strictly opt-in and experimental. They are disabled by default to enforce our local-first security stance.

## 7. Screenshots and Documentation
Some documentation links might 404, and screenshots may still be actively rendering for the final repository push.

Thank you for testing GOAT! Review our `GOAT_FEATURE_MATRIX.md` for a comprehensive view of what works.
