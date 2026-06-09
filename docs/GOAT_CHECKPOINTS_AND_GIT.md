# Checkpoints and Git Safety

## Rollback Behavior
- **Safe by Default**: `/rollback <id>` defaults to showing a plan. It will NOT destructively overwrite your working directory without explicit confirmation.
- **Rollback Types**:
  - `/rollback plan <id>`: Shows what files would be modified by a rollback.
  - `/rollback restore <id>`: (Planned) Attempts to safely restore files from backup.
  - `/rollback destructive <id>`: Requires ApprovalGate to run `git reset --hard && git clean -fd`.

## Commit Behavior
- **Message Generation**: `/commit message` generates a deterministic summary of changes based on `git status` and `git diff`. (AI generation planned).
- **Safety checks**: `/commit create` checks for secret-like files (e.g. `.env`, `id_rsa`) and blocks commit staging unless explicitly overridden.

## Auto-checkpoints
- When enabled in config, GOAT captures a snapshot (checkpoint) before applying patch diffs or dangerous bash commands.

## Limitations
- Large binary files and secret files are intentionally excluded from checkpoint snapshots for security and size considerations.
