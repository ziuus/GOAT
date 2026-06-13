#![allow(dead_code)]
use crate::brain_index::BrainIndexManager;
use crate::llm::LlmRouter;
use crate::models::ModelChain;
use crate::reports::ReportManager;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

// Enum matching requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SocialPlatform {
    #[default]
    Generic,
    LinkedIn,
    X,
    Reddit,
    GitHub,
    IndieHackers,
    HackerNews,
    Blog,
    Email,
    Discord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SocialContentKind {
    LaunchPost,
    BuildInPublicUpdate,
    FounderStory,
    FeatureAnnouncement,
    ProblemValidationPost,
    ResearchSummaryPost,
    ChangelogPost,
    CommunityReply,
    ColdOutreachDraft,
    RedditPost,
    LinkedinPost,
    XThread,
    BlogOutline,
    EmailDraft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SocialContentStatus {
    #[default]
    Draft,
    Reviewed,
    NeedsSources,
    NeedsApproval,
    ApprovedToCopy,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SocialSpamRisk {
    Low,
    Medium,
    High,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialVoiceRule {
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialBrandConstraint {
    pub constraint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialToneProfile {
    pub professionalism: String, // professional, casual
    pub technicality: String,    // technical, educational
    pub pacing: String,          // concise, detailed
    pub storytelling: String,    // founder-story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProfile {
    pub id: String,
    pub name: String,
    pub tone: SocialToneProfile,
    pub voice_rules: Vec<SocialVoiceRule>,
    pub constraints: Vec<SocialBrandConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContentSourceRef {
    pub source_id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContentSafetyNote {
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDraftVariant {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDraft {
    pub variants: Vec<SocialDraftVariant>,
    pub status: SocialContentStatus,
    pub spam_risk: SocialSpamRisk,
    pub safety_notes: Vec<SocialContentSafetyNote>,
    pub source_refs: Vec<SocialContentSourceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContentAsset {
    pub id: String,
    pub kind: SocialContentKind,
    pub platform: SocialPlatform,
    pub draft: SocialDraft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDistributionPolicy {
    pub allowed: bool,
    pub community_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPlan {
    pub id: String,
    pub goal: String,
    pub channels: Vec<SocialPlatform>,
    pub assets_needed: Vec<SocialContentKind>,
    pub checklists: Vec<String>,
    pub risks: Vec<String>,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialCalendarItem {
    pub day: i32,
    pub platform: SocialPlatform,
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContentCalendar {
    pub id: String,
    pub items: Vec<SocialCalendarItem>,
}

pub struct SocializerAgent {
    base_dir: PathBuf,
    profiles: HashMap<String, SocialProfile>,
    drafts: HashMap<String, SocialContentAsset>,
    launch_plans: HashMap<String, LaunchPlan>,
    calendars: HashMap<String, SocialContentCalendar>,
}

impl SocializerAgent {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let base_dir = home.join(".local/share/goat/agents/prime/socializer");
        fs::create_dir_all(&base_dir)?;

        let mut agent = Self {
            base_dir,
            profiles: HashMap::new(),
            drafts: HashMap::new(),
            launch_plans: HashMap::new(),
            calendars: HashMap::new(),
        };
        Ok(agent)
    }

    pub async fn deep_generate_draft(
        &mut self,
        _id: &str, // contextual id like campaign or project
        platform: SocialPlatform,
        kind: SocialContentKind,
        _brain_manager: &BrainIndexManager,
        llm_router: &LlmRouter,
        model_chain: &ModelChain,
    ) -> Result<SocialContentAsset> {
        let sys_prompt = String::from(
            "You are the GOAT Socializer Agent. You create ethical, non-spammy content drafts.\n\
             Return ONLY a JSON object representing the SocialContentAsset. No markdown.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"kind\": \"LaunchPost\",\n\
               \"platform\": \"Generic\",\n\
               \"draft\": {\n\
                   \"variants\": [{\"title\": \"string\", \"body\": \"string\"}],\n\
                   \"status\": \"Draft\",\n\
                   \"spam_risk\": \"Low\",\n\
                   \"safety_notes\": [{\"note\": \"string\"}],\n\
                   \"source_refs\": []\n\
               }\n\
             }",
        );

        let user_prompt = format!(
            "Generate a draft for platform {:?} of kind {:?}.",
            platform, kind
        );

        let messages = vec![
            crate::llm::Message {
                role: "system".to_string(),
                content: Some(sys_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
            crate::llm::Message {
                role: "user".to_string(),
                content: Some(user_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let (response, _) = llm_router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow!("LLM failed: {}", e))?;
        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        let mut asset = serde_json::from_str::<SocialContentAsset>(cleaned)
            .map_err(|e| anyhow!("Parse error: {}", e))?;
        asset.id = Uuid::new_v4().to_string();
        self.drafts.insert(asset.id.clone(), asset.clone());
        Ok(asset)
    }

    pub async fn deep_generate_launch_plan(
        &mut self,
        goal: &str,
        _brain_manager: &BrainIndexManager,
        llm_router: &LlmRouter,
        model_chain: &ModelChain,
    ) -> Result<LaunchPlan> {
        let sys_prompt = String::from(
            "You are the GOAT Socializer Agent. You create ethical launch plans.\n\
             Return ONLY a JSON object representing the LaunchPlan. No markdown.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"goal\": \"string\",\n\
               \"channels\": [\"Generic\"],\n\
               \"assets_needed\": [\"LaunchPost\"],\n\
               \"checklists\": [\"string\"],\n\
               \"risks\": [\"string\"],\n\
               \"metrics\": [\"string\"]\n\
             }",
        );

        let user_prompt = format!("Generate a launch plan for goal: {}", goal);

        let messages = vec![
            crate::llm::Message {
                role: "system".to_string(),
                content: Some(sys_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
            crate::llm::Message {
                role: "user".to_string(),
                content: Some(user_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let (response, _) = llm_router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow!("LLM failed: {}", e))?;
        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        let mut plan = serde_json::from_str::<LaunchPlan>(cleaned)
            .map_err(|e| anyhow!("Parse error: {}", e))?;
        plan.id = Uuid::new_v4().to_string();
        self.launch_plans.insert(plan.id.clone(), plan.clone());
        Ok(plan)
    }

    pub async fn deep_generate_calendar(
        &mut self,
        goal: &str,
        _brain_manager: &BrainIndexManager,
        llm_router: &LlmRouter,
        model_chain: &ModelChain,
    ) -> Result<SocialContentCalendar> {
        let sys_prompt = String::from(
            "You are the GOAT Socializer Agent. You create ethical content calendars.\n\
             Return ONLY a JSON object representing the SocialContentCalendar. No markdown.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"items\": [{\"day\": 1, \"platform\": \"Generic\", \"theme\": \"string\"}]\n\
             }",
        );

        let user_prompt = format!("Generate a content calendar for goal: {}", goal);

        let messages = vec![
            crate::llm::Message {
                role: "system".to_string(),
                content: Some(sys_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
            crate::llm::Message {
                role: "user".to_string(),
                content: Some(user_prompt),
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let (response, _) = llm_router
            .completion_with_fallback(model_chain, messages, None)
            .await
            .map_err(|e| anyhow!("LLM failed: {}", e))?;
        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        let mut cal = serde_json::from_str::<SocialContentCalendar>(cleaned)
            .map_err(|e| anyhow!("Parse error: {}", e))?;
        cal.id = Uuid::new_v4().to_string();
        self.calendars.insert(cal.id.clone(), cal.clone());
        Ok(cal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socializer_agent_creation() {
        let agent = SocializerAgent::new();
        assert!(agent.is_ok());
    }

    #[test]
    fn test_enums_serialize_correctly() {
        let platform = SocialPlatform::LinkedIn;
        let serialized = serde_json::to_string(&platform).unwrap();
        assert_eq!(serialized, "\"LinkedIn\"");
    }
}
