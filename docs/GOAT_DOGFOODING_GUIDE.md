# GOAT Dogfooding Guide

This guide outlines 5 core workflows to test GOAT's usability end-to-end. Use these to verify the dashboard and backend are functional before a release.

## 1. Student Workflow
**Goal:** Create a structured learning path for a new topic.
1. Open Dashboard > **Learner**.
2. Click **New Track**.
3. Enter "Master Rust Concurrency" and select the "Rust Programming" domain.
4. Click **Start Journey**.
5. Verify the track appears in the active list and opens the Learner OS shell.
6. Check that the roadmap tree renders correctly.

## 2. Founder Workflow
**Goal:** Validate a new startup idea.
1. Open Dashboard > **Cofounder**.
2. Click the **+** (New Idea) button.
3. Enter "AI-Powered Code Reviewer" and a short description.
4. Save the idea.
5. Verify the idea appears in the sidebar list.
6. Verify the "Run Validation" button shows a safe "Coming Soon" or disabled state.

## 3. Prompt Workflow
**Goal:** Refine a rough prompt into a structured agent instruction.
1. Open Dashboard > **PromptForge**.
2. Type a messy prompt: "Make me a website with react and tailwind that looks cool."
3. Click **Improve Prompt**.
4. Verify the refined prompt appears in the output window.
5. Verify the history list updates (or safely falls back to a mock/empty state).

## 4. Research Workflow
**Goal:** Test the experimental researcher UI.
1. Open Dashboard > **Researcher**.
2. Verify the page loads cleanly with the "Experimental" and "Empty State" UI.
3. Verify no cryptic errors are thrown.

## 5. Operator Workflow
**Goal:** Test the experimental operator UI.
1. Open Dashboard > **Operator**.
2. Verify the page loads cleanly with the "Experimental" and "Empty State" UI.
3. Verify the "ApprovalGate Protected" badge is visible and reassuring.
