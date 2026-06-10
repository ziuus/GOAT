# GOAT Dashboard UX & Theme System

## Design System

The GOAT dashboard utilizes a custom design system built with Next.js, React, and Tailwind CSS. The design philosophy emphasizes a **dark premium developer vibe**, similar to tools like Linear and Raycast.

### Core Primitives
We have implemented several reusable UI components in `src/components/ui/`:
- **Card**: For structured content containers with subtle borders and backdrop blurs.
- **Button**: A versatile button component supporting multiple variants (`default`, `destructive`, `outline`, `secondary`, `ghost`).
- **Badge**: For status indicators and tags.
- **Input**: Clean, accessible text inputs.
- **Skeleton**: Loading state placeholders.
- **EmptyState**: Standardized empty states with icons for a better user experience.
- **StatusDot**: Animated pulsing dots to indicate live connection statuses.

## Theme System

The dashboard supports a robust CSS-variable based theme system. Currently supported themes:

1. **GOAT Dark (Default)**: A rich, deep dark theme with subtle blue/purple tints.
2. **Minimal Dark**: A pure black (`#000000`) theme with high contrast borders.
3. **High Contrast**: Designed for accessibility with distinct borders and bright text.

Themes are toggled via a `<select>` dropdown on the Settings page. The active theme is persisted in `localStorage` (`goat-theme`) and applied to the `<html>` element as a CSS class (`theme-goat-dark`, etc.).

## Command Palette
A global command palette is available via `Ctrl+K` or `Cmd+K`. Built using `cmdk`, it provides rapid keyboard-centric navigation across the dashboard.

## Editor and Diff Viewer
Due to offline environment restrictions, `@monaco-editor/react` is partially supported as a planned enhancement. Currently, the dashboard utilizes a highly polished **Graceful Custom Viewer**:
- **Repo Explorer**: Features file line numbers, syntax-like text coloring, and safe read-only rendering.
- **Diff Viewer**: Displays `git diff` outputs with inline Git coloring (green for additions, red for deletions, purple for hunks) and truncation notices for large outputs.
