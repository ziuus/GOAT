# GOAT Branding Strategy

This document outlines the official brand identity and logo usage strategy for GOAT (The Local-first Agent OS).

## Core Asset

* **File**: `namelogo.png` (located in the project root).
* **Usage**: This is the single source of truth for the graphical wordmark.
* **Modification**: Do not rename, generate a new logo, or require font files. All graphical UI elements must reference this file or its copies.

## Dashboard Branding

* **Asset**: `namelogo.png` (copied to `apps/dashboard/public/namelogo.png`).
* **Implementation**: The dashboard uses the image in the Sidebar header, the Home page hero/header, and the About/Settings area.
* **Fallbacks**: Text-based `GOAT` fallbacks are preserved where an image might fail to load. The dashboard maintains its dark-first style.

## TUI (Terminal UI) Branding

* **Strategy**: Pure ASCII/Unicode fallback. Terminal image protocols (like Sixel or Kitty image protocol) are not yet strictly required to avoid breaking compatibility with basic terminals.
* **Startup**: A multi-line ASCII logo (`goat_ascii_logo()`) is pushed to the chat log upon startup.
* **Header**: The TUI status bar uses a compact text wordmark (`🐐 GOAT`) alongside the subtitle `Local-first Agent OS`.

## CLI Strategy

* **Strategy**: Minimal, textual branding.
* **Commands**: Standard CLI help output (`goat --help`) starts with `GOAT — Local-first Agent OS` to reinforce the brand identity without printing huge banners that break standard CLI expectations.
