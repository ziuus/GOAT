import re

def update_file(path, search, replace):
    with open(path, 'r') as f:
        content = f.read()
    if search in content:
        content = content.replace(search, replace)
        with open(path, 'w') as f:
            f.write(content)
        print(f"Updated {path}")
    else:
        print(f"Search string not found in {path}")

# CHANGELOG.md
changelog = """## [0.12.0] — Phase 3.4 & 3.5: Checkpoints, Git Branch/Commit, and Safety Hardening (2026-06-09)

### Added
- **Checkpoint System**: `/checkpoint create`, `/checkpoint list`, `/checkpoint show`, `/checkpoint diff`.
- **Auto-checkpoints**: Automatically create a safety checkpoint before applying patches or using write_file (if enabled in config).
- **Rollback System (Safe)**: `/rollback <id>` defaults to safe plan mode. `/rollback destructive <id>` securely restores the workspace to a previous checkpoint via ApprovalGate.
- **Git Branch Management**: `/branch current`, `/branch create <name>` (requires ApprovalGate).
- **Commit Preparation**: `/commit message` dynamically parses `git status` and `git diff` for a deterministic message. `/commit create` performs safety checks and blocks secret-like files from being staged automatically (requires ApprovalGate).
- **Status Updates**: `/status` and `/changes` display current branch, dirty state, and checkpoint hints.

---

## [0.11.0]"""

update_file("CHANGELOG.md", "## [0.12.0] — Phase 3.4: Checkpoint / Rollback + Git Branch / Commit Workflow (2026-06-09)\n\n### Added\n- **Checkpoint System**: `/checkpoint create`, `/checkpoint list`, `/checkpoint show`, `/checkpoint diff`\n- **Auto-checkpoints**: Automatically create a safety checkpoint before applying patches or using write_file (if enabled in config).\n- **Rollback System**: `/rollback <id>` securely restores the workspace to a previous checkpoint via ApprovalGate.\n- **Git Branch Management**: `/branch current`, `/branch create <name>` (requires ApprovalGate).\n- **Commit Preparation**: `/commit message`, `/commit create` (requires ApprovalGate).\n- **Status Updates**: `/status` and `/changes` now display current branch, dirty state, and checkpoint hints.\n\n---\n\n## [0.11.0]", changelog)

# README.md
update_file("README.md", "*   **Phase 3.5:** Context-Aware Multi-File Chat (Next)", "*   **Phase 3.5:** Git Safety Hardening & Commit Workflow Polish (Complete)\n*   **Phase 3.6:** Context-Aware Multi-File Chat (Next)")

