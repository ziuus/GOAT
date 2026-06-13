use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    Draft,
    Planned,
    Running,
    Blocked,
    Completed,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MissionType {
    BuildFeature,
    FixBug,
    Research,
    Documentation,
    Refactor,
    Test,
    Deploy,
    Learn,
    SystemTask,
    BusinessValidation,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPlanStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub assigned_agent: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRef {
    pub name: String,
    pub role: String,
    pub status: String, // "implemented", "planned", "unavailable"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    pub mission_id: String,
    pub title: String,
    pub raw_goal: String,
    pub mission_type: MissionType,
    pub recommended_agents: Vec<AgentRef>,
    pub plan_steps: Vec<MissionPlanStep>,
    pub expected_artifacts: Vec<String>,
    pub status: MissionStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub linked_project: Option<String>,
    pub progress: u8,
    pub notes: Vec<String>,
    pub risks: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPlanReq {
    pub goal: String,
    pub project_id: Option<String>,
    pub constraints: Option<Vec<String>>,
}

pub struct MissionControlManager {
    missions_dir: PathBuf,
    missions: Arc<Mutex<HashMap<String, Mission>>>,
}

impl MissionControlManager {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let missions_dir = home.join(".local/share/goat/missions");

        let manager = Self {
            missions_dir,
            missions: Arc::new(Mutex::new(HashMap::new())),
        };

        manager.ensure_dir();
        manager.load_all();
        manager
    }

    fn ensure_dir(&self) {
        if !self.missions_dir.exists() {
            let _ = fs::create_dir_all(&self.missions_dir);
        }
    }

    fn load_all(&self) {
        if let Ok(entries) = fs::read_dir(&self.missions_dir) {
            let mut missions_map = self.missions.lock().unwrap();
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(mission) = serde_json::from_str::<Mission>(&content) {
                            missions_map.insert(mission.mission_id.clone(), mission);
                        }
                    }
                }
            }
        }
    }

    fn save_mission(&self, mission: &Mission) {
        self.ensure_dir();
        let path = self
            .missions_dir
            .join(format!("{}.json", mission.mission_id));
        if let Ok(content) = serde_json::to_string_pretty(mission) {
            let _ = fs::write(path, content);
        }
    }

    pub fn get_missions(&self) -> Vec<Mission> {
        let missions_map = self.missions.lock().unwrap();
        let mut missions: Vec<Mission> = missions_map.values().cloned().collect();
        missions.sort_by_key(|m| -m.updated_at);
        missions
    }

    pub fn get_mission(&self, id: &str) -> Option<Mission> {
        let missions_map = self.missions.lock().unwrap();
        missions_map.get(id).cloned()
    }

    pub fn log_diff_analysis(
        &self,
        mission_id: &str,
        analysis: &crate::diff_analyzer::DiffAnalysis,
    ) {
        if let Some(mut mission) = self.get_mission(mission_id) {
            let note = format!(
                "[Diff Analysis] ID: {} | Risk: {:?} | Source: {:?}",
                analysis.analysis_id, analysis.risk_level, analysis.source_type
            );
            mission.notes.push(note);
            mission.updated_at = chrono::Utc::now().timestamp();
            self.update_mission(&mission);
        }
    }

    pub fn update_mission(&self, mission: &Mission) {
        let mut missions_map = self.missions.lock().unwrap();
        missions_map.insert(mission.mission_id.clone(), mission.clone());
        drop(missions_map);
        self.save_mission(mission);
    }

    pub fn plan_goal(&self, req: &MissionPlanReq) -> Mission {
        let goal_lower = req.goal.to_lowercase();

        let mission_type = if goal_lower.contains("fix")
            || goal_lower.contains("bug")
            || goal_lower.contains("issue")
        {
            MissionType::FixBug
        } else if goal_lower.contains("research")
            || goal_lower.contains("find out")
            || goal_lower.contains("investigate")
        {
            MissionType::Research
        } else if goal_lower.contains("doc")
            || goal_lower.contains("readme")
            || goal_lower.contains("guide")
        {
            MissionType::Documentation
        } else if goal_lower.contains("refactor")
            || goal_lower.contains("clean up")
            || goal_lower.contains("optimize")
        {
            MissionType::Refactor
        } else if goal_lower.contains("test")
            || goal_lower.contains("spec")
            || goal_lower.contains("coverage")
        {
            MissionType::Test
        } else if goal_lower.contains("deploy")
            || goal_lower.contains("release")
            || goal_lower.contains("publish")
        {
            MissionType::Deploy
        } else if goal_lower.contains("learn")
            || goal_lower.contains("understand")
            || goal_lower.contains("teach me")
        {
            MissionType::Learn
        } else if goal_lower.contains("validate")
            || goal_lower.contains("market")
            || goal_lower.contains("business")
        {
            MissionType::BusinessValidation
        } else {
            MissionType::BuildFeature
        };

        let mut recommended_agents = vec![];
        let mut plan_steps = vec![];
        let mut expected_artifacts = vec![];
        let mut title = "New Mission".to_string();

        match mission_type {
            MissionType::BuildFeature => {
                title = "Build New Feature".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Builder".to_string(),
                        role: "Implementation".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Reviewer".to_string(),
                        role: "Code Quality".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Tester".to_string(),
                        role: "Validation".to_string(),
                        status: "planned".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Analyze Requirements".to_string(),
                        description: "Review user goal and existing codebase constraints."
                            .to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Implement Feature".to_string(),
                        description: "Write the code to implement the feature.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-3".to_string(),
                        title: "Review & Test".to_string(),
                        description: "Ensure feature meets quality standards.".to_string(),
                        assigned_agent: Some("Reviewer".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec![
                    "Source Code Changes".to_string(),
                    "Feature Tests".to_string(),
                ];
            }
            MissionType::FixBug => {
                title = "Resolve Bug / Incident".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Operator".to_string(),
                        role: "Log Analysis".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Builder".to_string(),
                        role: "Patch Generation".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Reviewer".to_string(),
                        role: "Patch Validation".to_string(),
                        status: "implemented".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Reproduce Issue".to_string(),
                        description: "Analyze logs and reproduce the failure.".to_string(),
                        assigned_agent: Some("Operator".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Generate Patch".to_string(),
                        description: "Fix the offending code.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts =
                    vec!["Root Cause Analysis".to_string(), "Code Patch".to_string()];
            }
            MissionType::Research => {
                title = "Deep Research & Analysis".to_string();
                recommended_agents = vec![AgentRef {
                    name: "Researcher".to_string(),
                    role: "Information Gathering".to_string(),
                    status: "implemented".to_string(),
                }];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Gather Sources".to_string(),
                        description: "Find relevant documentation and codebase context."
                            .to_string(),
                        assigned_agent: Some("Researcher".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Synthesize Findings".to_string(),
                        description: "Compile a comprehensive report.".to_string(),
                        assigned_agent: Some("Researcher".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec![
                    "Research Report".to_string(),
                    "Source Citations".to_string(),
                ];
            }
            MissionType::Documentation => {
                title = "Update Documentation".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "DocAgent".to_string(),
                        role: "Documentation".to_string(),
                        status: "planned".to_string(),
                    },
                    AgentRef {
                        name: "Builder".to_string(),
                        role: "Fallback Documentation".to_string(),
                        status: "implemented".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Audit Docs".to_string(),
                        description: "Identify missing or outdated documentation.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Write Content".to_string(),
                        description: "Draft the new documentation.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec!["Markdown Files".to_string()];
            }
            MissionType::Refactor => {
                title = "Codebase Refactoring".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Builder".to_string(),
                        role: "Implementation".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Reviewer".to_string(),
                        role: "Validation".to_string(),
                        status: "implemented".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Identify Targets".to_string(),
                        description: "Find code smells and technical debt.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Apply Refactor".to_string(),
                        description: "Restructure code without changing behavior.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec!["Refactoring Patch".to_string()];
            }
            MissionType::Test => {
                title = "Test Suite Expansion".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Tester".to_string(),
                        role: "Test Generation".to_string(),
                        status: "planned".to_string(),
                    },
                    AgentRef {
                        name: "Builder".to_string(),
                        role: "Fallback Test Gen".to_string(),
                        status: "implemented".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Coverage Analysis".to_string(),
                        description: "Identify untested paths.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Write Tests".to_string(),
                        description: "Generate unit and integration tests.".to_string(),
                        assigned_agent: Some("Builder".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec!["Test Files".to_string(), "Coverage Report".to_string()];
            }
            MissionType::Deploy => {
                title = "Release & Deployment".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Operator".to_string(),
                        role: "Deployment Execution".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "SecurityAgent".to_string(),
                        role: "Security Audit".to_string(),
                        status: "planned".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Pre-flight Check".to_string(),
                        description: "Verify build passes and artifacts exist.".to_string(),
                        assigned_agent: Some("Operator".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Execute Deploy".to_string(),
                        description: "Push artifacts to target environment.".to_string(),
                        assigned_agent: Some("Operator".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec![
                    "Deployment Log".to_string(),
                    "Release Artifacts".to_string(),
                ];
            }
            MissionType::Learn => {
                title = "Learning & Onboarding".to_string();
                recommended_agents = vec![AgentRef {
                    name: "Learner".to_string(),
                    role: "Knowledge Transfer".to_string(),
                    status: "implemented".to_string(),
                }];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Index Context".to_string(),
                        description: "Read project documentation and code.".to_string(),
                        assigned_agent: Some("Learner".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "Explain Concepts".to_string(),
                        description: "Provide interactive learning session.".to_string(),
                        assigned_agent: Some("Learner".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec!["Learning Summary".to_string()];
            }
            MissionType::BusinessValidation => {
                title = "Business Strategy & Validation".to_string();
                recommended_agents = vec![
                    AgentRef {
                        name: "Cofounder".to_string(),
                        role: "Strategy".to_string(),
                        status: "implemented".to_string(),
                    },
                    AgentRef {
                        name: "Socializer".to_string(),
                        role: "Outreach".to_string(),
                        status: "implemented".to_string(),
                    },
                ];
                plan_steps = vec![
                    MissionPlanStep {
                        id: "step-1".to_string(),
                        title: "Market Analysis".to_string(),
                        description: "Analyze competitors and market fit.".to_string(),
                        assigned_agent: Some("Cofounder".to_string()),
                        status: "pending".to_string(),
                    },
                    MissionPlanStep {
                        id: "step-2".to_string(),
                        title: "User Feedback".to_string(),
                        description: "Generate distribution strategy.".to_string(),
                        assigned_agent: Some("Socializer".to_string()),
                        status: "pending".to_string(),
                    },
                ];
                expected_artifacts = vec![
                    "Strategy Deck".to_string(),
                    "Social Distribution Plan".to_string(),
                ];
            }
            _ => {
                title = "General System Task".to_string();
                recommended_agents = vec![AgentRef {
                    name: "Operator".to_string(),
                    role: "Execution".to_string(),
                    status: "implemented".to_string(),
                }];
                plan_steps = vec![MissionPlanStep {
                    id: "step-1".to_string(),
                    title: "Execute Script".to_string(),
                    description: "Run system commands to satisfy goal.".to_string(),
                    assigned_agent: Some("Operator".to_string()),
                    status: "pending".to_string(),
                }];
                expected_artifacts = vec!["Execution Log".to_string()];
            }
        }

        let mission_id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp_millis();

        let mut notes =
            vec!["Mission plan automatically generated via rule-based logic.".to_string()];

        // Inject memory context if available
        let paths = crate::paths::GoatPaths::resolve().unwrap();
        let memory_cfg = crate::config::MemoryConfig::default();
        let memory_mgr = crate::memory::MemoryManager::new(&paths, memory_cfg);

        let mut memories_found = 0;

        if let Some(ref pid) = req.project_id {
            if let Ok(mems) = memory_mgr.search_structured_memories(pid) {
                for mem in mems
                    .iter()
                    .filter(|m| m.status == crate::memory::MemoryStatus::Active)
                    .take(3)
                {
                    notes.push(format!("Memory [{:?}]: {}", mem.kind, mem.title));
                    memories_found += 1;
                }
            }
        }

        if let Ok(sys_mems) = memory_mgr.search_structured_memories(&req.goal) {
            for mem in sys_mems
                .iter()
                .filter(|m| {
                    m.status == crate::memory::MemoryStatus::Active
                        && (m.scope == crate::memory::MemoryScope::System
                            || m.scope == crate::memory::MemoryScope::User)
                })
                .take(2)
            {
                notes.push(format!("Relevant Memory [{:?}]: {}", mem.kind, mem.title));
                memories_found += 1;
            }
        }

        if memories_found > 0 {
            notes.push(format!(
                "Injected {} relevant memories into context.",
                memories_found
            ));
        }

        let mission = Mission {
            mission_id,
            title,
            raw_goal: req.goal.clone(),
            mission_type,
            recommended_agents,
            plan_steps,
            expected_artifacts,
            status: MissionStatus::Planned,
            created_at: now,
            updated_at: now,
            linked_project: req.project_id.clone(),
            progress: 0,
            notes,
            risks: vec!["Auto-generated plans may miss domain-specific nuances.".to_string()],
            next_actions: vec!["Review the plan and click 'Start Mission' when ready.".to_string()],
        };

        let mut missions_map = self.missions.lock().unwrap();
        missions_map.insert(mission.mission_id.clone(), mission.clone());
        drop(missions_map);

        self.save_mission(&mission);

        mission
    }
}
