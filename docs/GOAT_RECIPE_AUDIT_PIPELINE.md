# GOAT Recipe Audit Pipeline

Every recipe goes through an audit process before installation.

## Heuristics
1. `rm -rf`, `sudo` -> Critical (Destructive / Privileged)
2. `curl | sh` -> High (Remote execution)
3. `git push --force` -> High (Destructive)

## Output
- `risk_level`: Low, Medium, High, Critical
- `warnings`: Array of specific findings
- `recommended_action`: `safe_to_install` or `review_required`
- `required_approvals`: Suggested policy (e.g., `admin`).

If a recipe hits High/Critical, it is marked as `review_required`.
