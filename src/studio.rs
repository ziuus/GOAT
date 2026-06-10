use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DraftType {
    Prompt,
    Skill,
    Agent,
    Workflow,
    Comparison,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioDraft {
    pub id: String,
    pub draft_type: DraftType,
    pub content: serde_json::Value,
    pub created_at: i64,
}

#[derive(Clone)]
pub struct StudioManager {
    drafts: Arc<Mutex<HashMap<String, StudioDraft>>>,
    storage_path: PathBuf,
}

impl StudioManager {
    pub fn new() -> Self {
        let storage_path = dirs::data_dir()
            .map(|d| d.join("goat/studio/studio_drafts.json"))
            .unwrap_or_else(|| PathBuf::from("studio_drafts.json"));

        let mut sm = StudioManager {
            drafts: Arc::new(Mutex::new(HashMap::new())),
            storage_path,
        };
        sm.load_drafts();
        sm
    }

    fn load_drafts(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.storage_path) {
            if let Ok(drafts) = serde_json::from_str::<Vec<StudioDraft>>(&data) {
                let mut map = self.drafts.lock().unwrap();
                for draft in drafts {
                    map.insert(draft.id.clone(), draft);
                }
            }
        }
    }

    fn save_drafts(&self) {
        if let Some(parent) = self.storage_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let map = self.drafts.lock().unwrap();
        let drafts: Vec<&StudioDraft> = map.values().collect();
        if let Ok(data) = serde_json::to_string_pretty(&drafts) {
            let _ = fs::write(&self.storage_path, data);
        }
    }

    pub fn list_drafts(&self) -> Vec<StudioDraft> {
        let map = self.drafts.lock().unwrap();
        let mut list: Vec<StudioDraft> = map.values().cloned().collect();
        list.sort_by_key(|d| d.created_at);
        list
    }

    pub fn get_draft(&self, id: &str) -> Option<StudioDraft> {
        let map = self.drafts.lock().unwrap();
        map.get(id).cloned()
    }

    pub fn save_draft(&self, draft_type: DraftType, content: serde_json::Value) -> StudioDraft {
        let draft = StudioDraft {
            id: Uuid::new_v4().to_string(),
            draft_type,
            content,
            created_at: chrono::Utc::now().timestamp(),
        };
        {
            let mut map = self.drafts.lock().unwrap();
            map.insert(draft.id.clone(), draft.clone());
        }
        self.save_drafts();
        draft
    }

    pub fn delete_draft(&self, id: &str) {
        {
            let mut map = self.drafts.lock().unwrap();
            map.remove(id);
        }
        self.save_drafts();
    }
}
