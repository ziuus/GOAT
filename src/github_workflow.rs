use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubWorkflowConfig {
    pub enabled: bool,
    pub provider: String,
    pub default_remote: String,
    pub require_approval_for_push: bool,
    pub require_approval_for_pr: bool,
    pub require_clean_worktree_for_branch: bool,
    pub allow_github_cli: bool,
    pub allow_api_client: bool,
    pub token_env: String,
    pub default_base_branch: String,
    pub branch_prefix: String,
    pub pr_draft_by_default: bool,
}

impl Default for GitHubWorkflowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: "github".to_string(),
            default_remote: "origin".to_string(),
            require_approval_for_push: true,
            require_approval_for_pr: true,
            require_clean_worktree_for_branch: false,
            allow_github_cli: true,
            allow_api_client: false,
            token_env: "GITHUB_TOKEN".to_string(),
            default_base_branch: "main".to_string(),
            branch_prefix: "goat/".to_string(),
            pr_draft_by_default: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GitHubWorkflowState {
    Unlinked,
    IssueLinked,
    BranchPlanned,
    BranchCreated,
    ChangesDetected,
    CommitReady,
    PrDraftReady,
    PrApprovalRequired,
    PrCreated,
    ReviewReady,
    Completed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssueRef {
    pub number: String,
    pub title: String,
    pub url: String,
    pub body_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubBranchPlan {
    pub suggested_name: String,
    pub base_branch: String,
    pub issue_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPrDraft {
    pub title: String,
    pub body: String,
    pub base: String,
    pub head: String,
    pub is_draft: bool,
}

pub struct GitHubWorkflowManager {
    pub config: GitHubWorkflowConfig,
    pub state: GitHubWorkflowState,
    pub linked_issue: Option<GitHubIssueRef>,
    pub branch_plan: Option<GitHubBranchPlan>,
    pub pr_draft: Option<GitHubPrDraft>,
}

impl GitHubWorkflowManager {
    pub fn new(config: GitHubWorkflowConfig) -> Self {
        Self {
            config,
            state: GitHubWorkflowState::Unlinked,
            linked_issue: None,
            branch_plan: None,
            pr_draft: None,
        }
    }

    pub fn status(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "state": format!("{:?}", self.state),
            "linked_issue": self.linked_issue,
            "branch_plan": self.branch_plan,
            "pr_draft": self.pr_draft,
        }))
    }

    pub fn link_issue(&mut self, id: &str) -> Result<()> {
        self.linked_issue = Some(GitHubIssueRef {
            number: id.to_string(),
            title: format!("Resolve issue #{}", id),
            url: format!("https://github.com/org/repo/issues/{}", id),
            body_snippet: "Fetched issue body snippet...".to_string(),
        });
        self.state = GitHubWorkflowState::IssueLinked;
        Ok(())
    }

    pub fn unlink_issue(&mut self) -> Result<()> {
        self.linked_issue = None;
        self.state = GitHubWorkflowState::Unlinked;
        Ok(())
    }

    pub fn plan_branch(&mut self) -> Result<GitHubBranchPlan> {
        let name = if let Some(iss) = &self.linked_issue {
            format!("{}{}-{}", self.config.branch_prefix, iss.number, iss.title.replace(" ", "-").to_lowercase())
        } else {
            format!("{}feature-branch", self.config.branch_prefix)
        };
        
        let plan = GitHubBranchPlan {
            suggested_name: name,
            base_branch: self.config.default_base_branch.clone(),
            issue_ref: self.linked_issue.as_ref().map(|i| i.number.clone()),
        };
        self.branch_plan = Some(plan.clone());
        self.state = GitHubWorkflowState::BranchPlanned;
        Ok(plan)
    }

    pub fn draft_pr(&mut self) -> Result<GitHubPrDraft> {
        let draft = GitHubPrDraft {
            title: self.linked_issue.as_ref().map(|i| i.title.clone()).unwrap_or_else(|| "New PR".into()),
            body: "## Summary\nAuto-generated PR body from timeline and jobs.\n\n## Checklist\n- [ ] Tests pass\n- [ ] Docs updated".into(),
            base: self.config.default_base_branch.clone(),
            head: self.branch_plan.as_ref().map(|p| p.suggested_name.clone()).unwrap_or_else(|| "head-branch".into()),
            is_draft: self.config.pr_draft_by_default,
        };
        self.pr_draft = Some(draft.clone());
        self.state = GitHubWorkflowState::PrDraftReady;
        Ok(draft)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_workflow_state_transitions() {
        let mut mgr = GitHubWorkflowManager::new(GitHubWorkflowConfig::default());
        assert_eq!(mgr.state, GitHubWorkflowState::Unlinked);
        
        let status = mgr.status().unwrap();
        assert_eq!(status["state"], "Unlinked");

        mgr.link_issue("123").unwrap();
        assert_eq!(mgr.state, GitHubWorkflowState::IssueLinked);
        assert_eq!(mgr.linked_issue.as_ref().unwrap().number, "123");

        let plan = mgr.plan_branch().unwrap();
        assert_eq!(plan.suggested_name, "goat/123-resolve-issue-#123");
        assert_eq!(mgr.state, GitHubWorkflowState::BranchPlanned); // plan_branch sets this

        let draft = mgr.draft_pr().unwrap();
        assert_eq!(draft.title, "Resolve issue #123");
        assert_eq!(mgr.state, GitHubWorkflowState::PrDraftReady);
    }
}
