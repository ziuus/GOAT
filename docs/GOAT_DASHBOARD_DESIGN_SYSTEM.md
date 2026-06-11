# GOAT Dashboard Design System

## Core Primitives
The dashboard is built on React 19, Next.js 15, and Tailwind CSS. The design system lives in `apps/dashboard/src/components/ui`.

## Components
1. **`PageShell`**: Wraps every page with consistent max-width, padding, and an optional header section.
2. **`PageHeader`**: Standardized header with a title, subtitle, and primary actions.
3. **`SectionHeader`**: Used for grouping content within a page.
4. **`FeatureCard`**: A flexible card for agent status, workflows, and metrics.
5. **`AgentCard`**: Specifically tailored for the Prime Agents overview, detailing domain, status, and quick actions.
6. **`StatusBadge`**: Small pill-shaped indicators for online/offline/idle/running states.
7. **`SafetyNotice`**: A high-visibility banner indicating ApprovalGate protection or disabled integrations.
8. **`EmptyState` / `LoadingState` / `ErrorState`**: Centralized placeholders to maintain visual consistency.
9. **`CommandHint`**: Small inline text showing CLI equivalents for power users.

## Tailwind Constants
- **Backgrounds**: Dark-first `#0A0A0A` for body, `bg-white/[0.02]` for cards.
- **Borders**: Soft `border-white/5` to `border-white/10`.
- **Accents**: Indigo (`indigo-500`) for primary actions, Amber for warnings, Emerald for success.
- **Glass**: Restrained glassmorphism. Use `backdrop-blur-sm` sparingly.

## File Locations
- `apps/dashboard/src/components/ui/*.tsx`
- `apps/dashboard/src/app/globals.css`
