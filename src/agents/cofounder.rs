use crate::error::GoatError;
use crate::reports::{ReportKind, ReportManager, ReportSection, ReportTemplate};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub type CofounderIdeaId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CofounderWorkflowState {
    #[default]
    IdeaLogged,
    Validating,
    NeedsEvidence,
    Validated,
    Scored,
    MvpScoped,
    OutreachPlanned,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderProblem {
    pub description: String,
    pub pain_intensity: u8, // 1-10
    pub urgency: u8,        // 1-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetUserSegment {
    pub description: String,
    pub demographics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentAlternative {
    pub name: String,
    pub description: String,
    pub weaknesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderAssumption {
    pub id: String,
    pub description: String,
    pub is_critical: bool,
    pub validation_status: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderRisk {
    pub description: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderEvidenceRef {
    pub source_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoToMarketChannel {
    pub name: String,
    pub feasibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderIdea {
    pub id: CofounderIdeaId,
    pub title: String,
    pub one_line_pitch: String,
    pub target_user: TargetUserSegment,
    pub problem: FounderProblem,
    pub current_alternatives: Vec<CurrentAlternative>,
    pub willingness_to_pay_hypothesis: String,
    pub distribution_channels: Vec<GoToMarketChannel>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<FounderAssumption>,
    pub risks: Vec<FounderRisk>,
    pub evidence_refs: Vec<FounderEvidenceRef>,
    pub validation_status: String,
    pub decision_state: FounderDecision,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketSignalType {
    UserInterview,
    WaitlistSignup,
    ManualInquiry,
    CompetitorActivity,
    SearchTrendNote,
    ForumDemandNote,
    PaidPreOrder,
    LandingPageConversion,
    ColdOutreachReply,
    PersonalObservation,
    ResearchSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketSignalStrength {
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketSignalStatus {
    Raw,
    Analyzed,
    LinkedToAssumption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSignal {
    pub id: String,
    pub idea_id: String,
    pub signal_type: MarketSignalType,
    pub strength: MarketSignalStrength,
    pub source: String,
    pub description: String,
    pub linked_assumptions: Vec<String>,
    pub status: MarketSignalStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExperimentStep {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExperimentMetric {
    pub name: String,
    pub target_threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExperimentResult {
    pub success: bool,
    pub evidence_collected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExperiment {
    pub id: String,
    pub idea_id: String,
    pub experiment_type: String, // e.g. "landing page smoke test"
    pub steps: Vec<ValidationExperimentStep>,
    pub metrics: Vec<ValidationExperimentMetric>,
    pub result: Option<ValidationExperimentResult>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPFeature {
    pub name: String,
    pub description: String,
    pub is_must_have: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPNonGoal {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPBuildRisk {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPWeekPlan {
    pub week_number: u8,
    pub goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPBuilderHandoff {
    pub ready: bool,
    pub technical_unknowns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MVPHypothesis {
    pub id: String,
    pub idea_id: String,
    pub features: Vec<MVPFeature>,
    pub non_goals: Vec<MVPNonGoal>,
    pub risks: Vec<MVPBuildRisk>,
    pub weekly_plans: Vec<MVPWeekPlan>,
    pub builder_handoff: MVPBuilderHandoff,
    pub riskiest_assumption_tested: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    pub name: String,
    pub price: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRationale {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WillingnessToPaySignal {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingHypothesis {
    pub id: String,
    pub idea_id: String,
    pub free_manual_test_option: String,
    pub tiers: Vec<PricingTier>,
    pub rationale: Vec<PricingRationale>,
    pub value_metric: String,
    pub pricing_risks: Vec<String>,
    pub evidence_needed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FounderDecision {
    Undecided,
    Continue,
    Pivot,
    Kill,
}

impl Default for FounderDecision {
    fn default() -> Self {
        Self::Undecided
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderScorecard {
    pub idea_id: String,
    pub problem_clarity: u8,
    pub target_user_specificity: u8,
    pub pain_severity: u8,
    pub urgency: u8,
    pub competitor_differentiation: u8,
    pub evidence_strength: u8,
    pub total_score: u8,
    pub evidence_grade: String, // A, B, C, F
    pub confidence_level: String,
    pub missing_evidence: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderHandoff {
    pub id: String,
    pub idea_id: String,
    pub validated_problem: String,
    pub target_user: String,
    pub mvp_scope_summary: String,
    pub acceptance_criteria: Vec<String>,
    pub constraints: Vec<String>,
    pub risks: Vec<String>,
    pub validation_metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CofounderReport {
    pub idea_id: String,
    pub summary: String,
}

pub struct CofounderManager {
    base_dir: PathBuf,
    pub ideas: HashMap<String, CofounderIdea>,
    pub signals: HashMap<String, Vec<MarketSignal>>,
    pub experiments: HashMap<String, Vec<ValidationExperiment>>,
    pub mvps: HashMap<String, MVPHypothesis>,
    pub pricing: HashMap<String, PricingHypothesis>,
    pub handoffs: HashMap<String, FounderHandoff>,
}

impl CofounderManager {
    pub fn new() -> Result<Self> {
        let paths = crate::paths::GoatPaths::resolve()?;
        let base_dir = paths.data_dir.join("agents/prime/cofounder");
        fs::create_dir_all(&base_dir)?;

        let mut manager = Self {
            base_dir,
            ideas: HashMap::new(),
            signals: HashMap::new(),
            experiments: HashMap::new(),
            mvps: HashMap::new(),
            pricing: HashMap::new(),
            handoffs: HashMap::new(),
        };
        manager.load_all()?;
        Ok(manager)
    }

    fn ideas_file(&self) -> PathBuf {
        self.base_dir.join("ideas.jsonl")
    }
    fn signals_file(&self) -> PathBuf {
        self.base_dir.join("signals.jsonl")
    }
    fn experiments_file(&self) -> PathBuf {
        self.base_dir.join("experiments.jsonl")
    }
    fn mvps_file(&self) -> PathBuf {
        self.base_dir.join("mvps.jsonl")
    }
    fn pricing_file(&self) -> PathBuf {
        self.base_dir.join("pricing.jsonl")
    }
    fn handoffs_file(&self) -> PathBuf {
        self.base_dir.join("handoffs.jsonl")
    }

    fn load_all(&mut self) -> Result<()> {
        if self.ideas_file().exists() {
            for line in fs::read_to_string(self.ideas_file())?.lines() {
                if let Ok(idea) = serde_json::from_str::<CofounderIdea>(line) {
                    self.ideas.insert(idea.id.clone(), idea);
                }
            }
        }
        if self.signals_file().exists() {
            for line in fs::read_to_string(self.signals_file())?.lines() {
                if let Ok(sig) = serde_json::from_str::<MarketSignal>(line) {
                    self.signals
                        .entry(sig.idea_id.clone())
                        .or_default()
                        .push(sig);
                }
            }
        }
        if self.experiments_file().exists() {
            for line in fs::read_to_string(self.experiments_file())?.lines() {
                if let Ok(exp) = serde_json::from_str::<ValidationExperiment>(line) {
                    self.experiments
                        .entry(exp.idea_id.clone())
                        .or_default()
                        .push(exp);
                }
            }
        }
        if self.mvps_file().exists() {
            for line in fs::read_to_string(self.mvps_file())?.lines() {
                if let Ok(mvp) = serde_json::from_str::<MVPHypothesis>(line) {
                    self.mvps.insert(mvp.idea_id.clone(), mvp);
                }
            }
        }
        if self.pricing_file().exists() {
            for line in fs::read_to_string(self.pricing_file())?.lines() {
                if let Ok(ph) = serde_json::from_str::<PricingHypothesis>(line) {
                    self.pricing.insert(ph.idea_id.clone(), ph);
                }
            }
        }
        if self.handoffs_file().exists() {
            for line in fs::read_to_string(self.handoffs_file())?.lines() {
                if let Ok(ho) = serde_json::from_str::<FounderHandoff>(line) {
                    self.handoffs.insert(ho.idea_id.clone(), ho);
                }
            }
        }
        Ok(())
    }

    pub fn save_all(&self) -> Result<()> {
        let mut lines = Vec::new();
        for item in self.ideas.values() {
            lines.push(serde_json::to_string(item)?);
        }
        fs::write(self.ideas_file(), lines.join("\n"))?;

        lines.clear();
        for list in self.signals.values() {
            for item in list {
                lines.push(serde_json::to_string(item)?);
            }
        }
        fs::write(self.signals_file(), lines.join("\n"))?;

        lines.clear();
        for list in self.experiments.values() {
            for item in list {
                lines.push(serde_json::to_string(item)?);
            }
        }
        fs::write(self.experiments_file(), lines.join("\n"))?;

        lines.clear();
        for item in self.mvps.values() {
            lines.push(serde_json::to_string(item)?);
        }
        fs::write(self.mvps_file(), lines.join("\n"))?;

        lines.clear();
        for item in self.pricing.values() {
            lines.push(serde_json::to_string(item)?);
        }
        fs::write(self.pricing_file(), lines.join("\n"))?;

        lines.clear();
        for item in self.handoffs.values() {
            lines.push(serde_json::to_string(item)?);
        }
        fs::write(self.handoffs_file(), lines.join("\n"))?;

        Ok(())
    }

    pub fn add_idea(
        &mut self,
        title: String,
        pitch: String,
        audience: String,
    ) -> Result<CofounderIdea> {
        let id = Uuid::new_v4().to_string();
        let idea = CofounderIdea {
            id: id.clone(),
            title,
            one_line_pitch: pitch,
            target_user: TargetUserSegment {
                description: audience,
                demographics: vec![],
            },
            problem: FounderProblem {
                description: "Needs definition".into(),
                pain_intensity: 0,
                urgency: 0,
            },
            current_alternatives: vec![],
            willingness_to_pay_hypothesis: "Unknown".into(),
            distribution_channels: vec![],
            constraints: vec![],
            assumptions: vec![],
            risks: vec![],
            evidence_refs: vec![],
            validation_status: "New".into(),
            decision_state: FounderDecision::Undecided,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };
        self.ideas.insert(id, idea.clone());
        self.save_all()?;
        Ok(idea)
    }

    pub fn get_idea(&self, id: &str) -> Option<CofounderIdea> {
        self.ideas.get(id).cloned()
    }

    pub fn list_ideas(&self) -> Vec<CofounderIdea> {
        self.ideas.values().cloned().collect()
    }

    pub fn add_signal(&mut self, idea_id: &str, signal: MarketSignal) -> Result<()> {
        self.signals
            .entry(idea_id.to_string())
            .or_default()
            .push(signal);
        self.save_all()?;
        Ok(())
    }

    pub fn add_experiment(&mut self, idea_id: &str, exp: ValidationExperiment) -> Result<()> {
        self.experiments
            .entry(idea_id.to_string())
            .or_default()
            .push(exp);
        self.save_all()?;
        Ok(())
    }

    pub fn generate_mvp_scope(&mut self, idea_id: &str) -> Result<MVPHypothesis> {
        let mvp = MVPHypothesis {
            id: Uuid::new_v4().to_string(),
            idea_id: idea_id.to_string(),
            features: vec![MVPFeature {
                name: "Landing page smoke test".into(),
                description: "Collect emails".into(),
                is_must_have: true,
            }],
            non_goals: vec![MVPNonGoal {
                description: "Full backend automation".into(),
            }],
            risks: vec![MVPBuildRisk {
                description: "Technical unknowns in APIs".into(),
            }],
            weekly_plans: vec![MVPWeekPlan {
                week_number: 1,
                goals: vec!["Validate demand".into()],
            }],
            builder_handoff: MVPBuilderHandoff {
                ready: false,
                technical_unknowns: vec![],
            },
            riskiest_assumption_tested: "People want this".into(),
        };
        self.mvps.insert(idea_id.to_string(), mvp.clone());
        self.save_all()?;
        Ok(mvp)
    }

    pub fn generate_pricing_hypothesis(&mut self, idea_id: &str) -> Result<PricingHypothesis> {
        let ph = PricingHypothesis {
            id: Uuid::new_v4().to_string(),
            idea_id: idea_id.to_string(),
            free_manual_test_option: "Manual onboarding via concierge".into(),
            tiers: vec![PricingTier {
                name: "Starter".into(),
                price: "$10/mo".into(),
                description: "Basic value".into(),
            }],
            rationale: vec![PricingRationale {
                reason: "Competitors charge $20".into(),
            }],
            value_metric: "Usage".into(),
            pricing_risks: vec!["Willingness to pay might be lower".into()],
            evidence_needed: vec!["Paid pre-orders".into()],
        };
        self.pricing.insert(idea_id.to_string(), ph.clone());
        self.save_all()?;
        Ok(ph)
    }

    pub fn generate_builder_handoff(&mut self, idea_id: &str) -> Result<FounderHandoff> {
        let ho = FounderHandoff {
            id: Uuid::new_v4().to_string(),
            idea_id: idea_id.to_string(),
            validated_problem: "Clear pain identified".into(),
            target_user: "Specific niche".into(),
            mvp_scope_summary: "Core loop only".into(),
            acceptance_criteria: vec!["Users can complete core flow".into()],
            constraints: vec!["Local-first only".into()],
            risks: vec!["Distribution is hard".into()],
            validation_metric: "10 paid users".into(),
        };
        self.handoffs.insert(idea_id.to_string(), ho.clone());
        self.save_all()?;
        Ok(ho)
    }

    pub fn generate_scorecard(&mut self, id: &str) -> Result<CofounderScorecard> {
        let mut evidence_score = 0;
        if let Some(signals) = self.signals.get(id) {
            evidence_score = signals.len() as u8 * 5;
        }

        let total = 20 + evidence_score.min(40);
        let grade = if total > 50 { "B" } else { "F" };

        let score = CofounderScorecard {
            idea_id: id.to_string(),
            problem_clarity: 5,
            target_user_specificity: 5,
            pain_severity: 5,
            urgency: 5,
            competitor_differentiation: 5,
            evidence_strength: evidence_score.min(10),
            total_score: total.min(100),
            evidence_grade: grade.to_string(),
            confidence_level: if total > 50 {
                "Moderate".into()
            } else {
                "Low".into()
            },
            missing_evidence: vec!["Need direct user interviews".into()],
            recommendation: "Collect more market signals before building".into(),
        };
        Ok(score)
    }

    pub fn generate_report(&self, id: &str) -> Result<CofounderReport> {
        let rm = ReportManager::new();
        let template = ReportTemplate {
            kind: ReportKind::FounderValidationReport,
            title: format!("Cofounder Validation Report {}", id),
            sections: vec![
                ReportSection {
                    heading: "Executive Summary".into(),
                    body: "Evidence-first analysis".into(),
                },
                ReportSection {
                    heading: "Missing Evidence".into(),
                    body: "Market signals needed.".into(),
                },
                ReportSection {
                    heading: "Limitations".into(),
                    body: "No real interviews conducted yet.".into(),
                },
            ],
        };
        let rep = rm
            .generate_report(template)
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(CofounderReport {
            idea_id: id.to_string(),
            summary: format!("Generated report {}", rep.id),
        })
    }

    pub async fn deep_evaluate_idea(
        &mut self,
        id: &str,
        brain_manager: &crate::brain_index::BrainIndexManager,
        llm_router: &crate::llm::LlmRouter,
        model_chain: &crate::models::ModelChain,
    ) -> Result<CofounderScorecard> {
        let idea = self
            .ideas
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Idea not found"))?;
        let signals = self.signals.get(id).cloned().unwrap_or_default();
        let experiments = self.experiments.get(id).cloned().unwrap_or_default();

        let mut sys_prompt = String::from(
            "You are the GOAT Cofounder AI. You evaluate startup ideas ruthlessly using evidence-first principles.\n\
             Do not accept fake data or assumptions as fact. Evaluate strictly on market signals, validation experiments, and clear pain points.\n\
             Return ONLY a JSON object representing the CofounderScorecard. No markdown formatting, just pure JSON.\n\
             The JSON format must be:\n\
             {\n\
               \"idea_id\": \"string\",\n\
               \"total_score\": number,\n\
               \"market_pain_score\": number,\n\
               \"evidence_score\": number,\n\
               \"feasibility_score\": number,\n\
               \"differentiation_score\": number,\n\
               \"critical_risks\": [\"string\"],\n\
               \"strongest_signals\": [\"string\"],\n\
               \"verdict\": \"string\",\n\
               \"next_action\": \"string\"\n\
             }",
        );

        let mut user_prompt = format!(
            "Evaluate this idea:\nTitle: {}\nPitch: {}\nAudience: {:?}\n\n",
            idea.title, idea.one_line_pitch, idea.target_user
        );

        user_prompt.push_str("Market Signals:\n");
        for s in &signals {
            user_prompt.push_str(&format!(
                "- [{:?}] (Strength: {:?}) {}\n",
                s.signal_type, s.strength, s.description
            ));
        }

        user_prompt.push_str("\nValidation Experiments:\n");
        for e in &experiments {
            let status = if e.result.is_some() {
                "Completed"
            } else {
                "Pending"
            };
            user_prompt.push_str(&format!("- [{}] Status: {}\n", e.experiment_type, status));
            if let Some(r) = &e.result {
                user_prompt.push_str(&format!(
                    "  Success: {}, Evidence: {}\n",
                    r.success, r.evidence_collected
                ));
            }
        }

        // Fetch similar past ideas from the brain
        let context_packer =
            crate::agent_quality::AgentContextPacker::new(brain_manager, "cofounder");
        if let Ok(context) = context_packer.pack_for_task(&idea.title).await {
            user_prompt.push_str(&format!(
                "\nBrain Context / Past Knowledge:\n{:?}\n",
                context
            ));
        }

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
            .map_err(|e| anyhow!("LLM Evaluation failed: {}", e))?;

        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<CofounderScorecard>(cleaned) {
            Ok(mut scorecard) => {
                scorecard.idea_id = id.to_string();
                Ok(scorecard)
            }
            Err(_) => {
                // Fallback to hardcoded mock if LLM fails to return valid JSON
                self.generate_scorecard(id)
            }
        }
    }

    pub async fn deep_generate_mvp_scope(
        &mut self,
        id: &str,
        _brain_manager: &crate::brain_index::BrainIndexManager,
        llm_router: &crate::llm::LlmRouter,
        model_chain: &crate::models::ModelChain,
    ) -> Result<MVPHypothesis> {
        let idea = self
            .ideas
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Idea not found"))?;

        let sys_prompt = String::from(
            "You are the GOAT Cofounder AI. You are scoping an MVP for a validated idea.\n\
             Be ruthlessly minimal. Only include features necessary to test the riskiest assumption.\n\
             Return ONLY a JSON object representing the MVPHypothesis. No markdown formatting, just pure JSON.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"idea_id\": \"string\",\n\
               \"features\": [{\"name\": \"string\", \"description\": \"string\", \"is_must_have\": bool}],\n\
               \"non_goals\": [{\"description\": \"string\"}],\n\
               \"risks\": [{\"description\": \"string\"}],\n\
               \"weekly_plans\": [{\"week_number\": number, \"goals\": [\"string\"]}],\n\
               \"builder_handoff\": {\"ready\": bool, \"technical_unknowns\": [\"string\"]},\n\
               \"riskiest_assumption_tested\": \"string\"\n\
             }",
        );

        let user_prompt = format!(
            "Scope an MVP for this idea:\nTitle: {}\nPitch: {}\nAudience: {:?}\nProblem: {:?}\n",
            idea.title, idea.one_line_pitch, idea.target_user, idea.problem
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
            .map_err(|e| anyhow!("LLM Evaluation failed: {}", e))?;

        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<MVPHypothesis>(cleaned) {
            Ok(mut mvp) => {
                mvp.idea_id = id.to_string();
                mvp.id = uuid::Uuid::new_v4().to_string();
                Ok(mvp)
            }
            Err(_) => self.generate_mvp_scope(id),
        }
    }

    pub async fn deep_generate_pricing_hypothesis(
        &mut self,
        id: &str,
        _brain_manager: &crate::brain_index::BrainIndexManager,
        llm_router: &crate::llm::LlmRouter,
        model_chain: &crate::models::ModelChain,
    ) -> Result<PricingHypothesis> {
        let idea = self
            .ideas
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Idea not found"))?;

        let sys_prompt = String::from(
            "You are the GOAT Cofounder AI. You are creating a pricing hypothesis for a startup idea.\n\
             Focus on the value metric and willingness to pay. Do not arbitrarily pick standard SaaS pricing unless justified.\n\
             Return ONLY a JSON object representing the PricingHypothesis. No markdown formatting, just pure JSON.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"idea_id\": \"string\",\n\
               \"free_manual_test_option\": \"string\",\n\
               \"tiers\": [{\"name\": \"string\", \"price\": \"string\", \"description\": \"string\"}],\n\
               \"rationale\": [{\"reason\": \"string\"}],\n\
               \"value_metric\": \"string\",\n\
               \"pricing_risks\": [\"string\"],\n\
               \"evidence_needed\": [\"string\"]\n\
             }",
        );

        let user_prompt = format!(
            "Generate pricing hypothesis for this idea:\nTitle: {}\nPitch: {}\nAudience: {:?}\nProblem: {:?}\n",
            idea.title, idea.one_line_pitch, idea.target_user, idea.problem
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
            .map_err(|e| anyhow!("LLM Evaluation failed: {}", e))?;

        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<PricingHypothesis>(cleaned) {
            Ok(mut pricing) => {
                pricing.idea_id = id.to_string();
                pricing.id = uuid::Uuid::new_v4().to_string();
                Ok(pricing)
            }
            Err(_) => self.generate_pricing_hypothesis(id),
        }
    }

    pub async fn deep_generate_builder_handoff(
        &mut self,
        id: &str,
        _brain_manager: &crate::brain_index::BrainIndexManager,
        llm_router: &crate::llm::LlmRouter,
        model_chain: &crate::models::ModelChain,
    ) -> Result<FounderHandoff> {
        let idea = self
            .ideas
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Idea not found"))?;

        let sys_prompt = String::from(
            "You are the GOAT Cofounder AI. You are creating a builder handoff document.\n\
             Translate the business requirements and validated assumptions into technical direction.\n\
             Return ONLY a JSON object representing the FounderHandoff. No markdown formatting, just pure JSON.\n\
             The JSON format must be:\n\
             {\n\
               \"id\": \"string\",\n\
               \"idea_id\": \"string\",\n\
               \"validated_problem\": \"string\",\n\
               \"target_user\": \"string\",\n\
               \"mvp_scope_summary\": \"string\",\n\
               \"acceptance_criteria\": [\"string\"],\n\
               \"constraints\": [\"string\"],\n\
               \"risks\": [\"string\"],\n\
               \"validation_metric\": \"string\"\n\
             }",
        );

        let user_prompt = format!(
            "Generate builder handoff for this idea:\nTitle: {}\nPitch: {}\nAudience: {:?}\nProblem: {:?}\n",
            idea.title, idea.one_line_pitch, idea.target_user, idea.problem
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
            .map_err(|e| anyhow!("LLM Evaluation failed: {}", e))?;

        let text = response.content.unwrap_or_default().trim().to_string();
        let cleaned = text
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<FounderHandoff>(cleaned) {
            Ok(mut handoff) => {
                handoff.idea_id = id.to_string();
                handoff.id = uuid::Uuid::new_v4().to_string();
                Ok(handoff)
            }
            Err(_) => self.generate_builder_handoff(id),
        }
    }
}
