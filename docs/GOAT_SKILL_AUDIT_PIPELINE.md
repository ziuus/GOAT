# GOAT Skill Audit Pipeline

Every skill fetched from the marketplace must undergo auditing before installation.

## Audit Checks
- Suspicious shell commands (e.g. `rm -rf`, `sudo`)
- Arbitrary code execution patterns (e.g. `curl | sh`, `wget -qO-`)
- Secret strings and exfiltration targets
- Unexpected `git` operations or tool permissions

## Audit Output
The `SkillAuditReport` returns:
- **Risk Level**: `low`, `medium`, `high`, `critical`.
- **Warnings**: Extracted suspicious lines/patterns.
- **Action**: `safe_to_install` or `review_required`. High/Critical risks automatically default to `review_required` and will pause execution until explicit user consent via `ApprovalGate` is granted.
