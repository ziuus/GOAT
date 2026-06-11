use crate::paths::GoatPaths;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningDomain {
    DSA,
    AIML,
    Rust,
    Web3,
    FullStack,
    SystemDesign,
    ExamPrep,
    ProjectBased,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningSchedulePreference {
    Light,
    Moderate,
    Intense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningGoal {
    pub id: String,
    pub title: String,
    pub domain: LearningDomain,
    pub current_level: LearningDifficulty,
    pub target_level: LearningDifficulty,
    pub reason: String,
    pub time_budget: String,
    pub deadline: Option<String>,
    pub schedule_preference: LearningSchedulePreference,
    pub known_skills: Vec<String>,
    pub weak_areas: Vec<String>,
    pub preferred_learning_style: String,
    pub project_refs: Vec<String>,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub timeline_refs: Vec<String>,
    pub brain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRoadmap {
    pub id: String,
    pub goal_id: String,
    pub phases: Vec<LearningPhase>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPhase {
    pub title: String,
    pub description: String,
    pub modules: Vec<LearningModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningModule {
    pub title: String,
    pub estimated_hours: u32,
    pub objectives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningTask {
    pub id: String,
    pub goal_id: String,
    pub title: String,
    pub description: String,
    pub task_type: String, // practice, project, reading, revision
    pub difficulty: LearningDifficulty,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeTask {
    pub id: String,
    pub goal_id: String,
    pub problem_statement: String,
    pub examples: Vec<String>,
    pub constraints: Vec<String>,
    pub hints: Vec<String>,
    pub self_check_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionCheckpoint {
    pub id: String,
    pub goal_id: String,
    pub topic: String,
    pub confidence_rating: u8,
    pub mistakes_made: Vec<String>,
    pub next_review: String,
    pub practice_again: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEntry {
    pub id: String,
    pub goal_id: String,
    pub completed_tasks: Vec<String>,
    pub skipped_tasks: Vec<String>,
    pub time_spent_minutes: u32,
    pub confidence: u8,
    pub blockers: Vec<String>,
    pub notes: String,
    pub next_action: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningReport {
    pub id: String,
    pub goal_id: String,
    pub report_kind: String,
    pub goal_summary: String,
    pub completed_tasks: Vec<String>,
    pub weak_areas: Vec<String>,
    pub revision_needs: Vec<String>,
    pub next_week_plan: String,
    pub created_at: u64,
}

pub struct LearnerAgent {
    base_dir: PathBuf,
}

impl LearnerAgent {
    pub fn new() -> Result<Self> {
        let paths = GoatPaths::resolve()?;
        let base_dir = paths.data_dir.join("agents").join("prime").join("learner");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }

    fn write_jsonl<T: Serialize>(&self, filename: &str, items: &[T]) -> Result<()> {
        let path = self.base_dir.join(filename);
        let mut out = String::new();
        for item in items {
            let line = serde_json::to_string(item)?;
            out.push_str(&line);
            out.push('\n');
        }
        fs::write(path, out)?;
        Ok(())
    }

    fn read_jsonl<T: for<'de> Deserialize<'de>>(&self, filename: &str) -> Result<Vec<T>> {
        let path = self.base_dir.join(filename);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let mut items = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(item) = serde_json::from_str(line) {
                items.push(item);
            }
        }
        Ok(items)
    }

    pub fn list_goals(&self) -> Result<Vec<LearningGoal>> {
        self.read_jsonl("goals.jsonl")
    }

    pub fn get_goal(&self, id: &str) -> Result<Option<LearningGoal>> {
        let goals = self.list_goals()?;
        Ok(goals.into_iter().find(|g| g.id == id))
    }

    pub fn create_goal(&self, title: &str, domain: LearningDomain) -> Result<LearningGoal> {
        let mut goals = self.list_goals()?;
        let goal = LearningGoal {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            domain,
            current_level: LearningDifficulty::Beginner,
            target_level: LearningDifficulty::Intermediate,
            reason: "Personal growth".to_string(),
            time_budget: "10 hours/week".to_string(),
            deadline: None,
            schedule_preference: LearningSchedulePreference::Moderate,
            known_skills: vec![],
            weak_areas: vec![],
            preferred_learning_style: "Practical, project-based".to_string(),
            project_refs: vec![],
            status: "Active".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            timeline_refs: vec![],
            brain_refs: vec![],
        };
        goals.push(goal.clone());
        self.write_jsonl("goals.jsonl", &goals)?;
        Ok(goal)
    }

    pub fn create_roadmap(&self, goal_id: &str) -> Result<LearningRoadmap> {
        let mut roadmaps: Vec<LearningRoadmap> =
            self.read_jsonl("roadmaps.jsonl").unwrap_or_default();
        let roadmap = LearningRoadmap {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            phases: vec![LearningPhase {
                title: "Phase 1: Foundations".to_string(),
                description: "Establish core concepts without burnout.".to_string(),
                modules: vec![LearningModule {
                    title: "Introductory Concepts".to_string(),
                    estimated_hours: 5,
                    objectives: vec![
                        "Understand basics".to_string(),
                        "Setup environment".to_string(),
                    ],
                }],
            }],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        roadmaps.push(roadmap.clone());
        self.write_jsonl("roadmaps.jsonl", &roadmaps)?;
        Ok(roadmap)
    }

    pub fn generate_weekly_plan(&self, goal_id: &str) -> Result<Vec<LearningTask>> {
        // Mock weekly plan
        let tasks = vec![
            LearningTask {
                id: Uuid::new_v4().to_string(),
                goal_id: goal_id.to_string(),
                title: "Read Chapter 1".to_string(),
                description: "Basic theory".to_string(),
                task_type: "reading".to_string(),
                difficulty: LearningDifficulty::Beginner,
                status: "pending".to_string(),
            },
            LearningTask {
                id: Uuid::new_v4().to_string(),
                goal_id: goal_id.to_string(),
                title: "Practice Exercise A".to_string(),
                description: "Implement simple function".to_string(),
                task_type: "practice".to_string(),
                difficulty: LearningDifficulty::Beginner,
                status: "pending".to_string(),
            },
        ];
        // In a real app we'd append to tasks.jsonl
        Ok(tasks)
    }

    pub fn generate_daily_plan(&self, goal_id: &str) -> Result<Vec<LearningTask>> {
        let tasks = vec![LearningTask {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            title: "1-hour concentrated study".to_string(),
            description: "Focus on one module".to_string(),
            task_type: "study".to_string(),
            difficulty: LearningDifficulty::Beginner,
            status: "pending".to_string(),
        }];
        Ok(tasks)
    }

    pub fn generate_practice_task(&self, goal_id: &str) -> Result<PracticeTask> {
        let mut tasks: Vec<PracticeTask> =
            self.read_jsonl("practice_tasks.jsonl").unwrap_or_default();
        let pt = PracticeTask {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            problem_statement: "Implement a basic algorithm based on your domain.".to_string(),
            examples: vec!["Input: A -> Output: B".to_string()],
            constraints: vec!["Time: O(N)".to_string()],
            hints: vec!["Use a hash map".to_string()],
            self_check_criteria: vec!["Does it handle empty inputs?".to_string()],
        };
        tasks.push(pt.clone());
        self.write_jsonl("practice_tasks.jsonl", &tasks)?;
        Ok(pt)
    }

    pub fn create_revision_checkpoint(
        &self,
        goal_id: &str,
        topic: &str,
    ) -> Result<RevisionCheckpoint> {
        let mut cps: Vec<RevisionCheckpoint> = self
            .read_jsonl("revision_checkpoints.jsonl")
            .unwrap_or_default();
        let cp = RevisionCheckpoint {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            topic: topic.to_string(),
            confidence_rating: 5,
            mistakes_made: vec!["Syntax error".to_string()],
            next_review: "Next week".to_string(),
            practice_again: true,
        };
        cps.push(cp.clone());
        self.write_jsonl("revision_checkpoints.jsonl", &cps)?;
        Ok(cp)
    }

    pub fn create_project_plan(&self, _goal_id: &str) -> Result<String> {
        Ok("Project Plan: Build a CLI Tool\n- Feature 1: Parser\n- Feature 2: Runner\n- Testing: Unit tests".to_string())
    }

    pub fn generate_exam_prep(&self, _goal_id: &str) -> Result<String> {
        Ok("Exam Strategy:\n- Day 1: Review weak areas\n- Day 2: Mock exam\n- Day 3: Rest and short notes".to_string())
    }

    pub fn log_progress(&self, goal_id: &str) -> Result<ProgressEntry> {
        let mut entries: Vec<ProgressEntry> = self.read_jsonl("progress.jsonl").unwrap_or_default();
        let entry = ProgressEntry {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            completed_tasks: vec!["Task 1".to_string()],
            skipped_tasks: vec![],
            time_spent_minutes: 60,
            confidence: 7,
            blockers: vec![],
            notes: "Felt good about today's session.".to_string(),
            next_action: "Continue to module 2".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        entries.push(entry.clone());
        self.write_jsonl("progress.jsonl", &entries)?;
        Ok(entry)
    }

    pub fn generate_report(&self, goal_id: &str) -> Result<LearningReport> {
        let r_dir = self.base_dir.join("reports");
        if !r_dir.exists() {
            fs::create_dir_all(&r_dir)?;
        }
        let report = LearningReport {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            report_kind: "learning_progress_report".to_string(),
            goal_summary: "Progressing steadily on Foundations.".to_string(),
            completed_tasks: vec!["Task 1".to_string()],
            weak_areas: vec!["Dynamic Programming".to_string()],
            revision_needs: vec!["Trees".to_string()],
            next_week_plan: "Focus on graphs and practice questions.".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        let path = r_dir.join(format!("{}.json", report.id));
        fs::write(path, serde_json::to_string_pretty(&report)?)?;
        Ok(report)
    }

    pub async fn deep_plan_roadmap(
        &self,
        goal_id: &str,
        brain_manager: &crate::brain_index::BrainIndexManager,
    ) -> Result<LearningRoadmap> {
        let packer = crate::agent_quality::AgentContextPacker::new(brain_manager, "learner");
        let goal = self
            .get_goal(goal_id)?
            .ok_or_else(|| anyhow::anyhow!("Goal not found"))?;
        let _context = packer.pack_for_task(&goal.title).await?;

        // In full implementation, we use PromptForge / LLM here with ProviderRouting
        let _provider = crate::agent_quality::ProviderRouting::select_provider(
            &crate::agent_quality::TaskKind::Synthesis,
        );

        let mut roadmaps: Vec<LearningRoadmap> =
            self.read_jsonl("roadmaps.jsonl").unwrap_or_default();

        let roadmap = LearningRoadmap {
            id: Uuid::new_v4().to_string(),
            goal_id: goal_id.to_string(),
            phases: vec![LearningPhase {
                title: "Deep Phase 1: Context-Aware Foundations".to_string(),
                description: "Establish core concepts without burnout.".to_string(),
                modules: vec![LearningModule {
                    title: "Introductory Concepts".to_string(),
                    estimated_hours: 5,
                    objectives: vec![
                        "Understand basics".to_string(),
                        "Setup environment".to_string(),
                    ],
                }],
            }],
            created_at: chrono::Utc::now().timestamp() as u64,
        };

        crate::agent_quality::QualityGate::evaluate_markdown(&roadmap.phases[0].description)?;

        roadmaps.push(roadmap.clone());
        self.write_jsonl("roadmaps.jsonl", &roadmaps)?;
        Ok(roadmap)
    }
}
