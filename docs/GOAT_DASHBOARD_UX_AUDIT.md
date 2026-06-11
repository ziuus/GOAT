# GOAT Dashboard UX Audit (Phase 5.26 Update)

**Date:** June 2026

## Goal
Audit the existing dashboard routes and propose improvements for a modern, clean, premium "Agent OS" feel that serves builders, students, founders, creators, researchers, and operators.

## 1. Navigation Problems
* **Flat Sidebar Hierarchy**: The sidebar has over 30 items without proper grouping. `navItems` in `Sidebar.tsx` mixes home, agents, system, and logs all in one flat list.
* **Missing "OS" Context**: The sidebar lacks a clear active connection state indicating the daemon's heartbeat or "ApprovalGate Protected" trust signals.

## 2. Home Page (`/`)
* **Weak Landing**: Currently an overview of metrics but lacks a clear "Agent OS" feeling. It does not instantly answer "What is GOAT?" or provide immediate quick actions for builders, founders, or learners.
* **Missing Workflows**: No clear paths like "Validate an Idea", "Plan Learning", or "Refine Prompt".

## 3. Prime Agents Pages (`/agents`, `/cofounder`, etc.)
* **Visual Clutter**: The agents page displays agents but looks like a generic feature list rather than a "Command Center".
* **Inconsistent Layouts**: Each agent page has a slightly different header or structure. The empty/loading/error states are missing or basic.
* **Developer Jargon**: Too many labels describe the technical backend implementation rather than the user intent.

## 4. PromptForge (`/promptforge`)
* **No Compiler Feel**: It lacks the robust feel of an "intelligent compiler." Needs better diff/comparison views, explicit warnings that it doesn't execute tasks, and a prominent mode selector.

## 5. UI Components & Style
* **Inconsistent Cards & Spacing**: Cards vary in padding and border styles.
* **Glassy Overload vs. Flat Design**: Needs a balanced "restrained glassmorphism" dark-first theme. Currently, some areas feel cramped.
* **Missing Design System**: No centralized components like `PageShell`, `EmptyState`, or `FeatureCard` in a shared location for all views.

## 6. Learner OS Integration
* **Preservation Needed**: The Learner OS has good UX patterns from Phase 5.25. These should be preserved and elevated into global reusable components.

## Proposed Design System Approach
1. Create `components/ui/` with primitives (`PageShell`, `SectionHeader`, `FeatureCard`, `StatusBadge`, `EmptyState`, `LoadingState`, `ErrorState`, `SafetyNotice`).
2. Restructure `Sidebar.tsx` into categorized groups (`Intelligence`, `Workflows`, `System`).
3. Refactor the `page.tsx` routes to use `PageShell`.
