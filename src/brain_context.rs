use crate::brain_models::*;
use crate::brain_index::BrainIndexManager;
use anyhow::Result;

pub struct BrainContextPackBuilder<'a> {
    manager: &'a BrainIndexManager,
    query: String,
    agent_id: Option<String>,
    project_id: Option<String>,
    max_items: usize,
    max_chars: usize,
    include_reports: bool,
    include_timeline: bool,
    include_runtime: bool,
    include_files: bool,
    include_promptforge: bool,
}

impl<'a> BrainContextPackBuilder<'a> {
    pub fn new(manager: &'a BrainIndexManager, query: String) -> Self {
        Self {
            manager,
            query,
            agent_id: None,
            project_id: None,
            max_items: 10,
            max_chars: 16000, // 16k chars default
            include_reports: true,
            include_timeline: true,
            include_runtime: true,
            include_files: true,
            include_promptforge: true,
        }
    }

    pub fn with_agent(mut self, agent_id: String) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    pub fn with_project(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn limit_items(mut self, max: usize) -> Self {
        self.max_items = max;
        self
    }

    pub fn limit_chars(mut self, max: usize) -> Self {
        self.max_chars = max;
        self
    }

    pub async fn build(self) -> Result<BrainContextPack> {
        let mut kind_filter = Vec::new();
        kind_filter.push(BrainDocumentKind::Memory);
        kind_filter.push(BrainDocumentKind::Skill);
        kind_filter.push(BrainDocumentKind::Recipe);
        
        if self.include_reports {
            kind_filter.push(BrainDocumentKind::Report);
            kind_filter.push(BrainDocumentKind::OperatorReport);
            kind_filter.push(BrainDocumentKind::DesignerReview);
            kind_filter.push(BrainDocumentKind::ResearcherBrief);
        }
        if self.include_timeline {
            kind_filter.push(BrainDocumentKind::TimelineEvent);
        }
        if self.include_runtime {
            kind_filter.push(BrainDocumentKind::RuntimeJob);
            kind_filter.push(BrainDocumentKind::RuntimeArtifact);
            kind_filter.push(BrainDocumentKind::Job);
        }
        if self.include_files {
            kind_filter.push(BrainDocumentKind::File);
        }
        if self.include_promptforge {
            kind_filter.push(BrainDocumentKind::PromptForgeHistory);
            kind_filter.push(BrainDocumentKind::PromptForgeTemplate);
        }

        let sq = BrainSearchQuery {
            q: self.query.clone(),
            limit: self.max_items * 2, // overfetch slightly
            kind_filter: Some(kind_filter),
            mode: BrainSearchMode::Hybrid,
            agent_id: self.agent_id.clone(),
            project_id: self.project_id.clone(),
        };

        let start = std::time::Instant::now();
        let results = self.manager.search(&sq).await?;
        let elapsed = start.elapsed();

        let mut items = Vec::new();
        let mut current_chars = 0;
        let mut source_refs = Vec::new();
        let mut top_score = 0.0;

        for res in results {
            if top_score == 0.0 {
                top_score = res.score;
            }
            if current_chars + res.document.body.len() > self.max_chars {
                continue;
            }
            current_chars += res.document.body.len();
            source_refs.push(format!("{}::{}", res.document.source.source_kind.clone() as u8, res.document.source.source_id.clone()));
            items.push(res.document);
            if items.len() >= self.max_items {
                break;
            }
        }

        let summary = format!("Context Pack generated for query '{}'. Included {} items.", self.query, items.len());

        let trace = BrainRecallTrace {
            query: self.query.clone(),
            mode: BrainSearchMode::Hybrid,
            returned_results: items.len(),
            top_score,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        Ok(BrainContextPack {
            title: format!("Context Pack: {}", self.query),
            items,
            source_refs,
            summary,
            warnings: vec![],
            estimated_size: current_chars,
            recall_trace: vec![trace],
        })
    }
}
