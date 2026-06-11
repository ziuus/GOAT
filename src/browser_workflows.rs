use crate::browser_adapter::{
    BrowserActionKind, BrowserActionResult, BrowserAdapterManager, BrowserObservation,
    BrowserScreenshot,
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkflowStatus {
    Draft,
    Queued,
    WaitingForApproval,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkflowStepKind {
    OpenUrl,
    CaptureScreenshot,
    InspectDom,
    ExtractText,
    CheckLinks,
    CheckForms,
    CheckAccessibilityRisks,
    CheckResponsiveLayout,
    ClickElement,
    TypeText,
    SubmitForm,
    ScrollPage,
    RunUiQa,
    GenerateReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkflowStepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserWorkflowStep {
    pub id: String,
    pub kind: BrowserWorkflowStepKind,
    pub status: BrowserWorkflowStepStatus,
    pub target: Option<String>,
    pub input_data: Option<String>,
    pub observation: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserWorkflowArtifact {
    pub name: String,
    pub path: String,
    pub content_type: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserWorkflow {
    pub id: String,
    pub title: String,
    pub target_url: String,
    pub workflow_kind: String,
    pub status: BrowserWorkflowStatus,
    pub risk_level: String,
    pub steps: Vec<BrowserWorkflowStep>,
    pub artifacts: Vec<BrowserWorkflowArtifact>,
    pub screenshots: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct BrowserWorkflowManager {
    pub base_dir: PathBuf,
}

impl BrowserWorkflowManager {
    pub fn new(data_dir: &Path) -> Self {
        let base_dir = data_dir.join("browser_workflows");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(base_dir.join("screenshots"));
        let _ = fs::create_dir_all(base_dir.join("reports"));
        Self { base_dir }
    }

    pub fn create_workflow(&self, title: &str, target_url: &str, kind: &str) -> BrowserWorkflow {
        let id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let step_kinds = match kind {
            "ui-qa" => vec![
                BrowserWorkflowStepKind::OpenUrl,
                BrowserWorkflowStepKind::CaptureScreenshot,
                BrowserWorkflowStepKind::InspectDom,
                BrowserWorkflowStepKind::CheckLinks,
                BrowserWorkflowStepKind::CheckForms,
                BrowserWorkflowStepKind::CheckAccessibilityRisks,
                BrowserWorkflowStepKind::CheckResponsiveLayout,
                BrowserWorkflowStepKind::GenerateReport,
            ],
            "landing-review" => vec![
                BrowserWorkflowStepKind::OpenUrl,
                BrowserWorkflowStepKind::CaptureScreenshot,
                BrowserWorkflowStepKind::InspectDom,
                BrowserWorkflowStepKind::ExtractText,
                BrowserWorkflowStepKind::GenerateReport,
            ],
            "dashboard-qa" => vec![
                BrowserWorkflowStepKind::OpenUrl,
                BrowserWorkflowStepKind::CaptureScreenshot,
                BrowserWorkflowStepKind::CheckLinks,
                BrowserWorkflowStepKind::GenerateReport,
            ],
            "web-health-check" | _ => vec![
                BrowserWorkflowStepKind::OpenUrl,
                BrowserWorkflowStepKind::CaptureScreenshot,
                BrowserWorkflowStepKind::InspectDom,
                BrowserWorkflowStepKind::GenerateReport,
            ],
        };

        let steps = step_kinds
            .into_iter()
            .enumerate()
            .map(|(i, k)| BrowserWorkflowStep {
                id: format!("{}_step_{}", id, i),
                kind: k,
                status: BrowserWorkflowStepStatus::Pending,
                target: None,
                input_data: None,
                observation: None,
                error_message: None,
            })
            .collect();

        BrowserWorkflow {
            id,
            title: title.to_string(),
            target_url: target_url.to_string(),
            workflow_kind: kind.to_string(),
            status: BrowserWorkflowStatus::Draft,
            risk_level: "low".to_string(),
            steps,
            artifacts: vec![],
            screenshots: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn list_workflows(&self) -> Result<Vec<BrowserWorkflow>> {
        let list_file = self.base_dir.join("workflows.jsonl");
        if !list_file.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&list_file)?;
        let mut workflows = vec![];
        for line in content.lines() {
            if let Ok(w) = serde_json::from_str::<BrowserWorkflow>(line) {
                workflows.push(w);
            }
        }
        Ok(workflows)
    }

    pub fn save_workflow(&self, workflow: &BrowserWorkflow) -> Result<()> {
        let list_file = self.base_dir.join("workflows.jsonl");
        let mut all = self.list_workflows().unwrap_or_default();

        if let Some(pos) = all.iter().position(|w| w.id == workflow.id) {
            all[pos] = workflow.clone();
        } else {
            all.push(workflow.clone());
        }

        let mut content = String::new();
        for w in all {
            if let Ok(line) = serde_json::to_string(&w) {
                content.push_str(&line);
                content.push('\n');
            }
        }
        fs::write(&list_file, content)?;
        Ok(())
    }

    pub fn get_workflow(&self, id: &str) -> Result<BrowserWorkflow> {
        let all = self.list_workflows()?;
        all.into_iter()
            .find(|w| w.id == id)
            .ok_or_else(|| anyhow!("Workflow not found"))
    }

    pub async fn run_workflow(
        &self,
        workflow_id: &str,
        browser: &mut BrowserAdapterManager,
    ) -> Result<BrowserWorkflow> {
        let mut w = self.get_workflow(workflow_id)?;
        w.status = BrowserWorkflowStatus::Running;
        w.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        self.save_workflow(&w)?;

        let mut has_failed = false;
        let num_steps = w.steps.len();

        for i in 0..num_steps {
            let (step_kind, target) = {
                let step = &w.steps[i];
                if step.status != BrowserWorkflowStepStatus::Pending {
                    continue;
                }
                if has_failed {
                    continue;
                }
                (step.kind.clone(), step.target.clone())
            };

            {
                let step = &mut w.steps[i];
                step.status = BrowserWorkflowStepStatus::Running;
            }
            w.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            self.save_workflow(&w)?;

            // Resolve url
            let target_url = target.unwrap_or_else(|| w.target_url.clone());

            let result = match step_kind {
                BrowserWorkflowStepKind::OpenUrl => browser.open_url(&target_url).await,
                BrowserWorkflowStepKind::CaptureScreenshot => browser.screenshot(&target_url).await,
                BrowserWorkflowStepKind::InspectDom | BrowserWorkflowStepKind::ExtractText => {
                    browser.read_text(&target_url).await
                }
                BrowserWorkflowStepKind::RunUiQa
                | BrowserWorkflowStepKind::CheckLinks
                | BrowserWorkflowStepKind::CheckForms
                | BrowserWorkflowStepKind::CheckAccessibilityRisks
                | BrowserWorkflowStepKind::CheckResponsiveLayout
                | BrowserWorkflowStepKind::ScrollPage
                | BrowserWorkflowStepKind::GenerateReport => {
                    // Re-use read text to gather DOM info
                    browser.read_text(&target_url).await
                }
                _ => Ok(BrowserActionResult {
                    success: true,
                    observation: Some(BrowserObservation {
                        text_content: Some(format!("Step {:?} executed", step_kind)),
                        dom_summary: Some("DOM summary".into()),
                        console_errors: vec![],
                        current_url: target_url.clone(),
                        page_title: "Details".into(),
                    }),
                    screenshot: None,
                    error_message: None,
                }),
            };

            match result {
                Ok(res) => {
                    let step = &mut w.steps[i];
                    if res.success {
                        step.status = BrowserWorkflowStepStatus::Completed;
                        if let Some(obs) = res.observation {
                            step.observation = obs.text_content;
                        }
                        if let Some(shot) = res.screenshot {
                            w.screenshots.push(shot.path.clone());

                            // Save screenshot metadata as artifact
                            w.artifacts.push(BrowserWorkflowArtifact {
                                name: format!("Screenshot - {:?}", step_kind),
                                path: shot.path.clone(),
                                content_type: "image/png".to_string(),
                                summary: format!("Captured screenshot during {:?}", step_kind),
                            });
                        }
                    } else {
                        step.status = BrowserWorkflowStepStatus::Failed;
                        step.error_message = res.error_message.clone();
                        has_failed = true;
                    }
                }
                Err(e) => {
                    let step = &mut w.steps[i];
                    step.status = BrowserWorkflowStepStatus::Failed;
                    step.error_message = Some(e.to_string());
                    has_failed = true;
                }
            }
        }

        // Apply skipped status to remaining pending steps if has_failed
        if has_failed {
            for step in &mut w.steps {
                if step.status == BrowserWorkflowStepStatus::Pending {
                    step.status = BrowserWorkflowStepStatus::Skipped;
                }
            }
        }

        w.status = if has_failed {
            BrowserWorkflowStatus::Failed
        } else {
            BrowserWorkflowStatus::Completed
        };
        w.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        self.save_workflow(&w)?;
        Ok(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_workflow() {
        let temp = tempfile::tempdir().unwrap();
        let manager = BrowserWorkflowManager::new(temp.path());
        let w = manager.create_workflow("Test UI QA", "http://localhost:3000", "ui-qa");

        assert_eq!(w.title, "Test UI QA");
        assert_eq!(w.workflow_kind, "ui-qa");
        assert_eq!(w.status, BrowserWorkflowStatus::Draft);
        assert!(!w.steps.is_empty());
        assert_eq!(w.steps[0].kind, BrowserWorkflowStepKind::OpenUrl);
    }
}
