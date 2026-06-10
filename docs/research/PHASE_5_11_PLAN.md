# Phase 5.11: GitHub Workflow Plan

## 1. Inspect Current Systems
- **Git helpers / Commit workflow**: Existing GOAT uses basic system commands for git via `src/repo_map.rs` or shell exec.
- **Timeline**: Existing `src/timeline.rs` tracks events. We'll add GitHub-specific events.
- **ApprovalGate**: Controls operations. Will be required before creating a branch, pushing, or creating a PR.
- **Dashboard / Quick Access**: Can easily route `\gh` to `/github` handlers.

## 2. GitHub CLI Check
The `GitHubWorkflowManager` will detect `gh` via `which gh`. If not found, it degrades gracefully to standard `git` where possible or requests the user install `gh` for PR and Issue functions. We *do not* require it strictly, nor store GitHub tokens directly.

## 3. GitHub Workflow Architecture
`src/github_workflow.rs` defines `GitHubWorkflowManager`. It stores current linked issues, branch plans, and drafts in memory (and syncs to session state if needed). 
- `GitHubIssueRef`
- `GitHubBranchPlan`
- `GitHubPrDraft`
We will expose operations like `link_issue()`, `plan_branch()`, `create_branch()`, `draft_pr()`, `push_and_pr()`.

## 4. Auth / Token Safety Model
- **No tokens stored**: GOAT relies on the system's `gh auth status` and standard `git` credentials.
- Commands previewed: `gh pr create` will be shown via ApprovalGate *before* execution.
- Redaction: We will not log payloads containing token-like strings in the timeline or console.

## 5. Issue → Branch → PR Flow
1. User calls `/github issue link 42`. GOAT queries issue 42.
2. User calls `/github branch plan`. GOAT suggests `goat/42-fix-login`.
3. User calls `/github branch create`. GOAT creates branch.
4. User works, creates commits.
5. User calls `/github pr draft`. GOAT summarizes work into a draft.
6. User calls `/github pr create`. GOAT sends `gh pr create` via ApprovalGate.

## 6. ApprovalGate Points
- `github_branch_create`: Required if the worktree is dirty to avoid trashing state.
- `github_push`: Required before running `git push`.
- `github_pr_create`: Required before running `gh pr create`.

## 7. Dashboard GitHub UI Design
- Path: `/github`
- Will display current repo status (branch, dirty/clean).
- Has an interactive panel to input an Issue URL/ID to link.
- Shows planned branch names and generated PR drafts in a preview box.
- Contains "Request Push/PR" buttons which trigger the ApprovalGate flow (visible as overlay).

## 8. Implemented vs Partial
- **Implemented**: Issue linking (mock/partial fetching), branch planning, branch creation (local git), PR drafting (metadata aggregation), ApprovalGate integration, UI, Quick Access prefixes.
- **Partial**: Real `gh` CLI invocation may be stubbed or partial based on environment constraints (to prevent breaking if `gh` is unauthenticated). Brain semantic ingestion of PR drafts will be partial.
