use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::reports::ReportManager;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SocializerWorkflowState {
    #[default]
    Draft,
    AudienceMapped,
    ChannelStrategyDefined,
    ContentDrafted,
    LaunchPlanned,
    Active,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerCampaign {
    pub id: String,
    pub title: String,
    pub project_or_idea_ref: Option<String>,
    pub target_audience: String,
    pub value_proposition: String,
    pub state: SocializerWorkflowState,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerAudience {
    pub segments: Vec<String>,
    pub pain_points: Vec<String>,
    pub gathering_places: Vec<String>,
    pub objections: Vec<String>,
    pub trust_signals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerChannel {
    pub name: String,
    pub fit_score: u8,
    pub reason: String,
    pub content_type: String,
    pub risks: Vec<String>,
    pub etiquette: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerContentAngle {
    pub angle_type: String, // story, problem, build-in-public, lesson-learned, etc.
    pub target_platform: String,
    pub hook: String,
    pub main_point: String,
    pub cta: String,
    pub spam_risk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerContentDraft {
    pub platform: String,
    pub title_options: Vec<String>,
    pub body: String,
    pub non_promotional_version: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerLaunchPlan {
    pub pre_launch_checklist: Vec<String>,
    pub launch_day_plan: Vec<String>,
    pub metrics_to_track: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerCalendarItem {
    pub day: i32,
    pub platform: String,
    pub theme: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerFeedbackLoop {
    pub comments_summary: String,
    pub user_objections: Vec<String>,
    pub positive_signals: Vec<String>,
    pub negative_signals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializerEthicsPolicy {
    pub allowed: bool,
    pub violations: Vec<String>,
}

pub struct SocializerAgent {
    base_dir: PathBuf,
    campaigns: HashMap<String, SocializerCampaign>,
}

impl SocializerAgent {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/agents/prime/socializer");
        fs::create_dir_all(&base_dir)?;

        let mut agent = Self {
            base_dir,
            campaigns: HashMap::new(),
        };
        agent.load_campaigns()?;
        Ok(agent)
    }

    fn campaigns_file(&self) -> PathBuf {
        self.base_dir.join("campaigns.jsonl")
    }

    fn load_campaigns(&mut self) -> Result<()> {
        let path = self.campaigns_file();
        if !path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&path)?;
        for line in content.lines() {
            if line.trim().is_empty() { continue; }
            if let Ok(campaign) = serde_json::from_str::<SocializerCampaign>(line) {
                self.campaigns.insert(campaign.id.clone(), campaign);
            }
        }
        Ok(())
    }

    fn save_campaigns(&self) -> Result<()> {
        let path = self.campaigns_file();
        let mut lines = Vec::new();
        for campaign in self.campaigns.values() {
            if let Ok(json) = serde_json::to_string(campaign) {
                lines.push(json);
            }
        }
        fs::write(&path, lines.join("\n"))?;
        Ok(())
    }

    pub fn list_campaigns(&self) -> Vec<SocializerCampaign> {
        self.campaigns.values().cloned().collect()
    }

    pub fn get_campaign(&self, id: &str) -> Option<SocializerCampaign> {
        self.campaigns.get(id).cloned()
    }

    pub fn add_campaign(&mut self, title: String, target_audience: String, value_proposition: String, project_or_idea_ref: Option<String>) -> Result<SocializerCampaign> {
        let id = Uuid::new_v4().to_string();
        let campaign = SocializerCampaign {
            id: id.clone(),
            title,
            target_audience,
            value_proposition,
            project_or_idea_ref,
            state: SocializerWorkflowState::Draft,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };
        self.campaigns.insert(id, campaign.clone());
        self.save_campaigns()?;
        Ok(campaign)
    }

    pub fn generate_audience_map(&mut self, id: &str) -> Result<SocializerAudience> {
        if let Some(campaign) = self.campaigns.get_mut(id) {
            campaign.state = SocializerWorkflowState::AudienceMapped;
            campaign.updated_at = chrono::Utc::now().timestamp();
            self.save_campaigns()?;
        } else {
            return Err(anyhow::anyhow!("Campaign {} not found", id));
        }

        Ok(SocializerAudience {
            segments: vec!["Founders".to_string(), "Developers".to_string()],
            pain_points: vec!["Too much boilerplate".to_string()],
            gathering_places: vec!["Reddit/r/Entrepreneur".to_string(), "Hacker News".to_string()],
            objections: vec!["Too expensive".to_string()],
            trust_signals: vec!["Open source".to_string()],
        })
    }

    pub fn generate_channel_strategy(&mut self, id: &str) -> Result<Vec<SocializerChannel>> {
        if let Some(campaign) = self.campaigns.get_mut(id) {
            campaign.state = SocializerWorkflowState::ChannelStrategyDefined;
            campaign.updated_at = chrono::Utc::now().timestamp();
            self.save_campaigns()?;
        } else {
            return Err(anyhow::anyhow!("Campaign {} not found", id));
        }

        Ok(vec![
            SocializerChannel {
                name: "Reddit".to_string(),
                fit_score: 85,
                reason: "Good for highly technical deep dives.".to_string(),
                content_type: "Story/Architecture post".to_string(),
                risks: vec!["Hostile to pure marketing".to_string()],
                etiquette: vec!["No links in post body".to_string()],
            },
            SocializerChannel {
                name: "LinkedIn".to_string(),
                fit_score: 90,
                reason: "Professional audience".to_string(),
                content_type: "Build in public update".to_string(),
                risks: vec!["Can sound cringe if over-hyped".to_string()],
                etiquette: vec!["Be authentic".to_string()],
            }
        ])
    }

    pub fn generate_content_angles(&self, _id: &str) -> Result<Vec<SocializerContentAngle>> {
        Ok(vec![
            SocializerContentAngle {
                angle_type: "Lesson Learned".to_string(),
                target_platform: "X".to_string(),
                hook: "I spent 3 weeks building X, here is why it failed.".to_string(),
                main_point: "Validation is more important than code.".to_string(),
                cta: "Follow for more updates.".to_string(),
                spam_risk: "Low".to_string(),
            }
        ])
    }

    pub fn generate_draft(&mut self, id: &str, platform: &str) -> Result<SocializerContentDraft> {
        if let Some(campaign) = self.campaigns.get_mut(id) {
            campaign.state = SocializerWorkflowState::ContentDrafted;
            campaign.updated_at = chrono::Utc::now().timestamp();
            self.save_campaigns()?;
        } else {
            return Err(anyhow::anyhow!("Campaign {} not found", id));
        }

        Ok(SocializerContentDraft {
            platform: platform.to_string(),
            title_options: vec!["How we built X".to_string(), "The story behind X".to_string()],
            body: format!("Here is the draft for {}.", platform),
            non_promotional_version: "Just wanted to share our learnings...".to_string(),
            warnings: vec!["Do not post this in self-promotion free zones.".to_string()],
        })
    }

    pub fn generate_launch_plan(&mut self, id: &str) -> Result<SocializerLaunchPlan> {
        if let Some(campaign) = self.campaigns.get_mut(id) {
            campaign.state = SocializerWorkflowState::LaunchPlanned;
            campaign.updated_at = chrono::Utc::now().timestamp();
            self.save_campaigns()?;
        } else {
            return Err(anyhow::anyhow!("Campaign {} not found", id));
        }

        Ok(SocializerLaunchPlan {
            pre_launch_checklist: vec!["Warm up audience".to_string(), "Prepare assets".to_string()],
            launch_day_plan: vec!["Post on Product Hunt".to_string(), "Send Newsletter".to_string()],
            metrics_to_track: vec!["Signups".to_string(), "Visitors".to_string()],
        })
    }

    pub fn generate_calendar(&self, _id: &str) -> Result<Vec<SocializerCalendarItem>> {
        Ok(vec![
            SocializerCalendarItem { day: 1, platform: "X".to_string(), theme: "Teaser".to_string(), status: "Draft".to_string() },
            SocializerCalendarItem { day: 2, platform: "LinkedIn".to_string(), theme: "Story".to_string(), status: "Idea".to_string() },
        ])
    }

    pub fn generate_outreach(&self, _id: &str) -> Result<SocializerContentDraft> {
        Ok(SocializerContentDraft {
            platform: "Email".to_string(),
            title_options: vec!["Quick question about [Topic]".to_string()],
            body: "Hi [Name],\n\nI'm building [Project] and would love your feedback.\n\nBest,\n[My Name]".to_string(),
            non_promotional_version: "Hi [Name], saw your post on X and wanted to connect.".to_string(),
            warnings: vec!["Ensure [Name] and [Topic] are highly personalized.".to_string()],
        })
    }

    pub fn track_feedback(&self, _id: &str) -> Result<SocializerFeedbackLoop> {
        Ok(SocializerFeedbackLoop {
            comments_summary: "Mostly positive, some pricing concerns.".to_string(),
            user_objections: vec!["Price too high".to_string()],
            positive_signals: vec!["Love the UI".to_string()],
            negative_signals: vec!["Missing X feature".to_string()],
        })
    }

    pub fn generate_report(&self, id: &str) -> Result<String> {
        let report_manager = ReportManager::new();
        let template = crate::reports::ReportTemplate {
            kind: crate::reports::ReportKind::General,
            title: format!("Socializer Distribution Report {}", id),
            sections: vec![
                crate::reports::ReportSection {
                    heading: "Campaign Summary".to_string(),
                    body: format!("Report for campaign {}", id),
                }
            ],
        };
        let output = report_manager.generate_report(template)
            .map_err(|e| anyhow::anyhow!("Report Error: {}", e))?;
        Ok(format!("Report {} generated.", output.id))
    }
}
