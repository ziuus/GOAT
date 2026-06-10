# GOAT Automation Recipe Marketplace

The GOAT Recipe Marketplace allows users to discover, audit, install, and enable automation recipes.
Recipes encapsulate workflows, hooks, schedules, and job definitions into reusable units.

## Key Principles
1. **Never execute remote/generated recipes directly:** Recipes must be installed to local storage first.
2. **Default Disabled:** Installing a recipe only creates its local footprint. It remains disabled by default.
3. **Approval Required:** Enabling an action-based recipe requires going through the `ApprovalGate`.
4. **No Cloud Sync:** All state is local.

## Features
- **Built-In Catalog**: Safe, predefined recipes like `cargo-check-on-save`.
- **Audit Pipeline**: Static analysis of shell strings and operations to gauge Risk Level.
- **AI Studio Integration**: Convert workflow drafts into recipes.
- **Memory Galaxy Integration**: Convert learned workflow candidates into drafts or find matches.

## API Endpoints
All available under `/v1/recipes/`.
