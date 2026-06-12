use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GoatProject {
    pub id: String,
    pub goal: String,
    pub status: String,
    pub agent_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub context_refs: Vec<String>,
    pub next_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPlanReq {
    pub goal: String,
    pub project_id: Option<String>,
    pub constraints: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPlanRes {
    pub goal_type: String,
    pub suggested_agents: Vec<String>,
    pub suggested_workflow: String,
    pub expected_artifacts: Vec<String>,
    pub required_approvals: Vec<String>,
    pub next_actions: Vec<String>,
    pub safety_notes: Vec<String>,
}

pub struct MissionControlManager {}

impl MissionControlManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn plan_goal(&self, req: &MissionPlanReq) -> MissionPlanRes {
        let goal_lower = req.goal.to_lowercase();
        
        let mut goal_type = "mixed".to_string();
        let mut suggested_agents = vec![];
        let mut suggested_workflow = "Default Execution".to_string();
        let mut expected_artifacts = vec![];
        let mut required_approvals = vec!["ApprovalGate for execution".to_string()];

        if goal_lower.contains("validate") && goal_lower.contains("marketplace") {
            goal_type = "validate".to_string();
            suggested_agents = vec!["Cofounder".to_string(), "Researcher".to_string(), "Socializer".to_string()];
            suggested_workflow = "Idea Validation & Distribution".to_string();
            expected_artifacts = vec!["Cofounder Validation Report".to_string(), "Researcher Brief".to_string()];
        } else if goal_lower.contains("ui") || goal_lower.contains("design") {
            goal_type = "design".to_string();
            suggested_agents = vec!["Designer".to_string(), "Builder".to_string()];
            suggested_workflow = "UI Implementation".to_string();
            expected_artifacts = vec!["Design Review".to_string(), "UI Patch".to_string()];
        } else if goal_lower.contains("fix") || goal_lower.contains("release") {
            goal_type = "operate".to_string();
            suggested_agents = vec!["Operator".to_string(), "Builder".to_string()];
            suggested_workflow = "Incident Resolution".to_string();
            expected_artifacts = vec!["Operator Log Report".to_string(), "Patch".to_string()];
        } else if goal_lower.contains("learn") {
            goal_type = "learn".to_string();
            suggested_agents = vec!["Learner".to_string(), "Researcher".to_string()];
            suggested_workflow = "Learning Path".to_string();
            expected_artifacts = vec!["Learning Roadmap".to_string()];
        } else if goal_lower.contains("launch") {
            goal_type = "mixed".to_string();
            suggested_agents = vec!["Cofounder".to_string(), "Designer".to_string(), "Socializer".to_string(), "Operator".to_string()];
            suggested_workflow = "Product Launch".to_string();
            expected_artifacts = vec!["Launch Checklist".to_string(), "Social Drafts".to_string()];
        } else {
            suggested_agents = vec!["Builder".to_string()];
            expected_artifacts = vec!["Execution Plan".to_string()];
        }

        MissionPlanRes {
            goal_type,
            suggested_agents,
            suggested_workflow,
            expected_artifacts,
            required_approvals,
            next_actions: vec!["Review the plan and approve execution.".to_string()],
            safety_notes: vec!["This is a safe planning phase. No execution happens until approved.".to_string()],
        }
    }
}
